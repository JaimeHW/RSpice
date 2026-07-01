#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18310_e25840, assign18310_e25840_d_n0, assign18310_e25840_d_n2, assign18310_e25840_d_n6, assign18310_e25840_d_n7, assign18310_e25840_d_n10, assign18310_e25840_d_n11, assign18310_e25840_d_n12, assign18310_e25840_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18310_e25840;
        locals.var_idspt_dn0 = assign18310_e25840_d_n0;
        locals.var_idspt_dn2 = assign18310_e25840_d_n2;
        locals.var_idspt_dn6 = assign18310_e25840_d_n6;
        locals.var_idspt_dn7 = assign18310_e25840_d_n7;
        locals.var_idspt_dn10 = assign18310_e25840_d_n10;
        locals.var_idspt_dn11 = assign18310_e25840_d_n11;
        locals.var_idspt_dn12 = assign18310_e25840_d_n12;
        locals.var_idspt_dn17 = assign18310_e25840_d_n17;
        locals.var_idspt_rv = 0.0;

        let assign18320_e25847: f64 = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard570 = assign18320_e25847;
        locals.var_guard570_rv = 0.0;

        let (assign18330_e25857, assign18330_e25857_d_n0, assign18330_e25857_d_n2, assign18330_e25857_d_n6, assign18330_e25857_d_n7, assign18330_e25857_d_n10, assign18330_e25857_d_n11, assign18330_e25857_d_n12, assign18330_e25857_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18330_e25854: f64 = (locals.var_vds - locals.var_pds);
        let assign18330_e25855: f64 = (0.5 * assign18330_e25854);
        (assign18330_e25855, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn12 - locals.var_pds_dn12)), (0.5 * (locals.var_vds_dn17 - locals.var_pds_dn17)),)
    } else {
        (locals.var_t1__blk561, locals.var_t1__blk561_dn0, locals.var_t1__blk561_dn2, locals.var_t1__blk561_dn6, locals.var_t1__blk561_dn7, locals.var_t1__blk561_dn10, locals.var_t1__blk561_dn11, locals.var_t1__blk561_dn12, locals.var_t1__blk561_dn17,)
    }
};
        locals.var_t1__blk561 = assign18330_e25857;
        locals.var_t1__blk561_dn0 = assign18330_e25857_d_n0;
        locals.var_t1__blk561_dn2 = assign18330_e25857_d_n2;
        locals.var_t1__blk561_dn6 = assign18330_e25857_d_n6;
        locals.var_t1__blk561_dn7 = assign18330_e25857_d_n7;
        locals.var_t1__blk561_dn10 = assign18330_e25857_d_n10;
        locals.var_t1__blk561_dn11 = assign18330_e25857_d_n11;
        locals.var_t1__blk561_dn12 = assign18330_e25857_d_n12;
        locals.var_t1__blk561_dn17 = assign18330_e25857_d_n17;
        locals.var_t1__blk561_rv = 0.0;

        let (assign18340_e25867, assign18340_e25867_d_n0, assign18340_e25867_d_n2, assign18340_e25867_d_n6, assign18340_e25867_d_n7, assign18340_e25867_d_n10, assign18340_e25867_d_n11, assign18340_e25867_d_n12, assign18340_e25867_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18340_e25863: f64 = (2.0 * locals.var_t1__blk561);
        let assign18340_e25865: f64 = (assign18340_e25863 / 0.01);
        (assign18340_e25865, ((2.0 * locals.var_t1__blk561_dn0) / 0.01), ((2.0 * locals.var_t1__blk561_dn2) / 0.01), ((2.0 * locals.var_t1__blk561_dn6) / 0.01), ((2.0 * locals.var_t1__blk561_dn7) / 0.01), ((2.0 * locals.var_t1__blk561_dn10) / 0.01), ((2.0 * locals.var_t1__blk561_dn11) / 0.01), ((2.0 * locals.var_t1__blk561_dn12) / 0.01), ((2.0 * locals.var_t1__blk561_dn17) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18340_e25867;
        locals.var_tmf1_dn0 = assign18340_e25867_d_n0;
        locals.var_tmf1_dn2 = assign18340_e25867_d_n2;
        locals.var_tmf1_dn6 = assign18340_e25867_d_n6;
        locals.var_tmf1_dn7 = assign18340_e25867_d_n7;
        locals.var_tmf1_dn10 = assign18340_e25867_d_n10;
        locals.var_tmf1_dn11 = assign18340_e25867_d_n11;
        locals.var_tmf1_dn12 = assign18340_e25867_d_n12;
        locals.var_tmf1_dn17 = assign18340_e25867_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18350_e25909, assign18350_e25909_d_n0, assign18350_e25909_d_n2, assign18350_e25909_d_n6, assign18350_e25909_d_n7, assign18350_e25909_d_n10, assign18350_e25909_d_n11, assign18350_e25909_d_n12, assign18350_e25909_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18350_e25875: f64 = (1.0 / 2.0);
        let assign18350_e25879: f64 = (1.0 / 6.0);
        let assign18350_e25883: f64 = (1.0 / 24.0);
        let assign18350_e25887: f64 = (1.0 / 120.0);
        let assign18350_e25891: f64 = (1.0 / 720.0);
        let assign18350_e25895: f64 = (1.0 / 5040.0);
        let assign18350_e25896: f64 = (locals.var_tmf1 * assign18350_e25895);
        let assign18350_e25897: f64 = (assign18350_e25891 + assign18350_e25896);
        let assign18350_e25898: f64 = (locals.var_tmf1 * assign18350_e25897);
        let assign18350_e25899: f64 = (assign18350_e25887 + assign18350_e25898);
        let assign18350_e25900: f64 = (locals.var_tmf1 * assign18350_e25899);
        let assign18350_e25901: f64 = (assign18350_e25883 + assign18350_e25900);
        let assign18350_e25902: f64 = (locals.var_tmf1 * assign18350_e25901);
        let assign18350_e25903: f64 = (assign18350_e25879 + assign18350_e25902);
        let assign18350_e25904: f64 = (locals.var_tmf1 * assign18350_e25903);
        let assign18350_e25905: f64 = (assign18350_e25875 + assign18350_e25904);
        let assign18350_e25906: f64 = (locals.var_tmf1 * assign18350_e25905);
        let assign18350_e25907: f64 = (1.0 + assign18350_e25906);
        (assign18350_e25907, ((locals.var_tmf1_dn0 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn2 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn6 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn7 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn10 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn11 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn12 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign18350_e25895))))))))))), ((locals.var_tmf1_dn17 * assign18350_e25905) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18350_e25903) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18350_e25901) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18350_e25899) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18350_e25897) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign18350_e25895))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign18350_e25909;
        locals.var_tmf2_dn0 = assign18350_e25909_d_n0;
        locals.var_tmf2_dn2 = assign18350_e25909_d_n2;
        locals.var_tmf2_dn6 = assign18350_e25909_d_n6;
        locals.var_tmf2_dn7 = assign18350_e25909_d_n7;
        locals.var_tmf2_dn10 = assign18350_e25909_d_n10;
        locals.var_tmf2_dn11 = assign18350_e25909_d_n11;
        locals.var_tmf2_dn12 = assign18350_e25909_d_n12;
        locals.var_tmf2_dn17 = assign18350_e25909_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign18360_e25917, assign18360_e25917_d_n0, assign18360_e25917_d_n2, assign18360_e25917_d_n6, assign18360_e25917_d_n7, assign18360_e25917_d_n10, assign18360_e25917_d_n11, assign18360_e25917_d_n12, assign18360_e25917_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18360_e25915: f64 = (0.01 / locals.var_tmf2);
        (assign18360_e25915, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6__blk567, locals.var_t6__blk567_dn0, locals.var_t6__blk567_dn2, locals.var_t6__blk567_dn6, locals.var_t6__blk567_dn7, locals.var_t6__blk567_dn10, locals.var_t6__blk567_dn11, locals.var_t6__blk567_dn12, locals.var_t6__blk567_dn17,)
    }
};
        locals.var_t6__blk567 = assign18360_e25917;
        locals.var_t6__blk567_dn0 = assign18360_e25917_d_n0;
        locals.var_t6__blk567_dn2 = assign18360_e25917_d_n2;
        locals.var_t6__blk567_dn6 = assign18360_e25917_d_n6;
        locals.var_t6__blk567_dn7 = assign18360_e25917_d_n7;
        locals.var_t6__blk567_dn10 = assign18360_e25917_d_n10;
        locals.var_t6__blk567_dn11 = assign18360_e25917_d_n11;
        locals.var_t6__blk567_dn12 = assign18360_e25917_d_n12;
        locals.var_t6__blk567_dn17 = assign18360_e25917_d_n17;
        locals.var_t6__blk567_rv = 0.0;

        let (assign18370_e25927, assign18370_e25927_d_n0, assign18370_e25927_d_n2, assign18370_e25927_d_n6, assign18370_e25927_d_n7, assign18370_e25927_d_n10, assign18370_e25927_d_n11, assign18370_e25927_d_n12, assign18370_e25927_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18370_e25924: f64 = (locals.var_ps0 + locals.var_t6__blk567);
        let assign18370_e25925: f64 = (1.1 - assign18370_e25924);
        (assign18370_e25925, (-(locals.var_ps0_dn0 + locals.var_t6__blk567_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6__blk567_dn2)), (-(locals.var_ps0_dn6 + locals.var_t6__blk567_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6__blk567_dn7)), (-(locals.var_ps0_dn10 + locals.var_t6__blk567_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6__blk567_dn11)), (-(locals.var_ps0_dn12 + locals.var_t6__blk567_dn12)), (-(locals.var_ps0_dn17 + locals.var_t6__blk567_dn17)),)
    } else {
        (locals.var_t1__blk561, locals.var_t1__blk561_dn0, locals.var_t1__blk561_dn2, locals.var_t1__blk561_dn6, locals.var_t1__blk561_dn7, locals.var_t1__blk561_dn10, locals.var_t1__blk561_dn11, locals.var_t1__blk561_dn12, locals.var_t1__blk561_dn17,)
    }
};
        locals.var_t1__blk561 = assign18370_e25927;
        locals.var_t1__blk561_dn0 = assign18370_e25927_d_n0;
        locals.var_t1__blk561_dn2 = assign18370_e25927_d_n2;
        locals.var_t1__blk561_dn6 = assign18370_e25927_d_n6;
        locals.var_t1__blk561_dn7 = assign18370_e25927_d_n7;
        locals.var_t1__blk561_dn10 = assign18370_e25927_d_n10;
        locals.var_t1__blk561_dn11 = assign18370_e25927_d_n11;
        locals.var_t1__blk561_dn12 = assign18370_e25927_d_n12;
        locals.var_t1__blk561_dn17 = assign18370_e25927_d_n17;
        locals.var_t1__blk561_rv = 0.0;

        let (assign18380_e25942, assign18380_e25942_d_n0, assign18380_e25942_d_n2, assign18380_e25942_d_n6, assign18380_e25942_d_n7, assign18380_e25942_d_n10, assign18380_e25942_d_n11, assign18380_e25942_d_n12, assign18380_e25942_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18380_e25933: f64 = (locals.var_t1__blk561 * locals.var_t1__blk561);
        let assign18380_e25936: f64 = (4.0 * 0.05);
        let assign18380_e25938: f64 = (assign18380_e25936 * 0.05);
        let assign18380_e25939: f64 = (assign18380_e25933 + assign18380_e25938);
        let assign18380_e25940: f64 = (assign18380_e25939).sqrt();
        (assign18380_e25940, (((locals.var_t1__blk561_dn0 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn0)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn2 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn2)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn6 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn6)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn7 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn7)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn10 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn10)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn11 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn11)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn12 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn12)) / (2.0 * assign18380_e25940)), (((locals.var_t1__blk561_dn17 * locals.var_t1__blk561) + (locals.var_t1__blk561 * locals.var_t1__blk561_dn17)) / (2.0 * assign18380_e25940)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18380_e25942;
        locals.var_tmf1_dn0 = assign18380_e25942_d_n0;
        locals.var_tmf1_dn2 = assign18380_e25942_d_n2;
        locals.var_tmf1_dn6 = assign18380_e25942_d_n6;
        locals.var_tmf1_dn7 = assign18380_e25942_d_n7;
        locals.var_tmf1_dn10 = assign18380_e25942_d_n10;
        locals.var_tmf1_dn11 = assign18380_e25942_d_n11;
        locals.var_tmf1_dn12 = assign18380_e25942_d_n12;
        locals.var_tmf1_dn17 = assign18380_e25942_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18390_e25956, assign18390_e25956_d_n0, assign18390_e25956_d_n2, assign18390_e25956_d_n6, assign18390_e25956_d_n7, assign18390_e25956_d_n10, assign18390_e25956_d_n11, assign18390_e25956_d_n12, assign18390_e25956_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18390_e25949: f64 = (locals.var_t1__blk561 + locals.var_tmf1);
        let assign18390_e25950: f64 = (0.5 * assign18390_e25949);
        let assign18390_e25953: f64 = (1e-10 * 0.05);
        let assign18390_e25954: f64 = (assign18390_e25950 + assign18390_e25953);
        (assign18390_e25954, (0.5 * (locals.var_t1__blk561_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1__blk561_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1__blk561_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1__blk561_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1__blk561_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1__blk561_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1__blk561_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1__blk561_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk569, locals.var_t2__blk569_dn0, locals.var_t2__blk569_dn2, locals.var_t2__blk569_dn6, locals.var_t2__blk569_dn7, locals.var_t2__blk569_dn10, locals.var_t2__blk569_dn11, locals.var_t2__blk569_dn12, locals.var_t2__blk569_dn17,)
    }
};
        locals.var_t2__blk569 = assign18390_e25956;
        locals.var_t2__blk569_dn0 = assign18390_e25956_d_n0;
        locals.var_t2__blk569_dn2 = assign18390_e25956_d_n2;
        locals.var_t2__blk569_dn6 = assign18390_e25956_d_n6;
        locals.var_t2__blk569_dn7 = assign18390_e25956_d_n7;
        locals.var_t2__blk569_dn10 = assign18390_e25956_d_n10;
        locals.var_t2__blk569_dn11 = assign18390_e25956_d_n11;
        locals.var_t2__blk569_dn12 = assign18390_e25956_d_n12;
        locals.var_t2__blk569_dn17 = assign18390_e25956_d_n17;
        locals.var_t2__blk569_rv = 0.0;

        let assign18400_e25959: f64 = if locals.var_t2__blk569 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign18400_e25959;
        locals.var_guard571_rv = 0.0;

        let (assign18410_e25967, assign18410_e25967_d_n0, assign18410_e25967_d_n2, assign18410_e25967_d_n6, assign18410_e25967_d_n7, assign18410_e25967_d_n10, assign18410_e25967_d_n11, assign18410_e25967_d_n12, assign18410_e25967_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk569, locals.var_t2__blk569_dn0, locals.var_t2__blk569_dn2, locals.var_t2__blk569_dn6, locals.var_t2__blk569_dn7, locals.var_t2__blk569_dn10, locals.var_t2__blk569_dn11, locals.var_t2__blk569_dn12, locals.var_t2__blk569_dn17,)
    }
};
        locals.var_t2__blk569 = assign18410_e25967;
        locals.var_t2__blk569_dn0 = assign18410_e25967_d_n0;
        locals.var_t2__blk569_dn2 = assign18410_e25967_d_n2;
        locals.var_t2__blk569_dn6 = assign18410_e25967_d_n6;
        locals.var_t2__blk569_dn7 = assign18410_e25967_d_n7;
        locals.var_t2__blk569_dn10 = assign18410_e25967_d_n10;
        locals.var_t2__blk569_dn11 = assign18410_e25967_d_n11;
        locals.var_t2__blk569_dn12 = assign18410_e25967_d_n12;
        locals.var_t2__blk569_dn17 = assign18410_e25967_d_n17;
        locals.var_t2__blk569_rv = 0.0;

        let (assign18420_e25975, assign18420_e25975_d_n0, assign18420_e25975_d_n2, assign18420_e25975_d_n6, assign18420_e25975_d_n7, assign18420_e25975_d_n10, assign18420_e25975_d_n11, assign18420_e25975_d_n12, assign18420_e25975_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18420_e25973: f64 = (locals.var_beta * locals.var_ptl0);
        (assign18420_e25973, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_ptl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk562, locals.var_t0__blk562_dn0, locals.var_t0__blk562_dn2, locals.var_t0__blk562_dn6, locals.var_t0__blk562_dn7, locals.var_t0__blk562_dn10, locals.var_t0__blk562_dn11, locals.var_t0__blk562_dn12, locals.var_t0__blk562_dn17,)
    }
};
        locals.var_t0__blk562 = assign18420_e25975;
        locals.var_t0__blk562_dn0 = assign18420_e25975_d_n0;
        locals.var_t0__blk562_dn2 = assign18420_e25975_d_n2;
        locals.var_t0__blk562_dn6 = assign18420_e25975_d_n6;
        locals.var_t0__blk562_dn7 = assign18420_e25975_d_n7;
        locals.var_t0__blk562_dn10 = assign18420_e25975_d_n10;
        locals.var_t0__blk562_dn11 = assign18420_e25975_d_n11;
        locals.var_t0__blk562_dn12 = assign18420_e25975_d_n12;
        locals.var_t0__blk562_dn17 = assign18420_e25975_d_n17;
        locals.var_t0__blk562_rv = 0.0;

        let (assign18430_e25983, assign18430_e25983_d_n0, assign18430_e25983_d_n2, assign18430_e25983_d_n6, assign18430_e25983_d_n7, assign18430_e25983_d_n10, assign18430_e25983_d_n11, assign18430_e25983_d_n12, assign18430_e25983_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18430_e25981: f64 = (locals.var_c_fox * locals.var_t0__blk562);
        (assign18430_e25981, ((locals.var_c_fox_dn0 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn0)), ((locals.var_c_fox_dn2 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn2)), ((locals.var_c_fox_dn6 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn6)), ((locals.var_c_fox_dn7 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn7)), ((locals.var_c_fox_dn10 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn10)), ((locals.var_c_fox_dn11 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn11)), ((locals.var_c_fox_dn12 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn12)), ((locals.var_c_fox_dn17 * locals.var_t0__blk562) + (locals.var_c_fox * locals.var_t0__blk562_dn17)),)
    } else {
        (locals.var_t3__blk563, locals.var_t3__blk563_dn0, locals.var_t3__blk563_dn2, locals.var_t3__blk563_dn6, locals.var_t3__blk563_dn7, locals.var_t3__blk563_dn10, locals.var_t3__blk563_dn11, locals.var_t3__blk563_dn12, locals.var_t3__blk563_dn17,)
    }
};
        locals.var_t3__blk563 = assign18430_e25983;
        locals.var_t3__blk563_dn0 = assign18430_e25983_d_n0;
        locals.var_t3__blk563_dn2 = assign18430_e25983_d_n2;
        locals.var_t3__blk563_dn6 = assign18430_e25983_d_n6;
        locals.var_t3__blk563_dn7 = assign18430_e25983_d_n7;
        locals.var_t3__blk563_dn10 = assign18430_e25983_d_n10;
        locals.var_t3__blk563_dn11 = assign18430_e25983_d_n11;
        locals.var_t3__blk563_dn12 = assign18430_e25983_d_n12;
        locals.var_t3__blk563_dn17 = assign18430_e25983_d_n17;
        locals.var_t3__blk563_rv = 0.0;

        let (assign18440_e25991, assign18440_e25991_d_n0, assign18440_e25991_d_n2, assign18440_e25991_d_n6, assign18440_e25991_d_n7, assign18440_e25991_d_n10, assign18440_e25991_d_n11, assign18440_e25991_d_n12, assign18440_e25991_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18440_e25989: f64 = (locals.var_t2__blk569).powf(p.p245);
        (assign18440_e25989, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn0)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn0 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn2)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn2 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn6)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn6 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn7)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn7 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn10)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn10 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn11)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn11 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn12)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn12 / locals.var_t2__blk569))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk569).powf(p.p245 - 1.0) * locals.var_t2__blk569_dn17)) } } else { (assign18440_e25989 * (p.p245 * (locals.var_t2__blk569_dn17 / locals.var_t2__blk569))) },)
    } else {
        (locals.var_t0__blk562, locals.var_t0__blk562_dn0, locals.var_t0__blk562_dn2, locals.var_t0__blk562_dn6, locals.var_t0__blk562_dn7, locals.var_t0__blk562_dn10, locals.var_t0__blk562_dn11, locals.var_t0__blk562_dn12, locals.var_t0__blk562_dn17,)
    }
};
        locals.var_t0__blk562 = assign18440_e25991;
        locals.var_t0__blk562_dn0 = assign18440_e25991_d_n0;
        locals.var_t0__blk562_dn2 = assign18440_e25991_d_n2;
        locals.var_t0__blk562_dn6 = assign18440_e25991_d_n6;
        locals.var_t0__blk562_dn7 = assign18440_e25991_d_n7;
        locals.var_t0__blk562_dn10 = assign18440_e25991_d_n10;
        locals.var_t0__blk562_dn11 = assign18440_e25991_d_n11;
        locals.var_t0__blk562_dn12 = assign18440_e25991_d_n12;
        locals.var_t0__blk562_dn17 = assign18440_e25991_d_n17;
        locals.var_t0__blk562_rv = 0.0;

        let (assign18450_e25999, assign18450_e25999_d_n0, assign18450_e25999_d_n2, assign18450_e25999_d_n6, assign18450_e25999_d_n7, assign18450_e25999_d_n10, assign18450_e25999_d_n11, assign18450_e25999_d_n12, assign18450_e25999_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18450_e25997: f64 = (locals.var_t3__blk563 * locals.var_t0__blk562);
        (assign18450_e25997, ((locals.var_t3__blk563_dn0 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn0)), ((locals.var_t3__blk563_dn2 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn2)), ((locals.var_t3__blk563_dn6 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn6)), ((locals.var_t3__blk563_dn7 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn7)), ((locals.var_t3__blk563_dn10 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn10)), ((locals.var_t3__blk563_dn11 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn11)), ((locals.var_t3__blk563_dn12 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn12)), ((locals.var_t3__blk563_dn17 * locals.var_t0__blk562) + (locals.var_t3__blk563 * locals.var_t0__blk562_dn17)),)
    } else {
        (locals.var_t9__blk564, locals.var_t9__blk564_dn0, locals.var_t9__blk564_dn2, locals.var_t9__blk564_dn6, locals.var_t9__blk564_dn7, locals.var_t9__blk564_dn10, locals.var_t9__blk564_dn11, locals.var_t9__blk564_dn12, locals.var_t9__blk564_dn17,)
    }
};
        locals.var_t9__blk564 = assign18450_e25999;
        locals.var_t9__blk564_dn0 = assign18450_e25999_d_n0;
        locals.var_t9__blk564_dn2 = assign18450_e25999_d_n2;
        locals.var_t9__blk564_dn6 = assign18450_e25999_d_n6;
        locals.var_t9__blk564_dn7 = assign18450_e25999_d_n7;
        locals.var_t9__blk564_dn10 = assign18450_e25999_d_n10;
        locals.var_t9__blk564_dn11 = assign18450_e25999_d_n11;
        locals.var_t9__blk564_dn12 = assign18450_e25999_d_n12;
        locals.var_t9__blk564_dn17 = assign18450_e25999_d_n17;
        locals.var_t9__blk564_rv = 0.0;

        let (assign18460_e26009, assign18460_e26009_d_n0, assign18460_e26009_d_n2, assign18460_e26009_d_n6, assign18460_e26009_d_n7, assign18460_e26009_d_n10, assign18460_e26009_d_n11, assign18460_e26009_d_n12, assign18460_e26009_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18460_e26006: f64 = (locals.var_vdsz * p.p246);
        let assign18460_e26007: f64 = (1.0 + assign18460_e26006);
        (assign18460_e26007, (locals.var_vdsz_dn0 * p.p246), (locals.var_vdsz_dn2 * p.p246), (locals.var_vdsz_dn6 * p.p246), (locals.var_vdsz_dn7 * p.p246), (locals.var_vdsz_dn10 * p.p246), (locals.var_vdsz_dn11 * p.p246), (locals.var_vdsz_dn12 * p.p246), (locals.var_vdsz_dn17 * p.p246),)
    } else {
        (locals.var_t4__blk565, locals.var_t4__blk565_dn0, locals.var_t4__blk565_dn2, locals.var_t4__blk565_dn6, locals.var_t4__blk565_dn7, locals.var_t4__blk565_dn10, locals.var_t4__blk565_dn11, locals.var_t4__blk565_dn12, locals.var_t4__blk565_dn17,)
    }
};
        locals.var_t4__blk565 = assign18460_e26009;
        locals.var_t4__blk565_dn0 = assign18460_e26009_d_n0;
        locals.var_t4__blk565_dn2 = assign18460_e26009_d_n2;
        locals.var_t4__blk565_dn6 = assign18460_e26009_d_n6;
        locals.var_t4__blk565_dn7 = assign18460_e26009_d_n7;
        locals.var_t4__blk565_dn10 = assign18460_e26009_d_n10;
        locals.var_t4__blk565_dn11 = assign18460_e26009_d_n11;
        locals.var_t4__blk565_dn12 = assign18460_e26009_d_n12;
        locals.var_t4__blk565_dn17 = assign18460_e26009_d_n17;
        locals.var_t4__blk565_rv = 0.0;

        let (assign18470_e26015, assign18470_e26015_d_n0, assign18470_e26015_d_n2, assign18470_e26015_d_n6, assign18470_e26015_d_n7, assign18470_e26015_d_n10, assign18470_e26015_d_n11, assign18470_e26015_d_n12, assign18470_e26015_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk562, locals.var_t0__blk562_dn0, locals.var_t0__blk562_dn2, locals.var_t0__blk562_dn6, locals.var_t0__blk562_dn7, locals.var_t0__blk562_dn10, locals.var_t0__blk562_dn11, locals.var_t0__blk562_dn12, locals.var_t0__blk562_dn17,)
    }
};
        locals.var_t0__blk562 = assign18470_e26015;
        locals.var_t0__blk562_dn0 = assign18470_e26015_d_n0;
        locals.var_t0__blk562_dn2 = assign18470_e26015_d_n2;
        locals.var_t0__blk562_dn6 = assign18470_e26015_d_n6;
        locals.var_t0__blk562_dn7 = assign18470_e26015_d_n7;
        locals.var_t0__blk562_dn10 = assign18470_e26015_d_n10;
        locals.var_t0__blk562_dn11 = assign18470_e26015_d_n11;
        locals.var_t0__blk562_dn12 = assign18470_e26015_d_n12;
        locals.var_t0__blk562_dn17 = assign18470_e26015_d_n17;
        locals.var_t0__blk562_rv = 0.0;

        let assign18480_e26022: f64 = if ((locals.var_subversion < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard572 = assign18480_e26022;
        locals.var_guard572_rv = 0.0;

        let (assign18490_e26034, assign18490_e26034_d_n0, assign18490_e26034_d_n2, assign18490_e26034_d_n6, assign18490_e26034_d_n7, assign18490_e26034_d_n10, assign18490_e26034_d_n11, assign18490_e26034_d_n12, assign18490_e26034_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) && (locals.var_guard572 != 0.0)) {
        let assign18490_e26030: f64 = (locals.var_ps0 + locals.var_t6__blk567);
        let assign18490_e26032: f64 = (assign18490_e26030 - locals.var_vbsz);
        (assign18490_e26032, ((locals.var_ps0_dn0 + locals.var_t6__blk567_dn0) - locals.var_vbsz_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk567_dn2) - locals.var_vbsz_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk567_dn6) - locals.var_vbsz_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk567_dn7) - locals.var_vbsz_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk567_dn10) - locals.var_vbsz_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk567_dn11) - locals.var_vbsz_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk567_dn12) - locals.var_vbsz_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk567_dn17) - locals.var_vbsz_dn17),)
    } else {
        (locals.var_t5__blk566, locals.var_t5__blk566_dn0, locals.var_t5__blk566_dn2, locals.var_t5__blk566_dn6, locals.var_t5__blk566_dn7, locals.var_t5__blk566_dn10, locals.var_t5__blk566_dn11, locals.var_t5__blk566_dn12, locals.var_t5__blk566_dn17,)
    }
};
        locals.var_t5__blk566 = assign18490_e26034;
        locals.var_t5__blk566_dn0 = assign18490_e26034_d_n0;
        locals.var_t5__blk566_dn2 = assign18490_e26034_d_n2;
        locals.var_t5__blk566_dn6 = assign18490_e26034_d_n6;
        locals.var_t5__blk566_dn7 = assign18490_e26034_d_n7;
        locals.var_t5__blk566_dn10 = assign18490_e26034_d_n10;
        locals.var_t5__blk566_dn11 = assign18490_e26034_d_n11;
        locals.var_t5__blk566_dn12 = assign18490_e26034_d_n12;
        locals.var_t5__blk566_dn17 = assign18490_e26034_d_n17;
        locals.var_t5__blk566_rv = 0.0;

        let (assign18500_e26047, assign18500_e26047_d_n0, assign18500_e26047_d_n2, assign18500_e26047_d_n6, assign18500_e26047_d_n7, assign18500_e26047_d_n10, assign18500_e26047_d_n11, assign18500_e26047_d_n12, assign18500_e26047_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) && (locals.var_guard572 == 0.0)) {
        let assign18500_e26043: f64 = (locals.var_ps0 + locals.var_t6__blk567);
        let assign18500_e26045: f64 = (assign18500_e26043 - locals.var_phi_b0_soi);
        (assign18500_e26045, ((locals.var_ps0_dn0 + locals.var_t6__blk567_dn0) - locals.var_phi_b0_soi_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk567_dn2) - locals.var_phi_b0_soi_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk567_dn6) - locals.var_phi_b0_soi_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk567_dn7) - locals.var_phi_b0_soi_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk567_dn10) - locals.var_phi_b0_soi_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk567_dn11) - locals.var_phi_b0_soi_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk567_dn12) - locals.var_phi_b0_soi_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk567_dn17) - locals.var_phi_b0_soi_dn17),)
    } else {
        (locals.var_t5__blk566, locals.var_t5__blk566_dn0, locals.var_t5__blk566_dn2, locals.var_t5__blk566_dn6, locals.var_t5__blk566_dn7, locals.var_t5__blk566_dn10, locals.var_t5__blk566_dn11, locals.var_t5__blk566_dn12, locals.var_t5__blk566_dn17,)
    }
};
        locals.var_t5__blk566 = assign18500_e26047;
        locals.var_t5__blk566_dn0 = assign18500_e26047_d_n0;
        locals.var_t5__blk566_dn2 = assign18500_e26047_d_n2;
        locals.var_t5__blk566_dn6 = assign18500_e26047_d_n6;
        locals.var_t5__blk566_dn7 = assign18500_e26047_d_n7;
        locals.var_t5__blk566_dn10 = assign18500_e26047_d_n10;
        locals.var_t5__blk566_dn11 = assign18500_e26047_d_n11;
        locals.var_t5__blk566_dn12 = assign18500_e26047_d_n12;
        locals.var_t5__blk566_dn17 = assign18500_e26047_d_n17;
        locals.var_t5__blk566_rv = 0.0;

        let (assign18510_e26059, assign18510_e26059_d_n0, assign18510_e26059_d_n2, assign18510_e26059_d_n6, assign18510_e26059_d_n7, assign18510_e26059_d_n10, assign18510_e26059_d_n11, assign18510_e26059_d_n12, assign18510_e26059_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18510_e26054: f64 = (locals.var_vdsz * locals.var_t0__blk562);
        let assign18510_e26056: f64 = (assign18510_e26054 * locals.var_t5__blk566);
        let assign18510_e26057: f64 = (locals.var_t4__blk565 + assign18510_e26056);
        (assign18510_e26057, (locals.var_t4__blk565_dn0 + ((((locals.var_vdsz_dn0 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn0)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn0))), (locals.var_t4__blk565_dn2 + ((((locals.var_vdsz_dn2 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn2)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn2))), (locals.var_t4__blk565_dn6 + ((((locals.var_vdsz_dn6 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn6)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn6))), (locals.var_t4__blk565_dn7 + ((((locals.var_vdsz_dn7 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn7)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn7))), (locals.var_t4__blk565_dn10 + ((((locals.var_vdsz_dn10 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn10)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn10))), (locals.var_t4__blk565_dn11 + ((((locals.var_vdsz_dn11 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn11)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn11))), (locals.var_t4__blk565_dn12 + ((((locals.var_vdsz_dn12 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn12)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn12))), (locals.var_t4__blk565_dn17 + ((((locals.var_vdsz_dn17 * locals.var_t0__blk562) + (locals.var_vdsz * locals.var_t0__blk562_dn17)) * locals.var_t5__blk566) + (assign18510_e26054 * locals.var_t5__blk566_dn17))),)
    } else {
        (locals.var_t4__blk565, locals.var_t4__blk565_dn0, locals.var_t4__blk565_dn2, locals.var_t4__blk565_dn6, locals.var_t4__blk565_dn7, locals.var_t4__blk565_dn10, locals.var_t4__blk565_dn11, locals.var_t4__blk565_dn12, locals.var_t4__blk565_dn17,)
    }
};
        locals.var_t4__blk565 = assign18510_e26059;
        locals.var_t4__blk565_dn0 = assign18510_e26059_d_n0;
        locals.var_t4__blk565_dn2 = assign18510_e26059_d_n2;
        locals.var_t4__blk565_dn6 = assign18510_e26059_d_n6;
        locals.var_t4__blk565_dn7 = assign18510_e26059_d_n7;
        locals.var_t4__blk565_dn10 = assign18510_e26059_d_n10;
        locals.var_t4__blk565_dn11 = assign18510_e26059_d_n11;
        locals.var_t4__blk565_dn12 = assign18510_e26059_d_n12;
        locals.var_t4__blk565_dn17 = assign18510_e26059_d_n17;
        locals.var_t4__blk565_rv = 0.0;

        let (assign18520_e26067, assign18520_e26067_d_n0, assign18520_e26067_d_n2, assign18520_e26067_d_n6, assign18520_e26067_d_n7, assign18520_e26067_d_n10, assign18520_e26067_d_n11, assign18520_e26067_d_n12, assign18520_e26067_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        let assign18520_e26065: f64 = (locals.var_t9__blk564 * locals.var_t4__blk565);
        (assign18520_e26065, ((locals.var_t9__blk564_dn0 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn0)), ((locals.var_t9__blk564_dn2 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn2)), ((locals.var_t9__blk564_dn6 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn6)), ((locals.var_t9__blk564_dn7 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn7)), ((locals.var_t9__blk564_dn10 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn10)), ((locals.var_t9__blk564_dn11 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn11)), ((locals.var_t9__blk564_dn12 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn12)), ((locals.var_t9__blk564_dn17 * locals.var_t4__blk565) + (locals.var_t9__blk564 * locals.var_t4__blk565_dn17)),)
    } else {
        (locals.var_t6__blk567, locals.var_t6__blk567_dn0, locals.var_t6__blk567_dn2, locals.var_t6__blk567_dn6, locals.var_t6__blk567_dn7, locals.var_t6__blk567_dn10, locals.var_t6__blk567_dn11, locals.var_t6__blk567_dn12, locals.var_t6__blk567_dn17,)
    }
};
        locals.var_t6__blk567 = assign18520_e26067;
        locals.var_t6__blk567_dn0 = assign18520_e26067_d_n0;
        locals.var_t6__blk567_dn2 = assign18520_e26067_d_n2;
        locals.var_t6__blk567_dn6 = assign18520_e26067_d_n6;
        locals.var_t6__blk567_dn7 = assign18520_e26067_d_n7;
        locals.var_t6__blk567_dn10 = assign18520_e26067_d_n10;
        locals.var_t6__blk567_dn11 = assign18520_e26067_d_n11;
        locals.var_t6__blk567_dn12 = assign18520_e26067_d_n12;
        locals.var_t6__blk567_dn17 = assign18520_e26067_d_n17;
        locals.var_t6__blk567_rv = 0.0;

        let (assign18530_e26073, assign18530_e26073_d_n0, assign18530_e26073_d_n2, assign18530_e26073_d_n6, assign18530_e26073_d_n7, assign18530_e26073_d_n10, assign18530_e26073_d_n11, assign18530_e26073_d_n12, assign18530_e26073_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 != 0.0)) {
        (locals.var_t6__blk567, locals.var_t6__blk567_dn0, locals.var_t6__blk567_dn2, locals.var_t6__blk567_dn6, locals.var_t6__blk567_dn7, locals.var_t6__blk567_dn10, locals.var_t6__blk567_dn11, locals.var_t6__blk567_dn12, locals.var_t6__blk567_dn17,)
    } else {
        (locals.var_t9__blk564, locals.var_t9__blk564_dn0, locals.var_t9__blk564_dn2, locals.var_t9__blk564_dn6, locals.var_t9__blk564_dn7, locals.var_t9__blk564_dn10, locals.var_t9__blk564_dn11, locals.var_t9__blk564_dn12, locals.var_t9__blk564_dn17,)
    }
};
        locals.var_t9__blk564 = assign18530_e26073;
        locals.var_t9__blk564_dn0 = assign18530_e26073_d_n0;
        locals.var_t9__blk564_dn2 = assign18530_e26073_d_n2;
        locals.var_t9__blk564_dn6 = assign18530_e26073_d_n6;
        locals.var_t9__blk564_dn7 = assign18530_e26073_d_n7;
        locals.var_t9__blk564_dn10 = assign18530_e26073_d_n10;
        locals.var_t9__blk564_dn11 = assign18530_e26073_d_n11;
        locals.var_t9__blk564_dn12 = assign18530_e26073_d_n12;
        locals.var_t9__blk564_dn17 = assign18530_e26073_d_n17;
        locals.var_t9__blk564_rv = 0.0;

        let (assign18540_e26080, assign18540_e26080_d_n0, assign18540_e26080_d_n2, assign18540_e26080_d_n6, assign18540_e26080_d_n7, assign18540_e26080_d_n10, assign18540_e26080_d_n11, assign18540_e26080_d_n12, assign18540_e26080_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard570 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9__blk564, locals.var_t9__blk564_dn0, locals.var_t9__blk564_dn2, locals.var_t9__blk564_dn6, locals.var_t9__blk564_dn7, locals.var_t9__blk564_dn10, locals.var_t9__blk564_dn11, locals.var_t9__blk564_dn12, locals.var_t9__blk564_dn17,)
    }
};
        locals.var_t9__blk564 = assign18540_e26080;
        locals.var_t9__blk564_dn0 = assign18540_e26080_d_n0;
        locals.var_t9__blk564_dn2 = assign18540_e26080_d_n2;
        locals.var_t9__blk564_dn6 = assign18540_e26080_d_n6;
        locals.var_t9__blk564_dn7 = assign18540_e26080_d_n7;
        locals.var_t9__blk564_dn10 = assign18540_e26080_d_n10;
        locals.var_t9__blk564_dn11 = assign18540_e26080_d_n11;
        locals.var_t9__blk564_dn12 = assign18540_e26080_d_n12;
        locals.var_t9__blk564_dn17 = assign18540_e26080_d_n17;
        locals.var_t9__blk564_rv = 0.0;

        let assign18550_e26083: f64 = if p.p248 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign18550_e26083;
        locals.var_guard573_rv = 0.0;

        let (assign18560_e26091, assign18560_e26091_d_n0, assign18560_e26091_d_n2, assign18560_e26091_d_n6, assign18560_e26091_d_n7, assign18560_e26091_d_n10, assign18560_e26091_d_n11, assign18560_e26091_d_n12, assign18560_e26091_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard573 != 0.0)) {
        let assign18560_e26089: f64 = (locals.var_beta * locals.var_gdl0);
        (assign18560_e26089, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_gdl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk561, locals.var_t1__blk561_dn0, locals.var_t1__blk561_dn2, locals.var_t1__blk561_dn6, locals.var_t1__blk561_dn7, locals.var_t1__blk561_dn10, locals.var_t1__blk561_dn11, locals.var_t1__blk561_dn12, locals.var_t1__blk561_dn17,)
    }
};
        locals.var_t1__blk561 = assign18560_e26091;
        locals.var_t1__blk561_dn0 = assign18560_e26091_d_n0;
        locals.var_t1__blk561_dn2 = assign18560_e26091_d_n2;
        locals.var_t1__blk561_dn6 = assign18560_e26091_d_n6;
        locals.var_t1__blk561_dn7 = assign18560_e26091_d_n7;
        locals.var_t1__blk561_dn10 = assign18560_e26091_d_n10;
        locals.var_t1__blk561_dn11 = assign18560_e26091_d_n11;
        locals.var_t1__blk561_dn12 = assign18560_e26091_d_n12;
        locals.var_t1__blk561_dn17 = assign18560_e26091_d_n17;
        locals.var_t1__blk561_rv = 0.0;

        let (assign18570_e26099, assign18570_e26099_d_n0, assign18570_e26099_d_n2, assign18570_e26099_d_n6, assign18570_e26099_d_n7, assign18570_e26099_d_n10, assign18570_e26099_d_n11, assign18570_e26099_d_n12, assign18570_e26099_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard573 != 0.0)) {
        let assign18570_e26097: f64 = (locals.var_c_fox * locals.var_t1__blk561);
        (assign18570_e26097, ((locals.var_c_fox_dn0 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn0)), ((locals.var_c_fox_dn2 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn2)), ((locals.var_c_fox_dn6 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn6)), ((locals.var_c_fox_dn7 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn7)), ((locals.var_c_fox_dn10 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn10)), ((locals.var_c_fox_dn11 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn11)), ((locals.var_c_fox_dn12 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn12)), ((locals.var_c_fox_dn17 * locals.var_t1__blk561) + (locals.var_c_fox * locals.var_t1__blk561_dn17)),)
    } else {
        (locals.var_t2__blk569, locals.var_t2__blk569_dn0, locals.var_t2__blk569_dn2, locals.var_t2__blk569_dn6, locals.var_t2__blk569_dn7, locals.var_t2__blk569_dn10, locals.var_t2__blk569_dn11, locals.var_t2__blk569_dn12, locals.var_t2__blk569_dn17,)
    }
};
        locals.var_t2__blk569 = assign18570_e26099;
        locals.var_t2__blk569_dn0 = assign18570_e26099_d_n0;
        locals.var_t2__blk569_dn2 = assign18570_e26099_d_n2;
        locals.var_t2__blk569_dn6 = assign18570_e26099_d_n6;
        locals.var_t2__blk569_dn7 = assign18570_e26099_d_n7;
        locals.var_t2__blk569_dn10 = assign18570_e26099_d_n10;
        locals.var_t2__blk569_dn11 = assign18570_e26099_d_n11;
        locals.var_t2__blk569_dn12 = assign18570_e26099_d_n12;
        locals.var_t2__blk569_dn17 = assign18570_e26099_d_n17;
        locals.var_t2__blk569_rv = 0.0;

        let (assign18580_e26107, assign18580_e26107_d_n0, assign18580_e26107_d_n2, assign18580_e26107_d_n6, assign18580_e26107_d_n7, assign18580_e26107_d_n10, assign18580_e26107_d_n11, assign18580_e26107_d_n12, assign18580_e26107_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard573 != 0.0)) {
        let assign18580_e26105: f64 = (locals.var_t2__blk569 * locals.var_vdsz);
        (assign18580_e26105, ((locals.var_t2__blk569_dn0 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn0)), ((locals.var_t2__blk569_dn2 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn2)), ((locals.var_t2__blk569_dn6 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn6)), ((locals.var_t2__blk569_dn7 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn7)), ((locals.var_t2__blk569_dn10 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn10)), ((locals.var_t2__blk569_dn11 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn11)), ((locals.var_t2__blk569_dn12 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn12)), ((locals.var_t2__blk569_dn17 * locals.var_vdsz) + (locals.var_t2__blk569 * locals.var_vdsz_dn17)),)
    } else {
        (locals.var_t8__blk568, locals.var_t8__blk568_dn0, locals.var_t8__blk568_dn2, locals.var_t8__blk568_dn6, locals.var_t8__blk568_dn7, locals.var_t8__blk568_dn10, locals.var_t8__blk568_dn11, locals.var_t8__blk568_dn12, locals.var_t8__blk568_dn17,)
    }
};
        locals.var_t8__blk568 = assign18580_e26107;
        locals.var_t8__blk568_dn0 = assign18580_e26107_d_n0;
        locals.var_t8__blk568_dn2 = assign18580_e26107_d_n2;
        locals.var_t8__blk568_dn6 = assign18580_e26107_d_n6;
        locals.var_t8__blk568_dn7 = assign18580_e26107_d_n7;
        locals.var_t8__blk568_dn10 = assign18580_e26107_d_n10;
        locals.var_t8__blk568_dn11 = assign18580_e26107_d_n11;
        locals.var_t8__blk568_dn12 = assign18580_e26107_d_n12;
        locals.var_t8__blk568_dn17 = assign18580_e26107_d_n17;
        locals.var_t8__blk568_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18590_e26114, assign18590_e26114_d_n0, assign18590_e26114_d_n2, assign18590_e26114_d_n6, assign18590_e26114_d_n7, assign18590_e26114_d_n10, assign18590_e26114_d_n11, assign18590_e26114_d_n12, assign18590_e26114_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard573 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8__blk568, locals.var_t8__blk568_dn0, locals.var_t8__blk568_dn2, locals.var_t8__blk568_dn6, locals.var_t8__blk568_dn7, locals.var_t8__blk568_dn10, locals.var_t8__blk568_dn11, locals.var_t8__blk568_dn12, locals.var_t8__blk568_dn17,)
    }
};
        locals.var_t8__blk568 = assign18590_e26114;
        locals.var_t8__blk568_dn0 = assign18590_e26114_d_n0;
        locals.var_t8__blk568_dn2 = assign18590_e26114_d_n2;
        locals.var_t8__blk568_dn6 = assign18590_e26114_d_n6;
        locals.var_t8__blk568_dn7 = assign18590_e26114_d_n7;
        locals.var_t8__blk568_dn10 = assign18590_e26114_d_n10;
        locals.var_t8__blk568_dn11 = assign18590_e26114_d_n11;
        locals.var_t8__blk568_dn12 = assign18590_e26114_d_n12;
        locals.var_t8__blk568_dn17 = assign18590_e26114_d_n17;
        locals.var_t8__blk568_rv = 0.0;

        let assign18600_e26117: f64 = (locals.var_t9__blk564 + locals.var_t8__blk568);
        let assign18600_e26119: f64 = if assign18600_e26117 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign18600_e26119;
        locals.var_guard574_rv = 0.0;

        let (assign18610_e26129, assign18610_e26129_d_n0, assign18610_e26129_d_n2, assign18610_e26129_d_n6, assign18610_e26129_d_n7, assign18610_e26129_d_n10, assign18610_e26129_d_n11, assign18610_e26129_d_n12, assign18610_e26129_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard574 != 0.0)) {
        let assign18610_e26126: f64 = (locals.var_t9__blk564 + locals.var_t8__blk568);
        let assign18610_e26127: f64 = (locals.var_pds * assign18610_e26126);
        (assign18610_e26127, ((locals.var_pds_dn0 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn0 + locals.var_t8__blk568_dn0))), ((locals.var_pds_dn2 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn2 + locals.var_t8__blk568_dn2))), ((locals.var_pds_dn6 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn6 + locals.var_t8__blk568_dn6))), ((locals.var_pds_dn7 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn7 + locals.var_t8__blk568_dn7))), ((locals.var_pds_dn10 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn10 + locals.var_t8__blk568_dn10))), ((locals.var_pds_dn11 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn11 + locals.var_t8__blk568_dn11))), ((locals.var_pds_dn12 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn12 + locals.var_t8__blk568_dn12))), ((locals.var_pds_dn17 * assign18610_e26126) + (locals.var_pds * (locals.var_t9__blk564_dn17 + locals.var_t8__blk568_dn17))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn12, locals.var_idd1_dn17,)
    }
};
        locals.var_idd1 = assign18610_e26129;
        locals.var_idd1_dn0 = assign18610_e26129_d_n0;
        locals.var_idd1_dn2 = assign18610_e26129_d_n2;
        locals.var_idd1_dn6 = assign18610_e26129_d_n6;
        locals.var_idd1_dn7 = assign18610_e26129_d_n7;
        locals.var_idd1_dn10 = assign18610_e26129_d_n10;
        locals.var_idd1_dn11 = assign18610_e26129_d_n11;
        locals.var_idd1_dn12 = assign18610_e26129_d_n12;
        locals.var_idd1_dn17 = assign18610_e26129_d_n17;
        locals.var_idd1_rv = 0.0;

        let (assign18620_e26139, assign18620_e26139_d_n0, assign18620_e26139_d_n2, assign18620_e26139_d_n6, assign18620_e26139_d_n7, assign18620_e26139_d_n10, assign18620_e26139_d_n11, assign18620_e26139_d_n12, assign18620_e26139_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard574 != 0.0)) {
        let assign18620_e26135: f64 = (locals.var_betawl * locals.var_idd1);
        let assign18620_e26137: f64 = (assign18620_e26135 * locals.var_mu);
        (assign18620_e26137, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn12)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn17)) * locals.var_mu) + (assign18620_e26135 * locals.var_mu_dn17)),)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18620_e26139;
        locals.var_idspt_dn0 = assign18620_e26139_d_n0;
        locals.var_idspt_dn2 = assign18620_e26139_d_n2;
        locals.var_idspt_dn6 = assign18620_e26139_d_n6;
        locals.var_idspt_dn7 = assign18620_e26139_d_n7;
        locals.var_idspt_dn10 = assign18620_e26139_d_n10;
        locals.var_idspt_dn11 = assign18620_e26139_d_n11;
        locals.var_idspt_dn12 = assign18620_e26139_d_n12;
        locals.var_idspt_dn17 = assign18620_e26139_d_n17;
        locals.var_idspt_rv = 0.0;

        let (assign18630_e26145, assign18630_e26145_d_n0, assign18630_e26145_d_n2, assign18630_e26145_d_n6, assign18630_e26145_d_n7, assign18630_e26145_d_n10, assign18630_e26145_d_n11, assign18630_e26145_d_n12, assign18630_e26145_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        let assign18630_e26143: f64 = (locals.var_ids0 + locals.var_idspt);
        (assign18630_e26143, (locals.var_ids0_dn0 + locals.var_idspt_dn0), (locals.var_ids0_dn2 + locals.var_idspt_dn2), (locals.var_ids0_dn6 + locals.var_idspt_dn6), (locals.var_ids0_dn7 + locals.var_idspt_dn7), (locals.var_ids0_dn10 + locals.var_idspt_dn10), (locals.var_ids0_dn11 + locals.var_idspt_dn11), (locals.var_ids0_dn12 + locals.var_idspt_dn12), (locals.var_ids0_dn17 + locals.var_idspt_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign18630_e26145;
        locals.var_ids_dn0 = assign18630_e26145_d_n0;
        locals.var_ids_dn2 = assign18630_e26145_d_n2;
        locals.var_ids_dn6 = assign18630_e26145_d_n6;
        locals.var_ids_dn7 = assign18630_e26145_d_n7;
        locals.var_ids_dn10 = assign18630_e26145_d_n10;
        locals.var_ids_dn11 = assign18630_e26145_d_n11;
        locals.var_ids_dn12 = assign18630_e26145_d_n12;
        locals.var_ids_dn17 = assign18630_e26145_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign18640_e26149, assign18640_e26149_d_n0, assign18640_e26149_d_n2, assign18640_e26149_d_n6, assign18640_e26149_d_n7, assign18640_e26149_d_n10, assign18640_e26149_d_n11, assign18640_e26149_d_n12, assign18640_e26149_d_n17,) = {
    if (locals.var_guard507 != 0.0) {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn12, locals.var_idspt0_dn17,)
    }
};
        locals.var_idspt0 = assign18640_e26149;
        locals.var_idspt0_dn0 = assign18640_e26149_d_n0;
        locals.var_idspt0_dn2 = assign18640_e26149_d_n2;
        locals.var_idspt0_dn6 = assign18640_e26149_d_n6;
        locals.var_idspt0_dn7 = assign18640_e26149_d_n7;
        locals.var_idspt0_dn10 = assign18640_e26149_d_n10;
        locals.var_idspt0_dn11 = assign18640_e26149_d_n11;
        locals.var_idspt0_dn12 = assign18640_e26149_d_n12;
        locals.var_idspt0_dn17 = assign18640_e26149_d_n17;
        locals.var_idspt0_rv = 0.0;

        let assign18650_e26152: f64 = if p.p33 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign18650_e26152;
        locals.var_guard584_rv = 0.0;

        let (assign18660_e26158, assign18660_e26158_d_n0, assign18660_e26158_d_n2, assign18660_e26158_d_n6, assign18660_e26158_d_n7, assign18660_e26158_d_n10, assign18660_e26158_d_n11, assign18660_e26158_d_n12, assign18660_e26158_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn12, locals.var_wdpl_dn17,)
    } else {
        (locals.var_t2__blk577, locals.var_t2__blk577_dn0, locals.var_t2__blk577_dn2, locals.var_t2__blk577_dn6, locals.var_t2__blk577_dn7, locals.var_t2__blk577_dn10, locals.var_t2__blk577_dn11, locals.var_t2__blk577_dn12, locals.var_t2__blk577_dn17,)
    }
};
        locals.var_t2__blk577 = assign18660_e26158;
        locals.var_t2__blk577_dn0 = assign18660_e26158_d_n0;
        locals.var_t2__blk577_dn2 = assign18660_e26158_d_n2;
        locals.var_t2__blk577_dn6 = assign18660_e26158_d_n6;
        locals.var_t2__blk577_dn7 = assign18660_e26158_d_n7;
        locals.var_t2__blk577_dn10 = assign18660_e26158_d_n10;
        locals.var_t2__blk577_dn11 = assign18660_e26158_d_n11;
        locals.var_t2__blk577_dn12 = assign18660_e26158_d_n12;
        locals.var_t2__blk577_dn17 = assign18660_e26158_d_n17;
        locals.var_t2__blk577_rv = 0.0;

        let (assign18670_e26166, assign18670_e26166_d_n0, assign18670_e26166_d_n2, assign18670_e26166_d_n6, assign18670_e26166_d_n7, assign18670_e26166_d_n10, assign18670_e26166_d_n11, assign18670_e26166_d_n12, assign18670_e26166_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18670_e26164: f64 = (locals.var_lgatesm - p.p71);
        (assign18670_e26164, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk578, locals.var_t3__blk578_dn0, locals.var_t3__blk578_dn2, locals.var_t3__blk578_dn6, locals.var_t3__blk578_dn7, locals.var_t3__blk578_dn10, locals.var_t3__blk578_dn11, locals.var_t3__blk578_dn12, locals.var_t3__blk578_dn17,)
    }
};
        locals.var_t3__blk578 = assign18670_e26166;
        locals.var_t3__blk578_dn0 = assign18670_e26166_d_n0;
        locals.var_t3__blk578_dn2 = assign18670_e26166_d_n2;
        locals.var_t3__blk578_dn6 = assign18670_e26166_d_n6;
        locals.var_t3__blk578_dn7 = assign18670_e26166_d_n7;
        locals.var_t3__blk578_dn10 = assign18670_e26166_d_n10;
        locals.var_t3__blk578_dn11 = assign18670_e26166_d_n11;
        locals.var_t3__blk578_dn12 = assign18670_e26166_d_n12;
        locals.var_t3__blk578_dn17 = assign18670_e26166_d_n17;
        locals.var_t3__blk578_rv = 0.0;

        let (assign18680_e26176, assign18680_e26176_d_n0, assign18680_e26176_d_n2, assign18680_e26176_d_n6, assign18680_e26176_d_n7, assign18680_e26176_d_n10, assign18680_e26176_d_n11, assign18680_e26176_d_n12, assign18680_e26176_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18680_e26173: f64 = (locals.var_t3__blk578 * locals.var_t3__blk578);
        let assign18680_e26174: f64 = (1.0 / assign18680_e26173);
        (assign18680_e26174, (-(((locals.var_t3__blk578_dn0 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn0)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn2 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn2)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn6 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn6)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn7 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn7)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn10 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn10)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn11 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn11)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn12 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn12)) / (assign18680_e26173 * assign18680_e26173))), (-(((locals.var_t3__blk578_dn17 * locals.var_t3__blk578) + (locals.var_t3__blk578 * locals.var_t3__blk578_dn17)) / (assign18680_e26173 * assign18680_e26173))),)
    } else {
        (locals.var_t4__blk579, locals.var_t4__blk579_dn0, locals.var_t4__blk579_dn2, locals.var_t4__blk579_dn6, locals.var_t4__blk579_dn7, locals.var_t4__blk579_dn10, locals.var_t4__blk579_dn11, locals.var_t4__blk579_dn12, locals.var_t4__blk579_dn17,)
    }
};
        locals.var_t4__blk579 = assign18680_e26176;
        locals.var_t4__blk579_dn0 = assign18680_e26176_d_n0;
        locals.var_t4__blk579_dn2 = assign18680_e26176_d_n2;
        locals.var_t4__blk579_dn6 = assign18680_e26176_d_n6;
        locals.var_t4__blk579_dn7 = assign18680_e26176_d_n7;
        locals.var_t4__blk579_dn10 = assign18680_e26176_d_n10;
        locals.var_t4__blk579_dn11 = assign18680_e26176_d_n11;
        locals.var_t4__blk579_dn12 = assign18680_e26176_d_n12;
        locals.var_t4__blk579_dn17 = assign18680_e26176_d_n17;
        locals.var_t4__blk579_rv = 0.0;

        let (assign18690_e26194, assign18690_e26194_d_n0, assign18690_e26194_d_n2, assign18690_e26194_d_n6, assign18690_e26194_d_n7, assign18690_e26194_d_n10, assign18690_e26194_d_n11, assign18690_e26194_d_n12, assign18690_e26194_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18690_e26183: f64 = (p.p69 - locals.var_pb20b);
        let assign18690_e26184: f64 = (2.0 * assign18690_e26183);
        let assign18690_e26187: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        let assign18690_e26188: f64 = (assign18690_e26184 * assign18690_e26187);
        let assign18690_e26190: f64 = (assign18690_e26188 * locals.var_t2__blk577);
        let assign18690_e26192: f64 = (assign18690_e26190 * locals.var_t4__blk579);
        (assign18690_e26192, (((((((2.0 * (-locals.var_pb20b_dn0)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn0))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn0)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn2))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn2)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn2)), (((((((2.0 * (-locals.var_pb20b_dn6)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn6))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn6)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn7))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn7)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn7)), (((((((2.0 * (-locals.var_pb20b_dn10)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn10))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn10)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn11))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn11)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn11)), (((((((2.0 * (-locals.var_pb20b_dn12)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn12))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn12)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn12)), (((((((2.0 * (-locals.var_pb20b_dn17)) * assign18690_e26187) + (assign18690_e26184 * (1.034943e-10 * locals.var_c_fox_inv_dn17))) * locals.var_t2__blk577) + (assign18690_e26188 * locals.var_t2__blk577_dn17)) * locals.var_t4__blk579) + (assign18690_e26190 * locals.var_t4__blk579_dn17)),)
    } else {
        (locals.var_t5__blk580, locals.var_t5__blk580_dn0, locals.var_t5__blk580_dn2, locals.var_t5__blk580_dn6, locals.var_t5__blk580_dn7, locals.var_t5__blk580_dn10, locals.var_t5__blk580_dn11, locals.var_t5__blk580_dn12, locals.var_t5__blk580_dn17,)
    }
};
        locals.var_t5__blk580 = assign18690_e26194;
        locals.var_t5__blk580_dn0 = assign18690_e26194_d_n0;
        locals.var_t5__blk580_dn2 = assign18690_e26194_d_n2;
        locals.var_t5__blk580_dn6 = assign18690_e26194_d_n6;
        locals.var_t5__blk580_dn7 = assign18690_e26194_d_n7;
        locals.var_t5__blk580_dn10 = assign18690_e26194_d_n10;
        locals.var_t5__blk580_dn11 = assign18690_e26194_d_n11;
        locals.var_t5__blk580_dn12 = assign18690_e26194_d_n12;
        locals.var_t5__blk580_dn17 = assign18690_e26194_d_n17;
        locals.var_t5__blk580_rv = 0.0;

        let (assign18700_e26202, assign18700_e26202_d_n0, assign18700_e26202_d_n2, assign18700_e26202_d_n6, assign18700_e26202_d_n7, assign18700_e26202_d_n10, assign18700_e26202_d_n11, assign18700_e26202_d_n12, assign18700_e26202_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18700_e26200: f64 = (locals.var_t5__blk580 * locals.var_sqrt_pbsum);
        (assign18700_e26200, ((locals.var_t5__blk580_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5__blk580_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5__blk580_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5__blk580_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5__blk580_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5__blk580_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5__blk580_dn12 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn12)), ((locals.var_t5__blk580_dn17 * locals.var_sqrt_pbsum) + (locals.var_t5__blk580 * locals.var_sqrt_pbsum_dn17)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn12, locals.var_dvth0_dn17,)
    }
};
        locals.var_dvth0 = assign18700_e26202;
        locals.var_dvth0_dn0 = assign18700_e26202_d_n0;
        locals.var_dvth0_dn2 = assign18700_e26202_d_n2;
        locals.var_dvth0_dn6 = assign18700_e26202_d_n6;
        locals.var_dvth0_dn7 = assign18700_e26202_d_n7;
        locals.var_dvth0_dn10 = assign18700_e26202_d_n10;
        locals.var_dvth0_dn11 = assign18700_e26202_d_n11;
        locals.var_dvth0_dn12 = assign18700_e26202_d_n12;
        locals.var_dvth0_dn17 = assign18700_e26202_d_n17;
        locals.var_dvth0_rv = 0.0;

        let (assign18710_e26212, assign18710_e26212_d_n0, assign18710_e26212_d_n2, assign18710_e26212_d_n6, assign18710_e26212_d_n7, assign18710_e26212_d_n10, assign18710_e26212_d_n11, assign18710_e26212_d_n12, assign18710_e26212_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18710_e26209: f64 = (p.p155 * locals.var_vdsz);
        let assign18710_e26210: f64 = (p.p154 + assign18710_e26209);
        (assign18710_e26210, (p.p155 * locals.var_vdsz_dn0), (p.p155 * locals.var_vdsz_dn2), (p.p155 * locals.var_vdsz_dn6), (p.p155 * locals.var_vdsz_dn7), (p.p155 * locals.var_vdsz_dn10), (p.p155 * locals.var_vdsz_dn11), (p.p155 * locals.var_vdsz_dn12), (p.p155 * locals.var_vdsz_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18710_e26212;
        locals.var_t1w_dn0 = assign18710_e26212_d_n0;
        locals.var_t1w_dn2 = assign18710_e26212_d_n2;
        locals.var_t1w_dn6 = assign18710_e26212_d_n6;
        locals.var_t1w_dn7 = assign18710_e26212_d_n7;
        locals.var_t1w_dn10 = assign18710_e26212_d_n10;
        locals.var_t1w_dn11 = assign18710_e26212_d_n11;
        locals.var_t1w_dn12 = assign18710_e26212_d_n12;
        locals.var_t1w_dn17 = assign18710_e26212_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18720_e26220, assign18720_e26220_d_n0, assign18720_e26220_d_n2, assign18720_e26220_d_n6, assign18720_e26220_d_n7, assign18720_e26220_d_n10, assign18720_e26220_d_n11, assign18720_e26220_d_n12, assign18720_e26220_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18720_e26218: f64 = (locals.var_dvth0 * locals.var_t1w);
        (assign18720_e26218, ((locals.var_dvth0_dn0 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn2)), ((locals.var_dvth0_dn6 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn7)), ((locals.var_dvth0_dn10 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn11)), ((locals.var_dvth0_dn12 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn12)), ((locals.var_dvth0_dn17 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn17)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn12, locals.var_dvthscsti_dn17,)
    }
};
        locals.var_dvthscsti = assign18720_e26220;
        locals.var_dvthscsti_dn0 = assign18720_e26220_d_n0;
        locals.var_dvthscsti_dn2 = assign18720_e26220_d_n2;
        locals.var_dvthscsti_dn6 = assign18720_e26220_d_n6;
        locals.var_dvthscsti_dn7 = assign18720_e26220_d_n7;
        locals.var_dvthscsti_dn10 = assign18720_e26220_d_n10;
        locals.var_dvthscsti_dn11 = assign18720_e26220_d_n11;
        locals.var_dvthscsti_dn12 = assign18720_e26220_d_n12;
        locals.var_dvthscsti_dn17 = assign18720_e26220_d_n17;
        locals.var_dvthscsti_rv = 0.0;

        let (assign18730_e26230, assign18730_e26230_d_n0, assign18730_e26230_d_n2, assign18730_e26230_d_n6, assign18730_e26230_d_n7, assign18730_e26230_d_n10, assign18730_e26230_d_n11, assign18730_e26230_d_n12, assign18730_e26230_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18730_e26227: f64 = (p.p157 * locals.var_vds);
        let assign18730_e26228: f64 = (p.p156 - assign18730_e26227);
        (assign18730_e26228, (-(p.p157 * locals.var_vds_dn0)), (-(p.p157 * locals.var_vds_dn2)), (-(p.p157 * locals.var_vds_dn6)), (-(p.p157 * locals.var_vds_dn7)), (-(p.p157 * locals.var_vds_dn10)), (-(p.p157 * locals.var_vds_dn11)), (-(p.p157 * locals.var_vds_dn12)), (-(p.p157 * locals.var_vds_dn17)),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign18730_e26230;
        locals.var_t1__blk575_dn0 = assign18730_e26230_d_n0;
        locals.var_t1__blk575_dn2 = assign18730_e26230_d_n2;
        locals.var_t1__blk575_dn6 = assign18730_e26230_d_n6;
        locals.var_t1__blk575_dn7 = assign18730_e26230_d_n7;
        locals.var_t1__blk575_dn10 = assign18730_e26230_d_n10;
        locals.var_t1__blk575_dn11 = assign18730_e26230_d_n11;
        locals.var_t1__blk575_dn12 = assign18730_e26230_d_n12;
        locals.var_t1__blk575_dn17 = assign18730_e26230_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let (assign18740_e26242, assign18740_e26242_d_n0, assign18740_e26242_d_n2, assign18740_e26242_d_n6, assign18740_e26242_d_n7, assign18740_e26242_d_n10, assign18740_e26242_d_n11, assign18740_e26242_d_n12, assign18740_e26242_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18740_e26236: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign18740_e26238: f64 = (assign18740_e26236 + locals.var_t1__blk575);
        let assign18740_e26240: f64 = (assign18740_e26238 + locals.var_dvthscsti);
        (assign18740_e26240, ((locals.var_vgsz_dn0 + locals.var_t1__blk575_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz_dn2 + locals.var_t1__blk575_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz_dn6 + locals.var_t1__blk575_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz_dn7 + locals.var_t1__blk575_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz_dn10 + locals.var_t1__blk575_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz_dn11 + locals.var_t1__blk575_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz_dn12 + locals.var_t1__blk575_dn12) + locals.var_dvthscsti_dn12), ((locals.var_vgsz_dn17 + locals.var_t1__blk575_dn17) + locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12, locals.var_vgssti_dn17,)
    }
};
        locals.var_vgssti = assign18740_e26242;
        locals.var_vgssti_dn0 = assign18740_e26242_d_n0;
        locals.var_vgssti_dn2 = assign18740_e26242_d_n2;
        locals.var_vgssti_dn6 = assign18740_e26242_d_n6;
        locals.var_vgssti_dn7 = assign18740_e26242_d_n7;
        locals.var_vgssti_dn10 = assign18740_e26242_d_n10;
        locals.var_vgssti_dn11 = assign18740_e26242_d_n11;
        locals.var_vgssti_dn12 = assign18740_e26242_d_n12;
        locals.var_vgssti_dn17 = assign18740_e26242_d_n17;
        locals.var_vgssti_rv = 0.0;

        let (assign18750_e26252, assign18750_e26252_d_n0, assign18750_e26252_d_n2, assign18750_e26252_d_n6, assign18750_e26252_d_n7, assign18750_e26252_d_n10, assign18750_e26252_d_n11, assign18750_e26252_d_n12, assign18750_e26252_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18750_e26248: f64 = (locals.var_costi0_p2 * locals.var_c_fox_inv);
        let assign18750_e26250: f64 = (assign18750_e26248 * locals.var_c_fox_inv);
        (assign18750_e26250, ((((locals.var_costi0_p2_dn0 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn2)), ((((locals.var_costi0_p2_dn6 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn7)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn7)), ((((locals.var_costi0_p2_dn10 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn11)), ((((locals.var_costi0_p2_dn12 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn12)), ((((locals.var_costi0_p2_dn17 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn17)) * locals.var_c_fox_inv) + (assign18750_e26248 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12, locals.var_costi3_dn17,)
    }
};
        locals.var_costi3 = assign18750_e26252;
        locals.var_costi3_dn0 = assign18750_e26252_d_n0;
        locals.var_costi3_dn2 = assign18750_e26252_d_n2;
        locals.var_costi3_dn6 = assign18750_e26252_d_n6;
        locals.var_costi3_dn7 = assign18750_e26252_d_n7;
        locals.var_costi3_dn10 = assign18750_e26252_d_n10;
        locals.var_costi3_dn11 = assign18750_e26252_d_n11;
        locals.var_costi3_dn12 = assign18750_e26252_d_n12;
        locals.var_costi3_dn17 = assign18750_e26252_d_n17;
        locals.var_costi3_rv = 0.0;

        let (assign18760_e26262, assign18760_e26262_d_n0, assign18760_e26262_d_n2, assign18760_e26262_d_n6, assign18760_e26262_d_n7, assign18760_e26262_d_n10, assign18760_e26262_d_n11, assign18760_e26262_d_n12, assign18760_e26262_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18760_e26258: f64 = (locals.var_costi3 * locals.var_beta);
        let assign18760_e26260: f64 = (assign18760_e26258 * 0.5);
        (assign18760_e26260, ((locals.var_costi3_dn0 * locals.var_beta) * 0.5), ((locals.var_costi3_dn2 * locals.var_beta) * 0.5), ((locals.var_costi3_dn6 * locals.var_beta) * 0.5), ((locals.var_costi3_dn7 * locals.var_beta) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), ((locals.var_costi3_dn11 * locals.var_beta) * 0.5), ((locals.var_costi3_dn12 * locals.var_beta) * 0.5), ((locals.var_costi3_dn17 * locals.var_beta) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12, locals.var_costi4_dn17,)
    }
};
        locals.var_costi4 = assign18760_e26262;
        locals.var_costi4_dn0 = assign18760_e26262_d_n0;
        locals.var_costi4_dn2 = assign18760_e26262_d_n2;
        locals.var_costi4_dn6 = assign18760_e26262_d_n6;
        locals.var_costi4_dn7 = assign18760_e26262_d_n7;
        locals.var_costi4_dn10 = assign18760_e26262_d_n10;
        locals.var_costi4_dn11 = assign18760_e26262_d_n11;
        locals.var_costi4_dn12 = assign18760_e26262_d_n12;
        locals.var_costi4_dn17 = assign18760_e26262_d_n17;
        locals.var_costi4_rv = 0.0;

        let (assign18770_e26272, assign18770_e26272_d_n0, assign18770_e26272_d_n2, assign18770_e26272_d_n6, assign18770_e26272_d_n7, assign18770_e26272_d_n10, assign18770_e26272_d_n11, assign18770_e26272_d_n12, assign18770_e26272_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18770_e26268: f64 = (locals.var_costi4 * locals.var_beta);
        let assign18770_e26270: f64 = (assign18770_e26268 * 2.0);
        (assign18770_e26270, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn7 * locals.var_beta) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0), ((locals.var_costi4_dn17 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12, locals.var_costi5_dn17,)
    }
};
        locals.var_costi5 = assign18770_e26272;
        locals.var_costi5_dn0 = assign18770_e26272_d_n0;
        locals.var_costi5_dn2 = assign18770_e26272_d_n2;
        locals.var_costi5_dn6 = assign18770_e26272_d_n6;
        locals.var_costi5_dn7 = assign18770_e26272_d_n7;
        locals.var_costi5_dn10 = assign18770_e26272_d_n10;
        locals.var_costi5_dn11 = assign18770_e26272_d_n11;
        locals.var_costi5_dn12 = assign18770_e26272_d_n12;
        locals.var_costi5_dn17 = assign18770_e26272_d_n17;
        locals.var_costi5_rv = 0.0;

        let (assign18780_e26292, assign18780_e26292_d_n0, assign18780_e26292_d_n2, assign18780_e26292_d_n6, assign18780_e26292_d_n7, assign18780_e26292_d_n10, assign18780_e26292_d_n11, assign18780_e26292_d_n12, assign18780_e26292_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18780_e26280: f64 = (locals.var_beta * 0.25);
        let assign18780_e26281: f64 = (locals.var_costi3 * assign18780_e26280);
        let assign18780_e26282: f64 = (locals.var_beta_inv - assign18780_e26281);
        let assign18780_e26284: f64 = (assign18780_e26282 + locals.var_vfb);
        let assign18780_e26286: f64 = (assign18780_e26284 - p.p156);
        let assign18780_e26288: f64 = (assign18780_e26286 - locals.var_dvthscsti);
        let assign18780_e26290: f64 = (assign18780_e26288 + 1e-50);
        (assign18780_e26290, ((-(locals.var_costi3_dn0 * assign18780_e26280)) - locals.var_dvthscsti_dn0), ((-(locals.var_costi3_dn2 * assign18780_e26280)) - locals.var_dvthscsti_dn2), ((-(locals.var_costi3_dn6 * assign18780_e26280)) - locals.var_dvthscsti_dn6), ((-(locals.var_costi3_dn7 * assign18780_e26280)) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * assign18780_e26280) + (locals.var_costi3 * (locals.var_beta_dn10 * 0.25)))) - locals.var_dvthscsti_dn10), ((-(locals.var_costi3_dn11 * assign18780_e26280)) - locals.var_dvthscsti_dn11), ((-(locals.var_costi3_dn12 * assign18780_e26280)) - locals.var_dvthscsti_dn12), ((-(locals.var_costi3_dn17 * assign18780_e26280)) - locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_t10__blk581, locals.var_t10__blk581_dn0, locals.var_t10__blk581_dn2, locals.var_t10__blk581_dn6, locals.var_t10__blk581_dn7, locals.var_t10__blk581_dn10, locals.var_t10__blk581_dn11, locals.var_t10__blk581_dn12, locals.var_t10__blk581_dn17,)
    }
};
        locals.var_t10__blk581 = assign18780_e26292;
        locals.var_t10__blk581_dn0 = assign18780_e26292_d_n0;
        locals.var_t10__blk581_dn2 = assign18780_e26292_d_n2;
        locals.var_t10__blk581_dn6 = assign18780_e26292_d_n6;
        locals.var_t10__blk581_dn7 = assign18780_e26292_d_n7;
        locals.var_t10__blk581_dn10 = assign18780_e26292_d_n10;
        locals.var_t10__blk581_dn11 = assign18780_e26292_d_n11;
        locals.var_t10__blk581_dn12 = assign18780_e26292_d_n12;
        locals.var_t10__blk581_dn17 = assign18780_e26292_d_n17;
        locals.var_t10__blk581_rv = 0.0;

        let (assign18790_e26302, assign18790_e26302_d_n0, assign18790_e26302_d_n2, assign18790_e26302_d_n6, assign18790_e26302_d_n7, assign18790_e26302_d_n10, assign18790_e26302_d_n11, assign18790_e26302_d_n12, assign18790_e26302_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18790_e26298: f64 = (locals.var_vgsz - locals.var_t10__blk581);
        let assign18790_e26300: f64 = (assign18790_e26298 - 0.005);
        (assign18790_e26300, (locals.var_vgsz_dn0 - locals.var_t10__blk581_dn0), (locals.var_vgsz_dn2 - locals.var_t10__blk581_dn2), (locals.var_vgsz_dn6 - locals.var_t10__blk581_dn6), (locals.var_vgsz_dn7 - locals.var_t10__blk581_dn7), (locals.var_vgsz_dn10 - locals.var_t10__blk581_dn10), (locals.var_vgsz_dn11 - locals.var_t10__blk581_dn11), (locals.var_vgsz_dn12 - locals.var_t10__blk581_dn12), (locals.var_vgsz_dn17 - locals.var_t10__blk581_dn17),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign18790_e26302;
        locals.var_t1__blk575_dn0 = assign18790_e26302_d_n0;
        locals.var_t1__blk575_dn2 = assign18790_e26302_d_n2;
        locals.var_t1__blk575_dn6 = assign18790_e26302_d_n6;
        locals.var_t1__blk575_dn7 = assign18790_e26302_d_n7;
        locals.var_t1__blk575_dn10 = assign18790_e26302_d_n10;
        locals.var_t1__blk575_dn11 = assign18790_e26302_d_n11;
        locals.var_t1__blk575_dn12 = assign18790_e26302_d_n12;
        locals.var_t1__blk575_dn17 = assign18790_e26302_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let (assign18800_e26314, assign18800_e26314_d_n0, assign18800_e26314_d_n2, assign18800_e26314_d_n6, assign18800_e26314_d_n7, assign18800_e26314_d_n10, assign18800_e26314_d_n11, assign18800_e26314_d_n12, assign18800_e26314_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let (assign18800_e26312,) = {
            if (locals.var_t10__blk581 >= 0.0) {
                (1.0,)
            } else {
                let assign18800_e26311: f64 = (-1.0);
                (assign18800_e26311,)
            }
        };
        (assign18800_e26312, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18800_e26314;
        locals.var_t0_dn0 = assign18800_e26314_d_n0;
        locals.var_t0_dn2 = assign18800_e26314_d_n2;
        locals.var_t0_dn6 = assign18800_e26314_d_n6;
        locals.var_t0_dn7 = assign18800_e26314_d_n7;
        locals.var_t0_dn10 = assign18800_e26314_d_n10;
        locals.var_t0_dn11 = assign18800_e26314_d_n11;
        locals.var_t0_dn12 = assign18800_e26314_d_n12;
        locals.var_t0_dn17 = assign18800_e26314_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18810_e26331, assign18810_e26331_d_n0, assign18810_e26331_d_n2, assign18810_e26331_d_n6, assign18810_e26331_d_n7, assign18810_e26331_d_n10, assign18810_e26331_d_n11, assign18810_e26331_d_n12, assign18810_e26331_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18810_e26320: f64 = (locals.var_t1__blk575 * locals.var_t1__blk575);
        let assign18810_e26323: f64 = (locals.var_t0 * 4.0);
        let assign18810_e26325: f64 = (assign18810_e26323 * locals.var_t10__blk581);
        let assign18810_e26327: f64 = (assign18810_e26325 * 0.005);
        let assign18810_e26328: f64 = (assign18810_e26320 + assign18810_e26327);
        let assign18810_e26329: f64 = (assign18810_e26328).sqrt();
        (assign18810_e26329, ((((locals.var_t1__blk575_dn0 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn0)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn2 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn2)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn6 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn6)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn7 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn7)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn10 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn10)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn11 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn11)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn12 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn12)) + ((((locals.var_t0_dn12 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn12)) * 0.005)) / (2.0 * assign18810_e26329)), ((((locals.var_t1__blk575_dn17 * locals.var_t1__blk575) + (locals.var_t1__blk575 * locals.var_t1__blk575_dn17)) + ((((locals.var_t0_dn17 * 4.0) * locals.var_t10__blk581) + (assign18810_e26323 * locals.var_t10__blk581_dn17)) * 0.005)) / (2.0 * assign18810_e26329)),)
    } else {
        (locals.var_t2__blk577, locals.var_t2__blk577_dn0, locals.var_t2__blk577_dn2, locals.var_t2__blk577_dn6, locals.var_t2__blk577_dn7, locals.var_t2__blk577_dn10, locals.var_t2__blk577_dn11, locals.var_t2__blk577_dn12, locals.var_t2__blk577_dn17,)
    }
};
        locals.var_t2__blk577 = assign18810_e26331;
        locals.var_t2__blk577_dn0 = assign18810_e26331_d_n0;
        locals.var_t2__blk577_dn2 = assign18810_e26331_d_n2;
        locals.var_t2__blk577_dn6 = assign18810_e26331_d_n6;
        locals.var_t2__blk577_dn7 = assign18810_e26331_d_n7;
        locals.var_t2__blk577_dn10 = assign18810_e26331_d_n10;
        locals.var_t2__blk577_dn11 = assign18810_e26331_d_n11;
        locals.var_t2__blk577_dn12 = assign18810_e26331_d_n12;
        locals.var_t2__blk577_dn17 = assign18810_e26331_d_n17;
        locals.var_t2__blk577_rv = 0.0;

        let (assign18820_e26351, assign18820_e26351_d_n0, assign18820_e26351_d_n2, assign18820_e26351_d_n6, assign18820_e26351_d_n7, assign18820_e26351_d_n10, assign18820_e26351_d_n11, assign18820_e26351_d_n12, assign18820_e26351_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18820_e26339: f64 = (locals.var_t1__blk575 + locals.var_t2__blk577);
        let assign18820_e26340: f64 = (0.5 * assign18820_e26339);
        let assign18820_e26341: f64 = (locals.var_t10__blk581 + assign18820_e26340);
        let assign18820_e26343: f64 = (assign18820_e26341 - locals.var_vfb);
        let assign18820_e26345: f64 = (assign18820_e26343 + p.p156);
        let assign18820_e26347: f64 = (assign18820_e26345 + locals.var_dvthscsti);
        let assign18820_e26349: f64 = (assign18820_e26347 - locals.var_vbspz);
        (assign18820_e26349, (((locals.var_t10__blk581_dn0 + (0.5 * (locals.var_t1__blk575_dn0 + locals.var_t2__blk577_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbspz_dn0), (((locals.var_t10__blk581_dn2 + (0.5 * (locals.var_t1__blk575_dn2 + locals.var_t2__blk577_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbspz_dn2), (((locals.var_t10__blk581_dn6 + (0.5 * (locals.var_t1__blk575_dn6 + locals.var_t2__blk577_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbspz_dn6), (((locals.var_t10__blk581_dn7 + (0.5 * (locals.var_t1__blk575_dn7 + locals.var_t2__blk577_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbspz_dn7), (((locals.var_t10__blk581_dn10 + (0.5 * (locals.var_t1__blk575_dn10 + locals.var_t2__blk577_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbspz_dn10), (((locals.var_t10__blk581_dn11 + (0.5 * (locals.var_t1__blk575_dn11 + locals.var_t2__blk577_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbspz_dn11), (((locals.var_t10__blk581_dn12 + (0.5 * (locals.var_t1__blk575_dn12 + locals.var_t2__blk577_dn12))) + locals.var_dvthscsti_dn12) - locals.var_vbspz_dn12), (((locals.var_t10__blk581_dn17 + (0.5 * (locals.var_t1__blk575_dn17 + locals.var_t2__blk577_dn17))) + locals.var_dvthscsti_dn17) - locals.var_vbspz_dn17),)
    } else {
        (locals.var_t3__blk578, locals.var_t3__blk578_dn0, locals.var_t3__blk578_dn2, locals.var_t3__blk578_dn6, locals.var_t3__blk578_dn7, locals.var_t3__blk578_dn10, locals.var_t3__blk578_dn11, locals.var_t3__blk578_dn12, locals.var_t3__blk578_dn17,)
    }
};
        locals.var_t3__blk578 = assign18820_e26351;
        locals.var_t3__blk578_dn0 = assign18820_e26351_d_n0;
        locals.var_t3__blk578_dn2 = assign18820_e26351_d_n2;
        locals.var_t3__blk578_dn6 = assign18820_e26351_d_n6;
        locals.var_t3__blk578_dn7 = assign18820_e26351_d_n7;
        locals.var_t3__blk578_dn10 = assign18820_e26351_d_n10;
        locals.var_t3__blk578_dn11 = assign18820_e26351_d_n11;
        locals.var_t3__blk578_dn12 = assign18820_e26351_d_n12;
        locals.var_t3__blk578_dn17 = assign18820_e26351_d_n17;
        locals.var_t3__blk578_rv = 0.0;

        let (assign18830_e26361, assign18830_e26361_d_n0, assign18830_e26361_d_n2, assign18830_e26361_d_n6, assign18830_e26361_d_n7, assign18830_e26361_d_n10, assign18830_e26361_d_n11, assign18830_e26361_d_n12, assign18830_e26361_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18830_e26357: f64 = (locals.var_beta * locals.var_t3__blk578);
        let assign18830_e26359: f64 = (assign18830_e26357 - 1.0);
        (assign18830_e26359, (locals.var_beta * locals.var_t3__blk578_dn0), (locals.var_beta * locals.var_t3__blk578_dn2), (locals.var_beta * locals.var_t3__blk578_dn6), (locals.var_beta * locals.var_t3__blk578_dn7), ((locals.var_beta_dn10 * locals.var_t3__blk578) + (locals.var_beta * locals.var_t3__blk578_dn10)), (locals.var_beta * locals.var_t3__blk578_dn11), (locals.var_beta * locals.var_t3__blk578_dn12), (locals.var_beta * locals.var_t3__blk578_dn17),)
    } else {
        (locals.var_t4__blk579, locals.var_t4__blk579_dn0, locals.var_t4__blk579_dn2, locals.var_t4__blk579_dn6, locals.var_t4__blk579_dn7, locals.var_t4__blk579_dn10, locals.var_t4__blk579_dn11, locals.var_t4__blk579_dn12, locals.var_t4__blk579_dn17,)
    }
};
        locals.var_t4__blk579 = assign18830_e26361;
        locals.var_t4__blk579_dn0 = assign18830_e26361_d_n0;
        locals.var_t4__blk579_dn2 = assign18830_e26361_d_n2;
        locals.var_t4__blk579_dn6 = assign18830_e26361_d_n6;
        locals.var_t4__blk579_dn7 = assign18830_e26361_d_n7;
        locals.var_t4__blk579_dn10 = assign18830_e26361_d_n10;
        locals.var_t4__blk579_dn11 = assign18830_e26361_d_n11;
        locals.var_t4__blk579_dn12 = assign18830_e26361_d_n12;
        locals.var_t4__blk579_dn17 = assign18830_e26361_d_n17;
        locals.var_t4__blk579_rv = 0.0;

        let (assign18840_e26369, assign18840_e26369_d_n0, assign18840_e26369_d_n2, assign18840_e26369_d_n6, assign18840_e26369_d_n7, assign18840_e26369_d_n10, assign18840_e26369_d_n11, assign18840_e26369_d_n12, assign18840_e26369_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18840_e26367: f64 = (4.0 / locals.var_costi5);
        (assign18840_e26367, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn12) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn17) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5__blk580, locals.var_t5__blk580_dn0, locals.var_t5__blk580_dn2, locals.var_t5__blk580_dn6, locals.var_t5__blk580_dn7, locals.var_t5__blk580_dn10, locals.var_t5__blk580_dn11, locals.var_t5__blk580_dn12, locals.var_t5__blk580_dn17,)
    }
};
        locals.var_t5__blk580 = assign18840_e26369;
        locals.var_t5__blk580_dn0 = assign18840_e26369_d_n0;
        locals.var_t5__blk580_dn2 = assign18840_e26369_d_n2;
        locals.var_t5__blk580_dn6 = assign18840_e26369_d_n6;
        locals.var_t5__blk580_dn7 = assign18840_e26369_d_n7;
        locals.var_t5__blk580_dn10 = assign18840_e26369_d_n10;
        locals.var_t5__blk580_dn11 = assign18840_e26369_d_n11;
        locals.var_t5__blk580_dn12 = assign18840_e26369_d_n12;
        locals.var_t5__blk580_dn17 = assign18840_e26369_d_n17;
        locals.var_t5__blk580_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign18850_e26379, assign18850_e26379_d_n0, assign18850_e26379_d_n2, assign18850_e26379_d_n6, assign18850_e26379_d_n7, assign18850_e26379_d_n10, assign18850_e26379_d_n11, assign18850_e26379_d_n12, assign18850_e26379_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18850_e26376: f64 = (locals.var_t4__blk579 * locals.var_t5__blk580);
        let assign18850_e26377: f64 = (1.0 + assign18850_e26376);
        (assign18850_e26377, ((locals.var_t4__blk579_dn0 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn0)), ((locals.var_t4__blk579_dn2 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn2)), ((locals.var_t4__blk579_dn6 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn6)), ((locals.var_t4__blk579_dn7 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn7)), ((locals.var_t4__blk579_dn10 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn10)), ((locals.var_t4__blk579_dn11 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn11)), ((locals.var_t4__blk579_dn12 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn12)), ((locals.var_t4__blk579_dn17 * locals.var_t5__blk580) + (locals.var_t4__blk579 * locals.var_t5__blk580_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18850_e26379;
        locals.var_t1w_dn0 = assign18850_e26379_d_n0;
        locals.var_t1w_dn2 = assign18850_e26379_d_n2;
        locals.var_t1w_dn6 = assign18850_e26379_d_n6;
        locals.var_t1w_dn7 = assign18850_e26379_d_n7;
        locals.var_t1w_dn10 = assign18850_e26379_d_n10;
        locals.var_t1w_dn11 = assign18850_e26379_d_n11;
        locals.var_t1w_dn12 = assign18850_e26379_d_n12;
        locals.var_t1w_dn17 = assign18850_e26379_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18860_e26394, assign18860_e26394_d_n0, assign18860_e26394_d_n2, assign18860_e26394_d_n6, assign18860_e26394_d_n7, assign18860_e26394_d_n10, assign18860_e26394_d_n11, assign18860_e26394_d_n12, assign18860_e26394_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18860_e26385: f64 = (locals.var_t1w * locals.var_t1w);
        let assign18860_e26388: f64 = (4.0 * 0.01);
        let assign18860_e26390: f64 = (assign18860_e26388 * 0.01);
        let assign18860_e26391: f64 = (assign18860_e26385 + assign18860_e26390);
        let assign18860_e26392: f64 = (assign18860_e26391).sqrt();
        (assign18860_e26392, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign18860_e26392)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign18860_e26392)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18860_e26394;
        locals.var_tmf1_dn0 = assign18860_e26394_d_n0;
        locals.var_tmf1_dn2 = assign18860_e26394_d_n2;
        locals.var_tmf1_dn6 = assign18860_e26394_d_n6;
        locals.var_tmf1_dn7 = assign18860_e26394_d_n7;
        locals.var_tmf1_dn10 = assign18860_e26394_d_n10;
        locals.var_tmf1_dn11 = assign18860_e26394_d_n11;
        locals.var_tmf1_dn12 = assign18860_e26394_d_n12;
        locals.var_tmf1_dn17 = assign18860_e26394_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18870_e26408, assign18870_e26408_d_n0, assign18870_e26408_d_n2, assign18870_e26408_d_n6, assign18870_e26408_d_n7, assign18870_e26408_d_n10, assign18870_e26408_d_n11, assign18870_e26408_d_n12, assign18870_e26408_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18870_e26401: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign18870_e26402: f64 = (0.5 * assign18870_e26401);
        let assign18870_e26405: f64 = (1e-10 * 0.01);
        let assign18870_e26406: f64 = (assign18870_e26402 + assign18870_e26405);
        (assign18870_e26406, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign18870_e26408;
        locals.var_t1__blk575_dn0 = assign18870_e26408_d_n0;
        locals.var_t1__blk575_dn2 = assign18870_e26408_d_n2;
        locals.var_t1__blk575_dn6 = assign18870_e26408_d_n6;
        locals.var_t1__blk575_dn7 = assign18870_e26408_d_n7;
        locals.var_t1__blk575_dn10 = assign18870_e26408_d_n10;
        locals.var_t1__blk575_dn11 = assign18870_e26408_d_n11;
        locals.var_t1__blk575_dn12 = assign18870_e26408_d_n12;
        locals.var_t1__blk575_dn17 = assign18870_e26408_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let assign18880_e26411: f64 = if locals.var_t1__blk575 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign18880_e26411;
        locals.var_guard585_rv = 0.0;

        let (assign18890_e26419, assign18890_e26419_d_n0, assign18890_e26419_d_n2, assign18890_e26419_d_n6, assign18890_e26419_d_n7, assign18890_e26419_d_n10, assign18890_e26419_d_n11, assign18890_e26419_d_n12, assign18890_e26419_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign18890_e26419;
        locals.var_t1__blk575_dn0 = assign18890_e26419_d_n0;
        locals.var_t1__blk575_dn2 = assign18890_e26419_d_n2;
        locals.var_t1__blk575_dn6 = assign18890_e26419_d_n6;
        locals.var_t1__blk575_dn7 = assign18890_e26419_d_n7;
        locals.var_t1__blk575_dn10 = assign18890_e26419_d_n10;
        locals.var_t1__blk575_dn11 = assign18890_e26419_d_n11;
        locals.var_t1__blk575_dn12 = assign18890_e26419_d_n12;
        locals.var_t1__blk575_dn17 = assign18890_e26419_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let (assign18900_e26428, assign18900_e26428_d_n0, assign18900_e26428_d_n2, assign18900_e26428_d_n6, assign18900_e26428_d_n7, assign18900_e26428_d_n10, assign18900_e26428_d_n11, assign18900_e26428_d_n12, assign18900_e26428_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18900_e26425: f64 = (locals.var_t1__blk575 + 1e-50);
        let assign18900_e26426: f64 = (assign18900_e26425).sqrt();
        (assign18900_e26426, (locals.var_t1__blk575_dn0 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn2 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn6 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn7 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn10 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn11 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn12 / (2.0 * assign18900_e26426)), (locals.var_t1__blk575_dn17 / (2.0 * assign18900_e26426)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12, locals.var_costi6_dn17,)
    }
};
        locals.var_costi6 = assign18900_e26428;
        locals.var_costi6_dn0 = assign18900_e26428_d_n0;
        locals.var_costi6_dn2 = assign18900_e26428_d_n2;
        locals.var_costi6_dn6 = assign18900_e26428_d_n6;
        locals.var_costi6_dn7 = assign18900_e26428_d_n7;
        locals.var_costi6_dn10 = assign18900_e26428_d_n10;
        locals.var_costi6_dn11 = assign18900_e26428_d_n11;
        locals.var_costi6_dn12 = assign18900_e26428_d_n12;
        locals.var_costi6_dn17 = assign18900_e26428_d_n17;
        locals.var_costi6_rv = 0.0;

        let (assign18910_e26440, assign18910_e26440_d_n0, assign18910_e26440_d_n2, assign18910_e26440_d_n6, assign18910_e26440_d_n7, assign18910_e26440_d_n10, assign18910_e26440_d_n11, assign18910_e26440_d_n12, assign18910_e26440_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18910_e26436: f64 = (1.0 - locals.var_costi6);
        let assign18910_e26437: f64 = (locals.var_costi4 * assign18910_e26436);
        let assign18910_e26438: f64 = (locals.var_vgssti + assign18910_e26437);
        (assign18910_e26438, (locals.var_vgssti_dn0 + ((locals.var_costi4_dn0 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn0)))), (locals.var_vgssti_dn2 + ((locals.var_costi4_dn2 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn2)))), (locals.var_vgssti_dn6 + ((locals.var_costi4_dn6 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn6)))), (locals.var_vgssti_dn7 + ((locals.var_costi4_dn7 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn7)))), (locals.var_vgssti_dn10 + ((locals.var_costi4_dn10 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn10)))), (locals.var_vgssti_dn11 + ((locals.var_costi4_dn11 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn11)))), (locals.var_vgssti_dn12 + ((locals.var_costi4_dn12 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn12)))), (locals.var_vgssti_dn17 + ((locals.var_costi4_dn17 * assign18910_e26436) + (locals.var_costi4 * (-locals.var_costi6_dn17)))),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12, locals.var_psasti_dn17,)
    }
};
        locals.var_psasti = assign18910_e26440;
        locals.var_psasti_dn0 = assign18910_e26440_d_n0;
        locals.var_psasti_dn2 = assign18910_e26440_d_n2;
        locals.var_psasti_dn6 = assign18910_e26440_d_n6;
        locals.var_psasti_dn7 = assign18910_e26440_d_n7;
        locals.var_psasti_dn10 = assign18910_e26440_d_n10;
        locals.var_psasti_dn11 = assign18910_e26440_d_n11;
        locals.var_psasti_dn12 = assign18910_e26440_d_n12;
        locals.var_psasti_dn17 = assign18910_e26440_d_n17;
        locals.var_psasti_rv = 0.0;

        let (assign18920_e26454, assign18920_e26454_d_n0, assign18920_e26454_d_n2, assign18920_e26454_d_n6, assign18920_e26454_d_n7, assign18920_e26454_d_n10, assign18920_e26454_d_n11, assign18920_e26454_d_n12, assign18920_e26454_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18920_e26449: f64 = (locals.var_vgssti + 1e-50);
        let assign18920_e26450: f64 = (2.0 / assign18920_e26449);
        let assign18920_e26451: f64 = (locals.var_beta + assign18920_e26450);
        let assign18920_e26452: f64 = (1.0 / assign18920_e26451);
        (assign18920_e26452, (-((-((2.0 * locals.var_vgssti_dn0) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))), (-((-((2.0 * locals.var_vgssti_dn2) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))), (-((-((2.0 * locals.var_vgssti_dn6) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))), (-((-((2.0 * locals.var_vgssti_dn7) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign18920_e26449 * assign18920_e26449)))) / (assign18920_e26451 * assign18920_e26451))), (-((-((2.0 * locals.var_vgssti_dn11) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))), (-((-((2.0 * locals.var_vgssti_dn12) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))), (-((-((2.0 * locals.var_vgssti_dn17) / (assign18920_e26449 * assign18920_e26449))) / (assign18920_e26451 * assign18920_e26451))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18920_e26454;
        locals.var_t0_dn0 = assign18920_e26454_d_n0;
        locals.var_t0_dn2 = assign18920_e26454_d_n2;
        locals.var_t0_dn6 = assign18920_e26454_d_n6;
        locals.var_t0_dn7 = assign18920_e26454_d_n7;
        locals.var_t0_dn10 = assign18920_e26454_d_n10;
        locals.var_t0_dn11 = assign18920_e26454_d_n11;
        locals.var_t0_dn12 = assign18920_e26454_d_n12;
        locals.var_t0_dn17 = assign18920_e26454_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18930_e26471, assign18930_e26471_d_n0, assign18930_e26471_d_n2, assign18930_e26471_d_n6, assign18930_e26471_d_n7, assign18930_e26471_d_n10, assign18930_e26471_d_n11, assign18930_e26471_d_n12, assign18930_e26471_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18930_e26460: f64 = (1.0 / locals.var_costi1);
        let assign18930_e26462: f64 = (assign18930_e26460 / locals.var_costi3);
        let assign18930_e26465: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign18930_e26466: f64 = (assign18930_e26462 * assign18930_e26465);
        let assign18930_e26467: f64 = (assign18930_e26466).ln();
        let assign18930_e26469: f64 = (assign18930_e26467 * locals.var_t0);
        (assign18930_e26469, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn12)), (((((((((-(locals.var_costi1_dn17 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18930_e26460 * locals.var_costi3_dn17)) / (locals.var_costi3 * locals.var_costi3)) * assign18930_e26465) + (assign18930_e26462 * ((locals.var_vgssti_dn17 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn17)))) / assign18930_e26466) * locals.var_t0) + (assign18930_e26467 * locals.var_t0_dn17)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12, locals.var_psbsti_dn17,)
    }
};
        locals.var_psbsti = assign18930_e26471;
        locals.var_psbsti_dn0 = assign18930_e26471_d_n0;
        locals.var_psbsti_dn2 = assign18930_e26471_d_n2;
        locals.var_psbsti_dn6 = assign18930_e26471_d_n6;
        locals.var_psbsti_dn7 = assign18930_e26471_d_n7;
        locals.var_psbsti_dn10 = assign18930_e26471_d_n10;
        locals.var_psbsti_dn11 = assign18930_e26471_d_n11;
        locals.var_psbsti_dn12 = assign18930_e26471_d_n12;
        locals.var_psbsti_dn17 = assign18930_e26471_d_n17;
        locals.var_psbsti_rv = 0.0;

        let (assign18940_e26481, assign18940_e26481_d_n0, assign18940_e26481_d_n2, assign18940_e26481_d_n6, assign18940_e26481_d_n7, assign18940_e26481_d_n10, assign18940_e26481_d_n11, assign18940_e26481_d_n12, assign18940_e26481_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18940_e26478: f64 = (locals.var_vgssti + 1e-50);
        let assign18940_e26479: f64 = (locals.var_psbsti / assign18940_e26478);
        (assign18940_e26479, (((locals.var_psbsti_dn0 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn0)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn2 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn2)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn6 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn6)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn7 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn7)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn10 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn10)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn11 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn11)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn12 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn12)) / (assign18940_e26478 * assign18940_e26478)), (((locals.var_psbsti_dn17 * assign18940_e26478) - (locals.var_psbsti * locals.var_vgssti_dn17)) / (assign18940_e26478 * assign18940_e26478)),)
    } else {
        (locals.var_t3__blk578, locals.var_t3__blk578_dn0, locals.var_t3__blk578_dn2, locals.var_t3__blk578_dn6, locals.var_t3__blk578_dn7, locals.var_t3__blk578_dn10, locals.var_t3__blk578_dn11, locals.var_t3__blk578_dn12, locals.var_t3__blk578_dn17,)
    }
};
        locals.var_t3__blk578 = assign18940_e26481;
        locals.var_t3__blk578_dn0 = assign18940_e26481_d_n0;
        locals.var_t3__blk578_dn2 = assign18940_e26481_d_n2;
        locals.var_t3__blk578_dn6 = assign18940_e26481_d_n6;
        locals.var_t3__blk578_dn7 = assign18940_e26481_d_n7;
        locals.var_t3__blk578_dn10 = assign18940_e26481_d_n10;
        locals.var_t3__blk578_dn11 = assign18940_e26481_d_n11;
        locals.var_t3__blk578_dn12 = assign18940_e26481_d_n12;
        locals.var_t3__blk578_dn17 = assign18940_e26481_d_n17;
        locals.var_t3__blk578_rv = 0.0;

        let (assign18950_e26491, assign18950_e26491_d_n0, assign18950_e26491_d_n2, assign18950_e26491_d_n6, assign18950_e26491_d_n7, assign18950_e26491_d_n10, assign18950_e26491_d_n11, assign18950_e26491_d_n12, assign18950_e26491_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18950_e26487: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign18950_e26489: f64 = (assign18950_e26487 - 0.002);
        (assign18950_e26489, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12), (locals.var_psbsti_dn17 - locals.var_psasti_dn17),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12, locals.var_psab_dn17,)
    }
};
        locals.var_psab = assign18950_e26491;
        locals.var_psab_dn0 = assign18950_e26491_d_n0;
        locals.var_psab_dn2 = assign18950_e26491_d_n2;
        locals.var_psab_dn6 = assign18950_e26491_d_n6;
        locals.var_psab_dn7 = assign18950_e26491_d_n7;
        locals.var_psab_dn10 = assign18950_e26491_d_n10;
        locals.var_psab_dn11 = assign18950_e26491_d_n11;
        locals.var_psab_dn12 = assign18950_e26491_d_n12;
        locals.var_psab_dn17 = assign18950_e26491_d_n17;
        locals.var_psab_rv = 0.0;

        let (assign18960_e26506, assign18960_e26506_d_n0, assign18960_e26506_d_n2, assign18960_e26506_d_n6, assign18960_e26506_d_n7, assign18960_e26506_d_n10, assign18960_e26506_d_n11, assign18960_e26506_d_n12, assign18960_e26506_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18960_e26497: f64 = (locals.var_psab * locals.var_psab);
        let assign18960_e26500: f64 = (4.0 * 0.002);
        let assign18960_e26502: f64 = (assign18960_e26500 * locals.var_psbsti);
        let assign18960_e26503: f64 = (assign18960_e26497 + assign18960_e26502);
        let assign18960_e26504: f64 = (assign18960_e26503).sqrt();
        (assign18960_e26504, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign18960_e26500 * locals.var_psbsti_dn0)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign18960_e26500 * locals.var_psbsti_dn2)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign18960_e26500 * locals.var_psbsti_dn6)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign18960_e26500 * locals.var_psbsti_dn7)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign18960_e26500 * locals.var_psbsti_dn10)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign18960_e26500 * locals.var_psbsti_dn11)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign18960_e26500 * locals.var_psbsti_dn12)) / (2.0 * assign18960_e26504)), ((((locals.var_psab_dn17 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn17)) + (assign18960_e26500 * locals.var_psbsti_dn17)) / (2.0 * assign18960_e26504)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18960_e26506;
        locals.var_t0_dn0 = assign18960_e26506_d_n0;
        locals.var_t0_dn2 = assign18960_e26506_d_n2;
        locals.var_t0_dn6 = assign18960_e26506_d_n6;
        locals.var_t0_dn7 = assign18960_e26506_d_n7;
        locals.var_t0_dn10 = assign18960_e26506_d_n10;
        locals.var_t0_dn11 = assign18960_e26506_d_n11;
        locals.var_t0_dn12 = assign18960_e26506_d_n12;
        locals.var_t0_dn17 = assign18960_e26506_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18970_e26518, assign18970_e26518_d_n0, assign18970_e26518_d_n2, assign18970_e26518_d_n6, assign18970_e26518_d_n7, assign18970_e26518_d_n10, assign18970_e26518_d_n11, assign18970_e26518_d_n12, assign18970_e26518_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18970_e26514: f64 = (locals.var_psab + locals.var_t0);
        let assign18970_e26515: f64 = (0.5 * assign18970_e26514);
        let assign18970_e26516: f64 = (locals.var_psbsti - assign18970_e26515);
        (assign18970_e26516, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + locals.var_t0_dn12))), (locals.var_psbsti_dn17 - (0.5 * (locals.var_psab_dn17 + locals.var_t0_dn17))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    }
};
        locals.var_psti = assign18970_e26518;
        locals.var_psti_dn0 = assign18970_e26518_d_n0;
        locals.var_psti_dn2 = assign18970_e26518_d_n2;
        locals.var_psti_dn6 = assign18970_e26518_d_n6;
        locals.var_psti_dn7 = assign18970_e26518_d_n7;
        locals.var_psti_dn10 = assign18970_e26518_d_n10;
        locals.var_psti_dn11 = assign18970_e26518_d_n11;
        locals.var_psti_dn12 = assign18970_e26518_d_n12;
        locals.var_psti_dn17 = assign18970_e26518_d_n17;
        locals.var_psti_rv = 0.0;

        let (assign18980_e26526, assign18980_e26526_d_n0, assign18980_e26526_d_n2, assign18980_e26526_d_n6, assign18980_e26526_d_n7, assign18980_e26526_d_n10, assign18980_e26526_d_n11, assign18980_e26526_d_n12, assign18980_e26526_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18980_e26524: f64 = (1.0 / locals.var_t0);
        (assign18980_e26524, (-(locals.var_t0_dn0 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn2 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn12 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn17 / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign18980_e26526;
        locals.var_t1__blk575_dn0 = assign18980_e26526_d_n0;
        locals.var_t1__blk575_dn2 = assign18980_e26526_d_n2;
        locals.var_t1__blk575_dn6 = assign18980_e26526_d_n6;
        locals.var_t1__blk575_dn7 = assign18980_e26526_d_n7;
        locals.var_t1__blk575_dn10 = assign18980_e26526_d_n10;
        locals.var_t1__blk575_dn11 = assign18980_e26526_d_n11;
        locals.var_t1__blk575_dn12 = assign18980_e26526_d_n12;
        locals.var_t1__blk575_dn17 = assign18980_e26526_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let (assign18990_e26537, assign18990_e26537_d_n0, assign18990_e26537_d_n2, assign18990_e26537_d_n6, assign18990_e26537_d_n7, assign18990_e26537_d_n10, assign18990_e26537_d_n11, assign18990_e26537_d_n12, assign18990_e26537_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign18990_e26533: f64 = (locals.var_beta * locals.var_psti);
        let assign18990_e26534: f64 = (assign18990_e26533).exp();
        let assign18990_e26535: f64 = (locals.var_costi1 * assign18990_e26534);
        (assign18990_e26535, ((locals.var_costi1_dn0 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn0)))), ((locals.var_costi1_dn2 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn2)))), ((locals.var_costi1_dn6 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn6)))), ((locals.var_costi1_dn7 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn7)))), ((locals.var_costi1_dn10 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn11 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn11)))), ((locals.var_costi1_dn12 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn12)))), ((locals.var_costi1_dn17 * assign18990_e26534) + (locals.var_costi1 * (assign18990_e26534 * (locals.var_beta * locals.var_psti_dn17)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18990_e26537;
        locals.var_t0_dn0 = assign18990_e26537_d_n0;
        locals.var_t0_dn2 = assign18990_e26537_d_n2;
        locals.var_t0_dn6 = assign18990_e26537_d_n6;
        locals.var_t0_dn7 = assign18990_e26537_d_n7;
        locals.var_t0_dn10 = assign18990_e26537_d_n10;
        locals.var_t0_dn11 = assign18990_e26537_d_n11;
        locals.var_t0_dn12 = assign18990_e26537_d_n12;
        locals.var_t0_dn17 = assign18990_e26537_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign19000_e26551, assign19000_e26551_d_n0, assign19000_e26551_d_n2, assign19000_e26551_d_n6, assign19000_e26551_d_n7, assign19000_e26551_d_n10, assign19000_e26551_d_n11, assign19000_e26551_d_n12, assign19000_e26551_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19000_e26544: f64 = (locals.var_psti - locals.var_vbspz);
        let assign19000_e26545: f64 = (locals.var_beta * assign19000_e26544);
        let assign19000_e26547: f64 = (assign19000_e26545 - 1.0);
        let assign19000_e26549: f64 = (assign19000_e26547 + locals.var_t0);
        (assign19000_e26549, ((locals.var_beta * (locals.var_psti_dn0 - locals.var_vbspz_dn0)) + locals.var_t0_dn0), ((locals.var_beta * (locals.var_psti_dn2 - locals.var_vbspz_dn2)) + locals.var_t0_dn2), ((locals.var_beta * (locals.var_psti_dn6 - locals.var_vbspz_dn6)) + locals.var_t0_dn6), ((locals.var_beta * (locals.var_psti_dn7 - locals.var_vbspz_dn7)) + locals.var_t0_dn7), (((locals.var_beta_dn10 * assign19000_e26544) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbspz_dn10))) + locals.var_t0_dn10), ((locals.var_beta * (locals.var_psti_dn11 - locals.var_vbspz_dn11)) + locals.var_t0_dn11), ((locals.var_beta * (locals.var_psti_dn12 - locals.var_vbspz_dn12)) + locals.var_t0_dn12), ((locals.var_beta * (locals.var_psti_dn17 - locals.var_vbspz_dn17)) + locals.var_t0_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19000_e26551;
        locals.var_t1w_dn0 = assign19000_e26551_d_n0;
        locals.var_t1w_dn2 = assign19000_e26551_d_n2;
        locals.var_t1w_dn6 = assign19000_e26551_d_n6;
        locals.var_t1w_dn7 = assign19000_e26551_d_n7;
        locals.var_t1w_dn10 = assign19000_e26551_d_n10;
        locals.var_t1w_dn11 = assign19000_e26551_d_n11;
        locals.var_t1w_dn12 = assign19000_e26551_d_n12;
        locals.var_t1w_dn17 = assign19000_e26551_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19010_e26566, assign19010_e26566_d_n0, assign19010_e26566_d_n2, assign19010_e26566_d_n6, assign19010_e26566_d_n7, assign19010_e26566_d_n10, assign19010_e26566_d_n11, assign19010_e26566_d_n12, assign19010_e26566_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19010_e26557: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19010_e26560: f64 = (4.0 * 0.01);
        let assign19010_e26562: f64 = (assign19010_e26560 * 0.01);
        let assign19010_e26563: f64 = (assign19010_e26557 + assign19010_e26562);
        let assign19010_e26564: f64 = (assign19010_e26563).sqrt();
        (assign19010_e26564, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19010_e26564)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19010_e26564)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19010_e26566;
        locals.var_tmf1_dn0 = assign19010_e26566_d_n0;
        locals.var_tmf1_dn2 = assign19010_e26566_d_n2;
        locals.var_tmf1_dn6 = assign19010_e26566_d_n6;
        locals.var_tmf1_dn7 = assign19010_e26566_d_n7;
        locals.var_tmf1_dn10 = assign19010_e26566_d_n10;
        locals.var_tmf1_dn11 = assign19010_e26566_d_n11;
        locals.var_tmf1_dn12 = assign19010_e26566_d_n12;
        locals.var_tmf1_dn17 = assign19010_e26566_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19020_e26580, assign19020_e26580_d_n0, assign19020_e26580_d_n2, assign19020_e26580_d_n6, assign19020_e26580_d_n7, assign19020_e26580_d_n10, assign19020_e26580_d_n11, assign19020_e26580_d_n12, assign19020_e26580_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19020_e26573: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19020_e26574: f64 = (0.5 * assign19020_e26573);
        let assign19020_e26577: f64 = (1e-10 * 0.01);
        let assign19020_e26578: f64 = (assign19020_e26574 + assign19020_e26577);
        (assign19020_e26578, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign19020_e26580;
        locals.var_t1__blk575_dn0 = assign19020_e26580_d_n0;
        locals.var_t1__blk575_dn2 = assign19020_e26580_d_n2;
        locals.var_t1__blk575_dn6 = assign19020_e26580_d_n6;
        locals.var_t1__blk575_dn7 = assign19020_e26580_d_n7;
        locals.var_t1__blk575_dn10 = assign19020_e26580_d_n10;
        locals.var_t1__blk575_dn11 = assign19020_e26580_d_n11;
        locals.var_t1__blk575_dn12 = assign19020_e26580_d_n12;
        locals.var_t1__blk575_dn17 = assign19020_e26580_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let assign19030_e26583: f64 = if locals.var_t1__blk575 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign19030_e26583;
        locals.var_guard586_rv = 0.0;

        let (assign19040_e26591, assign19040_e26591_d_n0, assign19040_e26591_d_n2, assign19040_e26591_d_n6, assign19040_e26591_d_n7, assign19040_e26591_d_n10, assign19040_e26591_d_n11, assign19040_e26591_d_n12, assign19040_e26591_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard586 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign19040_e26591;
        locals.var_t1__blk575_dn0 = assign19040_e26591_d_n0;
        locals.var_t1__blk575_dn2 = assign19040_e26591_d_n2;
        locals.var_t1__blk575_dn6 = assign19040_e26591_d_n6;
        locals.var_t1__blk575_dn7 = assign19040_e26591_d_n7;
        locals.var_t1__blk575_dn10 = assign19040_e26591_d_n10;
        locals.var_t1__blk575_dn11 = assign19040_e26591_d_n11;
        locals.var_t1__blk575_dn12 = assign19040_e26591_d_n12;
        locals.var_t1__blk575_dn17 = assign19040_e26591_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let (assign19050_e26602, assign19050_e26602_d_n0, assign19050_e26602_d_n2, assign19050_e26602_d_n6, assign19050_e26602_d_n7, assign19050_e26602_d_n10, assign19050_e26602_d_n11, assign19050_e26602_d_n12, assign19050_e26602_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19050_e26598: f64 = (10.0 * 2.220446049250313e-16);
        let assign19050_e26599: f64 = (locals.var_t1__blk575 + assign19050_e26598);
        let assign19050_e26600: f64 = (assign19050_e26599).sqrt();
        (assign19050_e26600, (locals.var_t1__blk575_dn0 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn2 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn6 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn7 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn10 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn11 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn12 / (2.0 * assign19050_e26600)), (locals.var_t1__blk575_dn17 / (2.0 * assign19050_e26600)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign19050_e26602;
        locals.var_sq1sti_dn0 = assign19050_e26602_d_n0;
        locals.var_sq1sti_dn2 = assign19050_e26602_d_n2;
        locals.var_sq1sti_dn6 = assign19050_e26602_d_n6;
        locals.var_sq1sti_dn7 = assign19050_e26602_d_n7;
        locals.var_sq1sti_dn10 = assign19050_e26602_d_n10;
        locals.var_sq1sti_dn11 = assign19050_e26602_d_n11;
        locals.var_sq1sti_dn12 = assign19050_e26602_d_n12;
        locals.var_sq1sti_dn17 = assign19050_e26602_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign19060_e26614, assign19060_e26614_d_n0, assign19060_e26614_d_n2, assign19060_e26614_d_n6, assign19060_e26614_d_n7, assign19060_e26614_d_n10, assign19060_e26614_d_n11, assign19060_e26614_d_n12, assign19060_e26614_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19060_e26609: f64 = (locals.var_psti - locals.var_vbspz);
        let assign19060_e26610: f64 = (locals.var_beta * assign19060_e26609);
        let assign19060_e26612: f64 = (assign19060_e26610 - 1.0);
        (assign19060_e26612, (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbspz_dn0)), (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbspz_dn2)), (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbspz_dn6)), (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbspz_dn7)), ((locals.var_beta_dn10 * assign19060_e26609) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbspz_dn10))), (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbspz_dn11)), (locals.var_beta * (locals.var_psti_dn12 - locals.var_vbspz_dn12)), (locals.var_beta * (locals.var_psti_dn17 - locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19060_e26614;
        locals.var_t1w_dn0 = assign19060_e26614_d_n0;
        locals.var_t1w_dn2 = assign19060_e26614_d_n2;
        locals.var_t1w_dn6 = assign19060_e26614_d_n6;
        locals.var_t1w_dn7 = assign19060_e26614_d_n7;
        locals.var_t1w_dn10 = assign19060_e26614_d_n10;
        locals.var_t1w_dn11 = assign19060_e26614_d_n11;
        locals.var_t1w_dn12 = assign19060_e26614_d_n12;
        locals.var_t1w_dn17 = assign19060_e26614_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19070_e26629, assign19070_e26629_d_n0, assign19070_e26629_d_n2, assign19070_e26629_d_n6, assign19070_e26629_d_n7, assign19070_e26629_d_n10, assign19070_e26629_d_n11, assign19070_e26629_d_n12, assign19070_e26629_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19070_e26620: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19070_e26623: f64 = (4.0 * 0.01);
        let assign19070_e26625: f64 = (assign19070_e26623 * 0.01);
        let assign19070_e26626: f64 = (assign19070_e26620 + assign19070_e26625);
        let assign19070_e26627: f64 = (assign19070_e26626).sqrt();
        (assign19070_e26627, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19070_e26627)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19070_e26627)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19070_e26629;
        locals.var_tmf1_dn0 = assign19070_e26629_d_n0;
        locals.var_tmf1_dn2 = assign19070_e26629_d_n2;
        locals.var_tmf1_dn6 = assign19070_e26629_d_n6;
        locals.var_tmf1_dn7 = assign19070_e26629_d_n7;
        locals.var_tmf1_dn10 = assign19070_e26629_d_n10;
        locals.var_tmf1_dn11 = assign19070_e26629_d_n11;
        locals.var_tmf1_dn12 = assign19070_e26629_d_n12;
        locals.var_tmf1_dn17 = assign19070_e26629_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19080_e26643, assign19080_e26643_d_n0, assign19080_e26643_d_n2, assign19080_e26643_d_n6, assign19080_e26643_d_n7, assign19080_e26643_d_n10, assign19080_e26643_d_n11, assign19080_e26643_d_n12, assign19080_e26643_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19080_e26636: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19080_e26637: f64 = (0.5 * assign19080_e26636);
        let assign19080_e26640: f64 = (1e-10 * 0.01);
        let assign19080_e26641: f64 = (assign19080_e26637 + assign19080_e26640);
        (assign19080_e26641, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign19080_e26643;
        locals.var_t1__blk575_dn0 = assign19080_e26643_d_n0;
        locals.var_t1__blk575_dn2 = assign19080_e26643_d_n2;
        locals.var_t1__blk575_dn6 = assign19080_e26643_d_n6;
        locals.var_t1__blk575_dn7 = assign19080_e26643_d_n7;
        locals.var_t1__blk575_dn10 = assign19080_e26643_d_n10;
        locals.var_t1__blk575_dn11 = assign19080_e26643_d_n11;
        locals.var_t1__blk575_dn12 = assign19080_e26643_d_n12;
        locals.var_t1__blk575_dn17 = assign19080_e26643_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let assign19090_e26646: f64 = if locals.var_t1__blk575 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign19090_e26646;
        locals.var_guard587_rv = 0.0;

        let (assign19100_e26654, assign19100_e26654_d_n0, assign19100_e26654_d_n2, assign19100_e26654_d_n6, assign19100_e26654_d_n7, assign19100_e26654_d_n10, assign19100_e26654_d_n11, assign19100_e26654_d_n12, assign19100_e26654_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard587 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign19100_e26654;
        locals.var_t1__blk575_dn0 = assign19100_e26654_d_n0;
        locals.var_t1__blk575_dn2 = assign19100_e26654_d_n2;
        locals.var_t1__blk575_dn6 = assign19100_e26654_d_n6;
        locals.var_t1__blk575_dn7 = assign19100_e26654_d_n7;
        locals.var_t1__blk575_dn10 = assign19100_e26654_d_n10;
        locals.var_t1__blk575_dn11 = assign19100_e26654_d_n11;
        locals.var_t1__blk575_dn12 = assign19100_e26654_d_n12;
        locals.var_t1__blk575_dn17 = assign19100_e26654_d_n17;
        locals.var_t1__blk575_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign19110_e26665, assign19110_e26665_d_n0, assign19110_e26665_d_n2, assign19110_e26665_d_n6, assign19110_e26665_d_n7, assign19110_e26665_d_n10, assign19110_e26665_d_n11, assign19110_e26665_d_n12, assign19110_e26665_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19110_e26661: f64 = (10.0 * 2.220446049250313e-16);
        let assign19110_e26662: f64 = (locals.var_t1__blk575 + assign19110_e26661);
        let assign19110_e26663: f64 = (assign19110_e26662).sqrt();
        (assign19110_e26663, (locals.var_t1__blk575_dn0 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn2 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn6 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn7 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn10 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn11 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn12 / (2.0 * assign19110_e26663)), (locals.var_t1__blk575_dn17 / (2.0 * assign19110_e26663)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign19110_e26665;
        locals.var_sq2sti_dn0 = assign19110_e26665_d_n0;
        locals.var_sq2sti_dn2 = assign19110_e26665_d_n2;
        locals.var_sq2sti_dn6 = assign19110_e26665_d_n6;
        locals.var_sq2sti_dn7 = assign19110_e26665_d_n7;
        locals.var_sq2sti_dn10 = assign19110_e26665_d_n10;
        locals.var_sq2sti_dn11 = assign19110_e26665_d_n11;
        locals.var_sq2sti_dn12 = assign19110_e26665_d_n12;
        locals.var_sq2sti_dn17 = assign19110_e26665_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let (assign19120_e26675, assign19120_e26675_d_n0, assign19120_e26675_d_n2, assign19120_e26675_d_n6, assign19120_e26675_d_n7, assign19120_e26675_d_n10, assign19120_e26675_d_n11, assign19120_e26675_d_n12, assign19120_e26675_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19120_e26672: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign19120_e26673: f64 = (locals.var_costi0 * assign19120_e26672);
        (assign19120_e26673, ((locals.var_costi0_dn0 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn6 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn10 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn12 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn12 - locals.var_sq2sti_dn12))), ((locals.var_costi0_dn17 * assign19120_e26672) + (locals.var_costi0 * (locals.var_sq1sti_dn17 - locals.var_sq2sti_dn17))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn12, locals.var_qn0sti_dn17,)
    }
};
        locals.var_qn0sti = assign19120_e26675;
        locals.var_qn0sti_dn0 = assign19120_e26675_d_n0;
        locals.var_qn0sti_dn2 = assign19120_e26675_d_n2;
        locals.var_qn0sti_dn6 = assign19120_e26675_d_n6;
        locals.var_qn0sti_dn7 = assign19120_e26675_d_n7;
        locals.var_qn0sti_dn10 = assign19120_e26675_d_n10;
        locals.var_qn0sti_dn11 = assign19120_e26675_d_n11;
        locals.var_qn0sti_dn12 = assign19120_e26675_d_n12;
        locals.var_qn0sti_dn17 = assign19120_e26675_d_n17;
        locals.var_qn0sti_rv = 0.0;

        let (assign19130_e26683, assign19130_e26683_d_n0, assign19130_e26683_d_n2, assign19130_e26683_d_n6, assign19130_e26683_d_n7, assign19130_e26683_d_n10, assign19130_e26683_d_n11, assign19130_e26683_d_n12, assign19130_e26683_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19130_e26681: f64 = (locals.var_psasti - locals.var_psti);
        (assign19130_e26681, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn7 - locals.var_psti_dn7), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn11 - locals.var_psti_dn11), (locals.var_psasti_dn12 - locals.var_psti_dn12), (locals.var_psasti_dn17 - locals.var_psti_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19130_e26683;
        locals.var_t1w_dn0 = assign19130_e26683_d_n0;
        locals.var_t1w_dn2 = assign19130_e26683_d_n2;
        locals.var_t1w_dn6 = assign19130_e26683_d_n6;
        locals.var_t1w_dn7 = assign19130_e26683_d_n7;
        locals.var_t1w_dn10 = assign19130_e26683_d_n10;
        locals.var_t1w_dn11 = assign19130_e26683_d_n11;
        locals.var_t1w_dn12 = assign19130_e26683_d_n12;
        locals.var_t1w_dn17 = assign19130_e26683_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19140_e26698, assign19140_e26698_d_n0, assign19140_e26698_d_n2, assign19140_e26698_d_n6, assign19140_e26698_d_n7, assign19140_e26698_d_n10, assign19140_e26698_d_n11, assign19140_e26698_d_n12, assign19140_e26698_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19140_e26689: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19140_e26692: f64 = (4.0 * 0.1);
        let assign19140_e26694: f64 = (assign19140_e26692 * 0.1);
        let assign19140_e26695: f64 = (assign19140_e26689 + assign19140_e26694);
        let assign19140_e26696: f64 = (assign19140_e26695).sqrt();
        (assign19140_e26696, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19140_e26696)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19140_e26696)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19140_e26698;
        locals.var_tmf1_dn0 = assign19140_e26698_d_n0;
        locals.var_tmf1_dn2 = assign19140_e26698_d_n2;
        locals.var_tmf1_dn6 = assign19140_e26698_d_n6;
        locals.var_tmf1_dn7 = assign19140_e26698_d_n7;
        locals.var_tmf1_dn10 = assign19140_e26698_d_n10;
        locals.var_tmf1_dn11 = assign19140_e26698_d_n11;
        locals.var_tmf1_dn12 = assign19140_e26698_d_n12;
        locals.var_tmf1_dn17 = assign19140_e26698_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19150_e26712, assign19150_e26712_d_n0, assign19150_e26712_d_n2, assign19150_e26712_d_n6, assign19150_e26712_d_n7, assign19150_e26712_d_n10, assign19150_e26712_d_n11, assign19150_e26712_d_n12, assign19150_e26712_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19150_e26705: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19150_e26706: f64 = (0.5 * assign19150_e26705);
        let assign19150_e26709: f64 = (1e-10 * 0.1);
        let assign19150_e26710: f64 = (assign19150_e26706 + assign19150_e26709);
        (assign19150_e26710, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign19150_e26712;
        locals.var_t1__blk575_dn0 = assign19150_e26712_d_n0;
        locals.var_t1__blk575_dn2 = assign19150_e26712_d_n2;
        locals.var_t1__blk575_dn6 = assign19150_e26712_d_n6;
        locals.var_t1__blk575_dn7 = assign19150_e26712_d_n7;
        locals.var_t1__blk575_dn10 = assign19150_e26712_d_n10;
        locals.var_t1__blk575_dn11 = assign19150_e26712_d_n11;
        locals.var_t1__blk575_dn12 = assign19150_e26712_d_n12;
        locals.var_t1__blk575_dn17 = assign19150_e26712_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let assign19160_e26715: f64 = if locals.var_t1__blk575 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign19160_e26715;
        locals.var_guard588_rv = 0.0;

        let (assign19170_e26723, assign19170_e26723_d_n0, assign19170_e26723_d_n2, assign19170_e26723_d_n6, assign19170_e26723_d_n7, assign19170_e26723_d_n10, assign19170_e26723_d_n11, assign19170_e26723_d_n12, assign19170_e26723_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard588 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk575, locals.var_t1__blk575_dn0, locals.var_t1__blk575_dn2, locals.var_t1__blk575_dn6, locals.var_t1__blk575_dn7, locals.var_t1__blk575_dn10, locals.var_t1__blk575_dn11, locals.var_t1__blk575_dn12, locals.var_t1__blk575_dn17,)
    }
};
        locals.var_t1__blk575 = assign19170_e26723;
        locals.var_t1__blk575_dn0 = assign19170_e26723_d_n0;
        locals.var_t1__blk575_dn2 = assign19170_e26723_d_n2;
        locals.var_t1__blk575_dn6 = assign19170_e26723_d_n6;
        locals.var_t1__blk575_dn7 = assign19170_e26723_d_n7;
        locals.var_t1__blk575_dn10 = assign19170_e26723_d_n10;
        locals.var_t1__blk575_dn11 = assign19170_e26723_d_n11;
        locals.var_t1__blk575_dn12 = assign19170_e26723_d_n12;
        locals.var_t1__blk575_dn17 = assign19170_e26723_d_n17;
        locals.var_t1__blk575_rv = 0.0;

        let (assign19180_e26735, assign19180_e26735_d_n0, assign19180_e26735_d_n2, assign19180_e26735_d_n6, assign19180_e26735_d_n7, assign19180_e26735_d_n10, assign19180_e26735_d_n11, assign19180_e26735_d_n12, assign19180_e26735_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19180_e26731: f64 = (10.0 * 2.220446049250313e-16);
        let assign19180_e26732: f64 = (locals.var_t1__blk575 + assign19180_e26731);
        let assign19180_e26733: f64 = (locals.var_vds / assign19180_e26732);
        (assign19180_e26733, (((locals.var_vds_dn0 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn0)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn2 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn2)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn6 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn6)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn7 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn7)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn10 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn10)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn11 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn11)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn12 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn12)) / (assign19180_e26732 * assign19180_e26732)), (((locals.var_vds_dn17 * assign19180_e26732) - (locals.var_vds * locals.var_t1__blk575_dn17)) / (assign19180_e26732 * assign19180_e26732)),)
    } else {
        (locals.var_tx__blk582, locals.var_tx__blk582_dn0, locals.var_tx__blk582_dn2, locals.var_tx__blk582_dn6, locals.var_tx__blk582_dn7, locals.var_tx__blk582_dn10, locals.var_tx__blk582_dn11, locals.var_tx__blk582_dn12, locals.var_tx__blk582_dn17,)
    }
};
        locals.var_tx__blk582 = assign19180_e26735;
        locals.var_tx__blk582_dn0 = assign19180_e26735_d_n0;
        locals.var_tx__blk582_dn2 = assign19180_e26735_d_n2;
        locals.var_tx__blk582_dn6 = assign19180_e26735_d_n6;
        locals.var_tx__blk582_dn7 = assign19180_e26735_d_n7;
        locals.var_tx__blk582_dn10 = assign19180_e26735_d_n10;
        locals.var_tx__blk582_dn11 = assign19180_e26735_d_n11;
        locals.var_tx__blk582_dn12 = assign19180_e26735_d_n12;
        locals.var_tx__blk582_dn17 = assign19180_e26735_d_n17;
        locals.var_tx__blk582_rv = 0.0;

        let (assign19190_e26743, assign19190_e26743_d_n0, assign19190_e26743_d_n2, assign19190_e26743_d_n6, assign19190_e26743_d_n7, assign19190_e26743_d_n10, assign19190_e26743_d_n11, assign19190_e26743_d_n12, assign19190_e26743_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19190_e26741: f64 = (locals.var_tx__blk582 * locals.var_tx__blk582);
        (assign19190_e26741, ((locals.var_tx__blk582_dn0 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn0)), ((locals.var_tx__blk582_dn2 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn2)), ((locals.var_tx__blk582_dn6 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn6)), ((locals.var_tx__blk582_dn7 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn7)), ((locals.var_tx__blk582_dn10 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn10)), ((locals.var_tx__blk582_dn11 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn11)), ((locals.var_tx__blk582_dn12 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn12)), ((locals.var_tx__blk582_dn17 * locals.var_tx__blk582) + (locals.var_tx__blk582 * locals.var_tx__blk582_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign19190_e26743;
        locals.var_x2_dn0 = assign19190_e26743_d_n0;
        locals.var_x2_dn2 = assign19190_e26743_d_n2;
        locals.var_x2_dn6 = assign19190_e26743_d_n6;
        locals.var_x2_dn7 = assign19190_e26743_d_n7;
        locals.var_x2_dn10 = assign19190_e26743_d_n10;
        locals.var_x2_dn11 = assign19190_e26743_d_n11;
        locals.var_x2_dn12 = assign19190_e26743_d_n12;
        locals.var_x2_dn17 = assign19190_e26743_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign19200_e26751, assign19200_e26751_d_n0, assign19200_e26751_d_n2, assign19200_e26751_d_n6, assign19200_e26751_d_n7, assign19200_e26751_d_n10, assign19200_e26751_d_n11, assign19200_e26751_d_n12, assign19200_e26751_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19200_e26749: f64 = 1.0;
        (assign19200_e26749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign19200_e26751;
        locals.var_xmax2_dn0 = assign19200_e26751_d_n0;
        locals.var_xmax2_dn2 = assign19200_e26751_d_n2;
        locals.var_xmax2_dn6 = assign19200_e26751_d_n6;
        locals.var_xmax2_dn7 = assign19200_e26751_d_n7;
        locals.var_xmax2_dn10 = assign19200_e26751_d_n10;
        locals.var_xmax2_dn11 = assign19200_e26751_d_n11;
        locals.var_xmax2_dn12 = assign19200_e26751_d_n12;
        locals.var_xmax2_dn17 = assign19200_e26751_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign19210_e26757, assign19210_e26757_d_n0, assign19210_e26757_d_n2, assign19210_e26757_d_n6, assign19210_e26757_d_n7, assign19210_e26757_d_n10, assign19210_e26757_d_n11, assign19210_e26757_d_n12, assign19210_e26757_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19210_e26757;
        locals.var_xp_dn0 = assign19210_e26757_d_n0;
        locals.var_xp_dn2 = assign19210_e26757_d_n2;
        locals.var_xp_dn6 = assign19210_e26757_d_n6;
        locals.var_xp_dn7 = assign19210_e26757_d_n7;
        locals.var_xp_dn10 = assign19210_e26757_d_n10;
        locals.var_xp_dn11 = assign19210_e26757_d_n11;
        locals.var_xp_dn12 = assign19210_e26757_d_n12;
        locals.var_xp_dn17 = assign19210_e26757_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19220_e26763, assign19220_e26763_d_n0, assign19220_e26763_d_n2, assign19220_e26763_d_n6, assign19220_e26763_d_n7, assign19220_e26763_d_n10, assign19220_e26763_d_n11, assign19220_e26763_d_n12, assign19220_e26763_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19220_e26763;
        locals.var_xmp_dn0 = assign19220_e26763_d_n0;
        locals.var_xmp_dn2 = assign19220_e26763_d_n2;
        locals.var_xmp_dn6 = assign19220_e26763_d_n6;
        locals.var_xmp_dn7 = assign19220_e26763_d_n7;
        locals.var_xmp_dn10 = assign19220_e26763_d_n10;
        locals.var_xmp_dn11 = assign19220_e26763_d_n11;
        locals.var_xmp_dn12 = assign19220_e26763_d_n12;
        locals.var_xmp_dn17 = assign19220_e26763_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19230_e26769,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign19230_e26769;
        locals.var_m0_rv = 0.0;

        let (assign19240_e26775,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19240_e26775;
        locals.var_mm_rv = 0.0;

        let (assign19250_e26781, assign19250_e26781_d_n0, assign19250_e26781_d_n2, assign19250_e26781_d_n6, assign19250_e26781_d_n7, assign19250_e26781_d_n10, assign19250_e26781_d_n11, assign19250_e26781_d_n12, assign19250_e26781_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign19250_e26781;
        locals.var_arg_dn0 = assign19250_e26781_d_n0;
        locals.var_arg_dn2 = assign19250_e26781_d_n2;
        locals.var_arg_dn6 = assign19250_e26781_d_n6;
        locals.var_arg_dn7 = assign19250_e26781_d_n7;
        locals.var_arg_dn10 = assign19250_e26781_d_n10;
        locals.var_arg_dn11 = assign19250_e26781_d_n11;
        locals.var_arg_dn12 = assign19250_e26781_d_n12;
        locals.var_arg_dn17 = assign19250_e26781_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign19260_e26787, assign19260_e26787_d_n0, assign19260_e26787_d_n2, assign19260_e26787_d_n6, assign19260_e26787_d_n7, assign19260_e26787_d_n10, assign19260_e26787_d_n11, assign19260_e26787_d_n12, assign19260_e26787_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19260_e26787;
        locals.var_dnm_dn0 = assign19260_e26787_d_n0;
        locals.var_dnm_dn2 = assign19260_e26787_d_n2;
        locals.var_dnm_dn6 = assign19260_e26787_d_n6;
        locals.var_dnm_dn7 = assign19260_e26787_d_n7;
        locals.var_dnm_dn10 = assign19260_e26787_d_n10;
        locals.var_dnm_dn11 = assign19260_e26787_d_n11;
        locals.var_dnm_dn12 = assign19260_e26787_d_n12;
        locals.var_dnm_dn17 = assign19260_e26787_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19270_e26795, assign19270_e26795_d_n0, assign19270_e26795_d_n2, assign19270_e26795_d_n6, assign19270_e26795_d_n7, assign19270_e26795_d_n10, assign19270_e26795_d_n11, assign19270_e26795_d_n12, assign19270_e26795_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19270_e26793: f64 = (locals.var_xp * locals.var_x2);
        (assign19270_e26793, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19270_e26795;
        locals.var_xp_dn0 = assign19270_e26795_d_n0;
        locals.var_xp_dn2 = assign19270_e26795_d_n2;
        locals.var_xp_dn6 = assign19270_e26795_d_n6;
        locals.var_xp_dn7 = assign19270_e26795_d_n7;
        locals.var_xp_dn10 = assign19270_e26795_d_n10;
        locals.var_xp_dn11 = assign19270_e26795_d_n11;
        locals.var_xp_dn12 = assign19270_e26795_d_n12;
        locals.var_xp_dn17 = assign19270_e26795_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19280_e26803, assign19280_e26803_d_n0, assign19280_e26803_d_n2, assign19280_e26803_d_n6, assign19280_e26803_d_n7, assign19280_e26803_d_n10, assign19280_e26803_d_n11, assign19280_e26803_d_n12, assign19280_e26803_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19280_e26801: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19280_e26801, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19280_e26803;
        locals.var_xmp_dn0 = assign19280_e26803_d_n0;
        locals.var_xmp_dn2 = assign19280_e26803_d_n2;
        locals.var_xmp_dn6 = assign19280_e26803_d_n6;
        locals.var_xmp_dn7 = assign19280_e26803_d_n7;
        locals.var_xmp_dn10 = assign19280_e26803_d_n10;
        locals.var_xmp_dn11 = assign19280_e26803_d_n11;
        locals.var_xmp_dn12 = assign19280_e26803_d_n12;
        locals.var_xmp_dn17 = assign19280_e26803_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19290_e26811, assign19290_e26811_d_n0, assign19290_e26811_d_n2, assign19290_e26811_d_n6, assign19290_e26811_d_n7, assign19290_e26811_d_n10, assign19290_e26811_d_n11, assign19290_e26811_d_n12, assign19290_e26811_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19290_e26809: f64 = (locals.var_xp * locals.var_x2);
        (assign19290_e26809, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19290_e26811;
        locals.var_xp_dn0 = assign19290_e26811_d_n0;
        locals.var_xp_dn2 = assign19290_e26811_d_n2;
        locals.var_xp_dn6 = assign19290_e26811_d_n6;
        locals.var_xp_dn7 = assign19290_e26811_d_n7;
        locals.var_xp_dn10 = assign19290_e26811_d_n10;
        locals.var_xp_dn11 = assign19290_e26811_d_n11;
        locals.var_xp_dn12 = assign19290_e26811_d_n12;
        locals.var_xp_dn17 = assign19290_e26811_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19300_e26819, assign19300_e26819_d_n0, assign19300_e26819_d_n2, assign19300_e26819_d_n6, assign19300_e26819_d_n7, assign19300_e26819_d_n10, assign19300_e26819_d_n11, assign19300_e26819_d_n12, assign19300_e26819_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19300_e26817: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19300_e26817, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19300_e26819;
        locals.var_xmp_dn0 = assign19300_e26819_d_n0;
        locals.var_xmp_dn2 = assign19300_e26819_d_n2;
        locals.var_xmp_dn6 = assign19300_e26819_d_n6;
        locals.var_xmp_dn7 = assign19300_e26819_d_n7;
        locals.var_xmp_dn10 = assign19300_e26819_d_n10;
        locals.var_xmp_dn11 = assign19300_e26819_d_n11;
        locals.var_xmp_dn12 = assign19300_e26819_d_n12;
        locals.var_xmp_dn17 = assign19300_e26819_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19310_e26827, assign19310_e26827_d_n0, assign19310_e26827_d_n2, assign19310_e26827_d_n6, assign19310_e26827_d_n7, assign19310_e26827_d_n10, assign19310_e26827_d_n11, assign19310_e26827_d_n12, assign19310_e26827_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19310_e26825: f64 = (locals.var_xp * locals.var_x2);
        (assign19310_e26825, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19310_e26827;
        locals.var_xp_dn0 = assign19310_e26827_d_n0;
        locals.var_xp_dn2 = assign19310_e26827_d_n2;
        locals.var_xp_dn6 = assign19310_e26827_d_n6;
        locals.var_xp_dn7 = assign19310_e26827_d_n7;
        locals.var_xp_dn10 = assign19310_e26827_d_n10;
        locals.var_xp_dn11 = assign19310_e26827_d_n11;
        locals.var_xp_dn12 = assign19310_e26827_d_n12;
        locals.var_xp_dn17 = assign19310_e26827_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19320_e26835, assign19320_e26835_d_n0, assign19320_e26835_d_n2, assign19320_e26835_d_n6, assign19320_e26835_d_n7, assign19320_e26835_d_n10, assign19320_e26835_d_n11, assign19320_e26835_d_n12, assign19320_e26835_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19320_e26833: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19320_e26833, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19320_e26835;
        locals.var_xmp_dn0 = assign19320_e26835_d_n0;
        locals.var_xmp_dn2 = assign19320_e26835_d_n2;
        locals.var_xmp_dn6 = assign19320_e26835_d_n6;
        locals.var_xmp_dn7 = assign19320_e26835_d_n7;
        locals.var_xmp_dn10 = assign19320_e26835_d_n10;
        locals.var_xmp_dn11 = assign19320_e26835_d_n11;
        locals.var_xmp_dn12 = assign19320_e26835_d_n12;
        locals.var_xmp_dn17 = assign19320_e26835_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19330_e26843, assign19330_e26843_d_n0, assign19330_e26843_d_n2, assign19330_e26843_d_n6, assign19330_e26843_d_n7, assign19330_e26843_d_n10, assign19330_e26843_d_n11, assign19330_e26843_d_n12, assign19330_e26843_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19330_e26841: f64 = (locals.var_xp * locals.var_x2);
        (assign19330_e26841, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19330_e26843;
        locals.var_xp_dn0 = assign19330_e26843_d_n0;
        locals.var_xp_dn2 = assign19330_e26843_d_n2;
        locals.var_xp_dn6 = assign19330_e26843_d_n6;
        locals.var_xp_dn7 = assign19330_e26843_d_n7;
        locals.var_xp_dn10 = assign19330_e26843_d_n10;
        locals.var_xp_dn11 = assign19330_e26843_d_n11;
        locals.var_xp_dn12 = assign19330_e26843_d_n12;
        locals.var_xp_dn17 = assign19330_e26843_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19340_e26851, assign19340_e26851_d_n0, assign19340_e26851_d_n2, assign19340_e26851_d_n6, assign19340_e26851_d_n7, assign19340_e26851_d_n10, assign19340_e26851_d_n11, assign19340_e26851_d_n12, assign19340_e26851_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19340_e26849: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19340_e26849, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19340_e26851;
        locals.var_xmp_dn0 = assign19340_e26851_d_n0;
        locals.var_xmp_dn2 = assign19340_e26851_d_n2;
        locals.var_xmp_dn6 = assign19340_e26851_d_n6;
        locals.var_xmp_dn7 = assign19340_e26851_d_n7;
        locals.var_xmp_dn10 = assign19340_e26851_d_n10;
        locals.var_xmp_dn11 = assign19340_e26851_d_n11;
        locals.var_xmp_dn12 = assign19340_e26851_d_n12;
        locals.var_xmp_dn17 = assign19340_e26851_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19350_e26859, assign19350_e26859_d_n0, assign19350_e26859_d_n2, assign19350_e26859_d_n6, assign19350_e26859_d_n7, assign19350_e26859_d_n10, assign19350_e26859_d_n11, assign19350_e26859_d_n12, assign19350_e26859_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19350_e26857: f64 = (locals.var_xp + locals.var_xmp);
        (assign19350_e26857, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign19350_e26859;
        locals.var_arg_dn0 = assign19350_e26859_d_n0;
        locals.var_arg_dn2 = assign19350_e26859_d_n2;
        locals.var_arg_dn6 = assign19350_e26859_d_n6;
        locals.var_arg_dn7 = assign19350_e26859_d_n7;
        locals.var_arg_dn10 = assign19350_e26859_d_n10;
        locals.var_arg_dn11 = assign19350_e26859_d_n11;
        locals.var_arg_dn12 = assign19350_e26859_d_n12;
        locals.var_arg_dn17 = assign19350_e26859_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign19360_e26865, assign19360_e26865_d_n0, assign19360_e26865_d_n2, assign19360_e26865_d_n6, assign19360_e26865_d_n7, assign19360_e26865_d_n10, assign19360_e26865_d_n11, assign19360_e26865_d_n12, assign19360_e26865_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19360_e26865;
        locals.var_dnm_dn0 = assign19360_e26865_d_n0;
        locals.var_dnm_dn2 = assign19360_e26865_d_n2;
        locals.var_dnm_dn6 = assign19360_e26865_d_n6;
        locals.var_dnm_dn7 = assign19360_e26865_d_n7;
        locals.var_dnm_dn10 = assign19360_e26865_d_n10;
        locals.var_dnm_dn11 = assign19360_e26865_d_n11;
        locals.var_dnm_dn12 = assign19360_e26865_d_n12;
        locals.var_dnm_dn17 = assign19360_e26865_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign19370_e26880: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard589 = assign19370_e26880;
        locals.var_guard589_rv = 0.0;

        let assign19380_e26883: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign19380_e26883;
        locals.var_guard590_rv = 0.0;

        let (assign19390_e26893,) = {
    if ((((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) && (locals.var_guard590 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19390_e26893;
        locals.var_mm_rv = 0.0;

        let assign19400_e26896: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign19400_e26896;
        locals.var_guard591_rv = 0.0;

        let (assign19410_e26909,) = {
    if (((((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19410_e26909;
        locals.var_mm_rv = 0.0;

        let assign19420_e26912: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign19420_e26912;
        locals.var_guard592_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19430_e26928,) = {
    if ((((((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19430_e26928;
        locals.var_mm_rv = 0.0;

        let assign19440_e26931: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign19440_e26931;
        locals.var_guard593_rv = 0.0;

        let (assign19450_e26950,) = {
    if (((((((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 == 0.0)) && (locals.var_guard593 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19450_e26950;
        locals.var_mm_rv = 0.0;

        let (assign19460_e26958,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign19460_e26958;
        locals.var_m0_rv = 0.0;

        let mut assign19470_loop_guard: usize = 0;
        while {
            let assign19470_cond_e26967: f64 = if ((((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign19470_cond_e26967 != 0.0
        } {
            assign19470_loop_guard += 1;
            assert!(assign19470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign19470_body0_e26976, assign19470_body0_e26976_d_n0, assign19470_body0_e26976_d_n2, assign19470_body0_e26976_d_n6, assign19470_body0_e26976_d_n7, assign19470_body0_e26976_d_n10, assign19470_body0_e26976_d_n11, assign19470_body0_e26976_d_n12, assign19470_body0_e26976_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) {
        let assign19470_body0_e26974: f64 = (locals.var_dnm).sqrt();
        (assign19470_body0_e26974, (locals.var_dnm_dn0 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn2 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn6 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn7 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn10 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn11 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn12 / (2.0 * assign19470_body0_e26974)), (locals.var_dnm_dn17 / (2.0 * assign19470_body0_e26974)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign19470_body0_e26976;
            locals.var_dnm_dn0 = assign19470_body0_e26976_d_n0;
            locals.var_dnm_dn2 = assign19470_body0_e26976_d_n2;
            locals.var_dnm_dn6 = assign19470_body0_e26976_d_n6;
            locals.var_dnm_dn7 = assign19470_body0_e26976_d_n7;
            locals.var_dnm_dn10 = assign19470_body0_e26976_d_n10;
            locals.var_dnm_dn11 = assign19470_body0_e26976_d_n11;
            locals.var_dnm_dn12 = assign19470_body0_e26976_d_n12;
            locals.var_dnm_dn17 = assign19470_body0_e26976_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign19470_body1_e26986,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 != 0.0)) {
        let assign19470_body1_e26984: f64 = (locals.var_m0 + 1.0);
        (assign19470_body1_e26984,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign19470_body1_e26986;
            locals.var_m0_rv = 0.0;
        }

        let (assign19480_e27001, assign19480_e27001_d_n0, assign19480_e27001_d_n2, assign19480_e27001_d_n6, assign19480_e27001_d_n7, assign19480_e27001_d_n10, assign19480_e27001_d_n11, assign19480_e27001_d_n12, assign19480_e27001_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard589 == 0.0)) {
        let assign19480_e26997: f64 = (2.0 * 4.0);
        let assign19480_e26998: f64 = (1.0 / assign19480_e26997);
        let assign19480_e26999: f64 = (locals.var_dnm).powf(assign19480_e26998);
        (assign19480_e26999, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn0)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn2)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn6)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn7)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn10)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn11)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn12)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19480_e26998) as f64).is_finite() && ((assign19480_e26998) as f64).fract() == 0.0 { if assign19480_e26998 == 0.0 { 0.0 } else { (assign19480_e26998 * ((locals.var_dnm).powf(assign19480_e26998 - 1.0) * locals.var_dnm_dn17)) } } else { (assign19480_e26999 * (assign19480_e26998 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19480_e27001;
        locals.var_dnm_dn0 = assign19480_e27001_d_n0;
        locals.var_dnm_dn2 = assign19480_e27001_d_n2;
        locals.var_dnm_dn6 = assign19480_e27001_d_n6;
        locals.var_dnm_dn7 = assign19480_e27001_d_n7;
        locals.var_dnm_dn10 = assign19480_e27001_d_n10;
        locals.var_dnm_dn11 = assign19480_e27001_d_n11;
        locals.var_dnm_dn12 = assign19480_e27001_d_n12;
        locals.var_dnm_dn17 = assign19480_e27001_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19490_e27009, assign19490_e27009_d_n0, assign19490_e27009_d_n2, assign19490_e27009_d_n6, assign19490_e27009_d_n7, assign19490_e27009_d_n10, assign19490_e27009_d_n11, assign19490_e27009_d_n12, assign19490_e27009_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19490_e27007: f64 = (1.0 / locals.var_dnm);
        (assign19490_e27007, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19490_e27009;
        locals.var_dnm_dn0 = assign19490_e27009_d_n0;
        locals.var_dnm_dn2 = assign19490_e27009_d_n2;
        locals.var_dnm_dn6 = assign19490_e27009_d_n6;
        locals.var_dnm_dn7 = assign19490_e27009_d_n7;
        locals.var_dnm_dn10 = assign19490_e27009_d_n10;
        locals.var_dnm_dn11 = assign19490_e27009_d_n11;
        locals.var_dnm_dn12 = assign19490_e27009_d_n12;
        locals.var_dnm_dn17 = assign19490_e27009_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19500_e27019, assign19500_e27019_d_n0, assign19500_e27019_d_n2, assign19500_e27019_d_n6, assign19500_e27019_d_n7, assign19500_e27019_d_n10, assign19500_e27019_d_n11, assign19500_e27019_d_n12, assign19500_e27019_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19500_e27015: f64 = locals.var_tx__blk582;
        let assign19500_e27017: f64 = (assign19500_e27015 * locals.var_dnm);
        (assign19500_e27017, ((locals.var_tx__blk582_dn0 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn0)), ((locals.var_tx__blk582_dn2 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn2)), ((locals.var_tx__blk582_dn6 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn6)), ((locals.var_tx__blk582_dn7 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn7)), ((locals.var_tx__blk582_dn10 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn10)), ((locals.var_tx__blk582_dn11 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn11)), ((locals.var_tx__blk582_dn12 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn12)), ((locals.var_tx__blk582_dn17 * locals.var_dnm) + (assign19500_e27015 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty__blk583, locals.var_ty__blk583_dn0, locals.var_ty__blk583_dn2, locals.var_ty__blk583_dn6, locals.var_ty__blk583_dn7, locals.var_ty__blk583_dn10, locals.var_ty__blk583_dn11, locals.var_ty__blk583_dn12, locals.var_ty__blk583_dn17,)
    }
};
        locals.var_ty__blk583 = assign19500_e27019;
        locals.var_ty__blk583_dn0 = assign19500_e27019_d_n0;
        locals.var_ty__blk583_dn2 = assign19500_e27019_d_n2;
        locals.var_ty__blk583_dn6 = assign19500_e27019_d_n6;
        locals.var_ty__blk583_dn7 = assign19500_e27019_d_n7;
        locals.var_ty__blk583_dn10 = assign19500_e27019_d_n10;
        locals.var_ty__blk583_dn11 = assign19500_e27019_d_n11;
        locals.var_ty__blk583_dn12 = assign19500_e27019_d_n12;
        locals.var_ty__blk583_dn17 = assign19500_e27019_d_n17;
        locals.var_ty__blk583_rv = 0.0;

        let (assign19510_e27031, assign19510_e27031_d_n10,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19510_e27025: f64 = (2.0 * locals.var_uc_wsti);
        let assign19510_e27027: f64 = (assign19510_e27025 * p.p9);
        let assign19510_e27029: f64 = (assign19510_e27027 * locals.var_beta_inv);
        (assign19510_e27029, (assign19510_e27027 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn10,)
    }
};
        locals.var_costi7 = assign19510_e27031;
        locals.var_costi7_dn10 = assign19510_e27031_d_n10;
        locals.var_costi7_rv = 0.0;

        let (assign19520_e27045, assign19520_e27045_d_n0, assign19520_e27045_d_n2, assign19520_e27045_d_n6, assign19520_e27045_d_n7, assign19520_e27045_d_n10, assign19520_e27045_d_n11, assign19520_e27045_d_n12, assign19520_e27045_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19520_e27037: f64 = (locals.var_costi7 * locals.var_mu);
        let assign19520_e27039: f64 = (assign19520_e27037 * locals.var_qn0sti);
        let assign19520_e27041: f64 = (assign19520_e27039 * locals.var_ty__blk583);
        let assign19520_e27043: f64 = (assign19520_e27041 / locals.var_lch);
        (assign19520_e27043, ((((((((locals.var_costi7 * locals.var_mu_dn0) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn0)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn0)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn2) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn2)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn2)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn6) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn6)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn6)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn7) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn7)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn7)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn10)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn10)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn11) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn11)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn11)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn12) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn12)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn12)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn17) * locals.var_qn0sti) + (assign19520_e27037 * locals.var_qn0sti_dn17)) * locals.var_ty__blk583) + (assign19520_e27039 * locals.var_ty__blk583_dn17)) * locals.var_lch) - (assign19520_e27041 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    }
};
        locals.var_idssti = assign19520_e27045;
        locals.var_idssti_dn0 = assign19520_e27045_d_n0;
        locals.var_idssti_dn2 = assign19520_e27045_d_n2;
        locals.var_idssti_dn6 = assign19520_e27045_d_n6;
        locals.var_idssti_dn7 = assign19520_e27045_d_n7;
        locals.var_idssti_dn10 = assign19520_e27045_d_n10;
        locals.var_idssti_dn11 = assign19520_e27045_d_n11;
        locals.var_idssti_dn12 = assign19520_e27045_d_n12;
        locals.var_idssti_dn17 = assign19520_e27045_d_n17;
        locals.var_idssti_rv = 0.0;

        let (assign19530_e27053, assign19530_e27053_d_n0, assign19530_e27053_d_n2, assign19530_e27053_d_n6, assign19530_e27053_d_n7, assign19530_e27053_d_n10, assign19530_e27053_d_n11, assign19530_e27053_d_n12, assign19530_e27053_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign19530_e27051: f64 = (locals.var_ids + locals.var_idssti);
        (assign19530_e27051, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn11 + locals.var_idssti_dn11), (locals.var_ids_dn12 + locals.var_idssti_dn12), (locals.var_ids_dn17 + locals.var_idssti_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign19530_e27053;
        locals.var_ids_dn0 = assign19530_e27053_d_n0;
        locals.var_ids_dn2 = assign19530_e27053_d_n2;
        locals.var_ids_dn6 = assign19530_e27053_d_n6;
        locals.var_ids_dn7 = assign19530_e27053_d_n7;
        locals.var_ids_dn10 = assign19530_e27053_d_n10;
        locals.var_ids_dn11 = assign19530_e27053_d_n11;
        locals.var_ids_dn12 = assign19530_e27053_d_n12;
        locals.var_ids_dn17 = assign19530_e27053_d_n17;
        locals.var_ids_rv = 0.0;

        let assign19540_e27060: f64 = if ((p.p30 != 0.0) && (p.p32 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard594 = assign19540_e27060;
        locals.var_guard594_rv = 0.0;

        let (assign19550_e27068, assign19550_e27068_d_n0, assign19550_e27068_d_n2, assign19550_e27068_d_n6, assign19550_e27068_d_n7, assign19550_e27068_d_n10, assign19550_e27068_d_n11, assign19550_e27068_d_n12, assign19550_e27068_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19550_e27066: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign19550_e27066, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn11 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn11)), ((locals.var_vgvt_dn12 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn12)), ((locals.var_vgvt_dn17 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn17)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19550_e27068;
        locals.var_kusai00_dn0 = assign19550_e27068_d_n0;
        locals.var_kusai00_dn2 = assign19550_e27068_d_n2;
        locals.var_kusai00_dn6 = assign19550_e27068_d_n6;
        locals.var_kusai00_dn7 = assign19550_e27068_d_n7;
        locals.var_kusai00_dn10 = assign19550_e27068_d_n10;
        locals.var_kusai00_dn11 = assign19550_e27068_d_n11;
        locals.var_kusai00_dn12 = assign19550_e27068_d_n12;
        locals.var_kusai00_dn17 = assign19550_e27068_d_n17;
        locals.var_kusai00_rv = 0.0;

        let (assign19560_e27080, assign19560_e27080_d_n0, assign19560_e27080_d_n2, assign19560_e27080_d_n6, assign19560_e27080_d_n7, assign19560_e27080_d_n10, assign19560_e27080_d_n11, assign19560_e27080_d_n12, assign19560_e27080_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19560_e27074: f64 = (2.0 * locals.var_beta_inv);
        let assign19560_e27076: f64 = (assign19560_e27074 * locals.var_c_fox_inv);
        let assign19560_e27078: f64 = (assign19560_e27076 * locals.var_idd);
        (assign19560_e27078, (((assign19560_e27074 * locals.var_c_fox_inv_dn0) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn0)), (((assign19560_e27074 * locals.var_c_fox_inv_dn2) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn2)), (((assign19560_e27074 * locals.var_c_fox_inv_dn6) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn6)), (((assign19560_e27074 * locals.var_c_fox_inv_dn7) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_c_fox_inv) + (assign19560_e27074 * locals.var_c_fox_inv_dn10)) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn10)), (((assign19560_e27074 * locals.var_c_fox_inv_dn11) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn11)), (((assign19560_e27074 * locals.var_c_fox_inv_dn12) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn12)), (((assign19560_e27074 * locals.var_c_fox_inv_dn17) * locals.var_idd) + (assign19560_e27076 * locals.var_idd_dn17)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn10, locals.var_kusaidd_dn11, locals.var_kusaidd_dn12, locals.var_kusaidd_dn17,)
    }
};
        locals.var_kusaidd = assign19560_e27080;
        locals.var_kusaidd_dn0 = assign19560_e27080_d_n0;
        locals.var_kusaidd_dn2 = assign19560_e27080_d_n2;
        locals.var_kusaidd_dn6 = assign19560_e27080_d_n6;
        locals.var_kusaidd_dn7 = assign19560_e27080_d_n7;
        locals.var_kusaidd_dn10 = assign19560_e27080_d_n10;
        locals.var_kusaidd_dn11 = assign19560_e27080_d_n11;
        locals.var_kusaidd_dn12 = assign19560_e27080_d_n12;
        locals.var_kusaidd_dn17 = assign19560_e27080_d_n17;
        locals.var_kusaidd_rv = 0.0;

        let (assign19570_e27088, assign19570_e27088_d_n0, assign19570_e27088_d_n2, assign19570_e27088_d_n6, assign19570_e27088_d_n7, assign19570_e27088_d_n10, assign19570_e27088_d_n11, assign19570_e27088_d_n12, assign19570_e27088_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19570_e27086: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign19570_e27086, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn11 - locals.var_kusaidd_dn11), (locals.var_kusai00_dn12 - locals.var_kusaidd_dn12), (locals.var_kusai00_dn17 - locals.var_kusaidd_dn17),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19570_e27088;
        locals.var_kusail_dn0 = assign19570_e27088_d_n0;
        locals.var_kusail_dn2 = assign19570_e27088_d_n2;
        locals.var_kusail_dn6 = assign19570_e27088_d_n6;
        locals.var_kusail_dn7 = assign19570_e27088_d_n7;
        locals.var_kusail_dn10 = assign19570_e27088_d_n10;
        locals.var_kusail_dn11 = assign19570_e27088_d_n11;
        locals.var_kusail_dn12 = assign19570_e27088_d_n12;
        locals.var_kusail_dn17 = assign19570_e27088_d_n17;
        locals.var_kusail_rv = 0.0;

        let (assign19580_e27103, assign19580_e27103_d_n0, assign19580_e27103_d_n2, assign19580_e27103_d_n6, assign19580_e27103_d_n7, assign19580_e27103_d_n10, assign19580_e27103_d_n11, assign19580_e27103_d_n12, assign19580_e27103_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19580_e27094: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign19580_e27097: f64 = (4.0 * 0.001);
        let assign19580_e27099: f64 = (assign19580_e27097 * 0.001);
        let assign19580_e27100: f64 = (assign19580_e27094 + assign19580_e27099);
        let assign19580_e27101: f64 = (assign19580_e27100).sqrt();
        (assign19580_e27101, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)) / (2.0 * assign19580_e27101)), (((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)) / (2.0 * assign19580_e27101)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19580_e27103;
        locals.var_tmf1_dn0 = assign19580_e27103_d_n0;
        locals.var_tmf1_dn2 = assign19580_e27103_d_n2;
        locals.var_tmf1_dn6 = assign19580_e27103_d_n6;
        locals.var_tmf1_dn7 = assign19580_e27103_d_n7;
        locals.var_tmf1_dn10 = assign19580_e27103_d_n10;
        locals.var_tmf1_dn11 = assign19580_e27103_d_n11;
        locals.var_tmf1_dn12 = assign19580_e27103_d_n12;
        locals.var_tmf1_dn17 = assign19580_e27103_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19590_e27117, assign19590_e27117_d_n0, assign19590_e27117_d_n2, assign19590_e27117_d_n6, assign19590_e27117_d_n7, assign19590_e27117_d_n10, assign19590_e27117_d_n11, assign19590_e27117_d_n12, assign19590_e27117_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19590_e27110: f64 = (locals.var_kusai00 + locals.var_tmf1);
        let assign19590_e27111: f64 = (0.5 * assign19590_e27110);
        let assign19590_e27114: f64 = (1e-10 * 0.001);
        let assign19590_e27115: f64 = (assign19590_e27111 + assign19590_e27114);
        (assign19590_e27115, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_kusai00_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_kusai00_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_kusai00_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19590_e27117;
        locals.var_kusai00_dn0 = assign19590_e27117_d_n0;
        locals.var_kusai00_dn2 = assign19590_e27117_d_n2;
        locals.var_kusai00_dn6 = assign19590_e27117_d_n6;
        locals.var_kusai00_dn7 = assign19590_e27117_d_n7;
        locals.var_kusai00_dn10 = assign19590_e27117_d_n10;
        locals.var_kusai00_dn11 = assign19590_e27117_d_n11;
        locals.var_kusai00_dn12 = assign19590_e27117_d_n12;
        locals.var_kusai00_dn17 = assign19590_e27117_d_n17;
        locals.var_kusai00_rv = 0.0;

        let assign19600_e27120: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign19600_e27120;
        locals.var_guard595_rv = 0.0;

        let (assign19610_e27128, assign19610_e27128_d_n0, assign19610_e27128_d_n2, assign19610_e27128_d_n6, assign19610_e27128_d_n7, assign19610_e27128_d_n10, assign19610_e27128_d_n11, assign19610_e27128_d_n12, assign19610_e27128_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19610_e27128;
        locals.var_kusai00_dn0 = assign19610_e27128_d_n0;
        locals.var_kusai00_dn2 = assign19610_e27128_d_n2;
        locals.var_kusai00_dn6 = assign19610_e27128_d_n6;
        locals.var_kusai00_dn7 = assign19610_e27128_d_n7;
        locals.var_kusai00_dn10 = assign19610_e27128_d_n10;
        locals.var_kusai00_dn11 = assign19610_e27128_d_n11;
        locals.var_kusai00_dn12 = assign19610_e27128_d_n12;
        locals.var_kusai00_dn17 = assign19610_e27128_d_n17;
        locals.var_kusai00_rv = 0.0;

        let (assign19620_e27143, assign19620_e27143_d_n0, assign19620_e27143_d_n2, assign19620_e27143_d_n6, assign19620_e27143_d_n7, assign19620_e27143_d_n10, assign19620_e27143_d_n11, assign19620_e27143_d_n12, assign19620_e27143_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19620_e27134: f64 = (locals.var_kusail * locals.var_kusail);
        let assign19620_e27137: f64 = (4.0 * 0.001);
        let assign19620_e27139: f64 = (assign19620_e27137 * 0.001);
        let assign19620_e27140: f64 = (assign19620_e27134 + assign19620_e27139);
        let assign19620_e27141: f64 = (assign19620_e27140).sqrt();
        (assign19620_e27141, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)) / (2.0 * assign19620_e27141)), (((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)) / (2.0 * assign19620_e27141)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19620_e27143;
        locals.var_tmf1_dn0 = assign19620_e27143_d_n0;
        locals.var_tmf1_dn2 = assign19620_e27143_d_n2;
        locals.var_tmf1_dn6 = assign19620_e27143_d_n6;
        locals.var_tmf1_dn7 = assign19620_e27143_d_n7;
        locals.var_tmf1_dn10 = assign19620_e27143_d_n10;
        locals.var_tmf1_dn11 = assign19620_e27143_d_n11;
        locals.var_tmf1_dn12 = assign19620_e27143_d_n12;
        locals.var_tmf1_dn17 = assign19620_e27143_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19630_e27157, assign19630_e27157_d_n0, assign19630_e27157_d_n2, assign19630_e27157_d_n6, assign19630_e27157_d_n7, assign19630_e27157_d_n10, assign19630_e27157_d_n11, assign19630_e27157_d_n12, assign19630_e27157_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19630_e27150: f64 = (locals.var_kusail + locals.var_tmf1);
        let assign19630_e27151: f64 = (0.5 * assign19630_e27150);
        let assign19630_e27154: f64 = (1e-10 * 0.001);
        let assign19630_e27155: f64 = (assign19630_e27151 + assign19630_e27154);
        (assign19630_e27155, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_kusail_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_kusail_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_kusail_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19630_e27157;
        locals.var_kusail_dn0 = assign19630_e27157_d_n0;
        locals.var_kusail_dn2 = assign19630_e27157_d_n2;
        locals.var_kusail_dn6 = assign19630_e27157_d_n6;
        locals.var_kusail_dn7 = assign19630_e27157_d_n7;
        locals.var_kusail_dn10 = assign19630_e27157_d_n10;
        locals.var_kusail_dn11 = assign19630_e27157_d_n11;
        locals.var_kusail_dn12 = assign19630_e27157_d_n12;
        locals.var_kusail_dn17 = assign19630_e27157_d_n17;
        locals.var_kusail_rv = 0.0;

        let assign19640_e27160: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign19640_e27160;
        locals.var_guard596_rv = 0.0;

        let (assign19650_e27168, assign19650_e27168_d_n0, assign19650_e27168_d_n2, assign19650_e27168_d_n6, assign19650_e27168_d_n7, assign19650_e27168_d_n10, assign19650_e27168_d_n11, assign19650_e27168_d_n12, assign19650_e27168_d_n17,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) && (locals.var_guard596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19650_e27168;
        locals.var_kusail_dn0 = assign19650_e27168_d_n0;
        locals.var_kusail_dn2 = assign19650_e27168_d_n2;
        locals.var_kusail_dn6 = assign19650_e27168_d_n6;
        locals.var_kusail_dn7 = assign19650_e27168_d_n7;
        locals.var_kusail_dn10 = assign19650_e27168_d_n10;
        locals.var_kusail_dn11 = assign19650_e27168_d_n11;
        locals.var_kusail_dn12 = assign19650_e27168_d_n12;
        locals.var_kusail_dn17 = assign19650_e27168_d_n17;
        locals.var_kusail_rv = 0.0;

        let (assign19660_e27176, assign19660_e27176_d_n0, assign19660_e27176_d_n2, assign19660_e27176_d_n6, assign19660_e27176_d_n7, assign19660_e27176_d_n10, assign19660_e27176_d_n11, assign19660_e27176_d_n12, assign19660_e27176_d_n17,) = {
    if ((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign19660_e27174: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign19660_e27174, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn11 - locals.var_kusail_dn11), (locals.var_kusai00_dn12 - locals.var_kusail_dn12), (locals.var_kusai00_dn17 - locals.var_kusail_dn17),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn10, locals.var_kusai00l_dn11, locals.var_kusai00l_dn12, locals.var_kusai00l_dn17,)
    }
};
        locals.var_kusai00l = assign19660_e27176;
        locals.var_kusai00l_dn0 = assign19660_e27176_d_n0;
        locals.var_kusai00l_dn2 = assign19660_e27176_d_n2;
        locals.var_kusai00l_dn6 = assign19660_e27176_d_n6;
        locals.var_kusai00l_dn7 = assign19660_e27176_d_n7;
        locals.var_kusai00l_dn10 = assign19660_e27176_d_n10;
        locals.var_kusai00l_dn11 = assign19660_e27176_d_n11;
        locals.var_kusai00l_dn12 = assign19660_e27176_d_n12;
        locals.var_kusai00l_dn17 = assign19660_e27176_d_n17;
        locals.var_kusai00l_rv = 0.0;

        let assign19670_e27180: f64 = (10.0 * 2.220446049250313e-16);
        let assign19670_e27185: f64 = (10.0 * 2.220446049250313e-16);
        let assign19670_e27187: f64 = if ((locals.var_qn0 < assign19670_e27180) || (locals.var_kusai00l < assign19670_e27185)) { 1.0 } else { 0.0 };
        locals.var_guard597 = assign19670_e27187;
        locals.var_guard597_rv = 0.0;

        let (assign19680_e27195,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) && (locals.var_guard597 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign19680_e27195;
        locals.var_flg_ign_rv = 0.0;

        let (assign19690_e27204,) = {
    if (((locals.var_guard507 != 0.0) && (locals.var_guard594 != 0.0)) && (locals.var_guard597 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign19690_e27204;
        locals.var_flg_ign_rv = 0.0;

        locals.var_idsorg = locals.var_ids;
        locals.var_idsorg_dn0 = locals.var_ids_dn0;
        locals.var_idsorg_dn2 = locals.var_ids_dn2;
        locals.var_idsorg_dn6 = locals.var_ids_dn6;
        locals.var_idsorg_dn7 = locals.var_ids_dn7;
        locals.var_idsorg_dn10 = locals.var_ids_dn10;
        locals.var_idsorg_dn11 = locals.var_ids_dn11;
        locals.var_idsorg_dn12 = locals.var_ids_dn12;
        locals.var_idsorg_dn17 = locals.var_ids_dn17;
        locals.var_idsorg_rv = 0.0;

        locals.var_idspt1 = 0.0;
        locals.var_idspt1_dn0 = 0.0;
        locals.var_idspt1_dn2 = 0.0;
        locals.var_idspt1_dn6 = 0.0;
        locals.var_idspt1_dn7 = 0.0;
        locals.var_idspt1_dn10 = 0.0;
        locals.var_idspt1_dn11 = 0.0;
        locals.var_idspt1_dn12 = 0.0;
        locals.var_idspt1_dn17 = 0.0;
        locals.var_idspt1_rv = 0.0;

        let assign19720_e27213: f64 = if ((p.p281 > 0.0) && (p.p285 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard598 = assign19720_e27213;
        locals.var_guard598_rv = 0.0;

        let (assign19730_e27217,) = {
    if (locals.var_guard598 != 0.0) {
        (locals.var_lgleff,)
    } else {
        (locals.var_leff__blk605,)
    }
};
        locals.var_leff__blk605 = assign19730_e27217;
        locals.var_leff__blk605_rv = 0.0;

        let (assign19740_e27221,) = {
    if (locals.var_guard598 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi__blk609,)
    }
};
        locals.var_t_soi__blk609 = assign19740_e27221;
        locals.var_t_soi__blk609_rv = 0.0;

        let (assign19750_e27233, assign19750_e27233_d_n0, assign19750_e27233_d_n2, assign19750_e27233_d_n6, assign19750_e27233_d_n7, assign19750_e27233_d_n10, assign19750_e27233_d_n11, assign19750_e27233_d_n12, assign19750_e27233_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19750_e27225: f64 = (locals.var_vgs - locals.var_vfb);
        let assign19750_e27227: f64 = (assign19750_e27225 + locals.var_dvth);
        let assign19750_e27229: f64 = (assign19750_e27227 - locals.var_dppg);
        let assign19750_e27231: f64 = (assign19750_e27229 - p.p286);
        (assign19750_e27231, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), (locals.var_dvth_dn10 - locals.var_dppg_dn10), ((locals.var_vgs_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), (locals.var_dvth_dn12 - locals.var_dppg_dn12), (locals.var_dvth_dn17 - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgp__blk610, locals.var_vgp__blk610_dn0, locals.var_vgp__blk610_dn2, locals.var_vgp__blk610_dn6, locals.var_vgp__blk610_dn7, locals.var_vgp__blk610_dn10, locals.var_vgp__blk610_dn11, locals.var_vgp__blk610_dn12, locals.var_vgp__blk610_dn17,)
    }
};
        locals.var_vgp__blk610 = assign19750_e27233;
        locals.var_vgp__blk610_dn0 = assign19750_e27233_d_n0;
        locals.var_vgp__blk610_dn2 = assign19750_e27233_d_n2;
        locals.var_vgp__blk610_dn6 = assign19750_e27233_d_n6;
        locals.var_vgp__blk610_dn7 = assign19750_e27233_d_n7;
        locals.var_vgp__blk610_dn10 = assign19750_e27233_d_n10;
        locals.var_vgp__blk610_dn11 = assign19750_e27233_d_n11;
        locals.var_vgp__blk610_dn12 = assign19750_e27233_d_n12;
        locals.var_vgp__blk610_dn17 = assign19750_e27233_d_n17;
        locals.var_vgp__blk610_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19760_e27239, assign19760_e27239_d_n0, assign19760_e27239_d_n2, assign19760_e27239_d_n6, assign19760_e27239_d_n7, assign19760_e27239_d_n10, assign19760_e27239_d_n11, assign19760_e27239_d_n12, assign19760_e27239_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19760_e27237: f64 = (locals.var_vth + p.p286);
        (assign19760_e27237, locals.var_vth_dn0, locals.var_vth_dn2, locals.var_vth_dn6, locals.var_vth_dn7, locals.var_vth_dn10, locals.var_vth_dn11, locals.var_vth_dn12, locals.var_vth_dn17,)
    } else {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn12, locals.var_wk_vth_dn17,)
    }
};
        locals.var_wk_vth = assign19760_e27239;
        locals.var_wk_vth_dn0 = assign19760_e27239_d_n0;
        locals.var_wk_vth_dn2 = assign19760_e27239_d_n2;
        locals.var_wk_vth_dn6 = assign19760_e27239_d_n6;
        locals.var_wk_vth_dn7 = assign19760_e27239_d_n7;
        locals.var_wk_vth_dn10 = assign19760_e27239_d_n10;
        locals.var_wk_vth_dn11 = assign19760_e27239_d_n11;
        locals.var_wk_vth_dn12 = assign19760_e27239_d_n12;
        locals.var_wk_vth_dn17 = assign19760_e27239_d_n17;
        locals.var_wk_vth_rv = 0.0;

        let (assign19770_e27243,) = {
    if (locals.var_guard598 != 0.0) {
        (p.p285,)
    } else {
        (locals.var_wk_mu,)
    }
};
        locals.var_wk_mu = assign19770_e27243;
        locals.var_wk_mu_rv = 0.0;

        let (assign19780_e27247,) = {
    if (locals.var_guard598 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_wk_xj,)
    }
};
        locals.var_wk_xj = assign19780_e27247;
        locals.var_wk_xj_rv = 0.0;

        let (assign19790_e27251,) = {
    if (locals.var_guard598 != 0.0) {
        (locals.var_mks_njunc,)
    } else {
        (locals.var_uc_wk_njunc,)
    }
};
        locals.var_uc_wk_njunc = assign19790_e27251;
        locals.var_uc_wk_njunc_rv = 0.0;

        let (assign19800_e27264, assign19800_e27264_d_n0, assign19800_e27264_d_n2, assign19800_e27264_d_n6, assign19800_e27264_d_n7, assign19800_e27264_d_n10, assign19800_e27264_d_n11, assign19800_e27264_d_n12, assign19800_e27264_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19800_e27256: f64 = (locals.var_uc_wk_njunc / locals.var_nin);
        let assign19800_e27258: f64 = (assign19800_e27256 * locals.var_nsub);
        let assign19800_e27260: f64 = (assign19800_e27258 / locals.var_nin);
        let assign19800_e27261: f64 = (assign19800_e27260).ln();
        let assign19800_e27262: f64 = (locals.var_beta_inv * assign19800_e27261);
        (assign19800_e27262, (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn0)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn2)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn6)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn7)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)), ((locals.var_beta_inv_dn10 * assign19800_e27261) + (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn10)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260))), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn11)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn12)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19800_e27256 * locals.var_nsub_dn17)) * locals.var_nin) - (assign19800_e27258 * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign19800_e27260)),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn12, locals.var_vbipn_dn17,)
    }
};
        locals.var_vbipn = assign19800_e27264;
        locals.var_vbipn_dn0 = assign19800_e27264_d_n0;
        locals.var_vbipn_dn2 = assign19800_e27264_d_n2;
        locals.var_vbipn_dn6 = assign19800_e27264_d_n6;
        locals.var_vbipn_dn7 = assign19800_e27264_d_n7;
        locals.var_vbipn_dn10 = assign19800_e27264_d_n10;
        locals.var_vbipn_dn11 = assign19800_e27264_d_n11;
        locals.var_vbipn_dn12 = assign19800_e27264_d_n12;
        locals.var_vbipn_dn17 = assign19800_e27264_d_n17;
        locals.var_vbipn_rv = 0.0;

        let (assign19810_e27273, assign19810_e27273_d_n0, assign19810_e27273_d_n2, assign19810_e27273_d_n6, assign19810_e27273_d_n7, assign19810_e27273_d_n10, assign19810_e27273_d_n11, assign19810_e27273_d_n12, assign19810_e27273_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let (assign19810_e27271, assign19810_e27271_d_n0, assign19810_e27271_d_n2, assign19810_e27271_d_n6, assign19810_e27271_d_n7, assign19810_e27271_d_n10, assign19810_e27271_d_n11, assign19810_e27271_d_n12, assign19810_e27271_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
            } else {
                (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
            }
        };
        (assign19810_e27271, assign19810_e27271_d_n0, assign19810_e27271_d_n2, assign19810_e27271_d_n6, assign19810_e27271_d_n7, assign19810_e27271_d_n10, assign19810_e27271_d_n11, assign19810_e27271_d_n12, assign19810_e27271_d_n17,)
    } else {
        (locals.var_vbs__blk601, locals.var_vbs__blk601_dn0, locals.var_vbs__blk601_dn2, locals.var_vbs__blk601_dn6, locals.var_vbs__blk601_dn7, locals.var_vbs__blk601_dn10, locals.var_vbs__blk601_dn11, locals.var_vbs__blk601_dn12, locals.var_vbs__blk601_dn17,)
    }
};
        locals.var_vbs__blk601 = assign19810_e27273;
        locals.var_vbs__blk601_dn0 = assign19810_e27273_d_n0;
        locals.var_vbs__blk601_dn2 = assign19810_e27273_d_n2;
        locals.var_vbs__blk601_dn6 = assign19810_e27273_d_n6;
        locals.var_vbs__blk601_dn7 = assign19810_e27273_d_n7;
        locals.var_vbs__blk601_dn10 = assign19810_e27273_d_n10;
        locals.var_vbs__blk601_dn11 = assign19810_e27273_d_n11;
        locals.var_vbs__blk601_dn12 = assign19810_e27273_d_n12;
        locals.var_vbs__blk601_dn17 = assign19810_e27273_d_n17;
        locals.var_vbs__blk601_rv = 0.0;

        let (assign19820_e27294, assign19820_e27294_d_n0, assign19820_e27294_d_n2, assign19820_e27294_d_n6, assign19820_e27294_d_n7, assign19820_e27294_d_n10, assign19820_e27294_d_n11, assign19820_e27294_d_n12, assign19820_e27294_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19820_e27277: f64 = (2.0 * 1.6021918e-19);
        let assign19820_e27280: f64 = (locals.var_vbipn - locals.var_vbs__blk601);
        let assign19820_e27281: f64 = (assign19820_e27277 * assign19820_e27280);
        let assign19820_e27283: f64 = (assign19820_e27281 / 1.034943e-10);
        let assign19820_e27285: f64 = (assign19820_e27283 * locals.var_nsub);
        let assign19820_e27287: f64 = (assign19820_e27285 * locals.var_uc_wk_njunc);
        let assign19820_e27290: f64 = (locals.var_nsub + locals.var_uc_wk_njunc);
        let assign19820_e27291: f64 = (assign19820_e27287 / assign19820_e27290);
        let assign19820_e27292: f64 = (assign19820_e27291).sqrt();
        (assign19820_e27292, (((((((((assign19820_e27277 * (locals.var_vbipn_dn0 - locals.var_vbs__blk601_dn0)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn0)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn0)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn2 - locals.var_vbs__blk601_dn2)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn2)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn2)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn6 - locals.var_vbs__blk601_dn6)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn6)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn6)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn7 - locals.var_vbs__blk601_dn7)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn7)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn7)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn10 - locals.var_vbs__blk601_dn10)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn10)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn10)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn11 - locals.var_vbs__blk601_dn11)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn11)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn11)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn12 - locals.var_vbs__blk601_dn12)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn12)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn12)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)), (((((((((assign19820_e27277 * (locals.var_vbipn_dn17 - locals.var_vbs__blk601_dn17)) / 1.034943e-10) * locals.var_nsub) + (assign19820_e27283 * locals.var_nsub_dn17)) * locals.var_uc_wk_njunc) * assign19820_e27290) - (assign19820_e27287 * locals.var_nsub_dn17)) / (assign19820_e27290 * assign19820_e27290)) / (2.0 * assign19820_e27292)),)
    } else {
        (locals.var_ec__blk606, locals.var_ec__blk606_dn0, locals.var_ec__blk606_dn2, locals.var_ec__blk606_dn6, locals.var_ec__blk606_dn7, locals.var_ec__blk606_dn10, locals.var_ec__blk606_dn11, locals.var_ec__blk606_dn12, locals.var_ec__blk606_dn17,)
    }
};
        locals.var_ec__blk606 = assign19820_e27294;
        locals.var_ec__blk606_dn0 = assign19820_e27294_d_n0;
        locals.var_ec__blk606_dn2 = assign19820_e27294_d_n2;
        locals.var_ec__blk606_dn6 = assign19820_e27294_d_n6;
        locals.var_ec__blk606_dn7 = assign19820_e27294_d_n7;
        locals.var_ec__blk606_dn10 = assign19820_e27294_d_n10;
        locals.var_ec__blk606_dn11 = assign19820_e27294_d_n11;
        locals.var_ec__blk606_dn12 = assign19820_e27294_d_n12;
        locals.var_ec__blk606_dn17 = assign19820_e27294_d_n17;
        locals.var_ec__blk606_rv = 0.0;

        let (assign19830_e27300, assign19830_e27300_d_n0, assign19830_e27300_d_n2, assign19830_e27300_d_n6, assign19830_e27300_d_n7, assign19830_e27300_d_n10, assign19830_e27300_d_n11, assign19830_e27300_d_n12, assign19830_e27300_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19830_e27298: f64 = (locals.var_ec__blk606 * locals.var_leff__blk605);
        (assign19830_e27298, (locals.var_ec__blk606_dn0 * locals.var_leff__blk605), (locals.var_ec__blk606_dn2 * locals.var_leff__blk605), (locals.var_ec__blk606_dn6 * locals.var_leff__blk605), (locals.var_ec__blk606_dn7 * locals.var_leff__blk605), (locals.var_ec__blk606_dn10 * locals.var_leff__blk605), (locals.var_ec__blk606_dn11 * locals.var_leff__blk605), (locals.var_ec__blk606_dn12 * locals.var_leff__blk605), (locals.var_ec__blk606_dn17 * locals.var_leff__blk605),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn10, locals.var_wk_dn11, locals.var_wk_dn12, locals.var_wk_dn17,)
    }
};
        locals.var_wk = assign19830_e27300;
        locals.var_wk_dn0 = assign19830_e27300_d_n0;
        locals.var_wk_dn2 = assign19830_e27300_d_n2;
        locals.var_wk_dn6 = assign19830_e27300_d_n6;
        locals.var_wk_dn7 = assign19830_e27300_d_n7;
        locals.var_wk_dn10 = assign19830_e27300_d_n10;
        locals.var_wk_dn11 = assign19830_e27300_d_n11;
        locals.var_wk_dn12 = assign19830_e27300_d_n12;
        locals.var_wk_dn17 = assign19830_e27300_d_n17;
        locals.var_wk_rv = 0.0;

        let (assign19840_e27313, assign19840_e27313_d_n0, assign19840_e27313_d_n2, assign19840_e27313_d_n6, assign19840_e27313_d_n7, assign19840_e27313_d_n10, assign19840_e27313_d_n11, assign19840_e27313_d_n12, assign19840_e27313_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19840_e27303: f64 = (-0.25);
        let assign19840_e27305: f64 = (assign19840_e27303 * locals.var_wk);
        let assign19840_e27307: f64 = (assign19840_e27305 * locals.var_wk);
        let assign19840_e27310: f64 = (locals.var_vds + locals.var_wk);
        let assign19840_e27311: f64 = (assign19840_e27307 / assign19840_e27310);
        (assign19840_e27311, ((((((assign19840_e27303 * locals.var_wk_dn0) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn0)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn2) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn2)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn6) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn6)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn7) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn7)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn10) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn10)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn11) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn11)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn11 + locals.var_wk_dn11))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn12) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn12)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn12 + locals.var_wk_dn12))) / (assign19840_e27310 * assign19840_e27310)), ((((((assign19840_e27303 * locals.var_wk_dn17) * locals.var_wk) + (assign19840_e27305 * locals.var_wk_dn17)) * assign19840_e27310) - (assign19840_e27307 * (locals.var_vds_dn17 + locals.var_wk_dn17))) / (assign19840_e27310 * assign19840_e27310)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    }
};
        locals.var_dphi_vds = assign19840_e27313;
        locals.var_dphi_vds_dn0 = assign19840_e27313_d_n0;
        locals.var_dphi_vds_dn2 = assign19840_e27313_d_n2;
        locals.var_dphi_vds_dn6 = assign19840_e27313_d_n6;
        locals.var_dphi_vds_dn7 = assign19840_e27313_d_n7;
        locals.var_dphi_vds_dn10 = assign19840_e27313_d_n10;
        locals.var_dphi_vds_dn11 = assign19840_e27313_d_n11;
        locals.var_dphi_vds_dn12 = assign19840_e27313_d_n12;
        locals.var_dphi_vds_dn17 = assign19840_e27313_d_n17;
        locals.var_dphi_vds_rv = 0.0;

        let (assign19850_e27317, assign19850_e27317_d_n0, assign19850_e27317_d_n2, assign19850_e27317_d_n6, assign19850_e27317_d_n7, assign19850_e27317_d_n10, assign19850_e27317_d_n11, assign19850_e27317_d_n12, assign19850_e27317_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    } else {
        (locals.var_vbs__blk625, locals.var_vbs__blk625_dn0, locals.var_vbs__blk625_dn2, locals.var_vbs__blk625_dn6, locals.var_vbs__blk625_dn7, locals.var_vbs__blk625_dn10, locals.var_vbs__blk625_dn11, locals.var_vbs__blk625_dn12, locals.var_vbs__blk625_dn17,)
    }
};
        locals.var_vbs__blk625 = assign19850_e27317;
        locals.var_vbs__blk625_dn0 = assign19850_e27317_d_n0;
        locals.var_vbs__blk625_dn2 = assign19850_e27317_d_n2;
        locals.var_vbs__blk625_dn6 = assign19850_e27317_d_n6;
        locals.var_vbs__blk625_dn7 = assign19850_e27317_d_n7;
        locals.var_vbs__blk625_dn10 = assign19850_e27317_d_n10;
        locals.var_vbs__blk625_dn11 = assign19850_e27317_d_n11;
        locals.var_vbs__blk625_dn12 = assign19850_e27317_d_n12;
        locals.var_vbs__blk625_dn17 = assign19850_e27317_d_n17;
        locals.var_vbs__blk625_rv = 0.0;

        let (assign19860_e27321, assign19860_e27321_d_n0, assign19860_e27321_d_n2, assign19860_e27321_d_n6, assign19860_e27321_d_n7, assign19860_e27321_d_n10, assign19860_e27321_d_n11, assign19860_e27321_d_n12, assign19860_e27321_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn12, locals.var_wk_vth_dn17,)
    } else {
        (locals.var_vth__blk626, locals.var_vth__blk626_dn0, locals.var_vth__blk626_dn2, locals.var_vth__blk626_dn6, locals.var_vth__blk626_dn7, locals.var_vth__blk626_dn10, locals.var_vth__blk626_dn11, locals.var_vth__blk626_dn12, locals.var_vth__blk626_dn17,)
    }
};
        locals.var_vth__blk626 = assign19860_e27321;
        locals.var_vth__blk626_dn0 = assign19860_e27321_d_n0;
        locals.var_vth__blk626_dn2 = assign19860_e27321_d_n2;
        locals.var_vth__blk626_dn6 = assign19860_e27321_d_n6;
        locals.var_vth__blk626_dn7 = assign19860_e27321_d_n7;
        locals.var_vth__blk626_dn10 = assign19860_e27321_d_n10;
        locals.var_vth__blk626_dn11 = assign19860_e27321_d_n11;
        locals.var_vth__blk626_dn12 = assign19860_e27321_d_n12;
        locals.var_vth__blk626_dn17 = assign19860_e27321_d_n17;
        locals.var_vth__blk626_rv = 0.0;

        let (assign19870_e27339, assign19870_e27339_d_n0, assign19870_e27339_d_n2, assign19870_e27339_d_n6, assign19870_e27339_d_n7, assign19870_e27339_d_n10, assign19870_e27339_d_n11, assign19870_e27339_d_n12, assign19870_e27339_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19870_e27328: f64 = (locals.var_vgp__blk610 - locals.var_vbs__blk625);
        let assign19870_e27329: f64 = (locals.var_beta * assign19870_e27328);
        let assign19870_e27331: f64 = (assign19870_e27329 - 1.0);
        let assign19870_e27332: f64 = (4.0 * assign19870_e27331);
        let assign19870_e27335: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign19870_e27336: f64 = (assign19870_e27332 / assign19870_e27335);
        let assign19870_e27337: f64 = (1.0 + assign19870_e27336);
        (assign19870_e27337, ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn0 - locals.var_vbs__blk625_dn0))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn2 - locals.var_vbs__blk625_dn2))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn6 - locals.var_vbs__blk625_dn6))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn7 - locals.var_vbs__blk625_dn7))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * ((locals.var_beta_dn10 * assign19870_e27328) + (locals.var_beta * (locals.var_vgp__blk610_dn10 - locals.var_vbs__blk625_dn10)))) * assign19870_e27335) - (assign19870_e27332 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn11 - locals.var_vbs__blk625_dn11))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn12 - locals.var_vbs__blk625_dn12))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk610_dn17 - locals.var_vbs__blk625_dn17))) * assign19870_e27335) - (assign19870_e27332 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign19870_e27335 * assign19870_e27335)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign19870_e27339;
        locals.var_tx_dn0 = assign19870_e27339_d_n0;
        locals.var_tx_dn2 = assign19870_e27339_d_n2;
        locals.var_tx_dn6 = assign19870_e27339_d_n6;
        locals.var_tx_dn7 = assign19870_e27339_d_n7;
        locals.var_tx_dn10 = assign19870_e27339_d_n10;
        locals.var_tx_dn11 = assign19870_e27339_d_n11;
        locals.var_tx_dn12 = assign19870_e27339_d_n12;
        locals.var_tx_dn17 = assign19870_e27339_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign19880_e27352, assign19880_e27352_d_n0, assign19880_e27352_d_n2, assign19880_e27352_d_n6, assign19880_e27352_d_n7, assign19880_e27352_d_n10, assign19880_e27352_d_n11, assign19880_e27352_d_n12, assign19880_e27352_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19880_e27344: f64 = (10.0 * 2.220446049250313e-16);
        let (assign19880_e27350, assign19880_e27350_d_n0, assign19880_e27350_d_n2, assign19880_e27350_d_n6, assign19880_e27350_d_n7, assign19880_e27350_d_n10, assign19880_e27350_d_n11, assign19880_e27350_d_n12, assign19880_e27350_d_n17,) = {
            if (locals.var_tx >= assign19880_e27344) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign19880_e27349: f64 = (10.0 * 2.220446049250313e-16);
                (assign19880_e27349, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign19880_e27350, assign19880_e27350_d_n0, assign19880_e27350_d_n2, assign19880_e27350_d_n6, assign19880_e27350_d_n7, assign19880_e27350_d_n10, assign19880_e27350_d_n11, assign19880_e27350_d_n12, assign19880_e27350_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign19880_e27352;
        locals.var_tx_dn0 = assign19880_e27352_d_n0;
        locals.var_tx_dn2 = assign19880_e27352_d_n2;
        locals.var_tx_dn6 = assign19880_e27352_d_n6;
        locals.var_tx_dn7 = assign19880_e27352_d_n7;
        locals.var_tx_dn10 = assign19880_e27352_d_n10;
        locals.var_tx_dn11 = assign19880_e27352_d_n11;
        locals.var_tx_dn12 = assign19880_e27352_d_n12;
        locals.var_tx_dn17 = assign19880_e27352_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign19890_e27367, assign19890_e27367_d_n0, assign19890_e27367_d_n2, assign19890_e27367_d_n6, assign19890_e27367_d_n7, assign19890_e27367_d_n10, assign19890_e27367_d_n11, assign19890_e27367_d_n12, assign19890_e27367_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign19890_e27357: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign19890_e27359: f64 = (assign19890_e27357 * 0.5);
        let assign19890_e27362: f64 = (locals.var_tx).sqrt();
        let assign19890_e27363: f64 = (1.0 - assign19890_e27362);
        let assign19890_e27364: f64 = (assign19890_e27359 * assign19890_e27363);
        let assign19890_e27365: f64 = (locals.var_vgp__blk610 + assign19890_e27364);
        (assign19890_e27365, (locals.var_vgp__blk610_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn0 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn2 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn6 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn7 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn10 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn11 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn12 / (2.0 * assign19890_e27362)))))), (locals.var_vgp__blk610_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign19890_e27363) + (assign19890_e27359 * (-(locals.var_tx_dn17 / (2.0 * assign19890_e27362)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign19890_e27367;
        locals.var_ps0_inia_dn0 = assign19890_e27367_d_n0;
        locals.var_ps0_inia_dn2 = assign19890_e27367_d_n2;
        locals.var_ps0_inia_dn6 = assign19890_e27367_d_n6;
        locals.var_ps0_inia_dn7 = assign19890_e27367_d_n7;
        locals.var_ps0_inia_dn10 = assign19890_e27367_d_n10;
        locals.var_ps0_inia_dn11 = assign19890_e27367_d_n11;
        locals.var_ps0_inia_dn12 = assign19890_e27367_d_n12;
        locals.var_ps0_inia_dn17 = assign19890_e27367_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let assign19900_e27371: f64 = (locals.var_vfb + locals.var_vth__blk626);
        let assign19900_e27373: f64 = (assign19900_e27371 * 0.5);
        let assign19900_e27374: f64 = if locals.var_vgs < assign19900_e27373 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign19900_e27374;
        locals.var_guard627_rv = 0.0;

        let (assign19910_e27380,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard627 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_pprv,)
    }
};
        locals.var_flg_pprv = assign19910_e27380;
        locals.var_flg_pprv_rv = 0.0;

        let assign19920_e27385: f64 = if ((locals.var_flg_pprv == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard628 = assign19920_e27385;
        locals.var_guard628_rv = 0.0;

        let (assign19930_e27395, assign19930_e27395_d_n0, assign19930_e27395_d_n2, assign19930_e27395_d_n6, assign19930_e27395_d_n7, assign19930_e27395_d_n10, assign19930_e27395_d_n11, assign19930_e27395_d_n12, assign19930_e27395_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) {
        let assign19930_e27392: f64 = (locals.var_ps0_inia - locals.var_vbs__blk625);
        let assign19930_e27393: f64 = (locals.var_beta * assign19930_e27392);
        (assign19930_e27393, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbs__blk625_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbs__blk625_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbs__blk625_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbs__blk625_dn7)), ((locals.var_beta_dn10 * assign19930_e27392) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbs__blk625_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbs__blk625_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbs__blk625_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbs__blk625_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign19930_e27395;
        locals.var_chi_dn0 = assign19930_e27395_d_n0;
        locals.var_chi_dn2 = assign19930_e27395_d_n2;
        locals.var_chi_dn6 = assign19930_e27395_d_n6;
        locals.var_chi_dn7 = assign19930_e27395_d_n7;
        locals.var_chi_dn10 = assign19930_e27395_d_n10;
        locals.var_chi_dn11 = assign19930_e27395_d_n11;
        locals.var_chi_dn12 = assign19930_e27395_d_n12;
        locals.var_chi_dn17 = assign19930_e27395_d_n17;
        locals.var_chi_rv = 0.0;

        let assign19940_e27398: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign19940_e27398;
        locals.var_guard629_rv = 0.0;

        let (assign19950_e27410, assign19950_e27410_d_n0, assign19950_e27410_d_n2, assign19950_e27410_d_n6, assign19950_e27410_d_n7, assign19950_e27410_d_n10, assign19950_e27410_d_n11, assign19950_e27410_d_n12, assign19950_e27410_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign19950_e27407: f64 = (locals.var_vgp__blk610 - locals.var_vbs__blk625);
        let assign19950_e27408: f64 = (locals.var_beta * assign19950_e27407);
        (assign19950_e27408, (locals.var_beta * (locals.var_vgp__blk610_dn0 - locals.var_vbs__blk625_dn0)), (locals.var_beta * (locals.var_vgp__blk610_dn2 - locals.var_vbs__blk625_dn2)), (locals.var_beta * (locals.var_vgp__blk610_dn6 - locals.var_vbs__blk625_dn6)), (locals.var_beta * (locals.var_vgp__blk610_dn7 - locals.var_vbs__blk625_dn7)), ((locals.var_beta_dn10 * assign19950_e27407) + (locals.var_beta * (locals.var_vgp__blk610_dn10 - locals.var_vbs__blk625_dn10))), (locals.var_beta * (locals.var_vgp__blk610_dn11 - locals.var_vbs__blk625_dn11)), (locals.var_beta * (locals.var_vgp__blk610_dn12 - locals.var_vbs__blk625_dn12)), (locals.var_beta * (locals.var_vgp__blk610_dn17 - locals.var_vbs__blk625_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign19950_e27410;
        locals.var_ty_dn0 = assign19950_e27410_d_n0;
        locals.var_ty_dn2 = assign19950_e27410_d_n2;
        locals.var_ty_dn6 = assign19950_e27410_d_n6;
        locals.var_ty_dn7 = assign19950_e27410_d_n7;
        locals.var_ty_dn10 = assign19950_e27410_d_n10;
        locals.var_ty_dn11 = assign19950_e27410_d_n11;
        locals.var_ty_dn12 = assign19950_e27410_d_n12;
        locals.var_ty_dn17 = assign19950_e27410_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign19960_e27426, assign19960_e27426_d_n0, assign19960_e27426_d_n2, assign19960_e27426_d_n6, assign19960_e27426_d_n7, assign19960_e27426_d_n10, assign19960_e27426_d_n11, assign19960_e27426_d_n12, assign19960_e27426_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign19960_e27419: f64 = (1.414213562373095 / 108.0);
        let assign19960_e27421: f64 = (assign19960_e27419 * locals.var_beta);
        let assign19960_e27423: f64 = (assign19960_e27421 * locals.var_fac1);
        let assign19960_e27424: f64 = (1.0 / assign19960_e27423);
        (assign19960_e27424, (-((assign19960_e27421 * locals.var_fac1_dn0) / (assign19960_e27423 * assign19960_e27423))), (-((assign19960_e27421 * locals.var_fac1_dn2) / (assign19960_e27423 * assign19960_e27423))), (-((assign19960_e27421 * locals.var_fac1_dn6) / (assign19960_e27423 * assign19960_e27423))), (-((assign19960_e27421 * locals.var_fac1_dn7) / (assign19960_e27423 * assign19960_e27423))), (-((((assign19960_e27419 * locals.var_beta_dn10) * locals.var_fac1) + (assign19960_e27421 * locals.var_fac1_dn10)) / (assign19960_e27423 * assign19960_e27423))), (-((assign19960_e27421 * locals.var_fac1_dn11) / (assign19960_e27423 * assign19960_e27423))), (-((assign19960_e27421 * locals.var_fac1_dn12) / (assign19960_e27423 * assign19960_e27423))), (-((assign19960_e27421 * locals.var_fac1_dn17) / (assign19960_e27423 * assign19960_e27423))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign19960_e27426;
        locals.var_t1_dn0 = assign19960_e27426_d_n0;
        locals.var_t1_dn2 = assign19960_e27426_d_n2;
        locals.var_t1_dn6 = assign19960_e27426_d_n6;
        locals.var_t1_dn7 = assign19960_e27426_d_n7;
        locals.var_t1_dn10 = assign19960_e27426_d_n10;
        locals.var_t1_dn11 = assign19960_e27426_d_n11;
        locals.var_t1_dn12 = assign19960_e27426_d_n12;
        locals.var_t1_dn17 = assign19960_e27426_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign19970_e27438, assign19970_e27438_d_n0, assign19970_e27438_d_n2, assign19970_e27438_d_n6, assign19970_e27438_d_n7, assign19970_e27438_d_n10, assign19970_e27438_d_n11, assign19970_e27438_d_n12, assign19970_e27438_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign19970_e27435: f64 = (3.0 * locals.var_t1);
        let assign19970_e27436: f64 = (81.0 + assign19970_e27435);
        (assign19970_e27436, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign19970_e27438;
        locals.var_t2_dn0 = assign19970_e27438_d_n0;
        locals.var_t2_dn2 = assign19970_e27438_d_n2;
        locals.var_t2_dn6 = assign19970_e27438_d_n6;
        locals.var_t2_dn7 = assign19970_e27438_d_n7;
        locals.var_t2_dn10 = assign19970_e27438_d_n10;
        locals.var_t2_dn11 = assign19970_e27438_d_n11;
        locals.var_t2_dn12 = assign19970_e27438_d_n12;
        locals.var_t2_dn17 = assign19970_e27438_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign19980_e27457, assign19980_e27457_d_n0, assign19980_e27457_d_n2, assign19980_e27457_d_n6, assign19980_e27457_d_n7, assign19980_e27457_d_n10, assign19980_e27457_d_n11, assign19980_e27457_d_n12, assign19980_e27457_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign19980_e27445: f64 = (-2916.0);
        let assign19980_e27448: f64 = (81.0 * locals.var_t1);
        let assign19980_e27449: f64 = (assign19980_e27445 - assign19980_e27448);
        let assign19980_e27452: f64 = (27.0 * locals.var_t1);
        let assign19980_e27454: f64 = (assign19980_e27452 * locals.var_ty);
        let assign19980_e27455: f64 = (assign19980_e27449 + assign19980_e27454);
        (assign19980_e27455, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign19980_e27452 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign19980_e27457;
        locals.var_t3_dn0 = assign19980_e27457_d_n0;
        locals.var_t3_dn2 = assign19980_e27457_d_n2;
        locals.var_t3_dn6 = assign19980_e27457_d_n6;
        locals.var_t3_dn7 = assign19980_e27457_d_n7;
        locals.var_t3_dn10 = assign19980_e27457_d_n10;
        locals.var_t3_dn11 = assign19980_e27457_d_n11;
        locals.var_t3_dn12 = assign19980_e27457_d_n12;
        locals.var_t3_dn17 = assign19980_e27457_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign19990_e27477, assign19990_e27477_d_n0, assign19990_e27477_d_n2, assign19990_e27477_d_n6, assign19990_e27477_d_n7, assign19990_e27477_d_n10, assign19990_e27477_d_n11, assign19990_e27477_d_n12, assign19990_e27477_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign19990_e27467: f64 = (54.0 + locals.var_t1);
        let assign19990_e27468: f64 = (81.0 * assign19990_e27467);
        let assign19990_e27469: f64 = (1458.0 - assign19990_e27468);
        let assign19990_e27472: f64 = (27.0 * locals.var_t1);
        let assign19990_e27474: f64 = (assign19990_e27472 * locals.var_ty);
        let assign19990_e27475: f64 = (assign19990_e27469 + assign19990_e27474);
        (assign19990_e27475, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign19990_e27472 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign19990_e27477;
        locals.var_t4_dn0 = assign19990_e27477_d_n0;
        locals.var_t4_dn2 = assign19990_e27477_d_n2;
        locals.var_t4_dn6 = assign19990_e27477_d_n6;
        locals.var_t4_dn7 = assign19990_e27477_d_n7;
        locals.var_t4_dn10 = assign19990_e27477_d_n10;
        locals.var_t4_dn11 = assign19990_e27477_d_n11;
        locals.var_t4_dn12 = assign19990_e27477_d_n12;
        locals.var_t4_dn17 = assign19990_e27477_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20000_e27487, assign20000_e27487_d_n0, assign20000_e27487_d_n2, assign20000_e27487_d_n6, assign20000_e27487_d_n7, assign20000_e27487_d_n10, assign20000_e27487_d_n11, assign20000_e27487_d_n12, assign20000_e27487_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20000_e27485: f64 = (locals.var_t4 * locals.var_t4);
        (assign20000_e27485, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20000_e27487;
        locals.var_t4_dn0 = assign20000_e27487_d_n0;
        locals.var_t4_dn2 = assign20000_e27487_d_n2;
        locals.var_t4_dn6 = assign20000_e27487_d_n6;
        locals.var_t4_dn7 = assign20000_e27487_d_n7;
        locals.var_t4_dn10 = assign20000_e27487_d_n10;
        locals.var_t4_dn11 = assign20000_e27487_d_n11;
        locals.var_t4_dn12 = assign20000_e27487_d_n12;
        locals.var_t4_dn17 = assign20000_e27487_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20010_e27508, assign20010_e27508_d_n0, assign20010_e27508_d_n2, assign20010_e27508_d_n6, assign20010_e27508_d_n7, assign20010_e27508_d_n10, assign20010_e27508_d_n11, assign20010_e27508_d_n12, assign20010_e27508_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20010_e27496: f64 = (4.0 * locals.var_t2);
        let assign20010_e27498: f64 = (assign20010_e27496 * locals.var_t2);
        let assign20010_e27500: f64 = (assign20010_e27498 * locals.var_t2);
        let assign20010_e27502: f64 = (assign20010_e27500 + locals.var_t4);
        let assign20010_e27503: f64 = (assign20010_e27502).sqrt();
        let assign20010_e27504: f64 = (locals.var_t3 + assign20010_e27503);
        let assign20010_e27506: f64 = (assign20010_e27504).powf(0.3333333333333333);
        (assign20010_e27506, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn0)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn0)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn2)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn2)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn6)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn6)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn7)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn7)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn10)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn10)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn11)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn11)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn12)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn12)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign20010_e27503))) / assign20010_e27504))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20010_e27504).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn17)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign20010_e27503))))) } } else { (assign20010_e27506 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign20010_e27496 * locals.var_t2_dn17)) * locals.var_t2) + (assign20010_e27498 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign20010_e27503))) / assign20010_e27504))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign20010_e27508;
        locals.var_t5_dn0 = assign20010_e27508_d_n0;
        locals.var_t5_dn2 = assign20010_e27508_d_n2;
        locals.var_t5_dn6 = assign20010_e27508_d_n6;
        locals.var_t5_dn7 = assign20010_e27508_d_n7;
        locals.var_t5_dn10 = assign20010_e27508_d_n10;
        locals.var_t5_dn11 = assign20010_e27508_d_n11;
        locals.var_t5_dn12 = assign20010_e27508_d_n12;
        locals.var_t5_dn17 = assign20010_e27508_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign20020_e27532, assign20020_e27532_d_n0, assign20020_e27532_d_n2, assign20020_e27532_d_n6, assign20020_e27532_d_n7, assign20020_e27532_d_n10, assign20020_e27532_d_n11, assign20020_e27532_d_n12, assign20020_e27532_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20020_e27517: f64 = (1.259921049894873 * locals.var_t2);
        let assign20020_e27520: f64 = (3.0 * locals.var_t5);
        let assign20020_e27521: f64 = (assign20020_e27517 / assign20020_e27520);
        let assign20020_e27522: f64 = (3.0 - assign20020_e27521);
        let assign20020_e27526: f64 = (3.0 * 1.259921049894873);
        let assign20020_e27527: f64 = (1.0 / assign20020_e27526);
        let assign20020_e27529: f64 = (assign20020_e27527 * locals.var_t5);
        let assign20020_e27530: f64 = (assign20020_e27522 + assign20020_e27529);
        (assign20020_e27530, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn0))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn2))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn6))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn7))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn10))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn11))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn12))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign20020_e27520) - (assign20020_e27517 * (3.0 * locals.var_t5_dn17))) / (assign20020_e27520 * assign20020_e27520))) + (assign20020_e27527 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20020_e27532;
        locals.var_tx_dn0 = assign20020_e27532_d_n0;
        locals.var_tx_dn2 = assign20020_e27532_d_n2;
        locals.var_tx_dn6 = assign20020_e27532_d_n6;
        locals.var_tx_dn7 = assign20020_e27532_d_n7;
        locals.var_tx_dn10 = assign20020_e27532_d_n10;
        locals.var_tx_dn11 = assign20020_e27532_d_n11;
        locals.var_tx_dn12 = assign20020_e27532_d_n12;
        locals.var_tx_dn17 = assign20020_e27532_d_n17;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20030_e27544, assign20030_e27544_d_n0, assign20030_e27544_d_n2, assign20030_e27544_d_n6, assign20030_e27544_d_n7, assign20030_e27544_d_n10, assign20030_e27544_d_n11, assign20030_e27544_d_n12, assign20030_e27544_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20030_e27540: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign20030_e27542: f64 = (assign20030_e27540 + locals.var_vbs__blk625);
        (assign20030_e27542, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbs__blk625_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbs__blk625_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbs__blk625_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbs__blk625_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbs__blk625_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbs__blk625_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbs__blk625_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbs__blk625_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20030_e27544;
        locals.var_ps0_inia_dn0 = assign20030_e27544_d_n0;
        locals.var_ps0_inia_dn2 = assign20030_e27544_d_n2;
        locals.var_ps0_inia_dn6 = assign20030_e27544_d_n6;
        locals.var_ps0_inia_dn7 = assign20030_e27544_d_n7;
        locals.var_ps0_inia_dn10 = assign20030_e27544_d_n10;
        locals.var_ps0_inia_dn11 = assign20030_e27544_d_n11;
        locals.var_ps0_inia_dn12 = assign20030_e27544_d_n12;
        locals.var_ps0_inia_dn17 = assign20030_e27544_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20040_e27552, assign20040_e27552_d_n0, assign20040_e27552_d_n2, assign20040_e27552_d_n6, assign20040_e27552_d_n7, assign20040_e27552_d_n10, assign20040_e27552_d_n11, assign20040_e27552_d_n12, assign20040_e27552_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20040_e27552;
        locals.var_ps0_ini_dn0 = assign20040_e27552_d_n0;
        locals.var_ps0_ini_dn2 = assign20040_e27552_d_n2;
        locals.var_ps0_ini_dn6 = assign20040_e27552_d_n6;
        locals.var_ps0_ini_dn7 = assign20040_e27552_d_n7;
        locals.var_ps0_ini_dn10 = assign20040_e27552_d_n10;
        locals.var_ps0_ini_dn11 = assign20040_e27552_d_n11;
        locals.var_ps0_ini_dn12 = assign20040_e27552_d_n12;
        locals.var_ps0_ini_dn17 = assign20040_e27552_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20050_e27555: f64 = (locals.var_vgs - locals.var_shift);
        let assign20050_e27557: f64 = if assign20050_e27555 <= locals.var_vth__blk626 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign20050_e27557;
        locals.var_guard630_rv = 0.0;

        let assign20060_e27560: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign20060_e27560;
        locals.var_guard631_rv = 0.0;

        let (assign20070_e27575, assign20070_e27575_d_n0, assign20070_e27575_d_n2, assign20070_e27575_d_n6, assign20070_e27575_d_n7, assign20070_e27575_d_n10, assign20070_e27575_d_n11, assign20070_e27575_d_n12, assign20070_e27575_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20070_e27573: f64 = (1.0 / locals.var_c_fox);
        (assign20070_e27573, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20070_e27575;
        locals.var_t0_dn0 = assign20070_e27575_d_n0;
        locals.var_t0_dn2 = assign20070_e27575_d_n2;
        locals.var_t0_dn6 = assign20070_e27575_d_n6;
        locals.var_t0_dn7 = assign20070_e27575_d_n7;
        locals.var_t0_dn10 = assign20070_e27575_d_n10;
        locals.var_t0_dn11 = assign20070_e27575_d_n11;
        locals.var_t0_dn12 = assign20070_e27575_d_n12;
        locals.var_t0_dn17 = assign20070_e27575_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20080_e27590, assign20080_e27590_d_n0, assign20080_e27590_d_n2, assign20080_e27590_d_n6, assign20080_e27590_d_n7, assign20080_e27590_d_n10, assign20080_e27590_d_n11, assign20080_e27590_d_n12, assign20080_e27590_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20080_e27588: f64 = (locals.var_t_soi__blk609 / 1.034943e-10);
        (assign20080_e27588, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20080_e27590;
        locals.var_t1_dn0 = assign20080_e27590_d_n0;
        locals.var_t1_dn2 = assign20080_e27590_d_n2;
        locals.var_t1_dn6 = assign20080_e27590_d_n6;
        locals.var_t1_dn7 = assign20080_e27590_d_n7;
        locals.var_t1_dn10 = assign20080_e27590_d_n10;
        locals.var_t1_dn11 = assign20080_e27590_d_n11;
        locals.var_t1_dn12 = assign20080_e27590_d_n12;
        locals.var_t1_dn17 = assign20080_e27590_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20090_e27605, assign20090_e27605_d_n0, assign20090_e27605_d_n2, assign20090_e27605_d_n6, assign20090_e27605_d_n7, assign20090_e27605_d_n10, assign20090_e27605_d_n11, assign20090_e27605_d_n12, assign20090_e27605_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20090_e27603: f64 = (1.0 / locals.var_c_box);
        (assign20090_e27603, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20090_e27605;
        locals.var_t2_dn0 = assign20090_e27605_d_n0;
        locals.var_t2_dn2 = assign20090_e27605_d_n2;
        locals.var_t2_dn6 = assign20090_e27605_d_n6;
        locals.var_t2_dn7 = assign20090_e27605_d_n7;
        locals.var_t2_dn10 = assign20090_e27605_d_n10;
        locals.var_t2_dn11 = assign20090_e27605_d_n11;
        locals.var_t2_dn12 = assign20090_e27605_d_n12;
        locals.var_t2_dn17 = assign20090_e27605_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20100_e27624, assign20100_e27624_d_n0, assign20100_e27624_d_n2, assign20100_e27624_d_n6, assign20100_e27624_d_n7, assign20100_e27624_d_n10, assign20100_e27624_d_n11, assign20100_e27624_d_n12, assign20100_e27624_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20100_e27619: f64 = (locals.var_t0 + locals.var_t1);
        let assign20100_e27621: f64 = (assign20100_e27619 + locals.var_t2);
        let assign20100_e27622: f64 = (1.0 / assign20100_e27621);
        (assign20100_e27622, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20100_e27621 * assign20100_e27621))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20100_e27621 * assign20100_e27621))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20100_e27624;
        locals.var_t3_dn0 = assign20100_e27624_d_n0;
        locals.var_t3_dn2 = assign20100_e27624_d_n2;
        locals.var_t3_dn6 = assign20100_e27624_d_n6;
        locals.var_t3_dn7 = assign20100_e27624_d_n7;
        locals.var_t3_dn10 = assign20100_e27624_d_n10;
        locals.var_t3_dn11 = assign20100_e27624_d_n11;
        locals.var_t3_dn12 = assign20100_e27624_d_n12;
        locals.var_t3_dn17 = assign20100_e27624_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20110_e27650, assign20110_e27650_d_n0, assign20110_e27650_d_n2, assign20110_e27650_d_n6, assign20110_e27650_d_n7, assign20110_e27650_d_n10, assign20110_e27650_d_n11, assign20110_e27650_d_n12, assign20110_e27650_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20110_e27638: f64 = (locals.var_vgp__blk610 - locals.var_vbsbiz);
        let assign20110_e27642: f64 = (0.5 * locals.var_t1);
        let assign20110_e27643: f64 = (locals.var_t2 + assign20110_e27642);
        let assign20110_e27645: f64 = (-locals.var_q_s0_dep_ini);
        let assign20110_e27646: f64 = (assign20110_e27643 * assign20110_e27645);
        let assign20110_e27647: f64 = (assign20110_e27638 + assign20110_e27646);
        let assign20110_e27648: f64 = (locals.var_t3 * assign20110_e27647);
        (assign20110_e27648, ((locals.var_t3_dn0 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20110_e27647) + (locals.var_t3 * ((locals.var_vgp__blk610_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20110_e27645) + (assign20110_e27643 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20110_e27650;
        locals.var_t4_dn0 = assign20110_e27650_d_n0;
        locals.var_t4_dn2 = assign20110_e27650_d_n2;
        locals.var_t4_dn6 = assign20110_e27650_d_n6;
        locals.var_t4_dn7 = assign20110_e27650_d_n7;
        locals.var_t4_dn10 = assign20110_e27650_d_n10;
        locals.var_t4_dn11 = assign20110_e27650_d_n11;
        locals.var_t4_dn12 = assign20110_e27650_d_n12;
        locals.var_t4_dn17 = assign20110_e27650_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20120_e27667, assign20120_e27667_d_n0, assign20120_e27667_d_n2, assign20120_e27667_d_n6, assign20120_e27667_d_n7, assign20120_e27667_d_n10, assign20120_e27667_d_n11, assign20120_e27667_d_n12, assign20120_e27667_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20120_e27664: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20120_e27665: f64 = (locals.var_vgp__blk610 - assign20120_e27664);
        (assign20120_e27665, (locals.var_vgp__blk610_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20120_e27667;
        locals.var_ps0_inia_dn0 = assign20120_e27667_d_n0;
        locals.var_ps0_inia_dn2 = assign20120_e27667_d_n2;
        locals.var_ps0_inia_dn6 = assign20120_e27667_d_n6;
        locals.var_ps0_inia_dn7 = assign20120_e27667_d_n7;
        locals.var_ps0_inia_dn10 = assign20120_e27667_d_n10;
        locals.var_ps0_inia_dn11 = assign20120_e27667_d_n11;
        locals.var_ps0_inia_dn12 = assign20120_e27667_d_n12;
        locals.var_ps0_inia_dn17 = assign20120_e27667_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20130_e27678, assign20130_e27678_d_n0, assign20130_e27678_d_n2, assign20130_e27678_d_n6, assign20130_e27678_d_n7, assign20130_e27678_d_n10, assign20130_e27678_d_n11, assign20130_e27678_d_n12, assign20130_e27678_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20130_e27678;
        locals.var_ps0_ini_dn0 = assign20130_e27678_d_n0;
        locals.var_ps0_ini_dn2 = assign20130_e27678_d_n2;
        locals.var_ps0_ini_dn6 = assign20130_e27678_d_n6;
        locals.var_ps0_ini_dn7 = assign20130_e27678_d_n7;
        locals.var_ps0_ini_dn10 = assign20130_e27678_d_n10;
        locals.var_ps0_ini_dn11 = assign20130_e27678_d_n11;
        locals.var_ps0_ini_dn12 = assign20130_e27678_d_n12;
        locals.var_ps0_ini_dn17 = assign20130_e27678_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20140_e27694, assign20140_e27694_d_n0, assign20140_e27694_d_n2, assign20140_e27694_d_n6, assign20140_e27694_d_n7, assign20140_e27694_d_n10, assign20140_e27694_d_n11, assign20140_e27694_d_n12, assign20140_e27694_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20140_e27690: f64 = (1.0 / locals.var_cnst1soi);
        let assign20140_e27692: f64 = (assign20140_e27690 / locals.var_cnstc_foxi);
        (assign20140_e27692, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20140_e27690 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20140_e27694;
        locals.var_t1_dn0 = assign20140_e27694_d_n0;
        locals.var_t1_dn2 = assign20140_e27694_d_n2;
        locals.var_t1_dn6 = assign20140_e27694_d_n6;
        locals.var_t1_dn7 = assign20140_e27694_d_n7;
        locals.var_t1_dn10 = assign20140_e27694_d_n10;
        locals.var_t1_dn11 = assign20140_e27694_d_n11;
        locals.var_t1_dn12 = assign20140_e27694_d_n12;
        locals.var_t1_dn17 = assign20140_e27694_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20150_e27714, assign20150_e27714_d_n0, assign20150_e27714_d_n2, assign20150_e27714_d_n6, assign20150_e27714_d_n7, assign20150_e27714_d_n10, assign20150_e27714_d_n11, assign20150_e27714_d_n12, assign20150_e27714_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20150_e27707: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20150_e27708: f64 = (locals.var_t1 * assign20150_e27707);
        let assign20150_e27711: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20150_e27712: f64 = (assign20150_e27708 * assign20150_e27711);
        (assign20150_e27712, ((((locals.var_t1_dn0 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn0 - locals.var_shift_dn0))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn2 - locals.var_shift_dn2))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn6 - locals.var_shift_dn6))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn7 - locals.var_shift_dn7))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn10 - locals.var_shift_dn10))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn11 - locals.var_shift_dn11))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn12 - locals.var_shift_dn12))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign20150_e27707) + (locals.var_t1 * (locals.var_vgp__blk610_dn17 - locals.var_shift_dn17))) * assign20150_e27711) + (assign20150_e27708 * (locals.var_vgp__blk610_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20150_e27714;
        locals.var_t2_dn0 = assign20150_e27714_d_n0;
        locals.var_t2_dn2 = assign20150_e27714_d_n2;
        locals.var_t2_dn6 = assign20150_e27714_d_n6;
        locals.var_t2_dn7 = assign20150_e27714_d_n7;
        locals.var_t2_dn10 = assign20150_e27714_d_n10;
        locals.var_t2_dn11 = assign20150_e27714_d_n11;
        locals.var_t2_dn12 = assign20150_e27714_d_n12;
        locals.var_t2_dn17 = assign20150_e27714_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20160_e27732, assign20160_e27732_d_n0, assign20160_e27732_d_n2, assign20160_e27732_d_n6, assign20160_e27732_d_n7, assign20160_e27732_d_n10, assign20160_e27732_d_n11, assign20160_e27732_d_n12, assign20160_e27732_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20160_e27728: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20160_e27729: f64 = (2.0 / assign20160_e27728);
        let assign20160_e27730: f64 = (locals.var_beta + assign20160_e27729);
        (assign20160_e27730, (-((2.0 * (locals.var_vgp__blk610_dn0 - locals.var_shift_dn0)) / (assign20160_e27728 * assign20160_e27728))), (-((2.0 * (locals.var_vgp__blk610_dn2 - locals.var_shift_dn2)) / (assign20160_e27728 * assign20160_e27728))), (-((2.0 * (locals.var_vgp__blk610_dn6 - locals.var_shift_dn6)) / (assign20160_e27728 * assign20160_e27728))), (-((2.0 * (locals.var_vgp__blk610_dn7 - locals.var_shift_dn7)) / (assign20160_e27728 * assign20160_e27728))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp__blk610_dn10 - locals.var_shift_dn10)) / (assign20160_e27728 * assign20160_e27728)))), (-((2.0 * (locals.var_vgp__blk610_dn11 - locals.var_shift_dn11)) / (assign20160_e27728 * assign20160_e27728))), (-((2.0 * (locals.var_vgp__blk610_dn12 - locals.var_shift_dn12)) / (assign20160_e27728 * assign20160_e27728))), (-((2.0 * (locals.var_vgp__blk610_dn17 - locals.var_shift_dn17)) / (assign20160_e27728 * assign20160_e27728))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20160_e27732;
        locals.var_t3_dn0 = assign20160_e27732_d_n0;
        locals.var_t3_dn2 = assign20160_e27732_d_n2;
        locals.var_t3_dn6 = assign20160_e27732_d_n6;
        locals.var_t3_dn7 = assign20160_e27732_d_n7;
        locals.var_t3_dn10 = assign20160_e27732_d_n10;
        locals.var_t3_dn11 = assign20160_e27732_d_n11;
        locals.var_t3_dn12 = assign20160_e27732_d_n12;
        locals.var_t3_dn17 = assign20160_e27732_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20170_e27749, assign20170_e27749_d_n0, assign20170_e27749_d_n2, assign20170_e27749_d_n6, assign20170_e27749_d_n7, assign20170_e27749_d_n10, assign20170_e27749_d_n11, assign20170_e27749_d_n12, assign20170_e27749_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20170_e27743: f64 = (locals.var_t2).ln();
        let assign20170_e27745: f64 = (assign20170_e27743 / locals.var_t3);
        let assign20170_e27747: f64 = (assign20170_e27745 + p.p287);
        (assign20170_e27747, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign20170_e27743 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign20170_e27749;
        locals.var_ps0_inib_dn0 = assign20170_e27749_d_n0;
        locals.var_ps0_inib_dn2 = assign20170_e27749_d_n2;
        locals.var_ps0_inib_dn6 = assign20170_e27749_d_n6;
        locals.var_ps0_inib_dn7 = assign20170_e27749_d_n7;
        locals.var_ps0_inib_dn10 = assign20170_e27749_d_n10;
        locals.var_ps0_inib_dn11 = assign20170_e27749_d_n11;
        locals.var_ps0_inib_dn12 = assign20170_e27749_d_n12;
        locals.var_ps0_inib_dn17 = assign20170_e27749_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign20180_e27765, assign20180_e27765_d_n0, assign20180_e27765_d_n2, assign20180_e27765_d_n6, assign20180_e27765_d_n7, assign20180_e27765_d_n10, assign20180_e27765_d_n11, assign20180_e27765_d_n12, assign20180_e27765_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20180_e27761: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign20180_e27763: f64 = (assign20180_e27761 - 0.0008);
        (assign20180_e27763, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20180_e27765;
        locals.var_tmf1_dn0 = assign20180_e27765_d_n0;
        locals.var_tmf1_dn2 = assign20180_e27765_d_n2;
        locals.var_tmf1_dn6 = assign20180_e27765_d_n6;
        locals.var_tmf1_dn7 = assign20180_e27765_d_n7;
        locals.var_tmf1_dn10 = assign20180_e27765_d_n10;
        locals.var_tmf1_dn11 = assign20180_e27765_d_n11;
        locals.var_tmf1_dn12 = assign20180_e27765_d_n12;
        locals.var_tmf1_dn17 = assign20180_e27765_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20190_e27781, assign20190_e27781_d_n0, assign20190_e27781_d_n2, assign20190_e27781_d_n6, assign20190_e27781_d_n7, assign20190_e27781_d_n10, assign20190_e27781_d_n11, assign20190_e27781_d_n12, assign20190_e27781_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20190_e27777: f64 = (4.0 * locals.var_ps0_inib);
        let assign20190_e27779: f64 = (assign20190_e27777 * 0.0008);
        (assign20190_e27779, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20190_e27781;
        locals.var_tmf2_dn0 = assign20190_e27781_d_n0;
        locals.var_tmf2_dn2 = assign20190_e27781_d_n2;
        locals.var_tmf2_dn6 = assign20190_e27781_d_n6;
        locals.var_tmf2_dn7 = assign20190_e27781_d_n7;
        locals.var_tmf2_dn10 = assign20190_e27781_d_n10;
        locals.var_tmf2_dn11 = assign20190_e27781_d_n11;
        locals.var_tmf2_dn12 = assign20190_e27781_d_n12;
        locals.var_tmf2_dn17 = assign20190_e27781_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20200_e27799, assign20200_e27799_d_n0, assign20200_e27799_d_n2, assign20200_e27799_d_n6, assign20200_e27799_d_n7, assign20200_e27799_d_n10, assign20200_e27799_d_n11, assign20200_e27799_d_n12, assign20200_e27799_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let (assign20200_e27797, assign20200_e27797_d_n0, assign20200_e27797_d_n2, assign20200_e27797_d_n6, assign20200_e27797_d_n7, assign20200_e27797_d_n10, assign20200_e27797_d_n11, assign20200_e27797_d_n12, assign20200_e27797_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign20200_e27796: f64 = (-locals.var_tmf2);
                (assign20200_e27796, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign20200_e27797, assign20200_e27797_d_n0, assign20200_e27797_d_n2, assign20200_e27797_d_n6, assign20200_e27797_d_n7, assign20200_e27797_d_n10, assign20200_e27797_d_n11, assign20200_e27797_d_n12, assign20200_e27797_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20200_e27799;
        locals.var_tmf2_dn0 = assign20200_e27799_d_n0;
        locals.var_tmf2_dn2 = assign20200_e27799_d_n2;
        locals.var_tmf2_dn6 = assign20200_e27799_d_n6;
        locals.var_tmf2_dn7 = assign20200_e27799_d_n7;
        locals.var_tmf2_dn10 = assign20200_e27799_d_n10;
        locals.var_tmf2_dn11 = assign20200_e27799_d_n11;
        locals.var_tmf2_dn12 = assign20200_e27799_d_n12;
        locals.var_tmf2_dn17 = assign20200_e27799_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20210_e27816, assign20210_e27816_d_n0, assign20210_e27816_d_n2, assign20210_e27816_d_n6, assign20210_e27816_d_n7, assign20210_e27816_d_n10, assign20210_e27816_d_n11, assign20210_e27816_d_n12, assign20210_e27816_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20210_e27811: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20210_e27813: f64 = (assign20210_e27811 + locals.var_tmf2);
        let assign20210_e27814: f64 = (assign20210_e27813).sqrt();
        (assign20210_e27814, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20210_e27814)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign20210_e27814)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20210_e27816;
        locals.var_tmf2_dn0 = assign20210_e27816_d_n0;
        locals.var_tmf2_dn2 = assign20210_e27816_d_n2;
        locals.var_tmf2_dn6 = assign20210_e27816_d_n6;
        locals.var_tmf2_dn7 = assign20210_e27816_d_n7;
        locals.var_tmf2_dn10 = assign20210_e27816_d_n10;
        locals.var_tmf2_dn11 = assign20210_e27816_d_n11;
        locals.var_tmf2_dn12 = assign20210_e27816_d_n12;
        locals.var_tmf2_dn17 = assign20210_e27816_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20220_e27834, assign20220_e27834_d_n0, assign20220_e27834_d_n2, assign20220_e27834_d_n6, assign20220_e27834_d_n7, assign20220_e27834_d_n10, assign20220_e27834_d_n11, assign20220_e27834_d_n12, assign20220_e27834_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign20220_e27830: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20220_e27831: f64 = (0.5 * assign20220_e27830);
        let assign20220_e27832: f64 = (locals.var_ps0_inib - assign20220_e27831);
        (assign20220_e27832, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20220_e27834;
        locals.var_ps0_ini_dn0 = assign20220_e27834_d_n0;
        locals.var_ps0_ini_dn2 = assign20220_e27834_d_n2;
        locals.var_ps0_ini_dn6 = assign20220_e27834_d_n6;
        locals.var_ps0_ini_dn7 = assign20220_e27834_d_n7;
        locals.var_ps0_ini_dn10 = assign20220_e27834_d_n10;
        locals.var_ps0_ini_dn11 = assign20220_e27834_d_n11;
        locals.var_ps0_ini_dn12 = assign20220_e27834_d_n12;
        locals.var_ps0_ini_dn17 = assign20220_e27834_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20230_e27837: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign20230_e27837;
        locals.var_guard632_rv = 0.0;

        let assign20240_e27840: f64 = (locals.var_vgs - locals.var_shift);
        let assign20240_e27842: f64 = if assign20240_e27840 <= locals.var_vth__blk626 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign20240_e27842;
        locals.var_guard633_rv = 0.0;

        let (assign20250_e27854, assign20250_e27854_d_n0, assign20250_e27854_d_n2, assign20250_e27854_d_n6, assign20250_e27854_d_n7, assign20250_e27854_d_n10, assign20250_e27854_d_n11, assign20250_e27854_d_n12, assign20250_e27854_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20250_e27852: f64 = (1.0 / locals.var_c_fox);
        (assign20250_e27852, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20250_e27854;
        locals.var_t0_dn0 = assign20250_e27854_d_n0;
        locals.var_t0_dn2 = assign20250_e27854_d_n2;
        locals.var_t0_dn6 = assign20250_e27854_d_n6;
        locals.var_t0_dn7 = assign20250_e27854_d_n7;
        locals.var_t0_dn10 = assign20250_e27854_d_n10;
        locals.var_t0_dn11 = assign20250_e27854_d_n11;
        locals.var_t0_dn12 = assign20250_e27854_d_n12;
        locals.var_t0_dn17 = assign20250_e27854_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20260_e27866, assign20260_e27866_d_n0, assign20260_e27866_d_n2, assign20260_e27866_d_n6, assign20260_e27866_d_n7, assign20260_e27866_d_n10, assign20260_e27866_d_n11, assign20260_e27866_d_n12, assign20260_e27866_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20260_e27864: f64 = (locals.var_t_soi__blk609 / 1.034943e-10);
        (assign20260_e27864, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20260_e27866;
        locals.var_t1_dn0 = assign20260_e27866_d_n0;
        locals.var_t1_dn2 = assign20260_e27866_d_n2;
        locals.var_t1_dn6 = assign20260_e27866_d_n6;
        locals.var_t1_dn7 = assign20260_e27866_d_n7;
        locals.var_t1_dn10 = assign20260_e27866_d_n10;
        locals.var_t1_dn11 = assign20260_e27866_d_n11;
        locals.var_t1_dn12 = assign20260_e27866_d_n12;
        locals.var_t1_dn17 = assign20260_e27866_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20270_e27878, assign20270_e27878_d_n0, assign20270_e27878_d_n2, assign20270_e27878_d_n6, assign20270_e27878_d_n7, assign20270_e27878_d_n10, assign20270_e27878_d_n11, assign20270_e27878_d_n12, assign20270_e27878_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20270_e27876: f64 = (1.0 / locals.var_c_box);
        (assign20270_e27876, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20270_e27878;
        locals.var_t2_dn0 = assign20270_e27878_d_n0;
        locals.var_t2_dn2 = assign20270_e27878_d_n2;
        locals.var_t2_dn6 = assign20270_e27878_d_n6;
        locals.var_t2_dn7 = assign20270_e27878_d_n7;
        locals.var_t2_dn10 = assign20270_e27878_d_n10;
        locals.var_t2_dn11 = assign20270_e27878_d_n11;
        locals.var_t2_dn12 = assign20270_e27878_d_n12;
        locals.var_t2_dn17 = assign20270_e27878_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20280_e27894, assign20280_e27894_d_n0, assign20280_e27894_d_n2, assign20280_e27894_d_n6, assign20280_e27894_d_n7, assign20280_e27894_d_n10, assign20280_e27894_d_n11, assign20280_e27894_d_n12, assign20280_e27894_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20280_e27889: f64 = (locals.var_t0 + locals.var_t1);
        let assign20280_e27891: f64 = (assign20280_e27889 + locals.var_t2);
        let assign20280_e27892: f64 = (1.0 / assign20280_e27891);
        (assign20280_e27892, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20280_e27891 * assign20280_e27891))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20280_e27891 * assign20280_e27891))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20280_e27894;
        locals.var_t3_dn0 = assign20280_e27894_d_n0;
        locals.var_t3_dn2 = assign20280_e27894_d_n2;
        locals.var_t3_dn6 = assign20280_e27894_d_n6;
        locals.var_t3_dn7 = assign20280_e27894_d_n7;
        locals.var_t3_dn10 = assign20280_e27894_d_n10;
        locals.var_t3_dn11 = assign20280_e27894_d_n11;
        locals.var_t3_dn12 = assign20280_e27894_d_n12;
        locals.var_t3_dn17 = assign20280_e27894_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20290_e27917, assign20290_e27917_d_n0, assign20290_e27917_d_n2, assign20290_e27917_d_n6, assign20290_e27917_d_n7, assign20290_e27917_d_n10, assign20290_e27917_d_n11, assign20290_e27917_d_n12, assign20290_e27917_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20290_e27905: f64 = (locals.var_vgp__blk610 - locals.var_vbsbiz);
        let assign20290_e27909: f64 = (0.5 * locals.var_t1);
        let assign20290_e27910: f64 = (locals.var_t2 + assign20290_e27909);
        let assign20290_e27912: f64 = (-locals.var_q_s0_dep_ini);
        let assign20290_e27913: f64 = (assign20290_e27910 * assign20290_e27912);
        let assign20290_e27914: f64 = (assign20290_e27905 + assign20290_e27913);
        let assign20290_e27915: f64 = (locals.var_t3 * assign20290_e27914);
        (assign20290_e27915, ((locals.var_t3_dn0 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20290_e27914) + (locals.var_t3 * ((locals.var_vgp__blk610_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20290_e27912) + (assign20290_e27910 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20290_e27917;
        locals.var_t4_dn0 = assign20290_e27917_d_n0;
        locals.var_t4_dn2 = assign20290_e27917_d_n2;
        locals.var_t4_dn6 = assign20290_e27917_d_n6;
        locals.var_t4_dn7 = assign20290_e27917_d_n7;
        locals.var_t4_dn10 = assign20290_e27917_d_n10;
        locals.var_t4_dn11 = assign20290_e27917_d_n11;
        locals.var_t4_dn12 = assign20290_e27917_d_n12;
        locals.var_t4_dn17 = assign20290_e27917_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20300_e27931, assign20300_e27931_d_n0, assign20300_e27931_d_n2, assign20300_e27931_d_n6, assign20300_e27931_d_n7, assign20300_e27931_d_n10, assign20300_e27931_d_n11, assign20300_e27931_d_n12, assign20300_e27931_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20300_e27928: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20300_e27929: f64 = (locals.var_vgp__blk610 - assign20300_e27928);
        (assign20300_e27929, (locals.var_vgp__blk610_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20300_e27931;
        locals.var_ps0_inia_dn0 = assign20300_e27931_d_n0;
        locals.var_ps0_inia_dn2 = assign20300_e27931_d_n2;
        locals.var_ps0_inia_dn6 = assign20300_e27931_d_n6;
        locals.var_ps0_inia_dn7 = assign20300_e27931_d_n7;
        locals.var_ps0_inia_dn10 = assign20300_e27931_d_n10;
        locals.var_ps0_inia_dn11 = assign20300_e27931_d_n11;
        locals.var_ps0_inia_dn12 = assign20300_e27931_d_n12;
        locals.var_ps0_inia_dn17 = assign20300_e27931_d_n17;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20310_e27941, assign20310_e27941_d_n0, assign20310_e27941_d_n2, assign20310_e27941_d_n6, assign20310_e27941_d_n7, assign20310_e27941_d_n10, assign20310_e27941_d_n11, assign20310_e27941_d_n12, assign20310_e27941_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20310_e27941;
        locals.var_ps0_ini_dn0 = assign20310_e27941_d_n0;
        locals.var_ps0_ini_dn2 = assign20310_e27941_d_n2;
        locals.var_ps0_ini_dn6 = assign20310_e27941_d_n6;
        locals.var_ps0_ini_dn7 = assign20310_e27941_d_n7;
        locals.var_ps0_ini_dn10 = assign20310_e27941_d_n10;
        locals.var_ps0_ini_dn11 = assign20310_e27941_d_n11;
        locals.var_ps0_ini_dn12 = assign20310_e27941_d_n12;
        locals.var_ps0_ini_dn17 = assign20310_e27941_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20320_e27954, assign20320_e27954_d_n0, assign20320_e27954_d_n2, assign20320_e27954_d_n6, assign20320_e27954_d_n7, assign20320_e27954_d_n10, assign20320_e27954_d_n11, assign20320_e27954_d_n12, assign20320_e27954_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        let assign20320_e27952: f64 = (1.0 / locals.var_c_fox);
        (assign20320_e27952, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20320_e27954;
        locals.var_t0_dn0 = assign20320_e27954_d_n0;
        locals.var_t0_dn2 = assign20320_e27954_d_n2;
        locals.var_t0_dn6 = assign20320_e27954_d_n6;
        locals.var_t0_dn7 = assign20320_e27954_d_n7;
        locals.var_t0_dn10 = assign20320_e27954_d_n10;
        locals.var_t0_dn11 = assign20320_e27954_d_n11;
        locals.var_t0_dn12 = assign20320_e27954_d_n12;
        locals.var_t0_dn17 = assign20320_e27954_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20330_e27967, assign20330_e27967_d_n0, assign20330_e27967_d_n2, assign20330_e27967_d_n6, assign20330_e27967_d_n7, assign20330_e27967_d_n10, assign20330_e27967_d_n11, assign20330_e27967_d_n12, assign20330_e27967_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        let assign20330_e27965: f64 = (locals.var_t_soi__blk609 / 1.034943e-10);
        (assign20330_e27965, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20330_e27967;
        locals.var_t1_dn0 = assign20330_e27967_d_n0;
        locals.var_t1_dn2 = assign20330_e27967_d_n2;
        locals.var_t1_dn6 = assign20330_e27967_d_n6;
        locals.var_t1_dn7 = assign20330_e27967_d_n7;
        locals.var_t1_dn10 = assign20330_e27967_d_n10;
        locals.var_t1_dn11 = assign20330_e27967_d_n11;
        locals.var_t1_dn12 = assign20330_e27967_d_n12;
        locals.var_t1_dn17 = assign20330_e27967_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20340_e27980, assign20340_e27980_d_n0, assign20340_e27980_d_n2, assign20340_e27980_d_n6, assign20340_e27980_d_n7, assign20340_e27980_d_n10, assign20340_e27980_d_n11, assign20340_e27980_d_n12, assign20340_e27980_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        let assign20340_e27978: f64 = (1.0 / locals.var_c_box);
        (assign20340_e27978, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20340_e27980;
        locals.var_t2_dn0 = assign20340_e27980_d_n0;
        locals.var_t2_dn2 = assign20340_e27980_d_n2;
        locals.var_t2_dn6 = assign20340_e27980_d_n6;
        locals.var_t2_dn7 = assign20340_e27980_d_n7;
        locals.var_t2_dn10 = assign20340_e27980_d_n10;
        locals.var_t2_dn11 = assign20340_e27980_d_n11;
        locals.var_t2_dn12 = assign20340_e27980_d_n12;
        locals.var_t2_dn17 = assign20340_e27980_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20350_e27997, assign20350_e27997_d_n0, assign20350_e27997_d_n2, assign20350_e27997_d_n6, assign20350_e27997_d_n7, assign20350_e27997_d_n10, assign20350_e27997_d_n11, assign20350_e27997_d_n12, assign20350_e27997_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        let assign20350_e27992: f64 = (locals.var_t0 + locals.var_t1);
        let assign20350_e27994: f64 = (assign20350_e27992 + locals.var_t2);
        let assign20350_e27995: f64 = (1.0 / assign20350_e27994);
        (assign20350_e27995, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20350_e27994 * assign20350_e27994))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20350_e27994 * assign20350_e27994))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20350_e27997;
        locals.var_t3_dn0 = assign20350_e27997_d_n0;
        locals.var_t3_dn2 = assign20350_e27997_d_n2;
        locals.var_t3_dn6 = assign20350_e27997_d_n6;
        locals.var_t3_dn7 = assign20350_e27997_d_n7;
        locals.var_t3_dn10 = assign20350_e27997_d_n10;
        locals.var_t3_dn11 = assign20350_e27997_d_n11;
        locals.var_t3_dn12 = assign20350_e27997_d_n12;
        locals.var_t3_dn17 = assign20350_e27997_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20360_e28021, assign20360_e28021_d_n0, assign20360_e28021_d_n2, assign20360_e28021_d_n6, assign20360_e28021_d_n7, assign20360_e28021_d_n10, assign20360_e28021_d_n11, assign20360_e28021_d_n12, assign20360_e28021_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        let assign20360_e28009: f64 = (locals.var_vgp__blk610 - locals.var_vbsbiz);
        let assign20360_e28013: f64 = (0.5 * locals.var_t1);
        let assign20360_e28014: f64 = (locals.var_t2 + assign20360_e28013);
        let assign20360_e28016: f64 = (-locals.var_q_s0_dep_ini);
        let assign20360_e28017: f64 = (assign20360_e28014 * assign20360_e28016);
        let assign20360_e28018: f64 = (assign20360_e28009 + assign20360_e28017);
        let assign20360_e28019: f64 = (locals.var_t3 * assign20360_e28018);
        (assign20360_e28019, ((locals.var_t3_dn0 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20360_e28018) + (locals.var_t3 * ((locals.var_vgp__blk610_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20360_e28016) + (assign20360_e28014 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20360_e28021;
        locals.var_t4_dn0 = assign20360_e28021_d_n0;
        locals.var_t4_dn2 = assign20360_e28021_d_n2;
        locals.var_t4_dn6 = assign20360_e28021_d_n6;
        locals.var_t4_dn7 = assign20360_e28021_d_n7;
        locals.var_t4_dn10 = assign20360_e28021_d_n10;
        locals.var_t4_dn11 = assign20360_e28021_d_n11;
        locals.var_t4_dn12 = assign20360_e28021_d_n12;
        locals.var_t4_dn17 = assign20360_e28021_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20370_e28036, assign20370_e28036_d_n0, assign20370_e28036_d_n2, assign20370_e28036_d_n6, assign20370_e28036_d_n7, assign20370_e28036_d_n10, assign20370_e28036_d_n11, assign20370_e28036_d_n12, assign20370_e28036_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        let assign20370_e28033: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20370_e28034: f64 = (locals.var_vgp__blk610 - assign20370_e28033);
        (assign20370_e28034, (locals.var_vgp__blk610_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk610_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20370_e28036;
        locals.var_ps0_inia_dn0 = assign20370_e28036_d_n0;
        locals.var_ps0_inia_dn2 = assign20370_e28036_d_n2;
        locals.var_ps0_inia_dn6 = assign20370_e28036_d_n6;
        locals.var_ps0_inia_dn7 = assign20370_e28036_d_n7;
        locals.var_ps0_inia_dn10 = assign20370_e28036_d_n10;
        locals.var_ps0_inia_dn11 = assign20370_e28036_d_n11;
        locals.var_ps0_inia_dn12 = assign20370_e28036_d_n12;
        locals.var_ps0_inia_dn17 = assign20370_e28036_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20380_e28047, assign20380_e28047_d_n0, assign20380_e28047_d_n2, assign20380_e28047_d_n6, assign20380_e28047_d_n7, assign20380_e28047_d_n10, assign20380_e28047_d_n11, assign20380_e28047_d_n12, assign20380_e28047_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20380_e28047;
        locals.var_ps0_ini_dn0 = assign20380_e28047_d_n0;
        locals.var_ps0_ini_dn2 = assign20380_e28047_d_n2;
        locals.var_ps0_ini_dn6 = assign20380_e28047_d_n6;
        locals.var_ps0_ini_dn7 = assign20380_e28047_d_n7;
        locals.var_ps0_ini_dn10 = assign20380_e28047_d_n10;
        locals.var_ps0_ini_dn11 = assign20380_e28047_d_n11;
        locals.var_ps0_ini_dn12 = assign20380_e28047_d_n12;
        locals.var_ps0_ini_dn17 = assign20380_e28047_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20390_e28050: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20390_e28052: f64 = if assign20390_e28050 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign20390_e28052;
        locals.var_guard634_rv = 0.0;

        let (assign20400_e28069, assign20400_e28069_d_n0, assign20400_e28069_d_n2, assign20400_e28069_d_n6, assign20400_e28069_d_n7, assign20400_e28069_d_n10, assign20400_e28069_d_n11, assign20400_e28069_d_n12, assign20400_e28069_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign20400_e28065: f64 = (1.0 / locals.var_cnst1soi);
        let assign20400_e28067: f64 = (assign20400_e28065 / locals.var_cnstc_foxi);
        (assign20400_e28067, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20400_e28065 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20400_e28069;
        locals.var_t1_dn0 = assign20400_e28069_d_n0;
        locals.var_t1_dn2 = assign20400_e28069_d_n2;
        locals.var_t1_dn6 = assign20400_e28069_d_n6;
        locals.var_t1_dn7 = assign20400_e28069_d_n7;
        locals.var_t1_dn10 = assign20400_e28069_d_n10;
        locals.var_t1_dn11 = assign20400_e28069_d_n11;
        locals.var_t1_dn12 = assign20400_e28069_d_n12;
        locals.var_t1_dn17 = assign20400_e28069_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20410_e28090, assign20410_e28090_d_n0, assign20410_e28090_d_n2, assign20410_e28090_d_n6, assign20410_e28090_d_n7, assign20410_e28090_d_n10, assign20410_e28090_d_n11, assign20410_e28090_d_n12, assign20410_e28090_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign20410_e28083: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20410_e28084: f64 = (locals.var_t1 * assign20410_e28083);
        let assign20410_e28087: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20410_e28088: f64 = (assign20410_e28084 * assign20410_e28087);
        (assign20410_e28088, ((((locals.var_t1_dn0 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn0 - locals.var_shift_dn0))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn2 - locals.var_shift_dn2))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn6 - locals.var_shift_dn6))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn7 - locals.var_shift_dn7))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn10 - locals.var_shift_dn10))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn11 - locals.var_shift_dn11))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn12 - locals.var_shift_dn12))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign20410_e28083) + (locals.var_t1 * (locals.var_vgp__blk610_dn17 - locals.var_shift_dn17))) * assign20410_e28087) + (assign20410_e28084 * (locals.var_vgp__blk610_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20410_e28090;
        locals.var_t2_dn0 = assign20410_e28090_d_n0;
        locals.var_t2_dn2 = assign20410_e28090_d_n2;
        locals.var_t2_dn6 = assign20410_e28090_d_n6;
        locals.var_t2_dn7 = assign20410_e28090_d_n7;
        locals.var_t2_dn10 = assign20410_e28090_d_n10;
        locals.var_t2_dn11 = assign20410_e28090_d_n11;
        locals.var_t2_dn12 = assign20410_e28090_d_n12;
        locals.var_t2_dn17 = assign20410_e28090_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20420_e28109, assign20420_e28109_d_n0, assign20420_e28109_d_n2, assign20420_e28109_d_n6, assign20420_e28109_d_n7, assign20420_e28109_d_n10, assign20420_e28109_d_n11, assign20420_e28109_d_n12, assign20420_e28109_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign20420_e28105: f64 = (locals.var_vgp__blk610 - locals.var_shift);
        let assign20420_e28106: f64 = (2.0 / assign20420_e28105);
        let assign20420_e28107: f64 = (locals.var_beta + assign20420_e28106);
        (assign20420_e28107, (-((2.0 * (locals.var_vgp__blk610_dn0 - locals.var_shift_dn0)) / (assign20420_e28105 * assign20420_e28105))), (-((2.0 * (locals.var_vgp__blk610_dn2 - locals.var_shift_dn2)) / (assign20420_e28105 * assign20420_e28105))), (-((2.0 * (locals.var_vgp__blk610_dn6 - locals.var_shift_dn6)) / (assign20420_e28105 * assign20420_e28105))), (-((2.0 * (locals.var_vgp__blk610_dn7 - locals.var_shift_dn7)) / (assign20420_e28105 * assign20420_e28105))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp__blk610_dn10 - locals.var_shift_dn10)) / (assign20420_e28105 * assign20420_e28105)))), (-((2.0 * (locals.var_vgp__blk610_dn11 - locals.var_shift_dn11)) / (assign20420_e28105 * assign20420_e28105))), (-((2.0 * (locals.var_vgp__blk610_dn12 - locals.var_shift_dn12)) / (assign20420_e28105 * assign20420_e28105))), (-((2.0 * (locals.var_vgp__blk610_dn17 - locals.var_shift_dn17)) / (assign20420_e28105 * assign20420_e28105))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20420_e28109;
        locals.var_t3_dn0 = assign20420_e28109_d_n0;
        locals.var_t3_dn2 = assign20420_e28109_d_n2;
        locals.var_t3_dn6 = assign20420_e28109_d_n6;
        locals.var_t3_dn7 = assign20420_e28109_d_n7;
        locals.var_t3_dn10 = assign20420_e28109_d_n10;
        locals.var_t3_dn11 = assign20420_e28109_d_n11;
        locals.var_t3_dn12 = assign20420_e28109_d_n12;
        locals.var_t3_dn17 = assign20420_e28109_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20430_e28127, assign20430_e28127_d_n0, assign20430_e28127_d_n2, assign20430_e28127_d_n6, assign20430_e28127_d_n7, assign20430_e28127_d_n10, assign20430_e28127_d_n11, assign20430_e28127_d_n12, assign20430_e28127_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign20430_e28121: f64 = (locals.var_t2).ln();
        let assign20430_e28123: f64 = (assign20430_e28121 / locals.var_t3);
        let assign20430_e28125: f64 = (assign20430_e28123 + p.p287);
        (assign20430_e28125, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign20430_e28121 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign20430_e28127;
        locals.var_ps0_inib_dn0 = assign20430_e28127_d_n0;
        locals.var_ps0_inib_dn2 = assign20430_e28127_d_n2;
        locals.var_ps0_inib_dn6 = assign20430_e28127_d_n6;
        locals.var_ps0_inib_dn7 = assign20430_e28127_d_n7;
        locals.var_ps0_inib_dn10 = assign20430_e28127_d_n10;
        locals.var_ps0_inib_dn11 = assign20430_e28127_d_n11;
        locals.var_ps0_inib_dn12 = assign20430_e28127_d_n12;
        locals.var_ps0_inib_dn17 = assign20430_e28127_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let assign20440_e28131: f64 = (locals.var_ps0_inib * 0.98);
        let assign20440_e28133: f64 = (assign20440_e28131 - 0.4);
        let assign20440_e28138: f64 = if ((locals.var_ps0_inia > assign20440_e28133) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard635 = assign20440_e28138;
        locals.var_guard635_rv = 0.0;

        let (assign20450_e28159, assign20450_e28159_d_n0, assign20450_e28159_d_n2, assign20450_e28159_d_n6, assign20450_e28159_d_n7, assign20450_e28159_d_n10, assign20450_e28159_d_n11, assign20450_e28159_d_n12, assign20450_e28159_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20450_e28154: f64 = (locals.var_ps0_inib * 0.98);
        let assign20450_e28155: f64 = (locals.var_ps0_inia - assign20450_e28154);
        let assign20450_e28157: f64 = (assign20450_e28155 + 0.4);
        (assign20450_e28157, (locals.var_ps0_inia_dn0 - (locals.var_ps0_inib_dn0 * 0.98)), (locals.var_ps0_inia_dn2 - (locals.var_ps0_inib_dn2 * 0.98)), (locals.var_ps0_inia_dn6 - (locals.var_ps0_inib_dn6 * 0.98)), (locals.var_ps0_inia_dn7 - (locals.var_ps0_inib_dn7 * 0.98)), (locals.var_ps0_inia_dn10 - (locals.var_ps0_inib_dn10 * 0.98)), (locals.var_ps0_inia_dn11 - (locals.var_ps0_inib_dn11 * 0.98)), (locals.var_ps0_inia_dn12 - (locals.var_ps0_inib_dn12 * 0.98)), (locals.var_ps0_inia_dn17 - (locals.var_ps0_inib_dn17 * 0.98)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20450_e28159;
        locals.var_tmf1_dn0 = assign20450_e28159_d_n0;
        locals.var_tmf1_dn2 = assign20450_e28159_d_n2;
        locals.var_tmf1_dn6 = assign20450_e28159_d_n6;
        locals.var_tmf1_dn7 = assign20450_e28159_d_n7;
        locals.var_tmf1_dn10 = assign20450_e28159_d_n10;
        locals.var_tmf1_dn11 = assign20450_e28159_d_n11;
        locals.var_tmf1_dn12 = assign20450_e28159_d_n12;
        locals.var_tmf1_dn17 = assign20450_e28159_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20460_e28176, assign20460_e28176_d_n0, assign20460_e28176_d_n2, assign20460_e28176_d_n6, assign20460_e28176_d_n7, assign20460_e28176_d_n10, assign20460_e28176_d_n11, assign20460_e28176_d_n12, assign20460_e28176_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20460_e28174: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign20460_e28174, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign20460_e28176;
        locals.var_x2_dn0 = assign20460_e28176_d_n0;
        locals.var_x2_dn2 = assign20460_e28176_d_n2;
        locals.var_x2_dn6 = assign20460_e28176_d_n6;
        locals.var_x2_dn7 = assign20460_e28176_d_n7;
        locals.var_x2_dn10 = assign20460_e28176_d_n10;
        locals.var_x2_dn11 = assign20460_e28176_d_n11;
        locals.var_x2_dn12 = assign20460_e28176_d_n12;
        locals.var_x2_dn17 = assign20460_e28176_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign20470_e28193, assign20470_e28193_d_n0, assign20470_e28193_d_n2, assign20470_e28193_d_n6, assign20470_e28193_d_n7, assign20470_e28193_d_n10, assign20470_e28193_d_n11, assign20470_e28193_d_n12, assign20470_e28193_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20470_e28191: f64 = (0.4 * 0.4);
        (assign20470_e28191, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign20470_e28193;
        locals.var_xmax2_dn0 = assign20470_e28193_d_n0;
        locals.var_xmax2_dn2 = assign20470_e28193_d_n2;
        locals.var_xmax2_dn6 = assign20470_e28193_d_n6;
        locals.var_xmax2_dn7 = assign20470_e28193_d_n7;
        locals.var_xmax2_dn10 = assign20470_e28193_d_n10;
        locals.var_xmax2_dn11 = assign20470_e28193_d_n11;
        locals.var_xmax2_dn12 = assign20470_e28193_d_n12;
        locals.var_xmax2_dn17 = assign20470_e28193_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign20480_e28208, assign20480_e28208_d_n0, assign20480_e28208_d_n2, assign20480_e28208_d_n6, assign20480_e28208_d_n7, assign20480_e28208_d_n10, assign20480_e28208_d_n11, assign20480_e28208_d_n12, assign20480_e28208_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20480_e28208;
        locals.var_xp_dn0 = assign20480_e28208_d_n0;
        locals.var_xp_dn2 = assign20480_e28208_d_n2;
        locals.var_xp_dn6 = assign20480_e28208_d_n6;
        locals.var_xp_dn7 = assign20480_e28208_d_n7;
        locals.var_xp_dn10 = assign20480_e28208_d_n10;
        locals.var_xp_dn11 = assign20480_e28208_d_n11;
        locals.var_xp_dn12 = assign20480_e28208_d_n12;
        locals.var_xp_dn17 = assign20480_e28208_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20490_e28223, assign20490_e28223_d_n0, assign20490_e28223_d_n2, assign20490_e28223_d_n6, assign20490_e28223_d_n7, assign20490_e28223_d_n10, assign20490_e28223_d_n11, assign20490_e28223_d_n12, assign20490_e28223_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20490_e28223;
        locals.var_xmp_dn0 = assign20490_e28223_d_n0;
        locals.var_xmp_dn2 = assign20490_e28223_d_n2;
        locals.var_xmp_dn6 = assign20490_e28223_d_n6;
        locals.var_xmp_dn7 = assign20490_e28223_d_n7;
        locals.var_xmp_dn10 = assign20490_e28223_d_n10;
        locals.var_xmp_dn11 = assign20490_e28223_d_n11;
        locals.var_xmp_dn12 = assign20490_e28223_d_n12;
        locals.var_xmp_dn17 = assign20490_e28223_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20500_e28238,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign20500_e28238;
        locals.var_m0_rv = 0.0;

        let (assign20510_e28253,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20510_e28253;
        locals.var_mm_rv = 0.0;

        let (assign20520_e28268, assign20520_e28268_d_n0, assign20520_e28268_d_n2, assign20520_e28268_d_n6, assign20520_e28268_d_n7, assign20520_e28268_d_n10, assign20520_e28268_d_n11, assign20520_e28268_d_n12, assign20520_e28268_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign20520_e28268;
        locals.var_arg_dn0 = assign20520_e28268_d_n0;
        locals.var_arg_dn2 = assign20520_e28268_d_n2;
        locals.var_arg_dn6 = assign20520_e28268_d_n6;
        locals.var_arg_dn7 = assign20520_e28268_d_n7;
        locals.var_arg_dn10 = assign20520_e28268_d_n10;
        locals.var_arg_dn11 = assign20520_e28268_d_n11;
        locals.var_arg_dn12 = assign20520_e28268_d_n12;
        locals.var_arg_dn17 = assign20520_e28268_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign20530_e28283, assign20530_e28283_d_n0, assign20530_e28283_d_n2, assign20530_e28283_d_n6, assign20530_e28283_d_n7, assign20530_e28283_d_n10, assign20530_e28283_d_n11, assign20530_e28283_d_n12, assign20530_e28283_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20530_e28283;
        locals.var_dnm_dn0 = assign20530_e28283_d_n0;
        locals.var_dnm_dn2 = assign20530_e28283_d_n2;
        locals.var_dnm_dn6 = assign20530_e28283_d_n6;
        locals.var_dnm_dn7 = assign20530_e28283_d_n7;
        locals.var_dnm_dn10 = assign20530_e28283_d_n10;
        locals.var_dnm_dn11 = assign20530_e28283_d_n11;
        locals.var_dnm_dn12 = assign20530_e28283_d_n12;
        locals.var_dnm_dn17 = assign20530_e28283_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20540_e28300, assign20540_e28300_d_n0, assign20540_e28300_d_n2, assign20540_e28300_d_n6, assign20540_e28300_d_n7, assign20540_e28300_d_n10, assign20540_e28300_d_n11, assign20540_e28300_d_n12, assign20540_e28300_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20540_e28298: f64 = (locals.var_xp * locals.var_x2);
        (assign20540_e28298, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20540_e28300;
        locals.var_xp_dn0 = assign20540_e28300_d_n0;
        locals.var_xp_dn2 = assign20540_e28300_d_n2;
        locals.var_xp_dn6 = assign20540_e28300_d_n6;
        locals.var_xp_dn7 = assign20540_e28300_d_n7;
        locals.var_xp_dn10 = assign20540_e28300_d_n10;
        locals.var_xp_dn11 = assign20540_e28300_d_n11;
        locals.var_xp_dn12 = assign20540_e28300_d_n12;
        locals.var_xp_dn17 = assign20540_e28300_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20550_e28317, assign20550_e28317_d_n0, assign20550_e28317_d_n2, assign20550_e28317_d_n6, assign20550_e28317_d_n7, assign20550_e28317_d_n10, assign20550_e28317_d_n11, assign20550_e28317_d_n12, assign20550_e28317_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20550_e28315: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign20550_e28315, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20550_e28317;
        locals.var_xmp_dn0 = assign20550_e28317_d_n0;
        locals.var_xmp_dn2 = assign20550_e28317_d_n2;
        locals.var_xmp_dn6 = assign20550_e28317_d_n6;
        locals.var_xmp_dn7 = assign20550_e28317_d_n7;
        locals.var_xmp_dn10 = assign20550_e28317_d_n10;
        locals.var_xmp_dn11 = assign20550_e28317_d_n11;
        locals.var_xmp_dn12 = assign20550_e28317_d_n12;
        locals.var_xmp_dn17 = assign20550_e28317_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20560_e28334, assign20560_e28334_d_n0, assign20560_e28334_d_n2, assign20560_e28334_d_n6, assign20560_e28334_d_n7, assign20560_e28334_d_n10, assign20560_e28334_d_n11, assign20560_e28334_d_n12, assign20560_e28334_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20560_e28332: f64 = (locals.var_xp * locals.var_x2);
        (assign20560_e28332, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20560_e28334;
        locals.var_xp_dn0 = assign20560_e28334_d_n0;
        locals.var_xp_dn2 = assign20560_e28334_d_n2;
        locals.var_xp_dn6 = assign20560_e28334_d_n6;
        locals.var_xp_dn7 = assign20560_e28334_d_n7;
        locals.var_xp_dn10 = assign20560_e28334_d_n10;
        locals.var_xp_dn11 = assign20560_e28334_d_n11;
        locals.var_xp_dn12 = assign20560_e28334_d_n12;
        locals.var_xp_dn17 = assign20560_e28334_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20570_e28351, assign20570_e28351_d_n0, assign20570_e28351_d_n2, assign20570_e28351_d_n6, assign20570_e28351_d_n7, assign20570_e28351_d_n10, assign20570_e28351_d_n11, assign20570_e28351_d_n12, assign20570_e28351_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20570_e28349: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign20570_e28349, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20570_e28351;
        locals.var_xmp_dn0 = assign20570_e28351_d_n0;
        locals.var_xmp_dn2 = assign20570_e28351_d_n2;
        locals.var_xmp_dn6 = assign20570_e28351_d_n6;
        locals.var_xmp_dn7 = assign20570_e28351_d_n7;
        locals.var_xmp_dn10 = assign20570_e28351_d_n10;
        locals.var_xmp_dn11 = assign20570_e28351_d_n11;
        locals.var_xmp_dn12 = assign20570_e28351_d_n12;
        locals.var_xmp_dn17 = assign20570_e28351_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20580_e28368, assign20580_e28368_d_n0, assign20580_e28368_d_n2, assign20580_e28368_d_n6, assign20580_e28368_d_n7, assign20580_e28368_d_n10, assign20580_e28368_d_n11, assign20580_e28368_d_n12, assign20580_e28368_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20580_e28366: f64 = (locals.var_xp + locals.var_xmp);
        (assign20580_e28366, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign20580_e28368;
        locals.var_arg_dn0 = assign20580_e28368_d_n0;
        locals.var_arg_dn2 = assign20580_e28368_d_n2;
        locals.var_arg_dn6 = assign20580_e28368_d_n6;
        locals.var_arg_dn7 = assign20580_e28368_d_n7;
        locals.var_arg_dn10 = assign20580_e28368_d_n10;
        locals.var_arg_dn11 = assign20580_e28368_d_n11;
        locals.var_arg_dn12 = assign20580_e28368_d_n12;
        locals.var_arg_dn17 = assign20580_e28368_d_n17;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20590_e28383, assign20590_e28383_d_n0, assign20590_e28383_d_n2, assign20590_e28383_d_n6, assign20590_e28383_d_n7, assign20590_e28383_d_n10, assign20590_e28383_d_n11, assign20590_e28383_d_n12, assign20590_e28383_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20590_e28383;
        locals.var_dnm_dn0 = assign20590_e28383_d_n0;
        locals.var_dnm_dn2 = assign20590_e28383_d_n2;
        locals.var_dnm_dn6 = assign20590_e28383_d_n6;
        locals.var_dnm_dn7 = assign20590_e28383_d_n7;
        locals.var_dnm_dn10 = assign20590_e28383_d_n10;
        locals.var_dnm_dn11 = assign20590_e28383_d_n11;
        locals.var_dnm_dn12 = assign20590_e28383_d_n12;
        locals.var_dnm_dn17 = assign20590_e28383_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign20600_e28398: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard636 = assign20600_e28398;
        locals.var_guard636_rv = 0.0;

        let assign20610_e28401: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign20610_e28401;
        locals.var_guard637_rv = 0.0;

        let (assign20620_e28420,) = {
    if ((((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20620_e28420;
        locals.var_mm_rv = 0.0;

        let assign20630_e28423: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign20630_e28423;
        locals.var_guard638_rv = 0.0;

        let (assign20640_e28445,) = {
    if (((((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20640_e28445;
        locals.var_mm_rv = 0.0;

        let assign20650_e28448: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign20650_e28448;
        locals.var_guard639_rv = 0.0;

        let (assign20660_e28473,) = {
    if ((((((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20660_e28473;
        locals.var_mm_rv = 0.0;

        let assign20670_e28476: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign20670_e28476;
        locals.var_guard640_rv = 0.0;

        let (assign20680_e28504,) = {
    if (((((((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 == 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20680_e28504;
        locals.var_mm_rv = 0.0;

        let (assign20690_e28521,) = {
    if (((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign20690_e28521;
        locals.var_m0_rv = 0.0;

        let mut assign20700_loop_guard: usize = 0;
        while {
            let assign20700_cond_e28539: f64 = if ((((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign20700_cond_e28539 != 0.0
        } {
            assign20700_loop_guard += 1;
            assert!(assign20700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign20700_body0_e28557, assign20700_body0_e28557_d_n0, assign20700_body0_e28557_d_n2, assign20700_body0_e28557_d_n6, assign20700_body0_e28557_d_n7, assign20700_body0_e28557_d_n10, assign20700_body0_e28557_d_n11, assign20700_body0_e28557_d_n12, assign20700_body0_e28557_d_n17,) = {
    if (((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign20700_body0_e28555: f64 = (locals.var_dnm).sqrt();
        (assign20700_body0_e28555, (locals.var_dnm_dn0 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn2 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn6 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn7 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn10 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn11 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn12 / (2.0 * assign20700_body0_e28555)), (locals.var_dnm_dn17 / (2.0 * assign20700_body0_e28555)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign20700_body0_e28557;
            locals.var_dnm_dn0 = assign20700_body0_e28557_d_n0;
            locals.var_dnm_dn2 = assign20700_body0_e28557_d_n2;
            locals.var_dnm_dn6 = assign20700_body0_e28557_d_n6;
            locals.var_dnm_dn7 = assign20700_body0_e28557_d_n7;
            locals.var_dnm_dn10 = assign20700_body0_e28557_d_n10;
            locals.var_dnm_dn11 = assign20700_body0_e28557_d_n11;
            locals.var_dnm_dn12 = assign20700_body0_e28557_d_n12;
            locals.var_dnm_dn17 = assign20700_body0_e28557_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign20700_body1_e28576,) = {
    if (((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign20700_body1_e28574: f64 = (locals.var_m0 + 1.0);
        (assign20700_body1_e28574,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign20700_body1_e28576;
            locals.var_m0_rv = 0.0;
        }

        let (assign20710_e28600, assign20710_e28600_d_n0, assign20710_e28600_d_n2, assign20710_e28600_d_n6, assign20710_e28600_d_n7, assign20710_e28600_d_n10, assign20710_e28600_d_n11, assign20710_e28600_d_n12, assign20710_e28600_d_n17,) = {
    if (((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 == 0.0)) {
        let assign20710_e28596: f64 = (2.0 * 2.0);
        let assign20710_e28597: f64 = (1.0 / assign20710_e28596);
        let assign20710_e28598: f64 = (locals.var_dnm).powf(assign20710_e28597);
        (assign20710_e28598, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn0)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn2)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn6)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn7)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn10)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn11)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn12)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20710_e28597) as f64).is_finite() && ((assign20710_e28597) as f64).fract() == 0.0 { if assign20710_e28597 == 0.0 { 0.0 } else { (assign20710_e28597 * ((locals.var_dnm).powf(assign20710_e28597 - 1.0) * locals.var_dnm_dn17)) } } else { (assign20710_e28598 * (assign20710_e28597 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20710_e28600;
        locals.var_dnm_dn0 = assign20710_e28600_d_n0;
        locals.var_dnm_dn2 = assign20710_e28600_d_n2;
        locals.var_dnm_dn6 = assign20710_e28600_d_n6;
        locals.var_dnm_dn7 = assign20710_e28600_d_n7;
        locals.var_dnm_dn10 = assign20710_e28600_d_n10;
        locals.var_dnm_dn11 = assign20710_e28600_d_n11;
        locals.var_dnm_dn12 = assign20710_e28600_d_n12;
        locals.var_dnm_dn17 = assign20710_e28600_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20720_e28617, assign20720_e28617_d_n0, assign20720_e28617_d_n2, assign20720_e28617_d_n6, assign20720_e28617_d_n7, assign20720_e28617_d_n10, assign20720_e28617_d_n11, assign20720_e28617_d_n12, assign20720_e28617_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20720_e28615: f64 = (1.0 / locals.var_dnm);
        (assign20720_e28615, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20720_e28617;
        locals.var_dnm_dn0 = assign20720_e28617_d_n0;
        locals.var_dnm_dn2 = assign20720_e28617_d_n2;
        locals.var_dnm_dn6 = assign20720_e28617_d_n6;
        locals.var_dnm_dn7 = assign20720_e28617_d_n7;
        locals.var_dnm_dn10 = assign20720_e28617_d_n10;
        locals.var_dnm_dn11 = assign20720_e28617_d_n11;
        locals.var_dnm_dn12 = assign20720_e28617_d_n12;
        locals.var_dnm_dn17 = assign20720_e28617_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20730_e28636, assign20730_e28636_d_n0, assign20730_e28636_d_n2, assign20730_e28636_d_n6, assign20730_e28636_d_n7, assign20730_e28636_d_n10, assign20730_e28636_d_n11, assign20730_e28636_d_n12, assign20730_e28636_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20730_e28632: f64 = (locals.var_tmf1 * 0.4);
        let assign20730_e28634: f64 = (assign20730_e28632 * locals.var_dnm);
        (assign20730_e28634, (((locals.var_tmf1_dn0 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.4) * locals.var_dnm) + (assign20730_e28632 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign20730_e28636;
        locals.var_tmf0_dn0 = assign20730_e28636_d_n0;
        locals.var_tmf0_dn2 = assign20730_e28636_d_n2;
        locals.var_tmf0_dn6 = assign20730_e28636_d_n6;
        locals.var_tmf0_dn7 = assign20730_e28636_d_n7;
        locals.var_tmf0_dn10 = assign20730_e28636_d_n10;
        locals.var_tmf0_dn11 = assign20730_e28636_d_n11;
        locals.var_tmf0_dn12 = assign20730_e28636_d_n12;
        locals.var_tmf0_dn17 = assign20730_e28636_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign20740_e28657, assign20740_e28657_d_n0, assign20740_e28657_d_n2, assign20740_e28657_d_n6, assign20740_e28657_d_n7, assign20740_e28657_d_n10, assign20740_e28657_d_n11, assign20740_e28657_d_n12, assign20740_e28657_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20740_e28651: f64 = (locals.var_ps0_inib * 0.98);
        let assign20740_e28653: f64 = (assign20740_e28651 - 0.4);
        let assign20740_e28655: f64 = (assign20740_e28653 + locals.var_tmf0);
        (assign20740_e28655, ((locals.var_ps0_inib_dn0 * 0.98) + locals.var_tmf0_dn0), ((locals.var_ps0_inib_dn2 * 0.98) + locals.var_tmf0_dn2), ((locals.var_ps0_inib_dn6 * 0.98) + locals.var_tmf0_dn6), ((locals.var_ps0_inib_dn7 * 0.98) + locals.var_tmf0_dn7), ((locals.var_ps0_inib_dn10 * 0.98) + locals.var_tmf0_dn10), ((locals.var_ps0_inib_dn11 * 0.98) + locals.var_tmf0_dn11), ((locals.var_ps0_inib_dn12 * 0.98) + locals.var_tmf0_dn12), ((locals.var_ps0_inib_dn17 * 0.98) + locals.var_tmf0_dn17),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20740_e28657;
        locals.var_ps0_ini_dn0 = assign20740_e28657_d_n0;
        locals.var_ps0_ini_dn2 = assign20740_e28657_d_n2;
        locals.var_ps0_ini_dn6 = assign20740_e28657_d_n6;
        locals.var_ps0_ini_dn7 = assign20740_e28657_d_n7;
        locals.var_ps0_ini_dn10 = assign20740_e28657_d_n10;
        locals.var_ps0_ini_dn11 = assign20740_e28657_d_n11;
        locals.var_ps0_ini_dn12 = assign20740_e28657_d_n12;
        locals.var_ps0_ini_dn17 = assign20740_e28657_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20750_e28673, assign20750_e28673_d_n0, assign20750_e28673_d_n2, assign20750_e28673_d_n6, assign20750_e28673_d_n7, assign20750_e28673_d_n10, assign20750_e28673_d_n11, assign20750_e28673_d_n12, assign20750_e28673_d_n17,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard628 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20750_e28673;
        locals.var_ps0_ini_dn0 = assign20750_e28673_d_n0;
        locals.var_ps0_ini_dn2 = assign20750_e28673_d_n2;
        locals.var_ps0_ini_dn6 = assign20750_e28673_d_n6;
        locals.var_ps0_ini_dn7 = assign20750_e28673_d_n7;
        locals.var_ps0_ini_dn10 = assign20750_e28673_d_n10;
        locals.var_ps0_ini_dn11 = assign20750_e28673_d_n11;
        locals.var_ps0_ini_dn12 = assign20750_e28673_d_n12;
        locals.var_ps0_ini_dn17 = assign20750_e28673_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20760_e28681, assign20760_e28681_d_n0, assign20760_e28681_d_n2, assign20760_e28681_d_n6, assign20760_e28681_d_n7, assign20760_e28681_d_n10, assign20760_e28681_d_n11, assign20760_e28681_d_n12, assign20760_e28681_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign20760_e28678: f64 = (5e-12 / 2.0);
        let assign20760_e28679: f64 = (locals.var_vbs__blk625 + assign20760_e28678);
        (assign20760_e28679, locals.var_vbs__blk625_dn0, locals.var_vbs__blk625_dn2, locals.var_vbs__blk625_dn6, locals.var_vbs__blk625_dn7, locals.var_vbs__blk625_dn10, locals.var_vbs__blk625_dn11, locals.var_vbs__blk625_dn12, locals.var_vbs__blk625_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20760_e28681;
        locals.var_tx_dn0 = assign20760_e28681_d_n0;
        locals.var_tx_dn2 = assign20760_e28681_d_n2;
        locals.var_tx_dn6 = assign20760_e28681_d_n6;
        locals.var_tx_dn7 = assign20760_e28681_d_n7;
        locals.var_tx_dn10 = assign20760_e28681_d_n10;
        locals.var_tx_dn11 = assign20760_e28681_d_n11;
        locals.var_tx_dn12 = assign20760_e28681_d_n12;
        locals.var_tx_dn17 = assign20760_e28681_d_n17;
        locals.var_tx_rv = 0.0;

        let assign20770_e28684: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard641 = assign20770_e28684;
        locals.var_guard641_rv = 0.0;

        let (assign20780_e28690, assign20780_e28690_d_n0, assign20780_e28690_d_n2, assign20780_e28690_d_n6, assign20780_e28690_d_n7, assign20780_e28690_d_n10, assign20780_e28690_d_n11, assign20780_e28690_d_n12, assign20780_e28690_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard641 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20780_e28690;
        locals.var_ps0_ini_dn0 = assign20780_e28690_d_n0;
        locals.var_ps0_ini_dn2 = assign20780_e28690_d_n2;
        locals.var_ps0_ini_dn6 = assign20780_e28690_d_n6;
        locals.var_ps0_ini_dn7 = assign20780_e28690_d_n7;
        locals.var_ps0_ini_dn10 = assign20780_e28690_d_n10;
        locals.var_ps0_ini_dn11 = assign20780_e28690_d_n11;
        locals.var_ps0_ini_dn12 = assign20780_e28690_d_n12;
        locals.var_ps0_ini_dn17 = assign20780_e28690_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20790_e28694, assign20790_e28694_d_n0, assign20790_e28694_d_n2, assign20790_e28694_d_n6, assign20790_e28694_d_n7, assign20790_e28694_d_n10, assign20790_e28694_d_n11, assign20790_e28694_d_n12, assign20790_e28694_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0__blk608, locals.var_ps0__blk608_dn0, locals.var_ps0__blk608_dn2, locals.var_ps0__blk608_dn6, locals.var_ps0__blk608_dn7, locals.var_ps0__blk608_dn10, locals.var_ps0__blk608_dn11, locals.var_ps0__blk608_dn12, locals.var_ps0__blk608_dn17,)
    }
};
        locals.var_ps0__blk608 = assign20790_e28694;
        locals.var_ps0__blk608_dn0 = assign20790_e28694_d_n0;
        locals.var_ps0__blk608_dn2 = assign20790_e28694_d_n2;
        locals.var_ps0__blk608_dn6 = assign20790_e28694_d_n6;
        locals.var_ps0__blk608_dn7 = assign20790_e28694_d_n7;
        locals.var_ps0__blk608_dn10 = assign20790_e28694_d_n10;
        locals.var_ps0__blk608_dn11 = assign20790_e28694_d_n11;
        locals.var_ps0__blk608_dn12 = assign20790_e28694_d_n12;
        locals.var_ps0__blk608_dn17 = assign20790_e28694_d_n17;
        locals.var_ps0__blk608_rv = 0.0;

        let (assign20800_e28698, assign20800_e28698_d_n0, assign20800_e28698_d_n2, assign20800_e28698_d_n6, assign20800_e28698_d_n7, assign20800_e28698_d_n10, assign20800_e28698_d_n11, assign20800_e28698_d_n12, assign20800_e28698_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign20800_e28698;
        locals.var_psl_lim_dn0 = assign20800_e28698_d_n0;
        locals.var_psl_lim_dn2 = assign20800_e28698_d_n2;
        locals.var_psl_lim_dn6 = assign20800_e28698_d_n6;
        locals.var_psl_lim_dn7 = assign20800_e28698_d_n7;
        locals.var_psl_lim_dn10 = assign20800_e28698_d_n10;
        locals.var_psl_lim_dn11 = assign20800_e28698_d_n11;
        locals.var_psl_lim_dn12 = assign20800_e28698_d_n12;
        locals.var_psl_lim_dn17 = assign20800_e28698_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign20810_e28713, assign20810_e28713_d_n0, assign20810_e28713_d_n2, assign20810_e28713_d_n6, assign20810_e28713_d_n7, assign20810_e28713_d_n10, assign20810_e28713_d_n11, assign20810_e28713_d_n12, assign20810_e28713_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let assign20810_e28704: f64 = (locals.var_ps0_inia - locals.var_ps0__blk608);
        let (assign20810_e28711, assign20810_e28711_d_n0, assign20810_e28711_d_n2, assign20810_e28711_d_n6, assign20810_e28711_d_n7, assign20810_e28711_d_n10, assign20810_e28711_d_n11, assign20810_e28711_d_n12, assign20810_e28711_d_n17,) = {
            if (assign20810_e28704 >= 0.0) {
                let assign20810_e28709: f64 = (locals.var_ps0_inia - locals.var_ps0__blk608);
                (assign20810_e28709, (locals.var_ps0_inia_dn0 - locals.var_ps0__blk608_dn0), (locals.var_ps0_inia_dn2 - locals.var_ps0__blk608_dn2), (locals.var_ps0_inia_dn6 - locals.var_ps0__blk608_dn6), (locals.var_ps0_inia_dn7 - locals.var_ps0__blk608_dn7), (locals.var_ps0_inia_dn10 - locals.var_ps0__blk608_dn10), (locals.var_ps0_inia_dn11 - locals.var_ps0__blk608_dn11), (locals.var_ps0_inia_dn12 - locals.var_ps0__blk608_dn12), (locals.var_ps0_inia_dn17 - locals.var_ps0__blk608_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign20810_e28711, assign20810_e28711_d_n0, assign20810_e28711_d_n2, assign20810_e28711_d_n6, assign20810_e28711_d_n7, assign20810_e28711_d_n10, assign20810_e28711_d_n11, assign20810_e28711_d_n12, assign20810_e28711_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign20810_e28713;
        locals.var_pds_max_dn0 = assign20810_e28713_d_n0;
        locals.var_pds_max_dn2 = assign20810_e28713_d_n2;
        locals.var_pds_max_dn6 = assign20810_e28713_d_n6;
        locals.var_pds_max_dn7 = assign20810_e28713_d_n7;
        locals.var_pds_max_dn10 = assign20810_e28713_d_n10;
        locals.var_pds_max_dn11 = assign20810_e28713_d_n11;
        locals.var_pds_max_dn12 = assign20810_e28713_d_n12;
        locals.var_pds_max_dn17 = assign20810_e28713_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign20820_e28727, assign20820_e28727_d_n0, assign20820_e28727_d_n2, assign20820_e28727_d_n6, assign20820_e28727_d_n7, assign20820_e28727_d_n10, assign20820_e28727_d_n11, assign20820_e28727_d_n12, assign20820_e28727_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let assign20820_e28719: f64 = (1.0 + 0.3);
        let assign20820_e28721: f64 = (assign20820_e28719 * locals.var_pds_max);
        let assign20820_e28723: f64 = (assign20820_e28721 - p.p287);
        let assign20820_e28725: f64 = (assign20820_e28723 - 0.03);
        (assign20820_e28725, (assign20820_e28719 * locals.var_pds_max_dn0), (assign20820_e28719 * locals.var_pds_max_dn2), (assign20820_e28719 * locals.var_pds_max_dn6), (assign20820_e28719 * locals.var_pds_max_dn7), (assign20820_e28719 * locals.var_pds_max_dn10), (assign20820_e28719 * locals.var_pds_max_dn11), (assign20820_e28719 * locals.var_pds_max_dn12), (assign20820_e28719 * locals.var_pds_max_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20820_e28727;
        locals.var_tmf1_dn0 = assign20820_e28727_d_n0;
        locals.var_tmf1_dn2 = assign20820_e28727_d_n2;
        locals.var_tmf1_dn6 = assign20820_e28727_d_n6;
        locals.var_tmf1_dn7 = assign20820_e28727_d_n7;
        locals.var_tmf1_dn10 = assign20820_e28727_d_n10;
        locals.var_tmf1_dn11 = assign20820_e28727_d_n11;
        locals.var_tmf1_dn12 = assign20820_e28727_d_n12;
        locals.var_tmf1_dn17 = assign20820_e28727_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20830_e28741, assign20830_e28741_d_n0, assign20830_e28741_d_n2, assign20830_e28741_d_n6, assign20830_e28741_d_n7, assign20830_e28741_d_n10, assign20830_e28741_d_n11, assign20830_e28741_d_n12, assign20830_e28741_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let assign20830_e28734: f64 = (1.0 + 0.3);
        let assign20830_e28736: f64 = (assign20830_e28734 * locals.var_pds_max);
        let assign20830_e28737: f64 = (4.0 * assign20830_e28736);
        let assign20830_e28739: f64 = (assign20830_e28737 * 0.03);
        (assign20830_e28739, ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign20830_e28734 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20830_e28741;
        locals.var_tmf2_dn0 = assign20830_e28741_d_n0;
        locals.var_tmf2_dn2 = assign20830_e28741_d_n2;
        locals.var_tmf2_dn6 = assign20830_e28741_d_n6;
        locals.var_tmf2_dn7 = assign20830_e28741_d_n7;
        locals.var_tmf2_dn10 = assign20830_e28741_d_n10;
        locals.var_tmf2_dn11 = assign20830_e28741_d_n11;
        locals.var_tmf2_dn12 = assign20830_e28741_d_n12;
        locals.var_tmf2_dn17 = assign20830_e28741_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20840_e28753, assign20840_e28753_d_n0, assign20840_e28753_d_n2, assign20840_e28753_d_n6, assign20840_e28753_d_n7, assign20840_e28753_d_n10, assign20840_e28753_d_n11, assign20840_e28753_d_n12, assign20840_e28753_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let (assign20840_e28751, assign20840_e28751_d_n0, assign20840_e28751_d_n2, assign20840_e28751_d_n6, assign20840_e28751_d_n7, assign20840_e28751_d_n10, assign20840_e28751_d_n11, assign20840_e28751_d_n12, assign20840_e28751_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign20840_e28750: f64 = (-locals.var_tmf2);
                (assign20840_e28750, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign20840_e28751, assign20840_e28751_d_n0, assign20840_e28751_d_n2, assign20840_e28751_d_n6, assign20840_e28751_d_n7, assign20840_e28751_d_n10, assign20840_e28751_d_n11, assign20840_e28751_d_n12, assign20840_e28751_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20840_e28753;
        locals.var_tmf2_dn0 = assign20840_e28753_d_n0;
        locals.var_tmf2_dn2 = assign20840_e28753_d_n2;
        locals.var_tmf2_dn6 = assign20840_e28753_d_n6;
        locals.var_tmf2_dn7 = assign20840_e28753_d_n7;
        locals.var_tmf2_dn10 = assign20840_e28753_d_n10;
        locals.var_tmf2_dn11 = assign20840_e28753_d_n11;
        locals.var_tmf2_dn12 = assign20840_e28753_d_n12;
        locals.var_tmf2_dn17 = assign20840_e28753_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20850_e28764, assign20850_e28764_d_n0, assign20850_e28764_d_n2, assign20850_e28764_d_n6, assign20850_e28764_d_n7, assign20850_e28764_d_n10, assign20850_e28764_d_n11, assign20850_e28764_d_n12, assign20850_e28764_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let assign20850_e28759: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20850_e28761: f64 = (assign20850_e28759 + locals.var_tmf2);
        let assign20850_e28762: f64 = (assign20850_e28761).sqrt();
        (assign20850_e28762, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20850_e28762)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign20850_e28762)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20850_e28764;
        locals.var_tmf2_dn0 = assign20850_e28764_d_n0;
        locals.var_tmf2_dn2 = assign20850_e28764_d_n2;
        locals.var_tmf2_dn6 = assign20850_e28764_d_n6;
        locals.var_tmf2_dn7 = assign20850_e28764_d_n7;
        locals.var_tmf2_dn10 = assign20850_e28764_d_n10;
        locals.var_tmf2_dn11 = assign20850_e28764_d_n11;
        locals.var_tmf2_dn12 = assign20850_e28764_d_n12;
        locals.var_tmf2_dn17 = assign20850_e28764_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20860_e28780, assign20860_e28780_d_n0, assign20860_e28780_d_n2, assign20860_e28780_d_n6, assign20860_e28780_d_n7, assign20860_e28780_d_n10, assign20860_e28780_d_n11, assign20860_e28780_d_n12, assign20860_e28780_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let assign20860_e28770: f64 = (1.0 + 0.3);
        let assign20860_e28772: f64 = (assign20860_e28770 * locals.var_pds_max);
        let assign20860_e28776: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20860_e28777: f64 = (0.5 * assign20860_e28776);
        let assign20860_e28778: f64 = (assign20860_e28772 - assign20860_e28777);
        (assign20860_e28778, ((assign20860_e28770 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign20860_e28770 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign20860_e28770 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign20860_e28770 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign20860_e28770 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign20860_e28770 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign20860_e28770 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign20860_e28770 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20860_e28780;
        locals.var_pds_ini_dn0 = assign20860_e28780_d_n0;
        locals.var_pds_ini_dn2 = assign20860_e28780_d_n2;
        locals.var_pds_ini_dn6 = assign20860_e28780_d_n6;
        locals.var_pds_ini_dn7 = assign20860_e28780_d_n7;
        locals.var_pds_ini_dn10 = assign20860_e28780_d_n10;
        locals.var_pds_ini_dn11 = assign20860_e28780_d_n11;
        locals.var_pds_ini_dn12 = assign20860_e28780_d_n12;
        locals.var_pds_ini_dn17 = assign20860_e28780_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign20870_e28791, assign20870_e28791_d_n0, assign20870_e28791_d_n2, assign20870_e28791_d_n6, assign20870_e28791_d_n7, assign20870_e28791_d_n10, assign20870_e28791_d_n11, assign20870_e28791_d_n12, assign20870_e28791_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let (assign20870_e28789, assign20870_e28789_d_n0, assign20870_e28789_d_n2, assign20870_e28789_d_n6, assign20870_e28789_d_n7, assign20870_e28789_d_n10, assign20870_e28789_d_n11, assign20870_e28789_d_n12, assign20870_e28789_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign20870_e28789, assign20870_e28789_d_n0, assign20870_e28789_d_n2, assign20870_e28789_d_n6, assign20870_e28789_d_n7, assign20870_e28789_d_n10, assign20870_e28789_d_n11, assign20870_e28789_d_n12, assign20870_e28789_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20870_e28791;
        locals.var_pds_ini_dn0 = assign20870_e28791_d_n0;
        locals.var_pds_ini_dn2 = assign20870_e28791_d_n2;
        locals.var_pds_ini_dn6 = assign20870_e28791_d_n6;
        locals.var_pds_ini_dn7 = assign20870_e28791_d_n7;
        locals.var_pds_ini_dn10 = assign20870_e28791_d_n10;
        locals.var_pds_ini_dn11 = assign20870_e28791_d_n11;
        locals.var_pds_ini_dn12 = assign20870_e28791_d_n12;
        locals.var_pds_ini_dn17 = assign20870_e28791_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign20880_e28794: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign20880_e28794;
        locals.var_guard642_rv = 0.0;

        let (assign20890_e28802, assign20890_e28802_d_n0, assign20890_e28802_d_n2, assign20890_e28802_d_n6, assign20890_e28802_d_n7, assign20890_e28802_d_n10, assign20890_e28802_d_n11, assign20890_e28802_d_n12, assign20890_e28802_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (0.0 != 0.0)) && (locals.var_guard642 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20890_e28802;
        locals.var_pds_ini_dn0 = assign20890_e28802_d_n0;
        locals.var_pds_ini_dn2 = assign20890_e28802_d_n2;
        locals.var_pds_ini_dn6 = assign20890_e28802_d_n6;
        locals.var_pds_ini_dn7 = assign20890_e28802_d_n7;
        locals.var_pds_ini_dn10 = assign20890_e28802_d_n10;
        locals.var_pds_ini_dn11 = assign20890_e28802_d_n11;
        locals.var_pds_ini_dn12 = assign20890_e28802_d_n12;
        locals.var_pds_ini_dn17 = assign20890_e28802_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign20900_e28805: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard643 = assign20900_e28805;
        locals.var_guard643_rv = 0.0;

        let (assign20910_e28816, assign20910_e28816_d_n0, assign20910_e28816_d_n2, assign20910_e28816_d_n6, assign20910_e28816_d_n7, assign20910_e28816_d_n10, assign20910_e28816_d_n11, assign20910_e28816_d_n12, assign20910_e28816_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (0.0 != 0.0)) && (locals.var_guard642 == 0.0)) && (locals.var_guard643 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20910_e28816;
        locals.var_pds_ini_dn0 = assign20910_e28816_d_n0;
        locals.var_pds_ini_dn2 = assign20910_e28816_d_n2;
        locals.var_pds_ini_dn6 = assign20910_e28816_d_n6;
        locals.var_pds_ini_dn7 = assign20910_e28816_d_n7;
        locals.var_pds_ini_dn10 = assign20910_e28816_d_n10;
        locals.var_pds_ini_dn11 = assign20910_e28816_d_n11;
        locals.var_pds_ini_dn12 = assign20910_e28816_d_n12;
        locals.var_pds_ini_dn17 = assign20910_e28816_d_n17;
        locals.var_pds_ini_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20920_e28824, assign20920_e28824_d_n0, assign20920_e28824_d_n2, assign20920_e28824_d_n6, assign20920_e28824_d_n7, assign20920_e28824_d_n10, assign20920_e28824_d_n11, assign20920_e28824_d_n12, assign20920_e28824_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (0.0 != 0.0)) {
        let assign20920_e28822: f64 = (locals.var_ps0__blk608 + locals.var_pds_ini);
        (assign20920_e28822, (locals.var_ps0__blk608_dn0 + locals.var_pds_ini_dn0), (locals.var_ps0__blk608_dn2 + locals.var_pds_ini_dn2), (locals.var_ps0__blk608_dn6 + locals.var_pds_ini_dn6), (locals.var_ps0__blk608_dn7 + locals.var_pds_ini_dn7), (locals.var_ps0__blk608_dn10 + locals.var_pds_ini_dn10), (locals.var_ps0__blk608_dn11 + locals.var_pds_ini_dn11), (locals.var_ps0__blk608_dn12 + locals.var_pds_ini_dn12), (locals.var_ps0__blk608_dn17 + locals.var_pds_ini_dn17),)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign20920_e28824;
        locals.var_psl_lim_dn0 = assign20920_e28824_d_n0;
        locals.var_psl_lim_dn2 = assign20920_e28824_d_n2;
        locals.var_psl_lim_dn6 = assign20920_e28824_d_n6;
        locals.var_psl_lim_dn7 = assign20920_e28824_d_n7;
        locals.var_psl_lim_dn10 = assign20920_e28824_d_n10;
        locals.var_psl_lim_dn11 = assign20920_e28824_d_n11;
        locals.var_psl_lim_dn12 = assign20920_e28824_d_n12;
        locals.var_psl_lim_dn17 = assign20920_e28824_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let assign20930_e28827: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign20930_e28827;
        locals.var_guard644_rv = 0.0;

        let (assign20940_e28833, assign20940_e28833_d_n0, assign20940_e28833_d_n2, assign20940_e28833_d_n6, assign20940_e28833_d_n7, assign20940_e28833_d_n10, assign20940_e28833_d_n11, assign20940_e28833_d_n12, assign20940_e28833_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) {
        (locals.var_ps0__blk608, locals.var_ps0__blk608_dn0, locals.var_ps0__blk608_dn2, locals.var_ps0__blk608_dn6, locals.var_ps0__blk608_dn7, locals.var_ps0__blk608_dn10, locals.var_ps0__blk608_dn11, locals.var_ps0__blk608_dn12, locals.var_ps0__blk608_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20940_e28833;
        locals.var_ps0_ini_dn0 = assign20940_e28833_d_n0;
        locals.var_ps0_ini_dn2 = assign20940_e28833_d_n2;
        locals.var_ps0_ini_dn6 = assign20940_e28833_d_n6;
        locals.var_ps0_ini_dn7 = assign20940_e28833_d_n7;
        locals.var_ps0_ini_dn10 = assign20940_e28833_d_n10;
        locals.var_ps0_ini_dn11 = assign20940_e28833_d_n11;
        locals.var_ps0_ini_dn12 = assign20940_e28833_d_n12;
        locals.var_ps0_ini_dn17 = assign20940_e28833_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20950_e28839, assign20950_e28839_d_n0, assign20950_e28839_d_n2, assign20950_e28839_d_n6, assign20950_e28839_d_n7, assign20950_e28839_d_n10, assign20950_e28839_d_n11, assign20950_e28839_d_n12, assign20950_e28839_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    } else {
        (locals.var_vbcs_cl__blk645, locals.var_vbcs_cl__blk645_dn0, locals.var_vbcs_cl__blk645_dn2, locals.var_vbcs_cl__blk645_dn6, locals.var_vbcs_cl__blk645_dn7, locals.var_vbcs_cl__blk645_dn10, locals.var_vbcs_cl__blk645_dn11, locals.var_vbcs_cl__blk645_dn12, locals.var_vbcs_cl__blk645_dn17,)
    }
};
        locals.var_vbcs_cl__blk645 = assign20950_e28839;
        locals.var_vbcs_cl__blk645_dn0 = assign20950_e28839_d_n0;
        locals.var_vbcs_cl__blk645_dn2 = assign20950_e28839_d_n2;
        locals.var_vbcs_cl__blk645_dn6 = assign20950_e28839_d_n6;
        locals.var_vbcs_cl__blk645_dn7 = assign20950_e28839_d_n7;
        locals.var_vbcs_cl__blk645_dn10 = assign20950_e28839_d_n10;
        locals.var_vbcs_cl__blk645_dn11 = assign20950_e28839_d_n11;
        locals.var_vbcs_cl__blk645_dn12 = assign20950_e28839_d_n12;
        locals.var_vbcs_cl__blk645_dn17 = assign20950_e28839_d_n17;
        locals.var_vbcs_cl__blk645_rv = 0.0;

        let (assign20960_e28853, assign20960_e28853_d_n0, assign20960_e28853_d_n2, assign20960_e28853_d_n6, assign20960_e28853_d_n7, assign20960_e28853_d_n10, assign20960_e28853_d_n11, assign20960_e28853_d_n12, assign20960_e28853_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) {
        let assign20960_e28845: f64 = (locals.var_vfb - locals.var_dvth);
        let assign20960_e28847: f64 = (assign20960_e28845 + locals.var_dppg);
        let assign20960_e28849: f64 = (assign20960_e28847 + locals.var_vbcs_cl__blk645);
        let assign20960_e28851: f64 = (assign20960_e28849 + p.p286);
        (assign20960_e28851, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbcs_cl__blk645_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbcs_cl__blk645_dn2), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbcs_cl__blk645_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbcs_cl__blk645_dn7), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbcs_cl__blk645_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbcs_cl__blk645_dn11), (((-locals.var_dvth_dn12) + locals.var_dppg_dn12) + locals.var_vbcs_cl__blk645_dn12), (((-locals.var_dvth_dn17) + locals.var_dppg_dn17) + locals.var_vbcs_cl__blk645_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign20960_e28853;
        locals.var_vgs_fb_dn0 = assign20960_e28853_d_n0;
        locals.var_vgs_fb_dn2 = assign20960_e28853_d_n2;
        locals.var_vgs_fb_dn6 = assign20960_e28853_d_n6;
        locals.var_vgs_fb_dn7 = assign20960_e28853_d_n7;
        locals.var_vgs_fb_dn10 = assign20960_e28853_d_n10;
        locals.var_vgs_fb_dn11 = assign20960_e28853_d_n11;
        locals.var_vgs_fb_dn12 = assign20960_e28853_d_n12;
        locals.var_vgs_fb_dn17 = assign20960_e28853_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let assign20970_e28856: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard647 = assign20970_e28856;
        locals.var_guard647_rv = 0.0;

        let (assign20980_e28865,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign20980_e28863: f64 = (-1.0);
        (assign20980_e28863,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign20980_e28865;
        locals.var_flg_zone_rv = 0.0;

        let (assign20990_e28881, assign20990_e28881_d_n0, assign20990_e28881_d_n2, assign20990_e28881_d_n6, assign20990_e28881_d_n7, assign20990_e28881_d_n10, assign20990_e28881_d_n11, assign20990_e28881_d_n12, assign20990_e28881_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign20990_e28873: f64 = (2.0 * locals.var_beta_inv);
        let assign20990_e28875: f64 = (-locals.var_vgs_min);
        let assign20990_e28877: f64 = (assign20990_e28875 / locals.var_fac1);
        let assign20990_e28878: f64 = (assign20990_e28877).ln();
        let assign20990_e28879: f64 = (assign20990_e28873 * assign20990_e28878);
        (assign20990_e28879, (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)), (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)), (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)), (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)), (((2.0 * locals.var_beta_inv_dn10) * assign20990_e28878) + (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877))), (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)), (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)), (assign20990_e28873 * ((-((assign20990_e28875 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign20990_e28877)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign20990_e28881;
        locals.var_ps0_min_dn0 = assign20990_e28881_d_n0;
        locals.var_ps0_min_dn2 = assign20990_e28881_d_n2;
        locals.var_ps0_min_dn6 = assign20990_e28881_d_n6;
        locals.var_ps0_min_dn7 = assign20990_e28881_d_n7;
        locals.var_ps0_min_dn10 = assign20990_e28881_d_n10;
        locals.var_ps0_min_dn11 = assign20990_e28881_d_n11;
        locals.var_ps0_min_dn12 = assign20990_e28881_d_n12;
        locals.var_ps0_min_dn17 = assign20990_e28881_d_n17;
        locals.var_ps0_min_rv = 0.0;

        let (assign21000_e28893, assign21000_e28893_d_n0, assign21000_e28893_d_n2, assign21000_e28893_d_n6, assign21000_e28893_d_n7, assign21000_e28893_d_n10, assign21000_e28893_d_n11, assign21000_e28893_d_n12, assign21000_e28893_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21000_e28890: f64 = (locals.var_vgp__blk610 - locals.var_vbcs_cl__blk645);
        let assign21000_e28891: f64 = (locals.var_beta * assign21000_e28890);
        (assign21000_e28891, (locals.var_beta * (locals.var_vgp__blk610_dn0 - locals.var_vbcs_cl__blk645_dn0)), (locals.var_beta * (locals.var_vgp__blk610_dn2 - locals.var_vbcs_cl__blk645_dn2)), (locals.var_beta * (locals.var_vgp__blk610_dn6 - locals.var_vbcs_cl__blk645_dn6)), (locals.var_beta * (locals.var_vgp__blk610_dn7 - locals.var_vbcs_cl__blk645_dn7)), ((locals.var_beta_dn10 * assign21000_e28890) + (locals.var_beta * (locals.var_vgp__blk610_dn10 - locals.var_vbcs_cl__blk645_dn10))), (locals.var_beta * (locals.var_vgp__blk610_dn11 - locals.var_vbcs_cl__blk645_dn11)), (locals.var_beta * (locals.var_vgp__blk610_dn12 - locals.var_vbcs_cl__blk645_dn12)), (locals.var_beta * (locals.var_vgp__blk610_dn17 - locals.var_vbcs_cl__blk645_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign21000_e28893;
        locals.var_tx_dn0 = assign21000_e28893_d_n0;
        locals.var_tx_dn2 = assign21000_e28893_d_n2;
        locals.var_tx_dn6 = assign21000_e28893_d_n6;
        locals.var_tx_dn7 = assign21000_e28893_d_n7;
        locals.var_tx_dn10 = assign21000_e28893_d_n10;
        locals.var_tx_dn11 = assign21000_e28893_d_n11;
        locals.var_tx_dn12 = assign21000_e28893_d_n12;
        locals.var_tx_dn17 = assign21000_e28893_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign21010_e28905, assign21010_e28905_d_n0, assign21010_e28905_d_n2, assign21010_e28905_d_n6, assign21010_e28905_d_n7, assign21010_e28905_d_n10, assign21010_e28905_d_n11, assign21010_e28905_d_n12, assign21010_e28905_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21010_e28902: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign21010_e28903: f64 = (1.0 / assign21010_e28902);
        (assign21010_e28903, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign21010_e28902 * assign21010_e28902))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign21010_e28902 * assign21010_e28902))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign21010_e28902 * assign21010_e28902))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign21010_e28902 * assign21010_e28902))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign21010_e28902 * assign21010_e28902))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign21010_e28902 * assign21010_e28902))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign21010_e28902 * assign21010_e28902))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign21010_e28902 * assign21010_e28902))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21010_e28905;
        locals.var_t1_dn0 = assign21010_e28905_d_n0;
        locals.var_t1_dn2 = assign21010_e28905_d_n2;
        locals.var_t1_dn6 = assign21010_e28905_d_n6;
        locals.var_t1_dn7 = assign21010_e28905_d_n7;
        locals.var_t1_dn10 = assign21010_e28905_d_n10;
        locals.var_t1_dn11 = assign21010_e28905_d_n11;
        locals.var_t1_dn12 = assign21010_e28905_d_n12;
        locals.var_t1_dn17 = assign21010_e28905_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21020_e28915, assign21020_e28915_d_n0, assign21020_e28915_d_n2, assign21020_e28915_d_n6, assign21020_e28915_d_n7, assign21020_e28915_d_n10, assign21020_e28915_d_n11, assign21020_e28915_d_n12, assign21020_e28915_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21020_e28913: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign21020_e28913, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign21020_e28915;
        locals.var_ty_dn0 = assign21020_e28915_d_n0;
        locals.var_ty_dn2 = assign21020_e28915_d_n2;
        locals.var_ty_dn6 = assign21020_e28915_d_n6;
        locals.var_ty_dn7 = assign21020_e28915_d_n7;
        locals.var_ty_dn10 = assign21020_e28915_d_n10;
        locals.var_ty_dn11 = assign21020_e28915_d_n11;
        locals.var_ty_dn12 = assign21020_e28915_d_n12;
        locals.var_ty_dn17 = assign21020_e28915_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign21030_e28929, assign21030_e28929_d_n0, assign21030_e28929_d_n2, assign21030_e28929_d_n6, assign21030_e28929_d_n7, assign21030_e28929_d_n10, assign21030_e28929_d_n11, assign21030_e28929_d_n12, assign21030_e28929_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21030_e28924: f64 = (3.0 * 1.414213562373095);
        let assign21030_e28926: f64 = (assign21030_e28924 * locals.var_ty);
        let assign21030_e28927: f64 = (2.0 + assign21030_e28926);
        (assign21030_e28927, (assign21030_e28924 * locals.var_ty_dn0), (assign21030_e28924 * locals.var_ty_dn2), (assign21030_e28924 * locals.var_ty_dn6), (assign21030_e28924 * locals.var_ty_dn7), (assign21030_e28924 * locals.var_ty_dn10), (assign21030_e28924 * locals.var_ty_dn11), (assign21030_e28924 * locals.var_ty_dn12), (assign21030_e28924 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign21030_e28929;
        locals.var_ac41_dn0 = assign21030_e28929_d_n0;
        locals.var_ac41_dn2 = assign21030_e28929_d_n2;
        locals.var_ac41_dn6 = assign21030_e28929_d_n6;
        locals.var_ac41_dn7 = assign21030_e28929_d_n7;
        locals.var_ac41_dn10 = assign21030_e28929_d_n10;
        locals.var_ac41_dn11 = assign21030_e28929_d_n11;
        locals.var_ac41_dn12 = assign21030_e28929_d_n12;
        locals.var_ac41_dn17 = assign21030_e28929_d_n17;
        locals.var_ac41_rv = 0.0;

        let (assign21040_e28943, assign21040_e28943_d_n0, assign21040_e28943_d_n2, assign21040_e28943_d_n6, assign21040_e28943_d_n7, assign21040_e28943_d_n10, assign21040_e28943_d_n11, assign21040_e28943_d_n12, assign21040_e28943_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21040_e28937: f64 = (8.0 * locals.var_ac41);
        let assign21040_e28939: f64 = (assign21040_e28937 * locals.var_ac41);
        let assign21040_e28941: f64 = (assign21040_e28939 * locals.var_ac41);
        (assign21040_e28941, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign21040_e28937 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign21040_e28939 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign21040_e28943;
        locals.var_ac4_dn0 = assign21040_e28943_d_n0;
        locals.var_ac4_dn2 = assign21040_e28943_d_n2;
        locals.var_ac4_dn6 = assign21040_e28943_d_n6;
        locals.var_ac4_dn7 = assign21040_e28943_d_n7;
        locals.var_ac4_dn10 = assign21040_e28943_d_n10;
        locals.var_ac4_dn11 = assign21040_e28943_d_n11;
        locals.var_ac4_dn12 = assign21040_e28943_d_n12;
        locals.var_ac4_dn17 = assign21040_e28943_d_n17;
        locals.var_ac4_rv = 0.0;

        let (assign21050_e28953, assign21050_e28953_d_n0, assign21050_e28953_d_n2, assign21050_e28953_d_n6, assign21050_e28953_d_n7, assign21050_e28953_d_n10, assign21050_e28953_d_n11, assign21050_e28953_d_n12, assign21050_e28953_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21050_e28951: f64 = (locals.var_tx - 2.0);
        (assign21050_e28951, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign21050_e28953;
        locals.var_t4_dn0 = assign21050_e28953_d_n0;
        locals.var_t4_dn2 = assign21050_e28953_d_n2;
        locals.var_t4_dn6 = assign21050_e28953_d_n6;
        locals.var_t4_dn7 = assign21050_e28953_d_n7;
        locals.var_t4_dn10 = assign21050_e28953_d_n10;
        locals.var_t4_dn11 = assign21050_e28953_d_n11;
        locals.var_t4_dn12 = assign21050_e28953_d_n12;
        locals.var_t4_dn17 = assign21050_e28953_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign21060_e28965, assign21060_e28965_d_n0, assign21060_e28965_d_n2, assign21060_e28965_d_n6, assign21060_e28965_d_n7, assign21060_e28965_d_n10, assign21060_e28965_d_n11, assign21060_e28965_d_n12, assign21060_e28965_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21060_e28961: f64 = (9.0 * locals.var_ty);
        let assign21060_e28963: f64 = (assign21060_e28961 * locals.var_t4);
        (assign21060_e28963, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign21060_e28961 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign21060_e28965;
        locals.var_t5_dn0 = assign21060_e28965_d_n0;
        locals.var_t5_dn2 = assign21060_e28965_d_n2;
        locals.var_t5_dn6 = assign21060_e28965_d_n6;
        locals.var_t5_dn7 = assign21060_e28965_d_n7;
        locals.var_t5_dn10 = assign21060_e28965_d_n10;
        locals.var_t5_dn11 = assign21060_e28965_d_n11;
        locals.var_t5_dn12 = assign21060_e28965_d_n12;
        locals.var_t5_dn17 = assign21060_e28965_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign21070_e28977, assign21070_e28977_d_n0, assign21070_e28977_d_n2, assign21070_e28977_d_n6, assign21070_e28977_d_n7, assign21070_e28977_d_n10, assign21070_e28977_d_n11, assign21070_e28977_d_n12, assign21070_e28977_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21070_e28973: f64 = (7.0 * 1.414213562373095);
        let assign21070_e28975: f64 = (assign21070_e28973 - locals.var_t5);
        (assign21070_e28975, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign21070_e28977;
        locals.var_ac31_dn0 = assign21070_e28977_d_n0;
        locals.var_ac31_dn2 = assign21070_e28977_d_n2;
        locals.var_ac31_dn6 = assign21070_e28977_d_n6;
        locals.var_ac31_dn7 = assign21070_e28977_d_n7;
        locals.var_ac31_dn10 = assign21070_e28977_d_n10;
        locals.var_ac31_dn11 = assign21070_e28977_d_n11;
        locals.var_ac31_dn12 = assign21070_e28977_d_n12;
        locals.var_ac31_dn17 = assign21070_e28977_d_n17;
        locals.var_ac31_rv = 0.0;

        let (assign21080_e28987, assign21080_e28987_d_n0, assign21080_e28987_d_n2, assign21080_e28987_d_n6, assign21080_e28987_d_n7, assign21080_e28987_d_n10, assign21080_e28987_d_n11, assign21080_e28987_d_n12, assign21080_e28987_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21080_e28985: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign21080_e28985, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign21080_e28987;
        locals.var_ac3_dn0 = assign21080_e28987_d_n0;
        locals.var_ac3_dn2 = assign21080_e28987_d_n2;
        locals.var_ac3_dn6 = assign21080_e28987_d_n6;
        locals.var_ac3_dn7 = assign21080_e28987_d_n7;
        locals.var_ac3_dn10 = assign21080_e28987_d_n10;
        locals.var_ac3_dn11 = assign21080_e28987_d_n11;
        locals.var_ac3_dn12 = assign21080_e28987_d_n12;
        locals.var_ac3_dn17 = assign21080_e28987_d_n17;
        locals.var_ac3_rv = 0.0;

        let assign21090_e28991: f64 = (locals.var_ac3 * 1e-8);
        let assign21090_e28992: f64 = if locals.var_ac4 < assign21090_e28991 { 1.0 } else { 0.0 };
        locals.var_guard648 = assign21090_e28992;
        locals.var_guard648_rv = 0.0;

        let (assign21100_e29015, assign21100_e29015_d_n0, assign21100_e29015_d_n2, assign21100_e29015_d_n6, assign21100_e29015_d_n7, assign21100_e29015_d_n10, assign21100_e29015_d_n11, assign21100_e29015_d_n12, assign21100_e29015_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign21100_e29001: f64 = (-7.0);
        let assign21100_e29003: f64 = (assign21100_e29001 * 1.414213562373095);
        let assign21100_e29005: f64 = (assign21100_e29003 + locals.var_ac31);
        let assign21100_e29008: f64 = (0.5 * locals.var_ac4);
        let assign21100_e29010: f64 = (assign21100_e29008 / locals.var_ac31);
        let assign21100_e29011: f64 = (assign21100_e29005 + assign21100_e29010);
        let assign21100_e29013: f64 = (assign21100_e29011 + locals.var_t5);
        (assign21100_e29013, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign21100_e29008 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign21100_e29015;
        locals.var_ac1_dn0 = assign21100_e29015_d_n0;
        locals.var_ac1_dn2 = assign21100_e29015_d_n2;
        locals.var_ac1_dn6 = assign21100_e29015_d_n6;
        locals.var_ac1_dn7 = assign21100_e29015_d_n7;
        locals.var_ac1_dn10 = assign21100_e29015_d_n10;
        locals.var_ac1_dn11 = assign21100_e29015_d_n11;
        locals.var_ac1_dn12 = assign21100_e29015_d_n12;
        locals.var_ac1_dn17 = assign21100_e29015_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign21110_e29029, assign21110_e29029_d_n0, assign21110_e29029_d_n2, assign21110_e29029_d_n6, assign21110_e29029_d_n7, assign21110_e29029_d_n10, assign21110_e29029_d_n11, assign21110_e29029_d_n12, assign21110_e29029_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 == 0.0)) {
        let assign21110_e29026: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign21110_e29027: f64 = (assign21110_e29026).sqrt();
        (assign21110_e29027, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign21110_e29027)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign21110_e29027)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign21110_e29029;
        locals.var_ac2_dn0 = assign21110_e29029_d_n0;
        locals.var_ac2_dn2 = assign21110_e29029_d_n2;
        locals.var_ac2_dn6 = assign21110_e29029_d_n6;
        locals.var_ac2_dn7 = assign21110_e29029_d_n7;
        locals.var_ac2_dn10 = assign21110_e29029_d_n10;
        locals.var_ac2_dn11 = assign21110_e29029_d_n11;
        locals.var_ac2_dn12 = assign21110_e29029_d_n12;
        locals.var_ac2_dn17 = assign21110_e29029_d_n17;
        locals.var_ac2_rv = 0.0;

        let (assign21120_e29047, assign21120_e29047_d_n0, assign21120_e29047_d_n2, assign21120_e29047_d_n6, assign21120_e29047_d_n7, assign21120_e29047_d_n10, assign21120_e29047_d_n11, assign21120_e29047_d_n12, assign21120_e29047_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 == 0.0)) {
        let assign21120_e29039: f64 = (-7.0);
        let assign21120_e29041: f64 = (assign21120_e29039 * 1.414213562373095);
        let assign21120_e29043: f64 = (assign21120_e29041 + locals.var_ac2);
        let assign21120_e29045: f64 = (assign21120_e29043 + locals.var_t5);
        (assign21120_e29045, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign21120_e29047;
        locals.var_ac1_dn0 = assign21120_e29047_d_n0;
        locals.var_ac1_dn2 = assign21120_e29047_d_n2;
        locals.var_ac1_dn6 = assign21120_e29047_d_n6;
        locals.var_ac1_dn7 = assign21120_e29047_d_n7;
        locals.var_ac1_dn10 = assign21120_e29047_d_n10;
        locals.var_ac1_dn11 = assign21120_e29047_d_n11;
        locals.var_ac1_dn12 = assign21120_e29047_d_n12;
        locals.var_ac1_dn17 = assign21120_e29047_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign21130_e29057, assign21130_e29057_d_n0, assign21130_e29057_d_n2, assign21130_e29057_d_n6, assign21130_e29057_d_n7, assign21130_e29057_d_n10, assign21130_e29057_d_n11, assign21130_e29057_d_n12, assign21130_e29057_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21130_e29055: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign21130_e29055, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign21130_e29055 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign21130_e29057;
        locals.var_acd_dn0 = assign21130_e29057_d_n0;
        locals.var_acd_dn2 = assign21130_e29057_d_n2;
        locals.var_acd_dn6 = assign21130_e29057_d_n6;
        locals.var_acd_dn7 = assign21130_e29057_d_n7;
        locals.var_acd_dn10 = assign21130_e29057_d_n10;
        locals.var_acd_dn11 = assign21130_e29057_d_n11;
        locals.var_acd_dn12 = assign21130_e29057_d_n12;
        locals.var_acd_dn17 = assign21130_e29057_d_n17;
        locals.var_acd_rv = 0.0;

        let (assign21140_e29082, assign21140_e29082_d_n0, assign21140_e29082_d_n2, assign21140_e29082_d_n6, assign21140_e29082_d_n7, assign21140_e29082_d_n10, assign21140_e29082_d_n11, assign21140_e29082_d_n12, assign21140_e29082_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21140_e29064: f64 = (-4.0);
        let assign21140_e29066: f64 = (assign21140_e29064 * 1.414213562373095);
        let assign21140_e29069: f64 = (12.0 * locals.var_ty);
        let assign21140_e29070: f64 = (assign21140_e29066 - assign21140_e29069);
        let assign21140_e29073: f64 = (2.0 * locals.var_acd);
        let assign21140_e29074: f64 = (assign21140_e29070 + assign21140_e29073);
        let assign21140_e29077: f64 = (1.414213562373095 * locals.var_acd);
        let assign21140_e29079: f64 = (assign21140_e29077 * locals.var_acd);
        let assign21140_e29080: f64 = (assign21140_e29074 + assign21140_e29079);
        (assign21140_e29080, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign21140_e29077 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign21140_e29082;
        locals.var_acn_dn0 = assign21140_e29082_d_n0;
        locals.var_acn_dn2 = assign21140_e29082_d_n2;
        locals.var_acn_dn6 = assign21140_e29082_d_n6;
        locals.var_acn_dn7 = assign21140_e29082_d_n7;
        locals.var_acn_dn10 = assign21140_e29082_d_n10;
        locals.var_acn_dn11 = assign21140_e29082_d_n11;
        locals.var_acn_dn12 = assign21140_e29082_d_n12;
        locals.var_acn_dn17 = assign21140_e29082_d_n17;
        locals.var_acn_rv = 0.0;

        let (assign21150_e29092, assign21150_e29092_d_n0, assign21150_e29092_d_n2, assign21150_e29092_d_n6, assign21150_e29092_d_n7, assign21150_e29092_d_n10, assign21150_e29092_d_n11, assign21150_e29092_d_n12, assign21150_e29092_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21150_e29090: f64 = (1.0 / locals.var_acd);
        (assign21150_e29090, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21150_e29092;
        locals.var_t1_dn0 = assign21150_e29092_d_n0;
        locals.var_t1_dn2 = assign21150_e29092_d_n2;
        locals.var_t1_dn6 = assign21150_e29092_d_n6;
        locals.var_t1_dn7 = assign21150_e29092_d_n7;
        locals.var_t1_dn10 = assign21150_e29092_d_n10;
        locals.var_t1_dn11 = assign21150_e29092_d_n11;
        locals.var_t1_dn12 = assign21150_e29092_d_n12;
        locals.var_t1_dn17 = assign21150_e29092_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21160_e29102, assign21160_e29102_d_n0, assign21160_e29102_d_n2, assign21160_e29102_d_n6, assign21160_e29102_d_n7, assign21160_e29102_d_n10, assign21160_e29102_d_n11, assign21160_e29102_d_n12, assign21160_e29102_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21160_e29100: f64 = (locals.var_acn * locals.var_t1);
        (assign21160_e29100, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign21160_e29102;
        locals.var_chi_dn0 = assign21160_e29102_d_n0;
        locals.var_chi_dn2 = assign21160_e29102_d_n2;
        locals.var_chi_dn6 = assign21160_e29102_d_n6;
        locals.var_chi_dn7 = assign21160_e29102_d_n7;
        locals.var_chi_dn10 = assign21160_e29102_d_n10;
        locals.var_chi_dn11 = assign21160_e29102_d_n11;
        locals.var_chi_dn12 = assign21160_e29102_d_n12;
        locals.var_chi_dn17 = assign21160_e29102_d_n17;
        locals.var_chi_rv = 0.0;

        let (assign21170_e29114, assign21170_e29114_d_n0, assign21170_e29114_d_n2, assign21170_e29114_d_n6, assign21170_e29114_d_n7, assign21170_e29114_d_n10, assign21170_e29114_d_n11, assign21170_e29114_d_n12, assign21170_e29114_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21170_e29110: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign21170_e29112: f64 = (assign21170_e29110 + locals.var_vbcs_cl__blk645);
        (assign21170_e29112, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl__blk645_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl__blk645_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign21170_e29114;
        locals.var_psa_dn0 = assign21170_e29114_d_n0;
        locals.var_psa_dn2 = assign21170_e29114_d_n2;
        locals.var_psa_dn6 = assign21170_e29114_d_n6;
        locals.var_psa_dn7 = assign21170_e29114_d_n7;
        locals.var_psa_dn10 = assign21170_e29114_d_n10;
        locals.var_psa_dn11 = assign21170_e29114_d_n11;
        locals.var_psa_dn12 = assign21170_e29114_d_n12;
        locals.var_psa_dn17 = assign21170_e29114_d_n17;
        locals.var_psa_rv = 0.0;

        let (assign21180_e29124, assign21180_e29124_d_n0, assign21180_e29124_d_n2, assign21180_e29124_d_n6, assign21180_e29124_d_n7, assign21180_e29124_d_n10, assign21180_e29124_d_n11, assign21180_e29124_d_n12, assign21180_e29124_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21180_e29122: f64 = (locals.var_psa - locals.var_vbcs_cl__blk645);
        (assign21180_e29122, (locals.var_psa_dn0 - locals.var_vbcs_cl__blk645_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl__blk645_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl__blk645_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl__blk645_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl__blk645_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl__blk645_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl__blk645_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl__blk645_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21180_e29124;
        locals.var_t1_dn0 = assign21180_e29124_d_n0;
        locals.var_t1_dn2 = assign21180_e29124_d_n2;
        locals.var_t1_dn6 = assign21180_e29124_d_n6;
        locals.var_t1_dn7 = assign21180_e29124_d_n7;
        locals.var_t1_dn10 = assign21180_e29124_d_n10;
        locals.var_t1_dn11 = assign21180_e29124_d_n11;
        locals.var_t1_dn12 = assign21180_e29124_d_n12;
        locals.var_t1_dn17 = assign21180_e29124_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21190_e29134, assign21190_e29134_d_n0, assign21190_e29134_d_n2, assign21190_e29134_d_n6, assign21190_e29134_d_n7, assign21190_e29134_d_n10, assign21190_e29134_d_n11, assign21190_e29134_d_n12, assign21190_e29134_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21190_e29132: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign21190_e29132, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21190_e29134;
        locals.var_t2_dn0 = assign21190_e29134_d_n0;
        locals.var_t2_dn2 = assign21190_e29134_d_n2;
        locals.var_t2_dn6 = assign21190_e29134_d_n6;
        locals.var_t2_dn7 = assign21190_e29134_d_n7;
        locals.var_t2_dn10 = assign21190_e29134_d_n10;
        locals.var_t2_dn11 = assign21190_e29134_d_n11;
        locals.var_t2_dn12 = assign21190_e29134_d_n12;
        locals.var_t2_dn17 = assign21190_e29134_d_n17;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21200_e29147, assign21200_e29147_d_n0, assign21200_e29147_d_n2, assign21200_e29147_d_n6, assign21200_e29147_d_n7, assign21200_e29147_d_n10, assign21200_e29147_d_n11, assign21200_e29147_d_n12, assign21200_e29147_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21200_e29143: f64 = (locals.var_t2 * locals.var_t2);
        let assign21200_e29144: f64 = (1.0 + assign21200_e29143);
        let assign21200_e29145: f64 = (assign21200_e29144).sqrt();
        (assign21200_e29145, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign21200_e29145)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign21200_e29145)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign21200_e29147;
        locals.var_t3_dn0 = assign21200_e29147_d_n0;
        locals.var_t3_dn2 = assign21200_e29147_d_n2;
        locals.var_t3_dn6 = assign21200_e29147_d_n6;
        locals.var_t3_dn7 = assign21200_e29147_d_n7;
        locals.var_t3_dn10 = assign21200_e29147_d_n10;
        locals.var_t3_dn11 = assign21200_e29147_d_n11;
        locals.var_t3_dn12 = assign21200_e29147_d_n12;
        locals.var_t3_dn17 = assign21200_e29147_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign21210_e29159, assign21210_e29159_d_n0, assign21210_e29159_d_n2, assign21210_e29159_d_n6, assign21210_e29159_d_n7, assign21210_e29159_d_n10, assign21210_e29159_d_n11, assign21210_e29159_d_n12, assign21210_e29159_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21210_e29155: f64 = (locals.var_t1 / locals.var_t3);
        let assign21210_e29157: f64 = (assign21210_e29155 + locals.var_vbcs_cl__blk645);
        (assign21210_e29157, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk645_dn17),)
    } else {
        (locals.var_ps0__blk608, locals.var_ps0__blk608_dn0, locals.var_ps0__blk608_dn2, locals.var_ps0__blk608_dn6, locals.var_ps0__blk608_dn7, locals.var_ps0__blk608_dn10, locals.var_ps0__blk608_dn11, locals.var_ps0__blk608_dn12, locals.var_ps0__blk608_dn17,)
    }
};
        locals.var_ps0__blk608 = assign21210_e29159;
        locals.var_ps0__blk608_dn0 = assign21210_e29159_d_n0;
        locals.var_ps0__blk608_dn2 = assign21210_e29159_d_n2;
        locals.var_ps0__blk608_dn6 = assign21210_e29159_d_n6;
        locals.var_ps0__blk608_dn7 = assign21210_e29159_d_n7;
        locals.var_ps0__blk608_dn10 = assign21210_e29159_d_n10;
        locals.var_ps0__blk608_dn11 = assign21210_e29159_d_n11;
        locals.var_ps0__blk608_dn12 = assign21210_e29159_d_n12;
        locals.var_ps0__blk608_dn17 = assign21210_e29159_d_n17;
        locals.var_ps0__blk608_rv = 0.0;

        let (assign21220_e29173, assign21220_e29173_d_n0, assign21220_e29173_d_n2, assign21220_e29173_d_n6, assign21220_e29173_d_n7, assign21220_e29173_d_n10, assign21220_e29173_d_n11, assign21220_e29173_d_n12, assign21220_e29173_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21220_e29169: f64 = (locals.var_vbcs_cl__blk645 - p.p287);
        let assign21220_e29170: f64 = (locals.var_beta * assign21220_e29169);
        let assign21220_e29171: f64 = (assign21220_e29170).exp();
        (assign21220_e29171, (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn0)), (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn2)), (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn6)), (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn7)), (assign21220_e29171 * ((locals.var_beta_dn10 * assign21220_e29169) + (locals.var_beta * locals.var_vbcs_cl__blk645_dn10))), (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn11)), (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn12)), (assign21220_e29171 * (locals.var_beta * locals.var_vbcs_cl__blk645_dn17)),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign21220_e29173;
        locals.var_exp_bvbsvds_dn0 = assign21220_e29173_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign21220_e29173_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign21220_e29173_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign21220_e29173_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign21220_e29173_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign21220_e29173_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign21220_e29173_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign21220_e29173_d_n17;
        locals.var_exp_bvbsvds_rv = 0.0;

        let (assign21230_e29182,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign21230_e29182;
        locals.var_flg_conv_rv = 0.0;

        let (assign21240_e29191, assign21240_e29191_d_n0, assign21240_e29191_d_n2, assign21240_e29191_d_n6, assign21240_e29191_d_n7, assign21240_e29191_d_n10, assign21240_e29191_d_n11, assign21240_e29191_d_n12, assign21240_e29191_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi__blk646, locals.var_phi_s0_soi__blk646_dn0, locals.var_phi_s0_soi__blk646_dn2, locals.var_phi_s0_soi__blk646_dn6, locals.var_phi_s0_soi__blk646_dn7, locals.var_phi_s0_soi__blk646_dn10, locals.var_phi_s0_soi__blk646_dn11, locals.var_phi_s0_soi__blk646_dn12, locals.var_phi_s0_soi__blk646_dn17,)
    }
};
        locals.var_phi_s0_soi__blk646 = assign21240_e29191;
        locals.var_phi_s0_soi__blk646_dn0 = assign21240_e29191_d_n0;
        locals.var_phi_s0_soi__blk646_dn2 = assign21240_e29191_d_n2;
        locals.var_phi_s0_soi__blk646_dn6 = assign21240_e29191_d_n6;
        locals.var_phi_s0_soi__blk646_dn7 = assign21240_e29191_d_n7;
        locals.var_phi_s0_soi__blk646_dn10 = assign21240_e29191_d_n10;
        locals.var_phi_s0_soi__blk646_dn11 = assign21240_e29191_d_n11;
        locals.var_phi_s0_soi__blk646_dn12 = assign21240_e29191_d_n12;
        locals.var_phi_s0_soi__blk646_dn17 = assign21240_e29191_d_n17;
        locals.var_phi_s0_soi__blk646_rv = 0.0;

        let (assign21250_e29208, assign21250_e29208_d_n0, assign21250_e29208_d_n2, assign21250_e29208_d_n6, assign21250_e29208_d_n7, assign21250_e29208_d_n10, assign21250_e29208_d_n11, assign21250_e29208_d_n12, assign21250_e29208_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21250_e29200: f64 = (locals.var_q_nsub * p.p237);
        let assign21250_e29202: f64 = (assign21250_e29200 * p.p237);
        let assign21250_e29204: f64 = (assign21250_e29202 / 2.0);
        let assign21250_e29206: f64 = (assign21250_e29204 / 1.034943e-10);
        (assign21250_e29206, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign21250_e29208;
        locals.var_dphi_sb_dn0 = assign21250_e29208_d_n0;
        locals.var_dphi_sb_dn2 = assign21250_e29208_d_n2;
        locals.var_dphi_sb_dn6 = assign21250_e29208_d_n6;
        locals.var_dphi_sb_dn7 = assign21250_e29208_d_n7;
        locals.var_dphi_sb_dn10 = assign21250_e29208_d_n10;
        locals.var_dphi_sb_dn11 = assign21250_e29208_d_n11;
        locals.var_dphi_sb_dn12 = assign21250_e29208_d_n12;
        locals.var_dphi_sb_dn17 = assign21250_e29208_d_n17;
        locals.var_dphi_sb_rv = 0.0;

        let (assign21260_e29222, assign21260_e29222_d_n0, assign21260_e29222_d_n2, assign21260_e29222_d_n6, assign21260_e29222_d_n7, assign21260_e29222_d_n10, assign21260_e29222_d_n11, assign21260_e29222_d_n12, assign21260_e29222_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21260_e29217: f64 = (2.0 * locals.var_beta);
        let assign21260_e29219: f64 = (assign21260_e29217 * locals.var_dphi_sb);
        let assign21260_e29220: f64 = (assign21260_e29219).sqrt();
        (assign21260_e29220, ((assign21260_e29217 * locals.var_dphi_sb_dn0) / (2.0 * assign21260_e29220)), ((assign21260_e29217 * locals.var_dphi_sb_dn2) / (2.0 * assign21260_e29220)), ((assign21260_e29217 * locals.var_dphi_sb_dn6) / (2.0 * assign21260_e29220)), ((assign21260_e29217 * locals.var_dphi_sb_dn7) / (2.0 * assign21260_e29220)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign21260_e29217 * locals.var_dphi_sb_dn10)) / (2.0 * assign21260_e29220)), ((assign21260_e29217 * locals.var_dphi_sb_dn11) / (2.0 * assign21260_e29220)), ((assign21260_e29217 * locals.var_dphi_sb_dn12) / (2.0 * assign21260_e29220)), ((assign21260_e29217 * locals.var_dphi_sb_dn17) / (2.0 * assign21260_e29220)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign21260_e29222;
        locals.var_t0_dn0 = assign21260_e29222_d_n0;
        locals.var_t0_dn2 = assign21260_e29222_d_n2;
        locals.var_t0_dn6 = assign21260_e29222_d_n6;
        locals.var_t0_dn7 = assign21260_e29222_d_n7;
        locals.var_t0_dn10 = assign21260_e29222_d_n10;
        locals.var_t0_dn11 = assign21260_e29222_d_n11;
        locals.var_t0_dn12 = assign21260_e29222_d_n12;
        locals.var_t0_dn17 = assign21260_e29222_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign21270_e29238, assign21270_e29238_d_n0, assign21270_e29238_d_n2, assign21270_e29238_d_n6, assign21270_e29238_d_n7, assign21270_e29238_d_n10, assign21270_e29238_d_n11, assign21270_e29238_d_n12, assign21270_e29238_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21270_e29230: f64 = (locals.var_t0).exp();
        let assign21270_e29232: f64 = (-locals.var_t0);
        let assign21270_e29233: f64 = (assign21270_e29232).exp();
        let assign21270_e29234: f64 = (assign21270_e29230 + assign21270_e29233);
        let assign21270_e29236: f64 = (assign21270_e29234 / 2.0);
        (assign21270_e29236, (((assign21270_e29230 * locals.var_t0_dn0) + (assign21270_e29233 * (-locals.var_t0_dn0))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn2) + (assign21270_e29233 * (-locals.var_t0_dn2))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn6) + (assign21270_e29233 * (-locals.var_t0_dn6))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn7) + (assign21270_e29233 * (-locals.var_t0_dn7))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn10) + (assign21270_e29233 * (-locals.var_t0_dn10))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn11) + (assign21270_e29233 * (-locals.var_t0_dn11))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn12) + (assign21270_e29233 * (-locals.var_t0_dn12))) / 2.0), (((assign21270_e29230 * locals.var_t0_dn17) + (assign21270_e29233 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21270_e29238;
        locals.var_t1_dn0 = assign21270_e29238_d_n0;
        locals.var_t1_dn2 = assign21270_e29238_d_n2;
        locals.var_t1_dn6 = assign21270_e29238_d_n6;
        locals.var_t1_dn7 = assign21270_e29238_d_n7;
        locals.var_t1_dn10 = assign21270_e29238_d_n10;
        locals.var_t1_dn11 = assign21270_e29238_d_n11;
        locals.var_t1_dn12 = assign21270_e29238_d_n12;
        locals.var_t1_dn17 = assign21270_e29238_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21280_e29250, assign21280_e29250_d_n0, assign21280_e29250_d_n2, assign21280_e29250_d_n6, assign21280_e29250_d_n7, assign21280_e29250_d_n10, assign21280_e29250_d_n11, assign21280_e29250_d_n12, assign21280_e29250_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21280_e29246: f64 = (locals.var_t1).ln();
        let assign21280_e29248: f64 = (assign21280_e29246 / locals.var_dphi_sb);
        (assign21280_e29248, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign21280_e29246 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign21280_e29250;
        locals.var_c_sb_dn0 = assign21280_e29250_d_n0;
        locals.var_c_sb_dn2 = assign21280_e29250_d_n2;
        locals.var_c_sb_dn6 = assign21280_e29250_d_n6;
        locals.var_c_sb_dn7 = assign21280_e29250_d_n7;
        locals.var_c_sb_dn10 = assign21280_e29250_d_n10;
        locals.var_c_sb_dn11 = assign21280_e29250_d_n11;
        locals.var_c_sb_dn12 = assign21280_e29250_d_n12;
        locals.var_c_sb_dn17 = assign21280_e29250_d_n17;
        locals.var_c_sb_rv = 0.0;

        let (assign21290_e29259,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign21290_e29259;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign21300_loop_guard: usize = 0;
        while {
            let assign21300_cond_e29269: f64 = (locals.var_lp_s0_max + 1.0);
            let assign21300_cond_e29271: f64 = if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_lp_s0 <= assign21300_cond_e29269)) { 1.0 } else { 0.0 };
            assign21300_cond_e29271 != 0.0
        } {
            assign21300_loop_guard += 1;
            assert!(assign21300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign21300_body0_e29282, assign21300_body0_e29282_d_n0, assign21300_body0_e29282_d_n2, assign21300_body0_e29282_d_n6, assign21300_body0_e29282_d_n7, assign21300_body0_e29282_d_n10, assign21300_body0_e29282_d_n11, assign21300_body0_e29282_d_n12, assign21300_body0_e29282_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body0_e29280: f64 = (locals.var_phi_s0_soi__blk646 - locals.var_vbcs_cl__blk645);
        (assign21300_body0_e29280, (locals.var_phi_s0_soi__blk646_dn0 - locals.var_vbcs_cl__blk645_dn0), (locals.var_phi_s0_soi__blk646_dn2 - locals.var_vbcs_cl__blk645_dn2), (locals.var_phi_s0_soi__blk646_dn6 - locals.var_vbcs_cl__blk645_dn6), (locals.var_phi_s0_soi__blk646_dn7 - locals.var_vbcs_cl__blk645_dn7), (locals.var_phi_s0_soi__blk646_dn10 - locals.var_vbcs_cl__blk645_dn10), (locals.var_phi_s0_soi__blk646_dn11 - locals.var_vbcs_cl__blk645_dn11), (locals.var_phi_s0_soi__blk646_dn12 - locals.var_vbcs_cl__blk645_dn12), (locals.var_phi_s0_soi__blk646_dn17 - locals.var_vbcs_cl__blk645_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign21300_body0_e29282;
            locals.var_phi_soi0_dn0 = assign21300_body0_e29282_d_n0;
            locals.var_phi_soi0_dn2 = assign21300_body0_e29282_d_n2;
            locals.var_phi_soi0_dn6 = assign21300_body0_e29282_d_n6;
            locals.var_phi_soi0_dn7 = assign21300_body0_e29282_d_n7;
            locals.var_phi_soi0_dn10 = assign21300_body0_e29282_d_n10;
            locals.var_phi_soi0_dn11 = assign21300_body0_e29282_d_n11;
            locals.var_phi_soi0_dn12 = assign21300_body0_e29282_d_n12;
            locals.var_phi_soi0_dn17 = assign21300_body0_e29282_d_n17;
            locals.var_phi_soi0_rv = 0.0;
            let (assign21300_body1_e29293, assign21300_body1_e29293_d_n0, assign21300_body1_e29293_d_n2, assign21300_body1_e29293_d_n6, assign21300_body1_e29293_d_n7, assign21300_body1_e29293_d_n10, assign21300_body1_e29293_d_n11, assign21300_body1_e29293_d_n12, assign21300_body1_e29293_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body1_e29291: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign21300_body1_e29291, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign21300_body1_e29293;
            locals.var_chi_dn0 = assign21300_body1_e29293_d_n0;
            locals.var_chi_dn2 = assign21300_body1_e29293_d_n2;
            locals.var_chi_dn6 = assign21300_body1_e29293_d_n6;
            locals.var_chi_dn7 = assign21300_body1_e29293_d_n7;
            locals.var_chi_dn10 = assign21300_body1_e29293_d_n10;
            locals.var_chi_dn11 = assign21300_body1_e29293_d_n11;
            locals.var_chi_dn12 = assign21300_body1_e29293_d_n12;
            locals.var_chi_dn17 = assign21300_body1_e29293_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign21300_body2_e29306, assign21300_body2_e29306_d_n0, assign21300_body2_e29306_d_n2, assign21300_body2_e29306_d_n6, assign21300_body2_e29306_d_n7, assign21300_body2_e29306_d_n10, assign21300_body2_e29306_d_n11, assign21300_body2_e29306_d_n12, assign21300_body2_e29306_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body2_e29303: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign21300_body2_e29304: f64 = (locals.var_c_sb * assign21300_body2_e29303);
        (assign21300_body2_e29304, ((locals.var_c_sb_dn0 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign21300_body2_e29303) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign21300_body2_e29306;
            locals.var_ty_dn0 = assign21300_body2_e29306_d_n0;
            locals.var_ty_dn2 = assign21300_body2_e29306_d_n2;
            locals.var_ty_dn6 = assign21300_body2_e29306_d_n6;
            locals.var_ty_dn7 = assign21300_body2_e29306_d_n7;
            locals.var_ty_dn10 = assign21300_body2_e29306_d_n10;
            locals.var_ty_dn11 = assign21300_body2_e29306_d_n11;
            locals.var_ty_dn12 = assign21300_body2_e29306_d_n12;
            locals.var_ty_dn17 = assign21300_body2_e29306_d_n17;
            locals.var_ty_rv = 0.0;
            let assign21300_body3_e29309: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard649 = assign21300_body3_e29309;
            locals.var_guard649_rv = 0.0;
            let (assign21300_body4_e29321, assign21300_body4_e29321_d_n0, assign21300_body4_e29321_d_n2, assign21300_body4_e29321_d_n6, assign21300_body4_e29321_d_n7, assign21300_body4_e29321_d_n10, assign21300_body4_e29321_d_n11, assign21300_body4_e29321_d_n12, assign21300_body4_e29321_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21300_body4_e29319: f64 = (locals.var_ty).exp();
        (assign21300_body4_e29319, (assign21300_body4_e29319 * locals.var_ty_dn0), (assign21300_body4_e29319 * locals.var_ty_dn2), (assign21300_body4_e29319 * locals.var_ty_dn6), (assign21300_body4_e29319 * locals.var_ty_dn7), (assign21300_body4_e29319 * locals.var_ty_dn10), (assign21300_body4_e29319 * locals.var_ty_dn11), (assign21300_body4_e29319 * locals.var_ty_dn12), (assign21300_body4_e29319 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21300_body4_e29321;
            locals.var_t1_dn0 = assign21300_body4_e29321_d_n0;
            locals.var_t1_dn2 = assign21300_body4_e29321_d_n2;
            locals.var_t1_dn6 = assign21300_body4_e29321_d_n6;
            locals.var_t1_dn7 = assign21300_body4_e29321_d_n7;
            locals.var_t1_dn10 = assign21300_body4_e29321_d_n10;
            locals.var_t1_dn11 = assign21300_body4_e29321_d_n11;
            locals.var_t1_dn12 = assign21300_body4_e29321_d_n12;
            locals.var_t1_dn17 = assign21300_body4_e29321_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21300_body5_e29336, assign21300_body5_e29336_d_n0, assign21300_body5_e29336_d_n2, assign21300_body5_e29336_d_n6, assign21300_body5_e29336_d_n7, assign21300_body5_e29336_d_n10, assign21300_body5_e29336_d_n11, assign21300_body5_e29336_d_n12, assign21300_body5_e29336_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21300_body5_e29331: f64 = (-locals.var_c_sb);
        let assign21300_body5_e29333: f64 = (assign21300_body5_e29331 * locals.var_dphi_sb);
        let assign21300_body5_e29334: f64 = (assign21300_body5_e29333).exp();
        (assign21300_body5_e29334, (assign21300_body5_e29334 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn0))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn2))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn6))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn7))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn10))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn11))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn12))), (assign21300_body5_e29334 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign21300_body5_e29331 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21300_body5_e29336;
            locals.var_t0_dn0 = assign21300_body5_e29336_d_n0;
            locals.var_t0_dn2 = assign21300_body5_e29336_d_n2;
            locals.var_t0_dn6 = assign21300_body5_e29336_d_n6;
            locals.var_t0_dn7 = assign21300_body5_e29336_d_n7;
            locals.var_t0_dn10 = assign21300_body5_e29336_d_n10;
            locals.var_t0_dn11 = assign21300_body5_e29336_d_n11;
            locals.var_t0_dn12 = assign21300_body5_e29336_d_n12;
            locals.var_t0_dn17 = assign21300_body5_e29336_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21300_body6_e29349, assign21300_body6_e29349_d_n0, assign21300_body6_e29349_d_n2, assign21300_body6_e29349_d_n6, assign21300_body6_e29349_d_n7, assign21300_body6_e29349_d_n10, assign21300_body6_e29349_d_n11, assign21300_body6_e29349_d_n12, assign21300_body6_e29349_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21300_body6_e29347: f64 = (locals.var_t1 - locals.var_t0);
        (assign21300_body6_e29347, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign21300_body6_e29349;
            locals.var_t2_dn0 = assign21300_body6_e29349_d_n0;
            locals.var_t2_dn2 = assign21300_body6_e29349_d_n2;
            locals.var_t2_dn6 = assign21300_body6_e29349_d_n6;
            locals.var_t2_dn7 = assign21300_body6_e29349_d_n7;
            locals.var_t2_dn10 = assign21300_body6_e29349_d_n10;
            locals.var_t2_dn11 = assign21300_body6_e29349_d_n11;
            locals.var_t2_dn12 = assign21300_body6_e29349_d_n12;
            locals.var_t2_dn17 = assign21300_body6_e29349_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign21300_body7_e29365, assign21300_body7_e29365_d_n0, assign21300_body7_e29365_d_n2, assign21300_body7_e29365_d_n6, assign21300_body7_e29365_d_n7, assign21300_body7_e29365_d_n10, assign21300_body7_e29365_d_n11, assign21300_body7_e29365_d_n12, assign21300_body7_e29365_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21300_body7_e29360: f64 = (1.0 + locals.var_t2);
        let assign21300_body7_e29361: f64 = (assign21300_body7_e29360).ln();
        let assign21300_body7_e29363: f64 = (assign21300_body7_e29361 / locals.var_c_sb);
        (assign21300_body7_e29363, ((((locals.var_t2_dn0 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign21300_body7_e29360) * locals.var_c_sb) - (assign21300_body7_e29361 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign21300_body7_e29365;
            locals.var_phi_soib_dn0 = assign21300_body7_e29365_d_n0;
            locals.var_phi_soib_dn2 = assign21300_body7_e29365_d_n2;
            locals.var_phi_soib_dn6 = assign21300_body7_e29365_d_n6;
            locals.var_phi_soib_dn7 = assign21300_body7_e29365_d_n7;
            locals.var_phi_soib_dn10 = assign21300_body7_e29365_d_n10;
            locals.var_phi_soib_dn11 = assign21300_body7_e29365_d_n11;
            locals.var_phi_soib_dn12 = assign21300_body7_e29365_d_n12;
            locals.var_phi_soib_dn17 = assign21300_body7_e29365_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign21300_body8_e29380, assign21300_body8_e29380_d_n0, assign21300_body8_e29380_d_n2, assign21300_body8_e29380_d_n6, assign21300_body8_e29380_d_n7, assign21300_body8_e29380_d_n10, assign21300_body8_e29380_d_n11, assign21300_body8_e29380_d_n12, assign21300_body8_e29380_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21300_body8_e29377: f64 = (1.0 + locals.var_t2);
        let assign21300_body8_e29378: f64 = (locals.var_t1 / assign21300_body8_e29377);
        (assign21300_body8_e29378, (((locals.var_t1_dn0 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn0)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn2 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn2)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn6 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn6)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn7 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn7)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn10 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn10)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn11 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn11)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn12 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn12)) / (assign21300_body8_e29377 * assign21300_body8_e29377)), (((locals.var_t1_dn17 * assign21300_body8_e29377) - (locals.var_t1 * locals.var_t2_dn17)) / (assign21300_body8_e29377 * assign21300_body8_e29377)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign21300_body8_e29380;
            locals.var_phi_soib_dpss_dn0 = assign21300_body8_e29380_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign21300_body8_e29380_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign21300_body8_e29380_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign21300_body8_e29380_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign21300_body8_e29380_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign21300_body8_e29380_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign21300_body8_e29380_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign21300_body8_e29380_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign21300_body9_e29394, assign21300_body9_e29394_d_n0, assign21300_body9_e29394_d_n2, assign21300_body9_e29394_d_n6, assign21300_body9_e29394_d_n7, assign21300_body9_e29394_d_n10, assign21300_body9_e29394_d_n11, assign21300_body9_e29394_d_n12, assign21300_body9_e29394_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21300_body9_e29392: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign21300_body9_e29392, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign21300_body9_e29394;
            locals.var_phi_soib_dn0 = assign21300_body9_e29394_d_n0;
            locals.var_phi_soib_dn2 = assign21300_body9_e29394_d_n2;
            locals.var_phi_soib_dn6 = assign21300_body9_e29394_d_n6;
            locals.var_phi_soib_dn7 = assign21300_body9_e29394_d_n7;
            locals.var_phi_soib_dn10 = assign21300_body9_e29394_d_n10;
            locals.var_phi_soib_dn11 = assign21300_body9_e29394_d_n11;
            locals.var_phi_soib_dn12 = assign21300_body9_e29394_d_n12;
            locals.var_phi_soib_dn17 = assign21300_body9_e29394_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign21300_body10_e29406, assign21300_body10_e29406_d_n0, assign21300_body10_e29406_d_n2, assign21300_body10_e29406_d_n6, assign21300_body10_e29406_d_n7, assign21300_body10_e29406_d_n10, assign21300_body10_e29406_d_n11, assign21300_body10_e29406_d_n12, assign21300_body10_e29406_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard649 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign21300_body10_e29406;
            locals.var_phi_soib_dpss_dn0 = assign21300_body10_e29406_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign21300_body10_e29406_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign21300_body10_e29406_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign21300_body10_e29406_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign21300_body10_e29406_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign21300_body10_e29406_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign21300_body10_e29406_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign21300_body10_e29406_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign21300_body11_e29417, assign21300_body11_e29417_d_n0, assign21300_body11_e29417_d_n2, assign21300_body11_e29417_d_n6, assign21300_body11_e29417_d_n7, assign21300_body11_e29417_d_n10, assign21300_body11_e29417_d_n11, assign21300_body11_e29417_d_n12, assign21300_body11_e29417_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body11_e29415: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign21300_body11_e29415, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign21300_body11_e29417;
            locals.var_chib_dn0 = assign21300_body11_e29417_d_n0;
            locals.var_chib_dn2 = assign21300_body11_e29417_d_n2;
            locals.var_chib_dn6 = assign21300_body11_e29417_d_n6;
            locals.var_chib_dn7 = assign21300_body11_e29417_d_n7;
            locals.var_chib_dn10 = assign21300_body11_e29417_d_n10;
            locals.var_chib_dn11 = assign21300_body11_e29417_d_n11;
            locals.var_chib_dn12 = assign21300_body11_e29417_d_n12;
            locals.var_chib_dn17 = assign21300_body11_e29417_d_n17;
            locals.var_chib_rv = 0.0;
            let assign21300_body12_e29419: f64 = (locals.var_chi).abs();
            let assign21300_body12_e29421: f64 = if assign21300_body12_e29419 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard650 = assign21300_body12_e29421;
            locals.var_guard650_rv = 0.0;
            let (assign21300_body13_e29439, assign21300_body13_e29439_d_n0, assign21300_body13_e29439_d_n2, assign21300_body13_e29439_d_n6, assign21300_body13_e29439_d_n7, assign21300_body13_e29439_d_n10, assign21300_body13_e29439_d_n11, assign21300_body13_e29439_d_n12, assign21300_body13_e29439_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21300_body13_e29433: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign21300_body13_e29434: f64 = (1.0 - assign21300_body13_e29433);
        let assign21300_body13_e29436: f64 = (assign21300_body13_e29434 / 2.0);
        let assign21300_body13_e29437: f64 = (assign21300_body13_e29436).sqrt();
        (assign21300_body13_e29437, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign21300_body13_e29437)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign21300_body13_e29437)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21300_body13_e29439;
            locals.var_t0_dn0 = assign21300_body13_e29439_d_n0;
            locals.var_t0_dn2 = assign21300_body13_e29439_d_n2;
            locals.var_t0_dn6 = assign21300_body13_e29439_d_n6;
            locals.var_t0_dn7 = assign21300_body13_e29439_d_n7;
            locals.var_t0_dn10 = assign21300_body13_e29439_d_n10;
            locals.var_t0_dn11 = assign21300_body13_e29439_d_n11;
            locals.var_t0_dn12 = assign21300_body13_e29439_d_n12;
            locals.var_t0_dn17 = assign21300_body13_e29439_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21300_body14_e29452, assign21300_body14_e29452_d_n0, assign21300_body14_e29452_d_n2, assign21300_body14_e29452_d_n6, assign21300_body14_e29452_d_n7, assign21300_body14_e29452_d_n10, assign21300_body14_e29452_d_n11, assign21300_body14_e29452_d_n12, assign21300_body14_e29452_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21300_body14_e29450: f64 = (locals.var_chi * locals.var_t0);
        (assign21300_body14_e29450, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21300_body14_e29452;
            locals.var_fb_dn0 = assign21300_body14_e29452_d_n0;
            locals.var_fb_dn2 = assign21300_body14_e29452_d_n2;
            locals.var_fb_dn6 = assign21300_body14_e29452_d_n6;
            locals.var_fb_dn7 = assign21300_body14_e29452_d_n7;
            locals.var_fb_dn10 = assign21300_body14_e29452_d_n10;
            locals.var_fb_dn11 = assign21300_body14_e29452_d_n11;
            locals.var_fb_dn12 = assign21300_body14_e29452_d_n12;
            locals.var_fb_dn17 = assign21300_body14_e29452_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21300_body15_e29465, assign21300_body15_e29465_d_n0, assign21300_body15_e29465_d_n2, assign21300_body15_e29465_d_n6, assign21300_body15_e29465_d_n7, assign21300_body15_e29465_d_n10, assign21300_body15_e29465_d_n11, assign21300_body15_e29465_d_n12, assign21300_body15_e29465_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21300_body15_e29463: f64 = (locals.var_beta * locals.var_t0);
        (assign21300_body15_e29463, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21300_body15_e29465;
            locals.var_fb_dpss_dn0 = assign21300_body15_e29465_d_n0;
            locals.var_fb_dpss_dn2 = assign21300_body15_e29465_d_n2;
            locals.var_fb_dpss_dn6 = assign21300_body15_e29465_d_n6;
            locals.var_fb_dpss_dn7 = assign21300_body15_e29465_d_n7;
            locals.var_fb_dpss_dn10 = assign21300_body15_e29465_d_n10;
            locals.var_fb_dpss_dn11 = assign21300_body15_e29465_d_n11;
            locals.var_fb_dpss_dn12 = assign21300_body15_e29465_d_n12;
            locals.var_fb_dpss_dn17 = assign21300_body15_e29465_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21300_body16_e29468: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard651 = assign21300_body16_e29468;
            locals.var_guard651_rv = 0.0;
            let (assign21300_body17_e29482, assign21300_body17_e29482_d_n0, assign21300_body17_e29482_d_n2, assign21300_body17_e29482_d_n6, assign21300_body17_e29482_d_n7, assign21300_body17_e29482_d_n10, assign21300_body17_e29482_d_n11, assign21300_body17_e29482_d_n12, assign21300_body17_e29482_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21300_body17_e29480: f64 = (-locals.var_fb);
        (assign21300_body17_e29480, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21300_body17_e29482;
            locals.var_fb_dn0 = assign21300_body17_e29482_d_n0;
            locals.var_fb_dn2 = assign21300_body17_e29482_d_n2;
            locals.var_fb_dn6 = assign21300_body17_e29482_d_n6;
            locals.var_fb_dn7 = assign21300_body17_e29482_d_n7;
            locals.var_fb_dn10 = assign21300_body17_e29482_d_n10;
            locals.var_fb_dn11 = assign21300_body17_e29482_d_n11;
            locals.var_fb_dn12 = assign21300_body17_e29482_d_n12;
            locals.var_fb_dn17 = assign21300_body17_e29482_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21300_body18_e29496, assign21300_body18_e29496_d_n0, assign21300_body18_e29496_d_n2, assign21300_body18_e29496_d_n6, assign21300_body18_e29496_d_n7, assign21300_body18_e29496_d_n10, assign21300_body18_e29496_d_n11, assign21300_body18_e29496_d_n12, assign21300_body18_e29496_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21300_body18_e29494: f64 = (-locals.var_fb_dpss);
        (assign21300_body18_e29494, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21300_body18_e29496;
            locals.var_fb_dpss_dn0 = assign21300_body18_e29496_d_n0;
            locals.var_fb_dpss_dn2 = assign21300_body18_e29496_d_n2;
            locals.var_fb_dpss_dn6 = assign21300_body18_e29496_d_n6;
            locals.var_fb_dpss_dn7 = assign21300_body18_e29496_d_n7;
            locals.var_fb_dpss_dn10 = assign21300_body18_e29496_d_n10;
            locals.var_fb_dpss_dn11 = assign21300_body18_e29496_d_n11;
            locals.var_fb_dpss_dn12 = assign21300_body18_e29496_d_n12;
            locals.var_fb_dpss_dn17 = assign21300_body18_e29496_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21300_body19_e29498: f64 = (locals.var_chi).abs();
            let assign21300_body19_e29500: f64 = if assign21300_body19_e29498 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard652 = assign21300_body19_e29500;
            locals.var_guard652_rv = 0.0;
            let (assign21300_body20_e29536, assign21300_body20_e29536_d_n0, assign21300_body20_e29536_d_n2, assign21300_body20_e29536_d_n6, assign21300_body20_e29536_d_n7, assign21300_body20_e29536_d_n10, assign21300_body20_e29536_d_n11, assign21300_body20_e29536_d_n12, assign21300_body20_e29536_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21300_body20_e29514: f64 = (locals.var_chi * locals.var_chi);
        let assign21300_body20_e29516: f64 = (assign21300_body20_e29514 / 2.0);
        let assign21300_body20_e29520: f64 = (locals.var_chi / 3.0);
        let assign21300_body20_e29524: f64 = (locals.var_chi / 4.0);
        let assign21300_body20_e29528: f64 = (locals.var_chi / 5.0);
        let assign21300_body20_e29529: f64 = (1.0 - assign21300_body20_e29528);
        let assign21300_body20_e29530: f64 = (assign21300_body20_e29524 * assign21300_body20_e29529);
        let assign21300_body20_e29531: f64 = (1.0 - assign21300_body20_e29530);
        let assign21300_body20_e29532: f64 = (assign21300_body20_e29520 * assign21300_body20_e29531);
        let assign21300_body20_e29533: f64 = (1.0 - assign21300_body20_e29532);
        let assign21300_body20_e29534: f64 = (assign21300_body20_e29516 * assign21300_body20_e29533);
        (assign21300_body20_e29534, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn0 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn0 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn2 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn2 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn6 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn6 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn7 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn7 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn10 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn10 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn11 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn11 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn12 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn12 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign21300_body20_e29533) + (assign21300_body20_e29516 * (-(((locals.var_chi_dn17 / 3.0) * assign21300_body20_e29531) + (assign21300_body20_e29520 * (-(((locals.var_chi_dn17 / 4.0) * assign21300_body20_e29529) + (assign21300_body20_e29524 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21300_body20_e29536;
            locals.var_t0_dn0 = assign21300_body20_e29536_d_n0;
            locals.var_t0_dn2 = assign21300_body20_e29536_d_n2;
            locals.var_t0_dn6 = assign21300_body20_e29536_d_n6;
            locals.var_t0_dn7 = assign21300_body20_e29536_d_n7;
            locals.var_t0_dn10 = assign21300_body20_e29536_d_n10;
            locals.var_t0_dn11 = assign21300_body20_e29536_d_n11;
            locals.var_t0_dn12 = assign21300_body20_e29536_d_n12;
            locals.var_t0_dn17 = assign21300_body20_e29536_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21300_body21_e29568, assign21300_body21_e29568_d_n0, assign21300_body21_e29568_d_n2, assign21300_body21_e29568_d_n6, assign21300_body21_e29568_d_n7, assign21300_body21_e29568_d_n10, assign21300_body21_e29568_d_n11, assign21300_body21_e29568_d_n12, assign21300_body21_e29568_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21300_body21_e29552: f64 = (locals.var_chi / 2.0);
        let assign21300_body21_e29556: f64 = (locals.var_chi / 3.0);
        let assign21300_body21_e29560: f64 = (locals.var_chi / 4.0);
        let assign21300_body21_e29561: f64 = (1.0 - assign21300_body21_e29560);
        let assign21300_body21_e29562: f64 = (assign21300_body21_e29556 * assign21300_body21_e29561);
        let assign21300_body21_e29563: f64 = (1.0 - assign21300_body21_e29562);
        let assign21300_body21_e29564: f64 = (assign21300_body21_e29552 * assign21300_body21_e29563);
        let assign21300_body21_e29565: f64 = (1.0 - assign21300_body21_e29564);
        let assign21300_body21_e29566: f64 = (locals.var_chi * assign21300_body21_e29565);
        (assign21300_body21_e29566, ((locals.var_chi_dn0 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn0 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn2 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn6 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn7 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn10 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn11 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn12 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign21300_body21_e29565) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign21300_body21_e29563) + (assign21300_body21_e29552 * (-(((locals.var_chi_dn17 / 3.0) * assign21300_body21_e29561) + (assign21300_body21_e29556 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21300_body21_e29568;
            locals.var_t1_dn0 = assign21300_body21_e29568_d_n0;
            locals.var_t1_dn2 = assign21300_body21_e29568_d_n2;
            locals.var_t1_dn6 = assign21300_body21_e29568_d_n6;
            locals.var_t1_dn7 = assign21300_body21_e29568_d_n7;
            locals.var_t1_dn10 = assign21300_body21_e29568_d_n10;
            locals.var_t1_dn11 = assign21300_body21_e29568_d_n11;
            locals.var_t1_dn12 = assign21300_body21_e29568_d_n12;
            locals.var_t1_dn17 = assign21300_body21_e29568_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21300_body22_e29604, assign21300_body22_e29604_d_n0, assign21300_body22_e29604_d_n2, assign21300_body22_e29604_d_n6, assign21300_body22_e29604_d_n7, assign21300_body22_e29604_d_n10, assign21300_body22_e29604_d_n11, assign21300_body22_e29604_d_n12, assign21300_body22_e29604_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21300_body22_e29582: f64 = (locals.var_chib * locals.var_chib);
        let assign21300_body22_e29584: f64 = (assign21300_body22_e29582 / 2.0);
        let assign21300_body22_e29588: f64 = (locals.var_chib / 3.0);
        let assign21300_body22_e29592: f64 = (locals.var_chib / 4.0);
        let assign21300_body22_e29596: f64 = (locals.var_chib / 5.0);
        let assign21300_body22_e29597: f64 = (1.0 - assign21300_body22_e29596);
        let assign21300_body22_e29598: f64 = (assign21300_body22_e29592 * assign21300_body22_e29597);
        let assign21300_body22_e29599: f64 = (1.0 - assign21300_body22_e29598);
        let assign21300_body22_e29600: f64 = (assign21300_body22_e29588 * assign21300_body22_e29599);
        let assign21300_body22_e29601: f64 = (1.0 - assign21300_body22_e29600);
        let assign21300_body22_e29602: f64 = (assign21300_body22_e29584 * assign21300_body22_e29601);
        (assign21300_body22_e29602, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn0 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn0 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn2 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn2 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn6 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn6 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn7 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn7 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn10 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn10 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn11 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn11 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn12 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn12 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign21300_body22_e29601) + (assign21300_body22_e29584 * (-(((locals.var_chib_dn17 / 3.0) * assign21300_body22_e29599) + (assign21300_body22_e29588 * (-(((locals.var_chib_dn17 / 4.0) * assign21300_body22_e29597) + (assign21300_body22_e29592 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign21300_body22_e29604;
            locals.var_t2_dn0 = assign21300_body22_e29604_d_n0;
            locals.var_t2_dn2 = assign21300_body22_e29604_d_n2;
            locals.var_t2_dn6 = assign21300_body22_e29604_d_n6;
            locals.var_t2_dn7 = assign21300_body22_e29604_d_n7;
            locals.var_t2_dn10 = assign21300_body22_e29604_d_n10;
            locals.var_t2_dn11 = assign21300_body22_e29604_d_n11;
            locals.var_t2_dn12 = assign21300_body22_e29604_d_n12;
            locals.var_t2_dn17 = assign21300_body22_e29604_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign21300_body23_e29636, assign21300_body23_e29636_d_n0, assign21300_body23_e29636_d_n2, assign21300_body23_e29636_d_n6, assign21300_body23_e29636_d_n7, assign21300_body23_e29636_d_n10, assign21300_body23_e29636_d_n11, assign21300_body23_e29636_d_n12, assign21300_body23_e29636_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21300_body23_e29620: f64 = (locals.var_chib / 2.0);
        let assign21300_body23_e29624: f64 = (locals.var_chib / 3.0);
        let assign21300_body23_e29628: f64 = (locals.var_chib / 4.0);
        let assign21300_body23_e29629: f64 = (1.0 - assign21300_body23_e29628);
        let assign21300_body23_e29630: f64 = (assign21300_body23_e29624 * assign21300_body23_e29629);
        let assign21300_body23_e29631: f64 = (1.0 - assign21300_body23_e29630);
        let assign21300_body23_e29632: f64 = (assign21300_body23_e29620 * assign21300_body23_e29631);
        let assign21300_body23_e29633: f64 = (1.0 - assign21300_body23_e29632);
        let assign21300_body23_e29634: f64 = (locals.var_chib * assign21300_body23_e29633);
        (assign21300_body23_e29634, ((locals.var_chib_dn0 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn0 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn2 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn6 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn7 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn10 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn11 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn12 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign21300_body23_e29633) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign21300_body23_e29631) + (assign21300_body23_e29620 * (-(((locals.var_chib_dn17 / 3.0) * assign21300_body23_e29629) + (assign21300_body23_e29624 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign21300_body23_e29636;
            locals.var_t3_dn0 = assign21300_body23_e29636_d_n0;
            locals.var_t3_dn2 = assign21300_body23_e29636_d_n2;
            locals.var_t3_dn6 = assign21300_body23_e29636_d_n6;
            locals.var_t3_dn7 = assign21300_body23_e29636_d_n7;
            locals.var_t3_dn10 = assign21300_body23_e29636_d_n10;
            locals.var_t3_dn11 = assign21300_body23_e29636_d_n11;
            locals.var_t3_dn12 = assign21300_body23_e29636_d_n12;
            locals.var_t3_dn17 = assign21300_body23_e29636_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign21300_body24_e29653, assign21300_body24_e29653_d_n0, assign21300_body24_e29653_d_n2, assign21300_body24_e29653_d_n6, assign21300_body24_e29653_d_n7, assign21300_body24_e29653_d_n10, assign21300_body24_e29653_d_n11, assign21300_body24_e29653_d_n12, assign21300_body24_e29653_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21300_body24_e29650: f64 = (locals.var_t0 - locals.var_t2);
        let assign21300_body24_e29651: f64 = (assign21300_body24_e29650).sqrt();
        (assign21300_body24_e29651, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign21300_body24_e29651)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign21300_body24_e29651)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21300_body24_e29653;
            locals.var_fb_dn0 = assign21300_body24_e29653_d_n0;
            locals.var_fb_dn2 = assign21300_body24_e29653_d_n2;
            locals.var_fb_dn6 = assign21300_body24_e29653_d_n6;
            locals.var_fb_dn7 = assign21300_body24_e29653_d_n7;
            locals.var_fb_dn10 = assign21300_body24_e29653_d_n10;
            locals.var_fb_dn11 = assign21300_body24_e29653_d_n11;
            locals.var_fb_dn12 = assign21300_body24_e29653_d_n12;
            locals.var_fb_dn17 = assign21300_body24_e29653_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21300_body25_e29677, assign21300_body25_e29677_d_n0, assign21300_body25_e29677_d_n2, assign21300_body25_e29677_d_n6, assign21300_body25_e29677_d_n7, assign21300_body25_e29677_d_n10, assign21300_body25_e29677_d_n11, assign21300_body25_e29677_d_n12, assign21300_body25_e29677_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21300_body25_e29667: f64 = (locals.var_beta * 0.5);
        let assign21300_body25_e29671: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign21300_body25_e29672: f64 = (locals.var_t1 - assign21300_body25_e29671);
        let assign21300_body25_e29673: f64 = (assign21300_body25_e29667 * assign21300_body25_e29672);
        let assign21300_body25_e29675: f64 = (assign21300_body25_e29673 / locals.var_fb);
        (assign21300_body25_e29675, ((((assign21300_body25_e29667 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body25_e29667 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body25_e29667 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body25_e29667 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign21300_body25_e29672) + (assign21300_body25_e29667 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body25_e29667 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body25_e29667 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body25_e29667 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign21300_body25_e29673 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21300_body25_e29677;
            locals.var_fb_dpss_dn0 = assign21300_body25_e29677_d_n0;
            locals.var_fb_dpss_dn2 = assign21300_body25_e29677_d_n2;
            locals.var_fb_dpss_dn6 = assign21300_body25_e29677_d_n6;
            locals.var_fb_dpss_dn7 = assign21300_body25_e29677_d_n7;
            locals.var_fb_dpss_dn10 = assign21300_body25_e29677_d_n10;
            locals.var_fb_dpss_dn11 = assign21300_body25_e29677_d_n11;
            locals.var_fb_dpss_dn12 = assign21300_body25_e29677_d_n12;
            locals.var_fb_dpss_dn17 = assign21300_body25_e29677_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign21300_body26_e29694, assign21300_body26_e29694_d_n0, assign21300_body26_e29694_d_n2, assign21300_body26_e29694_d_n6, assign21300_body26_e29694_d_n7, assign21300_body26_e29694_d_n10, assign21300_body26_e29694_d_n11, assign21300_body26_e29694_d_n12, assign21300_body26_e29694_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign21300_body26_e29691: f64 = (-locals.var_chi);
        let assign21300_body26_e29692: f64 = (assign21300_body26_e29691).exp();
        (assign21300_body26_e29692, (assign21300_body26_e29692 * (-locals.var_chi_dn0)), (assign21300_body26_e29692 * (-locals.var_chi_dn2)), (assign21300_body26_e29692 * (-locals.var_chi_dn6)), (assign21300_body26_e29692 * (-locals.var_chi_dn7)), (assign21300_body26_e29692 * (-locals.var_chi_dn10)), (assign21300_body26_e29692 * (-locals.var_chi_dn11)), (assign21300_body26_e29692 * (-locals.var_chi_dn12)), (assign21300_body26_e29692 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21300_body26_e29694;
            locals.var_t0_dn0 = assign21300_body26_e29694_d_n0;
            locals.var_t0_dn2 = assign21300_body26_e29694_d_n2;
            locals.var_t0_dn6 = assign21300_body26_e29694_d_n6;
            locals.var_t0_dn7 = assign21300_body26_e29694_d_n7;
            locals.var_t0_dn10 = assign21300_body26_e29694_d_n10;
            locals.var_t0_dn11 = assign21300_body26_e29694_d_n11;
            locals.var_t0_dn12 = assign21300_body26_e29694_d_n12;
            locals.var_t0_dn17 = assign21300_body26_e29694_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21300_body27_e29711, assign21300_body27_e29711_d_n0, assign21300_body27_e29711_d_n2, assign21300_body27_e29711_d_n6, assign21300_body27_e29711_d_n7, assign21300_body27_e29711_d_n10, assign21300_body27_e29711_d_n11, assign21300_body27_e29711_d_n12, assign21300_body27_e29711_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign21300_body27_e29708: f64 = (-locals.var_chib);
        let assign21300_body27_e29709: f64 = (assign21300_body27_e29708).exp();
        (assign21300_body27_e29709, (assign21300_body27_e29709 * (-locals.var_chib_dn0)), (assign21300_body27_e29709 * (-locals.var_chib_dn2)), (assign21300_body27_e29709 * (-locals.var_chib_dn6)), (assign21300_body27_e29709 * (-locals.var_chib_dn7)), (assign21300_body27_e29709 * (-locals.var_chib_dn10)), (assign21300_body27_e29709 * (-locals.var_chib_dn11)), (assign21300_body27_e29709 * (-locals.var_chib_dn12)), (assign21300_body27_e29709 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21300_body27_e29711;
            locals.var_t1_dn0 = assign21300_body27_e29711_d_n0;
            locals.var_t1_dn2 = assign21300_body27_e29711_d_n2;
            locals.var_t1_dn6 = assign21300_body27_e29711_d_n6;
            locals.var_t1_dn7 = assign21300_body27_e29711_d_n7;
            locals.var_t1_dn10 = assign21300_body27_e29711_d_n10;
            locals.var_t1_dn11 = assign21300_body27_e29711_d_n11;
            locals.var_t1_dn12 = assign21300_body27_e29711_d_n12;
            locals.var_t1_dn17 = assign21300_body27_e29711_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21300_body28_e29733, assign21300_body28_e29733_d_n0, assign21300_body28_e29733_d_n2, assign21300_body28_e29733_d_n6, assign21300_body28_e29733_d_n7, assign21300_body28_e29733_d_n10, assign21300_body28_e29733_d_n11, assign21300_body28_e29733_d_n12, assign21300_body28_e29733_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign21300_body28_e29726: f64 = (locals.var_chi - locals.var_chib);
        let assign21300_body28_e29729: f64 = (locals.var_t0 - locals.var_t1);
        let assign21300_body28_e29730: f64 = (assign21300_body28_e29726 + assign21300_body28_e29729);
        let assign21300_body28_e29731: f64 = (assign21300_body28_e29730).sqrt();
        (assign21300_body28_e29731, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign21300_body28_e29731)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign21300_body28_e29731)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21300_body28_e29733;
            locals.var_fb_dn0 = assign21300_body28_e29733_d_n0;
            locals.var_fb_dn2 = assign21300_body28_e29733_d_n2;
            locals.var_fb_dn6 = assign21300_body28_e29733_d_n6;
            locals.var_fb_dn7 = assign21300_body28_e29733_d_n7;
            locals.var_fb_dn10 = assign21300_body28_e29733_d_n10;
            locals.var_fb_dn11 = assign21300_body28_e29733_d_n11;
            locals.var_fb_dn12 = assign21300_body28_e29733_d_n12;
            locals.var_fb_dn17 = assign21300_body28_e29733_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21300_body29_e29762, assign21300_body29_e29762_d_n0, assign21300_body29_e29762_d_n2, assign21300_body29_e29762_d_n6, assign21300_body29_e29762_d_n7, assign21300_body29_e29762_d_n10, assign21300_body29_e29762_d_n11, assign21300_body29_e29762_d_n12, assign21300_body29_e29762_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign21300_body29_e29748: f64 = (locals.var_beta * 0.5);
        let assign21300_body29_e29751: f64 = (1.0 - locals.var_t0);
        let assign21300_body29_e29755: f64 = (1.0 - locals.var_t1);
        let assign21300_body29_e29756: f64 = (locals.var_phi_soib_dpss * assign21300_body29_e29755);
        let assign21300_body29_e29757: f64 = (assign21300_body29_e29751 - assign21300_body29_e29756);
        let assign21300_body29_e29758: f64 = (assign21300_body29_e29748 * assign21300_body29_e29757);
        let assign21300_body29_e29760: f64 = (assign21300_body29_e29758 / locals.var_fb);
        (assign21300_body29_e29760, ((((assign21300_body29_e29748 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body29_e29748 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body29_e29748 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body29_e29748 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign21300_body29_e29757) + (assign21300_body29_e29748 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body29_e29748 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body29_e29748 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign21300_body29_e29748 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign21300_body29_e29755) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign21300_body29_e29758 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21300_body29_e29762;
            locals.var_fb_dpss_dn0 = assign21300_body29_e29762_d_n0;
            locals.var_fb_dpss_dn2 = assign21300_body29_e29762_d_n2;
            locals.var_fb_dpss_dn6 = assign21300_body29_e29762_d_n6;
            locals.var_fb_dpss_dn7 = assign21300_body29_e29762_d_n7;
            locals.var_fb_dpss_dn10 = assign21300_body29_e29762_d_n10;
            locals.var_fb_dpss_dn11 = assign21300_body29_e29762_d_n11;
            locals.var_fb_dpss_dn12 = assign21300_body29_e29762_d_n12;
            locals.var_fb_dpss_dn17 = assign21300_body29_e29762_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21300_body30_e29769: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard653 = assign21300_body30_e29769;
            locals.var_guard653_rv = 0.0;
            let (assign21300_body31_e29781,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign21300_body31_e29779: f64 = (-1.0);
        (assign21300_body31_e29779,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign21300_body31_e29781;
            locals.var_flg_zone_rv = 0.0;
            let assign21300_body32_e29784: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard654 = assign21300_body32_e29784;
            locals.var_guard654_rv = 0.0;
            let (assign21300_body33_e29796, assign21300_body33_e29796_d_n0, assign21300_body33_e29796_d_n2, assign21300_body33_e29796_d_n6, assign21300_body33_e29796_d_n7, assign21300_body33_e29796_d_n10, assign21300_body33_e29796_d_n11, assign21300_body33_e29796_d_n12, assign21300_body33_e29796_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21300_body33_e29794: f64 = (-locals.var_fb);
        (assign21300_body33_e29794, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21300_body33_e29796;
            locals.var_fs02_dn0 = assign21300_body33_e29796_d_n0;
            locals.var_fs02_dn2 = assign21300_body33_e29796_d_n2;
            locals.var_fs02_dn6 = assign21300_body33_e29796_d_n6;
            locals.var_fs02_dn7 = assign21300_body33_e29796_d_n7;
            locals.var_fs02_dn10 = assign21300_body33_e29796_d_n10;
            locals.var_fs02_dn11 = assign21300_body33_e29796_d_n11;
            locals.var_fs02_dn12 = assign21300_body33_e29796_d_n12;
            locals.var_fs02_dn17 = assign21300_body33_e29796_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21300_body34_e29808, assign21300_body34_e29808_d_n0, assign21300_body34_e29808_d_n2, assign21300_body34_e29808_d_n6, assign21300_body34_e29808_d_n7, assign21300_body34_e29808_d_n10, assign21300_body34_e29808_d_n11, assign21300_body34_e29808_d_n12, assign21300_body34_e29808_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21300_body34_e29806: f64 = (-locals.var_fb_dpss);
        (assign21300_body34_e29806, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21300_body34_e29808;
            locals.var_fs02_dps0_dn0 = assign21300_body34_e29808_d_n0;
            locals.var_fs02_dps0_dn2 = assign21300_body34_e29808_d_n2;
            locals.var_fs02_dps0_dn6 = assign21300_body34_e29808_d_n6;
            locals.var_fs02_dps0_dn7 = assign21300_body34_e29808_d_n7;
            locals.var_fs02_dps0_dn10 = assign21300_body34_e29808_d_n10;
            locals.var_fs02_dps0_dn11 = assign21300_body34_e29808_d_n11;
            locals.var_fs02_dps0_dn12 = assign21300_body34_e29808_d_n12;
            locals.var_fs02_dps0_dn17 = assign21300_body34_e29808_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign21300_body35_e29811: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard655 = assign21300_body35_e29811;
            locals.var_guard655_rv = 0.0;
            let (assign21300_body36_e29825, assign21300_body36_e29825_d_n0, assign21300_body36_e29825_d_n2, assign21300_body36_e29825_d_n6, assign21300_body36_e29825_d_n7, assign21300_body36_e29825_d_n10, assign21300_body36_e29825_d_n11, assign21300_body36_e29825_d_n12, assign21300_body36_e29825_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21300_body36_e29825;
            locals.var_fs02_dn0 = assign21300_body36_e29825_d_n0;
            locals.var_fs02_dn2 = assign21300_body36_e29825_d_n2;
            locals.var_fs02_dn6 = assign21300_body36_e29825_d_n6;
            locals.var_fs02_dn7 = assign21300_body36_e29825_d_n7;
            locals.var_fs02_dn10 = assign21300_body36_e29825_d_n10;
            locals.var_fs02_dn11 = assign21300_body36_e29825_d_n11;
            locals.var_fs02_dn12 = assign21300_body36_e29825_d_n12;
            locals.var_fs02_dn17 = assign21300_body36_e29825_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21300_body37_e29839, assign21300_body37_e29839_d_n0, assign21300_body37_e29839_d_n2, assign21300_body37_e29839_d_n6, assign21300_body37_e29839_d_n7, assign21300_body37_e29839_d_n10, assign21300_body37_e29839_d_n11, assign21300_body37_e29839_d_n12, assign21300_body37_e29839_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21300_body37_e29839;
            locals.var_fs02_dps0_dn0 = assign21300_body37_e29839_d_n0;
            locals.var_fs02_dps0_dn2 = assign21300_body37_e29839_d_n2;
            locals.var_fs02_dps0_dn6 = assign21300_body37_e29839_d_n6;
            locals.var_fs02_dps0_dn7 = assign21300_body37_e29839_d_n7;
            locals.var_fs02_dps0_dn10 = assign21300_body37_e29839_d_n10;
            locals.var_fs02_dps0_dn11 = assign21300_body37_e29839_d_n11;
            locals.var_fs02_dps0_dn12 = assign21300_body37_e29839_d_n12;
            locals.var_fs02_dps0_dn17 = assign21300_body37_e29839_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign21300_body38_e29858, assign21300_body38_e29858_d_n0, assign21300_body38_e29858_d_n2, assign21300_body38_e29858_d_n6, assign21300_body38_e29858_d_n7, assign21300_body38_e29858_d_n10, assign21300_body38_e29858_d_n11, assign21300_body38_e29858_d_n12, assign21300_body38_e29858_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) {
        let assign21300_body38_e29855: f64 = (locals.var_phi_s0_soi__blk646 - p.p287);
        let assign21300_body38_e29856: f64 = (locals.var_beta * assign21300_body38_e29855);
        (assign21300_body38_e29856, (locals.var_beta * locals.var_phi_s0_soi__blk646_dn0), (locals.var_beta * locals.var_phi_s0_soi__blk646_dn2), (locals.var_beta * locals.var_phi_s0_soi__blk646_dn6), (locals.var_beta * locals.var_phi_s0_soi__blk646_dn7), ((locals.var_beta_dn10 * assign21300_body38_e29855) + (locals.var_beta * locals.var_phi_s0_soi__blk646_dn10)), (locals.var_beta * locals.var_phi_s0_soi__blk646_dn11), (locals.var_beta * locals.var_phi_s0_soi__blk646_dn12), (locals.var_beta * locals.var_phi_s0_soi__blk646_dn17),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign21300_body38_e29858;
            locals.var_rho_dn0 = assign21300_body38_e29858_d_n0;
            locals.var_rho_dn2 = assign21300_body38_e29858_d_n2;
            locals.var_rho_dn6 = assign21300_body38_e29858_d_n6;
            locals.var_rho_dn7 = assign21300_body38_e29858_d_n7;
            locals.var_rho_dn10 = assign21300_body38_e29858_d_n10;
            locals.var_rho_dn11 = assign21300_body38_e29858_d_n11;
            locals.var_rho_dn12 = assign21300_body38_e29858_d_n12;
            locals.var_rho_dn17 = assign21300_body38_e29858_d_n17;
            locals.var_rho_rv = 0.0;
            let (assign21300_body39_e29874, assign21300_body39_e29874_d_n0, assign21300_body39_e29874_d_n2, assign21300_body39_e29874_d_n6, assign21300_body39_e29874_d_n7, assign21300_body39_e29874_d_n10, assign21300_body39_e29874_d_n11, assign21300_body39_e29874_d_n12, assign21300_body39_e29874_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) {
        let assign21300_body39_e29872: f64 = (locals.var_rho).exp();
        (assign21300_body39_e29872, (assign21300_body39_e29872 * locals.var_rho_dn0), (assign21300_body39_e29872 * locals.var_rho_dn2), (assign21300_body39_e29872 * locals.var_rho_dn6), (assign21300_body39_e29872 * locals.var_rho_dn7), (assign21300_body39_e29872 * locals.var_rho_dn10), (assign21300_body39_e29872 * locals.var_rho_dn11), (assign21300_body39_e29872 * locals.var_rho_dn12), (assign21300_body39_e29872 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign21300_body39_e29874;
            locals.var_exp_rho_dn0 = assign21300_body39_e29874_d_n0;
            locals.var_exp_rho_dn2 = assign21300_body39_e29874_d_n2;
            locals.var_exp_rho_dn6 = assign21300_body39_e29874_d_n6;
            locals.var_exp_rho_dn7 = assign21300_body39_e29874_d_n7;
            locals.var_exp_rho_dn10 = assign21300_body39_e29874_d_n10;
            locals.var_exp_rho_dn11 = assign21300_body39_e29874_d_n11;
            locals.var_exp_rho_dn12 = assign21300_body39_e29874_d_n12;
            locals.var_exp_rho_dn17 = assign21300_body39_e29874_d_n17;
            locals.var_exp_rho_rv = 0.0;
            let (assign21300_body40_e29897, assign21300_body40_e29897_d_n0, assign21300_body40_e29897_d_n2, assign21300_body40_e29897_d_n6, assign21300_body40_e29897_d_n7, assign21300_body40_e29897_d_n10, assign21300_body40_e29897_d_n11, assign21300_body40_e29897_d_n12, assign21300_body40_e29897_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) {
        let assign21300_body40_e29892: f64 = (locals.var_chi + 1.0);
        let assign21300_body40_e29893: f64 = (locals.var_exp_bvbsvds * assign21300_body40_e29892);
        let assign21300_body40_e29894: f64 = (locals.var_exp_rho - assign21300_body40_e29893);
        let assign21300_body40_e29895: f64 = (locals.var_cnst1soi * assign21300_body40_e29894);
        (assign21300_body40_e29895, ((locals.var_cnst1soi_dn0 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign21300_body40_e29894) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign21300_body40_e29892) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign21300_body40_e29897;
            locals.var_fs01_dn0 = assign21300_body40_e29897_d_n0;
            locals.var_fs01_dn2 = assign21300_body40_e29897_d_n2;
            locals.var_fs01_dn6 = assign21300_body40_e29897_d_n6;
            locals.var_fs01_dn7 = assign21300_body40_e29897_d_n7;
            locals.var_fs01_dn10 = assign21300_body40_e29897_d_n10;
            locals.var_fs01_dn11 = assign21300_body40_e29897_d_n11;
            locals.var_fs01_dn12 = assign21300_body40_e29897_d_n12;
            locals.var_fs01_dn17 = assign21300_body40_e29897_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign21300_body41_e29918, assign21300_body41_e29918_d_n0, assign21300_body41_e29918_d_n2, assign21300_body41_e29918_d_n6, assign21300_body41_e29918_d_n7, assign21300_body41_e29918_d_n10, assign21300_body41_e29918_d_n11, assign21300_body41_e29918_d_n12, assign21300_body41_e29918_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) {
        let assign21300_body41_e29912: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign21300_body41_e29915: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign21300_body41_e29916: f64 = (assign21300_body41_e29912 * assign21300_body41_e29915);
        (assign21300_body41_e29916, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign21300_body41_e29915) + (assign21300_body41_e29912 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign21300_body41_e29918;
            locals.var_fs01_dps0_dn0 = assign21300_body41_e29918_d_n0;
            locals.var_fs01_dps0_dn2 = assign21300_body41_e29918_d_n2;
            locals.var_fs01_dps0_dn6 = assign21300_body41_e29918_d_n6;
            locals.var_fs01_dps0_dn7 = assign21300_body41_e29918_d_n7;
            locals.var_fs01_dps0_dn10 = assign21300_body41_e29918_d_n10;
            locals.var_fs01_dps0_dn11 = assign21300_body41_e29918_d_n11;
            locals.var_fs01_dps0_dn12 = assign21300_body41_e29918_d_n12;
            locals.var_fs01_dps0_dn17 = assign21300_body41_e29918_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign21300_body42_e29938, assign21300_body42_e29938_d_n0, assign21300_body42_e29938_d_n2, assign21300_body42_e29938_d_n6, assign21300_body42_e29938_d_n7, assign21300_body42_e29938_d_n10, assign21300_body42_e29938_d_n11, assign21300_body42_e29938_d_n12, assign21300_body42_e29938_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) {
        let assign21300_body42_e29933: f64 = (locals.var_fb * locals.var_fb);
        let assign21300_body42_e29935: f64 = (assign21300_body42_e29933 + locals.var_fs01);
        let assign21300_body42_e29936: f64 = (assign21300_body42_e29935).sqrt();
        (assign21300_body42_e29936, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign21300_body42_e29936)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign21300_body42_e29936)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21300_body42_e29938;
            locals.var_fs02_dn0 = assign21300_body42_e29938_d_n0;
            locals.var_fs02_dn2 = assign21300_body42_e29938_d_n2;
            locals.var_fs02_dn6 = assign21300_body42_e29938_d_n6;
            locals.var_fs02_dn7 = assign21300_body42_e29938_d_n7;
            locals.var_fs02_dn10 = assign21300_body42_e29938_d_n10;
            locals.var_fs02_dn11 = assign21300_body42_e29938_d_n11;
            locals.var_fs02_dn12 = assign21300_body42_e29938_d_n12;
            locals.var_fs02_dn17 = assign21300_body42_e29938_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21300_body43_e29963, assign21300_body43_e29963_d_n0, assign21300_body43_e29963_d_n2, assign21300_body43_e29963_d_n6, assign21300_body43_e29963_d_n7, assign21300_body43_e29963_d_n10, assign21300_body43_e29963_d_n11, assign21300_body43_e29963_d_n12, assign21300_body43_e29963_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) {
        let assign21300_body43_e29954: f64 = (2.0 * locals.var_fb_dpss);
        let assign21300_body43_e29956: f64 = (assign21300_body43_e29954 * locals.var_fb);
        let assign21300_body43_e29958: f64 = (assign21300_body43_e29956 + locals.var_fs01_dps0);
        let assign21300_body43_e29959: f64 = (0.5 * assign21300_body43_e29958);
        let assign21300_body43_e29961: f64 = (assign21300_body43_e29959 / locals.var_fs02);
        (assign21300_body43_e29961, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign21300_body43_e29954 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign21300_body43_e29959 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21300_body43_e29963;
            locals.var_fs02_dps0_dn0 = assign21300_body43_e29963_d_n0;
            locals.var_fs02_dps0_dn2 = assign21300_body43_e29963_d_n2;
            locals.var_fs02_dps0_dn6 = assign21300_body43_e29963_d_n6;
            locals.var_fs02_dps0_dn7 = assign21300_body43_e29963_d_n7;
            locals.var_fs02_dps0_dn10 = assign21300_body43_e29963_d_n10;
            locals.var_fs02_dps0_dn11 = assign21300_body43_e29963_d_n11;
            locals.var_fs02_dps0_dn12 = assign21300_body43_e29963_d_n12;
            locals.var_fs02_dps0_dn17 = assign21300_body43_e29963_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign21300_body44_e29979, assign21300_body44_e29979_d_n0, assign21300_body44_e29979_d_n2, assign21300_body44_e29979_d_n6, assign21300_body44_e29979_d_n7, assign21300_body44_e29979_d_n10, assign21300_body44_e29979_d_n11, assign21300_body44_e29979_d_n12, assign21300_body44_e29979_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body44_e29971: f64 = (-locals.var_vgp__blk610);
        let assign21300_body44_e29973: f64 = (assign21300_body44_e29971 + locals.var_phi_s0_soi__blk646);
        let assign21300_body44_e29976: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign21300_body44_e29977: f64 = (assign21300_body44_e29973 + assign21300_body44_e29976);
        (assign21300_body44_e29977, (((-locals.var_vgp__blk610_dn0) + locals.var_phi_s0_soi__blk646_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk610_dn2) + locals.var_phi_s0_soi__blk646_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk610_dn6) + locals.var_phi_s0_soi__blk646_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk610_dn7) + locals.var_phi_s0_soi__blk646_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk610_dn10) + locals.var_phi_s0_soi__blk646_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk610_dn11) + locals.var_phi_s0_soi__blk646_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (((-locals.var_vgp__blk610_dn12) + locals.var_phi_s0_soi__blk646_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))), (((-locals.var_vgp__blk610_dn17) + locals.var_phi_s0_soi__blk646_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign21300_body44_e29979;
            locals.var_fs0_dn0 = assign21300_body44_e29979_d_n0;
            locals.var_fs0_dn2 = assign21300_body44_e29979_d_n2;
            locals.var_fs0_dn6 = assign21300_body44_e29979_d_n6;
            locals.var_fs0_dn7 = assign21300_body44_e29979_d_n7;
            locals.var_fs0_dn10 = assign21300_body44_e29979_d_n10;
            locals.var_fs0_dn11 = assign21300_body44_e29979_d_n11;
            locals.var_fs0_dn12 = assign21300_body44_e29979_d_n12;
            locals.var_fs0_dn17 = assign21300_body44_e29979_d_n17;
            locals.var_fs0_rv = 0.0;
            let (assign21300_body45_e29992, assign21300_body45_e29992_d_n0, assign21300_body45_e29992_d_n2, assign21300_body45_e29992_d_n6, assign21300_body45_e29992_d_n7, assign21300_body45_e29992_d_n10, assign21300_body45_e29992_d_n11, assign21300_body45_e29992_d_n12, assign21300_body45_e29992_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body45_e29989: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign21300_body45_e29990: f64 = (1.0 + assign21300_body45_e29989);
        (assign21300_body45_e29990, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign21300_body45_e29992;
            locals.var_fs0_dps0_dn0 = assign21300_body45_e29992_d_n0;
            locals.var_fs0_dps0_dn2 = assign21300_body45_e29992_d_n2;
            locals.var_fs0_dps0_dn6 = assign21300_body45_e29992_d_n6;
            locals.var_fs0_dps0_dn7 = assign21300_body45_e29992_d_n7;
            locals.var_fs0_dps0_dn10 = assign21300_body45_e29992_d_n10;
            locals.var_fs0_dps0_dn11 = assign21300_body45_e29992_d_n11;
            locals.var_fs0_dps0_dn12 = assign21300_body45_e29992_d_n12;
            locals.var_fs0_dps0_dn17 = assign21300_body45_e29992_d_n17;
            locals.var_fs0_dps0_rv = 0.0;
            let assign21300_body46_e29995: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard656 = assign21300_body46_e29995;
            locals.var_guard656_rv = 0.0;
            let (assign21300_body47_e30008,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard656 != 0.0)) {
        let assign21300_body47_e30006: f64 = (locals.var_lp_s0_max + 1.0);
        (assign21300_body47_e30006,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign21300_body47_e30008;
            locals.var_lp_s0_rv = 0.0;
            let (assign21300_body48_e30023, assign21300_body48_e30023_d_n0, assign21300_body48_e30023_d_n2, assign21300_body48_e30023_d_n6, assign21300_body48_e30023_d_n7, assign21300_body48_e30023_d_n10, assign21300_body48_e30023_d_n11, assign21300_body48_e30023_d_n12, assign21300_body48_e30023_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard656 == 0.0)) {
        let assign21300_body48_e30019: f64 = (-locals.var_fs0);
        let assign21300_body48_e30021: f64 = (assign21300_body48_e30019 / locals.var_fs0_dps0);
        (assign21300_body48_e30021, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign21300_body48_e30019 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign21300_body48_e30023;
            locals.var_dps0_dn0 = assign21300_body48_e30023_d_n0;
            locals.var_dps0_dn2 = assign21300_body48_e30023_d_n2;
            locals.var_dps0_dn6 = assign21300_body48_e30023_d_n6;
            locals.var_dps0_dn7 = assign21300_body48_e30023_d_n7;
            locals.var_dps0_dn10 = assign21300_body48_e30023_d_n10;
            locals.var_dps0_dn11 = assign21300_body48_e30023_d_n11;
            locals.var_dps0_dn12 = assign21300_body48_e30023_d_n12;
            locals.var_dps0_dn17 = assign21300_body48_e30023_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign21300_body49_e30048, assign21300_body49_e30048_d_n0, assign21300_body49_e30048_d_n2, assign21300_body49_e30048_d_n6, assign21300_body49_e30048_d_n7, assign21300_body49_e30048_d_n10, assign21300_body49_e30048_d_n11, assign21300_body49_e30048_d_n12, assign21300_body49_e30048_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard656 == 0.0)) {
        let assign21300_body49_e30035: f64 = (0.5 * 0.1);
        let assign21300_body49_e30039: f64 = (locals.var_phi_s0_soi__blk646).abs();
        let (assign21300_body49_e30044, assign21300_body49_e30044_d_n0, assign21300_body49_e30044_d_n2, assign21300_body49_e30044_d_n6, assign21300_body49_e30044_d_n7, assign21300_body49_e30044_d_n10, assign21300_body49_e30044_d_n11, assign21300_body49_e30044_d_n12, assign21300_body49_e30044_d_n17,) = {
            if (1.0 >= assign21300_body49_e30039) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign21300_body49_e30043: f64 = (locals.var_phi_s0_soi__blk646).abs();
                (assign21300_body49_e30043, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn0 } else { (-locals.var_phi_s0_soi__blk646_dn0) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn2 } else { (-locals.var_phi_s0_soi__blk646_dn2) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn6 } else { (-locals.var_phi_s0_soi__blk646_dn6) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn7 } else { (-locals.var_phi_s0_soi__blk646_dn7) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn10 } else { (-locals.var_phi_s0_soi__blk646_dn10) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn11 } else { (-locals.var_phi_s0_soi__blk646_dn11) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn12 } else { (-locals.var_phi_s0_soi__blk646_dn12) }, if locals.var_phi_s0_soi__blk646 >= 0.0 { locals.var_phi_s0_soi__blk646_dn17 } else { (-locals.var_phi_s0_soi__blk646_dn17) },)
            }
        };
        let assign21300_body49_e30045: f64 = (1.0 + assign21300_body49_e30044);
        let assign21300_body49_e30046: f64 = (assign21300_body49_e30035 * assign21300_body49_e30045);
        (assign21300_body49_e30046, (assign21300_body49_e30035 * assign21300_body49_e30044_d_n0), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n2), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n6), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n7), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n10), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n11), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n12), (assign21300_body49_e30035 * assign21300_body49_e30044_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign21300_body49_e30048;
            locals.var_dplim_dn0 = assign21300_body49_e30048_d_n0;
            locals.var_dplim_dn2 = assign21300_body49_e30048_d_n2;
            locals.var_dplim_dn6 = assign21300_body49_e30048_d_n6;
            locals.var_dplim_dn7 = assign21300_body49_e30048_d_n7;
            locals.var_dplim_dn10 = assign21300_body49_e30048_d_n10;
            locals.var_dplim_dn11 = assign21300_body49_e30048_d_n11;
            locals.var_dplim_dn12 = assign21300_body49_e30048_d_n12;
            locals.var_dplim_dn17 = assign21300_body49_e30048_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign21300_body50_e30050: f64 = (locals.var_dps0).abs();
            let assign21300_body50_e30052: f64 = if assign21300_body50_e30050 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard657 = assign21300_body50_e30052;
            locals.var_guard657_rv = 0.0;
            let (assign21300_body51_e30074, assign21300_body51_e30074_d_n0, assign21300_body51_e30074_d_n2, assign21300_body51_e30074_d_n6, assign21300_body51_e30074_d_n7, assign21300_body51_e30074_d_n10, assign21300_body51_e30074_d_n11, assign21300_body51_e30074_d_n12, assign21300_body51_e30074_d_n17,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 != 0.0)) {
        let (assign21300_body51_e30071,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign21300_body51_e30070: f64 = (-1.0);
                (assign21300_body51_e30070,)
            }
        };
        let assign21300_body51_e30072: f64 = (locals.var_dplim * assign21300_body51_e30071);
        (assign21300_body51_e30072, (locals.var_dplim_dn0 * assign21300_body51_e30071), (locals.var_dplim_dn2 * assign21300_body51_e30071), (locals.var_dplim_dn6 * assign21300_body51_e30071), (locals.var_dplim_dn7 * assign21300_body51_e30071), (locals.var_dplim_dn10 * assign21300_body51_e30071), (locals.var_dplim_dn11 * assign21300_body51_e30071), (locals.var_dplim_dn12 * assign21300_body51_e30071), (locals.var_dplim_dn17 * assign21300_body51_e30071),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign21300_body51_e30074;
            locals.var_dps0_dn0 = assign21300_body51_e30074_d_n0;
            locals.var_dps0_dn2 = assign21300_body51_e30074_d_n2;
            locals.var_dps0_dn6 = assign21300_body51_e30074_d_n6;
            locals.var_dps0_dn7 = assign21300_body51_e30074_d_n7;
            locals.var_dps0_dn10 = assign21300_body51_e30074_d_n10;
            locals.var_dps0_dn11 = assign21300_body51_e30074_d_n11;
            locals.var_dps0_dn12 = assign21300_body51_e30074_d_n12;
            locals.var_dps0_dn17 = assign21300_body51_e30074_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign21300_body52_e30088, assign21300_body52_e30088_d_n0, assign21300_body52_e30088_d_n2, assign21300_body52_e30088_d_n6, assign21300_body52_e30088_d_n7, assign21300_body52_e30088_d_n10, assign21300_body52_e30088_d_n11, assign21300_body52_e30088_d_n12, assign21300_body52_e30088_d_n17,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard656 == 0.0)) {
        let assign21300_body52_e30086: f64 = (locals.var_phi_s0_soi__blk646 + locals.var_dps0);
        (assign21300_body52_e30086, (locals.var_phi_s0_soi__blk646_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi__blk646_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi__blk646_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi__blk646_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi__blk646_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi__blk646_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi__blk646_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi__blk646_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi__blk646, locals.var_phi_s0_soi__blk646_dn0, locals.var_phi_s0_soi__blk646_dn2, locals.var_phi_s0_soi__blk646_dn6, locals.var_phi_s0_soi__blk646_dn7, locals.var_phi_s0_soi__blk646_dn10, locals.var_phi_s0_soi__blk646_dn11, locals.var_phi_s0_soi__blk646_dn12, locals.var_phi_s0_soi__blk646_dn17,)
    }
};
            locals.var_phi_s0_soi__blk646 = assign21300_body52_e30088;
            locals.var_phi_s0_soi__blk646_dn0 = assign21300_body52_e30088_d_n0;
            locals.var_phi_s0_soi__blk646_dn2 = assign21300_body52_e30088_d_n2;
            locals.var_phi_s0_soi__blk646_dn6 = assign21300_body52_e30088_d_n6;
            locals.var_phi_s0_soi__blk646_dn7 = assign21300_body52_e30088_d_n7;
            locals.var_phi_s0_soi__blk646_dn10 = assign21300_body52_e30088_d_n10;
            locals.var_phi_s0_soi__blk646_dn11 = assign21300_body52_e30088_d_n11;
            locals.var_phi_s0_soi__blk646_dn12 = assign21300_body52_e30088_d_n12;
            locals.var_phi_s0_soi__blk646_dn17 = assign21300_body52_e30088_d_n17;
            locals.var_phi_s0_soi__blk646_rv = 0.0;
            let assign21300_body53_e30090: f64 = (locals.var_dps0).abs();
            let assign21300_body53_e30094: f64 = (locals.var_fs0).abs();
            let assign21300_body53_e30097: f64 = if ((assign21300_body53_e30090 <= 5e-12) && (assign21300_body53_e30094 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard658 = assign21300_body53_e30097;
            locals.var_guard658_rv = 0.0;
            let (assign21300_body54_e30111,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard658 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign21300_body54_e30111;
            locals.var_flg_conv_rv = 0.0;
            let (assign21300_body55_e30122,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21300_body55_e30120: f64 = (locals.var_lp_s0 + 1.0);
        (assign21300_body55_e30120,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign21300_body55_e30122;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign21310_e30131, assign21310_e30131_d_n0, assign21310_e30131_d_n2, assign21310_e30131_d_n6, assign21310_e30131_d_n7, assign21310_e30131_d_n10, assign21310_e30131_d_n11, assign21310_e30131_d_n12, assign21310_e30131_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard644 != 0.0)) && (locals.var_guard647 == 0.0)) {
        (locals.var_phi_s0_soi__blk646, locals.var_phi_s0_soi__blk646_dn0, locals.var_phi_s0_soi__blk646_dn2, locals.var_phi_s0_soi__blk646_dn6, locals.var_phi_s0_soi__blk646_dn7, locals.var_phi_s0_soi__blk646_dn10, locals.var_phi_s0_soi__blk646_dn11, locals.var_phi_s0_soi__blk646_dn12, locals.var_phi_s0_soi__blk646_dn17,)
    } else {
        (locals.var_ps0__blk608, locals.var_ps0__blk608_dn0, locals.var_ps0__blk608_dn2, locals.var_ps0__blk608_dn6, locals.var_ps0__blk608_dn7, locals.var_ps0__blk608_dn10, locals.var_ps0__blk608_dn11, locals.var_ps0__blk608_dn12, locals.var_ps0__blk608_dn17,)
    }
};
        locals.var_ps0__blk608 = assign21310_e30131;
        locals.var_ps0__blk608_dn0 = assign21310_e30131_d_n0;
        locals.var_ps0__blk608_dn2 = assign21310_e30131_d_n2;
        locals.var_ps0__blk608_dn6 = assign21310_e30131_d_n6;
        locals.var_ps0__blk608_dn7 = assign21310_e30131_d_n7;
        locals.var_ps0__blk608_dn10 = assign21310_e30131_d_n10;
        locals.var_ps0__blk608_dn11 = assign21310_e30131_d_n11;
        locals.var_ps0__blk608_dn12 = assign21310_e30131_d_n12;
        locals.var_ps0__blk608_dn17 = assign21310_e30131_d_n17;
        locals.var_ps0__blk608_rv = 0.0;

        let (assign21320_e30140, assign21320_e30140_d_n0, assign21320_e30140_d_n2, assign21320_e30140_d_n6, assign21320_e30140_d_n7, assign21320_e30140_d_n10, assign21320_e30140_d_n11, assign21320_e30140_d_n12, assign21320_e30140_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21320_e30134: f64 = (-locals.var_beta);
        let assign21320_e30137: f64 = (locals.var_ps0__blk608 - locals.var_dphi_vds);
        let assign21320_e30138: f64 = (assign21320_e30134 * assign21320_e30137);
        (assign21320_e30138, (assign21320_e30134 * (locals.var_ps0__blk608_dn0 - locals.var_dphi_vds_dn0)), (assign21320_e30134 * (locals.var_ps0__blk608_dn2 - locals.var_dphi_vds_dn2)), (assign21320_e30134 * (locals.var_ps0__blk608_dn6 - locals.var_dphi_vds_dn6)), (assign21320_e30134 * (locals.var_ps0__blk608_dn7 - locals.var_dphi_vds_dn7)), (((-locals.var_beta_dn10) * assign21320_e30137) + (assign21320_e30134 * (locals.var_ps0__blk608_dn10 - locals.var_dphi_vds_dn10))), (assign21320_e30134 * (locals.var_ps0__blk608_dn11 - locals.var_dphi_vds_dn11)), (assign21320_e30134 * (locals.var_ps0__blk608_dn12 - locals.var_dphi_vds_dn12)), (assign21320_e30134 * (locals.var_ps0__blk608_dn17 - locals.var_dphi_vds_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign21320_e30140;
        locals.var_t5_dn0 = assign21320_e30140_d_n0;
        locals.var_t5_dn2 = assign21320_e30140_d_n2;
        locals.var_t5_dn6 = assign21320_e30140_d_n6;
        locals.var_t5_dn7 = assign21320_e30140_d_n7;
        locals.var_t5_dn10 = assign21320_e30140_d_n10;
        locals.var_t5_dn11 = assign21320_e30140_d_n11;
        locals.var_t5_dn12 = assign21320_e30140_d_n12;
        locals.var_t5_dn17 = assign21320_e30140_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign21330_e30150,) = {
    if (locals.var_guard598 != 0.0) {
        let (assign21330_e30148,) = {
            if (locals.var_t5 >= 0.0) {
                (1.0,)
            } else {
                let assign21330_e30147: f64 = (-1.0);
                (assign21330_e30147,)
            }
        };
        (assign21330_e30148,)
    } else {
        (locals.var_t5sign,)
    }
};
        locals.var_t5sign = assign21330_e30150;
        locals.var_t5sign_rv = 0.0;

        let (assign21340_e30156, assign21340_e30156_d_n0, assign21340_e30156_d_n2, assign21340_e30156_d_n6, assign21340_e30156_d_n7, assign21340_e30156_d_n10, assign21340_e30156_d_n11, assign21340_e30156_d_n12, assign21340_e30156_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21340_e30154: f64 = (locals.var_t5sign * locals.var_t5);
        (assign21340_e30154, (locals.var_t5sign * locals.var_t5_dn0), (locals.var_t5sign * locals.var_t5_dn2), (locals.var_t5sign * locals.var_t5_dn6), (locals.var_t5sign * locals.var_t5_dn7), (locals.var_t5sign * locals.var_t5_dn10), (locals.var_t5sign * locals.var_t5_dn11), (locals.var_t5sign * locals.var_t5_dn12), (locals.var_t5sign * locals.var_t5_dn17),)
    } else {
        (locals.var_t5y, locals.var_t5y_dn0, locals.var_t5y_dn2, locals.var_t5y_dn6, locals.var_t5y_dn7, locals.var_t5y_dn10, locals.var_t5y_dn11, locals.var_t5y_dn12, locals.var_t5y_dn17,)
    }
};
        locals.var_t5y = assign21340_e30156;
        locals.var_t5y_dn0 = assign21340_e30156_d_n0;
        locals.var_t5y_dn2 = assign21340_e30156_d_n2;
        locals.var_t5y_dn6 = assign21340_e30156_d_n6;
        locals.var_t5y_dn7 = assign21340_e30156_d_n7;
        locals.var_t5y_dn10 = assign21340_e30156_d_n10;
        locals.var_t5y_dn11 = assign21340_e30156_d_n11;
        locals.var_t5y_dn12 = assign21340_e30156_d_n12;
        locals.var_t5y_dn17 = assign21340_e30156_d_n17;
        locals.var_t5y_rv = 0.0;

        let (assign21350_e30161, assign21350_e30161_d_n0, assign21350_e30161_d_n2, assign21350_e30161_d_n6, assign21350_e30161_d_n7, assign21350_e30161_d_n10, assign21350_e30161_d_n11, assign21350_e30161_d_n12, assign21350_e30161_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21350_e30159: f64 = (locals.var_t5).exp();
        (assign21350_e30159, (assign21350_e30159 * locals.var_t5_dn0), (assign21350_e30159 * locals.var_t5_dn2), (assign21350_e30159 * locals.var_t5_dn6), (assign21350_e30159 * locals.var_t5_dn7), (assign21350_e30159 * locals.var_t5_dn10), (assign21350_e30159 * locals.var_t5_dn11), (assign21350_e30159 * locals.var_t5_dn12), (assign21350_e30159 * locals.var_t5_dn17),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign21350_e30161;
        locals.var_t6_dn0 = assign21350_e30161_d_n0;
        locals.var_t6_dn2 = assign21350_e30161_d_n2;
        locals.var_t6_dn6 = assign21350_e30161_d_n6;
        locals.var_t6_dn7 = assign21350_e30161_d_n7;
        locals.var_t6_dn10 = assign21350_e30161_d_n10;
        locals.var_t6_dn11 = assign21350_e30161_d_n11;
        locals.var_t6_dn12 = assign21350_e30161_d_n12;
        locals.var_t6_dn17 = assign21350_e30161_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign21360_e30169, assign21360_e30169_d_n0, assign21360_e30169_d_n2, assign21360_e30169_d_n6, assign21360_e30169_d_n7, assign21360_e30169_d_n10, assign21360_e30169_d_n11, assign21360_e30169_d_n12, assign21360_e30169_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21360_e30165: f64 = (locals.var_t6 - 1.0);
        let assign21360_e30167: f64 = (assign21360_e30165 - locals.var_t5);
        (assign21360_e30167, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn11 - locals.var_t5_dn11), (locals.var_t6_dn12 - locals.var_t5_dn12), (locals.var_t6_dn17 - locals.var_t5_dn17),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
        locals.var_t7 = assign21360_e30169;
        locals.var_t7_dn0 = assign21360_e30169_d_n0;
        locals.var_t7_dn2 = assign21360_e30169_d_n2;
        locals.var_t7_dn6 = assign21360_e30169_d_n6;
        locals.var_t7_dn7 = assign21360_e30169_d_n7;
        locals.var_t7_dn10 = assign21360_e30169_d_n10;
        locals.var_t7_dn11 = assign21360_e30169_d_n11;
        locals.var_t7_dn12 = assign21360_e30169_d_n12;
        locals.var_t7_dn17 = assign21360_e30169_d_n17;
        locals.var_t7_rv = 0.0;

        let assign21370_e30172: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign21370_e30172;
        locals.var_guard659_rv = 0.0;

        let (assign21380_e30182, assign21380_e30182_d_n0, assign21380_e30182_d_n2, assign21380_e30182_d_n6, assign21380_e30182_d_n7, assign21380_e30182_d_n10, assign21380_e30182_d_n11, assign21380_e30182_d_n12, assign21380_e30182_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard659 != 0.0)) {
        let assign21380_e30177: f64 = (-locals.var_cnst0soi);
        let assign21380_e30179: f64 = (locals.var_t7).sqrt();
        let assign21380_e30180: f64 = (assign21380_e30177 * assign21380_e30179);
        (assign21380_e30180, (((-locals.var_cnst0soi_dn0) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn0 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn2) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn2 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn6) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn6 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn7) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn7 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn10) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn10 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn11) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn11 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn12) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn12 / (2.0 * assign21380_e30179)))), (((-locals.var_cnst0soi_dn17) * assign21380_e30179) + (assign21380_e30177 * (locals.var_t7_dn17 / (2.0 * assign21380_e30179)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21380_e30182;
        locals.var_qbu_dn0 = assign21380_e30182_d_n0;
        locals.var_qbu_dn2 = assign21380_e30182_d_n2;
        locals.var_qbu_dn6 = assign21380_e30182_d_n6;
        locals.var_qbu_dn7 = assign21380_e30182_d_n7;
        locals.var_qbu_dn10 = assign21380_e30182_d_n10;
        locals.var_qbu_dn11 = assign21380_e30182_d_n11;
        locals.var_qbu_dn12 = assign21380_e30182_d_n12;
        locals.var_qbu_dn17 = assign21380_e30182_d_n17;
        locals.var_qbu_rv = 0.0;

        let assign21390_e30185: f64 = if locals.var_t5y > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign21390_e30185;
        locals.var_guard660_rv = 0.0;

        let (assign21400_e30197, assign21400_e30197_d_n0, assign21400_e30197_d_n2, assign21400_e30197_d_n6, assign21400_e30197_d_n7, assign21400_e30197_d_n10, assign21400_e30197_d_n11, assign21400_e30197_d_n12, assign21400_e30197_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign21400_e30194: f64 = (locals.var_t7).sqrt();
        let assign21400_e30195: f64 = (locals.var_cnst0soi * assign21400_e30194);
        (assign21400_e30195, ((locals.var_cnst0soi_dn0 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn0 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn2 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn2 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn6 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn6 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn7 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn7 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn10 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn10 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn11 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn11 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn12 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn12 / (2.0 * assign21400_e30194)))), ((locals.var_cnst0soi_dn17 * assign21400_e30194) + (locals.var_cnst0soi * (locals.var_t7_dn17 / (2.0 * assign21400_e30194)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21400_e30197;
        locals.var_qbu_dn0 = assign21400_e30197_d_n0;
        locals.var_qbu_dn2 = assign21400_e30197_d_n2;
        locals.var_qbu_dn6 = assign21400_e30197_d_n6;
        locals.var_qbu_dn7 = assign21400_e30197_d_n7;
        locals.var_qbu_dn10 = assign21400_e30197_d_n10;
        locals.var_qbu_dn11 = assign21400_e30197_d_n11;
        locals.var_qbu_dn12 = assign21400_e30197_d_n12;
        locals.var_qbu_dn17 = assign21400_e30197_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign21410_e30225, assign21410_e30225_d_n0, assign21410_e30225_d_n2, assign21410_e30225_d_n6, assign21410_e30225_d_n7, assign21410_e30225_d_n10, assign21410_e30225_d_n11, assign21410_e30225_d_n12, assign21410_e30225_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign21410_e30206: f64 = (-locals.var_t5sign);
        let assign21410_e30208: f64 = (assign21410_e30206 * locals.var_t5y);
        let assign21410_e30210: f64 = (assign21410_e30208 * 0.7071067811865475);
        let assign21410_e30214: f64 = (locals.var_t5y * 0.3333333333333333);
        let assign21410_e30218: f64 = (0.25 * locals.var_t5y);
        let assign21410_e30219: f64 = (1.0 + assign21410_e30218);
        let assign21410_e30220: f64 = (assign21410_e30214 * assign21410_e30219);
        let assign21410_e30221: f64 = (1.0 + assign21410_e30220);
        let assign21410_e30222: f64 = (assign21410_e30221).sqrt();
        let assign21410_e30223: f64 = (assign21410_e30210 * assign21410_e30222);
        (assign21410_e30223, ((((assign21410_e30206 * locals.var_t5y_dn0) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn0 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn0))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn2) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn2 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn2))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn6) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn6 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn6))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn7) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn7 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn7))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn10) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn10 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn10))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn11) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn11 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn11))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn12) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn12 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn12))) / (2.0 * assign21410_e30222)))), ((((assign21410_e30206 * locals.var_t5y_dn17) * 0.7071067811865475) * assign21410_e30222) + (assign21410_e30210 * ((((locals.var_t5y_dn17 * 0.3333333333333333) * assign21410_e30219) + (assign21410_e30214 * (0.25 * locals.var_t5y_dn17))) / (2.0 * assign21410_e30222)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21410_e30225;
        locals.var_qbu_dn0 = assign21410_e30225_d_n0;
        locals.var_qbu_dn2 = assign21410_e30225_d_n2;
        locals.var_qbu_dn6 = assign21410_e30225_d_n6;
        locals.var_qbu_dn7 = assign21410_e30225_d_n7;
        locals.var_qbu_dn10 = assign21410_e30225_d_n10;
        locals.var_qbu_dn11 = assign21410_e30225_d_n11;
        locals.var_qbu_dn12 = assign21410_e30225_d_n12;
        locals.var_qbu_dn17 = assign21410_e30225_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign21420_e30238, assign21420_e30238_d_n0, assign21420_e30238_d_n2, assign21420_e30238_d_n6, assign21420_e30238_d_n7, assign21420_e30238_d_n10, assign21420_e30238_d_n11, assign21420_e30238_d_n12, assign21420_e30238_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21420_e30229: f64 = (locals.var_qbu * locals.var_qbu);
        let assign21420_e30232: f64 = (4.0 * 1e-6);
        let assign21420_e30234: f64 = (assign21420_e30232 * 1e-6);
        let assign21420_e30235: f64 = (assign21420_e30229 + assign21420_e30234);
        let assign21420_e30236: f64 = (assign21420_e30235).sqrt();
        (assign21420_e30236, (((locals.var_qbu_dn0 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn0)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn2 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn2)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn6 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn6)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn7 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn7)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn10 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn10)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn11 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn11)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn12 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn12)) / (2.0 * assign21420_e30236)), (((locals.var_qbu_dn17 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn17)) / (2.0 * assign21420_e30236)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21420_e30238;
        locals.var_tmf1_dn0 = assign21420_e30238_d_n0;
        locals.var_tmf1_dn2 = assign21420_e30238_d_n2;
        locals.var_tmf1_dn6 = assign21420_e30238_d_n6;
        locals.var_tmf1_dn7 = assign21420_e30238_d_n7;
        locals.var_tmf1_dn10 = assign21420_e30238_d_n10;
        locals.var_tmf1_dn11 = assign21420_e30238_d_n11;
        locals.var_tmf1_dn12 = assign21420_e30238_d_n12;
        locals.var_tmf1_dn17 = assign21420_e30238_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21430_e30250, assign21430_e30250_d_n0, assign21430_e30250_d_n2, assign21430_e30250_d_n6, assign21430_e30250_d_n7, assign21430_e30250_d_n10, assign21430_e30250_d_n11, assign21430_e30250_d_n12, assign21430_e30250_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21430_e30243: f64 = (locals.var_qbu + locals.var_tmf1);
        let assign21430_e30244: f64 = (0.5 * assign21430_e30243);
        let assign21430_e30247: f64 = (1e-10 * 1e-6);
        let assign21430_e30248: f64 = (assign21430_e30244 + assign21430_e30247);
        (assign21430_e30248, (0.5 * (locals.var_qbu_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_qbu_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_qbu_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn12, locals.var_wqbu_dn17,)
    }
};
        locals.var_wqbu = assign21430_e30250;
        locals.var_wqbu_dn0 = assign21430_e30250_d_n0;
        locals.var_wqbu_dn2 = assign21430_e30250_d_n2;
        locals.var_wqbu_dn6 = assign21430_e30250_d_n6;
        locals.var_wqbu_dn7 = assign21430_e30250_d_n7;
        locals.var_wqbu_dn10 = assign21430_e30250_d_n10;
        locals.var_wqbu_dn11 = assign21430_e30250_d_n11;
        locals.var_wqbu_dn12 = assign21430_e30250_d_n12;
        locals.var_wqbu_dn17 = assign21430_e30250_d_n17;
        locals.var_wqbu_rv = 0.0;

        let assign21440_e30253: f64 = if locals.var_wqbu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign21440_e30253;
        locals.var_guard661_rv = 0.0;

        let (assign21450_e30259, assign21450_e30259_d_n0, assign21450_e30259_d_n2, assign21450_e30259_d_n6, assign21450_e30259_d_n7, assign21450_e30259_d_n10, assign21450_e30259_d_n11, assign21450_e30259_d_n12, assign21450_e30259_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn12, locals.var_wqbu_dn17,)
    }
};
        locals.var_wqbu = assign21450_e30259;
        locals.var_wqbu_dn0 = assign21450_e30259_d_n0;
        locals.var_wqbu_dn2 = assign21450_e30259_d_n2;
        locals.var_wqbu_dn6 = assign21450_e30259_d_n6;
        locals.var_wqbu_dn7 = assign21450_e30259_d_n7;
        locals.var_wqbu_dn10 = assign21450_e30259_d_n10;
        locals.var_wqbu_dn11 = assign21450_e30259_d_n11;
        locals.var_wqbu_dn12 = assign21450_e30259_d_n12;
        locals.var_wqbu_dn17 = assign21450_e30259_d_n17;
        locals.var_wqbu_rv = 0.0;

        let (assign21460_e30267, assign21460_e30267_d_n0, assign21460_e30267_d_n2, assign21460_e30267_d_n6, assign21460_e30267_d_n7, assign21460_e30267_d_n10, assign21460_e30267_d_n11, assign21460_e30267_d_n12, assign21460_e30267_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21460_e30264: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign21460_e30265: f64 = (locals.var_wqbu / assign21460_e30264);
        (assign21460_e30265, (((locals.var_wqbu_dn0 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn2 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn6 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn7 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn10 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn11 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn11))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn12 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn12))) / (assign21460_e30264 * assign21460_e30264)), (((locals.var_wqbu_dn17 * assign21460_e30264) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn17))) / (assign21460_e30264 * assign21460_e30264)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn12, locals.var_wdep_dn17,)
    }
};
        locals.var_wdep = assign21460_e30267;
        locals.var_wdep_dn0 = assign21460_e30267_d_n0;
        locals.var_wdep_dn2 = assign21460_e30267_d_n2;
        locals.var_wdep_dn6 = assign21460_e30267_d_n6;
        locals.var_wdep_dn7 = assign21460_e30267_d_n7;
        locals.var_wdep_dn10 = assign21460_e30267_d_n10;
        locals.var_wdep_dn11 = assign21460_e30267_d_n11;
        locals.var_wdep_dn12 = assign21460_e30267_d_n12;
        locals.var_wdep_dn17 = assign21460_e30267_d_n17;
        locals.var_wdep_rv = 0.0;

        let (assign21470_e30273, assign21470_e30273_d_n0, assign21470_e30273_d_n2, assign21470_e30273_d_n6, assign21470_e30273_d_n7, assign21470_e30273_d_n10, assign21470_e30273_d_n11, assign21470_e30273_d_n12, assign21470_e30273_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21470_e30271: f64 = (locals.var_wdep - locals.var_wk_xj);
        (assign21470_e30271, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn12, locals.var_wdep_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21470_e30273;
        locals.var_t1_dn0 = assign21470_e30273_d_n0;
        locals.var_t1_dn2 = assign21470_e30273_d_n2;
        locals.var_t1_dn6 = assign21470_e30273_d_n6;
        locals.var_t1_dn7 = assign21470_e30273_d_n7;
        locals.var_t1_dn10 = assign21470_e30273_d_n10;
        locals.var_t1_dn11 = assign21470_e30273_d_n11;
        locals.var_t1_dn12 = assign21470_e30273_d_n12;
        locals.var_t1_dn17 = assign21470_e30273_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21480_e30279, assign21480_e30279_d_n0, assign21480_e30279_d_n2, assign21480_e30279_d_n6, assign21480_e30279_d_n7, assign21480_e30279_d_n10, assign21480_e30279_d_n11, assign21480_e30279_d_n12, assign21480_e30279_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21480_e30277: f64 = (locals.var_wdep * 0.01);
        (assign21480_e30277, (locals.var_wdep_dn0 * 0.01), (locals.var_wdep_dn2 * 0.01), (locals.var_wdep_dn6 * 0.01), (locals.var_wdep_dn7 * 0.01), (locals.var_wdep_dn10 * 0.01), (locals.var_wdep_dn11 * 0.01), (locals.var_wdep_dn12 * 0.01), (locals.var_wdep_dn17 * 0.01),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn12, locals.var_delta_dn17,)
    }
};
        locals.var_delta = assign21480_e30279;
        locals.var_delta_dn0 = assign21480_e30279_d_n0;
        locals.var_delta_dn2 = assign21480_e30279_d_n2;
        locals.var_delta_dn6 = assign21480_e30279_d_n6;
        locals.var_delta_dn7 = assign21480_e30279_d_n7;
        locals.var_delta_dn10 = assign21480_e30279_d_n10;
        locals.var_delta_dn11 = assign21480_e30279_d_n11;
        locals.var_delta_dn12 = assign21480_e30279_d_n12;
        locals.var_delta_dn17 = assign21480_e30279_d_n17;
        locals.var_delta_rv = 0.0;

        let (assign21490_e30292, assign21490_e30292_d_n0, assign21490_e30292_d_n2, assign21490_e30292_d_n6, assign21490_e30292_d_n7, assign21490_e30292_d_n10, assign21490_e30292_d_n11, assign21490_e30292_d_n12, assign21490_e30292_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21490_e30283: f64 = (locals.var_t1 * locals.var_t1);
        let assign21490_e30286: f64 = (4.0 * locals.var_delta);
        let assign21490_e30288: f64 = (assign21490_e30286 * locals.var_delta);
        let assign21490_e30289: f64 = (assign21490_e30283 + assign21490_e30288);
        let assign21490_e30290: f64 = (assign21490_e30289).sqrt();
        (assign21490_e30290, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn0))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn2))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn6))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn7))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn10))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn11))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (((4.0 * locals.var_delta_dn12) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn12))) / (2.0 * assign21490_e30290)), ((((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) + (((4.0 * locals.var_delta_dn17) * locals.var_delta) + (assign21490_e30286 * locals.var_delta_dn17))) / (2.0 * assign21490_e30290)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21490_e30292;
        locals.var_tmf1_dn0 = assign21490_e30292_d_n0;
        locals.var_tmf1_dn2 = assign21490_e30292_d_n2;
        locals.var_tmf1_dn6 = assign21490_e30292_d_n6;
        locals.var_tmf1_dn7 = assign21490_e30292_d_n7;
        locals.var_tmf1_dn10 = assign21490_e30292_d_n10;
        locals.var_tmf1_dn11 = assign21490_e30292_d_n11;
        locals.var_tmf1_dn12 = assign21490_e30292_d_n12;
        locals.var_tmf1_dn17 = assign21490_e30292_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21500_e30304, assign21500_e30304_d_n0, assign21500_e30304_d_n2, assign21500_e30304_d_n6, assign21500_e30304_d_n7, assign21500_e30304_d_n10, assign21500_e30304_d_n11, assign21500_e30304_d_n12, assign21500_e30304_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21500_e30297: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign21500_e30298: f64 = (0.5 * assign21500_e30297);
        let assign21500_e30301: f64 = (1e-10 * locals.var_delta);
        let assign21500_e30302: f64 = (assign21500_e30298 + assign21500_e30301);
        (assign21500_e30302, ((0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)) + (1e-10 * locals.var_delta_dn0)), ((0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)) + (1e-10 * locals.var_delta_dn2)), ((0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)) + (1e-10 * locals.var_delta_dn6)), ((0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)) + (1e-10 * locals.var_delta_dn7)), ((0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)) + (1e-10 * locals.var_delta_dn10)), ((0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11)) + (1e-10 * locals.var_delta_dn11)), ((0.5 * (locals.var_t1_dn12 + locals.var_tmf1_dn12)) + (1e-10 * locals.var_delta_dn12)), ((0.5 * (locals.var_t1_dn17 + locals.var_tmf1_dn17)) + (1e-10 * locals.var_delta_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21500_e30304;
        locals.var_t2_dn0 = assign21500_e30304_d_n0;
        locals.var_t2_dn2 = assign21500_e30304_d_n2;
        locals.var_t2_dn6 = assign21500_e30304_d_n6;
        locals.var_t2_dn7 = assign21500_e30304_d_n7;
        locals.var_t2_dn10 = assign21500_e30304_d_n10;
        locals.var_t2_dn11 = assign21500_e30304_d_n11;
        locals.var_t2_dn12 = assign21500_e30304_d_n12;
        locals.var_t2_dn17 = assign21500_e30304_d_n17;
        locals.var_t2_rv = 0.0;

        let assign21510_e30307: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign21510_e30307;
        locals.var_guard662_rv = 0.0;

        let (assign21520_e30313, assign21520_e30313_d_n0, assign21520_e30313_d_n2, assign21520_e30313_d_n6, assign21520_e30313_d_n7, assign21520_e30313_d_n10, assign21520_e30313_d_n11, assign21520_e30313_d_n12, assign21520_e30313_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard662 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21520_e30313;
        locals.var_t2_dn0 = assign21520_e30313_d_n0;
        locals.var_t2_dn2 = assign21520_e30313_d_n2;
        locals.var_t2_dn6 = assign21520_e30313_d_n6;
        locals.var_t2_dn7 = assign21520_e30313_d_n7;
        locals.var_t2_dn10 = assign21520_e30313_d_n10;
        locals.var_t2_dn11 = assign21520_e30313_d_n11;
        locals.var_t2_dn12 = assign21520_e30313_d_n12;
        locals.var_t2_dn17 = assign21520_e30313_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign21530_e30323, assign21530_e30323_d_n0, assign21530_e30323_d_n2, assign21530_e30323_d_n6, assign21530_e30323_d_n7, assign21530_e30323_d_n10, assign21530_e30323_d_n11, assign21530_e30323_d_n12, assign21530_e30323_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21530_e30317: f64 = (locals.var_t2 / locals.var_wdep);
        let assign21530_e30319: f64 = (assign21530_e30317 * locals.var_t2);
        let assign21530_e30321: f64 = (assign21530_e30319 / locals.var_wdep);
        (assign21530_e30321, ((((((((locals.var_t2_dn0 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn0)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn0)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn0)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn2 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn2)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn2)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn2)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn6 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn6)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn6)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn6)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn7 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn7)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn7)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn7)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn10 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn10)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn10)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn10)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn11 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn11)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn11)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn11)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn12 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn12)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn12)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn12)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn17 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn17)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21530_e30317 * locals.var_t2_dn17)) * locals.var_wdep) - (assign21530_e30319 * locals.var_wdep_dn17)) / (locals.var_wdep * locals.var_wdep)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn10, locals.var_wfactor_dn11, locals.var_wfactor_dn12, locals.var_wfactor_dn17,)
    }
};
        locals.var_wfactor = assign21530_e30323;
        locals.var_wfactor_dn0 = assign21530_e30323_d_n0;
        locals.var_wfactor_dn2 = assign21530_e30323_d_n2;
        locals.var_wfactor_dn6 = assign21530_e30323_d_n6;
        locals.var_wfactor_dn7 = assign21530_e30323_d_n7;
        locals.var_wfactor_dn10 = assign21530_e30323_d_n10;
        locals.var_wfactor_dn11 = assign21530_e30323_d_n11;
        locals.var_wfactor_dn12 = assign21530_e30323_d_n12;
        locals.var_wfactor_dn17 = assign21530_e30323_d_n17;
        locals.var_wfactor_rv = 0.0;

        let (assign21540_e30333, assign21540_e30333_d_n0, assign21540_e30333_d_n2, assign21540_e30333_d_n6, assign21540_e30333_d_n7, assign21540_e30333_d_n10, assign21540_e30333_d_n11, assign21540_e30333_d_n12, assign21540_e30333_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21540_e30327: f64 = (locals.var_ps0__blk608 - locals.var_dphi_vds);
        let assign21540_e30329: f64 = (assign21540_e30327 * locals.var_wfactor);
        let assign21540_e30331: f64 = (assign21540_e30329 + locals.var_dphi_vds);
        (assign21540_e30331, ((((locals.var_ps0__blk608_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk608_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk608_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk608_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk608_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk608_dn11 - locals.var_dphi_vds_dn11) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn11)) + locals.var_dphi_vds_dn11), ((((locals.var_ps0__blk608_dn12 - locals.var_dphi_vds_dn12) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn12)) + locals.var_dphi_vds_dn12), ((((locals.var_ps0__blk608_dn17 - locals.var_dphi_vds_dn17) * locals.var_wfactor) + (assign21540_e30327 * locals.var_wfactor_dn17)) + locals.var_dphi_vds_dn17),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn10, locals.var_phim_dn11, locals.var_phim_dn12, locals.var_phim_dn17,)
    }
};
        locals.var_phim = assign21540_e30333;
        locals.var_phim_dn0 = assign21540_e30333_d_n0;
        locals.var_phim_dn2 = assign21540_e30333_d_n2;
        locals.var_phim_dn6 = assign21540_e30333_d_n6;
        locals.var_phim_dn7 = assign21540_e30333_d_n7;
        locals.var_phim_dn10 = assign21540_e30333_d_n10;
        locals.var_phim_dn11 = assign21540_e30333_d_n11;
        locals.var_phim_dn12 = assign21540_e30333_d_n12;
        locals.var_phim_dn17 = assign21540_e30333_d_n17;
        locals.var_phim_rv = 0.0;

        let (assign21550_e30347, assign21550_e30347_d_n0, assign21550_e30347_d_n2, assign21550_e30347_d_n6, assign21550_e30347_d_n7, assign21550_e30347_d_n10, assign21550_e30347_d_n11, assign21550_e30347_d_n12, assign21550_e30347_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21550_e30337: f64 = (locals.var_beta * locals.var_phim);
        let assign21550_e30338: f64 = (assign21550_e30337).exp();
        let assign21550_e30342: f64 = (locals.var_phim - locals.var_vds);
        let assign21550_e30343: f64 = (locals.var_beta * assign21550_e30342);
        let assign21550_e30344: f64 = (assign21550_e30343).exp();
        let assign21550_e30345: f64 = (assign21550_e30338 - assign21550_e30344);
        (assign21550_e30345, ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn0)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn0 - locals.var_vds_dn0)))), ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn2)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn2 - locals.var_vds_dn2)))), ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn6)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn6 - locals.var_vds_dn6)))), ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn7)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn7 - locals.var_vds_dn7)))), ((assign21550_e30338 * ((locals.var_beta_dn10 * locals.var_phim) + (locals.var_beta * locals.var_phim_dn10))) - (assign21550_e30344 * ((locals.var_beta_dn10 * assign21550_e30342) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_vds_dn10))))), ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn11)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn11 - locals.var_vds_dn11)))), ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn12)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn12 - locals.var_vds_dn12)))), ((assign21550_e30338 * (locals.var_beta * locals.var_phim_dn17)) - (assign21550_e30344 * (locals.var_beta * (locals.var_phim_dn17 - locals.var_vds_dn17)))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign21550_e30347;
        locals.var_ty_dn0 = assign21550_e30347_d_n0;
        locals.var_ty_dn2 = assign21550_e30347_d_n2;
        locals.var_ty_dn6 = assign21550_e30347_d_n6;
        locals.var_ty_dn7 = assign21550_e30347_d_n7;
        locals.var_ty_dn10 = assign21550_e30347_d_n10;
        locals.var_ty_dn11 = assign21550_e30347_d_n11;
        locals.var_ty_dn12 = assign21550_e30347_d_n12;
        locals.var_ty_dn17 = assign21550_e30347_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign21560_e30358,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21560_e30351: f64 = (2.0 * 1.6021918e-19);
        let assign21560_e30353: f64 = (assign21560_e30351 * locals.var_uc_wk_njunc);
        let assign21560_e30355: f64 = (assign21560_e30353 * 1.034943e-10);
        let assign21560_e30356: f64 = (assign21560_e30355).sqrt();
        (assign21560_e30356,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign21560_e30358;
        locals.var_conpt00_rv = 0.0;

        let (assign21570_e30365, assign21570_e30365_d_n10,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21570_e30362: f64 = (locals.var_beta_inv).sqrt();
        let assign21570_e30363: f64 = (locals.var_conpt00 * assign21570_e30362);
        (assign21570_e30363, (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign21570_e30362))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn10,)
    }
};
        locals.var_conpt0 = assign21570_e30365;
        locals.var_conpt0_dn10 = assign21570_e30365_d_n10;
        locals.var_conpt0_rv = 0.0;

        let (assign21580_e30373, assign21580_e30373_d_n0, assign21580_e30373_d_n2, assign21580_e30373_d_n6, assign21580_e30373_d_n7, assign21580_e30373_d_n10, assign21580_e30373_d_n11, assign21580_e30373_d_n12, assign21580_e30373_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21580_e30370: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign21580_e30371: f64 = (locals.var_beta * assign21580_e30370);
        (assign21580_e30371, (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0)), (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2)), (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6)), (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7)), ((locals.var_beta_dn10 * assign21580_e30370) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), (locals.var_beta * (locals.var_phim_dn11 - locals.var_dphi_vds_dn11)), (locals.var_beta * (locals.var_phim_dn12 - locals.var_dphi_vds_dn12)), (locals.var_beta * (locals.var_phim_dn17 - locals.var_dphi_vds_dn17)),)
    } else {
        (locals.var_t1w__blk607, locals.var_t1w__blk607_dn0, locals.var_t1w__blk607_dn2, locals.var_t1w__blk607_dn6, locals.var_t1w__blk607_dn7, locals.var_t1w__blk607_dn10, locals.var_t1w__blk607_dn11, locals.var_t1w__blk607_dn12, locals.var_t1w__blk607_dn17,)
    }
};
        locals.var_t1w__blk607 = assign21580_e30373;
        locals.var_t1w__blk607_dn0 = assign21580_e30373_d_n0;
        locals.var_t1w__blk607_dn2 = assign21580_e30373_d_n2;
        locals.var_t1w__blk607_dn6 = assign21580_e30373_d_n6;
        locals.var_t1w__blk607_dn7 = assign21580_e30373_d_n7;
        locals.var_t1w__blk607_dn10 = assign21580_e30373_d_n10;
        locals.var_t1w__blk607_dn11 = assign21580_e30373_d_n11;
        locals.var_t1w__blk607_dn12 = assign21580_e30373_d_n12;
        locals.var_t1w__blk607_dn17 = assign21580_e30373_d_n17;
        locals.var_t1w__blk607_rv = 0.0;

        let assign21590_e30378: f64 = (0.2 * locals.var_beta);
        let assign21590_e30379: f64 = assign21590_e30378;
        let assign21590_e30383: f64 = (0.2 * locals.var_beta);
        let assign21590_e30386: f64 = if ((locals.var_t1w__blk607 < assign21590_e30379) && (assign21590_e30383 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard663 = assign21590_e30386;
        locals.var_guard663_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign21600_e30398, assign21600_e30398_d_n0, assign21600_e30398_d_n2, assign21600_e30398_d_n6, assign21600_e30398_d_n7, assign21600_e30398_d_n10, assign21600_e30398_d_n11, assign21600_e30398_d_n12, assign21600_e30398_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21600_e30393: f64 = (0.2 * locals.var_beta);
        let assign21600_e30394: f64 = assign21600_e30393;
        let assign21600_e30396: f64 = (assign21600_e30394 - locals.var_t1w__blk607);
        (assign21600_e30396, (-locals.var_t1w__blk607_dn0), (-locals.var_t1w__blk607_dn2), (-locals.var_t1w__blk607_dn6), (-locals.var_t1w__blk607_dn7), ((0.2 * locals.var_beta_dn10) - locals.var_t1w__blk607_dn10), (-locals.var_t1w__blk607_dn11), (-locals.var_t1w__blk607_dn12), (-locals.var_t1w__blk607_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21600_e30398;
        locals.var_tmf1_dn0 = assign21600_e30398_d_n0;
        locals.var_tmf1_dn2 = assign21600_e30398_d_n2;
        locals.var_tmf1_dn6 = assign21600_e30398_d_n6;
        locals.var_tmf1_dn7 = assign21600_e30398_d_n7;
        locals.var_tmf1_dn10 = assign21600_e30398_d_n10;
        locals.var_tmf1_dn11 = assign21600_e30398_d_n11;
        locals.var_tmf1_dn12 = assign21600_e30398_d_n12;
        locals.var_tmf1_dn17 = assign21600_e30398_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21610_e30406, assign21610_e30406_d_n0, assign21610_e30406_d_n2, assign21610_e30406_d_n6, assign21610_e30406_d_n7, assign21610_e30406_d_n10, assign21610_e30406_d_n11, assign21610_e30406_d_n12, assign21610_e30406_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21610_e30404: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21610_e30404, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign21610_e30406;
        locals.var_x2_dn0 = assign21610_e30406_d_n0;
        locals.var_x2_dn2 = assign21610_e30406_d_n2;
        locals.var_x2_dn6 = assign21610_e30406_d_n6;
        locals.var_x2_dn7 = assign21610_e30406_d_n7;
        locals.var_x2_dn10 = assign21610_e30406_d_n10;
        locals.var_x2_dn11 = assign21610_e30406_d_n11;
        locals.var_x2_dn12 = assign21610_e30406_d_n12;
        locals.var_x2_dn17 = assign21610_e30406_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign21620_e30418, assign21620_e30418_d_n0, assign21620_e30418_d_n2, assign21620_e30418_d_n6, assign21620_e30418_d_n7, assign21620_e30418_d_n10, assign21620_e30418_d_n11, assign21620_e30418_d_n12, assign21620_e30418_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21620_e30412: f64 = (0.2 * locals.var_beta);
        let assign21620_e30415: f64 = (0.2 * locals.var_beta);
        let assign21620_e30416: f64 = (assign21620_e30412 * assign21620_e30415);
        (assign21620_e30416, 0.0, 0.0, 0.0, 0.0, (((0.2 * locals.var_beta_dn10) * assign21620_e30415) + (assign21620_e30412 * (0.2 * locals.var_beta_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign21620_e30418;
        locals.var_xmax2_dn0 = assign21620_e30418_d_n0;
        locals.var_xmax2_dn2 = assign21620_e30418_d_n2;
        locals.var_xmax2_dn6 = assign21620_e30418_d_n6;
        locals.var_xmax2_dn7 = assign21620_e30418_d_n7;
        locals.var_xmax2_dn10 = assign21620_e30418_d_n10;
        locals.var_xmax2_dn11 = assign21620_e30418_d_n11;
        locals.var_xmax2_dn12 = assign21620_e30418_d_n12;
        locals.var_xmax2_dn17 = assign21620_e30418_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign21630_e30424, assign21630_e30424_d_n0, assign21630_e30424_d_n2, assign21630_e30424_d_n6, assign21630_e30424_d_n7, assign21630_e30424_d_n10, assign21630_e30424_d_n11, assign21630_e30424_d_n12, assign21630_e30424_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign21630_e30424;
        locals.var_xp_dn0 = assign21630_e30424_d_n0;
        locals.var_xp_dn2 = assign21630_e30424_d_n2;
        locals.var_xp_dn6 = assign21630_e30424_d_n6;
        locals.var_xp_dn7 = assign21630_e30424_d_n7;
        locals.var_xp_dn10 = assign21630_e30424_d_n10;
        locals.var_xp_dn11 = assign21630_e30424_d_n11;
        locals.var_xp_dn12 = assign21630_e30424_d_n12;
        locals.var_xp_dn17 = assign21630_e30424_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign21640_e30430, assign21640_e30430_d_n0, assign21640_e30430_d_n2, assign21640_e30430_d_n6, assign21640_e30430_d_n7, assign21640_e30430_d_n10, assign21640_e30430_d_n11, assign21640_e30430_d_n12, assign21640_e30430_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign21640_e30430;
        locals.var_xmp_dn0 = assign21640_e30430_d_n0;
        locals.var_xmp_dn2 = assign21640_e30430_d_n2;
        locals.var_xmp_dn6 = assign21640_e30430_d_n6;
        locals.var_xmp_dn7 = assign21640_e30430_d_n7;
        locals.var_xmp_dn10 = assign21640_e30430_d_n10;
        locals.var_xmp_dn11 = assign21640_e30430_d_n11;
        locals.var_xmp_dn12 = assign21640_e30430_d_n12;
        locals.var_xmp_dn17 = assign21640_e30430_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign21650_e30436,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign21650_e30436;
        locals.var_m0_rv = 0.0;

        let (assign21660_e30442,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21660_e30442;
        locals.var_mm_rv = 0.0;

        let (assign21670_e30448, assign21670_e30448_d_n0, assign21670_e30448_d_n2, assign21670_e30448_d_n6, assign21670_e30448_d_n7, assign21670_e30448_d_n10, assign21670_e30448_d_n11, assign21670_e30448_d_n12, assign21670_e30448_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign21670_e30448;
        locals.var_arg_dn0 = assign21670_e30448_d_n0;
        locals.var_arg_dn2 = assign21670_e30448_d_n2;
        locals.var_arg_dn6 = assign21670_e30448_d_n6;
        locals.var_arg_dn7 = assign21670_e30448_d_n7;
        locals.var_arg_dn10 = assign21670_e30448_d_n10;
        locals.var_arg_dn11 = assign21670_e30448_d_n11;
        locals.var_arg_dn12 = assign21670_e30448_d_n12;
        locals.var_arg_dn17 = assign21670_e30448_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign21680_e30454, assign21680_e30454_d_n0, assign21680_e30454_d_n2, assign21680_e30454_d_n6, assign21680_e30454_d_n7, assign21680_e30454_d_n10, assign21680_e30454_d_n11, assign21680_e30454_d_n12, assign21680_e30454_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21680_e30454;
        locals.var_dnm_dn0 = assign21680_e30454_d_n0;
        locals.var_dnm_dn2 = assign21680_e30454_d_n2;
        locals.var_dnm_dn6 = assign21680_e30454_d_n6;
        locals.var_dnm_dn7 = assign21680_e30454_d_n7;
        locals.var_dnm_dn10 = assign21680_e30454_d_n10;
        locals.var_dnm_dn11 = assign21680_e30454_d_n11;
        locals.var_dnm_dn12 = assign21680_e30454_d_n12;
        locals.var_dnm_dn17 = assign21680_e30454_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21690_e30462, assign21690_e30462_d_n0, assign21690_e30462_d_n2, assign21690_e30462_d_n6, assign21690_e30462_d_n7, assign21690_e30462_d_n10, assign21690_e30462_d_n11, assign21690_e30462_d_n12, assign21690_e30462_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21690_e30460: f64 = (locals.var_xp * locals.var_x2);
        (assign21690_e30460, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign21690_e30462;
        locals.var_xp_dn0 = assign21690_e30462_d_n0;
        locals.var_xp_dn2 = assign21690_e30462_d_n2;
        locals.var_xp_dn6 = assign21690_e30462_d_n6;
        locals.var_xp_dn7 = assign21690_e30462_d_n7;
        locals.var_xp_dn10 = assign21690_e30462_d_n10;
        locals.var_xp_dn11 = assign21690_e30462_d_n11;
        locals.var_xp_dn12 = assign21690_e30462_d_n12;
        locals.var_xp_dn17 = assign21690_e30462_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign21700_e30470, assign21700_e30470_d_n0, assign21700_e30470_d_n2, assign21700_e30470_d_n6, assign21700_e30470_d_n7, assign21700_e30470_d_n10, assign21700_e30470_d_n11, assign21700_e30470_d_n12, assign21700_e30470_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21700_e30468: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign21700_e30468, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign21700_e30470;
        locals.var_xmp_dn0 = assign21700_e30470_d_n0;
        locals.var_xmp_dn2 = assign21700_e30470_d_n2;
        locals.var_xmp_dn6 = assign21700_e30470_d_n6;
        locals.var_xmp_dn7 = assign21700_e30470_d_n7;
        locals.var_xmp_dn10 = assign21700_e30470_d_n10;
        locals.var_xmp_dn11 = assign21700_e30470_d_n11;
        locals.var_xmp_dn12 = assign21700_e30470_d_n12;
        locals.var_xmp_dn17 = assign21700_e30470_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign21710_e30478, assign21710_e30478_d_n0, assign21710_e30478_d_n2, assign21710_e30478_d_n6, assign21710_e30478_d_n7, assign21710_e30478_d_n10, assign21710_e30478_d_n11, assign21710_e30478_d_n12, assign21710_e30478_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21710_e30476: f64 = (locals.var_xp + locals.var_xmp);
        (assign21710_e30476, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign21710_e30478;
        locals.var_arg_dn0 = assign21710_e30478_d_n0;
        locals.var_arg_dn2 = assign21710_e30478_d_n2;
        locals.var_arg_dn6 = assign21710_e30478_d_n6;
        locals.var_arg_dn7 = assign21710_e30478_d_n7;
        locals.var_arg_dn10 = assign21710_e30478_d_n10;
        locals.var_arg_dn11 = assign21710_e30478_d_n11;
        locals.var_arg_dn12 = assign21710_e30478_d_n12;
        locals.var_arg_dn17 = assign21710_e30478_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign21720_e30484, assign21720_e30484_d_n0, assign21720_e30484_d_n2, assign21720_e30484_d_n6, assign21720_e30484_d_n7, assign21720_e30484_d_n10, assign21720_e30484_d_n11, assign21720_e30484_d_n12, assign21720_e30484_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21720_e30484;
        locals.var_dnm_dn0 = assign21720_e30484_d_n0;
        locals.var_dnm_dn2 = assign21720_e30484_d_n2;
        locals.var_dnm_dn6 = assign21720_e30484_d_n6;
        locals.var_dnm_dn7 = assign21720_e30484_d_n7;
        locals.var_dnm_dn10 = assign21720_e30484_d_n10;
        locals.var_dnm_dn11 = assign21720_e30484_d_n11;
        locals.var_dnm_dn12 = assign21720_e30484_d_n12;
        locals.var_dnm_dn17 = assign21720_e30484_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign21730_e30499: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard664 = assign21730_e30499;
        locals.var_guard664_rv = 0.0;

        let assign21740_e30502: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign21740_e30502;
        locals.var_guard665_rv = 0.0;

        let (assign21750_e30512,) = {
    if ((((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21750_e30512;
        locals.var_mm_rv = 0.0;

        let assign21760_e30515: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign21760_e30515;
        locals.var_guard666_rv = 0.0;

        let (assign21770_e30528,) = {
    if (((((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21770_e30528;
        locals.var_mm_rv = 0.0;

        let assign21780_e30531: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign21780_e30531;
        locals.var_guard667_rv = 0.0;

        let (assign21790_e30547,) = {
    if ((((((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) && (locals.var_guard667 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21790_e30547;
        locals.var_mm_rv = 0.0;

        let assign21800_e30550: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign21800_e30550;
        locals.var_guard668_rv = 0.0;

        let (assign21810_e30569,) = {
    if (((((((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21810_e30569;
        locals.var_mm_rv = 0.0;

        let (assign21820_e30577,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign21820_e30577;
        locals.var_m0_rv = 0.0;

        let mut assign21830_loop_guard: usize = 0;
        while {
            let assign21830_cond_e30586: f64 = if ((((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign21830_cond_e30586 != 0.0
        } {
            assign21830_loop_guard += 1;
            assert!(assign21830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign21830_body0_e30595, assign21830_body0_e30595_d_n0, assign21830_body0_e30595_d_n2, assign21830_body0_e30595_d_n6, assign21830_body0_e30595_d_n7, assign21830_body0_e30595_d_n10, assign21830_body0_e30595_d_n11, assign21830_body0_e30595_d_n12, assign21830_body0_e30595_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign21830_body0_e30593: f64 = (locals.var_dnm).sqrt();
        (assign21830_body0_e30593, (locals.var_dnm_dn0 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn2 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn6 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn7 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn10 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn11 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn12 / (2.0 * assign21830_body0_e30593)), (locals.var_dnm_dn17 / (2.0 * assign21830_body0_e30593)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign21830_body0_e30595;
            locals.var_dnm_dn0 = assign21830_body0_e30595_d_n0;
            locals.var_dnm_dn2 = assign21830_body0_e30595_d_n2;
            locals.var_dnm_dn6 = assign21830_body0_e30595_d_n6;
            locals.var_dnm_dn7 = assign21830_body0_e30595_d_n7;
            locals.var_dnm_dn10 = assign21830_body0_e30595_d_n10;
            locals.var_dnm_dn11 = assign21830_body0_e30595_d_n11;
            locals.var_dnm_dn12 = assign21830_body0_e30595_d_n12;
            locals.var_dnm_dn17 = assign21830_body0_e30595_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign21830_body1_e30605,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign21830_body1_e30603: f64 = (locals.var_m0 + 1.0);
        (assign21830_body1_e30603,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign21830_body1_e30605;
            locals.var_m0_rv = 0.0;
        }

        let (assign21840_e30620, assign21840_e30620_d_n0, assign21840_e30620_d_n2, assign21840_e30620_d_n6, assign21840_e30620_d_n7, assign21840_e30620_d_n10, assign21840_e30620_d_n11, assign21840_e30620_d_n12, assign21840_e30620_d_n17,) = {
    if (((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 == 0.0)) {
        let assign21840_e30616: f64 = 2.0;
        let assign21840_e30617: f64 = (1.0 / assign21840_e30616);
        let assign21840_e30618: f64 = (locals.var_dnm).powf(assign21840_e30617);
        (assign21840_e30618, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn0)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn2)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn6)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn7)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn10)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn11)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn12)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21840_e30617) as f64).is_finite() && ((assign21840_e30617) as f64).fract() == 0.0 { if assign21840_e30617 == 0.0 { 0.0 } else { (assign21840_e30617 * ((locals.var_dnm).powf(assign21840_e30617 - 1.0) * locals.var_dnm_dn17)) } } else { (assign21840_e30618 * (assign21840_e30617 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21840_e30620;
        locals.var_dnm_dn0 = assign21840_e30620_d_n0;
        locals.var_dnm_dn2 = assign21840_e30620_d_n2;
        locals.var_dnm_dn6 = assign21840_e30620_d_n6;
        locals.var_dnm_dn7 = assign21840_e30620_d_n7;
        locals.var_dnm_dn10 = assign21840_e30620_d_n10;
        locals.var_dnm_dn11 = assign21840_e30620_d_n11;
        locals.var_dnm_dn12 = assign21840_e30620_d_n12;
        locals.var_dnm_dn17 = assign21840_e30620_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21850_e30628, assign21850_e30628_d_n0, assign21850_e30628_d_n2, assign21850_e30628_d_n6, assign21850_e30628_d_n7, assign21850_e30628_d_n10, assign21850_e30628_d_n11, assign21850_e30628_d_n12, assign21850_e30628_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21850_e30626: f64 = (1.0 / locals.var_dnm);
        (assign21850_e30626, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21850_e30628;
        locals.var_dnm_dn0 = assign21850_e30628_d_n0;
        locals.var_dnm_dn2 = assign21850_e30628_d_n2;
        locals.var_dnm_dn6 = assign21850_e30628_d_n6;
        locals.var_dnm_dn7 = assign21850_e30628_d_n7;
        locals.var_dnm_dn10 = assign21850_e30628_d_n10;
        locals.var_dnm_dn11 = assign21850_e30628_d_n11;
        locals.var_dnm_dn12 = assign21850_e30628_d_n12;
        locals.var_dnm_dn17 = assign21850_e30628_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21860_e30640, assign21860_e30640_d_n0, assign21860_e30640_d_n2, assign21860_e30640_d_n6, assign21860_e30640_d_n7, assign21860_e30640_d_n10, assign21860_e30640_d_n11, assign21860_e30640_d_n12, assign21860_e30640_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21860_e30635: f64 = (0.2 * locals.var_beta);
        let assign21860_e30636: f64 = (locals.var_tmf1 * assign21860_e30635);
        let assign21860_e30638: f64 = (assign21860_e30636 * locals.var_dnm);
        (assign21860_e30638, (((locals.var_tmf1_dn0 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign21860_e30635) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign21860_e30635) * locals.var_dnm) + (assign21860_e30636 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign21860_e30640;
        locals.var_tmf0_dn0 = assign21860_e30640_d_n0;
        locals.var_tmf0_dn2 = assign21860_e30640_d_n2;
        locals.var_tmf0_dn6 = assign21860_e30640_d_n6;
        locals.var_tmf0_dn7 = assign21860_e30640_d_n7;
        locals.var_tmf0_dn10 = assign21860_e30640_d_n10;
        locals.var_tmf0_dn11 = assign21860_e30640_d_n11;
        locals.var_tmf0_dn12 = assign21860_e30640_d_n12;
        locals.var_tmf0_dn17 = assign21860_e30640_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign21870_e30652, assign21870_e30652_d_n0, assign21870_e30652_d_n2, assign21870_e30652_d_n6, assign21870_e30652_d_n7, assign21870_e30652_d_n10, assign21870_e30652_d_n11, assign21870_e30652_d_n12, assign21870_e30652_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign21870_e30647: f64 = (0.2 * locals.var_beta);
        let assign21870_e30648: f64 = assign21870_e30647;
        let assign21870_e30650: f64 = (assign21870_e30648 - locals.var_tmf0);
        (assign21870_e30650, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21870_e30652;
        locals.var_t1_dn0 = assign21870_e30652_d_n0;
        locals.var_t1_dn2 = assign21870_e30652_d_n2;
        locals.var_t1_dn6 = assign21870_e30652_d_n6;
        locals.var_t1_dn7 = assign21870_e30652_d_n7;
        locals.var_t1_dn10 = assign21870_e30652_d_n10;
        locals.var_t1_dn11 = assign21870_e30652_d_n11;
        locals.var_t1_dn12 = assign21870_e30652_d_n12;
        locals.var_t1_dn17 = assign21870_e30652_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21880_e30659, assign21880_e30659_d_n0, assign21880_e30659_d_n2, assign21880_e30659_d_n6, assign21880_e30659_d_n7, assign21880_e30659_d_n10, assign21880_e30659_d_n11, assign21880_e30659_d_n12, assign21880_e30659_d_n17,) = {
    if ((locals.var_guard598 != 0.0) && (locals.var_guard663 == 0.0)) {
        (locals.var_t1w__blk607, locals.var_t1w__blk607_dn0, locals.var_t1w__blk607_dn2, locals.var_t1w__blk607_dn6, locals.var_t1w__blk607_dn7, locals.var_t1w__blk607_dn10, locals.var_t1w__blk607_dn11, locals.var_t1w__blk607_dn12, locals.var_t1w__blk607_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21880_e30659;
        locals.var_t1_dn0 = assign21880_e30659_d_n0;
        locals.var_t1_dn2 = assign21880_e30659_d_n2;
        locals.var_t1_dn6 = assign21880_e30659_d_n6;
        locals.var_t1_dn7 = assign21880_e30659_d_n7;
        locals.var_t1_dn10 = assign21880_e30659_d_n10;
        locals.var_t1_dn11 = assign21880_e30659_d_n11;
        locals.var_t1_dn12 = assign21880_e30659_d_n12;
        locals.var_t1_dn17 = assign21880_e30659_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21890_e30668, assign21890_e30668_d_n0, assign21890_e30668_d_n2, assign21890_e30668_d_n6, assign21890_e30668_d_n7, assign21890_e30668_d_n10, assign21890_e30668_d_n11, assign21890_e30668_d_n12, assign21890_e30668_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21890_e30664: f64 = (10.0 * 2.220446049250313e-16);
        let assign21890_e30665: f64 = (locals.var_t1 + assign21890_e30664);
        let assign21890_e30666: f64 = (assign21890_e30665).sqrt();
        (assign21890_e30666, (locals.var_t1_dn0 / (2.0 * assign21890_e30666)), (locals.var_t1_dn2 / (2.0 * assign21890_e30666)), (locals.var_t1_dn6 / (2.0 * assign21890_e30666)), (locals.var_t1_dn7 / (2.0 * assign21890_e30666)), (locals.var_t1_dn10 / (2.0 * assign21890_e30666)), (locals.var_t1_dn11 / (2.0 * assign21890_e30666)), (locals.var_t1_dn12 / (2.0 * assign21890_e30666)), (locals.var_t1_dn17 / (2.0 * assign21890_e30666)),)
    } else {
        (locals.var_sq1npt, locals.var_sq1npt_dn0, locals.var_sq1npt_dn2, locals.var_sq1npt_dn6, locals.var_sq1npt_dn7, locals.var_sq1npt_dn10, locals.var_sq1npt_dn11, locals.var_sq1npt_dn12, locals.var_sq1npt_dn17,)
    }
};
        locals.var_sq1npt = assign21890_e30668;
        locals.var_sq1npt_dn0 = assign21890_e30668_d_n0;
        locals.var_sq1npt_dn2 = assign21890_e30668_d_n2;
        locals.var_sq1npt_dn6 = assign21890_e30668_d_n6;
        locals.var_sq1npt_dn7 = assign21890_e30668_d_n7;
        locals.var_sq1npt_dn10 = assign21890_e30668_d_n10;
        locals.var_sq1npt_dn11 = assign21890_e30668_d_n11;
        locals.var_sq1npt_dn12 = assign21890_e30668_d_n12;
        locals.var_sq1npt_dn17 = assign21890_e30668_d_n17;
        locals.var_sq1npt_rv = 0.0;

        let (assign21900_e30674, assign21900_e30674_d_n0, assign21900_e30674_d_n2, assign21900_e30674_d_n6, assign21900_e30674_d_n7, assign21900_e30674_d_n10, assign21900_e30674_d_n11, assign21900_e30674_d_n12, assign21900_e30674_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21900_e30672: f64 = (locals.var_conpt0 * locals.var_sq1npt);
        (assign21900_e30672, (locals.var_conpt0 * locals.var_sq1npt_dn0), (locals.var_conpt0 * locals.var_sq1npt_dn2), (locals.var_conpt0 * locals.var_sq1npt_dn6), (locals.var_conpt0 * locals.var_sq1npt_dn7), ((locals.var_conpt0_dn10 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn10)), (locals.var_conpt0 * locals.var_sq1npt_dn11), (locals.var_conpt0 * locals.var_sq1npt_dn12), (locals.var_conpt0 * locals.var_sq1npt_dn17),)
    } else {
        (locals.var_qn0npt, locals.var_qn0npt_dn0, locals.var_qn0npt_dn2, locals.var_qn0npt_dn6, locals.var_qn0npt_dn7, locals.var_qn0npt_dn10, locals.var_qn0npt_dn11, locals.var_qn0npt_dn12, locals.var_qn0npt_dn17,)
    }
};
        locals.var_qn0npt = assign21900_e30674;
        locals.var_qn0npt_dn0 = assign21900_e30674_d_n0;
        locals.var_qn0npt_dn2 = assign21900_e30674_d_n2;
        locals.var_qn0npt_dn6 = assign21900_e30674_d_n6;
        locals.var_qn0npt_dn7 = assign21900_e30674_d_n7;
        locals.var_qn0npt_dn10 = assign21900_e30674_d_n10;
        locals.var_qn0npt_dn11 = assign21900_e30674_d_n11;
        locals.var_qn0npt_dn12 = assign21900_e30674_d_n12;
        locals.var_qn0npt_dn17 = assign21900_e30674_d_n17;
        locals.var_qn0npt_rv = 0.0;

        let (assign21910_e30684, assign21910_e30684_d_n0, assign21910_e30684_d_n2, assign21910_e30684_d_n6, assign21910_e30684_d_n7, assign21910_e30684_d_n10, assign21910_e30684_d_n11, assign21910_e30684_d_n12, assign21910_e30684_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21910_e30678: f64 = (2.0 * locals.var_beta_inv);
        let assign21910_e30680: f64 = (assign21910_e30678 / locals.var_leff__blk605);
        let assign21910_e30682: f64 = (assign21910_e30680 * locals.var_qn0npt);
        (assign21910_e30682, (assign21910_e30680 * locals.var_qn0npt_dn0), (assign21910_e30680 * locals.var_qn0npt_dn2), (assign21910_e30680 * locals.var_qn0npt_dn6), (assign21910_e30680 * locals.var_qn0npt_dn7), ((((2.0 * locals.var_beta_inv_dn10) / locals.var_leff__blk605) * locals.var_qn0npt) + (assign21910_e30680 * locals.var_qn0npt_dn10)), (assign21910_e30680 * locals.var_qn0npt_dn11), (assign21910_e30680 * locals.var_qn0npt_dn12), (assign21910_e30680 * locals.var_qn0npt_dn17),)
    } else {
        (locals.var_wk_jnpt_a, locals.var_wk_jnpt_a_dn0, locals.var_wk_jnpt_a_dn2, locals.var_wk_jnpt_a_dn6, locals.var_wk_jnpt_a_dn7, locals.var_wk_jnpt_a_dn10, locals.var_wk_jnpt_a_dn11, locals.var_wk_jnpt_a_dn12, locals.var_wk_jnpt_a_dn17,)
    }
};
        locals.var_wk_jnpt_a = assign21910_e30684;
        locals.var_wk_jnpt_a_dn0 = assign21910_e30684_d_n0;
        locals.var_wk_jnpt_a_dn2 = assign21910_e30684_d_n2;
        locals.var_wk_jnpt_a_dn6 = assign21910_e30684_d_n6;
        locals.var_wk_jnpt_a_dn7 = assign21910_e30684_d_n7;
        locals.var_wk_jnpt_a_dn10 = assign21910_e30684_d_n10;
        locals.var_wk_jnpt_a_dn11 = assign21910_e30684_d_n11;
        locals.var_wk_jnpt_a_dn12 = assign21910_e30684_d_n12;
        locals.var_wk_jnpt_a_dn17 = assign21910_e30684_d_n17;
        locals.var_wk_jnpt_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21920_e30694, assign21920_e30694_d_n0, assign21920_e30694_d_n2, assign21920_e30694_d_n6, assign21920_e30694_d_n7, assign21920_e30694_d_n10, assign21920_e30694_d_n11, assign21920_e30694_d_n12, assign21920_e30694_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21920_e30688: f64 = (locals.var_wk_jnpt_a * locals.var_wk_mu);
        let assign21920_e30690: f64 = (assign21920_e30688 * locals.var_weff_nf);
        let assign21920_e30692: f64 = (assign21920_e30690 * locals.var_ty);
        (assign21920_e30692, ((((locals.var_wk_jnpt_a_dn0 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn0)), ((((locals.var_wk_jnpt_a_dn2 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn2)), ((((locals.var_wk_jnpt_a_dn6 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn6)), ((((locals.var_wk_jnpt_a_dn7 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn7)), ((((locals.var_wk_jnpt_a_dn10 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn10)), ((((locals.var_wk_jnpt_a_dn11 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn11)), ((((locals.var_wk_jnpt_a_dn12 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn12)), ((((locals.var_wk_jnpt_a_dn17 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn17)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn12, locals.var_idspt1_dn17,)
    }
};
        locals.var_idspt1 = assign21920_e30694;
        locals.var_idspt1_dn0 = assign21920_e30694_d_n0;
        locals.var_idspt1_dn2 = assign21920_e30694_d_n2;
        locals.var_idspt1_dn6 = assign21920_e30694_d_n6;
        locals.var_idspt1_dn7 = assign21920_e30694_d_n7;
        locals.var_idspt1_dn10 = assign21920_e30694_d_n10;
        locals.var_idspt1_dn11 = assign21920_e30694_d_n11;
        locals.var_idspt1_dn12 = assign21920_e30694_d_n12;
        locals.var_idspt1_dn17 = assign21920_e30694_d_n17;
        locals.var_idspt1_rv = 0.0;

        let (assign21930_e30700, assign21930_e30700_d_n0, assign21930_e30700_d_n2, assign21930_e30700_d_n6, assign21930_e30700_d_n7, assign21930_e30700_d_n10, assign21930_e30700_d_n11, assign21930_e30700_d_n12, assign21930_e30700_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21930_e30698: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign21930_e30698, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn11 + locals.var_idspt1_dn11), (locals.var_idsorg_dn12 + locals.var_idspt1_dn12), (locals.var_idsorg_dn17 + locals.var_idspt1_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign21930_e30700;
        locals.var_ids_dn0 = assign21930_e30700_d_n0;
        locals.var_ids_dn2 = assign21930_e30700_d_n2;
        locals.var_ids_dn6 = assign21930_e30700_d_n6;
        locals.var_ids_dn7 = assign21930_e30700_d_n7;
        locals.var_ids_dn10 = assign21930_e30700_d_n10;
        locals.var_ids_dn11 = assign21930_e30700_d_n11;
        locals.var_ids_dn12 = assign21930_e30700_d_n12;
        locals.var_ids_dn17 = assign21930_e30700_d_n17;
        locals.var_ids_rv = 0.0;

        let assign21940_e30703: f64 = (locals.var_idspt0 + locals.var_idspt1);
        locals.var_idspt = assign21940_e30703;
        locals.var_idspt_dn0 = (locals.var_idspt0_dn0 + locals.var_idspt1_dn0);
        locals.var_idspt_dn2 = (locals.var_idspt0_dn2 + locals.var_idspt1_dn2);
        locals.var_idspt_dn6 = (locals.var_idspt0_dn6 + locals.var_idspt1_dn6);
        locals.var_idspt_dn7 = (locals.var_idspt0_dn7 + locals.var_idspt1_dn7);
        locals.var_idspt_dn10 = (locals.var_idspt0_dn10 + locals.var_idspt1_dn10);
        locals.var_idspt_dn11 = (locals.var_idspt0_dn11 + locals.var_idspt1_dn11);
        locals.var_idspt_dn12 = (locals.var_idspt0_dn12 + locals.var_idspt1_dn12);
        locals.var_idspt_dn17 = (locals.var_idspt0_dn17 + locals.var_idspt1_dn17);
        locals.var_idspt_rv = 0.0;

        let assign21950_e30710: f64 = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard669 = assign21950_e30710;
        locals.var_guard669_rv = 0.0;

        let assign21960_e30717: f64 = if ((locals.var_flg_noqi == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign21960_e30717;
        locals.var_guard682_rv = 0.0;

        let (assign21970_e30723, assign21970_e30723_d_n0, assign21970_e30723_d_n2, assign21970_e30723_d_n6, assign21970_e30723_d_n7, assign21970_e30723_d_n10, assign21970_e30723_d_n11, assign21970_e30723_d_n12, assign21970_e30723_d_n17,) = {
    if ((locals.var_guard669 != 0.0) && (locals.var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21970_e30723;
        locals.var_isub_dn0 = assign21970_e30723_d_n0;
        locals.var_isub_dn2 = assign21970_e30723_d_n2;
        locals.var_isub_dn6 = assign21970_e30723_d_n6;
        locals.var_isub_dn7 = assign21970_e30723_d_n7;
        locals.var_isub_dn10 = assign21970_e30723_d_n10;
        locals.var_isub_dn11 = assign21970_e30723_d_n11;
        locals.var_isub_dn12 = assign21970_e30723_d_n12;
        locals.var_isub_dn17 = assign21970_e30723_d_n17;
        locals.var_isub_rv = 0.0;

        let assign21980_e30730: f64 = if ((p.p117 <= 0.0) || (locals.var_mks_vmax <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard683 = assign21980_e30730;
        locals.var_guard683_rv = 0.0;

        let (assign21990_e30739, assign21990_e30739_d_n0, assign21990_e30739_d_n2, assign21990_e30739_d_n6, assign21990_e30739_d_n7, assign21990_e30739_d_n10, assign21990_e30739_d_n11, assign21990_e30739_d_n12, assign21990_e30739_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21990_e30739;
        locals.var_isub_dn0 = assign21990_e30739_d_n0;
        locals.var_isub_dn2 = assign21990_e30739_d_n2;
        locals.var_isub_dn6 = assign21990_e30739_d_n6;
        locals.var_isub_dn7 = assign21990_e30739_d_n7;
        locals.var_isub_dn10 = assign21990_e30739_d_n10;
        locals.var_isub_dn11 = assign21990_e30739_d_n11;
        locals.var_isub_dn12 = assign21990_e30739_d_n12;
        locals.var_isub_dn17 = assign21990_e30739_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign22000_e30757, assign22000_e30757_d_n0, assign22000_e30757_d_n2, assign22000_e30757_d_n6, assign22000_e30757_d_n7, assign22000_e30757_d_n10, assign22000_e30757_d_n11, assign22000_e30757_d_n12, assign22000_e30757_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) {
        let assign22000_e30749: f64 = (locals.var_vgsz - locals.var_vfbsub0);
        let assign22000_e30751: f64 = (assign22000_e30749 + locals.var_dvth);
        let assign22000_e30753: f64 = (assign22000_e30751 - locals.var_dppg);
        let assign22000_e30755: f64 = (assign22000_e30753 + p.p48);
        (assign22000_e30755, ((locals.var_vgsz_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_vgsz_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    }
};
        locals.var_vgpsub = assign22000_e30757;
        locals.var_vgpsub_dn0 = assign22000_e30757_d_n0;
        locals.var_vgpsub_dn2 = assign22000_e30757_d_n2;
        locals.var_vgpsub_dn6 = assign22000_e30757_d_n6;
        locals.var_vgpsub_dn7 = assign22000_e30757_d_n7;
        locals.var_vgpsub_dn10 = assign22000_e30757_d_n10;
        locals.var_vgpsub_dn11 = assign22000_e30757_d_n11;
        locals.var_vgpsub_dn12 = assign22000_e30757_d_n12;
        locals.var_vgpsub_dn17 = assign22000_e30757_d_n17;
        locals.var_vgpsub_rv = 0.0;

        let assign22010_e30760: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign22010_e30760;
        locals.var_guard684_rv = 0.0;

        let (assign22020_e30772, assign22020_e30772_d_n0, assign22020_e30772_d_n2, assign22020_e30772_d_n6, assign22020_e30772_d_n7, assign22020_e30772_d_n10, assign22020_e30772_d_n11, assign22020_e30772_d_n12, assign22020_e30772_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_t1__blk670, locals.var_t1__blk670_dn0, locals.var_t1__blk670_dn2, locals.var_t1__blk670_dn6, locals.var_t1__blk670_dn7, locals.var_t1__blk670_dn10, locals.var_t1__blk670_dn11, locals.var_t1__blk670_dn12, locals.var_t1__blk670_dn17,)
    }
};
        locals.var_t1__blk670 = assign22020_e30772;
        locals.var_t1__blk670_dn0 = assign22020_e30772_d_n0;
        locals.var_t1__blk670_dn2 = assign22020_e30772_d_n2;
        locals.var_t1__blk670_dn6 = assign22020_e30772_d_n6;
        locals.var_t1__blk670_dn7 = assign22020_e30772_d_n7;
        locals.var_t1__blk670_dn10 = assign22020_e30772_d_n10;
        locals.var_t1__blk670_dn11 = assign22020_e30772_d_n11;
        locals.var_t1__blk670_dn12 = assign22020_e30772_d_n12;
        locals.var_t1__blk670_dn17 = assign22020_e30772_d_n17;
        locals.var_t1__blk670_rv = 0.0;

        let (assign22030_e30786, assign22030_e30786_d_n0, assign22030_e30786_d_n2, assign22030_e30786_d_n6, assign22030_e30786_d_n7, assign22030_e30786_d_n10, assign22030_e30786_d_n11, assign22030_e30786_d_n12, assign22030_e30786_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22030_e30784: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign22030_e30784, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)), ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_t7__blk677, locals.var_t7__blk677_dn0, locals.var_t7__blk677_dn2, locals.var_t7__blk677_dn6, locals.var_t7__blk677_dn7, locals.var_t7__blk677_dn10, locals.var_t7__blk677_dn11, locals.var_t7__blk677_dn12, locals.var_t7__blk677_dn17,)
    }
};
        locals.var_t7__blk677 = assign22030_e30786;
        locals.var_t7__blk677_dn0 = assign22030_e30786_d_n0;
        locals.var_t7__blk677_dn2 = assign22030_e30786_d_n2;
        locals.var_t7__blk677_dn6 = assign22030_e30786_d_n6;
        locals.var_t7__blk677_dn7 = assign22030_e30786_d_n7;
        locals.var_t7__blk677_dn10 = assign22030_e30786_d_n10;
        locals.var_t7__blk677_dn11 = assign22030_e30786_d_n11;
        locals.var_t7__blk677_dn12 = assign22030_e30786_d_n12;
        locals.var_t7__blk677_dn17 = assign22030_e30786_d_n17;
        locals.var_t7__blk677_rv = 0.0;

        let (assign22040_e30798, assign22040_e30798_d_n0, assign22040_e30798_d_n2, assign22040_e30798_d_n6, assign22040_e30798_d_n7, assign22040_e30798_d_n10, assign22040_e30798_d_n11, assign22040_e30798_d_n12, assign22040_e30798_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_qnsub_esi, locals.var_qnsub_esi_dn0, locals.var_qnsub_esi_dn2, locals.var_qnsub_esi_dn6, locals.var_qnsub_esi_dn7, locals.var_qnsub_esi_dn10, locals.var_qnsub_esi_dn11, locals.var_qnsub_esi_dn12, locals.var_qnsub_esi_dn17,)
    } else {
        (locals.var_t8__blk678, locals.var_t8__blk678_dn0, locals.var_t8__blk678_dn2, locals.var_t8__blk678_dn6, locals.var_t8__blk678_dn7, locals.var_t8__blk678_dn10, locals.var_t8__blk678_dn11, locals.var_t8__blk678_dn12, locals.var_t8__blk678_dn17,)
    }
};
        locals.var_t8__blk678 = assign22040_e30798;
        locals.var_t8__blk678_dn0 = assign22040_e30798_d_n0;
        locals.var_t8__blk678_dn2 = assign22040_e30798_d_n2;
        locals.var_t8__blk678_dn6 = assign22040_e30798_d_n6;
        locals.var_t8__blk678_dn7 = assign22040_e30798_d_n7;
        locals.var_t8__blk678_dn10 = assign22040_e30798_d_n10;
        locals.var_t8__blk678_dn11 = assign22040_e30798_d_n11;
        locals.var_t8__blk678_dn12 = assign22040_e30798_d_n12;
        locals.var_t8__blk678_dn17 = assign22040_e30798_d_n17;
        locals.var_t8__blk678_rv = 0.0;

        let (assign22050_e30812, assign22050_e30812_d_n0, assign22050_e30812_d_n2, assign22050_e30812_d_n6, assign22050_e30812_d_n7, assign22050_e30812_d_n10, assign22050_e30812_d_n11, assign22050_e30812_d_n12, assign22050_e30812_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22050_e30810: f64 = (locals.var_t8__blk678 / locals.var_t7__blk677);
        (assign22050_e30810, (((locals.var_t8__blk678_dn0 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn0)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn2 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn2)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn6 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn6)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn7 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn7)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn10 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn10)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn11 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn11)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn12 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn12)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn17 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn17)) / (locals.var_t7__blk677 * locals.var_t7__blk677)),)
    } else {
        (locals.var_t3__blk672, locals.var_t3__blk672_dn0, locals.var_t3__blk672_dn2, locals.var_t3__blk672_dn6, locals.var_t3__blk672_dn7, locals.var_t3__blk672_dn10, locals.var_t3__blk672_dn11, locals.var_t3__blk672_dn12, locals.var_t3__blk672_dn17,)
    }
};
        locals.var_t3__blk672 = assign22050_e30812;
        locals.var_t3__blk672_dn0 = assign22050_e30812_d_n0;
        locals.var_t3__blk672_dn2 = assign22050_e30812_d_n2;
        locals.var_t3__blk672_dn6 = assign22050_e30812_d_n6;
        locals.var_t3__blk672_dn7 = assign22050_e30812_d_n7;
        locals.var_t3__blk672_dn10 = assign22050_e30812_d_n10;
        locals.var_t3__blk672_dn11 = assign22050_e30812_d_n11;
        locals.var_t3__blk672_dn12 = assign22050_e30812_d_n12;
        locals.var_t3__blk672_dn17 = assign22050_e30812_d_n17;
        locals.var_t3__blk672_rv = 0.0;

        let (assign22060_e30826, assign22060_e30826_d_n0, assign22060_e30826_d_n2, assign22060_e30826_d_n6, assign22060_e30826_d_n7, assign22060_e30826_d_n10, assign22060_e30826_d_n11, assign22060_e30826_d_n12, assign22060_e30826_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22060_e30824: f64 = (2.0 / locals.var_t8__blk678);
        (assign22060_e30824, (-((2.0 * locals.var_t8__blk678_dn0) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn2) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn6) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn7) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn10) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn11) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn12) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn17) / (locals.var_t8__blk678 * locals.var_t8__blk678))),)
    } else {
        (locals.var_t9__blk679, locals.var_t9__blk679_dn0, locals.var_t9__blk679_dn2, locals.var_t9__blk679_dn6, locals.var_t9__blk679_dn7, locals.var_t9__blk679_dn10, locals.var_t9__blk679_dn11, locals.var_t9__blk679_dn12, locals.var_t9__blk679_dn17,)
    }
};
        locals.var_t9__blk679 = assign22060_e30826;
        locals.var_t9__blk679_dn0 = assign22060_e30826_d_n0;
        locals.var_t9__blk679_dn2 = assign22060_e30826_d_n2;
        locals.var_t9__blk679_dn6 = assign22060_e30826_d_n6;
        locals.var_t9__blk679_dn7 = assign22060_e30826_d_n7;
        locals.var_t9__blk679_dn10 = assign22060_e30826_d_n10;
        locals.var_t9__blk679_dn11 = assign22060_e30826_d_n11;
        locals.var_t9__blk679_dn12 = assign22060_e30826_d_n12;
        locals.var_t9__blk679_dn17 = assign22060_e30826_d_n17;
        locals.var_t9__blk679_rv = 0.0;

        let (assign22070_e30840, assign22070_e30840_d_n0, assign22070_e30840_d_n2, assign22070_e30840_d_n6, assign22070_e30840_d_n7, assign22070_e30840_d_n10, assign22070_e30840_d_n11, assign22070_e30840_d_n12, assign22070_e30840_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22070_e30838: f64 = (locals.var_t9__blk679 * locals.var_t7__blk677);
        (assign22070_e30838, ((locals.var_t9__blk679_dn0 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn0)), ((locals.var_t9__blk679_dn2 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn2)), ((locals.var_t9__blk679_dn6 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn6)), ((locals.var_t9__blk679_dn7 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn7)), ((locals.var_t9__blk679_dn10 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn10)), ((locals.var_t9__blk679_dn11 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn11)), ((locals.var_t9__blk679_dn12 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn12)), ((locals.var_t9__blk679_dn17 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn17)),)
    } else {
        (locals.var_t4__blk673, locals.var_t4__blk673_dn0, locals.var_t4__blk673_dn2, locals.var_t4__blk673_dn6, locals.var_t4__blk673_dn7, locals.var_t4__blk673_dn10, locals.var_t4__blk673_dn11, locals.var_t4__blk673_dn12, locals.var_t4__blk673_dn17,)
    }
};
        locals.var_t4__blk673 = assign22070_e30840;
        locals.var_t4__blk673_dn0 = assign22070_e30840_d_n0;
        locals.var_t4__blk673_dn2 = assign22070_e30840_d_n2;
        locals.var_t4__blk673_dn6 = assign22070_e30840_d_n6;
        locals.var_t4__blk673_dn7 = assign22070_e30840_d_n7;
        locals.var_t4__blk673_dn10 = assign22070_e30840_d_n10;
        locals.var_t4__blk673_dn11 = assign22070_e30840_d_n11;
        locals.var_t4__blk673_dn12 = assign22070_e30840_d_n12;
        locals.var_t4__blk673_dn17 = assign22070_e30840_d_n17;
        locals.var_t4__blk673_rv = 0.0;

        let (assign22080_e30858, assign22080_e30858_d_n0, assign22080_e30858_d_n2, assign22080_e30858_d_n6, assign22080_e30858_d_n7, assign22080_e30858_d_n10, assign22080_e30858_d_n11, assign22080_e30858_d_n12, assign22080_e30858_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22080_e30852: f64 = (locals.var_t1__blk670 - locals.var_beta_inv);
        let assign22080_e30855: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22080_e30856: f64 = (assign22080_e30852 - assign22080_e30855);
        (assign22080_e30856, (locals.var_t1__blk670_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk670_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk670_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk670_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk670_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk670_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk670_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk670_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22080_e30858;
        locals.var_t5__blk674_dn0 = assign22080_e30858_d_n0;
        locals.var_t5__blk674_dn2 = assign22080_e30858_d_n2;
        locals.var_t5__blk674_dn6 = assign22080_e30858_d_n6;
        locals.var_t5__blk674_dn7 = assign22080_e30858_d_n7;
        locals.var_t5__blk674_dn10 = assign22080_e30858_d_n10;
        locals.var_t5__blk674_dn11 = assign22080_e30858_d_n11;
        locals.var_t5__blk674_dn12 = assign22080_e30858_d_n12;
        locals.var_t5__blk674_dn17 = assign22080_e30858_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22090_e30874, assign22090_e30874_d_n0, assign22090_e30874_d_n2, assign22090_e30874_d_n6, assign22090_e30874_d_n7, assign22090_e30874_d_n10, assign22090_e30874_d_n11, assign22090_e30874_d_n12, assign22090_e30874_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22090_e30870: f64 = (p.p49 * locals.var_qhs);
        let assign22090_e30872: f64 = (assign22090_e30870 / locals.var_c_soi);
        (assign22090_e30872, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22090_e30874;
        locals.var_dvbssub_dn0 = assign22090_e30874_d_n0;
        locals.var_dvbssub_dn2 = assign22090_e30874_d_n2;
        locals.var_dvbssub_dn6 = assign22090_e30874_d_n6;
        locals.var_dvbssub_dn7 = assign22090_e30874_d_n7;
        locals.var_dvbssub_dn10 = assign22090_e30874_d_n10;
        locals.var_dvbssub_dn11 = assign22090_e30874_d_n11;
        locals.var_dvbssub_dn12 = assign22090_e30874_d_n12;
        locals.var_dvbssub_dn17 = assign22090_e30874_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22100_e30890, assign22100_e30890_d_n0, assign22100_e30890_d_n2, assign22100_e30890_d_n6, assign22100_e30890_d_n7, assign22100_e30890_d_n10, assign22100_e30890_d_n11, assign22100_e30890_d_n12, assign22100_e30890_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22100_e30887: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22100_e30888: f64 = (locals.var_t5__blk674 - assign22100_e30887);
        (assign22100_e30888, (locals.var_t5__blk674_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk674_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk674_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk674_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk674_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk674_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk674_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk674_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22100_e30890;
        locals.var_t5__blk674_dn0 = assign22100_e30890_d_n0;
        locals.var_t5__blk674_dn2 = assign22100_e30890_d_n2;
        locals.var_t5__blk674_dn6 = assign22100_e30890_d_n6;
        locals.var_t5__blk674_dn7 = assign22100_e30890_d_n7;
        locals.var_t5__blk674_dn10 = assign22100_e30890_d_n10;
        locals.var_t5__blk674_dn11 = assign22100_e30890_d_n11;
        locals.var_t5__blk674_dn12 = assign22100_e30890_d_n12;
        locals.var_t5__blk674_dn17 = assign22100_e30890_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22110_e30906, assign22110_e30906_d_n0, assign22110_e30906_d_n2, assign22110_e30906_d_n6, assign22110_e30906_d_n7, assign22110_e30906_d_n10, assign22110_e30906_d_n11, assign22110_e30906_d_n12, assign22110_e30906_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22110_e30903: f64 = (locals.var_t4__blk673 * locals.var_t5__blk674);
        let assign22110_e30904: f64 = (1.0 + assign22110_e30903);
        (assign22110_e30904, ((locals.var_t4__blk673_dn0 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn0)), ((locals.var_t4__blk673_dn2 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn2)), ((locals.var_t4__blk673_dn6 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn6)), ((locals.var_t4__blk673_dn7 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn7)), ((locals.var_t4__blk673_dn10 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn10)), ((locals.var_t4__blk673_dn11 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn11)), ((locals.var_t4__blk673_dn12 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn12)), ((locals.var_t4__blk673_dn17 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn17)),)
    } else {
        (locals.var_t6w__blk676, locals.var_t6w__blk676_dn0, locals.var_t6w__blk676_dn2, locals.var_t6w__blk676_dn6, locals.var_t6w__blk676_dn7, locals.var_t6w__blk676_dn10, locals.var_t6w__blk676_dn11, locals.var_t6w__blk676_dn12, locals.var_t6w__blk676_dn17,)
    }
};
        locals.var_t6w__blk676 = assign22110_e30906;
        locals.var_t6w__blk676_dn0 = assign22110_e30906_d_n0;
        locals.var_t6w__blk676_dn2 = assign22110_e30906_d_n2;
        locals.var_t6w__blk676_dn6 = assign22110_e30906_d_n6;
        locals.var_t6w__blk676_dn7 = assign22110_e30906_d_n7;
        locals.var_t6w__blk676_dn10 = assign22110_e30906_d_n10;
        locals.var_t6w__blk676_dn11 = assign22110_e30906_d_n11;
        locals.var_t6w__blk676_dn12 = assign22110_e30906_d_n12;
        locals.var_t6w__blk676_dn17 = assign22110_e30906_d_n17;
        locals.var_t6w__blk676_rv = 0.0;

        let (assign22120_e30927, assign22120_e30927_d_n0, assign22120_e30927_d_n2, assign22120_e30927_d_n6, assign22120_e30927_d_n7, assign22120_e30927_d_n10, assign22120_e30927_d_n11, assign22120_e30927_d_n12, assign22120_e30927_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22120_e30918: f64 = (locals.var_t6w__blk676 * locals.var_t6w__blk676);
        let assign22120_e30921: f64 = (4.0 * 0.001);
        let assign22120_e30923: f64 = (assign22120_e30921 * 0.001);
        let assign22120_e30924: f64 = (assign22120_e30918 + assign22120_e30923);
        let assign22120_e30925: f64 = (assign22120_e30924).sqrt();
        (assign22120_e30925, (((locals.var_t6w__blk676_dn0 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn0)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn2 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn2)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn6 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn6)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn7 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn7)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn10 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn10)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn11 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn11)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn12 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn12)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn17 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn17)) / (2.0 * assign22120_e30925)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22120_e30927;
        locals.var_tmf1_dn0 = assign22120_e30927_d_n0;
        locals.var_tmf1_dn2 = assign22120_e30927_d_n2;
        locals.var_tmf1_dn6 = assign22120_e30927_d_n6;
        locals.var_tmf1_dn7 = assign22120_e30927_d_n7;
        locals.var_tmf1_dn10 = assign22120_e30927_d_n10;
        locals.var_tmf1_dn11 = assign22120_e30927_d_n11;
        locals.var_tmf1_dn12 = assign22120_e30927_d_n12;
        locals.var_tmf1_dn17 = assign22120_e30927_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22130_e30947, assign22130_e30947_d_n0, assign22130_e30947_d_n2, assign22130_e30947_d_n6, assign22130_e30947_d_n7, assign22130_e30947_d_n10, assign22130_e30947_d_n11, assign22130_e30947_d_n12, assign22130_e30947_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22130_e30940: f64 = (locals.var_t6w__blk676 + locals.var_tmf1);
        let assign22130_e30941: f64 = (0.5 * assign22130_e30940);
        let assign22130_e30944: f64 = (1e-10 * 0.001);
        let assign22130_e30945: f64 = (assign22130_e30941 + assign22130_e30944);
        (assign22130_e30945, (0.5 * (locals.var_t6w__blk676_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk676_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk676_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk676_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk676_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk676_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk676_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk676_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22130_e30947;
        locals.var_t6__blk675_dn0 = assign22130_e30947_d_n0;
        locals.var_t6__blk675_dn2 = assign22130_e30947_d_n2;
        locals.var_t6__blk675_dn6 = assign22130_e30947_d_n6;
        locals.var_t6__blk675_dn7 = assign22130_e30947_d_n7;
        locals.var_t6__blk675_dn10 = assign22130_e30947_d_n10;
        locals.var_t6__blk675_dn11 = assign22130_e30947_d_n11;
        locals.var_t6__blk675_dn12 = assign22130_e30947_d_n12;
        locals.var_t6__blk675_dn17 = assign22130_e30947_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let assign22140_e30950: f64 = if locals.var_t6__blk675 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard685 = assign22140_e30950;
        locals.var_guard685_rv = 0.0;

        let (assign22150_e30964, assign22150_e30964_d_n0, assign22150_e30964_d_n2, assign22150_e30964_d_n6, assign22150_e30964_d_n7, assign22150_e30964_d_n10, assign22150_e30964_d_n11, assign22150_e30964_d_n12, assign22150_e30964_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22150_e30964;
        locals.var_t6__blk675_dn0 = assign22150_e30964_d_n0;
        locals.var_t6__blk675_dn2 = assign22150_e30964_d_n2;
        locals.var_t6__blk675_dn6 = assign22150_e30964_d_n6;
        locals.var_t6__blk675_dn7 = assign22150_e30964_d_n7;
        locals.var_t6__blk675_dn10 = assign22150_e30964_d_n10;
        locals.var_t6__blk675_dn11 = assign22150_e30964_d_n11;
        locals.var_t6__blk675_dn12 = assign22150_e30964_d_n12;
        locals.var_t6__blk675_dn17 = assign22150_e30964_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22160_e30978, assign22160_e30978_d_n0, assign22160_e30978_d_n2, assign22160_e30978_d_n6, assign22160_e30978_d_n7, assign22160_e30978_d_n10, assign22160_e30978_d_n11, assign22160_e30978_d_n12, assign22160_e30978_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22160_e30976: f64 = (locals.var_t6__blk675 + 1e-50);
        (assign22160_e30976, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22160_e30978;
        locals.var_t6__blk675_dn0 = assign22160_e30978_d_n0;
        locals.var_t6__blk675_dn2 = assign22160_e30978_d_n2;
        locals.var_t6__blk675_dn6 = assign22160_e30978_d_n6;
        locals.var_t6__blk675_dn7 = assign22160_e30978_d_n7;
        locals.var_t6__blk675_dn10 = assign22160_e30978_d_n10;
        locals.var_t6__blk675_dn11 = assign22160_e30978_d_n11;
        locals.var_t6__blk675_dn12 = assign22160_e30978_d_n12;
        locals.var_t6__blk675_dn17 = assign22160_e30978_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22170_e30991, assign22170_e30991_d_n0, assign22170_e30991_d_n2, assign22170_e30991_d_n6, assign22170_e30991_d_n7, assign22170_e30991_d_n10, assign22170_e30991_d_n11, assign22170_e30991_d_n12, assign22170_e30991_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22170_e30989: f64 = (locals.var_t6__blk675).sqrt();
        (assign22170_e30989, (locals.var_t6__blk675_dn0 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn2 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn6 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn7 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn10 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn11 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn12 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn17 / (2.0 * assign22170_e30989)),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22170_e30991;
        locals.var_t6__blk675_dn0 = assign22170_e30991_d_n0;
        locals.var_t6__blk675_dn2 = assign22170_e30991_d_n2;
        locals.var_t6__blk675_dn6 = assign22170_e30991_d_n6;
        locals.var_t6__blk675_dn7 = assign22170_e30991_d_n7;
        locals.var_t6__blk675_dn10 = assign22170_e30991_d_n10;
        locals.var_t6__blk675_dn11 = assign22170_e30991_d_n11;
        locals.var_t6__blk675_dn12 = assign22170_e30991_d_n12;
        locals.var_t6__blk675_dn17 = assign22170_e30991_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22180_e31011, assign22180_e31011_d_n0, assign22180_e31011_d_n2, assign22180_e31011_d_n6, assign22180_e31011_d_n7, assign22180_e31011_d_n10, assign22180_e31011_d_n11, assign22180_e31011_d_n12, assign22180_e31011_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22180_e31003: f64 = (locals.var_t1__blk670 * locals.var_uc_svgs);
        let assign22180_e31007: f64 = (1.0 - locals.var_t6__blk675);
        let assign22180_e31008: f64 = (locals.var_t3__blk672 * assign22180_e31007);
        let assign22180_e31009: f64 = (assign22180_e31003 + assign22180_e31008);
        (assign22180_e31009, ((locals.var_t1__blk670_dn0 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn0 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn0)))), ((locals.var_t1__blk670_dn2 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn2 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn2)))), ((locals.var_t1__blk670_dn6 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn6 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn6)))), ((locals.var_t1__blk670_dn7 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn7 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn7)))), ((locals.var_t1__blk670_dn10 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn10 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn10)))), ((locals.var_t1__blk670_dn11 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn11 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn11)))), ((locals.var_t1__blk670_dn12 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn12 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn12)))), ((locals.var_t1__blk670_dn17 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn17 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn17)))),)
    } else {
        (locals.var_psislsat__blk680, locals.var_psislsat__blk680_dn0, locals.var_psislsat__blk680_dn2, locals.var_psislsat__blk680_dn6, locals.var_psislsat__blk680_dn7, locals.var_psislsat__blk680_dn10, locals.var_psislsat__blk680_dn11, locals.var_psislsat__blk680_dn12, locals.var_psislsat__blk680_dn17,)
    }
};
        locals.var_psislsat__blk680 = assign22180_e31011;
        locals.var_psislsat__blk680_dn0 = assign22180_e31011_d_n0;
        locals.var_psislsat__blk680_dn2 = assign22180_e31011_d_n2;
        locals.var_psislsat__blk680_dn6 = assign22180_e31011_d_n6;
        locals.var_psislsat__blk680_dn7 = assign22180_e31011_d_n7;
        locals.var_psislsat__blk680_dn10 = assign22180_e31011_d_n10;
        locals.var_psislsat__blk680_dn11 = assign22180_e31011_d_n11;
        locals.var_psislsat__blk680_dn12 = assign22180_e31011_d_n12;
        locals.var_psislsat__blk680_dn17 = assign22180_e31011_d_n17;
        locals.var_psislsat__blk680_rv = 0.0;

        let (assign22190_e31033, assign22190_e31033_d_n0, assign22190_e31033_d_n2, assign22190_e31033_d_n6, assign22190_e31033_d_n7, assign22190_e31033_d_n10, assign22190_e31033_d_n11, assign22190_e31033_d_n12, assign22190_e31033_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22190_e31023: f64 = (p.p122 * locals.var_vdsz);
        let assign22190_e31025: f64 = (assign22190_e31023 + locals.var_ps0z);
        let assign22190_e31028: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign22190_e31030: f64 = (assign22190_e31028 * locals.var_psislsat__blk680);
        let assign22190_e31031: f64 = (assign22190_e31025 - assign22190_e31030);
        (assign22190_e31031, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0z_dn0) - (assign22190_e31028 * locals.var_psislsat__blk680_dn0)), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0z_dn2) - (assign22190_e31028 * locals.var_psislsat__blk680_dn2)), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0z_dn6) - (assign22190_e31028 * locals.var_psislsat__blk680_dn6)), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0z_dn7) - (assign22190_e31028 * locals.var_psislsat__blk680_dn7)), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0z_dn10) - (assign22190_e31028 * locals.var_psislsat__blk680_dn10)), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0z_dn11) - (assign22190_e31028 * locals.var_psislsat__blk680_dn11)), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0z_dn12) - (assign22190_e31028 * locals.var_psislsat__blk680_dn12)), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0z_dn17) - (assign22190_e31028 * locals.var_psislsat__blk680_dn17)),)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22190_e31033;
        locals.var_psisubsat__blk681_dn0 = assign22190_e31033_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22190_e31033_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22190_e31033_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22190_e31033_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22190_e31033_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22190_e31033_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22190_e31033_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22190_e31033_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22200_e31054, assign22200_e31054_d_n0, assign22200_e31054_d_n2, assign22200_e31054_d_n6, assign22200_e31054_d_n7, assign22200_e31054_d_n10, assign22200_e31054_d_n11, assign22200_e31054_d_n12, assign22200_e31054_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22200_e31045: f64 = (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681);
        let assign22200_e31048: f64 = (4.0 * 0.01);
        let assign22200_e31050: f64 = (assign22200_e31048 * 0.01);
        let assign22200_e31051: f64 = (assign22200_e31045 + assign22200_e31050);
        let assign22200_e31052: f64 = (assign22200_e31051).sqrt();
        (assign22200_e31052, (((locals.var_psisubsat__blk681_dn0 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn0)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn2 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn2)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn6 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn6)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn7 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn7)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn10 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn10)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn11 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn11)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn12 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn12)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn17 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn17)) / (2.0 * assign22200_e31052)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22200_e31054;
        locals.var_tmf1_dn0 = assign22200_e31054_d_n0;
        locals.var_tmf1_dn2 = assign22200_e31054_d_n2;
        locals.var_tmf1_dn6 = assign22200_e31054_d_n6;
        locals.var_tmf1_dn7 = assign22200_e31054_d_n7;
        locals.var_tmf1_dn10 = assign22200_e31054_d_n10;
        locals.var_tmf1_dn11 = assign22200_e31054_d_n11;
        locals.var_tmf1_dn12 = assign22200_e31054_d_n12;
        locals.var_tmf1_dn17 = assign22200_e31054_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22210_e31074, assign22210_e31074_d_n0, assign22210_e31074_d_n2, assign22210_e31074_d_n6, assign22210_e31074_d_n7, assign22210_e31074_d_n10, assign22210_e31074_d_n11, assign22210_e31074_d_n12, assign22210_e31074_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22210_e31067: f64 = (locals.var_psisubsat__blk681 + locals.var_tmf1);
        let assign22210_e31068: f64 = (0.5 * assign22210_e31067);
        let assign22210_e31071: f64 = (1e-10 * 0.01);
        let assign22210_e31072: f64 = (assign22210_e31068 + assign22210_e31071);
        (assign22210_e31072, (0.5 * (locals.var_psisubsat__blk681_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat__blk681_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat__blk681_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat__blk681_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat__blk681_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat__blk681_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat__blk681_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat__blk681_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22210_e31074;
        locals.var_psisubsat__blk681_dn0 = assign22210_e31074_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22210_e31074_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22210_e31074_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22210_e31074_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22210_e31074_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22210_e31074_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22210_e31074_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22210_e31074_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let assign22220_e31077: f64 = if locals.var_psisubsat__blk681 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign22220_e31077;
        locals.var_guard686_rv = 0.0;

        let (assign22230_e31091, assign22230_e31091_d_n0, assign22230_e31091_d_n2, assign22230_e31091_d_n6, assign22230_e31091_d_n7, assign22230_e31091_d_n10, assign22230_e31091_d_n11, assign22230_e31091_d_n12, assign22230_e31091_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard686 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22230_e31091;
        locals.var_psisubsat__blk681_dn0 = assign22230_e31091_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22230_e31091_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22230_e31091_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22230_e31091_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22230_e31091_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22230_e31091_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22230_e31091_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22230_e31091_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22240_e31106, assign22240_e31106_d_n0, assign22240_e31106_d_n2, assign22240_e31106_d_n6, assign22240_e31106_d_n7, assign22240_e31106_d_n10, assign22240_e31106_d_n11, assign22240_e31106_d_n12, assign22240_e31106_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22240_e31104: f64 = (locals.var_vg2const * locals.var_vgpsub);
        (assign22240_e31104, ((locals.var_vg2const_dn0 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn2)), ((locals.var_vg2const_dn6 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn7)), ((locals.var_vg2const_dn10 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn10)), ((locals.var_vg2const_dn11 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn11)), ((locals.var_vg2const_dn12 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn12)), ((locals.var_vg2const_dn17 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn17)),)
    } else {
        (locals.var_t1__blk670, locals.var_t1__blk670_dn0, locals.var_t1__blk670_dn2, locals.var_t1__blk670_dn6, locals.var_t1__blk670_dn7, locals.var_t1__blk670_dn10, locals.var_t1__blk670_dn11, locals.var_t1__blk670_dn12, locals.var_t1__blk670_dn17,)
    }
};
        locals.var_t1__blk670 = assign22240_e31106;
        locals.var_t1__blk670_dn0 = assign22240_e31106_d_n0;
        locals.var_t1__blk670_dn2 = assign22240_e31106_d_n2;
        locals.var_t1__blk670_dn6 = assign22240_e31106_d_n6;
        locals.var_t1__blk670_dn7 = assign22240_e31106_d_n7;
        locals.var_t1__blk670_dn10 = assign22240_e31106_d_n10;
        locals.var_t1__blk670_dn11 = assign22240_e31106_d_n11;
        locals.var_t1__blk670_dn12 = assign22240_e31106_d_n12;
        locals.var_t1__blk670_dn17 = assign22240_e31106_d_n17;
        locals.var_t1__blk670_rv = 0.0;

        let (assign22250_e31123, assign22250_e31123_d_n0, assign22250_e31123_d_n2, assign22250_e31123_d_n6, assign22250_e31123_d_n7, assign22250_e31123_d_n10, assign22250_e31123_d_n11, assign22250_e31123_d_n12, assign22250_e31123_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22250_e31120: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign22250_e31121: f64 = (locals.var_qnsub_esi / assign22250_e31120);
        (assign22250_e31121, (((locals.var_qnsub_esi_dn0 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn2 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn6 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn7 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn10 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn11 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn12 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn17 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign22250_e31120 * assign22250_e31120)),)
    } else {
        (locals.var_t3__blk672, locals.var_t3__blk672_dn0, locals.var_t3__blk672_dn2, locals.var_t3__blk672_dn6, locals.var_t3__blk672_dn7, locals.var_t3__blk672_dn10, locals.var_t3__blk672_dn11, locals.var_t3__blk672_dn12, locals.var_t3__blk672_dn17,)
    }
};
        locals.var_t3__blk672 = assign22250_e31123;
        locals.var_t3__blk672_dn0 = assign22250_e31123_d_n0;
        locals.var_t3__blk672_dn2 = assign22250_e31123_d_n2;
        locals.var_t3__blk672_dn6 = assign22250_e31123_d_n6;
        locals.var_t3__blk672_dn7 = assign22250_e31123_d_n7;
        locals.var_t3__blk672_dn10 = assign22250_e31123_d_n10;
        locals.var_t3__blk672_dn11 = assign22250_e31123_d_n11;
        locals.var_t3__blk672_dn12 = assign22250_e31123_d_n12;
        locals.var_t3__blk672_dn17 = assign22250_e31123_d_n17;
        locals.var_t3__blk672_rv = 0.0;

        let (assign22260_e31142, assign22260_e31142_d_n0, assign22260_e31142_d_n2, assign22260_e31142_d_n6, assign22260_e31142_d_n7, assign22260_e31142_d_n10, assign22260_e31142_d_n11, assign22260_e31142_d_n12, assign22260_e31142_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22260_e31136: f64 = (2.0 / locals.var_qnsub_esi);
        let assign22260_e31139: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign22260_e31140: f64 = (assign22260_e31136 * assign22260_e31139);
        (assign22260_e31140, (((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))), (((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))), (((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))), (((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))), (((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))), (((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))), (((-((2.0 * locals.var_qnsub_esi_dn12) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))), (((-((2.0 * locals.var_qnsub_esi_dn17) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))),)
    } else {
        (locals.var_t4__blk673, locals.var_t4__blk673_dn0, locals.var_t4__blk673_dn2, locals.var_t4__blk673_dn6, locals.var_t4__blk673_dn7, locals.var_t4__blk673_dn10, locals.var_t4__blk673_dn11, locals.var_t4__blk673_dn12, locals.var_t4__blk673_dn17,)
    }
};
        locals.var_t4__blk673 = assign22260_e31142;
        locals.var_t4__blk673_dn0 = assign22260_e31142_d_n0;
        locals.var_t4__blk673_dn2 = assign22260_e31142_d_n2;
        locals.var_t4__blk673_dn6 = assign22260_e31142_d_n6;
        locals.var_t4__blk673_dn7 = assign22260_e31142_d_n7;
        locals.var_t4__blk673_dn10 = assign22260_e31142_d_n10;
        locals.var_t4__blk673_dn11 = assign22260_e31142_d_n11;
        locals.var_t4__blk673_dn12 = assign22260_e31142_d_n12;
        locals.var_t4__blk673_dn17 = assign22260_e31142_d_n17;
        locals.var_t4__blk673_rv = 0.0;

        let (assign22270_e31161, assign22270_e31161_d_n0, assign22270_e31161_d_n2, assign22270_e31161_d_n6, assign22270_e31161_d_n7, assign22270_e31161_d_n10, assign22270_e31161_d_n11, assign22270_e31161_d_n12, assign22270_e31161_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22270_e31155: f64 = (locals.var_t1__blk670 - locals.var_beta_inv);
        let assign22270_e31158: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22270_e31159: f64 = (assign22270_e31155 - assign22270_e31158);
        (assign22270_e31159, (locals.var_t1__blk670_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk670_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk670_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk670_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk670_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk670_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk670_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk670_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22270_e31161;
        locals.var_t5__blk674_dn0 = assign22270_e31161_d_n0;
        locals.var_t5__blk674_dn2 = assign22270_e31161_d_n2;
        locals.var_t5__blk674_dn6 = assign22270_e31161_d_n6;
        locals.var_t5__blk674_dn7 = assign22270_e31161_d_n7;
        locals.var_t5__blk674_dn10 = assign22270_e31161_d_n10;
        locals.var_t5__blk674_dn11 = assign22270_e31161_d_n11;
        locals.var_t5__blk674_dn12 = assign22270_e31161_d_n12;
        locals.var_t5__blk674_dn17 = assign22270_e31161_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22280_e31178, assign22280_e31178_d_n0, assign22280_e31178_d_n2, assign22280_e31178_d_n6, assign22280_e31178_d_n7, assign22280_e31178_d_n10, assign22280_e31178_d_n11, assign22280_e31178_d_n12, assign22280_e31178_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22280_e31174: f64 = (p.p49 * locals.var_qhs);
        let assign22280_e31176: f64 = (assign22280_e31174 / locals.var_c_soi);
        (assign22280_e31176, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22280_e31178;
        locals.var_dvbssub_dn0 = assign22280_e31178_d_n0;
        locals.var_dvbssub_dn2 = assign22280_e31178_d_n2;
        locals.var_dvbssub_dn6 = assign22280_e31178_d_n6;
        locals.var_dvbssub_dn7 = assign22280_e31178_d_n7;
        locals.var_dvbssub_dn10 = assign22280_e31178_d_n10;
        locals.var_dvbssub_dn11 = assign22280_e31178_d_n11;
        locals.var_dvbssub_dn12 = assign22280_e31178_d_n12;
        locals.var_dvbssub_dn17 = assign22280_e31178_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22290_e31195, assign22290_e31195_d_n0, assign22290_e31195_d_n2, assign22290_e31195_d_n6, assign22290_e31195_d_n7, assign22290_e31195_d_n10, assign22290_e31195_d_n11, assign22290_e31195_d_n12, assign22290_e31195_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22290_e31192: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22290_e31193: f64 = (locals.var_t5__blk674 - assign22290_e31192);
        (assign22290_e31193, (locals.var_t5__blk674_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk674_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk674_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk674_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk674_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk674_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk674_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk674_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22290_e31195;
        locals.var_t5__blk674_dn0 = assign22290_e31195_d_n0;
        locals.var_t5__blk674_dn2 = assign22290_e31195_d_n2;
        locals.var_t5__blk674_dn6 = assign22290_e31195_d_n6;
        locals.var_t5__blk674_dn7 = assign22290_e31195_d_n7;
        locals.var_t5__blk674_dn10 = assign22290_e31195_d_n10;
        locals.var_t5__blk674_dn11 = assign22290_e31195_d_n11;
        locals.var_t5__blk674_dn12 = assign22290_e31195_d_n12;
        locals.var_t5__blk674_dn17 = assign22290_e31195_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22300_e31212, assign22300_e31212_d_n0, assign22300_e31212_d_n2, assign22300_e31212_d_n6, assign22300_e31212_d_n7, assign22300_e31212_d_n10, assign22300_e31212_d_n11, assign22300_e31212_d_n12, assign22300_e31212_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22300_e31209: f64 = (locals.var_t4__blk673 * locals.var_t5__blk674);
        let assign22300_e31210: f64 = (1.0 + assign22300_e31209);
        (assign22300_e31210, ((locals.var_t4__blk673_dn0 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn0)), ((locals.var_t4__blk673_dn2 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn2)), ((locals.var_t4__blk673_dn6 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn6)), ((locals.var_t4__blk673_dn7 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn7)), ((locals.var_t4__blk673_dn10 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn10)), ((locals.var_t4__blk673_dn11 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn11)), ((locals.var_t4__blk673_dn12 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn12)), ((locals.var_t4__blk673_dn17 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn17)),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22300_e31212;
        locals.var_t6__blk675_dn0 = assign22300_e31212_d_n0;
        locals.var_t6__blk675_dn2 = assign22300_e31212_d_n2;
        locals.var_t6__blk675_dn6 = assign22300_e31212_d_n6;
        locals.var_t6__blk675_dn7 = assign22300_e31212_d_n7;
        locals.var_t6__blk675_dn10 = assign22300_e31212_d_n10;
        locals.var_t6__blk675_dn11 = assign22300_e31212_d_n11;
        locals.var_t6__blk675_dn12 = assign22300_e31212_d_n12;
        locals.var_t6__blk675_dn17 = assign22300_e31212_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22310_e31229, assign22310_e31229_d_n0, assign22310_e31229_d_n2, assign22310_e31229_d_n6, assign22310_e31229_d_n7, assign22310_e31229_d_n10, assign22310_e31229_d_n11, assign22310_e31229_d_n12, assign22310_e31229_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22310_e31226: f64 = (1.0 + locals.var_t4__blk673);
        let assign22310_e31227: f64 = (2.0 * assign22310_e31226);
        (assign22310_e31227, (2.0 * locals.var_t4__blk673_dn0), (2.0 * locals.var_t4__blk673_dn2), (2.0 * locals.var_t4__blk673_dn6), (2.0 * locals.var_t4__blk673_dn7), (2.0 * locals.var_t4__blk673_dn10), (2.0 * locals.var_t4__blk673_dn11), (2.0 * locals.var_t4__blk673_dn12), (2.0 * locals.var_t4__blk673_dn17),)
    } else {
        (locals.var_t7__blk677, locals.var_t7__blk677_dn0, locals.var_t7__blk677_dn2, locals.var_t7__blk677_dn6, locals.var_t7__blk677_dn7, locals.var_t7__blk677_dn10, locals.var_t7__blk677_dn11, locals.var_t7__blk677_dn12, locals.var_t7__blk677_dn17,)
    }
};
        locals.var_t7__blk677 = assign22310_e31229;
        locals.var_t7__blk677_dn0 = assign22310_e31229_d_n0;
        locals.var_t7__blk677_dn2 = assign22310_e31229_d_n2;
        locals.var_t7__blk677_dn6 = assign22310_e31229_d_n6;
        locals.var_t7__blk677_dn7 = assign22310_e31229_d_n7;
        locals.var_t7__blk677_dn10 = assign22310_e31229_d_n10;
        locals.var_t7__blk677_dn11 = assign22310_e31229_d_n11;
        locals.var_t7__blk677_dn12 = assign22310_e31229_d_n12;
        locals.var_t7__blk677_dn17 = assign22310_e31229_d_n17;
        locals.var_t7__blk677_rv = 0.0;

        let assign22320_e31233: f64 = (1e-50 + locals.var_t7__blk677);
        let assign22320_e31238: f64 = if ((locals.var_t6__blk675 < assign22320_e31233) && (locals.var_t7__blk677 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard687 = assign22320_e31238;
        locals.var_guard687_rv = 0.0;

        let (assign22330_e31257, assign22330_e31257_d_n0, assign22330_e31257_d_n2, assign22330_e31257_d_n6, assign22330_e31257_d_n7, assign22330_e31257_d_n10, assign22330_e31257_d_n11, assign22330_e31257_d_n12, assign22330_e31257_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22330_e31253: f64 = (1e-50 + locals.var_t7__blk677);
        let assign22330_e31255: f64 = (assign22330_e31253 - locals.var_t6__blk675);
        (assign22330_e31255, (locals.var_t7__blk677_dn0 - locals.var_t6__blk675_dn0), (locals.var_t7__blk677_dn2 - locals.var_t6__blk675_dn2), (locals.var_t7__blk677_dn6 - locals.var_t6__blk675_dn6), (locals.var_t7__blk677_dn7 - locals.var_t6__blk675_dn7), (locals.var_t7__blk677_dn10 - locals.var_t6__blk675_dn10), (locals.var_t7__blk677_dn11 - locals.var_t6__blk675_dn11), (locals.var_t7__blk677_dn12 - locals.var_t6__blk675_dn12), (locals.var_t7__blk677_dn17 - locals.var_t6__blk675_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22330_e31257;
        locals.var_tmf1_dn0 = assign22330_e31257_d_n0;
        locals.var_tmf1_dn2 = assign22330_e31257_d_n2;
        locals.var_tmf1_dn6 = assign22330_e31257_d_n6;
        locals.var_tmf1_dn7 = assign22330_e31257_d_n7;
        locals.var_tmf1_dn10 = assign22330_e31257_d_n10;
        locals.var_tmf1_dn11 = assign22330_e31257_d_n11;
        locals.var_tmf1_dn12 = assign22330_e31257_d_n12;
        locals.var_tmf1_dn17 = assign22330_e31257_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22340_e31274, assign22340_e31274_d_n0, assign22340_e31274_d_n2, assign22340_e31274_d_n6, assign22340_e31274_d_n7, assign22340_e31274_d_n10, assign22340_e31274_d_n11, assign22340_e31274_d_n12, assign22340_e31274_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22340_e31272: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign22340_e31272, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign22340_e31274;
        locals.var_x2_dn0 = assign22340_e31274_d_n0;
        locals.var_x2_dn2 = assign22340_e31274_d_n2;
        locals.var_x2_dn6 = assign22340_e31274_d_n6;
        locals.var_x2_dn7 = assign22340_e31274_d_n7;
        locals.var_x2_dn10 = assign22340_e31274_d_n10;
        locals.var_x2_dn11 = assign22340_e31274_d_n11;
        locals.var_x2_dn12 = assign22340_e31274_d_n12;
        locals.var_x2_dn17 = assign22340_e31274_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign22350_e31291, assign22350_e31291_d_n0, assign22350_e31291_d_n2, assign22350_e31291_d_n6, assign22350_e31291_d_n7, assign22350_e31291_d_n10, assign22350_e31291_d_n11, assign22350_e31291_d_n12, assign22350_e31291_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22350_e31289: f64 = (locals.var_t7__blk677 * locals.var_t7__blk677);
        (assign22350_e31289, ((locals.var_t7__blk677_dn0 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn0)), ((locals.var_t7__blk677_dn2 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn2)), ((locals.var_t7__blk677_dn6 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn6)), ((locals.var_t7__blk677_dn7 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn7)), ((locals.var_t7__blk677_dn10 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn10)), ((locals.var_t7__blk677_dn11 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn11)), ((locals.var_t7__blk677_dn12 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn12)), ((locals.var_t7__blk677_dn17 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign22350_e31291;
        locals.var_xmax2_dn0 = assign22350_e31291_d_n0;
        locals.var_xmax2_dn2 = assign22350_e31291_d_n2;
        locals.var_xmax2_dn6 = assign22350_e31291_d_n6;
        locals.var_xmax2_dn7 = assign22350_e31291_d_n7;
        locals.var_xmax2_dn10 = assign22350_e31291_d_n10;
        locals.var_xmax2_dn11 = assign22350_e31291_d_n11;
        locals.var_xmax2_dn12 = assign22350_e31291_d_n12;
        locals.var_xmax2_dn17 = assign22350_e31291_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign22360_e31306, assign22360_e31306_d_n0, assign22360_e31306_d_n2, assign22360_e31306_d_n6, assign22360_e31306_d_n7, assign22360_e31306_d_n10, assign22360_e31306_d_n11, assign22360_e31306_d_n12, assign22360_e31306_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22360_e31306;
        locals.var_xp_dn0 = assign22360_e31306_d_n0;
        locals.var_xp_dn2 = assign22360_e31306_d_n2;
        locals.var_xp_dn6 = assign22360_e31306_d_n6;
        locals.var_xp_dn7 = assign22360_e31306_d_n7;
        locals.var_xp_dn10 = assign22360_e31306_d_n10;
        locals.var_xp_dn11 = assign22360_e31306_d_n11;
        locals.var_xp_dn12 = assign22360_e31306_d_n12;
        locals.var_xp_dn17 = assign22360_e31306_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22370_e31321, assign22370_e31321_d_n0, assign22370_e31321_d_n2, assign22370_e31321_d_n6, assign22370_e31321_d_n7, assign22370_e31321_d_n10, assign22370_e31321_d_n11, assign22370_e31321_d_n12, assign22370_e31321_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22370_e31321;
        locals.var_xmp_dn0 = assign22370_e31321_d_n0;
        locals.var_xmp_dn2 = assign22370_e31321_d_n2;
        locals.var_xmp_dn6 = assign22370_e31321_d_n6;
        locals.var_xmp_dn7 = assign22370_e31321_d_n7;
        locals.var_xmp_dn10 = assign22370_e31321_d_n10;
        locals.var_xmp_dn11 = assign22370_e31321_d_n11;
        locals.var_xmp_dn12 = assign22370_e31321_d_n12;
        locals.var_xmp_dn17 = assign22370_e31321_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22380_e31336,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign22380_e31336;
        locals.var_m0_rv = 0.0;

        let (assign22390_e31351,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22390_e31351;
        locals.var_mm_rv = 0.0;

        let (assign22400_e31366, assign22400_e31366_d_n0, assign22400_e31366_d_n2, assign22400_e31366_d_n6, assign22400_e31366_d_n7, assign22400_e31366_d_n10, assign22400_e31366_d_n11, assign22400_e31366_d_n12, assign22400_e31366_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign22400_e31366;
        locals.var_arg_dn0 = assign22400_e31366_d_n0;
        locals.var_arg_dn2 = assign22400_e31366_d_n2;
        locals.var_arg_dn6 = assign22400_e31366_d_n6;
        locals.var_arg_dn7 = assign22400_e31366_d_n7;
        locals.var_arg_dn10 = assign22400_e31366_d_n10;
        locals.var_arg_dn11 = assign22400_e31366_d_n11;
        locals.var_arg_dn12 = assign22400_e31366_d_n12;
        locals.var_arg_dn17 = assign22400_e31366_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign22410_e31381, assign22410_e31381_d_n0, assign22410_e31381_d_n2, assign22410_e31381_d_n6, assign22410_e31381_d_n7, assign22410_e31381_d_n10, assign22410_e31381_d_n11, assign22410_e31381_d_n12, assign22410_e31381_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22410_e31381;
        locals.var_dnm_dn0 = assign22410_e31381_d_n0;
        locals.var_dnm_dn2 = assign22410_e31381_d_n2;
        locals.var_dnm_dn6 = assign22410_e31381_d_n6;
        locals.var_dnm_dn7 = assign22410_e31381_d_n7;
        locals.var_dnm_dn10 = assign22410_e31381_d_n10;
        locals.var_dnm_dn11 = assign22410_e31381_d_n11;
        locals.var_dnm_dn12 = assign22410_e31381_d_n12;
        locals.var_dnm_dn17 = assign22410_e31381_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22420_e31398, assign22420_e31398_d_n0, assign22420_e31398_d_n2, assign22420_e31398_d_n6, assign22420_e31398_d_n7, assign22420_e31398_d_n10, assign22420_e31398_d_n11, assign22420_e31398_d_n12, assign22420_e31398_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22420_e31396: f64 = (locals.var_xp * locals.var_x2);
        (assign22420_e31396, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22420_e31398;
        locals.var_xp_dn0 = assign22420_e31398_d_n0;
        locals.var_xp_dn2 = assign22420_e31398_d_n2;
        locals.var_xp_dn6 = assign22420_e31398_d_n6;
        locals.var_xp_dn7 = assign22420_e31398_d_n7;
        locals.var_xp_dn10 = assign22420_e31398_d_n10;
        locals.var_xp_dn11 = assign22420_e31398_d_n11;
        locals.var_xp_dn12 = assign22420_e31398_d_n12;
        locals.var_xp_dn17 = assign22420_e31398_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22430_e31415, assign22430_e31415_d_n0, assign22430_e31415_d_n2, assign22430_e31415_d_n6, assign22430_e31415_d_n7, assign22430_e31415_d_n10, assign22430_e31415_d_n11, assign22430_e31415_d_n12, assign22430_e31415_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22430_e31413: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22430_e31413, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22430_e31415;
        locals.var_xmp_dn0 = assign22430_e31415_d_n0;
        locals.var_xmp_dn2 = assign22430_e31415_d_n2;
        locals.var_xmp_dn6 = assign22430_e31415_d_n6;
        locals.var_xmp_dn7 = assign22430_e31415_d_n7;
        locals.var_xmp_dn10 = assign22430_e31415_d_n10;
        locals.var_xmp_dn11 = assign22430_e31415_d_n11;
        locals.var_xmp_dn12 = assign22430_e31415_d_n12;
        locals.var_xmp_dn17 = assign22430_e31415_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22440_e31432, assign22440_e31432_d_n0, assign22440_e31432_d_n2, assign22440_e31432_d_n6, assign22440_e31432_d_n7, assign22440_e31432_d_n10, assign22440_e31432_d_n11, assign22440_e31432_d_n12, assign22440_e31432_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22440_e31430: f64 = (locals.var_xp * locals.var_x2);
        (assign22440_e31430, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22440_e31432;
        locals.var_xp_dn0 = assign22440_e31432_d_n0;
        locals.var_xp_dn2 = assign22440_e31432_d_n2;
        locals.var_xp_dn6 = assign22440_e31432_d_n6;
        locals.var_xp_dn7 = assign22440_e31432_d_n7;
        locals.var_xp_dn10 = assign22440_e31432_d_n10;
        locals.var_xp_dn11 = assign22440_e31432_d_n11;
        locals.var_xp_dn12 = assign22440_e31432_d_n12;
        locals.var_xp_dn17 = assign22440_e31432_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22450_e31449, assign22450_e31449_d_n0, assign22450_e31449_d_n2, assign22450_e31449_d_n6, assign22450_e31449_d_n7, assign22450_e31449_d_n10, assign22450_e31449_d_n11, assign22450_e31449_d_n12, assign22450_e31449_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22450_e31447: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22450_e31447, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22450_e31449;
        locals.var_xmp_dn0 = assign22450_e31449_d_n0;
        locals.var_xmp_dn2 = assign22450_e31449_d_n2;
        locals.var_xmp_dn6 = assign22450_e31449_d_n6;
        locals.var_xmp_dn7 = assign22450_e31449_d_n7;
        locals.var_xmp_dn10 = assign22450_e31449_d_n10;
        locals.var_xmp_dn11 = assign22450_e31449_d_n11;
        locals.var_xmp_dn12 = assign22450_e31449_d_n12;
        locals.var_xmp_dn17 = assign22450_e31449_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22460_e31466, assign22460_e31466_d_n0, assign22460_e31466_d_n2, assign22460_e31466_d_n6, assign22460_e31466_d_n7, assign22460_e31466_d_n10, assign22460_e31466_d_n11, assign22460_e31466_d_n12, assign22460_e31466_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22460_e31464: f64 = (locals.var_xp * locals.var_x2);
        (assign22460_e31464, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22460_e31466;
        locals.var_xp_dn0 = assign22460_e31466_d_n0;
        locals.var_xp_dn2 = assign22460_e31466_d_n2;
        locals.var_xp_dn6 = assign22460_e31466_d_n6;
        locals.var_xp_dn7 = assign22460_e31466_d_n7;
        locals.var_xp_dn10 = assign22460_e31466_d_n10;
        locals.var_xp_dn11 = assign22460_e31466_d_n11;
        locals.var_xp_dn12 = assign22460_e31466_d_n12;
        locals.var_xp_dn17 = assign22460_e31466_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22470_e31483, assign22470_e31483_d_n0, assign22470_e31483_d_n2, assign22470_e31483_d_n6, assign22470_e31483_d_n7, assign22470_e31483_d_n10, assign22470_e31483_d_n11, assign22470_e31483_d_n12, assign22470_e31483_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22470_e31481: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22470_e31481, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22470_e31483;
        locals.var_xmp_dn0 = assign22470_e31483_d_n0;
        locals.var_xmp_dn2 = assign22470_e31483_d_n2;
        locals.var_xmp_dn6 = assign22470_e31483_d_n6;
        locals.var_xmp_dn7 = assign22470_e31483_d_n7;
        locals.var_xmp_dn10 = assign22470_e31483_d_n10;
        locals.var_xmp_dn11 = assign22470_e31483_d_n11;
        locals.var_xmp_dn12 = assign22470_e31483_d_n12;
        locals.var_xmp_dn17 = assign22470_e31483_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22480_e31500, assign22480_e31500_d_n0, assign22480_e31500_d_n2, assign22480_e31500_d_n6, assign22480_e31500_d_n7, assign22480_e31500_d_n10, assign22480_e31500_d_n11, assign22480_e31500_d_n12, assign22480_e31500_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22480_e31498: f64 = (locals.var_xp * locals.var_x2);
        (assign22480_e31498, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22480_e31500;
        locals.var_xp_dn0 = assign22480_e31500_d_n0;
        locals.var_xp_dn2 = assign22480_e31500_d_n2;
        locals.var_xp_dn6 = assign22480_e31500_d_n6;
        locals.var_xp_dn7 = assign22480_e31500_d_n7;
        locals.var_xp_dn10 = assign22480_e31500_d_n10;
        locals.var_xp_dn11 = assign22480_e31500_d_n11;
        locals.var_xp_dn12 = assign22480_e31500_d_n12;
        locals.var_xp_dn17 = assign22480_e31500_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22490_e31517, assign22490_e31517_d_n0, assign22490_e31517_d_n2, assign22490_e31517_d_n6, assign22490_e31517_d_n7, assign22490_e31517_d_n10, assign22490_e31517_d_n11, assign22490_e31517_d_n12, assign22490_e31517_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22490_e31515: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22490_e31515, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22490_e31517;
        locals.var_xmp_dn0 = assign22490_e31517_d_n0;
        locals.var_xmp_dn2 = assign22490_e31517_d_n2;
        locals.var_xmp_dn6 = assign22490_e31517_d_n6;
        locals.var_xmp_dn7 = assign22490_e31517_d_n7;
        locals.var_xmp_dn10 = assign22490_e31517_d_n10;
        locals.var_xmp_dn11 = assign22490_e31517_d_n11;
        locals.var_xmp_dn12 = assign22490_e31517_d_n12;
        locals.var_xmp_dn17 = assign22490_e31517_d_n17;
        locals.var_xmp_rv = 0.0;

    }
}
