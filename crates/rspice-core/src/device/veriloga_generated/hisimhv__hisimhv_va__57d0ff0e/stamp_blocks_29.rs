#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_75(
        locals: &mut StampLocals,
    ) {
        let assign25310_e23582: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard620 = assign25310_e23582;
        locals.var_guard620_rv = 0.0;

        let assign25320_e23586: f64 = (locals.var_phi_s0_dep - 0.02);
        let assign25320_e23591: f64 = if ((locals.var_phi_b0_dep > assign25320_e23586) && (0.02 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard621 = assign25320_e23591;
        locals.var_guard621_rv = 0.0;

        let (assign25330_e23605, assign25330_e23605_d_n0, assign25330_e23605_d_n2, assign25330_e23605_d_n4, assign25330_e23605_d_n5, assign25330_e23605_d_n6, assign25330_e23605_d_n7, assign25330_e23605_d_n8, assign25330_e23605_d_n9, assign25330_e23605_d_n10, assign25330_e23605_d_n11, assign25330_e23605_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25330_e23601: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep);
        let assign25330_e23603: f64 = (assign25330_e23601 + 0.02);
        (assign25330_e23603, (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign25330_e23605;
        locals.var_tmf1_dn0 = assign25330_e23605_d_n0;
        locals.var_tmf1_dn2 = assign25330_e23605_d_n2;
        locals.var_tmf1_dn4 = assign25330_e23605_d_n4;
        locals.var_tmf1_dn5 = assign25330_e23605_d_n5;
        locals.var_tmf1_dn6 = assign25330_e23605_d_n6;
        locals.var_tmf1_dn7 = assign25330_e23605_d_n7;
        locals.var_tmf1_dn8 = assign25330_e23605_d_n8;
        locals.var_tmf1_dn9 = assign25330_e23605_d_n9;
        locals.var_tmf1_dn10 = assign25330_e23605_d_n10;
        locals.var_tmf1_dn11 = assign25330_e23605_d_n11;
        locals.var_tmf1_dn14 = assign25330_e23605_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign25340_e23617, assign25340_e23617_d_n0, assign25340_e23617_d_n2, assign25340_e23617_d_n4, assign25340_e23617_d_n5, assign25340_e23617_d_n6, assign25340_e23617_d_n7, assign25340_e23617_d_n8, assign25340_e23617_d_n9, assign25340_e23617_d_n10, assign25340_e23617_d_n11, assign25340_e23617_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25340_e23615: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25340_e23615, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign25340_e23617;
        locals.var_x2_dn0 = assign25340_e23617_d_n0;
        locals.var_x2_dn2 = assign25340_e23617_d_n2;
        locals.var_x2_dn4 = assign25340_e23617_d_n4;
        locals.var_x2_dn5 = assign25340_e23617_d_n5;
        locals.var_x2_dn6 = assign25340_e23617_d_n6;
        locals.var_x2_dn7 = assign25340_e23617_d_n7;
        locals.var_x2_dn8 = assign25340_e23617_d_n8;
        locals.var_x2_dn9 = assign25340_e23617_d_n9;
        locals.var_x2_dn10 = assign25340_e23617_d_n10;
        locals.var_x2_dn11 = assign25340_e23617_d_n11;
        locals.var_x2_dn14 = assign25340_e23617_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign25350_e23629, assign25350_e23629_d_n0, assign25350_e23629_d_n2, assign25350_e23629_d_n4, assign25350_e23629_d_n5, assign25350_e23629_d_n6, assign25350_e23629_d_n7, assign25350_e23629_d_n8, assign25350_e23629_d_n9, assign25350_e23629_d_n10, assign25350_e23629_d_n11, assign25350_e23629_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25350_e23627: f64 = (0.02 * 0.02);
        (assign25350_e23627, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign25350_e23629;
        locals.var_xmax2_dn0 = assign25350_e23629_d_n0;
        locals.var_xmax2_dn2 = assign25350_e23629_d_n2;
        locals.var_xmax2_dn4 = assign25350_e23629_d_n4;
        locals.var_xmax2_dn5 = assign25350_e23629_d_n5;
        locals.var_xmax2_dn6 = assign25350_e23629_d_n6;
        locals.var_xmax2_dn7 = assign25350_e23629_d_n7;
        locals.var_xmax2_dn8 = assign25350_e23629_d_n8;
        locals.var_xmax2_dn9 = assign25350_e23629_d_n9;
        locals.var_xmax2_dn10 = assign25350_e23629_d_n10;
        locals.var_xmax2_dn11 = assign25350_e23629_d_n11;
        locals.var_xmax2_dn14 = assign25350_e23629_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign25360_e23639, assign25360_e23639_d_n0, assign25360_e23639_d_n2, assign25360_e23639_d_n4, assign25360_e23639_d_n5, assign25360_e23639_d_n6, assign25360_e23639_d_n7, assign25360_e23639_d_n8, assign25360_e23639_d_n9, assign25360_e23639_d_n10, assign25360_e23639_d_n11, assign25360_e23639_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25360_e23639;
        locals.var_xp_dn0 = assign25360_e23639_d_n0;
        locals.var_xp_dn2 = assign25360_e23639_d_n2;
        locals.var_xp_dn4 = assign25360_e23639_d_n4;
        locals.var_xp_dn5 = assign25360_e23639_d_n5;
        locals.var_xp_dn6 = assign25360_e23639_d_n6;
        locals.var_xp_dn7 = assign25360_e23639_d_n7;
        locals.var_xp_dn8 = assign25360_e23639_d_n8;
        locals.var_xp_dn9 = assign25360_e23639_d_n9;
        locals.var_xp_dn10 = assign25360_e23639_d_n10;
        locals.var_xp_dn11 = assign25360_e23639_d_n11;
        locals.var_xp_dn14 = assign25360_e23639_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25370_e23649, assign25370_e23649_d_n0, assign25370_e23649_d_n2, assign25370_e23649_d_n4, assign25370_e23649_d_n5, assign25370_e23649_d_n6, assign25370_e23649_d_n7, assign25370_e23649_d_n8, assign25370_e23649_d_n9, assign25370_e23649_d_n10, assign25370_e23649_d_n11, assign25370_e23649_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25370_e23649;
        locals.var_xmp_dn0 = assign25370_e23649_d_n0;
        locals.var_xmp_dn2 = assign25370_e23649_d_n2;
        locals.var_xmp_dn4 = assign25370_e23649_d_n4;
        locals.var_xmp_dn5 = assign25370_e23649_d_n5;
        locals.var_xmp_dn6 = assign25370_e23649_d_n6;
        locals.var_xmp_dn7 = assign25370_e23649_d_n7;
        locals.var_xmp_dn8 = assign25370_e23649_d_n8;
        locals.var_xmp_dn9 = assign25370_e23649_d_n9;
        locals.var_xmp_dn10 = assign25370_e23649_d_n10;
        locals.var_xmp_dn11 = assign25370_e23649_d_n11;
        locals.var_xmp_dn14 = assign25370_e23649_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25380_e23659,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25380_e23659;
        locals.var_m0_rv = 0.0;

        let (assign25390_e23669,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25390_e23669;
        locals.var_mm_rv = 0.0;

        let (assign25400_e23679, assign25400_e23679_d_n0, assign25400_e23679_d_n2, assign25400_e23679_d_n4, assign25400_e23679_d_n5, assign25400_e23679_d_n6, assign25400_e23679_d_n7, assign25400_e23679_d_n8, assign25400_e23679_d_n9, assign25400_e23679_d_n10, assign25400_e23679_d_n11, assign25400_e23679_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25400_e23679;
        locals.var_arg_dn0 = assign25400_e23679_d_n0;
        locals.var_arg_dn2 = assign25400_e23679_d_n2;
        locals.var_arg_dn4 = assign25400_e23679_d_n4;
        locals.var_arg_dn5 = assign25400_e23679_d_n5;
        locals.var_arg_dn6 = assign25400_e23679_d_n6;
        locals.var_arg_dn7 = assign25400_e23679_d_n7;
        locals.var_arg_dn8 = assign25400_e23679_d_n8;
        locals.var_arg_dn9 = assign25400_e23679_d_n9;
        locals.var_arg_dn10 = assign25400_e23679_d_n10;
        locals.var_arg_dn11 = assign25400_e23679_d_n11;
        locals.var_arg_dn14 = assign25400_e23679_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25410_e23689, assign25410_e23689_d_n0, assign25410_e23689_d_n2, assign25410_e23689_d_n4, assign25410_e23689_d_n5, assign25410_e23689_d_n6, assign25410_e23689_d_n7, assign25410_e23689_d_n8, assign25410_e23689_d_n9, assign25410_e23689_d_n10, assign25410_e23689_d_n11, assign25410_e23689_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25410_e23689;
        locals.var_dnm_dn0 = assign25410_e23689_d_n0;
        locals.var_dnm_dn2 = assign25410_e23689_d_n2;
        locals.var_dnm_dn4 = assign25410_e23689_d_n4;
        locals.var_dnm_dn5 = assign25410_e23689_d_n5;
        locals.var_dnm_dn6 = assign25410_e23689_d_n6;
        locals.var_dnm_dn7 = assign25410_e23689_d_n7;
        locals.var_dnm_dn8 = assign25410_e23689_d_n8;
        locals.var_dnm_dn9 = assign25410_e23689_d_n9;
        locals.var_dnm_dn10 = assign25410_e23689_d_n10;
        locals.var_dnm_dn11 = assign25410_e23689_d_n11;
        locals.var_dnm_dn14 = assign25410_e23689_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25420_e23701, assign25420_e23701_d_n0, assign25420_e23701_d_n2, assign25420_e23701_d_n4, assign25420_e23701_d_n5, assign25420_e23701_d_n6, assign25420_e23701_d_n7, assign25420_e23701_d_n8, assign25420_e23701_d_n9, assign25420_e23701_d_n10, assign25420_e23701_d_n11, assign25420_e23701_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25420_e23699: f64 = (locals.var_xp * locals.var_x2);
        (assign25420_e23699, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25420_e23701;
        locals.var_xp_dn0 = assign25420_e23701_d_n0;
        locals.var_xp_dn2 = assign25420_e23701_d_n2;
        locals.var_xp_dn4 = assign25420_e23701_d_n4;
        locals.var_xp_dn5 = assign25420_e23701_d_n5;
        locals.var_xp_dn6 = assign25420_e23701_d_n6;
        locals.var_xp_dn7 = assign25420_e23701_d_n7;
        locals.var_xp_dn8 = assign25420_e23701_d_n8;
        locals.var_xp_dn9 = assign25420_e23701_d_n9;
        locals.var_xp_dn10 = assign25420_e23701_d_n10;
        locals.var_xp_dn11 = assign25420_e23701_d_n11;
        locals.var_xp_dn14 = assign25420_e23701_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25430_e23713, assign25430_e23713_d_n0, assign25430_e23713_d_n2, assign25430_e23713_d_n4, assign25430_e23713_d_n5, assign25430_e23713_d_n6, assign25430_e23713_d_n7, assign25430_e23713_d_n8, assign25430_e23713_d_n9, assign25430_e23713_d_n10, assign25430_e23713_d_n11, assign25430_e23713_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25430_e23711: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25430_e23711, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25430_e23713;
        locals.var_xmp_dn0 = assign25430_e23713_d_n0;
        locals.var_xmp_dn2 = assign25430_e23713_d_n2;
        locals.var_xmp_dn4 = assign25430_e23713_d_n4;
        locals.var_xmp_dn5 = assign25430_e23713_d_n5;
        locals.var_xmp_dn6 = assign25430_e23713_d_n6;
        locals.var_xmp_dn7 = assign25430_e23713_d_n7;
        locals.var_xmp_dn8 = assign25430_e23713_d_n8;
        locals.var_xmp_dn9 = assign25430_e23713_d_n9;
        locals.var_xmp_dn10 = assign25430_e23713_d_n10;
        locals.var_xmp_dn11 = assign25430_e23713_d_n11;
        locals.var_xmp_dn14 = assign25430_e23713_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25440_e23725, assign25440_e23725_d_n0, assign25440_e23725_d_n2, assign25440_e23725_d_n4, assign25440_e23725_d_n5, assign25440_e23725_d_n6, assign25440_e23725_d_n7, assign25440_e23725_d_n8, assign25440_e23725_d_n9, assign25440_e23725_d_n10, assign25440_e23725_d_n11, assign25440_e23725_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25440_e23723: f64 = (locals.var_xp * locals.var_x2);
        (assign25440_e23723, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25440_e23725;
        locals.var_xp_dn0 = assign25440_e23725_d_n0;
        locals.var_xp_dn2 = assign25440_e23725_d_n2;
        locals.var_xp_dn4 = assign25440_e23725_d_n4;
        locals.var_xp_dn5 = assign25440_e23725_d_n5;
        locals.var_xp_dn6 = assign25440_e23725_d_n6;
        locals.var_xp_dn7 = assign25440_e23725_d_n7;
        locals.var_xp_dn8 = assign25440_e23725_d_n8;
        locals.var_xp_dn9 = assign25440_e23725_d_n9;
        locals.var_xp_dn10 = assign25440_e23725_d_n10;
        locals.var_xp_dn11 = assign25440_e23725_d_n11;
        locals.var_xp_dn14 = assign25440_e23725_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25450_e23737, assign25450_e23737_d_n0, assign25450_e23737_d_n2, assign25450_e23737_d_n4, assign25450_e23737_d_n5, assign25450_e23737_d_n6, assign25450_e23737_d_n7, assign25450_e23737_d_n8, assign25450_e23737_d_n9, assign25450_e23737_d_n10, assign25450_e23737_d_n11, assign25450_e23737_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25450_e23735: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25450_e23735, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25450_e23737;
        locals.var_xmp_dn0 = assign25450_e23737_d_n0;
        locals.var_xmp_dn2 = assign25450_e23737_d_n2;
        locals.var_xmp_dn4 = assign25450_e23737_d_n4;
        locals.var_xmp_dn5 = assign25450_e23737_d_n5;
        locals.var_xmp_dn6 = assign25450_e23737_d_n6;
        locals.var_xmp_dn7 = assign25450_e23737_d_n7;
        locals.var_xmp_dn8 = assign25450_e23737_d_n8;
        locals.var_xmp_dn9 = assign25450_e23737_d_n9;
        locals.var_xmp_dn10 = assign25450_e23737_d_n10;
        locals.var_xmp_dn11 = assign25450_e23737_d_n11;
        locals.var_xmp_dn14 = assign25450_e23737_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25460_e23749, assign25460_e23749_d_n0, assign25460_e23749_d_n2, assign25460_e23749_d_n4, assign25460_e23749_d_n5, assign25460_e23749_d_n6, assign25460_e23749_d_n7, assign25460_e23749_d_n8, assign25460_e23749_d_n9, assign25460_e23749_d_n10, assign25460_e23749_d_n11, assign25460_e23749_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25460_e23747: f64 = (locals.var_xp + locals.var_xmp);
        (assign25460_e23747, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25460_e23749;
        locals.var_arg_dn0 = assign25460_e23749_d_n0;
        locals.var_arg_dn2 = assign25460_e23749_d_n2;
        locals.var_arg_dn4 = assign25460_e23749_d_n4;
        locals.var_arg_dn5 = assign25460_e23749_d_n5;
        locals.var_arg_dn6 = assign25460_e23749_d_n6;
        locals.var_arg_dn7 = assign25460_e23749_d_n7;
        locals.var_arg_dn8 = assign25460_e23749_d_n8;
        locals.var_arg_dn9 = assign25460_e23749_d_n9;
        locals.var_arg_dn10 = assign25460_e23749_d_n10;
        locals.var_arg_dn11 = assign25460_e23749_d_n11;
        locals.var_arg_dn14 = assign25460_e23749_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25470_e23759, assign25470_e23759_d_n0, assign25470_e23759_d_n2, assign25470_e23759_d_n4, assign25470_e23759_d_n5, assign25470_e23759_d_n6, assign25470_e23759_d_n7, assign25470_e23759_d_n8, assign25470_e23759_d_n9, assign25470_e23759_d_n10, assign25470_e23759_d_n11, assign25470_e23759_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25470_e23759;
        locals.var_dnm_dn0 = assign25470_e23759_d_n0;
        locals.var_dnm_dn2 = assign25470_e23759_d_n2;
        locals.var_dnm_dn4 = assign25470_e23759_d_n4;
        locals.var_dnm_dn5 = assign25470_e23759_d_n5;
        locals.var_dnm_dn6 = assign25470_e23759_d_n6;
        locals.var_dnm_dn7 = assign25470_e23759_d_n7;
        locals.var_dnm_dn8 = assign25470_e23759_d_n8;
        locals.var_dnm_dn9 = assign25470_e23759_d_n9;
        locals.var_dnm_dn10 = assign25470_e23759_d_n10;
        locals.var_dnm_dn11 = assign25470_e23759_d_n11;
        locals.var_dnm_dn14 = assign25470_e23759_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign25480_e23774: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard622 = assign25480_e23774;
        locals.var_guard622_rv = 0.0;

        let assign25490_e23777: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard623 = assign25490_e23777;
        locals.var_guard623_rv = 0.0;

        let (assign25500_e23791,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) && (locals.var_guard623 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25500_e23791;
        locals.var_mm_rv = 0.0;

        let assign25510_e23794: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard624 = assign25510_e23794;
        locals.var_guard624_rv = 0.0;

        let (assign25520_e23811,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard624 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25520_e23811;
        locals.var_mm_rv = 0.0;

        let assign25530_e23814: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign25530_e23814;
        locals.var_guard625_rv = 0.0;

        let (assign25540_e23834,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard624 == 0.0)) && (locals.var_guard625 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25540_e23834;
        locals.var_mm_rv = 0.0;

        let assign25550_e23837: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard626 = assign25550_e23837;
        locals.var_guard626_rv = 0.0;

        let (assign25560_e23860,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard624 == 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard626 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25560_e23860;
        locals.var_mm_rv = 0.0;

        let (assign25570_e23872,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25570_e23872;
        locals.var_m0_rv = 0.0;

        let mut assign25580_loop_guard: usize = 0;
        while {
            let assign25580_cond_e23885: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign25580_cond_e23885 != 0.0
        } {
            assign25580_loop_guard += 1;
            assert!(assign25580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign25580_body0_e23898, assign25580_body0_e23898_d_n0, assign25580_body0_e23898_d_n2, assign25580_body0_e23898_d_n4, assign25580_body0_e23898_d_n5, assign25580_body0_e23898_d_n6, assign25580_body0_e23898_d_n7, assign25580_body0_e23898_d_n8, assign25580_body0_e23898_d_n9, assign25580_body0_e23898_d_n10, assign25580_body0_e23898_d_n11, assign25580_body0_e23898_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) {
        let assign25580_body0_e23896: f64 = (locals.var_dnm).sqrt();
        (assign25580_body0_e23896, (locals.var_dnm_dn0 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn2 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn4 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn5 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn6 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn7 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn8 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn9 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn10 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn11 / (2.0 * assign25580_body0_e23896)), (locals.var_dnm_dn14 / (2.0 * assign25580_body0_e23896)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign25580_body0_e23898;
            locals.var_dnm_dn0 = assign25580_body0_e23898_d_n0;
            locals.var_dnm_dn2 = assign25580_body0_e23898_d_n2;
            locals.var_dnm_dn4 = assign25580_body0_e23898_d_n4;
            locals.var_dnm_dn5 = assign25580_body0_e23898_d_n5;
            locals.var_dnm_dn6 = assign25580_body0_e23898_d_n6;
            locals.var_dnm_dn7 = assign25580_body0_e23898_d_n7;
            locals.var_dnm_dn8 = assign25580_body0_e23898_d_n8;
            locals.var_dnm_dn9 = assign25580_body0_e23898_d_n9;
            locals.var_dnm_dn10 = assign25580_body0_e23898_d_n10;
            locals.var_dnm_dn11 = assign25580_body0_e23898_d_n11;
            locals.var_dnm_dn14 = assign25580_body0_e23898_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign25580_body1_e23912,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 != 0.0)) {
        let assign25580_body1_e23910: f64 = (locals.var_m0 + 1.0);
        (assign25580_body1_e23910,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign25580_body1_e23912;
            locals.var_m0_rv = 0.0;
        }

        let (assign25590_e23936, assign25590_e23936_d_n0, assign25590_e23936_d_n2, assign25590_e23936_d_n4, assign25590_e23936_d_n5, assign25590_e23936_d_n6, assign25590_e23936_d_n7, assign25590_e23936_d_n8, assign25590_e23936_d_n9, assign25590_e23936_d_n10, assign25590_e23936_d_n11, assign25590_e23936_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) && (locals.var_guard622 == 0.0)) {
        let (assign25590_e23934, assign25590_e23934_d_n0, assign25590_e23934_d_n2, assign25590_e23934_d_n4, assign25590_e23934_d_n5, assign25590_e23934_d_n6, assign25590_e23934_d_n7, assign25590_e23934_d_n8, assign25590_e23934_d_n9, assign25590_e23934_d_n10, assign25590_e23934_d_n11, assign25590_e23934_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign25590_e23931: f64 = (2.0 * 2.0);
                let assign25590_e23932: f64 = (1.0 / assign25590_e23931);
                let assign25590_e23933: f64 = (locals.var_dnm).powf(assign25590_e23932);
                (assign25590_e23933, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn0)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn2)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn4)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn5)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn6)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn7)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn8)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn9)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn10)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn11)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25590_e23932) as f64).is_finite() && ((assign25590_e23932) as f64).fract() == 0.0 { if assign25590_e23932 == 0.0 { 0.0 } else { (assign25590_e23932 * ((locals.var_dnm).powf(assign25590_e23932 - 1.0) * locals.var_dnm_dn14)) } } else { (assign25590_e23933 * (assign25590_e23932 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign25590_e23934, assign25590_e23934_d_n0, assign25590_e23934_d_n2, assign25590_e23934_d_n4, assign25590_e23934_d_n5, assign25590_e23934_d_n6, assign25590_e23934_d_n7, assign25590_e23934_d_n8, assign25590_e23934_d_n9, assign25590_e23934_d_n10, assign25590_e23934_d_n11, assign25590_e23934_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25590_e23936;
        locals.var_dnm_dn0 = assign25590_e23936_d_n0;
        locals.var_dnm_dn2 = assign25590_e23936_d_n2;
        locals.var_dnm_dn4 = assign25590_e23936_d_n4;
        locals.var_dnm_dn5 = assign25590_e23936_d_n5;
        locals.var_dnm_dn6 = assign25590_e23936_d_n6;
        locals.var_dnm_dn7 = assign25590_e23936_d_n7;
        locals.var_dnm_dn8 = assign25590_e23936_d_n8;
        locals.var_dnm_dn9 = assign25590_e23936_d_n9;
        locals.var_dnm_dn10 = assign25590_e23936_d_n10;
        locals.var_dnm_dn11 = assign25590_e23936_d_n11;
        locals.var_dnm_dn14 = assign25590_e23936_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25600_e23948, assign25600_e23948_d_n0, assign25600_e23948_d_n2, assign25600_e23948_d_n4, assign25600_e23948_d_n5, assign25600_e23948_d_n6, assign25600_e23948_d_n7, assign25600_e23948_d_n8, assign25600_e23948_d_n9, assign25600_e23948_d_n10, assign25600_e23948_d_n11, assign25600_e23948_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25600_e23946: f64 = (1.0 / locals.var_dnm);
        (assign25600_e23946, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25600_e23948;
        locals.var_dnm_dn0 = assign25600_e23948_d_n0;
        locals.var_dnm_dn2 = assign25600_e23948_d_n2;
        locals.var_dnm_dn4 = assign25600_e23948_d_n4;
        locals.var_dnm_dn5 = assign25600_e23948_d_n5;
        locals.var_dnm_dn6 = assign25600_e23948_d_n6;
        locals.var_dnm_dn7 = assign25600_e23948_d_n7;
        locals.var_dnm_dn8 = assign25600_e23948_d_n8;
        locals.var_dnm_dn9 = assign25600_e23948_d_n9;
        locals.var_dnm_dn10 = assign25600_e23948_d_n10;
        locals.var_dnm_dn11 = assign25600_e23948_d_n11;
        locals.var_dnm_dn14 = assign25600_e23948_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25610_e23962, assign25610_e23962_d_n0, assign25610_e23962_d_n2, assign25610_e23962_d_n4, assign25610_e23962_d_n5, assign25610_e23962_d_n6, assign25610_e23962_d_n7, assign25610_e23962_d_n8, assign25610_e23962_d_n9, assign25610_e23962_d_n10, assign25610_e23962_d_n11, assign25610_e23962_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25610_e23958: f64 = (locals.var_tmf1 * 0.02);
        let assign25610_e23960: f64 = (assign25610_e23958 * locals.var_dnm);
        (assign25610_e23960, (((locals.var_tmf1_dn0 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.02) * locals.var_dnm) + (assign25610_e23958 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign25610_e23962;
        locals.var_tmf0_dn0 = assign25610_e23962_d_n0;
        locals.var_tmf0_dn2 = assign25610_e23962_d_n2;
        locals.var_tmf0_dn4 = assign25610_e23962_d_n4;
        locals.var_tmf0_dn5 = assign25610_e23962_d_n5;
        locals.var_tmf0_dn6 = assign25610_e23962_d_n6;
        locals.var_tmf0_dn7 = assign25610_e23962_d_n7;
        locals.var_tmf0_dn8 = assign25610_e23962_d_n8;
        locals.var_tmf0_dn9 = assign25610_e23962_d_n9;
        locals.var_tmf0_dn10 = assign25610_e23962_d_n10;
        locals.var_tmf0_dn11 = assign25610_e23962_d_n11;
        locals.var_tmf0_dn14 = assign25610_e23962_d_n14;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign25620_e23978, assign25620_e23978_d_n0, assign25620_e23978_d_n2, assign25620_e23978_d_n4, assign25620_e23978_d_n5, assign25620_e23978_d_n6, assign25620_e23978_d_n7, assign25620_e23978_d_n8, assign25620_e23978_d_n9, assign25620_e23978_d_n10, assign25620_e23978_d_n11, assign25620_e23978_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25620_e23972: f64 = (0.02 * locals.var_xmp);
        let assign25620_e23974: f64 = (assign25620_e23972 * locals.var_dnm);
        let assign25620_e23976: f64 = (assign25620_e23974 / locals.var_arg);
        (assign25620_e23976, ((((((0.02 * locals.var_xmp_dn0) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn0)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn2) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn2)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn4) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn4)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn5) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn5)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn6) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn6)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn7) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn7)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn8) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn8)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn9) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn9)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn10) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn10)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn11) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn11)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn14) * locals.var_dnm) + (assign25620_e23972 * locals.var_dnm_dn14)) * locals.var_arg) - (assign25620_e23974 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25620_e23978;
        locals.var_t1_dn0 = assign25620_e23978_d_n0;
        locals.var_t1_dn2 = assign25620_e23978_d_n2;
        locals.var_t1_dn4 = assign25620_e23978_d_n4;
        locals.var_t1_dn5 = assign25620_e23978_d_n5;
        locals.var_t1_dn6 = assign25620_e23978_d_n6;
        locals.var_t1_dn7 = assign25620_e23978_d_n7;
        locals.var_t1_dn8 = assign25620_e23978_d_n8;
        locals.var_t1_dn9 = assign25620_e23978_d_n9;
        locals.var_t1_dn10 = assign25620_e23978_d_n10;
        locals.var_t1_dn11 = assign25620_e23978_d_n11;
        locals.var_t1_dn14 = assign25620_e23978_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25630_e23992, assign25630_e23992_d_n0, assign25630_e23992_d_n2, assign25630_e23992_d_n4, assign25630_e23992_d_n5, assign25630_e23992_d_n6, assign25630_e23992_d_n7, assign25630_e23992_d_n8, assign25630_e23992_d_n9, assign25630_e23992_d_n10, assign25630_e23992_d_n11, assign25630_e23992_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        let assign25630_e23988: f64 = (locals.var_phi_s0_dep - 0.02);
        let assign25630_e23990: f64 = (assign25630_e23988 + locals.var_tmf0);
        (assign25630_e23990, (locals.var_phi_s0_dep_dn0 + locals.var_tmf0_dn0), (locals.var_phi_s0_dep_dn2 + locals.var_tmf0_dn2), (locals.var_phi_s0_dep_dn4 + locals.var_tmf0_dn4), (locals.var_phi_s0_dep_dn5 + locals.var_tmf0_dn5), (locals.var_phi_s0_dep_dn6 + locals.var_tmf0_dn6), (locals.var_phi_s0_dep_dn7 + locals.var_tmf0_dn7), (locals.var_phi_s0_dep_dn8 + locals.var_tmf0_dn8), (locals.var_phi_s0_dep_dn9 + locals.var_tmf0_dn9), (locals.var_phi_s0_dep_dn10 + locals.var_tmf0_dn10), (locals.var_phi_s0_dep_dn11 + locals.var_tmf0_dn11), (locals.var_phi_s0_dep_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign25630_e23992;
        locals.var_phi_b0_dep_dn0 = assign25630_e23992_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25630_e23992_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25630_e23992_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25630_e23992_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25630_e23992_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25630_e23992_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25630_e23992_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25630_e23992_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25630_e23992_d_n10;
        locals.var_phi_b0_dep_dn11 = assign25630_e23992_d_n11;
        locals.var_phi_b0_dep_dn14 = assign25630_e23992_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25640_e24002, assign25640_e24002_d_n0, assign25640_e24002_d_n2, assign25640_e24002_d_n4, assign25640_e24002_d_n5, assign25640_e24002_d_n6, assign25640_e24002_d_n7, assign25640_e24002_d_n8, assign25640_e24002_d_n9, assign25640_e24002_d_n10, assign25640_e24002_d_n11, assign25640_e24002_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25640_e24002;
        locals.var_t1_dn0 = assign25640_e24002_d_n0;
        locals.var_t1_dn2 = assign25640_e24002_d_n2;
        locals.var_t1_dn4 = assign25640_e24002_d_n4;
        locals.var_t1_dn5 = assign25640_e24002_d_n5;
        locals.var_t1_dn6 = assign25640_e24002_d_n6;
        locals.var_t1_dn7 = assign25640_e24002_d_n7;
        locals.var_t1_dn8 = assign25640_e24002_d_n8;
        locals.var_t1_dn9 = assign25640_e24002_d_n9;
        locals.var_t1_dn10 = assign25640_e24002_d_n10;
        locals.var_t1_dn11 = assign25640_e24002_d_n11;
        locals.var_t1_dn14 = assign25640_e24002_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25650_e24013, assign25650_e24013_d_n0, assign25650_e24013_d_n2, assign25650_e24013_d_n4, assign25650_e24013_d_n5, assign25650_e24013_d_n6, assign25650_e24013_d_n7, assign25650_e24013_d_n8, assign25650_e24013_d_n9, assign25650_e24013_d_n10, assign25650_e24013_d_n11, assign25650_e24013_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign25650_e24013;
        locals.var_phi_b0_dep_dn0 = assign25650_e24013_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25650_e24013_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25650_e24013_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25650_e24013_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25650_e24013_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25650_e24013_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25650_e24013_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25650_e24013_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25650_e24013_d_n10;
        locals.var_phi_b0_dep_dn11 = assign25650_e24013_d_n11;
        locals.var_phi_b0_dep_dn14 = assign25650_e24013_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25660_e24024, assign25660_e24024_d_n0, assign25660_e24024_d_n2, assign25660_e24024_d_n4, assign25660_e24024_d_n5, assign25660_e24024_d_n6, assign25660_e24024_d_n7, assign25660_e24024_d_n8, assign25660_e24024_d_n9, assign25660_e24024_d_n10, assign25660_e24024_d_n11, assign25660_e24024_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25660_e24024;
        locals.var_t1_dn0 = assign25660_e24024_d_n0;
        locals.var_t1_dn2 = assign25660_e24024_d_n2;
        locals.var_t1_dn4 = assign25660_e24024_d_n4;
        locals.var_t1_dn5 = assign25660_e24024_d_n5;
        locals.var_t1_dn6 = assign25660_e24024_d_n6;
        locals.var_t1_dn7 = assign25660_e24024_d_n7;
        locals.var_t1_dn8 = assign25660_e24024_d_n8;
        locals.var_t1_dn9 = assign25660_e24024_d_n9;
        locals.var_t1_dn10 = assign25660_e24024_d_n10;
        locals.var_t1_dn11 = assign25660_e24024_d_n11;
        locals.var_t1_dn14 = assign25660_e24024_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25670_e24038, assign25670_e24038_d_n0, assign25670_e24038_d_n2, assign25670_e24038_d_n4, assign25670_e24038_d_n5, assign25670_e24038_d_n6, assign25670_e24038_d_n7, assign25670_e24038_d_n8, assign25670_e24038_d_n9, assign25670_e24038_d_n10, assign25670_e24038_d_n11, assign25670_e24038_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign25670_e24031: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign25670_e24033: f64 = (assign25670_e24031 + locals.var_vbscl__blk439);
        let assign25670_e24035: f64 = (assign25670_e24033 - locals.var_vbi_dep);
        let assign25670_e24036: f64 = (locals.var_ndepmpnsub_inv1 * assign25670_e24035);
        (assign25670_e24036, ((locals.var_ndepmpnsub_inv1_dn0 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0))), ((locals.var_ndepmpnsub_inv1_dn2 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2))), ((locals.var_ndepmpnsub_inv1_dn4 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4))), ((locals.var_ndepmpnsub_inv1_dn5 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5))), ((locals.var_ndepmpnsub_inv1_dn6 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6))), ((locals.var_ndepmpnsub_inv1_dn7 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7))), ((locals.var_ndepmpnsub_inv1_dn8 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8))), ((locals.var_ndepmpnsub_inv1_dn9 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9))), ((locals.var_ndepmpnsub_inv1_dn10 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10))), ((locals.var_ndepmpnsub_inv1_dn11 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11))), ((locals.var_ndepmpnsub_inv1_dn14 * assign25670_e24035) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign25670_e24038;
        locals.var_phi_j0_dep_dn0 = assign25670_e24038_d_n0;
        locals.var_phi_j0_dep_dn2 = assign25670_e24038_d_n2;
        locals.var_phi_j0_dep_dn4 = assign25670_e24038_d_n4;
        locals.var_phi_j0_dep_dn5 = assign25670_e24038_d_n5;
        locals.var_phi_j0_dep_dn6 = assign25670_e24038_d_n6;
        locals.var_phi_j0_dep_dn7 = assign25670_e24038_d_n7;
        locals.var_phi_j0_dep_dn8 = assign25670_e24038_d_n8;
        locals.var_phi_j0_dep_dn9 = assign25670_e24038_d_n9;
        locals.var_phi_j0_dep_dn10 = assign25670_e24038_d_n10;
        locals.var_phi_j0_dep_dn11 = assign25670_e24038_d_n11;
        locals.var_phi_j0_dep_dn14 = assign25670_e24038_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign25680_e24048, assign25680_e24048_d_n0, assign25680_e24048_d_n2, assign25680_e24048_d_n4, assign25680_e24048_d_n5, assign25680_e24048_d_n6, assign25680_e24048_d_n7, assign25680_e24048_d_n8, assign25680_e24048_d_n9, assign25680_e24048_d_n10, assign25680_e24048_d_n11, assign25680_e24048_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign25680_e24045: f64 = (locals.var_phi_s0_dep - locals.var_phi_b0_dep);
        let assign25680_e24046: f64 = (locals.var_beta * assign25680_e24045);
        (assign25680_e24046, ((locals.var_beta_dn0 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn0 - locals.var_phi_b0_dep_dn0))), ((locals.var_beta_dn2 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn2 - locals.var_phi_b0_dep_dn2))), ((locals.var_beta_dn4 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn4 - locals.var_phi_b0_dep_dn4))), ((locals.var_beta_dn5 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn5 - locals.var_phi_b0_dep_dn5))), ((locals.var_beta_dn6 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn6 - locals.var_phi_b0_dep_dn6))), ((locals.var_beta_dn7 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn7 - locals.var_phi_b0_dep_dn7))), ((locals.var_beta_dn8 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn8 - locals.var_phi_b0_dep_dn8))), ((locals.var_beta_dn9 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn9 - locals.var_phi_b0_dep_dn9))), ((locals.var_beta_dn10 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn10 - locals.var_phi_b0_dep_dn10))), ((locals.var_beta_dn11 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn11 - locals.var_phi_b0_dep_dn11))), ((locals.var_beta_dn14 * assign25680_e24045) + (locals.var_beta * (locals.var_phi_s0_dep_dn14 - locals.var_phi_b0_dep_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25680_e24048;
        locals.var_t1_dn0 = assign25680_e24048_d_n0;
        locals.var_t1_dn2 = assign25680_e24048_d_n2;
        locals.var_t1_dn4 = assign25680_e24048_d_n4;
        locals.var_t1_dn5 = assign25680_e24048_d_n5;
        locals.var_t1_dn6 = assign25680_e24048_d_n6;
        locals.var_t1_dn7 = assign25680_e24048_d_n7;
        locals.var_t1_dn8 = assign25680_e24048_d_n8;
        locals.var_t1_dn9 = assign25680_e24048_d_n9;
        locals.var_t1_dn10 = assign25680_e24048_d_n10;
        locals.var_t1_dn11 = assign25680_e24048_d_n11;
        locals.var_t1_dn14 = assign25680_e24048_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25690_e24055, assign25690_e24055_d_n0, assign25690_e24055_d_n2, assign25690_e24055_d_n4, assign25690_e24055_d_n5, assign25690_e24055_d_n6, assign25690_e24055_d_n7, assign25690_e24055_d_n8, assign25690_e24055_d_n9, assign25690_e24055_d_n10, assign25690_e24055_d_n11, assign25690_e24055_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign25690_e24053: f64 = (locals.var_t1).exp();
        (assign25690_e24053, (assign25690_e24053 * locals.var_t1_dn0), (assign25690_e24053 * locals.var_t1_dn2), (assign25690_e24053 * locals.var_t1_dn4), (assign25690_e24053 * locals.var_t1_dn5), (assign25690_e24053 * locals.var_t1_dn6), (assign25690_e24053 * locals.var_t1_dn7), (assign25690_e24053 * locals.var_t1_dn8), (assign25690_e24053 * locals.var_t1_dn9), (assign25690_e24053 * locals.var_t1_dn10), (assign25690_e24053 * locals.var_t1_dn11), (assign25690_e24053 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25690_e24055;
        locals.var_t2_dn0 = assign25690_e24055_d_n0;
        locals.var_t2_dn2 = assign25690_e24055_d_n2;
        locals.var_t2_dn4 = assign25690_e24055_d_n4;
        locals.var_t2_dn5 = assign25690_e24055_d_n5;
        locals.var_t2_dn6 = assign25690_e24055_d_n6;
        locals.var_t2_dn7 = assign25690_e24055_d_n7;
        locals.var_t2_dn8 = assign25690_e24055_d_n8;
        locals.var_t2_dn9 = assign25690_e24055_d_n9;
        locals.var_t2_dn10 = assign25690_e24055_d_n10;
        locals.var_t2_dn11 = assign25690_e24055_d_n11;
        locals.var_t2_dn14 = assign25690_e24055_d_n14;
        locals.var_t2_rv = 0.0;

        let assign25700_e24058: f64 = if locals.var_phi_s0_dep >= locals.var_phi_b0_dep { 1.0 } else { 0.0 };
        locals.var_guard627 = assign25700_e24058;
        locals.var_guard627_rv = 0.0;

        let (assign25710_e24076, assign25710_e24076_d_n0, assign25710_e24076_d_n2, assign25710_e24076_d_n4, assign25710_e24076_d_n5, assign25710_e24076_d_n6, assign25710_e24076_d_n7, assign25710_e24076_d_n8, assign25710_e24076_d_n9, assign25710_e24076_d_n10, assign25710_e24076_d_n11, assign25710_e24076_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign25710_e24065: f64 = (-locals.var_cnst0);
        let assign25710_e24068: f64 = (locals.var_t2 - 1.0);
        let assign25710_e24070: f64 = (assign25710_e24068 - locals.var_t1);
        let assign25710_e24072: f64 = (assign25710_e24070 + 1e-15);
        let assign25710_e24073: f64 = (assign25710_e24072).sqrt();
        let assign25710_e24074: f64 = (assign25710_e24065 * assign25710_e24073);
        (assign25710_e24074, (((-locals.var_cnst0_dn0) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn2) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn4) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn5) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn6) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn7) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn8) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn9) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn10) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn11) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign25710_e24073)))), (((-locals.var_cnst0_dn14) * assign25710_e24073) + (assign25710_e24065 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign25710_e24073)))),)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign25710_e24076;
        locals.var_q_s0_dn0 = assign25710_e24076_d_n0;
        locals.var_q_s0_dn2 = assign25710_e24076_d_n2;
        locals.var_q_s0_dn4 = assign25710_e24076_d_n4;
        locals.var_q_s0_dn5 = assign25710_e24076_d_n5;
        locals.var_q_s0_dn6 = assign25710_e24076_d_n6;
        locals.var_q_s0_dn7 = assign25710_e24076_d_n7;
        locals.var_q_s0_dn8 = assign25710_e24076_d_n8;
        locals.var_q_s0_dn9 = assign25710_e24076_d_n9;
        locals.var_q_s0_dn10 = assign25710_e24076_d_n10;
        locals.var_q_s0_dn11 = assign25710_e24076_d_n11;
        locals.var_q_s0_dn14 = assign25710_e24076_d_n14;
        locals.var_q_s0_rv = 0.0;

        let (assign25720_e24084, assign25720_e24084_d_n0, assign25720_e24084_d_n2, assign25720_e24084_d_n4, assign25720_e24084_d_n5, assign25720_e24084_d_n6, assign25720_e24084_d_n7, assign25720_e24084_d_n8, assign25720_e24084_d_n9, assign25720_e24084_d_n10, assign25720_e24084_d_n11, assign25720_e24084_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    } else {
        (locals.var_q_n0__blk542, locals.var_q_n0__blk542_dn0, locals.var_q_n0__blk542_dn2, locals.var_q_n0__blk542_dn4, locals.var_q_n0__blk542_dn5, locals.var_q_n0__blk542_dn6, locals.var_q_n0__blk542_dn7, locals.var_q_n0__blk542_dn8, locals.var_q_n0__blk542_dn9, locals.var_q_n0__blk542_dn10, locals.var_q_n0__blk542_dn11, locals.var_q_n0__blk542_dn14,)
    }
};
        locals.var_q_n0__blk542 = assign25720_e24084;
        locals.var_q_n0__blk542_dn0 = assign25720_e24084_d_n0;
        locals.var_q_n0__blk542_dn2 = assign25720_e24084_d_n2;
        locals.var_q_n0__blk542_dn4 = assign25720_e24084_d_n4;
        locals.var_q_n0__blk542_dn5 = assign25720_e24084_d_n5;
        locals.var_q_n0__blk542_dn6 = assign25720_e24084_d_n6;
        locals.var_q_n0__blk542_dn7 = assign25720_e24084_d_n7;
        locals.var_q_n0__blk542_dn8 = assign25720_e24084_d_n8;
        locals.var_q_n0__blk542_dn9 = assign25720_e24084_d_n9;
        locals.var_q_n0__blk542_dn10 = assign25720_e24084_d_n10;
        locals.var_q_n0__blk542_dn11 = assign25720_e24084_d_n11;
        locals.var_q_n0__blk542_dn14 = assign25720_e24084_d_n14;
        locals.var_q_n0__blk542_rv = 0.0;

        let (assign25730_e24092, assign25730_e24092_d_n0, assign25730_e24092_d_n2, assign25730_e24092_d_n4, assign25730_e24092_d_n5, assign25730_e24092_d_n6, assign25730_e24092_d_n7, assign25730_e24092_d_n8, assign25730_e24092_d_n9, assign25730_e24092_d_n10, assign25730_e24092_d_n11, assign25730_e24092_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign25730_e24092;
        locals.var_q_s0_dep_dn0 = assign25730_e24092_d_n0;
        locals.var_q_s0_dep_dn2 = assign25730_e24092_d_n2;
        locals.var_q_s0_dep_dn4 = assign25730_e24092_d_n4;
        locals.var_q_s0_dep_dn5 = assign25730_e24092_d_n5;
        locals.var_q_s0_dep_dn6 = assign25730_e24092_d_n6;
        locals.var_q_s0_dep_dn7 = assign25730_e24092_d_n7;
        locals.var_q_s0_dep_dn8 = assign25730_e24092_d_n8;
        locals.var_q_s0_dep_dn9 = assign25730_e24092_d_n9;
        locals.var_q_s0_dep_dn10 = assign25730_e24092_d_n10;
        locals.var_q_s0_dep_dn11 = assign25730_e24092_d_n11;
        locals.var_q_s0_dep_dn14 = assign25730_e24092_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign25740_e24100, assign25740_e24100_d_n0, assign25740_e24100_d_n2, assign25740_e24100_d_n4, assign25740_e24100_d_n5, assign25740_e24100_d_n6, assign25740_e24100_d_n7, assign25740_e24100_d_n8, assign25740_e24100_d_n9, assign25740_e24100_d_n10, assign25740_e24100_d_n11, assign25740_e24100_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    }
};
        locals.var_q_sub0 = assign25740_e24100;
        locals.var_q_sub0_dn0 = assign25740_e24100_d_n0;
        locals.var_q_sub0_dn2 = assign25740_e24100_d_n2;
        locals.var_q_sub0_dn4 = assign25740_e24100_d_n4;
        locals.var_q_sub0_dn5 = assign25740_e24100_d_n5;
        locals.var_q_sub0_dn6 = assign25740_e24100_d_n6;
        locals.var_q_sub0_dn7 = assign25740_e24100_d_n7;
        locals.var_q_sub0_dn8 = assign25740_e24100_d_n8;
        locals.var_q_sub0_dn9 = assign25740_e24100_d_n9;
        locals.var_q_sub0_dn10 = assign25740_e24100_d_n10;
        locals.var_q_sub0_dn11 = assign25740_e24100_d_n11;
        locals.var_q_sub0_dn14 = assign25740_e24100_d_n14;
        locals.var_q_sub0_rv = 0.0;

        let (assign25750_e24113, assign25750_e24113_d_n0, assign25750_e24113_d_n2, assign25750_e24113_d_n4, assign25750_e24113_d_n5, assign25750_e24113_d_n6, assign25750_e24113_d_n7, assign25750_e24113_d_n8, assign25750_e24113_d_n9, assign25750_e24113_d_n10, assign25750_e24113_d_n11, assign25750_e24113_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign25750_e24109: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25750_e24110: f64 = (locals.var_c_2esipq_ndepm * assign25750_e24109);
        let assign25750_e24111: f64 = (assign25750_e24110).sqrt();
        (assign25750_e24111, (((locals.var_c_2esipq_ndepm_dn0 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn2 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn4 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn5 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn6 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn7 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn8 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn9 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn10 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn11 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign25750_e24111)), (((locals.var_c_2esipq_ndepm_dn14 * assign25750_e24109) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign25750_e24111)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign25750_e24113;
        locals.var_w_b0_dn0 = assign25750_e24113_d_n0;
        locals.var_w_b0_dn2 = assign25750_e24113_d_n2;
        locals.var_w_b0_dn4 = assign25750_e24113_d_n4;
        locals.var_w_b0_dn5 = assign25750_e24113_d_n5;
        locals.var_w_b0_dn6 = assign25750_e24113_d_n6;
        locals.var_w_b0_dn7 = assign25750_e24113_d_n7;
        locals.var_w_b0_dn8 = assign25750_e24113_d_n8;
        locals.var_w_b0_dn9 = assign25750_e24113_d_n9;
        locals.var_w_b0_dn10 = assign25750_e24113_d_n10;
        locals.var_w_b0_dn11 = assign25750_e24113_d_n11;
        locals.var_w_b0_dn14 = assign25750_e24113_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign25760_e24117: f64 = (locals.var_uc_depthn - 1e-8);
        let assign25760_e24122: f64 = if ((locals.var_w_b0 > assign25760_e24117) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard628 = assign25760_e24122;
        locals.var_guard628_rv = 0.0;

        let (assign25770_e24136, assign25770_e24136_d_n0, assign25770_e24136_d_n2, assign25770_e24136_d_n4, assign25770_e24136_d_n5, assign25770_e24136_d_n6, assign25770_e24136_d_n7, assign25770_e24136_d_n8, assign25770_e24136_d_n9, assign25770_e24136_d_n10, assign25770_e24136_d_n11, assign25770_e24136_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25770_e24132: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign25770_e24134: f64 = (assign25770_e24132 + 1e-8);
        (assign25770_e24134, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign25770_e24136;
        locals.var_tmf1_dn0 = assign25770_e24136_d_n0;
        locals.var_tmf1_dn2 = assign25770_e24136_d_n2;
        locals.var_tmf1_dn4 = assign25770_e24136_d_n4;
        locals.var_tmf1_dn5 = assign25770_e24136_d_n5;
        locals.var_tmf1_dn6 = assign25770_e24136_d_n6;
        locals.var_tmf1_dn7 = assign25770_e24136_d_n7;
        locals.var_tmf1_dn8 = assign25770_e24136_d_n8;
        locals.var_tmf1_dn9 = assign25770_e24136_d_n9;
        locals.var_tmf1_dn10 = assign25770_e24136_d_n10;
        locals.var_tmf1_dn11 = assign25770_e24136_d_n11;
        locals.var_tmf1_dn14 = assign25770_e24136_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign25780_e24148, assign25780_e24148_d_n0, assign25780_e24148_d_n2, assign25780_e24148_d_n4, assign25780_e24148_d_n5, assign25780_e24148_d_n6, assign25780_e24148_d_n7, assign25780_e24148_d_n8, assign25780_e24148_d_n9, assign25780_e24148_d_n10, assign25780_e24148_d_n11, assign25780_e24148_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25780_e24146: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25780_e24146, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign25780_e24148;
        locals.var_x2_dn0 = assign25780_e24148_d_n0;
        locals.var_x2_dn2 = assign25780_e24148_d_n2;
        locals.var_x2_dn4 = assign25780_e24148_d_n4;
        locals.var_x2_dn5 = assign25780_e24148_d_n5;
        locals.var_x2_dn6 = assign25780_e24148_d_n6;
        locals.var_x2_dn7 = assign25780_e24148_d_n7;
        locals.var_x2_dn8 = assign25780_e24148_d_n8;
        locals.var_x2_dn9 = assign25780_e24148_d_n9;
        locals.var_x2_dn10 = assign25780_e24148_d_n10;
        locals.var_x2_dn11 = assign25780_e24148_d_n11;
        locals.var_x2_dn14 = assign25780_e24148_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign25790_e24160, assign25790_e24160_d_n0, assign25790_e24160_d_n2, assign25790_e24160_d_n4, assign25790_e24160_d_n5, assign25790_e24160_d_n6, assign25790_e24160_d_n7, assign25790_e24160_d_n8, assign25790_e24160_d_n9, assign25790_e24160_d_n10, assign25790_e24160_d_n11, assign25790_e24160_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25790_e24158: f64 = (1e-8 * 1e-8);
        (assign25790_e24158, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign25790_e24160;
        locals.var_xmax2_dn0 = assign25790_e24160_d_n0;
        locals.var_xmax2_dn2 = assign25790_e24160_d_n2;
        locals.var_xmax2_dn4 = assign25790_e24160_d_n4;
        locals.var_xmax2_dn5 = assign25790_e24160_d_n5;
        locals.var_xmax2_dn6 = assign25790_e24160_d_n6;
        locals.var_xmax2_dn7 = assign25790_e24160_d_n7;
        locals.var_xmax2_dn8 = assign25790_e24160_d_n8;
        locals.var_xmax2_dn9 = assign25790_e24160_d_n9;
        locals.var_xmax2_dn10 = assign25790_e24160_d_n10;
        locals.var_xmax2_dn11 = assign25790_e24160_d_n11;
        locals.var_xmax2_dn14 = assign25790_e24160_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign25800_e24170, assign25800_e24170_d_n0, assign25800_e24170_d_n2, assign25800_e24170_d_n4, assign25800_e24170_d_n5, assign25800_e24170_d_n6, assign25800_e24170_d_n7, assign25800_e24170_d_n8, assign25800_e24170_d_n9, assign25800_e24170_d_n10, assign25800_e24170_d_n11, assign25800_e24170_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25800_e24170;
        locals.var_xp_dn0 = assign25800_e24170_d_n0;
        locals.var_xp_dn2 = assign25800_e24170_d_n2;
        locals.var_xp_dn4 = assign25800_e24170_d_n4;
        locals.var_xp_dn5 = assign25800_e24170_d_n5;
        locals.var_xp_dn6 = assign25800_e24170_d_n6;
        locals.var_xp_dn7 = assign25800_e24170_d_n7;
        locals.var_xp_dn8 = assign25800_e24170_d_n8;
        locals.var_xp_dn9 = assign25800_e24170_d_n9;
        locals.var_xp_dn10 = assign25800_e24170_d_n10;
        locals.var_xp_dn11 = assign25800_e24170_d_n11;
        locals.var_xp_dn14 = assign25800_e24170_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25810_e24180, assign25810_e24180_d_n0, assign25810_e24180_d_n2, assign25810_e24180_d_n4, assign25810_e24180_d_n5, assign25810_e24180_d_n6, assign25810_e24180_d_n7, assign25810_e24180_d_n8, assign25810_e24180_d_n9, assign25810_e24180_d_n10, assign25810_e24180_d_n11, assign25810_e24180_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25810_e24180;
        locals.var_xmp_dn0 = assign25810_e24180_d_n0;
        locals.var_xmp_dn2 = assign25810_e24180_d_n2;
        locals.var_xmp_dn4 = assign25810_e24180_d_n4;
        locals.var_xmp_dn5 = assign25810_e24180_d_n5;
        locals.var_xmp_dn6 = assign25810_e24180_d_n6;
        locals.var_xmp_dn7 = assign25810_e24180_d_n7;
        locals.var_xmp_dn8 = assign25810_e24180_d_n8;
        locals.var_xmp_dn9 = assign25810_e24180_d_n9;
        locals.var_xmp_dn10 = assign25810_e24180_d_n10;
        locals.var_xmp_dn11 = assign25810_e24180_d_n11;
        locals.var_xmp_dn14 = assign25810_e24180_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25820_e24190,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25820_e24190;
        locals.var_m0_rv = 0.0;

        let (assign25830_e24200,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25830_e24200;
        locals.var_mm_rv = 0.0;

        let (assign25840_e24210, assign25840_e24210_d_n0, assign25840_e24210_d_n2, assign25840_e24210_d_n4, assign25840_e24210_d_n5, assign25840_e24210_d_n6, assign25840_e24210_d_n7, assign25840_e24210_d_n8, assign25840_e24210_d_n9, assign25840_e24210_d_n10, assign25840_e24210_d_n11, assign25840_e24210_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25840_e24210;
        locals.var_arg_dn0 = assign25840_e24210_d_n0;
        locals.var_arg_dn2 = assign25840_e24210_d_n2;
        locals.var_arg_dn4 = assign25840_e24210_d_n4;
        locals.var_arg_dn5 = assign25840_e24210_d_n5;
        locals.var_arg_dn6 = assign25840_e24210_d_n6;
        locals.var_arg_dn7 = assign25840_e24210_d_n7;
        locals.var_arg_dn8 = assign25840_e24210_d_n8;
        locals.var_arg_dn9 = assign25840_e24210_d_n9;
        locals.var_arg_dn10 = assign25840_e24210_d_n10;
        locals.var_arg_dn11 = assign25840_e24210_d_n11;
        locals.var_arg_dn14 = assign25840_e24210_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25850_e24220, assign25850_e24220_d_n0, assign25850_e24220_d_n2, assign25850_e24220_d_n4, assign25850_e24220_d_n5, assign25850_e24220_d_n6, assign25850_e24220_d_n7, assign25850_e24220_d_n8, assign25850_e24220_d_n9, assign25850_e24220_d_n10, assign25850_e24220_d_n11, assign25850_e24220_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25850_e24220;
        locals.var_dnm_dn0 = assign25850_e24220_d_n0;
        locals.var_dnm_dn2 = assign25850_e24220_d_n2;
        locals.var_dnm_dn4 = assign25850_e24220_d_n4;
        locals.var_dnm_dn5 = assign25850_e24220_d_n5;
        locals.var_dnm_dn6 = assign25850_e24220_d_n6;
        locals.var_dnm_dn7 = assign25850_e24220_d_n7;
        locals.var_dnm_dn8 = assign25850_e24220_d_n8;
        locals.var_dnm_dn9 = assign25850_e24220_d_n9;
        locals.var_dnm_dn10 = assign25850_e24220_d_n10;
        locals.var_dnm_dn11 = assign25850_e24220_d_n11;
        locals.var_dnm_dn14 = assign25850_e24220_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25860_e24232, assign25860_e24232_d_n0, assign25860_e24232_d_n2, assign25860_e24232_d_n4, assign25860_e24232_d_n5, assign25860_e24232_d_n6, assign25860_e24232_d_n7, assign25860_e24232_d_n8, assign25860_e24232_d_n9, assign25860_e24232_d_n10, assign25860_e24232_d_n11, assign25860_e24232_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25860_e24230: f64 = (locals.var_xp * locals.var_x2);
        (assign25860_e24230, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25860_e24232;
        locals.var_xp_dn0 = assign25860_e24232_d_n0;
        locals.var_xp_dn2 = assign25860_e24232_d_n2;
        locals.var_xp_dn4 = assign25860_e24232_d_n4;
        locals.var_xp_dn5 = assign25860_e24232_d_n5;
        locals.var_xp_dn6 = assign25860_e24232_d_n6;
        locals.var_xp_dn7 = assign25860_e24232_d_n7;
        locals.var_xp_dn8 = assign25860_e24232_d_n8;
        locals.var_xp_dn9 = assign25860_e24232_d_n9;
        locals.var_xp_dn10 = assign25860_e24232_d_n10;
        locals.var_xp_dn11 = assign25860_e24232_d_n11;
        locals.var_xp_dn14 = assign25860_e24232_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign25870_e24244, assign25870_e24244_d_n0, assign25870_e24244_d_n2, assign25870_e24244_d_n4, assign25870_e24244_d_n5, assign25870_e24244_d_n6, assign25870_e24244_d_n7, assign25870_e24244_d_n8, assign25870_e24244_d_n9, assign25870_e24244_d_n10, assign25870_e24244_d_n11, assign25870_e24244_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25870_e24242: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25870_e24242, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25870_e24244;
        locals.var_xmp_dn0 = assign25870_e24244_d_n0;
        locals.var_xmp_dn2 = assign25870_e24244_d_n2;
        locals.var_xmp_dn4 = assign25870_e24244_d_n4;
        locals.var_xmp_dn5 = assign25870_e24244_d_n5;
        locals.var_xmp_dn6 = assign25870_e24244_d_n6;
        locals.var_xmp_dn7 = assign25870_e24244_d_n7;
        locals.var_xmp_dn8 = assign25870_e24244_d_n8;
        locals.var_xmp_dn9 = assign25870_e24244_d_n9;
        locals.var_xmp_dn10 = assign25870_e24244_d_n10;
        locals.var_xmp_dn11 = assign25870_e24244_d_n11;
        locals.var_xmp_dn14 = assign25870_e24244_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25880_e24256, assign25880_e24256_d_n0, assign25880_e24256_d_n2, assign25880_e24256_d_n4, assign25880_e24256_d_n5, assign25880_e24256_d_n6, assign25880_e24256_d_n7, assign25880_e24256_d_n8, assign25880_e24256_d_n9, assign25880_e24256_d_n10, assign25880_e24256_d_n11, assign25880_e24256_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25880_e24254: f64 = (locals.var_xp * locals.var_x2);
        (assign25880_e24254, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25880_e24256;
        locals.var_xp_dn0 = assign25880_e24256_d_n0;
        locals.var_xp_dn2 = assign25880_e24256_d_n2;
        locals.var_xp_dn4 = assign25880_e24256_d_n4;
        locals.var_xp_dn5 = assign25880_e24256_d_n5;
        locals.var_xp_dn6 = assign25880_e24256_d_n6;
        locals.var_xp_dn7 = assign25880_e24256_d_n7;
        locals.var_xp_dn8 = assign25880_e24256_d_n8;
        locals.var_xp_dn9 = assign25880_e24256_d_n9;
        locals.var_xp_dn10 = assign25880_e24256_d_n10;
        locals.var_xp_dn11 = assign25880_e24256_d_n11;
        locals.var_xp_dn14 = assign25880_e24256_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25890_e24268, assign25890_e24268_d_n0, assign25890_e24268_d_n2, assign25890_e24268_d_n4, assign25890_e24268_d_n5, assign25890_e24268_d_n6, assign25890_e24268_d_n7, assign25890_e24268_d_n8, assign25890_e24268_d_n9, assign25890_e24268_d_n10, assign25890_e24268_d_n11, assign25890_e24268_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25890_e24266: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25890_e24266, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25890_e24268;
        locals.var_xmp_dn0 = assign25890_e24268_d_n0;
        locals.var_xmp_dn2 = assign25890_e24268_d_n2;
        locals.var_xmp_dn4 = assign25890_e24268_d_n4;
        locals.var_xmp_dn5 = assign25890_e24268_d_n5;
        locals.var_xmp_dn6 = assign25890_e24268_d_n6;
        locals.var_xmp_dn7 = assign25890_e24268_d_n7;
        locals.var_xmp_dn8 = assign25890_e24268_d_n8;
        locals.var_xmp_dn9 = assign25890_e24268_d_n9;
        locals.var_xmp_dn10 = assign25890_e24268_d_n10;
        locals.var_xmp_dn11 = assign25890_e24268_d_n11;
        locals.var_xmp_dn14 = assign25890_e24268_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25900_e24280, assign25900_e24280_d_n0, assign25900_e24280_d_n2, assign25900_e24280_d_n4, assign25900_e24280_d_n5, assign25900_e24280_d_n6, assign25900_e24280_d_n7, assign25900_e24280_d_n8, assign25900_e24280_d_n9, assign25900_e24280_d_n10, assign25900_e24280_d_n11, assign25900_e24280_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25900_e24278: f64 = (locals.var_xp + locals.var_xmp);
        (assign25900_e24278, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25900_e24280;
        locals.var_arg_dn0 = assign25900_e24280_d_n0;
        locals.var_arg_dn2 = assign25900_e24280_d_n2;
        locals.var_arg_dn4 = assign25900_e24280_d_n4;
        locals.var_arg_dn5 = assign25900_e24280_d_n5;
        locals.var_arg_dn6 = assign25900_e24280_d_n6;
        locals.var_arg_dn7 = assign25900_e24280_d_n7;
        locals.var_arg_dn8 = assign25900_e24280_d_n8;
        locals.var_arg_dn9 = assign25900_e24280_d_n9;
        locals.var_arg_dn10 = assign25900_e24280_d_n10;
        locals.var_arg_dn11 = assign25900_e24280_d_n11;
        locals.var_arg_dn14 = assign25900_e24280_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25910_e24290, assign25910_e24290_d_n0, assign25910_e24290_d_n2, assign25910_e24290_d_n4, assign25910_e24290_d_n5, assign25910_e24290_d_n6, assign25910_e24290_d_n7, assign25910_e24290_d_n8, assign25910_e24290_d_n9, assign25910_e24290_d_n10, assign25910_e24290_d_n11, assign25910_e24290_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25910_e24290;
        locals.var_dnm_dn0 = assign25910_e24290_d_n0;
        locals.var_dnm_dn2 = assign25910_e24290_d_n2;
        locals.var_dnm_dn4 = assign25910_e24290_d_n4;
        locals.var_dnm_dn5 = assign25910_e24290_d_n5;
        locals.var_dnm_dn6 = assign25910_e24290_d_n6;
        locals.var_dnm_dn7 = assign25910_e24290_d_n7;
        locals.var_dnm_dn8 = assign25910_e24290_d_n8;
        locals.var_dnm_dn9 = assign25910_e24290_d_n9;
        locals.var_dnm_dn10 = assign25910_e24290_d_n10;
        locals.var_dnm_dn11 = assign25910_e24290_d_n11;
        locals.var_dnm_dn14 = assign25910_e24290_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign25920_e24305: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard629 = assign25920_e24305;
        locals.var_guard629_rv = 0.0;

        let assign25930_e24308: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign25930_e24308;
        locals.var_guard630_rv = 0.0;

        let (assign25940_e24322,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) && (locals.var_guard630 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25940_e24322;
        locals.var_mm_rv = 0.0;

        let assign25950_e24325: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign25950_e24325;
        locals.var_guard631_rv = 0.0;

        let (assign25960_e24342,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25960_e24342;
        locals.var_mm_rv = 0.0;

        let assign25970_e24345: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign25970_e24345;
        locals.var_guard632_rv = 0.0;

        let (assign25980_e24365,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25980_e24365;
        locals.var_mm_rv = 0.0;

        let assign25990_e24368: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign25990_e24368;
        locals.var_guard633_rv = 0.0;

        let (assign26000_e24391,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26000_e24391;
        locals.var_mm_rv = 0.0;

        let (assign26010_e24403,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26010_e24403;
        locals.var_m0_rv = 0.0;

        let mut assign26020_loop_guard: usize = 0;
        while {
            let assign26020_cond_e24416: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26020_cond_e24416 != 0.0
        } {
            assign26020_loop_guard += 1;
            assert!(assign26020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26020_body0_e24429, assign26020_body0_e24429_d_n0, assign26020_body0_e24429_d_n2, assign26020_body0_e24429_d_n4, assign26020_body0_e24429_d_n5, assign26020_body0_e24429_d_n6, assign26020_body0_e24429_d_n7, assign26020_body0_e24429_d_n8, assign26020_body0_e24429_d_n9, assign26020_body0_e24429_d_n10, assign26020_body0_e24429_d_n11, assign26020_body0_e24429_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign26020_body0_e24427: f64 = (locals.var_dnm).sqrt();
        (assign26020_body0_e24427, (locals.var_dnm_dn0 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn2 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn4 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn5 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn6 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn7 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn8 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn9 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn10 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn11 / (2.0 * assign26020_body0_e24427)), (locals.var_dnm_dn14 / (2.0 * assign26020_body0_e24427)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign26020_body0_e24429;
            locals.var_dnm_dn0 = assign26020_body0_e24429_d_n0;
            locals.var_dnm_dn2 = assign26020_body0_e24429_d_n2;
            locals.var_dnm_dn4 = assign26020_body0_e24429_d_n4;
            locals.var_dnm_dn5 = assign26020_body0_e24429_d_n5;
            locals.var_dnm_dn6 = assign26020_body0_e24429_d_n6;
            locals.var_dnm_dn7 = assign26020_body0_e24429_d_n7;
            locals.var_dnm_dn8 = assign26020_body0_e24429_d_n8;
            locals.var_dnm_dn9 = assign26020_body0_e24429_d_n9;
            locals.var_dnm_dn10 = assign26020_body0_e24429_d_n10;
            locals.var_dnm_dn11 = assign26020_body0_e24429_d_n11;
            locals.var_dnm_dn14 = assign26020_body0_e24429_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign26020_body1_e24443,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign26020_body1_e24441: f64 = (locals.var_m0 + 1.0);
        (assign26020_body1_e24441,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26020_body1_e24443;
            locals.var_m0_rv = 0.0;
        }

        let (assign26030_e24467, assign26030_e24467_d_n0, assign26030_e24467_d_n2, assign26030_e24467_d_n4, assign26030_e24467_d_n5, assign26030_e24467_d_n6, assign26030_e24467_d_n7, assign26030_e24467_d_n8, assign26030_e24467_d_n9, assign26030_e24467_d_n10, assign26030_e24467_d_n11, assign26030_e24467_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 == 0.0)) {
        let (assign26030_e24465, assign26030_e24465_d_n0, assign26030_e24465_d_n2, assign26030_e24465_d_n4, assign26030_e24465_d_n5, assign26030_e24465_d_n6, assign26030_e24465_d_n7, assign26030_e24465_d_n8, assign26030_e24465_d_n9, assign26030_e24465_d_n10, assign26030_e24465_d_n11, assign26030_e24465_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26030_e24462: f64 = (2.0 * 2.0);
                let assign26030_e24463: f64 = (1.0 / assign26030_e24462);
                let assign26030_e24464: f64 = (locals.var_dnm).powf(assign26030_e24463);
                (assign26030_e24464, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn11)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26030_e24463) as f64).is_finite() && ((assign26030_e24463) as f64).fract() == 0.0 { if assign26030_e24463 == 0.0 { 0.0 } else { (assign26030_e24463 * ((locals.var_dnm).powf(assign26030_e24463 - 1.0) * locals.var_dnm_dn14)) } } else { (assign26030_e24464 * (assign26030_e24463 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign26030_e24465, assign26030_e24465_d_n0, assign26030_e24465_d_n2, assign26030_e24465_d_n4, assign26030_e24465_d_n5, assign26030_e24465_d_n6, assign26030_e24465_d_n7, assign26030_e24465_d_n8, assign26030_e24465_d_n9, assign26030_e24465_d_n10, assign26030_e24465_d_n11, assign26030_e24465_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26030_e24467;
        locals.var_dnm_dn0 = assign26030_e24467_d_n0;
        locals.var_dnm_dn2 = assign26030_e24467_d_n2;
        locals.var_dnm_dn4 = assign26030_e24467_d_n4;
        locals.var_dnm_dn5 = assign26030_e24467_d_n5;
        locals.var_dnm_dn6 = assign26030_e24467_d_n6;
        locals.var_dnm_dn7 = assign26030_e24467_d_n7;
        locals.var_dnm_dn8 = assign26030_e24467_d_n8;
        locals.var_dnm_dn9 = assign26030_e24467_d_n9;
        locals.var_dnm_dn10 = assign26030_e24467_d_n10;
        locals.var_dnm_dn11 = assign26030_e24467_d_n11;
        locals.var_dnm_dn14 = assign26030_e24467_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26040_e24479, assign26040_e24479_d_n0, assign26040_e24479_d_n2, assign26040_e24479_d_n4, assign26040_e24479_d_n5, assign26040_e24479_d_n6, assign26040_e24479_d_n7, assign26040_e24479_d_n8, assign26040_e24479_d_n9, assign26040_e24479_d_n10, assign26040_e24479_d_n11, assign26040_e24479_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign26040_e24477: f64 = (1.0 / locals.var_dnm);
        (assign26040_e24477, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26040_e24479;
        locals.var_dnm_dn0 = assign26040_e24479_d_n0;
        locals.var_dnm_dn2 = assign26040_e24479_d_n2;
        locals.var_dnm_dn4 = assign26040_e24479_d_n4;
        locals.var_dnm_dn5 = assign26040_e24479_d_n5;
        locals.var_dnm_dn6 = assign26040_e24479_d_n6;
        locals.var_dnm_dn7 = assign26040_e24479_d_n7;
        locals.var_dnm_dn8 = assign26040_e24479_d_n8;
        locals.var_dnm_dn9 = assign26040_e24479_d_n9;
        locals.var_dnm_dn10 = assign26040_e24479_d_n10;
        locals.var_dnm_dn11 = assign26040_e24479_d_n11;
        locals.var_dnm_dn14 = assign26040_e24479_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26050_e24493, assign26050_e24493_d_n0, assign26050_e24493_d_n2, assign26050_e24493_d_n4, assign26050_e24493_d_n5, assign26050_e24493_d_n6, assign26050_e24493_d_n7, assign26050_e24493_d_n8, assign26050_e24493_d_n9, assign26050_e24493_d_n10, assign26050_e24493_d_n11, assign26050_e24493_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign26050_e24489: f64 = (locals.var_tmf1 * 1e-8);
        let assign26050_e24491: f64 = (assign26050_e24489 * locals.var_dnm);
        (assign26050_e24491, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign26050_e24489 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign26050_e24493;
        locals.var_tmf0_dn0 = assign26050_e24493_d_n0;
        locals.var_tmf0_dn2 = assign26050_e24493_d_n2;
        locals.var_tmf0_dn4 = assign26050_e24493_d_n4;
        locals.var_tmf0_dn5 = assign26050_e24493_d_n5;
        locals.var_tmf0_dn6 = assign26050_e24493_d_n6;
        locals.var_tmf0_dn7 = assign26050_e24493_d_n7;
        locals.var_tmf0_dn8 = assign26050_e24493_d_n8;
        locals.var_tmf0_dn9 = assign26050_e24493_d_n9;
        locals.var_tmf0_dn10 = assign26050_e24493_d_n10;
        locals.var_tmf0_dn11 = assign26050_e24493_d_n11;
        locals.var_tmf0_dn14 = assign26050_e24493_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign26060_e24509, assign26060_e24509_d_n0, assign26060_e24509_d_n2, assign26060_e24509_d_n4, assign26060_e24509_d_n5, assign26060_e24509_d_n6, assign26060_e24509_d_n7, assign26060_e24509_d_n8, assign26060_e24509_d_n9, assign26060_e24509_d_n10, assign26060_e24509_d_n11, assign26060_e24509_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign26060_e24503: f64 = (1e-8 * locals.var_xmp);
        let assign26060_e24505: f64 = (assign26060_e24503 * locals.var_dnm);
        let assign26060_e24507: f64 = (assign26060_e24505 / locals.var_arg);
        (assign26060_e24507, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn11)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign26060_e24503 * locals.var_dnm_dn14)) * locals.var_arg) - (assign26060_e24505 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26060_e24509;
        locals.var_t3_dn0 = assign26060_e24509_d_n0;
        locals.var_t3_dn2 = assign26060_e24509_d_n2;
        locals.var_t3_dn4 = assign26060_e24509_d_n4;
        locals.var_t3_dn5 = assign26060_e24509_d_n5;
        locals.var_t3_dn6 = assign26060_e24509_d_n6;
        locals.var_t3_dn7 = assign26060_e24509_d_n7;
        locals.var_t3_dn8 = assign26060_e24509_d_n8;
        locals.var_t3_dn9 = assign26060_e24509_d_n9;
        locals.var_t3_dn10 = assign26060_e24509_d_n10;
        locals.var_t3_dn11 = assign26060_e24509_d_n11;
        locals.var_t3_dn14 = assign26060_e24509_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26070_e24523, assign26070_e24523_d_n0, assign26070_e24523_d_n2, assign26070_e24523_d_n4, assign26070_e24523_d_n5, assign26070_e24523_d_n6, assign26070_e24523_d_n7, assign26070_e24523_d_n8, assign26070_e24523_d_n9, assign26070_e24523_d_n10, assign26070_e24523_d_n11, assign26070_e24523_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign26070_e24519: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26070_e24521: f64 = (assign26070_e24519 + locals.var_tmf0);
        (assign26070_e24521, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26070_e24523;
        locals.var_w_b0_dn0 = assign26070_e24523_d_n0;
        locals.var_w_b0_dn2 = assign26070_e24523_d_n2;
        locals.var_w_b0_dn4 = assign26070_e24523_d_n4;
        locals.var_w_b0_dn5 = assign26070_e24523_d_n5;
        locals.var_w_b0_dn6 = assign26070_e24523_d_n6;
        locals.var_w_b0_dn7 = assign26070_e24523_d_n7;
        locals.var_w_b0_dn8 = assign26070_e24523_d_n8;
        locals.var_w_b0_dn9 = assign26070_e24523_d_n9;
        locals.var_w_b0_dn10 = assign26070_e24523_d_n10;
        locals.var_w_b0_dn11 = assign26070_e24523_d_n11;
        locals.var_w_b0_dn14 = assign26070_e24523_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26080_e24533, assign26080_e24533_d_n0, assign26080_e24533_d_n2, assign26080_e24533_d_n4, assign26080_e24533_d_n5, assign26080_e24533_d_n6, assign26080_e24533_d_n7, assign26080_e24533_d_n8, assign26080_e24533_d_n9, assign26080_e24533_d_n10, assign26080_e24533_d_n11, assign26080_e24533_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26080_e24533;
        locals.var_t3_dn0 = assign26080_e24533_d_n0;
        locals.var_t3_dn2 = assign26080_e24533_d_n2;
        locals.var_t3_dn4 = assign26080_e24533_d_n4;
        locals.var_t3_dn5 = assign26080_e24533_d_n5;
        locals.var_t3_dn6 = assign26080_e24533_d_n6;
        locals.var_t3_dn7 = assign26080_e24533_d_n7;
        locals.var_t3_dn8 = assign26080_e24533_d_n8;
        locals.var_t3_dn9 = assign26080_e24533_d_n9;
        locals.var_t3_dn10 = assign26080_e24533_d_n10;
        locals.var_t3_dn11 = assign26080_e24533_d_n11;
        locals.var_t3_dn14 = assign26080_e24533_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26090_e24544, assign26090_e24544_d_n0, assign26090_e24544_d_n2, assign26090_e24544_d_n4, assign26090_e24544_d_n5, assign26090_e24544_d_n6, assign26090_e24544_d_n7, assign26090_e24544_d_n8, assign26090_e24544_d_n9, assign26090_e24544_d_n10, assign26090_e24544_d_n11, assign26090_e24544_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26090_e24544;
        locals.var_w_b0_dn0 = assign26090_e24544_d_n0;
        locals.var_w_b0_dn2 = assign26090_e24544_d_n2;
        locals.var_w_b0_dn4 = assign26090_e24544_d_n4;
        locals.var_w_b0_dn5 = assign26090_e24544_d_n5;
        locals.var_w_b0_dn6 = assign26090_e24544_d_n6;
        locals.var_w_b0_dn7 = assign26090_e24544_d_n7;
        locals.var_w_b0_dn8 = assign26090_e24544_d_n8;
        locals.var_w_b0_dn9 = assign26090_e24544_d_n9;
        locals.var_w_b0_dn10 = assign26090_e24544_d_n10;
        locals.var_w_b0_dn11 = assign26090_e24544_d_n11;
        locals.var_w_b0_dn14 = assign26090_e24544_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26100_e24555, assign26100_e24555_d_n0, assign26100_e24555_d_n2, assign26100_e24555_d_n4, assign26100_e24555_d_n5, assign26100_e24555_d_n6, assign26100_e24555_d_n7, assign26100_e24555_d_n8, assign26100_e24555_d_n9, assign26100_e24555_d_n10, assign26100_e24555_d_n11, assign26100_e24555_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26100_e24555;
        locals.var_t3_dn0 = assign26100_e24555_d_n0;
        locals.var_t3_dn2 = assign26100_e24555_d_n2;
        locals.var_t3_dn4 = assign26100_e24555_d_n4;
        locals.var_t3_dn5 = assign26100_e24555_d_n5;
        locals.var_t3_dn6 = assign26100_e24555_d_n6;
        locals.var_t3_dn7 = assign26100_e24555_d_n7;
        locals.var_t3_dn8 = assign26100_e24555_d_n8;
        locals.var_t3_dn9 = assign26100_e24555_d_n9;
        locals.var_t3_dn10 = assign26100_e24555_d_n10;
        locals.var_t3_dn11 = assign26100_e24555_d_n11;
        locals.var_t3_dn14 = assign26100_e24555_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26110_e24570, assign26110_e24570_d_n0, assign26110_e24570_d_n2, assign26110_e24570_d_n4, assign26110_e24570_d_n5, assign26110_e24570_d_n6, assign26110_e24570_d_n7, assign26110_e24570_d_n8, assign26110_e24570_d_n9, assign26110_e24570_d_n10, assign26110_e24570_d_n11, assign26110_e24570_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign26110_e24564: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk439);
        let assign26110_e24566: f64 = (assign26110_e24564 + locals.var_vbi_dep);
        let assign26110_e24567: f64 = (locals.var_c_2esipq_nsub * assign26110_e24566);
        let assign26110_e24568: f64 = (assign26110_e24567).sqrt();
        (assign26110_e24568, (((locals.var_c_2esipq_nsub_dn0 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn2 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn4 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn5 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn6 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn7 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn8 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn9 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn10 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn11 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign26110_e24568)), (((locals.var_c_2esipq_nsub_dn14 * assign26110_e24566) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign26110_e24568)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign26110_e24570;
        locals.var_w_sub0_dn0 = assign26110_e24570_d_n0;
        locals.var_w_sub0_dn2 = assign26110_e24570_d_n2;
        locals.var_w_sub0_dn4 = assign26110_e24570_d_n4;
        locals.var_w_sub0_dn5 = assign26110_e24570_d_n5;
        locals.var_w_sub0_dn6 = assign26110_e24570_d_n6;
        locals.var_w_sub0_dn7 = assign26110_e24570_d_n7;
        locals.var_w_sub0_dn8 = assign26110_e24570_d_n8;
        locals.var_w_sub0_dn9 = assign26110_e24570_d_n9;
        locals.var_w_sub0_dn10 = assign26110_e24570_d_n10;
        locals.var_w_sub0_dn11 = assign26110_e24570_d_n11;
        locals.var_w_sub0_dn14 = assign26110_e24570_d_n14;
        locals.var_w_sub0_rv = 0.0;

        let (assign26120_e24580, assign26120_e24580_d_n0, assign26120_e24580_d_n2, assign26120_e24580_d_n4, assign26120_e24580_d_n5, assign26120_e24580_d_n6, assign26120_e24580_d_n7, assign26120_e24580_d_n8, assign26120_e24580_d_n9, assign26120_e24580_d_n10, assign26120_e24580_d_n11, assign26120_e24580_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign26120_e24578: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign26120_e24578, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn11 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn11)), ((locals.var_w_b0_dn14 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
        locals.var_q_b0_dep = assign26120_e24580;
        locals.var_q_b0_dep_dn0 = assign26120_e24580_d_n0;
        locals.var_q_b0_dep_dn2 = assign26120_e24580_d_n2;
        locals.var_q_b0_dep_dn4 = assign26120_e24580_d_n4;
        locals.var_q_b0_dep_dn5 = assign26120_e24580_d_n5;
        locals.var_q_b0_dep_dn6 = assign26120_e24580_d_n6;
        locals.var_q_b0_dep_dn7 = assign26120_e24580_d_n7;
        locals.var_q_b0_dep_dn8 = assign26120_e24580_d_n8;
        locals.var_q_b0_dep_dn9 = assign26120_e24580_d_n9;
        locals.var_q_b0_dep_dn10 = assign26120_e24580_d_n10;
        locals.var_q_b0_dep_dn11 = assign26120_e24580_d_n11;
        locals.var_q_b0_dep_dn14 = assign26120_e24580_d_n14;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign26130_e24591, assign26130_e24591_d_n0, assign26130_e24591_d_n2, assign26130_e24591_d_n4, assign26130_e24591_d_n5, assign26130_e24591_d_n6, assign26130_e24591_d_n7, assign26130_e24591_d_n8, assign26130_e24591_d_n9, assign26130_e24591_d_n10, assign26130_e24591_d_n11, assign26130_e24591_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign26130_e24587: f64 = (-locals.var_w_sub0);
        let assign26130_e24589: f64 = (assign26130_e24587 * locals.var_q_nsub__blk548);
        (assign26130_e24589, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn10)), (((-locals.var_w_sub0_dn11) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn11)), (((-locals.var_w_sub0_dn14) * locals.var_q_nsub__blk548) + (assign26130_e24587 * locals.var_q_nsub__blk548_dn14)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
        locals.var_q_sub0_dep = assign26130_e24591;
        locals.var_q_sub0_dep_dn0 = assign26130_e24591_d_n0;
        locals.var_q_sub0_dep_dn2 = assign26130_e24591_d_n2;
        locals.var_q_sub0_dep_dn4 = assign26130_e24591_d_n4;
        locals.var_q_sub0_dep_dn5 = assign26130_e24591_d_n5;
        locals.var_q_sub0_dep_dn6 = assign26130_e24591_d_n6;
        locals.var_q_sub0_dep_dn7 = assign26130_e24591_d_n7;
        locals.var_q_sub0_dep_dn8 = assign26130_e24591_d_n8;
        locals.var_q_sub0_dep_dn9 = assign26130_e24591_d_n9;
        locals.var_q_sub0_dep_dn10 = assign26130_e24591_d_n10;
        locals.var_q_sub0_dep_dn11 = assign26130_e24591_d_n11;
        locals.var_q_sub0_dep_dn14 = assign26130_e24591_d_n14;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign26140_e24606, assign26140_e24606_d_n0, assign26140_e24606_d_n2, assign26140_e24606_d_n4, assign26140_e24606_d_n5, assign26140_e24606_d_n6, assign26140_e24606_d_n7, assign26140_e24606_d_n8, assign26140_e24606_d_n9, assign26140_e24606_d_n10, assign26140_e24606_d_n11, assign26140_e24606_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26140_e24599: f64 = (-locals.var_beta);
        let assign26140_e24602: f64 = (locals.var_phi_s0_dep - locals.var_vbscl__blk439);
        let assign26140_e24603: f64 = (assign26140_e24599 * assign26140_e24602);
        let assign26140_e24604: f64 = (assign26140_e24603).exp();
        (assign26140_e24604, (assign26140_e24604 * (((-locals.var_beta_dn0) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn0 - locals.var_vbscl__blk439_dn0)))), (assign26140_e24604 * (((-locals.var_beta_dn2) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn2 - locals.var_vbscl__blk439_dn2)))), (assign26140_e24604 * (((-locals.var_beta_dn4) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn4 - locals.var_vbscl__blk439_dn4)))), (assign26140_e24604 * (((-locals.var_beta_dn5) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn5 - locals.var_vbscl__blk439_dn5)))), (assign26140_e24604 * (((-locals.var_beta_dn6) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn6 - locals.var_vbscl__blk439_dn6)))), (assign26140_e24604 * (((-locals.var_beta_dn7) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn7 - locals.var_vbscl__blk439_dn7)))), (assign26140_e24604 * (((-locals.var_beta_dn8) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn8 - locals.var_vbscl__blk439_dn8)))), (assign26140_e24604 * (((-locals.var_beta_dn9) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn9 - locals.var_vbscl__blk439_dn9)))), (assign26140_e24604 * (((-locals.var_beta_dn10) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn10 - locals.var_vbscl__blk439_dn10)))), (assign26140_e24604 * (((-locals.var_beta_dn11) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn11 - locals.var_vbscl__blk439_dn11)))), (assign26140_e24604 * (((-locals.var_beta_dn14) * assign26140_e24602) + (assign26140_e24599 * (locals.var_phi_s0_dep_dn14 - locals.var_vbscl__blk439_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26140_e24606;
        locals.var_t3_dn0 = assign26140_e24606_d_n0;
        locals.var_t3_dn2 = assign26140_e24606_d_n2;
        locals.var_t3_dn4 = assign26140_e24606_d_n4;
        locals.var_t3_dn5 = assign26140_e24606_d_n5;
        locals.var_t3_dn6 = assign26140_e24606_d_n6;
        locals.var_t3_dn7 = assign26140_e24606_d_n7;
        locals.var_t3_dn8 = assign26140_e24606_d_n8;
        locals.var_t3_dn9 = assign26140_e24606_d_n9;
        locals.var_t3_dn10 = assign26140_e24606_d_n10;
        locals.var_t3_dn11 = assign26140_e24606_d_n11;
        locals.var_t3_dn14 = assign26140_e24606_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign26150_e24621, assign26150_e24621_d_n0, assign26150_e24621_d_n2, assign26150_e24621_d_n4, assign26150_e24621_d_n5, assign26150_e24621_d_n6, assign26150_e24621_d_n7, assign26150_e24621_d_n8, assign26150_e24621_d_n9, assign26150_e24621_d_n10, assign26150_e24621_d_n11, assign26150_e24621_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26150_e24614: f64 = (-locals.var_beta);
        let assign26150_e24617: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk439);
        let assign26150_e24618: f64 = (assign26150_e24614 * assign26150_e24617);
        let assign26150_e24619: f64 = (assign26150_e24618).exp();
        (assign26150_e24619, (assign26150_e24619 * (((-locals.var_beta_dn0) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn0 - locals.var_vbscl__blk439_dn0)))), (assign26150_e24619 * (((-locals.var_beta_dn2) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn2 - locals.var_vbscl__blk439_dn2)))), (assign26150_e24619 * (((-locals.var_beta_dn4) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn4 - locals.var_vbscl__blk439_dn4)))), (assign26150_e24619 * (((-locals.var_beta_dn5) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn5 - locals.var_vbscl__blk439_dn5)))), (assign26150_e24619 * (((-locals.var_beta_dn6) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn6 - locals.var_vbscl__blk439_dn6)))), (assign26150_e24619 * (((-locals.var_beta_dn7) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn7 - locals.var_vbscl__blk439_dn7)))), (assign26150_e24619 * (((-locals.var_beta_dn8) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn8 - locals.var_vbscl__blk439_dn8)))), (assign26150_e24619 * (((-locals.var_beta_dn9) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn9 - locals.var_vbscl__blk439_dn9)))), (assign26150_e24619 * (((-locals.var_beta_dn10) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn10 - locals.var_vbscl__blk439_dn10)))), (assign26150_e24619 * (((-locals.var_beta_dn11) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn11 - locals.var_vbscl__blk439_dn11)))), (assign26150_e24619 * (((-locals.var_beta_dn14) * assign26150_e24617) + (assign26150_e24614 * (locals.var_phi_b0_dep_dn14 - locals.var_vbscl__blk439_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign26150_e24621;
        locals.var_t4_dn0 = assign26150_e24621_d_n0;
        locals.var_t4_dn2 = assign26150_e24621_d_n2;
        locals.var_t4_dn4 = assign26150_e24621_d_n4;
        locals.var_t4_dn5 = assign26150_e24621_d_n5;
        locals.var_t4_dn6 = assign26150_e24621_d_n6;
        locals.var_t4_dn7 = assign26150_e24621_d_n7;
        locals.var_t4_dn8 = assign26150_e24621_d_n8;
        locals.var_t4_dn9 = assign26150_e24621_d_n9;
        locals.var_t4_dn10 = assign26150_e24621_d_n10;
        locals.var_t4_dn11 = assign26150_e24621_d_n11;
        locals.var_t4_dn14 = assign26150_e24621_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign26160_e24645, assign26160_e24645_d_n0, assign26160_e24645_d_n2, assign26160_e24645_d_n4, assign26160_e24645_d_n5, assign26160_e24645_d_n6, assign26160_e24645_d_n7, assign26160_e24645_d_n8, assign26160_e24645_d_n9, assign26160_e24645_d_n10, assign26160_e24645_d_n11, assign26160_e24645_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26160_e24631: f64 = (locals.var_t2 - 1.0);
        let assign26160_e24633: f64 = (assign26160_e24631 - locals.var_t1);
        let assign26160_e24637: f64 = (locals.var_t3 - locals.var_t4);
        let assign26160_e24638: f64 = (locals.var_cnst1 * assign26160_e24637);
        let assign26160_e24639: f64 = (assign26160_e24633 + assign26160_e24638);
        let assign26160_e24641: f64 = (assign26160_e24639 + 1e-15);
        let assign26160_e24642: f64 = (assign26160_e24641).sqrt();
        let assign26160_e24643: f64 = (locals.var_cnst0 * assign26160_e24642);
        (assign26160_e24643, ((locals.var_cnst0_dn0 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn2 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn4 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn5 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn6 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn7 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn8 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn9 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn10 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn11 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign26160_e24642)))), ((locals.var_cnst0_dn14 * assign26160_e24642) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign26160_e24637) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign26160_e24642)))),)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign26160_e24645;
        locals.var_q_s0_dn0 = assign26160_e24645_d_n0;
        locals.var_q_s0_dn2 = assign26160_e24645_d_n2;
        locals.var_q_s0_dn4 = assign26160_e24645_d_n4;
        locals.var_q_s0_dn5 = assign26160_e24645_d_n5;
        locals.var_q_s0_dn6 = assign26160_e24645_d_n6;
        locals.var_q_s0_dn7 = assign26160_e24645_d_n7;
        locals.var_q_s0_dn8 = assign26160_e24645_d_n8;
        locals.var_q_s0_dn9 = assign26160_e24645_d_n9;
        locals.var_q_s0_dn10 = assign26160_e24645_d_n10;
        locals.var_q_s0_dn11 = assign26160_e24645_d_n11;
        locals.var_q_s0_dn14 = assign26160_e24645_d_n14;
        locals.var_q_s0_rv = 0.0;

        let assign26170_e24652: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard634 = assign26170_e24652;
        locals.var_guard634_rv = 0.0;

        let (assign26180_e24663, assign26180_e24663_d_n0, assign26180_e24663_d_n2, assign26180_e24663_d_n4, assign26180_e24663_d_n5, assign26180_e24663_d_n6, assign26180_e24663_d_n7, assign26180_e24663_d_n8, assign26180_e24663_d_n9, assign26180_e24663_d_n10, assign26180_e24663_d_n11, assign26180_e24663_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard634 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    }
};
        locals.var_q_sub0 = assign26180_e24663;
        locals.var_q_sub0_dn0 = assign26180_e24663_d_n0;
        locals.var_q_sub0_dn2 = assign26180_e24663_d_n2;
        locals.var_q_sub0_dn4 = assign26180_e24663_d_n4;
        locals.var_q_sub0_dn5 = assign26180_e24663_d_n5;
        locals.var_q_sub0_dn6 = assign26180_e24663_d_n6;
        locals.var_q_sub0_dn7 = assign26180_e24663_d_n7;
        locals.var_q_sub0_dn8 = assign26180_e24663_d_n8;
        locals.var_q_sub0_dn9 = assign26180_e24663_d_n9;
        locals.var_q_sub0_dn10 = assign26180_e24663_d_n10;
        locals.var_q_sub0_dn11 = assign26180_e24663_d_n11;
        locals.var_q_sub0_dn14 = assign26180_e24663_d_n14;
        locals.var_q_sub0_rv = 0.0;

        let (assign26190_e24674, assign26190_e24674_d_n0, assign26190_e24674_d_n2, assign26190_e24674_d_n4, assign26190_e24674_d_n5, assign26190_e24674_d_n6, assign26190_e24674_d_n7, assign26190_e24674_d_n8, assign26190_e24674_d_n9, assign26190_e24674_d_n10, assign26190_e24674_d_n11, assign26190_e24674_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard634 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign26190_e24674;
        locals.var_q_s0_dep_dn0 = assign26190_e24674_d_n0;
        locals.var_q_s0_dep_dn2 = assign26190_e24674_d_n2;
        locals.var_q_s0_dep_dn4 = assign26190_e24674_d_n4;
        locals.var_q_s0_dep_dn5 = assign26190_e24674_d_n5;
        locals.var_q_s0_dep_dn6 = assign26190_e24674_d_n6;
        locals.var_q_s0_dep_dn7 = assign26190_e24674_d_n7;
        locals.var_q_s0_dep_dn8 = assign26190_e24674_d_n8;
        locals.var_q_s0_dep_dn9 = assign26190_e24674_d_n9;
        locals.var_q_s0_dep_dn10 = assign26190_e24674_d_n10;
        locals.var_q_s0_dep_dn11 = assign26190_e24674_d_n11;
        locals.var_q_s0_dep_dn14 = assign26190_e24674_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign26200_e24708, assign26200_e24708_d_n0, assign26200_e24708_d_n2, assign26200_e24708_d_n4, assign26200_e24708_d_n5, assign26200_e24708_d_n6, assign26200_e24708_d_n7, assign26200_e24708_d_n8, assign26200_e24708_d_n9, assign26200_e24708_d_n10, assign26200_e24708_d_n11, assign26200_e24708_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard634 == 0.0)) {
        let assign26200_e24686: f64 = (-locals.var_t1);
        let assign26200_e24689: f64 = (-locals.var_beta);
        let assign26200_e24692: f64 = (locals.var_phi_s0_dep - locals.var_vbscl__blk439);
        let assign26200_e24693: f64 = (assign26200_e24689 * assign26200_e24692);
        let assign26200_e24694: f64 = (assign26200_e24693).exp();
        let assign26200_e24696: f64 = (-locals.var_beta);
        let assign26200_e24699: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk439);
        let assign26200_e24700: f64 = (assign26200_e24696 * assign26200_e24699);
        let assign26200_e24701: f64 = (assign26200_e24700).exp();
        let assign26200_e24702: f64 = (assign26200_e24694 - assign26200_e24701);
        let assign26200_e24703: f64 = (locals.var_cnst1 * assign26200_e24702);
        let assign26200_e24704: f64 = (assign26200_e24686 + assign26200_e24703);
        let assign26200_e24705: f64 = (assign26200_e24704).sqrt();
        let assign26200_e24706: f64 = (locals.var_cnst0 * assign26200_e24705);
        (assign26200_e24706, ((locals.var_cnst0_dn0 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn0) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn0 - locals.var_vbscl__blk439_dn0)))) - (assign26200_e24701 * (((-locals.var_beta_dn0) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn0 - locals.var_vbscl__blk439_dn0)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn2 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn2) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn2 - locals.var_vbscl__blk439_dn2)))) - (assign26200_e24701 * (((-locals.var_beta_dn2) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn2 - locals.var_vbscl__blk439_dn2)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn4 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn4) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn4 - locals.var_vbscl__blk439_dn4)))) - (assign26200_e24701 * (((-locals.var_beta_dn4) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn4 - locals.var_vbscl__blk439_dn4)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn5 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn5) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn5 - locals.var_vbscl__blk439_dn5)))) - (assign26200_e24701 * (((-locals.var_beta_dn5) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn5 - locals.var_vbscl__blk439_dn5)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn6 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn6) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn6 - locals.var_vbscl__blk439_dn6)))) - (assign26200_e24701 * (((-locals.var_beta_dn6) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn6 - locals.var_vbscl__blk439_dn6)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn7 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn7) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn7 - locals.var_vbscl__blk439_dn7)))) - (assign26200_e24701 * (((-locals.var_beta_dn7) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn7 - locals.var_vbscl__blk439_dn7)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn8 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn8) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn8 - locals.var_vbscl__blk439_dn8)))) - (assign26200_e24701 * (((-locals.var_beta_dn8) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn8 - locals.var_vbscl__blk439_dn8)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn9 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn9) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn9 - locals.var_vbscl__blk439_dn9)))) - (assign26200_e24701 * (((-locals.var_beta_dn9) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn9 - locals.var_vbscl__blk439_dn9)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn10 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn10) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn10 - locals.var_vbscl__blk439_dn10)))) - (assign26200_e24701 * (((-locals.var_beta_dn10) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn10 - locals.var_vbscl__blk439_dn10)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn11 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn11) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn11 - locals.var_vbscl__blk439_dn11)))) - (assign26200_e24701 * (((-locals.var_beta_dn11) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn11 - locals.var_vbscl__blk439_dn11)))))))) / (2.0 * assign26200_e24705)))), ((locals.var_cnst0_dn14 * assign26200_e24705) + (locals.var_cnst0 * (((-locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign26200_e24702) + (locals.var_cnst1 * ((assign26200_e24694 * (((-locals.var_beta_dn14) * assign26200_e24692) + (assign26200_e24689 * (locals.var_phi_s0_dep_dn14 - locals.var_vbscl__blk439_dn14)))) - (assign26200_e24701 * (((-locals.var_beta_dn14) * assign26200_e24699) + (assign26200_e24696 * (locals.var_phi_b0_dep_dn14 - locals.var_vbscl__blk439_dn14)))))))) / (2.0 * assign26200_e24705)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26200_e24708;
        locals.var_t3_dn0 = assign26200_e24708_d_n0;
        locals.var_t3_dn2 = assign26200_e24708_d_n2;
        locals.var_t3_dn4 = assign26200_e24708_d_n4;
        locals.var_t3_dn5 = assign26200_e24708_d_n5;
        locals.var_t3_dn6 = assign26200_e24708_d_n6;
        locals.var_t3_dn7 = assign26200_e24708_d_n7;
        locals.var_t3_dn8 = assign26200_e24708_d_n8;
        locals.var_t3_dn9 = assign26200_e24708_d_n9;
        locals.var_t3_dn10 = assign26200_e24708_d_n10;
        locals.var_t3_dn11 = assign26200_e24708_d_n11;
        locals.var_t3_dn14 = assign26200_e24708_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26210_e24726, assign26210_e24726_d_n0, assign26210_e24726_d_n2, assign26210_e24726_d_n4, assign26210_e24726_d_n5, assign26210_e24726_d_n6, assign26210_e24726_d_n7, assign26210_e24726_d_n8, assign26210_e24726_d_n9, assign26210_e24726_d_n10, assign26210_e24726_d_n11, assign26210_e24726_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard634 == 0.0)) {
        let assign26210_e24721: f64 = (-locals.var_t1);
        let assign26210_e24722: f64 = (assign26210_e24721).sqrt();
        let assign26210_e24723: f64 = (locals.var_cnst0 * assign26210_e24722);
        let assign26210_e24724: f64 = (locals.var_t3 - assign26210_e24723);
        (assign26210_e24724, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn11 - ((locals.var_cnst0_dn11 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn11) / (2.0 * assign26210_e24722))))), (locals.var_t3_dn14 - ((locals.var_cnst0_dn14 * assign26210_e24722) + (locals.var_cnst0 * ((-locals.var_t1_dn14) / (2.0 * assign26210_e24722))))),)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    }
};
        locals.var_q_sub0 = assign26210_e24726;
        locals.var_q_sub0_dn0 = assign26210_e24726_d_n0;
        locals.var_q_sub0_dn2 = assign26210_e24726_d_n2;
        locals.var_q_sub0_dn4 = assign26210_e24726_d_n4;
        locals.var_q_sub0_dn5 = assign26210_e24726_d_n5;
        locals.var_q_sub0_dn6 = assign26210_e24726_d_n6;
        locals.var_q_sub0_dn7 = assign26210_e24726_d_n7;
        locals.var_q_sub0_dn8 = assign26210_e24726_d_n8;
        locals.var_q_sub0_dn9 = assign26210_e24726_d_n9;
        locals.var_q_sub0_dn10 = assign26210_e24726_d_n10;
        locals.var_q_sub0_dn11 = assign26210_e24726_d_n11;
        locals.var_q_sub0_dn14 = assign26210_e24726_d_n14;
        locals.var_q_sub0_rv = 0.0;

        let (assign26220_e24747, assign26220_e24747_d_n0, assign26220_e24747_d_n2, assign26220_e24747_d_n4, assign26220_e24747_d_n5, assign26220_e24747_d_n6, assign26220_e24747_d_n7, assign26220_e24747_d_n8, assign26220_e24747_d_n9, assign26220_e24747_d_n10, assign26220_e24747_d_n11, assign26220_e24747_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard634 == 0.0)) {
        let assign26220_e24739: f64 = (locals.var_t2 - 1.0);
        let assign26220_e24741: f64 = (assign26220_e24739 - locals.var_t1);
        let assign26220_e24743: f64 = (assign26220_e24741 + 1e-15);
        let assign26220_e24744: f64 = (assign26220_e24743).sqrt();
        let assign26220_e24745: f64 = (locals.var_cnst0 * assign26220_e24744);
        (assign26220_e24745, ((locals.var_cnst0_dn0 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn2 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn4 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn5 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn6 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn7 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn8 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn9 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn10 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn11 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign26220_e24744)))), ((locals.var_cnst0_dn14 * assign26220_e24744) + (locals.var_cnst0 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign26220_e24744)))),)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign26220_e24747;
        locals.var_q_s0_dep_dn0 = assign26220_e24747_d_n0;
        locals.var_q_s0_dep_dn2 = assign26220_e24747_d_n2;
        locals.var_q_s0_dep_dn4 = assign26220_e24747_d_n4;
        locals.var_q_s0_dep_dn5 = assign26220_e24747_d_n5;
        locals.var_q_s0_dep_dn6 = assign26220_e24747_d_n6;
        locals.var_q_s0_dep_dn7 = assign26220_e24747_d_n7;
        locals.var_q_s0_dep_dn8 = assign26220_e24747_d_n8;
        locals.var_q_s0_dep_dn9 = assign26220_e24747_d_n9;
        locals.var_q_s0_dep_dn10 = assign26220_e24747_d_n10;
        locals.var_q_s0_dep_dn11 = assign26220_e24747_d_n11;
        locals.var_q_s0_dep_dn14 = assign26220_e24747_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign26230_e24756, assign26230_e24756_d_n0, assign26230_e24756_d_n2, assign26230_e24756_d_n4, assign26230_e24756_d_n5, assign26230_e24756_d_n6, assign26230_e24756_d_n7, assign26230_e24756_d_n8, assign26230_e24756_d_n9, assign26230_e24756_d_n10, assign26230_e24756_d_n11, assign26230_e24756_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_n0__blk542, locals.var_q_n0__blk542_dn0, locals.var_q_n0__blk542_dn2, locals.var_q_n0__blk542_dn4, locals.var_q_n0__blk542_dn5, locals.var_q_n0__blk542_dn6, locals.var_q_n0__blk542_dn7, locals.var_q_n0__blk542_dn8, locals.var_q_n0__blk542_dn9, locals.var_q_n0__blk542_dn10, locals.var_q_n0__blk542_dn11, locals.var_q_n0__blk542_dn14,)
    }
};
        locals.var_q_n0__blk542 = assign26230_e24756;
        locals.var_q_n0__blk542_dn0 = assign26230_e24756_d_n0;
        locals.var_q_n0__blk542_dn2 = assign26230_e24756_d_n2;
        locals.var_q_n0__blk542_dn4 = assign26230_e24756_d_n4;
        locals.var_q_n0__blk542_dn5 = assign26230_e24756_d_n5;
        locals.var_q_n0__blk542_dn6 = assign26230_e24756_d_n6;
        locals.var_q_n0__blk542_dn7 = assign26230_e24756_d_n7;
        locals.var_q_n0__blk542_dn8 = assign26230_e24756_d_n8;
        locals.var_q_n0__blk542_dn9 = assign26230_e24756_d_n9;
        locals.var_q_n0__blk542_dn10 = assign26230_e24756_d_n10;
        locals.var_q_n0__blk542_dn11 = assign26230_e24756_d_n11;
        locals.var_q_n0__blk542_dn14 = assign26230_e24756_d_n14;
        locals.var_q_n0__blk542_rv = 0.0;

        let (assign26240_e24767, assign26240_e24767_d_n0, assign26240_e24767_d_n2, assign26240_e24767_d_n4, assign26240_e24767_d_n5, assign26240_e24767_d_n6, assign26240_e24767_d_n7, assign26240_e24767_d_n8, assign26240_e24767_d_n9, assign26240_e24767_d_n10, assign26240_e24767_d_n11, assign26240_e24767_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26240_e24765: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        (assign26240_e24765, (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26240_e24767;
        locals.var_t1_dn0 = assign26240_e24767_d_n0;
        locals.var_t1_dn2 = assign26240_e24767_d_n2;
        locals.var_t1_dn4 = assign26240_e24767_d_n4;
        locals.var_t1_dn5 = assign26240_e24767_d_n5;
        locals.var_t1_dn6 = assign26240_e24767_d_n6;
        locals.var_t1_dn7 = assign26240_e24767_d_n7;
        locals.var_t1_dn8 = assign26240_e24767_d_n8;
        locals.var_t1_dn9 = assign26240_e24767_d_n9;
        locals.var_t1_dn10 = assign26240_e24767_d_n10;
        locals.var_t1_dn11 = assign26240_e24767_d_n11;
        locals.var_t1_dn14 = assign26240_e24767_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26250_e24771: f64 = 0.1;
        let assign26250_e24776: f64 = if ((locals.var_t1 < assign26250_e24771) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard635 = assign26250_e24776;
        locals.var_guard635_rv = 0.0;

        let (assign26260_e24791, assign26260_e24791_d_n0, assign26260_e24791_d_n2, assign26260_e24791_d_n4, assign26260_e24791_d_n5, assign26260_e24791_d_n6, assign26260_e24791_d_n7, assign26260_e24791_d_n8, assign26260_e24791_d_n9, assign26260_e24791_d_n10, assign26260_e24791_d_n11, assign26260_e24791_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26260_e24787: f64 = 0.1;
        let assign26260_e24789: f64 = (assign26260_e24787 - locals.var_t1);
        (assign26260_e24789, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign26260_e24791;
        locals.var_tmf1_dn0 = assign26260_e24791_d_n0;
        locals.var_tmf1_dn2 = assign26260_e24791_d_n2;
        locals.var_tmf1_dn4 = assign26260_e24791_d_n4;
        locals.var_tmf1_dn5 = assign26260_e24791_d_n5;
        locals.var_tmf1_dn6 = assign26260_e24791_d_n6;
        locals.var_tmf1_dn7 = assign26260_e24791_d_n7;
        locals.var_tmf1_dn8 = assign26260_e24791_d_n8;
        locals.var_tmf1_dn9 = assign26260_e24791_d_n9;
        locals.var_tmf1_dn10 = assign26260_e24791_d_n10;
        locals.var_tmf1_dn11 = assign26260_e24791_d_n11;
        locals.var_tmf1_dn14 = assign26260_e24791_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign26270_e24804, assign26270_e24804_d_n0, assign26270_e24804_d_n2, assign26270_e24804_d_n4, assign26270_e24804_d_n5, assign26270_e24804_d_n6, assign26270_e24804_d_n7, assign26270_e24804_d_n8, assign26270_e24804_d_n9, assign26270_e24804_d_n10, assign26270_e24804_d_n11, assign26270_e24804_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26270_e24802: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26270_e24802, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign26270_e24804;
        locals.var_x2_dn0 = assign26270_e24804_d_n0;
        locals.var_x2_dn2 = assign26270_e24804_d_n2;
        locals.var_x2_dn4 = assign26270_e24804_d_n4;
        locals.var_x2_dn5 = assign26270_e24804_d_n5;
        locals.var_x2_dn6 = assign26270_e24804_d_n6;
        locals.var_x2_dn7 = assign26270_e24804_d_n7;
        locals.var_x2_dn8 = assign26270_e24804_d_n8;
        locals.var_x2_dn9 = assign26270_e24804_d_n9;
        locals.var_x2_dn10 = assign26270_e24804_d_n10;
        locals.var_x2_dn11 = assign26270_e24804_d_n11;
        locals.var_x2_dn14 = assign26270_e24804_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign26280_e24817, assign26280_e24817_d_n0, assign26280_e24817_d_n2, assign26280_e24817_d_n4, assign26280_e24817_d_n5, assign26280_e24817_d_n6, assign26280_e24817_d_n7, assign26280_e24817_d_n8, assign26280_e24817_d_n9, assign26280_e24817_d_n10, assign26280_e24817_d_n11, assign26280_e24817_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26280_e24815: f64 = (0.1 * 0.1);
        (assign26280_e24815, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign26280_e24817;
        locals.var_xmax2_dn0 = assign26280_e24817_d_n0;
        locals.var_xmax2_dn2 = assign26280_e24817_d_n2;
        locals.var_xmax2_dn4 = assign26280_e24817_d_n4;
        locals.var_xmax2_dn5 = assign26280_e24817_d_n5;
        locals.var_xmax2_dn6 = assign26280_e24817_d_n6;
        locals.var_xmax2_dn7 = assign26280_e24817_d_n7;
        locals.var_xmax2_dn8 = assign26280_e24817_d_n8;
        locals.var_xmax2_dn9 = assign26280_e24817_d_n9;
        locals.var_xmax2_dn10 = assign26280_e24817_d_n10;
        locals.var_xmax2_dn11 = assign26280_e24817_d_n11;
        locals.var_xmax2_dn14 = assign26280_e24817_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign26290_e24828, assign26290_e24828_d_n0, assign26290_e24828_d_n2, assign26290_e24828_d_n4, assign26290_e24828_d_n5, assign26290_e24828_d_n6, assign26290_e24828_d_n7, assign26290_e24828_d_n8, assign26290_e24828_d_n9, assign26290_e24828_d_n10, assign26290_e24828_d_n11, assign26290_e24828_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26290_e24828;
        locals.var_xp_dn0 = assign26290_e24828_d_n0;
        locals.var_xp_dn2 = assign26290_e24828_d_n2;
        locals.var_xp_dn4 = assign26290_e24828_d_n4;
        locals.var_xp_dn5 = assign26290_e24828_d_n5;
        locals.var_xp_dn6 = assign26290_e24828_d_n6;
        locals.var_xp_dn7 = assign26290_e24828_d_n7;
        locals.var_xp_dn8 = assign26290_e24828_d_n8;
        locals.var_xp_dn9 = assign26290_e24828_d_n9;
        locals.var_xp_dn10 = assign26290_e24828_d_n10;
        locals.var_xp_dn11 = assign26290_e24828_d_n11;
        locals.var_xp_dn14 = assign26290_e24828_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26300_e24839, assign26300_e24839_d_n0, assign26300_e24839_d_n2, assign26300_e24839_d_n4, assign26300_e24839_d_n5, assign26300_e24839_d_n6, assign26300_e24839_d_n7, assign26300_e24839_d_n8, assign26300_e24839_d_n9, assign26300_e24839_d_n10, assign26300_e24839_d_n11, assign26300_e24839_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26300_e24839;
        locals.var_xmp_dn0 = assign26300_e24839_d_n0;
        locals.var_xmp_dn2 = assign26300_e24839_d_n2;
        locals.var_xmp_dn4 = assign26300_e24839_d_n4;
        locals.var_xmp_dn5 = assign26300_e24839_d_n5;
        locals.var_xmp_dn6 = assign26300_e24839_d_n6;
        locals.var_xmp_dn7 = assign26300_e24839_d_n7;
        locals.var_xmp_dn8 = assign26300_e24839_d_n8;
        locals.var_xmp_dn9 = assign26300_e24839_d_n9;
        locals.var_xmp_dn10 = assign26300_e24839_d_n10;
        locals.var_xmp_dn11 = assign26300_e24839_d_n11;
        locals.var_xmp_dn14 = assign26300_e24839_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26310_e24850,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26310_e24850;
        locals.var_m0_rv = 0.0;

        let (assign26320_e24861,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26320_e24861;
        locals.var_mm_rv = 0.0;

        let (assign26330_e24872, assign26330_e24872_d_n0, assign26330_e24872_d_n2, assign26330_e24872_d_n4, assign26330_e24872_d_n5, assign26330_e24872_d_n6, assign26330_e24872_d_n7, assign26330_e24872_d_n8, assign26330_e24872_d_n9, assign26330_e24872_d_n10, assign26330_e24872_d_n11, assign26330_e24872_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26330_e24872;
        locals.var_arg_dn0 = assign26330_e24872_d_n0;
        locals.var_arg_dn2 = assign26330_e24872_d_n2;
        locals.var_arg_dn4 = assign26330_e24872_d_n4;
        locals.var_arg_dn5 = assign26330_e24872_d_n5;
        locals.var_arg_dn6 = assign26330_e24872_d_n6;
        locals.var_arg_dn7 = assign26330_e24872_d_n7;
        locals.var_arg_dn8 = assign26330_e24872_d_n8;
        locals.var_arg_dn9 = assign26330_e24872_d_n9;
        locals.var_arg_dn10 = assign26330_e24872_d_n10;
        locals.var_arg_dn11 = assign26330_e24872_d_n11;
        locals.var_arg_dn14 = assign26330_e24872_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26340_e24883, assign26340_e24883_d_n0, assign26340_e24883_d_n2, assign26340_e24883_d_n4, assign26340_e24883_d_n5, assign26340_e24883_d_n6, assign26340_e24883_d_n7, assign26340_e24883_d_n8, assign26340_e24883_d_n9, assign26340_e24883_d_n10, assign26340_e24883_d_n11, assign26340_e24883_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26340_e24883;
        locals.var_dnm_dn0 = assign26340_e24883_d_n0;
        locals.var_dnm_dn2 = assign26340_e24883_d_n2;
        locals.var_dnm_dn4 = assign26340_e24883_d_n4;
        locals.var_dnm_dn5 = assign26340_e24883_d_n5;
        locals.var_dnm_dn6 = assign26340_e24883_d_n6;
        locals.var_dnm_dn7 = assign26340_e24883_d_n7;
        locals.var_dnm_dn8 = assign26340_e24883_d_n8;
        locals.var_dnm_dn9 = assign26340_e24883_d_n9;
        locals.var_dnm_dn10 = assign26340_e24883_d_n10;
        locals.var_dnm_dn11 = assign26340_e24883_d_n11;
        locals.var_dnm_dn14 = assign26340_e24883_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26350_e24896, assign26350_e24896_d_n0, assign26350_e24896_d_n2, assign26350_e24896_d_n4, assign26350_e24896_d_n5, assign26350_e24896_d_n6, assign26350_e24896_d_n7, assign26350_e24896_d_n8, assign26350_e24896_d_n9, assign26350_e24896_d_n10, assign26350_e24896_d_n11, assign26350_e24896_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26350_e24894: f64 = (locals.var_xp * locals.var_x2);
        (assign26350_e24894, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26350_e24896;
        locals.var_xp_dn0 = assign26350_e24896_d_n0;
        locals.var_xp_dn2 = assign26350_e24896_d_n2;
        locals.var_xp_dn4 = assign26350_e24896_d_n4;
        locals.var_xp_dn5 = assign26350_e24896_d_n5;
        locals.var_xp_dn6 = assign26350_e24896_d_n6;
        locals.var_xp_dn7 = assign26350_e24896_d_n7;
        locals.var_xp_dn8 = assign26350_e24896_d_n8;
        locals.var_xp_dn9 = assign26350_e24896_d_n9;
        locals.var_xp_dn10 = assign26350_e24896_d_n10;
        locals.var_xp_dn11 = assign26350_e24896_d_n11;
        locals.var_xp_dn14 = assign26350_e24896_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26360_e24909, assign26360_e24909_d_n0, assign26360_e24909_d_n2, assign26360_e24909_d_n4, assign26360_e24909_d_n5, assign26360_e24909_d_n6, assign26360_e24909_d_n7, assign26360_e24909_d_n8, assign26360_e24909_d_n9, assign26360_e24909_d_n10, assign26360_e24909_d_n11, assign26360_e24909_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26360_e24907: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26360_e24907, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26360_e24909;
        locals.var_xmp_dn0 = assign26360_e24909_d_n0;
        locals.var_xmp_dn2 = assign26360_e24909_d_n2;
        locals.var_xmp_dn4 = assign26360_e24909_d_n4;
        locals.var_xmp_dn5 = assign26360_e24909_d_n5;
        locals.var_xmp_dn6 = assign26360_e24909_d_n6;
        locals.var_xmp_dn7 = assign26360_e24909_d_n7;
        locals.var_xmp_dn8 = assign26360_e24909_d_n8;
        locals.var_xmp_dn9 = assign26360_e24909_d_n9;
        locals.var_xmp_dn10 = assign26360_e24909_d_n10;
        locals.var_xmp_dn11 = assign26360_e24909_d_n11;
        locals.var_xmp_dn14 = assign26360_e24909_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26370_e24922, assign26370_e24922_d_n0, assign26370_e24922_d_n2, assign26370_e24922_d_n4, assign26370_e24922_d_n5, assign26370_e24922_d_n6, assign26370_e24922_d_n7, assign26370_e24922_d_n8, assign26370_e24922_d_n9, assign26370_e24922_d_n10, assign26370_e24922_d_n11, assign26370_e24922_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26370_e24920: f64 = (locals.var_xp * locals.var_x2);
        (assign26370_e24920, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26370_e24922;
        locals.var_xp_dn0 = assign26370_e24922_d_n0;
        locals.var_xp_dn2 = assign26370_e24922_d_n2;
        locals.var_xp_dn4 = assign26370_e24922_d_n4;
        locals.var_xp_dn5 = assign26370_e24922_d_n5;
        locals.var_xp_dn6 = assign26370_e24922_d_n6;
        locals.var_xp_dn7 = assign26370_e24922_d_n7;
        locals.var_xp_dn8 = assign26370_e24922_d_n8;
        locals.var_xp_dn9 = assign26370_e24922_d_n9;
        locals.var_xp_dn10 = assign26370_e24922_d_n10;
        locals.var_xp_dn11 = assign26370_e24922_d_n11;
        locals.var_xp_dn14 = assign26370_e24922_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26380_e24935, assign26380_e24935_d_n0, assign26380_e24935_d_n2, assign26380_e24935_d_n4, assign26380_e24935_d_n5, assign26380_e24935_d_n6, assign26380_e24935_d_n7, assign26380_e24935_d_n8, assign26380_e24935_d_n9, assign26380_e24935_d_n10, assign26380_e24935_d_n11, assign26380_e24935_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26380_e24933: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26380_e24933, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26380_e24935;
        locals.var_xmp_dn0 = assign26380_e24935_d_n0;
        locals.var_xmp_dn2 = assign26380_e24935_d_n2;
        locals.var_xmp_dn4 = assign26380_e24935_d_n4;
        locals.var_xmp_dn5 = assign26380_e24935_d_n5;
        locals.var_xmp_dn6 = assign26380_e24935_d_n6;
        locals.var_xmp_dn7 = assign26380_e24935_d_n7;
        locals.var_xmp_dn8 = assign26380_e24935_d_n8;
        locals.var_xmp_dn9 = assign26380_e24935_d_n9;
        locals.var_xmp_dn10 = assign26380_e24935_d_n10;
        locals.var_xmp_dn11 = assign26380_e24935_d_n11;
        locals.var_xmp_dn14 = assign26380_e24935_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        locals: &mut StampLocals,
    ) {
        let (assign26390_e24948, assign26390_e24948_d_n0, assign26390_e24948_d_n2, assign26390_e24948_d_n4, assign26390_e24948_d_n5, assign26390_e24948_d_n6, assign26390_e24948_d_n7, assign26390_e24948_d_n8, assign26390_e24948_d_n9, assign26390_e24948_d_n10, assign26390_e24948_d_n11, assign26390_e24948_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26390_e24946: f64 = (locals.var_xp + locals.var_xmp);
        (assign26390_e24946, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26390_e24948;
        locals.var_arg_dn0 = assign26390_e24948_d_n0;
        locals.var_arg_dn2 = assign26390_e24948_d_n2;
        locals.var_arg_dn4 = assign26390_e24948_d_n4;
        locals.var_arg_dn5 = assign26390_e24948_d_n5;
        locals.var_arg_dn6 = assign26390_e24948_d_n6;
        locals.var_arg_dn7 = assign26390_e24948_d_n7;
        locals.var_arg_dn8 = assign26390_e24948_d_n8;
        locals.var_arg_dn9 = assign26390_e24948_d_n9;
        locals.var_arg_dn10 = assign26390_e24948_d_n10;
        locals.var_arg_dn11 = assign26390_e24948_d_n11;
        locals.var_arg_dn14 = assign26390_e24948_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26400_e24959, assign26400_e24959_d_n0, assign26400_e24959_d_n2, assign26400_e24959_d_n4, assign26400_e24959_d_n5, assign26400_e24959_d_n6, assign26400_e24959_d_n7, assign26400_e24959_d_n8, assign26400_e24959_d_n9, assign26400_e24959_d_n10, assign26400_e24959_d_n11, assign26400_e24959_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26400_e24959;
        locals.var_dnm_dn0 = assign26400_e24959_d_n0;
        locals.var_dnm_dn2 = assign26400_e24959_d_n2;
        locals.var_dnm_dn4 = assign26400_e24959_d_n4;
        locals.var_dnm_dn5 = assign26400_e24959_d_n5;
        locals.var_dnm_dn6 = assign26400_e24959_d_n6;
        locals.var_dnm_dn7 = assign26400_e24959_d_n7;
        locals.var_dnm_dn8 = assign26400_e24959_d_n8;
        locals.var_dnm_dn9 = assign26400_e24959_d_n9;
        locals.var_dnm_dn10 = assign26400_e24959_d_n10;
        locals.var_dnm_dn11 = assign26400_e24959_d_n11;
        locals.var_dnm_dn14 = assign26400_e24959_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign26410_e24974: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard636 = assign26410_e24974;
        locals.var_guard636_rv = 0.0;

        let assign26420_e24977: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign26420_e24977;
        locals.var_guard637_rv = 0.0;

        let (assign26430_e24992,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26430_e24992;
        locals.var_mm_rv = 0.0;

        let assign26440_e24995: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign26440_e24995;
        locals.var_guard638_rv = 0.0;

        let (assign26450_e25013,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26450_e25013;
        locals.var_mm_rv = 0.0;

        let assign26460_e25016: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign26460_e25016;
        locals.var_guard639_rv = 0.0;

        let (assign26470_e25037,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26470_e25037;
        locals.var_mm_rv = 0.0;

        let assign26480_e25040: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign26480_e25040;
        locals.var_guard640_rv = 0.0;

        let (assign26490_e25064,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 == 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26490_e25064;
        locals.var_mm_rv = 0.0;

        let (assign26500_e25077,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26500_e25077;
        locals.var_m0_rv = 0.0;

        let mut assign26510_loop_guard: usize = 0;
        while {
            let assign26510_cond_e25091: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26510_cond_e25091 != 0.0
        } {
            assign26510_loop_guard += 1;
            assert!(assign26510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26510_body0_e25105, assign26510_body0_e25105_d_n0, assign26510_body0_e25105_d_n2, assign26510_body0_e25105_d_n4, assign26510_body0_e25105_d_n5, assign26510_body0_e25105_d_n6, assign26510_body0_e25105_d_n7, assign26510_body0_e25105_d_n8, assign26510_body0_e25105_d_n9, assign26510_body0_e25105_d_n10, assign26510_body0_e25105_d_n11, assign26510_body0_e25105_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign26510_body0_e25103: f64 = (locals.var_dnm).sqrt();
        (assign26510_body0_e25103, (locals.var_dnm_dn0 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn2 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn4 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn5 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn6 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn7 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn8 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn9 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn10 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn11 / (2.0 * assign26510_body0_e25103)), (locals.var_dnm_dn14 / (2.0 * assign26510_body0_e25103)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign26510_body0_e25105;
            locals.var_dnm_dn0 = assign26510_body0_e25105_d_n0;
            locals.var_dnm_dn2 = assign26510_body0_e25105_d_n2;
            locals.var_dnm_dn4 = assign26510_body0_e25105_d_n4;
            locals.var_dnm_dn5 = assign26510_body0_e25105_d_n5;
            locals.var_dnm_dn6 = assign26510_body0_e25105_d_n6;
            locals.var_dnm_dn7 = assign26510_body0_e25105_d_n7;
            locals.var_dnm_dn8 = assign26510_body0_e25105_d_n8;
            locals.var_dnm_dn9 = assign26510_body0_e25105_d_n9;
            locals.var_dnm_dn10 = assign26510_body0_e25105_d_n10;
            locals.var_dnm_dn11 = assign26510_body0_e25105_d_n11;
            locals.var_dnm_dn14 = assign26510_body0_e25105_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign26510_body1_e25120,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign26510_body1_e25118: f64 = (locals.var_m0 + 1.0);
        (assign26510_body1_e25118,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26510_body1_e25120;
            locals.var_m0_rv = 0.0;
        }

        let (assign26520_e25145, assign26520_e25145_d_n0, assign26520_e25145_d_n2, assign26520_e25145_d_n4, assign26520_e25145_d_n5, assign26520_e25145_d_n6, assign26520_e25145_d_n7, assign26520_e25145_d_n8, assign26520_e25145_d_n9, assign26520_e25145_d_n10, assign26520_e25145_d_n11, assign26520_e25145_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 == 0.0)) {
        let (assign26520_e25143, assign26520_e25143_d_n0, assign26520_e25143_d_n2, assign26520_e25143_d_n4, assign26520_e25143_d_n5, assign26520_e25143_d_n6, assign26520_e25143_d_n7, assign26520_e25143_d_n8, assign26520_e25143_d_n9, assign26520_e25143_d_n10, assign26520_e25143_d_n11, assign26520_e25143_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26520_e25140: f64 = (2.0 * 2.0);
                let assign26520_e25141: f64 = (1.0 / assign26520_e25140);
                let assign26520_e25142: f64 = (locals.var_dnm).powf(assign26520_e25141);
                (assign26520_e25142, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn11)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26520_e25141) as f64).is_finite() && ((assign26520_e25141) as f64).fract() == 0.0 { if assign26520_e25141 == 0.0 { 0.0 } else { (assign26520_e25141 * ((locals.var_dnm).powf(assign26520_e25141 - 1.0) * locals.var_dnm_dn14)) } } else { (assign26520_e25142 * (assign26520_e25141 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign26520_e25143, assign26520_e25143_d_n0, assign26520_e25143_d_n2, assign26520_e25143_d_n4, assign26520_e25143_d_n5, assign26520_e25143_d_n6, assign26520_e25143_d_n7, assign26520_e25143_d_n8, assign26520_e25143_d_n9, assign26520_e25143_d_n10, assign26520_e25143_d_n11, assign26520_e25143_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26520_e25145;
        locals.var_dnm_dn0 = assign26520_e25145_d_n0;
        locals.var_dnm_dn2 = assign26520_e25145_d_n2;
        locals.var_dnm_dn4 = assign26520_e25145_d_n4;
        locals.var_dnm_dn5 = assign26520_e25145_d_n5;
        locals.var_dnm_dn6 = assign26520_e25145_d_n6;
        locals.var_dnm_dn7 = assign26520_e25145_d_n7;
        locals.var_dnm_dn8 = assign26520_e25145_d_n8;
        locals.var_dnm_dn9 = assign26520_e25145_d_n9;
        locals.var_dnm_dn10 = assign26520_e25145_d_n10;
        locals.var_dnm_dn11 = assign26520_e25145_d_n11;
        locals.var_dnm_dn14 = assign26520_e25145_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26530_e25158, assign26530_e25158_d_n0, assign26530_e25158_d_n2, assign26530_e25158_d_n4, assign26530_e25158_d_n5, assign26530_e25158_d_n6, assign26530_e25158_d_n7, assign26530_e25158_d_n8, assign26530_e25158_d_n9, assign26530_e25158_d_n10, assign26530_e25158_d_n11, assign26530_e25158_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26530_e25156: f64 = (1.0 / locals.var_dnm);
        (assign26530_e25156, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26530_e25158;
        locals.var_dnm_dn0 = assign26530_e25158_d_n0;
        locals.var_dnm_dn2 = assign26530_e25158_d_n2;
        locals.var_dnm_dn4 = assign26530_e25158_d_n4;
        locals.var_dnm_dn5 = assign26530_e25158_d_n5;
        locals.var_dnm_dn6 = assign26530_e25158_d_n6;
        locals.var_dnm_dn7 = assign26530_e25158_d_n7;
        locals.var_dnm_dn8 = assign26530_e25158_d_n8;
        locals.var_dnm_dn9 = assign26530_e25158_d_n9;
        locals.var_dnm_dn10 = assign26530_e25158_d_n10;
        locals.var_dnm_dn11 = assign26530_e25158_d_n11;
        locals.var_dnm_dn14 = assign26530_e25158_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26540_e25173, assign26540_e25173_d_n0, assign26540_e25173_d_n2, assign26540_e25173_d_n4, assign26540_e25173_d_n5, assign26540_e25173_d_n6, assign26540_e25173_d_n7, assign26540_e25173_d_n8, assign26540_e25173_d_n9, assign26540_e25173_d_n10, assign26540_e25173_d_n11, assign26540_e25173_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26540_e25169: f64 = (locals.var_tmf1 * 0.1);
        let assign26540_e25171: f64 = (assign26540_e25169 * locals.var_dnm);
        (assign26540_e25171, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign26540_e25169 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign26540_e25173;
        locals.var_tmf0_dn0 = assign26540_e25173_d_n0;
        locals.var_tmf0_dn2 = assign26540_e25173_d_n2;
        locals.var_tmf0_dn4 = assign26540_e25173_d_n4;
        locals.var_tmf0_dn5 = assign26540_e25173_d_n5;
        locals.var_tmf0_dn6 = assign26540_e25173_d_n6;
        locals.var_tmf0_dn7 = assign26540_e25173_d_n7;
        locals.var_tmf0_dn8 = assign26540_e25173_d_n8;
        locals.var_tmf0_dn9 = assign26540_e25173_d_n9;
        locals.var_tmf0_dn10 = assign26540_e25173_d_n10;
        locals.var_tmf0_dn11 = assign26540_e25173_d_n11;
        locals.var_tmf0_dn14 = assign26540_e25173_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign26550_e25190, assign26550_e25190_d_n0, assign26550_e25190_d_n2, assign26550_e25190_d_n4, assign26550_e25190_d_n5, assign26550_e25190_d_n6, assign26550_e25190_d_n7, assign26550_e25190_d_n8, assign26550_e25190_d_n9, assign26550_e25190_d_n10, assign26550_e25190_d_n11, assign26550_e25190_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26550_e25184: f64 = (0.1 * locals.var_xmp);
        let assign26550_e25186: f64 = (assign26550_e25184 * locals.var_dnm);
        let assign26550_e25188: f64 = (assign26550_e25186 / locals.var_arg);
        (assign26550_e25188, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn11)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign26550_e25184 * locals.var_dnm_dn14)) * locals.var_arg) - (assign26550_e25186 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign26550_e25190;
        locals.var_t0_dn0 = assign26550_e25190_d_n0;
        locals.var_t0_dn2 = assign26550_e25190_d_n2;
        locals.var_t0_dn4 = assign26550_e25190_d_n4;
        locals.var_t0_dn5 = assign26550_e25190_d_n5;
        locals.var_t0_dn6 = assign26550_e25190_d_n6;
        locals.var_t0_dn7 = assign26550_e25190_d_n7;
        locals.var_t0_dn8 = assign26550_e25190_d_n8;
        locals.var_t0_dn9 = assign26550_e25190_d_n9;
        locals.var_t0_dn10 = assign26550_e25190_d_n10;
        locals.var_t0_dn11 = assign26550_e25190_d_n11;
        locals.var_t0_dn14 = assign26550_e25190_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign26560_e25205, assign26560_e25205_d_n0, assign26560_e25205_d_n2, assign26560_e25205_d_n4, assign26560_e25205_d_n5, assign26560_e25205_d_n6, assign26560_e25205_d_n7, assign26560_e25205_d_n8, assign26560_e25205_d_n9, assign26560_e25205_d_n10, assign26560_e25205_d_n11, assign26560_e25205_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign26560_e25201: f64 = 0.1;
        let assign26560_e25203: f64 = (assign26560_e25201 - locals.var_tmf0);
        (assign26560_e25203, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26560_e25205;
        locals.var_t2_dn0 = assign26560_e25205_d_n0;
        locals.var_t2_dn2 = assign26560_e25205_d_n2;
        locals.var_t2_dn4 = assign26560_e25205_d_n4;
        locals.var_t2_dn5 = assign26560_e25205_d_n5;
        locals.var_t2_dn6 = assign26560_e25205_d_n6;
        locals.var_t2_dn7 = assign26560_e25205_d_n7;
        locals.var_t2_dn8 = assign26560_e25205_d_n8;
        locals.var_t2_dn9 = assign26560_e25205_d_n9;
        locals.var_t2_dn10 = assign26560_e25205_d_n10;
        locals.var_t2_dn11 = assign26560_e25205_d_n11;
        locals.var_t2_dn14 = assign26560_e25205_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26570_e25216, assign26570_e25216_d_n0, assign26570_e25216_d_n2, assign26570_e25216_d_n4, assign26570_e25216_d_n5, assign26570_e25216_d_n6, assign26570_e25216_d_n7, assign26570_e25216_d_n8, assign26570_e25216_d_n9, assign26570_e25216_d_n10, assign26570_e25216_d_n11, assign26570_e25216_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign26570_e25216;
        locals.var_t0_dn0 = assign26570_e25216_d_n0;
        locals.var_t0_dn2 = assign26570_e25216_d_n2;
        locals.var_t0_dn4 = assign26570_e25216_d_n4;
        locals.var_t0_dn5 = assign26570_e25216_d_n5;
        locals.var_t0_dn6 = assign26570_e25216_d_n6;
        locals.var_t0_dn7 = assign26570_e25216_d_n7;
        locals.var_t0_dn8 = assign26570_e25216_d_n8;
        locals.var_t0_dn9 = assign26570_e25216_d_n9;
        locals.var_t0_dn10 = assign26570_e25216_d_n10;
        locals.var_t0_dn11 = assign26570_e25216_d_n11;
        locals.var_t0_dn14 = assign26570_e25216_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign26580_e25228, assign26580_e25228_d_n0, assign26580_e25228_d_n2, assign26580_e25228_d_n4, assign26580_e25228_d_n5, assign26580_e25228_d_n6, assign26580_e25228_d_n7, assign26580_e25228_d_n8, assign26580_e25228_d_n9, assign26580_e25228_d_n10, assign26580_e25228_d_n11, assign26580_e25228_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26580_e25228;
        locals.var_t2_dn0 = assign26580_e25228_d_n0;
        locals.var_t2_dn2 = assign26580_e25228_d_n2;
        locals.var_t2_dn4 = assign26580_e25228_d_n4;
        locals.var_t2_dn5 = assign26580_e25228_d_n5;
        locals.var_t2_dn6 = assign26580_e25228_d_n6;
        locals.var_t2_dn7 = assign26580_e25228_d_n7;
        locals.var_t2_dn8 = assign26580_e25228_d_n8;
        locals.var_t2_dn9 = assign26580_e25228_d_n9;
        locals.var_t2_dn10 = assign26580_e25228_d_n10;
        locals.var_t2_dn11 = assign26580_e25228_d_n11;
        locals.var_t2_dn14 = assign26580_e25228_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26590_e25240, assign26590_e25240_d_n0, assign26590_e25240_d_n2, assign26590_e25240_d_n4, assign26590_e25240_d_n5, assign26590_e25240_d_n6, assign26590_e25240_d_n7, assign26590_e25240_d_n8, assign26590_e25240_d_n9, assign26590_e25240_d_n10, assign26590_e25240_d_n11, assign26590_e25240_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard635 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign26590_e25240;
        locals.var_t0_dn0 = assign26590_e25240_d_n0;
        locals.var_t0_dn2 = assign26590_e25240_d_n2;
        locals.var_t0_dn4 = assign26590_e25240_d_n4;
        locals.var_t0_dn5 = assign26590_e25240_d_n5;
        locals.var_t0_dn6 = assign26590_e25240_d_n6;
        locals.var_t0_dn7 = assign26590_e25240_d_n7;
        locals.var_t0_dn8 = assign26590_e25240_d_n8;
        locals.var_t0_dn9 = assign26590_e25240_d_n9;
        locals.var_t0_dn10 = assign26590_e25240_d_n10;
        locals.var_t0_dn11 = assign26590_e25240_d_n11;
        locals.var_t0_dn14 = assign26590_e25240_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign26600_e25252, assign26600_e25252_d_n0, assign26600_e25252_d_n2, assign26600_e25252_d_n4, assign26600_e25252_d_n5, assign26600_e25252_d_n6, assign26600_e25252_d_n7, assign26600_e25252_d_n8, assign26600_e25252_d_n9, assign26600_e25252_d_n10, assign26600_e25252_d_n11, assign26600_e25252_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26600_e25249: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign26600_e25250: f64 = (assign26600_e25249).sqrt();
        (assign26600_e25250, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign26600_e25250)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign26600_e25250)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26600_e25252;
        locals.var_w_b0_dn0 = assign26600_e25252_d_n0;
        locals.var_w_b0_dn2 = assign26600_e25252_d_n2;
        locals.var_w_b0_dn4 = assign26600_e25252_d_n4;
        locals.var_w_b0_dn5 = assign26600_e25252_d_n5;
        locals.var_w_b0_dn6 = assign26600_e25252_d_n6;
        locals.var_w_b0_dn7 = assign26600_e25252_d_n7;
        locals.var_w_b0_dn8 = assign26600_e25252_d_n8;
        locals.var_w_b0_dn9 = assign26600_e25252_d_n9;
        locals.var_w_b0_dn10 = assign26600_e25252_d_n10;
        locals.var_w_b0_dn11 = assign26600_e25252_d_n11;
        locals.var_w_b0_dn14 = assign26600_e25252_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign26610_e25256: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26610_e25261: f64 = if ((locals.var_w_b0 > assign26610_e25256) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard641 = assign26610_e25261;
        locals.var_guard641_rv = 0.0;

        let (assign26620_e25276, assign26620_e25276_d_n0, assign26620_e25276_d_n2, assign26620_e25276_d_n4, assign26620_e25276_d_n5, assign26620_e25276_d_n6, assign26620_e25276_d_n7, assign26620_e25276_d_n8, assign26620_e25276_d_n9, assign26620_e25276_d_n10, assign26620_e25276_d_n11, assign26620_e25276_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26620_e25272: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign26620_e25274: f64 = (assign26620_e25272 + 1e-8);
        (assign26620_e25274, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign26620_e25276;
        locals.var_tmf1_dn0 = assign26620_e25276_d_n0;
        locals.var_tmf1_dn2 = assign26620_e25276_d_n2;
        locals.var_tmf1_dn4 = assign26620_e25276_d_n4;
        locals.var_tmf1_dn5 = assign26620_e25276_d_n5;
        locals.var_tmf1_dn6 = assign26620_e25276_d_n6;
        locals.var_tmf1_dn7 = assign26620_e25276_d_n7;
        locals.var_tmf1_dn8 = assign26620_e25276_d_n8;
        locals.var_tmf1_dn9 = assign26620_e25276_d_n9;
        locals.var_tmf1_dn10 = assign26620_e25276_d_n10;
        locals.var_tmf1_dn11 = assign26620_e25276_d_n11;
        locals.var_tmf1_dn14 = assign26620_e25276_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign26630_e25289, assign26630_e25289_d_n0, assign26630_e25289_d_n2, assign26630_e25289_d_n4, assign26630_e25289_d_n5, assign26630_e25289_d_n6, assign26630_e25289_d_n7, assign26630_e25289_d_n8, assign26630_e25289_d_n9, assign26630_e25289_d_n10, assign26630_e25289_d_n11, assign26630_e25289_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26630_e25287: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26630_e25287, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign26630_e25289;
        locals.var_x2_dn0 = assign26630_e25289_d_n0;
        locals.var_x2_dn2 = assign26630_e25289_d_n2;
        locals.var_x2_dn4 = assign26630_e25289_d_n4;
        locals.var_x2_dn5 = assign26630_e25289_d_n5;
        locals.var_x2_dn6 = assign26630_e25289_d_n6;
        locals.var_x2_dn7 = assign26630_e25289_d_n7;
        locals.var_x2_dn8 = assign26630_e25289_d_n8;
        locals.var_x2_dn9 = assign26630_e25289_d_n9;
        locals.var_x2_dn10 = assign26630_e25289_d_n10;
        locals.var_x2_dn11 = assign26630_e25289_d_n11;
        locals.var_x2_dn14 = assign26630_e25289_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign26640_e25302, assign26640_e25302_d_n0, assign26640_e25302_d_n2, assign26640_e25302_d_n4, assign26640_e25302_d_n5, assign26640_e25302_d_n6, assign26640_e25302_d_n7, assign26640_e25302_d_n8, assign26640_e25302_d_n9, assign26640_e25302_d_n10, assign26640_e25302_d_n11, assign26640_e25302_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26640_e25300: f64 = (1e-8 * 1e-8);
        (assign26640_e25300, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign26640_e25302;
        locals.var_xmax2_dn0 = assign26640_e25302_d_n0;
        locals.var_xmax2_dn2 = assign26640_e25302_d_n2;
        locals.var_xmax2_dn4 = assign26640_e25302_d_n4;
        locals.var_xmax2_dn5 = assign26640_e25302_d_n5;
        locals.var_xmax2_dn6 = assign26640_e25302_d_n6;
        locals.var_xmax2_dn7 = assign26640_e25302_d_n7;
        locals.var_xmax2_dn8 = assign26640_e25302_d_n8;
        locals.var_xmax2_dn9 = assign26640_e25302_d_n9;
        locals.var_xmax2_dn10 = assign26640_e25302_d_n10;
        locals.var_xmax2_dn11 = assign26640_e25302_d_n11;
        locals.var_xmax2_dn14 = assign26640_e25302_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign26650_e25313, assign26650_e25313_d_n0, assign26650_e25313_d_n2, assign26650_e25313_d_n4, assign26650_e25313_d_n5, assign26650_e25313_d_n6, assign26650_e25313_d_n7, assign26650_e25313_d_n8, assign26650_e25313_d_n9, assign26650_e25313_d_n10, assign26650_e25313_d_n11, assign26650_e25313_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26650_e25313;
        locals.var_xp_dn0 = assign26650_e25313_d_n0;
        locals.var_xp_dn2 = assign26650_e25313_d_n2;
        locals.var_xp_dn4 = assign26650_e25313_d_n4;
        locals.var_xp_dn5 = assign26650_e25313_d_n5;
        locals.var_xp_dn6 = assign26650_e25313_d_n6;
        locals.var_xp_dn7 = assign26650_e25313_d_n7;
        locals.var_xp_dn8 = assign26650_e25313_d_n8;
        locals.var_xp_dn9 = assign26650_e25313_d_n9;
        locals.var_xp_dn10 = assign26650_e25313_d_n10;
        locals.var_xp_dn11 = assign26650_e25313_d_n11;
        locals.var_xp_dn14 = assign26650_e25313_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26660_e25324, assign26660_e25324_d_n0, assign26660_e25324_d_n2, assign26660_e25324_d_n4, assign26660_e25324_d_n5, assign26660_e25324_d_n6, assign26660_e25324_d_n7, assign26660_e25324_d_n8, assign26660_e25324_d_n9, assign26660_e25324_d_n10, assign26660_e25324_d_n11, assign26660_e25324_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26660_e25324;
        locals.var_xmp_dn0 = assign26660_e25324_d_n0;
        locals.var_xmp_dn2 = assign26660_e25324_d_n2;
        locals.var_xmp_dn4 = assign26660_e25324_d_n4;
        locals.var_xmp_dn5 = assign26660_e25324_d_n5;
        locals.var_xmp_dn6 = assign26660_e25324_d_n6;
        locals.var_xmp_dn7 = assign26660_e25324_d_n7;
        locals.var_xmp_dn8 = assign26660_e25324_d_n8;
        locals.var_xmp_dn9 = assign26660_e25324_d_n9;
        locals.var_xmp_dn10 = assign26660_e25324_d_n10;
        locals.var_xmp_dn11 = assign26660_e25324_d_n11;
        locals.var_xmp_dn14 = assign26660_e25324_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26670_e25335,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26670_e25335;
        locals.var_m0_rv = 0.0;

        let (assign26680_e25346,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26680_e25346;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign26690_e25357, assign26690_e25357_d_n0, assign26690_e25357_d_n2, assign26690_e25357_d_n4, assign26690_e25357_d_n5, assign26690_e25357_d_n6, assign26690_e25357_d_n7, assign26690_e25357_d_n8, assign26690_e25357_d_n9, assign26690_e25357_d_n10, assign26690_e25357_d_n11, assign26690_e25357_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26690_e25357;
        locals.var_arg_dn0 = assign26690_e25357_d_n0;
        locals.var_arg_dn2 = assign26690_e25357_d_n2;
        locals.var_arg_dn4 = assign26690_e25357_d_n4;
        locals.var_arg_dn5 = assign26690_e25357_d_n5;
        locals.var_arg_dn6 = assign26690_e25357_d_n6;
        locals.var_arg_dn7 = assign26690_e25357_d_n7;
        locals.var_arg_dn8 = assign26690_e25357_d_n8;
        locals.var_arg_dn9 = assign26690_e25357_d_n9;
        locals.var_arg_dn10 = assign26690_e25357_d_n10;
        locals.var_arg_dn11 = assign26690_e25357_d_n11;
        locals.var_arg_dn14 = assign26690_e25357_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26700_e25368, assign26700_e25368_d_n0, assign26700_e25368_d_n2, assign26700_e25368_d_n4, assign26700_e25368_d_n5, assign26700_e25368_d_n6, assign26700_e25368_d_n7, assign26700_e25368_d_n8, assign26700_e25368_d_n9, assign26700_e25368_d_n10, assign26700_e25368_d_n11, assign26700_e25368_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26700_e25368;
        locals.var_dnm_dn0 = assign26700_e25368_d_n0;
        locals.var_dnm_dn2 = assign26700_e25368_d_n2;
        locals.var_dnm_dn4 = assign26700_e25368_d_n4;
        locals.var_dnm_dn5 = assign26700_e25368_d_n5;
        locals.var_dnm_dn6 = assign26700_e25368_d_n6;
        locals.var_dnm_dn7 = assign26700_e25368_d_n7;
        locals.var_dnm_dn8 = assign26700_e25368_d_n8;
        locals.var_dnm_dn9 = assign26700_e25368_d_n9;
        locals.var_dnm_dn10 = assign26700_e25368_d_n10;
        locals.var_dnm_dn11 = assign26700_e25368_d_n11;
        locals.var_dnm_dn14 = assign26700_e25368_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26710_e25381, assign26710_e25381_d_n0, assign26710_e25381_d_n2, assign26710_e25381_d_n4, assign26710_e25381_d_n5, assign26710_e25381_d_n6, assign26710_e25381_d_n7, assign26710_e25381_d_n8, assign26710_e25381_d_n9, assign26710_e25381_d_n10, assign26710_e25381_d_n11, assign26710_e25381_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26710_e25379: f64 = (locals.var_xp * locals.var_x2);
        (assign26710_e25379, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26710_e25381;
        locals.var_xp_dn0 = assign26710_e25381_d_n0;
        locals.var_xp_dn2 = assign26710_e25381_d_n2;
        locals.var_xp_dn4 = assign26710_e25381_d_n4;
        locals.var_xp_dn5 = assign26710_e25381_d_n5;
        locals.var_xp_dn6 = assign26710_e25381_d_n6;
        locals.var_xp_dn7 = assign26710_e25381_d_n7;
        locals.var_xp_dn8 = assign26710_e25381_d_n8;
        locals.var_xp_dn9 = assign26710_e25381_d_n9;
        locals.var_xp_dn10 = assign26710_e25381_d_n10;
        locals.var_xp_dn11 = assign26710_e25381_d_n11;
        locals.var_xp_dn14 = assign26710_e25381_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26720_e25394, assign26720_e25394_d_n0, assign26720_e25394_d_n2, assign26720_e25394_d_n4, assign26720_e25394_d_n5, assign26720_e25394_d_n6, assign26720_e25394_d_n7, assign26720_e25394_d_n8, assign26720_e25394_d_n9, assign26720_e25394_d_n10, assign26720_e25394_d_n11, assign26720_e25394_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26720_e25392: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26720_e25392, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26720_e25394;
        locals.var_xmp_dn0 = assign26720_e25394_d_n0;
        locals.var_xmp_dn2 = assign26720_e25394_d_n2;
        locals.var_xmp_dn4 = assign26720_e25394_d_n4;
        locals.var_xmp_dn5 = assign26720_e25394_d_n5;
        locals.var_xmp_dn6 = assign26720_e25394_d_n6;
        locals.var_xmp_dn7 = assign26720_e25394_d_n7;
        locals.var_xmp_dn8 = assign26720_e25394_d_n8;
        locals.var_xmp_dn9 = assign26720_e25394_d_n9;
        locals.var_xmp_dn10 = assign26720_e25394_d_n10;
        locals.var_xmp_dn11 = assign26720_e25394_d_n11;
        locals.var_xmp_dn14 = assign26720_e25394_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26730_e25407, assign26730_e25407_d_n0, assign26730_e25407_d_n2, assign26730_e25407_d_n4, assign26730_e25407_d_n5, assign26730_e25407_d_n6, assign26730_e25407_d_n7, assign26730_e25407_d_n8, assign26730_e25407_d_n9, assign26730_e25407_d_n10, assign26730_e25407_d_n11, assign26730_e25407_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26730_e25405: f64 = (locals.var_xp * locals.var_x2);
        (assign26730_e25405, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26730_e25407;
        locals.var_xp_dn0 = assign26730_e25407_d_n0;
        locals.var_xp_dn2 = assign26730_e25407_d_n2;
        locals.var_xp_dn4 = assign26730_e25407_d_n4;
        locals.var_xp_dn5 = assign26730_e25407_d_n5;
        locals.var_xp_dn6 = assign26730_e25407_d_n6;
        locals.var_xp_dn7 = assign26730_e25407_d_n7;
        locals.var_xp_dn8 = assign26730_e25407_d_n8;
        locals.var_xp_dn9 = assign26730_e25407_d_n9;
        locals.var_xp_dn10 = assign26730_e25407_d_n10;
        locals.var_xp_dn11 = assign26730_e25407_d_n11;
        locals.var_xp_dn14 = assign26730_e25407_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26740_e25420, assign26740_e25420_d_n0, assign26740_e25420_d_n2, assign26740_e25420_d_n4, assign26740_e25420_d_n5, assign26740_e25420_d_n6, assign26740_e25420_d_n7, assign26740_e25420_d_n8, assign26740_e25420_d_n9, assign26740_e25420_d_n10, assign26740_e25420_d_n11, assign26740_e25420_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26740_e25418: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26740_e25418, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26740_e25420;
        locals.var_xmp_dn0 = assign26740_e25420_d_n0;
        locals.var_xmp_dn2 = assign26740_e25420_d_n2;
        locals.var_xmp_dn4 = assign26740_e25420_d_n4;
        locals.var_xmp_dn5 = assign26740_e25420_d_n5;
        locals.var_xmp_dn6 = assign26740_e25420_d_n6;
        locals.var_xmp_dn7 = assign26740_e25420_d_n7;
        locals.var_xmp_dn8 = assign26740_e25420_d_n8;
        locals.var_xmp_dn9 = assign26740_e25420_d_n9;
        locals.var_xmp_dn10 = assign26740_e25420_d_n10;
        locals.var_xmp_dn11 = assign26740_e25420_d_n11;
        locals.var_xmp_dn14 = assign26740_e25420_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26750_e25433, assign26750_e25433_d_n0, assign26750_e25433_d_n2, assign26750_e25433_d_n4, assign26750_e25433_d_n5, assign26750_e25433_d_n6, assign26750_e25433_d_n7, assign26750_e25433_d_n8, assign26750_e25433_d_n9, assign26750_e25433_d_n10, assign26750_e25433_d_n11, assign26750_e25433_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26750_e25431: f64 = (locals.var_xp + locals.var_xmp);
        (assign26750_e25431, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26750_e25433;
        locals.var_arg_dn0 = assign26750_e25433_d_n0;
        locals.var_arg_dn2 = assign26750_e25433_d_n2;
        locals.var_arg_dn4 = assign26750_e25433_d_n4;
        locals.var_arg_dn5 = assign26750_e25433_d_n5;
        locals.var_arg_dn6 = assign26750_e25433_d_n6;
        locals.var_arg_dn7 = assign26750_e25433_d_n7;
        locals.var_arg_dn8 = assign26750_e25433_d_n8;
        locals.var_arg_dn9 = assign26750_e25433_d_n9;
        locals.var_arg_dn10 = assign26750_e25433_d_n10;
        locals.var_arg_dn11 = assign26750_e25433_d_n11;
        locals.var_arg_dn14 = assign26750_e25433_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26760_e25444, assign26760_e25444_d_n0, assign26760_e25444_d_n2, assign26760_e25444_d_n4, assign26760_e25444_d_n5, assign26760_e25444_d_n6, assign26760_e25444_d_n7, assign26760_e25444_d_n8, assign26760_e25444_d_n9, assign26760_e25444_d_n10, assign26760_e25444_d_n11, assign26760_e25444_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26760_e25444;
        locals.var_dnm_dn0 = assign26760_e25444_d_n0;
        locals.var_dnm_dn2 = assign26760_e25444_d_n2;
        locals.var_dnm_dn4 = assign26760_e25444_d_n4;
        locals.var_dnm_dn5 = assign26760_e25444_d_n5;
        locals.var_dnm_dn6 = assign26760_e25444_d_n6;
        locals.var_dnm_dn7 = assign26760_e25444_d_n7;
        locals.var_dnm_dn8 = assign26760_e25444_d_n8;
        locals.var_dnm_dn9 = assign26760_e25444_d_n9;
        locals.var_dnm_dn10 = assign26760_e25444_d_n10;
        locals.var_dnm_dn11 = assign26760_e25444_d_n11;
        locals.var_dnm_dn14 = assign26760_e25444_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign26770_e25459: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard642 = assign26770_e25459;
        locals.var_guard642_rv = 0.0;

        let assign26780_e25462: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard643 = assign26780_e25462;
        locals.var_guard643_rv = 0.0;

        let (assign26790_e25477,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26790_e25477;
        locals.var_mm_rv = 0.0;

        let assign26800_e25480: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign26800_e25480;
        locals.var_guard644_rv = 0.0;

        let (assign26810_e25498,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard644 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26810_e25498;
        locals.var_mm_rv = 0.0;

        let assign26820_e25501: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard645 = assign26820_e25501;
        locals.var_guard645_rv = 0.0;

        let (assign26830_e25522,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard644 == 0.0)) && (locals.var_guard645 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26830_e25522;
        locals.var_mm_rv = 0.0;

        let assign26840_e25525: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign26840_e25525;
        locals.var_guard646_rv = 0.0;

        let (assign26850_e25549,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard644 == 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard646 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26850_e25549;
        locals.var_mm_rv = 0.0;

        let (assign26860_e25562,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26860_e25562;
        locals.var_m0_rv = 0.0;

        let mut assign26870_loop_guard: usize = 0;
        while {
            let assign26870_cond_e25576: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26870_cond_e25576 != 0.0
        } {
            assign26870_loop_guard += 1;
            assert!(assign26870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26870_body0_e25590, assign26870_body0_e25590_d_n0, assign26870_body0_e25590_d_n2, assign26870_body0_e25590_d_n4, assign26870_body0_e25590_d_n5, assign26870_body0_e25590_d_n6, assign26870_body0_e25590_d_n7, assign26870_body0_e25590_d_n8, assign26870_body0_e25590_d_n9, assign26870_body0_e25590_d_n10, assign26870_body0_e25590_d_n11, assign26870_body0_e25590_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) {
        let assign26870_body0_e25588: f64 = (locals.var_dnm).sqrt();
        (assign26870_body0_e25588, (locals.var_dnm_dn0 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn2 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn4 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn5 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn6 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn7 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn8 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn9 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn10 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn11 / (2.0 * assign26870_body0_e25588)), (locals.var_dnm_dn14 / (2.0 * assign26870_body0_e25588)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign26870_body0_e25590;
            locals.var_dnm_dn0 = assign26870_body0_e25590_d_n0;
            locals.var_dnm_dn2 = assign26870_body0_e25590_d_n2;
            locals.var_dnm_dn4 = assign26870_body0_e25590_d_n4;
            locals.var_dnm_dn5 = assign26870_body0_e25590_d_n5;
            locals.var_dnm_dn6 = assign26870_body0_e25590_d_n6;
            locals.var_dnm_dn7 = assign26870_body0_e25590_d_n7;
            locals.var_dnm_dn8 = assign26870_body0_e25590_d_n8;
            locals.var_dnm_dn9 = assign26870_body0_e25590_d_n9;
            locals.var_dnm_dn10 = assign26870_body0_e25590_d_n10;
            locals.var_dnm_dn11 = assign26870_body0_e25590_d_n11;
            locals.var_dnm_dn14 = assign26870_body0_e25590_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign26870_body1_e25605,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 != 0.0)) {
        let assign26870_body1_e25603: f64 = (locals.var_m0 + 1.0);
        (assign26870_body1_e25603,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26870_body1_e25605;
            locals.var_m0_rv = 0.0;
        }

        let (assign26880_e25630, assign26880_e25630_d_n0, assign26880_e25630_d_n2, assign26880_e25630_d_n4, assign26880_e25630_d_n5, assign26880_e25630_d_n6, assign26880_e25630_d_n7, assign26880_e25630_d_n8, assign26880_e25630_d_n9, assign26880_e25630_d_n10, assign26880_e25630_d_n11, assign26880_e25630_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) && (locals.var_guard642 == 0.0)) {
        let (assign26880_e25628, assign26880_e25628_d_n0, assign26880_e25628_d_n2, assign26880_e25628_d_n4, assign26880_e25628_d_n5, assign26880_e25628_d_n6, assign26880_e25628_d_n7, assign26880_e25628_d_n8, assign26880_e25628_d_n9, assign26880_e25628_d_n10, assign26880_e25628_d_n11, assign26880_e25628_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26880_e25625: f64 = (2.0 * 2.0);
                let assign26880_e25626: f64 = (1.0 / assign26880_e25625);
                let assign26880_e25627: f64 = (locals.var_dnm).powf(assign26880_e25626);
                (assign26880_e25627, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn11)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26880_e25626) as f64).is_finite() && ((assign26880_e25626) as f64).fract() == 0.0 { if assign26880_e25626 == 0.0 { 0.0 } else { (assign26880_e25626 * ((locals.var_dnm).powf(assign26880_e25626 - 1.0) * locals.var_dnm_dn14)) } } else { (assign26880_e25627 * (assign26880_e25626 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign26880_e25628, assign26880_e25628_d_n0, assign26880_e25628_d_n2, assign26880_e25628_d_n4, assign26880_e25628_d_n5, assign26880_e25628_d_n6, assign26880_e25628_d_n7, assign26880_e25628_d_n8, assign26880_e25628_d_n9, assign26880_e25628_d_n10, assign26880_e25628_d_n11, assign26880_e25628_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26880_e25630;
        locals.var_dnm_dn0 = assign26880_e25630_d_n0;
        locals.var_dnm_dn2 = assign26880_e25630_d_n2;
        locals.var_dnm_dn4 = assign26880_e25630_d_n4;
        locals.var_dnm_dn5 = assign26880_e25630_d_n5;
        locals.var_dnm_dn6 = assign26880_e25630_d_n6;
        locals.var_dnm_dn7 = assign26880_e25630_d_n7;
        locals.var_dnm_dn8 = assign26880_e25630_d_n8;
        locals.var_dnm_dn9 = assign26880_e25630_d_n9;
        locals.var_dnm_dn10 = assign26880_e25630_d_n10;
        locals.var_dnm_dn11 = assign26880_e25630_d_n11;
        locals.var_dnm_dn14 = assign26880_e25630_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26890_e25643, assign26890_e25643_d_n0, assign26890_e25643_d_n2, assign26890_e25643_d_n4, assign26890_e25643_d_n5, assign26890_e25643_d_n6, assign26890_e25643_d_n7, assign26890_e25643_d_n8, assign26890_e25643_d_n9, assign26890_e25643_d_n10, assign26890_e25643_d_n11, assign26890_e25643_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26890_e25641: f64 = (1.0 / locals.var_dnm);
        (assign26890_e25641, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26890_e25643;
        locals.var_dnm_dn0 = assign26890_e25643_d_n0;
        locals.var_dnm_dn2 = assign26890_e25643_d_n2;
        locals.var_dnm_dn4 = assign26890_e25643_d_n4;
        locals.var_dnm_dn5 = assign26890_e25643_d_n5;
        locals.var_dnm_dn6 = assign26890_e25643_d_n6;
        locals.var_dnm_dn7 = assign26890_e25643_d_n7;
        locals.var_dnm_dn8 = assign26890_e25643_d_n8;
        locals.var_dnm_dn9 = assign26890_e25643_d_n9;
        locals.var_dnm_dn10 = assign26890_e25643_d_n10;
        locals.var_dnm_dn11 = assign26890_e25643_d_n11;
        locals.var_dnm_dn14 = assign26890_e25643_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26900_e25658, assign26900_e25658_d_n0, assign26900_e25658_d_n2, assign26900_e25658_d_n4, assign26900_e25658_d_n5, assign26900_e25658_d_n6, assign26900_e25658_d_n7, assign26900_e25658_d_n8, assign26900_e25658_d_n9, assign26900_e25658_d_n10, assign26900_e25658_d_n11, assign26900_e25658_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26900_e25654: f64 = (locals.var_tmf1 * 1e-8);
        let assign26900_e25656: f64 = (assign26900_e25654 * locals.var_dnm);
        (assign26900_e25656, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign26900_e25654 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign26900_e25658;
        locals.var_tmf0_dn0 = assign26900_e25658_d_n0;
        locals.var_tmf0_dn2 = assign26900_e25658_d_n2;
        locals.var_tmf0_dn4 = assign26900_e25658_d_n4;
        locals.var_tmf0_dn5 = assign26900_e25658_d_n5;
        locals.var_tmf0_dn6 = assign26900_e25658_d_n6;
        locals.var_tmf0_dn7 = assign26900_e25658_d_n7;
        locals.var_tmf0_dn8 = assign26900_e25658_d_n8;
        locals.var_tmf0_dn9 = assign26900_e25658_d_n9;
        locals.var_tmf0_dn10 = assign26900_e25658_d_n10;
        locals.var_tmf0_dn11 = assign26900_e25658_d_n11;
        locals.var_tmf0_dn14 = assign26900_e25658_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign26910_e25675, assign26910_e25675_d_n0, assign26910_e25675_d_n2, assign26910_e25675_d_n4, assign26910_e25675_d_n5, assign26910_e25675_d_n6, assign26910_e25675_d_n7, assign26910_e25675_d_n8, assign26910_e25675_d_n9, assign26910_e25675_d_n10, assign26910_e25675_d_n11, assign26910_e25675_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26910_e25669: f64 = (1e-8 * locals.var_xmp);
        let assign26910_e25671: f64 = (assign26910_e25669 * locals.var_dnm);
        let assign26910_e25673: f64 = (assign26910_e25671 / locals.var_arg);
        (assign26910_e25673, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn11)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign26910_e25669 * locals.var_dnm_dn14)) * locals.var_arg) - (assign26910_e25671 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26910_e25675;
        locals.var_t3_dn0 = assign26910_e25675_d_n0;
        locals.var_t3_dn2 = assign26910_e25675_d_n2;
        locals.var_t3_dn4 = assign26910_e25675_d_n4;
        locals.var_t3_dn5 = assign26910_e25675_d_n5;
        locals.var_t3_dn6 = assign26910_e25675_d_n6;
        locals.var_t3_dn7 = assign26910_e25675_d_n7;
        locals.var_t3_dn8 = assign26910_e25675_d_n8;
        locals.var_t3_dn9 = assign26910_e25675_d_n9;
        locals.var_t3_dn10 = assign26910_e25675_d_n10;
        locals.var_t3_dn11 = assign26910_e25675_d_n11;
        locals.var_t3_dn14 = assign26910_e25675_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26920_e25690, assign26920_e25690_d_n0, assign26920_e25690_d_n2, assign26920_e25690_d_n4, assign26920_e25690_d_n5, assign26920_e25690_d_n6, assign26920_e25690_d_n7, assign26920_e25690_d_n8, assign26920_e25690_d_n9, assign26920_e25690_d_n10, assign26920_e25690_d_n11, assign26920_e25690_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        let assign26920_e25686: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26920_e25688: f64 = (assign26920_e25686 + locals.var_tmf0);
        (assign26920_e25688, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26920_e25690;
        locals.var_w_b0_dn0 = assign26920_e25690_d_n0;
        locals.var_w_b0_dn2 = assign26920_e25690_d_n2;
        locals.var_w_b0_dn4 = assign26920_e25690_d_n4;
        locals.var_w_b0_dn5 = assign26920_e25690_d_n5;
        locals.var_w_b0_dn6 = assign26920_e25690_d_n6;
        locals.var_w_b0_dn7 = assign26920_e25690_d_n7;
        locals.var_w_b0_dn8 = assign26920_e25690_d_n8;
        locals.var_w_b0_dn9 = assign26920_e25690_d_n9;
        locals.var_w_b0_dn10 = assign26920_e25690_d_n10;
        locals.var_w_b0_dn11 = assign26920_e25690_d_n11;
        locals.var_w_b0_dn14 = assign26920_e25690_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26930_e25701, assign26930_e25701_d_n0, assign26930_e25701_d_n2, assign26930_e25701_d_n4, assign26930_e25701_d_n5, assign26930_e25701_d_n6, assign26930_e25701_d_n7, assign26930_e25701_d_n8, assign26930_e25701_d_n9, assign26930_e25701_d_n10, assign26930_e25701_d_n11, assign26930_e25701_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26930_e25701;
        locals.var_t3_dn0 = assign26930_e25701_d_n0;
        locals.var_t3_dn2 = assign26930_e25701_d_n2;
        locals.var_t3_dn4 = assign26930_e25701_d_n4;
        locals.var_t3_dn5 = assign26930_e25701_d_n5;
        locals.var_t3_dn6 = assign26930_e25701_d_n6;
        locals.var_t3_dn7 = assign26930_e25701_d_n7;
        locals.var_t3_dn8 = assign26930_e25701_d_n8;
        locals.var_t3_dn9 = assign26930_e25701_d_n9;
        locals.var_t3_dn10 = assign26930_e25701_d_n10;
        locals.var_t3_dn11 = assign26930_e25701_d_n11;
        locals.var_t3_dn14 = assign26930_e25701_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26940_e25713, assign26940_e25713_d_n0, assign26940_e25713_d_n2, assign26940_e25713_d_n4, assign26940_e25713_d_n5, assign26940_e25713_d_n6, assign26940_e25713_d_n7, assign26940_e25713_d_n8, assign26940_e25713_d_n9, assign26940_e25713_d_n10, assign26940_e25713_d_n11, assign26940_e25713_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26940_e25713;
        locals.var_w_b0_dn0 = assign26940_e25713_d_n0;
        locals.var_w_b0_dn2 = assign26940_e25713_d_n2;
        locals.var_w_b0_dn4 = assign26940_e25713_d_n4;
        locals.var_w_b0_dn5 = assign26940_e25713_d_n5;
        locals.var_w_b0_dn6 = assign26940_e25713_d_n6;
        locals.var_w_b0_dn7 = assign26940_e25713_d_n7;
        locals.var_w_b0_dn8 = assign26940_e25713_d_n8;
        locals.var_w_b0_dn9 = assign26940_e25713_d_n9;
        locals.var_w_b0_dn10 = assign26940_e25713_d_n10;
        locals.var_w_b0_dn11 = assign26940_e25713_d_n11;
        locals.var_w_b0_dn14 = assign26940_e25713_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26950_e25725, assign26950_e25725_d_n0, assign26950_e25725_d_n2, assign26950_e25725_d_n4, assign26950_e25725_d_n5, assign26950_e25725_d_n6, assign26950_e25725_d_n7, assign26950_e25725_d_n8, assign26950_e25725_d_n9, assign26950_e25725_d_n10, assign26950_e25725_d_n11, assign26950_e25725_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard641 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26950_e25725;
        locals.var_t3_dn0 = assign26950_e25725_d_n0;
        locals.var_t3_dn2 = assign26950_e25725_d_n2;
        locals.var_t3_dn4 = assign26950_e25725_d_n4;
        locals.var_t3_dn5 = assign26950_e25725_d_n5;
        locals.var_t3_dn6 = assign26950_e25725_d_n6;
        locals.var_t3_dn7 = assign26950_e25725_d_n7;
        locals.var_t3_dn8 = assign26950_e25725_d_n8;
        locals.var_t3_dn9 = assign26950_e25725_d_n9;
        locals.var_t3_dn10 = assign26950_e25725_d_n10;
        locals.var_t3_dn11 = assign26950_e25725_d_n11;
        locals.var_t3_dn14 = assign26950_e25725_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26960_e25741, assign26960_e25741_d_n0, assign26960_e25741_d_n2, assign26960_e25741_d_n4, assign26960_e25741_d_n5, assign26960_e25741_d_n6, assign26960_e25741_d_n7, assign26960_e25741_d_n8, assign26960_e25741_d_n9, assign26960_e25741_d_n10, assign26960_e25741_d_n11, assign26960_e25741_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26960_e25735: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk439);
        let assign26960_e25737: f64 = (assign26960_e25735 + locals.var_vbi_dep);
        let assign26960_e25738: f64 = (locals.var_c_2esipq_nsub * assign26960_e25737);
        let assign26960_e25739: f64 = (assign26960_e25738).sqrt();
        (assign26960_e25739, (((locals.var_c_2esipq_nsub_dn0 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn2 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn4 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn5 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn6 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn7 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn8 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn9 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn10 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn11 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign26960_e25739)), (((locals.var_c_2esipq_nsub_dn14 * assign26960_e25737) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign26960_e25739)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign26960_e25741;
        locals.var_w_sub0_dn0 = assign26960_e25741_d_n0;
        locals.var_w_sub0_dn2 = assign26960_e25741_d_n2;
        locals.var_w_sub0_dn4 = assign26960_e25741_d_n4;
        locals.var_w_sub0_dn5 = assign26960_e25741_d_n5;
        locals.var_w_sub0_dn6 = assign26960_e25741_d_n6;
        locals.var_w_sub0_dn7 = assign26960_e25741_d_n7;
        locals.var_w_sub0_dn8 = assign26960_e25741_d_n8;
        locals.var_w_sub0_dn9 = assign26960_e25741_d_n9;
        locals.var_w_sub0_dn10 = assign26960_e25741_d_n10;
        locals.var_w_sub0_dn11 = assign26960_e25741_d_n11;
        locals.var_w_sub0_dn14 = assign26960_e25741_d_n14;
        locals.var_w_sub0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        locals: &mut StampLocals,
    ) {
        let (assign26970_e25752, assign26970_e25752_d_n0, assign26970_e25752_d_n2, assign26970_e25752_d_n4, assign26970_e25752_d_n5, assign26970_e25752_d_n6, assign26970_e25752_d_n7, assign26970_e25752_d_n8, assign26970_e25752_d_n9, assign26970_e25752_d_n10, assign26970_e25752_d_n11, assign26970_e25752_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26970_e25750: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign26970_e25750, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn11 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn11)), ((locals.var_w_b0_dn14 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
        locals.var_q_b0_dep = assign26970_e25752;
        locals.var_q_b0_dep_dn0 = assign26970_e25752_d_n0;
        locals.var_q_b0_dep_dn2 = assign26970_e25752_d_n2;
        locals.var_q_b0_dep_dn4 = assign26970_e25752_d_n4;
        locals.var_q_b0_dep_dn5 = assign26970_e25752_d_n5;
        locals.var_q_b0_dep_dn6 = assign26970_e25752_d_n6;
        locals.var_q_b0_dep_dn7 = assign26970_e25752_d_n7;
        locals.var_q_b0_dep_dn8 = assign26970_e25752_d_n8;
        locals.var_q_b0_dep_dn9 = assign26970_e25752_d_n9;
        locals.var_q_b0_dep_dn10 = assign26970_e25752_d_n10;
        locals.var_q_b0_dep_dn11 = assign26970_e25752_d_n11;
        locals.var_q_b0_dep_dn14 = assign26970_e25752_d_n14;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign26980_e25764, assign26980_e25764_d_n0, assign26980_e25764_d_n2, assign26980_e25764_d_n4, assign26980_e25764_d_n5, assign26980_e25764_d_n6, assign26980_e25764_d_n7, assign26980_e25764_d_n8, assign26980_e25764_d_n9, assign26980_e25764_d_n10, assign26980_e25764_d_n11, assign26980_e25764_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let assign26980_e25760: f64 = (-locals.var_w_sub0);
        let assign26980_e25762: f64 = (assign26980_e25760 * locals.var_q_nsub__blk548);
        (assign26980_e25762, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn10)), (((-locals.var_w_sub0_dn11) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn11)), (((-locals.var_w_sub0_dn14) * locals.var_q_nsub__blk548) + (assign26980_e25760 * locals.var_q_nsub__blk548_dn14)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
        locals.var_q_sub0_dep = assign26980_e25764;
        locals.var_q_sub0_dep_dn0 = assign26980_e25764_d_n0;
        locals.var_q_sub0_dep_dn2 = assign26980_e25764_d_n2;
        locals.var_q_sub0_dep_dn4 = assign26980_e25764_d_n4;
        locals.var_q_sub0_dep_dn5 = assign26980_e25764_d_n5;
        locals.var_q_sub0_dep_dn6 = assign26980_e25764_d_n6;
        locals.var_q_sub0_dep_dn7 = assign26980_e25764_d_n7;
        locals.var_q_sub0_dep_dn8 = assign26980_e25764_d_n8;
        locals.var_q_sub0_dep_dn9 = assign26980_e25764_d_n9;
        locals.var_q_sub0_dep_dn10 = assign26980_e25764_d_n10;
        locals.var_q_sub0_dep_dn11 = assign26980_e25764_d_n11;
        locals.var_q_sub0_dep_dn14 = assign26980_e25764_d_n14;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign26990_e25772, assign26990_e25772_d_n0, assign26990_e25772_d_n2, assign26990_e25772_d_n4, assign26990_e25772_d_n5, assign26990_e25772_d_n6, assign26990_e25772_d_n7, assign26990_e25772_d_n8, assign26990_e25772_d_n9, assign26990_e25772_d_n10, assign26990_e25772_d_n11, assign26990_e25772_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign26990_e25770: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        (assign26990_e25770, (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26990_e25772;
        locals.var_t1_dn0 = assign26990_e25772_d_n0;
        locals.var_t1_dn2 = assign26990_e25772_d_n2;
        locals.var_t1_dn4 = assign26990_e25772_d_n4;
        locals.var_t1_dn5 = assign26990_e25772_d_n5;
        locals.var_t1_dn6 = assign26990_e25772_d_n6;
        locals.var_t1_dn7 = assign26990_e25772_d_n7;
        locals.var_t1_dn8 = assign26990_e25772_d_n8;
        locals.var_t1_dn9 = assign26990_e25772_d_n9;
        locals.var_t1_dn10 = assign26990_e25772_d_n10;
        locals.var_t1_dn11 = assign26990_e25772_d_n11;
        locals.var_t1_dn14 = assign26990_e25772_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27000_e25776: f64 = 0.1;
        let assign27000_e25781: f64 = if ((locals.var_t1 < assign27000_e25776) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard647 = assign27000_e25781;
        locals.var_guard647_rv = 0.0;

        let (assign27010_e25793, assign27010_e25793_d_n0, assign27010_e25793_d_n2, assign27010_e25793_d_n4, assign27010_e25793_d_n5, assign27010_e25793_d_n6, assign27010_e25793_d_n7, assign27010_e25793_d_n8, assign27010_e25793_d_n9, assign27010_e25793_d_n10, assign27010_e25793_d_n11, assign27010_e25793_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27010_e25789: f64 = 0.1;
        let assign27010_e25791: f64 = (assign27010_e25789 - locals.var_t1);
        (assign27010_e25791, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign27010_e25793;
        locals.var_tmf1_dn0 = assign27010_e25793_d_n0;
        locals.var_tmf1_dn2 = assign27010_e25793_d_n2;
        locals.var_tmf1_dn4 = assign27010_e25793_d_n4;
        locals.var_tmf1_dn5 = assign27010_e25793_d_n5;
        locals.var_tmf1_dn6 = assign27010_e25793_d_n6;
        locals.var_tmf1_dn7 = assign27010_e25793_d_n7;
        locals.var_tmf1_dn8 = assign27010_e25793_d_n8;
        locals.var_tmf1_dn9 = assign27010_e25793_d_n9;
        locals.var_tmf1_dn10 = assign27010_e25793_d_n10;
        locals.var_tmf1_dn11 = assign27010_e25793_d_n11;
        locals.var_tmf1_dn14 = assign27010_e25793_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign27020_e25803, assign27020_e25803_d_n0, assign27020_e25803_d_n2, assign27020_e25803_d_n4, assign27020_e25803_d_n5, assign27020_e25803_d_n6, assign27020_e25803_d_n7, assign27020_e25803_d_n8, assign27020_e25803_d_n9, assign27020_e25803_d_n10, assign27020_e25803_d_n11, assign27020_e25803_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27020_e25801: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27020_e25801, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign27020_e25803;
        locals.var_x2_dn0 = assign27020_e25803_d_n0;
        locals.var_x2_dn2 = assign27020_e25803_d_n2;
        locals.var_x2_dn4 = assign27020_e25803_d_n4;
        locals.var_x2_dn5 = assign27020_e25803_d_n5;
        locals.var_x2_dn6 = assign27020_e25803_d_n6;
        locals.var_x2_dn7 = assign27020_e25803_d_n7;
        locals.var_x2_dn8 = assign27020_e25803_d_n8;
        locals.var_x2_dn9 = assign27020_e25803_d_n9;
        locals.var_x2_dn10 = assign27020_e25803_d_n10;
        locals.var_x2_dn11 = assign27020_e25803_d_n11;
        locals.var_x2_dn14 = assign27020_e25803_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign27030_e25813, assign27030_e25813_d_n0, assign27030_e25813_d_n2, assign27030_e25813_d_n4, assign27030_e25813_d_n5, assign27030_e25813_d_n6, assign27030_e25813_d_n7, assign27030_e25813_d_n8, assign27030_e25813_d_n9, assign27030_e25813_d_n10, assign27030_e25813_d_n11, assign27030_e25813_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27030_e25811: f64 = (0.1 * 0.1);
        (assign27030_e25811, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign27030_e25813;
        locals.var_xmax2_dn0 = assign27030_e25813_d_n0;
        locals.var_xmax2_dn2 = assign27030_e25813_d_n2;
        locals.var_xmax2_dn4 = assign27030_e25813_d_n4;
        locals.var_xmax2_dn5 = assign27030_e25813_d_n5;
        locals.var_xmax2_dn6 = assign27030_e25813_d_n6;
        locals.var_xmax2_dn7 = assign27030_e25813_d_n7;
        locals.var_xmax2_dn8 = assign27030_e25813_d_n8;
        locals.var_xmax2_dn9 = assign27030_e25813_d_n9;
        locals.var_xmax2_dn10 = assign27030_e25813_d_n10;
        locals.var_xmax2_dn11 = assign27030_e25813_d_n11;
        locals.var_xmax2_dn14 = assign27030_e25813_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign27040_e25821, assign27040_e25821_d_n0, assign27040_e25821_d_n2, assign27040_e25821_d_n4, assign27040_e25821_d_n5, assign27040_e25821_d_n6, assign27040_e25821_d_n7, assign27040_e25821_d_n8, assign27040_e25821_d_n9, assign27040_e25821_d_n10, assign27040_e25821_d_n11, assign27040_e25821_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27040_e25821;
        locals.var_xp_dn0 = assign27040_e25821_d_n0;
        locals.var_xp_dn2 = assign27040_e25821_d_n2;
        locals.var_xp_dn4 = assign27040_e25821_d_n4;
        locals.var_xp_dn5 = assign27040_e25821_d_n5;
        locals.var_xp_dn6 = assign27040_e25821_d_n6;
        locals.var_xp_dn7 = assign27040_e25821_d_n7;
        locals.var_xp_dn8 = assign27040_e25821_d_n8;
        locals.var_xp_dn9 = assign27040_e25821_d_n9;
        locals.var_xp_dn10 = assign27040_e25821_d_n10;
        locals.var_xp_dn11 = assign27040_e25821_d_n11;
        locals.var_xp_dn14 = assign27040_e25821_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27050_e25829, assign27050_e25829_d_n0, assign27050_e25829_d_n2, assign27050_e25829_d_n4, assign27050_e25829_d_n5, assign27050_e25829_d_n6, assign27050_e25829_d_n7, assign27050_e25829_d_n8, assign27050_e25829_d_n9, assign27050_e25829_d_n10, assign27050_e25829_d_n11, assign27050_e25829_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27050_e25829;
        locals.var_xmp_dn0 = assign27050_e25829_d_n0;
        locals.var_xmp_dn2 = assign27050_e25829_d_n2;
        locals.var_xmp_dn4 = assign27050_e25829_d_n4;
        locals.var_xmp_dn5 = assign27050_e25829_d_n5;
        locals.var_xmp_dn6 = assign27050_e25829_d_n6;
        locals.var_xmp_dn7 = assign27050_e25829_d_n7;
        locals.var_xmp_dn8 = assign27050_e25829_d_n8;
        locals.var_xmp_dn9 = assign27050_e25829_d_n9;
        locals.var_xmp_dn10 = assign27050_e25829_d_n10;
        locals.var_xmp_dn11 = assign27050_e25829_d_n11;
        locals.var_xmp_dn14 = assign27050_e25829_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27060_e25837,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27060_e25837;
        locals.var_m0_rv = 0.0;

        let (assign27070_e25845,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27070_e25845;
        locals.var_mm_rv = 0.0;

        let (assign27080_e25853, assign27080_e25853_d_n0, assign27080_e25853_d_n2, assign27080_e25853_d_n4, assign27080_e25853_d_n5, assign27080_e25853_d_n6, assign27080_e25853_d_n7, assign27080_e25853_d_n8, assign27080_e25853_d_n9, assign27080_e25853_d_n10, assign27080_e25853_d_n11, assign27080_e25853_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27080_e25853;
        locals.var_arg_dn0 = assign27080_e25853_d_n0;
        locals.var_arg_dn2 = assign27080_e25853_d_n2;
        locals.var_arg_dn4 = assign27080_e25853_d_n4;
        locals.var_arg_dn5 = assign27080_e25853_d_n5;
        locals.var_arg_dn6 = assign27080_e25853_d_n6;
        locals.var_arg_dn7 = assign27080_e25853_d_n7;
        locals.var_arg_dn8 = assign27080_e25853_d_n8;
        locals.var_arg_dn9 = assign27080_e25853_d_n9;
        locals.var_arg_dn10 = assign27080_e25853_d_n10;
        locals.var_arg_dn11 = assign27080_e25853_d_n11;
        locals.var_arg_dn14 = assign27080_e25853_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27090_e25861, assign27090_e25861_d_n0, assign27090_e25861_d_n2, assign27090_e25861_d_n4, assign27090_e25861_d_n5, assign27090_e25861_d_n6, assign27090_e25861_d_n7, assign27090_e25861_d_n8, assign27090_e25861_d_n9, assign27090_e25861_d_n10, assign27090_e25861_d_n11, assign27090_e25861_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27090_e25861;
        locals.var_dnm_dn0 = assign27090_e25861_d_n0;
        locals.var_dnm_dn2 = assign27090_e25861_d_n2;
        locals.var_dnm_dn4 = assign27090_e25861_d_n4;
        locals.var_dnm_dn5 = assign27090_e25861_d_n5;
        locals.var_dnm_dn6 = assign27090_e25861_d_n6;
        locals.var_dnm_dn7 = assign27090_e25861_d_n7;
        locals.var_dnm_dn8 = assign27090_e25861_d_n8;
        locals.var_dnm_dn9 = assign27090_e25861_d_n9;
        locals.var_dnm_dn10 = assign27090_e25861_d_n10;
        locals.var_dnm_dn11 = assign27090_e25861_d_n11;
        locals.var_dnm_dn14 = assign27090_e25861_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27100_e25871, assign27100_e25871_d_n0, assign27100_e25871_d_n2, assign27100_e25871_d_n4, assign27100_e25871_d_n5, assign27100_e25871_d_n6, assign27100_e25871_d_n7, assign27100_e25871_d_n8, assign27100_e25871_d_n9, assign27100_e25871_d_n10, assign27100_e25871_d_n11, assign27100_e25871_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27100_e25869: f64 = (locals.var_xp * locals.var_x2);
        (assign27100_e25869, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27100_e25871;
        locals.var_xp_dn0 = assign27100_e25871_d_n0;
        locals.var_xp_dn2 = assign27100_e25871_d_n2;
        locals.var_xp_dn4 = assign27100_e25871_d_n4;
        locals.var_xp_dn5 = assign27100_e25871_d_n5;
        locals.var_xp_dn6 = assign27100_e25871_d_n6;
        locals.var_xp_dn7 = assign27100_e25871_d_n7;
        locals.var_xp_dn8 = assign27100_e25871_d_n8;
        locals.var_xp_dn9 = assign27100_e25871_d_n9;
        locals.var_xp_dn10 = assign27100_e25871_d_n10;
        locals.var_xp_dn11 = assign27100_e25871_d_n11;
        locals.var_xp_dn14 = assign27100_e25871_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27110_e25881, assign27110_e25881_d_n0, assign27110_e25881_d_n2, assign27110_e25881_d_n4, assign27110_e25881_d_n5, assign27110_e25881_d_n6, assign27110_e25881_d_n7, assign27110_e25881_d_n8, assign27110_e25881_d_n9, assign27110_e25881_d_n10, assign27110_e25881_d_n11, assign27110_e25881_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27110_e25879: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27110_e25879, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27110_e25881;
        locals.var_xmp_dn0 = assign27110_e25881_d_n0;
        locals.var_xmp_dn2 = assign27110_e25881_d_n2;
        locals.var_xmp_dn4 = assign27110_e25881_d_n4;
        locals.var_xmp_dn5 = assign27110_e25881_d_n5;
        locals.var_xmp_dn6 = assign27110_e25881_d_n6;
        locals.var_xmp_dn7 = assign27110_e25881_d_n7;
        locals.var_xmp_dn8 = assign27110_e25881_d_n8;
        locals.var_xmp_dn9 = assign27110_e25881_d_n9;
        locals.var_xmp_dn10 = assign27110_e25881_d_n10;
        locals.var_xmp_dn11 = assign27110_e25881_d_n11;
        locals.var_xmp_dn14 = assign27110_e25881_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27120_e25891, assign27120_e25891_d_n0, assign27120_e25891_d_n2, assign27120_e25891_d_n4, assign27120_e25891_d_n5, assign27120_e25891_d_n6, assign27120_e25891_d_n7, assign27120_e25891_d_n8, assign27120_e25891_d_n9, assign27120_e25891_d_n10, assign27120_e25891_d_n11, assign27120_e25891_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27120_e25889: f64 = (locals.var_xp * locals.var_x2);
        (assign27120_e25889, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27120_e25891;
        locals.var_xp_dn0 = assign27120_e25891_d_n0;
        locals.var_xp_dn2 = assign27120_e25891_d_n2;
        locals.var_xp_dn4 = assign27120_e25891_d_n4;
        locals.var_xp_dn5 = assign27120_e25891_d_n5;
        locals.var_xp_dn6 = assign27120_e25891_d_n6;
        locals.var_xp_dn7 = assign27120_e25891_d_n7;
        locals.var_xp_dn8 = assign27120_e25891_d_n8;
        locals.var_xp_dn9 = assign27120_e25891_d_n9;
        locals.var_xp_dn10 = assign27120_e25891_d_n10;
        locals.var_xp_dn11 = assign27120_e25891_d_n11;
        locals.var_xp_dn14 = assign27120_e25891_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27130_e25901, assign27130_e25901_d_n0, assign27130_e25901_d_n2, assign27130_e25901_d_n4, assign27130_e25901_d_n5, assign27130_e25901_d_n6, assign27130_e25901_d_n7, assign27130_e25901_d_n8, assign27130_e25901_d_n9, assign27130_e25901_d_n10, assign27130_e25901_d_n11, assign27130_e25901_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27130_e25899: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27130_e25899, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27130_e25901;
        locals.var_xmp_dn0 = assign27130_e25901_d_n0;
        locals.var_xmp_dn2 = assign27130_e25901_d_n2;
        locals.var_xmp_dn4 = assign27130_e25901_d_n4;
        locals.var_xmp_dn5 = assign27130_e25901_d_n5;
        locals.var_xmp_dn6 = assign27130_e25901_d_n6;
        locals.var_xmp_dn7 = assign27130_e25901_d_n7;
        locals.var_xmp_dn8 = assign27130_e25901_d_n8;
        locals.var_xmp_dn9 = assign27130_e25901_d_n9;
        locals.var_xmp_dn10 = assign27130_e25901_d_n10;
        locals.var_xmp_dn11 = assign27130_e25901_d_n11;
        locals.var_xmp_dn14 = assign27130_e25901_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27140_e25911, assign27140_e25911_d_n0, assign27140_e25911_d_n2, assign27140_e25911_d_n4, assign27140_e25911_d_n5, assign27140_e25911_d_n6, assign27140_e25911_d_n7, assign27140_e25911_d_n8, assign27140_e25911_d_n9, assign27140_e25911_d_n10, assign27140_e25911_d_n11, assign27140_e25911_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27140_e25909: f64 = (locals.var_xp + locals.var_xmp);
        (assign27140_e25909, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27140_e25911;
        locals.var_arg_dn0 = assign27140_e25911_d_n0;
        locals.var_arg_dn2 = assign27140_e25911_d_n2;
        locals.var_arg_dn4 = assign27140_e25911_d_n4;
        locals.var_arg_dn5 = assign27140_e25911_d_n5;
        locals.var_arg_dn6 = assign27140_e25911_d_n6;
        locals.var_arg_dn7 = assign27140_e25911_d_n7;
        locals.var_arg_dn8 = assign27140_e25911_d_n8;
        locals.var_arg_dn9 = assign27140_e25911_d_n9;
        locals.var_arg_dn10 = assign27140_e25911_d_n10;
        locals.var_arg_dn11 = assign27140_e25911_d_n11;
        locals.var_arg_dn14 = assign27140_e25911_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27150_e25919, assign27150_e25919_d_n0, assign27150_e25919_d_n2, assign27150_e25919_d_n4, assign27150_e25919_d_n5, assign27150_e25919_d_n6, assign27150_e25919_d_n7, assign27150_e25919_d_n8, assign27150_e25919_d_n9, assign27150_e25919_d_n10, assign27150_e25919_d_n11, assign27150_e25919_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27150_e25919;
        locals.var_dnm_dn0 = assign27150_e25919_d_n0;
        locals.var_dnm_dn2 = assign27150_e25919_d_n2;
        locals.var_dnm_dn4 = assign27150_e25919_d_n4;
        locals.var_dnm_dn5 = assign27150_e25919_d_n5;
        locals.var_dnm_dn6 = assign27150_e25919_d_n6;
        locals.var_dnm_dn7 = assign27150_e25919_d_n7;
        locals.var_dnm_dn8 = assign27150_e25919_d_n8;
        locals.var_dnm_dn9 = assign27150_e25919_d_n9;
        locals.var_dnm_dn10 = assign27150_e25919_d_n10;
        locals.var_dnm_dn11 = assign27150_e25919_d_n11;
        locals.var_dnm_dn14 = assign27150_e25919_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign27160_e25934: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard648 = assign27160_e25934;
        locals.var_guard648_rv = 0.0;

        let assign27170_e25937: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard649 = assign27170_e25937;
        locals.var_guard649_rv = 0.0;

        let (assign27180_e25949,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27180_e25949;
        locals.var_mm_rv = 0.0;

        let assign27190_e25952: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign27190_e25952;
        locals.var_guard650_rv = 0.0;

        let (assign27200_e25967,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27200_e25967;
        locals.var_mm_rv = 0.0;

        let assign27210_e25970: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign27210_e25970;
        locals.var_guard651_rv = 0.0;

        let (assign27220_e25988,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard651 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27220_e25988;
        locals.var_mm_rv = 0.0;

        let assign27230_e25991: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign27230_e25991;
        locals.var_guard652_rv = 0.0;

        let (assign27240_e26012,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27240_e26012;
        locals.var_mm_rv = 0.0;

        let (assign27250_e26022,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27250_e26022;
        locals.var_m0_rv = 0.0;

        let mut assign27260_loop_guard: usize = 0;
        while {
            let assign27260_cond_e26033: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27260_cond_e26033 != 0.0
        } {
            assign27260_loop_guard += 1;
            assert!(assign27260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27260_body0_e26044, assign27260_body0_e26044_d_n0, assign27260_body0_e26044_d_n2, assign27260_body0_e26044_d_n4, assign27260_body0_e26044_d_n5, assign27260_body0_e26044_d_n6, assign27260_body0_e26044_d_n7, assign27260_body0_e26044_d_n8, assign27260_body0_e26044_d_n9, assign27260_body0_e26044_d_n10, assign27260_body0_e26044_d_n11, assign27260_body0_e26044_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign27260_body0_e26042: f64 = (locals.var_dnm).sqrt();
        (assign27260_body0_e26042, (locals.var_dnm_dn0 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn2 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn4 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn5 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn6 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn7 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn8 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn9 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn10 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn11 / (2.0 * assign27260_body0_e26042)), (locals.var_dnm_dn14 / (2.0 * assign27260_body0_e26042)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign27260_body0_e26044;
            locals.var_dnm_dn0 = assign27260_body0_e26044_d_n0;
            locals.var_dnm_dn2 = assign27260_body0_e26044_d_n2;
            locals.var_dnm_dn4 = assign27260_body0_e26044_d_n4;
            locals.var_dnm_dn5 = assign27260_body0_e26044_d_n5;
            locals.var_dnm_dn6 = assign27260_body0_e26044_d_n6;
            locals.var_dnm_dn7 = assign27260_body0_e26044_d_n7;
            locals.var_dnm_dn8 = assign27260_body0_e26044_d_n8;
            locals.var_dnm_dn9 = assign27260_body0_e26044_d_n9;
            locals.var_dnm_dn10 = assign27260_body0_e26044_d_n10;
            locals.var_dnm_dn11 = assign27260_body0_e26044_d_n11;
            locals.var_dnm_dn14 = assign27260_body0_e26044_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign27260_body1_e26056,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign27260_body1_e26054: f64 = (locals.var_m0 + 1.0);
        (assign27260_body1_e26054,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27260_body1_e26056;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_82(
        locals: &mut StampLocals,
    ) {
        let (assign27270_e26078, assign27270_e26078_d_n0, assign27270_e26078_d_n2, assign27270_e26078_d_n4, assign27270_e26078_d_n5, assign27270_e26078_d_n6, assign27270_e26078_d_n7, assign27270_e26078_d_n8, assign27270_e26078_d_n9, assign27270_e26078_d_n10, assign27270_e26078_d_n11, assign27270_e26078_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 == 0.0)) {
        let (assign27270_e26076, assign27270_e26076_d_n0, assign27270_e26076_d_n2, assign27270_e26076_d_n4, assign27270_e26076_d_n5, assign27270_e26076_d_n6, assign27270_e26076_d_n7, assign27270_e26076_d_n8, assign27270_e26076_d_n9, assign27270_e26076_d_n10, assign27270_e26076_d_n11, assign27270_e26076_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27270_e26073: f64 = (2.0 * 2.0);
                let assign27270_e26074: f64 = (1.0 / assign27270_e26073);
                let assign27270_e26075: f64 = (locals.var_dnm).powf(assign27270_e26074);
                (assign27270_e26075, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn11)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27270_e26074) as f64).is_finite() && ((assign27270_e26074) as f64).fract() == 0.0 { if assign27270_e26074 == 0.0 { 0.0 } else { (assign27270_e26074 * ((locals.var_dnm).powf(assign27270_e26074 - 1.0) * locals.var_dnm_dn14)) } } else { (assign27270_e26075 * (assign27270_e26074 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign27270_e26076, assign27270_e26076_d_n0, assign27270_e26076_d_n2, assign27270_e26076_d_n4, assign27270_e26076_d_n5, assign27270_e26076_d_n6, assign27270_e26076_d_n7, assign27270_e26076_d_n8, assign27270_e26076_d_n9, assign27270_e26076_d_n10, assign27270_e26076_d_n11, assign27270_e26076_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27270_e26078;
        locals.var_dnm_dn0 = assign27270_e26078_d_n0;
        locals.var_dnm_dn2 = assign27270_e26078_d_n2;
        locals.var_dnm_dn4 = assign27270_e26078_d_n4;
        locals.var_dnm_dn5 = assign27270_e26078_d_n5;
        locals.var_dnm_dn6 = assign27270_e26078_d_n6;
        locals.var_dnm_dn7 = assign27270_e26078_d_n7;
        locals.var_dnm_dn8 = assign27270_e26078_d_n8;
        locals.var_dnm_dn9 = assign27270_e26078_d_n9;
        locals.var_dnm_dn10 = assign27270_e26078_d_n10;
        locals.var_dnm_dn11 = assign27270_e26078_d_n11;
        locals.var_dnm_dn14 = assign27270_e26078_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27280_e26088, assign27280_e26088_d_n0, assign27280_e26088_d_n2, assign27280_e26088_d_n4, assign27280_e26088_d_n5, assign27280_e26088_d_n6, assign27280_e26088_d_n7, assign27280_e26088_d_n8, assign27280_e26088_d_n9, assign27280_e26088_d_n10, assign27280_e26088_d_n11, assign27280_e26088_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27280_e26086: f64 = (1.0 / locals.var_dnm);
        (assign27280_e26086, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27280_e26088;
        locals.var_dnm_dn0 = assign27280_e26088_d_n0;
        locals.var_dnm_dn2 = assign27280_e26088_d_n2;
        locals.var_dnm_dn4 = assign27280_e26088_d_n4;
        locals.var_dnm_dn5 = assign27280_e26088_d_n5;
        locals.var_dnm_dn6 = assign27280_e26088_d_n6;
        locals.var_dnm_dn7 = assign27280_e26088_d_n7;
        locals.var_dnm_dn8 = assign27280_e26088_d_n8;
        locals.var_dnm_dn9 = assign27280_e26088_d_n9;
        locals.var_dnm_dn10 = assign27280_e26088_d_n10;
        locals.var_dnm_dn11 = assign27280_e26088_d_n11;
        locals.var_dnm_dn14 = assign27280_e26088_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27290_e26100, assign27290_e26100_d_n0, assign27290_e26100_d_n2, assign27290_e26100_d_n4, assign27290_e26100_d_n5, assign27290_e26100_d_n6, assign27290_e26100_d_n7, assign27290_e26100_d_n8, assign27290_e26100_d_n9, assign27290_e26100_d_n10, assign27290_e26100_d_n11, assign27290_e26100_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27290_e26096: f64 = (locals.var_tmf1 * 0.1);
        let assign27290_e26098: f64 = (assign27290_e26096 * locals.var_dnm);
        (assign27290_e26098, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign27290_e26096 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign27290_e26100;
        locals.var_tmf0_dn0 = assign27290_e26100_d_n0;
        locals.var_tmf0_dn2 = assign27290_e26100_d_n2;
        locals.var_tmf0_dn4 = assign27290_e26100_d_n4;
        locals.var_tmf0_dn5 = assign27290_e26100_d_n5;
        locals.var_tmf0_dn6 = assign27290_e26100_d_n6;
        locals.var_tmf0_dn7 = assign27290_e26100_d_n7;
        locals.var_tmf0_dn8 = assign27290_e26100_d_n8;
        locals.var_tmf0_dn9 = assign27290_e26100_d_n9;
        locals.var_tmf0_dn10 = assign27290_e26100_d_n10;
        locals.var_tmf0_dn11 = assign27290_e26100_d_n11;
        locals.var_tmf0_dn14 = assign27290_e26100_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign27300_e26114, assign27300_e26114_d_n0, assign27300_e26114_d_n2, assign27300_e26114_d_n4, assign27300_e26114_d_n5, assign27300_e26114_d_n6, assign27300_e26114_d_n7, assign27300_e26114_d_n8, assign27300_e26114_d_n9, assign27300_e26114_d_n10, assign27300_e26114_d_n11, assign27300_e26114_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27300_e26108: f64 = (0.1 * locals.var_xmp);
        let assign27300_e26110: f64 = (assign27300_e26108 * locals.var_dnm);
        let assign27300_e26112: f64 = (assign27300_e26110 / locals.var_arg);
        (assign27300_e26112, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn11)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign27300_e26108 * locals.var_dnm_dn14)) * locals.var_arg) - (assign27300_e26110 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27300_e26114;
        locals.var_t0_dn0 = assign27300_e26114_d_n0;
        locals.var_t0_dn2 = assign27300_e26114_d_n2;
        locals.var_t0_dn4 = assign27300_e26114_d_n4;
        locals.var_t0_dn5 = assign27300_e26114_d_n5;
        locals.var_t0_dn6 = assign27300_e26114_d_n6;
        locals.var_t0_dn7 = assign27300_e26114_d_n7;
        locals.var_t0_dn8 = assign27300_e26114_d_n8;
        locals.var_t0_dn9 = assign27300_e26114_d_n9;
        locals.var_t0_dn10 = assign27300_e26114_d_n10;
        locals.var_t0_dn11 = assign27300_e26114_d_n11;
        locals.var_t0_dn14 = assign27300_e26114_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27310_e26126, assign27310_e26126_d_n0, assign27310_e26126_d_n2, assign27310_e26126_d_n4, assign27310_e26126_d_n5, assign27310_e26126_d_n6, assign27310_e26126_d_n7, assign27310_e26126_d_n8, assign27310_e26126_d_n9, assign27310_e26126_d_n10, assign27310_e26126_d_n11, assign27310_e26126_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign27310_e26122: f64 = 0.1;
        let assign27310_e26124: f64 = (assign27310_e26122 - locals.var_tmf0);
        (assign27310_e26124, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27310_e26126;
        locals.var_t2_dn0 = assign27310_e26126_d_n0;
        locals.var_t2_dn2 = assign27310_e26126_d_n2;
        locals.var_t2_dn4 = assign27310_e26126_d_n4;
        locals.var_t2_dn5 = assign27310_e26126_d_n5;
        locals.var_t2_dn6 = assign27310_e26126_d_n6;
        locals.var_t2_dn7 = assign27310_e26126_d_n7;
        locals.var_t2_dn8 = assign27310_e26126_d_n8;
        locals.var_t2_dn9 = assign27310_e26126_d_n9;
        locals.var_t2_dn10 = assign27310_e26126_d_n10;
        locals.var_t2_dn11 = assign27310_e26126_d_n11;
        locals.var_t2_dn14 = assign27310_e26126_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27320_e26134, assign27320_e26134_d_n0, assign27320_e26134_d_n2, assign27320_e26134_d_n4, assign27320_e26134_d_n5, assign27320_e26134_d_n6, assign27320_e26134_d_n7, assign27320_e26134_d_n8, assign27320_e26134_d_n9, assign27320_e26134_d_n10, assign27320_e26134_d_n11, assign27320_e26134_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27320_e26134;
        locals.var_t0_dn0 = assign27320_e26134_d_n0;
        locals.var_t0_dn2 = assign27320_e26134_d_n2;
        locals.var_t0_dn4 = assign27320_e26134_d_n4;
        locals.var_t0_dn5 = assign27320_e26134_d_n5;
        locals.var_t0_dn6 = assign27320_e26134_d_n6;
        locals.var_t0_dn7 = assign27320_e26134_d_n7;
        locals.var_t0_dn8 = assign27320_e26134_d_n8;
        locals.var_t0_dn9 = assign27320_e26134_d_n9;
        locals.var_t0_dn10 = assign27320_e26134_d_n10;
        locals.var_t0_dn11 = assign27320_e26134_d_n11;
        locals.var_t0_dn14 = assign27320_e26134_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27330_e26143, assign27330_e26143_d_n0, assign27330_e26143_d_n2, assign27330_e26143_d_n4, assign27330_e26143_d_n5, assign27330_e26143_d_n6, assign27330_e26143_d_n7, assign27330_e26143_d_n8, assign27330_e26143_d_n9, assign27330_e26143_d_n10, assign27330_e26143_d_n11, assign27330_e26143_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27330_e26143;
        locals.var_t2_dn0 = assign27330_e26143_d_n0;
        locals.var_t2_dn2 = assign27330_e26143_d_n2;
        locals.var_t2_dn4 = assign27330_e26143_d_n4;
        locals.var_t2_dn5 = assign27330_e26143_d_n5;
        locals.var_t2_dn6 = assign27330_e26143_d_n6;
        locals.var_t2_dn7 = assign27330_e26143_d_n7;
        locals.var_t2_dn8 = assign27330_e26143_d_n8;
        locals.var_t2_dn9 = assign27330_e26143_d_n9;
        locals.var_t2_dn10 = assign27330_e26143_d_n10;
        locals.var_t2_dn11 = assign27330_e26143_d_n11;
        locals.var_t2_dn14 = assign27330_e26143_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27340_e26152, assign27340_e26152_d_n0, assign27340_e26152_d_n2, assign27340_e26152_d_n4, assign27340_e26152_d_n5, assign27340_e26152_d_n6, assign27340_e26152_d_n7, assign27340_e26152_d_n8, assign27340_e26152_d_n9, assign27340_e26152_d_n10, assign27340_e26152_d_n11, assign27340_e26152_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard647 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27340_e26152;
        locals.var_t0_dn0 = assign27340_e26152_d_n0;
        locals.var_t0_dn2 = assign27340_e26152_d_n2;
        locals.var_t0_dn4 = assign27340_e26152_d_n4;
        locals.var_t0_dn5 = assign27340_e26152_d_n5;
        locals.var_t0_dn6 = assign27340_e26152_d_n6;
        locals.var_t0_dn7 = assign27340_e26152_d_n7;
        locals.var_t0_dn8 = assign27340_e26152_d_n8;
        locals.var_t0_dn9 = assign27340_e26152_d_n9;
        locals.var_t0_dn10 = assign27340_e26152_d_n10;
        locals.var_t0_dn11 = assign27340_e26152_d_n11;
        locals.var_t0_dn14 = assign27340_e26152_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27350_e26161, assign27350_e26161_d_n0, assign27350_e26161_d_n2, assign27350_e26161_d_n4, assign27350_e26161_d_n5, assign27350_e26161_d_n6, assign27350_e26161_d_n7, assign27350_e26161_d_n8, assign27350_e26161_d_n9, assign27350_e26161_d_n10, assign27350_e26161_d_n11, assign27350_e26161_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign27350_e26158: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign27350_e26159: f64 = (assign27350_e26158).sqrt();
        (assign27350_e26159, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign27350_e26159)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign27350_e26159)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign27350_e26161;
        locals.var_w_b0_dn0 = assign27350_e26161_d_n0;
        locals.var_w_b0_dn2 = assign27350_e26161_d_n2;
        locals.var_w_b0_dn4 = assign27350_e26161_d_n4;
        locals.var_w_b0_dn5 = assign27350_e26161_d_n5;
        locals.var_w_b0_dn6 = assign27350_e26161_d_n6;
        locals.var_w_b0_dn7 = assign27350_e26161_d_n7;
        locals.var_w_b0_dn8 = assign27350_e26161_d_n8;
        locals.var_w_b0_dn9 = assign27350_e26161_d_n9;
        locals.var_w_b0_dn10 = assign27350_e26161_d_n10;
        locals.var_w_b0_dn11 = assign27350_e26161_d_n11;
        locals.var_w_b0_dn14 = assign27350_e26161_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign27360_e26165: f64 = (locals.var_uc_depthn - 1e-8);
        let assign27360_e26170: f64 = if ((locals.var_w_b0 > assign27360_e26165) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard653 = assign27360_e26170;
        locals.var_guard653_rv = 0.0;

        let (assign27370_e26182, assign27370_e26182_d_n0, assign27370_e26182_d_n2, assign27370_e26182_d_n4, assign27370_e26182_d_n5, assign27370_e26182_d_n6, assign27370_e26182_d_n7, assign27370_e26182_d_n8, assign27370_e26182_d_n9, assign27370_e26182_d_n10, assign27370_e26182_d_n11, assign27370_e26182_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27370_e26178: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign27370_e26180: f64 = (assign27370_e26178 + 1e-8);
        (assign27370_e26180, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign27370_e26182;
        locals.var_tmf1_dn0 = assign27370_e26182_d_n0;
        locals.var_tmf1_dn2 = assign27370_e26182_d_n2;
        locals.var_tmf1_dn4 = assign27370_e26182_d_n4;
        locals.var_tmf1_dn5 = assign27370_e26182_d_n5;
        locals.var_tmf1_dn6 = assign27370_e26182_d_n6;
        locals.var_tmf1_dn7 = assign27370_e26182_d_n7;
        locals.var_tmf1_dn8 = assign27370_e26182_d_n8;
        locals.var_tmf1_dn9 = assign27370_e26182_d_n9;
        locals.var_tmf1_dn10 = assign27370_e26182_d_n10;
        locals.var_tmf1_dn11 = assign27370_e26182_d_n11;
        locals.var_tmf1_dn14 = assign27370_e26182_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign27380_e26192, assign27380_e26192_d_n0, assign27380_e26192_d_n2, assign27380_e26192_d_n4, assign27380_e26192_d_n5, assign27380_e26192_d_n6, assign27380_e26192_d_n7, assign27380_e26192_d_n8, assign27380_e26192_d_n9, assign27380_e26192_d_n10, assign27380_e26192_d_n11, assign27380_e26192_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27380_e26190: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27380_e26190, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign27380_e26192;
        locals.var_x2_dn0 = assign27380_e26192_d_n0;
        locals.var_x2_dn2 = assign27380_e26192_d_n2;
        locals.var_x2_dn4 = assign27380_e26192_d_n4;
        locals.var_x2_dn5 = assign27380_e26192_d_n5;
        locals.var_x2_dn6 = assign27380_e26192_d_n6;
        locals.var_x2_dn7 = assign27380_e26192_d_n7;
        locals.var_x2_dn8 = assign27380_e26192_d_n8;
        locals.var_x2_dn9 = assign27380_e26192_d_n9;
        locals.var_x2_dn10 = assign27380_e26192_d_n10;
        locals.var_x2_dn11 = assign27380_e26192_d_n11;
        locals.var_x2_dn14 = assign27380_e26192_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign27390_e26202, assign27390_e26202_d_n0, assign27390_e26202_d_n2, assign27390_e26202_d_n4, assign27390_e26202_d_n5, assign27390_e26202_d_n6, assign27390_e26202_d_n7, assign27390_e26202_d_n8, assign27390_e26202_d_n9, assign27390_e26202_d_n10, assign27390_e26202_d_n11, assign27390_e26202_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27390_e26200: f64 = (1e-8 * 1e-8);
        (assign27390_e26200, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign27390_e26202;
        locals.var_xmax2_dn0 = assign27390_e26202_d_n0;
        locals.var_xmax2_dn2 = assign27390_e26202_d_n2;
        locals.var_xmax2_dn4 = assign27390_e26202_d_n4;
        locals.var_xmax2_dn5 = assign27390_e26202_d_n5;
        locals.var_xmax2_dn6 = assign27390_e26202_d_n6;
        locals.var_xmax2_dn7 = assign27390_e26202_d_n7;
        locals.var_xmax2_dn8 = assign27390_e26202_d_n8;
        locals.var_xmax2_dn9 = assign27390_e26202_d_n9;
        locals.var_xmax2_dn10 = assign27390_e26202_d_n10;
        locals.var_xmax2_dn11 = assign27390_e26202_d_n11;
        locals.var_xmax2_dn14 = assign27390_e26202_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign27400_e26210, assign27400_e26210_d_n0, assign27400_e26210_d_n2, assign27400_e26210_d_n4, assign27400_e26210_d_n5, assign27400_e26210_d_n6, assign27400_e26210_d_n7, assign27400_e26210_d_n8, assign27400_e26210_d_n9, assign27400_e26210_d_n10, assign27400_e26210_d_n11, assign27400_e26210_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27400_e26210;
        locals.var_xp_dn0 = assign27400_e26210_d_n0;
        locals.var_xp_dn2 = assign27400_e26210_d_n2;
        locals.var_xp_dn4 = assign27400_e26210_d_n4;
        locals.var_xp_dn5 = assign27400_e26210_d_n5;
        locals.var_xp_dn6 = assign27400_e26210_d_n6;
        locals.var_xp_dn7 = assign27400_e26210_d_n7;
        locals.var_xp_dn8 = assign27400_e26210_d_n8;
        locals.var_xp_dn9 = assign27400_e26210_d_n9;
        locals.var_xp_dn10 = assign27400_e26210_d_n10;
        locals.var_xp_dn11 = assign27400_e26210_d_n11;
        locals.var_xp_dn14 = assign27400_e26210_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27410_e26218, assign27410_e26218_d_n0, assign27410_e26218_d_n2, assign27410_e26218_d_n4, assign27410_e26218_d_n5, assign27410_e26218_d_n6, assign27410_e26218_d_n7, assign27410_e26218_d_n8, assign27410_e26218_d_n9, assign27410_e26218_d_n10, assign27410_e26218_d_n11, assign27410_e26218_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27410_e26218;
        locals.var_xmp_dn0 = assign27410_e26218_d_n0;
        locals.var_xmp_dn2 = assign27410_e26218_d_n2;
        locals.var_xmp_dn4 = assign27410_e26218_d_n4;
        locals.var_xmp_dn5 = assign27410_e26218_d_n5;
        locals.var_xmp_dn6 = assign27410_e26218_d_n6;
        locals.var_xmp_dn7 = assign27410_e26218_d_n7;
        locals.var_xmp_dn8 = assign27410_e26218_d_n8;
        locals.var_xmp_dn9 = assign27410_e26218_d_n9;
        locals.var_xmp_dn10 = assign27410_e26218_d_n10;
        locals.var_xmp_dn11 = assign27410_e26218_d_n11;
        locals.var_xmp_dn14 = assign27410_e26218_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27420_e26226,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27420_e26226;
        locals.var_m0_rv = 0.0;

        let (assign27430_e26234,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27430_e26234;
        locals.var_mm_rv = 0.0;

        let (assign27440_e26242, assign27440_e26242_d_n0, assign27440_e26242_d_n2, assign27440_e26242_d_n4, assign27440_e26242_d_n5, assign27440_e26242_d_n6, assign27440_e26242_d_n7, assign27440_e26242_d_n8, assign27440_e26242_d_n9, assign27440_e26242_d_n10, assign27440_e26242_d_n11, assign27440_e26242_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27440_e26242;
        locals.var_arg_dn0 = assign27440_e26242_d_n0;
        locals.var_arg_dn2 = assign27440_e26242_d_n2;
        locals.var_arg_dn4 = assign27440_e26242_d_n4;
        locals.var_arg_dn5 = assign27440_e26242_d_n5;
        locals.var_arg_dn6 = assign27440_e26242_d_n6;
        locals.var_arg_dn7 = assign27440_e26242_d_n7;
        locals.var_arg_dn8 = assign27440_e26242_d_n8;
        locals.var_arg_dn9 = assign27440_e26242_d_n9;
        locals.var_arg_dn10 = assign27440_e26242_d_n10;
        locals.var_arg_dn11 = assign27440_e26242_d_n11;
        locals.var_arg_dn14 = assign27440_e26242_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27450_e26250, assign27450_e26250_d_n0, assign27450_e26250_d_n2, assign27450_e26250_d_n4, assign27450_e26250_d_n5, assign27450_e26250_d_n6, assign27450_e26250_d_n7, assign27450_e26250_d_n8, assign27450_e26250_d_n9, assign27450_e26250_d_n10, assign27450_e26250_d_n11, assign27450_e26250_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27450_e26250;
        locals.var_dnm_dn0 = assign27450_e26250_d_n0;
        locals.var_dnm_dn2 = assign27450_e26250_d_n2;
        locals.var_dnm_dn4 = assign27450_e26250_d_n4;
        locals.var_dnm_dn5 = assign27450_e26250_d_n5;
        locals.var_dnm_dn6 = assign27450_e26250_d_n6;
        locals.var_dnm_dn7 = assign27450_e26250_d_n7;
        locals.var_dnm_dn8 = assign27450_e26250_d_n8;
        locals.var_dnm_dn9 = assign27450_e26250_d_n9;
        locals.var_dnm_dn10 = assign27450_e26250_d_n10;
        locals.var_dnm_dn11 = assign27450_e26250_d_n11;
        locals.var_dnm_dn14 = assign27450_e26250_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27460_e26260, assign27460_e26260_d_n0, assign27460_e26260_d_n2, assign27460_e26260_d_n4, assign27460_e26260_d_n5, assign27460_e26260_d_n6, assign27460_e26260_d_n7, assign27460_e26260_d_n8, assign27460_e26260_d_n9, assign27460_e26260_d_n10, assign27460_e26260_d_n11, assign27460_e26260_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27460_e26258: f64 = (locals.var_xp * locals.var_x2);
        (assign27460_e26258, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27460_e26260;
        locals.var_xp_dn0 = assign27460_e26260_d_n0;
        locals.var_xp_dn2 = assign27460_e26260_d_n2;
        locals.var_xp_dn4 = assign27460_e26260_d_n4;
        locals.var_xp_dn5 = assign27460_e26260_d_n5;
        locals.var_xp_dn6 = assign27460_e26260_d_n6;
        locals.var_xp_dn7 = assign27460_e26260_d_n7;
        locals.var_xp_dn8 = assign27460_e26260_d_n8;
        locals.var_xp_dn9 = assign27460_e26260_d_n9;
        locals.var_xp_dn10 = assign27460_e26260_d_n10;
        locals.var_xp_dn11 = assign27460_e26260_d_n11;
        locals.var_xp_dn14 = assign27460_e26260_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27470_e26270, assign27470_e26270_d_n0, assign27470_e26270_d_n2, assign27470_e26270_d_n4, assign27470_e26270_d_n5, assign27470_e26270_d_n6, assign27470_e26270_d_n7, assign27470_e26270_d_n8, assign27470_e26270_d_n9, assign27470_e26270_d_n10, assign27470_e26270_d_n11, assign27470_e26270_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27470_e26268: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27470_e26268, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27470_e26270;
        locals.var_xmp_dn0 = assign27470_e26270_d_n0;
        locals.var_xmp_dn2 = assign27470_e26270_d_n2;
        locals.var_xmp_dn4 = assign27470_e26270_d_n4;
        locals.var_xmp_dn5 = assign27470_e26270_d_n5;
        locals.var_xmp_dn6 = assign27470_e26270_d_n6;
        locals.var_xmp_dn7 = assign27470_e26270_d_n7;
        locals.var_xmp_dn8 = assign27470_e26270_d_n8;
        locals.var_xmp_dn9 = assign27470_e26270_d_n9;
        locals.var_xmp_dn10 = assign27470_e26270_d_n10;
        locals.var_xmp_dn11 = assign27470_e26270_d_n11;
        locals.var_xmp_dn14 = assign27470_e26270_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27480_e26280, assign27480_e26280_d_n0, assign27480_e26280_d_n2, assign27480_e26280_d_n4, assign27480_e26280_d_n5, assign27480_e26280_d_n6, assign27480_e26280_d_n7, assign27480_e26280_d_n8, assign27480_e26280_d_n9, assign27480_e26280_d_n10, assign27480_e26280_d_n11, assign27480_e26280_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27480_e26278: f64 = (locals.var_xp * locals.var_x2);
        (assign27480_e26278, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27480_e26280;
        locals.var_xp_dn0 = assign27480_e26280_d_n0;
        locals.var_xp_dn2 = assign27480_e26280_d_n2;
        locals.var_xp_dn4 = assign27480_e26280_d_n4;
        locals.var_xp_dn5 = assign27480_e26280_d_n5;
        locals.var_xp_dn6 = assign27480_e26280_d_n6;
        locals.var_xp_dn7 = assign27480_e26280_d_n7;
        locals.var_xp_dn8 = assign27480_e26280_d_n8;
        locals.var_xp_dn9 = assign27480_e26280_d_n9;
        locals.var_xp_dn10 = assign27480_e26280_d_n10;
        locals.var_xp_dn11 = assign27480_e26280_d_n11;
        locals.var_xp_dn14 = assign27480_e26280_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27490_e26290, assign27490_e26290_d_n0, assign27490_e26290_d_n2, assign27490_e26290_d_n4, assign27490_e26290_d_n5, assign27490_e26290_d_n6, assign27490_e26290_d_n7, assign27490_e26290_d_n8, assign27490_e26290_d_n9, assign27490_e26290_d_n10, assign27490_e26290_d_n11, assign27490_e26290_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27490_e26288: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27490_e26288, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27490_e26290;
        locals.var_xmp_dn0 = assign27490_e26290_d_n0;
        locals.var_xmp_dn2 = assign27490_e26290_d_n2;
        locals.var_xmp_dn4 = assign27490_e26290_d_n4;
        locals.var_xmp_dn5 = assign27490_e26290_d_n5;
        locals.var_xmp_dn6 = assign27490_e26290_d_n6;
        locals.var_xmp_dn7 = assign27490_e26290_d_n7;
        locals.var_xmp_dn8 = assign27490_e26290_d_n8;
        locals.var_xmp_dn9 = assign27490_e26290_d_n9;
        locals.var_xmp_dn10 = assign27490_e26290_d_n10;
        locals.var_xmp_dn11 = assign27490_e26290_d_n11;
        locals.var_xmp_dn14 = assign27490_e26290_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27500_e26300, assign27500_e26300_d_n0, assign27500_e26300_d_n2, assign27500_e26300_d_n4, assign27500_e26300_d_n5, assign27500_e26300_d_n6, assign27500_e26300_d_n7, assign27500_e26300_d_n8, assign27500_e26300_d_n9, assign27500_e26300_d_n10, assign27500_e26300_d_n11, assign27500_e26300_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27500_e26298: f64 = (locals.var_xp + locals.var_xmp);
        (assign27500_e26298, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27500_e26300;
        locals.var_arg_dn0 = assign27500_e26300_d_n0;
        locals.var_arg_dn2 = assign27500_e26300_d_n2;
        locals.var_arg_dn4 = assign27500_e26300_d_n4;
        locals.var_arg_dn5 = assign27500_e26300_d_n5;
        locals.var_arg_dn6 = assign27500_e26300_d_n6;
        locals.var_arg_dn7 = assign27500_e26300_d_n7;
        locals.var_arg_dn8 = assign27500_e26300_d_n8;
        locals.var_arg_dn9 = assign27500_e26300_d_n9;
        locals.var_arg_dn10 = assign27500_e26300_d_n10;
        locals.var_arg_dn11 = assign27500_e26300_d_n11;
        locals.var_arg_dn14 = assign27500_e26300_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign27510_e26308, assign27510_e26308_d_n0, assign27510_e26308_d_n2, assign27510_e26308_d_n4, assign27510_e26308_d_n5, assign27510_e26308_d_n6, assign27510_e26308_d_n7, assign27510_e26308_d_n8, assign27510_e26308_d_n9, assign27510_e26308_d_n10, assign27510_e26308_d_n11, assign27510_e26308_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27510_e26308;
        locals.var_dnm_dn0 = assign27510_e26308_d_n0;
        locals.var_dnm_dn2 = assign27510_e26308_d_n2;
        locals.var_dnm_dn4 = assign27510_e26308_d_n4;
        locals.var_dnm_dn5 = assign27510_e26308_d_n5;
        locals.var_dnm_dn6 = assign27510_e26308_d_n6;
        locals.var_dnm_dn7 = assign27510_e26308_d_n7;
        locals.var_dnm_dn8 = assign27510_e26308_d_n8;
        locals.var_dnm_dn9 = assign27510_e26308_d_n9;
        locals.var_dnm_dn10 = assign27510_e26308_d_n10;
        locals.var_dnm_dn11 = assign27510_e26308_d_n11;
        locals.var_dnm_dn14 = assign27510_e26308_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign27520_e26323: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard654 = assign27520_e26323;
        locals.var_guard654_rv = 0.0;

        let assign27530_e26326: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign27530_e26326;
        locals.var_guard655_rv = 0.0;

        let (assign27540_e26338,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27540_e26338;
        locals.var_mm_rv = 0.0;

        let assign27550_e26341: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign27550_e26341;
        locals.var_guard656_rv = 0.0;

        let (assign27560_e26356,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27560_e26356;
        locals.var_mm_rv = 0.0;

        let assign27570_e26359: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign27570_e26359;
        locals.var_guard657_rv = 0.0;

        let (assign27580_e26377,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27580_e26377;
        locals.var_mm_rv = 0.0;

        let assign27590_e26380: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign27590_e26380;
        locals.var_guard658_rv = 0.0;

        let (assign27600_e26401,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27600_e26401;
        locals.var_mm_rv = 0.0;

        let (assign27610_e26411,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27610_e26411;
        locals.var_m0_rv = 0.0;

        let mut assign27620_loop_guard: usize = 0;
        while {
            let assign27620_cond_e26422: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27620_cond_e26422 != 0.0
        } {
            assign27620_loop_guard += 1;
            assert!(assign27620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27620_body0_e26433, assign27620_body0_e26433_d_n0, assign27620_body0_e26433_d_n2, assign27620_body0_e26433_d_n4, assign27620_body0_e26433_d_n5, assign27620_body0_e26433_d_n6, assign27620_body0_e26433_d_n7, assign27620_body0_e26433_d_n8, assign27620_body0_e26433_d_n9, assign27620_body0_e26433_d_n10, assign27620_body0_e26433_d_n11, assign27620_body0_e26433_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign27620_body0_e26431: f64 = (locals.var_dnm).sqrt();
        (assign27620_body0_e26431, (locals.var_dnm_dn0 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn2 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn4 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn5 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn6 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn7 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn8 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn9 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn10 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn11 / (2.0 * assign27620_body0_e26431)), (locals.var_dnm_dn14 / (2.0 * assign27620_body0_e26431)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign27620_body0_e26433;
            locals.var_dnm_dn0 = assign27620_body0_e26433_d_n0;
            locals.var_dnm_dn2 = assign27620_body0_e26433_d_n2;
            locals.var_dnm_dn4 = assign27620_body0_e26433_d_n4;
            locals.var_dnm_dn5 = assign27620_body0_e26433_d_n5;
            locals.var_dnm_dn6 = assign27620_body0_e26433_d_n6;
            locals.var_dnm_dn7 = assign27620_body0_e26433_d_n7;
            locals.var_dnm_dn8 = assign27620_body0_e26433_d_n8;
            locals.var_dnm_dn9 = assign27620_body0_e26433_d_n9;
            locals.var_dnm_dn10 = assign27620_body0_e26433_d_n10;
            locals.var_dnm_dn11 = assign27620_body0_e26433_d_n11;
            locals.var_dnm_dn14 = assign27620_body0_e26433_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign27620_body1_e26445,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign27620_body1_e26443: f64 = (locals.var_m0 + 1.0);
        (assign27620_body1_e26443,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27620_body1_e26445;
            locals.var_m0_rv = 0.0;
        }

        let (assign27630_e26467, assign27630_e26467_d_n0, assign27630_e26467_d_n2, assign27630_e26467_d_n4, assign27630_e26467_d_n5, assign27630_e26467_d_n6, assign27630_e26467_d_n7, assign27630_e26467_d_n8, assign27630_e26467_d_n9, assign27630_e26467_d_n10, assign27630_e26467_d_n11, assign27630_e26467_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let (assign27630_e26465, assign27630_e26465_d_n0, assign27630_e26465_d_n2, assign27630_e26465_d_n4, assign27630_e26465_d_n5, assign27630_e26465_d_n6, assign27630_e26465_d_n7, assign27630_e26465_d_n8, assign27630_e26465_d_n9, assign27630_e26465_d_n10, assign27630_e26465_d_n11, assign27630_e26465_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27630_e26462: f64 = (2.0 * 2.0);
                let assign27630_e26463: f64 = (1.0 / assign27630_e26462);
                let assign27630_e26464: f64 = (locals.var_dnm).powf(assign27630_e26463);
                (assign27630_e26464, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn11)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27630_e26463) as f64).is_finite() && ((assign27630_e26463) as f64).fract() == 0.0 { if assign27630_e26463 == 0.0 { 0.0 } else { (assign27630_e26463 * ((locals.var_dnm).powf(assign27630_e26463 - 1.0) * locals.var_dnm_dn14)) } } else { (assign27630_e26464 * (assign27630_e26463 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign27630_e26465, assign27630_e26465_d_n0, assign27630_e26465_d_n2, assign27630_e26465_d_n4, assign27630_e26465_d_n5, assign27630_e26465_d_n6, assign27630_e26465_d_n7, assign27630_e26465_d_n8, assign27630_e26465_d_n9, assign27630_e26465_d_n10, assign27630_e26465_d_n11, assign27630_e26465_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27630_e26467;
        locals.var_dnm_dn0 = assign27630_e26467_d_n0;
        locals.var_dnm_dn2 = assign27630_e26467_d_n2;
        locals.var_dnm_dn4 = assign27630_e26467_d_n4;
        locals.var_dnm_dn5 = assign27630_e26467_d_n5;
        locals.var_dnm_dn6 = assign27630_e26467_d_n6;
        locals.var_dnm_dn7 = assign27630_e26467_d_n7;
        locals.var_dnm_dn8 = assign27630_e26467_d_n8;
        locals.var_dnm_dn9 = assign27630_e26467_d_n9;
        locals.var_dnm_dn10 = assign27630_e26467_d_n10;
        locals.var_dnm_dn11 = assign27630_e26467_d_n11;
        locals.var_dnm_dn14 = assign27630_e26467_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27640_e26477, assign27640_e26477_d_n0, assign27640_e26477_d_n2, assign27640_e26477_d_n4, assign27640_e26477_d_n5, assign27640_e26477_d_n6, assign27640_e26477_d_n7, assign27640_e26477_d_n8, assign27640_e26477_d_n9, assign27640_e26477_d_n10, assign27640_e26477_d_n11, assign27640_e26477_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27640_e26475: f64 = (1.0 / locals.var_dnm);
        (assign27640_e26475, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27640_e26477;
        locals.var_dnm_dn0 = assign27640_e26477_d_n0;
        locals.var_dnm_dn2 = assign27640_e26477_d_n2;
        locals.var_dnm_dn4 = assign27640_e26477_d_n4;
        locals.var_dnm_dn5 = assign27640_e26477_d_n5;
        locals.var_dnm_dn6 = assign27640_e26477_d_n6;
        locals.var_dnm_dn7 = assign27640_e26477_d_n7;
        locals.var_dnm_dn8 = assign27640_e26477_d_n8;
        locals.var_dnm_dn9 = assign27640_e26477_d_n9;
        locals.var_dnm_dn10 = assign27640_e26477_d_n10;
        locals.var_dnm_dn11 = assign27640_e26477_d_n11;
        locals.var_dnm_dn14 = assign27640_e26477_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27650_e26489, assign27650_e26489_d_n0, assign27650_e26489_d_n2, assign27650_e26489_d_n4, assign27650_e26489_d_n5, assign27650_e26489_d_n6, assign27650_e26489_d_n7, assign27650_e26489_d_n8, assign27650_e26489_d_n9, assign27650_e26489_d_n10, assign27650_e26489_d_n11, assign27650_e26489_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27650_e26485: f64 = (locals.var_tmf1 * 1e-8);
        let assign27650_e26487: f64 = (assign27650_e26485 * locals.var_dnm);
        (assign27650_e26487, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign27650_e26485 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign27650_e26489;
        locals.var_tmf0_dn0 = assign27650_e26489_d_n0;
        locals.var_tmf0_dn2 = assign27650_e26489_d_n2;
        locals.var_tmf0_dn4 = assign27650_e26489_d_n4;
        locals.var_tmf0_dn5 = assign27650_e26489_d_n5;
        locals.var_tmf0_dn6 = assign27650_e26489_d_n6;
        locals.var_tmf0_dn7 = assign27650_e26489_d_n7;
        locals.var_tmf0_dn8 = assign27650_e26489_d_n8;
        locals.var_tmf0_dn9 = assign27650_e26489_d_n9;
        locals.var_tmf0_dn10 = assign27650_e26489_d_n10;
        locals.var_tmf0_dn11 = assign27650_e26489_d_n11;
        locals.var_tmf0_dn14 = assign27650_e26489_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign27660_e26503, assign27660_e26503_d_n0, assign27660_e26503_d_n2, assign27660_e26503_d_n4, assign27660_e26503_d_n5, assign27660_e26503_d_n6, assign27660_e26503_d_n7, assign27660_e26503_d_n8, assign27660_e26503_d_n9, assign27660_e26503_d_n10, assign27660_e26503_d_n11, assign27660_e26503_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27660_e26497: f64 = (1e-8 * locals.var_xmp);
        let assign27660_e26499: f64 = (assign27660_e26497 * locals.var_dnm);
        let assign27660_e26501: f64 = (assign27660_e26499 / locals.var_arg);
        (assign27660_e26501, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn11)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign27660_e26497 * locals.var_dnm_dn14)) * locals.var_arg) - (assign27660_e26499 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27660_e26503;
        locals.var_t3_dn0 = assign27660_e26503_d_n0;
        locals.var_t3_dn2 = assign27660_e26503_d_n2;
        locals.var_t3_dn4 = assign27660_e26503_d_n4;
        locals.var_t3_dn5 = assign27660_e26503_d_n5;
        locals.var_t3_dn6 = assign27660_e26503_d_n6;
        locals.var_t3_dn7 = assign27660_e26503_d_n7;
        locals.var_t3_dn8 = assign27660_e26503_d_n8;
        locals.var_t3_dn9 = assign27660_e26503_d_n9;
        locals.var_t3_dn10 = assign27660_e26503_d_n10;
        locals.var_t3_dn11 = assign27660_e26503_d_n11;
        locals.var_t3_dn14 = assign27660_e26503_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27670_e26515, assign27670_e26515_d_n0, assign27670_e26515_d_n2, assign27670_e26515_d_n4, assign27670_e26515_d_n5, assign27670_e26515_d_n6, assign27670_e26515_d_n7, assign27670_e26515_d_n8, assign27670_e26515_d_n9, assign27670_e26515_d_n10, assign27670_e26515_d_n11, assign27670_e26515_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign27670_e26511: f64 = (locals.var_uc_depthn - 1e-8);
        let assign27670_e26513: f64 = (assign27670_e26511 + locals.var_tmf0);
        (assign27670_e26513, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign27670_e26515;
        locals.var_w_b0_dn0 = assign27670_e26515_d_n0;
        locals.var_w_b0_dn2 = assign27670_e26515_d_n2;
        locals.var_w_b0_dn4 = assign27670_e26515_d_n4;
        locals.var_w_b0_dn5 = assign27670_e26515_d_n5;
        locals.var_w_b0_dn6 = assign27670_e26515_d_n6;
        locals.var_w_b0_dn7 = assign27670_e26515_d_n7;
        locals.var_w_b0_dn8 = assign27670_e26515_d_n8;
        locals.var_w_b0_dn9 = assign27670_e26515_d_n9;
        locals.var_w_b0_dn10 = assign27670_e26515_d_n10;
        locals.var_w_b0_dn11 = assign27670_e26515_d_n11;
        locals.var_w_b0_dn14 = assign27670_e26515_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign27680_e26523, assign27680_e26523_d_n0, assign27680_e26523_d_n2, assign27680_e26523_d_n4, assign27680_e26523_d_n5, assign27680_e26523_d_n6, assign27680_e26523_d_n7, assign27680_e26523_d_n8, assign27680_e26523_d_n9, assign27680_e26523_d_n10, assign27680_e26523_d_n11, assign27680_e26523_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27680_e26523;
        locals.var_t3_dn0 = assign27680_e26523_d_n0;
        locals.var_t3_dn2 = assign27680_e26523_d_n2;
        locals.var_t3_dn4 = assign27680_e26523_d_n4;
        locals.var_t3_dn5 = assign27680_e26523_d_n5;
        locals.var_t3_dn6 = assign27680_e26523_d_n6;
        locals.var_t3_dn7 = assign27680_e26523_d_n7;
        locals.var_t3_dn8 = assign27680_e26523_d_n8;
        locals.var_t3_dn9 = assign27680_e26523_d_n9;
        locals.var_t3_dn10 = assign27680_e26523_d_n10;
        locals.var_t3_dn11 = assign27680_e26523_d_n11;
        locals.var_t3_dn14 = assign27680_e26523_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27690_e26532, assign27690_e26532_d_n0, assign27690_e26532_d_n2, assign27690_e26532_d_n4, assign27690_e26532_d_n5, assign27690_e26532_d_n6, assign27690_e26532_d_n7, assign27690_e26532_d_n8, assign27690_e26532_d_n9, assign27690_e26532_d_n10, assign27690_e26532_d_n11, assign27690_e26532_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign27690_e26532;
        locals.var_w_b0_dn0 = assign27690_e26532_d_n0;
        locals.var_w_b0_dn2 = assign27690_e26532_d_n2;
        locals.var_w_b0_dn4 = assign27690_e26532_d_n4;
        locals.var_w_b0_dn5 = assign27690_e26532_d_n5;
        locals.var_w_b0_dn6 = assign27690_e26532_d_n6;
        locals.var_w_b0_dn7 = assign27690_e26532_d_n7;
        locals.var_w_b0_dn8 = assign27690_e26532_d_n8;
        locals.var_w_b0_dn9 = assign27690_e26532_d_n9;
        locals.var_w_b0_dn10 = assign27690_e26532_d_n10;
        locals.var_w_b0_dn11 = assign27690_e26532_d_n11;
        locals.var_w_b0_dn14 = assign27690_e26532_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign27700_e26541, assign27700_e26541_d_n0, assign27700_e26541_d_n2, assign27700_e26541_d_n4, assign27700_e26541_d_n5, assign27700_e26541_d_n6, assign27700_e26541_d_n7, assign27700_e26541_d_n8, assign27700_e26541_d_n9, assign27700_e26541_d_n10, assign27700_e26541_d_n11, assign27700_e26541_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard653 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27700_e26541;
        locals.var_t3_dn0 = assign27700_e26541_d_n0;
        locals.var_t3_dn2 = assign27700_e26541_d_n2;
        locals.var_t3_dn4 = assign27700_e26541_d_n4;
        locals.var_t3_dn5 = assign27700_e26541_d_n5;
        locals.var_t3_dn6 = assign27700_e26541_d_n6;
        locals.var_t3_dn7 = assign27700_e26541_d_n7;
        locals.var_t3_dn8 = assign27700_e26541_d_n8;
        locals.var_t3_dn9 = assign27700_e26541_d_n9;
        locals.var_t3_dn10 = assign27700_e26541_d_n10;
        locals.var_t3_dn11 = assign27700_e26541_d_n11;
        locals.var_t3_dn14 = assign27700_e26541_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27710_e26549, assign27710_e26549_d_n0, assign27710_e26549_d_n2, assign27710_e26549_d_n4, assign27710_e26549_d_n5, assign27710_e26549_d_n6, assign27710_e26549_d_n7, assign27710_e26549_d_n8, assign27710_e26549_d_n9, assign27710_e26549_d_n10, assign27710_e26549_d_n11, assign27710_e26549_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign27710_e26547: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep);
        (assign27710_e26547, (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27710_e26549;
        locals.var_t1_dn0 = assign27710_e26549_d_n0;
        locals.var_t1_dn2 = assign27710_e26549_d_n2;
        locals.var_t1_dn4 = assign27710_e26549_d_n4;
        locals.var_t1_dn5 = assign27710_e26549_d_n5;
        locals.var_t1_dn6 = assign27710_e26549_d_n6;
        locals.var_t1_dn7 = assign27710_e26549_d_n7;
        locals.var_t1_dn8 = assign27710_e26549_d_n8;
        locals.var_t1_dn9 = assign27710_e26549_d_n9;
        locals.var_t1_dn10 = assign27710_e26549_d_n10;
        locals.var_t1_dn11 = assign27710_e26549_d_n11;
        locals.var_t1_dn14 = assign27710_e26549_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27720_e26553: f64 = 0.05;
        let assign27720_e26558: f64 = if ((locals.var_t1 < assign27720_e26553) && (0.05 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard659 = assign27720_e26558;
        locals.var_guard659_rv = 0.0;

        let (assign27730_e26570, assign27730_e26570_d_n0, assign27730_e26570_d_n2, assign27730_e26570_d_n4, assign27730_e26570_d_n5, assign27730_e26570_d_n6, assign27730_e26570_d_n7, assign27730_e26570_d_n8, assign27730_e26570_d_n9, assign27730_e26570_d_n10, assign27730_e26570_d_n11, assign27730_e26570_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27730_e26566: f64 = 0.05;
        let assign27730_e26568: f64 = (assign27730_e26566 - locals.var_t1);
        (assign27730_e26568, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign27730_e26570;
        locals.var_tmf1_dn0 = assign27730_e26570_d_n0;
        locals.var_tmf1_dn2 = assign27730_e26570_d_n2;
        locals.var_tmf1_dn4 = assign27730_e26570_d_n4;
        locals.var_tmf1_dn5 = assign27730_e26570_d_n5;
        locals.var_tmf1_dn6 = assign27730_e26570_d_n6;
        locals.var_tmf1_dn7 = assign27730_e26570_d_n7;
        locals.var_tmf1_dn8 = assign27730_e26570_d_n8;
        locals.var_tmf1_dn9 = assign27730_e26570_d_n9;
        locals.var_tmf1_dn10 = assign27730_e26570_d_n10;
        locals.var_tmf1_dn11 = assign27730_e26570_d_n11;
        locals.var_tmf1_dn14 = assign27730_e26570_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign27740_e26580, assign27740_e26580_d_n0, assign27740_e26580_d_n2, assign27740_e26580_d_n4, assign27740_e26580_d_n5, assign27740_e26580_d_n6, assign27740_e26580_d_n7, assign27740_e26580_d_n8, assign27740_e26580_d_n9, assign27740_e26580_d_n10, assign27740_e26580_d_n11, assign27740_e26580_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27740_e26578: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27740_e26578, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign27740_e26580;
        locals.var_x2_dn0 = assign27740_e26580_d_n0;
        locals.var_x2_dn2 = assign27740_e26580_d_n2;
        locals.var_x2_dn4 = assign27740_e26580_d_n4;
        locals.var_x2_dn5 = assign27740_e26580_d_n5;
        locals.var_x2_dn6 = assign27740_e26580_d_n6;
        locals.var_x2_dn7 = assign27740_e26580_d_n7;
        locals.var_x2_dn8 = assign27740_e26580_d_n8;
        locals.var_x2_dn9 = assign27740_e26580_d_n9;
        locals.var_x2_dn10 = assign27740_e26580_d_n10;
        locals.var_x2_dn11 = assign27740_e26580_d_n11;
        locals.var_x2_dn14 = assign27740_e26580_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign27750_e26590, assign27750_e26590_d_n0, assign27750_e26590_d_n2, assign27750_e26590_d_n4, assign27750_e26590_d_n5, assign27750_e26590_d_n6, assign27750_e26590_d_n7, assign27750_e26590_d_n8, assign27750_e26590_d_n9, assign27750_e26590_d_n10, assign27750_e26590_d_n11, assign27750_e26590_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27750_e26588: f64 = (0.05 * 0.05);
        (assign27750_e26588, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign27750_e26590;
        locals.var_xmax2_dn0 = assign27750_e26590_d_n0;
        locals.var_xmax2_dn2 = assign27750_e26590_d_n2;
        locals.var_xmax2_dn4 = assign27750_e26590_d_n4;
        locals.var_xmax2_dn5 = assign27750_e26590_d_n5;
        locals.var_xmax2_dn6 = assign27750_e26590_d_n6;
        locals.var_xmax2_dn7 = assign27750_e26590_d_n7;
        locals.var_xmax2_dn8 = assign27750_e26590_d_n8;
        locals.var_xmax2_dn9 = assign27750_e26590_d_n9;
        locals.var_xmax2_dn10 = assign27750_e26590_d_n10;
        locals.var_xmax2_dn11 = assign27750_e26590_d_n11;
        locals.var_xmax2_dn14 = assign27750_e26590_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign27760_e26598, assign27760_e26598_d_n0, assign27760_e26598_d_n2, assign27760_e26598_d_n4, assign27760_e26598_d_n5, assign27760_e26598_d_n6, assign27760_e26598_d_n7, assign27760_e26598_d_n8, assign27760_e26598_d_n9, assign27760_e26598_d_n10, assign27760_e26598_d_n11, assign27760_e26598_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27760_e26598;
        locals.var_xp_dn0 = assign27760_e26598_d_n0;
        locals.var_xp_dn2 = assign27760_e26598_d_n2;
        locals.var_xp_dn4 = assign27760_e26598_d_n4;
        locals.var_xp_dn5 = assign27760_e26598_d_n5;
        locals.var_xp_dn6 = assign27760_e26598_d_n6;
        locals.var_xp_dn7 = assign27760_e26598_d_n7;
        locals.var_xp_dn8 = assign27760_e26598_d_n8;
        locals.var_xp_dn9 = assign27760_e26598_d_n9;
        locals.var_xp_dn10 = assign27760_e26598_d_n10;
        locals.var_xp_dn11 = assign27760_e26598_d_n11;
        locals.var_xp_dn14 = assign27760_e26598_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27770_e26606, assign27770_e26606_d_n0, assign27770_e26606_d_n2, assign27770_e26606_d_n4, assign27770_e26606_d_n5, assign27770_e26606_d_n6, assign27770_e26606_d_n7, assign27770_e26606_d_n8, assign27770_e26606_d_n9, assign27770_e26606_d_n10, assign27770_e26606_d_n11, assign27770_e26606_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27770_e26606;
        locals.var_xmp_dn0 = assign27770_e26606_d_n0;
        locals.var_xmp_dn2 = assign27770_e26606_d_n2;
        locals.var_xmp_dn4 = assign27770_e26606_d_n4;
        locals.var_xmp_dn5 = assign27770_e26606_d_n5;
        locals.var_xmp_dn6 = assign27770_e26606_d_n6;
        locals.var_xmp_dn7 = assign27770_e26606_d_n7;
        locals.var_xmp_dn8 = assign27770_e26606_d_n8;
        locals.var_xmp_dn9 = assign27770_e26606_d_n9;
        locals.var_xmp_dn10 = assign27770_e26606_d_n10;
        locals.var_xmp_dn11 = assign27770_e26606_d_n11;
        locals.var_xmp_dn14 = assign27770_e26606_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27780_e26614,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27780_e26614;
        locals.var_m0_rv = 0.0;

        let (assign27790_e26622,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27790_e26622;
        locals.var_mm_rv = 0.0;

        let (assign27800_e26630, assign27800_e26630_d_n0, assign27800_e26630_d_n2, assign27800_e26630_d_n4, assign27800_e26630_d_n5, assign27800_e26630_d_n6, assign27800_e26630_d_n7, assign27800_e26630_d_n8, assign27800_e26630_d_n9, assign27800_e26630_d_n10, assign27800_e26630_d_n11, assign27800_e26630_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27800_e26630;
        locals.var_arg_dn0 = assign27800_e26630_d_n0;
        locals.var_arg_dn2 = assign27800_e26630_d_n2;
        locals.var_arg_dn4 = assign27800_e26630_d_n4;
        locals.var_arg_dn5 = assign27800_e26630_d_n5;
        locals.var_arg_dn6 = assign27800_e26630_d_n6;
        locals.var_arg_dn7 = assign27800_e26630_d_n7;
        locals.var_arg_dn8 = assign27800_e26630_d_n8;
        locals.var_arg_dn9 = assign27800_e26630_d_n9;
        locals.var_arg_dn10 = assign27800_e26630_d_n10;
        locals.var_arg_dn11 = assign27800_e26630_d_n11;
        locals.var_arg_dn14 = assign27800_e26630_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign27810_e26638, assign27810_e26638_d_n0, assign27810_e26638_d_n2, assign27810_e26638_d_n4, assign27810_e26638_d_n5, assign27810_e26638_d_n6, assign27810_e26638_d_n7, assign27810_e26638_d_n8, assign27810_e26638_d_n9, assign27810_e26638_d_n10, assign27810_e26638_d_n11, assign27810_e26638_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27810_e26638;
        locals.var_dnm_dn0 = assign27810_e26638_d_n0;
        locals.var_dnm_dn2 = assign27810_e26638_d_n2;
        locals.var_dnm_dn4 = assign27810_e26638_d_n4;
        locals.var_dnm_dn5 = assign27810_e26638_d_n5;
        locals.var_dnm_dn6 = assign27810_e26638_d_n6;
        locals.var_dnm_dn7 = assign27810_e26638_d_n7;
        locals.var_dnm_dn8 = assign27810_e26638_d_n8;
        locals.var_dnm_dn9 = assign27810_e26638_d_n9;
        locals.var_dnm_dn10 = assign27810_e26638_d_n10;
        locals.var_dnm_dn11 = assign27810_e26638_d_n11;
        locals.var_dnm_dn14 = assign27810_e26638_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27820_e26648, assign27820_e26648_d_n0, assign27820_e26648_d_n2, assign27820_e26648_d_n4, assign27820_e26648_d_n5, assign27820_e26648_d_n6, assign27820_e26648_d_n7, assign27820_e26648_d_n8, assign27820_e26648_d_n9, assign27820_e26648_d_n10, assign27820_e26648_d_n11, assign27820_e26648_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27820_e26646: f64 = (locals.var_xp * locals.var_x2);
        (assign27820_e26646, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27820_e26648;
        locals.var_xp_dn0 = assign27820_e26648_d_n0;
        locals.var_xp_dn2 = assign27820_e26648_d_n2;
        locals.var_xp_dn4 = assign27820_e26648_d_n4;
        locals.var_xp_dn5 = assign27820_e26648_d_n5;
        locals.var_xp_dn6 = assign27820_e26648_d_n6;
        locals.var_xp_dn7 = assign27820_e26648_d_n7;
        locals.var_xp_dn8 = assign27820_e26648_d_n8;
        locals.var_xp_dn9 = assign27820_e26648_d_n9;
        locals.var_xp_dn10 = assign27820_e26648_d_n10;
        locals.var_xp_dn11 = assign27820_e26648_d_n11;
        locals.var_xp_dn14 = assign27820_e26648_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27830_e26658, assign27830_e26658_d_n0, assign27830_e26658_d_n2, assign27830_e26658_d_n4, assign27830_e26658_d_n5, assign27830_e26658_d_n6, assign27830_e26658_d_n7, assign27830_e26658_d_n8, assign27830_e26658_d_n9, assign27830_e26658_d_n10, assign27830_e26658_d_n11, assign27830_e26658_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27830_e26656: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27830_e26656, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27830_e26658;
        locals.var_xmp_dn0 = assign27830_e26658_d_n0;
        locals.var_xmp_dn2 = assign27830_e26658_d_n2;
        locals.var_xmp_dn4 = assign27830_e26658_d_n4;
        locals.var_xmp_dn5 = assign27830_e26658_d_n5;
        locals.var_xmp_dn6 = assign27830_e26658_d_n6;
        locals.var_xmp_dn7 = assign27830_e26658_d_n7;
        locals.var_xmp_dn8 = assign27830_e26658_d_n8;
        locals.var_xmp_dn9 = assign27830_e26658_d_n9;
        locals.var_xmp_dn10 = assign27830_e26658_d_n10;
        locals.var_xmp_dn11 = assign27830_e26658_d_n11;
        locals.var_xmp_dn14 = assign27830_e26658_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27840_e26668, assign27840_e26668_d_n0, assign27840_e26668_d_n2, assign27840_e26668_d_n4, assign27840_e26668_d_n5, assign27840_e26668_d_n6, assign27840_e26668_d_n7, assign27840_e26668_d_n8, assign27840_e26668_d_n9, assign27840_e26668_d_n10, assign27840_e26668_d_n11, assign27840_e26668_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27840_e26666: f64 = (locals.var_xp * locals.var_x2);
        (assign27840_e26666, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27840_e26668;
        locals.var_xp_dn0 = assign27840_e26668_d_n0;
        locals.var_xp_dn2 = assign27840_e26668_d_n2;
        locals.var_xp_dn4 = assign27840_e26668_d_n4;
        locals.var_xp_dn5 = assign27840_e26668_d_n5;
        locals.var_xp_dn6 = assign27840_e26668_d_n6;
        locals.var_xp_dn7 = assign27840_e26668_d_n7;
        locals.var_xp_dn8 = assign27840_e26668_d_n8;
        locals.var_xp_dn9 = assign27840_e26668_d_n9;
        locals.var_xp_dn10 = assign27840_e26668_d_n10;
        locals.var_xp_dn11 = assign27840_e26668_d_n11;
        locals.var_xp_dn14 = assign27840_e26668_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27850_e26678, assign27850_e26678_d_n0, assign27850_e26678_d_n2, assign27850_e26678_d_n4, assign27850_e26678_d_n5, assign27850_e26678_d_n6, assign27850_e26678_d_n7, assign27850_e26678_d_n8, assign27850_e26678_d_n9, assign27850_e26678_d_n10, assign27850_e26678_d_n11, assign27850_e26678_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27850_e26676: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27850_e26676, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27850_e26678;
        locals.var_xmp_dn0 = assign27850_e26678_d_n0;
        locals.var_xmp_dn2 = assign27850_e26678_d_n2;
        locals.var_xmp_dn4 = assign27850_e26678_d_n4;
        locals.var_xmp_dn5 = assign27850_e26678_d_n5;
        locals.var_xmp_dn6 = assign27850_e26678_d_n6;
        locals.var_xmp_dn7 = assign27850_e26678_d_n7;
        locals.var_xmp_dn8 = assign27850_e26678_d_n8;
        locals.var_xmp_dn9 = assign27850_e26678_d_n9;
        locals.var_xmp_dn10 = assign27850_e26678_d_n10;
        locals.var_xmp_dn11 = assign27850_e26678_d_n11;
        locals.var_xmp_dn14 = assign27850_e26678_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27860_e26688, assign27860_e26688_d_n0, assign27860_e26688_d_n2, assign27860_e26688_d_n4, assign27860_e26688_d_n5, assign27860_e26688_d_n6, assign27860_e26688_d_n7, assign27860_e26688_d_n8, assign27860_e26688_d_n9, assign27860_e26688_d_n10, assign27860_e26688_d_n11, assign27860_e26688_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign27860_e26686: f64 = (locals.var_xp + locals.var_xmp);
        (assign27860_e26686, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27860_e26688;
        locals.var_arg_dn0 = assign27860_e26688_d_n0;
        locals.var_arg_dn2 = assign27860_e26688_d_n2;
        locals.var_arg_dn4 = assign27860_e26688_d_n4;
        locals.var_arg_dn5 = assign27860_e26688_d_n5;
        locals.var_arg_dn6 = assign27860_e26688_d_n6;
        locals.var_arg_dn7 = assign27860_e26688_d_n7;
        locals.var_arg_dn8 = assign27860_e26688_d_n8;
        locals.var_arg_dn9 = assign27860_e26688_d_n9;
        locals.var_arg_dn10 = assign27860_e26688_d_n10;
        locals.var_arg_dn11 = assign27860_e26688_d_n11;
        locals.var_arg_dn14 = assign27860_e26688_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27870_e26696, assign27870_e26696_d_n0, assign27870_e26696_d_n2, assign27870_e26696_d_n4, assign27870_e26696_d_n5, assign27870_e26696_d_n6, assign27870_e26696_d_n7, assign27870_e26696_d_n8, assign27870_e26696_d_n9, assign27870_e26696_d_n10, assign27870_e26696_d_n11, assign27870_e26696_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27870_e26696;
        locals.var_dnm_dn0 = assign27870_e26696_d_n0;
        locals.var_dnm_dn2 = assign27870_e26696_d_n2;
        locals.var_dnm_dn4 = assign27870_e26696_d_n4;
        locals.var_dnm_dn5 = assign27870_e26696_d_n5;
        locals.var_dnm_dn6 = assign27870_e26696_d_n6;
        locals.var_dnm_dn7 = assign27870_e26696_d_n7;
        locals.var_dnm_dn8 = assign27870_e26696_d_n8;
        locals.var_dnm_dn9 = assign27870_e26696_d_n9;
        locals.var_dnm_dn10 = assign27870_e26696_d_n10;
        locals.var_dnm_dn11 = assign27870_e26696_d_n11;
        locals.var_dnm_dn14 = assign27870_e26696_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign27880_e26711: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard660 = assign27880_e26711;
        locals.var_guard660_rv = 0.0;

        let assign27890_e26714: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign27890_e26714;
        locals.var_guard661_rv = 0.0;

        let (assign27900_e26726,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27900_e26726;
        locals.var_mm_rv = 0.0;

        let assign27910_e26729: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign27910_e26729;
        locals.var_guard662_rv = 0.0;

        let (assign27920_e26744,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27920_e26744;
        locals.var_mm_rv = 0.0;

        let assign27930_e26747: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign27930_e26747;
        locals.var_guard663_rv = 0.0;

        let (assign27940_e26765,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 == 0.0)) && (locals.var_guard663 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27940_e26765;
        locals.var_mm_rv = 0.0;

        let assign27950_e26768: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign27950_e26768;
        locals.var_guard664_rv = 0.0;

        let (assign27960_e26789,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27960_e26789;
        locals.var_mm_rv = 0.0;

        let (assign27970_e26799,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27970_e26799;
        locals.var_m0_rv = 0.0;

        let mut assign27980_loop_guard: usize = 0;
        while {
            let assign27980_cond_e26810: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27980_cond_e26810 != 0.0
        } {
            assign27980_loop_guard += 1;
            assert!(assign27980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27980_body0_e26821, assign27980_body0_e26821_d_n0, assign27980_body0_e26821_d_n2, assign27980_body0_e26821_d_n4, assign27980_body0_e26821_d_n5, assign27980_body0_e26821_d_n6, assign27980_body0_e26821_d_n7, assign27980_body0_e26821_d_n8, assign27980_body0_e26821_d_n9, assign27980_body0_e26821_d_n10, assign27980_body0_e26821_d_n11, assign27980_body0_e26821_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign27980_body0_e26819: f64 = (locals.var_dnm).sqrt();
        (assign27980_body0_e26819, (locals.var_dnm_dn0 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn2 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn4 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn5 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn6 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn7 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn8 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn9 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn10 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn11 / (2.0 * assign27980_body0_e26819)), (locals.var_dnm_dn14 / (2.0 * assign27980_body0_e26819)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign27980_body0_e26821;
            locals.var_dnm_dn0 = assign27980_body0_e26821_d_n0;
            locals.var_dnm_dn2 = assign27980_body0_e26821_d_n2;
            locals.var_dnm_dn4 = assign27980_body0_e26821_d_n4;
            locals.var_dnm_dn5 = assign27980_body0_e26821_d_n5;
            locals.var_dnm_dn6 = assign27980_body0_e26821_d_n6;
            locals.var_dnm_dn7 = assign27980_body0_e26821_d_n7;
            locals.var_dnm_dn8 = assign27980_body0_e26821_d_n8;
            locals.var_dnm_dn9 = assign27980_body0_e26821_d_n9;
            locals.var_dnm_dn10 = assign27980_body0_e26821_d_n10;
            locals.var_dnm_dn11 = assign27980_body0_e26821_d_n11;
            locals.var_dnm_dn14 = assign27980_body0_e26821_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign27980_body1_e26833,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign27980_body1_e26831: f64 = (locals.var_m0 + 1.0);
        (assign27980_body1_e26831,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27980_body1_e26833;
            locals.var_m0_rv = 0.0;
        }

        let (assign27990_e26855, assign27990_e26855_d_n0, assign27990_e26855_d_n2, assign27990_e26855_d_n4, assign27990_e26855_d_n5, assign27990_e26855_d_n6, assign27990_e26855_d_n7, assign27990_e26855_d_n8, assign27990_e26855_d_n9, assign27990_e26855_d_n10, assign27990_e26855_d_n11, assign27990_e26855_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let (assign27990_e26853, assign27990_e26853_d_n0, assign27990_e26853_d_n2, assign27990_e26853_d_n4, assign27990_e26853_d_n5, assign27990_e26853_d_n6, assign27990_e26853_d_n7, assign27990_e26853_d_n8, assign27990_e26853_d_n9, assign27990_e26853_d_n10, assign27990_e26853_d_n11, assign27990_e26853_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27990_e26850: f64 = (2.0 * 2.0);
                let assign27990_e26851: f64 = (1.0 / assign27990_e26850);
                let assign27990_e26852: f64 = (locals.var_dnm).powf(assign27990_e26851);
                (assign27990_e26852, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn11)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27990_e26851) as f64).is_finite() && ((assign27990_e26851) as f64).fract() == 0.0 { if assign27990_e26851 == 0.0 { 0.0 } else { (assign27990_e26851 * ((locals.var_dnm).powf(assign27990_e26851 - 1.0) * locals.var_dnm_dn14)) } } else { (assign27990_e26852 * (assign27990_e26851 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign27990_e26853, assign27990_e26853_d_n0, assign27990_e26853_d_n2, assign27990_e26853_d_n4, assign27990_e26853_d_n5, assign27990_e26853_d_n6, assign27990_e26853_d_n7, assign27990_e26853_d_n8, assign27990_e26853_d_n9, assign27990_e26853_d_n10, assign27990_e26853_d_n11, assign27990_e26853_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27990_e26855;
        locals.var_dnm_dn0 = assign27990_e26855_d_n0;
        locals.var_dnm_dn2 = assign27990_e26855_d_n2;
        locals.var_dnm_dn4 = assign27990_e26855_d_n4;
        locals.var_dnm_dn5 = assign27990_e26855_d_n5;
        locals.var_dnm_dn6 = assign27990_e26855_d_n6;
        locals.var_dnm_dn7 = assign27990_e26855_d_n7;
        locals.var_dnm_dn8 = assign27990_e26855_d_n8;
        locals.var_dnm_dn9 = assign27990_e26855_d_n9;
        locals.var_dnm_dn10 = assign27990_e26855_d_n10;
        locals.var_dnm_dn11 = assign27990_e26855_d_n11;
        locals.var_dnm_dn14 = assign27990_e26855_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28000_e26865, assign28000_e26865_d_n0, assign28000_e26865_d_n2, assign28000_e26865_d_n4, assign28000_e26865_d_n5, assign28000_e26865_d_n6, assign28000_e26865_d_n7, assign28000_e26865_d_n8, assign28000_e26865_d_n9, assign28000_e26865_d_n10, assign28000_e26865_d_n11, assign28000_e26865_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign28000_e26863: f64 = (1.0 / locals.var_dnm);
        (assign28000_e26863, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28000_e26865;
        locals.var_dnm_dn0 = assign28000_e26865_d_n0;
        locals.var_dnm_dn2 = assign28000_e26865_d_n2;
        locals.var_dnm_dn4 = assign28000_e26865_d_n4;
        locals.var_dnm_dn5 = assign28000_e26865_d_n5;
        locals.var_dnm_dn6 = assign28000_e26865_d_n6;
        locals.var_dnm_dn7 = assign28000_e26865_d_n7;
        locals.var_dnm_dn8 = assign28000_e26865_d_n8;
        locals.var_dnm_dn9 = assign28000_e26865_d_n9;
        locals.var_dnm_dn10 = assign28000_e26865_d_n10;
        locals.var_dnm_dn11 = assign28000_e26865_d_n11;
        locals.var_dnm_dn14 = assign28000_e26865_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28010_e26877, assign28010_e26877_d_n0, assign28010_e26877_d_n2, assign28010_e26877_d_n4, assign28010_e26877_d_n5, assign28010_e26877_d_n6, assign28010_e26877_d_n7, assign28010_e26877_d_n8, assign28010_e26877_d_n9, assign28010_e26877_d_n10, assign28010_e26877_d_n11, assign28010_e26877_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign28010_e26873: f64 = (locals.var_tmf1 * 0.05);
        let assign28010_e26875: f64 = (assign28010_e26873 * locals.var_dnm);
        (assign28010_e26875, (((locals.var_tmf1_dn0 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.05) * locals.var_dnm) + (assign28010_e26873 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28010_e26877;
        locals.var_tmf0_dn0 = assign28010_e26877_d_n0;
        locals.var_tmf0_dn2 = assign28010_e26877_d_n2;
        locals.var_tmf0_dn4 = assign28010_e26877_d_n4;
        locals.var_tmf0_dn5 = assign28010_e26877_d_n5;
        locals.var_tmf0_dn6 = assign28010_e26877_d_n6;
        locals.var_tmf0_dn7 = assign28010_e26877_d_n7;
        locals.var_tmf0_dn8 = assign28010_e26877_d_n8;
        locals.var_tmf0_dn9 = assign28010_e26877_d_n9;
        locals.var_tmf0_dn10 = assign28010_e26877_d_n10;
        locals.var_tmf0_dn11 = assign28010_e26877_d_n11;
        locals.var_tmf0_dn14 = assign28010_e26877_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign28020_e26891, assign28020_e26891_d_n0, assign28020_e26891_d_n2, assign28020_e26891_d_n4, assign28020_e26891_d_n5, assign28020_e26891_d_n6, assign28020_e26891_d_n7, assign28020_e26891_d_n8, assign28020_e26891_d_n9, assign28020_e26891_d_n10, assign28020_e26891_d_n11, assign28020_e26891_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign28020_e26885: f64 = (0.05 * locals.var_xmp);
        let assign28020_e26887: f64 = (assign28020_e26885 * locals.var_dnm);
        let assign28020_e26889: f64 = (assign28020_e26887 / locals.var_arg);
        (assign28020_e26889, ((((((0.05 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28020_e26891;
        locals.var_t0_dn0 = assign28020_e26891_d_n0;
        locals.var_t0_dn2 = assign28020_e26891_d_n2;
        locals.var_t0_dn4 = assign28020_e26891_d_n4;
        locals.var_t0_dn5 = assign28020_e26891_d_n5;
        locals.var_t0_dn6 = assign28020_e26891_d_n6;
        locals.var_t0_dn7 = assign28020_e26891_d_n7;
        locals.var_t0_dn8 = assign28020_e26891_d_n8;
        locals.var_t0_dn9 = assign28020_e26891_d_n9;
        locals.var_t0_dn10 = assign28020_e26891_d_n10;
        locals.var_t0_dn11 = assign28020_e26891_d_n11;
        locals.var_t0_dn14 = assign28020_e26891_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign28030_e26903, assign28030_e26903_d_n0, assign28030_e26903_d_n2, assign28030_e26903_d_n4, assign28030_e26903_d_n5, assign28030_e26903_d_n6, assign28030_e26903_d_n7, assign28030_e26903_d_n8, assign28030_e26903_d_n9, assign28030_e26903_d_n10, assign28030_e26903_d_n11, assign28030_e26903_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign28030_e26899: f64 = 0.05;
        let assign28030_e26901: f64 = (assign28030_e26899 - locals.var_tmf0);
        (assign28030_e26901, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28030_e26903;
        locals.var_t2_dn0 = assign28030_e26903_d_n0;
        locals.var_t2_dn2 = assign28030_e26903_d_n2;
        locals.var_t2_dn4 = assign28030_e26903_d_n4;
        locals.var_t2_dn5 = assign28030_e26903_d_n5;
        locals.var_t2_dn6 = assign28030_e26903_d_n6;
        locals.var_t2_dn7 = assign28030_e26903_d_n7;
        locals.var_t2_dn8 = assign28030_e26903_d_n8;
        locals.var_t2_dn9 = assign28030_e26903_d_n9;
        locals.var_t2_dn10 = assign28030_e26903_d_n10;
        locals.var_t2_dn11 = assign28030_e26903_d_n11;
        locals.var_t2_dn14 = assign28030_e26903_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28040_e26911, assign28040_e26911_d_n0, assign28040_e26911_d_n2, assign28040_e26911_d_n4, assign28040_e26911_d_n5, assign28040_e26911_d_n6, assign28040_e26911_d_n7, assign28040_e26911_d_n8, assign28040_e26911_d_n9, assign28040_e26911_d_n10, assign28040_e26911_d_n11, assign28040_e26911_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28040_e26911;
        locals.var_t0_dn0 = assign28040_e26911_d_n0;
        locals.var_t0_dn2 = assign28040_e26911_d_n2;
        locals.var_t0_dn4 = assign28040_e26911_d_n4;
        locals.var_t0_dn5 = assign28040_e26911_d_n5;
        locals.var_t0_dn6 = assign28040_e26911_d_n6;
        locals.var_t0_dn7 = assign28040_e26911_d_n7;
        locals.var_t0_dn8 = assign28040_e26911_d_n8;
        locals.var_t0_dn9 = assign28040_e26911_d_n9;
        locals.var_t0_dn10 = assign28040_e26911_d_n10;
        locals.var_t0_dn11 = assign28040_e26911_d_n11;
        locals.var_t0_dn14 = assign28040_e26911_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign28050_e26920, assign28050_e26920_d_n0, assign28050_e26920_d_n2, assign28050_e26920_d_n4, assign28050_e26920_d_n5, assign28050_e26920_d_n6, assign28050_e26920_d_n7, assign28050_e26920_d_n8, assign28050_e26920_d_n9, assign28050_e26920_d_n10, assign28050_e26920_d_n11, assign28050_e26920_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28050_e26920;
        locals.var_t2_dn0 = assign28050_e26920_d_n0;
        locals.var_t2_dn2 = assign28050_e26920_d_n2;
        locals.var_t2_dn4 = assign28050_e26920_d_n4;
        locals.var_t2_dn5 = assign28050_e26920_d_n5;
        locals.var_t2_dn6 = assign28050_e26920_d_n6;
        locals.var_t2_dn7 = assign28050_e26920_d_n7;
        locals.var_t2_dn8 = assign28050_e26920_d_n8;
        locals.var_t2_dn9 = assign28050_e26920_d_n9;
        locals.var_t2_dn10 = assign28050_e26920_d_n10;
        locals.var_t2_dn11 = assign28050_e26920_d_n11;
        locals.var_t2_dn14 = assign28050_e26920_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28060_e26929, assign28060_e26929_d_n0, assign28060_e26929_d_n2, assign28060_e26929_d_n4, assign28060_e26929_d_n5, assign28060_e26929_d_n6, assign28060_e26929_d_n7, assign28060_e26929_d_n8, assign28060_e26929_d_n9, assign28060_e26929_d_n10, assign28060_e26929_d_n11, assign28060_e26929_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28060_e26929;
        locals.var_t0_dn0 = assign28060_e26929_d_n0;
        locals.var_t0_dn2 = assign28060_e26929_d_n2;
        locals.var_t0_dn4 = assign28060_e26929_d_n4;
        locals.var_t0_dn5 = assign28060_e26929_d_n5;
        locals.var_t0_dn6 = assign28060_e26929_d_n6;
        locals.var_t0_dn7 = assign28060_e26929_d_n7;
        locals.var_t0_dn8 = assign28060_e26929_d_n8;
        locals.var_t0_dn9 = assign28060_e26929_d_n9;
        locals.var_t0_dn10 = assign28060_e26929_d_n10;
        locals.var_t0_dn11 = assign28060_e26929_d_n11;
        locals.var_t0_dn14 = assign28060_e26929_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign28070_e26938, assign28070_e26938_d_n0, assign28070_e26938_d_n2, assign28070_e26938_d_n4, assign28070_e26938_d_n5, assign28070_e26938_d_n6, assign28070_e26938_d_n7, assign28070_e26938_d_n8, assign28070_e26938_d_n9, assign28070_e26938_d_n10, assign28070_e26938_d_n11, assign28070_e26938_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign28070_e26935: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign28070_e26936: f64 = (assign28070_e26935).sqrt();
        (assign28070_e26936, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign28070_e26936)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn11, locals.var_w_s0_dn14,)
    }
};
        locals.var_w_s0 = assign28070_e26938;
        locals.var_w_s0_dn0 = assign28070_e26938_d_n0;
        locals.var_w_s0_dn2 = assign28070_e26938_d_n2;
        locals.var_w_s0_dn4 = assign28070_e26938_d_n4;
        locals.var_w_s0_dn5 = assign28070_e26938_d_n5;
        locals.var_w_s0_dn6 = assign28070_e26938_d_n6;
        locals.var_w_s0_dn7 = assign28070_e26938_d_n7;
        locals.var_w_s0_dn8 = assign28070_e26938_d_n8;
        locals.var_w_s0_dn9 = assign28070_e26938_d_n9;
        locals.var_w_s0_dn10 = assign28070_e26938_d_n10;
        locals.var_w_s0_dn11 = assign28070_e26938_d_n11;
        locals.var_w_s0_dn14 = assign28070_e26938_d_n14;
        locals.var_w_s0_rv = 0.0;

        let (assign28080_e26948, assign28080_e26948_d_n0, assign28080_e26948_d_n2, assign28080_e26948_d_n4, assign28080_e26948_d_n5, assign28080_e26948_d_n6, assign28080_e26948_d_n7, assign28080_e26948_d_n8, assign28080_e26948_d_n9, assign28080_e26948_d_n10, assign28080_e26948_d_n11, assign28080_e26948_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign28080_e26944: f64 = (locals.var_uc_depthn - locals.var_w_b0);
        let assign28080_e26946: f64 = (assign28080_e26944 - locals.var_w_s0);
        (assign28080_e26946, ((locals.var_uc_depthn_dn0 - locals.var_w_b0_dn0) - locals.var_w_s0_dn0), ((locals.var_uc_depthn_dn2 - locals.var_w_b0_dn2) - locals.var_w_s0_dn2), ((locals.var_uc_depthn_dn4 - locals.var_w_b0_dn4) - locals.var_w_s0_dn4), ((locals.var_uc_depthn_dn5 - locals.var_w_b0_dn5) - locals.var_w_s0_dn5), ((locals.var_uc_depthn_dn6 - locals.var_w_b0_dn6) - locals.var_w_s0_dn6), ((locals.var_uc_depthn_dn7 - locals.var_w_b0_dn7) - locals.var_w_s0_dn7), ((locals.var_uc_depthn_dn8 - locals.var_w_b0_dn8) - locals.var_w_s0_dn8), ((locals.var_uc_depthn_dn9 - locals.var_w_b0_dn9) - locals.var_w_s0_dn9), ((locals.var_uc_depthn_dn10 - locals.var_w_b0_dn10) - locals.var_w_s0_dn10), ((locals.var_uc_depthn_dn11 - locals.var_w_b0_dn11) - locals.var_w_s0_dn11), ((locals.var_uc_depthn_dn14 - locals.var_w_b0_dn14) - locals.var_w_s0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28080_e26948;
        locals.var_t1_dn0 = assign28080_e26948_d_n0;
        locals.var_t1_dn2 = assign28080_e26948_d_n2;
        locals.var_t1_dn4 = assign28080_e26948_d_n4;
        locals.var_t1_dn5 = assign28080_e26948_d_n5;
        locals.var_t1_dn6 = assign28080_e26948_d_n6;
        locals.var_t1_dn7 = assign28080_e26948_d_n7;
        locals.var_t1_dn8 = assign28080_e26948_d_n8;
        locals.var_t1_dn9 = assign28080_e26948_d_n9;
        locals.var_t1_dn10 = assign28080_e26948_d_n10;
        locals.var_t1_dn11 = assign28080_e26948_d_n11;
        locals.var_t1_dn14 = assign28080_e26948_d_n14;
        locals.var_t1_rv = 0.0;

        let assign28090_e26952: f64 = (1e-25 + 1e-18);
        let assign28090_e26957: f64 = if ((locals.var_t1 < assign28090_e26952) && (1e-18 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard665 = assign28090_e26957;
        locals.var_guard665_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        locals: &mut StampLocals,
    ) {
        let (assign28100_e26969, assign28100_e26969_d_n0, assign28100_e26969_d_n2, assign28100_e26969_d_n4, assign28100_e26969_d_n5, assign28100_e26969_d_n6, assign28100_e26969_d_n7, assign28100_e26969_d_n8, assign28100_e26969_d_n9, assign28100_e26969_d_n10, assign28100_e26969_d_n11, assign28100_e26969_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28100_e26965: f64 = (1e-25 + 1e-18);
        let assign28100_e26967: f64 = (assign28100_e26965 - locals.var_t1);
        (assign28100_e26967, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28100_e26969;
        locals.var_tmf1_dn0 = assign28100_e26969_d_n0;
        locals.var_tmf1_dn2 = assign28100_e26969_d_n2;
        locals.var_tmf1_dn4 = assign28100_e26969_d_n4;
        locals.var_tmf1_dn5 = assign28100_e26969_d_n5;
        locals.var_tmf1_dn6 = assign28100_e26969_d_n6;
        locals.var_tmf1_dn7 = assign28100_e26969_d_n7;
        locals.var_tmf1_dn8 = assign28100_e26969_d_n8;
        locals.var_tmf1_dn9 = assign28100_e26969_d_n9;
        locals.var_tmf1_dn10 = assign28100_e26969_d_n10;
        locals.var_tmf1_dn11 = assign28100_e26969_d_n11;
        locals.var_tmf1_dn14 = assign28100_e26969_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign28110_e26979, assign28110_e26979_d_n0, assign28110_e26979_d_n2, assign28110_e26979_d_n4, assign28110_e26979_d_n5, assign28110_e26979_d_n6, assign28110_e26979_d_n7, assign28110_e26979_d_n8, assign28110_e26979_d_n9, assign28110_e26979_d_n10, assign28110_e26979_d_n11, assign28110_e26979_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28110_e26977: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28110_e26977, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28110_e26979;
        locals.var_x2_dn0 = assign28110_e26979_d_n0;
        locals.var_x2_dn2 = assign28110_e26979_d_n2;
        locals.var_x2_dn4 = assign28110_e26979_d_n4;
        locals.var_x2_dn5 = assign28110_e26979_d_n5;
        locals.var_x2_dn6 = assign28110_e26979_d_n6;
        locals.var_x2_dn7 = assign28110_e26979_d_n7;
        locals.var_x2_dn8 = assign28110_e26979_d_n8;
        locals.var_x2_dn9 = assign28110_e26979_d_n9;
        locals.var_x2_dn10 = assign28110_e26979_d_n10;
        locals.var_x2_dn11 = assign28110_e26979_d_n11;
        locals.var_x2_dn14 = assign28110_e26979_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign28120_e26989, assign28120_e26989_d_n0, assign28120_e26989_d_n2, assign28120_e26989_d_n4, assign28120_e26989_d_n5, assign28120_e26989_d_n6, assign28120_e26989_d_n7, assign28120_e26989_d_n8, assign28120_e26989_d_n9, assign28120_e26989_d_n10, assign28120_e26989_d_n11, assign28120_e26989_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28120_e26987: f64 = (1e-18 * 1e-18);
        (assign28120_e26987, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28120_e26989;
        locals.var_xmax2_dn0 = assign28120_e26989_d_n0;
        locals.var_xmax2_dn2 = assign28120_e26989_d_n2;
        locals.var_xmax2_dn4 = assign28120_e26989_d_n4;
        locals.var_xmax2_dn5 = assign28120_e26989_d_n5;
        locals.var_xmax2_dn6 = assign28120_e26989_d_n6;
        locals.var_xmax2_dn7 = assign28120_e26989_d_n7;
        locals.var_xmax2_dn8 = assign28120_e26989_d_n8;
        locals.var_xmax2_dn9 = assign28120_e26989_d_n9;
        locals.var_xmax2_dn10 = assign28120_e26989_d_n10;
        locals.var_xmax2_dn11 = assign28120_e26989_d_n11;
        locals.var_xmax2_dn14 = assign28120_e26989_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign28130_e26997, assign28130_e26997_d_n0, assign28130_e26997_d_n2, assign28130_e26997_d_n4, assign28130_e26997_d_n5, assign28130_e26997_d_n6, assign28130_e26997_d_n7, assign28130_e26997_d_n8, assign28130_e26997_d_n9, assign28130_e26997_d_n10, assign28130_e26997_d_n11, assign28130_e26997_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28130_e26997;
        locals.var_xp_dn0 = assign28130_e26997_d_n0;
        locals.var_xp_dn2 = assign28130_e26997_d_n2;
        locals.var_xp_dn4 = assign28130_e26997_d_n4;
        locals.var_xp_dn5 = assign28130_e26997_d_n5;
        locals.var_xp_dn6 = assign28130_e26997_d_n6;
        locals.var_xp_dn7 = assign28130_e26997_d_n7;
        locals.var_xp_dn8 = assign28130_e26997_d_n8;
        locals.var_xp_dn9 = assign28130_e26997_d_n9;
        locals.var_xp_dn10 = assign28130_e26997_d_n10;
        locals.var_xp_dn11 = assign28130_e26997_d_n11;
        locals.var_xp_dn14 = assign28130_e26997_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28140_e27005, assign28140_e27005_d_n0, assign28140_e27005_d_n2, assign28140_e27005_d_n4, assign28140_e27005_d_n5, assign28140_e27005_d_n6, assign28140_e27005_d_n7, assign28140_e27005_d_n8, assign28140_e27005_d_n9, assign28140_e27005_d_n10, assign28140_e27005_d_n11, assign28140_e27005_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28140_e27005;
        locals.var_xmp_dn0 = assign28140_e27005_d_n0;
        locals.var_xmp_dn2 = assign28140_e27005_d_n2;
        locals.var_xmp_dn4 = assign28140_e27005_d_n4;
        locals.var_xmp_dn5 = assign28140_e27005_d_n5;
        locals.var_xmp_dn6 = assign28140_e27005_d_n6;
        locals.var_xmp_dn7 = assign28140_e27005_d_n7;
        locals.var_xmp_dn8 = assign28140_e27005_d_n8;
        locals.var_xmp_dn9 = assign28140_e27005_d_n9;
        locals.var_xmp_dn10 = assign28140_e27005_d_n10;
        locals.var_xmp_dn11 = assign28140_e27005_d_n11;
        locals.var_xmp_dn14 = assign28140_e27005_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28150_e27013,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28150_e27013;
        locals.var_m0_rv = 0.0;

        let (assign28160_e27021,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28160_e27021;
        locals.var_mm_rv = 0.0;

        let (assign28170_e27029, assign28170_e27029_d_n0, assign28170_e27029_d_n2, assign28170_e27029_d_n4, assign28170_e27029_d_n5, assign28170_e27029_d_n6, assign28170_e27029_d_n7, assign28170_e27029_d_n8, assign28170_e27029_d_n9, assign28170_e27029_d_n10, assign28170_e27029_d_n11, assign28170_e27029_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28170_e27029;
        locals.var_arg_dn0 = assign28170_e27029_d_n0;
        locals.var_arg_dn2 = assign28170_e27029_d_n2;
        locals.var_arg_dn4 = assign28170_e27029_d_n4;
        locals.var_arg_dn5 = assign28170_e27029_d_n5;
        locals.var_arg_dn6 = assign28170_e27029_d_n6;
        locals.var_arg_dn7 = assign28170_e27029_d_n7;
        locals.var_arg_dn8 = assign28170_e27029_d_n8;
        locals.var_arg_dn9 = assign28170_e27029_d_n9;
        locals.var_arg_dn10 = assign28170_e27029_d_n10;
        locals.var_arg_dn11 = assign28170_e27029_d_n11;
        locals.var_arg_dn14 = assign28170_e27029_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign28180_e27037, assign28180_e27037_d_n0, assign28180_e27037_d_n2, assign28180_e27037_d_n4, assign28180_e27037_d_n5, assign28180_e27037_d_n6, assign28180_e27037_d_n7, assign28180_e27037_d_n8, assign28180_e27037_d_n9, assign28180_e27037_d_n10, assign28180_e27037_d_n11, assign28180_e27037_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28180_e27037;
        locals.var_dnm_dn0 = assign28180_e27037_d_n0;
        locals.var_dnm_dn2 = assign28180_e27037_d_n2;
        locals.var_dnm_dn4 = assign28180_e27037_d_n4;
        locals.var_dnm_dn5 = assign28180_e27037_d_n5;
        locals.var_dnm_dn6 = assign28180_e27037_d_n6;
        locals.var_dnm_dn7 = assign28180_e27037_d_n7;
        locals.var_dnm_dn8 = assign28180_e27037_d_n8;
        locals.var_dnm_dn9 = assign28180_e27037_d_n9;
        locals.var_dnm_dn10 = assign28180_e27037_d_n10;
        locals.var_dnm_dn11 = assign28180_e27037_d_n11;
        locals.var_dnm_dn14 = assign28180_e27037_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28190_e27047, assign28190_e27047_d_n0, assign28190_e27047_d_n2, assign28190_e27047_d_n4, assign28190_e27047_d_n5, assign28190_e27047_d_n6, assign28190_e27047_d_n7, assign28190_e27047_d_n8, assign28190_e27047_d_n9, assign28190_e27047_d_n10, assign28190_e27047_d_n11, assign28190_e27047_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28190_e27045: f64 = (locals.var_xp * locals.var_x2);
        (assign28190_e27045, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28190_e27047;
        locals.var_xp_dn0 = assign28190_e27047_d_n0;
        locals.var_xp_dn2 = assign28190_e27047_d_n2;
        locals.var_xp_dn4 = assign28190_e27047_d_n4;
        locals.var_xp_dn5 = assign28190_e27047_d_n5;
        locals.var_xp_dn6 = assign28190_e27047_d_n6;
        locals.var_xp_dn7 = assign28190_e27047_d_n7;
        locals.var_xp_dn8 = assign28190_e27047_d_n8;
        locals.var_xp_dn9 = assign28190_e27047_d_n9;
        locals.var_xp_dn10 = assign28190_e27047_d_n10;
        locals.var_xp_dn11 = assign28190_e27047_d_n11;
        locals.var_xp_dn14 = assign28190_e27047_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28200_e27057, assign28200_e27057_d_n0, assign28200_e27057_d_n2, assign28200_e27057_d_n4, assign28200_e27057_d_n5, assign28200_e27057_d_n6, assign28200_e27057_d_n7, assign28200_e27057_d_n8, assign28200_e27057_d_n9, assign28200_e27057_d_n10, assign28200_e27057_d_n11, assign28200_e27057_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28200_e27055: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28200_e27055, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28200_e27057;
        locals.var_xmp_dn0 = assign28200_e27057_d_n0;
        locals.var_xmp_dn2 = assign28200_e27057_d_n2;
        locals.var_xmp_dn4 = assign28200_e27057_d_n4;
        locals.var_xmp_dn5 = assign28200_e27057_d_n5;
        locals.var_xmp_dn6 = assign28200_e27057_d_n6;
        locals.var_xmp_dn7 = assign28200_e27057_d_n7;
        locals.var_xmp_dn8 = assign28200_e27057_d_n8;
        locals.var_xmp_dn9 = assign28200_e27057_d_n9;
        locals.var_xmp_dn10 = assign28200_e27057_d_n10;
        locals.var_xmp_dn11 = assign28200_e27057_d_n11;
        locals.var_xmp_dn14 = assign28200_e27057_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28210_e27067, assign28210_e27067_d_n0, assign28210_e27067_d_n2, assign28210_e27067_d_n4, assign28210_e27067_d_n5, assign28210_e27067_d_n6, assign28210_e27067_d_n7, assign28210_e27067_d_n8, assign28210_e27067_d_n9, assign28210_e27067_d_n10, assign28210_e27067_d_n11, assign28210_e27067_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28210_e27065: f64 = (locals.var_xp * locals.var_x2);
        (assign28210_e27065, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28210_e27067;
        locals.var_xp_dn0 = assign28210_e27067_d_n0;
        locals.var_xp_dn2 = assign28210_e27067_d_n2;
        locals.var_xp_dn4 = assign28210_e27067_d_n4;
        locals.var_xp_dn5 = assign28210_e27067_d_n5;
        locals.var_xp_dn6 = assign28210_e27067_d_n6;
        locals.var_xp_dn7 = assign28210_e27067_d_n7;
        locals.var_xp_dn8 = assign28210_e27067_d_n8;
        locals.var_xp_dn9 = assign28210_e27067_d_n9;
        locals.var_xp_dn10 = assign28210_e27067_d_n10;
        locals.var_xp_dn11 = assign28210_e27067_d_n11;
        locals.var_xp_dn14 = assign28210_e27067_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28220_e27077, assign28220_e27077_d_n0, assign28220_e27077_d_n2, assign28220_e27077_d_n4, assign28220_e27077_d_n5, assign28220_e27077_d_n6, assign28220_e27077_d_n7, assign28220_e27077_d_n8, assign28220_e27077_d_n9, assign28220_e27077_d_n10, assign28220_e27077_d_n11, assign28220_e27077_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28220_e27075: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28220_e27075, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28220_e27077;
        locals.var_xmp_dn0 = assign28220_e27077_d_n0;
        locals.var_xmp_dn2 = assign28220_e27077_d_n2;
        locals.var_xmp_dn4 = assign28220_e27077_d_n4;
        locals.var_xmp_dn5 = assign28220_e27077_d_n5;
        locals.var_xmp_dn6 = assign28220_e27077_d_n6;
        locals.var_xmp_dn7 = assign28220_e27077_d_n7;
        locals.var_xmp_dn8 = assign28220_e27077_d_n8;
        locals.var_xmp_dn9 = assign28220_e27077_d_n9;
        locals.var_xmp_dn10 = assign28220_e27077_d_n10;
        locals.var_xmp_dn11 = assign28220_e27077_d_n11;
        locals.var_xmp_dn14 = assign28220_e27077_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28230_e27087, assign28230_e27087_d_n0, assign28230_e27087_d_n2, assign28230_e27087_d_n4, assign28230_e27087_d_n5, assign28230_e27087_d_n6, assign28230_e27087_d_n7, assign28230_e27087_d_n8, assign28230_e27087_d_n9, assign28230_e27087_d_n10, assign28230_e27087_d_n11, assign28230_e27087_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28230_e27085: f64 = (locals.var_xp + locals.var_xmp);
        (assign28230_e27085, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28230_e27087;
        locals.var_arg_dn0 = assign28230_e27087_d_n0;
        locals.var_arg_dn2 = assign28230_e27087_d_n2;
        locals.var_arg_dn4 = assign28230_e27087_d_n4;
        locals.var_arg_dn5 = assign28230_e27087_d_n5;
        locals.var_arg_dn6 = assign28230_e27087_d_n6;
        locals.var_arg_dn7 = assign28230_e27087_d_n7;
        locals.var_arg_dn8 = assign28230_e27087_d_n8;
        locals.var_arg_dn9 = assign28230_e27087_d_n9;
        locals.var_arg_dn10 = assign28230_e27087_d_n10;
        locals.var_arg_dn11 = assign28230_e27087_d_n11;
        locals.var_arg_dn14 = assign28230_e27087_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign28240_e27095, assign28240_e27095_d_n0, assign28240_e27095_d_n2, assign28240_e27095_d_n4, assign28240_e27095_d_n5, assign28240_e27095_d_n6, assign28240_e27095_d_n7, assign28240_e27095_d_n8, assign28240_e27095_d_n9, assign28240_e27095_d_n10, assign28240_e27095_d_n11, assign28240_e27095_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28240_e27095;
        locals.var_dnm_dn0 = assign28240_e27095_d_n0;
        locals.var_dnm_dn2 = assign28240_e27095_d_n2;
        locals.var_dnm_dn4 = assign28240_e27095_d_n4;
        locals.var_dnm_dn5 = assign28240_e27095_d_n5;
        locals.var_dnm_dn6 = assign28240_e27095_d_n6;
        locals.var_dnm_dn7 = assign28240_e27095_d_n7;
        locals.var_dnm_dn8 = assign28240_e27095_d_n8;
        locals.var_dnm_dn9 = assign28240_e27095_d_n9;
        locals.var_dnm_dn10 = assign28240_e27095_d_n10;
        locals.var_dnm_dn11 = assign28240_e27095_d_n11;
        locals.var_dnm_dn14 = assign28240_e27095_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign28250_e27110: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard666 = assign28250_e27110;
        locals.var_guard666_rv = 0.0;

        let assign28260_e27113: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign28260_e27113;
        locals.var_guard667_rv = 0.0;

        let (assign28270_e27125,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28270_e27125;
        locals.var_mm_rv = 0.0;

        let assign28280_e27128: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign28280_e27128;
        locals.var_guard668_rv = 0.0;

        let (assign28290_e27143,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28290_e27143;
        locals.var_mm_rv = 0.0;

        let assign28300_e27146: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign28300_e27146;
        locals.var_guard669_rv = 0.0;

        let (assign28310_e27164,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) && (locals.var_guard669 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28310_e27164;
        locals.var_mm_rv = 0.0;

        let assign28320_e27167: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign28320_e27167;
        locals.var_guard670_rv = 0.0;

        let (assign28330_e27188,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28330_e27188;
        locals.var_mm_rv = 0.0;

        let (assign28340_e27198,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28340_e27198;
        locals.var_m0_rv = 0.0;

        let mut assign28350_loop_guard: usize = 0;
        while {
            let assign28350_cond_e27209: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28350_cond_e27209 != 0.0
        } {
            assign28350_loop_guard += 1;
            assert!(assign28350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28350_body0_e27220, assign28350_body0_e27220_d_n0, assign28350_body0_e27220_d_n2, assign28350_body0_e27220_d_n4, assign28350_body0_e27220_d_n5, assign28350_body0_e27220_d_n6, assign28350_body0_e27220_d_n7, assign28350_body0_e27220_d_n8, assign28350_body0_e27220_d_n9, assign28350_body0_e27220_d_n10, assign28350_body0_e27220_d_n11, assign28350_body0_e27220_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        let assign28350_body0_e27218: f64 = (locals.var_dnm).sqrt();
        (assign28350_body0_e27218, (locals.var_dnm_dn0 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn2 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn4 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn5 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn6 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn7 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn8 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn9 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn10 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn11 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn14 / (2.0 * assign28350_body0_e27218)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign28350_body0_e27220;
            locals.var_dnm_dn0 = assign28350_body0_e27220_d_n0;
            locals.var_dnm_dn2 = assign28350_body0_e27220_d_n2;
            locals.var_dnm_dn4 = assign28350_body0_e27220_d_n4;
            locals.var_dnm_dn5 = assign28350_body0_e27220_d_n5;
            locals.var_dnm_dn6 = assign28350_body0_e27220_d_n6;
            locals.var_dnm_dn7 = assign28350_body0_e27220_d_n7;
            locals.var_dnm_dn8 = assign28350_body0_e27220_d_n8;
            locals.var_dnm_dn9 = assign28350_body0_e27220_d_n9;
            locals.var_dnm_dn10 = assign28350_body0_e27220_d_n10;
            locals.var_dnm_dn11 = assign28350_body0_e27220_d_n11;
            locals.var_dnm_dn14 = assign28350_body0_e27220_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign28350_body1_e27232,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        let assign28350_body1_e27230: f64 = (locals.var_m0 + 1.0);
        (assign28350_body1_e27230,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28350_body1_e27232;
            locals.var_m0_rv = 0.0;
        }

        let (assign28360_e27254, assign28360_e27254_d_n0, assign28360_e27254_d_n2, assign28360_e27254_d_n4, assign28360_e27254_d_n5, assign28360_e27254_d_n6, assign28360_e27254_d_n7, assign28360_e27254_d_n8, assign28360_e27254_d_n9, assign28360_e27254_d_n10, assign28360_e27254_d_n11, assign28360_e27254_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 == 0.0)) {
        let (assign28360_e27252, assign28360_e27252_d_n0, assign28360_e27252_d_n2, assign28360_e27252_d_n4, assign28360_e27252_d_n5, assign28360_e27252_d_n6, assign28360_e27252_d_n7, assign28360_e27252_d_n8, assign28360_e27252_d_n9, assign28360_e27252_d_n10, assign28360_e27252_d_n11, assign28360_e27252_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28360_e27249: f64 = (2.0 * 2.0);
                let assign28360_e27250: f64 = (1.0 / assign28360_e27249);
                let assign28360_e27251: f64 = (locals.var_dnm).powf(assign28360_e27250);
                (assign28360_e27251, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn11)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn14)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign28360_e27252, assign28360_e27252_d_n0, assign28360_e27252_d_n2, assign28360_e27252_d_n4, assign28360_e27252_d_n5, assign28360_e27252_d_n6, assign28360_e27252_d_n7, assign28360_e27252_d_n8, assign28360_e27252_d_n9, assign28360_e27252_d_n10, assign28360_e27252_d_n11, assign28360_e27252_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28360_e27254;
        locals.var_dnm_dn0 = assign28360_e27254_d_n0;
        locals.var_dnm_dn2 = assign28360_e27254_d_n2;
        locals.var_dnm_dn4 = assign28360_e27254_d_n4;
        locals.var_dnm_dn5 = assign28360_e27254_d_n5;
        locals.var_dnm_dn6 = assign28360_e27254_d_n6;
        locals.var_dnm_dn7 = assign28360_e27254_d_n7;
        locals.var_dnm_dn8 = assign28360_e27254_d_n8;
        locals.var_dnm_dn9 = assign28360_e27254_d_n9;
        locals.var_dnm_dn10 = assign28360_e27254_d_n10;
        locals.var_dnm_dn11 = assign28360_e27254_d_n11;
        locals.var_dnm_dn14 = assign28360_e27254_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28370_e27264, assign28370_e27264_d_n0, assign28370_e27264_d_n2, assign28370_e27264_d_n4, assign28370_e27264_d_n5, assign28370_e27264_d_n6, assign28370_e27264_d_n7, assign28370_e27264_d_n8, assign28370_e27264_d_n9, assign28370_e27264_d_n10, assign28370_e27264_d_n11, assign28370_e27264_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28370_e27262: f64 = (1.0 / locals.var_dnm);
        (assign28370_e27262, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28370_e27264;
        locals.var_dnm_dn0 = assign28370_e27264_d_n0;
        locals.var_dnm_dn2 = assign28370_e27264_d_n2;
        locals.var_dnm_dn4 = assign28370_e27264_d_n4;
        locals.var_dnm_dn5 = assign28370_e27264_d_n5;
        locals.var_dnm_dn6 = assign28370_e27264_d_n6;
        locals.var_dnm_dn7 = assign28370_e27264_d_n7;
        locals.var_dnm_dn8 = assign28370_e27264_d_n8;
        locals.var_dnm_dn9 = assign28370_e27264_d_n9;
        locals.var_dnm_dn10 = assign28370_e27264_d_n10;
        locals.var_dnm_dn11 = assign28370_e27264_d_n11;
        locals.var_dnm_dn14 = assign28370_e27264_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28380_e27276, assign28380_e27276_d_n0, assign28380_e27276_d_n2, assign28380_e27276_d_n4, assign28380_e27276_d_n5, assign28380_e27276_d_n6, assign28380_e27276_d_n7, assign28380_e27276_d_n8, assign28380_e27276_d_n9, assign28380_e27276_d_n10, assign28380_e27276_d_n11, assign28380_e27276_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28380_e27272: f64 = (locals.var_tmf1 * 1e-18);
        let assign28380_e27274: f64 = (assign28380_e27272 * locals.var_dnm);
        (assign28380_e27274, (((locals.var_tmf1_dn0 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28380_e27276;
        locals.var_tmf0_dn0 = assign28380_e27276_d_n0;
        locals.var_tmf0_dn2 = assign28380_e27276_d_n2;
        locals.var_tmf0_dn4 = assign28380_e27276_d_n4;
        locals.var_tmf0_dn5 = assign28380_e27276_d_n5;
        locals.var_tmf0_dn6 = assign28380_e27276_d_n6;
        locals.var_tmf0_dn7 = assign28380_e27276_d_n7;
        locals.var_tmf0_dn8 = assign28380_e27276_d_n8;
        locals.var_tmf0_dn9 = assign28380_e27276_d_n9;
        locals.var_tmf0_dn10 = assign28380_e27276_d_n10;
        locals.var_tmf0_dn11 = assign28380_e27276_d_n11;
        locals.var_tmf0_dn14 = assign28380_e27276_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign28390_e27290, assign28390_e27290_d_n0, assign28390_e27290_d_n2, assign28390_e27290_d_n4, assign28390_e27290_d_n5, assign28390_e27290_d_n6, assign28390_e27290_d_n7, assign28390_e27290_d_n8, assign28390_e27290_d_n9, assign28390_e27290_d_n10, assign28390_e27290_d_n11, assign28390_e27290_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28390_e27284: f64 = (1e-18 * locals.var_xmp);
        let assign28390_e27286: f64 = (assign28390_e27284 * locals.var_dnm);
        let assign28390_e27288: f64 = (assign28390_e27286 / locals.var_arg);
        (assign28390_e27288, ((((((1e-18 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28390_e27290;
        locals.var_t0_dn0 = assign28390_e27290_d_n0;
        locals.var_t0_dn2 = assign28390_e27290_d_n2;
        locals.var_t0_dn4 = assign28390_e27290_d_n4;
        locals.var_t0_dn5 = assign28390_e27290_d_n5;
        locals.var_t0_dn6 = assign28390_e27290_d_n6;
        locals.var_t0_dn7 = assign28390_e27290_d_n7;
        locals.var_t0_dn8 = assign28390_e27290_d_n8;
        locals.var_t0_dn9 = assign28390_e27290_d_n9;
        locals.var_t0_dn10 = assign28390_e27290_d_n10;
        locals.var_t0_dn11 = assign28390_e27290_d_n11;
        locals.var_t0_dn14 = assign28390_e27290_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        locals: &mut StampLocals,
    ) {
        let (assign28400_e27302, assign28400_e27302_d_n0, assign28400_e27302_d_n2, assign28400_e27302_d_n4, assign28400_e27302_d_n5, assign28400_e27302_d_n6, assign28400_e27302_d_n7, assign28400_e27302_d_n8, assign28400_e27302_d_n9, assign28400_e27302_d_n10, assign28400_e27302_d_n11, assign28400_e27302_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28400_e27298: f64 = (1e-25 + 1e-18);
        let assign28400_e27300: f64 = (assign28400_e27298 - locals.var_tmf0);
        (assign28400_e27300, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn11, locals.var_w_res0_dn14,)
    }
};
        locals.var_w_res0 = assign28400_e27302;
        locals.var_w_res0_dn0 = assign28400_e27302_d_n0;
        locals.var_w_res0_dn2 = assign28400_e27302_d_n2;
        locals.var_w_res0_dn4 = assign28400_e27302_d_n4;
        locals.var_w_res0_dn5 = assign28400_e27302_d_n5;
        locals.var_w_res0_dn6 = assign28400_e27302_d_n6;
        locals.var_w_res0_dn7 = assign28400_e27302_d_n7;
        locals.var_w_res0_dn8 = assign28400_e27302_d_n8;
        locals.var_w_res0_dn9 = assign28400_e27302_d_n9;
        locals.var_w_res0_dn10 = assign28400_e27302_d_n10;
        locals.var_w_res0_dn11 = assign28400_e27302_d_n11;
        locals.var_w_res0_dn14 = assign28400_e27302_d_n14;
        locals.var_w_res0_rv = 0.0;

        let (assign28410_e27310, assign28410_e27310_d_n0, assign28410_e27310_d_n2, assign28410_e27310_d_n4, assign28410_e27310_d_n5, assign28410_e27310_d_n6, assign28410_e27310_d_n7, assign28410_e27310_d_n8, assign28410_e27310_d_n9, assign28410_e27310_d_n10, assign28410_e27310_d_n11, assign28410_e27310_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28410_e27310;
        locals.var_t0_dn0 = assign28410_e27310_d_n0;
        locals.var_t0_dn2 = assign28410_e27310_d_n2;
        locals.var_t0_dn4 = assign28410_e27310_d_n4;
        locals.var_t0_dn5 = assign28410_e27310_d_n5;
        locals.var_t0_dn6 = assign28410_e27310_d_n6;
        locals.var_t0_dn7 = assign28410_e27310_d_n7;
        locals.var_t0_dn8 = assign28410_e27310_d_n8;
        locals.var_t0_dn9 = assign28410_e27310_d_n9;
        locals.var_t0_dn10 = assign28410_e27310_d_n10;
        locals.var_t0_dn11 = assign28410_e27310_d_n11;
        locals.var_t0_dn14 = assign28410_e27310_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign28420_e27319, assign28420_e27319_d_n0, assign28420_e27319_d_n2, assign28420_e27319_d_n4, assign28420_e27319_d_n5, assign28420_e27319_d_n6, assign28420_e27319_d_n7, assign28420_e27319_d_n8, assign28420_e27319_d_n9, assign28420_e27319_d_n10, assign28420_e27319_d_n11, assign28420_e27319_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn11, locals.var_w_res0_dn14,)
    }
};
        locals.var_w_res0 = assign28420_e27319;
        locals.var_w_res0_dn0 = assign28420_e27319_d_n0;
        locals.var_w_res0_dn2 = assign28420_e27319_d_n2;
        locals.var_w_res0_dn4 = assign28420_e27319_d_n4;
        locals.var_w_res0_dn5 = assign28420_e27319_d_n5;
        locals.var_w_res0_dn6 = assign28420_e27319_d_n6;
        locals.var_w_res0_dn7 = assign28420_e27319_d_n7;
        locals.var_w_res0_dn8 = assign28420_e27319_d_n8;
        locals.var_w_res0_dn9 = assign28420_e27319_d_n9;
        locals.var_w_res0_dn10 = assign28420_e27319_d_n10;
        locals.var_w_res0_dn11 = assign28420_e27319_d_n11;
        locals.var_w_res0_dn14 = assign28420_e27319_d_n14;
        locals.var_w_res0_rv = 0.0;

        let (assign28430_e27328, assign28430_e27328_d_n0, assign28430_e27328_d_n2, assign28430_e27328_d_n4, assign28430_e27328_d_n5, assign28430_e27328_d_n6, assign28430_e27328_d_n7, assign28430_e27328_d_n8, assign28430_e27328_d_n9, assign28430_e27328_d_n10, assign28430_e27328_d_n11, assign28430_e27328_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28430_e27328;
        locals.var_t0_dn0 = assign28430_e27328_d_n0;
        locals.var_t0_dn2 = assign28430_e27328_d_n2;
        locals.var_t0_dn4 = assign28430_e27328_d_n4;
        locals.var_t0_dn5 = assign28430_e27328_d_n5;
        locals.var_t0_dn6 = assign28430_e27328_d_n6;
        locals.var_t0_dn7 = assign28430_e27328_d_n7;
        locals.var_t0_dn8 = assign28430_e27328_d_n8;
        locals.var_t0_dn9 = assign28430_e27328_d_n9;
        locals.var_t0_dn10 = assign28430_e27328_d_n10;
        locals.var_t0_dn11 = assign28430_e27328_d_n11;
        locals.var_t0_dn14 = assign28430_e27328_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign28440_e27337, assign28440_e27337_d_n0, assign28440_e27337_d_n2, assign28440_e27337_d_n4, assign28440_e27337_d_n5, assign28440_e27337_d_n6, assign28440_e27337_d_n7, assign28440_e27337_d_n8, assign28440_e27337_d_n9, assign28440_e27337_d_n10, assign28440_e27337_d_n11, assign28440_e27337_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign28440_e27333: f64 = (-locals.var_w_res0);
        let assign28440_e27335: f64 = (assign28440_e27333 * locals.var_q_ndepm);
        (assign28440_e27335, (((-locals.var_w_res0_dn0) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn0)), (((-locals.var_w_res0_dn2) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn2)), (((-locals.var_w_res0_dn4) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn4)), (((-locals.var_w_res0_dn5) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn5)), (((-locals.var_w_res0_dn6) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn6)), (((-locals.var_w_res0_dn7) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn7)), (((-locals.var_w_res0_dn8) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn8)), (((-locals.var_w_res0_dn9) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn9)), (((-locals.var_w_res0_dn10) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn10)), (((-locals.var_w_res0_dn11) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn11)), (((-locals.var_w_res0_dn14) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_qn_res0, locals.var_qn_res0_dn0, locals.var_qn_res0_dn2, locals.var_qn_res0_dn4, locals.var_qn_res0_dn5, locals.var_qn_res0_dn6, locals.var_qn_res0_dn7, locals.var_qn_res0_dn8, locals.var_qn_res0_dn9, locals.var_qn_res0_dn10, locals.var_qn_res0_dn11, locals.var_qn_res0_dn14,)
    }
};
        locals.var_qn_res0 = assign28440_e27337;
        locals.var_qn_res0_dn0 = assign28440_e27337_d_n0;
        locals.var_qn_res0_dn2 = assign28440_e27337_d_n2;
        locals.var_qn_res0_dn4 = assign28440_e27337_d_n4;
        locals.var_qn_res0_dn5 = assign28440_e27337_d_n5;
        locals.var_qn_res0_dn6 = assign28440_e27337_d_n6;
        locals.var_qn_res0_dn7 = assign28440_e27337_d_n7;
        locals.var_qn_res0_dn8 = assign28440_e27337_d_n8;
        locals.var_qn_res0_dn9 = assign28440_e27337_d_n9;
        locals.var_qn_res0_dn10 = assign28440_e27337_d_n10;
        locals.var_qn_res0_dn11 = assign28440_e27337_d_n11;
        locals.var_qn_res0_dn14 = assign28440_e27337_d_n14;
        locals.var_qn_res0_rv = 0.0;

        let assign28450_e27344: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard671 = assign28450_e27344;
        locals.var_guard671_rv = 0.0;

        let assign28460_e27348: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28460_e27353: f64 = if ((locals.var_phi_s0_dep > assign28460_e27348) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard672 = assign28460_e27353;
        locals.var_guard672_rv = 0.0;

        let (assign28470_e27367, assign28470_e27367_d_n0, assign28470_e27367_d_n2, assign28470_e27367_d_n4, assign28470_e27367_d_n5, assign28470_e27367_d_n6, assign28470_e27367_d_n7, assign28470_e27367_d_n8, assign28470_e27367_d_n9, assign28470_e27367_d_n10, assign28470_e27367_d_n11, assign28470_e27367_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28470_e27363: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign28470_e27365: f64 = (assign28470_e27363 + 0.8);
        (assign28470_e27365, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28470_e27367;
        locals.var_tmf1_dn0 = assign28470_e27367_d_n0;
        locals.var_tmf1_dn2 = assign28470_e27367_d_n2;
        locals.var_tmf1_dn4 = assign28470_e27367_d_n4;
        locals.var_tmf1_dn5 = assign28470_e27367_d_n5;
        locals.var_tmf1_dn6 = assign28470_e27367_d_n6;
        locals.var_tmf1_dn7 = assign28470_e27367_d_n7;
        locals.var_tmf1_dn8 = assign28470_e27367_d_n8;
        locals.var_tmf1_dn9 = assign28470_e27367_d_n9;
        locals.var_tmf1_dn10 = assign28470_e27367_d_n10;
        locals.var_tmf1_dn11 = assign28470_e27367_d_n11;
        locals.var_tmf1_dn14 = assign28470_e27367_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign28480_e27379, assign28480_e27379_d_n0, assign28480_e27379_d_n2, assign28480_e27379_d_n4, assign28480_e27379_d_n5, assign28480_e27379_d_n6, assign28480_e27379_d_n7, assign28480_e27379_d_n8, assign28480_e27379_d_n9, assign28480_e27379_d_n10, assign28480_e27379_d_n11, assign28480_e27379_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28480_e27377: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28480_e27377, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28480_e27379;
        locals.var_x2_dn0 = assign28480_e27379_d_n0;
        locals.var_x2_dn2 = assign28480_e27379_d_n2;
        locals.var_x2_dn4 = assign28480_e27379_d_n4;
        locals.var_x2_dn5 = assign28480_e27379_d_n5;
        locals.var_x2_dn6 = assign28480_e27379_d_n6;
        locals.var_x2_dn7 = assign28480_e27379_d_n7;
        locals.var_x2_dn8 = assign28480_e27379_d_n8;
        locals.var_x2_dn9 = assign28480_e27379_d_n9;
        locals.var_x2_dn10 = assign28480_e27379_d_n10;
        locals.var_x2_dn11 = assign28480_e27379_d_n11;
        locals.var_x2_dn14 = assign28480_e27379_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign28490_e27391, assign28490_e27391_d_n0, assign28490_e27391_d_n2, assign28490_e27391_d_n4, assign28490_e27391_d_n5, assign28490_e27391_d_n6, assign28490_e27391_d_n7, assign28490_e27391_d_n8, assign28490_e27391_d_n9, assign28490_e27391_d_n10, assign28490_e27391_d_n11, assign28490_e27391_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28490_e27389: f64 = (0.8 * 0.8);
        (assign28490_e27389, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28490_e27391;
        locals.var_xmax2_dn0 = assign28490_e27391_d_n0;
        locals.var_xmax2_dn2 = assign28490_e27391_d_n2;
        locals.var_xmax2_dn4 = assign28490_e27391_d_n4;
        locals.var_xmax2_dn5 = assign28490_e27391_d_n5;
        locals.var_xmax2_dn6 = assign28490_e27391_d_n6;
        locals.var_xmax2_dn7 = assign28490_e27391_d_n7;
        locals.var_xmax2_dn8 = assign28490_e27391_d_n8;
        locals.var_xmax2_dn9 = assign28490_e27391_d_n9;
        locals.var_xmax2_dn10 = assign28490_e27391_d_n10;
        locals.var_xmax2_dn11 = assign28490_e27391_d_n11;
        locals.var_xmax2_dn14 = assign28490_e27391_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign28500_e27401, assign28500_e27401_d_n0, assign28500_e27401_d_n2, assign28500_e27401_d_n4, assign28500_e27401_d_n5, assign28500_e27401_d_n6, assign28500_e27401_d_n7, assign28500_e27401_d_n8, assign28500_e27401_d_n9, assign28500_e27401_d_n10, assign28500_e27401_d_n11, assign28500_e27401_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28500_e27401;
        locals.var_xp_dn0 = assign28500_e27401_d_n0;
        locals.var_xp_dn2 = assign28500_e27401_d_n2;
        locals.var_xp_dn4 = assign28500_e27401_d_n4;
        locals.var_xp_dn5 = assign28500_e27401_d_n5;
        locals.var_xp_dn6 = assign28500_e27401_d_n6;
        locals.var_xp_dn7 = assign28500_e27401_d_n7;
        locals.var_xp_dn8 = assign28500_e27401_d_n8;
        locals.var_xp_dn9 = assign28500_e27401_d_n9;
        locals.var_xp_dn10 = assign28500_e27401_d_n10;
        locals.var_xp_dn11 = assign28500_e27401_d_n11;
        locals.var_xp_dn14 = assign28500_e27401_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28510_e27411, assign28510_e27411_d_n0, assign28510_e27411_d_n2, assign28510_e27411_d_n4, assign28510_e27411_d_n5, assign28510_e27411_d_n6, assign28510_e27411_d_n7, assign28510_e27411_d_n8, assign28510_e27411_d_n9, assign28510_e27411_d_n10, assign28510_e27411_d_n11, assign28510_e27411_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28510_e27411;
        locals.var_xmp_dn0 = assign28510_e27411_d_n0;
        locals.var_xmp_dn2 = assign28510_e27411_d_n2;
        locals.var_xmp_dn4 = assign28510_e27411_d_n4;
        locals.var_xmp_dn5 = assign28510_e27411_d_n5;
        locals.var_xmp_dn6 = assign28510_e27411_d_n6;
        locals.var_xmp_dn7 = assign28510_e27411_d_n7;
        locals.var_xmp_dn8 = assign28510_e27411_d_n8;
        locals.var_xmp_dn9 = assign28510_e27411_d_n9;
        locals.var_xmp_dn10 = assign28510_e27411_d_n10;
        locals.var_xmp_dn11 = assign28510_e27411_d_n11;
        locals.var_xmp_dn14 = assign28510_e27411_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28520_e27421,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28520_e27421;
        locals.var_m0_rv = 0.0;

        let (assign28530_e27431,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28530_e27431;
        locals.var_mm_rv = 0.0;

        let (assign28540_e27441, assign28540_e27441_d_n0, assign28540_e27441_d_n2, assign28540_e27441_d_n4, assign28540_e27441_d_n5, assign28540_e27441_d_n6, assign28540_e27441_d_n7, assign28540_e27441_d_n8, assign28540_e27441_d_n9, assign28540_e27441_d_n10, assign28540_e27441_d_n11, assign28540_e27441_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28540_e27441;
        locals.var_arg_dn0 = assign28540_e27441_d_n0;
        locals.var_arg_dn2 = assign28540_e27441_d_n2;
        locals.var_arg_dn4 = assign28540_e27441_d_n4;
        locals.var_arg_dn5 = assign28540_e27441_d_n5;
        locals.var_arg_dn6 = assign28540_e27441_d_n6;
        locals.var_arg_dn7 = assign28540_e27441_d_n7;
        locals.var_arg_dn8 = assign28540_e27441_d_n8;
        locals.var_arg_dn9 = assign28540_e27441_d_n9;
        locals.var_arg_dn10 = assign28540_e27441_d_n10;
        locals.var_arg_dn11 = assign28540_e27441_d_n11;
        locals.var_arg_dn14 = assign28540_e27441_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign28550_e27451, assign28550_e27451_d_n0, assign28550_e27451_d_n2, assign28550_e27451_d_n4, assign28550_e27451_d_n5, assign28550_e27451_d_n6, assign28550_e27451_d_n7, assign28550_e27451_d_n8, assign28550_e27451_d_n9, assign28550_e27451_d_n10, assign28550_e27451_d_n11, assign28550_e27451_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28550_e27451;
        locals.var_dnm_dn0 = assign28550_e27451_d_n0;
        locals.var_dnm_dn2 = assign28550_e27451_d_n2;
        locals.var_dnm_dn4 = assign28550_e27451_d_n4;
        locals.var_dnm_dn5 = assign28550_e27451_d_n5;
        locals.var_dnm_dn6 = assign28550_e27451_d_n6;
        locals.var_dnm_dn7 = assign28550_e27451_d_n7;
        locals.var_dnm_dn8 = assign28550_e27451_d_n8;
        locals.var_dnm_dn9 = assign28550_e27451_d_n9;
        locals.var_dnm_dn10 = assign28550_e27451_d_n10;
        locals.var_dnm_dn11 = assign28550_e27451_d_n11;
        locals.var_dnm_dn14 = assign28550_e27451_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28560_e27463, assign28560_e27463_d_n0, assign28560_e27463_d_n2, assign28560_e27463_d_n4, assign28560_e27463_d_n5, assign28560_e27463_d_n6, assign28560_e27463_d_n7, assign28560_e27463_d_n8, assign28560_e27463_d_n9, assign28560_e27463_d_n10, assign28560_e27463_d_n11, assign28560_e27463_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28560_e27461: f64 = (locals.var_xp * locals.var_x2);
        (assign28560_e27461, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28560_e27463;
        locals.var_xp_dn0 = assign28560_e27463_d_n0;
        locals.var_xp_dn2 = assign28560_e27463_d_n2;
        locals.var_xp_dn4 = assign28560_e27463_d_n4;
        locals.var_xp_dn5 = assign28560_e27463_d_n5;
        locals.var_xp_dn6 = assign28560_e27463_d_n6;
        locals.var_xp_dn7 = assign28560_e27463_d_n7;
        locals.var_xp_dn8 = assign28560_e27463_d_n8;
        locals.var_xp_dn9 = assign28560_e27463_d_n9;
        locals.var_xp_dn10 = assign28560_e27463_d_n10;
        locals.var_xp_dn11 = assign28560_e27463_d_n11;
        locals.var_xp_dn14 = assign28560_e27463_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28570_e27475, assign28570_e27475_d_n0, assign28570_e27475_d_n2, assign28570_e27475_d_n4, assign28570_e27475_d_n5, assign28570_e27475_d_n6, assign28570_e27475_d_n7, assign28570_e27475_d_n8, assign28570_e27475_d_n9, assign28570_e27475_d_n10, assign28570_e27475_d_n11, assign28570_e27475_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28570_e27473: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28570_e27473, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28570_e27475;
        locals.var_xmp_dn0 = assign28570_e27475_d_n0;
        locals.var_xmp_dn2 = assign28570_e27475_d_n2;
        locals.var_xmp_dn4 = assign28570_e27475_d_n4;
        locals.var_xmp_dn5 = assign28570_e27475_d_n5;
        locals.var_xmp_dn6 = assign28570_e27475_d_n6;
        locals.var_xmp_dn7 = assign28570_e27475_d_n7;
        locals.var_xmp_dn8 = assign28570_e27475_d_n8;
        locals.var_xmp_dn9 = assign28570_e27475_d_n9;
        locals.var_xmp_dn10 = assign28570_e27475_d_n10;
        locals.var_xmp_dn11 = assign28570_e27475_d_n11;
        locals.var_xmp_dn14 = assign28570_e27475_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28580_e27487, assign28580_e27487_d_n0, assign28580_e27487_d_n2, assign28580_e27487_d_n4, assign28580_e27487_d_n5, assign28580_e27487_d_n6, assign28580_e27487_d_n7, assign28580_e27487_d_n8, assign28580_e27487_d_n9, assign28580_e27487_d_n10, assign28580_e27487_d_n11, assign28580_e27487_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28580_e27485: f64 = (locals.var_xp * locals.var_x2);
        (assign28580_e27485, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28580_e27487;
        locals.var_xp_dn0 = assign28580_e27487_d_n0;
        locals.var_xp_dn2 = assign28580_e27487_d_n2;
        locals.var_xp_dn4 = assign28580_e27487_d_n4;
        locals.var_xp_dn5 = assign28580_e27487_d_n5;
        locals.var_xp_dn6 = assign28580_e27487_d_n6;
        locals.var_xp_dn7 = assign28580_e27487_d_n7;
        locals.var_xp_dn8 = assign28580_e27487_d_n8;
        locals.var_xp_dn9 = assign28580_e27487_d_n9;
        locals.var_xp_dn10 = assign28580_e27487_d_n10;
        locals.var_xp_dn11 = assign28580_e27487_d_n11;
        locals.var_xp_dn14 = assign28580_e27487_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28590_e27499, assign28590_e27499_d_n0, assign28590_e27499_d_n2, assign28590_e27499_d_n4, assign28590_e27499_d_n5, assign28590_e27499_d_n6, assign28590_e27499_d_n7, assign28590_e27499_d_n8, assign28590_e27499_d_n9, assign28590_e27499_d_n10, assign28590_e27499_d_n11, assign28590_e27499_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28590_e27497: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28590_e27497, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28590_e27499;
        locals.var_xmp_dn0 = assign28590_e27499_d_n0;
        locals.var_xmp_dn2 = assign28590_e27499_d_n2;
        locals.var_xmp_dn4 = assign28590_e27499_d_n4;
        locals.var_xmp_dn5 = assign28590_e27499_d_n5;
        locals.var_xmp_dn6 = assign28590_e27499_d_n6;
        locals.var_xmp_dn7 = assign28590_e27499_d_n7;
        locals.var_xmp_dn8 = assign28590_e27499_d_n8;
        locals.var_xmp_dn9 = assign28590_e27499_d_n9;
        locals.var_xmp_dn10 = assign28590_e27499_d_n10;
        locals.var_xmp_dn11 = assign28590_e27499_d_n11;
        locals.var_xmp_dn14 = assign28590_e27499_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28600_e27511, assign28600_e27511_d_n0, assign28600_e27511_d_n2, assign28600_e27511_d_n4, assign28600_e27511_d_n5, assign28600_e27511_d_n6, assign28600_e27511_d_n7, assign28600_e27511_d_n8, assign28600_e27511_d_n9, assign28600_e27511_d_n10, assign28600_e27511_d_n11, assign28600_e27511_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28600_e27509: f64 = (locals.var_xp + locals.var_xmp);
        (assign28600_e27509, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28600_e27511;
        locals.var_arg_dn0 = assign28600_e27511_d_n0;
        locals.var_arg_dn2 = assign28600_e27511_d_n2;
        locals.var_arg_dn4 = assign28600_e27511_d_n4;
        locals.var_arg_dn5 = assign28600_e27511_d_n5;
        locals.var_arg_dn6 = assign28600_e27511_d_n6;
        locals.var_arg_dn7 = assign28600_e27511_d_n7;
        locals.var_arg_dn8 = assign28600_e27511_d_n8;
        locals.var_arg_dn9 = assign28600_e27511_d_n9;
        locals.var_arg_dn10 = assign28600_e27511_d_n10;
        locals.var_arg_dn11 = assign28600_e27511_d_n11;
        locals.var_arg_dn14 = assign28600_e27511_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign28610_e27521, assign28610_e27521_d_n0, assign28610_e27521_d_n2, assign28610_e27521_d_n4, assign28610_e27521_d_n5, assign28610_e27521_d_n6, assign28610_e27521_d_n7, assign28610_e27521_d_n8, assign28610_e27521_d_n9, assign28610_e27521_d_n10, assign28610_e27521_d_n11, assign28610_e27521_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28610_e27521;
        locals.var_dnm_dn0 = assign28610_e27521_d_n0;
        locals.var_dnm_dn2 = assign28610_e27521_d_n2;
        locals.var_dnm_dn4 = assign28610_e27521_d_n4;
        locals.var_dnm_dn5 = assign28610_e27521_d_n5;
        locals.var_dnm_dn6 = assign28610_e27521_d_n6;
        locals.var_dnm_dn7 = assign28610_e27521_d_n7;
        locals.var_dnm_dn8 = assign28610_e27521_d_n8;
        locals.var_dnm_dn9 = assign28610_e27521_d_n9;
        locals.var_dnm_dn10 = assign28610_e27521_d_n10;
        locals.var_dnm_dn11 = assign28610_e27521_d_n11;
        locals.var_dnm_dn14 = assign28610_e27521_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign28620_e27536: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard673 = assign28620_e27536;
        locals.var_guard673_rv = 0.0;

        let assign28630_e27539: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign28630_e27539;
        locals.var_guard674_rv = 0.0;

        let (assign28640_e27553,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28640_e27553;
        locals.var_mm_rv = 0.0;

        let assign28650_e27556: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign28650_e27556;
        locals.var_guard675_rv = 0.0;

        let (assign28660_e27573,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28660_e27573;
        locals.var_mm_rv = 0.0;

        let assign28670_e27576: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign28670_e27576;
        locals.var_guard676_rv = 0.0;

        let (assign28680_e27596,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28680_e27596;
        locals.var_mm_rv = 0.0;

        let assign28690_e27599: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign28690_e27599;
        locals.var_guard677_rv = 0.0;

        let (assign28700_e27622,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28700_e27622;
        locals.var_mm_rv = 0.0;

        let (assign28710_e27634,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28710_e27634;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        locals: &mut StampLocals,
    ) {
        let mut assign28720_loop_guard: usize = 0;
        while {
            let assign28720_cond_e27647: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28720_cond_e27647 != 0.0
        } {
            assign28720_loop_guard += 1;
            assert!(assign28720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28720_body0_e27660, assign28720_body0_e27660_d_n0, assign28720_body0_e27660_d_n2, assign28720_body0_e27660_d_n4, assign28720_body0_e27660_d_n5, assign28720_body0_e27660_d_n6, assign28720_body0_e27660_d_n7, assign28720_body0_e27660_d_n8, assign28720_body0_e27660_d_n9, assign28720_body0_e27660_d_n10, assign28720_body0_e27660_d_n11, assign28720_body0_e27660_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        let assign28720_body0_e27658: f64 = (locals.var_dnm).sqrt();
        (assign28720_body0_e27658, (locals.var_dnm_dn0 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn2 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn4 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn5 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn6 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn7 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn8 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn9 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn10 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn11 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn14 / (2.0 * assign28720_body0_e27658)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign28720_body0_e27660;
            locals.var_dnm_dn0 = assign28720_body0_e27660_d_n0;
            locals.var_dnm_dn2 = assign28720_body0_e27660_d_n2;
            locals.var_dnm_dn4 = assign28720_body0_e27660_d_n4;
            locals.var_dnm_dn5 = assign28720_body0_e27660_d_n5;
            locals.var_dnm_dn6 = assign28720_body0_e27660_d_n6;
            locals.var_dnm_dn7 = assign28720_body0_e27660_d_n7;
            locals.var_dnm_dn8 = assign28720_body0_e27660_d_n8;
            locals.var_dnm_dn9 = assign28720_body0_e27660_d_n9;
            locals.var_dnm_dn10 = assign28720_body0_e27660_d_n10;
            locals.var_dnm_dn11 = assign28720_body0_e27660_d_n11;
            locals.var_dnm_dn14 = assign28720_body0_e27660_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign28720_body1_e27674,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        let assign28720_body1_e27672: f64 = (locals.var_m0 + 1.0);
        (assign28720_body1_e27672,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28720_body1_e27674;
            locals.var_m0_rv = 0.0;
        }

        let (assign28730_e27698, assign28730_e27698_d_n0, assign28730_e27698_d_n2, assign28730_e27698_d_n4, assign28730_e27698_d_n5, assign28730_e27698_d_n6, assign28730_e27698_d_n7, assign28730_e27698_d_n8, assign28730_e27698_d_n9, assign28730_e27698_d_n10, assign28730_e27698_d_n11, assign28730_e27698_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 == 0.0)) {
        let (assign28730_e27696, assign28730_e27696_d_n0, assign28730_e27696_d_n2, assign28730_e27696_d_n4, assign28730_e27696_d_n5, assign28730_e27696_d_n6, assign28730_e27696_d_n7, assign28730_e27696_d_n8, assign28730_e27696_d_n9, assign28730_e27696_d_n10, assign28730_e27696_d_n11, assign28730_e27696_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28730_e27693: f64 = (2.0 * 2.0);
                let assign28730_e27694: f64 = (1.0 / assign28730_e27693);
                let assign28730_e27695: f64 = (locals.var_dnm).powf(assign28730_e27694);
                (assign28730_e27695, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn11)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn14)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign28730_e27696, assign28730_e27696_d_n0, assign28730_e27696_d_n2, assign28730_e27696_d_n4, assign28730_e27696_d_n5, assign28730_e27696_d_n6, assign28730_e27696_d_n7, assign28730_e27696_d_n8, assign28730_e27696_d_n9, assign28730_e27696_d_n10, assign28730_e27696_d_n11, assign28730_e27696_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28730_e27698;
        locals.var_dnm_dn0 = assign28730_e27698_d_n0;
        locals.var_dnm_dn2 = assign28730_e27698_d_n2;
        locals.var_dnm_dn4 = assign28730_e27698_d_n4;
        locals.var_dnm_dn5 = assign28730_e27698_d_n5;
        locals.var_dnm_dn6 = assign28730_e27698_d_n6;
        locals.var_dnm_dn7 = assign28730_e27698_d_n7;
        locals.var_dnm_dn8 = assign28730_e27698_d_n8;
        locals.var_dnm_dn9 = assign28730_e27698_d_n9;
        locals.var_dnm_dn10 = assign28730_e27698_d_n10;
        locals.var_dnm_dn11 = assign28730_e27698_d_n11;
        locals.var_dnm_dn14 = assign28730_e27698_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28740_e27710, assign28740_e27710_d_n0, assign28740_e27710_d_n2, assign28740_e27710_d_n4, assign28740_e27710_d_n5, assign28740_e27710_d_n6, assign28740_e27710_d_n7, assign28740_e27710_d_n8, assign28740_e27710_d_n9, assign28740_e27710_d_n10, assign28740_e27710_d_n11, assign28740_e27710_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28740_e27708: f64 = (1.0 / locals.var_dnm);
        (assign28740_e27708, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28740_e27710;
        locals.var_dnm_dn0 = assign28740_e27710_d_n0;
        locals.var_dnm_dn2 = assign28740_e27710_d_n2;
        locals.var_dnm_dn4 = assign28740_e27710_d_n4;
        locals.var_dnm_dn5 = assign28740_e27710_d_n5;
        locals.var_dnm_dn6 = assign28740_e27710_d_n6;
        locals.var_dnm_dn7 = assign28740_e27710_d_n7;
        locals.var_dnm_dn8 = assign28740_e27710_d_n8;
        locals.var_dnm_dn9 = assign28740_e27710_d_n9;
        locals.var_dnm_dn10 = assign28740_e27710_d_n10;
        locals.var_dnm_dn11 = assign28740_e27710_d_n11;
        locals.var_dnm_dn14 = assign28740_e27710_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28750_e27724, assign28750_e27724_d_n0, assign28750_e27724_d_n2, assign28750_e27724_d_n4, assign28750_e27724_d_n5, assign28750_e27724_d_n6, assign28750_e27724_d_n7, assign28750_e27724_d_n8, assign28750_e27724_d_n9, assign28750_e27724_d_n10, assign28750_e27724_d_n11, assign28750_e27724_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28750_e27720: f64 = (locals.var_tmf1 * 0.8);
        let assign28750_e27722: f64 = (assign28750_e27720 * locals.var_dnm);
        (assign28750_e27722, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28750_e27724;
        locals.var_tmf0_dn0 = assign28750_e27724_d_n0;
        locals.var_tmf0_dn2 = assign28750_e27724_d_n2;
        locals.var_tmf0_dn4 = assign28750_e27724_d_n4;
        locals.var_tmf0_dn5 = assign28750_e27724_d_n5;
        locals.var_tmf0_dn6 = assign28750_e27724_d_n6;
        locals.var_tmf0_dn7 = assign28750_e27724_d_n7;
        locals.var_tmf0_dn8 = assign28750_e27724_d_n8;
        locals.var_tmf0_dn9 = assign28750_e27724_d_n9;
        locals.var_tmf0_dn10 = assign28750_e27724_d_n10;
        locals.var_tmf0_dn11 = assign28750_e27724_d_n11;
        locals.var_tmf0_dn14 = assign28750_e27724_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign28760_e27740, assign28760_e27740_d_n0, assign28760_e27740_d_n2, assign28760_e27740_d_n4, assign28760_e27740_d_n5, assign28760_e27740_d_n6, assign28760_e27740_d_n7, assign28760_e27740_d_n8, assign28760_e27740_d_n9, assign28760_e27740_d_n10, assign28760_e27740_d_n11, assign28760_e27740_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28760_e27734: f64 = (0.8 * locals.var_xmp);
        let assign28760_e27736: f64 = (assign28760_e27734 * locals.var_dnm);
        let assign28760_e27738: f64 = (assign28760_e27736 / locals.var_arg);
        (assign28760_e27738, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28760_e27740;
        locals.var_t1_dn0 = assign28760_e27740_d_n0;
        locals.var_t1_dn2 = assign28760_e27740_d_n2;
        locals.var_t1_dn4 = assign28760_e27740_d_n4;
        locals.var_t1_dn5 = assign28760_e27740_d_n5;
        locals.var_t1_dn6 = assign28760_e27740_d_n6;
        locals.var_t1_dn7 = assign28760_e27740_d_n7;
        locals.var_t1_dn8 = assign28760_e27740_d_n8;
        locals.var_t1_dn9 = assign28760_e27740_d_n9;
        locals.var_t1_dn10 = assign28760_e27740_d_n10;
        locals.var_t1_dn11 = assign28760_e27740_d_n11;
        locals.var_t1_dn14 = assign28760_e27740_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28770_e27754, assign28770_e27754_d_n0, assign28770_e27754_d_n2, assign28770_e27754_d_n4, assign28770_e27754_d_n5, assign28770_e27754_d_n6, assign28770_e27754_d_n7, assign28770_e27754_d_n8, assign28770_e27754_d_n9, assign28770_e27754_d_n10, assign28770_e27754_d_n11, assign28770_e27754_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28770_e27750: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28770_e27752: f64 = (assign28770_e27750 + locals.var_tmf0);
        (assign28770_e27752, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn11 + locals.var_tmf0_dn11), (locals.var_vds_maxb0_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28770_e27754;
        locals.var_t2_dn0 = assign28770_e27754_d_n0;
        locals.var_t2_dn2 = assign28770_e27754_d_n2;
        locals.var_t2_dn4 = assign28770_e27754_d_n4;
        locals.var_t2_dn5 = assign28770_e27754_d_n5;
        locals.var_t2_dn6 = assign28770_e27754_d_n6;
        locals.var_t2_dn7 = assign28770_e27754_d_n7;
        locals.var_t2_dn8 = assign28770_e27754_d_n8;
        locals.var_t2_dn9 = assign28770_e27754_d_n9;
        locals.var_t2_dn10 = assign28770_e27754_d_n10;
        locals.var_t2_dn11 = assign28770_e27754_d_n11;
        locals.var_t2_dn14 = assign28770_e27754_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28780_e27764, assign28780_e27764_d_n0, assign28780_e27764_d_n2, assign28780_e27764_d_n4, assign28780_e27764_d_n5, assign28780_e27764_d_n6, assign28780_e27764_d_n7, assign28780_e27764_d_n8, assign28780_e27764_d_n9, assign28780_e27764_d_n10, assign28780_e27764_d_n11, assign28780_e27764_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28780_e27764;
        locals.var_t1_dn0 = assign28780_e27764_d_n0;
        locals.var_t1_dn2 = assign28780_e27764_d_n2;
        locals.var_t1_dn4 = assign28780_e27764_d_n4;
        locals.var_t1_dn5 = assign28780_e27764_d_n5;
        locals.var_t1_dn6 = assign28780_e27764_d_n6;
        locals.var_t1_dn7 = assign28780_e27764_d_n7;
        locals.var_t1_dn8 = assign28780_e27764_d_n8;
        locals.var_t1_dn9 = assign28780_e27764_d_n9;
        locals.var_t1_dn10 = assign28780_e27764_d_n10;
        locals.var_t1_dn11 = assign28780_e27764_d_n11;
        locals.var_t1_dn14 = assign28780_e27764_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28790_e27775, assign28790_e27775_d_n0, assign28790_e27775_d_n2, assign28790_e27775_d_n4, assign28790_e27775_d_n5, assign28790_e27775_d_n6, assign28790_e27775_d_n7, assign28790_e27775_d_n8, assign28790_e27775_d_n9, assign28790_e27775_d_n10, assign28790_e27775_d_n11, assign28790_e27775_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28790_e27775;
        locals.var_t2_dn0 = assign28790_e27775_d_n0;
        locals.var_t2_dn2 = assign28790_e27775_d_n2;
        locals.var_t2_dn4 = assign28790_e27775_d_n4;
        locals.var_t2_dn5 = assign28790_e27775_d_n5;
        locals.var_t2_dn6 = assign28790_e27775_d_n6;
        locals.var_t2_dn7 = assign28790_e27775_d_n7;
        locals.var_t2_dn8 = assign28790_e27775_d_n8;
        locals.var_t2_dn9 = assign28790_e27775_d_n9;
        locals.var_t2_dn10 = assign28790_e27775_d_n10;
        locals.var_t2_dn11 = assign28790_e27775_d_n11;
        locals.var_t2_dn14 = assign28790_e27775_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28800_e27786, assign28800_e27786_d_n0, assign28800_e27786_d_n2, assign28800_e27786_d_n4, assign28800_e27786_d_n5, assign28800_e27786_d_n6, assign28800_e27786_d_n7, assign28800_e27786_d_n8, assign28800_e27786_d_n9, assign28800_e27786_d_n10, assign28800_e27786_d_n11, assign28800_e27786_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28800_e27786;
        locals.var_t1_dn0 = assign28800_e27786_d_n0;
        locals.var_t1_dn2 = assign28800_e27786_d_n2;
        locals.var_t1_dn4 = assign28800_e27786_d_n4;
        locals.var_t1_dn5 = assign28800_e27786_d_n5;
        locals.var_t1_dn6 = assign28800_e27786_d_n6;
        locals.var_t1_dn7 = assign28800_e27786_d_n7;
        locals.var_t1_dn8 = assign28800_e27786_d_n8;
        locals.var_t1_dn9 = assign28800_e27786_d_n9;
        locals.var_t1_dn10 = assign28800_e27786_d_n10;
        locals.var_t1_dn11 = assign28800_e27786_d_n11;
        locals.var_t1_dn14 = assign28800_e27786_d_n14;
        locals.var_t1_rv = 0.0;

        let assign28810_e27790: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28810_e27795: f64 = if ((locals.var_phib_ref > assign28810_e27790) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard678 = assign28810_e27795;
        locals.var_guard678_rv = 0.0;

        let (assign28820_e27810, assign28820_e27810_d_n0, assign28820_e27810_d_n2, assign28820_e27810_d_n4, assign28820_e27810_d_n5, assign28820_e27810_d_n6, assign28820_e27810_d_n7, assign28820_e27810_d_n8, assign28820_e27810_d_n9, assign28820_e27810_d_n10, assign28820_e27810_d_n11, assign28820_e27810_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28820_e27806: f64 = (locals.var_phib_ref - locals.var_vds_maxb0);
        let assign28820_e27808: f64 = (assign28820_e27806 + 0.8);
        (assign28820_e27808, (locals.var_phib_ref_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phib_ref_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phib_ref_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phib_ref_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phib_ref_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phib_ref_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phib_ref_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phib_ref_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phib_ref_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phib_ref_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phib_ref_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28820_e27810;
        locals.var_tmf1_dn0 = assign28820_e27810_d_n0;
        locals.var_tmf1_dn2 = assign28820_e27810_d_n2;
        locals.var_tmf1_dn4 = assign28820_e27810_d_n4;
        locals.var_tmf1_dn5 = assign28820_e27810_d_n5;
        locals.var_tmf1_dn6 = assign28820_e27810_d_n6;
        locals.var_tmf1_dn7 = assign28820_e27810_d_n7;
        locals.var_tmf1_dn8 = assign28820_e27810_d_n8;
        locals.var_tmf1_dn9 = assign28820_e27810_d_n9;
        locals.var_tmf1_dn10 = assign28820_e27810_d_n10;
        locals.var_tmf1_dn11 = assign28820_e27810_d_n11;
        locals.var_tmf1_dn14 = assign28820_e27810_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign28830_e27823, assign28830_e27823_d_n0, assign28830_e27823_d_n2, assign28830_e27823_d_n4, assign28830_e27823_d_n5, assign28830_e27823_d_n6, assign28830_e27823_d_n7, assign28830_e27823_d_n8, assign28830_e27823_d_n9, assign28830_e27823_d_n10, assign28830_e27823_d_n11, assign28830_e27823_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28830_e27821: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28830_e27821, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28830_e27823;
        locals.var_x2_dn0 = assign28830_e27823_d_n0;
        locals.var_x2_dn2 = assign28830_e27823_d_n2;
        locals.var_x2_dn4 = assign28830_e27823_d_n4;
        locals.var_x2_dn5 = assign28830_e27823_d_n5;
        locals.var_x2_dn6 = assign28830_e27823_d_n6;
        locals.var_x2_dn7 = assign28830_e27823_d_n7;
        locals.var_x2_dn8 = assign28830_e27823_d_n8;
        locals.var_x2_dn9 = assign28830_e27823_d_n9;
        locals.var_x2_dn10 = assign28830_e27823_d_n10;
        locals.var_x2_dn11 = assign28830_e27823_d_n11;
        locals.var_x2_dn14 = assign28830_e27823_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign28840_e27836, assign28840_e27836_d_n0, assign28840_e27836_d_n2, assign28840_e27836_d_n4, assign28840_e27836_d_n5, assign28840_e27836_d_n6, assign28840_e27836_d_n7, assign28840_e27836_d_n8, assign28840_e27836_d_n9, assign28840_e27836_d_n10, assign28840_e27836_d_n11, assign28840_e27836_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28840_e27834: f64 = (0.8 * 0.8);
        (assign28840_e27834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28840_e27836;
        locals.var_xmax2_dn0 = assign28840_e27836_d_n0;
        locals.var_xmax2_dn2 = assign28840_e27836_d_n2;
        locals.var_xmax2_dn4 = assign28840_e27836_d_n4;
        locals.var_xmax2_dn5 = assign28840_e27836_d_n5;
        locals.var_xmax2_dn6 = assign28840_e27836_d_n6;
        locals.var_xmax2_dn7 = assign28840_e27836_d_n7;
        locals.var_xmax2_dn8 = assign28840_e27836_d_n8;
        locals.var_xmax2_dn9 = assign28840_e27836_d_n9;
        locals.var_xmax2_dn10 = assign28840_e27836_d_n10;
        locals.var_xmax2_dn11 = assign28840_e27836_d_n11;
        locals.var_xmax2_dn14 = assign28840_e27836_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign28850_e27847, assign28850_e27847_d_n0, assign28850_e27847_d_n2, assign28850_e27847_d_n4, assign28850_e27847_d_n5, assign28850_e27847_d_n6, assign28850_e27847_d_n7, assign28850_e27847_d_n8, assign28850_e27847_d_n9, assign28850_e27847_d_n10, assign28850_e27847_d_n11, assign28850_e27847_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28850_e27847;
        locals.var_xp_dn0 = assign28850_e27847_d_n0;
        locals.var_xp_dn2 = assign28850_e27847_d_n2;
        locals.var_xp_dn4 = assign28850_e27847_d_n4;
        locals.var_xp_dn5 = assign28850_e27847_d_n5;
        locals.var_xp_dn6 = assign28850_e27847_d_n6;
        locals.var_xp_dn7 = assign28850_e27847_d_n7;
        locals.var_xp_dn8 = assign28850_e27847_d_n8;
        locals.var_xp_dn9 = assign28850_e27847_d_n9;
        locals.var_xp_dn10 = assign28850_e27847_d_n10;
        locals.var_xp_dn11 = assign28850_e27847_d_n11;
        locals.var_xp_dn14 = assign28850_e27847_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28860_e27858, assign28860_e27858_d_n0, assign28860_e27858_d_n2, assign28860_e27858_d_n4, assign28860_e27858_d_n5, assign28860_e27858_d_n6, assign28860_e27858_d_n7, assign28860_e27858_d_n8, assign28860_e27858_d_n9, assign28860_e27858_d_n10, assign28860_e27858_d_n11, assign28860_e27858_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28860_e27858;
        locals.var_xmp_dn0 = assign28860_e27858_d_n0;
        locals.var_xmp_dn2 = assign28860_e27858_d_n2;
        locals.var_xmp_dn4 = assign28860_e27858_d_n4;
        locals.var_xmp_dn5 = assign28860_e27858_d_n5;
        locals.var_xmp_dn6 = assign28860_e27858_d_n6;
        locals.var_xmp_dn7 = assign28860_e27858_d_n7;
        locals.var_xmp_dn8 = assign28860_e27858_d_n8;
        locals.var_xmp_dn9 = assign28860_e27858_d_n9;
        locals.var_xmp_dn10 = assign28860_e27858_d_n10;
        locals.var_xmp_dn11 = assign28860_e27858_d_n11;
        locals.var_xmp_dn14 = assign28860_e27858_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28870_e27869,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28870_e27869;
        locals.var_m0_rv = 0.0;

        let (assign28880_e27880,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28880_e27880;
        locals.var_mm_rv = 0.0;

        let (assign28890_e27891, assign28890_e27891_d_n0, assign28890_e27891_d_n2, assign28890_e27891_d_n4, assign28890_e27891_d_n5, assign28890_e27891_d_n6, assign28890_e27891_d_n7, assign28890_e27891_d_n8, assign28890_e27891_d_n9, assign28890_e27891_d_n10, assign28890_e27891_d_n11, assign28890_e27891_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28890_e27891;
        locals.var_arg_dn0 = assign28890_e27891_d_n0;
        locals.var_arg_dn2 = assign28890_e27891_d_n2;
        locals.var_arg_dn4 = assign28890_e27891_d_n4;
        locals.var_arg_dn5 = assign28890_e27891_d_n5;
        locals.var_arg_dn6 = assign28890_e27891_d_n6;
        locals.var_arg_dn7 = assign28890_e27891_d_n7;
        locals.var_arg_dn8 = assign28890_e27891_d_n8;
        locals.var_arg_dn9 = assign28890_e27891_d_n9;
        locals.var_arg_dn10 = assign28890_e27891_d_n10;
        locals.var_arg_dn11 = assign28890_e27891_d_n11;
        locals.var_arg_dn14 = assign28890_e27891_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign28900_e27902, assign28900_e27902_d_n0, assign28900_e27902_d_n2, assign28900_e27902_d_n4, assign28900_e27902_d_n5, assign28900_e27902_d_n6, assign28900_e27902_d_n7, assign28900_e27902_d_n8, assign28900_e27902_d_n9, assign28900_e27902_d_n10, assign28900_e27902_d_n11, assign28900_e27902_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28900_e27902;
        locals.var_dnm_dn0 = assign28900_e27902_d_n0;
        locals.var_dnm_dn2 = assign28900_e27902_d_n2;
        locals.var_dnm_dn4 = assign28900_e27902_d_n4;
        locals.var_dnm_dn5 = assign28900_e27902_d_n5;
        locals.var_dnm_dn6 = assign28900_e27902_d_n6;
        locals.var_dnm_dn7 = assign28900_e27902_d_n7;
        locals.var_dnm_dn8 = assign28900_e27902_d_n8;
        locals.var_dnm_dn9 = assign28900_e27902_d_n9;
        locals.var_dnm_dn10 = assign28900_e27902_d_n10;
        locals.var_dnm_dn11 = assign28900_e27902_d_n11;
        locals.var_dnm_dn14 = assign28900_e27902_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign28910_e27915, assign28910_e27915_d_n0, assign28910_e27915_d_n2, assign28910_e27915_d_n4, assign28910_e27915_d_n5, assign28910_e27915_d_n6, assign28910_e27915_d_n7, assign28910_e27915_d_n8, assign28910_e27915_d_n9, assign28910_e27915_d_n10, assign28910_e27915_d_n11, assign28910_e27915_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28910_e27913: f64 = (locals.var_xp * locals.var_x2);
        (assign28910_e27913, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28910_e27915;
        locals.var_xp_dn0 = assign28910_e27915_d_n0;
        locals.var_xp_dn2 = assign28910_e27915_d_n2;
        locals.var_xp_dn4 = assign28910_e27915_d_n4;
        locals.var_xp_dn5 = assign28910_e27915_d_n5;
        locals.var_xp_dn6 = assign28910_e27915_d_n6;
        locals.var_xp_dn7 = assign28910_e27915_d_n7;
        locals.var_xp_dn8 = assign28910_e27915_d_n8;
        locals.var_xp_dn9 = assign28910_e27915_d_n9;
        locals.var_xp_dn10 = assign28910_e27915_d_n10;
        locals.var_xp_dn11 = assign28910_e27915_d_n11;
        locals.var_xp_dn14 = assign28910_e27915_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28920_e27928, assign28920_e27928_d_n0, assign28920_e27928_d_n2, assign28920_e27928_d_n4, assign28920_e27928_d_n5, assign28920_e27928_d_n6, assign28920_e27928_d_n7, assign28920_e27928_d_n8, assign28920_e27928_d_n9, assign28920_e27928_d_n10, assign28920_e27928_d_n11, assign28920_e27928_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28920_e27926: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28920_e27926, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28920_e27928;
        locals.var_xmp_dn0 = assign28920_e27928_d_n0;
        locals.var_xmp_dn2 = assign28920_e27928_d_n2;
        locals.var_xmp_dn4 = assign28920_e27928_d_n4;
        locals.var_xmp_dn5 = assign28920_e27928_d_n5;
        locals.var_xmp_dn6 = assign28920_e27928_d_n6;
        locals.var_xmp_dn7 = assign28920_e27928_d_n7;
        locals.var_xmp_dn8 = assign28920_e27928_d_n8;
        locals.var_xmp_dn9 = assign28920_e27928_d_n9;
        locals.var_xmp_dn10 = assign28920_e27928_d_n10;
        locals.var_xmp_dn11 = assign28920_e27928_d_n11;
        locals.var_xmp_dn14 = assign28920_e27928_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28930_e27941, assign28930_e27941_d_n0, assign28930_e27941_d_n2, assign28930_e27941_d_n4, assign28930_e27941_d_n5, assign28930_e27941_d_n6, assign28930_e27941_d_n7, assign28930_e27941_d_n8, assign28930_e27941_d_n9, assign28930_e27941_d_n10, assign28930_e27941_d_n11, assign28930_e27941_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28930_e27939: f64 = (locals.var_xp * locals.var_x2);
        (assign28930_e27939, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28930_e27941;
        locals.var_xp_dn0 = assign28930_e27941_d_n0;
        locals.var_xp_dn2 = assign28930_e27941_d_n2;
        locals.var_xp_dn4 = assign28930_e27941_d_n4;
        locals.var_xp_dn5 = assign28930_e27941_d_n5;
        locals.var_xp_dn6 = assign28930_e27941_d_n6;
        locals.var_xp_dn7 = assign28930_e27941_d_n7;
        locals.var_xp_dn8 = assign28930_e27941_d_n8;
        locals.var_xp_dn9 = assign28930_e27941_d_n9;
        locals.var_xp_dn10 = assign28930_e27941_d_n10;
        locals.var_xp_dn11 = assign28930_e27941_d_n11;
        locals.var_xp_dn14 = assign28930_e27941_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign28940_e27954, assign28940_e27954_d_n0, assign28940_e27954_d_n2, assign28940_e27954_d_n4, assign28940_e27954_d_n5, assign28940_e27954_d_n6, assign28940_e27954_d_n7, assign28940_e27954_d_n8, assign28940_e27954_d_n9, assign28940_e27954_d_n10, assign28940_e27954_d_n11, assign28940_e27954_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28940_e27952: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28940_e27952, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28940_e27954;
        locals.var_xmp_dn0 = assign28940_e27954_d_n0;
        locals.var_xmp_dn2 = assign28940_e27954_d_n2;
        locals.var_xmp_dn4 = assign28940_e27954_d_n4;
        locals.var_xmp_dn5 = assign28940_e27954_d_n5;
        locals.var_xmp_dn6 = assign28940_e27954_d_n6;
        locals.var_xmp_dn7 = assign28940_e27954_d_n7;
        locals.var_xmp_dn8 = assign28940_e27954_d_n8;
        locals.var_xmp_dn9 = assign28940_e27954_d_n9;
        locals.var_xmp_dn10 = assign28940_e27954_d_n10;
        locals.var_xmp_dn11 = assign28940_e27954_d_n11;
        locals.var_xmp_dn14 = assign28940_e27954_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign28950_e27967, assign28950_e27967_d_n0, assign28950_e27967_d_n2, assign28950_e27967_d_n4, assign28950_e27967_d_n5, assign28950_e27967_d_n6, assign28950_e27967_d_n7, assign28950_e27967_d_n8, assign28950_e27967_d_n9, assign28950_e27967_d_n10, assign28950_e27967_d_n11, assign28950_e27967_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28950_e27965: f64 = (locals.var_xp + locals.var_xmp);
        (assign28950_e27965, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28950_e27967;
        locals.var_arg_dn0 = assign28950_e27967_d_n0;
        locals.var_arg_dn2 = assign28950_e27967_d_n2;
        locals.var_arg_dn4 = assign28950_e27967_d_n4;
        locals.var_arg_dn5 = assign28950_e27967_d_n5;
        locals.var_arg_dn6 = assign28950_e27967_d_n6;
        locals.var_arg_dn7 = assign28950_e27967_d_n7;
        locals.var_arg_dn8 = assign28950_e27967_d_n8;
        locals.var_arg_dn9 = assign28950_e27967_d_n9;
        locals.var_arg_dn10 = assign28950_e27967_d_n10;
        locals.var_arg_dn11 = assign28950_e27967_d_n11;
        locals.var_arg_dn14 = assign28950_e27967_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        locals: &mut StampLocals,
    ) {
        let (assign28960_e27978, assign28960_e27978_d_n0, assign28960_e27978_d_n2, assign28960_e27978_d_n4, assign28960_e27978_d_n5, assign28960_e27978_d_n6, assign28960_e27978_d_n7, assign28960_e27978_d_n8, assign28960_e27978_d_n9, assign28960_e27978_d_n10, assign28960_e27978_d_n11, assign28960_e27978_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28960_e27978;
        locals.var_dnm_dn0 = assign28960_e27978_d_n0;
        locals.var_dnm_dn2 = assign28960_e27978_d_n2;
        locals.var_dnm_dn4 = assign28960_e27978_d_n4;
        locals.var_dnm_dn5 = assign28960_e27978_d_n5;
        locals.var_dnm_dn6 = assign28960_e27978_d_n6;
        locals.var_dnm_dn7 = assign28960_e27978_d_n7;
        locals.var_dnm_dn8 = assign28960_e27978_d_n8;
        locals.var_dnm_dn9 = assign28960_e27978_d_n9;
        locals.var_dnm_dn10 = assign28960_e27978_d_n10;
        locals.var_dnm_dn11 = assign28960_e27978_d_n11;
        locals.var_dnm_dn14 = assign28960_e27978_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign28970_e27993: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard679 = assign28970_e27993;
        locals.var_guard679_rv = 0.0;

        let assign28980_e27996: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign28980_e27996;
        locals.var_guard680_rv = 0.0;

        let (assign28990_e28011,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28990_e28011;
        locals.var_mm_rv = 0.0;

        let assign29000_e28014: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign29000_e28014;
        locals.var_guard681_rv = 0.0;

        let (assign29010_e28032,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29010_e28032;
        locals.var_mm_rv = 0.0;

        let assign29020_e28035: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign29020_e28035;
        locals.var_guard682_rv = 0.0;

        let (assign29030_e28056,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29030_e28056;
        locals.var_mm_rv = 0.0;

        let assign29040_e28059: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign29040_e28059;
        locals.var_guard683_rv = 0.0;

        let (assign29050_e28083,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29050_e28083;
        locals.var_mm_rv = 0.0;

        let (assign29060_e28096,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29060_e28096;
        locals.var_m0_rv = 0.0;

        let mut assign29070_loop_guard: usize = 0;
        while {
            let assign29070_cond_e28110: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29070_cond_e28110 != 0.0
        } {
            assign29070_loop_guard += 1;
            assert!(assign29070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29070_body0_e28124, assign29070_body0_e28124_d_n0, assign29070_body0_e28124_d_n2, assign29070_body0_e28124_d_n4, assign29070_body0_e28124_d_n5, assign29070_body0_e28124_d_n6, assign29070_body0_e28124_d_n7, assign29070_body0_e28124_d_n8, assign29070_body0_e28124_d_n9, assign29070_body0_e28124_d_n10, assign29070_body0_e28124_d_n11, assign29070_body0_e28124_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        let assign29070_body0_e28122: f64 = (locals.var_dnm).sqrt();
        (assign29070_body0_e28122, (locals.var_dnm_dn0 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn2 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn4 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn5 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn6 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn7 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn8 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn9 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn10 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn11 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn14 / (2.0 * assign29070_body0_e28122)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29070_body0_e28124;
            locals.var_dnm_dn0 = assign29070_body0_e28124_d_n0;
            locals.var_dnm_dn2 = assign29070_body0_e28124_d_n2;
            locals.var_dnm_dn4 = assign29070_body0_e28124_d_n4;
            locals.var_dnm_dn5 = assign29070_body0_e28124_d_n5;
            locals.var_dnm_dn6 = assign29070_body0_e28124_d_n6;
            locals.var_dnm_dn7 = assign29070_body0_e28124_d_n7;
            locals.var_dnm_dn8 = assign29070_body0_e28124_d_n8;
            locals.var_dnm_dn9 = assign29070_body0_e28124_d_n9;
            locals.var_dnm_dn10 = assign29070_body0_e28124_d_n10;
            locals.var_dnm_dn11 = assign29070_body0_e28124_d_n11;
            locals.var_dnm_dn14 = assign29070_body0_e28124_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign29070_body1_e28139,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        let assign29070_body1_e28137: f64 = (locals.var_m0 + 1.0);
        (assign29070_body1_e28137,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29070_body1_e28139;
            locals.var_m0_rv = 0.0;
        }

        let (assign29080_e28164, assign29080_e28164_d_n0, assign29080_e28164_d_n2, assign29080_e28164_d_n4, assign29080_e28164_d_n5, assign29080_e28164_d_n6, assign29080_e28164_d_n7, assign29080_e28164_d_n8, assign29080_e28164_d_n9, assign29080_e28164_d_n10, assign29080_e28164_d_n11, assign29080_e28164_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 == 0.0)) {
        let (assign29080_e28162, assign29080_e28162_d_n0, assign29080_e28162_d_n2, assign29080_e28162_d_n4, assign29080_e28162_d_n5, assign29080_e28162_d_n6, assign29080_e28162_d_n7, assign29080_e28162_d_n8, assign29080_e28162_d_n9, assign29080_e28162_d_n10, assign29080_e28162_d_n11, assign29080_e28162_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29080_e28159: f64 = (2.0 * 2.0);
                let assign29080_e28160: f64 = (1.0 / assign29080_e28159);
                let assign29080_e28161: f64 = (locals.var_dnm).powf(assign29080_e28160);
                (assign29080_e28161, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29080_e28162, assign29080_e28162_d_n0, assign29080_e28162_d_n2, assign29080_e28162_d_n4, assign29080_e28162_d_n5, assign29080_e28162_d_n6, assign29080_e28162_d_n7, assign29080_e28162_d_n8, assign29080_e28162_d_n9, assign29080_e28162_d_n10, assign29080_e28162_d_n11, assign29080_e28162_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29080_e28164;
        locals.var_dnm_dn0 = assign29080_e28164_d_n0;
        locals.var_dnm_dn2 = assign29080_e28164_d_n2;
        locals.var_dnm_dn4 = assign29080_e28164_d_n4;
        locals.var_dnm_dn5 = assign29080_e28164_d_n5;
        locals.var_dnm_dn6 = assign29080_e28164_d_n6;
        locals.var_dnm_dn7 = assign29080_e28164_d_n7;
        locals.var_dnm_dn8 = assign29080_e28164_d_n8;
        locals.var_dnm_dn9 = assign29080_e28164_d_n9;
        locals.var_dnm_dn10 = assign29080_e28164_d_n10;
        locals.var_dnm_dn11 = assign29080_e28164_d_n11;
        locals.var_dnm_dn14 = assign29080_e28164_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign29090_e28177, assign29090_e28177_d_n0, assign29090_e28177_d_n2, assign29090_e28177_d_n4, assign29090_e28177_d_n5, assign29090_e28177_d_n6, assign29090_e28177_d_n7, assign29090_e28177_d_n8, assign29090_e28177_d_n9, assign29090_e28177_d_n10, assign29090_e28177_d_n11, assign29090_e28177_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29090_e28175: f64 = (1.0 / locals.var_dnm);
        (assign29090_e28175, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29090_e28177;
        locals.var_dnm_dn0 = assign29090_e28177_d_n0;
        locals.var_dnm_dn2 = assign29090_e28177_d_n2;
        locals.var_dnm_dn4 = assign29090_e28177_d_n4;
        locals.var_dnm_dn5 = assign29090_e28177_d_n5;
        locals.var_dnm_dn6 = assign29090_e28177_d_n6;
        locals.var_dnm_dn7 = assign29090_e28177_d_n7;
        locals.var_dnm_dn8 = assign29090_e28177_d_n8;
        locals.var_dnm_dn9 = assign29090_e28177_d_n9;
        locals.var_dnm_dn10 = assign29090_e28177_d_n10;
        locals.var_dnm_dn11 = assign29090_e28177_d_n11;
        locals.var_dnm_dn14 = assign29090_e28177_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign29100_e28192, assign29100_e28192_d_n0, assign29100_e28192_d_n2, assign29100_e28192_d_n4, assign29100_e28192_d_n5, assign29100_e28192_d_n6, assign29100_e28192_d_n7, assign29100_e28192_d_n8, assign29100_e28192_d_n9, assign29100_e28192_d_n10, assign29100_e28192_d_n11, assign29100_e28192_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29100_e28188: f64 = (locals.var_tmf1 * 0.8);
        let assign29100_e28190: f64 = (assign29100_e28188 * locals.var_dnm);
        (assign29100_e28190, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29100_e28192;
        locals.var_tmf0_dn0 = assign29100_e28192_d_n0;
        locals.var_tmf0_dn2 = assign29100_e28192_d_n2;
        locals.var_tmf0_dn4 = assign29100_e28192_d_n4;
        locals.var_tmf0_dn5 = assign29100_e28192_d_n5;
        locals.var_tmf0_dn6 = assign29100_e28192_d_n6;
        locals.var_tmf0_dn7 = assign29100_e28192_d_n7;
        locals.var_tmf0_dn8 = assign29100_e28192_d_n8;
        locals.var_tmf0_dn9 = assign29100_e28192_d_n9;
        locals.var_tmf0_dn10 = assign29100_e28192_d_n10;
        locals.var_tmf0_dn11 = assign29100_e28192_d_n11;
        locals.var_tmf0_dn14 = assign29100_e28192_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign29110_e28209, assign29110_e28209_d_n0, assign29110_e28209_d_n2, assign29110_e28209_d_n4, assign29110_e28209_d_n5, assign29110_e28209_d_n6, assign29110_e28209_d_n7, assign29110_e28209_d_n8, assign29110_e28209_d_n9, assign29110_e28209_d_n10, assign29110_e28209_d_n11, assign29110_e28209_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29110_e28203: f64 = (0.8 * locals.var_xmp);
        let assign29110_e28205: f64 = (assign29110_e28203 * locals.var_dnm);
        let assign29110_e28207: f64 = (assign29110_e28205 / locals.var_arg);
        (assign29110_e28207, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29110_e28209;
        locals.var_t0_dn0 = assign29110_e28209_d_n0;
        locals.var_t0_dn2 = assign29110_e28209_d_n2;
        locals.var_t0_dn4 = assign29110_e28209_d_n4;
        locals.var_t0_dn5 = assign29110_e28209_d_n5;
        locals.var_t0_dn6 = assign29110_e28209_d_n6;
        locals.var_t0_dn7 = assign29110_e28209_d_n7;
        locals.var_t0_dn8 = assign29110_e28209_d_n8;
        locals.var_t0_dn9 = assign29110_e28209_d_n9;
        locals.var_t0_dn10 = assign29110_e28209_d_n10;
        locals.var_t0_dn11 = assign29110_e28209_d_n11;
        locals.var_t0_dn14 = assign29110_e28209_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29120_e28224, assign29120_e28224_d_n0, assign29120_e28224_d_n2, assign29120_e28224_d_n4, assign29120_e28224_d_n5, assign29120_e28224_d_n6, assign29120_e28224_d_n7, assign29120_e28224_d_n8, assign29120_e28224_d_n9, assign29120_e28224_d_n10, assign29120_e28224_d_n11, assign29120_e28224_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29120_e28220: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign29120_e28222: f64 = (assign29120_e28220 + locals.var_tmf0);
        (assign29120_e28222, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn11 + locals.var_tmf0_dn11), (locals.var_vds_maxb0_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29120_e28224;
        locals.var_t2_dn0 = assign29120_e28224_d_n0;
        locals.var_t2_dn2 = assign29120_e28224_d_n2;
        locals.var_t2_dn4 = assign29120_e28224_d_n4;
        locals.var_t2_dn5 = assign29120_e28224_d_n5;
        locals.var_t2_dn6 = assign29120_e28224_d_n6;
        locals.var_t2_dn7 = assign29120_e28224_d_n7;
        locals.var_t2_dn8 = assign29120_e28224_d_n8;
        locals.var_t2_dn9 = assign29120_e28224_d_n9;
        locals.var_t2_dn10 = assign29120_e28224_d_n10;
        locals.var_t2_dn11 = assign29120_e28224_d_n11;
        locals.var_t2_dn14 = assign29120_e28224_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign29130_e28235, assign29130_e28235_d_n0, assign29130_e28235_d_n2, assign29130_e28235_d_n4, assign29130_e28235_d_n5, assign29130_e28235_d_n6, assign29130_e28235_d_n7, assign29130_e28235_d_n8, assign29130_e28235_d_n9, assign29130_e28235_d_n10, assign29130_e28235_d_n11, assign29130_e28235_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29130_e28235;
        locals.var_t0_dn0 = assign29130_e28235_d_n0;
        locals.var_t0_dn2 = assign29130_e28235_d_n2;
        locals.var_t0_dn4 = assign29130_e28235_d_n4;
        locals.var_t0_dn5 = assign29130_e28235_d_n5;
        locals.var_t0_dn6 = assign29130_e28235_d_n6;
        locals.var_t0_dn7 = assign29130_e28235_d_n7;
        locals.var_t0_dn8 = assign29130_e28235_d_n8;
        locals.var_t0_dn9 = assign29130_e28235_d_n9;
        locals.var_t0_dn10 = assign29130_e28235_d_n10;
        locals.var_t0_dn11 = assign29130_e28235_d_n11;
        locals.var_t0_dn14 = assign29130_e28235_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29140_e28247, assign29140_e28247_d_n0, assign29140_e28247_d_n2, assign29140_e28247_d_n4, assign29140_e28247_d_n5, assign29140_e28247_d_n6, assign29140_e28247_d_n7, assign29140_e28247_d_n8, assign29140_e28247_d_n9, assign29140_e28247_d_n10, assign29140_e28247_d_n11, assign29140_e28247_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 == 0.0)) {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn11, locals.var_phib_ref_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29140_e28247;
        locals.var_t2_dn0 = assign29140_e28247_d_n0;
        locals.var_t2_dn2 = assign29140_e28247_d_n2;
        locals.var_t2_dn4 = assign29140_e28247_d_n4;
        locals.var_t2_dn5 = assign29140_e28247_d_n5;
        locals.var_t2_dn6 = assign29140_e28247_d_n6;
        locals.var_t2_dn7 = assign29140_e28247_d_n7;
        locals.var_t2_dn8 = assign29140_e28247_d_n8;
        locals.var_t2_dn9 = assign29140_e28247_d_n9;
        locals.var_t2_dn10 = assign29140_e28247_d_n10;
        locals.var_t2_dn11 = assign29140_e28247_d_n11;
        locals.var_t2_dn14 = assign29140_e28247_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign29150_e28259, assign29150_e28259_d_n0, assign29150_e28259_d_n2, assign29150_e28259_d_n4, assign29150_e28259_d_n5, assign29150_e28259_d_n6, assign29150_e28259_d_n7, assign29150_e28259_d_n8, assign29150_e28259_d_n9, assign29150_e28259_d_n10, assign29150_e28259_d_n11, assign29150_e28259_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29150_e28259;
        locals.var_t0_dn0 = assign29150_e28259_d_n0;
        locals.var_t0_dn2 = assign29150_e28259_d_n2;
        locals.var_t0_dn4 = assign29150_e28259_d_n4;
        locals.var_t0_dn5 = assign29150_e28259_d_n5;
        locals.var_t0_dn6 = assign29150_e28259_d_n6;
        locals.var_t0_dn7 = assign29150_e28259_d_n7;
        locals.var_t0_dn8 = assign29150_e28259_d_n8;
        locals.var_t0_dn9 = assign29150_e28259_d_n9;
        locals.var_t0_dn10 = assign29150_e28259_d_n10;
        locals.var_t0_dn11 = assign29150_e28259_d_n11;
        locals.var_t0_dn14 = assign29150_e28259_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29160_e28277, assign29160_e28277_d_n0, assign29160_e28277_d_n2, assign29160_e28277_d_n4, assign29160_e28277_d_n5, assign29160_e28277_d_n6, assign29160_e28277_d_n7, assign29160_e28277_d_n8, assign29160_e28277_d_n9, assign29160_e28277_d_n10, assign29160_e28277_d_n11, assign29160_e28277_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29160_e28264: f64 = (-1.6021918e-19);
        let assign29160_e28266: f64 = (assign29160_e28264 * locals.var_uc_ndepm);
        let assign29160_e28270: f64 = (locals.var_t2 - locals.var_vds_maxb0);
        let assign29160_e28271: f64 = (locals.var_beta * assign29160_e28270);
        let assign29160_e28272: f64 = (assign29160_e28271).exp();
        let assign29160_e28273: f64 = (assign29160_e28266 * assign29160_e28272);
        let assign29160_e28275: f64 = (assign29160_e28273 * locals.var_w_b0);
        (assign29160_e28275, (((((assign29160_e28264 * locals.var_uc_ndepm_dn0) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn0 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn0 - locals.var_vds_maxb0_dn0)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn0)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn2) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn2 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn2 - locals.var_vds_maxb0_dn2)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn2)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn4) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn4 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn4 - locals.var_vds_maxb0_dn4)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn4)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn5) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn5 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn5 - locals.var_vds_maxb0_dn5)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn5)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn6) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn6 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn6 - locals.var_vds_maxb0_dn6)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn6)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn7) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn7 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn7 - locals.var_vds_maxb0_dn7)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn7)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn8) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn8 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn8 - locals.var_vds_maxb0_dn8)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn8)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn9) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn9 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn9 - locals.var_vds_maxb0_dn9)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn9)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn10) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn10 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn10 - locals.var_vds_maxb0_dn10)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn10)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn11) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn11 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn11 - locals.var_vds_maxb0_dn11)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn11)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn14) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn14 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn14 - locals.var_vds_maxb0_dn14)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn14)),)
    } else {
        (locals.var_qn_bac, locals.var_qn_bac_dn0, locals.var_qn_bac_dn2, locals.var_qn_bac_dn4, locals.var_qn_bac_dn5, locals.var_qn_bac_dn6, locals.var_qn_bac_dn7, locals.var_qn_bac_dn8, locals.var_qn_bac_dn9, locals.var_qn_bac_dn10, locals.var_qn_bac_dn11, locals.var_qn_bac_dn14,)
    }
};
        locals.var_qn_bac = assign29160_e28277;
        locals.var_qn_bac_dn0 = assign29160_e28277_d_n0;
        locals.var_qn_bac_dn2 = assign29160_e28277_d_n2;
        locals.var_qn_bac_dn4 = assign29160_e28277_d_n4;
        locals.var_qn_bac_dn5 = assign29160_e28277_d_n5;
        locals.var_qn_bac_dn6 = assign29160_e28277_d_n6;
        locals.var_qn_bac_dn7 = assign29160_e28277_d_n7;
        locals.var_qn_bac_dn8 = assign29160_e28277_d_n8;
        locals.var_qn_bac_dn9 = assign29160_e28277_d_n9;
        locals.var_qn_bac_dn10 = assign29160_e28277_d_n10;
        locals.var_qn_bac_dn11 = assign29160_e28277_d_n11;
        locals.var_qn_bac_dn14 = assign29160_e28277_d_n14;
        locals.var_qn_bac_rv = 0.0;

        let assign29170_e28280: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29170_e28283: f64 = 0.06;
        let assign29170_e28288: f64 = if ((assign29170_e28280 < assign29170_e28283) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard684 = assign29170_e28288;
        locals.var_guard684_rv = 0.0;

        let (assign29180_e28302, assign29180_e28302_d_n0, assign29180_e28302_d_n2, assign29180_e28302_d_n4, assign29180_e28302_d_n5, assign29180_e28302_d_n6, assign29180_e28302_d_n7, assign29180_e28302_d_n8, assign29180_e28302_d_n9, assign29180_e28302_d_n10, assign29180_e28302_d_n11, assign29180_e28302_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29180_e28296: f64 = 0.06;
        let assign29180_e28299: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29180_e28300: f64 = (assign29180_e28296 - assign29180_e28299);
        (assign29180_e28300, (-(locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0)), (-(locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2)), (-(locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4)), (-(locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5)), (-(locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6)), (-(locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7)), (-(locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8)), (-(locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9)), (-(locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10)), (-(locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11)), (-(locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29180_e28302;
        locals.var_tmf1_dn0 = assign29180_e28302_d_n0;
        locals.var_tmf1_dn2 = assign29180_e28302_d_n2;
        locals.var_tmf1_dn4 = assign29180_e28302_d_n4;
        locals.var_tmf1_dn5 = assign29180_e28302_d_n5;
        locals.var_tmf1_dn6 = assign29180_e28302_d_n6;
        locals.var_tmf1_dn7 = assign29180_e28302_d_n7;
        locals.var_tmf1_dn8 = assign29180_e28302_d_n8;
        locals.var_tmf1_dn9 = assign29180_e28302_d_n9;
        locals.var_tmf1_dn10 = assign29180_e28302_d_n10;
        locals.var_tmf1_dn11 = assign29180_e28302_d_n11;
        locals.var_tmf1_dn14 = assign29180_e28302_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign29190_e28312, assign29190_e28312_d_n0, assign29190_e28312_d_n2, assign29190_e28312_d_n4, assign29190_e28312_d_n5, assign29190_e28312_d_n6, assign29190_e28312_d_n7, assign29190_e28312_d_n8, assign29190_e28312_d_n9, assign29190_e28312_d_n10, assign29190_e28312_d_n11, assign29190_e28312_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29190_e28310: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29190_e28310, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign29190_e28312;
        locals.var_x2_dn0 = assign29190_e28312_d_n0;
        locals.var_x2_dn2 = assign29190_e28312_d_n2;
        locals.var_x2_dn4 = assign29190_e28312_d_n4;
        locals.var_x2_dn5 = assign29190_e28312_d_n5;
        locals.var_x2_dn6 = assign29190_e28312_d_n6;
        locals.var_x2_dn7 = assign29190_e28312_d_n7;
        locals.var_x2_dn8 = assign29190_e28312_d_n8;
        locals.var_x2_dn9 = assign29190_e28312_d_n9;
        locals.var_x2_dn10 = assign29190_e28312_d_n10;
        locals.var_x2_dn11 = assign29190_e28312_d_n11;
        locals.var_x2_dn14 = assign29190_e28312_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign29200_e28322, assign29200_e28322_d_n0, assign29200_e28322_d_n2, assign29200_e28322_d_n4, assign29200_e28322_d_n5, assign29200_e28322_d_n6, assign29200_e28322_d_n7, assign29200_e28322_d_n8, assign29200_e28322_d_n9, assign29200_e28322_d_n10, assign29200_e28322_d_n11, assign29200_e28322_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29200_e28320: f64 = (0.06 * 0.06);
        (assign29200_e28320, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign29200_e28322;
        locals.var_xmax2_dn0 = assign29200_e28322_d_n0;
        locals.var_xmax2_dn2 = assign29200_e28322_d_n2;
        locals.var_xmax2_dn4 = assign29200_e28322_d_n4;
        locals.var_xmax2_dn5 = assign29200_e28322_d_n5;
        locals.var_xmax2_dn6 = assign29200_e28322_d_n6;
        locals.var_xmax2_dn7 = assign29200_e28322_d_n7;
        locals.var_xmax2_dn8 = assign29200_e28322_d_n8;
        locals.var_xmax2_dn9 = assign29200_e28322_d_n9;
        locals.var_xmax2_dn10 = assign29200_e28322_d_n10;
        locals.var_xmax2_dn11 = assign29200_e28322_d_n11;
        locals.var_xmax2_dn14 = assign29200_e28322_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign29210_e28330, assign29210_e28330_d_n0, assign29210_e28330_d_n2, assign29210_e28330_d_n4, assign29210_e28330_d_n5, assign29210_e28330_d_n6, assign29210_e28330_d_n7, assign29210_e28330_d_n8, assign29210_e28330_d_n9, assign29210_e28330_d_n10, assign29210_e28330_d_n11, assign29210_e28330_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29210_e28330;
        locals.var_xp_dn0 = assign29210_e28330_d_n0;
        locals.var_xp_dn2 = assign29210_e28330_d_n2;
        locals.var_xp_dn4 = assign29210_e28330_d_n4;
        locals.var_xp_dn5 = assign29210_e28330_d_n5;
        locals.var_xp_dn6 = assign29210_e28330_d_n6;
        locals.var_xp_dn7 = assign29210_e28330_d_n7;
        locals.var_xp_dn8 = assign29210_e28330_d_n8;
        locals.var_xp_dn9 = assign29210_e28330_d_n9;
        locals.var_xp_dn10 = assign29210_e28330_d_n10;
        locals.var_xp_dn11 = assign29210_e28330_d_n11;
        locals.var_xp_dn14 = assign29210_e28330_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign29220_e28338, assign29220_e28338_d_n0, assign29220_e28338_d_n2, assign29220_e28338_d_n4, assign29220_e28338_d_n5, assign29220_e28338_d_n6, assign29220_e28338_d_n7, assign29220_e28338_d_n8, assign29220_e28338_d_n9, assign29220_e28338_d_n10, assign29220_e28338_d_n11, assign29220_e28338_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29220_e28338;
        locals.var_xmp_dn0 = assign29220_e28338_d_n0;
        locals.var_xmp_dn2 = assign29220_e28338_d_n2;
        locals.var_xmp_dn4 = assign29220_e28338_d_n4;
        locals.var_xmp_dn5 = assign29220_e28338_d_n5;
        locals.var_xmp_dn6 = assign29220_e28338_d_n6;
        locals.var_xmp_dn7 = assign29220_e28338_d_n7;
        locals.var_xmp_dn8 = assign29220_e28338_d_n8;
        locals.var_xmp_dn9 = assign29220_e28338_d_n9;
        locals.var_xmp_dn10 = assign29220_e28338_d_n10;
        locals.var_xmp_dn11 = assign29220_e28338_d_n11;
        locals.var_xmp_dn14 = assign29220_e28338_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign29230_e28346,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29230_e28346;
        locals.var_m0_rv = 0.0;

        let (assign29240_e28354,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29240_e28354;
        locals.var_mm_rv = 0.0;

        let (assign29250_e28362, assign29250_e28362_d_n0, assign29250_e28362_d_n2, assign29250_e28362_d_n4, assign29250_e28362_d_n5, assign29250_e28362_d_n6, assign29250_e28362_d_n7, assign29250_e28362_d_n8, assign29250_e28362_d_n9, assign29250_e28362_d_n10, assign29250_e28362_d_n11, assign29250_e28362_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29250_e28362;
        locals.var_arg_dn0 = assign29250_e28362_d_n0;
        locals.var_arg_dn2 = assign29250_e28362_d_n2;
        locals.var_arg_dn4 = assign29250_e28362_d_n4;
        locals.var_arg_dn5 = assign29250_e28362_d_n5;
        locals.var_arg_dn6 = assign29250_e28362_d_n6;
        locals.var_arg_dn7 = assign29250_e28362_d_n7;
        locals.var_arg_dn8 = assign29250_e28362_d_n8;
        locals.var_arg_dn9 = assign29250_e28362_d_n9;
        locals.var_arg_dn10 = assign29250_e28362_d_n10;
        locals.var_arg_dn11 = assign29250_e28362_d_n11;
        locals.var_arg_dn14 = assign29250_e28362_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        locals: &mut StampLocals,
    ) {
        let (assign29260_e28370, assign29260_e28370_d_n0, assign29260_e28370_d_n2, assign29260_e28370_d_n4, assign29260_e28370_d_n5, assign29260_e28370_d_n6, assign29260_e28370_d_n7, assign29260_e28370_d_n8, assign29260_e28370_d_n9, assign29260_e28370_d_n10, assign29260_e28370_d_n11, assign29260_e28370_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29260_e28370;
        locals.var_dnm_dn0 = assign29260_e28370_d_n0;
        locals.var_dnm_dn2 = assign29260_e28370_d_n2;
        locals.var_dnm_dn4 = assign29260_e28370_d_n4;
        locals.var_dnm_dn5 = assign29260_e28370_d_n5;
        locals.var_dnm_dn6 = assign29260_e28370_d_n6;
        locals.var_dnm_dn7 = assign29260_e28370_d_n7;
        locals.var_dnm_dn8 = assign29260_e28370_d_n8;
        locals.var_dnm_dn9 = assign29260_e28370_d_n9;
        locals.var_dnm_dn10 = assign29260_e28370_d_n10;
        locals.var_dnm_dn11 = assign29260_e28370_d_n11;
        locals.var_dnm_dn14 = assign29260_e28370_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign29270_e28380, assign29270_e28380_d_n0, assign29270_e28380_d_n2, assign29270_e28380_d_n4, assign29270_e28380_d_n5, assign29270_e28380_d_n6, assign29270_e28380_d_n7, assign29270_e28380_d_n8, assign29270_e28380_d_n9, assign29270_e28380_d_n10, assign29270_e28380_d_n11, assign29270_e28380_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29270_e28378: f64 = (locals.var_xp * locals.var_x2);
        (assign29270_e28378, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29270_e28380;
        locals.var_xp_dn0 = assign29270_e28380_d_n0;
        locals.var_xp_dn2 = assign29270_e28380_d_n2;
        locals.var_xp_dn4 = assign29270_e28380_d_n4;
        locals.var_xp_dn5 = assign29270_e28380_d_n5;
        locals.var_xp_dn6 = assign29270_e28380_d_n6;
        locals.var_xp_dn7 = assign29270_e28380_d_n7;
        locals.var_xp_dn8 = assign29270_e28380_d_n8;
        locals.var_xp_dn9 = assign29270_e28380_d_n9;
        locals.var_xp_dn10 = assign29270_e28380_d_n10;
        locals.var_xp_dn11 = assign29270_e28380_d_n11;
        locals.var_xp_dn14 = assign29270_e28380_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign29280_e28390, assign29280_e28390_d_n0, assign29280_e28390_d_n2, assign29280_e28390_d_n4, assign29280_e28390_d_n5, assign29280_e28390_d_n6, assign29280_e28390_d_n7, assign29280_e28390_d_n8, assign29280_e28390_d_n9, assign29280_e28390_d_n10, assign29280_e28390_d_n11, assign29280_e28390_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29280_e28388: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29280_e28388, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29280_e28390;
        locals.var_xmp_dn0 = assign29280_e28390_d_n0;
        locals.var_xmp_dn2 = assign29280_e28390_d_n2;
        locals.var_xmp_dn4 = assign29280_e28390_d_n4;
        locals.var_xmp_dn5 = assign29280_e28390_d_n5;
        locals.var_xmp_dn6 = assign29280_e28390_d_n6;
        locals.var_xmp_dn7 = assign29280_e28390_d_n7;
        locals.var_xmp_dn8 = assign29280_e28390_d_n8;
        locals.var_xmp_dn9 = assign29280_e28390_d_n9;
        locals.var_xmp_dn10 = assign29280_e28390_d_n10;
        locals.var_xmp_dn11 = assign29280_e28390_d_n11;
        locals.var_xmp_dn14 = assign29280_e28390_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign29290_e28400, assign29290_e28400_d_n0, assign29290_e28400_d_n2, assign29290_e28400_d_n4, assign29290_e28400_d_n5, assign29290_e28400_d_n6, assign29290_e28400_d_n7, assign29290_e28400_d_n8, assign29290_e28400_d_n9, assign29290_e28400_d_n10, assign29290_e28400_d_n11, assign29290_e28400_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29290_e28398: f64 = (locals.var_xp * locals.var_x2);
        (assign29290_e28398, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29290_e28400;
        locals.var_xp_dn0 = assign29290_e28400_d_n0;
        locals.var_xp_dn2 = assign29290_e28400_d_n2;
        locals.var_xp_dn4 = assign29290_e28400_d_n4;
        locals.var_xp_dn5 = assign29290_e28400_d_n5;
        locals.var_xp_dn6 = assign29290_e28400_d_n6;
        locals.var_xp_dn7 = assign29290_e28400_d_n7;
        locals.var_xp_dn8 = assign29290_e28400_d_n8;
        locals.var_xp_dn9 = assign29290_e28400_d_n9;
        locals.var_xp_dn10 = assign29290_e28400_d_n10;
        locals.var_xp_dn11 = assign29290_e28400_d_n11;
        locals.var_xp_dn14 = assign29290_e28400_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign29300_e28410, assign29300_e28410_d_n0, assign29300_e28410_d_n2, assign29300_e28410_d_n4, assign29300_e28410_d_n5, assign29300_e28410_d_n6, assign29300_e28410_d_n7, assign29300_e28410_d_n8, assign29300_e28410_d_n9, assign29300_e28410_d_n10, assign29300_e28410_d_n11, assign29300_e28410_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29300_e28408: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29300_e28408, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29300_e28410;
        locals.var_xmp_dn0 = assign29300_e28410_d_n0;
        locals.var_xmp_dn2 = assign29300_e28410_d_n2;
        locals.var_xmp_dn4 = assign29300_e28410_d_n4;
        locals.var_xmp_dn5 = assign29300_e28410_d_n5;
        locals.var_xmp_dn6 = assign29300_e28410_d_n6;
        locals.var_xmp_dn7 = assign29300_e28410_d_n7;
        locals.var_xmp_dn8 = assign29300_e28410_d_n8;
        locals.var_xmp_dn9 = assign29300_e28410_d_n9;
        locals.var_xmp_dn10 = assign29300_e28410_d_n10;
        locals.var_xmp_dn11 = assign29300_e28410_d_n11;
        locals.var_xmp_dn14 = assign29300_e28410_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign29310_e28420, assign29310_e28420_d_n0, assign29310_e28420_d_n2, assign29310_e28420_d_n4, assign29310_e28420_d_n5, assign29310_e28420_d_n6, assign29310_e28420_d_n7, assign29310_e28420_d_n8, assign29310_e28420_d_n9, assign29310_e28420_d_n10, assign29310_e28420_d_n11, assign29310_e28420_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29310_e28418: f64 = (locals.var_xp + locals.var_xmp);
        (assign29310_e28418, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29310_e28420;
        locals.var_arg_dn0 = assign29310_e28420_d_n0;
        locals.var_arg_dn2 = assign29310_e28420_d_n2;
        locals.var_arg_dn4 = assign29310_e28420_d_n4;
        locals.var_arg_dn5 = assign29310_e28420_d_n5;
        locals.var_arg_dn6 = assign29310_e28420_d_n6;
        locals.var_arg_dn7 = assign29310_e28420_d_n7;
        locals.var_arg_dn8 = assign29310_e28420_d_n8;
        locals.var_arg_dn9 = assign29310_e28420_d_n9;
        locals.var_arg_dn10 = assign29310_e28420_d_n10;
        locals.var_arg_dn11 = assign29310_e28420_d_n11;
        locals.var_arg_dn14 = assign29310_e28420_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign29320_e28428, assign29320_e28428_d_n0, assign29320_e28428_d_n2, assign29320_e28428_d_n4, assign29320_e28428_d_n5, assign29320_e28428_d_n6, assign29320_e28428_d_n7, assign29320_e28428_d_n8, assign29320_e28428_d_n9, assign29320_e28428_d_n10, assign29320_e28428_d_n11, assign29320_e28428_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29320_e28428;
        locals.var_dnm_dn0 = assign29320_e28428_d_n0;
        locals.var_dnm_dn2 = assign29320_e28428_d_n2;
        locals.var_dnm_dn4 = assign29320_e28428_d_n4;
        locals.var_dnm_dn5 = assign29320_e28428_d_n5;
        locals.var_dnm_dn6 = assign29320_e28428_d_n6;
        locals.var_dnm_dn7 = assign29320_e28428_d_n7;
        locals.var_dnm_dn8 = assign29320_e28428_d_n8;
        locals.var_dnm_dn9 = assign29320_e28428_d_n9;
        locals.var_dnm_dn10 = assign29320_e28428_d_n10;
        locals.var_dnm_dn11 = assign29320_e28428_d_n11;
        locals.var_dnm_dn14 = assign29320_e28428_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign29330_e28443: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard685 = assign29330_e28443;
        locals.var_guard685_rv = 0.0;

        let assign29340_e28446: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign29340_e28446;
        locals.var_guard686_rv = 0.0;

        let (assign29350_e28458,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29350_e28458;
        locals.var_mm_rv = 0.0;

        let assign29360_e28461: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign29360_e28461;
        locals.var_guard687_rv = 0.0;

        let (assign29370_e28476,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29370_e28476;
        locals.var_mm_rv = 0.0;

        let assign29380_e28479: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign29380_e28479;
        locals.var_guard688_rv = 0.0;

        let (assign29390_e28497,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard688 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29390_e28497;
        locals.var_mm_rv = 0.0;

        let assign29400_e28500: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign29400_e28500;
        locals.var_guard689_rv = 0.0;

        let (assign29410_e28521,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard688 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29410_e28521;
        locals.var_mm_rv = 0.0;

        let (assign29420_e28531,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29420_e28531;
        locals.var_m0_rv = 0.0;

        let mut assign29430_loop_guard: usize = 0;
        while {
            let assign29430_cond_e28542: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29430_cond_e28542 != 0.0
        } {
            assign29430_loop_guard += 1;
            assert!(assign29430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29430_body0_e28553, assign29430_body0_e28553_d_n0, assign29430_body0_e28553_d_n2, assign29430_body0_e28553_d_n4, assign29430_body0_e28553_d_n5, assign29430_body0_e28553_d_n6, assign29430_body0_e28553_d_n7, assign29430_body0_e28553_d_n8, assign29430_body0_e28553_d_n9, assign29430_body0_e28553_d_n10, assign29430_body0_e28553_d_n11, assign29430_body0_e28553_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        let assign29430_body0_e28551: f64 = (locals.var_dnm).sqrt();
        (assign29430_body0_e28551, (locals.var_dnm_dn0 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn2 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn4 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn5 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn6 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn7 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn8 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn9 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn10 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn11 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn14 / (2.0 * assign29430_body0_e28551)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29430_body0_e28553;
            locals.var_dnm_dn0 = assign29430_body0_e28553_d_n0;
            locals.var_dnm_dn2 = assign29430_body0_e28553_d_n2;
            locals.var_dnm_dn4 = assign29430_body0_e28553_d_n4;
            locals.var_dnm_dn5 = assign29430_body0_e28553_d_n5;
            locals.var_dnm_dn6 = assign29430_body0_e28553_d_n6;
            locals.var_dnm_dn7 = assign29430_body0_e28553_d_n7;
            locals.var_dnm_dn8 = assign29430_body0_e28553_d_n8;
            locals.var_dnm_dn9 = assign29430_body0_e28553_d_n9;
            locals.var_dnm_dn10 = assign29430_body0_e28553_d_n10;
            locals.var_dnm_dn11 = assign29430_body0_e28553_d_n11;
            locals.var_dnm_dn14 = assign29430_body0_e28553_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign29430_body1_e28565,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        let assign29430_body1_e28563: f64 = (locals.var_m0 + 1.0);
        (assign29430_body1_e28563,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29430_body1_e28565;
            locals.var_m0_rv = 0.0;
        }

        let (assign29440_e28587, assign29440_e28587_d_n0, assign29440_e28587_d_n2, assign29440_e28587_d_n4, assign29440_e28587_d_n5, assign29440_e28587_d_n6, assign29440_e28587_d_n7, assign29440_e28587_d_n8, assign29440_e28587_d_n9, assign29440_e28587_d_n10, assign29440_e28587_d_n11, assign29440_e28587_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 == 0.0)) {
        let (assign29440_e28585, assign29440_e28585_d_n0, assign29440_e28585_d_n2, assign29440_e28585_d_n4, assign29440_e28585_d_n5, assign29440_e28585_d_n6, assign29440_e28585_d_n7, assign29440_e28585_d_n8, assign29440_e28585_d_n9, assign29440_e28585_d_n10, assign29440_e28585_d_n11, assign29440_e28585_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29440_e28582: f64 = (2.0 * 2.0);
                let assign29440_e28583: f64 = (1.0 / assign29440_e28582);
                let assign29440_e28584: f64 = (locals.var_dnm).powf(assign29440_e28583);
                (assign29440_e28584, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29440_e28585, assign29440_e28585_d_n0, assign29440_e28585_d_n2, assign29440_e28585_d_n4, assign29440_e28585_d_n5, assign29440_e28585_d_n6, assign29440_e28585_d_n7, assign29440_e28585_d_n8, assign29440_e28585_d_n9, assign29440_e28585_d_n10, assign29440_e28585_d_n11, assign29440_e28585_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29440_e28587;
        locals.var_dnm_dn0 = assign29440_e28587_d_n0;
        locals.var_dnm_dn2 = assign29440_e28587_d_n2;
        locals.var_dnm_dn4 = assign29440_e28587_d_n4;
        locals.var_dnm_dn5 = assign29440_e28587_d_n5;
        locals.var_dnm_dn6 = assign29440_e28587_d_n6;
        locals.var_dnm_dn7 = assign29440_e28587_d_n7;
        locals.var_dnm_dn8 = assign29440_e28587_d_n8;
        locals.var_dnm_dn9 = assign29440_e28587_d_n9;
        locals.var_dnm_dn10 = assign29440_e28587_d_n10;
        locals.var_dnm_dn11 = assign29440_e28587_d_n11;
        locals.var_dnm_dn14 = assign29440_e28587_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign29450_e28597, assign29450_e28597_d_n0, assign29450_e28597_d_n2, assign29450_e28597_d_n4, assign29450_e28597_d_n5, assign29450_e28597_d_n6, assign29450_e28597_d_n7, assign29450_e28597_d_n8, assign29450_e28597_d_n9, assign29450_e28597_d_n10, assign29450_e28597_d_n11, assign29450_e28597_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29450_e28595: f64 = (1.0 / locals.var_dnm);
        (assign29450_e28595, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29450_e28597;
        locals.var_dnm_dn0 = assign29450_e28597_d_n0;
        locals.var_dnm_dn2 = assign29450_e28597_d_n2;
        locals.var_dnm_dn4 = assign29450_e28597_d_n4;
        locals.var_dnm_dn5 = assign29450_e28597_d_n5;
        locals.var_dnm_dn6 = assign29450_e28597_d_n6;
        locals.var_dnm_dn7 = assign29450_e28597_d_n7;
        locals.var_dnm_dn8 = assign29450_e28597_d_n8;
        locals.var_dnm_dn9 = assign29450_e28597_d_n9;
        locals.var_dnm_dn10 = assign29450_e28597_d_n10;
        locals.var_dnm_dn11 = assign29450_e28597_d_n11;
        locals.var_dnm_dn14 = assign29450_e28597_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign29460_e28609, assign29460_e28609_d_n0, assign29460_e28609_d_n2, assign29460_e28609_d_n4, assign29460_e28609_d_n5, assign29460_e28609_d_n6, assign29460_e28609_d_n7, assign29460_e28609_d_n8, assign29460_e28609_d_n9, assign29460_e28609_d_n10, assign29460_e28609_d_n11, assign29460_e28609_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29460_e28605: f64 = (locals.var_tmf1 * 0.06);
        let assign29460_e28607: f64 = (assign29460_e28605 * locals.var_dnm);
        (assign29460_e28607, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29460_e28609;
        locals.var_tmf0_dn0 = assign29460_e28609_d_n0;
        locals.var_tmf0_dn2 = assign29460_e28609_d_n2;
        locals.var_tmf0_dn4 = assign29460_e28609_d_n4;
        locals.var_tmf0_dn5 = assign29460_e28609_d_n5;
        locals.var_tmf0_dn6 = assign29460_e28609_d_n6;
        locals.var_tmf0_dn7 = assign29460_e28609_d_n7;
        locals.var_tmf0_dn8 = assign29460_e28609_d_n8;
        locals.var_tmf0_dn9 = assign29460_e28609_d_n9;
        locals.var_tmf0_dn10 = assign29460_e28609_d_n10;
        locals.var_tmf0_dn11 = assign29460_e28609_d_n11;
        locals.var_tmf0_dn14 = assign29460_e28609_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign29470_e28623, assign29470_e28623_d_n0, assign29470_e28623_d_n2, assign29470_e28623_d_n4, assign29470_e28623_d_n5, assign29470_e28623_d_n6, assign29470_e28623_d_n7, assign29470_e28623_d_n8, assign29470_e28623_d_n9, assign29470_e28623_d_n10, assign29470_e28623_d_n11, assign29470_e28623_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29470_e28617: f64 = (0.06 * locals.var_xmp);
        let assign29470_e28619: f64 = (assign29470_e28617 * locals.var_dnm);
        let assign29470_e28621: f64 = (assign29470_e28619 / locals.var_arg);
        (assign29470_e28621, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29470_e28623;
        locals.var_t0_dn0 = assign29470_e28623_d_n0;
        locals.var_t0_dn2 = assign29470_e28623_d_n2;
        locals.var_t0_dn4 = assign29470_e28623_d_n4;
        locals.var_t0_dn5 = assign29470_e28623_d_n5;
        locals.var_t0_dn6 = assign29470_e28623_d_n6;
        locals.var_t0_dn7 = assign29470_e28623_d_n7;
        locals.var_t0_dn8 = assign29470_e28623_d_n8;
        locals.var_t0_dn9 = assign29470_e28623_d_n9;
        locals.var_t0_dn10 = assign29470_e28623_d_n10;
        locals.var_t0_dn11 = assign29470_e28623_d_n11;
        locals.var_t0_dn14 = assign29470_e28623_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29480_e28635, assign29480_e28635_d_n0, assign29480_e28635_d_n2, assign29480_e28635_d_n4, assign29480_e28635_d_n5, assign29480_e28635_d_n6, assign29480_e28635_d_n7, assign29480_e28635_d_n8, assign29480_e28635_d_n9, assign29480_e28635_d_n10, assign29480_e28635_d_n11, assign29480_e28635_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29480_e28631: f64 = 0.06;
        let assign29480_e28633: f64 = (assign29480_e28631 - locals.var_tmf0);
        (assign29480_e28633, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29480_e28635;
        locals.var_t2_dn0 = assign29480_e28635_d_n0;
        locals.var_t2_dn2 = assign29480_e28635_d_n2;
        locals.var_t2_dn4 = assign29480_e28635_d_n4;
        locals.var_t2_dn5 = assign29480_e28635_d_n5;
        locals.var_t2_dn6 = assign29480_e28635_d_n6;
        locals.var_t2_dn7 = assign29480_e28635_d_n7;
        locals.var_t2_dn8 = assign29480_e28635_d_n8;
        locals.var_t2_dn9 = assign29480_e28635_d_n9;
        locals.var_t2_dn10 = assign29480_e28635_d_n10;
        locals.var_t2_dn11 = assign29480_e28635_d_n11;
        locals.var_t2_dn14 = assign29480_e28635_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign29490_e28643, assign29490_e28643_d_n0, assign29490_e28643_d_n2, assign29490_e28643_d_n4, assign29490_e28643_d_n5, assign29490_e28643_d_n6, assign29490_e28643_d_n7, assign29490_e28643_d_n8, assign29490_e28643_d_n9, assign29490_e28643_d_n10, assign29490_e28643_d_n11, assign29490_e28643_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29490_e28643;
        locals.var_t0_dn0 = assign29490_e28643_d_n0;
        locals.var_t0_dn2 = assign29490_e28643_d_n2;
        locals.var_t0_dn4 = assign29490_e28643_d_n4;
        locals.var_t0_dn5 = assign29490_e28643_d_n5;
        locals.var_t0_dn6 = assign29490_e28643_d_n6;
        locals.var_t0_dn7 = assign29490_e28643_d_n7;
        locals.var_t0_dn8 = assign29490_e28643_d_n8;
        locals.var_t0_dn9 = assign29490_e28643_d_n9;
        locals.var_t0_dn10 = assign29490_e28643_d_n10;
        locals.var_t0_dn11 = assign29490_e28643_d_n11;
        locals.var_t0_dn14 = assign29490_e28643_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29500_e28654, assign29500_e28654_d_n0, assign29500_e28654_d_n2, assign29500_e28654_d_n4, assign29500_e28654_d_n5, assign29500_e28654_d_n6, assign29500_e28654_d_n7, assign29500_e28654_d_n8, assign29500_e28654_d_n9, assign29500_e28654_d_n10, assign29500_e28654_d_n11, assign29500_e28654_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign29500_e28652: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        (assign29500_e28652, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29500_e28654;
        locals.var_t2_dn0 = assign29500_e28654_d_n0;
        locals.var_t2_dn2 = assign29500_e28654_d_n2;
        locals.var_t2_dn4 = assign29500_e28654_d_n4;
        locals.var_t2_dn5 = assign29500_e28654_d_n5;
        locals.var_t2_dn6 = assign29500_e28654_d_n6;
        locals.var_t2_dn7 = assign29500_e28654_d_n7;
        locals.var_t2_dn8 = assign29500_e28654_d_n8;
        locals.var_t2_dn9 = assign29500_e28654_d_n9;
        locals.var_t2_dn10 = assign29500_e28654_d_n10;
        locals.var_t2_dn11 = assign29500_e28654_d_n11;
        locals.var_t2_dn14 = assign29500_e28654_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign29510_e28663, assign29510_e28663_d_n0, assign29510_e28663_d_n2, assign29510_e28663_d_n4, assign29510_e28663_d_n5, assign29510_e28663_d_n6, assign29510_e28663_d_n7, assign29510_e28663_d_n8, assign29510_e28663_d_n9, assign29510_e28663_d_n10, assign29510_e28663_d_n11, assign29510_e28663_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29510_e28663;
        locals.var_t0_dn0 = assign29510_e28663_d_n0;
        locals.var_t0_dn2 = assign29510_e28663_d_n2;
        locals.var_t0_dn4 = assign29510_e28663_d_n4;
        locals.var_t0_dn5 = assign29510_e28663_d_n5;
        locals.var_t0_dn6 = assign29510_e28663_d_n6;
        locals.var_t0_dn7 = assign29510_e28663_d_n7;
        locals.var_t0_dn8 = assign29510_e28663_d_n8;
        locals.var_t0_dn9 = assign29510_e28663_d_n9;
        locals.var_t0_dn10 = assign29510_e28663_d_n10;
        locals.var_t0_dn11 = assign29510_e28663_d_n11;
        locals.var_t0_dn14 = assign29510_e28663_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29520_e28682, assign29520_e28682_d_n0, assign29520_e28682_d_n2, assign29520_e28682_d_n4, assign29520_e28682_d_n5, assign29520_e28682_d_n6, assign29520_e28682_d_n7, assign29520_e28682_d_n8, assign29520_e28682_d_n9, assign29520_e28682_d_n10, assign29520_e28682_d_n11, assign29520_e28682_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29520_e28669: f64 = (locals.var_beta * locals.var_t2);
        let assign29520_e28670: f64 = (assign29520_e28669).exp();
        let assign29520_e28672: f64 = (assign29520_e28670 - 1.0);
        let assign29520_e28675: f64 = (locals.var_beta * locals.var_t2);
        let assign29520_e28676: f64 = (assign29520_e28672 - assign29520_e28675);
        let assign29520_e28679: f64 = (10.0 * 2.220446049250313e-16);
        let assign29520_e28680: f64 = (assign29520_e28676 + assign29520_e28679);
        (assign29520_e28680, ((assign29520_e28670 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign29520_e28670 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign29520_e28670 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign29520_e28670 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign29520_e28670 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign29520_e28670 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign29520_e28670 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign29520_e28670 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign29520_e28670 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign29520_e28670 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign29520_e28670 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29520_e28682;
        locals.var_t4_dn0 = assign29520_e28682_d_n0;
        locals.var_t4_dn2 = assign29520_e28682_d_n2;
        locals.var_t4_dn4 = assign29520_e28682_d_n4;
        locals.var_t4_dn5 = assign29520_e28682_d_n5;
        locals.var_t4_dn6 = assign29520_e28682_d_n6;
        locals.var_t4_dn7 = assign29520_e28682_d_n7;
        locals.var_t4_dn8 = assign29520_e28682_d_n8;
        locals.var_t4_dn9 = assign29520_e28682_d_n9;
        locals.var_t4_dn10 = assign29520_e28682_d_n10;
        locals.var_t4_dn11 = assign29520_e28682_d_n11;
        locals.var_t4_dn14 = assign29520_e28682_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign29530_e28692, assign29530_e28692_d_n0, assign29530_e28692_d_n2, assign29530_e28692_d_n4, assign29530_e28692_d_n5, assign29530_e28692_d_n6, assign29530_e28692_d_n7, assign29530_e28692_d_n8, assign29530_e28692_d_n9, assign29530_e28692_d_n10, assign29530_e28692_d_n11, assign29530_e28692_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29530_e28687: f64 = (-locals.var_cnst0);
        let assign29530_e28689: f64 = (locals.var_t4).sqrt();
        let assign29530_e28690: f64 = (assign29530_e28687 * assign29530_e28689);
        (assign29530_e28690, (((-locals.var_cnst0_dn0) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn0 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn2) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn2 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn4) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn4 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn5) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn5 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn6) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn6 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn7) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn7 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn8) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn8 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn9) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn9 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn10) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn10 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn11) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn11 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn14) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn14 / (2.0 * assign29530_e28689)))),)
    } else {
        (locals.var_q_n0_cur, locals.var_q_n0_cur_dn0, locals.var_q_n0_cur_dn2, locals.var_q_n0_cur_dn4, locals.var_q_n0_cur_dn5, locals.var_q_n0_cur_dn6, locals.var_q_n0_cur_dn7, locals.var_q_n0_cur_dn8, locals.var_q_n0_cur_dn9, locals.var_q_n0_cur_dn10, locals.var_q_n0_cur_dn11, locals.var_q_n0_cur_dn14,)
    }
};
        locals.var_q_n0_cur = assign29530_e28692;
        locals.var_q_n0_cur_dn0 = assign29530_e28692_d_n0;
        locals.var_q_n0_cur_dn2 = assign29530_e28692_d_n2;
        locals.var_q_n0_cur_dn4 = assign29530_e28692_d_n4;
        locals.var_q_n0_cur_dn5 = assign29530_e28692_d_n5;
        locals.var_q_n0_cur_dn6 = assign29530_e28692_d_n6;
        locals.var_q_n0_cur_dn7 = assign29530_e28692_d_n7;
        locals.var_q_n0_cur_dn8 = assign29530_e28692_d_n8;
        locals.var_q_n0_cur_dn9 = assign29530_e28692_d_n9;
        locals.var_q_n0_cur_dn10 = assign29530_e28692_d_n10;
        locals.var_q_n0_cur_dn11 = assign29530_e28692_d_n11;
        locals.var_q_n0_cur_dn14 = assign29530_e28692_d_n14;
        locals.var_q_n0_cur_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
        locals: &mut StampLocals,
    ) {
        let (assign29540_e28707, assign29540_e28707_d_n0, assign29540_e28707_d_n2, assign29540_e28707_d_n4, assign29540_e28707_d_n5, assign29540_e28707_d_n6, assign29540_e28707_d_n7, assign29540_e28707_d_n8, assign29540_e28707_d_n9, assign29540_e28707_d_n10, assign29540_e28707_d_n11, assign29540_e28707_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29540_e28698: f64 = (locals.var_beta * 0.1);
        let assign29540_e28699: f64 = (assign29540_e28698).exp();
        let assign29540_e28701: f64 = (assign29540_e28699 - 1.0);
        let assign29540_e28704: f64 = (locals.var_beta * 0.1);
        let assign29540_e28705: f64 = (assign29540_e28701 - assign29540_e28704);
        (assign29540_e28705, ((assign29540_e28699 * (locals.var_beta_dn0 * 0.1)) - (locals.var_beta_dn0 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn2 * 0.1)) - (locals.var_beta_dn2 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn4 * 0.1)) - (locals.var_beta_dn4 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn5 * 0.1)) - (locals.var_beta_dn5 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn6 * 0.1)) - (locals.var_beta_dn6 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn7 * 0.1)) - (locals.var_beta_dn7 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn8 * 0.1)) - (locals.var_beta_dn8 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn9 * 0.1)) - (locals.var_beta_dn9 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn10 * 0.1)) - (locals.var_beta_dn10 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn11 * 0.1)) - (locals.var_beta_dn11 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn14 * 0.1)) - (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29540_e28707;
        locals.var_t4_dn0 = assign29540_e28707_d_n0;
        locals.var_t4_dn2 = assign29540_e28707_d_n2;
        locals.var_t4_dn4 = assign29540_e28707_d_n4;
        locals.var_t4_dn5 = assign29540_e28707_d_n5;
        locals.var_t4_dn6 = assign29540_e28707_d_n6;
        locals.var_t4_dn7 = assign29540_e28707_d_n7;
        locals.var_t4_dn8 = assign29540_e28707_d_n8;
        locals.var_t4_dn9 = assign29540_e28707_d_n9;
        locals.var_t4_dn10 = assign29540_e28707_d_n10;
        locals.var_t4_dn11 = assign29540_e28707_d_n11;
        locals.var_t4_dn14 = assign29540_e28707_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign29550_e28716, assign29550_e28716_d_n0, assign29550_e28716_d_n2, assign29550_e28716_d_n4, assign29550_e28716_d_n5, assign29550_e28716_d_n6, assign29550_e28716_d_n7, assign29550_e28716_d_n8, assign29550_e28716_d_n9, assign29550_e28716_d_n10, assign29550_e28716_d_n11, assign29550_e28716_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29550_e28713: f64 = (locals.var_t4).sqrt();
        let assign29550_e28714: f64 = (locals.var_cnst0 * assign29550_e28713);
        (assign29550_e28714, ((locals.var_cnst0_dn0 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn0 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn2 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn2 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn4 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn4 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn5 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn5 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn6 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn6 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn7 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn7 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn8 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn8 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn9 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn9 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn10 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn10 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn11 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn11 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn14 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn14 / (2.0 * assign29550_e28713)))),)
    } else {
        (locals.var_qn_delta, locals.var_qn_delta_dn0, locals.var_qn_delta_dn2, locals.var_qn_delta_dn4, locals.var_qn_delta_dn5, locals.var_qn_delta_dn6, locals.var_qn_delta_dn7, locals.var_qn_delta_dn8, locals.var_qn_delta_dn9, locals.var_qn_delta_dn10, locals.var_qn_delta_dn11, locals.var_qn_delta_dn14,)
    }
};
        locals.var_qn_delta = assign29550_e28716;
        locals.var_qn_delta_dn0 = assign29550_e28716_d_n0;
        locals.var_qn_delta_dn2 = assign29550_e28716_d_n2;
        locals.var_qn_delta_dn4 = assign29550_e28716_d_n4;
        locals.var_qn_delta_dn5 = assign29550_e28716_d_n5;
        locals.var_qn_delta_dn6 = assign29550_e28716_d_n6;
        locals.var_qn_delta_dn7 = assign29550_e28716_d_n7;
        locals.var_qn_delta_dn8 = assign29550_e28716_d_n8;
        locals.var_qn_delta_dn9 = assign29550_e28716_d_n9;
        locals.var_qn_delta_dn10 = assign29550_e28716_d_n10;
        locals.var_qn_delta_dn11 = assign29550_e28716_d_n11;
        locals.var_qn_delta_dn14 = assign29550_e28716_d_n14;
        locals.var_qn_delta_rv = 0.0;

        let (assign29560_e28722, assign29560_e28722_d_n0, assign29560_e28722_d_n2, assign29560_e28722_d_n4, assign29560_e28722_d_n5, assign29560_e28722_d_n6, assign29560_e28722_d_n7, assign29560_e28722_d_n8, assign29560_e28722_d_n9, assign29560_e28722_d_n10, assign29560_e28722_d_n11, assign29560_e28722_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    }
};
        locals.var_vdsorg = assign29560_e28722;
        locals.var_vdsorg_dn0 = assign29560_e28722_d_n0;
        locals.var_vdsorg_dn2 = assign29560_e28722_d_n2;
        locals.var_vdsorg_dn4 = assign29560_e28722_d_n4;
        locals.var_vdsorg_dn5 = assign29560_e28722_d_n5;
        locals.var_vdsorg_dn6 = assign29560_e28722_d_n6;
        locals.var_vdsorg_dn7 = assign29560_e28722_d_n7;
        locals.var_vdsorg_dn8 = assign29560_e28722_d_n8;
        locals.var_vdsorg_dn9 = assign29560_e28722_d_n9;
        locals.var_vdsorg_dn10 = assign29560_e28722_d_n10;
        locals.var_vdsorg_dn11 = assign29560_e28722_d_n11;
        locals.var_vdsorg_dn14 = assign29560_e28722_d_n14;
        locals.var_vdsorg_rv = 0.0;

        let assign29570_e28725: f64 = if locals.var_vds > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard690 = assign29570_e28725;
        locals.var_guard690_rv = 0.0;

        let (assign29580_e28737, assign29580_e28737_d_n0, assign29580_e28737_d_n2, assign29580_e28737_d_n4, assign29580_e28737_d_n5, assign29580_e28737_d_n6, assign29580_e28737_d_n7, assign29580_e28737_d_n8, assign29580_e28737_d_n9, assign29580_e28737_d_n10, assign29580_e28737_d_n11, assign29580_e28737_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29580_e28734: f64 = (locals.var_cox * locals.var_cox);
        let assign29580_e28735: f64 = (locals.var_q_ndepm_esi / assign29580_e28734);
        (assign29580_e28735, (((locals.var_q_ndepm_esi_dn0 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn2 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn4 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn5 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn6 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn7 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn8 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn9 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn10 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn11 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn14 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign29580_e28734 * assign29580_e28734)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29580_e28737;
        locals.var_t2_dn0 = assign29580_e28737_d_n0;
        locals.var_t2_dn2 = assign29580_e28737_d_n2;
        locals.var_t2_dn4 = assign29580_e28737_d_n4;
        locals.var_t2_dn5 = assign29580_e28737_d_n5;
        locals.var_t2_dn6 = assign29580_e28737_d_n6;
        locals.var_t2_dn7 = assign29580_e28737_d_n7;
        locals.var_t2_dn8 = assign29580_e28737_d_n8;
        locals.var_t2_dn9 = assign29580_e28737_d_n9;
        locals.var_t2_dn10 = assign29580_e28737_d_n10;
        locals.var_t2_dn11 = assign29580_e28737_d_n11;
        locals.var_t2_dn14 = assign29580_e28737_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign29590_e28751, assign29590_e28751_d_n0, assign29590_e28751_d_n2, assign29590_e28751_d_n4, assign29590_e28751_d_n5, assign29590_e28751_d_n6, assign29590_e28751_d_n7, assign29590_e28751_d_n8, assign29590_e28751_d_n9, assign29590_e28751_d_n10, assign29590_e28751_d_n11, assign29590_e28751_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29590_e28745: f64 = (locals.var_vgp + 2.0);
        let assign29590_e28747: f64 = (assign29590_e28745 - locals.var_beta_inv);
        let assign29590_e28749: f64 = (assign29590_e28747 - locals.var_vbsz__blk442);
        (assign29590_e28749, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_vgp_dn11 - locals.var_beta_inv_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_vgp_dn14 - locals.var_beta_inv_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29590_e28751;
        locals.var_t0_dn0 = assign29590_e28751_d_n0;
        locals.var_t0_dn2 = assign29590_e28751_d_n2;
        locals.var_t0_dn4 = assign29590_e28751_d_n4;
        locals.var_t0_dn5 = assign29590_e28751_d_n5;
        locals.var_t0_dn6 = assign29590_e28751_d_n6;
        locals.var_t0_dn7 = assign29590_e28751_d_n7;
        locals.var_t0_dn8 = assign29590_e28751_d_n8;
        locals.var_t0_dn9 = assign29590_e28751_d_n9;
        locals.var_t0_dn10 = assign29590_e28751_d_n10;
        locals.var_t0_dn11 = assign29590_e28751_d_n11;
        locals.var_t0_dn14 = assign29590_e28751_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29600_e28765, assign29600_e28765_d_n0, assign29600_e28765_d_n2, assign29600_e28765_d_n4, assign29600_e28765_d_n5, assign29600_e28765_d_n6, assign29600_e28765_d_n7, assign29600_e28765_d_n8, assign29600_e28765_d_n9, assign29600_e28765_d_n10, assign29600_e28765_d_n11, assign29600_e28765_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29600_e28760: f64 = (2.0 / locals.var_t2);
        let assign29600_e28762: f64 = (assign29600_e28760 * locals.var_t0);
        let assign29600_e28763: f64 = (1.0 + assign29600_e28762);
        (assign29600_e28763, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29600_e28765;
        locals.var_t4_dn0 = assign29600_e28765_d_n0;
        locals.var_t4_dn2 = assign29600_e28765_d_n2;
        locals.var_t4_dn4 = assign29600_e28765_d_n4;
        locals.var_t4_dn5 = assign29600_e28765_d_n5;
        locals.var_t4_dn6 = assign29600_e28765_d_n6;
        locals.var_t4_dn7 = assign29600_e28765_d_n7;
        locals.var_t4_dn8 = assign29600_e28765_d_n8;
        locals.var_t4_dn9 = assign29600_e28765_d_n9;
        locals.var_t4_dn10 = assign29600_e28765_d_n10;
        locals.var_t4_dn11 = assign29600_e28765_d_n11;
        locals.var_t4_dn14 = assign29600_e28765_d_n14;
        locals.var_t4_rv = 0.0;

        let assign29610_e28769: f64 = 2.0;
        let assign29610_e28774: f64 = if ((locals.var_t4 < assign29610_e28769) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard691 = assign29610_e28774;
        locals.var_guard691_rv = 0.0;

        let (assign29620_e28788, assign29620_e28788_d_n0, assign29620_e28788_d_n2, assign29620_e28788_d_n4, assign29620_e28788_d_n5, assign29620_e28788_d_n6, assign29620_e28788_d_n7, assign29620_e28788_d_n8, assign29620_e28788_d_n9, assign29620_e28788_d_n10, assign29620_e28788_d_n11, assign29620_e28788_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29620_e28784: f64 = 2.0;
        let assign29620_e28786: f64 = (assign29620_e28784 - locals.var_t4);
        (assign29620_e28786, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn11), (-locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29620_e28788;
        locals.var_tmf1_dn0 = assign29620_e28788_d_n0;
        locals.var_tmf1_dn2 = assign29620_e28788_d_n2;
        locals.var_tmf1_dn4 = assign29620_e28788_d_n4;
        locals.var_tmf1_dn5 = assign29620_e28788_d_n5;
        locals.var_tmf1_dn6 = assign29620_e28788_d_n6;
        locals.var_tmf1_dn7 = assign29620_e28788_d_n7;
        locals.var_tmf1_dn8 = assign29620_e28788_d_n8;
        locals.var_tmf1_dn9 = assign29620_e28788_d_n9;
        locals.var_tmf1_dn10 = assign29620_e28788_d_n10;
        locals.var_tmf1_dn11 = assign29620_e28788_d_n11;
        locals.var_tmf1_dn14 = assign29620_e28788_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign29630_e28800, assign29630_e28800_d_n0, assign29630_e28800_d_n2, assign29630_e28800_d_n4, assign29630_e28800_d_n5, assign29630_e28800_d_n6, assign29630_e28800_d_n7, assign29630_e28800_d_n8, assign29630_e28800_d_n9, assign29630_e28800_d_n10, assign29630_e28800_d_n11, assign29630_e28800_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29630_e28798: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29630_e28798, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign29630_e28800;
        locals.var_x2_dn0 = assign29630_e28800_d_n0;
        locals.var_x2_dn2 = assign29630_e28800_d_n2;
        locals.var_x2_dn4 = assign29630_e28800_d_n4;
        locals.var_x2_dn5 = assign29630_e28800_d_n5;
        locals.var_x2_dn6 = assign29630_e28800_d_n6;
        locals.var_x2_dn7 = assign29630_e28800_d_n7;
        locals.var_x2_dn8 = assign29630_e28800_d_n8;
        locals.var_x2_dn9 = assign29630_e28800_d_n9;
        locals.var_x2_dn10 = assign29630_e28800_d_n10;
        locals.var_x2_dn11 = assign29630_e28800_d_n11;
        locals.var_x2_dn14 = assign29630_e28800_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign29640_e28812, assign29640_e28812_d_n0, assign29640_e28812_d_n2, assign29640_e28812_d_n4, assign29640_e28812_d_n5, assign29640_e28812_d_n6, assign29640_e28812_d_n7, assign29640_e28812_d_n8, assign29640_e28812_d_n9, assign29640_e28812_d_n10, assign29640_e28812_d_n11, assign29640_e28812_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29640_e28810: f64 = (2.0 * 2.0);
        (assign29640_e28810, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign29640_e28812;
        locals.var_xmax2_dn0 = assign29640_e28812_d_n0;
        locals.var_xmax2_dn2 = assign29640_e28812_d_n2;
        locals.var_xmax2_dn4 = assign29640_e28812_d_n4;
        locals.var_xmax2_dn5 = assign29640_e28812_d_n5;
        locals.var_xmax2_dn6 = assign29640_e28812_d_n6;
        locals.var_xmax2_dn7 = assign29640_e28812_d_n7;
        locals.var_xmax2_dn8 = assign29640_e28812_d_n8;
        locals.var_xmax2_dn9 = assign29640_e28812_d_n9;
        locals.var_xmax2_dn10 = assign29640_e28812_d_n10;
        locals.var_xmax2_dn11 = assign29640_e28812_d_n11;
        locals.var_xmax2_dn14 = assign29640_e28812_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign29650_e28822, assign29650_e28822_d_n0, assign29650_e28822_d_n2, assign29650_e28822_d_n4, assign29650_e28822_d_n5, assign29650_e28822_d_n6, assign29650_e28822_d_n7, assign29650_e28822_d_n8, assign29650_e28822_d_n9, assign29650_e28822_d_n10, assign29650_e28822_d_n11, assign29650_e28822_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29650_e28822;
        locals.var_xp_dn0 = assign29650_e28822_d_n0;
        locals.var_xp_dn2 = assign29650_e28822_d_n2;
        locals.var_xp_dn4 = assign29650_e28822_d_n4;
        locals.var_xp_dn5 = assign29650_e28822_d_n5;
        locals.var_xp_dn6 = assign29650_e28822_d_n6;
        locals.var_xp_dn7 = assign29650_e28822_d_n7;
        locals.var_xp_dn8 = assign29650_e28822_d_n8;
        locals.var_xp_dn9 = assign29650_e28822_d_n9;
        locals.var_xp_dn10 = assign29650_e28822_d_n10;
        locals.var_xp_dn11 = assign29650_e28822_d_n11;
        locals.var_xp_dn14 = assign29650_e28822_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign29660_e28832, assign29660_e28832_d_n0, assign29660_e28832_d_n2, assign29660_e28832_d_n4, assign29660_e28832_d_n5, assign29660_e28832_d_n6, assign29660_e28832_d_n7, assign29660_e28832_d_n8, assign29660_e28832_d_n9, assign29660_e28832_d_n10, assign29660_e28832_d_n11, assign29660_e28832_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29660_e28832;
        locals.var_xmp_dn0 = assign29660_e28832_d_n0;
        locals.var_xmp_dn2 = assign29660_e28832_d_n2;
        locals.var_xmp_dn4 = assign29660_e28832_d_n4;
        locals.var_xmp_dn5 = assign29660_e28832_d_n5;
        locals.var_xmp_dn6 = assign29660_e28832_d_n6;
        locals.var_xmp_dn7 = assign29660_e28832_d_n7;
        locals.var_xmp_dn8 = assign29660_e28832_d_n8;
        locals.var_xmp_dn9 = assign29660_e28832_d_n9;
        locals.var_xmp_dn10 = assign29660_e28832_d_n10;
        locals.var_xmp_dn11 = assign29660_e28832_d_n11;
        locals.var_xmp_dn14 = assign29660_e28832_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign29670_e28842,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29670_e28842;
        locals.var_m0_rv = 0.0;

        let (assign29680_e28852,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29680_e28852;
        locals.var_mm_rv = 0.0;

        let (assign29690_e28862, assign29690_e28862_d_n0, assign29690_e28862_d_n2, assign29690_e28862_d_n4, assign29690_e28862_d_n5, assign29690_e28862_d_n6, assign29690_e28862_d_n7, assign29690_e28862_d_n8, assign29690_e28862_d_n9, assign29690_e28862_d_n10, assign29690_e28862_d_n11, assign29690_e28862_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29690_e28862;
        locals.var_arg_dn0 = assign29690_e28862_d_n0;
        locals.var_arg_dn2 = assign29690_e28862_d_n2;
        locals.var_arg_dn4 = assign29690_e28862_d_n4;
        locals.var_arg_dn5 = assign29690_e28862_d_n5;
        locals.var_arg_dn6 = assign29690_e28862_d_n6;
        locals.var_arg_dn7 = assign29690_e28862_d_n7;
        locals.var_arg_dn8 = assign29690_e28862_d_n8;
        locals.var_arg_dn9 = assign29690_e28862_d_n9;
        locals.var_arg_dn10 = assign29690_e28862_d_n10;
        locals.var_arg_dn11 = assign29690_e28862_d_n11;
        locals.var_arg_dn14 = assign29690_e28862_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign29700_e28872, assign29700_e28872_d_n0, assign29700_e28872_d_n2, assign29700_e28872_d_n4, assign29700_e28872_d_n5, assign29700_e28872_d_n6, assign29700_e28872_d_n7, assign29700_e28872_d_n8, assign29700_e28872_d_n9, assign29700_e28872_d_n10, assign29700_e28872_d_n11, assign29700_e28872_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29700_e28872;
        locals.var_dnm_dn0 = assign29700_e28872_d_n0;
        locals.var_dnm_dn2 = assign29700_e28872_d_n2;
        locals.var_dnm_dn4 = assign29700_e28872_d_n4;
        locals.var_dnm_dn5 = assign29700_e28872_d_n5;
        locals.var_dnm_dn6 = assign29700_e28872_d_n6;
        locals.var_dnm_dn7 = assign29700_e28872_d_n7;
        locals.var_dnm_dn8 = assign29700_e28872_d_n8;
        locals.var_dnm_dn9 = assign29700_e28872_d_n9;
        locals.var_dnm_dn10 = assign29700_e28872_d_n10;
        locals.var_dnm_dn11 = assign29700_e28872_d_n11;
        locals.var_dnm_dn14 = assign29700_e28872_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign29710_e28884, assign29710_e28884_d_n0, assign29710_e28884_d_n2, assign29710_e28884_d_n4, assign29710_e28884_d_n5, assign29710_e28884_d_n6, assign29710_e28884_d_n7, assign29710_e28884_d_n8, assign29710_e28884_d_n9, assign29710_e28884_d_n10, assign29710_e28884_d_n11, assign29710_e28884_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29710_e28882: f64 = (locals.var_xp * locals.var_x2);
        (assign29710_e28882, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29710_e28884;
        locals.var_xp_dn0 = assign29710_e28884_d_n0;
        locals.var_xp_dn2 = assign29710_e28884_d_n2;
        locals.var_xp_dn4 = assign29710_e28884_d_n4;
        locals.var_xp_dn5 = assign29710_e28884_d_n5;
        locals.var_xp_dn6 = assign29710_e28884_d_n6;
        locals.var_xp_dn7 = assign29710_e28884_d_n7;
        locals.var_xp_dn8 = assign29710_e28884_d_n8;
        locals.var_xp_dn9 = assign29710_e28884_d_n9;
        locals.var_xp_dn10 = assign29710_e28884_d_n10;
        locals.var_xp_dn11 = assign29710_e28884_d_n11;
        locals.var_xp_dn14 = assign29710_e28884_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign29720_e28896, assign29720_e28896_d_n0, assign29720_e28896_d_n2, assign29720_e28896_d_n4, assign29720_e28896_d_n5, assign29720_e28896_d_n6, assign29720_e28896_d_n7, assign29720_e28896_d_n8, assign29720_e28896_d_n9, assign29720_e28896_d_n10, assign29720_e28896_d_n11, assign29720_e28896_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29720_e28894: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29720_e28894, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29720_e28896;
        locals.var_xmp_dn0 = assign29720_e28896_d_n0;
        locals.var_xmp_dn2 = assign29720_e28896_d_n2;
        locals.var_xmp_dn4 = assign29720_e28896_d_n4;
        locals.var_xmp_dn5 = assign29720_e28896_d_n5;
        locals.var_xmp_dn6 = assign29720_e28896_d_n6;
        locals.var_xmp_dn7 = assign29720_e28896_d_n7;
        locals.var_xmp_dn8 = assign29720_e28896_d_n8;
        locals.var_xmp_dn9 = assign29720_e28896_d_n9;
        locals.var_xmp_dn10 = assign29720_e28896_d_n10;
        locals.var_xmp_dn11 = assign29720_e28896_d_n11;
        locals.var_xmp_dn14 = assign29720_e28896_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign29730_e28908, assign29730_e28908_d_n0, assign29730_e28908_d_n2, assign29730_e28908_d_n4, assign29730_e28908_d_n5, assign29730_e28908_d_n6, assign29730_e28908_d_n7, assign29730_e28908_d_n8, assign29730_e28908_d_n9, assign29730_e28908_d_n10, assign29730_e28908_d_n11, assign29730_e28908_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29730_e28906: f64 = (locals.var_xp * locals.var_x2);
        (assign29730_e28906, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29730_e28908;
        locals.var_xp_dn0 = assign29730_e28908_d_n0;
        locals.var_xp_dn2 = assign29730_e28908_d_n2;
        locals.var_xp_dn4 = assign29730_e28908_d_n4;
        locals.var_xp_dn5 = assign29730_e28908_d_n5;
        locals.var_xp_dn6 = assign29730_e28908_d_n6;
        locals.var_xp_dn7 = assign29730_e28908_d_n7;
        locals.var_xp_dn8 = assign29730_e28908_d_n8;
        locals.var_xp_dn9 = assign29730_e28908_d_n9;
        locals.var_xp_dn10 = assign29730_e28908_d_n10;
        locals.var_xp_dn11 = assign29730_e28908_d_n11;
        locals.var_xp_dn14 = assign29730_e28908_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign29740_e28920, assign29740_e28920_d_n0, assign29740_e28920_d_n2, assign29740_e28920_d_n4, assign29740_e28920_d_n5, assign29740_e28920_d_n6, assign29740_e28920_d_n7, assign29740_e28920_d_n8, assign29740_e28920_d_n9, assign29740_e28920_d_n10, assign29740_e28920_d_n11, assign29740_e28920_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29740_e28918: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29740_e28918, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29740_e28920;
        locals.var_xmp_dn0 = assign29740_e28920_d_n0;
        locals.var_xmp_dn2 = assign29740_e28920_d_n2;
        locals.var_xmp_dn4 = assign29740_e28920_d_n4;
        locals.var_xmp_dn5 = assign29740_e28920_d_n5;
        locals.var_xmp_dn6 = assign29740_e28920_d_n6;
        locals.var_xmp_dn7 = assign29740_e28920_d_n7;
        locals.var_xmp_dn8 = assign29740_e28920_d_n8;
        locals.var_xmp_dn9 = assign29740_e28920_d_n9;
        locals.var_xmp_dn10 = assign29740_e28920_d_n10;
        locals.var_xmp_dn11 = assign29740_e28920_d_n11;
        locals.var_xmp_dn14 = assign29740_e28920_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign29750_e28932, assign29750_e28932_d_n0, assign29750_e28932_d_n2, assign29750_e28932_d_n4, assign29750_e28932_d_n5, assign29750_e28932_d_n6, assign29750_e28932_d_n7, assign29750_e28932_d_n8, assign29750_e28932_d_n9, assign29750_e28932_d_n10, assign29750_e28932_d_n11, assign29750_e28932_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29750_e28930: f64 = (locals.var_xp + locals.var_xmp);
        (assign29750_e28930, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29750_e28932;
        locals.var_arg_dn0 = assign29750_e28932_d_n0;
        locals.var_arg_dn2 = assign29750_e28932_d_n2;
        locals.var_arg_dn4 = assign29750_e28932_d_n4;
        locals.var_arg_dn5 = assign29750_e28932_d_n5;
        locals.var_arg_dn6 = assign29750_e28932_d_n6;
        locals.var_arg_dn7 = assign29750_e28932_d_n7;
        locals.var_arg_dn8 = assign29750_e28932_d_n8;
        locals.var_arg_dn9 = assign29750_e28932_d_n9;
        locals.var_arg_dn10 = assign29750_e28932_d_n10;
        locals.var_arg_dn11 = assign29750_e28932_d_n11;
        locals.var_arg_dn14 = assign29750_e28932_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign29760_e28942, assign29760_e28942_d_n0, assign29760_e28942_d_n2, assign29760_e28942_d_n4, assign29760_e28942_d_n5, assign29760_e28942_d_n6, assign29760_e28942_d_n7, assign29760_e28942_d_n8, assign29760_e28942_d_n9, assign29760_e28942_d_n10, assign29760_e28942_d_n11, assign29760_e28942_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29760_e28942;
        locals.var_dnm_dn0 = assign29760_e28942_d_n0;
        locals.var_dnm_dn2 = assign29760_e28942_d_n2;
        locals.var_dnm_dn4 = assign29760_e28942_d_n4;
        locals.var_dnm_dn5 = assign29760_e28942_d_n5;
        locals.var_dnm_dn6 = assign29760_e28942_d_n6;
        locals.var_dnm_dn7 = assign29760_e28942_d_n7;
        locals.var_dnm_dn8 = assign29760_e28942_d_n8;
        locals.var_dnm_dn9 = assign29760_e28942_d_n9;
        locals.var_dnm_dn10 = assign29760_e28942_d_n10;
        locals.var_dnm_dn11 = assign29760_e28942_d_n11;
        locals.var_dnm_dn14 = assign29760_e28942_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign29770_e28957: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard692 = assign29770_e28957;
        locals.var_guard692_rv = 0.0;

        let assign29780_e28960: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign29780_e28960;
        locals.var_guard693_rv = 0.0;

        let (assign29790_e28974,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29790_e28974;
        locals.var_mm_rv = 0.0;

        let assign29800_e28977: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign29800_e28977;
        locals.var_guard694_rv = 0.0;

        let (assign29810_e28994,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29810_e28994;
        locals.var_mm_rv = 0.0;

        let assign29820_e28997: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard695 = assign29820_e28997;
        locals.var_guard695_rv = 0.0;

        let (assign29830_e29017,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 == 0.0)) && (locals.var_guard695 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29830_e29017;
        locals.var_mm_rv = 0.0;

        let assign29840_e29020: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard696 = assign29840_e29020;
        locals.var_guard696_rv = 0.0;

        let (assign29850_e29043,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 == 0.0)) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29850_e29043;
        locals.var_mm_rv = 0.0;

    }
}
