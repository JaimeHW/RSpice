#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_164(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82480_e123939, assign82480_e123939_d_n5, assign82480_e123939_d_n6, assign82480_e123939_d_n7, assign82480_e123939_d_n8, assign82480_e123939_d_n12, assign82480_e123939_d_n13, assign82480_e123939_d_n14, assign82480_e123939_d_n15, assign82480_e123939_d_n16, assign82480_e123939_d_n17, assign82480_e123939_d_n18, assign82480_e123939_d_n19, assign82480_e123939_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 == 0.0)) && (locals.var_guard2241 == 0.0)) {
        let assign82480_e123915: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123917: f64 = (assign82480_e123915 - 230.25850929940458);
        let assign82480_e123921: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123923: f64 = (assign82480_e123921 - 230.25850929940458);
        let assign82480_e123926: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123928: f64 = (assign82480_e123926 - 230.25850929940458);
        let assign82480_e123930: f64 = (assign82480_e123928 * 0.3333333333333333);
        let assign82480_e123931: f64 = (1.0 + assign82480_e123930);
        let assign82480_e123932: f64 = (assign82480_e123923 * assign82480_e123931);
        let assign82480_e123933: f64 = (0.5 * assign82480_e123932);
        let assign82480_e123934: f64 = (1.0 + assign82480_e123933);
        let assign82480_e123935: f64 = (assign82480_e123917 * assign82480_e123934);
        let assign82480_e123936: f64 = (1.0 + assign82480_e123935);
        let assign82480_e123937: f64 = (1e100 * assign82480_e123936);
        (assign82480_e123937, (1e100 * (((-locals.var_nqs_xbar_dn5) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn5) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn6) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn6) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn7) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn7) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn8) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn8) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn12) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn12) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn13) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn13) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn14) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn14) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn15) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn15) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn16) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn16) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn17) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn17) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn18) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn18) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn19) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn19) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn20) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn20) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82480_e123939;
        locals.var_nqs_temp_dn5 = assign82480_e123939_d_n5;
        locals.var_nqs_temp_dn6 = assign82480_e123939_d_n6;
        locals.var_nqs_temp_dn7 = assign82480_e123939_d_n7;
        locals.var_nqs_temp_dn8 = assign82480_e123939_d_n8;
        locals.var_nqs_temp_dn12 = assign82480_e123939_d_n12;
        locals.var_nqs_temp_dn13 = assign82480_e123939_d_n13;
        locals.var_nqs_temp_dn14 = assign82480_e123939_d_n14;
        locals.var_nqs_temp_dn15 = assign82480_e123939_d_n15;
        locals.var_nqs_temp_dn16 = assign82480_e123939_d_n16;
        locals.var_nqs_temp_dn17 = assign82480_e123939_d_n17;
        locals.var_nqs_temp_dn18 = assign82480_e123939_d_n18;
        locals.var_nqs_temp_dn19 = assign82480_e123939_d_n19;
        locals.var_nqs_temp_dn20 = assign82480_e123939_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82490_e123965, assign82490_e123965_d_n5, assign82490_e123965_d_n6, assign82490_e123965_d_n7, assign82490_e123965_d_n8, assign82490_e123965_d_n12, assign82490_e123965_d_n13, assign82490_e123965_d_n14, assign82490_e123965_d_n15, assign82490_e123965_d_n16, assign82490_e123965_d_n17, assign82490_e123965_d_n18, assign82490_e123965_d_n19, assign82490_e123965_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82490_e123963: f64 = (1.0 - locals.var_nqs_temp);
        (assign82490_e123963, (-locals.var_nqs_temp_dn5), (-locals.var_nqs_temp_dn6), (-locals.var_nqs_temp_dn7), (-locals.var_nqs_temp_dn8), (-locals.var_nqs_temp_dn12), (-locals.var_nqs_temp_dn13), (-locals.var_nqs_temp_dn14), (-locals.var_nqs_temp_dn15), (-locals.var_nqs_temp_dn16), (-locals.var_nqs_temp_dn17), (-locals.var_nqs_temp_dn18), (-locals.var_nqs_temp_dn19), (-locals.var_nqs_temp_dn20),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82490_e123965;
        locals.var_nqs_w_dn5 = assign82490_e123965_d_n5;
        locals.var_nqs_w_dn6 = assign82490_e123965_d_n6;
        locals.var_nqs_w_dn7 = assign82490_e123965_d_n7;
        locals.var_nqs_w_dn8 = assign82490_e123965_d_n8;
        locals.var_nqs_w_dn12 = assign82490_e123965_d_n12;
        locals.var_nqs_w_dn13 = assign82490_e123965_d_n13;
        locals.var_nqs_w_dn14 = assign82490_e123965_d_n14;
        locals.var_nqs_w_dn15 = assign82490_e123965_d_n15;
        locals.var_nqs_w_dn16 = assign82490_e123965_d_n16;
        locals.var_nqs_w_dn17 = assign82490_e123965_d_n17;
        locals.var_nqs_w_dn18 = assign82490_e123965_d_n18;
        locals.var_nqs_w_dn19 = assign82490_e123965_d_n19;
        locals.var_nqs_w_dn20 = assign82490_e123965_d_n20;
        locals.var_nqs_w_rv = 0.0;

        let (assign82500_e124004, assign82500_e124004_d_n5, assign82500_e124004_d_n6, assign82500_e124004_d_n7, assign82500_e124004_d_n8, assign82500_e124004_d_n12, assign82500_e124004_d_n13, assign82500_e124004_d_n14, assign82500_e124004_d_n15, assign82500_e124004_d_n16, assign82500_e124004_d_n17, assign82500_e124004_d_n18, assign82500_e124004_d_n19, assign82500_e124004_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82500_e123990: f64 = (locals.var_gp2 * 0.5);
        let assign82500_e123991: f64 = (locals.var_temp__blk1038 + assign82500_e123990);
        let assign82500_e123996: f64 = (locals.var_gp2 * 0.25);
        let assign82500_e123997: f64 = (locals.var_temp__blk1038 + assign82500_e123996);
        let assign82500_e123999: f64 = (assign82500_e123997 - locals.var_nqs_w);
        let assign82500_e124000: f64 = (assign82500_e123999).sqrt();
        let assign82500_e124001: f64 = (locals.var_gp * assign82500_e124000);
        let assign82500_e124002: f64 = (assign82500_e123991 - assign82500_e124001);
        (assign82500_e124002, ((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.5)) - ((locals.var_gp_dn5 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.25)) - locals.var_nqs_w_dn5) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.5)) - ((locals.var_gp_dn6 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.25)) - locals.var_nqs_w_dn6) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.5)) - ((locals.var_gp_dn7 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.25)) - locals.var_nqs_w_dn7) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.5)) - ((locals.var_gp_dn8 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.25)) - locals.var_nqs_w_dn8) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.5)) - ((locals.var_gp_dn12 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.25)) - locals.var_nqs_w_dn12) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.5)) - ((locals.var_gp_dn13 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.25)) - locals.var_nqs_w_dn13) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.5)) - ((locals.var_gp_dn14 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.25)) - locals.var_nqs_w_dn14) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.5)) - ((locals.var_gp_dn15 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.25)) - locals.var_nqs_w_dn15) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.5)) - ((locals.var_gp_dn16 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.25)) - locals.var_nqs_w_dn16) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.5)) - ((locals.var_gp_dn17 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.25)) - locals.var_nqs_w_dn17) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.5)) - ((locals.var_gp_dn18 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.25)) - locals.var_nqs_w_dn18) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.5)) - ((locals.var_gp_dn19 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.25)) - locals.var_nqs_w_dn19) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.5)) - ((locals.var_gp_dn20 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.25)) - locals.var_nqs_w_dn20) / (2.0 * assign82500_e124000))))),)
    } else {
        (locals.var_nqs_x0, locals.var_nqs_x0_dn5, locals.var_nqs_x0_dn6, locals.var_nqs_x0_dn7, locals.var_nqs_x0_dn8, locals.var_nqs_x0_dn12, locals.var_nqs_x0_dn13, locals.var_nqs_x0_dn14, locals.var_nqs_x0_dn15, locals.var_nqs_x0_dn16, locals.var_nqs_x0_dn17, locals.var_nqs_x0_dn18, locals.var_nqs_x0_dn19, locals.var_nqs_x0_dn20,)
    }
};
        locals.var_nqs_x0 = assign82500_e124004;
        locals.var_nqs_x0_dn5 = assign82500_e124004_d_n5;
        locals.var_nqs_x0_dn6 = assign82500_e124004_d_n6;
        locals.var_nqs_x0_dn7 = assign82500_e124004_d_n7;
        locals.var_nqs_x0_dn8 = assign82500_e124004_d_n8;
        locals.var_nqs_x0_dn12 = assign82500_e124004_d_n12;
        locals.var_nqs_x0_dn13 = assign82500_e124004_d_n13;
        locals.var_nqs_x0_dn14 = assign82500_e124004_d_n14;
        locals.var_nqs_x0_dn15 = assign82500_e124004_d_n15;
        locals.var_nqs_x0_dn16 = assign82500_e124004_d_n16;
        locals.var_nqs_x0_dn17 = assign82500_e124004_d_n17;
        locals.var_nqs_x0_dn18 = assign82500_e124004_d_n18;
        locals.var_nqs_x0_dn19 = assign82500_e124004_d_n19;
        locals.var_nqs_x0_dn20 = assign82500_e124004_d_n20;
        locals.var_nqs_x0_rv = 0.0;

        let assign82510_e124006: f64 = (-locals.var_nqs_x0);
        let assign82510_e124007: f64 = (assign82510_e124006).abs();
        let assign82510_e124009: f64 = if assign82510_e124007 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign82510_e124009;
        locals.var_guard2242_rv = 0.0;

        let (assign82520_e124037, assign82520_e124037_d_n5, assign82520_e124037_d_n6, assign82520_e124037_d_n7, assign82520_e124037_d_n8, assign82520_e124037_d_n12, assign82520_e124037_d_n13, assign82520_e124037_d_n14, assign82520_e124037_d_n15, assign82520_e124037_d_n16, assign82520_e124037_d_n17, assign82520_e124037_d_n18, assign82520_e124037_d_n19, assign82520_e124037_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign82520_e124034: f64 = (-locals.var_nqs_x0);
        let assign82520_e124035: f64 = (assign82520_e124034).exp();
        (assign82520_e124035, (assign82520_e124035 * (-locals.var_nqs_x0_dn5)), (assign82520_e124035 * (-locals.var_nqs_x0_dn6)), (assign82520_e124035 * (-locals.var_nqs_x0_dn7)), (assign82520_e124035 * (-locals.var_nqs_x0_dn8)), (assign82520_e124035 * (-locals.var_nqs_x0_dn12)), (assign82520_e124035 * (-locals.var_nqs_x0_dn13)), (assign82520_e124035 * (-locals.var_nqs_x0_dn14)), (assign82520_e124035 * (-locals.var_nqs_x0_dn15)), (assign82520_e124035 * (-locals.var_nqs_x0_dn16)), (assign82520_e124035 * (-locals.var_nqs_x0_dn17)), (assign82520_e124035 * (-locals.var_nqs_x0_dn18)), (assign82520_e124035 * (-locals.var_nqs_x0_dn19)), (assign82520_e124035 * (-locals.var_nqs_x0_dn20)),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82520_e124037;
        locals.var_nqs_d0_dn5 = assign82520_e124037_d_n5;
        locals.var_nqs_d0_dn6 = assign82520_e124037_d_n6;
        locals.var_nqs_d0_dn7 = assign82520_e124037_d_n7;
        locals.var_nqs_d0_dn8 = assign82520_e124037_d_n8;
        locals.var_nqs_d0_dn12 = assign82520_e124037_d_n12;
        locals.var_nqs_d0_dn13 = assign82520_e124037_d_n13;
        locals.var_nqs_d0_dn14 = assign82520_e124037_d_n14;
        locals.var_nqs_d0_dn15 = assign82520_e124037_d_n15;
        locals.var_nqs_d0_dn16 = assign82520_e124037_d_n16;
        locals.var_nqs_d0_dn17 = assign82520_e124037_d_n17;
        locals.var_nqs_d0_dn18 = assign82520_e124037_d_n18;
        locals.var_nqs_d0_dn19 = assign82520_e124037_d_n19;
        locals.var_nqs_d0_dn20 = assign82520_e124037_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let assign82530_e124039: f64 = (-locals.var_nqs_x0);
        let assign82530_e124041: f64 = if assign82530_e124039 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign82530_e124041;
        locals.var_guard2243_rv = 0.0;

        let (assign82540_e124098, assign82540_e124098_d_n5, assign82540_e124098_d_n6, assign82540_e124098_d_n7, assign82540_e124098_d_n8, assign82540_e124098_d_n12, assign82540_e124098_d_n13, assign82540_e124098_d_n14, assign82540_e124098_d_n15, assign82540_e124098_d_n16, assign82540_e124098_d_n17, assign82540_e124098_d_n18, assign82540_e124098_d_n19, assign82540_e124098_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 == 0.0)) && (locals.var_guard2243 != 0.0)) {
        let assign82540_e124071: f64 = (-230.25850929940458);
        let assign82540_e124073: f64 = (-locals.var_nqs_x0);
        let assign82540_e124074: f64 = (assign82540_e124071 - assign82540_e124073);
        let assign82540_e124078: f64 = (-230.25850929940458);
        let assign82540_e124080: f64 = (-locals.var_nqs_x0);
        let assign82540_e124081: f64 = (assign82540_e124078 - assign82540_e124080);
        let assign82540_e124084: f64 = (-230.25850929940458);
        let assign82540_e124086: f64 = (-locals.var_nqs_x0);
        let assign82540_e124087: f64 = (assign82540_e124084 - assign82540_e124086);
        let assign82540_e124089: f64 = (assign82540_e124087 * 0.3333333333333333);
        let assign82540_e124090: f64 = (1.0 + assign82540_e124089);
        let assign82540_e124091: f64 = (assign82540_e124081 * assign82540_e124090);
        let assign82540_e124092: f64 = (0.5 * assign82540_e124091);
        let assign82540_e124093: f64 = (1.0 + assign82540_e124092);
        let assign82540_e124094: f64 = (assign82540_e124074 * assign82540_e124093);
        let assign82540_e124095: f64 = (1.0 + assign82540_e124094);
        let assign82540_e124096: f64 = (1e-100 / assign82540_e124095);
        (assign82540_e124096, (-((1e-100 * (((-(-locals.var_nqs_x0_dn5)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn5)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn5)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn6)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn6)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn6)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn7)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn7)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn7)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn8)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn8)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn8)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn12)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn12)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn12)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn13)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn13)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn13)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn14)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn14)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn14)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn15)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn15)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn15)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn16)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn16)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn16)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn17)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn17)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn17)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn18)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn18)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn18)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn19)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn19)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn19)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn20)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn20)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn20)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82540_e124098;
        locals.var_nqs_d0_dn5 = assign82540_e124098_d_n5;
        locals.var_nqs_d0_dn6 = assign82540_e124098_d_n6;
        locals.var_nqs_d0_dn7 = assign82540_e124098_d_n7;
        locals.var_nqs_d0_dn8 = assign82540_e124098_d_n8;
        locals.var_nqs_d0_dn12 = assign82540_e124098_d_n12;
        locals.var_nqs_d0_dn13 = assign82540_e124098_d_n13;
        locals.var_nqs_d0_dn14 = assign82540_e124098_d_n14;
        locals.var_nqs_d0_dn15 = assign82540_e124098_d_n15;
        locals.var_nqs_d0_dn16 = assign82540_e124098_d_n16;
        locals.var_nqs_d0_dn17 = assign82540_e124098_d_n17;
        locals.var_nqs_d0_dn18 = assign82540_e124098_d_n18;
        locals.var_nqs_d0_dn19 = assign82540_e124098_d_n19;
        locals.var_nqs_d0_dn20 = assign82540_e124098_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82550_e124153, assign82550_e124153_d_n5, assign82550_e124153_d_n6, assign82550_e124153_d_n7, assign82550_e124153_d_n8, assign82550_e124153_d_n12, assign82550_e124153_d_n13, assign82550_e124153_d_n14, assign82550_e124153_d_n15, assign82550_e124153_d_n16, assign82550_e124153_d_n17, assign82550_e124153_d_n18, assign82550_e124153_d_n19, assign82550_e124153_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 == 0.0)) && (locals.var_guard2243 == 0.0)) {
        let assign82550_e124129: f64 = (-locals.var_nqs_x0);
        let assign82550_e124131: f64 = (assign82550_e124129 - 230.25850929940458);
        let assign82550_e124135: f64 = (-locals.var_nqs_x0);
        let assign82550_e124137: f64 = (assign82550_e124135 - 230.25850929940458);
        let assign82550_e124140: f64 = (-locals.var_nqs_x0);
        let assign82550_e124142: f64 = (assign82550_e124140 - 230.25850929940458);
        let assign82550_e124144: f64 = (assign82550_e124142 * 0.3333333333333333);
        let assign82550_e124145: f64 = (1.0 + assign82550_e124144);
        let assign82550_e124146: f64 = (assign82550_e124137 * assign82550_e124145);
        let assign82550_e124147: f64 = (0.5 * assign82550_e124146);
        let assign82550_e124148: f64 = (1.0 + assign82550_e124147);
        let assign82550_e124149: f64 = (assign82550_e124131 * assign82550_e124148);
        let assign82550_e124150: f64 = (1.0 + assign82550_e124149);
        let assign82550_e124151: f64 = (1e100 * assign82550_e124150);
        (assign82550_e124151, (1e100 * (((-locals.var_nqs_x0_dn5) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn5) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn6) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn6) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn7) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn7) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn8) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn8) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn12) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn12) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn13) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn13) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn14) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn14) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn15) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn15) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn16) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn16) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn17) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn17) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn18) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn18) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn19) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn19) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn20) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn20) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82550_e124153;
        locals.var_nqs_d0_dn5 = assign82550_e124153_d_n5;
        locals.var_nqs_d0_dn6 = assign82550_e124153_d_n6;
        locals.var_nqs_d0_dn7 = assign82550_e124153_d_n7;
        locals.var_nqs_d0_dn8 = assign82550_e124153_d_n8;
        locals.var_nqs_d0_dn12 = assign82550_e124153_d_n12;
        locals.var_nqs_d0_dn13 = assign82550_e124153_d_n13;
        locals.var_nqs_d0_dn14 = assign82550_e124153_d_n14;
        locals.var_nqs_d0_dn15 = assign82550_e124153_d_n15;
        locals.var_nqs_d0_dn16 = assign82550_e124153_d_n16;
        locals.var_nqs_d0_dn17 = assign82550_e124153_d_n17;
        locals.var_nqs_d0_dn18 = assign82550_e124153_d_n18;
        locals.var_nqs_d0_dn19 = assign82550_e124153_d_n19;
        locals.var_nqs_d0_dn20 = assign82550_e124153_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82560_e124183, assign82560_e124183_d_n5, assign82560_e124183_d_n6, assign82560_e124183_d_n7, assign82560_e124183_d_n8, assign82560_e124183_d_n12, assign82560_e124183_d_n13, assign82560_e124183_d_n14, assign82560_e124183_d_n15, assign82560_e124183_d_n16, assign82560_e124183_d_n17, assign82560_e124183_d_n18, assign82560_e124183_d_n19, assign82560_e124183_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82560_e124178: f64 = (locals.var_gp2 * 0.5);
        let assign82560_e124180: f64 = (assign82560_e124178 * locals.var_nqs_d0);
        let assign82560_e124181: f64 = (1.0 - assign82560_e124180);
        (assign82560_e124181, (-(((locals.var_gp2_dn5 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn5))), (-(((locals.var_gp2_dn6 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn6))), (-(((locals.var_gp2_dn7 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn7))), (-(((locals.var_gp2_dn8 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn8))), (-(((locals.var_gp2_dn12 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn12))), (-(((locals.var_gp2_dn13 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn13))), (-(((locals.var_gp2_dn14 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn14))), (-(((locals.var_gp2_dn15 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn15))), (-(((locals.var_gp2_dn16 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn16))), (-(((locals.var_gp2_dn17 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn17))), (-(((locals.var_gp2_dn18 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn18))), (-(((locals.var_gp2_dn19 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn19))), (-(((locals.var_gp2_dn20 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82560_e124183;
        locals.var_nqs_xi_dn5 = assign82560_e124183_d_n5;
        locals.var_nqs_xi_dn6 = assign82560_e124183_d_n6;
        locals.var_nqs_xi_dn7 = assign82560_e124183_d_n7;
        locals.var_nqs_xi_dn8 = assign82560_e124183_d_n8;
        locals.var_nqs_xi_dn12 = assign82560_e124183_d_n12;
        locals.var_nqs_xi_dn13 = assign82560_e124183_d_n13;
        locals.var_nqs_xi_dn14 = assign82560_e124183_d_n14;
        locals.var_nqs_xi_dn15 = assign82560_e124183_d_n15;
        locals.var_nqs_xi_dn16 = assign82560_e124183_d_n16;
        locals.var_nqs_xi_dn17 = assign82560_e124183_d_n17;
        locals.var_nqs_xi_dn18 = assign82560_e124183_d_n18;
        locals.var_nqs_xi_dn19 = assign82560_e124183_d_n19;
        locals.var_nqs_xi_dn20 = assign82560_e124183_d_n20;
        locals.var_nqs_xi_rv = 0.0;

        let (assign82570_e124217, assign82570_e124217_d_n5, assign82570_e124217_d_n6, assign82570_e124217_d_n7, assign82570_e124217_d_n8, assign82570_e124217_d_n12, assign82570_e124217_d_n13, assign82570_e124217_d_n14, assign82570_e124217_d_n15, assign82570_e124217_d_n16, assign82570_e124217_d_n17, assign82570_e124217_d_n18, assign82570_e124217_d_n19, assign82570_e124217_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82570_e124208: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82570_e124209: f64 = (2.0 * assign82570_e124208);
        let assign82570_e124213: f64 = (1.0 - locals.var_nqs_d0);
        let assign82570_e124214: f64 = (locals.var_gp2 * assign82570_e124213);
        let assign82570_e124215: f64 = (assign82570_e124209 + assign82570_e124214);
        (assign82570_e124215, ((2.0 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5)) + ((locals.var_gp2_dn5 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn5)))), ((2.0 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6)) + ((locals.var_gp2_dn6 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn6)))), ((2.0 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7)) + ((locals.var_gp2_dn7 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn7)))), ((2.0 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8)) + ((locals.var_gp2_dn8 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn8)))), ((2.0 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12)) + ((locals.var_gp2_dn12 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn12)))), ((2.0 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13)) + ((locals.var_gp2_dn13 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn13)))), ((2.0 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14)) + ((locals.var_gp2_dn14 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn14)))), ((2.0 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15)) + ((locals.var_gp2_dn15 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn15)))), ((2.0 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16)) + ((locals.var_gp2_dn16 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn16)))), ((2.0 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17)) + ((locals.var_gp2_dn17 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn17)))), ((2.0 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18)) + ((locals.var_gp2_dn18 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn18)))), ((2.0 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19)) + ((locals.var_gp2_dn19 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn19)))), ((2.0 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20)) + ((locals.var_gp2_dn20 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82570_e124217;
        locals.var_nqs_p_dn5 = assign82570_e124217_d_n5;
        locals.var_nqs_p_dn6 = assign82570_e124217_d_n6;
        locals.var_nqs_p_dn7 = assign82570_e124217_d_n7;
        locals.var_nqs_p_dn8 = assign82570_e124217_d_n8;
        locals.var_nqs_p_dn12 = assign82570_e124217_d_n12;
        locals.var_nqs_p_dn13 = assign82570_e124217_d_n13;
        locals.var_nqs_p_dn14 = assign82570_e124217_d_n14;
        locals.var_nqs_p_dn15 = assign82570_e124217_d_n15;
        locals.var_nqs_p_dn16 = assign82570_e124217_d_n16;
        locals.var_nqs_p_dn17 = assign82570_e124217_d_n17;
        locals.var_nqs_p_dn18 = assign82570_e124217_d_n18;
        locals.var_nqs_p_dn19 = assign82570_e124217_d_n19;
        locals.var_nqs_p_dn20 = assign82570_e124217_d_n20;
        locals.var_nqs_p_rv = 0.0;

        let (assign82580_e124255, assign82580_e124255_d_n5, assign82580_e124255_d_n6, assign82580_e124255_d_n7, assign82580_e124255_d_n8, assign82580_e124255_d_n12, assign82580_e124255_d_n13, assign82580_e124255_d_n14, assign82580_e124255_d_n15, assign82580_e124255_d_n16, assign82580_e124255_d_n17, assign82580_e124255_d_n18, assign82580_e124255_d_n19, assign82580_e124255_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82580_e124241: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82580_e124244: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82580_e124245: f64 = (assign82580_e124241 * assign82580_e124244);
        let assign82580_e124249: f64 = (locals.var_nqs_x0 - 1.0);
        let assign82580_e124251: f64 = (assign82580_e124249 + locals.var_nqs_d0);
        let assign82580_e124252: f64 = (locals.var_gp2 * assign82580_e124251);
        let assign82580_e124253: f64 = (assign82580_e124245 - assign82580_e124252);
        (assign82580_e124253, ((((locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5))) - ((locals.var_gp2_dn5 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn5 + locals.var_nqs_d0_dn5)))), ((((locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6))) - ((locals.var_gp2_dn6 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn6 + locals.var_nqs_d0_dn6)))), ((((locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7))) - ((locals.var_gp2_dn7 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn7 + locals.var_nqs_d0_dn7)))), ((((locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8))) - ((locals.var_gp2_dn8 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn8 + locals.var_nqs_d0_dn8)))), ((((locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12))) - ((locals.var_gp2_dn12 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn12 + locals.var_nqs_d0_dn12)))), ((((locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13))) - ((locals.var_gp2_dn13 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn13 + locals.var_nqs_d0_dn13)))), ((((locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14))) - ((locals.var_gp2_dn14 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn14 + locals.var_nqs_d0_dn14)))), ((((locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15))) - ((locals.var_gp2_dn15 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn15 + locals.var_nqs_d0_dn15)))), ((((locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16))) - ((locals.var_gp2_dn16 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn16 + locals.var_nqs_d0_dn16)))), ((((locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17))) - ((locals.var_gp2_dn17 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn17 + locals.var_nqs_d0_dn17)))), ((((locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18))) - ((locals.var_gp2_dn18 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn18 + locals.var_nqs_d0_dn18)))), ((((locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19))) - ((locals.var_gp2_dn19 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn19 + locals.var_nqs_d0_dn19)))), ((((locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20))) - ((locals.var_gp2_dn20 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn20 + locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82580_e124255;
        locals.var_nqs_q_dn5 = assign82580_e124255_d_n5;
        locals.var_nqs_q_dn6 = assign82580_e124255_d_n6;
        locals.var_nqs_q_dn7 = assign82580_e124255_d_n7;
        locals.var_nqs_q_dn8 = assign82580_e124255_d_n8;
        locals.var_nqs_q_dn12 = assign82580_e124255_d_n12;
        locals.var_nqs_q_dn13 = assign82580_e124255_d_n13;
        locals.var_nqs_q_dn14 = assign82580_e124255_d_n14;
        locals.var_nqs_q_dn15 = assign82580_e124255_d_n15;
        locals.var_nqs_q_dn16 = assign82580_e124255_d_n16;
        locals.var_nqs_q_dn17 = assign82580_e124255_d_n17;
        locals.var_nqs_q_dn18 = assign82580_e124255_d_n18;
        locals.var_nqs_q_dn19 = assign82580_e124255_d_n19;
        locals.var_nqs_q_dn20 = assign82580_e124255_d_n20;
        locals.var_nqs_q_rv = 0.0;

        let (assign82590_e124287, assign82590_e124287_d_n5, assign82590_e124287_d_n6, assign82590_e124287_d_n7, assign82590_e124287_d_n8, assign82590_e124287_d_n12, assign82590_e124287_d_n13, assign82590_e124287_d_n14, assign82590_e124287_d_n15, assign82590_e124287_d_n16, assign82590_e124287_d_n17, assign82590_e124287_d_n18, assign82590_e124287_d_n19, assign82590_e124287_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82590_e124279: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82590_e124282: f64 = (4.0 * locals.var_nqs_xi);
        let assign82590_e124284: f64 = (assign82590_e124282 * locals.var_nqs_q);
        let assign82590_e124285: f64 = (assign82590_e124279 - assign82590_e124284);
        (assign82590_e124285, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82590_e124287;
        locals.var_nqs_temp_dn5 = assign82590_e124287_d_n5;
        locals.var_nqs_temp_dn6 = assign82590_e124287_d_n6;
        locals.var_nqs_temp_dn7 = assign82590_e124287_d_n7;
        locals.var_nqs_temp_dn8 = assign82590_e124287_d_n8;
        locals.var_nqs_temp_dn12 = assign82590_e124287_d_n12;
        locals.var_nqs_temp_dn13 = assign82590_e124287_d_n13;
        locals.var_nqs_temp_dn14 = assign82590_e124287_d_n14;
        locals.var_nqs_temp_dn15 = assign82590_e124287_d_n15;
        locals.var_nqs_temp_dn16 = assign82590_e124287_d_n16;
        locals.var_nqs_temp_dn17 = assign82590_e124287_d_n17;
        locals.var_nqs_temp_dn18 = assign82590_e124287_d_n18;
        locals.var_nqs_temp_dn19 = assign82590_e124287_d_n19;
        locals.var_nqs_temp_dn20 = assign82590_e124287_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82600_e124318, assign82600_e124318_d_n5, assign82600_e124318_d_n6, assign82600_e124318_d_n7, assign82600_e124318_d_n8, assign82600_e124318_d_n12, assign82600_e124318_d_n13, assign82600_e124318_d_n14, assign82600_e124318_d_n15, assign82600_e124318_d_n16, assign82600_e124318_d_n17, assign82600_e124318_d_n18, assign82600_e124318_d_n19, assign82600_e124318_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82600_e124311: f64 = (2.0 * locals.var_nqs_q);
        let assign82600_e124314: f64 = (locals.var_nqs_temp).sqrt();
        let assign82600_e124315: f64 = (locals.var_nqs_p + assign82600_e124314);
        let assign82600_e124316: f64 = (assign82600_e124311 / assign82600_e124315);
        (assign82600_e124316, ((((2.0 * locals.var_nqs_q_dn5) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn6) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn7) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn8) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn12) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn13) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn14) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn15) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn16) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn17) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn18) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn19) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn20) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)),)
    } else {
        (locals.var_nqs_u, locals.var_nqs_u_dn5, locals.var_nqs_u_dn6, locals.var_nqs_u_dn7, locals.var_nqs_u_dn8, locals.var_nqs_u_dn12, locals.var_nqs_u_dn13, locals.var_nqs_u_dn14, locals.var_nqs_u_dn15, locals.var_nqs_u_dn16, locals.var_nqs_u_dn17, locals.var_nqs_u_dn18, locals.var_nqs_u_dn19, locals.var_nqs_u_dn20,)
    }
};
        locals.var_nqs_u = assign82600_e124318;
        locals.var_nqs_u_dn5 = assign82600_e124318_d_n5;
        locals.var_nqs_u_dn6 = assign82600_e124318_d_n6;
        locals.var_nqs_u_dn7 = assign82600_e124318_d_n7;
        locals.var_nqs_u_dn8 = assign82600_e124318_d_n8;
        locals.var_nqs_u_dn12 = assign82600_e124318_d_n12;
        locals.var_nqs_u_dn13 = assign82600_e124318_d_n13;
        locals.var_nqs_u_dn14 = assign82600_e124318_d_n14;
        locals.var_nqs_u_dn15 = assign82600_e124318_d_n15;
        locals.var_nqs_u_dn16 = assign82600_e124318_d_n16;
        locals.var_nqs_u_dn17 = assign82600_e124318_d_n17;
        locals.var_nqs_u_dn18 = assign82600_e124318_d_n18;
        locals.var_nqs_u_dn19 = assign82600_e124318_d_n19;
        locals.var_nqs_u_dn20 = assign82600_e124318_d_n20;
        locals.var_nqs_u_rv = 0.0;

        let assign82860_e124516: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign82860_e124516;
        locals.var_cgeff_dn5 = ((locals.var_cox_qm_dn5 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn5));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));
        locals.var_cgeff_dn12 = ((locals.var_cox_qm_dn12 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn12));
        locals.var_cgeff_dn13 = ((locals.var_cox_qm_dn13 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn13));
        locals.var_cgeff_dn14 = ((locals.var_cox_qm_dn14 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn14));
        locals.var_cgeff_dn15 = ((locals.var_cox_qm_dn15 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn15));
        locals.var_cgeff_dn16 = ((locals.var_cox_qm_dn16 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn16));
        locals.var_cgeff_dn17 = ((locals.var_cox_qm_dn17 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn17));
        locals.var_cgeff_dn18 = ((locals.var_cox_qm_dn18 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn18));
        locals.var_cgeff_dn19 = ((locals.var_cox_qm_dn19 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn19));
        locals.var_cgeff_dn20 = ((locals.var_cox_qm_dn20 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn20));
        locals.var_cgeff_rv = 0.0;

        let assign82920_e124528: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign82920_e124528;
        locals.var_guard2279_rv = 0.0;

        let assign83230_e124894: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign83230_e124894;
        locals.var_guard2284_rv = 0.0;

        let (assign83280_e124991, assign83280_e124991_d_n5, assign83280_e124991_d_n6, assign83280_e124991_d_n7, assign83280_e124991_d_n8, assign83280_e124991_d_n12, assign83280_e124991_d_n13, assign83280_e124991_d_n14, assign83280_e124991_d_n15, assign83280_e124991_d_n16, assign83280_e124991_d_n17, assign83280_e124991_d_n18, assign83280_e124991_d_n19, assign83280_e124991_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83280_e124981: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign83280_e124983: f64 = (assign83280_e124981 * locals.var_cox_qm);
        let assign83280_e124985: f64 = (assign83280_e124983 * locals.var_eta_p_ac);
        let assign83280_e124988: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign83280_e124989: f64 = (assign83280_e124985 / assign83280_e124988);
        (assign83280_e124989, (((((((((locals.var_gvsat_ac_dn5 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn5)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn5)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn5)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn5 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn5)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn6)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn7)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn8)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn12 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn12)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn12)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn12)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn12 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn12)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn13 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn13)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn13)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn13)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn13 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn13)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn14 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn14)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn14)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn14)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn14 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn14)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn15 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn15)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn15)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn15)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn15 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn15)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn16 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn16)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn16)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn16)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn16 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn16)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn17 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn17)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn17)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn17)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn17 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn17)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn18 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn18)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn18)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn18)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn18 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn18)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn19 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn19)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn19)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn19)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn19 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn19)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn20 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn20)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn20)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn20)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn20 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn20)))) / (assign83280_e124988 * assign83280_e124988)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn12, locals.var_cgeff_dn13, locals.var_cgeff_dn14, locals.var_cgeff_dn15, locals.var_cgeff_dn16, locals.var_cgeff_dn17, locals.var_cgeff_dn18, locals.var_cgeff_dn19, locals.var_cgeff_dn20,)
    }
};
        locals.var_cgeff = assign83280_e124991;
        locals.var_cgeff_dn5 = assign83280_e124991_d_n5;
        locals.var_cgeff_dn6 = assign83280_e124991_d_n6;
        locals.var_cgeff_dn7 = assign83280_e124991_d_n7;
        locals.var_cgeff_dn8 = assign83280_e124991_d_n8;
        locals.var_cgeff_dn12 = assign83280_e124991_d_n12;
        locals.var_cgeff_dn13 = assign83280_e124991_d_n13;
        locals.var_cgeff_dn14 = assign83280_e124991_d_n14;
        locals.var_cgeff_dn15 = assign83280_e124991_d_n15;
        locals.var_cgeff_dn16 = assign83280_e124991_d_n16;
        locals.var_cgeff_dn17 = assign83280_e124991_d_n17;
        locals.var_cgeff_dn18 = assign83280_e124991_d_n18;
        locals.var_cgeff_dn19 = assign83280_e124991_d_n19;
        locals.var_cgeff_dn20 = assign83280_e124991_d_n20;
        locals.var_cgeff_rv = 0.0;

        let assign83540_e125205: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign83540_e125205;
        locals.var_guard2288_rv = 0.0;

        let (assign83550_e125213, assign83550_e125213_d_n5, assign83550_e125213_d_n6, assign83550_e125213_d_n7, assign83550_e125213_d_n8, assign83550_e125213_d_n12, assign83550_e125213_d_n13, assign83550_e125213_d_n14, assign83550_e125213_d_n15, assign83550_e125213_d_n16, assign83550_e125213_d_n17, assign83550_e125213_d_n18, assign83550_e125213_d_n19, assign83550_e125213_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83550_e125209: f64 = (4.0 * locals.var_dsqredge);
        let assign83550_e125211: f64 = (assign83550_e125209 / locals.var_gfedge2);
        (assign83550_e125211, ((4.0 * locals.var_dsqredge_dn5) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn12) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn13) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn14) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn15) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn16) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn17) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn18) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn19) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn20) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83550_e125213;
        locals.var_temp1_dn5 = assign83550_e125213_d_n5;
        locals.var_temp1_dn6 = assign83550_e125213_d_n6;
        locals.var_temp1_dn7 = assign83550_e125213_d_n7;
        locals.var_temp1_dn8 = assign83550_e125213_d_n8;
        locals.var_temp1_dn12 = assign83550_e125213_d_n12;
        locals.var_temp1_dn13 = assign83550_e125213_d_n13;
        locals.var_temp1_dn14 = assign83550_e125213_d_n14;
        locals.var_temp1_dn15 = assign83550_e125213_d_n15;
        locals.var_temp1_dn16 = assign83550_e125213_d_n16;
        locals.var_temp1_dn17 = assign83550_e125213_d_n17;
        locals.var_temp1_dn18 = assign83550_e125213_d_n18;
        locals.var_temp1_dn19 = assign83550_e125213_d_n19;
        locals.var_temp1_dn20 = assign83550_e125213_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign83570_e125233, assign83570_e125233_d_n5, assign83570_e125233_d_n6, assign83570_e125233_d_n7, assign83570_e125233_d_n8, assign83570_e125233_d_n12, assign83570_e125233_d_n13, assign83570_e125233_d_n14, assign83570_e125233_d_n15, assign83570_e125233_d_n16, assign83570_e125233_d_n17, assign83570_e125233_d_n18, assign83570_e125233_d_n19, assign83570_e125233_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83570_e125231: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign83570_e125231, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83570_e125233;
        locals.var_temp1_dn5 = assign83570_e125233_d_n5;
        locals.var_temp1_dn6 = assign83570_e125233_d_n6;
        locals.var_temp1_dn7 = assign83570_e125233_d_n7;
        locals.var_temp1_dn8 = assign83570_e125233_d_n8;
        locals.var_temp1_dn12 = assign83570_e125233_d_n12;
        locals.var_temp1_dn13 = assign83570_e125233_d_n13;
        locals.var_temp1_dn14 = assign83570_e125233_d_n14;
        locals.var_temp1_dn15 = assign83570_e125233_d_n15;
        locals.var_temp1_dn16 = assign83570_e125233_d_n16;
        locals.var_temp1_dn17 = assign83570_e125233_d_n17;
        locals.var_temp1_dn18 = assign83570_e125233_d_n18;
        locals.var_temp1_dn19 = assign83570_e125233_d_n19;
        locals.var_temp1_dn20 = assign83570_e125233_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign83700_e125373, assign83700_e125373_d_n5, assign83700_e125373_d_n6, assign83700_e125373_d_n7, assign83700_e125373_d_n8, assign83700_e125373_d_n12, assign83700_e125373_d_n13, assign83700_e125373_d_n14, assign83700_e125373_d_n15, assign83700_e125373_d_n16, assign83700_e125373_d_n17, assign83700_e125373_d_n18, assign83700_e125373_d_n19, assign83700_e125373_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83700_e125371: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign83700_e125371, ((locals.var_alpha_dc_dn5 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn5)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), ((locals.var_alpha_dc_dn12 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn12)), ((locals.var_alpha_dc_dn13 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn13)), ((locals.var_alpha_dc_dn14 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn14)), ((locals.var_alpha_dc_dn15 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn15)), ((locals.var_alpha_dc_dn16 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn16)), ((locals.var_alpha_dc_dn17 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn17)), ((locals.var_alpha_dc_dn18 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn18)), ((locals.var_alpha_dc_dn19 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn19)), ((locals.var_alpha_dc_dn20 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn20)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83700_e125373;
        locals.var_temp1_dn5 = assign83700_e125373_d_n5;
        locals.var_temp1_dn6 = assign83700_e125373_d_n6;
        locals.var_temp1_dn7 = assign83700_e125373_d_n7;
        locals.var_temp1_dn8 = assign83700_e125373_d_n8;
        locals.var_temp1_dn12 = assign83700_e125373_d_n12;
        locals.var_temp1_dn13 = assign83700_e125373_d_n13;
        locals.var_temp1_dn14 = assign83700_e125373_d_n14;
        locals.var_temp1_dn15 = assign83700_e125373_d_n15;
        locals.var_temp1_dn16 = assign83700_e125373_d_n16;
        locals.var_temp1_dn17 = assign83700_e125373_d_n17;
        locals.var_temp1_dn18 = assign83700_e125373_d_n18;
        locals.var_temp1_dn19 = assign83700_e125373_d_n19;
        locals.var_temp1_dn20 = assign83700_e125373_d_n20;
        locals.var_temp1_rv = 0.0;

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
        idt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq39_e1275: f64 = (-locals.var_tnorm);
        let eq39_e1277: f64 = (eq39_e1275 * locals.var_fk1);
        let eq39_e1277_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn5));
        let eq39_e1277_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn6));
        let eq39_e1277_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn7));
        let eq39_e1277_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn8));
        let eq39_e1277_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn12));
        let eq39_e1277_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn13));
        let eq39_e1277_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn14));
        let eq39_e1277_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn15));
        let eq39_e1277_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn16));
        let eq39_e1277_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn17));
        let eq39_e1277_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn18));
        let eq39_e1277_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn19));
        let eq39_e1277_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn20));
        let eq39_e1279: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 0, eq39_e1277, locals.var_qp1_0);
        let eq39_e1280: f64 = (locals.var_vnorm_inv * eq39_e1279);
        let eq39_e1280_d_n5: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n5 * idt_scale));
        let eq39_e1280_d_n6: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n6 * idt_scale));
        let eq39_e1280_d_n7: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n7 * idt_scale));
        let eq39_e1280_d_n8: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n8 * idt_scale));
        let eq39_e1280_d_n12: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n12 * idt_scale));
        let eq39_e1280_d_n13: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n13 * idt_scale));
        let eq39_e1280_d_n14: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n14 * idt_scale));
        let eq39_e1280_d_n15: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n15 * idt_scale));
        let eq39_e1280_d_n16: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n16 * idt_scale));
        let eq39_e1280_d_n17: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n17 * idt_scale));
        let eq39_e1280_d_n18: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n18 * idt_scale));
        let eq39_e1280_d_n19: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n19 * idt_scale));
        let eq39_e1280_d_n20: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n20 * idt_scale));
        let eq39_value: f64 = eq39_e1280;
        let eq39_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq39_node_derivatives: [f64; 13] = [eq39_e1280_d_n5, eq39_e1280_d_n6, eq39_e1280_d_n7, eq39_e1280_d_n8, eq39_e1280_d_n12, eq39_e1280_d_n13, eq39_e1280_d_n14, eq39_e1280_d_n15, eq39_e1280_d_n16, eq39_e1280_d_n17, eq39_e1280_d_n18, eq39_e1280_d_n19, eq39_e1280_d_n20];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            8,
            eq39_value,
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
        );
        let eq41_e1288: f64 = (-locals.var_tnorm);
        let eq41_e1290: f64 = (eq41_e1288 * locals.var_fk2);
        let eq41_e1290_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn5));
        let eq41_e1290_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn6));
        let eq41_e1290_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn7));
        let eq41_e1290_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn8));
        let eq41_e1290_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn12));
        let eq41_e1290_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn13));
        let eq41_e1290_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn14));
        let eq41_e1290_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn15));
        let eq41_e1290_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn16));
        let eq41_e1290_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn17));
        let eq41_e1290_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn18));
        let eq41_e1290_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn19));
        let eq41_e1290_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn20));
        let eq41_e1292: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 1, eq41_e1290, locals.var_qp2_0);
        let eq41_e1293: f64 = (locals.var_vnorm_inv * eq41_e1292);
        let eq41_e1293_d_n5: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n5 * idt_scale));
        let eq41_e1293_d_n6: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n6 * idt_scale));
        let eq41_e1293_d_n7: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n7 * idt_scale));
        let eq41_e1293_d_n8: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n8 * idt_scale));
        let eq41_e1293_d_n12: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n12 * idt_scale));
        let eq41_e1293_d_n13: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n13 * idt_scale));
        let eq41_e1293_d_n14: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n14 * idt_scale));
        let eq41_e1293_d_n15: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n15 * idt_scale));
        let eq41_e1293_d_n16: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n16 * idt_scale));
        let eq41_e1293_d_n17: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n17 * idt_scale));
        let eq41_e1293_d_n18: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n18 * idt_scale));
        let eq41_e1293_d_n19: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n19 * idt_scale));
        let eq41_e1293_d_n20: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n20 * idt_scale));
        let eq41_value: f64 = eq41_e1293;
        let eq41_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq41_node_derivatives: [f64; 13] = [eq41_e1293_d_n5, eq41_e1293_d_n6, eq41_e1293_d_n7, eq41_e1293_d_n8, eq41_e1293_d_n12, eq41_e1293_d_n13, eq41_e1293_d_n14, eq41_e1293_d_n15, eq41_e1293_d_n16, eq41_e1293_d_n17, eq41_e1293_d_n18, eq41_e1293_d_n19, eq41_e1293_d_n20];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            10,
            eq41_value,
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
        );
        let eq43_e1301: f64 = (-locals.var_tnorm);
        let eq43_e1303: f64 = (eq43_e1301 * locals.var_fk3);
        let eq43_e1303_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn5));
        let eq43_e1303_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn6));
        let eq43_e1303_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn7));
        let eq43_e1303_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn8));
        let eq43_e1303_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn12));
        let eq43_e1303_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn13));
        let eq43_e1303_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn14));
        let eq43_e1303_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn15));
        let eq43_e1303_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn16));
        let eq43_e1303_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn17));
        let eq43_e1303_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn18));
        let eq43_e1303_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn19));
        let eq43_e1303_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn20));
        let eq43_e1305: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 2, eq43_e1303, locals.var_qp3_0);
        let eq43_e1306: f64 = (locals.var_vnorm_inv * eq43_e1305);
        let eq43_e1306_d_n5: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n5 * idt_scale));
        let eq43_e1306_d_n6: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n6 * idt_scale));
        let eq43_e1306_d_n7: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n7 * idt_scale));
        let eq43_e1306_d_n8: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n8 * idt_scale));
        let eq43_e1306_d_n12: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n12 * idt_scale));
        let eq43_e1306_d_n13: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n13 * idt_scale));
        let eq43_e1306_d_n14: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n14 * idt_scale));
        let eq43_e1306_d_n15: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n15 * idt_scale));
        let eq43_e1306_d_n16: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n16 * idt_scale));
        let eq43_e1306_d_n17: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n17 * idt_scale));
        let eq43_e1306_d_n18: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n18 * idt_scale));
        let eq43_e1306_d_n19: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n19 * idt_scale));
        let eq43_e1306_d_n20: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n20 * idt_scale));
        let eq43_value: f64 = eq43_e1306;
        let eq43_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq43_node_derivatives: [f64; 13] = [eq43_e1306_d_n5, eq43_e1306_d_n6, eq43_e1306_d_n7, eq43_e1306_d_n8, eq43_e1306_d_n12, eq43_e1306_d_n13, eq43_e1306_d_n14, eq43_e1306_d_n15, eq43_e1306_d_n16, eq43_e1306_d_n17, eq43_e1306_d_n18, eq43_e1306_d_n19, eq43_e1306_d_n20];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            12,
            eq43_value,
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
        );
        let eq45_e1314: f64 = (-locals.var_tnorm);
        let eq45_e1316: f64 = (eq45_e1314 * locals.var_fk4);
        let eq45_e1316_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn5));
        let eq45_e1316_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn6));
        let eq45_e1316_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn7));
        let eq45_e1316_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn8));
        let eq45_e1316_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn12));
        let eq45_e1316_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn13));
        let eq45_e1316_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn14));
        let eq45_e1316_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn15));
        let eq45_e1316_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn16));
        let eq45_e1316_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn17));
        let eq45_e1316_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn18));
        let eq45_e1316_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn19));
        let eq45_e1316_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn20));
        let eq45_e1318: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 3, eq45_e1316, locals.var_qp4_0);
        let eq45_e1319: f64 = (locals.var_vnorm_inv * eq45_e1318);
        let eq45_e1319_d_n5: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n5 * idt_scale));
        let eq45_e1319_d_n6: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n6 * idt_scale));
        let eq45_e1319_d_n7: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n7 * idt_scale));
        let eq45_e1319_d_n8: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n8 * idt_scale));
        let eq45_e1319_d_n12: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n12 * idt_scale));
        let eq45_e1319_d_n13: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n13 * idt_scale));
        let eq45_e1319_d_n14: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n14 * idt_scale));
        let eq45_e1319_d_n15: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n15 * idt_scale));
        let eq45_e1319_d_n16: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n16 * idt_scale));
        let eq45_e1319_d_n17: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n17 * idt_scale));
        let eq45_e1319_d_n18: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n18 * idt_scale));
        let eq45_e1319_d_n19: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n19 * idt_scale));
        let eq45_e1319_d_n20: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n20 * idt_scale));
        let eq45_value: f64 = eq45_e1319;
        let eq45_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq45_node_derivatives: [f64; 13] = [eq45_e1319_d_n5, eq45_e1319_d_n6, eq45_e1319_d_n7, eq45_e1319_d_n8, eq45_e1319_d_n12, eq45_e1319_d_n13, eq45_e1319_d_n14, eq45_e1319_d_n15, eq45_e1319_d_n16, eq45_e1319_d_n17, eq45_e1319_d_n18, eq45_e1319_d_n19, eq45_e1319_d_n20];
        let eq45_branch_derivative_indices: [usize; 0] = [];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            14,
            eq45_value,
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
        );
        let eq47_e1327: f64 = (-locals.var_tnorm);
        let eq47_e1329: f64 = (eq47_e1327 * locals.var_fk5);
        let eq47_e1329_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn5));
        let eq47_e1329_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn6));
        let eq47_e1329_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn7));
        let eq47_e1329_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn8));
        let eq47_e1329_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn12));
        let eq47_e1329_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn13));
        let eq47_e1329_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn14));
        let eq47_e1329_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn15));
        let eq47_e1329_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn16));
        let eq47_e1329_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn17));
        let eq47_e1329_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn18));
        let eq47_e1329_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn19));
        let eq47_e1329_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn20));
        let eq47_e1331: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 4, eq47_e1329, locals.var_qp5_0);
        let eq47_e1332: f64 = (locals.var_vnorm_inv * eq47_e1331);
        let eq47_e1332_d_n5: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n5 * idt_scale));
        let eq47_e1332_d_n6: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n6 * idt_scale));
        let eq47_e1332_d_n7: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n7 * idt_scale));
        let eq47_e1332_d_n8: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n8 * idt_scale));
        let eq47_e1332_d_n12: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n12 * idt_scale));
        let eq47_e1332_d_n13: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n13 * idt_scale));
        let eq47_e1332_d_n14: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n14 * idt_scale));
        let eq47_e1332_d_n15: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n15 * idt_scale));
        let eq47_e1332_d_n16: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n16 * idt_scale));
        let eq47_e1332_d_n17: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n17 * idt_scale));
        let eq47_e1332_d_n18: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n18 * idt_scale));
        let eq47_e1332_d_n19: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n19 * idt_scale));
        let eq47_e1332_d_n20: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n20 * idt_scale));
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq47_node_derivatives: [f64; 13] = [eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n12, eq47_e1332_d_n13, eq47_e1332_d_n14, eq47_e1332_d_n15, eq47_e1332_d_n16, eq47_e1332_d_n17, eq47_e1332_d_n18, eq47_e1332_d_n19, eq47_e1332_d_n20];
        let eq47_branch_derivative_indices: [usize; 0] = [];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            16,
            eq47_value,
            &eq47_node_derivative_indices,
            &eq47_node_derivatives,
            &eq47_branch_derivative_indices,
            &eq47_branch_derivatives,
        );
        let eq49_e1340: f64 = (-locals.var_tnorm);
        let eq49_e1342: f64 = (eq49_e1340 * locals.var_fk6);
        let eq49_e1342_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn5));
        let eq49_e1342_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn6));
        let eq49_e1342_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn7));
        let eq49_e1342_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn8));
        let eq49_e1342_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn12));
        let eq49_e1342_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn13));
        let eq49_e1342_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn14));
        let eq49_e1342_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn15));
        let eq49_e1342_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn16));
        let eq49_e1342_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn17));
        let eq49_e1342_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn18));
        let eq49_e1342_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn19));
        let eq49_e1342_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn20));
        let eq49_e1344: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 5, eq49_e1342, locals.var_qp6_0);
        let eq49_e1345: f64 = (locals.var_vnorm_inv * eq49_e1344);
        let eq49_e1345_d_n5: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n5 * idt_scale));
        let eq49_e1345_d_n6: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n6 * idt_scale));
        let eq49_e1345_d_n7: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n7 * idt_scale));
        let eq49_e1345_d_n8: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n8 * idt_scale));
        let eq49_e1345_d_n12: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n12 * idt_scale));
        let eq49_e1345_d_n13: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n13 * idt_scale));
        let eq49_e1345_d_n14: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n14 * idt_scale));
        let eq49_e1345_d_n15: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n15 * idt_scale));
        let eq49_e1345_d_n16: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n16 * idt_scale));
        let eq49_e1345_d_n17: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n17 * idt_scale));
        let eq49_e1345_d_n18: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n18 * idt_scale));
        let eq49_e1345_d_n19: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n19 * idt_scale));
        let eq49_e1345_d_n20: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n20 * idt_scale));
        let eq49_value: f64 = eq49_e1345;
        let eq49_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq49_node_derivatives: [f64; 13] = [eq49_e1345_d_n5, eq49_e1345_d_n6, eq49_e1345_d_n7, eq49_e1345_d_n8, eq49_e1345_d_n12, eq49_e1345_d_n13, eq49_e1345_d_n14, eq49_e1345_d_n15, eq49_e1345_d_n16, eq49_e1345_d_n17, eq49_e1345_d_n18, eq49_e1345_d_n19, eq49_e1345_d_n20];
        let eq49_branch_derivative_indices: [usize; 0] = [];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            18,
            eq49_value,
            &eq49_node_derivative_indices,
            &eq49_node_derivatives,
            &eq49_branch_derivative_indices,
            &eq49_branch_derivatives,
        );
        let eq51_e1353: f64 = (-locals.var_tnorm);
        let eq51_e1355: f64 = (eq51_e1353 * locals.var_fk7);
        let eq51_e1355_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn5));
        let eq51_e1355_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn6));
        let eq51_e1355_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn7));
        let eq51_e1355_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn8));
        let eq51_e1355_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn12));
        let eq51_e1355_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn13));
        let eq51_e1355_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn14));
        let eq51_e1355_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn15));
        let eq51_e1355_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn16));
        let eq51_e1355_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn17));
        let eq51_e1355_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn18));
        let eq51_e1355_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn19));
        let eq51_e1355_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn20));
        let eq51_e1357: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 6, eq51_e1355, locals.var_qp7_0);
        let eq51_e1358: f64 = (locals.var_vnorm_inv * eq51_e1357);
        let eq51_e1358_d_n5: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n5 * idt_scale));
        let eq51_e1358_d_n6: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n6 * idt_scale));
        let eq51_e1358_d_n7: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n7 * idt_scale));
        let eq51_e1358_d_n8: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n8 * idt_scale));
        let eq51_e1358_d_n12: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n12 * idt_scale));
        let eq51_e1358_d_n13: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n13 * idt_scale));
        let eq51_e1358_d_n14: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n14 * idt_scale));
        let eq51_e1358_d_n15: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n15 * idt_scale));
        let eq51_e1358_d_n16: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n16 * idt_scale));
        let eq51_e1358_d_n17: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n17 * idt_scale));
        let eq51_e1358_d_n18: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n18 * idt_scale));
        let eq51_e1358_d_n19: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n19 * idt_scale));
        let eq51_e1358_d_n20: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n20 * idt_scale));
        let eq51_value: f64 = eq51_e1358;
        let eq51_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq51_node_derivatives: [f64; 13] = [eq51_e1358_d_n5, eq51_e1358_d_n6, eq51_e1358_d_n7, eq51_e1358_d_n8, eq51_e1358_d_n12, eq51_e1358_d_n13, eq51_e1358_d_n14, eq51_e1358_d_n15, eq51_e1358_d_n16, eq51_e1358_d_n17, eq51_e1358_d_n18, eq51_e1358_d_n19, eq51_e1358_d_n20];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            20,
            eq51_value,
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
        );
        let eq53_e1366: f64 = (-locals.var_tnorm);
        let eq53_e1368: f64 = (eq53_e1366 * locals.var_fk8);
        let eq53_e1368_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn5));
        let eq53_e1368_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn6));
        let eq53_e1368_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn7));
        let eq53_e1368_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn8));
        let eq53_e1368_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn12));
        let eq53_e1368_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn13));
        let eq53_e1368_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn14));
        let eq53_e1368_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn15));
        let eq53_e1368_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn16));
        let eq53_e1368_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn17));
        let eq53_e1368_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn18));
        let eq53_e1368_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn19));
        let eq53_e1368_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn20));
        let eq53_e1370: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 7, eq53_e1368, locals.var_qp8_0);
        let eq53_e1371: f64 = (locals.var_vnorm_inv * eq53_e1370);
        let eq53_e1371_d_n5: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n5 * idt_scale));
        let eq53_e1371_d_n6: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n6 * idt_scale));
        let eq53_e1371_d_n7: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n7 * idt_scale));
        let eq53_e1371_d_n8: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n8 * idt_scale));
        let eq53_e1371_d_n12: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n12 * idt_scale));
        let eq53_e1371_d_n13: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n13 * idt_scale));
        let eq53_e1371_d_n14: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n14 * idt_scale));
        let eq53_e1371_d_n15: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n15 * idt_scale));
        let eq53_e1371_d_n16: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n16 * idt_scale));
        let eq53_e1371_d_n17: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n17 * idt_scale));
        let eq53_e1371_d_n18: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n18 * idt_scale));
        let eq53_e1371_d_n19: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n19 * idt_scale));
        let eq53_e1371_d_n20: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n20 * idt_scale));
        let eq53_value: f64 = eq53_e1371;
        let eq53_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq53_node_derivatives: [f64; 13] = [eq53_e1371_d_n5, eq53_e1371_d_n6, eq53_e1371_d_n7, eq53_e1371_d_n8, eq53_e1371_d_n12, eq53_e1371_d_n13, eq53_e1371_d_n14, eq53_e1371_d_n15, eq53_e1371_d_n16, eq53_e1371_d_n17, eq53_e1371_d_n18, eq53_e1371_d_n19, eq53_e1371_d_n20];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            22,
            eq53_value,
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
        );
        let eq55_e1379: f64 = (-locals.var_tnorm);
        let eq55_e1381: f64 = (eq55_e1379 * locals.var_fk9);
        let eq55_e1381_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn5));
        let eq55_e1381_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn6));
        let eq55_e1381_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn7));
        let eq55_e1381_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn8));
        let eq55_e1381_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn12));
        let eq55_e1381_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn13));
        let eq55_e1381_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn14));
        let eq55_e1381_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn15));
        let eq55_e1381_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn16));
        let eq55_e1381_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn17));
        let eq55_e1381_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn18));
        let eq55_e1381_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn19));
        let eq55_e1381_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn20));
        let eq55_e1383: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 8, eq55_e1381, locals.var_qp9_0);
        let eq55_e1384: f64 = (locals.var_vnorm_inv * eq55_e1383);
        let eq55_e1384_d_n5: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n5 * idt_scale));
        let eq55_e1384_d_n6: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n6 * idt_scale));
        let eq55_e1384_d_n7: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n7 * idt_scale));
        let eq55_e1384_d_n8: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n8 * idt_scale));
        let eq55_e1384_d_n12: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n12 * idt_scale));
        let eq55_e1384_d_n13: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n13 * idt_scale));
        let eq55_e1384_d_n14: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n14 * idt_scale));
        let eq55_e1384_d_n15: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n15 * idt_scale));
        let eq55_e1384_d_n16: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n16 * idt_scale));
        let eq55_e1384_d_n17: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n17 * idt_scale));
        let eq55_e1384_d_n18: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n18 * idt_scale));
        let eq55_e1384_d_n19: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n19 * idt_scale));
        let eq55_e1384_d_n20: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n20 * idt_scale));
        let eq55_value: f64 = eq55_e1384;
        let eq55_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq55_node_derivatives: [f64; 13] = [eq55_e1384_d_n5, eq55_e1384_d_n6, eq55_e1384_d_n7, eq55_e1384_d_n8, eq55_e1384_d_n12, eq55_e1384_d_n13, eq55_e1384_d_n14, eq55_e1384_d_n15, eq55_e1384_d_n16, eq55_e1384_d_n17, eq55_e1384_d_n18, eq55_e1384_d_n19, eq55_e1384_d_n20];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            24,
            eq55_value,
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
        );
        let eq67_e1463: f64 = (locals.var_mult_inst * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * locals.var_cgeff);
        let eq67_e1468_d_n5: f64 = (eq67_e1466 * locals.var_cgeff_dn5);
        let eq67_e1468_d_n6: f64 = (eq67_e1466 * locals.var_cgeff_dn6);
        let eq67_e1468_d_n7: f64 = (eq67_e1466 * locals.var_cgeff_dn7);
        let eq67_e1468_d_n8: f64 = (eq67_e1466 * locals.var_cgeff_dn8);
        let eq67_e1468_d_n12: f64 = (eq67_e1466 * locals.var_cgeff_dn12);
        let eq67_e1468_d_n13: f64 = (eq67_e1466 * locals.var_cgeff_dn13);
        let eq67_e1468_d_n14: f64 = (eq67_e1466 * locals.var_cgeff_dn14);
        let eq67_e1468_d_n15: f64 = (eq67_e1466 * locals.var_cgeff_dn15);
        let eq67_e1468_d_n16: f64 = (eq67_e1466 * locals.var_cgeff_dn16);
        let eq67_e1468_d_n17: f64 = (eq67_e1466 * locals.var_cgeff_dn17);
        let eq67_e1468_d_n18: f64 = (eq67_e1466 * locals.var_cgeff_dn18);
        let eq67_e1468_d_n19: f64 = (eq67_e1466 * locals.var_cgeff_dn19);
        let eq67_e1468_d_n20: f64 = (eq67_e1466 * locals.var_cgeff_dn20);
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq67_e1470);
        let eq67_e1472: f64 = (-eq67_e1471);
        let eq67_e1472_d_n4: f64 = (-(eq67_e1468 * ddt_scale));
        let eq67_e1472_d_n5: f64 = (-(eq67_e1470_d_n5 * ddt_scale));
        let eq67_e1472_d_n6: f64 = (-(eq67_e1470_d_n6 * ddt_scale));
        let eq67_e1472_d_n7: f64 = (-(eq67_e1470_d_n7 * ddt_scale));
        let eq67_e1472_d_n8: f64 = (-(eq67_e1470_d_n8 * ddt_scale));
        let eq67_e1472_d_n12: f64 = (-(eq67_e1470_d_n12 * ddt_scale));
        let eq67_e1472_d_n13: f64 = (-(eq67_e1470_d_n13 * ddt_scale));
        let eq67_e1472_d_n14: f64 = (-(eq67_e1470_d_n14 * ddt_scale));
        let eq67_e1472_d_n15: f64 = (-(eq67_e1470_d_n15 * ddt_scale));
        let eq67_e1472_d_n16: f64 = (-(eq67_e1470_d_n16 * ddt_scale));
        let eq67_e1472_d_n17: f64 = (-(eq67_e1470_d_n17 * ddt_scale));
        let eq67_e1472_d_n18: f64 = (-(eq67_e1470_d_n18 * ddt_scale));
        let eq67_e1472_d_n19: f64 = (-(eq67_e1470_d_n19 * ddt_scale));
        let eq67_e1472_d_n20: f64 = (-(eq67_e1470_d_n20 * ddt_scale));
        let eq67_value: f64 = eq67_e1472;
        let eq67_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq67_node_derivatives: [f64; 14] = [eq67_e1472_d_n4, eq67_e1472_d_n5, eq67_e1472_d_n6, eq67_e1472_d_n7, eq67_e1472_d_n8, eq67_e1472_d_n12, eq67_e1472_d_n13, eq67_e1472_d_n14, eq67_e1472_d_n15, eq67_e1472_d_n16, eq67_e1472_d_n17, eq67_e1472_d_n18, eq67_e1472_d_n19, eq67_e1472_d_n20];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (locals.var_mult_inst * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * locals.var_cgeff);
        let eq68_e1480_d_n5: f64 = (eq68_e1478 * locals.var_cgeff_dn5);
        let eq68_e1480_d_n6: f64 = (eq68_e1478 * locals.var_cgeff_dn6);
        let eq68_e1480_d_n7: f64 = (eq68_e1478 * locals.var_cgeff_dn7);
        let eq68_e1480_d_n8: f64 = (eq68_e1478 * locals.var_cgeff_dn8);
        let eq68_e1480_d_n12: f64 = (eq68_e1478 * locals.var_cgeff_dn12);
        let eq68_e1480_d_n13: f64 = (eq68_e1478 * locals.var_cgeff_dn13);
        let eq68_e1480_d_n14: f64 = (eq68_e1478 * locals.var_cgeff_dn14);
        let eq68_e1480_d_n15: f64 = (eq68_e1478 * locals.var_cgeff_dn15);
        let eq68_e1480_d_n16: f64 = (eq68_e1478 * locals.var_cgeff_dn16);
        let eq68_e1480_d_n17: f64 = (eq68_e1478 * locals.var_cgeff_dn17);
        let eq68_e1480_d_n18: f64 = (eq68_e1478 * locals.var_cgeff_dn18);
        let eq68_e1480_d_n19: f64 = (eq68_e1478 * locals.var_cgeff_dn19);
        let eq68_e1480_d_n20: f64 = (eq68_e1478 * locals.var_cgeff_dn20);
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq68_e1482);
        let eq68_e1484: f64 = (-eq68_e1483);
        let eq68_e1484_d_n4: f64 = (-(eq68_e1480 * ddt_scale));
        let eq68_e1484_d_n5: f64 = (-(eq68_e1482_d_n5 * ddt_scale));
        let eq68_e1484_d_n6: f64 = (-(eq68_e1482_d_n6 * ddt_scale));
        let eq68_e1484_d_n7: f64 = (-(eq68_e1482_d_n7 * ddt_scale));
        let eq68_e1484_d_n8: f64 = (-(eq68_e1482_d_n8 * ddt_scale));
        let eq68_e1484_d_n12: f64 = (-(eq68_e1482_d_n12 * ddt_scale));
        let eq68_e1484_d_n13: f64 = (-(eq68_e1482_d_n13 * ddt_scale));
        let eq68_e1484_d_n14: f64 = (-(eq68_e1482_d_n14 * ddt_scale));
        let eq68_e1484_d_n15: f64 = (-(eq68_e1482_d_n15 * ddt_scale));
        let eq68_e1484_d_n16: f64 = (-(eq68_e1482_d_n16 * ddt_scale));
        let eq68_e1484_d_n17: f64 = (-(eq68_e1482_d_n17 * ddt_scale));
        let eq68_e1484_d_n18: f64 = (-(eq68_e1482_d_n18 * ddt_scale));
        let eq68_e1484_d_n19: f64 = (-(eq68_e1482_d_n19 * ddt_scale));
        let eq68_e1484_d_n20: f64 = (-(eq68_e1482_d_n20 * ddt_scale));
        let eq68_value: f64 = eq68_e1484;
        let eq68_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq68_node_derivatives: [f64; 14] = [eq68_e1484_d_n4, eq68_e1484_d_n5, eq68_e1484_d_n6, eq68_e1484_d_n7, eq68_e1484_d_n8, eq68_e1484_d_n12, eq68_e1484_d_n13, eq68_e1484_d_n14, eq68_e1484_d_n15, eq68_e1484_d_n16, eq68_e1484_d_n17, eq68_e1484_d_n18, eq68_e1484_d_n19, eq68_e1484_d_n20];
        let eq68_branch_derivative_indices: [usize; 0] = [];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq68_value),
            &eq68_node_derivative_indices,
            &eq68_node_derivatives,
            &eq68_branch_derivative_indices,
            &eq68_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq67_e1463: f64 = (locals.var_mult_inst * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * locals.var_cgeff);
        let eq67_e1468_d_n5: f64 = (eq67_e1466 * locals.var_cgeff_dn5);
        let eq67_e1468_d_n6: f64 = (eq67_e1466 * locals.var_cgeff_dn6);
        let eq67_e1468_d_n7: f64 = (eq67_e1466 * locals.var_cgeff_dn7);
        let eq67_e1468_d_n8: f64 = (eq67_e1466 * locals.var_cgeff_dn8);
        let eq67_e1468_d_n12: f64 = (eq67_e1466 * locals.var_cgeff_dn12);
        let eq67_e1468_d_n13: f64 = (eq67_e1466 * locals.var_cgeff_dn13);
        let eq67_e1468_d_n14: f64 = (eq67_e1466 * locals.var_cgeff_dn14);
        let eq67_e1468_d_n15: f64 = (eq67_e1466 * locals.var_cgeff_dn15);
        let eq67_e1468_d_n16: f64 = (eq67_e1466 * locals.var_cgeff_dn16);
        let eq67_e1468_d_n17: f64 = (eq67_e1466 * locals.var_cgeff_dn17);
        let eq67_e1468_d_n18: f64 = (eq67_e1466 * locals.var_cgeff_dn18);
        let eq67_e1468_d_n19: f64 = (eq67_e1466 * locals.var_cgeff_dn19);
        let eq67_e1468_d_n20: f64 = (eq67_e1466 * locals.var_cgeff_dn20);
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1471_q: f64 = eq67_e1470;
        let eq67_e1472: f64 = (-eq67_e1470);
        let eq67_e1472_q: f64 = (-eq67_e1471_q);
        let eq67_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, (-eq67_e1468), (-eq67_e1470_d_n5), (-eq67_e1470_d_n6), (-eq67_e1470_d_n7), (-eq67_e1470_d_n8), 0.0, 0.0, 0.0, (-eq67_e1470_d_n12), (-eq67_e1470_d_n13), (-eq67_e1470_d_n14), (-eq67_e1470_d_n15), (-eq67_e1470_d_n16), (-eq67_e1470_d_n17), (-eq67_e1470_d_n18), (-eq67_e1470_d_n19), (-eq67_e1470_d_n20)];
        let eq67_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (locals.var_mult_inst * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * locals.var_cgeff);
        let eq68_e1480_d_n5: f64 = (eq68_e1478 * locals.var_cgeff_dn5);
        let eq68_e1480_d_n6: f64 = (eq68_e1478 * locals.var_cgeff_dn6);
        let eq68_e1480_d_n7: f64 = (eq68_e1478 * locals.var_cgeff_dn7);
        let eq68_e1480_d_n8: f64 = (eq68_e1478 * locals.var_cgeff_dn8);
        let eq68_e1480_d_n12: f64 = (eq68_e1478 * locals.var_cgeff_dn12);
        let eq68_e1480_d_n13: f64 = (eq68_e1478 * locals.var_cgeff_dn13);
        let eq68_e1480_d_n14: f64 = (eq68_e1478 * locals.var_cgeff_dn14);
        let eq68_e1480_d_n15: f64 = (eq68_e1478 * locals.var_cgeff_dn15);
        let eq68_e1480_d_n16: f64 = (eq68_e1478 * locals.var_cgeff_dn16);
        let eq68_e1480_d_n17: f64 = (eq68_e1478 * locals.var_cgeff_dn17);
        let eq68_e1480_d_n18: f64 = (eq68_e1478 * locals.var_cgeff_dn18);
        let eq68_e1480_d_n19: f64 = (eq68_e1478 * locals.var_cgeff_dn19);
        let eq68_e1480_d_n20: f64 = (eq68_e1478 * locals.var_cgeff_dn20);
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1483_q: f64 = eq68_e1482;
        let eq68_e1484: f64 = (-eq68_e1482);
        let eq68_e1484_q: f64 = (-eq68_e1483_q);
        let eq68_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, (-eq68_e1480), (-eq68_e1482_d_n5), (-eq68_e1482_d_n6), (-eq68_e1482_d_n7), (-eq68_e1482_d_n8), 0.0, 0.0, 0.0, (-eq68_e1482_d_n12), (-eq68_e1482_d_n13), (-eq68_e1482_d_n14), (-eq68_e1482_d_n15), (-eq68_e1482_d_n16), (-eq68_e1482_d_n17), (-eq68_e1482_d_n18), (-eq68_e1482_d_n19), (-eq68_e1482_d_n20)];
        let eq68_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq68_reactive_node_derivatives,
            branches,
            &eq68_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
