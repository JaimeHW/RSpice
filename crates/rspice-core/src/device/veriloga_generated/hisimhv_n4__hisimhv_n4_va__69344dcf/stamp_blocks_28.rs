#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_73(
        locals: &mut StampLocals,
    ) {
        let (assign25590_e23972, assign25590_e23972_d_n0, assign25590_e23972_d_n2, assign25590_e23972_d_n4, assign25590_e23972_d_n5, assign25590_e23972_d_n6, assign25590_e23972_d_n7, assign25590_e23972_d_n8, assign25590_e23972_d_n9, assign25590_e23972_d_n10, assign25590_e23972_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 != 0.0)) {
        let assign25590_e23968: f64 = (locals.var_phi_s0_dep - 0.02);
        let assign25590_e23970: f64 = (assign25590_e23968 + locals.var_tmf0);
        (assign25590_e23970, (locals.var_phi_s0_dep_dn0 + locals.var_tmf0_dn0), (locals.var_phi_s0_dep_dn2 + locals.var_tmf0_dn2), (locals.var_phi_s0_dep_dn4 + locals.var_tmf0_dn4), (locals.var_phi_s0_dep_dn5 + locals.var_tmf0_dn5), (locals.var_phi_s0_dep_dn6 + locals.var_tmf0_dn6), (locals.var_phi_s0_dep_dn7 + locals.var_tmf0_dn7), (locals.var_phi_s0_dep_dn8 + locals.var_tmf0_dn8), (locals.var_phi_s0_dep_dn9 + locals.var_tmf0_dn9), (locals.var_phi_s0_dep_dn10 + locals.var_tmf0_dn10), (locals.var_phi_s0_dep_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign25590_e23972;
        locals.var_phi_b0_dep_dn0 = assign25590_e23972_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25590_e23972_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25590_e23972_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25590_e23972_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25590_e23972_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25590_e23972_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25590_e23972_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25590_e23972_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25590_e23972_d_n10;
        locals.var_phi_b0_dep_dn13 = assign25590_e23972_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25600_e23982, assign25600_e23982_d_n0, assign25600_e23982_d_n2, assign25600_e23982_d_n4, assign25600_e23982_d_n5, assign25600_e23982_d_n6, assign25600_e23982_d_n7, assign25600_e23982_d_n8, assign25600_e23982_d_n9, assign25600_e23982_d_n10, assign25600_e23982_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign25600_e23982;
        locals.var_t1_dn0 = assign25600_e23982_d_n0;
        locals.var_t1_dn2 = assign25600_e23982_d_n2;
        locals.var_t1_dn4 = assign25600_e23982_d_n4;
        locals.var_t1_dn5 = assign25600_e23982_d_n5;
        locals.var_t1_dn6 = assign25600_e23982_d_n6;
        locals.var_t1_dn7 = assign25600_e23982_d_n7;
        locals.var_t1_dn8 = assign25600_e23982_d_n8;
        locals.var_t1_dn9 = assign25600_e23982_d_n9;
        locals.var_t1_dn10 = assign25600_e23982_d_n10;
        locals.var_t1_dn13 = assign25600_e23982_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign25610_e23993, assign25610_e23993_d_n0, assign25610_e23993_d_n2, assign25610_e23993_d_n4, assign25610_e23993_d_n5, assign25610_e23993_d_n6, assign25610_e23993_d_n7, assign25610_e23993_d_n8, assign25610_e23993_d_n9, assign25610_e23993_d_n10, assign25610_e23993_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign25610_e23993;
        locals.var_phi_b0_dep_dn0 = assign25610_e23993_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25610_e23993_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25610_e23993_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25610_e23993_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25610_e23993_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25610_e23993_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25610_e23993_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25610_e23993_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25610_e23993_d_n10;
        locals.var_phi_b0_dep_dn13 = assign25610_e23993_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25620_e24004, assign25620_e24004_d_n0, assign25620_e24004_d_n2, assign25620_e24004_d_n4, assign25620_e24004_d_n5, assign25620_e24004_d_n6, assign25620_e24004_d_n7, assign25620_e24004_d_n8, assign25620_e24004_d_n9, assign25620_e24004_d_n10, assign25620_e24004_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign25620_e24004;
        locals.var_t1_dn0 = assign25620_e24004_d_n0;
        locals.var_t1_dn2 = assign25620_e24004_d_n2;
        locals.var_t1_dn4 = assign25620_e24004_d_n4;
        locals.var_t1_dn5 = assign25620_e24004_d_n5;
        locals.var_t1_dn6 = assign25620_e24004_d_n6;
        locals.var_t1_dn7 = assign25620_e24004_d_n7;
        locals.var_t1_dn8 = assign25620_e24004_d_n8;
        locals.var_t1_dn9 = assign25620_e24004_d_n9;
        locals.var_t1_dn10 = assign25620_e24004_d_n10;
        locals.var_t1_dn13 = assign25620_e24004_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign25630_e24018, assign25630_e24018_d_n0, assign25630_e24018_d_n2, assign25630_e24018_d_n4, assign25630_e24018_d_n5, assign25630_e24018_d_n6, assign25630_e24018_d_n7, assign25630_e24018_d_n8, assign25630_e24018_d_n9, assign25630_e24018_d_n10, assign25630_e24018_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign25630_e24011: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign25630_e24013: f64 = (assign25630_e24011 + locals.var_vbscl__blk435);
        let assign25630_e24015: f64 = (assign25630_e24013 - locals.var_vbi_dep);
        let assign25630_e24016: f64 = (locals.var_ndepmpnsub_inv1 * assign25630_e24015);
        (assign25630_e24016, ((locals.var_ndepmpnsub_inv1_dn0 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk435_dn0) - locals.var_vbi_dep_dn0))), ((locals.var_ndepmpnsub_inv1_dn2 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk435_dn2) - locals.var_vbi_dep_dn2))), ((locals.var_ndepmpnsub_inv1_dn4 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk435_dn4) - locals.var_vbi_dep_dn4))), ((locals.var_ndepmpnsub_inv1_dn5 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk435_dn5) - locals.var_vbi_dep_dn5))), ((locals.var_ndepmpnsub_inv1_dn6 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk435_dn6) - locals.var_vbi_dep_dn6))), ((locals.var_ndepmpnsub_inv1_dn7 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk435_dn7) - locals.var_vbi_dep_dn7))), ((locals.var_ndepmpnsub_inv1_dn8 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk435_dn8) - locals.var_vbi_dep_dn8))), ((locals.var_ndepmpnsub_inv1_dn9 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk435_dn9) - locals.var_vbi_dep_dn9))), ((locals.var_ndepmpnsub_inv1_dn10 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk435_dn10) - locals.var_vbi_dep_dn10))), ((locals.var_ndepmpnsub_inv1_dn13 * assign25630_e24015) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn13 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn13)) + locals.var_vbscl__blk435_dn13) - locals.var_vbi_dep_dn13))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
        locals.var_phi_j0_dep = assign25630_e24018;
        locals.var_phi_j0_dep_dn0 = assign25630_e24018_d_n0;
        locals.var_phi_j0_dep_dn2 = assign25630_e24018_d_n2;
        locals.var_phi_j0_dep_dn4 = assign25630_e24018_d_n4;
        locals.var_phi_j0_dep_dn5 = assign25630_e24018_d_n5;
        locals.var_phi_j0_dep_dn6 = assign25630_e24018_d_n6;
        locals.var_phi_j0_dep_dn7 = assign25630_e24018_d_n7;
        locals.var_phi_j0_dep_dn8 = assign25630_e24018_d_n8;
        locals.var_phi_j0_dep_dn9 = assign25630_e24018_d_n9;
        locals.var_phi_j0_dep_dn10 = assign25630_e24018_d_n10;
        locals.var_phi_j0_dep_dn13 = assign25630_e24018_d_n13;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign25640_e24028, assign25640_e24028_d_n0, assign25640_e24028_d_n2, assign25640_e24028_d_n4, assign25640_e24028_d_n5, assign25640_e24028_d_n6, assign25640_e24028_d_n7, assign25640_e24028_d_n8, assign25640_e24028_d_n9, assign25640_e24028_d_n10, assign25640_e24028_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign25640_e24025: f64 = (locals.var_phi_s0_dep - locals.var_phi_b0_dep);
        let assign25640_e24026: f64 = (locals.var_beta * assign25640_e24025);
        (assign25640_e24026, ((locals.var_beta_dn0 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn0 - locals.var_phi_b0_dep_dn0))), ((locals.var_beta_dn2 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn2 - locals.var_phi_b0_dep_dn2))), ((locals.var_beta_dn4 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn4 - locals.var_phi_b0_dep_dn4))), ((locals.var_beta_dn5 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn5 - locals.var_phi_b0_dep_dn5))), ((locals.var_beta_dn6 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn6 - locals.var_phi_b0_dep_dn6))), ((locals.var_beta_dn7 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn7 - locals.var_phi_b0_dep_dn7))), ((locals.var_beta_dn8 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn8 - locals.var_phi_b0_dep_dn8))), ((locals.var_beta_dn9 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn9 - locals.var_phi_b0_dep_dn9))), ((locals.var_beta_dn10 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn10 - locals.var_phi_b0_dep_dn10))), ((locals.var_beta_dn13 * assign25640_e24025) + (locals.var_beta * (locals.var_phi_s0_dep_dn13 - locals.var_phi_b0_dep_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign25640_e24028;
        locals.var_t1_dn0 = assign25640_e24028_d_n0;
        locals.var_t1_dn2 = assign25640_e24028_d_n2;
        locals.var_t1_dn4 = assign25640_e24028_d_n4;
        locals.var_t1_dn5 = assign25640_e24028_d_n5;
        locals.var_t1_dn6 = assign25640_e24028_d_n6;
        locals.var_t1_dn7 = assign25640_e24028_d_n7;
        locals.var_t1_dn8 = assign25640_e24028_d_n8;
        locals.var_t1_dn9 = assign25640_e24028_d_n9;
        locals.var_t1_dn10 = assign25640_e24028_d_n10;
        locals.var_t1_dn13 = assign25640_e24028_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign25650_e24035, assign25650_e24035_d_n0, assign25650_e24035_d_n2, assign25650_e24035_d_n4, assign25650_e24035_d_n5, assign25650_e24035_d_n6, assign25650_e24035_d_n7, assign25650_e24035_d_n8, assign25650_e24035_d_n9, assign25650_e24035_d_n10, assign25650_e24035_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign25650_e24033: f64 = (locals.var_t1).exp();
        (assign25650_e24033, (assign25650_e24033 * locals.var_t1_dn0), (assign25650_e24033 * locals.var_t1_dn2), (assign25650_e24033 * locals.var_t1_dn4), (assign25650_e24033 * locals.var_t1_dn5), (assign25650_e24033 * locals.var_t1_dn6), (assign25650_e24033 * locals.var_t1_dn7), (assign25650_e24033 * locals.var_t1_dn8), (assign25650_e24033 * locals.var_t1_dn9), (assign25650_e24033 * locals.var_t1_dn10), (assign25650_e24033 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign25650_e24035;
        locals.var_t2_dn0 = assign25650_e24035_d_n0;
        locals.var_t2_dn2 = assign25650_e24035_d_n2;
        locals.var_t2_dn4 = assign25650_e24035_d_n4;
        locals.var_t2_dn5 = assign25650_e24035_d_n5;
        locals.var_t2_dn6 = assign25650_e24035_d_n6;
        locals.var_t2_dn7 = assign25650_e24035_d_n7;
        locals.var_t2_dn8 = assign25650_e24035_d_n8;
        locals.var_t2_dn9 = assign25650_e24035_d_n9;
        locals.var_t2_dn10 = assign25650_e24035_d_n10;
        locals.var_t2_dn13 = assign25650_e24035_d_n13;
        locals.var_t2_rv = 0.0;

        let assign25660_e24038: f64 = if locals.var_phi_s0_dep >= locals.var_phi_b0_dep { 1.0 } else { 0.0 };
        locals.var_guard623 = assign25660_e24038;
        locals.var_guard623_rv = 0.0;

        let (assign25670_e24056, assign25670_e24056_d_n0, assign25670_e24056_d_n2, assign25670_e24056_d_n4, assign25670_e24056_d_n5, assign25670_e24056_d_n6, assign25670_e24056_d_n7, assign25670_e24056_d_n8, assign25670_e24056_d_n9, assign25670_e24056_d_n10, assign25670_e24056_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign25670_e24045: f64 = (-locals.var_cnst0);
        let assign25670_e24048: f64 = (locals.var_t2 - 1.0);
        let assign25670_e24050: f64 = (assign25670_e24048 - locals.var_t1);
        let assign25670_e24052: f64 = (assign25670_e24050 + 1e-15);
        let assign25670_e24053: f64 = (assign25670_e24052).sqrt();
        let assign25670_e24054: f64 = (assign25670_e24045 * assign25670_e24053);
        (assign25670_e24054, (((-locals.var_cnst0_dn0) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn2) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn4) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn5) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn6) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn7) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn8) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn9) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn10) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign25670_e24053)))), (((-locals.var_cnst0_dn13) * assign25670_e24053) + (assign25670_e24045 * ((locals.var_t2_dn13 - locals.var_t1_dn13) / (2.0 * assign25670_e24053)))),)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn13,)
    }
};
        locals.var_q_s0 = assign25670_e24056;
        locals.var_q_s0_dn0 = assign25670_e24056_d_n0;
        locals.var_q_s0_dn2 = assign25670_e24056_d_n2;
        locals.var_q_s0_dn4 = assign25670_e24056_d_n4;
        locals.var_q_s0_dn5 = assign25670_e24056_d_n5;
        locals.var_q_s0_dn6 = assign25670_e24056_d_n6;
        locals.var_q_s0_dn7 = assign25670_e24056_d_n7;
        locals.var_q_s0_dn8 = assign25670_e24056_d_n8;
        locals.var_q_s0_dn9 = assign25670_e24056_d_n9;
        locals.var_q_s0_dn10 = assign25670_e24056_d_n10;
        locals.var_q_s0_dn13 = assign25670_e24056_d_n13;
        locals.var_q_s0_rv = 0.0;

        let (assign25680_e24064, assign25680_e24064_d_n0, assign25680_e24064_d_n2, assign25680_e24064_d_n4, assign25680_e24064_d_n5, assign25680_e24064_d_n6, assign25680_e24064_d_n7, assign25680_e24064_d_n8, assign25680_e24064_d_n9, assign25680_e24064_d_n10, assign25680_e24064_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn13,)
    } else {
        (locals.var_q_n0__blk538, locals.var_q_n0__blk538_dn0, locals.var_q_n0__blk538_dn2, locals.var_q_n0__blk538_dn4, locals.var_q_n0__blk538_dn5, locals.var_q_n0__blk538_dn6, locals.var_q_n0__blk538_dn7, locals.var_q_n0__blk538_dn8, locals.var_q_n0__blk538_dn9, locals.var_q_n0__blk538_dn10, locals.var_q_n0__blk538_dn13,)
    }
};
        locals.var_q_n0__blk538 = assign25680_e24064;
        locals.var_q_n0__blk538_dn0 = assign25680_e24064_d_n0;
        locals.var_q_n0__blk538_dn2 = assign25680_e24064_d_n2;
        locals.var_q_n0__blk538_dn4 = assign25680_e24064_d_n4;
        locals.var_q_n0__blk538_dn5 = assign25680_e24064_d_n5;
        locals.var_q_n0__blk538_dn6 = assign25680_e24064_d_n6;
        locals.var_q_n0__blk538_dn7 = assign25680_e24064_d_n7;
        locals.var_q_n0__blk538_dn8 = assign25680_e24064_d_n8;
        locals.var_q_n0__blk538_dn9 = assign25680_e24064_d_n9;
        locals.var_q_n0__blk538_dn10 = assign25680_e24064_d_n10;
        locals.var_q_n0__blk538_dn13 = assign25680_e24064_d_n13;
        locals.var_q_n0__blk538_rv = 0.0;

        let (assign25690_e24072, assign25690_e24072_d_n0, assign25690_e24072_d_n2, assign25690_e24072_d_n4, assign25690_e24072_d_n5, assign25690_e24072_d_n6, assign25690_e24072_d_n7, assign25690_e24072_d_n8, assign25690_e24072_d_n9, assign25690_e24072_d_n10, assign25690_e24072_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn13,)
    }
};
        locals.var_q_s0_dep = assign25690_e24072;
        locals.var_q_s0_dep_dn0 = assign25690_e24072_d_n0;
        locals.var_q_s0_dep_dn2 = assign25690_e24072_d_n2;
        locals.var_q_s0_dep_dn4 = assign25690_e24072_d_n4;
        locals.var_q_s0_dep_dn5 = assign25690_e24072_d_n5;
        locals.var_q_s0_dep_dn6 = assign25690_e24072_d_n6;
        locals.var_q_s0_dep_dn7 = assign25690_e24072_d_n7;
        locals.var_q_s0_dep_dn8 = assign25690_e24072_d_n8;
        locals.var_q_s0_dep_dn9 = assign25690_e24072_d_n9;
        locals.var_q_s0_dep_dn10 = assign25690_e24072_d_n10;
        locals.var_q_s0_dep_dn13 = assign25690_e24072_d_n13;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign25700_e24080, assign25700_e24080_d_n0, assign25700_e24080_d_n2, assign25700_e24080_d_n4, assign25700_e24080_d_n5, assign25700_e24080_d_n6, assign25700_e24080_d_n7, assign25700_e24080_d_n8, assign25700_e24080_d_n9, assign25700_e24080_d_n10, assign25700_e24080_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn13,)
    }
};
        locals.var_q_sub0 = assign25700_e24080;
        locals.var_q_sub0_dn0 = assign25700_e24080_d_n0;
        locals.var_q_sub0_dn2 = assign25700_e24080_d_n2;
        locals.var_q_sub0_dn4 = assign25700_e24080_d_n4;
        locals.var_q_sub0_dn5 = assign25700_e24080_d_n5;
        locals.var_q_sub0_dn6 = assign25700_e24080_d_n6;
        locals.var_q_sub0_dn7 = assign25700_e24080_d_n7;
        locals.var_q_sub0_dn8 = assign25700_e24080_d_n8;
        locals.var_q_sub0_dn9 = assign25700_e24080_d_n9;
        locals.var_q_sub0_dn10 = assign25700_e24080_d_n10;
        locals.var_q_sub0_dn13 = assign25700_e24080_d_n13;
        locals.var_q_sub0_rv = 0.0;

        let (assign25710_e24093, assign25710_e24093_d_n0, assign25710_e24093_d_n2, assign25710_e24093_d_n4, assign25710_e24093_d_n5, assign25710_e24093_d_n6, assign25710_e24093_d_n7, assign25710_e24093_d_n8, assign25710_e24093_d_n9, assign25710_e24093_d_n10, assign25710_e24093_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign25710_e24089: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25710_e24090: f64 = (locals.var_c_2esipq_ndepm * assign25710_e24089);
        let assign25710_e24091: f64 = (assign25710_e24090).sqrt();
        (assign25710_e24091, (((locals.var_c_2esipq_ndepm_dn0 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn2 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn4 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn5 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn6 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn7 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn8 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn9 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn10 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25710_e24091)), (((locals.var_c_2esipq_ndepm_dn13 * assign25710_e24089) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13))) / (2.0 * assign25710_e24091)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign25710_e24093;
        locals.var_w_b0_dn0 = assign25710_e24093_d_n0;
        locals.var_w_b0_dn2 = assign25710_e24093_d_n2;
        locals.var_w_b0_dn4 = assign25710_e24093_d_n4;
        locals.var_w_b0_dn5 = assign25710_e24093_d_n5;
        locals.var_w_b0_dn6 = assign25710_e24093_d_n6;
        locals.var_w_b0_dn7 = assign25710_e24093_d_n7;
        locals.var_w_b0_dn8 = assign25710_e24093_d_n8;
        locals.var_w_b0_dn9 = assign25710_e24093_d_n9;
        locals.var_w_b0_dn10 = assign25710_e24093_d_n10;
        locals.var_w_b0_dn13 = assign25710_e24093_d_n13;
        locals.var_w_b0_rv = 0.0;

        let assign25720_e24097: f64 = (locals.var_uc_depthn - 1e-8);
        let assign25720_e24102: f64 = if ((locals.var_w_b0 > assign25720_e24097) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard624 = assign25720_e24102;
        locals.var_guard624_rv = 0.0;

        let (assign25730_e24116, assign25730_e24116_d_n0, assign25730_e24116_d_n2, assign25730_e24116_d_n4, assign25730_e24116_d_n5, assign25730_e24116_d_n6, assign25730_e24116_d_n7, assign25730_e24116_d_n8, assign25730_e24116_d_n9, assign25730_e24116_d_n10, assign25730_e24116_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25730_e24112: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign25730_e24114: f64 = (assign25730_e24112 + 1e-8);
        (assign25730_e24114, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn13 - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign25730_e24116;
        locals.var_tmf1_dn0 = assign25730_e24116_d_n0;
        locals.var_tmf1_dn2 = assign25730_e24116_d_n2;
        locals.var_tmf1_dn4 = assign25730_e24116_d_n4;
        locals.var_tmf1_dn5 = assign25730_e24116_d_n5;
        locals.var_tmf1_dn6 = assign25730_e24116_d_n6;
        locals.var_tmf1_dn7 = assign25730_e24116_d_n7;
        locals.var_tmf1_dn8 = assign25730_e24116_d_n8;
        locals.var_tmf1_dn9 = assign25730_e24116_d_n9;
        locals.var_tmf1_dn10 = assign25730_e24116_d_n10;
        locals.var_tmf1_dn13 = assign25730_e24116_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign25740_e24128, assign25740_e24128_d_n0, assign25740_e24128_d_n2, assign25740_e24128_d_n4, assign25740_e24128_d_n5, assign25740_e24128_d_n6, assign25740_e24128_d_n7, assign25740_e24128_d_n8, assign25740_e24128_d_n9, assign25740_e24128_d_n10, assign25740_e24128_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25740_e24126: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25740_e24126, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign25740_e24128;
        locals.var_x2_dn0 = assign25740_e24128_d_n0;
        locals.var_x2_dn2 = assign25740_e24128_d_n2;
        locals.var_x2_dn4 = assign25740_e24128_d_n4;
        locals.var_x2_dn5 = assign25740_e24128_d_n5;
        locals.var_x2_dn6 = assign25740_e24128_d_n6;
        locals.var_x2_dn7 = assign25740_e24128_d_n7;
        locals.var_x2_dn8 = assign25740_e24128_d_n8;
        locals.var_x2_dn9 = assign25740_e24128_d_n9;
        locals.var_x2_dn10 = assign25740_e24128_d_n10;
        locals.var_x2_dn13 = assign25740_e24128_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign25750_e24140, assign25750_e24140_d_n0, assign25750_e24140_d_n2, assign25750_e24140_d_n4, assign25750_e24140_d_n5, assign25750_e24140_d_n6, assign25750_e24140_d_n7, assign25750_e24140_d_n8, assign25750_e24140_d_n9, assign25750_e24140_d_n10, assign25750_e24140_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25750_e24138: f64 = (1e-8 * 1e-8);
        (assign25750_e24138, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign25750_e24140;
        locals.var_xmax2_dn0 = assign25750_e24140_d_n0;
        locals.var_xmax2_dn2 = assign25750_e24140_d_n2;
        locals.var_xmax2_dn4 = assign25750_e24140_d_n4;
        locals.var_xmax2_dn5 = assign25750_e24140_d_n5;
        locals.var_xmax2_dn6 = assign25750_e24140_d_n6;
        locals.var_xmax2_dn7 = assign25750_e24140_d_n7;
        locals.var_xmax2_dn8 = assign25750_e24140_d_n8;
        locals.var_xmax2_dn9 = assign25750_e24140_d_n9;
        locals.var_xmax2_dn10 = assign25750_e24140_d_n10;
        locals.var_xmax2_dn13 = assign25750_e24140_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign25760_e24150, assign25760_e24150_d_n0, assign25760_e24150_d_n2, assign25760_e24150_d_n4, assign25760_e24150_d_n5, assign25760_e24150_d_n6, assign25760_e24150_d_n7, assign25760_e24150_d_n8, assign25760_e24150_d_n9, assign25760_e24150_d_n10, assign25760_e24150_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign25760_e24150;
        locals.var_xp_dn0 = assign25760_e24150_d_n0;
        locals.var_xp_dn2 = assign25760_e24150_d_n2;
        locals.var_xp_dn4 = assign25760_e24150_d_n4;
        locals.var_xp_dn5 = assign25760_e24150_d_n5;
        locals.var_xp_dn6 = assign25760_e24150_d_n6;
        locals.var_xp_dn7 = assign25760_e24150_d_n7;
        locals.var_xp_dn8 = assign25760_e24150_d_n8;
        locals.var_xp_dn9 = assign25760_e24150_d_n9;
        locals.var_xp_dn10 = assign25760_e24150_d_n10;
        locals.var_xp_dn13 = assign25760_e24150_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign25770_e24160, assign25770_e24160_d_n0, assign25770_e24160_d_n2, assign25770_e24160_d_n4, assign25770_e24160_d_n5, assign25770_e24160_d_n6, assign25770_e24160_d_n7, assign25770_e24160_d_n8, assign25770_e24160_d_n9, assign25770_e24160_d_n10, assign25770_e24160_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign25770_e24160;
        locals.var_xmp_dn0 = assign25770_e24160_d_n0;
        locals.var_xmp_dn2 = assign25770_e24160_d_n2;
        locals.var_xmp_dn4 = assign25770_e24160_d_n4;
        locals.var_xmp_dn5 = assign25770_e24160_d_n5;
        locals.var_xmp_dn6 = assign25770_e24160_d_n6;
        locals.var_xmp_dn7 = assign25770_e24160_d_n7;
        locals.var_xmp_dn8 = assign25770_e24160_d_n8;
        locals.var_xmp_dn9 = assign25770_e24160_d_n9;
        locals.var_xmp_dn10 = assign25770_e24160_d_n10;
        locals.var_xmp_dn13 = assign25770_e24160_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign25780_e24170,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25780_e24170;
        locals.var_m0_rv = 0.0;

        let (assign25790_e24180,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25790_e24180;
        locals.var_mm_rv = 0.0;

        let (assign25800_e24190, assign25800_e24190_d_n0, assign25800_e24190_d_n2, assign25800_e24190_d_n4, assign25800_e24190_d_n5, assign25800_e24190_d_n6, assign25800_e24190_d_n7, assign25800_e24190_d_n8, assign25800_e24190_d_n9, assign25800_e24190_d_n10, assign25800_e24190_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign25800_e24190;
        locals.var_arg_dn0 = assign25800_e24190_d_n0;
        locals.var_arg_dn2 = assign25800_e24190_d_n2;
        locals.var_arg_dn4 = assign25800_e24190_d_n4;
        locals.var_arg_dn5 = assign25800_e24190_d_n5;
        locals.var_arg_dn6 = assign25800_e24190_d_n6;
        locals.var_arg_dn7 = assign25800_e24190_d_n7;
        locals.var_arg_dn8 = assign25800_e24190_d_n8;
        locals.var_arg_dn9 = assign25800_e24190_d_n9;
        locals.var_arg_dn10 = assign25800_e24190_d_n10;
        locals.var_arg_dn13 = assign25800_e24190_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign25810_e24200, assign25810_e24200_d_n0, assign25810_e24200_d_n2, assign25810_e24200_d_n4, assign25810_e24200_d_n5, assign25810_e24200_d_n6, assign25810_e24200_d_n7, assign25810_e24200_d_n8, assign25810_e24200_d_n9, assign25810_e24200_d_n10, assign25810_e24200_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign25810_e24200;
        locals.var_dnm_dn0 = assign25810_e24200_d_n0;
        locals.var_dnm_dn2 = assign25810_e24200_d_n2;
        locals.var_dnm_dn4 = assign25810_e24200_d_n4;
        locals.var_dnm_dn5 = assign25810_e24200_d_n5;
        locals.var_dnm_dn6 = assign25810_e24200_d_n6;
        locals.var_dnm_dn7 = assign25810_e24200_d_n7;
        locals.var_dnm_dn8 = assign25810_e24200_d_n8;
        locals.var_dnm_dn9 = assign25810_e24200_d_n9;
        locals.var_dnm_dn10 = assign25810_e24200_d_n10;
        locals.var_dnm_dn13 = assign25810_e24200_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign25820_e24212, assign25820_e24212_d_n0, assign25820_e24212_d_n2, assign25820_e24212_d_n4, assign25820_e24212_d_n5, assign25820_e24212_d_n6, assign25820_e24212_d_n7, assign25820_e24212_d_n8, assign25820_e24212_d_n9, assign25820_e24212_d_n10, assign25820_e24212_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25820_e24210: f64 = (locals.var_xp * locals.var_x2);
        (assign25820_e24210, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign25820_e24212;
        locals.var_xp_dn0 = assign25820_e24212_d_n0;
        locals.var_xp_dn2 = assign25820_e24212_d_n2;
        locals.var_xp_dn4 = assign25820_e24212_d_n4;
        locals.var_xp_dn5 = assign25820_e24212_d_n5;
        locals.var_xp_dn6 = assign25820_e24212_d_n6;
        locals.var_xp_dn7 = assign25820_e24212_d_n7;
        locals.var_xp_dn8 = assign25820_e24212_d_n8;
        locals.var_xp_dn9 = assign25820_e24212_d_n9;
        locals.var_xp_dn10 = assign25820_e24212_d_n10;
        locals.var_xp_dn13 = assign25820_e24212_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign25830_e24224, assign25830_e24224_d_n0, assign25830_e24224_d_n2, assign25830_e24224_d_n4, assign25830_e24224_d_n5, assign25830_e24224_d_n6, assign25830_e24224_d_n7, assign25830_e24224_d_n8, assign25830_e24224_d_n9, assign25830_e24224_d_n10, assign25830_e24224_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25830_e24222: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25830_e24222, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign25830_e24224;
        locals.var_xmp_dn0 = assign25830_e24224_d_n0;
        locals.var_xmp_dn2 = assign25830_e24224_d_n2;
        locals.var_xmp_dn4 = assign25830_e24224_d_n4;
        locals.var_xmp_dn5 = assign25830_e24224_d_n5;
        locals.var_xmp_dn6 = assign25830_e24224_d_n6;
        locals.var_xmp_dn7 = assign25830_e24224_d_n7;
        locals.var_xmp_dn8 = assign25830_e24224_d_n8;
        locals.var_xmp_dn9 = assign25830_e24224_d_n9;
        locals.var_xmp_dn10 = assign25830_e24224_d_n10;
        locals.var_xmp_dn13 = assign25830_e24224_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign25840_e24236, assign25840_e24236_d_n0, assign25840_e24236_d_n2, assign25840_e24236_d_n4, assign25840_e24236_d_n5, assign25840_e24236_d_n6, assign25840_e24236_d_n7, assign25840_e24236_d_n8, assign25840_e24236_d_n9, assign25840_e24236_d_n10, assign25840_e24236_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25840_e24234: f64 = (locals.var_xp * locals.var_x2);
        (assign25840_e24234, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign25840_e24236;
        locals.var_xp_dn0 = assign25840_e24236_d_n0;
        locals.var_xp_dn2 = assign25840_e24236_d_n2;
        locals.var_xp_dn4 = assign25840_e24236_d_n4;
        locals.var_xp_dn5 = assign25840_e24236_d_n5;
        locals.var_xp_dn6 = assign25840_e24236_d_n6;
        locals.var_xp_dn7 = assign25840_e24236_d_n7;
        locals.var_xp_dn8 = assign25840_e24236_d_n8;
        locals.var_xp_dn9 = assign25840_e24236_d_n9;
        locals.var_xp_dn10 = assign25840_e24236_d_n10;
        locals.var_xp_dn13 = assign25840_e24236_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_74(
        locals: &mut StampLocals,
    ) {
        let (assign25850_e24248, assign25850_e24248_d_n0, assign25850_e24248_d_n2, assign25850_e24248_d_n4, assign25850_e24248_d_n5, assign25850_e24248_d_n6, assign25850_e24248_d_n7, assign25850_e24248_d_n8, assign25850_e24248_d_n9, assign25850_e24248_d_n10, assign25850_e24248_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25850_e24246: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25850_e24246, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign25850_e24248;
        locals.var_xmp_dn0 = assign25850_e24248_d_n0;
        locals.var_xmp_dn2 = assign25850_e24248_d_n2;
        locals.var_xmp_dn4 = assign25850_e24248_d_n4;
        locals.var_xmp_dn5 = assign25850_e24248_d_n5;
        locals.var_xmp_dn6 = assign25850_e24248_d_n6;
        locals.var_xmp_dn7 = assign25850_e24248_d_n7;
        locals.var_xmp_dn8 = assign25850_e24248_d_n8;
        locals.var_xmp_dn9 = assign25850_e24248_d_n9;
        locals.var_xmp_dn10 = assign25850_e24248_d_n10;
        locals.var_xmp_dn13 = assign25850_e24248_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign25860_e24260, assign25860_e24260_d_n0, assign25860_e24260_d_n2, assign25860_e24260_d_n4, assign25860_e24260_d_n5, assign25860_e24260_d_n6, assign25860_e24260_d_n7, assign25860_e24260_d_n8, assign25860_e24260_d_n9, assign25860_e24260_d_n10, assign25860_e24260_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign25860_e24258: f64 = (locals.var_xp + locals.var_xmp);
        (assign25860_e24258, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign25860_e24260;
        locals.var_arg_dn0 = assign25860_e24260_d_n0;
        locals.var_arg_dn2 = assign25860_e24260_d_n2;
        locals.var_arg_dn4 = assign25860_e24260_d_n4;
        locals.var_arg_dn5 = assign25860_e24260_d_n5;
        locals.var_arg_dn6 = assign25860_e24260_d_n6;
        locals.var_arg_dn7 = assign25860_e24260_d_n7;
        locals.var_arg_dn8 = assign25860_e24260_d_n8;
        locals.var_arg_dn9 = assign25860_e24260_d_n9;
        locals.var_arg_dn10 = assign25860_e24260_d_n10;
        locals.var_arg_dn13 = assign25860_e24260_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign25870_e24270, assign25870_e24270_d_n0, assign25870_e24270_d_n2, assign25870_e24270_d_n4, assign25870_e24270_d_n5, assign25870_e24270_d_n6, assign25870_e24270_d_n7, assign25870_e24270_d_n8, assign25870_e24270_d_n9, assign25870_e24270_d_n10, assign25870_e24270_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign25870_e24270;
        locals.var_dnm_dn0 = assign25870_e24270_d_n0;
        locals.var_dnm_dn2 = assign25870_e24270_d_n2;
        locals.var_dnm_dn4 = assign25870_e24270_d_n4;
        locals.var_dnm_dn5 = assign25870_e24270_d_n5;
        locals.var_dnm_dn6 = assign25870_e24270_d_n6;
        locals.var_dnm_dn7 = assign25870_e24270_d_n7;
        locals.var_dnm_dn8 = assign25870_e24270_d_n8;
        locals.var_dnm_dn9 = assign25870_e24270_d_n9;
        locals.var_dnm_dn10 = assign25870_e24270_d_n10;
        locals.var_dnm_dn13 = assign25870_e24270_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign25880_e24285: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard625 = assign25880_e24285;
        locals.var_guard625_rv = 0.0;

        let assign25890_e24288: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard626 = assign25890_e24288;
        locals.var_guard626_rv = 0.0;

        let (assign25900_e24302,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25900_e24302;
        locals.var_mm_rv = 0.0;

        let assign25910_e24305: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign25910_e24305;
        locals.var_guard627_rv = 0.0;

        let (assign25920_e24322,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard627 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25920_e24322;
        locals.var_mm_rv = 0.0;

        let assign25930_e24325: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign25930_e24325;
        locals.var_guard628_rv = 0.0;

        let (assign25940_e24345,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25940_e24345;
        locals.var_mm_rv = 0.0;

        let assign25950_e24348: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign25950_e24348;
        locals.var_guard629_rv = 0.0;

        let (assign25960_e24371,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 == 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) && (locals.var_guard629 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25960_e24371;
        locals.var_mm_rv = 0.0;

        let (assign25970_e24383,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25970_e24383;
        locals.var_m0_rv = 0.0;

        let mut assign25980_loop_guard: usize = 0;
        while {
            let assign25980_cond_e24396: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign25980_cond_e24396 != 0.0
        } {
            assign25980_loop_guard += 1;
            assert!(assign25980_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign25980_body0_e24409, assign25980_body0_e24409_d_n0, assign25980_body0_e24409_d_n2, assign25980_body0_e24409_d_n4, assign25980_body0_e24409_d_n5, assign25980_body0_e24409_d_n6, assign25980_body0_e24409_d_n7, assign25980_body0_e24409_d_n8, assign25980_body0_e24409_d_n9, assign25980_body0_e24409_d_n10, assign25980_body0_e24409_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign25980_body0_e24407: f64 = (locals.var_dnm).sqrt();
        (assign25980_body0_e24407, (locals.var_dnm_dn0 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn2 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn4 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn5 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn6 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn7 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn8 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn9 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn10 / (2.0 * assign25980_body0_e24407)), (locals.var_dnm_dn13 / (2.0 * assign25980_body0_e24407)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign25980_body0_e24409;
            locals.var_dnm_dn0 = assign25980_body0_e24409_d_n0;
            locals.var_dnm_dn2 = assign25980_body0_e24409_d_n2;
            locals.var_dnm_dn4 = assign25980_body0_e24409_d_n4;
            locals.var_dnm_dn5 = assign25980_body0_e24409_d_n5;
            locals.var_dnm_dn6 = assign25980_body0_e24409_d_n6;
            locals.var_dnm_dn7 = assign25980_body0_e24409_d_n7;
            locals.var_dnm_dn8 = assign25980_body0_e24409_d_n8;
            locals.var_dnm_dn9 = assign25980_body0_e24409_d_n9;
            locals.var_dnm_dn10 = assign25980_body0_e24409_d_n10;
            locals.var_dnm_dn13 = assign25980_body0_e24409_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign25980_body1_e24423,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign25980_body1_e24421: f64 = (locals.var_m0 + 1.0);
        (assign25980_body1_e24421,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign25980_body1_e24423;
            locals.var_m0_rv = 0.0;
        }

        let (assign25990_e24447, assign25990_e24447_d_n0, assign25990_e24447_d_n2, assign25990_e24447_d_n4, assign25990_e24447_d_n5, assign25990_e24447_d_n6, assign25990_e24447_d_n7, assign25990_e24447_d_n8, assign25990_e24447_d_n9, assign25990_e24447_d_n10, assign25990_e24447_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let (assign25990_e24445, assign25990_e24445_d_n0, assign25990_e24445_d_n2, assign25990_e24445_d_n4, assign25990_e24445_d_n5, assign25990_e24445_d_n6, assign25990_e24445_d_n7, assign25990_e24445_d_n8, assign25990_e24445_d_n9, assign25990_e24445_d_n10, assign25990_e24445_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign25990_e24442: f64 = (2.0 * 2.0);
                let assign25990_e24443: f64 = (1.0 / assign25990_e24442);
                let assign25990_e24444: f64 = (locals.var_dnm).powf(assign25990_e24443);
                (assign25990_e24444, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn0)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn2)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn4)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn5)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn6)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn7)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn8)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn9)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn10)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25990_e24443) as f64).is_finite() && ((assign25990_e24443) as f64).fract() == 0.0 { if assign25990_e24443 == 0.0 { 0.0 } else { (assign25990_e24443 * ((locals.var_dnm).powf(assign25990_e24443 - 1.0) * locals.var_dnm_dn13)) } } else { (assign25990_e24444 * (assign25990_e24443 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign25990_e24445, assign25990_e24445_d_n0, assign25990_e24445_d_n2, assign25990_e24445_d_n4, assign25990_e24445_d_n5, assign25990_e24445_d_n6, assign25990_e24445_d_n7, assign25990_e24445_d_n8, assign25990_e24445_d_n9, assign25990_e24445_d_n10, assign25990_e24445_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign25990_e24447;
        locals.var_dnm_dn0 = assign25990_e24447_d_n0;
        locals.var_dnm_dn2 = assign25990_e24447_d_n2;
        locals.var_dnm_dn4 = assign25990_e24447_d_n4;
        locals.var_dnm_dn5 = assign25990_e24447_d_n5;
        locals.var_dnm_dn6 = assign25990_e24447_d_n6;
        locals.var_dnm_dn7 = assign25990_e24447_d_n7;
        locals.var_dnm_dn8 = assign25990_e24447_d_n8;
        locals.var_dnm_dn9 = assign25990_e24447_d_n9;
        locals.var_dnm_dn10 = assign25990_e24447_d_n10;
        locals.var_dnm_dn13 = assign25990_e24447_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26000_e24459, assign26000_e24459_d_n0, assign26000_e24459_d_n2, assign26000_e24459_d_n4, assign26000_e24459_d_n5, assign26000_e24459_d_n6, assign26000_e24459_d_n7, assign26000_e24459_d_n8, assign26000_e24459_d_n9, assign26000_e24459_d_n10, assign26000_e24459_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign26000_e24457: f64 = (1.0 / locals.var_dnm);
        (assign26000_e24457, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26000_e24459;
        locals.var_dnm_dn0 = assign26000_e24459_d_n0;
        locals.var_dnm_dn2 = assign26000_e24459_d_n2;
        locals.var_dnm_dn4 = assign26000_e24459_d_n4;
        locals.var_dnm_dn5 = assign26000_e24459_d_n5;
        locals.var_dnm_dn6 = assign26000_e24459_d_n6;
        locals.var_dnm_dn7 = assign26000_e24459_d_n7;
        locals.var_dnm_dn8 = assign26000_e24459_d_n8;
        locals.var_dnm_dn9 = assign26000_e24459_d_n9;
        locals.var_dnm_dn10 = assign26000_e24459_d_n10;
        locals.var_dnm_dn13 = assign26000_e24459_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26010_e24473, assign26010_e24473_d_n0, assign26010_e24473_d_n2, assign26010_e24473_d_n4, assign26010_e24473_d_n5, assign26010_e24473_d_n6, assign26010_e24473_d_n7, assign26010_e24473_d_n8, assign26010_e24473_d_n9, assign26010_e24473_d_n10, assign26010_e24473_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign26010_e24469: f64 = (locals.var_tmf1 * 1e-8);
        let assign26010_e24471: f64 = (assign26010_e24469 * locals.var_dnm);
        (assign26010_e24471, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-8) * locals.var_dnm) + (assign26010_e24469 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign26010_e24473;
        locals.var_tmf0_dn0 = assign26010_e24473_d_n0;
        locals.var_tmf0_dn2 = assign26010_e24473_d_n2;
        locals.var_tmf0_dn4 = assign26010_e24473_d_n4;
        locals.var_tmf0_dn5 = assign26010_e24473_d_n5;
        locals.var_tmf0_dn6 = assign26010_e24473_d_n6;
        locals.var_tmf0_dn7 = assign26010_e24473_d_n7;
        locals.var_tmf0_dn8 = assign26010_e24473_d_n8;
        locals.var_tmf0_dn9 = assign26010_e24473_d_n9;
        locals.var_tmf0_dn10 = assign26010_e24473_d_n10;
        locals.var_tmf0_dn13 = assign26010_e24473_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign26020_e24489, assign26020_e24489_d_n0, assign26020_e24489_d_n2, assign26020_e24489_d_n4, assign26020_e24489_d_n5, assign26020_e24489_d_n6, assign26020_e24489_d_n7, assign26020_e24489_d_n8, assign26020_e24489_d_n9, assign26020_e24489_d_n10, assign26020_e24489_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign26020_e24483: f64 = (1e-8 * locals.var_xmp);
        let assign26020_e24485: f64 = (assign26020_e24483 * locals.var_dnm);
        let assign26020_e24487: f64 = (assign26020_e24485 / locals.var_arg);
        (assign26020_e24487, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign26020_e24483 * locals.var_dnm_dn13)) * locals.var_arg) - (assign26020_e24485 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26020_e24489;
        locals.var_t3_dn0 = assign26020_e24489_d_n0;
        locals.var_t3_dn2 = assign26020_e24489_d_n2;
        locals.var_t3_dn4 = assign26020_e24489_d_n4;
        locals.var_t3_dn5 = assign26020_e24489_d_n5;
        locals.var_t3_dn6 = assign26020_e24489_d_n6;
        locals.var_t3_dn7 = assign26020_e24489_d_n7;
        locals.var_t3_dn8 = assign26020_e24489_d_n8;
        locals.var_t3_dn9 = assign26020_e24489_d_n9;
        locals.var_t3_dn10 = assign26020_e24489_d_n10;
        locals.var_t3_dn13 = assign26020_e24489_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26030_e24503, assign26030_e24503_d_n0, assign26030_e24503_d_n2, assign26030_e24503_d_n4, assign26030_e24503_d_n5, assign26030_e24503_d_n6, assign26030_e24503_d_n7, assign26030_e24503_d_n8, assign26030_e24503_d_n9, assign26030_e24503_d_n10, assign26030_e24503_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        let assign26030_e24499: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26030_e24501: f64 = (assign26030_e24499 + locals.var_tmf0);
        (assign26030_e24501, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign26030_e24503;
        locals.var_w_b0_dn0 = assign26030_e24503_d_n0;
        locals.var_w_b0_dn2 = assign26030_e24503_d_n2;
        locals.var_w_b0_dn4 = assign26030_e24503_d_n4;
        locals.var_w_b0_dn5 = assign26030_e24503_d_n5;
        locals.var_w_b0_dn6 = assign26030_e24503_d_n6;
        locals.var_w_b0_dn7 = assign26030_e24503_d_n7;
        locals.var_w_b0_dn8 = assign26030_e24503_d_n8;
        locals.var_w_b0_dn9 = assign26030_e24503_d_n9;
        locals.var_w_b0_dn10 = assign26030_e24503_d_n10;
        locals.var_w_b0_dn13 = assign26030_e24503_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign26040_e24513, assign26040_e24513_d_n0, assign26040_e24513_d_n2, assign26040_e24513_d_n4, assign26040_e24513_d_n5, assign26040_e24513_d_n6, assign26040_e24513_d_n7, assign26040_e24513_d_n8, assign26040_e24513_d_n9, assign26040_e24513_d_n10, assign26040_e24513_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26040_e24513;
        locals.var_t3_dn0 = assign26040_e24513_d_n0;
        locals.var_t3_dn2 = assign26040_e24513_d_n2;
        locals.var_t3_dn4 = assign26040_e24513_d_n4;
        locals.var_t3_dn5 = assign26040_e24513_d_n5;
        locals.var_t3_dn6 = assign26040_e24513_d_n6;
        locals.var_t3_dn7 = assign26040_e24513_d_n7;
        locals.var_t3_dn8 = assign26040_e24513_d_n8;
        locals.var_t3_dn9 = assign26040_e24513_d_n9;
        locals.var_t3_dn10 = assign26040_e24513_d_n10;
        locals.var_t3_dn13 = assign26040_e24513_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26050_e24524, assign26050_e24524_d_n0, assign26050_e24524_d_n2, assign26050_e24524_d_n4, assign26050_e24524_d_n5, assign26050_e24524_d_n6, assign26050_e24524_d_n7, assign26050_e24524_d_n8, assign26050_e24524_d_n9, assign26050_e24524_d_n10, assign26050_e24524_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign26050_e24524;
        locals.var_w_b0_dn0 = assign26050_e24524_d_n0;
        locals.var_w_b0_dn2 = assign26050_e24524_d_n2;
        locals.var_w_b0_dn4 = assign26050_e24524_d_n4;
        locals.var_w_b0_dn5 = assign26050_e24524_d_n5;
        locals.var_w_b0_dn6 = assign26050_e24524_d_n6;
        locals.var_w_b0_dn7 = assign26050_e24524_d_n7;
        locals.var_w_b0_dn8 = assign26050_e24524_d_n8;
        locals.var_w_b0_dn9 = assign26050_e24524_d_n9;
        locals.var_w_b0_dn10 = assign26050_e24524_d_n10;
        locals.var_w_b0_dn13 = assign26050_e24524_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign26060_e24535, assign26060_e24535_d_n0, assign26060_e24535_d_n2, assign26060_e24535_d_n4, assign26060_e24535_d_n5, assign26060_e24535_d_n6, assign26060_e24535_d_n7, assign26060_e24535_d_n8, assign26060_e24535_d_n9, assign26060_e24535_d_n10, assign26060_e24535_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) && (locals.var_guard624 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26060_e24535;
        locals.var_t3_dn0 = assign26060_e24535_d_n0;
        locals.var_t3_dn2 = assign26060_e24535_d_n2;
        locals.var_t3_dn4 = assign26060_e24535_d_n4;
        locals.var_t3_dn5 = assign26060_e24535_d_n5;
        locals.var_t3_dn6 = assign26060_e24535_d_n6;
        locals.var_t3_dn7 = assign26060_e24535_d_n7;
        locals.var_t3_dn8 = assign26060_e24535_d_n8;
        locals.var_t3_dn9 = assign26060_e24535_d_n9;
        locals.var_t3_dn10 = assign26060_e24535_d_n10;
        locals.var_t3_dn13 = assign26060_e24535_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26070_e24550, assign26070_e24550_d_n0, assign26070_e24550_d_n2, assign26070_e24550_d_n4, assign26070_e24550_d_n5, assign26070_e24550_d_n6, assign26070_e24550_d_n7, assign26070_e24550_d_n8, assign26070_e24550_d_n9, assign26070_e24550_d_n10, assign26070_e24550_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign26070_e24544: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk435);
        let assign26070_e24546: f64 = (assign26070_e24544 + locals.var_vbi_dep);
        let assign26070_e24547: f64 = (locals.var_c_2esipq_nsub * assign26070_e24546);
        let assign26070_e24548: f64 = (assign26070_e24547).sqrt();
        (assign26070_e24548, (((locals.var_c_2esipq_nsub_dn0 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn2 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn4 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn5 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn6 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn7 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn8 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn9 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn10 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign26070_e24548)), (((locals.var_c_2esipq_nsub_dn13 * assign26070_e24546) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_vbi_dep_dn13))) / (2.0 * assign26070_e24548)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn13,)
    }
};
        locals.var_w_sub0 = assign26070_e24550;
        locals.var_w_sub0_dn0 = assign26070_e24550_d_n0;
        locals.var_w_sub0_dn2 = assign26070_e24550_d_n2;
        locals.var_w_sub0_dn4 = assign26070_e24550_d_n4;
        locals.var_w_sub0_dn5 = assign26070_e24550_d_n5;
        locals.var_w_sub0_dn6 = assign26070_e24550_d_n6;
        locals.var_w_sub0_dn7 = assign26070_e24550_d_n7;
        locals.var_w_sub0_dn8 = assign26070_e24550_d_n8;
        locals.var_w_sub0_dn9 = assign26070_e24550_d_n9;
        locals.var_w_sub0_dn10 = assign26070_e24550_d_n10;
        locals.var_w_sub0_dn13 = assign26070_e24550_d_n13;
        locals.var_w_sub0_rv = 0.0;

        let (assign26080_e24560, assign26080_e24560_d_n0, assign26080_e24560_d_n2, assign26080_e24560_d_n4, assign26080_e24560_d_n5, assign26080_e24560_d_n6, assign26080_e24560_d_n7, assign26080_e24560_d_n8, assign26080_e24560_d_n9, assign26080_e24560_d_n10, assign26080_e24560_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign26080_e24558: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign26080_e24558, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn13 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn13)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn13,)
    }
};
        locals.var_q_b0_dep = assign26080_e24560;
        locals.var_q_b0_dep_dn0 = assign26080_e24560_d_n0;
        locals.var_q_b0_dep_dn2 = assign26080_e24560_d_n2;
        locals.var_q_b0_dep_dn4 = assign26080_e24560_d_n4;
        locals.var_q_b0_dep_dn5 = assign26080_e24560_d_n5;
        locals.var_q_b0_dep_dn6 = assign26080_e24560_d_n6;
        locals.var_q_b0_dep_dn7 = assign26080_e24560_d_n7;
        locals.var_q_b0_dep_dn8 = assign26080_e24560_d_n8;
        locals.var_q_b0_dep_dn9 = assign26080_e24560_d_n9;
        locals.var_q_b0_dep_dn10 = assign26080_e24560_d_n10;
        locals.var_q_b0_dep_dn13 = assign26080_e24560_d_n13;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign26090_e24571, assign26090_e24571_d_n0, assign26090_e24571_d_n2, assign26090_e24571_d_n4, assign26090_e24571_d_n5, assign26090_e24571_d_n6, assign26090_e24571_d_n7, assign26090_e24571_d_n8, assign26090_e24571_d_n9, assign26090_e24571_d_n10, assign26090_e24571_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign26090_e24567: f64 = (-locals.var_w_sub0);
        let assign26090_e24569: f64 = (assign26090_e24567 * locals.var_q_nsub__blk544);
        (assign26090_e24569, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn10)), (((-locals.var_w_sub0_dn13) * locals.var_q_nsub__blk544) + (assign26090_e24567 * locals.var_q_nsub__blk544_dn13)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn13,)
    }
};
        locals.var_q_sub0_dep = assign26090_e24571;
        locals.var_q_sub0_dep_dn0 = assign26090_e24571_d_n0;
        locals.var_q_sub0_dep_dn2 = assign26090_e24571_d_n2;
        locals.var_q_sub0_dep_dn4 = assign26090_e24571_d_n4;
        locals.var_q_sub0_dep_dn5 = assign26090_e24571_d_n5;
        locals.var_q_sub0_dep_dn6 = assign26090_e24571_d_n6;
        locals.var_q_sub0_dep_dn7 = assign26090_e24571_d_n7;
        locals.var_q_sub0_dep_dn8 = assign26090_e24571_d_n8;
        locals.var_q_sub0_dep_dn9 = assign26090_e24571_d_n9;
        locals.var_q_sub0_dep_dn10 = assign26090_e24571_d_n10;
        locals.var_q_sub0_dep_dn13 = assign26090_e24571_d_n13;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign26100_e24586, assign26100_e24586_d_n0, assign26100_e24586_d_n2, assign26100_e24586_d_n4, assign26100_e24586_d_n5, assign26100_e24586_d_n6, assign26100_e24586_d_n7, assign26100_e24586_d_n8, assign26100_e24586_d_n9, assign26100_e24586_d_n10, assign26100_e24586_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26100_e24579: f64 = (-locals.var_beta);
        let assign26100_e24582: f64 = (locals.var_phi_s0_dep - locals.var_vbscl__blk435);
        let assign26100_e24583: f64 = (assign26100_e24579 * assign26100_e24582);
        let assign26100_e24584: f64 = (assign26100_e24583).exp();
        (assign26100_e24584, (assign26100_e24584 * (((-locals.var_beta_dn0) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn0 - locals.var_vbscl__blk435_dn0)))), (assign26100_e24584 * (((-locals.var_beta_dn2) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn2 - locals.var_vbscl__blk435_dn2)))), (assign26100_e24584 * (((-locals.var_beta_dn4) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn4 - locals.var_vbscl__blk435_dn4)))), (assign26100_e24584 * (((-locals.var_beta_dn5) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn5 - locals.var_vbscl__blk435_dn5)))), (assign26100_e24584 * (((-locals.var_beta_dn6) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn6 - locals.var_vbscl__blk435_dn6)))), (assign26100_e24584 * (((-locals.var_beta_dn7) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn7 - locals.var_vbscl__blk435_dn7)))), (assign26100_e24584 * (((-locals.var_beta_dn8) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn8 - locals.var_vbscl__blk435_dn8)))), (assign26100_e24584 * (((-locals.var_beta_dn9) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn9 - locals.var_vbscl__blk435_dn9)))), (assign26100_e24584 * (((-locals.var_beta_dn10) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn10 - locals.var_vbscl__blk435_dn10)))), (assign26100_e24584 * (((-locals.var_beta_dn13) * assign26100_e24582) + (assign26100_e24579 * (locals.var_phi_s0_dep_dn13 - locals.var_vbscl__blk435_dn13)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26100_e24586;
        locals.var_t3_dn0 = assign26100_e24586_d_n0;
        locals.var_t3_dn2 = assign26100_e24586_d_n2;
        locals.var_t3_dn4 = assign26100_e24586_d_n4;
        locals.var_t3_dn5 = assign26100_e24586_d_n5;
        locals.var_t3_dn6 = assign26100_e24586_d_n6;
        locals.var_t3_dn7 = assign26100_e24586_d_n7;
        locals.var_t3_dn8 = assign26100_e24586_d_n8;
        locals.var_t3_dn9 = assign26100_e24586_d_n9;
        locals.var_t3_dn10 = assign26100_e24586_d_n10;
        locals.var_t3_dn13 = assign26100_e24586_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26110_e24601, assign26110_e24601_d_n0, assign26110_e24601_d_n2, assign26110_e24601_d_n4, assign26110_e24601_d_n5, assign26110_e24601_d_n6, assign26110_e24601_d_n7, assign26110_e24601_d_n8, assign26110_e24601_d_n9, assign26110_e24601_d_n10, assign26110_e24601_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26110_e24594: f64 = (-locals.var_beta);
        let assign26110_e24597: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk435);
        let assign26110_e24598: f64 = (assign26110_e24594 * assign26110_e24597);
        let assign26110_e24599: f64 = (assign26110_e24598).exp();
        (assign26110_e24599, (assign26110_e24599 * (((-locals.var_beta_dn0) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn0 - locals.var_vbscl__blk435_dn0)))), (assign26110_e24599 * (((-locals.var_beta_dn2) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn2 - locals.var_vbscl__blk435_dn2)))), (assign26110_e24599 * (((-locals.var_beta_dn4) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn4 - locals.var_vbscl__blk435_dn4)))), (assign26110_e24599 * (((-locals.var_beta_dn5) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn5 - locals.var_vbscl__blk435_dn5)))), (assign26110_e24599 * (((-locals.var_beta_dn6) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn6 - locals.var_vbscl__blk435_dn6)))), (assign26110_e24599 * (((-locals.var_beta_dn7) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn7 - locals.var_vbscl__blk435_dn7)))), (assign26110_e24599 * (((-locals.var_beta_dn8) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn8 - locals.var_vbscl__blk435_dn8)))), (assign26110_e24599 * (((-locals.var_beta_dn9) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn9 - locals.var_vbscl__blk435_dn9)))), (assign26110_e24599 * (((-locals.var_beta_dn10) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn10 - locals.var_vbscl__blk435_dn10)))), (assign26110_e24599 * (((-locals.var_beta_dn13) * assign26110_e24597) + (assign26110_e24594 * (locals.var_phi_b0_dep_dn13 - locals.var_vbscl__blk435_dn13)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign26110_e24601;
        locals.var_t4_dn0 = assign26110_e24601_d_n0;
        locals.var_t4_dn2 = assign26110_e24601_d_n2;
        locals.var_t4_dn4 = assign26110_e24601_d_n4;
        locals.var_t4_dn5 = assign26110_e24601_d_n5;
        locals.var_t4_dn6 = assign26110_e24601_d_n6;
        locals.var_t4_dn7 = assign26110_e24601_d_n7;
        locals.var_t4_dn8 = assign26110_e24601_d_n8;
        locals.var_t4_dn9 = assign26110_e24601_d_n9;
        locals.var_t4_dn10 = assign26110_e24601_d_n10;
        locals.var_t4_dn13 = assign26110_e24601_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign26120_e24625, assign26120_e24625_d_n0, assign26120_e24625_d_n2, assign26120_e24625_d_n4, assign26120_e24625_d_n5, assign26120_e24625_d_n6, assign26120_e24625_d_n7, assign26120_e24625_d_n8, assign26120_e24625_d_n9, assign26120_e24625_d_n10, assign26120_e24625_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26120_e24611: f64 = (locals.var_t2 - 1.0);
        let assign26120_e24613: f64 = (assign26120_e24611 - locals.var_t1);
        let assign26120_e24617: f64 = (locals.var_t3 - locals.var_t4);
        let assign26120_e24618: f64 = (locals.var_cnst1 * assign26120_e24617);
        let assign26120_e24619: f64 = (assign26120_e24613 + assign26120_e24618);
        let assign26120_e24621: f64 = (assign26120_e24619 + 1e-15);
        let assign26120_e24622: f64 = (assign26120_e24621).sqrt();
        let assign26120_e24623: f64 = (locals.var_cnst0 * assign26120_e24622);
        (assign26120_e24623, ((locals.var_cnst0_dn0 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn2 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn4 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn5 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn6 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn7 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn8 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn9 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn10 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign26120_e24622)))), ((locals.var_cnst0_dn13 * assign26120_e24622) + (locals.var_cnst0 * (((locals.var_t2_dn13 - locals.var_t1_dn13) + ((locals.var_cnst1_dn13 * assign26120_e24617) + (locals.var_cnst1 * (locals.var_t3_dn13 - locals.var_t4_dn13)))) / (2.0 * assign26120_e24622)))),)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn13,)
    }
};
        locals.var_q_s0 = assign26120_e24625;
        locals.var_q_s0_dn0 = assign26120_e24625_d_n0;
        locals.var_q_s0_dn2 = assign26120_e24625_d_n2;
        locals.var_q_s0_dn4 = assign26120_e24625_d_n4;
        locals.var_q_s0_dn5 = assign26120_e24625_d_n5;
        locals.var_q_s0_dn6 = assign26120_e24625_d_n6;
        locals.var_q_s0_dn7 = assign26120_e24625_d_n7;
        locals.var_q_s0_dn8 = assign26120_e24625_d_n8;
        locals.var_q_s0_dn9 = assign26120_e24625_d_n9;
        locals.var_q_s0_dn10 = assign26120_e24625_d_n10;
        locals.var_q_s0_dn13 = assign26120_e24625_d_n13;
        locals.var_q_s0_rv = 0.0;

        let assign26130_e24632: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard630 = assign26130_e24632;
        locals.var_guard630_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        locals: &mut StampLocals,
    ) {
        let (assign26140_e24643, assign26140_e24643_d_n0, assign26140_e24643_d_n2, assign26140_e24643_d_n4, assign26140_e24643_d_n5, assign26140_e24643_d_n6, assign26140_e24643_d_n7, assign26140_e24643_d_n8, assign26140_e24643_d_n9, assign26140_e24643_d_n10, assign26140_e24643_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard630 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn13,)
    }
};
        locals.var_q_sub0 = assign26140_e24643;
        locals.var_q_sub0_dn0 = assign26140_e24643_d_n0;
        locals.var_q_sub0_dn2 = assign26140_e24643_d_n2;
        locals.var_q_sub0_dn4 = assign26140_e24643_d_n4;
        locals.var_q_sub0_dn5 = assign26140_e24643_d_n5;
        locals.var_q_sub0_dn6 = assign26140_e24643_d_n6;
        locals.var_q_sub0_dn7 = assign26140_e24643_d_n7;
        locals.var_q_sub0_dn8 = assign26140_e24643_d_n8;
        locals.var_q_sub0_dn9 = assign26140_e24643_d_n9;
        locals.var_q_sub0_dn10 = assign26140_e24643_d_n10;
        locals.var_q_sub0_dn13 = assign26140_e24643_d_n13;
        locals.var_q_sub0_rv = 0.0;

        let (assign26150_e24654, assign26150_e24654_d_n0, assign26150_e24654_d_n2, assign26150_e24654_d_n4, assign26150_e24654_d_n5, assign26150_e24654_d_n6, assign26150_e24654_d_n7, assign26150_e24654_d_n8, assign26150_e24654_d_n9, assign26150_e24654_d_n10, assign26150_e24654_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard630 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn13,)
    }
};
        locals.var_q_s0_dep = assign26150_e24654;
        locals.var_q_s0_dep_dn0 = assign26150_e24654_d_n0;
        locals.var_q_s0_dep_dn2 = assign26150_e24654_d_n2;
        locals.var_q_s0_dep_dn4 = assign26150_e24654_d_n4;
        locals.var_q_s0_dep_dn5 = assign26150_e24654_d_n5;
        locals.var_q_s0_dep_dn6 = assign26150_e24654_d_n6;
        locals.var_q_s0_dep_dn7 = assign26150_e24654_d_n7;
        locals.var_q_s0_dep_dn8 = assign26150_e24654_d_n8;
        locals.var_q_s0_dep_dn9 = assign26150_e24654_d_n9;
        locals.var_q_s0_dep_dn10 = assign26150_e24654_d_n10;
        locals.var_q_s0_dep_dn13 = assign26150_e24654_d_n13;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign26160_e24688, assign26160_e24688_d_n0, assign26160_e24688_d_n2, assign26160_e24688_d_n4, assign26160_e24688_d_n5, assign26160_e24688_d_n6, assign26160_e24688_d_n7, assign26160_e24688_d_n8, assign26160_e24688_d_n9, assign26160_e24688_d_n10, assign26160_e24688_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign26160_e24666: f64 = (-locals.var_t1);
        let assign26160_e24669: f64 = (-locals.var_beta);
        let assign26160_e24672: f64 = (locals.var_phi_s0_dep - locals.var_vbscl__blk435);
        let assign26160_e24673: f64 = (assign26160_e24669 * assign26160_e24672);
        let assign26160_e24674: f64 = (assign26160_e24673).exp();
        let assign26160_e24676: f64 = (-locals.var_beta);
        let assign26160_e24679: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk435);
        let assign26160_e24680: f64 = (assign26160_e24676 * assign26160_e24679);
        let assign26160_e24681: f64 = (assign26160_e24680).exp();
        let assign26160_e24682: f64 = (assign26160_e24674 - assign26160_e24681);
        let assign26160_e24683: f64 = (locals.var_cnst1 * assign26160_e24682);
        let assign26160_e24684: f64 = (assign26160_e24666 + assign26160_e24683);
        let assign26160_e24685: f64 = (assign26160_e24684).sqrt();
        let assign26160_e24686: f64 = (locals.var_cnst0 * assign26160_e24685);
        (assign26160_e24686, ((locals.var_cnst0_dn0 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn0) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn0 - locals.var_vbscl__blk435_dn0)))) - (assign26160_e24681 * (((-locals.var_beta_dn0) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn0 - locals.var_vbscl__blk435_dn0)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn2 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn2) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn2 - locals.var_vbscl__blk435_dn2)))) - (assign26160_e24681 * (((-locals.var_beta_dn2) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn2 - locals.var_vbscl__blk435_dn2)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn4 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn4) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn4 - locals.var_vbscl__blk435_dn4)))) - (assign26160_e24681 * (((-locals.var_beta_dn4) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn4 - locals.var_vbscl__blk435_dn4)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn5 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn5) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn5 - locals.var_vbscl__blk435_dn5)))) - (assign26160_e24681 * (((-locals.var_beta_dn5) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn5 - locals.var_vbscl__blk435_dn5)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn6 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn6) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn6 - locals.var_vbscl__blk435_dn6)))) - (assign26160_e24681 * (((-locals.var_beta_dn6) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn6 - locals.var_vbscl__blk435_dn6)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn7 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn7) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn7 - locals.var_vbscl__blk435_dn7)))) - (assign26160_e24681 * (((-locals.var_beta_dn7) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn7 - locals.var_vbscl__blk435_dn7)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn8 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn8) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn8 - locals.var_vbscl__blk435_dn8)))) - (assign26160_e24681 * (((-locals.var_beta_dn8) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn8 - locals.var_vbscl__blk435_dn8)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn9 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn9) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn9 - locals.var_vbscl__blk435_dn9)))) - (assign26160_e24681 * (((-locals.var_beta_dn9) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn9 - locals.var_vbscl__blk435_dn9)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn10 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn10) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn10 - locals.var_vbscl__blk435_dn10)))) - (assign26160_e24681 * (((-locals.var_beta_dn10) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn10 - locals.var_vbscl__blk435_dn10)))))))) / (2.0 * assign26160_e24685)))), ((locals.var_cnst0_dn13 * assign26160_e24685) + (locals.var_cnst0 * (((-locals.var_t1_dn13) + ((locals.var_cnst1_dn13 * assign26160_e24682) + (locals.var_cnst1 * ((assign26160_e24674 * (((-locals.var_beta_dn13) * assign26160_e24672) + (assign26160_e24669 * (locals.var_phi_s0_dep_dn13 - locals.var_vbscl__blk435_dn13)))) - (assign26160_e24681 * (((-locals.var_beta_dn13) * assign26160_e24679) + (assign26160_e24676 * (locals.var_phi_b0_dep_dn13 - locals.var_vbscl__blk435_dn13)))))))) / (2.0 * assign26160_e24685)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26160_e24688;
        locals.var_t3_dn0 = assign26160_e24688_d_n0;
        locals.var_t3_dn2 = assign26160_e24688_d_n2;
        locals.var_t3_dn4 = assign26160_e24688_d_n4;
        locals.var_t3_dn5 = assign26160_e24688_d_n5;
        locals.var_t3_dn6 = assign26160_e24688_d_n6;
        locals.var_t3_dn7 = assign26160_e24688_d_n7;
        locals.var_t3_dn8 = assign26160_e24688_d_n8;
        locals.var_t3_dn9 = assign26160_e24688_d_n9;
        locals.var_t3_dn10 = assign26160_e24688_d_n10;
        locals.var_t3_dn13 = assign26160_e24688_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26170_e24706, assign26170_e24706_d_n0, assign26170_e24706_d_n2, assign26170_e24706_d_n4, assign26170_e24706_d_n5, assign26170_e24706_d_n6, assign26170_e24706_d_n7, assign26170_e24706_d_n8, assign26170_e24706_d_n9, assign26170_e24706_d_n10, assign26170_e24706_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign26170_e24701: f64 = (-locals.var_t1);
        let assign26170_e24702: f64 = (assign26170_e24701).sqrt();
        let assign26170_e24703: f64 = (locals.var_cnst0 * assign26170_e24702);
        let assign26170_e24704: f64 = (locals.var_t3 - assign26170_e24703);
        (assign26170_e24704, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign26170_e24702))))), (locals.var_t3_dn13 - ((locals.var_cnst0_dn13 * assign26170_e24702) + (locals.var_cnst0 * ((-locals.var_t1_dn13) / (2.0 * assign26170_e24702))))),)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn13,)
    }
};
        locals.var_q_sub0 = assign26170_e24706;
        locals.var_q_sub0_dn0 = assign26170_e24706_d_n0;
        locals.var_q_sub0_dn2 = assign26170_e24706_d_n2;
        locals.var_q_sub0_dn4 = assign26170_e24706_d_n4;
        locals.var_q_sub0_dn5 = assign26170_e24706_d_n5;
        locals.var_q_sub0_dn6 = assign26170_e24706_d_n6;
        locals.var_q_sub0_dn7 = assign26170_e24706_d_n7;
        locals.var_q_sub0_dn8 = assign26170_e24706_d_n8;
        locals.var_q_sub0_dn9 = assign26170_e24706_d_n9;
        locals.var_q_sub0_dn10 = assign26170_e24706_d_n10;
        locals.var_q_sub0_dn13 = assign26170_e24706_d_n13;
        locals.var_q_sub0_rv = 0.0;

        let (assign26180_e24727, assign26180_e24727_d_n0, assign26180_e24727_d_n2, assign26180_e24727_d_n4, assign26180_e24727_d_n5, assign26180_e24727_d_n6, assign26180_e24727_d_n7, assign26180_e24727_d_n8, assign26180_e24727_d_n9, assign26180_e24727_d_n10, assign26180_e24727_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard630 == 0.0)) {
        let assign26180_e24719: f64 = (locals.var_t2 - 1.0);
        let assign26180_e24721: f64 = (assign26180_e24719 - locals.var_t1);
        let assign26180_e24723: f64 = (assign26180_e24721 + 1e-15);
        let assign26180_e24724: f64 = (assign26180_e24723).sqrt();
        let assign26180_e24725: f64 = (locals.var_cnst0 * assign26180_e24724);
        (assign26180_e24725, ((locals.var_cnst0_dn0 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn2 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn4 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn5 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn6 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn7 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn8 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn9 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn10 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign26180_e24724)))), ((locals.var_cnst0_dn13 * assign26180_e24724) + (locals.var_cnst0 * ((locals.var_t2_dn13 - locals.var_t1_dn13) / (2.0 * assign26180_e24724)))),)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn13,)
    }
};
        locals.var_q_s0_dep = assign26180_e24727;
        locals.var_q_s0_dep_dn0 = assign26180_e24727_d_n0;
        locals.var_q_s0_dep_dn2 = assign26180_e24727_d_n2;
        locals.var_q_s0_dep_dn4 = assign26180_e24727_d_n4;
        locals.var_q_s0_dep_dn5 = assign26180_e24727_d_n5;
        locals.var_q_s0_dep_dn6 = assign26180_e24727_d_n6;
        locals.var_q_s0_dep_dn7 = assign26180_e24727_d_n7;
        locals.var_q_s0_dep_dn8 = assign26180_e24727_d_n8;
        locals.var_q_s0_dep_dn9 = assign26180_e24727_d_n9;
        locals.var_q_s0_dep_dn10 = assign26180_e24727_d_n10;
        locals.var_q_s0_dep_dn13 = assign26180_e24727_d_n13;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign26190_e24736, assign26190_e24736_d_n0, assign26190_e24736_d_n2, assign26190_e24736_d_n4, assign26190_e24736_d_n5, assign26190_e24736_d_n6, assign26190_e24736_d_n7, assign26190_e24736_d_n8, assign26190_e24736_d_n9, assign26190_e24736_d_n10, assign26190_e24736_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_n0__blk538, locals.var_q_n0__blk538_dn0, locals.var_q_n0__blk538_dn2, locals.var_q_n0__blk538_dn4, locals.var_q_n0__blk538_dn5, locals.var_q_n0__blk538_dn6, locals.var_q_n0__blk538_dn7, locals.var_q_n0__blk538_dn8, locals.var_q_n0__blk538_dn9, locals.var_q_n0__blk538_dn10, locals.var_q_n0__blk538_dn13,)
    }
};
        locals.var_q_n0__blk538 = assign26190_e24736;
        locals.var_q_n0__blk538_dn0 = assign26190_e24736_d_n0;
        locals.var_q_n0__blk538_dn2 = assign26190_e24736_d_n2;
        locals.var_q_n0__blk538_dn4 = assign26190_e24736_d_n4;
        locals.var_q_n0__blk538_dn5 = assign26190_e24736_d_n5;
        locals.var_q_n0__blk538_dn6 = assign26190_e24736_d_n6;
        locals.var_q_n0__blk538_dn7 = assign26190_e24736_d_n7;
        locals.var_q_n0__blk538_dn8 = assign26190_e24736_d_n8;
        locals.var_q_n0__blk538_dn9 = assign26190_e24736_d_n9;
        locals.var_q_n0__blk538_dn10 = assign26190_e24736_d_n10;
        locals.var_q_n0__blk538_dn13 = assign26190_e24736_d_n13;
        locals.var_q_n0__blk538_rv = 0.0;

        let (assign26200_e24747, assign26200_e24747_d_n0, assign26200_e24747_d_n2, assign26200_e24747_d_n4, assign26200_e24747_d_n5, assign26200_e24747_d_n6, assign26200_e24747_d_n7, assign26200_e24747_d_n8, assign26200_e24747_d_n9, assign26200_e24747_d_n10, assign26200_e24747_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26200_e24745: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        (assign26200_e24745, (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10), (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign26200_e24747;
        locals.var_t1_dn0 = assign26200_e24747_d_n0;
        locals.var_t1_dn2 = assign26200_e24747_d_n2;
        locals.var_t1_dn4 = assign26200_e24747_d_n4;
        locals.var_t1_dn5 = assign26200_e24747_d_n5;
        locals.var_t1_dn6 = assign26200_e24747_d_n6;
        locals.var_t1_dn7 = assign26200_e24747_d_n7;
        locals.var_t1_dn8 = assign26200_e24747_d_n8;
        locals.var_t1_dn9 = assign26200_e24747_d_n9;
        locals.var_t1_dn10 = assign26200_e24747_d_n10;
        locals.var_t1_dn13 = assign26200_e24747_d_n13;
        locals.var_t1_rv = 0.0;

        let assign26210_e24751: f64 = 0.1;
        let assign26210_e24756: f64 = if ((locals.var_t1 < assign26210_e24751) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard631 = assign26210_e24756;
        locals.var_guard631_rv = 0.0;

        let (assign26220_e24771, assign26220_e24771_d_n0, assign26220_e24771_d_n2, assign26220_e24771_d_n4, assign26220_e24771_d_n5, assign26220_e24771_d_n6, assign26220_e24771_d_n7, assign26220_e24771_d_n8, assign26220_e24771_d_n9, assign26220_e24771_d_n10, assign26220_e24771_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26220_e24767: f64 = 0.1;
        let assign26220_e24769: f64 = (assign26220_e24767 - locals.var_t1);
        (assign26220_e24769, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign26220_e24771;
        locals.var_tmf1_dn0 = assign26220_e24771_d_n0;
        locals.var_tmf1_dn2 = assign26220_e24771_d_n2;
        locals.var_tmf1_dn4 = assign26220_e24771_d_n4;
        locals.var_tmf1_dn5 = assign26220_e24771_d_n5;
        locals.var_tmf1_dn6 = assign26220_e24771_d_n6;
        locals.var_tmf1_dn7 = assign26220_e24771_d_n7;
        locals.var_tmf1_dn8 = assign26220_e24771_d_n8;
        locals.var_tmf1_dn9 = assign26220_e24771_d_n9;
        locals.var_tmf1_dn10 = assign26220_e24771_d_n10;
        locals.var_tmf1_dn13 = assign26220_e24771_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign26230_e24784, assign26230_e24784_d_n0, assign26230_e24784_d_n2, assign26230_e24784_d_n4, assign26230_e24784_d_n5, assign26230_e24784_d_n6, assign26230_e24784_d_n7, assign26230_e24784_d_n8, assign26230_e24784_d_n9, assign26230_e24784_d_n10, assign26230_e24784_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26230_e24782: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26230_e24782, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign26230_e24784;
        locals.var_x2_dn0 = assign26230_e24784_d_n0;
        locals.var_x2_dn2 = assign26230_e24784_d_n2;
        locals.var_x2_dn4 = assign26230_e24784_d_n4;
        locals.var_x2_dn5 = assign26230_e24784_d_n5;
        locals.var_x2_dn6 = assign26230_e24784_d_n6;
        locals.var_x2_dn7 = assign26230_e24784_d_n7;
        locals.var_x2_dn8 = assign26230_e24784_d_n8;
        locals.var_x2_dn9 = assign26230_e24784_d_n9;
        locals.var_x2_dn10 = assign26230_e24784_d_n10;
        locals.var_x2_dn13 = assign26230_e24784_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign26240_e24797, assign26240_e24797_d_n0, assign26240_e24797_d_n2, assign26240_e24797_d_n4, assign26240_e24797_d_n5, assign26240_e24797_d_n6, assign26240_e24797_d_n7, assign26240_e24797_d_n8, assign26240_e24797_d_n9, assign26240_e24797_d_n10, assign26240_e24797_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26240_e24795: f64 = (0.1 * 0.1);
        (assign26240_e24795, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign26240_e24797;
        locals.var_xmax2_dn0 = assign26240_e24797_d_n0;
        locals.var_xmax2_dn2 = assign26240_e24797_d_n2;
        locals.var_xmax2_dn4 = assign26240_e24797_d_n4;
        locals.var_xmax2_dn5 = assign26240_e24797_d_n5;
        locals.var_xmax2_dn6 = assign26240_e24797_d_n6;
        locals.var_xmax2_dn7 = assign26240_e24797_d_n7;
        locals.var_xmax2_dn8 = assign26240_e24797_d_n8;
        locals.var_xmax2_dn9 = assign26240_e24797_d_n9;
        locals.var_xmax2_dn10 = assign26240_e24797_d_n10;
        locals.var_xmax2_dn13 = assign26240_e24797_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign26250_e24808, assign26250_e24808_d_n0, assign26250_e24808_d_n2, assign26250_e24808_d_n4, assign26250_e24808_d_n5, assign26250_e24808_d_n6, assign26250_e24808_d_n7, assign26250_e24808_d_n8, assign26250_e24808_d_n9, assign26250_e24808_d_n10, assign26250_e24808_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign26250_e24808;
        locals.var_xp_dn0 = assign26250_e24808_d_n0;
        locals.var_xp_dn2 = assign26250_e24808_d_n2;
        locals.var_xp_dn4 = assign26250_e24808_d_n4;
        locals.var_xp_dn5 = assign26250_e24808_d_n5;
        locals.var_xp_dn6 = assign26250_e24808_d_n6;
        locals.var_xp_dn7 = assign26250_e24808_d_n7;
        locals.var_xp_dn8 = assign26250_e24808_d_n8;
        locals.var_xp_dn9 = assign26250_e24808_d_n9;
        locals.var_xp_dn10 = assign26250_e24808_d_n10;
        locals.var_xp_dn13 = assign26250_e24808_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign26260_e24819, assign26260_e24819_d_n0, assign26260_e24819_d_n2, assign26260_e24819_d_n4, assign26260_e24819_d_n5, assign26260_e24819_d_n6, assign26260_e24819_d_n7, assign26260_e24819_d_n8, assign26260_e24819_d_n9, assign26260_e24819_d_n10, assign26260_e24819_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign26260_e24819;
        locals.var_xmp_dn0 = assign26260_e24819_d_n0;
        locals.var_xmp_dn2 = assign26260_e24819_d_n2;
        locals.var_xmp_dn4 = assign26260_e24819_d_n4;
        locals.var_xmp_dn5 = assign26260_e24819_d_n5;
        locals.var_xmp_dn6 = assign26260_e24819_d_n6;
        locals.var_xmp_dn7 = assign26260_e24819_d_n7;
        locals.var_xmp_dn8 = assign26260_e24819_d_n8;
        locals.var_xmp_dn9 = assign26260_e24819_d_n9;
        locals.var_xmp_dn10 = assign26260_e24819_d_n10;
        locals.var_xmp_dn13 = assign26260_e24819_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign26270_e24830,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26270_e24830;
        locals.var_m0_rv = 0.0;

        let (assign26280_e24841,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26280_e24841;
        locals.var_mm_rv = 0.0;

        let (assign26290_e24852, assign26290_e24852_d_n0, assign26290_e24852_d_n2, assign26290_e24852_d_n4, assign26290_e24852_d_n5, assign26290_e24852_d_n6, assign26290_e24852_d_n7, assign26290_e24852_d_n8, assign26290_e24852_d_n9, assign26290_e24852_d_n10, assign26290_e24852_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign26290_e24852;
        locals.var_arg_dn0 = assign26290_e24852_d_n0;
        locals.var_arg_dn2 = assign26290_e24852_d_n2;
        locals.var_arg_dn4 = assign26290_e24852_d_n4;
        locals.var_arg_dn5 = assign26290_e24852_d_n5;
        locals.var_arg_dn6 = assign26290_e24852_d_n6;
        locals.var_arg_dn7 = assign26290_e24852_d_n7;
        locals.var_arg_dn8 = assign26290_e24852_d_n8;
        locals.var_arg_dn9 = assign26290_e24852_d_n9;
        locals.var_arg_dn10 = assign26290_e24852_d_n10;
        locals.var_arg_dn13 = assign26290_e24852_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign26300_e24863, assign26300_e24863_d_n0, assign26300_e24863_d_n2, assign26300_e24863_d_n4, assign26300_e24863_d_n5, assign26300_e24863_d_n6, assign26300_e24863_d_n7, assign26300_e24863_d_n8, assign26300_e24863_d_n9, assign26300_e24863_d_n10, assign26300_e24863_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26300_e24863;
        locals.var_dnm_dn0 = assign26300_e24863_d_n0;
        locals.var_dnm_dn2 = assign26300_e24863_d_n2;
        locals.var_dnm_dn4 = assign26300_e24863_d_n4;
        locals.var_dnm_dn5 = assign26300_e24863_d_n5;
        locals.var_dnm_dn6 = assign26300_e24863_d_n6;
        locals.var_dnm_dn7 = assign26300_e24863_d_n7;
        locals.var_dnm_dn8 = assign26300_e24863_d_n8;
        locals.var_dnm_dn9 = assign26300_e24863_d_n9;
        locals.var_dnm_dn10 = assign26300_e24863_d_n10;
        locals.var_dnm_dn13 = assign26300_e24863_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26310_e24876, assign26310_e24876_d_n0, assign26310_e24876_d_n2, assign26310_e24876_d_n4, assign26310_e24876_d_n5, assign26310_e24876_d_n6, assign26310_e24876_d_n7, assign26310_e24876_d_n8, assign26310_e24876_d_n9, assign26310_e24876_d_n10, assign26310_e24876_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26310_e24874: f64 = (locals.var_xp * locals.var_x2);
        (assign26310_e24874, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign26310_e24876;
        locals.var_xp_dn0 = assign26310_e24876_d_n0;
        locals.var_xp_dn2 = assign26310_e24876_d_n2;
        locals.var_xp_dn4 = assign26310_e24876_d_n4;
        locals.var_xp_dn5 = assign26310_e24876_d_n5;
        locals.var_xp_dn6 = assign26310_e24876_d_n6;
        locals.var_xp_dn7 = assign26310_e24876_d_n7;
        locals.var_xp_dn8 = assign26310_e24876_d_n8;
        locals.var_xp_dn9 = assign26310_e24876_d_n9;
        locals.var_xp_dn10 = assign26310_e24876_d_n10;
        locals.var_xp_dn13 = assign26310_e24876_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign26320_e24889, assign26320_e24889_d_n0, assign26320_e24889_d_n2, assign26320_e24889_d_n4, assign26320_e24889_d_n5, assign26320_e24889_d_n6, assign26320_e24889_d_n7, assign26320_e24889_d_n8, assign26320_e24889_d_n9, assign26320_e24889_d_n10, assign26320_e24889_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26320_e24887: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26320_e24887, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign26320_e24889;
        locals.var_xmp_dn0 = assign26320_e24889_d_n0;
        locals.var_xmp_dn2 = assign26320_e24889_d_n2;
        locals.var_xmp_dn4 = assign26320_e24889_d_n4;
        locals.var_xmp_dn5 = assign26320_e24889_d_n5;
        locals.var_xmp_dn6 = assign26320_e24889_d_n6;
        locals.var_xmp_dn7 = assign26320_e24889_d_n7;
        locals.var_xmp_dn8 = assign26320_e24889_d_n8;
        locals.var_xmp_dn9 = assign26320_e24889_d_n9;
        locals.var_xmp_dn10 = assign26320_e24889_d_n10;
        locals.var_xmp_dn13 = assign26320_e24889_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign26330_e24902, assign26330_e24902_d_n0, assign26330_e24902_d_n2, assign26330_e24902_d_n4, assign26330_e24902_d_n5, assign26330_e24902_d_n6, assign26330_e24902_d_n7, assign26330_e24902_d_n8, assign26330_e24902_d_n9, assign26330_e24902_d_n10, assign26330_e24902_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26330_e24900: f64 = (locals.var_xp * locals.var_x2);
        (assign26330_e24900, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign26330_e24902;
        locals.var_xp_dn0 = assign26330_e24902_d_n0;
        locals.var_xp_dn2 = assign26330_e24902_d_n2;
        locals.var_xp_dn4 = assign26330_e24902_d_n4;
        locals.var_xp_dn5 = assign26330_e24902_d_n5;
        locals.var_xp_dn6 = assign26330_e24902_d_n6;
        locals.var_xp_dn7 = assign26330_e24902_d_n7;
        locals.var_xp_dn8 = assign26330_e24902_d_n8;
        locals.var_xp_dn9 = assign26330_e24902_d_n9;
        locals.var_xp_dn10 = assign26330_e24902_d_n10;
        locals.var_xp_dn13 = assign26330_e24902_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign26340_e24915, assign26340_e24915_d_n0, assign26340_e24915_d_n2, assign26340_e24915_d_n4, assign26340_e24915_d_n5, assign26340_e24915_d_n6, assign26340_e24915_d_n7, assign26340_e24915_d_n8, assign26340_e24915_d_n9, assign26340_e24915_d_n10, assign26340_e24915_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26340_e24913: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26340_e24913, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign26340_e24915;
        locals.var_xmp_dn0 = assign26340_e24915_d_n0;
        locals.var_xmp_dn2 = assign26340_e24915_d_n2;
        locals.var_xmp_dn4 = assign26340_e24915_d_n4;
        locals.var_xmp_dn5 = assign26340_e24915_d_n5;
        locals.var_xmp_dn6 = assign26340_e24915_d_n6;
        locals.var_xmp_dn7 = assign26340_e24915_d_n7;
        locals.var_xmp_dn8 = assign26340_e24915_d_n8;
        locals.var_xmp_dn9 = assign26340_e24915_d_n9;
        locals.var_xmp_dn10 = assign26340_e24915_d_n10;
        locals.var_xmp_dn13 = assign26340_e24915_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign26350_e24928, assign26350_e24928_d_n0, assign26350_e24928_d_n2, assign26350_e24928_d_n4, assign26350_e24928_d_n5, assign26350_e24928_d_n6, assign26350_e24928_d_n7, assign26350_e24928_d_n8, assign26350_e24928_d_n9, assign26350_e24928_d_n10, assign26350_e24928_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26350_e24926: f64 = (locals.var_xp + locals.var_xmp);
        (assign26350_e24926, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign26350_e24928;
        locals.var_arg_dn0 = assign26350_e24928_d_n0;
        locals.var_arg_dn2 = assign26350_e24928_d_n2;
        locals.var_arg_dn4 = assign26350_e24928_d_n4;
        locals.var_arg_dn5 = assign26350_e24928_d_n5;
        locals.var_arg_dn6 = assign26350_e24928_d_n6;
        locals.var_arg_dn7 = assign26350_e24928_d_n7;
        locals.var_arg_dn8 = assign26350_e24928_d_n8;
        locals.var_arg_dn9 = assign26350_e24928_d_n9;
        locals.var_arg_dn10 = assign26350_e24928_d_n10;
        locals.var_arg_dn13 = assign26350_e24928_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign26360_e24939, assign26360_e24939_d_n0, assign26360_e24939_d_n2, assign26360_e24939_d_n4, assign26360_e24939_d_n5, assign26360_e24939_d_n6, assign26360_e24939_d_n7, assign26360_e24939_d_n8, assign26360_e24939_d_n9, assign26360_e24939_d_n10, assign26360_e24939_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26360_e24939;
        locals.var_dnm_dn0 = assign26360_e24939_d_n0;
        locals.var_dnm_dn2 = assign26360_e24939_d_n2;
        locals.var_dnm_dn4 = assign26360_e24939_d_n4;
        locals.var_dnm_dn5 = assign26360_e24939_d_n5;
        locals.var_dnm_dn6 = assign26360_e24939_d_n6;
        locals.var_dnm_dn7 = assign26360_e24939_d_n7;
        locals.var_dnm_dn8 = assign26360_e24939_d_n8;
        locals.var_dnm_dn9 = assign26360_e24939_d_n9;
        locals.var_dnm_dn10 = assign26360_e24939_d_n10;
        locals.var_dnm_dn13 = assign26360_e24939_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign26370_e24954: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard632 = assign26370_e24954;
        locals.var_guard632_rv = 0.0;

        let assign26380_e24957: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign26380_e24957;
        locals.var_guard633_rv = 0.0;

        let (assign26390_e24972,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26390_e24972;
        locals.var_mm_rv = 0.0;

        let assign26400_e24975: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign26400_e24975;
        locals.var_guard634_rv = 0.0;

        let (assign26410_e24993,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26410_e24993;
        locals.var_mm_rv = 0.0;

        let assign26420_e24996: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign26420_e24996;
        locals.var_guard635_rv = 0.0;

        let (assign26430_e25017,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) && (locals.var_guard635 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26430_e25017;
        locals.var_mm_rv = 0.0;

        let assign26440_e25020: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign26440_e25020;
        locals.var_guard636_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign26450_e25044,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) && (locals.var_guard634 == 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26450_e25044;
        locals.var_mm_rv = 0.0;

        let (assign26460_e25057,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26460_e25057;
        locals.var_m0_rv = 0.0;

        let mut assign26470_loop_guard: usize = 0;
        while {
            let assign26470_cond_e25071: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26470_cond_e25071 != 0.0
        } {
            assign26470_loop_guard += 1;
            assert!(assign26470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26470_body0_e25085, assign26470_body0_e25085_d_n0, assign26470_body0_e25085_d_n2, assign26470_body0_e25085_d_n4, assign26470_body0_e25085_d_n5, assign26470_body0_e25085_d_n6, assign26470_body0_e25085_d_n7, assign26470_body0_e25085_d_n8, assign26470_body0_e25085_d_n9, assign26470_body0_e25085_d_n10, assign26470_body0_e25085_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) {
        let assign26470_body0_e25083: f64 = (locals.var_dnm).sqrt();
        (assign26470_body0_e25083, (locals.var_dnm_dn0 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn2 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn4 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn5 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn6 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn7 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn8 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn9 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn10 / (2.0 * assign26470_body0_e25083)), (locals.var_dnm_dn13 / (2.0 * assign26470_body0_e25083)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign26470_body0_e25085;
            locals.var_dnm_dn0 = assign26470_body0_e25085_d_n0;
            locals.var_dnm_dn2 = assign26470_body0_e25085_d_n2;
            locals.var_dnm_dn4 = assign26470_body0_e25085_d_n4;
            locals.var_dnm_dn5 = assign26470_body0_e25085_d_n5;
            locals.var_dnm_dn6 = assign26470_body0_e25085_d_n6;
            locals.var_dnm_dn7 = assign26470_body0_e25085_d_n7;
            locals.var_dnm_dn8 = assign26470_body0_e25085_d_n8;
            locals.var_dnm_dn9 = assign26470_body0_e25085_d_n9;
            locals.var_dnm_dn10 = assign26470_body0_e25085_d_n10;
            locals.var_dnm_dn13 = assign26470_body0_e25085_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign26470_body1_e25100,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 != 0.0)) {
        let assign26470_body1_e25098: f64 = (locals.var_m0 + 1.0);
        (assign26470_body1_e25098,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26470_body1_e25100;
            locals.var_m0_rv = 0.0;
        }

        let (assign26480_e25125, assign26480_e25125_d_n0, assign26480_e25125_d_n2, assign26480_e25125_d_n4, assign26480_e25125_d_n5, assign26480_e25125_d_n6, assign26480_e25125_d_n7, assign26480_e25125_d_n8, assign26480_e25125_d_n9, assign26480_e25125_d_n10, assign26480_e25125_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) && (locals.var_guard632 == 0.0)) {
        let (assign26480_e25123, assign26480_e25123_d_n0, assign26480_e25123_d_n2, assign26480_e25123_d_n4, assign26480_e25123_d_n5, assign26480_e25123_d_n6, assign26480_e25123_d_n7, assign26480_e25123_d_n8, assign26480_e25123_d_n9, assign26480_e25123_d_n10, assign26480_e25123_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26480_e25120: f64 = (2.0 * 2.0);
                let assign26480_e25121: f64 = (1.0 / assign26480_e25120);
                let assign26480_e25122: f64 = (locals.var_dnm).powf(assign26480_e25121);
                (assign26480_e25122, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26480_e25121) as f64).is_finite() && ((assign26480_e25121) as f64).fract() == 0.0 { if assign26480_e25121 == 0.0 { 0.0 } else { (assign26480_e25121 * ((locals.var_dnm).powf(assign26480_e25121 - 1.0) * locals.var_dnm_dn13)) } } else { (assign26480_e25122 * (assign26480_e25121 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign26480_e25123, assign26480_e25123_d_n0, assign26480_e25123_d_n2, assign26480_e25123_d_n4, assign26480_e25123_d_n5, assign26480_e25123_d_n6, assign26480_e25123_d_n7, assign26480_e25123_d_n8, assign26480_e25123_d_n9, assign26480_e25123_d_n10, assign26480_e25123_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26480_e25125;
        locals.var_dnm_dn0 = assign26480_e25125_d_n0;
        locals.var_dnm_dn2 = assign26480_e25125_d_n2;
        locals.var_dnm_dn4 = assign26480_e25125_d_n4;
        locals.var_dnm_dn5 = assign26480_e25125_d_n5;
        locals.var_dnm_dn6 = assign26480_e25125_d_n6;
        locals.var_dnm_dn7 = assign26480_e25125_d_n7;
        locals.var_dnm_dn8 = assign26480_e25125_d_n8;
        locals.var_dnm_dn9 = assign26480_e25125_d_n9;
        locals.var_dnm_dn10 = assign26480_e25125_d_n10;
        locals.var_dnm_dn13 = assign26480_e25125_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26490_e25138, assign26490_e25138_d_n0, assign26490_e25138_d_n2, assign26490_e25138_d_n4, assign26490_e25138_d_n5, assign26490_e25138_d_n6, assign26490_e25138_d_n7, assign26490_e25138_d_n8, assign26490_e25138_d_n9, assign26490_e25138_d_n10, assign26490_e25138_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26490_e25136: f64 = (1.0 / locals.var_dnm);
        (assign26490_e25136, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26490_e25138;
        locals.var_dnm_dn0 = assign26490_e25138_d_n0;
        locals.var_dnm_dn2 = assign26490_e25138_d_n2;
        locals.var_dnm_dn4 = assign26490_e25138_d_n4;
        locals.var_dnm_dn5 = assign26490_e25138_d_n5;
        locals.var_dnm_dn6 = assign26490_e25138_d_n6;
        locals.var_dnm_dn7 = assign26490_e25138_d_n7;
        locals.var_dnm_dn8 = assign26490_e25138_d_n8;
        locals.var_dnm_dn9 = assign26490_e25138_d_n9;
        locals.var_dnm_dn10 = assign26490_e25138_d_n10;
        locals.var_dnm_dn13 = assign26490_e25138_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26500_e25153, assign26500_e25153_d_n0, assign26500_e25153_d_n2, assign26500_e25153_d_n4, assign26500_e25153_d_n5, assign26500_e25153_d_n6, assign26500_e25153_d_n7, assign26500_e25153_d_n8, assign26500_e25153_d_n9, assign26500_e25153_d_n10, assign26500_e25153_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26500_e25149: f64 = (locals.var_tmf1 * 0.1);
        let assign26500_e25151: f64 = (assign26500_e25149 * locals.var_dnm);
        (assign26500_e25151, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign26500_e25149 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign26500_e25153;
        locals.var_tmf0_dn0 = assign26500_e25153_d_n0;
        locals.var_tmf0_dn2 = assign26500_e25153_d_n2;
        locals.var_tmf0_dn4 = assign26500_e25153_d_n4;
        locals.var_tmf0_dn5 = assign26500_e25153_d_n5;
        locals.var_tmf0_dn6 = assign26500_e25153_d_n6;
        locals.var_tmf0_dn7 = assign26500_e25153_d_n7;
        locals.var_tmf0_dn8 = assign26500_e25153_d_n8;
        locals.var_tmf0_dn9 = assign26500_e25153_d_n9;
        locals.var_tmf0_dn10 = assign26500_e25153_d_n10;
        locals.var_tmf0_dn13 = assign26500_e25153_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign26510_e25170, assign26510_e25170_d_n0, assign26510_e25170_d_n2, assign26510_e25170_d_n4, assign26510_e25170_d_n5, assign26510_e25170_d_n6, assign26510_e25170_d_n7, assign26510_e25170_d_n8, assign26510_e25170_d_n9, assign26510_e25170_d_n10, assign26510_e25170_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26510_e25164: f64 = (0.1 * locals.var_xmp);
        let assign26510_e25166: f64 = (assign26510_e25164 * locals.var_dnm);
        let assign26510_e25168: f64 = (assign26510_e25166 / locals.var_arg);
        (assign26510_e25168, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign26510_e25164 * locals.var_dnm_dn13)) * locals.var_arg) - (assign26510_e25166 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign26510_e25170;
        locals.var_t0_dn0 = assign26510_e25170_d_n0;
        locals.var_t0_dn2 = assign26510_e25170_d_n2;
        locals.var_t0_dn4 = assign26510_e25170_d_n4;
        locals.var_t0_dn5 = assign26510_e25170_d_n5;
        locals.var_t0_dn6 = assign26510_e25170_d_n6;
        locals.var_t0_dn7 = assign26510_e25170_d_n7;
        locals.var_t0_dn8 = assign26510_e25170_d_n8;
        locals.var_t0_dn9 = assign26510_e25170_d_n9;
        locals.var_t0_dn10 = assign26510_e25170_d_n10;
        locals.var_t0_dn13 = assign26510_e25170_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign26520_e25185, assign26520_e25185_d_n0, assign26520_e25185_d_n2, assign26520_e25185_d_n4, assign26520_e25185_d_n5, assign26520_e25185_d_n6, assign26520_e25185_d_n7, assign26520_e25185_d_n8, assign26520_e25185_d_n9, assign26520_e25185_d_n10, assign26520_e25185_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign26520_e25181: f64 = 0.1;
        let assign26520_e25183: f64 = (assign26520_e25181 - locals.var_tmf0);
        (assign26520_e25183, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign26520_e25185;
        locals.var_t2_dn0 = assign26520_e25185_d_n0;
        locals.var_t2_dn2 = assign26520_e25185_d_n2;
        locals.var_t2_dn4 = assign26520_e25185_d_n4;
        locals.var_t2_dn5 = assign26520_e25185_d_n5;
        locals.var_t2_dn6 = assign26520_e25185_d_n6;
        locals.var_t2_dn7 = assign26520_e25185_d_n7;
        locals.var_t2_dn8 = assign26520_e25185_d_n8;
        locals.var_t2_dn9 = assign26520_e25185_d_n9;
        locals.var_t2_dn10 = assign26520_e25185_d_n10;
        locals.var_t2_dn13 = assign26520_e25185_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign26530_e25196, assign26530_e25196_d_n0, assign26530_e25196_d_n2, assign26530_e25196_d_n4, assign26530_e25196_d_n5, assign26530_e25196_d_n6, assign26530_e25196_d_n7, assign26530_e25196_d_n8, assign26530_e25196_d_n9, assign26530_e25196_d_n10, assign26530_e25196_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign26530_e25196;
        locals.var_t0_dn0 = assign26530_e25196_d_n0;
        locals.var_t0_dn2 = assign26530_e25196_d_n2;
        locals.var_t0_dn4 = assign26530_e25196_d_n4;
        locals.var_t0_dn5 = assign26530_e25196_d_n5;
        locals.var_t0_dn6 = assign26530_e25196_d_n6;
        locals.var_t0_dn7 = assign26530_e25196_d_n7;
        locals.var_t0_dn8 = assign26530_e25196_d_n8;
        locals.var_t0_dn9 = assign26530_e25196_d_n9;
        locals.var_t0_dn10 = assign26530_e25196_d_n10;
        locals.var_t0_dn13 = assign26530_e25196_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign26540_e25208, assign26540_e25208_d_n0, assign26540_e25208_d_n2, assign26540_e25208_d_n4, assign26540_e25208_d_n5, assign26540_e25208_d_n6, assign26540_e25208_d_n7, assign26540_e25208_d_n8, assign26540_e25208_d_n9, assign26540_e25208_d_n10, assign26540_e25208_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign26540_e25208;
        locals.var_t2_dn0 = assign26540_e25208_d_n0;
        locals.var_t2_dn2 = assign26540_e25208_d_n2;
        locals.var_t2_dn4 = assign26540_e25208_d_n4;
        locals.var_t2_dn5 = assign26540_e25208_d_n5;
        locals.var_t2_dn6 = assign26540_e25208_d_n6;
        locals.var_t2_dn7 = assign26540_e25208_d_n7;
        locals.var_t2_dn8 = assign26540_e25208_d_n8;
        locals.var_t2_dn9 = assign26540_e25208_d_n9;
        locals.var_t2_dn10 = assign26540_e25208_d_n10;
        locals.var_t2_dn13 = assign26540_e25208_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign26550_e25220, assign26550_e25220_d_n0, assign26550_e25220_d_n2, assign26550_e25220_d_n4, assign26550_e25220_d_n5, assign26550_e25220_d_n6, assign26550_e25220_d_n7, assign26550_e25220_d_n8, assign26550_e25220_d_n9, assign26550_e25220_d_n10, assign26550_e25220_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard631 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign26550_e25220;
        locals.var_t0_dn0 = assign26550_e25220_d_n0;
        locals.var_t0_dn2 = assign26550_e25220_d_n2;
        locals.var_t0_dn4 = assign26550_e25220_d_n4;
        locals.var_t0_dn5 = assign26550_e25220_d_n5;
        locals.var_t0_dn6 = assign26550_e25220_d_n6;
        locals.var_t0_dn7 = assign26550_e25220_d_n7;
        locals.var_t0_dn8 = assign26550_e25220_d_n8;
        locals.var_t0_dn9 = assign26550_e25220_d_n9;
        locals.var_t0_dn10 = assign26550_e25220_d_n10;
        locals.var_t0_dn13 = assign26550_e25220_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign26560_e25232, assign26560_e25232_d_n0, assign26560_e25232_d_n2, assign26560_e25232_d_n4, assign26560_e25232_d_n5, assign26560_e25232_d_n6, assign26560_e25232_d_n7, assign26560_e25232_d_n8, assign26560_e25232_d_n9, assign26560_e25232_d_n10, assign26560_e25232_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26560_e25229: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign26560_e25230: f64 = (assign26560_e25229).sqrt();
        (assign26560_e25230, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign26560_e25230)), (((locals.var_c_2esipq_ndepm_dn13 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn13)) / (2.0 * assign26560_e25230)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign26560_e25232;
        locals.var_w_b0_dn0 = assign26560_e25232_d_n0;
        locals.var_w_b0_dn2 = assign26560_e25232_d_n2;
        locals.var_w_b0_dn4 = assign26560_e25232_d_n4;
        locals.var_w_b0_dn5 = assign26560_e25232_d_n5;
        locals.var_w_b0_dn6 = assign26560_e25232_d_n6;
        locals.var_w_b0_dn7 = assign26560_e25232_d_n7;
        locals.var_w_b0_dn8 = assign26560_e25232_d_n8;
        locals.var_w_b0_dn9 = assign26560_e25232_d_n9;
        locals.var_w_b0_dn10 = assign26560_e25232_d_n10;
        locals.var_w_b0_dn13 = assign26560_e25232_d_n13;
        locals.var_w_b0_rv = 0.0;

        let assign26570_e25236: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26570_e25241: f64 = if ((locals.var_w_b0 > assign26570_e25236) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard637 = assign26570_e25241;
        locals.var_guard637_rv = 0.0;

        let (assign26580_e25256, assign26580_e25256_d_n0, assign26580_e25256_d_n2, assign26580_e25256_d_n4, assign26580_e25256_d_n5, assign26580_e25256_d_n6, assign26580_e25256_d_n7, assign26580_e25256_d_n8, assign26580_e25256_d_n9, assign26580_e25256_d_n10, assign26580_e25256_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26580_e25252: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign26580_e25254: f64 = (assign26580_e25252 + 1e-8);
        (assign26580_e25254, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn13 - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign26580_e25256;
        locals.var_tmf1_dn0 = assign26580_e25256_d_n0;
        locals.var_tmf1_dn2 = assign26580_e25256_d_n2;
        locals.var_tmf1_dn4 = assign26580_e25256_d_n4;
        locals.var_tmf1_dn5 = assign26580_e25256_d_n5;
        locals.var_tmf1_dn6 = assign26580_e25256_d_n6;
        locals.var_tmf1_dn7 = assign26580_e25256_d_n7;
        locals.var_tmf1_dn8 = assign26580_e25256_d_n8;
        locals.var_tmf1_dn9 = assign26580_e25256_d_n9;
        locals.var_tmf1_dn10 = assign26580_e25256_d_n10;
        locals.var_tmf1_dn13 = assign26580_e25256_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign26590_e25269, assign26590_e25269_d_n0, assign26590_e25269_d_n2, assign26590_e25269_d_n4, assign26590_e25269_d_n5, assign26590_e25269_d_n6, assign26590_e25269_d_n7, assign26590_e25269_d_n8, assign26590_e25269_d_n9, assign26590_e25269_d_n10, assign26590_e25269_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26590_e25267: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26590_e25267, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign26590_e25269;
        locals.var_x2_dn0 = assign26590_e25269_d_n0;
        locals.var_x2_dn2 = assign26590_e25269_d_n2;
        locals.var_x2_dn4 = assign26590_e25269_d_n4;
        locals.var_x2_dn5 = assign26590_e25269_d_n5;
        locals.var_x2_dn6 = assign26590_e25269_d_n6;
        locals.var_x2_dn7 = assign26590_e25269_d_n7;
        locals.var_x2_dn8 = assign26590_e25269_d_n8;
        locals.var_x2_dn9 = assign26590_e25269_d_n9;
        locals.var_x2_dn10 = assign26590_e25269_d_n10;
        locals.var_x2_dn13 = assign26590_e25269_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign26600_e25282, assign26600_e25282_d_n0, assign26600_e25282_d_n2, assign26600_e25282_d_n4, assign26600_e25282_d_n5, assign26600_e25282_d_n6, assign26600_e25282_d_n7, assign26600_e25282_d_n8, assign26600_e25282_d_n9, assign26600_e25282_d_n10, assign26600_e25282_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26600_e25280: f64 = (1e-8 * 1e-8);
        (assign26600_e25280, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign26600_e25282;
        locals.var_xmax2_dn0 = assign26600_e25282_d_n0;
        locals.var_xmax2_dn2 = assign26600_e25282_d_n2;
        locals.var_xmax2_dn4 = assign26600_e25282_d_n4;
        locals.var_xmax2_dn5 = assign26600_e25282_d_n5;
        locals.var_xmax2_dn6 = assign26600_e25282_d_n6;
        locals.var_xmax2_dn7 = assign26600_e25282_d_n7;
        locals.var_xmax2_dn8 = assign26600_e25282_d_n8;
        locals.var_xmax2_dn9 = assign26600_e25282_d_n9;
        locals.var_xmax2_dn10 = assign26600_e25282_d_n10;
        locals.var_xmax2_dn13 = assign26600_e25282_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign26610_e25293, assign26610_e25293_d_n0, assign26610_e25293_d_n2, assign26610_e25293_d_n4, assign26610_e25293_d_n5, assign26610_e25293_d_n6, assign26610_e25293_d_n7, assign26610_e25293_d_n8, assign26610_e25293_d_n9, assign26610_e25293_d_n10, assign26610_e25293_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign26610_e25293;
        locals.var_xp_dn0 = assign26610_e25293_d_n0;
        locals.var_xp_dn2 = assign26610_e25293_d_n2;
        locals.var_xp_dn4 = assign26610_e25293_d_n4;
        locals.var_xp_dn5 = assign26610_e25293_d_n5;
        locals.var_xp_dn6 = assign26610_e25293_d_n6;
        locals.var_xp_dn7 = assign26610_e25293_d_n7;
        locals.var_xp_dn8 = assign26610_e25293_d_n8;
        locals.var_xp_dn9 = assign26610_e25293_d_n9;
        locals.var_xp_dn10 = assign26610_e25293_d_n10;
        locals.var_xp_dn13 = assign26610_e25293_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign26620_e25304, assign26620_e25304_d_n0, assign26620_e25304_d_n2, assign26620_e25304_d_n4, assign26620_e25304_d_n5, assign26620_e25304_d_n6, assign26620_e25304_d_n7, assign26620_e25304_d_n8, assign26620_e25304_d_n9, assign26620_e25304_d_n10, assign26620_e25304_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign26620_e25304;
        locals.var_xmp_dn0 = assign26620_e25304_d_n0;
        locals.var_xmp_dn2 = assign26620_e25304_d_n2;
        locals.var_xmp_dn4 = assign26620_e25304_d_n4;
        locals.var_xmp_dn5 = assign26620_e25304_d_n5;
        locals.var_xmp_dn6 = assign26620_e25304_d_n6;
        locals.var_xmp_dn7 = assign26620_e25304_d_n7;
        locals.var_xmp_dn8 = assign26620_e25304_d_n8;
        locals.var_xmp_dn9 = assign26620_e25304_d_n9;
        locals.var_xmp_dn10 = assign26620_e25304_d_n10;
        locals.var_xmp_dn13 = assign26620_e25304_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign26630_e25315,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26630_e25315;
        locals.var_m0_rv = 0.0;

        let (assign26640_e25326,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26640_e25326;
        locals.var_mm_rv = 0.0;

        let (assign26650_e25337, assign26650_e25337_d_n0, assign26650_e25337_d_n2, assign26650_e25337_d_n4, assign26650_e25337_d_n5, assign26650_e25337_d_n6, assign26650_e25337_d_n7, assign26650_e25337_d_n8, assign26650_e25337_d_n9, assign26650_e25337_d_n10, assign26650_e25337_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign26650_e25337;
        locals.var_arg_dn0 = assign26650_e25337_d_n0;
        locals.var_arg_dn2 = assign26650_e25337_d_n2;
        locals.var_arg_dn4 = assign26650_e25337_d_n4;
        locals.var_arg_dn5 = assign26650_e25337_d_n5;
        locals.var_arg_dn6 = assign26650_e25337_d_n6;
        locals.var_arg_dn7 = assign26650_e25337_d_n7;
        locals.var_arg_dn8 = assign26650_e25337_d_n8;
        locals.var_arg_dn9 = assign26650_e25337_d_n9;
        locals.var_arg_dn10 = assign26650_e25337_d_n10;
        locals.var_arg_dn13 = assign26650_e25337_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign26660_e25348, assign26660_e25348_d_n0, assign26660_e25348_d_n2, assign26660_e25348_d_n4, assign26660_e25348_d_n5, assign26660_e25348_d_n6, assign26660_e25348_d_n7, assign26660_e25348_d_n8, assign26660_e25348_d_n9, assign26660_e25348_d_n10, assign26660_e25348_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26660_e25348;
        locals.var_dnm_dn0 = assign26660_e25348_d_n0;
        locals.var_dnm_dn2 = assign26660_e25348_d_n2;
        locals.var_dnm_dn4 = assign26660_e25348_d_n4;
        locals.var_dnm_dn5 = assign26660_e25348_d_n5;
        locals.var_dnm_dn6 = assign26660_e25348_d_n6;
        locals.var_dnm_dn7 = assign26660_e25348_d_n7;
        locals.var_dnm_dn8 = assign26660_e25348_d_n8;
        locals.var_dnm_dn9 = assign26660_e25348_d_n9;
        locals.var_dnm_dn10 = assign26660_e25348_d_n10;
        locals.var_dnm_dn13 = assign26660_e25348_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26670_e25361, assign26670_e25361_d_n0, assign26670_e25361_d_n2, assign26670_e25361_d_n4, assign26670_e25361_d_n5, assign26670_e25361_d_n6, assign26670_e25361_d_n7, assign26670_e25361_d_n8, assign26670_e25361_d_n9, assign26670_e25361_d_n10, assign26670_e25361_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26670_e25359: f64 = (locals.var_xp * locals.var_x2);
        (assign26670_e25359, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign26670_e25361;
        locals.var_xp_dn0 = assign26670_e25361_d_n0;
        locals.var_xp_dn2 = assign26670_e25361_d_n2;
        locals.var_xp_dn4 = assign26670_e25361_d_n4;
        locals.var_xp_dn5 = assign26670_e25361_d_n5;
        locals.var_xp_dn6 = assign26670_e25361_d_n6;
        locals.var_xp_dn7 = assign26670_e25361_d_n7;
        locals.var_xp_dn8 = assign26670_e25361_d_n8;
        locals.var_xp_dn9 = assign26670_e25361_d_n9;
        locals.var_xp_dn10 = assign26670_e25361_d_n10;
        locals.var_xp_dn13 = assign26670_e25361_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign26680_e25374, assign26680_e25374_d_n0, assign26680_e25374_d_n2, assign26680_e25374_d_n4, assign26680_e25374_d_n5, assign26680_e25374_d_n6, assign26680_e25374_d_n7, assign26680_e25374_d_n8, assign26680_e25374_d_n9, assign26680_e25374_d_n10, assign26680_e25374_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26680_e25372: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26680_e25372, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign26680_e25374;
        locals.var_xmp_dn0 = assign26680_e25374_d_n0;
        locals.var_xmp_dn2 = assign26680_e25374_d_n2;
        locals.var_xmp_dn4 = assign26680_e25374_d_n4;
        locals.var_xmp_dn5 = assign26680_e25374_d_n5;
        locals.var_xmp_dn6 = assign26680_e25374_d_n6;
        locals.var_xmp_dn7 = assign26680_e25374_d_n7;
        locals.var_xmp_dn8 = assign26680_e25374_d_n8;
        locals.var_xmp_dn9 = assign26680_e25374_d_n9;
        locals.var_xmp_dn10 = assign26680_e25374_d_n10;
        locals.var_xmp_dn13 = assign26680_e25374_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign26690_e25387, assign26690_e25387_d_n0, assign26690_e25387_d_n2, assign26690_e25387_d_n4, assign26690_e25387_d_n5, assign26690_e25387_d_n6, assign26690_e25387_d_n7, assign26690_e25387_d_n8, assign26690_e25387_d_n9, assign26690_e25387_d_n10, assign26690_e25387_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26690_e25385: f64 = (locals.var_xp * locals.var_x2);
        (assign26690_e25385, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign26690_e25387;
        locals.var_xp_dn0 = assign26690_e25387_d_n0;
        locals.var_xp_dn2 = assign26690_e25387_d_n2;
        locals.var_xp_dn4 = assign26690_e25387_d_n4;
        locals.var_xp_dn5 = assign26690_e25387_d_n5;
        locals.var_xp_dn6 = assign26690_e25387_d_n6;
        locals.var_xp_dn7 = assign26690_e25387_d_n7;
        locals.var_xp_dn8 = assign26690_e25387_d_n8;
        locals.var_xp_dn9 = assign26690_e25387_d_n9;
        locals.var_xp_dn10 = assign26690_e25387_d_n10;
        locals.var_xp_dn13 = assign26690_e25387_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign26700_e25400, assign26700_e25400_d_n0, assign26700_e25400_d_n2, assign26700_e25400_d_n4, assign26700_e25400_d_n5, assign26700_e25400_d_n6, assign26700_e25400_d_n7, assign26700_e25400_d_n8, assign26700_e25400_d_n9, assign26700_e25400_d_n10, assign26700_e25400_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26700_e25398: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26700_e25398, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign26700_e25400;
        locals.var_xmp_dn0 = assign26700_e25400_d_n0;
        locals.var_xmp_dn2 = assign26700_e25400_d_n2;
        locals.var_xmp_dn4 = assign26700_e25400_d_n4;
        locals.var_xmp_dn5 = assign26700_e25400_d_n5;
        locals.var_xmp_dn6 = assign26700_e25400_d_n6;
        locals.var_xmp_dn7 = assign26700_e25400_d_n7;
        locals.var_xmp_dn8 = assign26700_e25400_d_n8;
        locals.var_xmp_dn9 = assign26700_e25400_d_n9;
        locals.var_xmp_dn10 = assign26700_e25400_d_n10;
        locals.var_xmp_dn13 = assign26700_e25400_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign26710_e25413, assign26710_e25413_d_n0, assign26710_e25413_d_n2, assign26710_e25413_d_n4, assign26710_e25413_d_n5, assign26710_e25413_d_n6, assign26710_e25413_d_n7, assign26710_e25413_d_n8, assign26710_e25413_d_n9, assign26710_e25413_d_n10, assign26710_e25413_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26710_e25411: f64 = (locals.var_xp + locals.var_xmp);
        (assign26710_e25411, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign26710_e25413;
        locals.var_arg_dn0 = assign26710_e25413_d_n0;
        locals.var_arg_dn2 = assign26710_e25413_d_n2;
        locals.var_arg_dn4 = assign26710_e25413_d_n4;
        locals.var_arg_dn5 = assign26710_e25413_d_n5;
        locals.var_arg_dn6 = assign26710_e25413_d_n6;
        locals.var_arg_dn7 = assign26710_e25413_d_n7;
        locals.var_arg_dn8 = assign26710_e25413_d_n8;
        locals.var_arg_dn9 = assign26710_e25413_d_n9;
        locals.var_arg_dn10 = assign26710_e25413_d_n10;
        locals.var_arg_dn13 = assign26710_e25413_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign26720_e25424, assign26720_e25424_d_n0, assign26720_e25424_d_n2, assign26720_e25424_d_n4, assign26720_e25424_d_n5, assign26720_e25424_d_n6, assign26720_e25424_d_n7, assign26720_e25424_d_n8, assign26720_e25424_d_n9, assign26720_e25424_d_n10, assign26720_e25424_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26720_e25424;
        locals.var_dnm_dn0 = assign26720_e25424_d_n0;
        locals.var_dnm_dn2 = assign26720_e25424_d_n2;
        locals.var_dnm_dn4 = assign26720_e25424_d_n4;
        locals.var_dnm_dn5 = assign26720_e25424_d_n5;
        locals.var_dnm_dn6 = assign26720_e25424_d_n6;
        locals.var_dnm_dn7 = assign26720_e25424_d_n7;
        locals.var_dnm_dn8 = assign26720_e25424_d_n8;
        locals.var_dnm_dn9 = assign26720_e25424_d_n9;
        locals.var_dnm_dn10 = assign26720_e25424_d_n10;
        locals.var_dnm_dn13 = assign26720_e25424_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign26730_e25439: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard638 = assign26730_e25439;
        locals.var_guard638_rv = 0.0;

        let assign26740_e25442: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign26740_e25442;
        locals.var_guard639_rv = 0.0;

        let (assign26750_e25457,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26750_e25457;
        locals.var_mm_rv = 0.0;

        let assign26760_e25460: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign26760_e25460;
        locals.var_guard640_rv = 0.0;

        let (assign26770_e25478,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26770_e25478;
        locals.var_mm_rv = 0.0;

        let assign26780_e25481: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign26780_e25481;
        locals.var_guard641_rv = 0.0;

        let (assign26790_e25502,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26790_e25502;
        locals.var_mm_rv = 0.0;

        let assign26800_e25505: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign26800_e25505;
        locals.var_guard642_rv = 0.0;

        let (assign26810_e25529,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 == 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26810_e25529;
        locals.var_mm_rv = 0.0;

        let (assign26820_e25542,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26820_e25542;
        locals.var_m0_rv = 0.0;

        let mut assign26830_loop_guard: usize = 0;
        while {
            let assign26830_cond_e25556: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26830_cond_e25556 != 0.0
        } {
            assign26830_loop_guard += 1;
            assert!(assign26830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26830_body0_e25570, assign26830_body0_e25570_d_n0, assign26830_body0_e25570_d_n2, assign26830_body0_e25570_d_n4, assign26830_body0_e25570_d_n5, assign26830_body0_e25570_d_n6, assign26830_body0_e25570_d_n7, assign26830_body0_e25570_d_n8, assign26830_body0_e25570_d_n9, assign26830_body0_e25570_d_n10, assign26830_body0_e25570_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) {
        let assign26830_body0_e25568: f64 = (locals.var_dnm).sqrt();
        (assign26830_body0_e25568, (locals.var_dnm_dn0 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn2 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn4 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn5 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn6 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn7 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn8 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn9 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn10 / (2.0 * assign26830_body0_e25568)), (locals.var_dnm_dn13 / (2.0 * assign26830_body0_e25568)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign26830_body0_e25570;
            locals.var_dnm_dn0 = assign26830_body0_e25570_d_n0;
            locals.var_dnm_dn2 = assign26830_body0_e25570_d_n2;
            locals.var_dnm_dn4 = assign26830_body0_e25570_d_n4;
            locals.var_dnm_dn5 = assign26830_body0_e25570_d_n5;
            locals.var_dnm_dn6 = assign26830_body0_e25570_d_n6;
            locals.var_dnm_dn7 = assign26830_body0_e25570_d_n7;
            locals.var_dnm_dn8 = assign26830_body0_e25570_d_n8;
            locals.var_dnm_dn9 = assign26830_body0_e25570_d_n9;
            locals.var_dnm_dn10 = assign26830_body0_e25570_d_n10;
            locals.var_dnm_dn13 = assign26830_body0_e25570_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign26830_body1_e25585,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) {
        let assign26830_body1_e25583: f64 = (locals.var_m0 + 1.0);
        (assign26830_body1_e25583,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26830_body1_e25585;
            locals.var_m0_rv = 0.0;
        }

        let (assign26840_e25610, assign26840_e25610_d_n0, assign26840_e25610_d_n2, assign26840_e25610_d_n4, assign26840_e25610_d_n5, assign26840_e25610_d_n6, assign26840_e25610_d_n7, assign26840_e25610_d_n8, assign26840_e25610_d_n9, assign26840_e25610_d_n10, assign26840_e25610_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 == 0.0)) {
        let (assign26840_e25608, assign26840_e25608_d_n0, assign26840_e25608_d_n2, assign26840_e25608_d_n4, assign26840_e25608_d_n5, assign26840_e25608_d_n6, assign26840_e25608_d_n7, assign26840_e25608_d_n8, assign26840_e25608_d_n9, assign26840_e25608_d_n10, assign26840_e25608_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26840_e25605: f64 = (2.0 * 2.0);
                let assign26840_e25606: f64 = (1.0 / assign26840_e25605);
                let assign26840_e25607: f64 = (locals.var_dnm).powf(assign26840_e25606);
                (assign26840_e25607, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26840_e25606) as f64).is_finite() && ((assign26840_e25606) as f64).fract() == 0.0 { if assign26840_e25606 == 0.0 { 0.0 } else { (assign26840_e25606 * ((locals.var_dnm).powf(assign26840_e25606 - 1.0) * locals.var_dnm_dn13)) } } else { (assign26840_e25607 * (assign26840_e25606 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign26840_e25608, assign26840_e25608_d_n0, assign26840_e25608_d_n2, assign26840_e25608_d_n4, assign26840_e25608_d_n5, assign26840_e25608_d_n6, assign26840_e25608_d_n7, assign26840_e25608_d_n8, assign26840_e25608_d_n9, assign26840_e25608_d_n10, assign26840_e25608_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26840_e25610;
        locals.var_dnm_dn0 = assign26840_e25610_d_n0;
        locals.var_dnm_dn2 = assign26840_e25610_d_n2;
        locals.var_dnm_dn4 = assign26840_e25610_d_n4;
        locals.var_dnm_dn5 = assign26840_e25610_d_n5;
        locals.var_dnm_dn6 = assign26840_e25610_d_n6;
        locals.var_dnm_dn7 = assign26840_e25610_d_n7;
        locals.var_dnm_dn8 = assign26840_e25610_d_n8;
        locals.var_dnm_dn9 = assign26840_e25610_d_n9;
        locals.var_dnm_dn10 = assign26840_e25610_d_n10;
        locals.var_dnm_dn13 = assign26840_e25610_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26850_e25623, assign26850_e25623_d_n0, assign26850_e25623_d_n2, assign26850_e25623_d_n4, assign26850_e25623_d_n5, assign26850_e25623_d_n6, assign26850_e25623_d_n7, assign26850_e25623_d_n8, assign26850_e25623_d_n9, assign26850_e25623_d_n10, assign26850_e25623_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26850_e25621: f64 = (1.0 / locals.var_dnm);
        (assign26850_e25621, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign26850_e25623;
        locals.var_dnm_dn0 = assign26850_e25623_d_n0;
        locals.var_dnm_dn2 = assign26850_e25623_d_n2;
        locals.var_dnm_dn4 = assign26850_e25623_d_n4;
        locals.var_dnm_dn5 = assign26850_e25623_d_n5;
        locals.var_dnm_dn6 = assign26850_e25623_d_n6;
        locals.var_dnm_dn7 = assign26850_e25623_d_n7;
        locals.var_dnm_dn8 = assign26850_e25623_d_n8;
        locals.var_dnm_dn9 = assign26850_e25623_d_n9;
        locals.var_dnm_dn10 = assign26850_e25623_d_n10;
        locals.var_dnm_dn13 = assign26850_e25623_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign26860_e25638, assign26860_e25638_d_n0, assign26860_e25638_d_n2, assign26860_e25638_d_n4, assign26860_e25638_d_n5, assign26860_e25638_d_n6, assign26860_e25638_d_n7, assign26860_e25638_d_n8, assign26860_e25638_d_n9, assign26860_e25638_d_n10, assign26860_e25638_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26860_e25634: f64 = (locals.var_tmf1 * 1e-8);
        let assign26860_e25636: f64 = (assign26860_e25634 * locals.var_dnm);
        (assign26860_e25636, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-8) * locals.var_dnm) + (assign26860_e25634 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign26860_e25638;
        locals.var_tmf0_dn0 = assign26860_e25638_d_n0;
        locals.var_tmf0_dn2 = assign26860_e25638_d_n2;
        locals.var_tmf0_dn4 = assign26860_e25638_d_n4;
        locals.var_tmf0_dn5 = assign26860_e25638_d_n5;
        locals.var_tmf0_dn6 = assign26860_e25638_d_n6;
        locals.var_tmf0_dn7 = assign26860_e25638_d_n7;
        locals.var_tmf0_dn8 = assign26860_e25638_d_n8;
        locals.var_tmf0_dn9 = assign26860_e25638_d_n9;
        locals.var_tmf0_dn10 = assign26860_e25638_d_n10;
        locals.var_tmf0_dn13 = assign26860_e25638_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign26870_e25655, assign26870_e25655_d_n0, assign26870_e25655_d_n2, assign26870_e25655_d_n4, assign26870_e25655_d_n5, assign26870_e25655_d_n6, assign26870_e25655_d_n7, assign26870_e25655_d_n8, assign26870_e25655_d_n9, assign26870_e25655_d_n10, assign26870_e25655_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26870_e25649: f64 = (1e-8 * locals.var_xmp);
        let assign26870_e25651: f64 = (assign26870_e25649 * locals.var_dnm);
        let assign26870_e25653: f64 = (assign26870_e25651 / locals.var_arg);
        (assign26870_e25653, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign26870_e25649 * locals.var_dnm_dn13)) * locals.var_arg) - (assign26870_e25651 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26870_e25655;
        locals.var_t3_dn0 = assign26870_e25655_d_n0;
        locals.var_t3_dn2 = assign26870_e25655_d_n2;
        locals.var_t3_dn4 = assign26870_e25655_d_n4;
        locals.var_t3_dn5 = assign26870_e25655_d_n5;
        locals.var_t3_dn6 = assign26870_e25655_d_n6;
        locals.var_t3_dn7 = assign26870_e25655_d_n7;
        locals.var_t3_dn8 = assign26870_e25655_d_n8;
        locals.var_t3_dn9 = assign26870_e25655_d_n9;
        locals.var_t3_dn10 = assign26870_e25655_d_n10;
        locals.var_t3_dn13 = assign26870_e25655_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26880_e25670, assign26880_e25670_d_n0, assign26880_e25670_d_n2, assign26880_e25670_d_n4, assign26880_e25670_d_n5, assign26880_e25670_d_n6, assign26880_e25670_d_n7, assign26880_e25670_d_n8, assign26880_e25670_d_n9, assign26880_e25670_d_n10, assign26880_e25670_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign26880_e25666: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26880_e25668: f64 = (assign26880_e25666 + locals.var_tmf0);
        (assign26880_e25668, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign26880_e25670;
        locals.var_w_b0_dn0 = assign26880_e25670_d_n0;
        locals.var_w_b0_dn2 = assign26880_e25670_d_n2;
        locals.var_w_b0_dn4 = assign26880_e25670_d_n4;
        locals.var_w_b0_dn5 = assign26880_e25670_d_n5;
        locals.var_w_b0_dn6 = assign26880_e25670_d_n6;
        locals.var_w_b0_dn7 = assign26880_e25670_d_n7;
        locals.var_w_b0_dn8 = assign26880_e25670_d_n8;
        locals.var_w_b0_dn9 = assign26880_e25670_d_n9;
        locals.var_w_b0_dn10 = assign26880_e25670_d_n10;
        locals.var_w_b0_dn13 = assign26880_e25670_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign26890_e25681, assign26890_e25681_d_n0, assign26890_e25681_d_n2, assign26890_e25681_d_n4, assign26890_e25681_d_n5, assign26890_e25681_d_n6, assign26890_e25681_d_n7, assign26890_e25681_d_n8, assign26890_e25681_d_n9, assign26890_e25681_d_n10, assign26890_e25681_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26890_e25681;
        locals.var_t3_dn0 = assign26890_e25681_d_n0;
        locals.var_t3_dn2 = assign26890_e25681_d_n2;
        locals.var_t3_dn4 = assign26890_e25681_d_n4;
        locals.var_t3_dn5 = assign26890_e25681_d_n5;
        locals.var_t3_dn6 = assign26890_e25681_d_n6;
        locals.var_t3_dn7 = assign26890_e25681_d_n7;
        locals.var_t3_dn8 = assign26890_e25681_d_n8;
        locals.var_t3_dn9 = assign26890_e25681_d_n9;
        locals.var_t3_dn10 = assign26890_e25681_d_n10;
        locals.var_t3_dn13 = assign26890_e25681_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26900_e25693, assign26900_e25693_d_n0, assign26900_e25693_d_n2, assign26900_e25693_d_n4, assign26900_e25693_d_n5, assign26900_e25693_d_n6, assign26900_e25693_d_n7, assign26900_e25693_d_n8, assign26900_e25693_d_n9, assign26900_e25693_d_n10, assign26900_e25693_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign26900_e25693;
        locals.var_w_b0_dn0 = assign26900_e25693_d_n0;
        locals.var_w_b0_dn2 = assign26900_e25693_d_n2;
        locals.var_w_b0_dn4 = assign26900_e25693_d_n4;
        locals.var_w_b0_dn5 = assign26900_e25693_d_n5;
        locals.var_w_b0_dn6 = assign26900_e25693_d_n6;
        locals.var_w_b0_dn7 = assign26900_e25693_d_n7;
        locals.var_w_b0_dn8 = assign26900_e25693_d_n8;
        locals.var_w_b0_dn9 = assign26900_e25693_d_n9;
        locals.var_w_b0_dn10 = assign26900_e25693_d_n10;
        locals.var_w_b0_dn13 = assign26900_e25693_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign26910_e25705, assign26910_e25705_d_n0, assign26910_e25705_d_n2, assign26910_e25705_d_n4, assign26910_e25705_d_n5, assign26910_e25705_d_n6, assign26910_e25705_d_n7, assign26910_e25705_d_n8, assign26910_e25705_d_n9, assign26910_e25705_d_n10, assign26910_e25705_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard637 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign26910_e25705;
        locals.var_t3_dn0 = assign26910_e25705_d_n0;
        locals.var_t3_dn2 = assign26910_e25705_d_n2;
        locals.var_t3_dn4 = assign26910_e25705_d_n4;
        locals.var_t3_dn5 = assign26910_e25705_d_n5;
        locals.var_t3_dn6 = assign26910_e25705_d_n6;
        locals.var_t3_dn7 = assign26910_e25705_d_n7;
        locals.var_t3_dn8 = assign26910_e25705_d_n8;
        locals.var_t3_dn9 = assign26910_e25705_d_n9;
        locals.var_t3_dn10 = assign26910_e25705_d_n10;
        locals.var_t3_dn13 = assign26910_e25705_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign26920_e25721, assign26920_e25721_d_n0, assign26920_e25721_d_n2, assign26920_e25721_d_n4, assign26920_e25721_d_n5, assign26920_e25721_d_n6, assign26920_e25721_d_n7, assign26920_e25721_d_n8, assign26920_e25721_d_n9, assign26920_e25721_d_n10, assign26920_e25721_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26920_e25715: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk435);
        let assign26920_e25717: f64 = (assign26920_e25715 + locals.var_vbi_dep);
        let assign26920_e25718: f64 = (locals.var_c_2esipq_nsub * assign26920_e25717);
        let assign26920_e25719: f64 = (assign26920_e25718).sqrt();
        (assign26920_e25719, (((locals.var_c_2esipq_nsub_dn0 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn2 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn4 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn5 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn6 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn7 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn8 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn9 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn10 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign26920_e25719)), (((locals.var_c_2esipq_nsub_dn13 * assign26920_e25717) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_vbi_dep_dn13))) / (2.0 * assign26920_e25719)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn13,)
    }
};
        locals.var_w_sub0 = assign26920_e25721;
        locals.var_w_sub0_dn0 = assign26920_e25721_d_n0;
        locals.var_w_sub0_dn2 = assign26920_e25721_d_n2;
        locals.var_w_sub0_dn4 = assign26920_e25721_d_n4;
        locals.var_w_sub0_dn5 = assign26920_e25721_d_n5;
        locals.var_w_sub0_dn6 = assign26920_e25721_d_n6;
        locals.var_w_sub0_dn7 = assign26920_e25721_d_n7;
        locals.var_w_sub0_dn8 = assign26920_e25721_d_n8;
        locals.var_w_sub0_dn9 = assign26920_e25721_d_n9;
        locals.var_w_sub0_dn10 = assign26920_e25721_d_n10;
        locals.var_w_sub0_dn13 = assign26920_e25721_d_n13;
        locals.var_w_sub0_rv = 0.0;

        let (assign26930_e25732, assign26930_e25732_d_n0, assign26930_e25732_d_n2, assign26930_e25732_d_n4, assign26930_e25732_d_n5, assign26930_e25732_d_n6, assign26930_e25732_d_n7, assign26930_e25732_d_n8, assign26930_e25732_d_n9, assign26930_e25732_d_n10, assign26930_e25732_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26930_e25730: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign26930_e25730, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn13 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn13)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn13,)
    }
};
        locals.var_q_b0_dep = assign26930_e25732;
        locals.var_q_b0_dep_dn0 = assign26930_e25732_d_n0;
        locals.var_q_b0_dep_dn2 = assign26930_e25732_d_n2;
        locals.var_q_b0_dep_dn4 = assign26930_e25732_d_n4;
        locals.var_q_b0_dep_dn5 = assign26930_e25732_d_n5;
        locals.var_q_b0_dep_dn6 = assign26930_e25732_d_n6;
        locals.var_q_b0_dep_dn7 = assign26930_e25732_d_n7;
        locals.var_q_b0_dep_dn8 = assign26930_e25732_d_n8;
        locals.var_q_b0_dep_dn9 = assign26930_e25732_d_n9;
        locals.var_q_b0_dep_dn10 = assign26930_e25732_d_n10;
        locals.var_q_b0_dep_dn13 = assign26930_e25732_d_n13;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign26940_e25744, assign26940_e25744_d_n0, assign26940_e25744_d_n2, assign26940_e25744_d_n4, assign26940_e25744_d_n5, assign26940_e25744_d_n6, assign26940_e25744_d_n7, assign26940_e25744_d_n8, assign26940_e25744_d_n9, assign26940_e25744_d_n10, assign26940_e25744_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard623 == 0.0)) {
        let assign26940_e25740: f64 = (-locals.var_w_sub0);
        let assign26940_e25742: f64 = (assign26940_e25740 * locals.var_q_nsub__blk544);
        (assign26940_e25742, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn10)), (((-locals.var_w_sub0_dn13) * locals.var_q_nsub__blk544) + (assign26940_e25740 * locals.var_q_nsub__blk544_dn13)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn13,)
    }
};
        locals.var_q_sub0_dep = assign26940_e25744;
        locals.var_q_sub0_dep_dn0 = assign26940_e25744_d_n0;
        locals.var_q_sub0_dep_dn2 = assign26940_e25744_d_n2;
        locals.var_q_sub0_dep_dn4 = assign26940_e25744_d_n4;
        locals.var_q_sub0_dep_dn5 = assign26940_e25744_d_n5;
        locals.var_q_sub0_dep_dn6 = assign26940_e25744_d_n6;
        locals.var_q_sub0_dep_dn7 = assign26940_e25744_d_n7;
        locals.var_q_sub0_dep_dn8 = assign26940_e25744_d_n8;
        locals.var_q_sub0_dep_dn9 = assign26940_e25744_d_n9;
        locals.var_q_sub0_dep_dn10 = assign26940_e25744_d_n10;
        locals.var_q_sub0_dep_dn13 = assign26940_e25744_d_n13;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign26950_e25752, assign26950_e25752_d_n0, assign26950_e25752_d_n2, assign26950_e25752_d_n4, assign26950_e25752_d_n5, assign26950_e25752_d_n6, assign26950_e25752_d_n7, assign26950_e25752_d_n8, assign26950_e25752_d_n9, assign26950_e25752_d_n10, assign26950_e25752_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign26950_e25750: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        (assign26950_e25750, (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10), (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign26950_e25752;
        locals.var_t1_dn0 = assign26950_e25752_d_n0;
        locals.var_t1_dn2 = assign26950_e25752_d_n2;
        locals.var_t1_dn4 = assign26950_e25752_d_n4;
        locals.var_t1_dn5 = assign26950_e25752_d_n5;
        locals.var_t1_dn6 = assign26950_e25752_d_n6;
        locals.var_t1_dn7 = assign26950_e25752_d_n7;
        locals.var_t1_dn8 = assign26950_e25752_d_n8;
        locals.var_t1_dn9 = assign26950_e25752_d_n9;
        locals.var_t1_dn10 = assign26950_e25752_d_n10;
        locals.var_t1_dn13 = assign26950_e25752_d_n13;
        locals.var_t1_rv = 0.0;

        let assign26960_e25756: f64 = 0.1;
        let assign26960_e25761: f64 = if ((locals.var_t1 < assign26960_e25756) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard643 = assign26960_e25761;
        locals.var_guard643_rv = 0.0;

        let (assign26970_e25773, assign26970_e25773_d_n0, assign26970_e25773_d_n2, assign26970_e25773_d_n4, assign26970_e25773_d_n5, assign26970_e25773_d_n6, assign26970_e25773_d_n7, assign26970_e25773_d_n8, assign26970_e25773_d_n9, assign26970_e25773_d_n10, assign26970_e25773_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign26970_e25769: f64 = 0.1;
        let assign26970_e25771: f64 = (assign26970_e25769 - locals.var_t1);
        (assign26970_e25771, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign26970_e25773;
        locals.var_tmf1_dn0 = assign26970_e25773_d_n0;
        locals.var_tmf1_dn2 = assign26970_e25773_d_n2;
        locals.var_tmf1_dn4 = assign26970_e25773_d_n4;
        locals.var_tmf1_dn5 = assign26970_e25773_d_n5;
        locals.var_tmf1_dn6 = assign26970_e25773_d_n6;
        locals.var_tmf1_dn7 = assign26970_e25773_d_n7;
        locals.var_tmf1_dn8 = assign26970_e25773_d_n8;
        locals.var_tmf1_dn9 = assign26970_e25773_d_n9;
        locals.var_tmf1_dn10 = assign26970_e25773_d_n10;
        locals.var_tmf1_dn13 = assign26970_e25773_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign26980_e25783, assign26980_e25783_d_n0, assign26980_e25783_d_n2, assign26980_e25783_d_n4, assign26980_e25783_d_n5, assign26980_e25783_d_n6, assign26980_e25783_d_n7, assign26980_e25783_d_n8, assign26980_e25783_d_n9, assign26980_e25783_d_n10, assign26980_e25783_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign26980_e25781: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26980_e25781, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign26980_e25783;
        locals.var_x2_dn0 = assign26980_e25783_d_n0;
        locals.var_x2_dn2 = assign26980_e25783_d_n2;
        locals.var_x2_dn4 = assign26980_e25783_d_n4;
        locals.var_x2_dn5 = assign26980_e25783_d_n5;
        locals.var_x2_dn6 = assign26980_e25783_d_n6;
        locals.var_x2_dn7 = assign26980_e25783_d_n7;
        locals.var_x2_dn8 = assign26980_e25783_d_n8;
        locals.var_x2_dn9 = assign26980_e25783_d_n9;
        locals.var_x2_dn10 = assign26980_e25783_d_n10;
        locals.var_x2_dn13 = assign26980_e25783_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign26990_e25793, assign26990_e25793_d_n0, assign26990_e25793_d_n2, assign26990_e25793_d_n4, assign26990_e25793_d_n5, assign26990_e25793_d_n6, assign26990_e25793_d_n7, assign26990_e25793_d_n8, assign26990_e25793_d_n9, assign26990_e25793_d_n10, assign26990_e25793_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign26990_e25791: f64 = (0.1 * 0.1);
        (assign26990_e25791, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign26990_e25793;
        locals.var_xmax2_dn0 = assign26990_e25793_d_n0;
        locals.var_xmax2_dn2 = assign26990_e25793_d_n2;
        locals.var_xmax2_dn4 = assign26990_e25793_d_n4;
        locals.var_xmax2_dn5 = assign26990_e25793_d_n5;
        locals.var_xmax2_dn6 = assign26990_e25793_d_n6;
        locals.var_xmax2_dn7 = assign26990_e25793_d_n7;
        locals.var_xmax2_dn8 = assign26990_e25793_d_n8;
        locals.var_xmax2_dn9 = assign26990_e25793_d_n9;
        locals.var_xmax2_dn10 = assign26990_e25793_d_n10;
        locals.var_xmax2_dn13 = assign26990_e25793_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign27000_e25801, assign27000_e25801_d_n0, assign27000_e25801_d_n2, assign27000_e25801_d_n4, assign27000_e25801_d_n5, assign27000_e25801_d_n6, assign27000_e25801_d_n7, assign27000_e25801_d_n8, assign27000_e25801_d_n9, assign27000_e25801_d_n10, assign27000_e25801_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27000_e25801;
        locals.var_xp_dn0 = assign27000_e25801_d_n0;
        locals.var_xp_dn2 = assign27000_e25801_d_n2;
        locals.var_xp_dn4 = assign27000_e25801_d_n4;
        locals.var_xp_dn5 = assign27000_e25801_d_n5;
        locals.var_xp_dn6 = assign27000_e25801_d_n6;
        locals.var_xp_dn7 = assign27000_e25801_d_n7;
        locals.var_xp_dn8 = assign27000_e25801_d_n8;
        locals.var_xp_dn9 = assign27000_e25801_d_n9;
        locals.var_xp_dn10 = assign27000_e25801_d_n10;
        locals.var_xp_dn13 = assign27000_e25801_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign27010_e25809, assign27010_e25809_d_n0, assign27010_e25809_d_n2, assign27010_e25809_d_n4, assign27010_e25809_d_n5, assign27010_e25809_d_n6, assign27010_e25809_d_n7, assign27010_e25809_d_n8, assign27010_e25809_d_n9, assign27010_e25809_d_n10, assign27010_e25809_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27010_e25809;
        locals.var_xmp_dn0 = assign27010_e25809_d_n0;
        locals.var_xmp_dn2 = assign27010_e25809_d_n2;
        locals.var_xmp_dn4 = assign27010_e25809_d_n4;
        locals.var_xmp_dn5 = assign27010_e25809_d_n5;
        locals.var_xmp_dn6 = assign27010_e25809_d_n6;
        locals.var_xmp_dn7 = assign27010_e25809_d_n7;
        locals.var_xmp_dn8 = assign27010_e25809_d_n8;
        locals.var_xmp_dn9 = assign27010_e25809_d_n9;
        locals.var_xmp_dn10 = assign27010_e25809_d_n10;
        locals.var_xmp_dn13 = assign27010_e25809_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27020_e25817,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27020_e25817;
        locals.var_m0_rv = 0.0;

        let (assign27030_e25825,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27030_e25825;
        locals.var_mm_rv = 0.0;

        let (assign27040_e25833, assign27040_e25833_d_n0, assign27040_e25833_d_n2, assign27040_e25833_d_n4, assign27040_e25833_d_n5, assign27040_e25833_d_n6, assign27040_e25833_d_n7, assign27040_e25833_d_n8, assign27040_e25833_d_n9, assign27040_e25833_d_n10, assign27040_e25833_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign27040_e25833;
        locals.var_arg_dn0 = assign27040_e25833_d_n0;
        locals.var_arg_dn2 = assign27040_e25833_d_n2;
        locals.var_arg_dn4 = assign27040_e25833_d_n4;
        locals.var_arg_dn5 = assign27040_e25833_d_n5;
        locals.var_arg_dn6 = assign27040_e25833_d_n6;
        locals.var_arg_dn7 = assign27040_e25833_d_n7;
        locals.var_arg_dn8 = assign27040_e25833_d_n8;
        locals.var_arg_dn9 = assign27040_e25833_d_n9;
        locals.var_arg_dn10 = assign27040_e25833_d_n10;
        locals.var_arg_dn13 = assign27040_e25833_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign27050_e25841, assign27050_e25841_d_n0, assign27050_e25841_d_n2, assign27050_e25841_d_n4, assign27050_e25841_d_n5, assign27050_e25841_d_n6, assign27050_e25841_d_n7, assign27050_e25841_d_n8, assign27050_e25841_d_n9, assign27050_e25841_d_n10, assign27050_e25841_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27050_e25841;
        locals.var_dnm_dn0 = assign27050_e25841_d_n0;
        locals.var_dnm_dn2 = assign27050_e25841_d_n2;
        locals.var_dnm_dn4 = assign27050_e25841_d_n4;
        locals.var_dnm_dn5 = assign27050_e25841_d_n5;
        locals.var_dnm_dn6 = assign27050_e25841_d_n6;
        locals.var_dnm_dn7 = assign27050_e25841_d_n7;
        locals.var_dnm_dn8 = assign27050_e25841_d_n8;
        locals.var_dnm_dn9 = assign27050_e25841_d_n9;
        locals.var_dnm_dn10 = assign27050_e25841_d_n10;
        locals.var_dnm_dn13 = assign27050_e25841_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27060_e25851, assign27060_e25851_d_n0, assign27060_e25851_d_n2, assign27060_e25851_d_n4, assign27060_e25851_d_n5, assign27060_e25851_d_n6, assign27060_e25851_d_n7, assign27060_e25851_d_n8, assign27060_e25851_d_n9, assign27060_e25851_d_n10, assign27060_e25851_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27060_e25849: f64 = (locals.var_xp * locals.var_x2);
        (assign27060_e25849, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27060_e25851;
        locals.var_xp_dn0 = assign27060_e25851_d_n0;
        locals.var_xp_dn2 = assign27060_e25851_d_n2;
        locals.var_xp_dn4 = assign27060_e25851_d_n4;
        locals.var_xp_dn5 = assign27060_e25851_d_n5;
        locals.var_xp_dn6 = assign27060_e25851_d_n6;
        locals.var_xp_dn7 = assign27060_e25851_d_n7;
        locals.var_xp_dn8 = assign27060_e25851_d_n8;
        locals.var_xp_dn9 = assign27060_e25851_d_n9;
        locals.var_xp_dn10 = assign27060_e25851_d_n10;
        locals.var_xp_dn13 = assign27060_e25851_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27070_e25861, assign27070_e25861_d_n0, assign27070_e25861_d_n2, assign27070_e25861_d_n4, assign27070_e25861_d_n5, assign27070_e25861_d_n6, assign27070_e25861_d_n7, assign27070_e25861_d_n8, assign27070_e25861_d_n9, assign27070_e25861_d_n10, assign27070_e25861_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27070_e25859: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27070_e25859, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27070_e25861;
        locals.var_xmp_dn0 = assign27070_e25861_d_n0;
        locals.var_xmp_dn2 = assign27070_e25861_d_n2;
        locals.var_xmp_dn4 = assign27070_e25861_d_n4;
        locals.var_xmp_dn5 = assign27070_e25861_d_n5;
        locals.var_xmp_dn6 = assign27070_e25861_d_n6;
        locals.var_xmp_dn7 = assign27070_e25861_d_n7;
        locals.var_xmp_dn8 = assign27070_e25861_d_n8;
        locals.var_xmp_dn9 = assign27070_e25861_d_n9;
        locals.var_xmp_dn10 = assign27070_e25861_d_n10;
        locals.var_xmp_dn13 = assign27070_e25861_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27080_e25871, assign27080_e25871_d_n0, assign27080_e25871_d_n2, assign27080_e25871_d_n4, assign27080_e25871_d_n5, assign27080_e25871_d_n6, assign27080_e25871_d_n7, assign27080_e25871_d_n8, assign27080_e25871_d_n9, assign27080_e25871_d_n10, assign27080_e25871_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27080_e25869: f64 = (locals.var_xp * locals.var_x2);
        (assign27080_e25869, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27080_e25871;
        locals.var_xp_dn0 = assign27080_e25871_d_n0;
        locals.var_xp_dn2 = assign27080_e25871_d_n2;
        locals.var_xp_dn4 = assign27080_e25871_d_n4;
        locals.var_xp_dn5 = assign27080_e25871_d_n5;
        locals.var_xp_dn6 = assign27080_e25871_d_n6;
        locals.var_xp_dn7 = assign27080_e25871_d_n7;
        locals.var_xp_dn8 = assign27080_e25871_d_n8;
        locals.var_xp_dn9 = assign27080_e25871_d_n9;
        locals.var_xp_dn10 = assign27080_e25871_d_n10;
        locals.var_xp_dn13 = assign27080_e25871_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27090_e25881, assign27090_e25881_d_n0, assign27090_e25881_d_n2, assign27090_e25881_d_n4, assign27090_e25881_d_n5, assign27090_e25881_d_n6, assign27090_e25881_d_n7, assign27090_e25881_d_n8, assign27090_e25881_d_n9, assign27090_e25881_d_n10, assign27090_e25881_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27090_e25879: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27090_e25879, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27090_e25881;
        locals.var_xmp_dn0 = assign27090_e25881_d_n0;
        locals.var_xmp_dn2 = assign27090_e25881_d_n2;
        locals.var_xmp_dn4 = assign27090_e25881_d_n4;
        locals.var_xmp_dn5 = assign27090_e25881_d_n5;
        locals.var_xmp_dn6 = assign27090_e25881_d_n6;
        locals.var_xmp_dn7 = assign27090_e25881_d_n7;
        locals.var_xmp_dn8 = assign27090_e25881_d_n8;
        locals.var_xmp_dn9 = assign27090_e25881_d_n9;
        locals.var_xmp_dn10 = assign27090_e25881_d_n10;
        locals.var_xmp_dn13 = assign27090_e25881_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27100_e25891, assign27100_e25891_d_n0, assign27100_e25891_d_n2, assign27100_e25891_d_n4, assign27100_e25891_d_n5, assign27100_e25891_d_n6, assign27100_e25891_d_n7, assign27100_e25891_d_n8, assign27100_e25891_d_n9, assign27100_e25891_d_n10, assign27100_e25891_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27100_e25889: f64 = (locals.var_xp + locals.var_xmp);
        (assign27100_e25889, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign27100_e25891;
        locals.var_arg_dn0 = assign27100_e25891_d_n0;
        locals.var_arg_dn2 = assign27100_e25891_d_n2;
        locals.var_arg_dn4 = assign27100_e25891_d_n4;
        locals.var_arg_dn5 = assign27100_e25891_d_n5;
        locals.var_arg_dn6 = assign27100_e25891_d_n6;
        locals.var_arg_dn7 = assign27100_e25891_d_n7;
        locals.var_arg_dn8 = assign27100_e25891_d_n8;
        locals.var_arg_dn9 = assign27100_e25891_d_n9;
        locals.var_arg_dn10 = assign27100_e25891_d_n10;
        locals.var_arg_dn13 = assign27100_e25891_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign27110_e25899, assign27110_e25899_d_n0, assign27110_e25899_d_n2, assign27110_e25899_d_n4, assign27110_e25899_d_n5, assign27110_e25899_d_n6, assign27110_e25899_d_n7, assign27110_e25899_d_n8, assign27110_e25899_d_n9, assign27110_e25899_d_n10, assign27110_e25899_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27110_e25899;
        locals.var_dnm_dn0 = assign27110_e25899_d_n0;
        locals.var_dnm_dn2 = assign27110_e25899_d_n2;
        locals.var_dnm_dn4 = assign27110_e25899_d_n4;
        locals.var_dnm_dn5 = assign27110_e25899_d_n5;
        locals.var_dnm_dn6 = assign27110_e25899_d_n6;
        locals.var_dnm_dn7 = assign27110_e25899_d_n7;
        locals.var_dnm_dn8 = assign27110_e25899_d_n8;
        locals.var_dnm_dn9 = assign27110_e25899_d_n9;
        locals.var_dnm_dn10 = assign27110_e25899_d_n10;
        locals.var_dnm_dn13 = assign27110_e25899_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign27120_e25914: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard644 = assign27120_e25914;
        locals.var_guard644_rv = 0.0;

        let assign27130_e25917: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard645 = assign27130_e25917;
        locals.var_guard645_rv = 0.0;

        let (assign27140_e25929,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27140_e25929;
        locals.var_mm_rv = 0.0;

        let assign27150_e25932: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign27150_e25932;
        locals.var_guard646_rv = 0.0;

        let (assign27160_e25947,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard646 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27160_e25947;
        locals.var_mm_rv = 0.0;

        let assign27170_e25950: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard647 = assign27170_e25950;
        locals.var_guard647_rv = 0.0;

        let (assign27180_e25968,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard646 == 0.0)) && (locals.var_guard647 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27180_e25968;
        locals.var_mm_rv = 0.0;

        let assign27190_e25971: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard648 = assign27190_e25971;
        locals.var_guard648_rv = 0.0;

        let (assign27200_e25992,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard646 == 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27200_e25992;
        locals.var_mm_rv = 0.0;

        let (assign27210_e26002,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27210_e26002;
        locals.var_m0_rv = 0.0;

        let mut assign27220_loop_guard: usize = 0;
        while {
            let assign27220_cond_e26013: f64 = if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27220_cond_e26013 != 0.0
        } {
            assign27220_loop_guard += 1;
            assert!(assign27220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27220_body0_e26024, assign27220_body0_e26024_d_n0, assign27220_body0_e26024_d_n2, assign27220_body0_e26024_d_n4, assign27220_body0_e26024_d_n5, assign27220_body0_e26024_d_n6, assign27220_body0_e26024_d_n7, assign27220_body0_e26024_d_n8, assign27220_body0_e26024_d_n9, assign27220_body0_e26024_d_n10, assign27220_body0_e26024_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) {
        let assign27220_body0_e26022: f64 = (locals.var_dnm).sqrt();
        (assign27220_body0_e26022, (locals.var_dnm_dn0 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn2 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn4 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn5 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn6 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn7 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn8 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn9 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn10 / (2.0 * assign27220_body0_e26022)), (locals.var_dnm_dn13 / (2.0 * assign27220_body0_e26022)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign27220_body0_e26024;
            locals.var_dnm_dn0 = assign27220_body0_e26024_d_n0;
            locals.var_dnm_dn2 = assign27220_body0_e26024_d_n2;
            locals.var_dnm_dn4 = assign27220_body0_e26024_d_n4;
            locals.var_dnm_dn5 = assign27220_body0_e26024_d_n5;
            locals.var_dnm_dn6 = assign27220_body0_e26024_d_n6;
            locals.var_dnm_dn7 = assign27220_body0_e26024_d_n7;
            locals.var_dnm_dn8 = assign27220_body0_e26024_d_n8;
            locals.var_dnm_dn9 = assign27220_body0_e26024_d_n9;
            locals.var_dnm_dn10 = assign27220_body0_e26024_d_n10;
            locals.var_dnm_dn13 = assign27220_body0_e26024_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign27220_body1_e26036,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) {
        let assign27220_body1_e26034: f64 = (locals.var_m0 + 1.0);
        (assign27220_body1_e26034,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27220_body1_e26036;
            locals.var_m0_rv = 0.0;
        }

        let (assign27230_e26058, assign27230_e26058_d_n0, assign27230_e26058_d_n2, assign27230_e26058_d_n4, assign27230_e26058_d_n5, assign27230_e26058_d_n6, assign27230_e26058_d_n7, assign27230_e26058_d_n8, assign27230_e26058_d_n9, assign27230_e26058_d_n10, assign27230_e26058_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 == 0.0)) {
        let (assign27230_e26056, assign27230_e26056_d_n0, assign27230_e26056_d_n2, assign27230_e26056_d_n4, assign27230_e26056_d_n5, assign27230_e26056_d_n6, assign27230_e26056_d_n7, assign27230_e26056_d_n8, assign27230_e26056_d_n9, assign27230_e26056_d_n10, assign27230_e26056_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27230_e26053: f64 = (2.0 * 2.0);
                let assign27230_e26054: f64 = (1.0 / assign27230_e26053);
                let assign27230_e26055: f64 = (locals.var_dnm).powf(assign27230_e26054);
                (assign27230_e26055, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27230_e26054) as f64).is_finite() && ((assign27230_e26054) as f64).fract() == 0.0 { if assign27230_e26054 == 0.0 { 0.0 } else { (assign27230_e26054 * ((locals.var_dnm).powf(assign27230_e26054 - 1.0) * locals.var_dnm_dn13)) } } else { (assign27230_e26055 * (assign27230_e26054 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign27230_e26056, assign27230_e26056_d_n0, assign27230_e26056_d_n2, assign27230_e26056_d_n4, assign27230_e26056_d_n5, assign27230_e26056_d_n6, assign27230_e26056_d_n7, assign27230_e26056_d_n8, assign27230_e26056_d_n9, assign27230_e26056_d_n10, assign27230_e26056_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27230_e26058;
        locals.var_dnm_dn0 = assign27230_e26058_d_n0;
        locals.var_dnm_dn2 = assign27230_e26058_d_n2;
        locals.var_dnm_dn4 = assign27230_e26058_d_n4;
        locals.var_dnm_dn5 = assign27230_e26058_d_n5;
        locals.var_dnm_dn6 = assign27230_e26058_d_n6;
        locals.var_dnm_dn7 = assign27230_e26058_d_n7;
        locals.var_dnm_dn8 = assign27230_e26058_d_n8;
        locals.var_dnm_dn9 = assign27230_e26058_d_n9;
        locals.var_dnm_dn10 = assign27230_e26058_d_n10;
        locals.var_dnm_dn13 = assign27230_e26058_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27240_e26068, assign27240_e26068_d_n0, assign27240_e26068_d_n2, assign27240_e26068_d_n4, assign27240_e26068_d_n5, assign27240_e26068_d_n6, assign27240_e26068_d_n7, assign27240_e26068_d_n8, assign27240_e26068_d_n9, assign27240_e26068_d_n10, assign27240_e26068_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27240_e26066: f64 = (1.0 / locals.var_dnm);
        (assign27240_e26066, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27240_e26068;
        locals.var_dnm_dn0 = assign27240_e26068_d_n0;
        locals.var_dnm_dn2 = assign27240_e26068_d_n2;
        locals.var_dnm_dn4 = assign27240_e26068_d_n4;
        locals.var_dnm_dn5 = assign27240_e26068_d_n5;
        locals.var_dnm_dn6 = assign27240_e26068_d_n6;
        locals.var_dnm_dn7 = assign27240_e26068_d_n7;
        locals.var_dnm_dn8 = assign27240_e26068_d_n8;
        locals.var_dnm_dn9 = assign27240_e26068_d_n9;
        locals.var_dnm_dn10 = assign27240_e26068_d_n10;
        locals.var_dnm_dn13 = assign27240_e26068_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27250_e26080, assign27250_e26080_d_n0, assign27250_e26080_d_n2, assign27250_e26080_d_n4, assign27250_e26080_d_n5, assign27250_e26080_d_n6, assign27250_e26080_d_n7, assign27250_e26080_d_n8, assign27250_e26080_d_n9, assign27250_e26080_d_n10, assign27250_e26080_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27250_e26076: f64 = (locals.var_tmf1 * 0.1);
        let assign27250_e26078: f64 = (assign27250_e26076 * locals.var_dnm);
        (assign27250_e26078, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign27250_e26076 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign27250_e26080;
        locals.var_tmf0_dn0 = assign27250_e26080_d_n0;
        locals.var_tmf0_dn2 = assign27250_e26080_d_n2;
        locals.var_tmf0_dn4 = assign27250_e26080_d_n4;
        locals.var_tmf0_dn5 = assign27250_e26080_d_n5;
        locals.var_tmf0_dn6 = assign27250_e26080_d_n6;
        locals.var_tmf0_dn7 = assign27250_e26080_d_n7;
        locals.var_tmf0_dn8 = assign27250_e26080_d_n8;
        locals.var_tmf0_dn9 = assign27250_e26080_d_n9;
        locals.var_tmf0_dn10 = assign27250_e26080_d_n10;
        locals.var_tmf0_dn13 = assign27250_e26080_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign27260_e26094, assign27260_e26094_d_n0, assign27260_e26094_d_n2, assign27260_e26094_d_n4, assign27260_e26094_d_n5, assign27260_e26094_d_n6, assign27260_e26094_d_n7, assign27260_e26094_d_n8, assign27260_e26094_d_n9, assign27260_e26094_d_n10, assign27260_e26094_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27260_e26088: f64 = (0.1 * locals.var_xmp);
        let assign27260_e26090: f64 = (assign27260_e26088 * locals.var_dnm);
        let assign27260_e26092: f64 = (assign27260_e26090 / locals.var_arg);
        (assign27260_e26092, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign27260_e26088 * locals.var_dnm_dn13)) * locals.var_arg) - (assign27260_e26090 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign27260_e26094;
        locals.var_t0_dn0 = assign27260_e26094_d_n0;
        locals.var_t0_dn2 = assign27260_e26094_d_n2;
        locals.var_t0_dn4 = assign27260_e26094_d_n4;
        locals.var_t0_dn5 = assign27260_e26094_d_n5;
        locals.var_t0_dn6 = assign27260_e26094_d_n6;
        locals.var_t0_dn7 = assign27260_e26094_d_n7;
        locals.var_t0_dn8 = assign27260_e26094_d_n8;
        locals.var_t0_dn9 = assign27260_e26094_d_n9;
        locals.var_t0_dn10 = assign27260_e26094_d_n10;
        locals.var_t0_dn13 = assign27260_e26094_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign27270_e26106, assign27270_e26106_d_n0, assign27270_e26106_d_n2, assign27270_e26106_d_n4, assign27270_e26106_d_n5, assign27270_e26106_d_n6, assign27270_e26106_d_n7, assign27270_e26106_d_n8, assign27270_e26106_d_n9, assign27270_e26106_d_n10, assign27270_e26106_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign27270_e26102: f64 = 0.1;
        let assign27270_e26104: f64 = (assign27270_e26102 - locals.var_tmf0);
        (assign27270_e26104, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign27270_e26106;
        locals.var_t2_dn0 = assign27270_e26106_d_n0;
        locals.var_t2_dn2 = assign27270_e26106_d_n2;
        locals.var_t2_dn4 = assign27270_e26106_d_n4;
        locals.var_t2_dn5 = assign27270_e26106_d_n5;
        locals.var_t2_dn6 = assign27270_e26106_d_n6;
        locals.var_t2_dn7 = assign27270_e26106_d_n7;
        locals.var_t2_dn8 = assign27270_e26106_d_n8;
        locals.var_t2_dn9 = assign27270_e26106_d_n9;
        locals.var_t2_dn10 = assign27270_e26106_d_n10;
        locals.var_t2_dn13 = assign27270_e26106_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign27280_e26114, assign27280_e26114_d_n0, assign27280_e26114_d_n2, assign27280_e26114_d_n4, assign27280_e26114_d_n5, assign27280_e26114_d_n6, assign27280_e26114_d_n7, assign27280_e26114_d_n8, assign27280_e26114_d_n9, assign27280_e26114_d_n10, assign27280_e26114_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign27280_e26114;
        locals.var_t0_dn0 = assign27280_e26114_d_n0;
        locals.var_t0_dn2 = assign27280_e26114_d_n2;
        locals.var_t0_dn4 = assign27280_e26114_d_n4;
        locals.var_t0_dn5 = assign27280_e26114_d_n5;
        locals.var_t0_dn6 = assign27280_e26114_d_n6;
        locals.var_t0_dn7 = assign27280_e26114_d_n7;
        locals.var_t0_dn8 = assign27280_e26114_d_n8;
        locals.var_t0_dn9 = assign27280_e26114_d_n9;
        locals.var_t0_dn10 = assign27280_e26114_d_n10;
        locals.var_t0_dn13 = assign27280_e26114_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign27290_e26123, assign27290_e26123_d_n0, assign27290_e26123_d_n2, assign27290_e26123_d_n4, assign27290_e26123_d_n5, assign27290_e26123_d_n6, assign27290_e26123_d_n7, assign27290_e26123_d_n8, assign27290_e26123_d_n9, assign27290_e26123_d_n10, assign27290_e26123_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign27290_e26123;
        locals.var_t2_dn0 = assign27290_e26123_d_n0;
        locals.var_t2_dn2 = assign27290_e26123_d_n2;
        locals.var_t2_dn4 = assign27290_e26123_d_n4;
        locals.var_t2_dn5 = assign27290_e26123_d_n5;
        locals.var_t2_dn6 = assign27290_e26123_d_n6;
        locals.var_t2_dn7 = assign27290_e26123_d_n7;
        locals.var_t2_dn8 = assign27290_e26123_d_n8;
        locals.var_t2_dn9 = assign27290_e26123_d_n9;
        locals.var_t2_dn10 = assign27290_e26123_d_n10;
        locals.var_t2_dn13 = assign27290_e26123_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign27300_e26132, assign27300_e26132_d_n0, assign27300_e26132_d_n2, assign27300_e26132_d_n4, assign27300_e26132_d_n5, assign27300_e26132_d_n6, assign27300_e26132_d_n7, assign27300_e26132_d_n8, assign27300_e26132_d_n9, assign27300_e26132_d_n10, assign27300_e26132_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard643 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign27300_e26132;
        locals.var_t0_dn0 = assign27300_e26132_d_n0;
        locals.var_t0_dn2 = assign27300_e26132_d_n2;
        locals.var_t0_dn4 = assign27300_e26132_d_n4;
        locals.var_t0_dn5 = assign27300_e26132_d_n5;
        locals.var_t0_dn6 = assign27300_e26132_d_n6;
        locals.var_t0_dn7 = assign27300_e26132_d_n7;
        locals.var_t0_dn8 = assign27300_e26132_d_n8;
        locals.var_t0_dn9 = assign27300_e26132_d_n9;
        locals.var_t0_dn10 = assign27300_e26132_d_n10;
        locals.var_t0_dn13 = assign27300_e26132_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        locals: &mut StampLocals,
    ) {
        let (assign27310_e26141, assign27310_e26141_d_n0, assign27310_e26141_d_n2, assign27310_e26141_d_n4, assign27310_e26141_d_n5, assign27310_e26141_d_n6, assign27310_e26141_d_n7, assign27310_e26141_d_n8, assign27310_e26141_d_n9, assign27310_e26141_d_n10, assign27310_e26141_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign27310_e26138: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign27310_e26139: f64 = (assign27310_e26138).sqrt();
        (assign27310_e26139, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign27310_e26139)), (((locals.var_c_2esipq_ndepm_dn13 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn13)) / (2.0 * assign27310_e26139)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign27310_e26141;
        locals.var_w_b0_dn0 = assign27310_e26141_d_n0;
        locals.var_w_b0_dn2 = assign27310_e26141_d_n2;
        locals.var_w_b0_dn4 = assign27310_e26141_d_n4;
        locals.var_w_b0_dn5 = assign27310_e26141_d_n5;
        locals.var_w_b0_dn6 = assign27310_e26141_d_n6;
        locals.var_w_b0_dn7 = assign27310_e26141_d_n7;
        locals.var_w_b0_dn8 = assign27310_e26141_d_n8;
        locals.var_w_b0_dn9 = assign27310_e26141_d_n9;
        locals.var_w_b0_dn10 = assign27310_e26141_d_n10;
        locals.var_w_b0_dn13 = assign27310_e26141_d_n13;
        locals.var_w_b0_rv = 0.0;

        let assign27320_e26145: f64 = (locals.var_uc_depthn - 1e-8);
        let assign27320_e26150: f64 = if ((locals.var_w_b0 > assign27320_e26145) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard649 = assign27320_e26150;
        locals.var_guard649_rv = 0.0;

        let (assign27330_e26162, assign27330_e26162_d_n0, assign27330_e26162_d_n2, assign27330_e26162_d_n4, assign27330_e26162_d_n5, assign27330_e26162_d_n6, assign27330_e26162_d_n7, assign27330_e26162_d_n8, assign27330_e26162_d_n9, assign27330_e26162_d_n10, assign27330_e26162_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27330_e26158: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign27330_e26160: f64 = (assign27330_e26158 + 1e-8);
        (assign27330_e26160, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn13 - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign27330_e26162;
        locals.var_tmf1_dn0 = assign27330_e26162_d_n0;
        locals.var_tmf1_dn2 = assign27330_e26162_d_n2;
        locals.var_tmf1_dn4 = assign27330_e26162_d_n4;
        locals.var_tmf1_dn5 = assign27330_e26162_d_n5;
        locals.var_tmf1_dn6 = assign27330_e26162_d_n6;
        locals.var_tmf1_dn7 = assign27330_e26162_d_n7;
        locals.var_tmf1_dn8 = assign27330_e26162_d_n8;
        locals.var_tmf1_dn9 = assign27330_e26162_d_n9;
        locals.var_tmf1_dn10 = assign27330_e26162_d_n10;
        locals.var_tmf1_dn13 = assign27330_e26162_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign27340_e26172, assign27340_e26172_d_n0, assign27340_e26172_d_n2, assign27340_e26172_d_n4, assign27340_e26172_d_n5, assign27340_e26172_d_n6, assign27340_e26172_d_n7, assign27340_e26172_d_n8, assign27340_e26172_d_n9, assign27340_e26172_d_n10, assign27340_e26172_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27340_e26170: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27340_e26170, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign27340_e26172;
        locals.var_x2_dn0 = assign27340_e26172_d_n0;
        locals.var_x2_dn2 = assign27340_e26172_d_n2;
        locals.var_x2_dn4 = assign27340_e26172_d_n4;
        locals.var_x2_dn5 = assign27340_e26172_d_n5;
        locals.var_x2_dn6 = assign27340_e26172_d_n6;
        locals.var_x2_dn7 = assign27340_e26172_d_n7;
        locals.var_x2_dn8 = assign27340_e26172_d_n8;
        locals.var_x2_dn9 = assign27340_e26172_d_n9;
        locals.var_x2_dn10 = assign27340_e26172_d_n10;
        locals.var_x2_dn13 = assign27340_e26172_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign27350_e26182, assign27350_e26182_d_n0, assign27350_e26182_d_n2, assign27350_e26182_d_n4, assign27350_e26182_d_n5, assign27350_e26182_d_n6, assign27350_e26182_d_n7, assign27350_e26182_d_n8, assign27350_e26182_d_n9, assign27350_e26182_d_n10, assign27350_e26182_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27350_e26180: f64 = (1e-8 * 1e-8);
        (assign27350_e26180, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign27350_e26182;
        locals.var_xmax2_dn0 = assign27350_e26182_d_n0;
        locals.var_xmax2_dn2 = assign27350_e26182_d_n2;
        locals.var_xmax2_dn4 = assign27350_e26182_d_n4;
        locals.var_xmax2_dn5 = assign27350_e26182_d_n5;
        locals.var_xmax2_dn6 = assign27350_e26182_d_n6;
        locals.var_xmax2_dn7 = assign27350_e26182_d_n7;
        locals.var_xmax2_dn8 = assign27350_e26182_d_n8;
        locals.var_xmax2_dn9 = assign27350_e26182_d_n9;
        locals.var_xmax2_dn10 = assign27350_e26182_d_n10;
        locals.var_xmax2_dn13 = assign27350_e26182_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign27360_e26190, assign27360_e26190_d_n0, assign27360_e26190_d_n2, assign27360_e26190_d_n4, assign27360_e26190_d_n5, assign27360_e26190_d_n6, assign27360_e26190_d_n7, assign27360_e26190_d_n8, assign27360_e26190_d_n9, assign27360_e26190_d_n10, assign27360_e26190_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27360_e26190;
        locals.var_xp_dn0 = assign27360_e26190_d_n0;
        locals.var_xp_dn2 = assign27360_e26190_d_n2;
        locals.var_xp_dn4 = assign27360_e26190_d_n4;
        locals.var_xp_dn5 = assign27360_e26190_d_n5;
        locals.var_xp_dn6 = assign27360_e26190_d_n6;
        locals.var_xp_dn7 = assign27360_e26190_d_n7;
        locals.var_xp_dn8 = assign27360_e26190_d_n8;
        locals.var_xp_dn9 = assign27360_e26190_d_n9;
        locals.var_xp_dn10 = assign27360_e26190_d_n10;
        locals.var_xp_dn13 = assign27360_e26190_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27370_e26198, assign27370_e26198_d_n0, assign27370_e26198_d_n2, assign27370_e26198_d_n4, assign27370_e26198_d_n5, assign27370_e26198_d_n6, assign27370_e26198_d_n7, assign27370_e26198_d_n8, assign27370_e26198_d_n9, assign27370_e26198_d_n10, assign27370_e26198_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27370_e26198;
        locals.var_xmp_dn0 = assign27370_e26198_d_n0;
        locals.var_xmp_dn2 = assign27370_e26198_d_n2;
        locals.var_xmp_dn4 = assign27370_e26198_d_n4;
        locals.var_xmp_dn5 = assign27370_e26198_d_n5;
        locals.var_xmp_dn6 = assign27370_e26198_d_n6;
        locals.var_xmp_dn7 = assign27370_e26198_d_n7;
        locals.var_xmp_dn8 = assign27370_e26198_d_n8;
        locals.var_xmp_dn9 = assign27370_e26198_d_n9;
        locals.var_xmp_dn10 = assign27370_e26198_d_n10;
        locals.var_xmp_dn13 = assign27370_e26198_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27380_e26206,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27380_e26206;
        locals.var_m0_rv = 0.0;

        let (assign27390_e26214,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27390_e26214;
        locals.var_mm_rv = 0.0;

        let (assign27400_e26222, assign27400_e26222_d_n0, assign27400_e26222_d_n2, assign27400_e26222_d_n4, assign27400_e26222_d_n5, assign27400_e26222_d_n6, assign27400_e26222_d_n7, assign27400_e26222_d_n8, assign27400_e26222_d_n9, assign27400_e26222_d_n10, assign27400_e26222_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign27400_e26222;
        locals.var_arg_dn0 = assign27400_e26222_d_n0;
        locals.var_arg_dn2 = assign27400_e26222_d_n2;
        locals.var_arg_dn4 = assign27400_e26222_d_n4;
        locals.var_arg_dn5 = assign27400_e26222_d_n5;
        locals.var_arg_dn6 = assign27400_e26222_d_n6;
        locals.var_arg_dn7 = assign27400_e26222_d_n7;
        locals.var_arg_dn8 = assign27400_e26222_d_n8;
        locals.var_arg_dn9 = assign27400_e26222_d_n9;
        locals.var_arg_dn10 = assign27400_e26222_d_n10;
        locals.var_arg_dn13 = assign27400_e26222_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign27410_e26230, assign27410_e26230_d_n0, assign27410_e26230_d_n2, assign27410_e26230_d_n4, assign27410_e26230_d_n5, assign27410_e26230_d_n6, assign27410_e26230_d_n7, assign27410_e26230_d_n8, assign27410_e26230_d_n9, assign27410_e26230_d_n10, assign27410_e26230_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27410_e26230;
        locals.var_dnm_dn0 = assign27410_e26230_d_n0;
        locals.var_dnm_dn2 = assign27410_e26230_d_n2;
        locals.var_dnm_dn4 = assign27410_e26230_d_n4;
        locals.var_dnm_dn5 = assign27410_e26230_d_n5;
        locals.var_dnm_dn6 = assign27410_e26230_d_n6;
        locals.var_dnm_dn7 = assign27410_e26230_d_n7;
        locals.var_dnm_dn8 = assign27410_e26230_d_n8;
        locals.var_dnm_dn9 = assign27410_e26230_d_n9;
        locals.var_dnm_dn10 = assign27410_e26230_d_n10;
        locals.var_dnm_dn13 = assign27410_e26230_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27420_e26240, assign27420_e26240_d_n0, assign27420_e26240_d_n2, assign27420_e26240_d_n4, assign27420_e26240_d_n5, assign27420_e26240_d_n6, assign27420_e26240_d_n7, assign27420_e26240_d_n8, assign27420_e26240_d_n9, assign27420_e26240_d_n10, assign27420_e26240_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27420_e26238: f64 = (locals.var_xp * locals.var_x2);
        (assign27420_e26238, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27420_e26240;
        locals.var_xp_dn0 = assign27420_e26240_d_n0;
        locals.var_xp_dn2 = assign27420_e26240_d_n2;
        locals.var_xp_dn4 = assign27420_e26240_d_n4;
        locals.var_xp_dn5 = assign27420_e26240_d_n5;
        locals.var_xp_dn6 = assign27420_e26240_d_n6;
        locals.var_xp_dn7 = assign27420_e26240_d_n7;
        locals.var_xp_dn8 = assign27420_e26240_d_n8;
        locals.var_xp_dn9 = assign27420_e26240_d_n9;
        locals.var_xp_dn10 = assign27420_e26240_d_n10;
        locals.var_xp_dn13 = assign27420_e26240_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27430_e26250, assign27430_e26250_d_n0, assign27430_e26250_d_n2, assign27430_e26250_d_n4, assign27430_e26250_d_n5, assign27430_e26250_d_n6, assign27430_e26250_d_n7, assign27430_e26250_d_n8, assign27430_e26250_d_n9, assign27430_e26250_d_n10, assign27430_e26250_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27430_e26248: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27430_e26248, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27430_e26250;
        locals.var_xmp_dn0 = assign27430_e26250_d_n0;
        locals.var_xmp_dn2 = assign27430_e26250_d_n2;
        locals.var_xmp_dn4 = assign27430_e26250_d_n4;
        locals.var_xmp_dn5 = assign27430_e26250_d_n5;
        locals.var_xmp_dn6 = assign27430_e26250_d_n6;
        locals.var_xmp_dn7 = assign27430_e26250_d_n7;
        locals.var_xmp_dn8 = assign27430_e26250_d_n8;
        locals.var_xmp_dn9 = assign27430_e26250_d_n9;
        locals.var_xmp_dn10 = assign27430_e26250_d_n10;
        locals.var_xmp_dn13 = assign27430_e26250_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27440_e26260, assign27440_e26260_d_n0, assign27440_e26260_d_n2, assign27440_e26260_d_n4, assign27440_e26260_d_n5, assign27440_e26260_d_n6, assign27440_e26260_d_n7, assign27440_e26260_d_n8, assign27440_e26260_d_n9, assign27440_e26260_d_n10, assign27440_e26260_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27440_e26258: f64 = (locals.var_xp * locals.var_x2);
        (assign27440_e26258, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27440_e26260;
        locals.var_xp_dn0 = assign27440_e26260_d_n0;
        locals.var_xp_dn2 = assign27440_e26260_d_n2;
        locals.var_xp_dn4 = assign27440_e26260_d_n4;
        locals.var_xp_dn5 = assign27440_e26260_d_n5;
        locals.var_xp_dn6 = assign27440_e26260_d_n6;
        locals.var_xp_dn7 = assign27440_e26260_d_n7;
        locals.var_xp_dn8 = assign27440_e26260_d_n8;
        locals.var_xp_dn9 = assign27440_e26260_d_n9;
        locals.var_xp_dn10 = assign27440_e26260_d_n10;
        locals.var_xp_dn13 = assign27440_e26260_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27450_e26270, assign27450_e26270_d_n0, assign27450_e26270_d_n2, assign27450_e26270_d_n4, assign27450_e26270_d_n5, assign27450_e26270_d_n6, assign27450_e26270_d_n7, assign27450_e26270_d_n8, assign27450_e26270_d_n9, assign27450_e26270_d_n10, assign27450_e26270_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27450_e26268: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27450_e26268, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27450_e26270;
        locals.var_xmp_dn0 = assign27450_e26270_d_n0;
        locals.var_xmp_dn2 = assign27450_e26270_d_n2;
        locals.var_xmp_dn4 = assign27450_e26270_d_n4;
        locals.var_xmp_dn5 = assign27450_e26270_d_n5;
        locals.var_xmp_dn6 = assign27450_e26270_d_n6;
        locals.var_xmp_dn7 = assign27450_e26270_d_n7;
        locals.var_xmp_dn8 = assign27450_e26270_d_n8;
        locals.var_xmp_dn9 = assign27450_e26270_d_n9;
        locals.var_xmp_dn10 = assign27450_e26270_d_n10;
        locals.var_xmp_dn13 = assign27450_e26270_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27460_e26280, assign27460_e26280_d_n0, assign27460_e26280_d_n2, assign27460_e26280_d_n4, assign27460_e26280_d_n5, assign27460_e26280_d_n6, assign27460_e26280_d_n7, assign27460_e26280_d_n8, assign27460_e26280_d_n9, assign27460_e26280_d_n10, assign27460_e26280_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27460_e26278: f64 = (locals.var_xp + locals.var_xmp);
        (assign27460_e26278, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign27460_e26280;
        locals.var_arg_dn0 = assign27460_e26280_d_n0;
        locals.var_arg_dn2 = assign27460_e26280_d_n2;
        locals.var_arg_dn4 = assign27460_e26280_d_n4;
        locals.var_arg_dn5 = assign27460_e26280_d_n5;
        locals.var_arg_dn6 = assign27460_e26280_d_n6;
        locals.var_arg_dn7 = assign27460_e26280_d_n7;
        locals.var_arg_dn8 = assign27460_e26280_d_n8;
        locals.var_arg_dn9 = assign27460_e26280_d_n9;
        locals.var_arg_dn10 = assign27460_e26280_d_n10;
        locals.var_arg_dn13 = assign27460_e26280_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign27470_e26288, assign27470_e26288_d_n0, assign27470_e26288_d_n2, assign27470_e26288_d_n4, assign27470_e26288_d_n5, assign27470_e26288_d_n6, assign27470_e26288_d_n7, assign27470_e26288_d_n8, assign27470_e26288_d_n9, assign27470_e26288_d_n10, assign27470_e26288_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27470_e26288;
        locals.var_dnm_dn0 = assign27470_e26288_d_n0;
        locals.var_dnm_dn2 = assign27470_e26288_d_n2;
        locals.var_dnm_dn4 = assign27470_e26288_d_n4;
        locals.var_dnm_dn5 = assign27470_e26288_d_n5;
        locals.var_dnm_dn6 = assign27470_e26288_d_n6;
        locals.var_dnm_dn7 = assign27470_e26288_d_n7;
        locals.var_dnm_dn8 = assign27470_e26288_d_n8;
        locals.var_dnm_dn9 = assign27470_e26288_d_n9;
        locals.var_dnm_dn10 = assign27470_e26288_d_n10;
        locals.var_dnm_dn13 = assign27470_e26288_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign27480_e26303: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard650 = assign27480_e26303;
        locals.var_guard650_rv = 0.0;

        let assign27490_e26306: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign27490_e26306;
        locals.var_guard651_rv = 0.0;

        let (assign27500_e26318,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27500_e26318;
        locals.var_mm_rv = 0.0;

        let assign27510_e26321: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign27510_e26321;
        locals.var_guard652_rv = 0.0;

        let (assign27520_e26336,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27520_e26336;
        locals.var_mm_rv = 0.0;

        let assign27530_e26339: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign27530_e26339;
        locals.var_guard653_rv = 0.0;

        let (assign27540_e26357,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27540_e26357;
        locals.var_mm_rv = 0.0;

        let assign27550_e26360: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign27550_e26360;
        locals.var_guard654_rv = 0.0;

        let (assign27560_e26381,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) && (locals.var_guard654 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27560_e26381;
        locals.var_mm_rv = 0.0;

        let (assign27570_e26391,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27570_e26391;
        locals.var_m0_rv = 0.0;

        let mut assign27580_loop_guard: usize = 0;
        while {
            let assign27580_cond_e26402: f64 = if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27580_cond_e26402 != 0.0
        } {
            assign27580_loop_guard += 1;
            assert!(assign27580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27580_body0_e26413, assign27580_body0_e26413_d_n0, assign27580_body0_e26413_d_n2, assign27580_body0_e26413_d_n4, assign27580_body0_e26413_d_n5, assign27580_body0_e26413_d_n6, assign27580_body0_e26413_d_n7, assign27580_body0_e26413_d_n8, assign27580_body0_e26413_d_n9, assign27580_body0_e26413_d_n10, assign27580_body0_e26413_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign27580_body0_e26411: f64 = (locals.var_dnm).sqrt();
        (assign27580_body0_e26411, (locals.var_dnm_dn0 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn2 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn4 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn5 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn6 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn7 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn8 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn9 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn10 / (2.0 * assign27580_body0_e26411)), (locals.var_dnm_dn13 / (2.0 * assign27580_body0_e26411)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign27580_body0_e26413;
            locals.var_dnm_dn0 = assign27580_body0_e26413_d_n0;
            locals.var_dnm_dn2 = assign27580_body0_e26413_d_n2;
            locals.var_dnm_dn4 = assign27580_body0_e26413_d_n4;
            locals.var_dnm_dn5 = assign27580_body0_e26413_d_n5;
            locals.var_dnm_dn6 = assign27580_body0_e26413_d_n6;
            locals.var_dnm_dn7 = assign27580_body0_e26413_d_n7;
            locals.var_dnm_dn8 = assign27580_body0_e26413_d_n8;
            locals.var_dnm_dn9 = assign27580_body0_e26413_d_n9;
            locals.var_dnm_dn10 = assign27580_body0_e26413_d_n10;
            locals.var_dnm_dn13 = assign27580_body0_e26413_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign27580_body1_e26425,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign27580_body1_e26423: f64 = (locals.var_m0 + 1.0);
        (assign27580_body1_e26423,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27580_body1_e26425;
            locals.var_m0_rv = 0.0;
        }

        let (assign27590_e26447, assign27590_e26447_d_n0, assign27590_e26447_d_n2, assign27590_e26447_d_n4, assign27590_e26447_d_n5, assign27590_e26447_d_n6, assign27590_e26447_d_n7, assign27590_e26447_d_n8, assign27590_e26447_d_n9, assign27590_e26447_d_n10, assign27590_e26447_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 == 0.0)) {
        let (assign27590_e26445, assign27590_e26445_d_n0, assign27590_e26445_d_n2, assign27590_e26445_d_n4, assign27590_e26445_d_n5, assign27590_e26445_d_n6, assign27590_e26445_d_n7, assign27590_e26445_d_n8, assign27590_e26445_d_n9, assign27590_e26445_d_n10, assign27590_e26445_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27590_e26442: f64 = (2.0 * 2.0);
                let assign27590_e26443: f64 = (1.0 / assign27590_e26442);
                let assign27590_e26444: f64 = (locals.var_dnm).powf(assign27590_e26443);
                (assign27590_e26444, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27590_e26443) as f64).is_finite() && ((assign27590_e26443) as f64).fract() == 0.0 { if assign27590_e26443 == 0.0 { 0.0 } else { (assign27590_e26443 * ((locals.var_dnm).powf(assign27590_e26443 - 1.0) * locals.var_dnm_dn13)) } } else { (assign27590_e26444 * (assign27590_e26443 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign27590_e26445, assign27590_e26445_d_n0, assign27590_e26445_d_n2, assign27590_e26445_d_n4, assign27590_e26445_d_n5, assign27590_e26445_d_n6, assign27590_e26445_d_n7, assign27590_e26445_d_n8, assign27590_e26445_d_n9, assign27590_e26445_d_n10, assign27590_e26445_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27590_e26447;
        locals.var_dnm_dn0 = assign27590_e26447_d_n0;
        locals.var_dnm_dn2 = assign27590_e26447_d_n2;
        locals.var_dnm_dn4 = assign27590_e26447_d_n4;
        locals.var_dnm_dn5 = assign27590_e26447_d_n5;
        locals.var_dnm_dn6 = assign27590_e26447_d_n6;
        locals.var_dnm_dn7 = assign27590_e26447_d_n7;
        locals.var_dnm_dn8 = assign27590_e26447_d_n8;
        locals.var_dnm_dn9 = assign27590_e26447_d_n9;
        locals.var_dnm_dn10 = assign27590_e26447_d_n10;
        locals.var_dnm_dn13 = assign27590_e26447_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27600_e26457, assign27600_e26457_d_n0, assign27600_e26457_d_n2, assign27600_e26457_d_n4, assign27600_e26457_d_n5, assign27600_e26457_d_n6, assign27600_e26457_d_n7, assign27600_e26457_d_n8, assign27600_e26457_d_n9, assign27600_e26457_d_n10, assign27600_e26457_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27600_e26455: f64 = (1.0 / locals.var_dnm);
        (assign27600_e26455, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27600_e26457;
        locals.var_dnm_dn0 = assign27600_e26457_d_n0;
        locals.var_dnm_dn2 = assign27600_e26457_d_n2;
        locals.var_dnm_dn4 = assign27600_e26457_d_n4;
        locals.var_dnm_dn5 = assign27600_e26457_d_n5;
        locals.var_dnm_dn6 = assign27600_e26457_d_n6;
        locals.var_dnm_dn7 = assign27600_e26457_d_n7;
        locals.var_dnm_dn8 = assign27600_e26457_d_n8;
        locals.var_dnm_dn9 = assign27600_e26457_d_n9;
        locals.var_dnm_dn10 = assign27600_e26457_d_n10;
        locals.var_dnm_dn13 = assign27600_e26457_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27610_e26469, assign27610_e26469_d_n0, assign27610_e26469_d_n2, assign27610_e26469_d_n4, assign27610_e26469_d_n5, assign27610_e26469_d_n6, assign27610_e26469_d_n7, assign27610_e26469_d_n8, assign27610_e26469_d_n9, assign27610_e26469_d_n10, assign27610_e26469_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27610_e26465: f64 = (locals.var_tmf1 * 1e-8);
        let assign27610_e26467: f64 = (assign27610_e26465 * locals.var_dnm);
        (assign27610_e26467, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-8) * locals.var_dnm) + (assign27610_e26465 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign27610_e26469;
        locals.var_tmf0_dn0 = assign27610_e26469_d_n0;
        locals.var_tmf0_dn2 = assign27610_e26469_d_n2;
        locals.var_tmf0_dn4 = assign27610_e26469_d_n4;
        locals.var_tmf0_dn5 = assign27610_e26469_d_n5;
        locals.var_tmf0_dn6 = assign27610_e26469_d_n6;
        locals.var_tmf0_dn7 = assign27610_e26469_d_n7;
        locals.var_tmf0_dn8 = assign27610_e26469_d_n8;
        locals.var_tmf0_dn9 = assign27610_e26469_d_n9;
        locals.var_tmf0_dn10 = assign27610_e26469_d_n10;
        locals.var_tmf0_dn13 = assign27610_e26469_d_n13;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign27620_e26483, assign27620_e26483_d_n0, assign27620_e26483_d_n2, assign27620_e26483_d_n4, assign27620_e26483_d_n5, assign27620_e26483_d_n6, assign27620_e26483_d_n7, assign27620_e26483_d_n8, assign27620_e26483_d_n9, assign27620_e26483_d_n10, assign27620_e26483_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27620_e26477: f64 = (1e-8 * locals.var_xmp);
        let assign27620_e26479: f64 = (assign27620_e26477 * locals.var_dnm);
        let assign27620_e26481: f64 = (assign27620_e26479 / locals.var_arg);
        (assign27620_e26481, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign27620_e26477 * locals.var_dnm_dn13)) * locals.var_arg) - (assign27620_e26479 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign27620_e26483;
        locals.var_t3_dn0 = assign27620_e26483_d_n0;
        locals.var_t3_dn2 = assign27620_e26483_d_n2;
        locals.var_t3_dn4 = assign27620_e26483_d_n4;
        locals.var_t3_dn5 = assign27620_e26483_d_n5;
        locals.var_t3_dn6 = assign27620_e26483_d_n6;
        locals.var_t3_dn7 = assign27620_e26483_d_n7;
        locals.var_t3_dn8 = assign27620_e26483_d_n8;
        locals.var_t3_dn9 = assign27620_e26483_d_n9;
        locals.var_t3_dn10 = assign27620_e26483_d_n10;
        locals.var_t3_dn13 = assign27620_e26483_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign27630_e26495, assign27630_e26495_d_n0, assign27630_e26495_d_n2, assign27630_e26495_d_n4, assign27630_e26495_d_n5, assign27630_e26495_d_n6, assign27630_e26495_d_n7, assign27630_e26495_d_n8, assign27630_e26495_d_n9, assign27630_e26495_d_n10, assign27630_e26495_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign27630_e26491: f64 = (locals.var_uc_depthn - 1e-8);
        let assign27630_e26493: f64 = (assign27630_e26491 + locals.var_tmf0);
        (assign27630_e26493, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign27630_e26495;
        locals.var_w_b0_dn0 = assign27630_e26495_d_n0;
        locals.var_w_b0_dn2 = assign27630_e26495_d_n2;
        locals.var_w_b0_dn4 = assign27630_e26495_d_n4;
        locals.var_w_b0_dn5 = assign27630_e26495_d_n5;
        locals.var_w_b0_dn6 = assign27630_e26495_d_n6;
        locals.var_w_b0_dn7 = assign27630_e26495_d_n7;
        locals.var_w_b0_dn8 = assign27630_e26495_d_n8;
        locals.var_w_b0_dn9 = assign27630_e26495_d_n9;
        locals.var_w_b0_dn10 = assign27630_e26495_d_n10;
        locals.var_w_b0_dn13 = assign27630_e26495_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign27640_e26503, assign27640_e26503_d_n0, assign27640_e26503_d_n2, assign27640_e26503_d_n4, assign27640_e26503_d_n5, assign27640_e26503_d_n6, assign27640_e26503_d_n7, assign27640_e26503_d_n8, assign27640_e26503_d_n9, assign27640_e26503_d_n10, assign27640_e26503_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign27640_e26503;
        locals.var_t3_dn0 = assign27640_e26503_d_n0;
        locals.var_t3_dn2 = assign27640_e26503_d_n2;
        locals.var_t3_dn4 = assign27640_e26503_d_n4;
        locals.var_t3_dn5 = assign27640_e26503_d_n5;
        locals.var_t3_dn6 = assign27640_e26503_d_n6;
        locals.var_t3_dn7 = assign27640_e26503_d_n7;
        locals.var_t3_dn8 = assign27640_e26503_d_n8;
        locals.var_t3_dn9 = assign27640_e26503_d_n9;
        locals.var_t3_dn10 = assign27640_e26503_d_n10;
        locals.var_t3_dn13 = assign27640_e26503_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign27650_e26512, assign27650_e26512_d_n0, assign27650_e26512_d_n2, assign27650_e26512_d_n4, assign27650_e26512_d_n5, assign27650_e26512_d_n6, assign27650_e26512_d_n7, assign27650_e26512_d_n8, assign27650_e26512_d_n9, assign27650_e26512_d_n10, assign27650_e26512_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign27650_e26512;
        locals.var_w_b0_dn0 = assign27650_e26512_d_n0;
        locals.var_w_b0_dn2 = assign27650_e26512_d_n2;
        locals.var_w_b0_dn4 = assign27650_e26512_d_n4;
        locals.var_w_b0_dn5 = assign27650_e26512_d_n5;
        locals.var_w_b0_dn6 = assign27650_e26512_d_n6;
        locals.var_w_b0_dn7 = assign27650_e26512_d_n7;
        locals.var_w_b0_dn8 = assign27650_e26512_d_n8;
        locals.var_w_b0_dn9 = assign27650_e26512_d_n9;
        locals.var_w_b0_dn10 = assign27650_e26512_d_n10;
        locals.var_w_b0_dn13 = assign27650_e26512_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign27660_e26521, assign27660_e26521_d_n0, assign27660_e26521_d_n2, assign27660_e26521_d_n4, assign27660_e26521_d_n5, assign27660_e26521_d_n6, assign27660_e26521_d_n7, assign27660_e26521_d_n8, assign27660_e26521_d_n9, assign27660_e26521_d_n10, assign27660_e26521_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard649 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign27660_e26521;
        locals.var_t3_dn0 = assign27660_e26521_d_n0;
        locals.var_t3_dn2 = assign27660_e26521_d_n2;
        locals.var_t3_dn4 = assign27660_e26521_d_n4;
        locals.var_t3_dn5 = assign27660_e26521_d_n5;
        locals.var_t3_dn6 = assign27660_e26521_d_n6;
        locals.var_t3_dn7 = assign27660_e26521_d_n7;
        locals.var_t3_dn8 = assign27660_e26521_d_n8;
        locals.var_t3_dn9 = assign27660_e26521_d_n9;
        locals.var_t3_dn10 = assign27660_e26521_d_n10;
        locals.var_t3_dn13 = assign27660_e26521_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign27670_e26529, assign27670_e26529_d_n0, assign27670_e26529_d_n2, assign27670_e26529_d_n4, assign27670_e26529_d_n5, assign27670_e26529_d_n6, assign27670_e26529_d_n7, assign27670_e26529_d_n8, assign27670_e26529_d_n9, assign27670_e26529_d_n10, assign27670_e26529_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign27670_e26527: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep);
        (assign27670_e26527, (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_b0_dep_dn13 - locals.var_phi_s0_dep_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign27670_e26529;
        locals.var_t1_dn0 = assign27670_e26529_d_n0;
        locals.var_t1_dn2 = assign27670_e26529_d_n2;
        locals.var_t1_dn4 = assign27670_e26529_d_n4;
        locals.var_t1_dn5 = assign27670_e26529_d_n5;
        locals.var_t1_dn6 = assign27670_e26529_d_n6;
        locals.var_t1_dn7 = assign27670_e26529_d_n7;
        locals.var_t1_dn8 = assign27670_e26529_d_n8;
        locals.var_t1_dn9 = assign27670_e26529_d_n9;
        locals.var_t1_dn10 = assign27670_e26529_d_n10;
        locals.var_t1_dn13 = assign27670_e26529_d_n13;
        locals.var_t1_rv = 0.0;

        let assign27680_e26533: f64 = 0.05;
        let assign27680_e26538: f64 = if ((locals.var_t1 < assign27680_e26533) && (0.05 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard655 = assign27680_e26538;
        locals.var_guard655_rv = 0.0;

        let (assign27690_e26550, assign27690_e26550_d_n0, assign27690_e26550_d_n2, assign27690_e26550_d_n4, assign27690_e26550_d_n5, assign27690_e26550_d_n6, assign27690_e26550_d_n7, assign27690_e26550_d_n8, assign27690_e26550_d_n9, assign27690_e26550_d_n10, assign27690_e26550_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27690_e26546: f64 = 0.05;
        let assign27690_e26548: f64 = (assign27690_e26546 - locals.var_t1);
        (assign27690_e26548, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign27690_e26550;
        locals.var_tmf1_dn0 = assign27690_e26550_d_n0;
        locals.var_tmf1_dn2 = assign27690_e26550_d_n2;
        locals.var_tmf1_dn4 = assign27690_e26550_d_n4;
        locals.var_tmf1_dn5 = assign27690_e26550_d_n5;
        locals.var_tmf1_dn6 = assign27690_e26550_d_n6;
        locals.var_tmf1_dn7 = assign27690_e26550_d_n7;
        locals.var_tmf1_dn8 = assign27690_e26550_d_n8;
        locals.var_tmf1_dn9 = assign27690_e26550_d_n9;
        locals.var_tmf1_dn10 = assign27690_e26550_d_n10;
        locals.var_tmf1_dn13 = assign27690_e26550_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign27700_e26560, assign27700_e26560_d_n0, assign27700_e26560_d_n2, assign27700_e26560_d_n4, assign27700_e26560_d_n5, assign27700_e26560_d_n6, assign27700_e26560_d_n7, assign27700_e26560_d_n8, assign27700_e26560_d_n9, assign27700_e26560_d_n10, assign27700_e26560_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27700_e26558: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27700_e26558, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign27700_e26560;
        locals.var_x2_dn0 = assign27700_e26560_d_n0;
        locals.var_x2_dn2 = assign27700_e26560_d_n2;
        locals.var_x2_dn4 = assign27700_e26560_d_n4;
        locals.var_x2_dn5 = assign27700_e26560_d_n5;
        locals.var_x2_dn6 = assign27700_e26560_d_n6;
        locals.var_x2_dn7 = assign27700_e26560_d_n7;
        locals.var_x2_dn8 = assign27700_e26560_d_n8;
        locals.var_x2_dn9 = assign27700_e26560_d_n9;
        locals.var_x2_dn10 = assign27700_e26560_d_n10;
        locals.var_x2_dn13 = assign27700_e26560_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign27710_e26570, assign27710_e26570_d_n0, assign27710_e26570_d_n2, assign27710_e26570_d_n4, assign27710_e26570_d_n5, assign27710_e26570_d_n6, assign27710_e26570_d_n7, assign27710_e26570_d_n8, assign27710_e26570_d_n9, assign27710_e26570_d_n10, assign27710_e26570_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27710_e26568: f64 = (0.05 * 0.05);
        (assign27710_e26568, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign27710_e26570;
        locals.var_xmax2_dn0 = assign27710_e26570_d_n0;
        locals.var_xmax2_dn2 = assign27710_e26570_d_n2;
        locals.var_xmax2_dn4 = assign27710_e26570_d_n4;
        locals.var_xmax2_dn5 = assign27710_e26570_d_n5;
        locals.var_xmax2_dn6 = assign27710_e26570_d_n6;
        locals.var_xmax2_dn7 = assign27710_e26570_d_n7;
        locals.var_xmax2_dn8 = assign27710_e26570_d_n8;
        locals.var_xmax2_dn9 = assign27710_e26570_d_n9;
        locals.var_xmax2_dn10 = assign27710_e26570_d_n10;
        locals.var_xmax2_dn13 = assign27710_e26570_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign27720_e26578, assign27720_e26578_d_n0, assign27720_e26578_d_n2, assign27720_e26578_d_n4, assign27720_e26578_d_n5, assign27720_e26578_d_n6, assign27720_e26578_d_n7, assign27720_e26578_d_n8, assign27720_e26578_d_n9, assign27720_e26578_d_n10, assign27720_e26578_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27720_e26578;
        locals.var_xp_dn0 = assign27720_e26578_d_n0;
        locals.var_xp_dn2 = assign27720_e26578_d_n2;
        locals.var_xp_dn4 = assign27720_e26578_d_n4;
        locals.var_xp_dn5 = assign27720_e26578_d_n5;
        locals.var_xp_dn6 = assign27720_e26578_d_n6;
        locals.var_xp_dn7 = assign27720_e26578_d_n7;
        locals.var_xp_dn8 = assign27720_e26578_d_n8;
        locals.var_xp_dn9 = assign27720_e26578_d_n9;
        locals.var_xp_dn10 = assign27720_e26578_d_n10;
        locals.var_xp_dn13 = assign27720_e26578_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27730_e26586, assign27730_e26586_d_n0, assign27730_e26586_d_n2, assign27730_e26586_d_n4, assign27730_e26586_d_n5, assign27730_e26586_d_n6, assign27730_e26586_d_n7, assign27730_e26586_d_n8, assign27730_e26586_d_n9, assign27730_e26586_d_n10, assign27730_e26586_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27730_e26586;
        locals.var_xmp_dn0 = assign27730_e26586_d_n0;
        locals.var_xmp_dn2 = assign27730_e26586_d_n2;
        locals.var_xmp_dn4 = assign27730_e26586_d_n4;
        locals.var_xmp_dn5 = assign27730_e26586_d_n5;
        locals.var_xmp_dn6 = assign27730_e26586_d_n6;
        locals.var_xmp_dn7 = assign27730_e26586_d_n7;
        locals.var_xmp_dn8 = assign27730_e26586_d_n8;
        locals.var_xmp_dn9 = assign27730_e26586_d_n9;
        locals.var_xmp_dn10 = assign27730_e26586_d_n10;
        locals.var_xmp_dn13 = assign27730_e26586_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27740_e26594,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27740_e26594;
        locals.var_m0_rv = 0.0;

        let (assign27750_e26602,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27750_e26602;
        locals.var_mm_rv = 0.0;

        let (assign27760_e26610, assign27760_e26610_d_n0, assign27760_e26610_d_n2, assign27760_e26610_d_n4, assign27760_e26610_d_n5, assign27760_e26610_d_n6, assign27760_e26610_d_n7, assign27760_e26610_d_n8, assign27760_e26610_d_n9, assign27760_e26610_d_n10, assign27760_e26610_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign27760_e26610;
        locals.var_arg_dn0 = assign27760_e26610_d_n0;
        locals.var_arg_dn2 = assign27760_e26610_d_n2;
        locals.var_arg_dn4 = assign27760_e26610_d_n4;
        locals.var_arg_dn5 = assign27760_e26610_d_n5;
        locals.var_arg_dn6 = assign27760_e26610_d_n6;
        locals.var_arg_dn7 = assign27760_e26610_d_n7;
        locals.var_arg_dn8 = assign27760_e26610_d_n8;
        locals.var_arg_dn9 = assign27760_e26610_d_n9;
        locals.var_arg_dn10 = assign27760_e26610_d_n10;
        locals.var_arg_dn13 = assign27760_e26610_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign27770_e26618, assign27770_e26618_d_n0, assign27770_e26618_d_n2, assign27770_e26618_d_n4, assign27770_e26618_d_n5, assign27770_e26618_d_n6, assign27770_e26618_d_n7, assign27770_e26618_d_n8, assign27770_e26618_d_n9, assign27770_e26618_d_n10, assign27770_e26618_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27770_e26618;
        locals.var_dnm_dn0 = assign27770_e26618_d_n0;
        locals.var_dnm_dn2 = assign27770_e26618_d_n2;
        locals.var_dnm_dn4 = assign27770_e26618_d_n4;
        locals.var_dnm_dn5 = assign27770_e26618_d_n5;
        locals.var_dnm_dn6 = assign27770_e26618_d_n6;
        locals.var_dnm_dn7 = assign27770_e26618_d_n7;
        locals.var_dnm_dn8 = assign27770_e26618_d_n8;
        locals.var_dnm_dn9 = assign27770_e26618_d_n9;
        locals.var_dnm_dn10 = assign27770_e26618_d_n10;
        locals.var_dnm_dn13 = assign27770_e26618_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27780_e26628, assign27780_e26628_d_n0, assign27780_e26628_d_n2, assign27780_e26628_d_n4, assign27780_e26628_d_n5, assign27780_e26628_d_n6, assign27780_e26628_d_n7, assign27780_e26628_d_n8, assign27780_e26628_d_n9, assign27780_e26628_d_n10, assign27780_e26628_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27780_e26626: f64 = (locals.var_xp * locals.var_x2);
        (assign27780_e26626, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27780_e26628;
        locals.var_xp_dn0 = assign27780_e26628_d_n0;
        locals.var_xp_dn2 = assign27780_e26628_d_n2;
        locals.var_xp_dn4 = assign27780_e26628_d_n4;
        locals.var_xp_dn5 = assign27780_e26628_d_n5;
        locals.var_xp_dn6 = assign27780_e26628_d_n6;
        locals.var_xp_dn7 = assign27780_e26628_d_n7;
        locals.var_xp_dn8 = assign27780_e26628_d_n8;
        locals.var_xp_dn9 = assign27780_e26628_d_n9;
        locals.var_xp_dn10 = assign27780_e26628_d_n10;
        locals.var_xp_dn13 = assign27780_e26628_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27790_e26638, assign27790_e26638_d_n0, assign27790_e26638_d_n2, assign27790_e26638_d_n4, assign27790_e26638_d_n5, assign27790_e26638_d_n6, assign27790_e26638_d_n7, assign27790_e26638_d_n8, assign27790_e26638_d_n9, assign27790_e26638_d_n10, assign27790_e26638_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27790_e26636: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27790_e26636, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27790_e26638;
        locals.var_xmp_dn0 = assign27790_e26638_d_n0;
        locals.var_xmp_dn2 = assign27790_e26638_d_n2;
        locals.var_xmp_dn4 = assign27790_e26638_d_n4;
        locals.var_xmp_dn5 = assign27790_e26638_d_n5;
        locals.var_xmp_dn6 = assign27790_e26638_d_n6;
        locals.var_xmp_dn7 = assign27790_e26638_d_n7;
        locals.var_xmp_dn8 = assign27790_e26638_d_n8;
        locals.var_xmp_dn9 = assign27790_e26638_d_n9;
        locals.var_xmp_dn10 = assign27790_e26638_d_n10;
        locals.var_xmp_dn13 = assign27790_e26638_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27800_e26648, assign27800_e26648_d_n0, assign27800_e26648_d_n2, assign27800_e26648_d_n4, assign27800_e26648_d_n5, assign27800_e26648_d_n6, assign27800_e26648_d_n7, assign27800_e26648_d_n8, assign27800_e26648_d_n9, assign27800_e26648_d_n10, assign27800_e26648_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27800_e26646: f64 = (locals.var_xp * locals.var_x2);
        (assign27800_e26646, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign27800_e26648;
        locals.var_xp_dn0 = assign27800_e26648_d_n0;
        locals.var_xp_dn2 = assign27800_e26648_d_n2;
        locals.var_xp_dn4 = assign27800_e26648_d_n4;
        locals.var_xp_dn5 = assign27800_e26648_d_n5;
        locals.var_xp_dn6 = assign27800_e26648_d_n6;
        locals.var_xp_dn7 = assign27800_e26648_d_n7;
        locals.var_xp_dn8 = assign27800_e26648_d_n8;
        locals.var_xp_dn9 = assign27800_e26648_d_n9;
        locals.var_xp_dn10 = assign27800_e26648_d_n10;
        locals.var_xp_dn13 = assign27800_e26648_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign27810_e26658, assign27810_e26658_d_n0, assign27810_e26658_d_n2, assign27810_e26658_d_n4, assign27810_e26658_d_n5, assign27810_e26658_d_n6, assign27810_e26658_d_n7, assign27810_e26658_d_n8, assign27810_e26658_d_n9, assign27810_e26658_d_n10, assign27810_e26658_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27810_e26656: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27810_e26656, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign27810_e26658;
        locals.var_xmp_dn0 = assign27810_e26658_d_n0;
        locals.var_xmp_dn2 = assign27810_e26658_d_n2;
        locals.var_xmp_dn4 = assign27810_e26658_d_n4;
        locals.var_xmp_dn5 = assign27810_e26658_d_n5;
        locals.var_xmp_dn6 = assign27810_e26658_d_n6;
        locals.var_xmp_dn7 = assign27810_e26658_d_n7;
        locals.var_xmp_dn8 = assign27810_e26658_d_n8;
        locals.var_xmp_dn9 = assign27810_e26658_d_n9;
        locals.var_xmp_dn10 = assign27810_e26658_d_n10;
        locals.var_xmp_dn13 = assign27810_e26658_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign27820_e26668, assign27820_e26668_d_n0, assign27820_e26668_d_n2, assign27820_e26668_d_n4, assign27820_e26668_d_n5, assign27820_e26668_d_n6, assign27820_e26668_d_n7, assign27820_e26668_d_n8, assign27820_e26668_d_n9, assign27820_e26668_d_n10, assign27820_e26668_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27820_e26666: f64 = (locals.var_xp + locals.var_xmp);
        (assign27820_e26666, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign27820_e26668;
        locals.var_arg_dn0 = assign27820_e26668_d_n0;
        locals.var_arg_dn2 = assign27820_e26668_d_n2;
        locals.var_arg_dn4 = assign27820_e26668_d_n4;
        locals.var_arg_dn5 = assign27820_e26668_d_n5;
        locals.var_arg_dn6 = assign27820_e26668_d_n6;
        locals.var_arg_dn7 = assign27820_e26668_d_n7;
        locals.var_arg_dn8 = assign27820_e26668_d_n8;
        locals.var_arg_dn9 = assign27820_e26668_d_n9;
        locals.var_arg_dn10 = assign27820_e26668_d_n10;
        locals.var_arg_dn13 = assign27820_e26668_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign27830_e26676, assign27830_e26676_d_n0, assign27830_e26676_d_n2, assign27830_e26676_d_n4, assign27830_e26676_d_n5, assign27830_e26676_d_n6, assign27830_e26676_d_n7, assign27830_e26676_d_n8, assign27830_e26676_d_n9, assign27830_e26676_d_n10, assign27830_e26676_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27830_e26676;
        locals.var_dnm_dn0 = assign27830_e26676_d_n0;
        locals.var_dnm_dn2 = assign27830_e26676_d_n2;
        locals.var_dnm_dn4 = assign27830_e26676_d_n4;
        locals.var_dnm_dn5 = assign27830_e26676_d_n5;
        locals.var_dnm_dn6 = assign27830_e26676_d_n6;
        locals.var_dnm_dn7 = assign27830_e26676_d_n7;
        locals.var_dnm_dn8 = assign27830_e26676_d_n8;
        locals.var_dnm_dn9 = assign27830_e26676_d_n9;
        locals.var_dnm_dn10 = assign27830_e26676_d_n10;
        locals.var_dnm_dn13 = assign27830_e26676_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign27840_e26691: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard656 = assign27840_e26691;
        locals.var_guard656_rv = 0.0;

        let assign27850_e26694: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign27850_e26694;
        locals.var_guard657_rv = 0.0;

        let (assign27860_e26706,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27860_e26706;
        locals.var_mm_rv = 0.0;

        let assign27870_e26709: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign27870_e26709;
        locals.var_guard658_rv = 0.0;

        let (assign27880_e26724,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27880_e26724;
        locals.var_mm_rv = 0.0;

        let assign27890_e26727: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign27890_e26727;
        locals.var_guard659_rv = 0.0;

        let (assign27900_e26745,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 == 0.0)) && (locals.var_guard659 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27900_e26745;
        locals.var_mm_rv = 0.0;

        let assign27910_e26748: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign27910_e26748;
        locals.var_guard660_rv = 0.0;

        let (assign27920_e26769,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 == 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27920_e26769;
        locals.var_mm_rv = 0.0;

        let (assign27930_e26779,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27930_e26779;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        locals: &mut StampLocals,
    ) {
        let mut assign27940_loop_guard: usize = 0;
        while {
            let assign27940_cond_e26790: f64 = if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27940_cond_e26790 != 0.0
        } {
            assign27940_loop_guard += 1;
            assert!(assign27940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27940_body0_e26801, assign27940_body0_e26801_d_n0, assign27940_body0_e26801_d_n2, assign27940_body0_e26801_d_n4, assign27940_body0_e26801_d_n5, assign27940_body0_e26801_d_n6, assign27940_body0_e26801_d_n7, assign27940_body0_e26801_d_n8, assign27940_body0_e26801_d_n9, assign27940_body0_e26801_d_n10, assign27940_body0_e26801_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) {
        let assign27940_body0_e26799: f64 = (locals.var_dnm).sqrt();
        (assign27940_body0_e26799, (locals.var_dnm_dn0 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn2 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn4 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn5 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn6 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn7 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn8 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn9 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn10 / (2.0 * assign27940_body0_e26799)), (locals.var_dnm_dn13 / (2.0 * assign27940_body0_e26799)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign27940_body0_e26801;
            locals.var_dnm_dn0 = assign27940_body0_e26801_d_n0;
            locals.var_dnm_dn2 = assign27940_body0_e26801_d_n2;
            locals.var_dnm_dn4 = assign27940_body0_e26801_d_n4;
            locals.var_dnm_dn5 = assign27940_body0_e26801_d_n5;
            locals.var_dnm_dn6 = assign27940_body0_e26801_d_n6;
            locals.var_dnm_dn7 = assign27940_body0_e26801_d_n7;
            locals.var_dnm_dn8 = assign27940_body0_e26801_d_n8;
            locals.var_dnm_dn9 = assign27940_body0_e26801_d_n9;
            locals.var_dnm_dn10 = assign27940_body0_e26801_d_n10;
            locals.var_dnm_dn13 = assign27940_body0_e26801_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign27940_body1_e26813,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 != 0.0)) {
        let assign27940_body1_e26811: f64 = (locals.var_m0 + 1.0);
        (assign27940_body1_e26811,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27940_body1_e26813;
            locals.var_m0_rv = 0.0;
        }

        let (assign27950_e26835, assign27950_e26835_d_n0, assign27950_e26835_d_n2, assign27950_e26835_d_n4, assign27950_e26835_d_n5, assign27950_e26835_d_n6, assign27950_e26835_d_n7, assign27950_e26835_d_n8, assign27950_e26835_d_n9, assign27950_e26835_d_n10, assign27950_e26835_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) && (locals.var_guard656 == 0.0)) {
        let (assign27950_e26833, assign27950_e26833_d_n0, assign27950_e26833_d_n2, assign27950_e26833_d_n4, assign27950_e26833_d_n5, assign27950_e26833_d_n6, assign27950_e26833_d_n7, assign27950_e26833_d_n8, assign27950_e26833_d_n9, assign27950_e26833_d_n10, assign27950_e26833_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27950_e26830: f64 = (2.0 * 2.0);
                let assign27950_e26831: f64 = (1.0 / assign27950_e26830);
                let assign27950_e26832: f64 = (locals.var_dnm).powf(assign27950_e26831);
                (assign27950_e26832, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27950_e26831) as f64).is_finite() && ((assign27950_e26831) as f64).fract() == 0.0 { if assign27950_e26831 == 0.0 { 0.0 } else { (assign27950_e26831 * ((locals.var_dnm).powf(assign27950_e26831 - 1.0) * locals.var_dnm_dn13)) } } else { (assign27950_e26832 * (assign27950_e26831 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign27950_e26833, assign27950_e26833_d_n0, assign27950_e26833_d_n2, assign27950_e26833_d_n4, assign27950_e26833_d_n5, assign27950_e26833_d_n6, assign27950_e26833_d_n7, assign27950_e26833_d_n8, assign27950_e26833_d_n9, assign27950_e26833_d_n10, assign27950_e26833_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27950_e26835;
        locals.var_dnm_dn0 = assign27950_e26835_d_n0;
        locals.var_dnm_dn2 = assign27950_e26835_d_n2;
        locals.var_dnm_dn4 = assign27950_e26835_d_n4;
        locals.var_dnm_dn5 = assign27950_e26835_d_n5;
        locals.var_dnm_dn6 = assign27950_e26835_d_n6;
        locals.var_dnm_dn7 = assign27950_e26835_d_n7;
        locals.var_dnm_dn8 = assign27950_e26835_d_n8;
        locals.var_dnm_dn9 = assign27950_e26835_d_n9;
        locals.var_dnm_dn10 = assign27950_e26835_d_n10;
        locals.var_dnm_dn13 = assign27950_e26835_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27960_e26845, assign27960_e26845_d_n0, assign27960_e26845_d_n2, assign27960_e26845_d_n4, assign27960_e26845_d_n5, assign27960_e26845_d_n6, assign27960_e26845_d_n7, assign27960_e26845_d_n8, assign27960_e26845_d_n9, assign27960_e26845_d_n10, assign27960_e26845_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27960_e26843: f64 = (1.0 / locals.var_dnm);
        (assign27960_e26843, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign27960_e26845;
        locals.var_dnm_dn0 = assign27960_e26845_d_n0;
        locals.var_dnm_dn2 = assign27960_e26845_d_n2;
        locals.var_dnm_dn4 = assign27960_e26845_d_n4;
        locals.var_dnm_dn5 = assign27960_e26845_d_n5;
        locals.var_dnm_dn6 = assign27960_e26845_d_n6;
        locals.var_dnm_dn7 = assign27960_e26845_d_n7;
        locals.var_dnm_dn8 = assign27960_e26845_d_n8;
        locals.var_dnm_dn9 = assign27960_e26845_d_n9;
        locals.var_dnm_dn10 = assign27960_e26845_d_n10;
        locals.var_dnm_dn13 = assign27960_e26845_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign27970_e26857, assign27970_e26857_d_n0, assign27970_e26857_d_n2, assign27970_e26857_d_n4, assign27970_e26857_d_n5, assign27970_e26857_d_n6, assign27970_e26857_d_n7, assign27970_e26857_d_n8, assign27970_e26857_d_n9, assign27970_e26857_d_n10, assign27970_e26857_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27970_e26853: f64 = (locals.var_tmf1 * 0.05);
        let assign27970_e26855: f64 = (assign27970_e26853 * locals.var_dnm);
        (assign27970_e26855, (((locals.var_tmf1_dn0 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.05) * locals.var_dnm) + (assign27970_e26853 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign27970_e26857;
        locals.var_tmf0_dn0 = assign27970_e26857_d_n0;
        locals.var_tmf0_dn2 = assign27970_e26857_d_n2;
        locals.var_tmf0_dn4 = assign27970_e26857_d_n4;
        locals.var_tmf0_dn5 = assign27970_e26857_d_n5;
        locals.var_tmf0_dn6 = assign27970_e26857_d_n6;
        locals.var_tmf0_dn7 = assign27970_e26857_d_n7;
        locals.var_tmf0_dn8 = assign27970_e26857_d_n8;
        locals.var_tmf0_dn9 = assign27970_e26857_d_n9;
        locals.var_tmf0_dn10 = assign27970_e26857_d_n10;
        locals.var_tmf0_dn13 = assign27970_e26857_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign27980_e26871, assign27980_e26871_d_n0, assign27980_e26871_d_n2, assign27980_e26871_d_n4, assign27980_e26871_d_n5, assign27980_e26871_d_n6, assign27980_e26871_d_n7, assign27980_e26871_d_n8, assign27980_e26871_d_n9, assign27980_e26871_d_n10, assign27980_e26871_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27980_e26865: f64 = (0.05 * locals.var_xmp);
        let assign27980_e26867: f64 = (assign27980_e26865 * locals.var_dnm);
        let assign27980_e26869: f64 = (assign27980_e26867 / locals.var_arg);
        (assign27980_e26869, ((((((0.05 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn13) * locals.var_dnm) + (assign27980_e26865 * locals.var_dnm_dn13)) * locals.var_arg) - (assign27980_e26867 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign27980_e26871;
        locals.var_t0_dn0 = assign27980_e26871_d_n0;
        locals.var_t0_dn2 = assign27980_e26871_d_n2;
        locals.var_t0_dn4 = assign27980_e26871_d_n4;
        locals.var_t0_dn5 = assign27980_e26871_d_n5;
        locals.var_t0_dn6 = assign27980_e26871_d_n6;
        locals.var_t0_dn7 = assign27980_e26871_d_n7;
        locals.var_t0_dn8 = assign27980_e26871_d_n8;
        locals.var_t0_dn9 = assign27980_e26871_d_n9;
        locals.var_t0_dn10 = assign27980_e26871_d_n10;
        locals.var_t0_dn13 = assign27980_e26871_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign27990_e26883, assign27990_e26883_d_n0, assign27990_e26883_d_n2, assign27990_e26883_d_n4, assign27990_e26883_d_n5, assign27990_e26883_d_n6, assign27990_e26883_d_n7, assign27990_e26883_d_n8, assign27990_e26883_d_n9, assign27990_e26883_d_n10, assign27990_e26883_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign27990_e26879: f64 = 0.05;
        let assign27990_e26881: f64 = (assign27990_e26879 - locals.var_tmf0);
        (assign27990_e26881, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign27990_e26883;
        locals.var_t2_dn0 = assign27990_e26883_d_n0;
        locals.var_t2_dn2 = assign27990_e26883_d_n2;
        locals.var_t2_dn4 = assign27990_e26883_d_n4;
        locals.var_t2_dn5 = assign27990_e26883_d_n5;
        locals.var_t2_dn6 = assign27990_e26883_d_n6;
        locals.var_t2_dn7 = assign27990_e26883_d_n7;
        locals.var_t2_dn8 = assign27990_e26883_d_n8;
        locals.var_t2_dn9 = assign27990_e26883_d_n9;
        locals.var_t2_dn10 = assign27990_e26883_d_n10;
        locals.var_t2_dn13 = assign27990_e26883_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign28000_e26891, assign28000_e26891_d_n0, assign28000_e26891_d_n2, assign28000_e26891_d_n4, assign28000_e26891_d_n5, assign28000_e26891_d_n6, assign28000_e26891_d_n7, assign28000_e26891_d_n8, assign28000_e26891_d_n9, assign28000_e26891_d_n10, assign28000_e26891_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign28000_e26891;
        locals.var_t0_dn0 = assign28000_e26891_d_n0;
        locals.var_t0_dn2 = assign28000_e26891_d_n2;
        locals.var_t0_dn4 = assign28000_e26891_d_n4;
        locals.var_t0_dn5 = assign28000_e26891_d_n5;
        locals.var_t0_dn6 = assign28000_e26891_d_n6;
        locals.var_t0_dn7 = assign28000_e26891_d_n7;
        locals.var_t0_dn8 = assign28000_e26891_d_n8;
        locals.var_t0_dn9 = assign28000_e26891_d_n9;
        locals.var_t0_dn10 = assign28000_e26891_d_n10;
        locals.var_t0_dn13 = assign28000_e26891_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign28010_e26900, assign28010_e26900_d_n0, assign28010_e26900_d_n2, assign28010_e26900_d_n4, assign28010_e26900_d_n5, assign28010_e26900_d_n6, assign28010_e26900_d_n7, assign28010_e26900_d_n8, assign28010_e26900_d_n9, assign28010_e26900_d_n10, assign28010_e26900_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign28010_e26900;
        locals.var_t2_dn0 = assign28010_e26900_d_n0;
        locals.var_t2_dn2 = assign28010_e26900_d_n2;
        locals.var_t2_dn4 = assign28010_e26900_d_n4;
        locals.var_t2_dn5 = assign28010_e26900_d_n5;
        locals.var_t2_dn6 = assign28010_e26900_d_n6;
        locals.var_t2_dn7 = assign28010_e26900_d_n7;
        locals.var_t2_dn8 = assign28010_e26900_d_n8;
        locals.var_t2_dn9 = assign28010_e26900_d_n9;
        locals.var_t2_dn10 = assign28010_e26900_d_n10;
        locals.var_t2_dn13 = assign28010_e26900_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign28020_e26909, assign28020_e26909_d_n0, assign28020_e26909_d_n2, assign28020_e26909_d_n4, assign28020_e26909_d_n5, assign28020_e26909_d_n6, assign28020_e26909_d_n7, assign28020_e26909_d_n8, assign28020_e26909_d_n9, assign28020_e26909_d_n10, assign28020_e26909_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard655 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign28020_e26909;
        locals.var_t0_dn0 = assign28020_e26909_d_n0;
        locals.var_t0_dn2 = assign28020_e26909_d_n2;
        locals.var_t0_dn4 = assign28020_e26909_d_n4;
        locals.var_t0_dn5 = assign28020_e26909_d_n5;
        locals.var_t0_dn6 = assign28020_e26909_d_n6;
        locals.var_t0_dn7 = assign28020_e26909_d_n7;
        locals.var_t0_dn8 = assign28020_e26909_d_n8;
        locals.var_t0_dn9 = assign28020_e26909_d_n9;
        locals.var_t0_dn10 = assign28020_e26909_d_n10;
        locals.var_t0_dn13 = assign28020_e26909_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign28030_e26918, assign28030_e26918_d_n0, assign28030_e26918_d_n2, assign28030_e26918_d_n4, assign28030_e26918_d_n5, assign28030_e26918_d_n6, assign28030_e26918_d_n7, assign28030_e26918_d_n8, assign28030_e26918_d_n9, assign28030_e26918_d_n10, assign28030_e26918_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign28030_e26915: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign28030_e26916: f64 = (assign28030_e26915).sqrt();
        (assign28030_e26916, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign28030_e26916)), (((locals.var_c_2esipq_ndepm_dn13 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn13)) / (2.0 * assign28030_e26916)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn13,)
    }
};
        locals.var_w_s0 = assign28030_e26918;
        locals.var_w_s0_dn0 = assign28030_e26918_d_n0;
        locals.var_w_s0_dn2 = assign28030_e26918_d_n2;
        locals.var_w_s0_dn4 = assign28030_e26918_d_n4;
        locals.var_w_s0_dn5 = assign28030_e26918_d_n5;
        locals.var_w_s0_dn6 = assign28030_e26918_d_n6;
        locals.var_w_s0_dn7 = assign28030_e26918_d_n7;
        locals.var_w_s0_dn8 = assign28030_e26918_d_n8;
        locals.var_w_s0_dn9 = assign28030_e26918_d_n9;
        locals.var_w_s0_dn10 = assign28030_e26918_d_n10;
        locals.var_w_s0_dn13 = assign28030_e26918_d_n13;
        locals.var_w_s0_rv = 0.0;

        let (assign28040_e26928, assign28040_e26928_d_n0, assign28040_e26928_d_n2, assign28040_e26928_d_n4, assign28040_e26928_d_n5, assign28040_e26928_d_n6, assign28040_e26928_d_n7, assign28040_e26928_d_n8, assign28040_e26928_d_n9, assign28040_e26928_d_n10, assign28040_e26928_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign28040_e26924: f64 = (locals.var_uc_depthn - locals.var_w_b0);
        let assign28040_e26926: f64 = (assign28040_e26924 - locals.var_w_s0);
        (assign28040_e26926, ((locals.var_uc_depthn_dn0 - locals.var_w_b0_dn0) - locals.var_w_s0_dn0), ((locals.var_uc_depthn_dn2 - locals.var_w_b0_dn2) - locals.var_w_s0_dn2), ((locals.var_uc_depthn_dn4 - locals.var_w_b0_dn4) - locals.var_w_s0_dn4), ((locals.var_uc_depthn_dn5 - locals.var_w_b0_dn5) - locals.var_w_s0_dn5), ((locals.var_uc_depthn_dn6 - locals.var_w_b0_dn6) - locals.var_w_s0_dn6), ((locals.var_uc_depthn_dn7 - locals.var_w_b0_dn7) - locals.var_w_s0_dn7), ((locals.var_uc_depthn_dn8 - locals.var_w_b0_dn8) - locals.var_w_s0_dn8), ((locals.var_uc_depthn_dn9 - locals.var_w_b0_dn9) - locals.var_w_s0_dn9), ((locals.var_uc_depthn_dn10 - locals.var_w_b0_dn10) - locals.var_w_s0_dn10), ((locals.var_uc_depthn_dn13 - locals.var_w_b0_dn13) - locals.var_w_s0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign28040_e26928;
        locals.var_t1_dn0 = assign28040_e26928_d_n0;
        locals.var_t1_dn2 = assign28040_e26928_d_n2;
        locals.var_t1_dn4 = assign28040_e26928_d_n4;
        locals.var_t1_dn5 = assign28040_e26928_d_n5;
        locals.var_t1_dn6 = assign28040_e26928_d_n6;
        locals.var_t1_dn7 = assign28040_e26928_d_n7;
        locals.var_t1_dn8 = assign28040_e26928_d_n8;
        locals.var_t1_dn9 = assign28040_e26928_d_n9;
        locals.var_t1_dn10 = assign28040_e26928_d_n10;
        locals.var_t1_dn13 = assign28040_e26928_d_n13;
        locals.var_t1_rv = 0.0;

        let assign28050_e26932: f64 = (1e-25 + 1e-18);
        let assign28050_e26937: f64 = if ((locals.var_t1 < assign28050_e26932) && (1e-18 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard661 = assign28050_e26937;
        locals.var_guard661_rv = 0.0;

        let (assign28060_e26949, assign28060_e26949_d_n0, assign28060_e26949_d_n2, assign28060_e26949_d_n4, assign28060_e26949_d_n5, assign28060_e26949_d_n6, assign28060_e26949_d_n7, assign28060_e26949_d_n8, assign28060_e26949_d_n9, assign28060_e26949_d_n10, assign28060_e26949_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28060_e26945: f64 = (1e-25 + 1e-18);
        let assign28060_e26947: f64 = (assign28060_e26945 - locals.var_t1);
        (assign28060_e26947, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign28060_e26949;
        locals.var_tmf1_dn0 = assign28060_e26949_d_n0;
        locals.var_tmf1_dn2 = assign28060_e26949_d_n2;
        locals.var_tmf1_dn4 = assign28060_e26949_d_n4;
        locals.var_tmf1_dn5 = assign28060_e26949_d_n5;
        locals.var_tmf1_dn6 = assign28060_e26949_d_n6;
        locals.var_tmf1_dn7 = assign28060_e26949_d_n7;
        locals.var_tmf1_dn8 = assign28060_e26949_d_n8;
        locals.var_tmf1_dn9 = assign28060_e26949_d_n9;
        locals.var_tmf1_dn10 = assign28060_e26949_d_n10;
        locals.var_tmf1_dn13 = assign28060_e26949_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign28070_e26959, assign28070_e26959_d_n0, assign28070_e26959_d_n2, assign28070_e26959_d_n4, assign28070_e26959_d_n5, assign28070_e26959_d_n6, assign28070_e26959_d_n7, assign28070_e26959_d_n8, assign28070_e26959_d_n9, assign28070_e26959_d_n10, assign28070_e26959_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28070_e26957: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28070_e26957, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign28070_e26959;
        locals.var_x2_dn0 = assign28070_e26959_d_n0;
        locals.var_x2_dn2 = assign28070_e26959_d_n2;
        locals.var_x2_dn4 = assign28070_e26959_d_n4;
        locals.var_x2_dn5 = assign28070_e26959_d_n5;
        locals.var_x2_dn6 = assign28070_e26959_d_n6;
        locals.var_x2_dn7 = assign28070_e26959_d_n7;
        locals.var_x2_dn8 = assign28070_e26959_d_n8;
        locals.var_x2_dn9 = assign28070_e26959_d_n9;
        locals.var_x2_dn10 = assign28070_e26959_d_n10;
        locals.var_x2_dn13 = assign28070_e26959_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign28080_e26969, assign28080_e26969_d_n0, assign28080_e26969_d_n2, assign28080_e26969_d_n4, assign28080_e26969_d_n5, assign28080_e26969_d_n6, assign28080_e26969_d_n7, assign28080_e26969_d_n8, assign28080_e26969_d_n9, assign28080_e26969_d_n10, assign28080_e26969_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28080_e26967: f64 = (1e-18 * 1e-18);
        (assign28080_e26967, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign28080_e26969;
        locals.var_xmax2_dn0 = assign28080_e26969_d_n0;
        locals.var_xmax2_dn2 = assign28080_e26969_d_n2;
        locals.var_xmax2_dn4 = assign28080_e26969_d_n4;
        locals.var_xmax2_dn5 = assign28080_e26969_d_n5;
        locals.var_xmax2_dn6 = assign28080_e26969_d_n6;
        locals.var_xmax2_dn7 = assign28080_e26969_d_n7;
        locals.var_xmax2_dn8 = assign28080_e26969_d_n8;
        locals.var_xmax2_dn9 = assign28080_e26969_d_n9;
        locals.var_xmax2_dn10 = assign28080_e26969_d_n10;
        locals.var_xmax2_dn13 = assign28080_e26969_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign28090_e26977, assign28090_e26977_d_n0, assign28090_e26977_d_n2, assign28090_e26977_d_n4, assign28090_e26977_d_n5, assign28090_e26977_d_n6, assign28090_e26977_d_n7, assign28090_e26977_d_n8, assign28090_e26977_d_n9, assign28090_e26977_d_n10, assign28090_e26977_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28090_e26977;
        locals.var_xp_dn0 = assign28090_e26977_d_n0;
        locals.var_xp_dn2 = assign28090_e26977_d_n2;
        locals.var_xp_dn4 = assign28090_e26977_d_n4;
        locals.var_xp_dn5 = assign28090_e26977_d_n5;
        locals.var_xp_dn6 = assign28090_e26977_d_n6;
        locals.var_xp_dn7 = assign28090_e26977_d_n7;
        locals.var_xp_dn8 = assign28090_e26977_d_n8;
        locals.var_xp_dn9 = assign28090_e26977_d_n9;
        locals.var_xp_dn10 = assign28090_e26977_d_n10;
        locals.var_xp_dn13 = assign28090_e26977_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28100_e26985, assign28100_e26985_d_n0, assign28100_e26985_d_n2, assign28100_e26985_d_n4, assign28100_e26985_d_n5, assign28100_e26985_d_n6, assign28100_e26985_d_n7, assign28100_e26985_d_n8, assign28100_e26985_d_n9, assign28100_e26985_d_n10, assign28100_e26985_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28100_e26985;
        locals.var_xmp_dn0 = assign28100_e26985_d_n0;
        locals.var_xmp_dn2 = assign28100_e26985_d_n2;
        locals.var_xmp_dn4 = assign28100_e26985_d_n4;
        locals.var_xmp_dn5 = assign28100_e26985_d_n5;
        locals.var_xmp_dn6 = assign28100_e26985_d_n6;
        locals.var_xmp_dn7 = assign28100_e26985_d_n7;
        locals.var_xmp_dn8 = assign28100_e26985_d_n8;
        locals.var_xmp_dn9 = assign28100_e26985_d_n9;
        locals.var_xmp_dn10 = assign28100_e26985_d_n10;
        locals.var_xmp_dn13 = assign28100_e26985_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28110_e26993,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28110_e26993;
        locals.var_m0_rv = 0.0;

        let (assign28120_e27001,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28120_e27001;
        locals.var_mm_rv = 0.0;

        let (assign28130_e27009, assign28130_e27009_d_n0, assign28130_e27009_d_n2, assign28130_e27009_d_n4, assign28130_e27009_d_n5, assign28130_e27009_d_n6, assign28130_e27009_d_n7, assign28130_e27009_d_n8, assign28130_e27009_d_n9, assign28130_e27009_d_n10, assign28130_e27009_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign28130_e27009;
        locals.var_arg_dn0 = assign28130_e27009_d_n0;
        locals.var_arg_dn2 = assign28130_e27009_d_n2;
        locals.var_arg_dn4 = assign28130_e27009_d_n4;
        locals.var_arg_dn5 = assign28130_e27009_d_n5;
        locals.var_arg_dn6 = assign28130_e27009_d_n6;
        locals.var_arg_dn7 = assign28130_e27009_d_n7;
        locals.var_arg_dn8 = assign28130_e27009_d_n8;
        locals.var_arg_dn9 = assign28130_e27009_d_n9;
        locals.var_arg_dn10 = assign28130_e27009_d_n10;
        locals.var_arg_dn13 = assign28130_e27009_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign28140_e27017, assign28140_e27017_d_n0, assign28140_e27017_d_n2, assign28140_e27017_d_n4, assign28140_e27017_d_n5, assign28140_e27017_d_n6, assign28140_e27017_d_n7, assign28140_e27017_d_n8, assign28140_e27017_d_n9, assign28140_e27017_d_n10, assign28140_e27017_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28140_e27017;
        locals.var_dnm_dn0 = assign28140_e27017_d_n0;
        locals.var_dnm_dn2 = assign28140_e27017_d_n2;
        locals.var_dnm_dn4 = assign28140_e27017_d_n4;
        locals.var_dnm_dn5 = assign28140_e27017_d_n5;
        locals.var_dnm_dn6 = assign28140_e27017_d_n6;
        locals.var_dnm_dn7 = assign28140_e27017_d_n7;
        locals.var_dnm_dn8 = assign28140_e27017_d_n8;
        locals.var_dnm_dn9 = assign28140_e27017_d_n9;
        locals.var_dnm_dn10 = assign28140_e27017_d_n10;
        locals.var_dnm_dn13 = assign28140_e27017_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28150_e27027, assign28150_e27027_d_n0, assign28150_e27027_d_n2, assign28150_e27027_d_n4, assign28150_e27027_d_n5, assign28150_e27027_d_n6, assign28150_e27027_d_n7, assign28150_e27027_d_n8, assign28150_e27027_d_n9, assign28150_e27027_d_n10, assign28150_e27027_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28150_e27025: f64 = (locals.var_xp * locals.var_x2);
        (assign28150_e27025, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28150_e27027;
        locals.var_xp_dn0 = assign28150_e27027_d_n0;
        locals.var_xp_dn2 = assign28150_e27027_d_n2;
        locals.var_xp_dn4 = assign28150_e27027_d_n4;
        locals.var_xp_dn5 = assign28150_e27027_d_n5;
        locals.var_xp_dn6 = assign28150_e27027_d_n6;
        locals.var_xp_dn7 = assign28150_e27027_d_n7;
        locals.var_xp_dn8 = assign28150_e27027_d_n8;
        locals.var_xp_dn9 = assign28150_e27027_d_n9;
        locals.var_xp_dn10 = assign28150_e27027_d_n10;
        locals.var_xp_dn13 = assign28150_e27027_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28160_e27037, assign28160_e27037_d_n0, assign28160_e27037_d_n2, assign28160_e27037_d_n4, assign28160_e27037_d_n5, assign28160_e27037_d_n6, assign28160_e27037_d_n7, assign28160_e27037_d_n8, assign28160_e27037_d_n9, assign28160_e27037_d_n10, assign28160_e27037_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28160_e27035: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28160_e27035, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28160_e27037;
        locals.var_xmp_dn0 = assign28160_e27037_d_n0;
        locals.var_xmp_dn2 = assign28160_e27037_d_n2;
        locals.var_xmp_dn4 = assign28160_e27037_d_n4;
        locals.var_xmp_dn5 = assign28160_e27037_d_n5;
        locals.var_xmp_dn6 = assign28160_e27037_d_n6;
        locals.var_xmp_dn7 = assign28160_e27037_d_n7;
        locals.var_xmp_dn8 = assign28160_e27037_d_n8;
        locals.var_xmp_dn9 = assign28160_e27037_d_n9;
        locals.var_xmp_dn10 = assign28160_e27037_d_n10;
        locals.var_xmp_dn13 = assign28160_e27037_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28170_e27047, assign28170_e27047_d_n0, assign28170_e27047_d_n2, assign28170_e27047_d_n4, assign28170_e27047_d_n5, assign28170_e27047_d_n6, assign28170_e27047_d_n7, assign28170_e27047_d_n8, assign28170_e27047_d_n9, assign28170_e27047_d_n10, assign28170_e27047_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28170_e27045: f64 = (locals.var_xp * locals.var_x2);
        (assign28170_e27045, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28170_e27047;
        locals.var_xp_dn0 = assign28170_e27047_d_n0;
        locals.var_xp_dn2 = assign28170_e27047_d_n2;
        locals.var_xp_dn4 = assign28170_e27047_d_n4;
        locals.var_xp_dn5 = assign28170_e27047_d_n5;
        locals.var_xp_dn6 = assign28170_e27047_d_n6;
        locals.var_xp_dn7 = assign28170_e27047_d_n7;
        locals.var_xp_dn8 = assign28170_e27047_d_n8;
        locals.var_xp_dn9 = assign28170_e27047_d_n9;
        locals.var_xp_dn10 = assign28170_e27047_d_n10;
        locals.var_xp_dn13 = assign28170_e27047_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        locals: &mut StampLocals,
    ) {
        let (assign28180_e27057, assign28180_e27057_d_n0, assign28180_e27057_d_n2, assign28180_e27057_d_n4, assign28180_e27057_d_n5, assign28180_e27057_d_n6, assign28180_e27057_d_n7, assign28180_e27057_d_n8, assign28180_e27057_d_n9, assign28180_e27057_d_n10, assign28180_e27057_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28180_e27055: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28180_e27055, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28180_e27057;
        locals.var_xmp_dn0 = assign28180_e27057_d_n0;
        locals.var_xmp_dn2 = assign28180_e27057_d_n2;
        locals.var_xmp_dn4 = assign28180_e27057_d_n4;
        locals.var_xmp_dn5 = assign28180_e27057_d_n5;
        locals.var_xmp_dn6 = assign28180_e27057_d_n6;
        locals.var_xmp_dn7 = assign28180_e27057_d_n7;
        locals.var_xmp_dn8 = assign28180_e27057_d_n8;
        locals.var_xmp_dn9 = assign28180_e27057_d_n9;
        locals.var_xmp_dn10 = assign28180_e27057_d_n10;
        locals.var_xmp_dn13 = assign28180_e27057_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28190_e27067, assign28190_e27067_d_n0, assign28190_e27067_d_n2, assign28190_e27067_d_n4, assign28190_e27067_d_n5, assign28190_e27067_d_n6, assign28190_e27067_d_n7, assign28190_e27067_d_n8, assign28190_e27067_d_n9, assign28190_e27067_d_n10, assign28190_e27067_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28190_e27065: f64 = (locals.var_xp + locals.var_xmp);
        (assign28190_e27065, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign28190_e27067;
        locals.var_arg_dn0 = assign28190_e27067_d_n0;
        locals.var_arg_dn2 = assign28190_e27067_d_n2;
        locals.var_arg_dn4 = assign28190_e27067_d_n4;
        locals.var_arg_dn5 = assign28190_e27067_d_n5;
        locals.var_arg_dn6 = assign28190_e27067_d_n6;
        locals.var_arg_dn7 = assign28190_e27067_d_n7;
        locals.var_arg_dn8 = assign28190_e27067_d_n8;
        locals.var_arg_dn9 = assign28190_e27067_d_n9;
        locals.var_arg_dn10 = assign28190_e27067_d_n10;
        locals.var_arg_dn13 = assign28190_e27067_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign28200_e27075, assign28200_e27075_d_n0, assign28200_e27075_d_n2, assign28200_e27075_d_n4, assign28200_e27075_d_n5, assign28200_e27075_d_n6, assign28200_e27075_d_n7, assign28200_e27075_d_n8, assign28200_e27075_d_n9, assign28200_e27075_d_n10, assign28200_e27075_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28200_e27075;
        locals.var_dnm_dn0 = assign28200_e27075_d_n0;
        locals.var_dnm_dn2 = assign28200_e27075_d_n2;
        locals.var_dnm_dn4 = assign28200_e27075_d_n4;
        locals.var_dnm_dn5 = assign28200_e27075_d_n5;
        locals.var_dnm_dn6 = assign28200_e27075_d_n6;
        locals.var_dnm_dn7 = assign28200_e27075_d_n7;
        locals.var_dnm_dn8 = assign28200_e27075_d_n8;
        locals.var_dnm_dn9 = assign28200_e27075_d_n9;
        locals.var_dnm_dn10 = assign28200_e27075_d_n10;
        locals.var_dnm_dn13 = assign28200_e27075_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign28210_e27090: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard662 = assign28210_e27090;
        locals.var_guard662_rv = 0.0;

        let assign28220_e27093: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign28220_e27093;
        locals.var_guard663_rv = 0.0;

        let (assign28230_e27105,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28230_e27105;
        locals.var_mm_rv = 0.0;

        let assign28240_e27108: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign28240_e27108;
        locals.var_guard664_rv = 0.0;

        let (assign28250_e27123,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28250_e27123;
        locals.var_mm_rv = 0.0;

        let assign28260_e27126: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign28260_e27126;
        locals.var_guard665_rv = 0.0;

        let (assign28270_e27144,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 == 0.0)) && (locals.var_guard665 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28270_e27144;
        locals.var_mm_rv = 0.0;

        let assign28280_e27147: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign28280_e27147;
        locals.var_guard666_rv = 0.0;

        let (assign28290_e27168,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28290_e27168;
        locals.var_mm_rv = 0.0;

        let (assign28300_e27178,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28300_e27178;
        locals.var_m0_rv = 0.0;

        let mut assign28310_loop_guard: usize = 0;
        while {
            let assign28310_cond_e27189: f64 = if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28310_cond_e27189 != 0.0
        } {
            assign28310_loop_guard += 1;
            assert!(assign28310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28310_body0_e27200, assign28310_body0_e27200_d_n0, assign28310_body0_e27200_d_n2, assign28310_body0_e27200_d_n4, assign28310_body0_e27200_d_n5, assign28310_body0_e27200_d_n6, assign28310_body0_e27200_d_n7, assign28310_body0_e27200_d_n8, assign28310_body0_e27200_d_n9, assign28310_body0_e27200_d_n10, assign28310_body0_e27200_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign28310_body0_e27198: f64 = (locals.var_dnm).sqrt();
        (assign28310_body0_e27198, (locals.var_dnm_dn0 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn2 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn4 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn5 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn6 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn7 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn8 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn9 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn10 / (2.0 * assign28310_body0_e27198)), (locals.var_dnm_dn13 / (2.0 * assign28310_body0_e27198)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign28310_body0_e27200;
            locals.var_dnm_dn0 = assign28310_body0_e27200_d_n0;
            locals.var_dnm_dn2 = assign28310_body0_e27200_d_n2;
            locals.var_dnm_dn4 = assign28310_body0_e27200_d_n4;
            locals.var_dnm_dn5 = assign28310_body0_e27200_d_n5;
            locals.var_dnm_dn6 = assign28310_body0_e27200_d_n6;
            locals.var_dnm_dn7 = assign28310_body0_e27200_d_n7;
            locals.var_dnm_dn8 = assign28310_body0_e27200_d_n8;
            locals.var_dnm_dn9 = assign28310_body0_e27200_d_n9;
            locals.var_dnm_dn10 = assign28310_body0_e27200_d_n10;
            locals.var_dnm_dn13 = assign28310_body0_e27200_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign28310_body1_e27212,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign28310_body1_e27210: f64 = (locals.var_m0 + 1.0);
        (assign28310_body1_e27210,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28310_body1_e27212;
            locals.var_m0_rv = 0.0;
        }

        let (assign28320_e27234, assign28320_e27234_d_n0, assign28320_e27234_d_n2, assign28320_e27234_d_n4, assign28320_e27234_d_n5, assign28320_e27234_d_n6, assign28320_e27234_d_n7, assign28320_e27234_d_n8, assign28320_e27234_d_n9, assign28320_e27234_d_n10, assign28320_e27234_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 == 0.0)) {
        let (assign28320_e27232, assign28320_e27232_d_n0, assign28320_e27232_d_n2, assign28320_e27232_d_n4, assign28320_e27232_d_n5, assign28320_e27232_d_n6, assign28320_e27232_d_n7, assign28320_e27232_d_n8, assign28320_e27232_d_n9, assign28320_e27232_d_n10, assign28320_e27232_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28320_e27229: f64 = (2.0 * 2.0);
                let assign28320_e27230: f64 = (1.0 / assign28320_e27229);
                let assign28320_e27231: f64 = (locals.var_dnm).powf(assign28320_e27230);
                (assign28320_e27231, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28320_e27230) as f64).is_finite() && ((assign28320_e27230) as f64).fract() == 0.0 { if assign28320_e27230 == 0.0 { 0.0 } else { (assign28320_e27230 * ((locals.var_dnm).powf(assign28320_e27230 - 1.0) * locals.var_dnm_dn13)) } } else { (assign28320_e27231 * (assign28320_e27230 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign28320_e27232, assign28320_e27232_d_n0, assign28320_e27232_d_n2, assign28320_e27232_d_n4, assign28320_e27232_d_n5, assign28320_e27232_d_n6, assign28320_e27232_d_n7, assign28320_e27232_d_n8, assign28320_e27232_d_n9, assign28320_e27232_d_n10, assign28320_e27232_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28320_e27234;
        locals.var_dnm_dn0 = assign28320_e27234_d_n0;
        locals.var_dnm_dn2 = assign28320_e27234_d_n2;
        locals.var_dnm_dn4 = assign28320_e27234_d_n4;
        locals.var_dnm_dn5 = assign28320_e27234_d_n5;
        locals.var_dnm_dn6 = assign28320_e27234_d_n6;
        locals.var_dnm_dn7 = assign28320_e27234_d_n7;
        locals.var_dnm_dn8 = assign28320_e27234_d_n8;
        locals.var_dnm_dn9 = assign28320_e27234_d_n9;
        locals.var_dnm_dn10 = assign28320_e27234_d_n10;
        locals.var_dnm_dn13 = assign28320_e27234_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28330_e27244, assign28330_e27244_d_n0, assign28330_e27244_d_n2, assign28330_e27244_d_n4, assign28330_e27244_d_n5, assign28330_e27244_d_n6, assign28330_e27244_d_n7, assign28330_e27244_d_n8, assign28330_e27244_d_n9, assign28330_e27244_d_n10, assign28330_e27244_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28330_e27242: f64 = (1.0 / locals.var_dnm);
        (assign28330_e27242, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28330_e27244;
        locals.var_dnm_dn0 = assign28330_e27244_d_n0;
        locals.var_dnm_dn2 = assign28330_e27244_d_n2;
        locals.var_dnm_dn4 = assign28330_e27244_d_n4;
        locals.var_dnm_dn5 = assign28330_e27244_d_n5;
        locals.var_dnm_dn6 = assign28330_e27244_d_n6;
        locals.var_dnm_dn7 = assign28330_e27244_d_n7;
        locals.var_dnm_dn8 = assign28330_e27244_d_n8;
        locals.var_dnm_dn9 = assign28330_e27244_d_n9;
        locals.var_dnm_dn10 = assign28330_e27244_d_n10;
        locals.var_dnm_dn13 = assign28330_e27244_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28340_e27256, assign28340_e27256_d_n0, assign28340_e27256_d_n2, assign28340_e27256_d_n4, assign28340_e27256_d_n5, assign28340_e27256_d_n6, assign28340_e27256_d_n7, assign28340_e27256_d_n8, assign28340_e27256_d_n9, assign28340_e27256_d_n10, assign28340_e27256_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28340_e27252: f64 = (locals.var_tmf1 * 1e-18);
        let assign28340_e27254: f64 = (assign28340_e27252 * locals.var_dnm);
        (assign28340_e27254, (((locals.var_tmf1_dn0 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-18) * locals.var_dnm) + (assign28340_e27252 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign28340_e27256;
        locals.var_tmf0_dn0 = assign28340_e27256_d_n0;
        locals.var_tmf0_dn2 = assign28340_e27256_d_n2;
        locals.var_tmf0_dn4 = assign28340_e27256_d_n4;
        locals.var_tmf0_dn5 = assign28340_e27256_d_n5;
        locals.var_tmf0_dn6 = assign28340_e27256_d_n6;
        locals.var_tmf0_dn7 = assign28340_e27256_d_n7;
        locals.var_tmf0_dn8 = assign28340_e27256_d_n8;
        locals.var_tmf0_dn9 = assign28340_e27256_d_n9;
        locals.var_tmf0_dn10 = assign28340_e27256_d_n10;
        locals.var_tmf0_dn13 = assign28340_e27256_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign28350_e27270, assign28350_e27270_d_n0, assign28350_e27270_d_n2, assign28350_e27270_d_n4, assign28350_e27270_d_n5, assign28350_e27270_d_n6, assign28350_e27270_d_n7, assign28350_e27270_d_n8, assign28350_e27270_d_n9, assign28350_e27270_d_n10, assign28350_e27270_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28350_e27264: f64 = (1e-18 * locals.var_xmp);
        let assign28350_e27266: f64 = (assign28350_e27264 * locals.var_dnm);
        let assign28350_e27268: f64 = (assign28350_e27266 / locals.var_arg);
        (assign28350_e27268, ((((((1e-18 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn13) * locals.var_dnm) + (assign28350_e27264 * locals.var_dnm_dn13)) * locals.var_arg) - (assign28350_e27266 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign28350_e27270;
        locals.var_t0_dn0 = assign28350_e27270_d_n0;
        locals.var_t0_dn2 = assign28350_e27270_d_n2;
        locals.var_t0_dn4 = assign28350_e27270_d_n4;
        locals.var_t0_dn5 = assign28350_e27270_d_n5;
        locals.var_t0_dn6 = assign28350_e27270_d_n6;
        locals.var_t0_dn7 = assign28350_e27270_d_n7;
        locals.var_t0_dn8 = assign28350_e27270_d_n8;
        locals.var_t0_dn9 = assign28350_e27270_d_n9;
        locals.var_t0_dn10 = assign28350_e27270_d_n10;
        locals.var_t0_dn13 = assign28350_e27270_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign28360_e27282, assign28360_e27282_d_n0, assign28360_e27282_d_n2, assign28360_e27282_d_n4, assign28360_e27282_d_n5, assign28360_e27282_d_n6, assign28360_e27282_d_n7, assign28360_e27282_d_n8, assign28360_e27282_d_n9, assign28360_e27282_d_n10, assign28360_e27282_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign28360_e27278: f64 = (1e-25 + 1e-18);
        let assign28360_e27280: f64 = (assign28360_e27278 - locals.var_tmf0);
        (assign28360_e27280, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn13,)
    }
};
        locals.var_w_res0 = assign28360_e27282;
        locals.var_w_res0_dn0 = assign28360_e27282_d_n0;
        locals.var_w_res0_dn2 = assign28360_e27282_d_n2;
        locals.var_w_res0_dn4 = assign28360_e27282_d_n4;
        locals.var_w_res0_dn5 = assign28360_e27282_d_n5;
        locals.var_w_res0_dn6 = assign28360_e27282_d_n6;
        locals.var_w_res0_dn7 = assign28360_e27282_d_n7;
        locals.var_w_res0_dn8 = assign28360_e27282_d_n8;
        locals.var_w_res0_dn9 = assign28360_e27282_d_n9;
        locals.var_w_res0_dn10 = assign28360_e27282_d_n10;
        locals.var_w_res0_dn13 = assign28360_e27282_d_n13;
        locals.var_w_res0_rv = 0.0;

        let (assign28370_e27290, assign28370_e27290_d_n0, assign28370_e27290_d_n2, assign28370_e27290_d_n4, assign28370_e27290_d_n5, assign28370_e27290_d_n6, assign28370_e27290_d_n7, assign28370_e27290_d_n8, assign28370_e27290_d_n9, assign28370_e27290_d_n10, assign28370_e27290_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign28370_e27290;
        locals.var_t0_dn0 = assign28370_e27290_d_n0;
        locals.var_t0_dn2 = assign28370_e27290_d_n2;
        locals.var_t0_dn4 = assign28370_e27290_d_n4;
        locals.var_t0_dn5 = assign28370_e27290_d_n5;
        locals.var_t0_dn6 = assign28370_e27290_d_n6;
        locals.var_t0_dn7 = assign28370_e27290_d_n7;
        locals.var_t0_dn8 = assign28370_e27290_d_n8;
        locals.var_t0_dn9 = assign28370_e27290_d_n9;
        locals.var_t0_dn10 = assign28370_e27290_d_n10;
        locals.var_t0_dn13 = assign28370_e27290_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign28380_e27299, assign28380_e27299_d_n0, assign28380_e27299_d_n2, assign28380_e27299_d_n4, assign28380_e27299_d_n5, assign28380_e27299_d_n6, assign28380_e27299_d_n7, assign28380_e27299_d_n8, assign28380_e27299_d_n9, assign28380_e27299_d_n10, assign28380_e27299_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn13,)
    }
};
        locals.var_w_res0 = assign28380_e27299;
        locals.var_w_res0_dn0 = assign28380_e27299_d_n0;
        locals.var_w_res0_dn2 = assign28380_e27299_d_n2;
        locals.var_w_res0_dn4 = assign28380_e27299_d_n4;
        locals.var_w_res0_dn5 = assign28380_e27299_d_n5;
        locals.var_w_res0_dn6 = assign28380_e27299_d_n6;
        locals.var_w_res0_dn7 = assign28380_e27299_d_n7;
        locals.var_w_res0_dn8 = assign28380_e27299_d_n8;
        locals.var_w_res0_dn9 = assign28380_e27299_d_n9;
        locals.var_w_res0_dn10 = assign28380_e27299_d_n10;
        locals.var_w_res0_dn13 = assign28380_e27299_d_n13;
        locals.var_w_res0_rv = 0.0;

        let (assign28390_e27308, assign28390_e27308_d_n0, assign28390_e27308_d_n2, assign28390_e27308_d_n4, assign28390_e27308_d_n5, assign28390_e27308_d_n6, assign28390_e27308_d_n7, assign28390_e27308_d_n8, assign28390_e27308_d_n9, assign28390_e27308_d_n10, assign28390_e27308_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard661 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign28390_e27308;
        locals.var_t0_dn0 = assign28390_e27308_d_n0;
        locals.var_t0_dn2 = assign28390_e27308_d_n2;
        locals.var_t0_dn4 = assign28390_e27308_d_n4;
        locals.var_t0_dn5 = assign28390_e27308_d_n5;
        locals.var_t0_dn6 = assign28390_e27308_d_n6;
        locals.var_t0_dn7 = assign28390_e27308_d_n7;
        locals.var_t0_dn8 = assign28390_e27308_d_n8;
        locals.var_t0_dn9 = assign28390_e27308_d_n9;
        locals.var_t0_dn10 = assign28390_e27308_d_n10;
        locals.var_t0_dn13 = assign28390_e27308_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign28400_e27317, assign28400_e27317_d_n0, assign28400_e27317_d_n2, assign28400_e27317_d_n4, assign28400_e27317_d_n5, assign28400_e27317_d_n6, assign28400_e27317_d_n7, assign28400_e27317_d_n8, assign28400_e27317_d_n9, assign28400_e27317_d_n10, assign28400_e27317_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign28400_e27313: f64 = (-locals.var_w_res0);
        let assign28400_e27315: f64 = (assign28400_e27313 * locals.var_q_ndepm);
        (assign28400_e27315, (((-locals.var_w_res0_dn0) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn0)), (((-locals.var_w_res0_dn2) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn2)), (((-locals.var_w_res0_dn4) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn4)), (((-locals.var_w_res0_dn5) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn5)), (((-locals.var_w_res0_dn6) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn6)), (((-locals.var_w_res0_dn7) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn7)), (((-locals.var_w_res0_dn8) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn8)), (((-locals.var_w_res0_dn9) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn9)), (((-locals.var_w_res0_dn10) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn10)), (((-locals.var_w_res0_dn13) * locals.var_q_ndepm) + (assign28400_e27313 * locals.var_q_ndepm_dn13)),)
    } else {
        (locals.var_qn_res0, locals.var_qn_res0_dn0, locals.var_qn_res0_dn2, locals.var_qn_res0_dn4, locals.var_qn_res0_dn5, locals.var_qn_res0_dn6, locals.var_qn_res0_dn7, locals.var_qn_res0_dn8, locals.var_qn_res0_dn9, locals.var_qn_res0_dn10, locals.var_qn_res0_dn13,)
    }
};
        locals.var_qn_res0 = assign28400_e27317;
        locals.var_qn_res0_dn0 = assign28400_e27317_d_n0;
        locals.var_qn_res0_dn2 = assign28400_e27317_d_n2;
        locals.var_qn_res0_dn4 = assign28400_e27317_d_n4;
        locals.var_qn_res0_dn5 = assign28400_e27317_d_n5;
        locals.var_qn_res0_dn6 = assign28400_e27317_d_n6;
        locals.var_qn_res0_dn7 = assign28400_e27317_d_n7;
        locals.var_qn_res0_dn8 = assign28400_e27317_d_n8;
        locals.var_qn_res0_dn9 = assign28400_e27317_d_n9;
        locals.var_qn_res0_dn10 = assign28400_e27317_d_n10;
        locals.var_qn_res0_dn13 = assign28400_e27317_d_n13;
        locals.var_qn_res0_rv = 0.0;

        let assign28410_e27324: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard667 = assign28410_e27324;
        locals.var_guard667_rv = 0.0;

        let assign28420_e27328: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28420_e27333: f64 = if ((locals.var_phi_s0_dep > assign28420_e27328) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard668 = assign28420_e27333;
        locals.var_guard668_rv = 0.0;

        let (assign28430_e27347, assign28430_e27347_d_n0, assign28430_e27347_d_n2, assign28430_e27347_d_n4, assign28430_e27347_d_n5, assign28430_e27347_d_n6, assign28430_e27347_d_n7, assign28430_e27347_d_n8, assign28430_e27347_d_n9, assign28430_e27347_d_n10, assign28430_e27347_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28430_e27343: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign28430_e27345: f64 = (assign28430_e27343 + 0.8);
        (assign28430_e27345, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn13 - locals.var_vds_maxb0_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign28430_e27347;
        locals.var_tmf1_dn0 = assign28430_e27347_d_n0;
        locals.var_tmf1_dn2 = assign28430_e27347_d_n2;
        locals.var_tmf1_dn4 = assign28430_e27347_d_n4;
        locals.var_tmf1_dn5 = assign28430_e27347_d_n5;
        locals.var_tmf1_dn6 = assign28430_e27347_d_n6;
        locals.var_tmf1_dn7 = assign28430_e27347_d_n7;
        locals.var_tmf1_dn8 = assign28430_e27347_d_n8;
        locals.var_tmf1_dn9 = assign28430_e27347_d_n9;
        locals.var_tmf1_dn10 = assign28430_e27347_d_n10;
        locals.var_tmf1_dn13 = assign28430_e27347_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign28440_e27359, assign28440_e27359_d_n0, assign28440_e27359_d_n2, assign28440_e27359_d_n4, assign28440_e27359_d_n5, assign28440_e27359_d_n6, assign28440_e27359_d_n7, assign28440_e27359_d_n8, assign28440_e27359_d_n9, assign28440_e27359_d_n10, assign28440_e27359_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28440_e27357: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28440_e27357, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign28440_e27359;
        locals.var_x2_dn0 = assign28440_e27359_d_n0;
        locals.var_x2_dn2 = assign28440_e27359_d_n2;
        locals.var_x2_dn4 = assign28440_e27359_d_n4;
        locals.var_x2_dn5 = assign28440_e27359_d_n5;
        locals.var_x2_dn6 = assign28440_e27359_d_n6;
        locals.var_x2_dn7 = assign28440_e27359_d_n7;
        locals.var_x2_dn8 = assign28440_e27359_d_n8;
        locals.var_x2_dn9 = assign28440_e27359_d_n9;
        locals.var_x2_dn10 = assign28440_e27359_d_n10;
        locals.var_x2_dn13 = assign28440_e27359_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign28450_e27371, assign28450_e27371_d_n0, assign28450_e27371_d_n2, assign28450_e27371_d_n4, assign28450_e27371_d_n5, assign28450_e27371_d_n6, assign28450_e27371_d_n7, assign28450_e27371_d_n8, assign28450_e27371_d_n9, assign28450_e27371_d_n10, assign28450_e27371_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28450_e27369: f64 = (0.8 * 0.8);
        (assign28450_e27369, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign28450_e27371;
        locals.var_xmax2_dn0 = assign28450_e27371_d_n0;
        locals.var_xmax2_dn2 = assign28450_e27371_d_n2;
        locals.var_xmax2_dn4 = assign28450_e27371_d_n4;
        locals.var_xmax2_dn5 = assign28450_e27371_d_n5;
        locals.var_xmax2_dn6 = assign28450_e27371_d_n6;
        locals.var_xmax2_dn7 = assign28450_e27371_d_n7;
        locals.var_xmax2_dn8 = assign28450_e27371_d_n8;
        locals.var_xmax2_dn9 = assign28450_e27371_d_n9;
        locals.var_xmax2_dn10 = assign28450_e27371_d_n10;
        locals.var_xmax2_dn13 = assign28450_e27371_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign28460_e27381, assign28460_e27381_d_n0, assign28460_e27381_d_n2, assign28460_e27381_d_n4, assign28460_e27381_d_n5, assign28460_e27381_d_n6, assign28460_e27381_d_n7, assign28460_e27381_d_n8, assign28460_e27381_d_n9, assign28460_e27381_d_n10, assign28460_e27381_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28460_e27381;
        locals.var_xp_dn0 = assign28460_e27381_d_n0;
        locals.var_xp_dn2 = assign28460_e27381_d_n2;
        locals.var_xp_dn4 = assign28460_e27381_d_n4;
        locals.var_xp_dn5 = assign28460_e27381_d_n5;
        locals.var_xp_dn6 = assign28460_e27381_d_n6;
        locals.var_xp_dn7 = assign28460_e27381_d_n7;
        locals.var_xp_dn8 = assign28460_e27381_d_n8;
        locals.var_xp_dn9 = assign28460_e27381_d_n9;
        locals.var_xp_dn10 = assign28460_e27381_d_n10;
        locals.var_xp_dn13 = assign28460_e27381_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28470_e27391, assign28470_e27391_d_n0, assign28470_e27391_d_n2, assign28470_e27391_d_n4, assign28470_e27391_d_n5, assign28470_e27391_d_n6, assign28470_e27391_d_n7, assign28470_e27391_d_n8, assign28470_e27391_d_n9, assign28470_e27391_d_n10, assign28470_e27391_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28470_e27391;
        locals.var_xmp_dn0 = assign28470_e27391_d_n0;
        locals.var_xmp_dn2 = assign28470_e27391_d_n2;
        locals.var_xmp_dn4 = assign28470_e27391_d_n4;
        locals.var_xmp_dn5 = assign28470_e27391_d_n5;
        locals.var_xmp_dn6 = assign28470_e27391_d_n6;
        locals.var_xmp_dn7 = assign28470_e27391_d_n7;
        locals.var_xmp_dn8 = assign28470_e27391_d_n8;
        locals.var_xmp_dn9 = assign28470_e27391_d_n9;
        locals.var_xmp_dn10 = assign28470_e27391_d_n10;
        locals.var_xmp_dn13 = assign28470_e27391_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28480_e27401,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28480_e27401;
        locals.var_m0_rv = 0.0;

        let (assign28490_e27411,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28490_e27411;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign28500_e27421, assign28500_e27421_d_n0, assign28500_e27421_d_n2, assign28500_e27421_d_n4, assign28500_e27421_d_n5, assign28500_e27421_d_n6, assign28500_e27421_d_n7, assign28500_e27421_d_n8, assign28500_e27421_d_n9, assign28500_e27421_d_n10, assign28500_e27421_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign28500_e27421;
        locals.var_arg_dn0 = assign28500_e27421_d_n0;
        locals.var_arg_dn2 = assign28500_e27421_d_n2;
        locals.var_arg_dn4 = assign28500_e27421_d_n4;
        locals.var_arg_dn5 = assign28500_e27421_d_n5;
        locals.var_arg_dn6 = assign28500_e27421_d_n6;
        locals.var_arg_dn7 = assign28500_e27421_d_n7;
        locals.var_arg_dn8 = assign28500_e27421_d_n8;
        locals.var_arg_dn9 = assign28500_e27421_d_n9;
        locals.var_arg_dn10 = assign28500_e27421_d_n10;
        locals.var_arg_dn13 = assign28500_e27421_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign28510_e27431, assign28510_e27431_d_n0, assign28510_e27431_d_n2, assign28510_e27431_d_n4, assign28510_e27431_d_n5, assign28510_e27431_d_n6, assign28510_e27431_d_n7, assign28510_e27431_d_n8, assign28510_e27431_d_n9, assign28510_e27431_d_n10, assign28510_e27431_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28510_e27431;
        locals.var_dnm_dn0 = assign28510_e27431_d_n0;
        locals.var_dnm_dn2 = assign28510_e27431_d_n2;
        locals.var_dnm_dn4 = assign28510_e27431_d_n4;
        locals.var_dnm_dn5 = assign28510_e27431_d_n5;
        locals.var_dnm_dn6 = assign28510_e27431_d_n6;
        locals.var_dnm_dn7 = assign28510_e27431_d_n7;
        locals.var_dnm_dn8 = assign28510_e27431_d_n8;
        locals.var_dnm_dn9 = assign28510_e27431_d_n9;
        locals.var_dnm_dn10 = assign28510_e27431_d_n10;
        locals.var_dnm_dn13 = assign28510_e27431_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28520_e27443, assign28520_e27443_d_n0, assign28520_e27443_d_n2, assign28520_e27443_d_n4, assign28520_e27443_d_n5, assign28520_e27443_d_n6, assign28520_e27443_d_n7, assign28520_e27443_d_n8, assign28520_e27443_d_n9, assign28520_e27443_d_n10, assign28520_e27443_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28520_e27441: f64 = (locals.var_xp * locals.var_x2);
        (assign28520_e27441, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28520_e27443;
        locals.var_xp_dn0 = assign28520_e27443_d_n0;
        locals.var_xp_dn2 = assign28520_e27443_d_n2;
        locals.var_xp_dn4 = assign28520_e27443_d_n4;
        locals.var_xp_dn5 = assign28520_e27443_d_n5;
        locals.var_xp_dn6 = assign28520_e27443_d_n6;
        locals.var_xp_dn7 = assign28520_e27443_d_n7;
        locals.var_xp_dn8 = assign28520_e27443_d_n8;
        locals.var_xp_dn9 = assign28520_e27443_d_n9;
        locals.var_xp_dn10 = assign28520_e27443_d_n10;
        locals.var_xp_dn13 = assign28520_e27443_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28530_e27455, assign28530_e27455_d_n0, assign28530_e27455_d_n2, assign28530_e27455_d_n4, assign28530_e27455_d_n5, assign28530_e27455_d_n6, assign28530_e27455_d_n7, assign28530_e27455_d_n8, assign28530_e27455_d_n9, assign28530_e27455_d_n10, assign28530_e27455_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28530_e27453: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28530_e27453, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28530_e27455;
        locals.var_xmp_dn0 = assign28530_e27455_d_n0;
        locals.var_xmp_dn2 = assign28530_e27455_d_n2;
        locals.var_xmp_dn4 = assign28530_e27455_d_n4;
        locals.var_xmp_dn5 = assign28530_e27455_d_n5;
        locals.var_xmp_dn6 = assign28530_e27455_d_n6;
        locals.var_xmp_dn7 = assign28530_e27455_d_n7;
        locals.var_xmp_dn8 = assign28530_e27455_d_n8;
        locals.var_xmp_dn9 = assign28530_e27455_d_n9;
        locals.var_xmp_dn10 = assign28530_e27455_d_n10;
        locals.var_xmp_dn13 = assign28530_e27455_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28540_e27467, assign28540_e27467_d_n0, assign28540_e27467_d_n2, assign28540_e27467_d_n4, assign28540_e27467_d_n5, assign28540_e27467_d_n6, assign28540_e27467_d_n7, assign28540_e27467_d_n8, assign28540_e27467_d_n9, assign28540_e27467_d_n10, assign28540_e27467_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28540_e27465: f64 = (locals.var_xp * locals.var_x2);
        (assign28540_e27465, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28540_e27467;
        locals.var_xp_dn0 = assign28540_e27467_d_n0;
        locals.var_xp_dn2 = assign28540_e27467_d_n2;
        locals.var_xp_dn4 = assign28540_e27467_d_n4;
        locals.var_xp_dn5 = assign28540_e27467_d_n5;
        locals.var_xp_dn6 = assign28540_e27467_d_n6;
        locals.var_xp_dn7 = assign28540_e27467_d_n7;
        locals.var_xp_dn8 = assign28540_e27467_d_n8;
        locals.var_xp_dn9 = assign28540_e27467_d_n9;
        locals.var_xp_dn10 = assign28540_e27467_d_n10;
        locals.var_xp_dn13 = assign28540_e27467_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28550_e27479, assign28550_e27479_d_n0, assign28550_e27479_d_n2, assign28550_e27479_d_n4, assign28550_e27479_d_n5, assign28550_e27479_d_n6, assign28550_e27479_d_n7, assign28550_e27479_d_n8, assign28550_e27479_d_n9, assign28550_e27479_d_n10, assign28550_e27479_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28550_e27477: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28550_e27477, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28550_e27479;
        locals.var_xmp_dn0 = assign28550_e27479_d_n0;
        locals.var_xmp_dn2 = assign28550_e27479_d_n2;
        locals.var_xmp_dn4 = assign28550_e27479_d_n4;
        locals.var_xmp_dn5 = assign28550_e27479_d_n5;
        locals.var_xmp_dn6 = assign28550_e27479_d_n6;
        locals.var_xmp_dn7 = assign28550_e27479_d_n7;
        locals.var_xmp_dn8 = assign28550_e27479_d_n8;
        locals.var_xmp_dn9 = assign28550_e27479_d_n9;
        locals.var_xmp_dn10 = assign28550_e27479_d_n10;
        locals.var_xmp_dn13 = assign28550_e27479_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28560_e27491, assign28560_e27491_d_n0, assign28560_e27491_d_n2, assign28560_e27491_d_n4, assign28560_e27491_d_n5, assign28560_e27491_d_n6, assign28560_e27491_d_n7, assign28560_e27491_d_n8, assign28560_e27491_d_n9, assign28560_e27491_d_n10, assign28560_e27491_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28560_e27489: f64 = (locals.var_xp + locals.var_xmp);
        (assign28560_e27489, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign28560_e27491;
        locals.var_arg_dn0 = assign28560_e27491_d_n0;
        locals.var_arg_dn2 = assign28560_e27491_d_n2;
        locals.var_arg_dn4 = assign28560_e27491_d_n4;
        locals.var_arg_dn5 = assign28560_e27491_d_n5;
        locals.var_arg_dn6 = assign28560_e27491_d_n6;
        locals.var_arg_dn7 = assign28560_e27491_d_n7;
        locals.var_arg_dn8 = assign28560_e27491_d_n8;
        locals.var_arg_dn9 = assign28560_e27491_d_n9;
        locals.var_arg_dn10 = assign28560_e27491_d_n10;
        locals.var_arg_dn13 = assign28560_e27491_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign28570_e27501, assign28570_e27501_d_n0, assign28570_e27501_d_n2, assign28570_e27501_d_n4, assign28570_e27501_d_n5, assign28570_e27501_d_n6, assign28570_e27501_d_n7, assign28570_e27501_d_n8, assign28570_e27501_d_n9, assign28570_e27501_d_n10, assign28570_e27501_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28570_e27501;
        locals.var_dnm_dn0 = assign28570_e27501_d_n0;
        locals.var_dnm_dn2 = assign28570_e27501_d_n2;
        locals.var_dnm_dn4 = assign28570_e27501_d_n4;
        locals.var_dnm_dn5 = assign28570_e27501_d_n5;
        locals.var_dnm_dn6 = assign28570_e27501_d_n6;
        locals.var_dnm_dn7 = assign28570_e27501_d_n7;
        locals.var_dnm_dn8 = assign28570_e27501_d_n8;
        locals.var_dnm_dn9 = assign28570_e27501_d_n9;
        locals.var_dnm_dn10 = assign28570_e27501_d_n10;
        locals.var_dnm_dn13 = assign28570_e27501_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign28580_e27516: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard669 = assign28580_e27516;
        locals.var_guard669_rv = 0.0;

        let assign28590_e27519: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign28590_e27519;
        locals.var_guard670_rv = 0.0;

        let (assign28600_e27533,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28600_e27533;
        locals.var_mm_rv = 0.0;

        let assign28610_e27536: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard671 = assign28610_e27536;
        locals.var_guard671_rv = 0.0;

        let (assign28620_e27553,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 == 0.0)) && (locals.var_guard671 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28620_e27553;
        locals.var_mm_rv = 0.0;

        let assign28630_e27556: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign28630_e27556;
        locals.var_guard672_rv = 0.0;

        let (assign28640_e27576,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 == 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard672 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28640_e27576;
        locals.var_mm_rv = 0.0;

        let assign28650_e27579: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard673 = assign28650_e27579;
        locals.var_guard673_rv = 0.0;

        let (assign28660_e27602,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 == 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28660_e27602;
        locals.var_mm_rv = 0.0;

        let (assign28670_e27614,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28670_e27614;
        locals.var_m0_rv = 0.0;

        let mut assign28680_loop_guard: usize = 0;
        while {
            let assign28680_cond_e27627: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28680_cond_e27627 != 0.0
        } {
            assign28680_loop_guard += 1;
            assert!(assign28680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28680_body0_e27640, assign28680_body0_e27640_d_n0, assign28680_body0_e27640_d_n2, assign28680_body0_e27640_d_n4, assign28680_body0_e27640_d_n5, assign28680_body0_e27640_d_n6, assign28680_body0_e27640_d_n7, assign28680_body0_e27640_d_n8, assign28680_body0_e27640_d_n9, assign28680_body0_e27640_d_n10, assign28680_body0_e27640_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) {
        let assign28680_body0_e27638: f64 = (locals.var_dnm).sqrt();
        (assign28680_body0_e27638, (locals.var_dnm_dn0 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn2 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn4 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn5 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn6 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn7 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn8 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn9 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn10 / (2.0 * assign28680_body0_e27638)), (locals.var_dnm_dn13 / (2.0 * assign28680_body0_e27638)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign28680_body0_e27640;
            locals.var_dnm_dn0 = assign28680_body0_e27640_d_n0;
            locals.var_dnm_dn2 = assign28680_body0_e27640_d_n2;
            locals.var_dnm_dn4 = assign28680_body0_e27640_d_n4;
            locals.var_dnm_dn5 = assign28680_body0_e27640_d_n5;
            locals.var_dnm_dn6 = assign28680_body0_e27640_d_n6;
            locals.var_dnm_dn7 = assign28680_body0_e27640_d_n7;
            locals.var_dnm_dn8 = assign28680_body0_e27640_d_n8;
            locals.var_dnm_dn9 = assign28680_body0_e27640_d_n9;
            locals.var_dnm_dn10 = assign28680_body0_e27640_d_n10;
            locals.var_dnm_dn13 = assign28680_body0_e27640_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign28680_body1_e27654,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) {
        let assign28680_body1_e27652: f64 = (locals.var_m0 + 1.0);
        (assign28680_body1_e27652,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28680_body1_e27654;
            locals.var_m0_rv = 0.0;
        }

        let (assign28690_e27678, assign28690_e27678_d_n0, assign28690_e27678_d_n2, assign28690_e27678_d_n4, assign28690_e27678_d_n5, assign28690_e27678_d_n6, assign28690_e27678_d_n7, assign28690_e27678_d_n8, assign28690_e27678_d_n9, assign28690_e27678_d_n10, assign28690_e27678_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 == 0.0)) {
        let (assign28690_e27676, assign28690_e27676_d_n0, assign28690_e27676_d_n2, assign28690_e27676_d_n4, assign28690_e27676_d_n5, assign28690_e27676_d_n6, assign28690_e27676_d_n7, assign28690_e27676_d_n8, assign28690_e27676_d_n9, assign28690_e27676_d_n10, assign28690_e27676_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28690_e27673: f64 = (2.0 * 2.0);
                let assign28690_e27674: f64 = (1.0 / assign28690_e27673);
                let assign28690_e27675: f64 = (locals.var_dnm).powf(assign28690_e27674);
                (assign28690_e27675, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28690_e27674) as f64).is_finite() && ((assign28690_e27674) as f64).fract() == 0.0 { if assign28690_e27674 == 0.0 { 0.0 } else { (assign28690_e27674 * ((locals.var_dnm).powf(assign28690_e27674 - 1.0) * locals.var_dnm_dn13)) } } else { (assign28690_e27675 * (assign28690_e27674 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign28690_e27676, assign28690_e27676_d_n0, assign28690_e27676_d_n2, assign28690_e27676_d_n4, assign28690_e27676_d_n5, assign28690_e27676_d_n6, assign28690_e27676_d_n7, assign28690_e27676_d_n8, assign28690_e27676_d_n9, assign28690_e27676_d_n10, assign28690_e27676_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28690_e27678;
        locals.var_dnm_dn0 = assign28690_e27678_d_n0;
        locals.var_dnm_dn2 = assign28690_e27678_d_n2;
        locals.var_dnm_dn4 = assign28690_e27678_d_n4;
        locals.var_dnm_dn5 = assign28690_e27678_d_n5;
        locals.var_dnm_dn6 = assign28690_e27678_d_n6;
        locals.var_dnm_dn7 = assign28690_e27678_d_n7;
        locals.var_dnm_dn8 = assign28690_e27678_d_n8;
        locals.var_dnm_dn9 = assign28690_e27678_d_n9;
        locals.var_dnm_dn10 = assign28690_e27678_d_n10;
        locals.var_dnm_dn13 = assign28690_e27678_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28700_e27690, assign28700_e27690_d_n0, assign28700_e27690_d_n2, assign28700_e27690_d_n4, assign28700_e27690_d_n5, assign28700_e27690_d_n6, assign28700_e27690_d_n7, assign28700_e27690_d_n8, assign28700_e27690_d_n9, assign28700_e27690_d_n10, assign28700_e27690_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28700_e27688: f64 = (1.0 / locals.var_dnm);
        (assign28700_e27688, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28700_e27690;
        locals.var_dnm_dn0 = assign28700_e27690_d_n0;
        locals.var_dnm_dn2 = assign28700_e27690_d_n2;
        locals.var_dnm_dn4 = assign28700_e27690_d_n4;
        locals.var_dnm_dn5 = assign28700_e27690_d_n5;
        locals.var_dnm_dn6 = assign28700_e27690_d_n6;
        locals.var_dnm_dn7 = assign28700_e27690_d_n7;
        locals.var_dnm_dn8 = assign28700_e27690_d_n8;
        locals.var_dnm_dn9 = assign28700_e27690_d_n9;
        locals.var_dnm_dn10 = assign28700_e27690_d_n10;
        locals.var_dnm_dn13 = assign28700_e27690_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28710_e27704, assign28710_e27704_d_n0, assign28710_e27704_d_n2, assign28710_e27704_d_n4, assign28710_e27704_d_n5, assign28710_e27704_d_n6, assign28710_e27704_d_n7, assign28710_e27704_d_n8, assign28710_e27704_d_n9, assign28710_e27704_d_n10, assign28710_e27704_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28710_e27700: f64 = (locals.var_tmf1 * 0.8);
        let assign28710_e27702: f64 = (assign28710_e27700 * locals.var_dnm);
        (assign28710_e27702, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.8) * locals.var_dnm) + (assign28710_e27700 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign28710_e27704;
        locals.var_tmf0_dn0 = assign28710_e27704_d_n0;
        locals.var_tmf0_dn2 = assign28710_e27704_d_n2;
        locals.var_tmf0_dn4 = assign28710_e27704_d_n4;
        locals.var_tmf0_dn5 = assign28710_e27704_d_n5;
        locals.var_tmf0_dn6 = assign28710_e27704_d_n6;
        locals.var_tmf0_dn7 = assign28710_e27704_d_n7;
        locals.var_tmf0_dn8 = assign28710_e27704_d_n8;
        locals.var_tmf0_dn9 = assign28710_e27704_d_n9;
        locals.var_tmf0_dn10 = assign28710_e27704_d_n10;
        locals.var_tmf0_dn13 = assign28710_e27704_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign28720_e27720, assign28720_e27720_d_n0, assign28720_e27720_d_n2, assign28720_e27720_d_n4, assign28720_e27720_d_n5, assign28720_e27720_d_n6, assign28720_e27720_d_n7, assign28720_e27720_d_n8, assign28720_e27720_d_n9, assign28720_e27720_d_n10, assign28720_e27720_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28720_e27714: f64 = (0.8 * locals.var_xmp);
        let assign28720_e27716: f64 = (assign28720_e27714 * locals.var_dnm);
        let assign28720_e27718: f64 = (assign28720_e27716 / locals.var_arg);
        (assign28720_e27718, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign28720_e27714 * locals.var_dnm_dn13)) * locals.var_arg) - (assign28720_e27716 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign28720_e27720;
        locals.var_t1_dn0 = assign28720_e27720_d_n0;
        locals.var_t1_dn2 = assign28720_e27720_d_n2;
        locals.var_t1_dn4 = assign28720_e27720_d_n4;
        locals.var_t1_dn5 = assign28720_e27720_d_n5;
        locals.var_t1_dn6 = assign28720_e27720_d_n6;
        locals.var_t1_dn7 = assign28720_e27720_d_n7;
        locals.var_t1_dn8 = assign28720_e27720_d_n8;
        locals.var_t1_dn9 = assign28720_e27720_d_n9;
        locals.var_t1_dn10 = assign28720_e27720_d_n10;
        locals.var_t1_dn13 = assign28720_e27720_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign28730_e27734, assign28730_e27734_d_n0, assign28730_e27734_d_n2, assign28730_e27734_d_n4, assign28730_e27734_d_n5, assign28730_e27734_d_n6, assign28730_e27734_d_n7, assign28730_e27734_d_n8, assign28730_e27734_d_n9, assign28730_e27734_d_n10, assign28730_e27734_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign28730_e27730: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28730_e27732: f64 = (assign28730_e27730 + locals.var_tmf0);
        (assign28730_e27732, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign28730_e27734;
        locals.var_t2_dn0 = assign28730_e27734_d_n0;
        locals.var_t2_dn2 = assign28730_e27734_d_n2;
        locals.var_t2_dn4 = assign28730_e27734_d_n4;
        locals.var_t2_dn5 = assign28730_e27734_d_n5;
        locals.var_t2_dn6 = assign28730_e27734_d_n6;
        locals.var_t2_dn7 = assign28730_e27734_d_n7;
        locals.var_t2_dn8 = assign28730_e27734_d_n8;
        locals.var_t2_dn9 = assign28730_e27734_d_n9;
        locals.var_t2_dn10 = assign28730_e27734_d_n10;
        locals.var_t2_dn13 = assign28730_e27734_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign28740_e27744, assign28740_e27744_d_n0, assign28740_e27744_d_n2, assign28740_e27744_d_n4, assign28740_e27744_d_n5, assign28740_e27744_d_n6, assign28740_e27744_d_n7, assign28740_e27744_d_n8, assign28740_e27744_d_n9, assign28740_e27744_d_n10, assign28740_e27744_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign28740_e27744;
        locals.var_t1_dn0 = assign28740_e27744_d_n0;
        locals.var_t1_dn2 = assign28740_e27744_d_n2;
        locals.var_t1_dn4 = assign28740_e27744_d_n4;
        locals.var_t1_dn5 = assign28740_e27744_d_n5;
        locals.var_t1_dn6 = assign28740_e27744_d_n6;
        locals.var_t1_dn7 = assign28740_e27744_d_n7;
        locals.var_t1_dn8 = assign28740_e27744_d_n8;
        locals.var_t1_dn9 = assign28740_e27744_d_n9;
        locals.var_t1_dn10 = assign28740_e27744_d_n10;
        locals.var_t1_dn13 = assign28740_e27744_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign28750_e27755, assign28750_e27755_d_n0, assign28750_e27755_d_n2, assign28750_e27755_d_n4, assign28750_e27755_d_n5, assign28750_e27755_d_n6, assign28750_e27755_d_n7, assign28750_e27755_d_n8, assign28750_e27755_d_n9, assign28750_e27755_d_n10, assign28750_e27755_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 == 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign28750_e27755;
        locals.var_t2_dn0 = assign28750_e27755_d_n0;
        locals.var_t2_dn2 = assign28750_e27755_d_n2;
        locals.var_t2_dn4 = assign28750_e27755_d_n4;
        locals.var_t2_dn5 = assign28750_e27755_d_n5;
        locals.var_t2_dn6 = assign28750_e27755_d_n6;
        locals.var_t2_dn7 = assign28750_e27755_d_n7;
        locals.var_t2_dn8 = assign28750_e27755_d_n8;
        locals.var_t2_dn9 = assign28750_e27755_d_n9;
        locals.var_t2_dn10 = assign28750_e27755_d_n10;
        locals.var_t2_dn13 = assign28750_e27755_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign28760_e27766, assign28760_e27766_d_n0, assign28760_e27766_d_n2, assign28760_e27766_d_n4, assign28760_e27766_d_n5, assign28760_e27766_d_n6, assign28760_e27766_d_n7, assign28760_e27766_d_n8, assign28760_e27766_d_n9, assign28760_e27766_d_n10, assign28760_e27766_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign28760_e27766;
        locals.var_t1_dn0 = assign28760_e27766_d_n0;
        locals.var_t1_dn2 = assign28760_e27766_d_n2;
        locals.var_t1_dn4 = assign28760_e27766_d_n4;
        locals.var_t1_dn5 = assign28760_e27766_d_n5;
        locals.var_t1_dn6 = assign28760_e27766_d_n6;
        locals.var_t1_dn7 = assign28760_e27766_d_n7;
        locals.var_t1_dn8 = assign28760_e27766_d_n8;
        locals.var_t1_dn9 = assign28760_e27766_d_n9;
        locals.var_t1_dn10 = assign28760_e27766_d_n10;
        locals.var_t1_dn13 = assign28760_e27766_d_n13;
        locals.var_t1_rv = 0.0;

        let assign28770_e27770: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28770_e27775: f64 = if ((locals.var_phib_ref > assign28770_e27770) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard674 = assign28770_e27775;
        locals.var_guard674_rv = 0.0;

        let (assign28780_e27790, assign28780_e27790_d_n0, assign28780_e27790_d_n2, assign28780_e27790_d_n4, assign28780_e27790_d_n5, assign28780_e27790_d_n6, assign28780_e27790_d_n7, assign28780_e27790_d_n8, assign28780_e27790_d_n9, assign28780_e27790_d_n10, assign28780_e27790_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28780_e27786: f64 = (locals.var_phib_ref - locals.var_vds_maxb0);
        let assign28780_e27788: f64 = (assign28780_e27786 + 0.8);
        (assign28780_e27788, (locals.var_phib_ref_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phib_ref_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phib_ref_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phib_ref_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phib_ref_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phib_ref_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phib_ref_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phib_ref_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phib_ref_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phib_ref_dn13 - locals.var_vds_maxb0_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign28780_e27790;
        locals.var_tmf1_dn0 = assign28780_e27790_d_n0;
        locals.var_tmf1_dn2 = assign28780_e27790_d_n2;
        locals.var_tmf1_dn4 = assign28780_e27790_d_n4;
        locals.var_tmf1_dn5 = assign28780_e27790_d_n5;
        locals.var_tmf1_dn6 = assign28780_e27790_d_n6;
        locals.var_tmf1_dn7 = assign28780_e27790_d_n7;
        locals.var_tmf1_dn8 = assign28780_e27790_d_n8;
        locals.var_tmf1_dn9 = assign28780_e27790_d_n9;
        locals.var_tmf1_dn10 = assign28780_e27790_d_n10;
        locals.var_tmf1_dn13 = assign28780_e27790_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign28790_e27803, assign28790_e27803_d_n0, assign28790_e27803_d_n2, assign28790_e27803_d_n4, assign28790_e27803_d_n5, assign28790_e27803_d_n6, assign28790_e27803_d_n7, assign28790_e27803_d_n8, assign28790_e27803_d_n9, assign28790_e27803_d_n10, assign28790_e27803_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28790_e27801: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28790_e27801, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign28790_e27803;
        locals.var_x2_dn0 = assign28790_e27803_d_n0;
        locals.var_x2_dn2 = assign28790_e27803_d_n2;
        locals.var_x2_dn4 = assign28790_e27803_d_n4;
        locals.var_x2_dn5 = assign28790_e27803_d_n5;
        locals.var_x2_dn6 = assign28790_e27803_d_n6;
        locals.var_x2_dn7 = assign28790_e27803_d_n7;
        locals.var_x2_dn8 = assign28790_e27803_d_n8;
        locals.var_x2_dn9 = assign28790_e27803_d_n9;
        locals.var_x2_dn10 = assign28790_e27803_d_n10;
        locals.var_x2_dn13 = assign28790_e27803_d_n13;
        locals.var_x2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign28800_e27816, assign28800_e27816_d_n0, assign28800_e27816_d_n2, assign28800_e27816_d_n4, assign28800_e27816_d_n5, assign28800_e27816_d_n6, assign28800_e27816_d_n7, assign28800_e27816_d_n8, assign28800_e27816_d_n9, assign28800_e27816_d_n10, assign28800_e27816_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28800_e27814: f64 = (0.8 * 0.8);
        (assign28800_e27814, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign28800_e27816;
        locals.var_xmax2_dn0 = assign28800_e27816_d_n0;
        locals.var_xmax2_dn2 = assign28800_e27816_d_n2;
        locals.var_xmax2_dn4 = assign28800_e27816_d_n4;
        locals.var_xmax2_dn5 = assign28800_e27816_d_n5;
        locals.var_xmax2_dn6 = assign28800_e27816_d_n6;
        locals.var_xmax2_dn7 = assign28800_e27816_d_n7;
        locals.var_xmax2_dn8 = assign28800_e27816_d_n8;
        locals.var_xmax2_dn9 = assign28800_e27816_d_n9;
        locals.var_xmax2_dn10 = assign28800_e27816_d_n10;
        locals.var_xmax2_dn13 = assign28800_e27816_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign28810_e27827, assign28810_e27827_d_n0, assign28810_e27827_d_n2, assign28810_e27827_d_n4, assign28810_e27827_d_n5, assign28810_e27827_d_n6, assign28810_e27827_d_n7, assign28810_e27827_d_n8, assign28810_e27827_d_n9, assign28810_e27827_d_n10, assign28810_e27827_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28810_e27827;
        locals.var_xp_dn0 = assign28810_e27827_d_n0;
        locals.var_xp_dn2 = assign28810_e27827_d_n2;
        locals.var_xp_dn4 = assign28810_e27827_d_n4;
        locals.var_xp_dn5 = assign28810_e27827_d_n5;
        locals.var_xp_dn6 = assign28810_e27827_d_n6;
        locals.var_xp_dn7 = assign28810_e27827_d_n7;
        locals.var_xp_dn8 = assign28810_e27827_d_n8;
        locals.var_xp_dn9 = assign28810_e27827_d_n9;
        locals.var_xp_dn10 = assign28810_e27827_d_n10;
        locals.var_xp_dn13 = assign28810_e27827_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28820_e27838, assign28820_e27838_d_n0, assign28820_e27838_d_n2, assign28820_e27838_d_n4, assign28820_e27838_d_n5, assign28820_e27838_d_n6, assign28820_e27838_d_n7, assign28820_e27838_d_n8, assign28820_e27838_d_n9, assign28820_e27838_d_n10, assign28820_e27838_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28820_e27838;
        locals.var_xmp_dn0 = assign28820_e27838_d_n0;
        locals.var_xmp_dn2 = assign28820_e27838_d_n2;
        locals.var_xmp_dn4 = assign28820_e27838_d_n4;
        locals.var_xmp_dn5 = assign28820_e27838_d_n5;
        locals.var_xmp_dn6 = assign28820_e27838_d_n6;
        locals.var_xmp_dn7 = assign28820_e27838_d_n7;
        locals.var_xmp_dn8 = assign28820_e27838_d_n8;
        locals.var_xmp_dn9 = assign28820_e27838_d_n9;
        locals.var_xmp_dn10 = assign28820_e27838_d_n10;
        locals.var_xmp_dn13 = assign28820_e27838_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28830_e27849,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28830_e27849;
        locals.var_m0_rv = 0.0;

        let (assign28840_e27860,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28840_e27860;
        locals.var_mm_rv = 0.0;

        let (assign28850_e27871, assign28850_e27871_d_n0, assign28850_e27871_d_n2, assign28850_e27871_d_n4, assign28850_e27871_d_n5, assign28850_e27871_d_n6, assign28850_e27871_d_n7, assign28850_e27871_d_n8, assign28850_e27871_d_n9, assign28850_e27871_d_n10, assign28850_e27871_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign28850_e27871;
        locals.var_arg_dn0 = assign28850_e27871_d_n0;
        locals.var_arg_dn2 = assign28850_e27871_d_n2;
        locals.var_arg_dn4 = assign28850_e27871_d_n4;
        locals.var_arg_dn5 = assign28850_e27871_d_n5;
        locals.var_arg_dn6 = assign28850_e27871_d_n6;
        locals.var_arg_dn7 = assign28850_e27871_d_n7;
        locals.var_arg_dn8 = assign28850_e27871_d_n8;
        locals.var_arg_dn9 = assign28850_e27871_d_n9;
        locals.var_arg_dn10 = assign28850_e27871_d_n10;
        locals.var_arg_dn13 = assign28850_e27871_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign28860_e27882, assign28860_e27882_d_n0, assign28860_e27882_d_n2, assign28860_e27882_d_n4, assign28860_e27882_d_n5, assign28860_e27882_d_n6, assign28860_e27882_d_n7, assign28860_e27882_d_n8, assign28860_e27882_d_n9, assign28860_e27882_d_n10, assign28860_e27882_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28860_e27882;
        locals.var_dnm_dn0 = assign28860_e27882_d_n0;
        locals.var_dnm_dn2 = assign28860_e27882_d_n2;
        locals.var_dnm_dn4 = assign28860_e27882_d_n4;
        locals.var_dnm_dn5 = assign28860_e27882_d_n5;
        locals.var_dnm_dn6 = assign28860_e27882_d_n6;
        locals.var_dnm_dn7 = assign28860_e27882_d_n7;
        locals.var_dnm_dn8 = assign28860_e27882_d_n8;
        locals.var_dnm_dn9 = assign28860_e27882_d_n9;
        locals.var_dnm_dn10 = assign28860_e27882_d_n10;
        locals.var_dnm_dn13 = assign28860_e27882_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign28870_e27895, assign28870_e27895_d_n0, assign28870_e27895_d_n2, assign28870_e27895_d_n4, assign28870_e27895_d_n5, assign28870_e27895_d_n6, assign28870_e27895_d_n7, assign28870_e27895_d_n8, assign28870_e27895_d_n9, assign28870_e27895_d_n10, assign28870_e27895_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28870_e27893: f64 = (locals.var_xp * locals.var_x2);
        (assign28870_e27893, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28870_e27895;
        locals.var_xp_dn0 = assign28870_e27895_d_n0;
        locals.var_xp_dn2 = assign28870_e27895_d_n2;
        locals.var_xp_dn4 = assign28870_e27895_d_n4;
        locals.var_xp_dn5 = assign28870_e27895_d_n5;
        locals.var_xp_dn6 = assign28870_e27895_d_n6;
        locals.var_xp_dn7 = assign28870_e27895_d_n7;
        locals.var_xp_dn8 = assign28870_e27895_d_n8;
        locals.var_xp_dn9 = assign28870_e27895_d_n9;
        locals.var_xp_dn10 = assign28870_e27895_d_n10;
        locals.var_xp_dn13 = assign28870_e27895_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28880_e27908, assign28880_e27908_d_n0, assign28880_e27908_d_n2, assign28880_e27908_d_n4, assign28880_e27908_d_n5, assign28880_e27908_d_n6, assign28880_e27908_d_n7, assign28880_e27908_d_n8, assign28880_e27908_d_n9, assign28880_e27908_d_n10, assign28880_e27908_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28880_e27906: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28880_e27906, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28880_e27908;
        locals.var_xmp_dn0 = assign28880_e27908_d_n0;
        locals.var_xmp_dn2 = assign28880_e27908_d_n2;
        locals.var_xmp_dn4 = assign28880_e27908_d_n4;
        locals.var_xmp_dn5 = assign28880_e27908_d_n5;
        locals.var_xmp_dn6 = assign28880_e27908_d_n6;
        locals.var_xmp_dn7 = assign28880_e27908_d_n7;
        locals.var_xmp_dn8 = assign28880_e27908_d_n8;
        locals.var_xmp_dn9 = assign28880_e27908_d_n9;
        locals.var_xmp_dn10 = assign28880_e27908_d_n10;
        locals.var_xmp_dn13 = assign28880_e27908_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28890_e27921, assign28890_e27921_d_n0, assign28890_e27921_d_n2, assign28890_e27921_d_n4, assign28890_e27921_d_n5, assign28890_e27921_d_n6, assign28890_e27921_d_n7, assign28890_e27921_d_n8, assign28890_e27921_d_n9, assign28890_e27921_d_n10, assign28890_e27921_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28890_e27919: f64 = (locals.var_xp * locals.var_x2);
        (assign28890_e27919, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign28890_e27921;
        locals.var_xp_dn0 = assign28890_e27921_d_n0;
        locals.var_xp_dn2 = assign28890_e27921_d_n2;
        locals.var_xp_dn4 = assign28890_e27921_d_n4;
        locals.var_xp_dn5 = assign28890_e27921_d_n5;
        locals.var_xp_dn6 = assign28890_e27921_d_n6;
        locals.var_xp_dn7 = assign28890_e27921_d_n7;
        locals.var_xp_dn8 = assign28890_e27921_d_n8;
        locals.var_xp_dn9 = assign28890_e27921_d_n9;
        locals.var_xp_dn10 = assign28890_e27921_d_n10;
        locals.var_xp_dn13 = assign28890_e27921_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign28900_e27934, assign28900_e27934_d_n0, assign28900_e27934_d_n2, assign28900_e27934_d_n4, assign28900_e27934_d_n5, assign28900_e27934_d_n6, assign28900_e27934_d_n7, assign28900_e27934_d_n8, assign28900_e27934_d_n9, assign28900_e27934_d_n10, assign28900_e27934_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28900_e27932: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28900_e27932, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign28900_e27934;
        locals.var_xmp_dn0 = assign28900_e27934_d_n0;
        locals.var_xmp_dn2 = assign28900_e27934_d_n2;
        locals.var_xmp_dn4 = assign28900_e27934_d_n4;
        locals.var_xmp_dn5 = assign28900_e27934_d_n5;
        locals.var_xmp_dn6 = assign28900_e27934_d_n6;
        locals.var_xmp_dn7 = assign28900_e27934_d_n7;
        locals.var_xmp_dn8 = assign28900_e27934_d_n8;
        locals.var_xmp_dn9 = assign28900_e27934_d_n9;
        locals.var_xmp_dn10 = assign28900_e27934_d_n10;
        locals.var_xmp_dn13 = assign28900_e27934_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign28910_e27947, assign28910_e27947_d_n0, assign28910_e27947_d_n2, assign28910_e27947_d_n4, assign28910_e27947_d_n5, assign28910_e27947_d_n6, assign28910_e27947_d_n7, assign28910_e27947_d_n8, assign28910_e27947_d_n9, assign28910_e27947_d_n10, assign28910_e27947_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign28910_e27945: f64 = (locals.var_xp + locals.var_xmp);
        (assign28910_e27945, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign28910_e27947;
        locals.var_arg_dn0 = assign28910_e27947_d_n0;
        locals.var_arg_dn2 = assign28910_e27947_d_n2;
        locals.var_arg_dn4 = assign28910_e27947_d_n4;
        locals.var_arg_dn5 = assign28910_e27947_d_n5;
        locals.var_arg_dn6 = assign28910_e27947_d_n6;
        locals.var_arg_dn7 = assign28910_e27947_d_n7;
        locals.var_arg_dn8 = assign28910_e27947_d_n8;
        locals.var_arg_dn9 = assign28910_e27947_d_n9;
        locals.var_arg_dn10 = assign28910_e27947_d_n10;
        locals.var_arg_dn13 = assign28910_e27947_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign28920_e27958, assign28920_e27958_d_n0, assign28920_e27958_d_n2, assign28920_e27958_d_n4, assign28920_e27958_d_n5, assign28920_e27958_d_n6, assign28920_e27958_d_n7, assign28920_e27958_d_n8, assign28920_e27958_d_n9, assign28920_e27958_d_n10, assign28920_e27958_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign28920_e27958;
        locals.var_dnm_dn0 = assign28920_e27958_d_n0;
        locals.var_dnm_dn2 = assign28920_e27958_d_n2;
        locals.var_dnm_dn4 = assign28920_e27958_d_n4;
        locals.var_dnm_dn5 = assign28920_e27958_d_n5;
        locals.var_dnm_dn6 = assign28920_e27958_d_n6;
        locals.var_dnm_dn7 = assign28920_e27958_d_n7;
        locals.var_dnm_dn8 = assign28920_e27958_d_n8;
        locals.var_dnm_dn9 = assign28920_e27958_d_n9;
        locals.var_dnm_dn10 = assign28920_e27958_d_n10;
        locals.var_dnm_dn13 = assign28920_e27958_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign28930_e27973: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard675 = assign28930_e27973;
        locals.var_guard675_rv = 0.0;

        let assign28940_e27976: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign28940_e27976;
        locals.var_guard676_rv = 0.0;

        let (assign28950_e27991,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) && (locals.var_guard676 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28950_e27991;
        locals.var_mm_rv = 0.0;

        let assign28960_e27994: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign28960_e27994;
        locals.var_guard677_rv = 0.0;

        let (assign28970_e28012,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28970_e28012;
        locals.var_mm_rv = 0.0;

        let assign28980_e28015: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign28980_e28015;
        locals.var_guard678_rv = 0.0;

        let (assign28990_e28036,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28990_e28036;
        locals.var_mm_rv = 0.0;

        let assign29000_e28039: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign29000_e28039;
        locals.var_guard679_rv = 0.0;

        let (assign29010_e28063,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 == 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29010_e28063;
        locals.var_mm_rv = 0.0;

        let (assign29020_e28076,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29020_e28076;
        locals.var_m0_rv = 0.0;

        let mut assign29030_loop_guard: usize = 0;
        while {
            let assign29030_cond_e28090: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29030_cond_e28090 != 0.0
        } {
            assign29030_loop_guard += 1;
            assert!(assign29030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29030_body0_e28104, assign29030_body0_e28104_d_n0, assign29030_body0_e28104_d_n2, assign29030_body0_e28104_d_n4, assign29030_body0_e28104_d_n5, assign29030_body0_e28104_d_n6, assign29030_body0_e28104_d_n7, assign29030_body0_e28104_d_n8, assign29030_body0_e28104_d_n9, assign29030_body0_e28104_d_n10, assign29030_body0_e28104_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) {
        let assign29030_body0_e28102: f64 = (locals.var_dnm).sqrt();
        (assign29030_body0_e28102, (locals.var_dnm_dn0 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn2 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn4 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn5 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn6 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn7 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn8 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn9 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn10 / (2.0 * assign29030_body0_e28102)), (locals.var_dnm_dn13 / (2.0 * assign29030_body0_e28102)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign29030_body0_e28104;
            locals.var_dnm_dn0 = assign29030_body0_e28104_d_n0;
            locals.var_dnm_dn2 = assign29030_body0_e28104_d_n2;
            locals.var_dnm_dn4 = assign29030_body0_e28104_d_n4;
            locals.var_dnm_dn5 = assign29030_body0_e28104_d_n5;
            locals.var_dnm_dn6 = assign29030_body0_e28104_d_n6;
            locals.var_dnm_dn7 = assign29030_body0_e28104_d_n7;
            locals.var_dnm_dn8 = assign29030_body0_e28104_d_n8;
            locals.var_dnm_dn9 = assign29030_body0_e28104_d_n9;
            locals.var_dnm_dn10 = assign29030_body0_e28104_d_n10;
            locals.var_dnm_dn13 = assign29030_body0_e28104_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign29030_body1_e28119,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) {
        let assign29030_body1_e28117: f64 = (locals.var_m0 + 1.0);
        (assign29030_body1_e28117,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29030_body1_e28119;
            locals.var_m0_rv = 0.0;
        }

        let (assign29040_e28144, assign29040_e28144_d_n0, assign29040_e28144_d_n2, assign29040_e28144_d_n4, assign29040_e28144_d_n5, assign29040_e28144_d_n6, assign29040_e28144_d_n7, assign29040_e28144_d_n8, assign29040_e28144_d_n9, assign29040_e28144_d_n10, assign29040_e28144_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 == 0.0)) {
        let (assign29040_e28142, assign29040_e28142_d_n0, assign29040_e28142_d_n2, assign29040_e28142_d_n4, assign29040_e28142_d_n5, assign29040_e28142_d_n6, assign29040_e28142_d_n7, assign29040_e28142_d_n8, assign29040_e28142_d_n9, assign29040_e28142_d_n10, assign29040_e28142_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29040_e28139: f64 = (2.0 * 2.0);
                let assign29040_e28140: f64 = (1.0 / assign29040_e28139);
                let assign29040_e28141: f64 = (locals.var_dnm).powf(assign29040_e28140);
                (assign29040_e28141, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29040_e28140) as f64).is_finite() && ((assign29040_e28140) as f64).fract() == 0.0 { if assign29040_e28140 == 0.0 { 0.0 } else { (assign29040_e28140 * ((locals.var_dnm).powf(assign29040_e28140 - 1.0) * locals.var_dnm_dn13)) } } else { (assign29040_e28141 * (assign29040_e28140 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign29040_e28142, assign29040_e28142_d_n0, assign29040_e28142_d_n2, assign29040_e28142_d_n4, assign29040_e28142_d_n5, assign29040_e28142_d_n6, assign29040_e28142_d_n7, assign29040_e28142_d_n8, assign29040_e28142_d_n9, assign29040_e28142_d_n10, assign29040_e28142_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29040_e28144;
        locals.var_dnm_dn0 = assign29040_e28144_d_n0;
        locals.var_dnm_dn2 = assign29040_e28144_d_n2;
        locals.var_dnm_dn4 = assign29040_e28144_d_n4;
        locals.var_dnm_dn5 = assign29040_e28144_d_n5;
        locals.var_dnm_dn6 = assign29040_e28144_d_n6;
        locals.var_dnm_dn7 = assign29040_e28144_d_n7;
        locals.var_dnm_dn8 = assign29040_e28144_d_n8;
        locals.var_dnm_dn9 = assign29040_e28144_d_n9;
        locals.var_dnm_dn10 = assign29040_e28144_d_n10;
        locals.var_dnm_dn13 = assign29040_e28144_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign29050_e28157, assign29050_e28157_d_n0, assign29050_e28157_d_n2, assign29050_e28157_d_n4, assign29050_e28157_d_n5, assign29050_e28157_d_n6, assign29050_e28157_d_n7, assign29050_e28157_d_n8, assign29050_e28157_d_n9, assign29050_e28157_d_n10, assign29050_e28157_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign29050_e28155: f64 = (1.0 / locals.var_dnm);
        (assign29050_e28155, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29050_e28157;
        locals.var_dnm_dn0 = assign29050_e28157_d_n0;
        locals.var_dnm_dn2 = assign29050_e28157_d_n2;
        locals.var_dnm_dn4 = assign29050_e28157_d_n4;
        locals.var_dnm_dn5 = assign29050_e28157_d_n5;
        locals.var_dnm_dn6 = assign29050_e28157_d_n6;
        locals.var_dnm_dn7 = assign29050_e28157_d_n7;
        locals.var_dnm_dn8 = assign29050_e28157_d_n8;
        locals.var_dnm_dn9 = assign29050_e28157_d_n9;
        locals.var_dnm_dn10 = assign29050_e28157_d_n10;
        locals.var_dnm_dn13 = assign29050_e28157_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign29060_e28172, assign29060_e28172_d_n0, assign29060_e28172_d_n2, assign29060_e28172_d_n4, assign29060_e28172_d_n5, assign29060_e28172_d_n6, assign29060_e28172_d_n7, assign29060_e28172_d_n8, assign29060_e28172_d_n9, assign29060_e28172_d_n10, assign29060_e28172_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign29060_e28168: f64 = (locals.var_tmf1 * 0.8);
        let assign29060_e28170: f64 = (assign29060_e28168 * locals.var_dnm);
        (assign29060_e28170, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.8) * locals.var_dnm) + (assign29060_e28168 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign29060_e28172;
        locals.var_tmf0_dn0 = assign29060_e28172_d_n0;
        locals.var_tmf0_dn2 = assign29060_e28172_d_n2;
        locals.var_tmf0_dn4 = assign29060_e28172_d_n4;
        locals.var_tmf0_dn5 = assign29060_e28172_d_n5;
        locals.var_tmf0_dn6 = assign29060_e28172_d_n6;
        locals.var_tmf0_dn7 = assign29060_e28172_d_n7;
        locals.var_tmf0_dn8 = assign29060_e28172_d_n8;
        locals.var_tmf0_dn9 = assign29060_e28172_d_n9;
        locals.var_tmf0_dn10 = assign29060_e28172_d_n10;
        locals.var_tmf0_dn13 = assign29060_e28172_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign29070_e28189, assign29070_e28189_d_n0, assign29070_e28189_d_n2, assign29070_e28189_d_n4, assign29070_e28189_d_n5, assign29070_e28189_d_n6, assign29070_e28189_d_n7, assign29070_e28189_d_n8, assign29070_e28189_d_n9, assign29070_e28189_d_n10, assign29070_e28189_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign29070_e28183: f64 = (0.8 * locals.var_xmp);
        let assign29070_e28185: f64 = (assign29070_e28183 * locals.var_dnm);
        let assign29070_e28187: f64 = (assign29070_e28185 / locals.var_arg);
        (assign29070_e28187, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign29070_e28183 * locals.var_dnm_dn13)) * locals.var_arg) - (assign29070_e28185 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29070_e28189;
        locals.var_t0_dn0 = assign29070_e28189_d_n0;
        locals.var_t0_dn2 = assign29070_e28189_d_n2;
        locals.var_t0_dn4 = assign29070_e28189_d_n4;
        locals.var_t0_dn5 = assign29070_e28189_d_n5;
        locals.var_t0_dn6 = assign29070_e28189_d_n6;
        locals.var_t0_dn7 = assign29070_e28189_d_n7;
        locals.var_t0_dn8 = assign29070_e28189_d_n8;
        locals.var_t0_dn9 = assign29070_e28189_d_n9;
        locals.var_t0_dn10 = assign29070_e28189_d_n10;
        locals.var_t0_dn13 = assign29070_e28189_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29080_e28204, assign29080_e28204_d_n0, assign29080_e28204_d_n2, assign29080_e28204_d_n4, assign29080_e28204_d_n5, assign29080_e28204_d_n6, assign29080_e28204_d_n7, assign29080_e28204_d_n8, assign29080_e28204_d_n9, assign29080_e28204_d_n10, assign29080_e28204_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign29080_e28200: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign29080_e28202: f64 = (assign29080_e28200 + locals.var_tmf0);
        (assign29080_e28202, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign29080_e28204;
        locals.var_t2_dn0 = assign29080_e28204_d_n0;
        locals.var_t2_dn2 = assign29080_e28204_d_n2;
        locals.var_t2_dn4 = assign29080_e28204_d_n4;
        locals.var_t2_dn5 = assign29080_e28204_d_n5;
        locals.var_t2_dn6 = assign29080_e28204_d_n6;
        locals.var_t2_dn7 = assign29080_e28204_d_n7;
        locals.var_t2_dn8 = assign29080_e28204_d_n8;
        locals.var_t2_dn9 = assign29080_e28204_d_n9;
        locals.var_t2_dn10 = assign29080_e28204_d_n10;
        locals.var_t2_dn13 = assign29080_e28204_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign29090_e28215, assign29090_e28215_d_n0, assign29090_e28215_d_n2, assign29090_e28215_d_n4, assign29090_e28215_d_n5, assign29090_e28215_d_n6, assign29090_e28215_d_n7, assign29090_e28215_d_n8, assign29090_e28215_d_n9, assign29090_e28215_d_n10, assign29090_e28215_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29090_e28215;
        locals.var_t0_dn0 = assign29090_e28215_d_n0;
        locals.var_t0_dn2 = assign29090_e28215_d_n2;
        locals.var_t0_dn4 = assign29090_e28215_d_n4;
        locals.var_t0_dn5 = assign29090_e28215_d_n5;
        locals.var_t0_dn6 = assign29090_e28215_d_n6;
        locals.var_t0_dn7 = assign29090_e28215_d_n7;
        locals.var_t0_dn8 = assign29090_e28215_d_n8;
        locals.var_t0_dn9 = assign29090_e28215_d_n9;
        locals.var_t0_dn10 = assign29090_e28215_d_n10;
        locals.var_t0_dn13 = assign29090_e28215_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        locals: &mut StampLocals,
    ) {
        let (assign29100_e28227, assign29100_e28227_d_n0, assign29100_e28227_d_n2, assign29100_e28227_d_n4, assign29100_e28227_d_n5, assign29100_e28227_d_n6, assign29100_e28227_d_n7, assign29100_e28227_d_n8, assign29100_e28227_d_n9, assign29100_e28227_d_n10, assign29100_e28227_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 == 0.0)) {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign29100_e28227;
        locals.var_t2_dn0 = assign29100_e28227_d_n0;
        locals.var_t2_dn2 = assign29100_e28227_d_n2;
        locals.var_t2_dn4 = assign29100_e28227_d_n4;
        locals.var_t2_dn5 = assign29100_e28227_d_n5;
        locals.var_t2_dn6 = assign29100_e28227_d_n6;
        locals.var_t2_dn7 = assign29100_e28227_d_n7;
        locals.var_t2_dn8 = assign29100_e28227_d_n8;
        locals.var_t2_dn9 = assign29100_e28227_d_n9;
        locals.var_t2_dn10 = assign29100_e28227_d_n10;
        locals.var_t2_dn13 = assign29100_e28227_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign29110_e28239, assign29110_e28239_d_n0, assign29110_e28239_d_n2, assign29110_e28239_d_n4, assign29110_e28239_d_n5, assign29110_e28239_d_n6, assign29110_e28239_d_n7, assign29110_e28239_d_n8, assign29110_e28239_d_n9, assign29110_e28239_d_n10, assign29110_e28239_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard674 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29110_e28239;
        locals.var_t0_dn0 = assign29110_e28239_d_n0;
        locals.var_t0_dn2 = assign29110_e28239_d_n2;
        locals.var_t0_dn4 = assign29110_e28239_d_n4;
        locals.var_t0_dn5 = assign29110_e28239_d_n5;
        locals.var_t0_dn6 = assign29110_e28239_d_n6;
        locals.var_t0_dn7 = assign29110_e28239_d_n7;
        locals.var_t0_dn8 = assign29110_e28239_d_n8;
        locals.var_t0_dn9 = assign29110_e28239_d_n9;
        locals.var_t0_dn10 = assign29110_e28239_d_n10;
        locals.var_t0_dn13 = assign29110_e28239_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29120_e28257, assign29120_e28257_d_n0, assign29120_e28257_d_n2, assign29120_e28257_d_n4, assign29120_e28257_d_n5, assign29120_e28257_d_n6, assign29120_e28257_d_n7, assign29120_e28257_d_n8, assign29120_e28257_d_n9, assign29120_e28257_d_n10, assign29120_e28257_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign29120_e28244: f64 = (-1.6021918e-19);
        let assign29120_e28246: f64 = (assign29120_e28244 * locals.var_uc_ndepm);
        let assign29120_e28250: f64 = (locals.var_t2 - locals.var_vds_maxb0);
        let assign29120_e28251: f64 = (locals.var_beta * assign29120_e28250);
        let assign29120_e28252: f64 = (assign29120_e28251).exp();
        let assign29120_e28253: f64 = (assign29120_e28246 * assign29120_e28252);
        let assign29120_e28255: f64 = (assign29120_e28253 * locals.var_w_b0);
        (assign29120_e28255, (((((assign29120_e28244 * locals.var_uc_ndepm_dn0) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn0 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn0 - locals.var_vds_maxb0_dn0)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn0)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn2) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn2 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn2 - locals.var_vds_maxb0_dn2)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn2)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn4) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn4 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn4 - locals.var_vds_maxb0_dn4)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn4)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn5) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn5 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn5 - locals.var_vds_maxb0_dn5)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn5)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn6) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn6 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn6 - locals.var_vds_maxb0_dn6)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn6)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn7) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn7 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn7 - locals.var_vds_maxb0_dn7)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn7)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn8) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn8 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn8 - locals.var_vds_maxb0_dn8)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn8)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn9) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn9 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn9 - locals.var_vds_maxb0_dn9)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn9)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn10) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn10 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn10 - locals.var_vds_maxb0_dn10)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn10)), (((((assign29120_e28244 * locals.var_uc_ndepm_dn13) * assign29120_e28252) + (assign29120_e28246 * (assign29120_e28252 * ((locals.var_beta_dn13 * assign29120_e28250) + (locals.var_beta * (locals.var_t2_dn13 - locals.var_vds_maxb0_dn13)))))) * locals.var_w_b0) + (assign29120_e28253 * locals.var_w_b0_dn13)),)
    } else {
        (locals.var_qn_bac, locals.var_qn_bac_dn0, locals.var_qn_bac_dn2, locals.var_qn_bac_dn4, locals.var_qn_bac_dn5, locals.var_qn_bac_dn6, locals.var_qn_bac_dn7, locals.var_qn_bac_dn8, locals.var_qn_bac_dn9, locals.var_qn_bac_dn10, locals.var_qn_bac_dn13,)
    }
};
        locals.var_qn_bac = assign29120_e28257;
        locals.var_qn_bac_dn0 = assign29120_e28257_d_n0;
        locals.var_qn_bac_dn2 = assign29120_e28257_d_n2;
        locals.var_qn_bac_dn4 = assign29120_e28257_d_n4;
        locals.var_qn_bac_dn5 = assign29120_e28257_d_n5;
        locals.var_qn_bac_dn6 = assign29120_e28257_d_n6;
        locals.var_qn_bac_dn7 = assign29120_e28257_d_n7;
        locals.var_qn_bac_dn8 = assign29120_e28257_d_n8;
        locals.var_qn_bac_dn9 = assign29120_e28257_d_n9;
        locals.var_qn_bac_dn10 = assign29120_e28257_d_n10;
        locals.var_qn_bac_dn13 = assign29120_e28257_d_n13;
        locals.var_qn_bac_rv = 0.0;

        let assign29130_e28260: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29130_e28263: f64 = 0.06;
        let assign29130_e28268: f64 = if ((assign29130_e28260 < assign29130_e28263) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard680 = assign29130_e28268;
        locals.var_guard680_rv = 0.0;

        let (assign29140_e28282, assign29140_e28282_d_n0, assign29140_e28282_d_n2, assign29140_e28282_d_n4, assign29140_e28282_d_n5, assign29140_e28282_d_n6, assign29140_e28282_d_n7, assign29140_e28282_d_n8, assign29140_e28282_d_n9, assign29140_e28282_d_n10, assign29140_e28282_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29140_e28276: f64 = 0.06;
        let assign29140_e28279: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29140_e28280: f64 = (assign29140_e28276 - assign29140_e28279);
        (assign29140_e28280, (-(locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0)), (-(locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2)), (-(locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4)), (-(locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5)), (-(locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6)), (-(locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7)), (-(locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8)), (-(locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9)), (-(locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10)), (-(locals.var_phi_s0_dep_dn13 - locals.var_vds_maxb0_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign29140_e28282;
        locals.var_tmf1_dn0 = assign29140_e28282_d_n0;
        locals.var_tmf1_dn2 = assign29140_e28282_d_n2;
        locals.var_tmf1_dn4 = assign29140_e28282_d_n4;
        locals.var_tmf1_dn5 = assign29140_e28282_d_n5;
        locals.var_tmf1_dn6 = assign29140_e28282_d_n6;
        locals.var_tmf1_dn7 = assign29140_e28282_d_n7;
        locals.var_tmf1_dn8 = assign29140_e28282_d_n8;
        locals.var_tmf1_dn9 = assign29140_e28282_d_n9;
        locals.var_tmf1_dn10 = assign29140_e28282_d_n10;
        locals.var_tmf1_dn13 = assign29140_e28282_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign29150_e28292, assign29150_e28292_d_n0, assign29150_e28292_d_n2, assign29150_e28292_d_n4, assign29150_e28292_d_n5, assign29150_e28292_d_n6, assign29150_e28292_d_n7, assign29150_e28292_d_n8, assign29150_e28292_d_n9, assign29150_e28292_d_n10, assign29150_e28292_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29150_e28290: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29150_e28290, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign29150_e28292;
        locals.var_x2_dn0 = assign29150_e28292_d_n0;
        locals.var_x2_dn2 = assign29150_e28292_d_n2;
        locals.var_x2_dn4 = assign29150_e28292_d_n4;
        locals.var_x2_dn5 = assign29150_e28292_d_n5;
        locals.var_x2_dn6 = assign29150_e28292_d_n6;
        locals.var_x2_dn7 = assign29150_e28292_d_n7;
        locals.var_x2_dn8 = assign29150_e28292_d_n8;
        locals.var_x2_dn9 = assign29150_e28292_d_n9;
        locals.var_x2_dn10 = assign29150_e28292_d_n10;
        locals.var_x2_dn13 = assign29150_e28292_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign29160_e28302, assign29160_e28302_d_n0, assign29160_e28302_d_n2, assign29160_e28302_d_n4, assign29160_e28302_d_n5, assign29160_e28302_d_n6, assign29160_e28302_d_n7, assign29160_e28302_d_n8, assign29160_e28302_d_n9, assign29160_e28302_d_n10, assign29160_e28302_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29160_e28300: f64 = (0.06 * 0.06);
        (assign29160_e28300, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign29160_e28302;
        locals.var_xmax2_dn0 = assign29160_e28302_d_n0;
        locals.var_xmax2_dn2 = assign29160_e28302_d_n2;
        locals.var_xmax2_dn4 = assign29160_e28302_d_n4;
        locals.var_xmax2_dn5 = assign29160_e28302_d_n5;
        locals.var_xmax2_dn6 = assign29160_e28302_d_n6;
        locals.var_xmax2_dn7 = assign29160_e28302_d_n7;
        locals.var_xmax2_dn8 = assign29160_e28302_d_n8;
        locals.var_xmax2_dn9 = assign29160_e28302_d_n9;
        locals.var_xmax2_dn10 = assign29160_e28302_d_n10;
        locals.var_xmax2_dn13 = assign29160_e28302_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign29170_e28310, assign29170_e28310_d_n0, assign29170_e28310_d_n2, assign29170_e28310_d_n4, assign29170_e28310_d_n5, assign29170_e28310_d_n6, assign29170_e28310_d_n7, assign29170_e28310_d_n8, assign29170_e28310_d_n9, assign29170_e28310_d_n10, assign29170_e28310_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign29170_e28310;
        locals.var_xp_dn0 = assign29170_e28310_d_n0;
        locals.var_xp_dn2 = assign29170_e28310_d_n2;
        locals.var_xp_dn4 = assign29170_e28310_d_n4;
        locals.var_xp_dn5 = assign29170_e28310_d_n5;
        locals.var_xp_dn6 = assign29170_e28310_d_n6;
        locals.var_xp_dn7 = assign29170_e28310_d_n7;
        locals.var_xp_dn8 = assign29170_e28310_d_n8;
        locals.var_xp_dn9 = assign29170_e28310_d_n9;
        locals.var_xp_dn10 = assign29170_e28310_d_n10;
        locals.var_xp_dn13 = assign29170_e28310_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign29180_e28318, assign29180_e28318_d_n0, assign29180_e28318_d_n2, assign29180_e28318_d_n4, assign29180_e28318_d_n5, assign29180_e28318_d_n6, assign29180_e28318_d_n7, assign29180_e28318_d_n8, assign29180_e28318_d_n9, assign29180_e28318_d_n10, assign29180_e28318_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign29180_e28318;
        locals.var_xmp_dn0 = assign29180_e28318_d_n0;
        locals.var_xmp_dn2 = assign29180_e28318_d_n2;
        locals.var_xmp_dn4 = assign29180_e28318_d_n4;
        locals.var_xmp_dn5 = assign29180_e28318_d_n5;
        locals.var_xmp_dn6 = assign29180_e28318_d_n6;
        locals.var_xmp_dn7 = assign29180_e28318_d_n7;
        locals.var_xmp_dn8 = assign29180_e28318_d_n8;
        locals.var_xmp_dn9 = assign29180_e28318_d_n9;
        locals.var_xmp_dn10 = assign29180_e28318_d_n10;
        locals.var_xmp_dn13 = assign29180_e28318_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign29190_e28326,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29190_e28326;
        locals.var_m0_rv = 0.0;

        let (assign29200_e28334,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29200_e28334;
        locals.var_mm_rv = 0.0;

        let (assign29210_e28342, assign29210_e28342_d_n0, assign29210_e28342_d_n2, assign29210_e28342_d_n4, assign29210_e28342_d_n5, assign29210_e28342_d_n6, assign29210_e28342_d_n7, assign29210_e28342_d_n8, assign29210_e28342_d_n9, assign29210_e28342_d_n10, assign29210_e28342_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign29210_e28342;
        locals.var_arg_dn0 = assign29210_e28342_d_n0;
        locals.var_arg_dn2 = assign29210_e28342_d_n2;
        locals.var_arg_dn4 = assign29210_e28342_d_n4;
        locals.var_arg_dn5 = assign29210_e28342_d_n5;
        locals.var_arg_dn6 = assign29210_e28342_d_n6;
        locals.var_arg_dn7 = assign29210_e28342_d_n7;
        locals.var_arg_dn8 = assign29210_e28342_d_n8;
        locals.var_arg_dn9 = assign29210_e28342_d_n9;
        locals.var_arg_dn10 = assign29210_e28342_d_n10;
        locals.var_arg_dn13 = assign29210_e28342_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign29220_e28350, assign29220_e28350_d_n0, assign29220_e28350_d_n2, assign29220_e28350_d_n4, assign29220_e28350_d_n5, assign29220_e28350_d_n6, assign29220_e28350_d_n7, assign29220_e28350_d_n8, assign29220_e28350_d_n9, assign29220_e28350_d_n10, assign29220_e28350_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29220_e28350;
        locals.var_dnm_dn0 = assign29220_e28350_d_n0;
        locals.var_dnm_dn2 = assign29220_e28350_d_n2;
        locals.var_dnm_dn4 = assign29220_e28350_d_n4;
        locals.var_dnm_dn5 = assign29220_e28350_d_n5;
        locals.var_dnm_dn6 = assign29220_e28350_d_n6;
        locals.var_dnm_dn7 = assign29220_e28350_d_n7;
        locals.var_dnm_dn8 = assign29220_e28350_d_n8;
        locals.var_dnm_dn9 = assign29220_e28350_d_n9;
        locals.var_dnm_dn10 = assign29220_e28350_d_n10;
        locals.var_dnm_dn13 = assign29220_e28350_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign29230_e28360, assign29230_e28360_d_n0, assign29230_e28360_d_n2, assign29230_e28360_d_n4, assign29230_e28360_d_n5, assign29230_e28360_d_n6, assign29230_e28360_d_n7, assign29230_e28360_d_n8, assign29230_e28360_d_n9, assign29230_e28360_d_n10, assign29230_e28360_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29230_e28358: f64 = (locals.var_xp * locals.var_x2);
        (assign29230_e28358, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign29230_e28360;
        locals.var_xp_dn0 = assign29230_e28360_d_n0;
        locals.var_xp_dn2 = assign29230_e28360_d_n2;
        locals.var_xp_dn4 = assign29230_e28360_d_n4;
        locals.var_xp_dn5 = assign29230_e28360_d_n5;
        locals.var_xp_dn6 = assign29230_e28360_d_n6;
        locals.var_xp_dn7 = assign29230_e28360_d_n7;
        locals.var_xp_dn8 = assign29230_e28360_d_n8;
        locals.var_xp_dn9 = assign29230_e28360_d_n9;
        locals.var_xp_dn10 = assign29230_e28360_d_n10;
        locals.var_xp_dn13 = assign29230_e28360_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign29240_e28370, assign29240_e28370_d_n0, assign29240_e28370_d_n2, assign29240_e28370_d_n4, assign29240_e28370_d_n5, assign29240_e28370_d_n6, assign29240_e28370_d_n7, assign29240_e28370_d_n8, assign29240_e28370_d_n9, assign29240_e28370_d_n10, assign29240_e28370_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29240_e28368: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29240_e28368, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign29240_e28370;
        locals.var_xmp_dn0 = assign29240_e28370_d_n0;
        locals.var_xmp_dn2 = assign29240_e28370_d_n2;
        locals.var_xmp_dn4 = assign29240_e28370_d_n4;
        locals.var_xmp_dn5 = assign29240_e28370_d_n5;
        locals.var_xmp_dn6 = assign29240_e28370_d_n6;
        locals.var_xmp_dn7 = assign29240_e28370_d_n7;
        locals.var_xmp_dn8 = assign29240_e28370_d_n8;
        locals.var_xmp_dn9 = assign29240_e28370_d_n9;
        locals.var_xmp_dn10 = assign29240_e28370_d_n10;
        locals.var_xmp_dn13 = assign29240_e28370_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign29250_e28380, assign29250_e28380_d_n0, assign29250_e28380_d_n2, assign29250_e28380_d_n4, assign29250_e28380_d_n5, assign29250_e28380_d_n6, assign29250_e28380_d_n7, assign29250_e28380_d_n8, assign29250_e28380_d_n9, assign29250_e28380_d_n10, assign29250_e28380_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29250_e28378: f64 = (locals.var_xp * locals.var_x2);
        (assign29250_e28378, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign29250_e28380;
        locals.var_xp_dn0 = assign29250_e28380_d_n0;
        locals.var_xp_dn2 = assign29250_e28380_d_n2;
        locals.var_xp_dn4 = assign29250_e28380_d_n4;
        locals.var_xp_dn5 = assign29250_e28380_d_n5;
        locals.var_xp_dn6 = assign29250_e28380_d_n6;
        locals.var_xp_dn7 = assign29250_e28380_d_n7;
        locals.var_xp_dn8 = assign29250_e28380_d_n8;
        locals.var_xp_dn9 = assign29250_e28380_d_n9;
        locals.var_xp_dn10 = assign29250_e28380_d_n10;
        locals.var_xp_dn13 = assign29250_e28380_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign29260_e28390, assign29260_e28390_d_n0, assign29260_e28390_d_n2, assign29260_e28390_d_n4, assign29260_e28390_d_n5, assign29260_e28390_d_n6, assign29260_e28390_d_n7, assign29260_e28390_d_n8, assign29260_e28390_d_n9, assign29260_e28390_d_n10, assign29260_e28390_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29260_e28388: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29260_e28388, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign29260_e28390;
        locals.var_xmp_dn0 = assign29260_e28390_d_n0;
        locals.var_xmp_dn2 = assign29260_e28390_d_n2;
        locals.var_xmp_dn4 = assign29260_e28390_d_n4;
        locals.var_xmp_dn5 = assign29260_e28390_d_n5;
        locals.var_xmp_dn6 = assign29260_e28390_d_n6;
        locals.var_xmp_dn7 = assign29260_e28390_d_n7;
        locals.var_xmp_dn8 = assign29260_e28390_d_n8;
        locals.var_xmp_dn9 = assign29260_e28390_d_n9;
        locals.var_xmp_dn10 = assign29260_e28390_d_n10;
        locals.var_xmp_dn13 = assign29260_e28390_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign29270_e28400, assign29270_e28400_d_n0, assign29270_e28400_d_n2, assign29270_e28400_d_n4, assign29270_e28400_d_n5, assign29270_e28400_d_n6, assign29270_e28400_d_n7, assign29270_e28400_d_n8, assign29270_e28400_d_n9, assign29270_e28400_d_n10, assign29270_e28400_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29270_e28398: f64 = (locals.var_xp + locals.var_xmp);
        (assign29270_e28398, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign29270_e28400;
        locals.var_arg_dn0 = assign29270_e28400_d_n0;
        locals.var_arg_dn2 = assign29270_e28400_d_n2;
        locals.var_arg_dn4 = assign29270_e28400_d_n4;
        locals.var_arg_dn5 = assign29270_e28400_d_n5;
        locals.var_arg_dn6 = assign29270_e28400_d_n6;
        locals.var_arg_dn7 = assign29270_e28400_d_n7;
        locals.var_arg_dn8 = assign29270_e28400_d_n8;
        locals.var_arg_dn9 = assign29270_e28400_d_n9;
        locals.var_arg_dn10 = assign29270_e28400_d_n10;
        locals.var_arg_dn13 = assign29270_e28400_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign29280_e28408, assign29280_e28408_d_n0, assign29280_e28408_d_n2, assign29280_e28408_d_n4, assign29280_e28408_d_n5, assign29280_e28408_d_n6, assign29280_e28408_d_n7, assign29280_e28408_d_n8, assign29280_e28408_d_n9, assign29280_e28408_d_n10, assign29280_e28408_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29280_e28408;
        locals.var_dnm_dn0 = assign29280_e28408_d_n0;
        locals.var_dnm_dn2 = assign29280_e28408_d_n2;
        locals.var_dnm_dn4 = assign29280_e28408_d_n4;
        locals.var_dnm_dn5 = assign29280_e28408_d_n5;
        locals.var_dnm_dn6 = assign29280_e28408_d_n6;
        locals.var_dnm_dn7 = assign29280_e28408_d_n7;
        locals.var_dnm_dn8 = assign29280_e28408_d_n8;
        locals.var_dnm_dn9 = assign29280_e28408_d_n9;
        locals.var_dnm_dn10 = assign29280_e28408_d_n10;
        locals.var_dnm_dn13 = assign29280_e28408_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign29290_e28423: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard681 = assign29290_e28423;
        locals.var_guard681_rv = 0.0;

        let assign29300_e28426: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign29300_e28426;
        locals.var_guard682_rv = 0.0;

        let (assign29310_e28438,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29310_e28438;
        locals.var_mm_rv = 0.0;

        let assign29320_e28441: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign29320_e28441;
        locals.var_guard683_rv = 0.0;

        let (assign29330_e28456,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29330_e28456;
        locals.var_mm_rv = 0.0;

        let assign29340_e28459: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign29340_e28459;
        locals.var_guard684_rv = 0.0;

        let (assign29350_e28477,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29350_e28477;
        locals.var_mm_rv = 0.0;

        let assign29360_e28480: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard685 = assign29360_e28480;
        locals.var_guard685_rv = 0.0;

        let (assign29370_e28501,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29370_e28501;
        locals.var_mm_rv = 0.0;

        let (assign29380_e28511,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29380_e28511;
        locals.var_m0_rv = 0.0;

        let mut assign29390_loop_guard: usize = 0;
        while {
            let assign29390_cond_e28522: f64 = if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29390_cond_e28522 != 0.0
        } {
            assign29390_loop_guard += 1;
            assert!(assign29390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29390_body0_e28533, assign29390_body0_e28533_d_n0, assign29390_body0_e28533_d_n2, assign29390_body0_e28533_d_n4, assign29390_body0_e28533_d_n5, assign29390_body0_e28533_d_n6, assign29390_body0_e28533_d_n7, assign29390_body0_e28533_d_n8, assign29390_body0_e28533_d_n9, assign29390_body0_e28533_d_n10, assign29390_body0_e28533_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) {
        let assign29390_body0_e28531: f64 = (locals.var_dnm).sqrt();
        (assign29390_body0_e28531, (locals.var_dnm_dn0 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn2 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn4 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn5 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn6 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn7 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn8 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn9 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn10 / (2.0 * assign29390_body0_e28531)), (locals.var_dnm_dn13 / (2.0 * assign29390_body0_e28531)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign29390_body0_e28533;
            locals.var_dnm_dn0 = assign29390_body0_e28533_d_n0;
            locals.var_dnm_dn2 = assign29390_body0_e28533_d_n2;
            locals.var_dnm_dn4 = assign29390_body0_e28533_d_n4;
            locals.var_dnm_dn5 = assign29390_body0_e28533_d_n5;
            locals.var_dnm_dn6 = assign29390_body0_e28533_d_n6;
            locals.var_dnm_dn7 = assign29390_body0_e28533_d_n7;
            locals.var_dnm_dn8 = assign29390_body0_e28533_d_n8;
            locals.var_dnm_dn9 = assign29390_body0_e28533_d_n9;
            locals.var_dnm_dn10 = assign29390_body0_e28533_d_n10;
            locals.var_dnm_dn13 = assign29390_body0_e28533_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign29390_body1_e28545,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 != 0.0)) {
        let assign29390_body1_e28543: f64 = (locals.var_m0 + 1.0);
        (assign29390_body1_e28543,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29390_body1_e28545;
            locals.var_m0_rv = 0.0;
        }

        let (assign29400_e28567, assign29400_e28567_d_n0, assign29400_e28567_d_n2, assign29400_e28567_d_n4, assign29400_e28567_d_n5, assign29400_e28567_d_n6, assign29400_e28567_d_n7, assign29400_e28567_d_n8, assign29400_e28567_d_n9, assign29400_e28567_d_n10, assign29400_e28567_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) && (locals.var_guard681 == 0.0)) {
        let (assign29400_e28565, assign29400_e28565_d_n0, assign29400_e28565_d_n2, assign29400_e28565_d_n4, assign29400_e28565_d_n5, assign29400_e28565_d_n6, assign29400_e28565_d_n7, assign29400_e28565_d_n8, assign29400_e28565_d_n9, assign29400_e28565_d_n10, assign29400_e28565_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29400_e28562: f64 = (2.0 * 2.0);
                let assign29400_e28563: f64 = (1.0 / assign29400_e28562);
                let assign29400_e28564: f64 = (locals.var_dnm).powf(assign29400_e28563);
                (assign29400_e28564, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29400_e28563) as f64).is_finite() && ((assign29400_e28563) as f64).fract() == 0.0 { if assign29400_e28563 == 0.0 { 0.0 } else { (assign29400_e28563 * ((locals.var_dnm).powf(assign29400_e28563 - 1.0) * locals.var_dnm_dn13)) } } else { (assign29400_e28564 * (assign29400_e28563 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign29400_e28565, assign29400_e28565_d_n0, assign29400_e28565_d_n2, assign29400_e28565_d_n4, assign29400_e28565_d_n5, assign29400_e28565_d_n6, assign29400_e28565_d_n7, assign29400_e28565_d_n8, assign29400_e28565_d_n9, assign29400_e28565_d_n10, assign29400_e28565_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29400_e28567;
        locals.var_dnm_dn0 = assign29400_e28567_d_n0;
        locals.var_dnm_dn2 = assign29400_e28567_d_n2;
        locals.var_dnm_dn4 = assign29400_e28567_d_n4;
        locals.var_dnm_dn5 = assign29400_e28567_d_n5;
        locals.var_dnm_dn6 = assign29400_e28567_d_n6;
        locals.var_dnm_dn7 = assign29400_e28567_d_n7;
        locals.var_dnm_dn8 = assign29400_e28567_d_n8;
        locals.var_dnm_dn9 = assign29400_e28567_d_n9;
        locals.var_dnm_dn10 = assign29400_e28567_d_n10;
        locals.var_dnm_dn13 = assign29400_e28567_d_n13;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        locals: &mut StampLocals,
    ) {
        let (assign29410_e28577, assign29410_e28577_d_n0, assign29410_e28577_d_n2, assign29410_e28577_d_n4, assign29410_e28577_d_n5, assign29410_e28577_d_n6, assign29410_e28577_d_n7, assign29410_e28577_d_n8, assign29410_e28577_d_n9, assign29410_e28577_d_n10, assign29410_e28577_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29410_e28575: f64 = (1.0 / locals.var_dnm);
        (assign29410_e28575, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29410_e28577;
        locals.var_dnm_dn0 = assign29410_e28577_d_n0;
        locals.var_dnm_dn2 = assign29410_e28577_d_n2;
        locals.var_dnm_dn4 = assign29410_e28577_d_n4;
        locals.var_dnm_dn5 = assign29410_e28577_d_n5;
        locals.var_dnm_dn6 = assign29410_e28577_d_n6;
        locals.var_dnm_dn7 = assign29410_e28577_d_n7;
        locals.var_dnm_dn8 = assign29410_e28577_d_n8;
        locals.var_dnm_dn9 = assign29410_e28577_d_n9;
        locals.var_dnm_dn10 = assign29410_e28577_d_n10;
        locals.var_dnm_dn13 = assign29410_e28577_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign29420_e28589, assign29420_e28589_d_n0, assign29420_e28589_d_n2, assign29420_e28589_d_n4, assign29420_e28589_d_n5, assign29420_e28589_d_n6, assign29420_e28589_d_n7, assign29420_e28589_d_n8, assign29420_e28589_d_n9, assign29420_e28589_d_n10, assign29420_e28589_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29420_e28585: f64 = (locals.var_tmf1 * 0.06);
        let assign29420_e28587: f64 = (assign29420_e28585 * locals.var_dnm);
        (assign29420_e28587, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.06) * locals.var_dnm) + (assign29420_e28585 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign29420_e28589;
        locals.var_tmf0_dn0 = assign29420_e28589_d_n0;
        locals.var_tmf0_dn2 = assign29420_e28589_d_n2;
        locals.var_tmf0_dn4 = assign29420_e28589_d_n4;
        locals.var_tmf0_dn5 = assign29420_e28589_d_n5;
        locals.var_tmf0_dn6 = assign29420_e28589_d_n6;
        locals.var_tmf0_dn7 = assign29420_e28589_d_n7;
        locals.var_tmf0_dn8 = assign29420_e28589_d_n8;
        locals.var_tmf0_dn9 = assign29420_e28589_d_n9;
        locals.var_tmf0_dn10 = assign29420_e28589_d_n10;
        locals.var_tmf0_dn13 = assign29420_e28589_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign29430_e28603, assign29430_e28603_d_n0, assign29430_e28603_d_n2, assign29430_e28603_d_n4, assign29430_e28603_d_n5, assign29430_e28603_d_n6, assign29430_e28603_d_n7, assign29430_e28603_d_n8, assign29430_e28603_d_n9, assign29430_e28603_d_n10, assign29430_e28603_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29430_e28597: f64 = (0.06 * locals.var_xmp);
        let assign29430_e28599: f64 = (assign29430_e28597 * locals.var_dnm);
        let assign29430_e28601: f64 = (assign29430_e28599 / locals.var_arg);
        (assign29430_e28601, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn13) * locals.var_dnm) + (assign29430_e28597 * locals.var_dnm_dn13)) * locals.var_arg) - (assign29430_e28599 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29430_e28603;
        locals.var_t0_dn0 = assign29430_e28603_d_n0;
        locals.var_t0_dn2 = assign29430_e28603_d_n2;
        locals.var_t0_dn4 = assign29430_e28603_d_n4;
        locals.var_t0_dn5 = assign29430_e28603_d_n5;
        locals.var_t0_dn6 = assign29430_e28603_d_n6;
        locals.var_t0_dn7 = assign29430_e28603_d_n7;
        locals.var_t0_dn8 = assign29430_e28603_d_n8;
        locals.var_t0_dn9 = assign29430_e28603_d_n9;
        locals.var_t0_dn10 = assign29430_e28603_d_n10;
        locals.var_t0_dn13 = assign29430_e28603_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29440_e28615, assign29440_e28615_d_n0, assign29440_e28615_d_n2, assign29440_e28615_d_n4, assign29440_e28615_d_n5, assign29440_e28615_d_n6, assign29440_e28615_d_n7, assign29440_e28615_d_n8, assign29440_e28615_d_n9, assign29440_e28615_d_n10, assign29440_e28615_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign29440_e28611: f64 = 0.06;
        let assign29440_e28613: f64 = (assign29440_e28611 - locals.var_tmf0);
        (assign29440_e28613, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign29440_e28615;
        locals.var_t2_dn0 = assign29440_e28615_d_n0;
        locals.var_t2_dn2 = assign29440_e28615_d_n2;
        locals.var_t2_dn4 = assign29440_e28615_d_n4;
        locals.var_t2_dn5 = assign29440_e28615_d_n5;
        locals.var_t2_dn6 = assign29440_e28615_d_n6;
        locals.var_t2_dn7 = assign29440_e28615_d_n7;
        locals.var_t2_dn8 = assign29440_e28615_d_n8;
        locals.var_t2_dn9 = assign29440_e28615_d_n9;
        locals.var_t2_dn10 = assign29440_e28615_d_n10;
        locals.var_t2_dn13 = assign29440_e28615_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign29450_e28623, assign29450_e28623_d_n0, assign29450_e28623_d_n2, assign29450_e28623_d_n4, assign29450_e28623_d_n5, assign29450_e28623_d_n6, assign29450_e28623_d_n7, assign29450_e28623_d_n8, assign29450_e28623_d_n9, assign29450_e28623_d_n10, assign29450_e28623_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29450_e28623;
        locals.var_t0_dn0 = assign29450_e28623_d_n0;
        locals.var_t0_dn2 = assign29450_e28623_d_n2;
        locals.var_t0_dn4 = assign29450_e28623_d_n4;
        locals.var_t0_dn5 = assign29450_e28623_d_n5;
        locals.var_t0_dn6 = assign29450_e28623_d_n6;
        locals.var_t0_dn7 = assign29450_e28623_d_n7;
        locals.var_t0_dn8 = assign29450_e28623_d_n8;
        locals.var_t0_dn9 = assign29450_e28623_d_n9;
        locals.var_t0_dn10 = assign29450_e28623_d_n10;
        locals.var_t0_dn13 = assign29450_e28623_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29460_e28634, assign29460_e28634_d_n0, assign29460_e28634_d_n2, assign29460_e28634_d_n4, assign29460_e28634_d_n5, assign29460_e28634_d_n6, assign29460_e28634_d_n7, assign29460_e28634_d_n8, assign29460_e28634_d_n9, assign29460_e28634_d_n10, assign29460_e28634_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 == 0.0)) {
        let assign29460_e28632: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        (assign29460_e28632, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn13 - locals.var_vds_maxb0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign29460_e28634;
        locals.var_t2_dn0 = assign29460_e28634_d_n0;
        locals.var_t2_dn2 = assign29460_e28634_d_n2;
        locals.var_t2_dn4 = assign29460_e28634_d_n4;
        locals.var_t2_dn5 = assign29460_e28634_d_n5;
        locals.var_t2_dn6 = assign29460_e28634_d_n6;
        locals.var_t2_dn7 = assign29460_e28634_d_n7;
        locals.var_t2_dn8 = assign29460_e28634_d_n8;
        locals.var_t2_dn9 = assign29460_e28634_d_n9;
        locals.var_t2_dn10 = assign29460_e28634_d_n10;
        locals.var_t2_dn13 = assign29460_e28634_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign29470_e28643, assign29470_e28643_d_n0, assign29470_e28643_d_n2, assign29470_e28643_d_n4, assign29470_e28643_d_n5, assign29470_e28643_d_n6, assign29470_e28643_d_n7, assign29470_e28643_d_n8, assign29470_e28643_d_n9, assign29470_e28643_d_n10, assign29470_e28643_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard680 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29470_e28643;
        locals.var_t0_dn0 = assign29470_e28643_d_n0;
        locals.var_t0_dn2 = assign29470_e28643_d_n2;
        locals.var_t0_dn4 = assign29470_e28643_d_n4;
        locals.var_t0_dn5 = assign29470_e28643_d_n5;
        locals.var_t0_dn6 = assign29470_e28643_d_n6;
        locals.var_t0_dn7 = assign29470_e28643_d_n7;
        locals.var_t0_dn8 = assign29470_e28643_d_n8;
        locals.var_t0_dn9 = assign29470_e28643_d_n9;
        locals.var_t0_dn10 = assign29470_e28643_d_n10;
        locals.var_t0_dn13 = assign29470_e28643_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29480_e28662, assign29480_e28662_d_n0, assign29480_e28662_d_n2, assign29480_e28662_d_n4, assign29480_e28662_d_n5, assign29480_e28662_d_n6, assign29480_e28662_d_n7, assign29480_e28662_d_n8, assign29480_e28662_d_n9, assign29480_e28662_d_n10, assign29480_e28662_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign29480_e28649: f64 = (locals.var_beta * locals.var_t2);
        let assign29480_e28650: f64 = (assign29480_e28649).exp();
        let assign29480_e28652: f64 = (assign29480_e28650 - 1.0);
        let assign29480_e28655: f64 = (locals.var_beta * locals.var_t2);
        let assign29480_e28656: f64 = (assign29480_e28652 - assign29480_e28655);
        let assign29480_e28659: f64 = (10.0 * 2.220446049250313e-16);
        let assign29480_e28660: f64 = (assign29480_e28656 + assign29480_e28659);
        (assign29480_e28660, ((assign29480_e28650 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign29480_e28650 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign29480_e28650 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign29480_e28650 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign29480_e28650 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign29480_e28650 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign29480_e28650 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign29480_e28650 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign29480_e28650 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign29480_e28650 * ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))) - ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign29480_e28662;
        locals.var_t4_dn0 = assign29480_e28662_d_n0;
        locals.var_t4_dn2 = assign29480_e28662_d_n2;
        locals.var_t4_dn4 = assign29480_e28662_d_n4;
        locals.var_t4_dn5 = assign29480_e28662_d_n5;
        locals.var_t4_dn6 = assign29480_e28662_d_n6;
        locals.var_t4_dn7 = assign29480_e28662_d_n7;
        locals.var_t4_dn8 = assign29480_e28662_d_n8;
        locals.var_t4_dn9 = assign29480_e28662_d_n9;
        locals.var_t4_dn10 = assign29480_e28662_d_n10;
        locals.var_t4_dn13 = assign29480_e28662_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign29490_e28672, assign29490_e28672_d_n0, assign29490_e28672_d_n2, assign29490_e28672_d_n4, assign29490_e28672_d_n5, assign29490_e28672_d_n6, assign29490_e28672_d_n7, assign29490_e28672_d_n8, assign29490_e28672_d_n9, assign29490_e28672_d_n10, assign29490_e28672_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign29490_e28667: f64 = (-locals.var_cnst0);
        let assign29490_e28669: f64 = (locals.var_t4).sqrt();
        let assign29490_e28670: f64 = (assign29490_e28667 * assign29490_e28669);
        (assign29490_e28670, (((-locals.var_cnst0_dn0) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn0 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn2) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn2 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn4) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn4 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn5) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn5 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn6) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn6 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn7) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn7 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn8) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn8 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn9) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn9 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn10) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn10 / (2.0 * assign29490_e28669)))), (((-locals.var_cnst0_dn13) * assign29490_e28669) + (assign29490_e28667 * (locals.var_t4_dn13 / (2.0 * assign29490_e28669)))),)
    } else {
        (locals.var_q_n0_cur, locals.var_q_n0_cur_dn0, locals.var_q_n0_cur_dn2, locals.var_q_n0_cur_dn4, locals.var_q_n0_cur_dn5, locals.var_q_n0_cur_dn6, locals.var_q_n0_cur_dn7, locals.var_q_n0_cur_dn8, locals.var_q_n0_cur_dn9, locals.var_q_n0_cur_dn10, locals.var_q_n0_cur_dn13,)
    }
};
        locals.var_q_n0_cur = assign29490_e28672;
        locals.var_q_n0_cur_dn0 = assign29490_e28672_d_n0;
        locals.var_q_n0_cur_dn2 = assign29490_e28672_d_n2;
        locals.var_q_n0_cur_dn4 = assign29490_e28672_d_n4;
        locals.var_q_n0_cur_dn5 = assign29490_e28672_d_n5;
        locals.var_q_n0_cur_dn6 = assign29490_e28672_d_n6;
        locals.var_q_n0_cur_dn7 = assign29490_e28672_d_n7;
        locals.var_q_n0_cur_dn8 = assign29490_e28672_d_n8;
        locals.var_q_n0_cur_dn9 = assign29490_e28672_d_n9;
        locals.var_q_n0_cur_dn10 = assign29490_e28672_d_n10;
        locals.var_q_n0_cur_dn13 = assign29490_e28672_d_n13;
        locals.var_q_n0_cur_rv = 0.0;

        let (assign29500_e28687, assign29500_e28687_d_n0, assign29500_e28687_d_n2, assign29500_e28687_d_n4, assign29500_e28687_d_n5, assign29500_e28687_d_n6, assign29500_e28687_d_n7, assign29500_e28687_d_n8, assign29500_e28687_d_n9, assign29500_e28687_d_n10, assign29500_e28687_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign29500_e28678: f64 = (locals.var_beta * 0.1);
        let assign29500_e28679: f64 = (assign29500_e28678).exp();
        let assign29500_e28681: f64 = (assign29500_e28679 - 1.0);
        let assign29500_e28684: f64 = (locals.var_beta * 0.1);
        let assign29500_e28685: f64 = (assign29500_e28681 - assign29500_e28684);
        (assign29500_e28685, ((assign29500_e28679 * (locals.var_beta_dn0 * 0.1)) - (locals.var_beta_dn0 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn2 * 0.1)) - (locals.var_beta_dn2 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn4 * 0.1)) - (locals.var_beta_dn4 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn5 * 0.1)) - (locals.var_beta_dn5 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn6 * 0.1)) - (locals.var_beta_dn6 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn7 * 0.1)) - (locals.var_beta_dn7 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn8 * 0.1)) - (locals.var_beta_dn8 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn9 * 0.1)) - (locals.var_beta_dn9 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn10 * 0.1)) - (locals.var_beta_dn10 * 0.1)), ((assign29500_e28679 * (locals.var_beta_dn13 * 0.1)) - (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign29500_e28687;
        locals.var_t4_dn0 = assign29500_e28687_d_n0;
        locals.var_t4_dn2 = assign29500_e28687_d_n2;
        locals.var_t4_dn4 = assign29500_e28687_d_n4;
        locals.var_t4_dn5 = assign29500_e28687_d_n5;
        locals.var_t4_dn6 = assign29500_e28687_d_n6;
        locals.var_t4_dn7 = assign29500_e28687_d_n7;
        locals.var_t4_dn8 = assign29500_e28687_d_n8;
        locals.var_t4_dn9 = assign29500_e28687_d_n9;
        locals.var_t4_dn10 = assign29500_e28687_d_n10;
        locals.var_t4_dn13 = assign29500_e28687_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign29510_e28696, assign29510_e28696_d_n0, assign29510_e28696_d_n2, assign29510_e28696_d_n4, assign29510_e28696_d_n5, assign29510_e28696_d_n6, assign29510_e28696_d_n7, assign29510_e28696_d_n8, assign29510_e28696_d_n9, assign29510_e28696_d_n10, assign29510_e28696_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign29510_e28693: f64 = (locals.var_t4).sqrt();
        let assign29510_e28694: f64 = (locals.var_cnst0 * assign29510_e28693);
        (assign29510_e28694, ((locals.var_cnst0_dn0 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn0 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn2 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn2 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn4 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn4 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn5 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn5 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn6 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn6 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn7 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn7 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn8 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn8 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn9 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn9 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn10 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn10 / (2.0 * assign29510_e28693)))), ((locals.var_cnst0_dn13 * assign29510_e28693) + (locals.var_cnst0 * (locals.var_t4_dn13 / (2.0 * assign29510_e28693)))),)
    } else {
        (locals.var_qn_delta, locals.var_qn_delta_dn0, locals.var_qn_delta_dn2, locals.var_qn_delta_dn4, locals.var_qn_delta_dn5, locals.var_qn_delta_dn6, locals.var_qn_delta_dn7, locals.var_qn_delta_dn8, locals.var_qn_delta_dn9, locals.var_qn_delta_dn10, locals.var_qn_delta_dn13,)
    }
};
        locals.var_qn_delta = assign29510_e28696;
        locals.var_qn_delta_dn0 = assign29510_e28696_d_n0;
        locals.var_qn_delta_dn2 = assign29510_e28696_d_n2;
        locals.var_qn_delta_dn4 = assign29510_e28696_d_n4;
        locals.var_qn_delta_dn5 = assign29510_e28696_d_n5;
        locals.var_qn_delta_dn6 = assign29510_e28696_d_n6;
        locals.var_qn_delta_dn7 = assign29510_e28696_d_n7;
        locals.var_qn_delta_dn8 = assign29510_e28696_d_n8;
        locals.var_qn_delta_dn9 = assign29510_e28696_d_n9;
        locals.var_qn_delta_dn10 = assign29510_e28696_d_n10;
        locals.var_qn_delta_dn13 = assign29510_e28696_d_n13;
        locals.var_qn_delta_rv = 0.0;

        let (assign29520_e28702, assign29520_e28702_d_n0, assign29520_e28702_d_n2, assign29520_e28702_d_n4, assign29520_e28702_d_n5, assign29520_e28702_d_n6, assign29520_e28702_d_n7, assign29520_e28702_d_n8, assign29520_e28702_d_n9, assign29520_e28702_d_n10, assign29520_e28702_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    }
};
        locals.var_vdsorg = assign29520_e28702;
        locals.var_vdsorg_dn0 = assign29520_e28702_d_n0;
        locals.var_vdsorg_dn2 = assign29520_e28702_d_n2;
        locals.var_vdsorg_dn4 = assign29520_e28702_d_n4;
        locals.var_vdsorg_dn5 = assign29520_e28702_d_n5;
        locals.var_vdsorg_dn6 = assign29520_e28702_d_n6;
        locals.var_vdsorg_dn7 = assign29520_e28702_d_n7;
        locals.var_vdsorg_dn8 = assign29520_e28702_d_n8;
        locals.var_vdsorg_dn9 = assign29520_e28702_d_n9;
        locals.var_vdsorg_dn10 = assign29520_e28702_d_n10;
        locals.var_vdsorg_dn13 = assign29520_e28702_d_n13;
        locals.var_vdsorg_rv = 0.0;

        let assign29530_e28705: f64 = if locals.var_vds > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign29530_e28705;
        locals.var_guard686_rv = 0.0;

        let (assign29540_e28717, assign29540_e28717_d_n0, assign29540_e28717_d_n2, assign29540_e28717_d_n4, assign29540_e28717_d_n5, assign29540_e28717_d_n6, assign29540_e28717_d_n7, assign29540_e28717_d_n8, assign29540_e28717_d_n9, assign29540_e28717_d_n10, assign29540_e28717_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29540_e28714: f64 = (locals.var_cox * locals.var_cox);
        let assign29540_e28715: f64 = (locals.var_q_ndepm_esi / assign29540_e28714);
        (assign29540_e28715, (((locals.var_q_ndepm_esi_dn0 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn2 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn4 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn5 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn6 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn7 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn8 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn9 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn10 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign29540_e28714 * assign29540_e28714)), (((locals.var_q_ndepm_esi_dn13 * assign29540_e28714) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn13 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn13)))) / (assign29540_e28714 * assign29540_e28714)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign29540_e28717;
        locals.var_t2_dn0 = assign29540_e28717_d_n0;
        locals.var_t2_dn2 = assign29540_e28717_d_n2;
        locals.var_t2_dn4 = assign29540_e28717_d_n4;
        locals.var_t2_dn5 = assign29540_e28717_d_n5;
        locals.var_t2_dn6 = assign29540_e28717_d_n6;
        locals.var_t2_dn7 = assign29540_e28717_d_n7;
        locals.var_t2_dn8 = assign29540_e28717_d_n8;
        locals.var_t2_dn9 = assign29540_e28717_d_n9;
        locals.var_t2_dn10 = assign29540_e28717_d_n10;
        locals.var_t2_dn13 = assign29540_e28717_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign29550_e28731, assign29550_e28731_d_n0, assign29550_e28731_d_n2, assign29550_e28731_d_n4, assign29550_e28731_d_n5, assign29550_e28731_d_n6, assign29550_e28731_d_n7, assign29550_e28731_d_n8, assign29550_e28731_d_n9, assign29550_e28731_d_n10, assign29550_e28731_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29550_e28725: f64 = (locals.var_vgp + 2.0);
        let assign29550_e28727: f64 = (assign29550_e28725 - locals.var_beta_inv);
        let assign29550_e28729: f64 = (assign29550_e28727 - locals.var_vbsz__blk438);
        (assign29550_e28729, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk438_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk438_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk438_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk438_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk438_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk438_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk438_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk438_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk438_dn10), ((locals.var_vgp_dn13 - locals.var_beta_inv_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29550_e28731;
        locals.var_t0_dn0 = assign29550_e28731_d_n0;
        locals.var_t0_dn2 = assign29550_e28731_d_n2;
        locals.var_t0_dn4 = assign29550_e28731_d_n4;
        locals.var_t0_dn5 = assign29550_e28731_d_n5;
        locals.var_t0_dn6 = assign29550_e28731_d_n6;
        locals.var_t0_dn7 = assign29550_e28731_d_n7;
        locals.var_t0_dn8 = assign29550_e28731_d_n8;
        locals.var_t0_dn9 = assign29550_e28731_d_n9;
        locals.var_t0_dn10 = assign29550_e28731_d_n10;
        locals.var_t0_dn13 = assign29550_e28731_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29560_e28745, assign29560_e28745_d_n0, assign29560_e28745_d_n2, assign29560_e28745_d_n4, assign29560_e28745_d_n5, assign29560_e28745_d_n6, assign29560_e28745_d_n7, assign29560_e28745_d_n8, assign29560_e28745_d_n9, assign29560_e28745_d_n10, assign29560_e28745_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29560_e28740: f64 = (2.0 / locals.var_t2);
        let assign29560_e28742: f64 = (assign29560_e28740 * locals.var_t0);
        let assign29560_e28743: f64 = (1.0 + assign29560_e28742);
        (assign29560_e28743, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29560_e28740 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign29560_e28745;
        locals.var_t4_dn0 = assign29560_e28745_d_n0;
        locals.var_t4_dn2 = assign29560_e28745_d_n2;
        locals.var_t4_dn4 = assign29560_e28745_d_n4;
        locals.var_t4_dn5 = assign29560_e28745_d_n5;
        locals.var_t4_dn6 = assign29560_e28745_d_n6;
        locals.var_t4_dn7 = assign29560_e28745_d_n7;
        locals.var_t4_dn8 = assign29560_e28745_d_n8;
        locals.var_t4_dn9 = assign29560_e28745_d_n9;
        locals.var_t4_dn10 = assign29560_e28745_d_n10;
        locals.var_t4_dn13 = assign29560_e28745_d_n13;
        locals.var_t4_rv = 0.0;

        let assign29570_e28749: f64 = 2.0;
        let assign29570_e28754: f64 = if ((locals.var_t4 < assign29570_e28749) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard687 = assign29570_e28754;
        locals.var_guard687_rv = 0.0;

        let (assign29580_e28768, assign29580_e28768_d_n0, assign29580_e28768_d_n2, assign29580_e28768_d_n4, assign29580_e28768_d_n5, assign29580_e28768_d_n6, assign29580_e28768_d_n7, assign29580_e28768_d_n8, assign29580_e28768_d_n9, assign29580_e28768_d_n10, assign29580_e28768_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29580_e28764: f64 = 2.0;
        let assign29580_e28766: f64 = (assign29580_e28764 - locals.var_t4);
        (assign29580_e28766, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign29580_e28768;
        locals.var_tmf1_dn0 = assign29580_e28768_d_n0;
        locals.var_tmf1_dn2 = assign29580_e28768_d_n2;
        locals.var_tmf1_dn4 = assign29580_e28768_d_n4;
        locals.var_tmf1_dn5 = assign29580_e28768_d_n5;
        locals.var_tmf1_dn6 = assign29580_e28768_d_n6;
        locals.var_tmf1_dn7 = assign29580_e28768_d_n7;
        locals.var_tmf1_dn8 = assign29580_e28768_d_n8;
        locals.var_tmf1_dn9 = assign29580_e28768_d_n9;
        locals.var_tmf1_dn10 = assign29580_e28768_d_n10;
        locals.var_tmf1_dn13 = assign29580_e28768_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign29590_e28780, assign29590_e28780_d_n0, assign29590_e28780_d_n2, assign29590_e28780_d_n4, assign29590_e28780_d_n5, assign29590_e28780_d_n6, assign29590_e28780_d_n7, assign29590_e28780_d_n8, assign29590_e28780_d_n9, assign29590_e28780_d_n10, assign29590_e28780_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29590_e28778: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29590_e28778, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign29590_e28780;
        locals.var_x2_dn0 = assign29590_e28780_d_n0;
        locals.var_x2_dn2 = assign29590_e28780_d_n2;
        locals.var_x2_dn4 = assign29590_e28780_d_n4;
        locals.var_x2_dn5 = assign29590_e28780_d_n5;
        locals.var_x2_dn6 = assign29590_e28780_d_n6;
        locals.var_x2_dn7 = assign29590_e28780_d_n7;
        locals.var_x2_dn8 = assign29590_e28780_d_n8;
        locals.var_x2_dn9 = assign29590_e28780_d_n9;
        locals.var_x2_dn10 = assign29590_e28780_d_n10;
        locals.var_x2_dn13 = assign29590_e28780_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign29600_e28792, assign29600_e28792_d_n0, assign29600_e28792_d_n2, assign29600_e28792_d_n4, assign29600_e28792_d_n5, assign29600_e28792_d_n6, assign29600_e28792_d_n7, assign29600_e28792_d_n8, assign29600_e28792_d_n9, assign29600_e28792_d_n10, assign29600_e28792_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29600_e28790: f64 = (2.0 * 2.0);
        (assign29600_e28790, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign29600_e28792;
        locals.var_xmax2_dn0 = assign29600_e28792_d_n0;
        locals.var_xmax2_dn2 = assign29600_e28792_d_n2;
        locals.var_xmax2_dn4 = assign29600_e28792_d_n4;
        locals.var_xmax2_dn5 = assign29600_e28792_d_n5;
        locals.var_xmax2_dn6 = assign29600_e28792_d_n6;
        locals.var_xmax2_dn7 = assign29600_e28792_d_n7;
        locals.var_xmax2_dn8 = assign29600_e28792_d_n8;
        locals.var_xmax2_dn9 = assign29600_e28792_d_n9;
        locals.var_xmax2_dn10 = assign29600_e28792_d_n10;
        locals.var_xmax2_dn13 = assign29600_e28792_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign29610_e28802, assign29610_e28802_d_n0, assign29610_e28802_d_n2, assign29610_e28802_d_n4, assign29610_e28802_d_n5, assign29610_e28802_d_n6, assign29610_e28802_d_n7, assign29610_e28802_d_n8, assign29610_e28802_d_n9, assign29610_e28802_d_n10, assign29610_e28802_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign29610_e28802;
        locals.var_xp_dn0 = assign29610_e28802_d_n0;
        locals.var_xp_dn2 = assign29610_e28802_d_n2;
        locals.var_xp_dn4 = assign29610_e28802_d_n4;
        locals.var_xp_dn5 = assign29610_e28802_d_n5;
        locals.var_xp_dn6 = assign29610_e28802_d_n6;
        locals.var_xp_dn7 = assign29610_e28802_d_n7;
        locals.var_xp_dn8 = assign29610_e28802_d_n8;
        locals.var_xp_dn9 = assign29610_e28802_d_n9;
        locals.var_xp_dn10 = assign29610_e28802_d_n10;
        locals.var_xp_dn13 = assign29610_e28802_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign29620_e28812, assign29620_e28812_d_n0, assign29620_e28812_d_n2, assign29620_e28812_d_n4, assign29620_e28812_d_n5, assign29620_e28812_d_n6, assign29620_e28812_d_n7, assign29620_e28812_d_n8, assign29620_e28812_d_n9, assign29620_e28812_d_n10, assign29620_e28812_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign29620_e28812;
        locals.var_xmp_dn0 = assign29620_e28812_d_n0;
        locals.var_xmp_dn2 = assign29620_e28812_d_n2;
        locals.var_xmp_dn4 = assign29620_e28812_d_n4;
        locals.var_xmp_dn5 = assign29620_e28812_d_n5;
        locals.var_xmp_dn6 = assign29620_e28812_d_n6;
        locals.var_xmp_dn7 = assign29620_e28812_d_n7;
        locals.var_xmp_dn8 = assign29620_e28812_d_n8;
        locals.var_xmp_dn9 = assign29620_e28812_d_n9;
        locals.var_xmp_dn10 = assign29620_e28812_d_n10;
        locals.var_xmp_dn13 = assign29620_e28812_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign29630_e28822,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29630_e28822;
        locals.var_m0_rv = 0.0;

        let (assign29640_e28832,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29640_e28832;
        locals.var_mm_rv = 0.0;

        let (assign29650_e28842, assign29650_e28842_d_n0, assign29650_e28842_d_n2, assign29650_e28842_d_n4, assign29650_e28842_d_n5, assign29650_e28842_d_n6, assign29650_e28842_d_n7, assign29650_e28842_d_n8, assign29650_e28842_d_n9, assign29650_e28842_d_n10, assign29650_e28842_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign29650_e28842;
        locals.var_arg_dn0 = assign29650_e28842_d_n0;
        locals.var_arg_dn2 = assign29650_e28842_d_n2;
        locals.var_arg_dn4 = assign29650_e28842_d_n4;
        locals.var_arg_dn5 = assign29650_e28842_d_n5;
        locals.var_arg_dn6 = assign29650_e28842_d_n6;
        locals.var_arg_dn7 = assign29650_e28842_d_n7;
        locals.var_arg_dn8 = assign29650_e28842_d_n8;
        locals.var_arg_dn9 = assign29650_e28842_d_n9;
        locals.var_arg_dn10 = assign29650_e28842_d_n10;
        locals.var_arg_dn13 = assign29650_e28842_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign29660_e28852, assign29660_e28852_d_n0, assign29660_e28852_d_n2, assign29660_e28852_d_n4, assign29660_e28852_d_n5, assign29660_e28852_d_n6, assign29660_e28852_d_n7, assign29660_e28852_d_n8, assign29660_e28852_d_n9, assign29660_e28852_d_n10, assign29660_e28852_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29660_e28852;
        locals.var_dnm_dn0 = assign29660_e28852_d_n0;
        locals.var_dnm_dn2 = assign29660_e28852_d_n2;
        locals.var_dnm_dn4 = assign29660_e28852_d_n4;
        locals.var_dnm_dn5 = assign29660_e28852_d_n5;
        locals.var_dnm_dn6 = assign29660_e28852_d_n6;
        locals.var_dnm_dn7 = assign29660_e28852_d_n7;
        locals.var_dnm_dn8 = assign29660_e28852_d_n8;
        locals.var_dnm_dn9 = assign29660_e28852_d_n9;
        locals.var_dnm_dn10 = assign29660_e28852_d_n10;
        locals.var_dnm_dn13 = assign29660_e28852_d_n13;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        locals: &mut StampLocals,
    ) {
        let (assign29670_e28864, assign29670_e28864_d_n0, assign29670_e28864_d_n2, assign29670_e28864_d_n4, assign29670_e28864_d_n5, assign29670_e28864_d_n6, assign29670_e28864_d_n7, assign29670_e28864_d_n8, assign29670_e28864_d_n9, assign29670_e28864_d_n10, assign29670_e28864_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29670_e28862: f64 = (locals.var_xp * locals.var_x2);
        (assign29670_e28862, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign29670_e28864;
        locals.var_xp_dn0 = assign29670_e28864_d_n0;
        locals.var_xp_dn2 = assign29670_e28864_d_n2;
        locals.var_xp_dn4 = assign29670_e28864_d_n4;
        locals.var_xp_dn5 = assign29670_e28864_d_n5;
        locals.var_xp_dn6 = assign29670_e28864_d_n6;
        locals.var_xp_dn7 = assign29670_e28864_d_n7;
        locals.var_xp_dn8 = assign29670_e28864_d_n8;
        locals.var_xp_dn9 = assign29670_e28864_d_n9;
        locals.var_xp_dn10 = assign29670_e28864_d_n10;
        locals.var_xp_dn13 = assign29670_e28864_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign29680_e28876, assign29680_e28876_d_n0, assign29680_e28876_d_n2, assign29680_e28876_d_n4, assign29680_e28876_d_n5, assign29680_e28876_d_n6, assign29680_e28876_d_n7, assign29680_e28876_d_n8, assign29680_e28876_d_n9, assign29680_e28876_d_n10, assign29680_e28876_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29680_e28874: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29680_e28874, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign29680_e28876;
        locals.var_xmp_dn0 = assign29680_e28876_d_n0;
        locals.var_xmp_dn2 = assign29680_e28876_d_n2;
        locals.var_xmp_dn4 = assign29680_e28876_d_n4;
        locals.var_xmp_dn5 = assign29680_e28876_d_n5;
        locals.var_xmp_dn6 = assign29680_e28876_d_n6;
        locals.var_xmp_dn7 = assign29680_e28876_d_n7;
        locals.var_xmp_dn8 = assign29680_e28876_d_n8;
        locals.var_xmp_dn9 = assign29680_e28876_d_n9;
        locals.var_xmp_dn10 = assign29680_e28876_d_n10;
        locals.var_xmp_dn13 = assign29680_e28876_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign29690_e28888, assign29690_e28888_d_n0, assign29690_e28888_d_n2, assign29690_e28888_d_n4, assign29690_e28888_d_n5, assign29690_e28888_d_n6, assign29690_e28888_d_n7, assign29690_e28888_d_n8, assign29690_e28888_d_n9, assign29690_e28888_d_n10, assign29690_e28888_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29690_e28886: f64 = (locals.var_xp * locals.var_x2);
        (assign29690_e28886, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign29690_e28888;
        locals.var_xp_dn0 = assign29690_e28888_d_n0;
        locals.var_xp_dn2 = assign29690_e28888_d_n2;
        locals.var_xp_dn4 = assign29690_e28888_d_n4;
        locals.var_xp_dn5 = assign29690_e28888_d_n5;
        locals.var_xp_dn6 = assign29690_e28888_d_n6;
        locals.var_xp_dn7 = assign29690_e28888_d_n7;
        locals.var_xp_dn8 = assign29690_e28888_d_n8;
        locals.var_xp_dn9 = assign29690_e28888_d_n9;
        locals.var_xp_dn10 = assign29690_e28888_d_n10;
        locals.var_xp_dn13 = assign29690_e28888_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign29700_e28900, assign29700_e28900_d_n0, assign29700_e28900_d_n2, assign29700_e28900_d_n4, assign29700_e28900_d_n5, assign29700_e28900_d_n6, assign29700_e28900_d_n7, assign29700_e28900_d_n8, assign29700_e28900_d_n9, assign29700_e28900_d_n10, assign29700_e28900_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29700_e28898: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29700_e28898, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign29700_e28900;
        locals.var_xmp_dn0 = assign29700_e28900_d_n0;
        locals.var_xmp_dn2 = assign29700_e28900_d_n2;
        locals.var_xmp_dn4 = assign29700_e28900_d_n4;
        locals.var_xmp_dn5 = assign29700_e28900_d_n5;
        locals.var_xmp_dn6 = assign29700_e28900_d_n6;
        locals.var_xmp_dn7 = assign29700_e28900_d_n7;
        locals.var_xmp_dn8 = assign29700_e28900_d_n8;
        locals.var_xmp_dn9 = assign29700_e28900_d_n9;
        locals.var_xmp_dn10 = assign29700_e28900_d_n10;
        locals.var_xmp_dn13 = assign29700_e28900_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign29710_e28912, assign29710_e28912_d_n0, assign29710_e28912_d_n2, assign29710_e28912_d_n4, assign29710_e28912_d_n5, assign29710_e28912_d_n6, assign29710_e28912_d_n7, assign29710_e28912_d_n8, assign29710_e28912_d_n9, assign29710_e28912_d_n10, assign29710_e28912_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29710_e28910: f64 = (locals.var_xp + locals.var_xmp);
        (assign29710_e28910, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign29710_e28912;
        locals.var_arg_dn0 = assign29710_e28912_d_n0;
        locals.var_arg_dn2 = assign29710_e28912_d_n2;
        locals.var_arg_dn4 = assign29710_e28912_d_n4;
        locals.var_arg_dn5 = assign29710_e28912_d_n5;
        locals.var_arg_dn6 = assign29710_e28912_d_n6;
        locals.var_arg_dn7 = assign29710_e28912_d_n7;
        locals.var_arg_dn8 = assign29710_e28912_d_n8;
        locals.var_arg_dn9 = assign29710_e28912_d_n9;
        locals.var_arg_dn10 = assign29710_e28912_d_n10;
        locals.var_arg_dn13 = assign29710_e28912_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign29720_e28922, assign29720_e28922_d_n0, assign29720_e28922_d_n2, assign29720_e28922_d_n4, assign29720_e28922_d_n5, assign29720_e28922_d_n6, assign29720_e28922_d_n7, assign29720_e28922_d_n8, assign29720_e28922_d_n9, assign29720_e28922_d_n10, assign29720_e28922_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29720_e28922;
        locals.var_dnm_dn0 = assign29720_e28922_d_n0;
        locals.var_dnm_dn2 = assign29720_e28922_d_n2;
        locals.var_dnm_dn4 = assign29720_e28922_d_n4;
        locals.var_dnm_dn5 = assign29720_e28922_d_n5;
        locals.var_dnm_dn6 = assign29720_e28922_d_n6;
        locals.var_dnm_dn7 = assign29720_e28922_d_n7;
        locals.var_dnm_dn8 = assign29720_e28922_d_n8;
        locals.var_dnm_dn9 = assign29720_e28922_d_n9;
        locals.var_dnm_dn10 = assign29720_e28922_d_n10;
        locals.var_dnm_dn13 = assign29720_e28922_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign29730_e28937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard688 = assign29730_e28937;
        locals.var_guard688_rv = 0.0;

        let assign29740_e28940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign29740_e28940;
        locals.var_guard689_rv = 0.0;

        let (assign29750_e28954,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29750_e28954;
        locals.var_mm_rv = 0.0;

        let assign29760_e28957: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard690 = assign29760_e28957;
        locals.var_guard690_rv = 0.0;

        let (assign29770_e28974,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29770_e28974;
        locals.var_mm_rv = 0.0;

        let assign29780_e28977: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign29780_e28977;
        locals.var_guard691_rv = 0.0;

        let (assign29790_e28997,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29790_e28997;
        locals.var_mm_rv = 0.0;

        let assign29800_e29000: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign29800_e29000;
        locals.var_guard692_rv = 0.0;

        let (assign29810_e29023,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29810_e29023;
        locals.var_mm_rv = 0.0;

        let (assign29820_e29035,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29820_e29035;
        locals.var_m0_rv = 0.0;

        let mut assign29830_loop_guard: usize = 0;
        while {
            let assign29830_cond_e29048: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29830_cond_e29048 != 0.0
        } {
            assign29830_loop_guard += 1;
            assert!(assign29830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29830_body0_e29061, assign29830_body0_e29061_d_n0, assign29830_body0_e29061_d_n2, assign29830_body0_e29061_d_n4, assign29830_body0_e29061_d_n5, assign29830_body0_e29061_d_n6, assign29830_body0_e29061_d_n7, assign29830_body0_e29061_d_n8, assign29830_body0_e29061_d_n9, assign29830_body0_e29061_d_n10, assign29830_body0_e29061_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29830_body0_e29059: f64 = (locals.var_dnm).sqrt();
        (assign29830_body0_e29059, (locals.var_dnm_dn0 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn2 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn4 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn5 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn6 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn7 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn8 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn9 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn10 / (2.0 * assign29830_body0_e29059)), (locals.var_dnm_dn13 / (2.0 * assign29830_body0_e29059)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign29830_body0_e29061;
            locals.var_dnm_dn0 = assign29830_body0_e29061_d_n0;
            locals.var_dnm_dn2 = assign29830_body0_e29061_d_n2;
            locals.var_dnm_dn4 = assign29830_body0_e29061_d_n4;
            locals.var_dnm_dn5 = assign29830_body0_e29061_d_n5;
            locals.var_dnm_dn6 = assign29830_body0_e29061_d_n6;
            locals.var_dnm_dn7 = assign29830_body0_e29061_d_n7;
            locals.var_dnm_dn8 = assign29830_body0_e29061_d_n8;
            locals.var_dnm_dn9 = assign29830_body0_e29061_d_n9;
            locals.var_dnm_dn10 = assign29830_body0_e29061_d_n10;
            locals.var_dnm_dn13 = assign29830_body0_e29061_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign29830_body1_e29075,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29830_body1_e29073: f64 = (locals.var_m0 + 1.0);
        (assign29830_body1_e29073,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29830_body1_e29075;
            locals.var_m0_rv = 0.0;
        }

        let (assign29840_e29099, assign29840_e29099_d_n0, assign29840_e29099_d_n2, assign29840_e29099_d_n4, assign29840_e29099_d_n5, assign29840_e29099_d_n6, assign29840_e29099_d_n7, assign29840_e29099_d_n8, assign29840_e29099_d_n9, assign29840_e29099_d_n10, assign29840_e29099_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) {
        let (assign29840_e29097, assign29840_e29097_d_n0, assign29840_e29097_d_n2, assign29840_e29097_d_n4, assign29840_e29097_d_n5, assign29840_e29097_d_n6, assign29840_e29097_d_n7, assign29840_e29097_d_n8, assign29840_e29097_d_n9, assign29840_e29097_d_n10, assign29840_e29097_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29840_e29094: f64 = (2.0 * 2.0);
                let assign29840_e29095: f64 = (1.0 / assign29840_e29094);
                let assign29840_e29096: f64 = (locals.var_dnm).powf(assign29840_e29095);
                (assign29840_e29096, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29840_e29095) as f64).is_finite() && ((assign29840_e29095) as f64).fract() == 0.0 { if assign29840_e29095 == 0.0 { 0.0 } else { (assign29840_e29095 * ((locals.var_dnm).powf(assign29840_e29095 - 1.0) * locals.var_dnm_dn13)) } } else { (assign29840_e29096 * (assign29840_e29095 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign29840_e29097, assign29840_e29097_d_n0, assign29840_e29097_d_n2, assign29840_e29097_d_n4, assign29840_e29097_d_n5, assign29840_e29097_d_n6, assign29840_e29097_d_n7, assign29840_e29097_d_n8, assign29840_e29097_d_n9, assign29840_e29097_d_n10, assign29840_e29097_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29840_e29099;
        locals.var_dnm_dn0 = assign29840_e29099_d_n0;
        locals.var_dnm_dn2 = assign29840_e29099_d_n2;
        locals.var_dnm_dn4 = assign29840_e29099_d_n4;
        locals.var_dnm_dn5 = assign29840_e29099_d_n5;
        locals.var_dnm_dn6 = assign29840_e29099_d_n6;
        locals.var_dnm_dn7 = assign29840_e29099_d_n7;
        locals.var_dnm_dn8 = assign29840_e29099_d_n8;
        locals.var_dnm_dn9 = assign29840_e29099_d_n9;
        locals.var_dnm_dn10 = assign29840_e29099_d_n10;
        locals.var_dnm_dn13 = assign29840_e29099_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign29850_e29111, assign29850_e29111_d_n0, assign29850_e29111_d_n2, assign29850_e29111_d_n4, assign29850_e29111_d_n5, assign29850_e29111_d_n6, assign29850_e29111_d_n7, assign29850_e29111_d_n8, assign29850_e29111_d_n9, assign29850_e29111_d_n10, assign29850_e29111_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29850_e29109: f64 = (1.0 / locals.var_dnm);
        (assign29850_e29109, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign29850_e29111;
        locals.var_dnm_dn0 = assign29850_e29111_d_n0;
        locals.var_dnm_dn2 = assign29850_e29111_d_n2;
        locals.var_dnm_dn4 = assign29850_e29111_d_n4;
        locals.var_dnm_dn5 = assign29850_e29111_d_n5;
        locals.var_dnm_dn6 = assign29850_e29111_d_n6;
        locals.var_dnm_dn7 = assign29850_e29111_d_n7;
        locals.var_dnm_dn8 = assign29850_e29111_d_n8;
        locals.var_dnm_dn9 = assign29850_e29111_d_n9;
        locals.var_dnm_dn10 = assign29850_e29111_d_n10;
        locals.var_dnm_dn13 = assign29850_e29111_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign29860_e29125, assign29860_e29125_d_n0, assign29860_e29125_d_n2, assign29860_e29125_d_n4, assign29860_e29125_d_n5, assign29860_e29125_d_n6, assign29860_e29125_d_n7, assign29860_e29125_d_n8, assign29860_e29125_d_n9, assign29860_e29125_d_n10, assign29860_e29125_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29860_e29121: f64 = (locals.var_tmf1 * 2.0);
        let assign29860_e29123: f64 = (assign29860_e29121 * locals.var_dnm);
        (assign29860_e29123, (((locals.var_tmf1_dn0 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 2.0) * locals.var_dnm) + (assign29860_e29121 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign29860_e29125;
        locals.var_tmf0_dn0 = assign29860_e29125_d_n0;
        locals.var_tmf0_dn2 = assign29860_e29125_d_n2;
        locals.var_tmf0_dn4 = assign29860_e29125_d_n4;
        locals.var_tmf0_dn5 = assign29860_e29125_d_n5;
        locals.var_tmf0_dn6 = assign29860_e29125_d_n6;
        locals.var_tmf0_dn7 = assign29860_e29125_d_n7;
        locals.var_tmf0_dn8 = assign29860_e29125_d_n8;
        locals.var_tmf0_dn9 = assign29860_e29125_d_n9;
        locals.var_tmf0_dn10 = assign29860_e29125_d_n10;
        locals.var_tmf0_dn13 = assign29860_e29125_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign29870_e29141, assign29870_e29141_d_n0, assign29870_e29141_d_n2, assign29870_e29141_d_n4, assign29870_e29141_d_n5, assign29870_e29141_d_n6, assign29870_e29141_d_n7, assign29870_e29141_d_n8, assign29870_e29141_d_n9, assign29870_e29141_d_n10, assign29870_e29141_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29870_e29135: f64 = (2.0 * locals.var_xmp);
        let assign29870_e29137: f64 = (assign29870_e29135 * locals.var_dnm);
        let assign29870_e29139: f64 = (assign29870_e29137 / locals.var_arg);
        (assign29870_e29139, ((((((2.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign29870_e29135 * locals.var_dnm_dn13)) * locals.var_arg) - (assign29870_e29137 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29870_e29141;
        locals.var_t0_dn0 = assign29870_e29141_d_n0;
        locals.var_t0_dn2 = assign29870_e29141_d_n2;
        locals.var_t0_dn4 = assign29870_e29141_d_n4;
        locals.var_t0_dn5 = assign29870_e29141_d_n5;
        locals.var_t0_dn6 = assign29870_e29141_d_n6;
        locals.var_t0_dn7 = assign29870_e29141_d_n7;
        locals.var_t0_dn8 = assign29870_e29141_d_n8;
        locals.var_t0_dn9 = assign29870_e29141_d_n9;
        locals.var_t0_dn10 = assign29870_e29141_d_n10;
        locals.var_t0_dn13 = assign29870_e29141_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29880_e29155, assign29880_e29155_d_n0, assign29880_e29155_d_n2, assign29880_e29155_d_n4, assign29880_e29155_d_n5, assign29880_e29155_d_n6, assign29880_e29155_d_n7, assign29880_e29155_d_n8, assign29880_e29155_d_n9, assign29880_e29155_d_n10, assign29880_e29155_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign29880_e29151: f64 = 2.0;
        let assign29880_e29153: f64 = (assign29880_e29151 - locals.var_tmf0);
        (assign29880_e29153, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign29880_e29155;
        locals.var_t9_dn0 = assign29880_e29155_d_n0;
        locals.var_t9_dn2 = assign29880_e29155_d_n2;
        locals.var_t9_dn4 = assign29880_e29155_d_n4;
        locals.var_t9_dn5 = assign29880_e29155_d_n5;
        locals.var_t9_dn6 = assign29880_e29155_d_n6;
        locals.var_t9_dn7 = assign29880_e29155_d_n7;
        locals.var_t9_dn8 = assign29880_e29155_d_n8;
        locals.var_t9_dn9 = assign29880_e29155_d_n9;
        locals.var_t9_dn10 = assign29880_e29155_d_n10;
        locals.var_t9_dn13 = assign29880_e29155_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign29890_e29165, assign29890_e29165_d_n0, assign29890_e29165_d_n2, assign29890_e29165_d_n4, assign29890_e29165_d_n5, assign29890_e29165_d_n6, assign29890_e29165_d_n7, assign29890_e29165_d_n8, assign29890_e29165_d_n9, assign29890_e29165_d_n10, assign29890_e29165_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29890_e29165;
        locals.var_t0_dn0 = assign29890_e29165_d_n0;
        locals.var_t0_dn2 = assign29890_e29165_d_n2;
        locals.var_t0_dn4 = assign29890_e29165_d_n4;
        locals.var_t0_dn5 = assign29890_e29165_d_n5;
        locals.var_t0_dn6 = assign29890_e29165_d_n6;
        locals.var_t0_dn7 = assign29890_e29165_d_n7;
        locals.var_t0_dn8 = assign29890_e29165_d_n8;
        locals.var_t0_dn9 = assign29890_e29165_d_n9;
        locals.var_t0_dn10 = assign29890_e29165_d_n10;
        locals.var_t0_dn13 = assign29890_e29165_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29900_e29176, assign29900_e29176_d_n0, assign29900_e29176_d_n2, assign29900_e29176_d_n4, assign29900_e29176_d_n5, assign29900_e29176_d_n6, assign29900_e29176_d_n7, assign29900_e29176_d_n8, assign29900_e29176_d_n9, assign29900_e29176_d_n10, assign29900_e29176_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign29900_e29176;
        locals.var_t9_dn0 = assign29900_e29176_d_n0;
        locals.var_t9_dn2 = assign29900_e29176_d_n2;
        locals.var_t9_dn4 = assign29900_e29176_d_n4;
        locals.var_t9_dn5 = assign29900_e29176_d_n5;
        locals.var_t9_dn6 = assign29900_e29176_d_n6;
        locals.var_t9_dn7 = assign29900_e29176_d_n7;
        locals.var_t9_dn8 = assign29900_e29176_d_n8;
        locals.var_t9_dn9 = assign29900_e29176_d_n9;
        locals.var_t9_dn10 = assign29900_e29176_d_n10;
        locals.var_t9_dn13 = assign29900_e29176_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign29910_e29187, assign29910_e29187_d_n0, assign29910_e29187_d_n2, assign29910_e29187_d_n4, assign29910_e29187_d_n5, assign29910_e29187_d_n6, assign29910_e29187_d_n7, assign29910_e29187_d_n8, assign29910_e29187_d_n9, assign29910_e29187_d_n10, assign29910_e29187_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign29910_e29187;
        locals.var_t0_dn0 = assign29910_e29187_d_n0;
        locals.var_t0_dn2 = assign29910_e29187_d_n2;
        locals.var_t0_dn4 = assign29910_e29187_d_n4;
        locals.var_t0_dn5 = assign29910_e29187_d_n5;
        locals.var_t0_dn6 = assign29910_e29187_d_n6;
        locals.var_t0_dn7 = assign29910_e29187_d_n7;
        locals.var_t0_dn8 = assign29910_e29187_d_n8;
        locals.var_t0_dn9 = assign29910_e29187_d_n9;
        locals.var_t0_dn10 = assign29910_e29187_d_n10;
        locals.var_t0_dn13 = assign29910_e29187_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign29920_e29197, assign29920_e29197_d_n0, assign29920_e29197_d_n2, assign29920_e29197_d_n4, assign29920_e29197_d_n5, assign29920_e29197_d_n6, assign29920_e29197_d_n7, assign29920_e29197_d_n8, assign29920_e29197_d_n9, assign29920_e29197_d_n10, assign29920_e29197_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29920_e29195: f64 = (locals.var_t9 + 1e-25);
        (assign29920_e29195, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign29920_e29197;
        locals.var_t9_dn0 = assign29920_e29197_d_n0;
        locals.var_t9_dn2 = assign29920_e29197_d_n2;
        locals.var_t9_dn4 = assign29920_e29197_d_n4;
        locals.var_t9_dn5 = assign29920_e29197_d_n5;
        locals.var_t9_dn6 = assign29920_e29197_d_n6;
        locals.var_t9_dn7 = assign29920_e29197_d_n7;
        locals.var_t9_dn8 = assign29920_e29197_d_n8;
        locals.var_t9_dn9 = assign29920_e29197_d_n9;
        locals.var_t9_dn10 = assign29920_e29197_d_n10;
        locals.var_t9_dn13 = assign29920_e29197_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign29930_e29206, assign29930_e29206_d_n0, assign29930_e29206_d_n2, assign29930_e29206_d_n4, assign29930_e29206_d_n5, assign29930_e29206_d_n6, assign29930_e29206_d_n7, assign29930_e29206_d_n8, assign29930_e29206_d_n9, assign29930_e29206_d_n10, assign29930_e29206_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29930_e29204: f64 = (locals.var_t9).sqrt();
        (assign29930_e29204, (locals.var_t9_dn0 / (2.0 * assign29930_e29204)), (locals.var_t9_dn2 / (2.0 * assign29930_e29204)), (locals.var_t9_dn4 / (2.0 * assign29930_e29204)), (locals.var_t9_dn5 / (2.0 * assign29930_e29204)), (locals.var_t9_dn6 / (2.0 * assign29930_e29204)), (locals.var_t9_dn7 / (2.0 * assign29930_e29204)), (locals.var_t9_dn8 / (2.0 * assign29930_e29204)), (locals.var_t9_dn9 / (2.0 * assign29930_e29204)), (locals.var_t9_dn10 / (2.0 * assign29930_e29204)), (locals.var_t9_dn13 / (2.0 * assign29930_e29204)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign29930_e29206;
        locals.var_t3_dn0 = assign29930_e29206_d_n0;
        locals.var_t3_dn2 = assign29930_e29206_d_n2;
        locals.var_t3_dn4 = assign29930_e29206_d_n4;
        locals.var_t3_dn5 = assign29930_e29206_d_n5;
        locals.var_t3_dn6 = assign29930_e29206_d_n6;
        locals.var_t3_dn7 = assign29930_e29206_d_n7;
        locals.var_t3_dn8 = assign29930_e29206_d_n8;
        locals.var_t3_dn9 = assign29930_e29206_d_n9;
        locals.var_t3_dn10 = assign29930_e29206_d_n10;
        locals.var_t3_dn13 = assign29930_e29206_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign29940_e29218, assign29940_e29218_d_n0, assign29940_e29218_d_n2, assign29940_e29218_d_n4, assign29940_e29218_d_n5, assign29940_e29218_d_n6, assign29940_e29218_d_n7, assign29940_e29218_d_n8, assign29940_e29218_d_n9, assign29940_e29218_d_n10, assign29940_e29218_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29940_e29215: f64 = (1.0 - locals.var_t3);
        let assign29940_e29216: f64 = (locals.var_t2 * assign29940_e29215);
        (assign29940_e29216, ((locals.var_t2_dn0 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn0))), ((locals.var_t2_dn2 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn2))), ((locals.var_t2_dn4 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn4))), ((locals.var_t2_dn5 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn5))), ((locals.var_t2_dn6 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn6))), ((locals.var_t2_dn7 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn7))), ((locals.var_t2_dn8 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn8))), ((locals.var_t2_dn9 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn9))), ((locals.var_t2_dn10 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn10))), ((locals.var_t2_dn13 * assign29940_e29215) + (locals.var_t2 * (-locals.var_t3_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign29940_e29218;
        locals.var_t4_dn0 = assign29940_e29218_d_n0;
        locals.var_t4_dn2 = assign29940_e29218_d_n2;
        locals.var_t4_dn4 = assign29940_e29218_d_n4;
        locals.var_t4_dn5 = assign29940_e29218_d_n5;
        locals.var_t4_dn6 = assign29940_e29218_d_n6;
        locals.var_t4_dn7 = assign29940_e29218_d_n7;
        locals.var_t4_dn8 = assign29940_e29218_d_n8;
        locals.var_t4_dn9 = assign29940_e29218_d_n9;
        locals.var_t4_dn10 = assign29940_e29218_d_n10;
        locals.var_t4_dn13 = assign29940_e29218_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign29950_e29230, assign29950_e29230_d_n0, assign29950_e29230_d_n2, assign29950_e29230_d_n4, assign29950_e29230_d_n5, assign29950_e29230_d_n6, assign29950_e29230_d_n7, assign29950_e29230_d_n8, assign29950_e29230_d_n9, assign29950_e29230_d_n10, assign29950_e29230_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign29950_e29226: f64 = (locals.var_vgp + 2.0);
        let assign29950_e29228: f64 = (assign29950_e29226 + locals.var_t4);
        (assign29950_e29228, (locals.var_vgp_dn0 + locals.var_t4_dn0), (locals.var_vgp_dn2 + locals.var_t4_dn2), (locals.var_vgp_dn4 + locals.var_t4_dn4), (locals.var_vgp_dn5 + locals.var_t4_dn5), (locals.var_vgp_dn6 + locals.var_t4_dn6), (locals.var_vgp_dn7 + locals.var_t4_dn7), (locals.var_vgp_dn8 + locals.var_t4_dn8), (locals.var_vgp_dn9 + locals.var_t4_dn9), (locals.var_vgp_dn10 + locals.var_t4_dn10), (locals.var_vgp_dn13 + locals.var_t4_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign29950_e29230;
        locals.var_t10_dn0 = assign29950_e29230_d_n0;
        locals.var_t10_dn2 = assign29950_e29230_d_n2;
        locals.var_t10_dn4 = assign29950_e29230_d_n4;
        locals.var_t10_dn5 = assign29950_e29230_d_n5;
        locals.var_t10_dn6 = assign29950_e29230_d_n6;
        locals.var_t10_dn7 = assign29950_e29230_d_n7;
        locals.var_t10_dn8 = assign29950_e29230_d_n8;
        locals.var_t10_dn9 = assign29950_e29230_d_n9;
        locals.var_t10_dn10 = assign29950_e29230_d_n10;
        locals.var_t10_dn13 = assign29950_e29230_d_n13;
        locals.var_t10_rv = 0.0;

        let assign29960_e29234: f64 = (0.3 + 0.2);
        let assign29960_e29239: f64 = if ((locals.var_t10 < assign29960_e29234) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard693 = assign29960_e29239;
        locals.var_guard693_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        locals: &mut StampLocals,
    ) {
        let (assign29970_e29253, assign29970_e29253_d_n0, assign29970_e29253_d_n2, assign29970_e29253_d_n4, assign29970_e29253_d_n5, assign29970_e29253_d_n6, assign29970_e29253_d_n7, assign29970_e29253_d_n8, assign29970_e29253_d_n9, assign29970_e29253_d_n10, assign29970_e29253_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign29970_e29249: f64 = (0.3 + 0.2);
        let assign29970_e29251: f64 = (assign29970_e29249 - locals.var_t10);
        (assign29970_e29251, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign29970_e29253;
        locals.var_tmf1_dn0 = assign29970_e29253_d_n0;
        locals.var_tmf1_dn2 = assign29970_e29253_d_n2;
        locals.var_tmf1_dn4 = assign29970_e29253_d_n4;
        locals.var_tmf1_dn5 = assign29970_e29253_d_n5;
        locals.var_tmf1_dn6 = assign29970_e29253_d_n6;
        locals.var_tmf1_dn7 = assign29970_e29253_d_n7;
        locals.var_tmf1_dn8 = assign29970_e29253_d_n8;
        locals.var_tmf1_dn9 = assign29970_e29253_d_n9;
        locals.var_tmf1_dn10 = assign29970_e29253_d_n10;
        locals.var_tmf1_dn13 = assign29970_e29253_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign29980_e29265, assign29980_e29265_d_n0, assign29980_e29265_d_n2, assign29980_e29265_d_n4, assign29980_e29265_d_n5, assign29980_e29265_d_n6, assign29980_e29265_d_n7, assign29980_e29265_d_n8, assign29980_e29265_d_n9, assign29980_e29265_d_n10, assign29980_e29265_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign29980_e29263: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29980_e29263, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign29980_e29265;
        locals.var_x2_dn0 = assign29980_e29265_d_n0;
        locals.var_x2_dn2 = assign29980_e29265_d_n2;
        locals.var_x2_dn4 = assign29980_e29265_d_n4;
        locals.var_x2_dn5 = assign29980_e29265_d_n5;
        locals.var_x2_dn6 = assign29980_e29265_d_n6;
        locals.var_x2_dn7 = assign29980_e29265_d_n7;
        locals.var_x2_dn8 = assign29980_e29265_d_n8;
        locals.var_x2_dn9 = assign29980_e29265_d_n9;
        locals.var_x2_dn10 = assign29980_e29265_d_n10;
        locals.var_x2_dn13 = assign29980_e29265_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign29990_e29277, assign29990_e29277_d_n0, assign29990_e29277_d_n2, assign29990_e29277_d_n4, assign29990_e29277_d_n5, assign29990_e29277_d_n6, assign29990_e29277_d_n7, assign29990_e29277_d_n8, assign29990_e29277_d_n9, assign29990_e29277_d_n10, assign29990_e29277_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign29990_e29275: f64 = (0.2 * 0.2);
        (assign29990_e29275, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign29990_e29277;
        locals.var_xmax2_dn0 = assign29990_e29277_d_n0;
        locals.var_xmax2_dn2 = assign29990_e29277_d_n2;
        locals.var_xmax2_dn4 = assign29990_e29277_d_n4;
        locals.var_xmax2_dn5 = assign29990_e29277_d_n5;
        locals.var_xmax2_dn6 = assign29990_e29277_d_n6;
        locals.var_xmax2_dn7 = assign29990_e29277_d_n7;
        locals.var_xmax2_dn8 = assign29990_e29277_d_n8;
        locals.var_xmax2_dn9 = assign29990_e29277_d_n9;
        locals.var_xmax2_dn10 = assign29990_e29277_d_n10;
        locals.var_xmax2_dn13 = assign29990_e29277_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign30000_e29287, assign30000_e29287_d_n0, assign30000_e29287_d_n2, assign30000_e29287_d_n4, assign30000_e29287_d_n5, assign30000_e29287_d_n6, assign30000_e29287_d_n7, assign30000_e29287_d_n8, assign30000_e29287_d_n9, assign30000_e29287_d_n10, assign30000_e29287_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign30000_e29287;
        locals.var_xp_dn0 = assign30000_e29287_d_n0;
        locals.var_xp_dn2 = assign30000_e29287_d_n2;
        locals.var_xp_dn4 = assign30000_e29287_d_n4;
        locals.var_xp_dn5 = assign30000_e29287_d_n5;
        locals.var_xp_dn6 = assign30000_e29287_d_n6;
        locals.var_xp_dn7 = assign30000_e29287_d_n7;
        locals.var_xp_dn8 = assign30000_e29287_d_n8;
        locals.var_xp_dn9 = assign30000_e29287_d_n9;
        locals.var_xp_dn10 = assign30000_e29287_d_n10;
        locals.var_xp_dn13 = assign30000_e29287_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign30010_e29297, assign30010_e29297_d_n0, assign30010_e29297_d_n2, assign30010_e29297_d_n4, assign30010_e29297_d_n5, assign30010_e29297_d_n6, assign30010_e29297_d_n7, assign30010_e29297_d_n8, assign30010_e29297_d_n9, assign30010_e29297_d_n10, assign30010_e29297_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign30010_e29297;
        locals.var_xmp_dn0 = assign30010_e29297_d_n0;
        locals.var_xmp_dn2 = assign30010_e29297_d_n2;
        locals.var_xmp_dn4 = assign30010_e29297_d_n4;
        locals.var_xmp_dn5 = assign30010_e29297_d_n5;
        locals.var_xmp_dn6 = assign30010_e29297_d_n6;
        locals.var_xmp_dn7 = assign30010_e29297_d_n7;
        locals.var_xmp_dn8 = assign30010_e29297_d_n8;
        locals.var_xmp_dn9 = assign30010_e29297_d_n9;
        locals.var_xmp_dn10 = assign30010_e29297_d_n10;
        locals.var_xmp_dn13 = assign30010_e29297_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign30020_e29307,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30020_e29307;
        locals.var_m0_rv = 0.0;

        let (assign30030_e29317,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30030_e29317;
        locals.var_mm_rv = 0.0;

        let (assign30040_e29327, assign30040_e29327_d_n0, assign30040_e29327_d_n2, assign30040_e29327_d_n4, assign30040_e29327_d_n5, assign30040_e29327_d_n6, assign30040_e29327_d_n7, assign30040_e29327_d_n8, assign30040_e29327_d_n9, assign30040_e29327_d_n10, assign30040_e29327_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign30040_e29327;
        locals.var_arg_dn0 = assign30040_e29327_d_n0;
        locals.var_arg_dn2 = assign30040_e29327_d_n2;
        locals.var_arg_dn4 = assign30040_e29327_d_n4;
        locals.var_arg_dn5 = assign30040_e29327_d_n5;
        locals.var_arg_dn6 = assign30040_e29327_d_n6;
        locals.var_arg_dn7 = assign30040_e29327_d_n7;
        locals.var_arg_dn8 = assign30040_e29327_d_n8;
        locals.var_arg_dn9 = assign30040_e29327_d_n9;
        locals.var_arg_dn10 = assign30040_e29327_d_n10;
        locals.var_arg_dn13 = assign30040_e29327_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign30050_e29337, assign30050_e29337_d_n0, assign30050_e29337_d_n2, assign30050_e29337_d_n4, assign30050_e29337_d_n5, assign30050_e29337_d_n6, assign30050_e29337_d_n7, assign30050_e29337_d_n8, assign30050_e29337_d_n9, assign30050_e29337_d_n10, assign30050_e29337_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign30050_e29337;
        locals.var_dnm_dn0 = assign30050_e29337_d_n0;
        locals.var_dnm_dn2 = assign30050_e29337_d_n2;
        locals.var_dnm_dn4 = assign30050_e29337_d_n4;
        locals.var_dnm_dn5 = assign30050_e29337_d_n5;
        locals.var_dnm_dn6 = assign30050_e29337_d_n6;
        locals.var_dnm_dn7 = assign30050_e29337_d_n7;
        locals.var_dnm_dn8 = assign30050_e29337_d_n8;
        locals.var_dnm_dn9 = assign30050_e29337_d_n9;
        locals.var_dnm_dn10 = assign30050_e29337_d_n10;
        locals.var_dnm_dn13 = assign30050_e29337_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign30060_e29349, assign30060_e29349_d_n0, assign30060_e29349_d_n2, assign30060_e29349_d_n4, assign30060_e29349_d_n5, assign30060_e29349_d_n6, assign30060_e29349_d_n7, assign30060_e29349_d_n8, assign30060_e29349_d_n9, assign30060_e29349_d_n10, assign30060_e29349_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30060_e29347: f64 = (locals.var_xp * locals.var_x2);
        (assign30060_e29347, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign30060_e29349;
        locals.var_xp_dn0 = assign30060_e29349_d_n0;
        locals.var_xp_dn2 = assign30060_e29349_d_n2;
        locals.var_xp_dn4 = assign30060_e29349_d_n4;
        locals.var_xp_dn5 = assign30060_e29349_d_n5;
        locals.var_xp_dn6 = assign30060_e29349_d_n6;
        locals.var_xp_dn7 = assign30060_e29349_d_n7;
        locals.var_xp_dn8 = assign30060_e29349_d_n8;
        locals.var_xp_dn9 = assign30060_e29349_d_n9;
        locals.var_xp_dn10 = assign30060_e29349_d_n10;
        locals.var_xp_dn13 = assign30060_e29349_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign30070_e29361, assign30070_e29361_d_n0, assign30070_e29361_d_n2, assign30070_e29361_d_n4, assign30070_e29361_d_n5, assign30070_e29361_d_n6, assign30070_e29361_d_n7, assign30070_e29361_d_n8, assign30070_e29361_d_n9, assign30070_e29361_d_n10, assign30070_e29361_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30070_e29359: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30070_e29359, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign30070_e29361;
        locals.var_xmp_dn0 = assign30070_e29361_d_n0;
        locals.var_xmp_dn2 = assign30070_e29361_d_n2;
        locals.var_xmp_dn4 = assign30070_e29361_d_n4;
        locals.var_xmp_dn5 = assign30070_e29361_d_n5;
        locals.var_xmp_dn6 = assign30070_e29361_d_n6;
        locals.var_xmp_dn7 = assign30070_e29361_d_n7;
        locals.var_xmp_dn8 = assign30070_e29361_d_n8;
        locals.var_xmp_dn9 = assign30070_e29361_d_n9;
        locals.var_xmp_dn10 = assign30070_e29361_d_n10;
        locals.var_xmp_dn13 = assign30070_e29361_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign30080_e29373, assign30080_e29373_d_n0, assign30080_e29373_d_n2, assign30080_e29373_d_n4, assign30080_e29373_d_n5, assign30080_e29373_d_n6, assign30080_e29373_d_n7, assign30080_e29373_d_n8, assign30080_e29373_d_n9, assign30080_e29373_d_n10, assign30080_e29373_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30080_e29371: f64 = (locals.var_xp * locals.var_x2);
        (assign30080_e29371, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign30080_e29373;
        locals.var_xp_dn0 = assign30080_e29373_d_n0;
        locals.var_xp_dn2 = assign30080_e29373_d_n2;
        locals.var_xp_dn4 = assign30080_e29373_d_n4;
        locals.var_xp_dn5 = assign30080_e29373_d_n5;
        locals.var_xp_dn6 = assign30080_e29373_d_n6;
        locals.var_xp_dn7 = assign30080_e29373_d_n7;
        locals.var_xp_dn8 = assign30080_e29373_d_n8;
        locals.var_xp_dn9 = assign30080_e29373_d_n9;
        locals.var_xp_dn10 = assign30080_e29373_d_n10;
        locals.var_xp_dn13 = assign30080_e29373_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign30090_e29385, assign30090_e29385_d_n0, assign30090_e29385_d_n2, assign30090_e29385_d_n4, assign30090_e29385_d_n5, assign30090_e29385_d_n6, assign30090_e29385_d_n7, assign30090_e29385_d_n8, assign30090_e29385_d_n9, assign30090_e29385_d_n10, assign30090_e29385_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30090_e29383: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30090_e29383, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign30090_e29385;
        locals.var_xmp_dn0 = assign30090_e29385_d_n0;
        locals.var_xmp_dn2 = assign30090_e29385_d_n2;
        locals.var_xmp_dn4 = assign30090_e29385_d_n4;
        locals.var_xmp_dn5 = assign30090_e29385_d_n5;
        locals.var_xmp_dn6 = assign30090_e29385_d_n6;
        locals.var_xmp_dn7 = assign30090_e29385_d_n7;
        locals.var_xmp_dn8 = assign30090_e29385_d_n8;
        locals.var_xmp_dn9 = assign30090_e29385_d_n9;
        locals.var_xmp_dn10 = assign30090_e29385_d_n10;
        locals.var_xmp_dn13 = assign30090_e29385_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign30100_e29397, assign30100_e29397_d_n0, assign30100_e29397_d_n2, assign30100_e29397_d_n4, assign30100_e29397_d_n5, assign30100_e29397_d_n6, assign30100_e29397_d_n7, assign30100_e29397_d_n8, assign30100_e29397_d_n9, assign30100_e29397_d_n10, assign30100_e29397_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30100_e29395: f64 = (locals.var_xp * locals.var_x2);
        (assign30100_e29395, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign30100_e29397;
        locals.var_xp_dn0 = assign30100_e29397_d_n0;
        locals.var_xp_dn2 = assign30100_e29397_d_n2;
        locals.var_xp_dn4 = assign30100_e29397_d_n4;
        locals.var_xp_dn5 = assign30100_e29397_d_n5;
        locals.var_xp_dn6 = assign30100_e29397_d_n6;
        locals.var_xp_dn7 = assign30100_e29397_d_n7;
        locals.var_xp_dn8 = assign30100_e29397_d_n8;
        locals.var_xp_dn9 = assign30100_e29397_d_n9;
        locals.var_xp_dn10 = assign30100_e29397_d_n10;
        locals.var_xp_dn13 = assign30100_e29397_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign30110_e29409, assign30110_e29409_d_n0, assign30110_e29409_d_n2, assign30110_e29409_d_n4, assign30110_e29409_d_n5, assign30110_e29409_d_n6, assign30110_e29409_d_n7, assign30110_e29409_d_n8, assign30110_e29409_d_n9, assign30110_e29409_d_n10, assign30110_e29409_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30110_e29407: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30110_e29407, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign30110_e29409;
        locals.var_xmp_dn0 = assign30110_e29409_d_n0;
        locals.var_xmp_dn2 = assign30110_e29409_d_n2;
        locals.var_xmp_dn4 = assign30110_e29409_d_n4;
        locals.var_xmp_dn5 = assign30110_e29409_d_n5;
        locals.var_xmp_dn6 = assign30110_e29409_d_n6;
        locals.var_xmp_dn7 = assign30110_e29409_d_n7;
        locals.var_xmp_dn8 = assign30110_e29409_d_n8;
        locals.var_xmp_dn9 = assign30110_e29409_d_n9;
        locals.var_xmp_dn10 = assign30110_e29409_d_n10;
        locals.var_xmp_dn13 = assign30110_e29409_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign30120_e29421, assign30120_e29421_d_n0, assign30120_e29421_d_n2, assign30120_e29421_d_n4, assign30120_e29421_d_n5, assign30120_e29421_d_n6, assign30120_e29421_d_n7, assign30120_e29421_d_n8, assign30120_e29421_d_n9, assign30120_e29421_d_n10, assign30120_e29421_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30120_e29419: f64 = (locals.var_xp * locals.var_x2);
        (assign30120_e29419, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign30120_e29421;
        locals.var_xp_dn0 = assign30120_e29421_d_n0;
        locals.var_xp_dn2 = assign30120_e29421_d_n2;
        locals.var_xp_dn4 = assign30120_e29421_d_n4;
        locals.var_xp_dn5 = assign30120_e29421_d_n5;
        locals.var_xp_dn6 = assign30120_e29421_d_n6;
        locals.var_xp_dn7 = assign30120_e29421_d_n7;
        locals.var_xp_dn8 = assign30120_e29421_d_n8;
        locals.var_xp_dn9 = assign30120_e29421_d_n9;
        locals.var_xp_dn10 = assign30120_e29421_d_n10;
        locals.var_xp_dn13 = assign30120_e29421_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign30130_e29433, assign30130_e29433_d_n0, assign30130_e29433_d_n2, assign30130_e29433_d_n4, assign30130_e29433_d_n5, assign30130_e29433_d_n6, assign30130_e29433_d_n7, assign30130_e29433_d_n8, assign30130_e29433_d_n9, assign30130_e29433_d_n10, assign30130_e29433_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30130_e29431: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30130_e29431, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign30130_e29433;
        locals.var_xmp_dn0 = assign30130_e29433_d_n0;
        locals.var_xmp_dn2 = assign30130_e29433_d_n2;
        locals.var_xmp_dn4 = assign30130_e29433_d_n4;
        locals.var_xmp_dn5 = assign30130_e29433_d_n5;
        locals.var_xmp_dn6 = assign30130_e29433_d_n6;
        locals.var_xmp_dn7 = assign30130_e29433_d_n7;
        locals.var_xmp_dn8 = assign30130_e29433_d_n8;
        locals.var_xmp_dn9 = assign30130_e29433_d_n9;
        locals.var_xmp_dn10 = assign30130_e29433_d_n10;
        locals.var_xmp_dn13 = assign30130_e29433_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign30140_e29445, assign30140_e29445_d_n0, assign30140_e29445_d_n2, assign30140_e29445_d_n4, assign30140_e29445_d_n5, assign30140_e29445_d_n6, assign30140_e29445_d_n7, assign30140_e29445_d_n8, assign30140_e29445_d_n9, assign30140_e29445_d_n10, assign30140_e29445_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign30140_e29443: f64 = (locals.var_xp + locals.var_xmp);
        (assign30140_e29443, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign30140_e29445;
        locals.var_arg_dn0 = assign30140_e29445_d_n0;
        locals.var_arg_dn2 = assign30140_e29445_d_n2;
        locals.var_arg_dn4 = assign30140_e29445_d_n4;
        locals.var_arg_dn5 = assign30140_e29445_d_n5;
        locals.var_arg_dn6 = assign30140_e29445_d_n6;
        locals.var_arg_dn7 = assign30140_e29445_d_n7;
        locals.var_arg_dn8 = assign30140_e29445_d_n8;
        locals.var_arg_dn9 = assign30140_e29445_d_n9;
        locals.var_arg_dn10 = assign30140_e29445_d_n10;
        locals.var_arg_dn13 = assign30140_e29445_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign30150_e29455, assign30150_e29455_d_n0, assign30150_e29455_d_n2, assign30150_e29455_d_n4, assign30150_e29455_d_n5, assign30150_e29455_d_n6, assign30150_e29455_d_n7, assign30150_e29455_d_n8, assign30150_e29455_d_n9, assign30150_e29455_d_n10, assign30150_e29455_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign30150_e29455;
        locals.var_dnm_dn0 = assign30150_e29455_d_n0;
        locals.var_dnm_dn2 = assign30150_e29455_d_n2;
        locals.var_dnm_dn4 = assign30150_e29455_d_n4;
        locals.var_dnm_dn5 = assign30150_e29455_d_n5;
        locals.var_dnm_dn6 = assign30150_e29455_d_n6;
        locals.var_dnm_dn7 = assign30150_e29455_d_n7;
        locals.var_dnm_dn8 = assign30150_e29455_d_n8;
        locals.var_dnm_dn9 = assign30150_e29455_d_n9;
        locals.var_dnm_dn10 = assign30150_e29455_d_n10;
        locals.var_dnm_dn13 = assign30150_e29455_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign30160_e29470: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard694 = assign30160_e29470;
        locals.var_guard694_rv = 0.0;

        let assign30170_e29473: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard695 = assign30170_e29473;
        locals.var_guard695_rv = 0.0;

        let (assign30180_e29487,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30180_e29487;
        locals.var_mm_rv = 0.0;

        let assign30190_e29490: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard696 = assign30190_e29490;
        locals.var_guard696_rv = 0.0;

        let (assign30200_e29507,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30200_e29507;
        locals.var_mm_rv = 0.0;

        let assign30210_e29510: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard697 = assign30210_e29510;
        locals.var_guard697_rv = 0.0;

        let (assign30220_e29530,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard697 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30220_e29530;
        locals.var_mm_rv = 0.0;

        let assign30230_e29533: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard698 = assign30230_e29533;
        locals.var_guard698_rv = 0.0;

        let (assign30240_e29556,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30240_e29556;
        locals.var_mm_rv = 0.0;

        let (assign30250_e29568,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30250_e29568;
        locals.var_m0_rv = 0.0;

        let mut assign30260_loop_guard: usize = 0;
        while {
            let assign30260_cond_e29581: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign30260_cond_e29581 != 0.0
        } {
            assign30260_loop_guard += 1;
            assert!(assign30260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign30260_body0_e29594, assign30260_body0_e29594_d_n0, assign30260_body0_e29594_d_n2, assign30260_body0_e29594_d_n4, assign30260_body0_e29594_d_n5, assign30260_body0_e29594_d_n6, assign30260_body0_e29594_d_n7, assign30260_body0_e29594_d_n8, assign30260_body0_e29594_d_n9, assign30260_body0_e29594_d_n10, assign30260_body0_e29594_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) {
        let assign30260_body0_e29592: f64 = (locals.var_dnm).sqrt();
        (assign30260_body0_e29592, (locals.var_dnm_dn0 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn2 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn4 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn5 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn6 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn7 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn8 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn9 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn10 / (2.0 * assign30260_body0_e29592)), (locals.var_dnm_dn13 / (2.0 * assign30260_body0_e29592)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign30260_body0_e29594;
            locals.var_dnm_dn0 = assign30260_body0_e29594_d_n0;
            locals.var_dnm_dn2 = assign30260_body0_e29594_d_n2;
            locals.var_dnm_dn4 = assign30260_body0_e29594_d_n4;
            locals.var_dnm_dn5 = assign30260_body0_e29594_d_n5;
            locals.var_dnm_dn6 = assign30260_body0_e29594_d_n6;
            locals.var_dnm_dn7 = assign30260_body0_e29594_d_n7;
            locals.var_dnm_dn8 = assign30260_body0_e29594_d_n8;
            locals.var_dnm_dn9 = assign30260_body0_e29594_d_n9;
            locals.var_dnm_dn10 = assign30260_body0_e29594_d_n10;
            locals.var_dnm_dn13 = assign30260_body0_e29594_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign30260_body1_e29608,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 != 0.0)) {
        let assign30260_body1_e29606: f64 = (locals.var_m0 + 1.0);
        (assign30260_body1_e29606,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign30260_body1_e29608;
            locals.var_m0_rv = 0.0;
        }

        let (assign30270_e29632, assign30270_e29632_d_n0, assign30270_e29632_d_n2, assign30270_e29632_d_n4, assign30270_e29632_d_n5, assign30270_e29632_d_n6, assign30270_e29632_d_n7, assign30270_e29632_d_n8, assign30270_e29632_d_n9, assign30270_e29632_d_n10, assign30270_e29632_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard693 != 0.0)) && (locals.var_guard694 == 0.0)) {
        let (assign30270_e29630, assign30270_e29630_d_n0, assign30270_e29630_d_n2, assign30270_e29630_d_n4, assign30270_e29630_d_n5, assign30270_e29630_d_n6, assign30270_e29630_d_n7, assign30270_e29630_d_n8, assign30270_e29630_d_n9, assign30270_e29630_d_n10, assign30270_e29630_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30270_e29627: f64 = (2.0 * 4.0);
                let assign30270_e29628: f64 = (1.0 / assign30270_e29627);
                let assign30270_e29629: f64 = (locals.var_dnm).powf(assign30270_e29628);
                (assign30270_e29629, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn0)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn2)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn4)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn5)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn6)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn7)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn8)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn9)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn10)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30270_e29628) as f64).is_finite() && ((assign30270_e29628) as f64).fract() == 0.0 { if assign30270_e29628 == 0.0 { 0.0 } else { (assign30270_e29628 * ((locals.var_dnm).powf(assign30270_e29628 - 1.0) * locals.var_dnm_dn13)) } } else { (assign30270_e29629 * (assign30270_e29628 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign30270_e29630, assign30270_e29630_d_n0, assign30270_e29630_d_n2, assign30270_e29630_d_n4, assign30270_e29630_d_n5, assign30270_e29630_d_n6, assign30270_e29630_d_n7, assign30270_e29630_d_n8, assign30270_e29630_d_n9, assign30270_e29630_d_n10, assign30270_e29630_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign30270_e29632;
        locals.var_dnm_dn0 = assign30270_e29632_d_n0;
        locals.var_dnm_dn2 = assign30270_e29632_d_n2;
        locals.var_dnm_dn4 = assign30270_e29632_d_n4;
        locals.var_dnm_dn5 = assign30270_e29632_d_n5;
        locals.var_dnm_dn6 = assign30270_e29632_d_n6;
        locals.var_dnm_dn7 = assign30270_e29632_d_n7;
        locals.var_dnm_dn8 = assign30270_e29632_d_n8;
        locals.var_dnm_dn9 = assign30270_e29632_d_n9;
        locals.var_dnm_dn10 = assign30270_e29632_d_n10;
        locals.var_dnm_dn13 = assign30270_e29632_d_n13;
        locals.var_dnm_rv = 0.0;

    }
}
