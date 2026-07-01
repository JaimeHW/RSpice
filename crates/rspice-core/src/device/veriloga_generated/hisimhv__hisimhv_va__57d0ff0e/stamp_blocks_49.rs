#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_392(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign102110_e154071, assign102110_e154071_d_n0, assign102110_e154071_d_n2, assign102110_e154071_d_n4, assign102110_e154071_d_n5, assign102110_e154071_d_n6, assign102110_e154071_d_n7, assign102110_e154071_d_n8, assign102110_e154071_d_n9, assign102110_e154071_d_n10, assign102110_e154071_d_n11, assign102110_e154071_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102110_e154066: f64 = locals.var_qbsld;
        let assign102110_e154068: f64 = (assign102110_e154066 - locals.var_qgos);
        let assign102110_e154069: f64 = (locals.var_mfactor * assign102110_e154068);
        (assign102110_e154069, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn11 - locals.var_qgos_dn11)), (locals.var_mfactor * (locals.var_qbsld_dn14 - locals.var_qgos_dn14)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn11, locals.var_qsov_dn14,)
    }
};
        locals.var_qsov = assign102110_e154071;
        locals.var_qsov_dn0 = assign102110_e154071_d_n0;
        locals.var_qsov_dn2 = assign102110_e154071_d_n2;
        locals.var_qsov_dn4 = assign102110_e154071_d_n4;
        locals.var_qsov_dn5 = assign102110_e154071_d_n5;
        locals.var_qsov_dn6 = assign102110_e154071_d_n6;
        locals.var_qsov_dn7 = assign102110_e154071_d_n7;
        locals.var_qsov_dn8 = assign102110_e154071_d_n8;
        locals.var_qsov_dn9 = assign102110_e154071_d_n9;
        locals.var_qsov_dn10 = assign102110_e154071_d_n10;
        locals.var_qsov_dn11 = assign102110_e154071_d_n11;
        locals.var_qsov_dn14 = assign102110_e154071_d_n14;
        locals.var_qsov_rv = 0.0;

        let (assign102120_e154084, assign102120_e154084_d_n0, assign102120_e154084_d_n2, assign102120_e154084_d_n4, assign102120_e154084_d_n5, assign102120_e154084_d_n6, assign102120_e154084_d_n7, assign102120_e154084_d_n8, assign102120_e154084_d_n9, assign102120_e154084_d_n10, assign102120_e154084_d_n11, assign102120_e154084_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102120_e154076: f64 = locals.var_qy;
        let assign102120_e154078: f64 = (assign102120_e154076 - locals.var_qovd_add);
        let assign102120_e154080: f64 = (assign102120_e154078 - locals.var_qovs_add);
        let assign102120_e154081: f64 = (locals.var_mfactor * assign102120_e154080);
        let assign102120_e154082: f64 = (locals.var_qge + assign102120_e154081);
        (assign102120_e154082, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((locals.var_qy_dn11 - locals.var_qovd_add_dn11) - locals.var_qovs_add_dn11))), (locals.var_qge_dn14 + (locals.var_mfactor * ((locals.var_qy_dn14 - locals.var_qovd_add_dn14) - locals.var_qovs_add_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign102120_e154084;
        locals.var_qge_dn0 = assign102120_e154084_d_n0;
        locals.var_qge_dn2 = assign102120_e154084_d_n2;
        locals.var_qge_dn4 = assign102120_e154084_d_n4;
        locals.var_qge_dn5 = assign102120_e154084_d_n5;
        locals.var_qge_dn6 = assign102120_e154084_d_n6;
        locals.var_qge_dn7 = assign102120_e154084_d_n7;
        locals.var_qge_dn8 = assign102120_e154084_d_n8;
        locals.var_qge_dn9 = assign102120_e154084_d_n9;
        locals.var_qge_dn10 = assign102120_e154084_d_n10;
        locals.var_qge_dn11 = assign102120_e154084_d_n11;
        locals.var_qge_dn14 = assign102120_e154084_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign102130_e154095, assign102130_e154095_d_n0, assign102130_e154095_d_n2, assign102130_e154095_d_n4, assign102130_e154095_d_n5, assign102130_e154095_d_n6, assign102130_e154095_d_n7, assign102130_e154095_d_n8, assign102130_e154095_d_n9, assign102130_e154095_d_n10, assign102130_e154095_d_n11, assign102130_e154095_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102130_e154089: f64 = (-locals.var_qy);
        let assign102130_e154091: f64 = (assign102130_e154089 + locals.var_qbdld_add);
        let assign102130_e154092: f64 = (locals.var_mfactor * assign102130_e154091);
        let assign102130_e154093: f64 = (locals.var_qde + assign102130_e154092);
        (assign102130_e154093, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((-locals.var_qy_dn11) + locals.var_qbdld_add_dn11))), (locals.var_qde_dn14 + (locals.var_mfactor * ((-locals.var_qy_dn14) + locals.var_qbdld_add_dn14))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign102130_e154095;
        locals.var_qde_dn0 = assign102130_e154095_d_n0;
        locals.var_qde_dn2 = assign102130_e154095_d_n2;
        locals.var_qde_dn4 = assign102130_e154095_d_n4;
        locals.var_qde_dn5 = assign102130_e154095_d_n5;
        locals.var_qde_dn6 = assign102130_e154095_d_n6;
        locals.var_qde_dn7 = assign102130_e154095_d_n7;
        locals.var_qde_dn8 = assign102130_e154095_d_n8;
        locals.var_qde_dn9 = assign102130_e154095_d_n9;
        locals.var_qde_dn10 = assign102130_e154095_d_n10;
        locals.var_qde_dn11 = assign102130_e154095_d_n11;
        locals.var_qde_dn14 = assign102130_e154095_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign102140_e154104, assign102140_e154104_d_n0, assign102140_e154104_d_n2, assign102140_e154104_d_n4, assign102140_e154104_d_n5, assign102140_e154104_d_n6, assign102140_e154104_d_n7, assign102140_e154104_d_n8, assign102140_e154104_d_n9, assign102140_e154104_d_n10, assign102140_e154104_d_n11, assign102140_e154104_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102140_e154100: f64 = locals.var_qbsld_add;
        let assign102140_e154101: f64 = (locals.var_mfactor * assign102140_e154100);
        let assign102140_e154102: f64 = (locals.var_qse + assign102140_e154101);
        (assign102140_e154102, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn11 + (locals.var_mfactor * locals.var_qbsld_add_dn11)), (locals.var_qse_dn14 + (locals.var_mfactor * locals.var_qbsld_add_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign102140_e154104;
        locals.var_qse_dn0 = assign102140_e154104_d_n0;
        locals.var_qse_dn2 = assign102140_e154104_d_n2;
        locals.var_qse_dn4 = assign102140_e154104_d_n4;
        locals.var_qse_dn5 = assign102140_e154104_d_n5;
        locals.var_qse_dn6 = assign102140_e154104_d_n6;
        locals.var_qse_dn7 = assign102140_e154104_d_n7;
        locals.var_qse_dn8 = assign102140_e154104_d_n8;
        locals.var_qse_dn9 = assign102140_e154104_d_n9;
        locals.var_qse_dn10 = assign102140_e154104_d_n10;
        locals.var_qse_dn11 = assign102140_e154104_d_n11;
        locals.var_qse_dn14 = assign102140_e154104_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign102150_e154113, assign102150_e154113_d_n0, assign102150_e154113_d_n2, assign102150_e154113_d_n4, assign102150_e154113_d_n5, assign102150_e154113_d_n6, assign102150_e154113_d_n7, assign102150_e154113_d_n8, assign102150_e154113_d_n9, assign102150_e154113_d_n10, assign102150_e154113_d_n11, assign102150_e154113_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102150_e154108: f64 = (-locals.var_qovdext);
        let assign102150_e154110: f64 = (assign102150_e154108 - locals.var_qovsext);
        let assign102150_e154111: f64 = (locals.var_mfactor * assign102150_e154110);
        (assign102150_e154111, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn11) - locals.var_qovsext_dn11)), (locals.var_mfactor * ((-locals.var_qovdext_dn14) - locals.var_qovsext_dn14)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14,)
    }
};
        locals.var_qgexte = assign102150_e154113;
        locals.var_qgexte_dn0 = assign102150_e154113_d_n0;
        locals.var_qgexte_dn2 = assign102150_e154113_d_n2;
        locals.var_qgexte_dn4 = assign102150_e154113_d_n4;
        locals.var_qgexte_dn5 = assign102150_e154113_d_n5;
        locals.var_qgexte_dn6 = assign102150_e154113_d_n6;
        locals.var_qgexte_dn7 = assign102150_e154113_d_n7;
        locals.var_qgexte_dn8 = assign102150_e154113_d_n8;
        locals.var_qgexte_dn9 = assign102150_e154113_d_n9;
        locals.var_qgexte_dn10 = assign102150_e154113_d_n10;
        locals.var_qgexte_dn11 = assign102150_e154113_d_n11;
        locals.var_qgexte_dn14 = assign102150_e154113_d_n14;
        locals.var_qgexte_rv = 0.0;

        let (assign102160_e154119, assign102160_e154119_d_n0, assign102160_e154119_d_n2, assign102160_e154119_d_n4, assign102160_e154119_d_n5, assign102160_e154119_d_n6, assign102160_e154119_d_n7, assign102160_e154119_d_n8, assign102160_e154119_d_n9, assign102160_e154119_d_n10, assign102160_e154119_d_n11, assign102160_e154119_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102160_e154117: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102160_e154117, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn11), (locals.var_mfactor * locals.var_qbdldext_dn14),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14,)
    }
};
        locals.var_qdexte = assign102160_e154119;
        locals.var_qdexte_dn0 = assign102160_e154119_d_n0;
        locals.var_qdexte_dn2 = assign102160_e154119_d_n2;
        locals.var_qdexte_dn4 = assign102160_e154119_d_n4;
        locals.var_qdexte_dn5 = assign102160_e154119_d_n5;
        locals.var_qdexte_dn6 = assign102160_e154119_d_n6;
        locals.var_qdexte_dn7 = assign102160_e154119_d_n7;
        locals.var_qdexte_dn8 = assign102160_e154119_d_n8;
        locals.var_qdexte_dn9 = assign102160_e154119_d_n9;
        locals.var_qdexte_dn10 = assign102160_e154119_d_n10;
        locals.var_qdexte_dn11 = assign102160_e154119_d_n11;
        locals.var_qdexte_dn14 = assign102160_e154119_d_n14;
        locals.var_qdexte_rv = 0.0;

        let (assign102170_e154125, assign102170_e154125_d_n0, assign102170_e154125_d_n2, assign102170_e154125_d_n4, assign102170_e154125_d_n5, assign102170_e154125_d_n6, assign102170_e154125_d_n7, assign102170_e154125_d_n8, assign102170_e154125_d_n9, assign102170_e154125_d_n10, assign102170_e154125_d_n11, assign102170_e154125_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102170_e154123: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102170_e154123, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn11), (locals.var_mfactor * locals.var_qbsldext_dn14),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn11, locals.var_qsexte_dn14,)
    }
};
        locals.var_qsexte = assign102170_e154125;
        locals.var_qsexte_dn0 = assign102170_e154125_d_n0;
        locals.var_qsexte_dn2 = assign102170_e154125_d_n2;
        locals.var_qsexte_dn4 = assign102170_e154125_d_n4;
        locals.var_qsexte_dn5 = assign102170_e154125_d_n5;
        locals.var_qsexte_dn6 = assign102170_e154125_d_n6;
        locals.var_qsexte_dn7 = assign102170_e154125_d_n7;
        locals.var_qsexte_dn8 = assign102170_e154125_d_n8;
        locals.var_qsexte_dn9 = assign102170_e154125_d_n9;
        locals.var_qsexte_dn10 = assign102170_e154125_d_n10;
        locals.var_qsexte_dn11 = assign102170_e154125_d_n11;
        locals.var_qsexte_dn14 = assign102170_e154125_d_n14;
        locals.var_qsexte_rv = 0.0;

        let (assign102180_e154136, assign102180_e154136_d_n0, assign102180_e154136_d_n2, assign102180_e154136_d_n7,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102180_e154130: f64 = (-locals.var_qfd);
        let assign102180_e154132: f64 = (assign102180_e154130 - locals.var_qgdo);
        let assign102180_e154133: f64 = (locals.var_mfactor * assign102180_e154132);
        let assign102180_e154134: f64 = (locals.var_qdp + assign102180_e154133);
        (assign102180_e154134, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn7 + (locals.var_mfactor * ((-locals.var_qfd_dn7) - locals.var_qgdo_dn7))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7,)
    }
};
        locals.var_qdp = assign102180_e154136;
        locals.var_qdp_dn0 = assign102180_e154136_d_n0;
        locals.var_qdp_dn2 = assign102180_e154136_d_n2;
        locals.var_qdp_dn7 = assign102180_e154136_d_n7;
        locals.var_qdp_rv = 0.0;

        let (assign102190_e154147, assign102190_e154147_d_n2, assign102190_e154147_d_n7,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102190_e154141: f64 = (-locals.var_qfs);
        let assign102190_e154143: f64 = (assign102190_e154141 - locals.var_qgso);
        let assign102190_e154144: f64 = (locals.var_mfactor * assign102190_e154143);
        let assign102190_e154145: f64 = (locals.var_qsp + assign102190_e154144);
        (assign102190_e154145, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn7 + (locals.var_mfactor * ((-locals.var_qfs_dn7) - locals.var_qgso_dn7))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7,)
    }
};
        locals.var_qsp = assign102190_e154147;
        locals.var_qsp_dn2 = assign102190_e154147_d_n2;
        locals.var_qsp_dn7 = assign102190_e154147_d_n7;
        locals.var_qsp_rv = 0.0;

        let assign102200_e154151: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102200_e154152: f64 = (locals.var_mfactor * assign102200_e154151);
        locals.var_isube = assign102200_e154152;
        locals.var_isube_dn0 = (locals.var_mfactor * (locals.var_isub_dn0 + locals.var_isubibpc_dn0));
        locals.var_isube_dn2 = (locals.var_mfactor * (locals.var_isub_dn2 + locals.var_isubibpc_dn2));
        locals.var_isube_dn4 = (locals.var_mfactor * (locals.var_isub_dn4 + locals.var_isubibpc_dn4));
        locals.var_isube_dn5 = (locals.var_mfactor * (locals.var_isub_dn5 + locals.var_isubibpc_dn5));
        locals.var_isube_dn6 = (locals.var_mfactor * (locals.var_isub_dn6 + locals.var_isubibpc_dn6));
        locals.var_isube_dn7 = (locals.var_mfactor * (locals.var_isub_dn7 + locals.var_isubibpc_dn7));
        locals.var_isube_dn8 = (locals.var_mfactor * (locals.var_isub_dn8 + locals.var_isubibpc_dn8));
        locals.var_isube_dn9 = (locals.var_mfactor * (locals.var_isub_dn9 + locals.var_isubibpc_dn9));
        locals.var_isube_dn10 = (locals.var_mfactor * (locals.var_isub_dn10 + locals.var_isubibpc_dn10));
        locals.var_isube_dn11 = (locals.var_mfactor * (locals.var_isub_dn11 + locals.var_isubibpc_dn11));
        locals.var_isube_dn14 = (locals.var_mfactor * (locals.var_isub_dn14 + locals.var_isubibpc_dn14));
        locals.var_isube_rv = 0.0;

        let assign102210_e154155: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102210_e154155;
        locals.var_isublde_dn0 = (locals.var_mfactor * locals.var_isubld_dn0);
        locals.var_isublde_dn2 = (locals.var_mfactor * locals.var_isubld_dn2);
        locals.var_isublde_dn4 = (locals.var_mfactor * locals.var_isubld_dn4);
        locals.var_isublde_dn5 = (locals.var_mfactor * locals.var_isubld_dn5);
        locals.var_isublde_dn6 = (locals.var_mfactor * locals.var_isubld_dn6);
        locals.var_isublde_dn7 = (locals.var_mfactor * locals.var_isubld_dn7);
        locals.var_isublde_dn8 = (locals.var_mfactor * locals.var_isubld_dn8);
        locals.var_isublde_dn9 = (locals.var_mfactor * locals.var_isubld_dn9);
        locals.var_isublde_dn10 = (locals.var_mfactor * locals.var_isubld_dn10);
        locals.var_isublde_dn11 = (locals.var_mfactor * locals.var_isubld_dn11);
        locals.var_isublde_dn14 = (locals.var_mfactor * locals.var_isubld_dn14);
        locals.var_isublde_rv = 0.0;

        let assign102330_e154222: f64 = (4.0 * 1.3806226e-23);
        let assign102330_e154224: f64 = (assign102330_e154222 * locals.var_ttemp);
        let assign102330_e154226: f64 = assign102330_e154224;
        locals.var_whi_noise = assign102330_e154226;
        locals.var_whi_noise_dn0 = (assign102330_e154222 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102330_e154222 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102330_e154222 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102330_e154222 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102330_e154222 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102330_e154222 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102330_e154222 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102330_e154222 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102330_e154222 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn11 = (assign102330_e154222 * locals.var_ttemp_dn11);
        locals.var_whi_noise_dn14 = (assign102330_e154222 * locals.var_ttemp_dn14);
        locals.var_whi_noise_rv = 0.0;

        let assign102350_e154232: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102350_e154232;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn9 = (locals.var_mfactor * locals.var_nthrml_dn9);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn14 = (locals.var_mfactor * locals.var_nthrml_dn14);
        locals.var_noithrml_rv = 0.0;

        let assign102360_e154235: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign102360_e154235;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn14 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign102370_e154238: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102370_e154238;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p87 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn14 = (p.p87 * locals.var_cgdbd_dn14);
        locals.var_cgdbd_rv = 0.0;

        let assign102380_e154241: f64 = locals.var_qge_dn8;
        locals.var_cgsbd = assign102380_e154241;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn14 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign102390_e154244: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102390_e154244;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p87 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn14 = (p.p87 * locals.var_cgsbd_dn14);
        locals.var_cgsbd_rv = 0.0;

        let (assign102400_e154250, assign102400_e154250_d_n0, assign102400_e154250_d_n2, assign102400_e154250_d_n4, assign102400_e154250_d_n5, assign102400_e154250_d_n6, assign102400_e154250_d_n7, assign102400_e154250_d_n8, assign102400_e154250_d_n9, assign102400_e154250_d_n10, assign102400_e154250_d_n11, assign102400_e154250_d_n14,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    }
};
        locals.var_cgsb = assign102400_e154250;
        locals.var_cgsb_dn0 = assign102400_e154250_d_n0;
        locals.var_cgsb_dn2 = assign102400_e154250_d_n2;
        locals.var_cgsb_dn4 = assign102400_e154250_d_n4;
        locals.var_cgsb_dn5 = assign102400_e154250_d_n5;
        locals.var_cgsb_dn6 = assign102400_e154250_d_n6;
        locals.var_cgsb_dn7 = assign102400_e154250_d_n7;
        locals.var_cgsb_dn8 = assign102400_e154250_d_n8;
        locals.var_cgsb_dn9 = assign102400_e154250_d_n9;
        locals.var_cgsb_dn10 = assign102400_e154250_d_n10;
        locals.var_cgsb_dn11 = assign102400_e154250_d_n11;
        locals.var_cgsb_dn14 = assign102400_e154250_d_n14;
        locals.var_cgsb_rv = 0.0;

        locals.var_noiigate = 0.0;
        locals.var_noiigate_dn0 = 0.0;
        locals.var_noiigate_dn2 = 0.0;
        locals.var_noiigate_dn4 = 0.0;
        locals.var_noiigate_dn5 = 0.0;
        locals.var_noiigate_dn6 = 0.0;
        locals.var_noiigate_dn7 = 0.0;
        locals.var_noiigate_dn8 = 0.0;
        locals.var_noiigate_dn9 = 0.0;
        locals.var_noiigate_dn10 = 0.0;
        locals.var_noiigate_dn11 = 0.0;
        locals.var_noiigate_dn14 = 0.0;
        locals.var_noiigate_rv = 0.0;

        let assign102430_e154270: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2338 = assign102430_e154270;
        locals.var_guard2338_rv = 0.0;

        let (assign102440_e154280, assign102440_e154280_d_n0, assign102440_e154280_d_n2, assign102440_e154280_d_n4, assign102440_e154280_d_n5, assign102440_e154280_d_n6, assign102440_e154280_d_n7, assign102440_e154280_d_n8, assign102440_e154280_d_n9, assign102440_e154280_d_n10, assign102440_e154280_d_n11, assign102440_e154280_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102440_e154274: f64 = (1e-6 * locals.var_cox);
        let assign102440_e154276: f64 = (assign102440_e154274 * locals.var_weffcv_nf);
        let assign102440_e154278: f64 = (assign102440_e154276 * locals.var_leff);
        (assign102440_e154278, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn11) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn14) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102440_e154280;
        locals.var_t0_dn0 = assign102440_e154280_d_n0;
        locals.var_t0_dn2 = assign102440_e154280_d_n2;
        locals.var_t0_dn4 = assign102440_e154280_d_n4;
        locals.var_t0_dn5 = assign102440_e154280_d_n5;
        locals.var_t0_dn6 = assign102440_e154280_d_n6;
        locals.var_t0_dn7 = assign102440_e154280_d_n7;
        locals.var_t0_dn8 = assign102440_e154280_d_n8;
        locals.var_t0_dn9 = assign102440_e154280_d_n9;
        locals.var_t0_dn10 = assign102440_e154280_d_n10;
        locals.var_t0_dn11 = assign102440_e154280_d_n11;
        locals.var_t0_dn14 = assign102440_e154280_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign102450_e154286, assign102450_e154286_d_n0, assign102450_e154286_d_n2, assign102450_e154286_d_n4, assign102450_e154286_d_n5, assign102450_e154286_d_n6, assign102450_e154286_d_n7, assign102450_e154286_d_n8, assign102450_e154286_d_n9, assign102450_e154286_d_n10, assign102450_e154286_d_n11, assign102450_e154286_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102450_e154284: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102450_e154284, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign102450_e154286;
        locals.var_t10_dn0 = assign102450_e154286_d_n0;
        locals.var_t10_dn2 = assign102450_e154286_d_n2;
        locals.var_t10_dn4 = assign102450_e154286_d_n4;
        locals.var_t10_dn5 = assign102450_e154286_d_n5;
        locals.var_t10_dn6 = assign102450_e154286_d_n6;
        locals.var_t10_dn7 = assign102450_e154286_d_n7;
        locals.var_t10_dn8 = assign102450_e154286_d_n8;
        locals.var_t10_dn9 = assign102450_e154286_d_n9;
        locals.var_t10_dn10 = assign102450_e154286_d_n10;
        locals.var_t10_dn11 = assign102450_e154286_d_n11;
        locals.var_t10_dn14 = assign102450_e154286_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign102460_e154300, assign102460_e154300_d_n0, assign102460_e154300_d_n2, assign102460_e154300_d_n4, assign102460_e154300_d_n5, assign102460_e154300_d_n6, assign102460_e154300_d_n7, assign102460_e154300_d_n8, assign102460_e154300_d_n9, assign102460_e154300_d_n10, assign102460_e154300_d_n11, assign102460_e154300_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102460_e154290: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102460_e154292: f64 = (assign102460_e154290 * locals.var_beta_inv);
        let assign102460_e154294: f64 = (assign102460_e154292 * locals.var_t10);
        let assign102460_e154296: f64 = (assign102460_e154294 * locals.var_t10);
        let assign102460_e154298: f64 = (assign102460_e154296 / locals.var_gds0_ign);
        (assign102460_e154298, ((((((((assign102460_e154290 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn0)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn2)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn4)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn5)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn6)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn7)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn8)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn9)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn10)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn11) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn11)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn14) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn14)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn14)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn14)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn14,)
    }
};
        locals.var_nign0 = assign102460_e154300;
        locals.var_nign0_dn0 = assign102460_e154300_d_n0;
        locals.var_nign0_dn2 = assign102460_e154300_d_n2;
        locals.var_nign0_dn4 = assign102460_e154300_d_n4;
        locals.var_nign0_dn5 = assign102460_e154300_d_n5;
        locals.var_nign0_dn6 = assign102460_e154300_d_n6;
        locals.var_nign0_dn7 = assign102460_e154300_d_n7;
        locals.var_nign0_dn8 = assign102460_e154300_d_n8;
        locals.var_nign0_dn9 = assign102460_e154300_d_n9;
        locals.var_nign0_dn10 = assign102460_e154300_d_n10;
        locals.var_nign0_dn11 = assign102460_e154300_d_n11;
        locals.var_nign0_dn14 = assign102460_e154300_d_n14;
        locals.var_nign0_rv = 0.0;

        let assign102470_e154304: f64 = (10.0 * 2.220446049250313e-16);
        let assign102470_e154309: f64 = (10.0 * 2.220446049250313e-16);
        let assign102470_e154311: f64 = if ((locals.var_kusai00l > assign102470_e154304) && (locals.var_vds > assign102470_e154309)) { 1.0 } else { 0.0 };
        locals.var_guard2339 = assign102470_e154311;
        locals.var_guard2339_rv = 0.0;

        let (assign102480_e154319, assign102480_e154319_d_n0, assign102480_e154319_d_n2, assign102480_e154319_d_n4, assign102480_e154319_d_n5, assign102480_e154319_d_n6, assign102480_e154319_d_n7, assign102480_e154319_d_n8, assign102480_e154319_d_n9, assign102480_e154319_d_n10, assign102480_e154319_d_n11, assign102480_e154319_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102480_e154317: f64 = (locals.var_muun / locals.var_mu);
        (assign102480_e154317, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn14 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn14)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn14,)
    }
};
        locals.var_mumoda = assign102480_e154319;
        locals.var_mumoda_dn0 = assign102480_e154319_d_n0;
        locals.var_mumoda_dn2 = assign102480_e154319_d_n2;
        locals.var_mumoda_dn4 = assign102480_e154319_d_n4;
        locals.var_mumoda_dn5 = assign102480_e154319_d_n5;
        locals.var_mumoda_dn6 = assign102480_e154319_d_n6;
        locals.var_mumoda_dn7 = assign102480_e154319_d_n7;
        locals.var_mumoda_dn8 = assign102480_e154319_d_n8;
        locals.var_mumoda_dn9 = assign102480_e154319_d_n9;
        locals.var_mumoda_dn10 = assign102480_e154319_d_n10;
        locals.var_mumoda_dn11 = assign102480_e154319_d_n11;
        locals.var_mumoda_dn14 = assign102480_e154319_d_n14;
        locals.var_mumoda_rv = 0.0;

        let (assign102490_e154331, assign102490_e154331_d_n0, assign102490_e154331_d_n2, assign102490_e154331_d_n4, assign102490_e154331_d_n5, assign102490_e154331_d_n6, assign102490_e154331_d_n7, assign102490_e154331_d_n8, assign102490_e154331_d_n9, assign102490_e154331_d_n10, assign102490_e154331_d_n11, assign102490_e154331_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102490_e154325: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102490_e154327: f64 = (assign102490_e154325 - locals.var_mumoda);
        let assign102490_e154329: f64 = (assign102490_e154327 / locals.var_vds);
        (assign102490_e154329, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn14) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn14)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn14,)
    }
};
        locals.var_mumodb = assign102490_e154331;
        locals.var_mumodb_dn0 = assign102490_e154331_d_n0;
        locals.var_mumodb_dn2 = assign102490_e154331_d_n2;
        locals.var_mumodb_dn4 = assign102490_e154331_d_n4;
        locals.var_mumodb_dn5 = assign102490_e154331_d_n5;
        locals.var_mumodb_dn6 = assign102490_e154331_d_n6;
        locals.var_mumodb_dn7 = assign102490_e154331_d_n7;
        locals.var_mumodb_dn8 = assign102490_e154331_d_n8;
        locals.var_mumodb_dn9 = assign102490_e154331_d_n9;
        locals.var_mumodb_dn10 = assign102490_e154331_d_n10;
        locals.var_mumodb_dn11 = assign102490_e154331_d_n11;
        locals.var_mumodb_dn14 = assign102490_e154331_d_n14;
        locals.var_mumodb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_393(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign102500_e154353, assign102500_e154353_d_n0, assign102500_e154353_d_n2, assign102500_e154353_d_n4, assign102500_e154353_d_n5, assign102500_e154353_d_n6, assign102500_e154353_d_n7, assign102500_e154353_d_n8, assign102500_e154353_d_n9, assign102500_e154353_d_n10, assign102500_e154353_d_n11, assign102500_e154353_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102500_e154338: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign102500_e154342: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign102500_e154343: f64 = (locals.var_kusai00 + assign102500_e154342);
        let assign102500_e154345: f64 = (assign102500_e154343 + locals.var_kusail);
        let assign102500_e154346: f64 = (assign102500_e154338 * assign102500_e154345);
        let assign102500_e154349: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign102500_e154350: f64 = (assign102500_e154346 / assign102500_e154349);
        let assign102500_e154351: f64 = (locals.var_mumoda + assign102500_e154350);
        (assign102500_e154351, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn9 + ((((((0.6666666666666667 * locals.var_mumodb_dn9) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn9 + ((locals.var_vgvt_dn9 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn14 + ((((((0.6666666666666667 * locals.var_mumodb_dn14) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn14 + ((locals.var_vgvt_dn14 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14))) / (assign102500_e154349 * assign102500_e154349))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn14,)
    }
};
        locals.var_correct_w1 = assign102500_e154353;
        locals.var_correct_w1_dn0 = assign102500_e154353_d_n0;
        locals.var_correct_w1_dn2 = assign102500_e154353_d_n2;
        locals.var_correct_w1_dn4 = assign102500_e154353_d_n4;
        locals.var_correct_w1_dn5 = assign102500_e154353_d_n5;
        locals.var_correct_w1_dn6 = assign102500_e154353_d_n6;
        locals.var_correct_w1_dn7 = assign102500_e154353_d_n7;
        locals.var_correct_w1_dn8 = assign102500_e154353_d_n8;
        locals.var_correct_w1_dn9 = assign102500_e154353_d_n9;
        locals.var_correct_w1_dn10 = assign102500_e154353_d_n10;
        locals.var_correct_w1_dn11 = assign102500_e154353_d_n11;
        locals.var_correct_w1_dn14 = assign102500_e154353_d_n14;
        locals.var_correct_w1_rv = 0.0;

        let (assign102510_e154362, assign102510_e154362_d_n0, assign102510_e154362_d_n2, assign102510_e154362_d_n4, assign102510_e154362_d_n5, assign102510_e154362_d_n6, assign102510_e154362_d_n7, assign102510_e154362_d_n8, assign102510_e154362_d_n9, assign102510_e154362_d_n10, assign102510_e154362_d_n11, assign102510_e154362_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102510_e154360: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign102510_e154360, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn14,)
    }
};
        locals.var_correct_w1 = assign102510_e154362;
        locals.var_correct_w1_dn0 = assign102510_e154362_d_n0;
        locals.var_correct_w1_dn2 = assign102510_e154362_d_n2;
        locals.var_correct_w1_dn4 = assign102510_e154362_d_n4;
        locals.var_correct_w1_dn5 = assign102510_e154362_d_n5;
        locals.var_correct_w1_dn6 = assign102510_e154362_d_n6;
        locals.var_correct_w1_dn7 = assign102510_e154362_d_n7;
        locals.var_correct_w1_dn8 = assign102510_e154362_d_n8;
        locals.var_correct_w1_dn9 = assign102510_e154362_d_n9;
        locals.var_correct_w1_dn10 = assign102510_e154362_d_n10;
        locals.var_correct_w1_dn11 = assign102510_e154362_d_n11;
        locals.var_correct_w1_dn14 = assign102510_e154362_d_n14;
        locals.var_correct_w1_rv = 0.0;

        let (assign102520_e154372, assign102520_e154372_d_n0, assign102520_e154372_d_n2, assign102520_e154372_d_n4, assign102520_e154372_d_n5, assign102520_e154372_d_n6, assign102520_e154372_d_n7, assign102520_e154372_d_n8, assign102520_e154372_d_n9, assign102520_e154372_d_n10, assign102520_e154372_d_n11, assign102520_e154372_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102520_e154366: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign102520_e154368: f64 = (assign102520_e154366 * locals.var_kusai_ig);
        let assign102520_e154370: f64 = (assign102520_e154368 * locals.var_correct_w1);
        (assign102520_e154370, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn9) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn9)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn9)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn14) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn14)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn14)),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102520_e154372;
        locals.var_noiigate_dn0 = assign102520_e154372_d_n0;
        locals.var_noiigate_dn2 = assign102520_e154372_d_n2;
        locals.var_noiigate_dn4 = assign102520_e154372_d_n4;
        locals.var_noiigate_dn5 = assign102520_e154372_d_n5;
        locals.var_noiigate_dn6 = assign102520_e154372_d_n6;
        locals.var_noiigate_dn7 = assign102520_e154372_d_n7;
        locals.var_noiigate_dn8 = assign102520_e154372_d_n8;
        locals.var_noiigate_dn9 = assign102520_e154372_d_n9;
        locals.var_noiigate_dn10 = assign102520_e154372_d_n10;
        locals.var_noiigate_dn11 = assign102520_e154372_d_n11;
        locals.var_noiigate_dn14 = assign102520_e154372_d_n14;
        locals.var_noiigate_rv = 0.0;

        let (assign102540_e154385, assign102540_e154385_d_n0, assign102540_e154385_d_n2, assign102540_e154385_d_n4, assign102540_e154385_d_n5, assign102540_e154385_d_n6, assign102540_e154385_d_n7, assign102540_e154385_d_n8, assign102540_e154385_d_n9, assign102540_e154385_d_n10, assign102540_e154385_d_n11, assign102540_e154385_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let (assign102540_e154383, assign102540_e154383_d_n0, assign102540_e154383_d_n2, assign102540_e154383_d_n4, assign102540_e154383_d_n5, assign102540_e154383_d_n6, assign102540_e154383_d_n7, assign102540_e154383_d_n8, assign102540_e154383_d_n9, assign102540_e154383_d_n10, assign102540_e154383_d_n11, assign102540_e154383_d_n14,) = {
            if (locals.var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
            }
        };
        (assign102540_e154383, assign102540_e154383_d_n0, assign102540_e154383_d_n2, assign102540_e154383_d_n4, assign102540_e154383_d_n5, assign102540_e154383_d_n6, assign102540_e154383_d_n7, assign102540_e154383_d_n8, assign102540_e154383_d_n9, assign102540_e154383_d_n10, assign102540_e154383_d_n11, assign102540_e154383_d_n14,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102540_e154385;
        locals.var_noiigate_dn0 = assign102540_e154385_d_n0;
        locals.var_noiigate_dn2 = assign102540_e154385_d_n2;
        locals.var_noiigate_dn4 = assign102540_e154385_d_n4;
        locals.var_noiigate_dn5 = assign102540_e154385_d_n5;
        locals.var_noiigate_dn6 = assign102540_e154385_d_n6;
        locals.var_noiigate_dn7 = assign102540_e154385_d_n7;
        locals.var_noiigate_dn8 = assign102540_e154385_d_n8;
        locals.var_noiigate_dn9 = assign102540_e154385_d_n9;
        locals.var_noiigate_dn10 = assign102540_e154385_d_n10;
        locals.var_noiigate_dn11 = assign102540_e154385_d_n11;
        locals.var_noiigate_dn14 = assign102540_e154385_d_n14;
        locals.var_noiigate_rv = 0.0;

        let (assign102550_e154395, assign102550_e154395_d_n0, assign102550_e154395_d_n2, assign102550_e154395_d_n4, assign102550_e154395_d_n5, assign102550_e154395_d_n6, assign102550_e154395_d_n7, assign102550_e154395_d_n8, assign102550_e154395_d_n9, assign102550_e154395_d_n10, assign102550_e154395_d_n11, assign102550_e154395_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102550_e154388: f64 = (-locals.var_t10);
        let (assign102550_e154393, assign102550_e154393_d_n0, assign102550_e154393_d_n2, assign102550_e154393_d_n4, assign102550_e154393_d_n5, assign102550_e154393_d_n6, assign102550_e154393_d_n7, assign102550_e154393_d_n8, assign102550_e154393_d_n9, assign102550_e154393_d_n10, assign102550_e154393_d_n11, assign102550_e154393_d_n14,) = {
            if (assign102550_e154388 > locals.var_t0) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102550_e154393, assign102550_e154393_d_n0, assign102550_e154393_d_n2, assign102550_e154393_d_n4, assign102550_e154393_d_n5, assign102550_e154393_d_n6, assign102550_e154393_d_n7, assign102550_e154393_d_n8, assign102550_e154393_d_n9, assign102550_e154393_d_n10, assign102550_e154393_d_n11, assign102550_e154393_d_n14,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102550_e154395;
        locals.var_noiigate_dn0 = assign102550_e154395_d_n0;
        locals.var_noiigate_dn2 = assign102550_e154395_d_n2;
        locals.var_noiigate_dn4 = assign102550_e154395_d_n4;
        locals.var_noiigate_dn5 = assign102550_e154395_d_n5;
        locals.var_noiigate_dn6 = assign102550_e154395_d_n6;
        locals.var_noiigate_dn7 = assign102550_e154395_d_n7;
        locals.var_noiigate_dn8 = assign102550_e154395_d_n8;
        locals.var_noiigate_dn9 = assign102550_e154395_d_n9;
        locals.var_noiigate_dn10 = assign102550_e154395_d_n10;
        locals.var_noiigate_dn11 = assign102550_e154395_d_n11;
        locals.var_noiigate_dn14 = assign102550_e154395_d_n14;
        locals.var_noiigate_rv = 0.0;

        let assign102570_e154408: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign102570_e154408;
        locals.var_sid_dn0 = ((locals.var_whi_noise_dn0 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn0));
        locals.var_sid_dn2 = ((locals.var_whi_noise_dn2 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn2));
        locals.var_sid_dn4 = ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4));
        locals.var_sid_dn5 = ((locals.var_whi_noise_dn5 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn5));
        locals.var_sid_dn6 = ((locals.var_whi_noise_dn6 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn6));
        locals.var_sid_dn7 = ((locals.var_whi_noise_dn7 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn7));
        locals.var_sid_dn8 = ((locals.var_whi_noise_dn8 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn8));
        locals.var_sid_dn9 = ((locals.var_whi_noise_dn9 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn9));
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = ((locals.var_whi_noise_dn11 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn11));
        locals.var_sid_dn14 = ((locals.var_whi_noise_dn14 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn14));
        locals.var_sid_rv = 0.0;

        let (assign102590_e154422, assign102590_e154422_d_n0, assign102590_e154422_d_n2, assign102590_e154422_d_n4, assign102590_e154422_d_n5, assign102590_e154422_d_n6, assign102590_e154422_d_n7, assign102590_e154422_d_n8, assign102590_e154422_d_n9, assign102590_e154422_d_n10, assign102590_e154422_d_n11, assign102590_e154422_d_n14,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign102590_e154419: f64 = (locals.var_noiigate / locals.var_sid);
        let assign102590_e154420: f64 = (assign102590_e154419).sqrt();
        (assign102590_e154420, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn9 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn9)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn14 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn14)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign102590_e154422;
        locals.var_sigrat_dn0 = assign102590_e154422_d_n0;
        locals.var_sigrat_dn2 = assign102590_e154422_d_n2;
        locals.var_sigrat_dn4 = assign102590_e154422_d_n4;
        locals.var_sigrat_dn5 = assign102590_e154422_d_n5;
        locals.var_sigrat_dn6 = assign102590_e154422_d_n6;
        locals.var_sigrat_dn7 = assign102590_e154422_d_n7;
        locals.var_sigrat_dn8 = assign102590_e154422_d_n8;
        locals.var_sigrat_dn9 = assign102590_e154422_d_n9;
        locals.var_sigrat_dn10 = assign102590_e154422_d_n10;
        locals.var_sigrat_dn11 = assign102590_e154422_d_n11;
        locals.var_sigrat_dn14 = assign102590_e154422_d_n14;
        locals.var_sigrat_rv = 0.0;

        let (assign102600_e154434, assign102600_e154434_d_n0, assign102600_e154434_d_n2, assign102600_e154434_d_n4, assign102600_e154434_d_n5, assign102600_e154434_d_n6, assign102600_e154434_d_n7, assign102600_e154434_d_n8, assign102600_e154434_d_n9, assign102600_e154434_d_n10, assign102600_e154434_d_n11, assign102600_e154434_d_n14,) = {
    if (locals.var_mode > 0.0) {
        let assign102600_e154429: f64 = (1.0 - locals.var_qdrat);
        let assign102600_e154430: f64 = (locals.var_sigrat * assign102600_e154429);
        (assign102600_e154430, ((locals.var_sigrat_dn0 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn14 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn14))),)
    } else {
        let assign102600_e154433: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102600_e154433, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn14 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn14)),)
    }
};
        locals.var_sigrat_s = assign102600_e154434;
        locals.var_sigrat_s_dn0 = assign102600_e154434_d_n0;
        locals.var_sigrat_s_dn2 = assign102600_e154434_d_n2;
        locals.var_sigrat_s_dn4 = assign102600_e154434_d_n4;
        locals.var_sigrat_s_dn5 = assign102600_e154434_d_n5;
        locals.var_sigrat_s_dn6 = assign102600_e154434_d_n6;
        locals.var_sigrat_s_dn7 = assign102600_e154434_d_n7;
        locals.var_sigrat_s_dn8 = assign102600_e154434_d_n8;
        locals.var_sigrat_s_dn9 = assign102600_e154434_d_n9;
        locals.var_sigrat_s_dn10 = assign102600_e154434_d_n10;
        locals.var_sigrat_s_dn11 = assign102600_e154434_d_n11;
        locals.var_sigrat_s_dn14 = assign102600_e154434_d_n14;
        locals.var_sigrat_s_rv = 0.0;

        let (assign102610_e154446, assign102610_e154446_d_n0, assign102610_e154446_d_n2, assign102610_e154446_d_n4, assign102610_e154446_d_n5, assign102610_e154446_d_n6, assign102610_e154446_d_n7, assign102610_e154446_d_n8, assign102610_e154446_d_n9, assign102610_e154446_d_n10, assign102610_e154446_d_n11, assign102610_e154446_d_n14,) = {
    if (locals.var_mode > 0.0) {
        let assign102610_e154440: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102610_e154440, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn14 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn14)),)
    } else {
        let assign102610_e154444: f64 = (1.0 - locals.var_qdrat);
        let assign102610_e154445: f64 = (locals.var_sigrat * assign102610_e154444);
        (assign102610_e154445, ((locals.var_sigrat_dn0 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn14 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn14))),)
    }
};
        locals.var_sigrat_d = assign102610_e154446;
        locals.var_sigrat_d_dn0 = assign102610_e154446_d_n0;
        locals.var_sigrat_d_dn2 = assign102610_e154446_d_n2;
        locals.var_sigrat_d_dn4 = assign102610_e154446_d_n4;
        locals.var_sigrat_d_dn5 = assign102610_e154446_d_n5;
        locals.var_sigrat_d_dn6 = assign102610_e154446_d_n6;
        locals.var_sigrat_d_dn7 = assign102610_e154446_d_n7;
        locals.var_sigrat_d_dn8 = assign102610_e154446_d_n8;
        locals.var_sigrat_d_dn9 = assign102610_e154446_d_n9;
        locals.var_sigrat_d_dn10 = assign102610_e154446_d_n10;
        locals.var_sigrat_d_dn11 = assign102610_e154446_d_n11;
        locals.var_sigrat_d_dn14 = assign102610_e154446_d_n14;
        locals.var_sigrat_d_rv = 0.0;

        locals.var_rsde = 0.0;
        locals.var_rsde_dn0 = 0.0;
        locals.var_rsde_dn2 = 0.0;
        locals.var_rsde_dn4 = 0.0;
        locals.var_rsde_dn5 = 0.0;
        locals.var_rsde_dn6 = 0.0;
        locals.var_rsde_dn7 = 0.0;
        locals.var_rsde_dn8 = 0.0;
        locals.var_rsde_dn9 = 0.0;
        locals.var_rsde_dn10 = 0.0;
        locals.var_rsde_dn11 = 0.0;
        locals.var_rsde_dn14 = 0.0;
        locals.var_rsde_rv = 0.0;

        locals.var_rdde = 0.0;
        locals.var_rdde_dn0 = 0.0;
        locals.var_rdde_dn2 = 0.0;
        locals.var_rdde_dn4 = 0.0;
        locals.var_rdde_dn5 = 0.0;
        locals.var_rdde_dn6 = 0.0;
        locals.var_rdde_dn7 = 0.0;
        locals.var_rdde_dn8 = 0.0;
        locals.var_rdde_dn9 = 0.0;
        locals.var_rdde_dn10 = 0.0;
        locals.var_rdde_dn11 = 0.0;
        locals.var_rdde_dn14 = 0.0;
        locals.var_rdde_rv = 0.0;

        let assign102640_e154451: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2340 = assign102640_e154451;
        locals.var_guard2340_rv = 0.0;

        let assign102650_e154454: f64 = if locals.var_flg_rs == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2341 = assign102650_e154454;
        locals.var_guard2341_rv = 0.0;

        let assign102660_e154461: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2342 = assign102660_e154461;
        locals.var_guard2342_rv = 0.0;

        let (assign102670_e154477, assign102670_e154477_d_n0, assign102670_e154477_d_n2, assign102670_e154477_d_n4, assign102670_e154477_d_n5, assign102670_e154477_d_n6, assign102670_e154477_d_n7, assign102670_e154477_d_n8, assign102670_e154477_d_n9, assign102670_e154477_d_n10, assign102670_e154477_d_n11, assign102670_e154477_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let (assign102670_e154475, assign102670_e154475_d_n0, assign102670_e154475_d_n2, assign102670_e154475_d_n4, assign102670_e154475_d_n5, assign102670_e154475_d_n6, assign102670_e154475_d_n7, assign102670_e154475_d_n8, assign102670_e154475_d_n9, assign102670_e154475_d_n10, assign102670_e154475_d_n11, assign102670_e154475_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102670_e154474: f64 = (locals.var_tratio).powf(p.p416);
                (assign102670_e154474, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn11)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn14)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign102670_e154475, assign102670_e154475_d_n0, assign102670_e154475_d_n2, assign102670_e154475_d_n4, assign102670_e154475_d_n5, assign102670_e154475_d_n6, assign102670_e154475_d_n7, assign102670_e154475_d_n8, assign102670_e154475_d_n9, assign102670_e154475_d_n10, assign102670_e154475_d_n11, assign102670_e154475_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102670_e154477;
        locals.var_t1_dn0 = assign102670_e154477_d_n0;
        locals.var_t1_dn2 = assign102670_e154477_d_n2;
        locals.var_t1_dn4 = assign102670_e154477_d_n4;
        locals.var_t1_dn5 = assign102670_e154477_d_n5;
        locals.var_t1_dn6 = assign102670_e154477_d_n6;
        locals.var_t1_dn7 = assign102670_e154477_d_n7;
        locals.var_t1_dn8 = assign102670_e154477_d_n8;
        locals.var_t1_dn9 = assign102670_e154477_d_n9;
        locals.var_t1_dn10 = assign102670_e154477_d_n10;
        locals.var_t1_dn11 = assign102670_e154477_d_n11;
        locals.var_t1_dn14 = assign102670_e154477_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign102680_e154488, assign102680_e154488_d_n0, assign102680_e154488_d_n2, assign102680_e154488_d_n4, assign102680_e154488_d_n5, assign102680_e154488_d_n6, assign102680_e154488_d_n7, assign102680_e154488_d_n8, assign102680_e154488_d_n9, assign102680_e154488_d_n10, assign102680_e154488_d_n11, assign102680_e154488_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102680_e154486: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign102680_e154486, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn11, locals.var_rrdrmues_dn14,)
    }
};
        locals.var_rrdrmues = assign102680_e154488;
        locals.var_rrdrmues_dn0 = assign102680_e154488_d_n0;
        locals.var_rrdrmues_dn2 = assign102680_e154488_d_n2;
        locals.var_rrdrmues_dn4 = assign102680_e154488_d_n4;
        locals.var_rrdrmues_dn5 = assign102680_e154488_d_n5;
        locals.var_rrdrmues_dn6 = assign102680_e154488_d_n6;
        locals.var_rrdrmues_dn7 = assign102680_e154488_d_n7;
        locals.var_rrdrmues_dn8 = assign102680_e154488_d_n8;
        locals.var_rrdrmues_dn9 = assign102680_e154488_d_n9;
        locals.var_rrdrmues_dn10 = assign102680_e154488_d_n10;
        locals.var_rrdrmues_dn11 = assign102680_e154488_d_n11;
        locals.var_rrdrmues_dn14 = assign102680_e154488_d_n14;
        locals.var_rrdrmues_rv = 0.0;

        let (assign102690_e154513, assign102690_e154513_d_n0, assign102690_e154513_d_n2, assign102690_e154513_d_n4, assign102690_e154513_d_n5, assign102690_e154513_d_n6, assign102690_e154513_d_n7, assign102690_e154513_d_n8, assign102690_e154513_d_n9, assign102690_e154513_d_n10, assign102690_e154513_d_n11, assign102690_e154513_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102690_e154498: f64 = (0.4 * locals.var_tratio);
        let assign102690_e154499: f64 = (1.8 + assign102690_e154498);
        let assign102690_e154502: f64 = (0.1 * locals.var_tratio);
        let assign102690_e154504: f64 = (assign102690_e154502 * locals.var_tratio);
        let assign102690_e154505: f64 = (assign102690_e154499 + assign102690_e154504);
        let assign102690_e154509: f64 = (1.0 - locals.var_tratio);
        let assign102690_e154510: f64 = (p.p418 * assign102690_e154509);
        let assign102690_e154511: f64 = (assign102690_e154505 - assign102690_e154510);
        (assign102690_e154511, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn11))) - (p.p418 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn14))) - (p.p418 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102690_e154513;
        locals.var_t0_dn0 = assign102690_e154513_d_n0;
        locals.var_t0_dn2 = assign102690_e154513_d_n2;
        locals.var_t0_dn4 = assign102690_e154513_d_n4;
        locals.var_t0_dn5 = assign102690_e154513_d_n5;
        locals.var_t0_dn6 = assign102690_e154513_d_n6;
        locals.var_t0_dn7 = assign102690_e154513_d_n7;
        locals.var_t0_dn8 = assign102690_e154513_d_n8;
        locals.var_t0_dn9 = assign102690_e154513_d_n9;
        locals.var_t0_dn10 = assign102690_e154513_d_n10;
        locals.var_t0_dn11 = assign102690_e154513_d_n11;
        locals.var_t0_dn14 = assign102690_e154513_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign102700_e154524, assign102700_e154524_d_n0, assign102700_e154524_d_n2, assign102700_e154524_d_n4, assign102700_e154524_d_n5, assign102700_e154524_d_n6, assign102700_e154524_d_n7, assign102700_e154524_d_n8, assign102700_e154524_d_n9, assign102700_e154524_d_n10, assign102700_e154524_d_n11, assign102700_e154524_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102700_e154522: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign102700_e154522, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn11, locals.var_rrdrvmaxs_dn14,)
    }
};
        locals.var_rrdrvmaxs = assign102700_e154524;
        locals.var_rrdrvmaxs_dn0 = assign102700_e154524_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign102700_e154524_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign102700_e154524_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign102700_e154524_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign102700_e154524_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign102700_e154524_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign102700_e154524_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign102700_e154524_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign102700_e154524_d_n10;
        locals.var_rrdrvmaxs_dn11 = assign102700_e154524_d_n11;
        locals.var_rrdrvmaxs_dn14 = assign102700_e154524_d_n14;
        locals.var_rrdrvmaxs_rv = 0.0;

        let (assign102710_e154539, assign102710_e154539_d_n0, assign102710_e154539_d_n2, assign102710_e154539_d_n4, assign102710_e154539_d_n5, assign102710_e154539_d_n6, assign102710_e154539_d_n7, assign102710_e154539_d_n8, assign102710_e154539_d_n9, assign102710_e154539_d_n10, assign102710_e154539_d_n11, assign102710_e154539_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102710_e154535: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign102710_e154536: f64 = (p.p439 * assign102710_e154535);
        let assign102710_e154537: f64 = (locals.var_uc_rdrbb_s + assign102710_e154536);
        (assign102710_e154537, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn11 + (p.p439 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_s_dn14 + (p.p439 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn11, locals.var_uc_rdrbb_s_dn14,)
    }
};
        locals.var_uc_rdrbb_s = assign102710_e154539;
        locals.var_uc_rdrbb_s_dn0 = assign102710_e154539_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign102710_e154539_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign102710_e154539_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign102710_e154539_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign102710_e154539_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign102710_e154539_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign102710_e154539_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign102710_e154539_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign102710_e154539_d_n10;
        locals.var_uc_rdrbb_s_dn11 = assign102710_e154539_d_n11;
        locals.var_uc_rdrbb_s_dn14 = assign102710_e154539_d_n14;
        locals.var_uc_rdrbb_s_rv = 0.0;

        let (assign102720_e154551, assign102720_e154551_d_n0, assign102720_e154551_d_n2, assign102720_e154551_d_n4, assign102720_e154551_d_n5, assign102720_e154551_d_n6, assign102720_e154551_d_n7, assign102720_e154551_d_n8, assign102720_e154551_d_n9, assign102720_e154551_d_n10, assign102720_e154551_d_n11, assign102720_e154551_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 == 0.0)) {
        let assign102720_e154547: f64 = ctx_temp;
        let assign102720_e154549: f64 = (assign102720_e154547 + p.p11);
        (assign102720_e154549, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign102720_e154551;
        locals.var_ttemp_dn0 = assign102720_e154551_d_n0;
        locals.var_ttemp_dn2 = assign102720_e154551_d_n2;
        locals.var_ttemp_dn4 = assign102720_e154551_d_n4;
        locals.var_ttemp_dn5 = assign102720_e154551_d_n5;
        locals.var_ttemp_dn6 = assign102720_e154551_d_n6;
        locals.var_ttemp_dn7 = assign102720_e154551_d_n7;
        locals.var_ttemp_dn8 = assign102720_e154551_d_n8;
        locals.var_ttemp_dn9 = assign102720_e154551_d_n9;
        locals.var_ttemp_dn10 = assign102720_e154551_d_n10;
        locals.var_ttemp_dn11 = assign102720_e154551_d_n11;
        locals.var_ttemp_dn14 = assign102720_e154551_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign102730_e154560,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102730_e154558: f64 = (locals.var_weff_ld * p.p7);
        (assign102730_e154558,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign102730_e154560;
        locals.var_weffld_nf_rv = 0.0;

        let (assign102740_e154567,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (p.p71,)
    } else {
        (locals.var_ldrifte_s,)
    }
};
        locals.var_ldrifte_s = assign102740_e154567;
        locals.var_ldrifte_s_rv = 0.0;

        let (assign102750_e154574,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_novers,)
    }
};
        locals.var_novers = assign102750_e154574;
        locals.var_novers_rv = 0.0;

        let (assign102760_e154583, assign102760_e154583_d_n0, assign102760_e154583_d_n2, assign102760_e154583_d_n4, assign102760_e154583_d_n5, assign102760_e154583_d_n6, assign102760_e154583_d_n7, assign102760_e154583_d_n8, assign102760_e154583_d_n9, assign102760_e154583_d_n10, assign102760_e154583_d_n11, assign102760_e154583_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102760_e154581: f64 = (locals.var_rrdrmues * locals.var_rdrmuele);
        (assign102760_e154581, (locals.var_rrdrmues_dn0 * locals.var_rdrmuele), (locals.var_rrdrmues_dn2 * locals.var_rdrmuele), (locals.var_rrdrmues_dn4 * locals.var_rdrmuele), (locals.var_rrdrmues_dn5 * locals.var_rdrmuele), (locals.var_rrdrmues_dn6 * locals.var_rdrmuele), (locals.var_rrdrmues_dn7 * locals.var_rdrmuele), (locals.var_rrdrmues_dn8 * locals.var_rdrmuele), (locals.var_rrdrmues_dn9 * locals.var_rdrmuele), (locals.var_rrdrmues_dn10 * locals.var_rdrmuele), (locals.var_rrdrmues_dn11 * locals.var_rdrmuele), (locals.var_rrdrmues_dn14 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0_s, locals.var_mu0_s_dn0, locals.var_mu0_s_dn2, locals.var_mu0_s_dn4, locals.var_mu0_s_dn5, locals.var_mu0_s_dn6, locals.var_mu0_s_dn7, locals.var_mu0_s_dn8, locals.var_mu0_s_dn9, locals.var_mu0_s_dn10, locals.var_mu0_s_dn11, locals.var_mu0_s_dn14,)
    }
};
        locals.var_mu0_s = assign102760_e154583;
        locals.var_mu0_s_dn0 = assign102760_e154583_d_n0;
        locals.var_mu0_s_dn2 = assign102760_e154583_d_n2;
        locals.var_mu0_s_dn4 = assign102760_e154583_d_n4;
        locals.var_mu0_s_dn5 = assign102760_e154583_d_n5;
        locals.var_mu0_s_dn6 = assign102760_e154583_d_n6;
        locals.var_mu0_s_dn7 = assign102760_e154583_d_n7;
        locals.var_mu0_s_dn8 = assign102760_e154583_d_n8;
        locals.var_mu0_s_dn9 = assign102760_e154583_d_n9;
        locals.var_mu0_s_dn10 = assign102760_e154583_d_n10;
        locals.var_mu0_s_dn11 = assign102760_e154583_d_n11;
        locals.var_mu0_s_dn14 = assign102760_e154583_d_n14;
        locals.var_mu0_s_rv = 0.0;

        let (assign102770_e154596, assign102770_e154596_d_n0, assign102770_e154596_d_n2, assign102770_e154596_d_n4, assign102770_e154596_d_n5, assign102770_e154596_d_n6, assign102770_e154596_d_n7, assign102770_e154596_d_n8, assign102770_e154596_d_n9, assign102770_e154596_d_n10, assign102770_e154596_d_n11, assign102770_e154596_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102770_e154590: f64 = (locals.var_rrdrvmaxs * locals.var_rdrvmaxwe);
        let assign102770_e154592: f64 = (assign102770_e154590 * locals.var_rdrvmaxle);
        let assign102770_e154594: f64 = (assign102770_e154592 + 1e-25);
        (assign102770_e154594, ((locals.var_rrdrvmaxs_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn14 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe_s, locals.var_vmaxe_s_dn0, locals.var_vmaxe_s_dn2, locals.var_vmaxe_s_dn4, locals.var_vmaxe_s_dn5, locals.var_vmaxe_s_dn6, locals.var_vmaxe_s_dn7, locals.var_vmaxe_s_dn8, locals.var_vmaxe_s_dn9, locals.var_vmaxe_s_dn10, locals.var_vmaxe_s_dn11, locals.var_vmaxe_s_dn14,)
    }
};
        locals.var_vmaxe_s = assign102770_e154596;
        locals.var_vmaxe_s_dn0 = assign102770_e154596_d_n0;
        locals.var_vmaxe_s_dn2 = assign102770_e154596_d_n2;
        locals.var_vmaxe_s_dn4 = assign102770_e154596_d_n4;
        locals.var_vmaxe_s_dn5 = assign102770_e154596_d_n5;
        locals.var_vmaxe_s_dn6 = assign102770_e154596_d_n6;
        locals.var_vmaxe_s_dn7 = assign102770_e154596_d_n7;
        locals.var_vmaxe_s_dn8 = assign102770_e154596_d_n8;
        locals.var_vmaxe_s_dn9 = assign102770_e154596_d_n9;
        locals.var_vmaxe_s_dn10 = assign102770_e154596_d_n10;
        locals.var_vmaxe_s_dn11 = assign102770_e154596_d_n11;
        locals.var_vmaxe_s_dn14 = assign102770_e154596_d_n14;
        locals.var_vmaxe_s_rv = 0.0;

        let (assign102780_e154605, assign102780_e154605_d_n2, assign102780_e154605_d_n8,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102780_e154603: f64 = (locals.var_vsps / locals.var_ldrifte_s);
        (assign102780_e154603, (locals.var_vsps_dn2 / locals.var_ldrifte_s), (locals.var_vsps_dn8 / locals.var_ldrifte_s),)
    } else {
        (locals.var_edri_s, locals.var_edri_s_dn2, locals.var_edri_s_dn8,)
    }
};
        locals.var_edri_s = assign102780_e154605;
        locals.var_edri_s_dn2 = assign102780_e154605_d_n2;
        locals.var_edri_s_dn8 = assign102780_e154605_d_n8;
        locals.var_edri_s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_394(
        locals: &mut StampLocals,
    ) {
        let (assign102790_e154614, assign102790_e154614_d_n0, assign102790_e154614_d_n2, assign102790_e154614_d_n4, assign102790_e154614_d_n5, assign102790_e154614_d_n6, assign102790_e154614_d_n7, assign102790_e154614_d_n8, assign102790_e154614_d_n9, assign102790_e154614_d_n10, assign102790_e154614_d_n11, assign102790_e154614_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102790_e154612: f64 = (locals.var_mu0_s * locals.var_edri_s);
        (assign102790_e154612, (locals.var_mu0_s_dn0 * locals.var_edri_s), ((locals.var_mu0_s_dn2 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn2)), (locals.var_mu0_s_dn4 * locals.var_edri_s), (locals.var_mu0_s_dn5 * locals.var_edri_s), (locals.var_mu0_s_dn6 * locals.var_edri_s), (locals.var_mu0_s_dn7 * locals.var_edri_s), ((locals.var_mu0_s_dn8 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn8)), (locals.var_mu0_s_dn9 * locals.var_edri_s), (locals.var_mu0_s_dn10 * locals.var_edri_s), (locals.var_mu0_s_dn11 * locals.var_edri_s), (locals.var_mu0_s_dn14 * locals.var_edri_s),)
    } else {
        (locals.var_vdri_s, locals.var_vdri_s_dn0, locals.var_vdri_s_dn2, locals.var_vdri_s_dn4, locals.var_vdri_s_dn5, locals.var_vdri_s_dn6, locals.var_vdri_s_dn7, locals.var_vdri_s_dn8, locals.var_vdri_s_dn9, locals.var_vdri_s_dn10, locals.var_vdri_s_dn11, locals.var_vdri_s_dn14,)
    }
};
        locals.var_vdri_s = assign102790_e154614;
        locals.var_vdri_s_dn0 = assign102790_e154614_d_n0;
        locals.var_vdri_s_dn2 = assign102790_e154614_d_n2;
        locals.var_vdri_s_dn4 = assign102790_e154614_d_n4;
        locals.var_vdri_s_dn5 = assign102790_e154614_d_n5;
        locals.var_vdri_s_dn6 = assign102790_e154614_d_n6;
        locals.var_vdri_s_dn7 = assign102790_e154614_d_n7;
        locals.var_vdri_s_dn8 = assign102790_e154614_d_n8;
        locals.var_vdri_s_dn9 = assign102790_e154614_d_n9;
        locals.var_vdri_s_dn10 = assign102790_e154614_d_n10;
        locals.var_vdri_s_dn11 = assign102790_e154614_d_n11;
        locals.var_vdri_s_dn14 = assign102790_e154614_d_n14;
        locals.var_vdri_s_rv = 0.0;

        let assign102800_e154617: f64 = if locals.var_vsps >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2343 = assign102800_e154617;
        locals.var_guard2343_rv = 0.0;

        let (assign102810_e154628, assign102810_e154628_d_n0, assign102810_e154628_d_n2, assign102810_e154628_d_n4, assign102810_e154628_d_n5, assign102810_e154628_d_n6, assign102810_e154628_d_n7, assign102810_e154628_d_n8, assign102810_e154628_d_n9, assign102810_e154628_d_n10, assign102810_e154628_d_n11, assign102810_e154628_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2343 != 0.0)) {
        let assign102810_e154626: f64 = (locals.var_vdri_s / locals.var_vmaxe_s);
        (assign102810_e154626, (((locals.var_vdri_s_dn0 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn2 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn4 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn5 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn6 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn7 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn8 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn9 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn10 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn11 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn11)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn14 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn14)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102810_e154628;
        locals.var_t1_dn0 = assign102810_e154628_d_n0;
        locals.var_t1_dn2 = assign102810_e154628_d_n2;
        locals.var_t1_dn4 = assign102810_e154628_d_n4;
        locals.var_t1_dn5 = assign102810_e154628_d_n5;
        locals.var_t1_dn6 = assign102810_e154628_d_n6;
        locals.var_t1_dn7 = assign102810_e154628_d_n7;
        locals.var_t1_dn8 = assign102810_e154628_d_n8;
        locals.var_t1_dn9 = assign102810_e154628_d_n9;
        locals.var_t1_dn10 = assign102810_e154628_d_n10;
        locals.var_t1_dn11 = assign102810_e154628_d_n11;
        locals.var_t1_dn14 = assign102810_e154628_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign102820_e154641, assign102820_e154641_d_n0, assign102820_e154641_d_n2, assign102820_e154641_d_n4, assign102820_e154641_d_n5, assign102820_e154641_d_n6, assign102820_e154641_d_n7, assign102820_e154641_d_n8, assign102820_e154641_d_n9, assign102820_e154641_d_n10, assign102820_e154641_d_n11, assign102820_e154641_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let assign102820_e154637: f64 = (-locals.var_vdri_s);
        let assign102820_e154639: f64 = (assign102820_e154637 / locals.var_vmaxe_s);
        (assign102820_e154639, ((((-locals.var_vdri_s_dn0) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn2) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn4) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn5) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn6) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn7) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn8) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn9) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn10) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn11) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn11)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn14) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn14)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102820_e154641;
        locals.var_t1_dn0 = assign102820_e154641_d_n0;
        locals.var_t1_dn2 = assign102820_e154641_d_n2;
        locals.var_t1_dn4 = assign102820_e154641_d_n4;
        locals.var_t1_dn5 = assign102820_e154641_d_n5;
        locals.var_t1_dn6 = assign102820_e154641_d_n6;
        locals.var_t1_dn7 = assign102820_e154641_d_n7;
        locals.var_t1_dn8 = assign102820_e154641_d_n8;
        locals.var_t1_dn9 = assign102820_e154641_d_n9;
        locals.var_t1_dn10 = assign102820_e154641_d_n10;
        locals.var_t1_dn11 = assign102820_e154641_d_n11;
        locals.var_t1_dn14 = assign102820_e154641_d_n14;
        locals.var_t1_rv = 0.0;

        let assign102830_e154645: f64 = (10.0 * 2.220446049250313e-16);
        let assign102830_e154646: f64 = (1.0 - assign102830_e154645);
        let assign102830_e154653: f64 = (10.0 * 2.220446049250313e-16);
        let assign102830_e154654: f64 = (1.0 + assign102830_e154653);
        let assign102830_e154656: f64 = if ((assign102830_e154646 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102830_e154654)) { 1.0 } else { 0.0 };
        locals.var_guard2344 = assign102830_e154656;
        locals.var_guard2344_rv = 0.0;

        let (assign102840_e154665, assign102840_e154665_d_n0, assign102840_e154665_d_n2, assign102840_e154665_d_n4, assign102840_e154665_d_n5, assign102840_e154665_d_n6, assign102840_e154665_d_n7, assign102840_e154665_d_n8, assign102840_e154665_d_n9, assign102840_e154665_d_n10, assign102840_e154665_d_n11, assign102840_e154665_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102840_e154665;
        locals.var_t3_dn0 = assign102840_e154665_d_n0;
        locals.var_t3_dn2 = assign102840_e154665_d_n2;
        locals.var_t3_dn4 = assign102840_e154665_d_n4;
        locals.var_t3_dn5 = assign102840_e154665_d_n5;
        locals.var_t3_dn6 = assign102840_e154665_d_n6;
        locals.var_t3_dn7 = assign102840_e154665_d_n7;
        locals.var_t3_dn8 = assign102840_e154665_d_n8;
        locals.var_t3_dn9 = assign102840_e154665_d_n9;
        locals.var_t3_dn10 = assign102840_e154665_d_n10;
        locals.var_t3_dn11 = assign102840_e154665_d_n11;
        locals.var_t3_dn14 = assign102840_e154665_d_n14;
        locals.var_t3_rv = 0.0;

        let assign102850_e154669: f64 = (10.0 * 2.220446049250313e-16);
        let assign102850_e154670: f64 = (2.0 - assign102850_e154669);
        let assign102850_e154677: f64 = (10.0 * 2.220446049250313e-16);
        let assign102850_e154678: f64 = (2.0 + assign102850_e154677);
        let assign102850_e154680: f64 = if ((assign102850_e154670 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102850_e154678)) { 1.0 } else { 0.0 };
        locals.var_guard2345 = assign102850_e154680;
        locals.var_guard2345_rv = 0.0;

        let (assign102860_e154692, assign102860_e154692_d_n0, assign102860_e154692_d_n2, assign102860_e154692_d_n4, assign102860_e154692_d_n5, assign102860_e154692_d_n6, assign102860_e154692_d_n7, assign102860_e154692_d_n8, assign102860_e154692_d_n9, assign102860_e154692_d_n10, assign102860_e154692_d_n11, assign102860_e154692_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102860_e154692;
        locals.var_t3_dn0 = assign102860_e154692_d_n0;
        locals.var_t3_dn2 = assign102860_e154692_d_n2;
        locals.var_t3_dn4 = assign102860_e154692_d_n4;
        locals.var_t3_dn5 = assign102860_e154692_d_n5;
        locals.var_t3_dn6 = assign102860_e154692_d_n6;
        locals.var_t3_dn7 = assign102860_e154692_d_n7;
        locals.var_t3_dn8 = assign102860_e154692_d_n8;
        locals.var_t3_dn9 = assign102860_e154692_d_n9;
        locals.var_t3_dn10 = assign102860_e154692_d_n10;
        locals.var_t3_dn11 = assign102860_e154692_d_n11;
        locals.var_t3_dn14 = assign102860_e154692_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign102870_e154709, assign102870_e154709_d_n0, assign102870_e154709_d_n2, assign102870_e154709_d_n4, assign102870_e154709_d_n5, assign102870_e154709_d_n6, assign102870_e154709_d_n7, assign102870_e154709_d_n8, assign102870_e154709_d_n9, assign102870_e154709_d_n10, assign102870_e154709_d_n11, assign102870_e154709_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 == 0.0)) {
        let assign102870_e154706: f64 = (locals.var_uc_rdrbb_s - 1.0);
        let assign102870_e154707: f64 = (locals.var_t1).powf(assign102870_e154706);
        (assign102870_e154707, if locals.var_uc_rdrbb_s_dn0 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn0)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn0 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn2 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn2)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn2 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn4 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn4)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn4 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn5 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn5)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn5 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn6 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn6)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn6 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn7 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn7)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn7 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn8 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn8)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn8 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn9 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn9)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn9 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn10 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn10)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn10 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn11 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn11)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn11 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn14 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn14)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn14 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn14 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102870_e154709;
        locals.var_t3_dn0 = assign102870_e154709_d_n0;
        locals.var_t3_dn2 = assign102870_e154709_d_n2;
        locals.var_t3_dn4 = assign102870_e154709_d_n4;
        locals.var_t3_dn5 = assign102870_e154709_d_n5;
        locals.var_t3_dn6 = assign102870_e154709_d_n6;
        locals.var_t3_dn7 = assign102870_e154709_d_n7;
        locals.var_t3_dn8 = assign102870_e154709_d_n8;
        locals.var_t3_dn9 = assign102870_e154709_d_n9;
        locals.var_t3_dn10 = assign102870_e154709_d_n10;
        locals.var_t3_dn11 = assign102870_e154709_d_n11;
        locals.var_t3_dn14 = assign102870_e154709_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign102880_e154718, assign102880_e154718_d_n0, assign102880_e154718_d_n2, assign102880_e154718_d_n4, assign102880_e154718_d_n5, assign102880_e154718_d_n6, assign102880_e154718_d_n7, assign102880_e154718_d_n8, assign102880_e154718_d_n9, assign102880_e154718_d_n10, assign102880_e154718_d_n11, assign102880_e154718_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102880_e154716: f64 = (locals.var_t1 * locals.var_t3);
        (assign102880_e154716, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign102880_e154718;
        locals.var_t2_dn0 = assign102880_e154718_d_n0;
        locals.var_t2_dn2 = assign102880_e154718_d_n2;
        locals.var_t2_dn4 = assign102880_e154718_d_n4;
        locals.var_t2_dn5 = assign102880_e154718_d_n5;
        locals.var_t2_dn6 = assign102880_e154718_d_n6;
        locals.var_t2_dn7 = assign102880_e154718_d_n7;
        locals.var_t2_dn8 = assign102880_e154718_d_n8;
        locals.var_t2_dn9 = assign102880_e154718_d_n9;
        locals.var_t2_dn10 = assign102880_e154718_d_n10;
        locals.var_t2_dn11 = assign102880_e154718_d_n11;
        locals.var_t2_dn14 = assign102880_e154718_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign102890_e154727, assign102890_e154727_d_n0, assign102890_e154727_d_n2, assign102890_e154727_d_n4, assign102890_e154727_d_n5, assign102890_e154727_d_n6, assign102890_e154727_d_n7, assign102890_e154727_d_n8, assign102890_e154727_d_n9, assign102890_e154727_d_n10, assign102890_e154727_d_n11, assign102890_e154727_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102890_e154725: f64 = (1.0 + locals.var_t2);
        (assign102890_e154725, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign102890_e154727;
        locals.var_t4_dn0 = assign102890_e154727_d_n0;
        locals.var_t4_dn2 = assign102890_e154727_d_n2;
        locals.var_t4_dn4 = assign102890_e154727_d_n4;
        locals.var_t4_dn5 = assign102890_e154727_d_n5;
        locals.var_t4_dn6 = assign102890_e154727_d_n6;
        locals.var_t4_dn7 = assign102890_e154727_d_n7;
        locals.var_t4_dn8 = assign102890_e154727_d_n8;
        locals.var_t4_dn9 = assign102890_e154727_d_n9;
        locals.var_t4_dn10 = assign102890_e154727_d_n10;
        locals.var_t4_dn11 = assign102890_e154727_d_n11;
        locals.var_t4_dn14 = assign102890_e154727_d_n14;
        locals.var_t4_rv = 0.0;

        let assign102900_e154731: f64 = (10.0 * 2.220446049250313e-16);
        let assign102900_e154732: f64 = (1.0 - assign102900_e154731);
        let assign102900_e154739: f64 = (10.0 * 2.220446049250313e-16);
        let assign102900_e154740: f64 = (1.0 + assign102900_e154739);
        let assign102900_e154742: f64 = if ((assign102900_e154732 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102900_e154740)) { 1.0 } else { 0.0 };
        locals.var_guard2346 = assign102900_e154742;
        locals.var_guard2346_rv = 0.0;

        let (assign102910_e154753, assign102910_e154753_d_n0, assign102910_e154753_d_n2, assign102910_e154753_d_n4, assign102910_e154753_d_n5, assign102910_e154753_d_n6, assign102910_e154753_d_n7, assign102910_e154753_d_n8, assign102910_e154753_d_n9, assign102910_e154753_d_n10, assign102910_e154753_d_n11, assign102910_e154753_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign102910_e154751: f64 = (1.0 / locals.var_t4);
        (assign102910_e154751, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102910_e154753;
        locals.var_t5_dn0 = assign102910_e154753_d_n0;
        locals.var_t5_dn2 = assign102910_e154753_d_n2;
        locals.var_t5_dn4 = assign102910_e154753_d_n4;
        locals.var_t5_dn5 = assign102910_e154753_d_n5;
        locals.var_t5_dn6 = assign102910_e154753_d_n6;
        locals.var_t5_dn7 = assign102910_e154753_d_n7;
        locals.var_t5_dn8 = assign102910_e154753_d_n8;
        locals.var_t5_dn9 = assign102910_e154753_d_n9;
        locals.var_t5_dn10 = assign102910_e154753_d_n10;
        locals.var_t5_dn11 = assign102910_e154753_d_n11;
        locals.var_t5_dn14 = assign102910_e154753_d_n14;
        locals.var_t5_rv = 0.0;

        let assign102920_e154757: f64 = (10.0 * 2.220446049250313e-16);
        let assign102920_e154758: f64 = (2.0 - assign102920_e154757);
        let assign102920_e154765: f64 = (10.0 * 2.220446049250313e-16);
        let assign102920_e154766: f64 = (2.0 + assign102920_e154765);
        let assign102920_e154768: f64 = if ((assign102920_e154758 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102920_e154766)) { 1.0 } else { 0.0 };
        locals.var_guard2347 = assign102920_e154768;
        locals.var_guard2347_rv = 0.0;

        let (assign102930_e154783, assign102930_e154783_d_n0, assign102930_e154783_d_n2, assign102930_e154783_d_n4, assign102930_e154783_d_n5, assign102930_e154783_d_n6, assign102930_e154783_d_n7, assign102930_e154783_d_n8, assign102930_e154783_d_n9, assign102930_e154783_d_n10, assign102930_e154783_d_n11, assign102930_e154783_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 != 0.0)) {
        let assign102930_e154780: f64 = (locals.var_t4).sqrt();
        let assign102930_e154781: f64 = (1.0 / assign102930_e154780);
        (assign102930_e154781, (-((locals.var_t4_dn0 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn2 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn4 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn5 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn6 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn7 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn8 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn9 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn10 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn11 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn14 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102930_e154783;
        locals.var_t5_dn0 = assign102930_e154783_d_n0;
        locals.var_t5_dn2 = assign102930_e154783_d_n2;
        locals.var_t5_dn4 = assign102930_e154783_d_n4;
        locals.var_t5_dn5 = assign102930_e154783_d_n5;
        locals.var_t5_dn6 = assign102930_e154783_d_n6;
        locals.var_t5_dn7 = assign102930_e154783_d_n7;
        locals.var_t5_dn8 = assign102930_e154783_d_n8;
        locals.var_t5_dn9 = assign102930_e154783_d_n9;
        locals.var_t5_dn10 = assign102930_e154783_d_n10;
        locals.var_t5_dn11 = assign102930_e154783_d_n11;
        locals.var_t5_dn14 = assign102930_e154783_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign102940_e154808, assign102940_e154808_d_n0, assign102940_e154808_d_n2, assign102940_e154808_d_n4, assign102940_e154808_d_n5, assign102940_e154808_d_n6, assign102940_e154808_d_n7, assign102940_e154808_d_n8, assign102940_e154808_d_n9, assign102940_e154808_d_n10, assign102940_e154808_d_n11, assign102940_e154808_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) {
        let (assign102940_e154806, assign102940_e154806_d_n0, assign102940_e154806_d_n2, assign102940_e154806_d_n4, assign102940_e154806_d_n5, assign102940_e154806_d_n6, assign102940_e154806_d_n7, assign102940_e154806_d_n8, assign102940_e154806_d_n9, assign102940_e154806_d_n10, assign102940_e154806_d_n11, assign102940_e154806_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102940_e154800: f64 = (-1.0);
                let assign102940_e154802: f64 = (assign102940_e154800 / locals.var_uc_rdrbb_s);
                let assign102940_e154804: f64 = (assign102940_e154802 - 1.0);
                let assign102940_e154805: f64 = (locals.var_t4).powf(assign102940_e154804);
                (assign102940_e154805, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn0)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn2)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn4)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn5)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn6)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn7)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn8)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn9)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn10)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn11) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn11)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn11) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn11 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn14) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn14)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn14) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn14 / locals.var_t4)))) },)
            }
        };
        (assign102940_e154806, assign102940_e154806_d_n0, assign102940_e154806_d_n2, assign102940_e154806_d_n4, assign102940_e154806_d_n5, assign102940_e154806_d_n6, assign102940_e154806_d_n7, assign102940_e154806_d_n8, assign102940_e154806_d_n9, assign102940_e154806_d_n10, assign102940_e154806_d_n11, assign102940_e154806_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign102940_e154808;
        locals.var_t6_dn0 = assign102940_e154808_d_n0;
        locals.var_t6_dn2 = assign102940_e154808_d_n2;
        locals.var_t6_dn4 = assign102940_e154808_d_n4;
        locals.var_t6_dn5 = assign102940_e154808_d_n5;
        locals.var_t6_dn6 = assign102940_e154808_d_n6;
        locals.var_t6_dn7 = assign102940_e154808_d_n7;
        locals.var_t6_dn8 = assign102940_e154808_d_n8;
        locals.var_t6_dn9 = assign102940_e154808_d_n9;
        locals.var_t6_dn10 = assign102940_e154808_d_n10;
        locals.var_t6_dn11 = assign102940_e154808_d_n11;
        locals.var_t6_dn14 = assign102940_e154808_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign102950_e154823, assign102950_e154823_d_n0, assign102950_e154823_d_n2, assign102950_e154823_d_n4, assign102950_e154823_d_n5, assign102950_e154823_d_n6, assign102950_e154823_d_n7, assign102950_e154823_d_n8, assign102950_e154823_d_n9, assign102950_e154823_d_n10, assign102950_e154823_d_n11, assign102950_e154823_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) {
        let assign102950_e154821: f64 = (locals.var_t4 * locals.var_t6);
        (assign102950_e154821, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102950_e154823;
        locals.var_t5_dn0 = assign102950_e154823_d_n0;
        locals.var_t5_dn2 = assign102950_e154823_d_n2;
        locals.var_t5_dn4 = assign102950_e154823_d_n4;
        locals.var_t5_dn5 = assign102950_e154823_d_n5;
        locals.var_t5_dn6 = assign102950_e154823_d_n6;
        locals.var_t5_dn7 = assign102950_e154823_d_n7;
        locals.var_t5_dn8 = assign102950_e154823_d_n8;
        locals.var_t5_dn9 = assign102950_e154823_d_n9;
        locals.var_t5_dn10 = assign102950_e154823_d_n10;
        locals.var_t5_dn11 = assign102950_e154823_d_n11;
        locals.var_t5_dn14 = assign102950_e154823_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign102960_e154832, assign102960_e154832_d_n0, assign102960_e154832_d_n2, assign102960_e154832_d_n4, assign102960_e154832_d_n5, assign102960_e154832_d_n6, assign102960_e154832_d_n7, assign102960_e154832_d_n8, assign102960_e154832_d_n9, assign102960_e154832_d_n10, assign102960_e154832_d_n11, assign102960_e154832_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102960_e154830: f64 = (locals.var_mu0_s * locals.var_t5);
        (assign102960_e154830, ((locals.var_mu0_s_dn0 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn0)), ((locals.var_mu0_s_dn2 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn2)), ((locals.var_mu0_s_dn4 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn4)), ((locals.var_mu0_s_dn5 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn5)), ((locals.var_mu0_s_dn6 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn6)), ((locals.var_mu0_s_dn7 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn7)), ((locals.var_mu0_s_dn8 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn8)), ((locals.var_mu0_s_dn9 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn9)), ((locals.var_mu0_s_dn10 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn10)), ((locals.var_mu0_s_dn11 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn11)), ((locals.var_mu0_s_dn14 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu_s, locals.var_mu_s_dn0, locals.var_mu_s_dn2, locals.var_mu_s_dn4, locals.var_mu_s_dn5, locals.var_mu_s_dn6, locals.var_mu_s_dn7, locals.var_mu_s_dn8, locals.var_mu_s_dn9, locals.var_mu_s_dn10, locals.var_mu_s_dn11, locals.var_mu_s_dn14,)
    }
};
        locals.var_mu_s = assign102960_e154832;
        locals.var_mu_s_dn0 = assign102960_e154832_d_n0;
        locals.var_mu_s_dn2 = assign102960_e154832_d_n2;
        locals.var_mu_s_dn4 = assign102960_e154832_d_n4;
        locals.var_mu_s_dn5 = assign102960_e154832_d_n5;
        locals.var_mu_s_dn6 = assign102960_e154832_d_n6;
        locals.var_mu_s_dn7 = assign102960_e154832_d_n7;
        locals.var_mu_s_dn8 = assign102960_e154832_d_n8;
        locals.var_mu_s_dn9 = assign102960_e154832_d_n9;
        locals.var_mu_s_dn10 = assign102960_e154832_d_n10;
        locals.var_mu_s_dn11 = assign102960_e154832_d_n11;
        locals.var_mu_s_dn14 = assign102960_e154832_d_n14;
        locals.var_mu_s_rv = 0.0;

        let (assign102970_e154839,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (locals.var_novers,)
    } else {
        (locals.var_carr_s,)
    }
};
        locals.var_carr_s = assign102970_e154839;
        locals.var_carr_s_rv = 0.0;

        let (assign102980_e154846,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (locals.var_xmax_s,)
    } else {
        (locals.var_xov_s,)
    }
};
        locals.var_xov_s = assign102980_e154846;
        locals.var_xov_s_rv = 0.0;

        let (assign102990_e154855, assign102990_e154855_d_n0, assign102990_e154855_d_n2, assign102990_e154855_d_n4, assign102990_e154855_d_n5, assign102990_e154855_d_n6, assign102990_e154855_d_n7, assign102990_e154855_d_n8, assign102990_e154855_d_n9, assign102990_e154855_d_n10, assign102990_e154855_d_n11, assign102990_e154855_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102990_e154853: f64 = (1.6021918e-19 / locals.var_ldrifte_s);
        (assign102990_e154853, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102990_e154855;
        locals.var_t1_dn0 = assign102990_e154855_d_n0;
        locals.var_t1_dn2 = assign102990_e154855_d_n2;
        locals.var_t1_dn4 = assign102990_e154855_d_n4;
        locals.var_t1_dn5 = assign102990_e154855_d_n5;
        locals.var_t1_dn6 = assign102990_e154855_d_n6;
        locals.var_t1_dn7 = assign102990_e154855_d_n7;
        locals.var_t1_dn8 = assign102990_e154855_d_n8;
        locals.var_t1_dn9 = assign102990_e154855_d_n9;
        locals.var_t1_dn10 = assign102990_e154855_d_n10;
        locals.var_t1_dn11 = assign102990_e154855_d_n11;
        locals.var_t1_dn14 = assign102990_e154855_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign103000_e154868, assign103000_e154868_d_n0, assign103000_e154868_d_n2, assign103000_e154868_d_n4, assign103000_e154868_d_n5, assign103000_e154868_d_n6, assign103000_e154868_d_n7, assign103000_e154868_d_n8, assign103000_e154868_d_n9, assign103000_e154868_d_n10, assign103000_e154868_d_n11, assign103000_e154868_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103000_e154862: f64 = (locals.var_t1 * locals.var_xov_s);
        let assign103000_e154864: f64 = (assign103000_e154862 * locals.var_mu_s);
        let assign103000_e154866: f64 = (assign103000_e154864 * locals.var_carr_s);
        (assign103000_e154866, ((((locals.var_t1_dn0 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn0)) * locals.var_carr_s), ((((locals.var_t1_dn2 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn2)) * locals.var_carr_s), ((((locals.var_t1_dn4 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn4)) * locals.var_carr_s), ((((locals.var_t1_dn5 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn5)) * locals.var_carr_s), ((((locals.var_t1_dn6 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn6)) * locals.var_carr_s), ((((locals.var_t1_dn7 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn7)) * locals.var_carr_s), ((((locals.var_t1_dn8 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn8)) * locals.var_carr_s), ((((locals.var_t1_dn9 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn9)) * locals.var_carr_s), ((((locals.var_t1_dn10 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn10)) * locals.var_carr_s), ((((locals.var_t1_dn11 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn11)) * locals.var_carr_s), ((((locals.var_t1_dn14 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn14)) * locals.var_carr_s),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103000_e154868;
        locals.var_gd_s_dn0 = assign103000_e154868_d_n0;
        locals.var_gd_s_dn2 = assign103000_e154868_d_n2;
        locals.var_gd_s_dn4 = assign103000_e154868_d_n4;
        locals.var_gd_s_dn5 = assign103000_e154868_d_n5;
        locals.var_gd_s_dn6 = assign103000_e154868_d_n6;
        locals.var_gd_s_dn7 = assign103000_e154868_d_n7;
        locals.var_gd_s_dn8 = assign103000_e154868_d_n8;
        locals.var_gd_s_dn9 = assign103000_e154868_d_n9;
        locals.var_gd_s_dn10 = assign103000_e154868_d_n10;
        locals.var_gd_s_dn11 = assign103000_e154868_d_n11;
        locals.var_gd_s_dn14 = assign103000_e154868_d_n14;
        locals.var_gd_s_rv = 0.0;

        let assign103010_e154872: f64 = 1e-25;
        let assign103010_e154877: f64 = if ((locals.var_gd_s < assign103010_e154872) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2348 = assign103010_e154877;
        locals.var_guard2348_rv = 0.0;

        let (assign103020_e154890, assign103020_e154890_d_n0, assign103020_e154890_d_n2, assign103020_e154890_d_n4, assign103020_e154890_d_n5, assign103020_e154890_d_n6, assign103020_e154890_d_n7, assign103020_e154890_d_n8, assign103020_e154890_d_n9, assign103020_e154890_d_n10, assign103020_e154890_d_n11, assign103020_e154890_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103020_e154886: f64 = 1e-25;
        let assign103020_e154888: f64 = (assign103020_e154886 - locals.var_gd_s);
        (assign103020_e154888, (-locals.var_gd_s_dn0), (-locals.var_gd_s_dn2), (-locals.var_gd_s_dn4), (-locals.var_gd_s_dn5), (-locals.var_gd_s_dn6), (-locals.var_gd_s_dn7), (-locals.var_gd_s_dn8), (-locals.var_gd_s_dn9), (-locals.var_gd_s_dn10), (-locals.var_gd_s_dn11), (-locals.var_gd_s_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103020_e154890;
        locals.var_tmf1_dn0 = assign103020_e154890_d_n0;
        locals.var_tmf1_dn2 = assign103020_e154890_d_n2;
        locals.var_tmf1_dn4 = assign103020_e154890_d_n4;
        locals.var_tmf1_dn5 = assign103020_e154890_d_n5;
        locals.var_tmf1_dn6 = assign103020_e154890_d_n6;
        locals.var_tmf1_dn7 = assign103020_e154890_d_n7;
        locals.var_tmf1_dn8 = assign103020_e154890_d_n8;
        locals.var_tmf1_dn9 = assign103020_e154890_d_n9;
        locals.var_tmf1_dn10 = assign103020_e154890_d_n10;
        locals.var_tmf1_dn11 = assign103020_e154890_d_n11;
        locals.var_tmf1_dn14 = assign103020_e154890_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign103030_e154901, assign103030_e154901_d_n0, assign103030_e154901_d_n2, assign103030_e154901_d_n4, assign103030_e154901_d_n5, assign103030_e154901_d_n6, assign103030_e154901_d_n7, assign103030_e154901_d_n8, assign103030_e154901_d_n9, assign103030_e154901_d_n10, assign103030_e154901_d_n11, assign103030_e154901_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103030_e154899: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign103030_e154899, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign103030_e154901;
        locals.var_x2_dn0 = assign103030_e154901_d_n0;
        locals.var_x2_dn2 = assign103030_e154901_d_n2;
        locals.var_x2_dn4 = assign103030_e154901_d_n4;
        locals.var_x2_dn5 = assign103030_e154901_d_n5;
        locals.var_x2_dn6 = assign103030_e154901_d_n6;
        locals.var_x2_dn7 = assign103030_e154901_d_n7;
        locals.var_x2_dn8 = assign103030_e154901_d_n8;
        locals.var_x2_dn9 = assign103030_e154901_d_n9;
        locals.var_x2_dn10 = assign103030_e154901_d_n10;
        locals.var_x2_dn11 = assign103030_e154901_d_n11;
        locals.var_x2_dn14 = assign103030_e154901_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign103040_e154912, assign103040_e154912_d_n0, assign103040_e154912_d_n2, assign103040_e154912_d_n4, assign103040_e154912_d_n5, assign103040_e154912_d_n6, assign103040_e154912_d_n7, assign103040_e154912_d_n8, assign103040_e154912_d_n9, assign103040_e154912_d_n10, assign103040_e154912_d_n11, assign103040_e154912_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103040_e154910: f64 = (1e-25 * 1e-25);
        (assign103040_e154910, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign103040_e154912;
        locals.var_xmax2_dn0 = assign103040_e154912_d_n0;
        locals.var_xmax2_dn2 = assign103040_e154912_d_n2;
        locals.var_xmax2_dn4 = assign103040_e154912_d_n4;
        locals.var_xmax2_dn5 = assign103040_e154912_d_n5;
        locals.var_xmax2_dn6 = assign103040_e154912_d_n6;
        locals.var_xmax2_dn7 = assign103040_e154912_d_n7;
        locals.var_xmax2_dn8 = assign103040_e154912_d_n8;
        locals.var_xmax2_dn9 = assign103040_e154912_d_n9;
        locals.var_xmax2_dn10 = assign103040_e154912_d_n10;
        locals.var_xmax2_dn11 = assign103040_e154912_d_n11;
        locals.var_xmax2_dn14 = assign103040_e154912_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign103050_e154921, assign103050_e154921_d_n0, assign103050_e154921_d_n2, assign103050_e154921_d_n4, assign103050_e154921_d_n5, assign103050_e154921_d_n6, assign103050_e154921_d_n7, assign103050_e154921_d_n8, assign103050_e154921_d_n9, assign103050_e154921_d_n10, assign103050_e154921_d_n11, assign103050_e154921_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103050_e154921;
        locals.var_xp_dn0 = assign103050_e154921_d_n0;
        locals.var_xp_dn2 = assign103050_e154921_d_n2;
        locals.var_xp_dn4 = assign103050_e154921_d_n4;
        locals.var_xp_dn5 = assign103050_e154921_d_n5;
        locals.var_xp_dn6 = assign103050_e154921_d_n6;
        locals.var_xp_dn7 = assign103050_e154921_d_n7;
        locals.var_xp_dn8 = assign103050_e154921_d_n8;
        locals.var_xp_dn9 = assign103050_e154921_d_n9;
        locals.var_xp_dn10 = assign103050_e154921_d_n10;
        locals.var_xp_dn11 = assign103050_e154921_d_n11;
        locals.var_xp_dn14 = assign103050_e154921_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_395(
        locals: &mut StampLocals,
    ) {
        let (assign103060_e154930, assign103060_e154930_d_n0, assign103060_e154930_d_n2, assign103060_e154930_d_n4, assign103060_e154930_d_n5, assign103060_e154930_d_n6, assign103060_e154930_d_n7, assign103060_e154930_d_n8, assign103060_e154930_d_n9, assign103060_e154930_d_n10, assign103060_e154930_d_n11, assign103060_e154930_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103060_e154930;
        locals.var_xmp_dn0 = assign103060_e154930_d_n0;
        locals.var_xmp_dn2 = assign103060_e154930_d_n2;
        locals.var_xmp_dn4 = assign103060_e154930_d_n4;
        locals.var_xmp_dn5 = assign103060_e154930_d_n5;
        locals.var_xmp_dn6 = assign103060_e154930_d_n6;
        locals.var_xmp_dn7 = assign103060_e154930_d_n7;
        locals.var_xmp_dn8 = assign103060_e154930_d_n8;
        locals.var_xmp_dn9 = assign103060_e154930_d_n9;
        locals.var_xmp_dn10 = assign103060_e154930_d_n10;
        locals.var_xmp_dn11 = assign103060_e154930_d_n11;
        locals.var_xmp_dn14 = assign103060_e154930_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign103070_e154939,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103070_e154939;
        locals.var_m0_rv = 0.0;

        let (assign103080_e154948,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103080_e154948;
        locals.var_mm_rv = 0.0;

        let (assign103090_e154957, assign103090_e154957_d_n0, assign103090_e154957_d_n2, assign103090_e154957_d_n4, assign103090_e154957_d_n5, assign103090_e154957_d_n6, assign103090_e154957_d_n7, assign103090_e154957_d_n8, assign103090_e154957_d_n9, assign103090_e154957_d_n10, assign103090_e154957_d_n11, assign103090_e154957_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign103090_e154957;
        locals.var_arg_dn0 = assign103090_e154957_d_n0;
        locals.var_arg_dn2 = assign103090_e154957_d_n2;
        locals.var_arg_dn4 = assign103090_e154957_d_n4;
        locals.var_arg_dn5 = assign103090_e154957_d_n5;
        locals.var_arg_dn6 = assign103090_e154957_d_n6;
        locals.var_arg_dn7 = assign103090_e154957_d_n7;
        locals.var_arg_dn8 = assign103090_e154957_d_n8;
        locals.var_arg_dn9 = assign103090_e154957_d_n9;
        locals.var_arg_dn10 = assign103090_e154957_d_n10;
        locals.var_arg_dn11 = assign103090_e154957_d_n11;
        locals.var_arg_dn14 = assign103090_e154957_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign103100_e154966, assign103100_e154966_d_n0, assign103100_e154966_d_n2, assign103100_e154966_d_n4, assign103100_e154966_d_n5, assign103100_e154966_d_n6, assign103100_e154966_d_n7, assign103100_e154966_d_n8, assign103100_e154966_d_n9, assign103100_e154966_d_n10, assign103100_e154966_d_n11, assign103100_e154966_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103100_e154966;
        locals.var_dnm_dn0 = assign103100_e154966_d_n0;
        locals.var_dnm_dn2 = assign103100_e154966_d_n2;
        locals.var_dnm_dn4 = assign103100_e154966_d_n4;
        locals.var_dnm_dn5 = assign103100_e154966_d_n5;
        locals.var_dnm_dn6 = assign103100_e154966_d_n6;
        locals.var_dnm_dn7 = assign103100_e154966_d_n7;
        locals.var_dnm_dn8 = assign103100_e154966_d_n8;
        locals.var_dnm_dn9 = assign103100_e154966_d_n9;
        locals.var_dnm_dn10 = assign103100_e154966_d_n10;
        locals.var_dnm_dn11 = assign103100_e154966_d_n11;
        locals.var_dnm_dn14 = assign103100_e154966_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign103110_e154977, assign103110_e154977_d_n0, assign103110_e154977_d_n2, assign103110_e154977_d_n4, assign103110_e154977_d_n5, assign103110_e154977_d_n6, assign103110_e154977_d_n7, assign103110_e154977_d_n8, assign103110_e154977_d_n9, assign103110_e154977_d_n10, assign103110_e154977_d_n11, assign103110_e154977_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103110_e154975: f64 = (locals.var_xp * locals.var_x2);
        (assign103110_e154975, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103110_e154977;
        locals.var_xp_dn0 = assign103110_e154977_d_n0;
        locals.var_xp_dn2 = assign103110_e154977_d_n2;
        locals.var_xp_dn4 = assign103110_e154977_d_n4;
        locals.var_xp_dn5 = assign103110_e154977_d_n5;
        locals.var_xp_dn6 = assign103110_e154977_d_n6;
        locals.var_xp_dn7 = assign103110_e154977_d_n7;
        locals.var_xp_dn8 = assign103110_e154977_d_n8;
        locals.var_xp_dn9 = assign103110_e154977_d_n9;
        locals.var_xp_dn10 = assign103110_e154977_d_n10;
        locals.var_xp_dn11 = assign103110_e154977_d_n11;
        locals.var_xp_dn14 = assign103110_e154977_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign103120_e154988, assign103120_e154988_d_n0, assign103120_e154988_d_n2, assign103120_e154988_d_n4, assign103120_e154988_d_n5, assign103120_e154988_d_n6, assign103120_e154988_d_n7, assign103120_e154988_d_n8, assign103120_e154988_d_n9, assign103120_e154988_d_n10, assign103120_e154988_d_n11, assign103120_e154988_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103120_e154986: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103120_e154986, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103120_e154988;
        locals.var_xmp_dn0 = assign103120_e154988_d_n0;
        locals.var_xmp_dn2 = assign103120_e154988_d_n2;
        locals.var_xmp_dn4 = assign103120_e154988_d_n4;
        locals.var_xmp_dn5 = assign103120_e154988_d_n5;
        locals.var_xmp_dn6 = assign103120_e154988_d_n6;
        locals.var_xmp_dn7 = assign103120_e154988_d_n7;
        locals.var_xmp_dn8 = assign103120_e154988_d_n8;
        locals.var_xmp_dn9 = assign103120_e154988_d_n9;
        locals.var_xmp_dn10 = assign103120_e154988_d_n10;
        locals.var_xmp_dn11 = assign103120_e154988_d_n11;
        locals.var_xmp_dn14 = assign103120_e154988_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign103130_e154999, assign103130_e154999_d_n0, assign103130_e154999_d_n2, assign103130_e154999_d_n4, assign103130_e154999_d_n5, assign103130_e154999_d_n6, assign103130_e154999_d_n7, assign103130_e154999_d_n8, assign103130_e154999_d_n9, assign103130_e154999_d_n10, assign103130_e154999_d_n11, assign103130_e154999_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103130_e154997: f64 = (locals.var_xp * locals.var_x2);
        (assign103130_e154997, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103130_e154999;
        locals.var_xp_dn0 = assign103130_e154999_d_n0;
        locals.var_xp_dn2 = assign103130_e154999_d_n2;
        locals.var_xp_dn4 = assign103130_e154999_d_n4;
        locals.var_xp_dn5 = assign103130_e154999_d_n5;
        locals.var_xp_dn6 = assign103130_e154999_d_n6;
        locals.var_xp_dn7 = assign103130_e154999_d_n7;
        locals.var_xp_dn8 = assign103130_e154999_d_n8;
        locals.var_xp_dn9 = assign103130_e154999_d_n9;
        locals.var_xp_dn10 = assign103130_e154999_d_n10;
        locals.var_xp_dn11 = assign103130_e154999_d_n11;
        locals.var_xp_dn14 = assign103130_e154999_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign103140_e155010, assign103140_e155010_d_n0, assign103140_e155010_d_n2, assign103140_e155010_d_n4, assign103140_e155010_d_n5, assign103140_e155010_d_n6, assign103140_e155010_d_n7, assign103140_e155010_d_n8, assign103140_e155010_d_n9, assign103140_e155010_d_n10, assign103140_e155010_d_n11, assign103140_e155010_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103140_e155008: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103140_e155008, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103140_e155010;
        locals.var_xmp_dn0 = assign103140_e155010_d_n0;
        locals.var_xmp_dn2 = assign103140_e155010_d_n2;
        locals.var_xmp_dn4 = assign103140_e155010_d_n4;
        locals.var_xmp_dn5 = assign103140_e155010_d_n5;
        locals.var_xmp_dn6 = assign103140_e155010_d_n6;
        locals.var_xmp_dn7 = assign103140_e155010_d_n7;
        locals.var_xmp_dn8 = assign103140_e155010_d_n8;
        locals.var_xmp_dn9 = assign103140_e155010_d_n9;
        locals.var_xmp_dn10 = assign103140_e155010_d_n10;
        locals.var_xmp_dn11 = assign103140_e155010_d_n11;
        locals.var_xmp_dn14 = assign103140_e155010_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign103150_e155021, assign103150_e155021_d_n0, assign103150_e155021_d_n2, assign103150_e155021_d_n4, assign103150_e155021_d_n5, assign103150_e155021_d_n6, assign103150_e155021_d_n7, assign103150_e155021_d_n8, assign103150_e155021_d_n9, assign103150_e155021_d_n10, assign103150_e155021_d_n11, assign103150_e155021_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103150_e155019: f64 = (locals.var_xp + locals.var_xmp);
        (assign103150_e155019, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign103150_e155021;
        locals.var_arg_dn0 = assign103150_e155021_d_n0;
        locals.var_arg_dn2 = assign103150_e155021_d_n2;
        locals.var_arg_dn4 = assign103150_e155021_d_n4;
        locals.var_arg_dn5 = assign103150_e155021_d_n5;
        locals.var_arg_dn6 = assign103150_e155021_d_n6;
        locals.var_arg_dn7 = assign103150_e155021_d_n7;
        locals.var_arg_dn8 = assign103150_e155021_d_n8;
        locals.var_arg_dn9 = assign103150_e155021_d_n9;
        locals.var_arg_dn10 = assign103150_e155021_d_n10;
        locals.var_arg_dn11 = assign103150_e155021_d_n11;
        locals.var_arg_dn14 = assign103150_e155021_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign103160_e155030, assign103160_e155030_d_n0, assign103160_e155030_d_n2, assign103160_e155030_d_n4, assign103160_e155030_d_n5, assign103160_e155030_d_n6, assign103160_e155030_d_n7, assign103160_e155030_d_n8, assign103160_e155030_d_n9, assign103160_e155030_d_n10, assign103160_e155030_d_n11, assign103160_e155030_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103160_e155030;
        locals.var_dnm_dn0 = assign103160_e155030_d_n0;
        locals.var_dnm_dn2 = assign103160_e155030_d_n2;
        locals.var_dnm_dn4 = assign103160_e155030_d_n4;
        locals.var_dnm_dn5 = assign103160_e155030_d_n5;
        locals.var_dnm_dn6 = assign103160_e155030_d_n6;
        locals.var_dnm_dn7 = assign103160_e155030_d_n7;
        locals.var_dnm_dn8 = assign103160_e155030_d_n8;
        locals.var_dnm_dn9 = assign103160_e155030_d_n9;
        locals.var_dnm_dn10 = assign103160_e155030_d_n10;
        locals.var_dnm_dn11 = assign103160_e155030_d_n11;
        locals.var_dnm_dn14 = assign103160_e155030_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign103170_e155045: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2349 = assign103170_e155045;
        locals.var_guard2349_rv = 0.0;

        let assign103180_e155048: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2350 = assign103180_e155048;
        locals.var_guard2350_rv = 0.0;

        let (assign103190_e155061,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103190_e155061;
        locals.var_mm_rv = 0.0;

        let assign103200_e155064: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2351 = assign103200_e155064;
        locals.var_guard2351_rv = 0.0;

        let (assign103210_e155080,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103210_e155080;
        locals.var_mm_rv = 0.0;

        let assign103220_e155083: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2352 = assign103220_e155083;
        locals.var_guard2352_rv = 0.0;

        let (assign103230_e155102,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 == 0.0)) && (locals.var_guard2352 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103230_e155102;
        locals.var_mm_rv = 0.0;

        let assign103240_e155105: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2353 = assign103240_e155105;
        locals.var_guard2353_rv = 0.0;

        let (assign103250_e155127,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 == 0.0)) && (locals.var_guard2352 == 0.0)) && (locals.var_guard2353 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103250_e155127;
        locals.var_mm_rv = 0.0;

        let (assign103260_e155138,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103260_e155138;
        locals.var_m0_rv = 0.0;

        let mut assign103270_loop_guard: usize = 0;
        while {
            let assign103270_cond_e155150: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign103270_cond_e155150 != 0.0
        } {
            assign103270_loop_guard += 1;
            assert!(assign103270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign103270_body0_e155162, assign103270_body0_e155162_d_n0, assign103270_body0_e155162_d_n2, assign103270_body0_e155162_d_n4, assign103270_body0_e155162_d_n5, assign103270_body0_e155162_d_n6, assign103270_body0_e155162_d_n7, assign103270_body0_e155162_d_n8, assign103270_body0_e155162_d_n9, assign103270_body0_e155162_d_n10, assign103270_body0_e155162_d_n11, assign103270_body0_e155162_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) {
        let assign103270_body0_e155160: f64 = (locals.var_dnm).sqrt();
        (assign103270_body0_e155160, (locals.var_dnm_dn0 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn2 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn4 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn5 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn6 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn7 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn8 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn9 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn10 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn11 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn14 / (2.0 * assign103270_body0_e155160)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign103270_body0_e155162;
            locals.var_dnm_dn0 = assign103270_body0_e155162_d_n0;
            locals.var_dnm_dn2 = assign103270_body0_e155162_d_n2;
            locals.var_dnm_dn4 = assign103270_body0_e155162_d_n4;
            locals.var_dnm_dn5 = assign103270_body0_e155162_d_n5;
            locals.var_dnm_dn6 = assign103270_body0_e155162_d_n6;
            locals.var_dnm_dn7 = assign103270_body0_e155162_d_n7;
            locals.var_dnm_dn8 = assign103270_body0_e155162_d_n8;
            locals.var_dnm_dn9 = assign103270_body0_e155162_d_n9;
            locals.var_dnm_dn10 = assign103270_body0_e155162_d_n10;
            locals.var_dnm_dn11 = assign103270_body0_e155162_d_n11;
            locals.var_dnm_dn14 = assign103270_body0_e155162_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign103270_body1_e155175,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) {
        let assign103270_body1_e155173: f64 = (locals.var_m0 + 1.0);
        (assign103270_body1_e155173,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign103270_body1_e155175;
            locals.var_m0_rv = 0.0;
        }

        let (assign103280_e155198, assign103280_e155198_d_n0, assign103280_e155198_d_n2, assign103280_e155198_d_n4, assign103280_e155198_d_n5, assign103280_e155198_d_n6, assign103280_e155198_d_n7, assign103280_e155198_d_n8, assign103280_e155198_d_n9, assign103280_e155198_d_n10, assign103280_e155198_d_n11, assign103280_e155198_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 == 0.0)) {
        let (assign103280_e155196, assign103280_e155196_d_n0, assign103280_e155196_d_n2, assign103280_e155196_d_n4, assign103280_e155196_d_n5, assign103280_e155196_d_n6, assign103280_e155196_d_n7, assign103280_e155196_d_n8, assign103280_e155196_d_n9, assign103280_e155196_d_n10, assign103280_e155196_d_n11, assign103280_e155196_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103280_e155193: f64 = (2.0 * 2.0);
                let assign103280_e155194: f64 = (1.0 / assign103280_e155193);
                let assign103280_e155195: f64 = (locals.var_dnm).powf(assign103280_e155194);
                (assign103280_e155195, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn0)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn2)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn4)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn5)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn6)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn7)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn8)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn9)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn10)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn11)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn14)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign103280_e155196, assign103280_e155196_d_n0, assign103280_e155196_d_n2, assign103280_e155196_d_n4, assign103280_e155196_d_n5, assign103280_e155196_d_n6, assign103280_e155196_d_n7, assign103280_e155196_d_n8, assign103280_e155196_d_n9, assign103280_e155196_d_n10, assign103280_e155196_d_n11, assign103280_e155196_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103280_e155198;
        locals.var_dnm_dn0 = assign103280_e155198_d_n0;
        locals.var_dnm_dn2 = assign103280_e155198_d_n2;
        locals.var_dnm_dn4 = assign103280_e155198_d_n4;
        locals.var_dnm_dn5 = assign103280_e155198_d_n5;
        locals.var_dnm_dn6 = assign103280_e155198_d_n6;
        locals.var_dnm_dn7 = assign103280_e155198_d_n7;
        locals.var_dnm_dn8 = assign103280_e155198_d_n8;
        locals.var_dnm_dn9 = assign103280_e155198_d_n9;
        locals.var_dnm_dn10 = assign103280_e155198_d_n10;
        locals.var_dnm_dn11 = assign103280_e155198_d_n11;
        locals.var_dnm_dn14 = assign103280_e155198_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign103290_e155209, assign103290_e155209_d_n0, assign103290_e155209_d_n2, assign103290_e155209_d_n4, assign103290_e155209_d_n5, assign103290_e155209_d_n6, assign103290_e155209_d_n7, assign103290_e155209_d_n8, assign103290_e155209_d_n9, assign103290_e155209_d_n10, assign103290_e155209_d_n11, assign103290_e155209_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103290_e155207: f64 = (1.0 / locals.var_dnm);
        (assign103290_e155207, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103290_e155209;
        locals.var_dnm_dn0 = assign103290_e155209_d_n0;
        locals.var_dnm_dn2 = assign103290_e155209_d_n2;
        locals.var_dnm_dn4 = assign103290_e155209_d_n4;
        locals.var_dnm_dn5 = assign103290_e155209_d_n5;
        locals.var_dnm_dn6 = assign103290_e155209_d_n6;
        locals.var_dnm_dn7 = assign103290_e155209_d_n7;
        locals.var_dnm_dn8 = assign103290_e155209_d_n8;
        locals.var_dnm_dn9 = assign103290_e155209_d_n9;
        locals.var_dnm_dn10 = assign103290_e155209_d_n10;
        locals.var_dnm_dn11 = assign103290_e155209_d_n11;
        locals.var_dnm_dn14 = assign103290_e155209_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign103300_e155222, assign103300_e155222_d_n0, assign103300_e155222_d_n2, assign103300_e155222_d_n4, assign103300_e155222_d_n5, assign103300_e155222_d_n6, assign103300_e155222_d_n7, assign103300_e155222_d_n8, assign103300_e155222_d_n9, assign103300_e155222_d_n10, assign103300_e155222_d_n11, assign103300_e155222_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103300_e155218: f64 = (locals.var_tmf1 * 1e-25);
        let assign103300_e155220: f64 = (assign103300_e155218 * locals.var_dnm);
        (assign103300_e155220, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign103300_e155222;
        locals.var_tmf0_dn0 = assign103300_e155222_d_n0;
        locals.var_tmf0_dn2 = assign103300_e155222_d_n2;
        locals.var_tmf0_dn4 = assign103300_e155222_d_n4;
        locals.var_tmf0_dn5 = assign103300_e155222_d_n5;
        locals.var_tmf0_dn6 = assign103300_e155222_d_n6;
        locals.var_tmf0_dn7 = assign103300_e155222_d_n7;
        locals.var_tmf0_dn8 = assign103300_e155222_d_n8;
        locals.var_tmf0_dn9 = assign103300_e155222_d_n9;
        locals.var_tmf0_dn10 = assign103300_e155222_d_n10;
        locals.var_tmf0_dn11 = assign103300_e155222_d_n11;
        locals.var_tmf0_dn14 = assign103300_e155222_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign103310_e155237, assign103310_e155237_d_n0, assign103310_e155237_d_n2, assign103310_e155237_d_n4, assign103310_e155237_d_n5, assign103310_e155237_d_n6, assign103310_e155237_d_n7, assign103310_e155237_d_n8, assign103310_e155237_d_n9, assign103310_e155237_d_n10, assign103310_e155237_d_n11, assign103310_e155237_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103310_e155231: f64 = (1e-25 * locals.var_xmp);
        let assign103310_e155233: f64 = (assign103310_e155231 * locals.var_dnm);
        let assign103310_e155235: f64 = (assign103310_e155233 / locals.var_arg);
        (assign103310_e155235, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn0)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn2)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn4)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn5)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn6)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn7)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn8)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn9)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn10)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn11) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn11)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn14) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn14)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103310_e155237;
        locals.var_t0_dn0 = assign103310_e155237_d_n0;
        locals.var_t0_dn2 = assign103310_e155237_d_n2;
        locals.var_t0_dn4 = assign103310_e155237_d_n4;
        locals.var_t0_dn5 = assign103310_e155237_d_n5;
        locals.var_t0_dn6 = assign103310_e155237_d_n6;
        locals.var_t0_dn7 = assign103310_e155237_d_n7;
        locals.var_t0_dn8 = assign103310_e155237_d_n8;
        locals.var_t0_dn9 = assign103310_e155237_d_n9;
        locals.var_t0_dn10 = assign103310_e155237_d_n10;
        locals.var_t0_dn11 = assign103310_e155237_d_n11;
        locals.var_t0_dn14 = assign103310_e155237_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign103320_e155250, assign103320_e155250_d_n0, assign103320_e155250_d_n2, assign103320_e155250_d_n4, assign103320_e155250_d_n5, assign103320_e155250_d_n6, assign103320_e155250_d_n7, assign103320_e155250_d_n8, assign103320_e155250_d_n9, assign103320_e155250_d_n10, assign103320_e155250_d_n11, assign103320_e155250_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103320_e155246: f64 = 1e-25;
        let assign103320_e155248: f64 = (assign103320_e155246 - locals.var_tmf0);
        (assign103320_e155248, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103320_e155250;
        locals.var_gd_s_dn0 = assign103320_e155250_d_n0;
        locals.var_gd_s_dn2 = assign103320_e155250_d_n2;
        locals.var_gd_s_dn4 = assign103320_e155250_d_n4;
        locals.var_gd_s_dn5 = assign103320_e155250_d_n5;
        locals.var_gd_s_dn6 = assign103320_e155250_d_n6;
        locals.var_gd_s_dn7 = assign103320_e155250_d_n7;
        locals.var_gd_s_dn8 = assign103320_e155250_d_n8;
        locals.var_gd_s_dn9 = assign103320_e155250_d_n9;
        locals.var_gd_s_dn10 = assign103320_e155250_d_n10;
        locals.var_gd_s_dn11 = assign103320_e155250_d_n11;
        locals.var_gd_s_dn14 = assign103320_e155250_d_n14;
        locals.var_gd_s_rv = 0.0;

        let (assign103330_e155259, assign103330_e155259_d_n0, assign103330_e155259_d_n2, assign103330_e155259_d_n4, assign103330_e155259_d_n5, assign103330_e155259_d_n6, assign103330_e155259_d_n7, assign103330_e155259_d_n8, assign103330_e155259_d_n9, assign103330_e155259_d_n10, assign103330_e155259_d_n11, assign103330_e155259_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103330_e155259;
        locals.var_t0_dn0 = assign103330_e155259_d_n0;
        locals.var_t0_dn2 = assign103330_e155259_d_n2;
        locals.var_t0_dn4 = assign103330_e155259_d_n4;
        locals.var_t0_dn5 = assign103330_e155259_d_n5;
        locals.var_t0_dn6 = assign103330_e155259_d_n6;
        locals.var_t0_dn7 = assign103330_e155259_d_n7;
        locals.var_t0_dn8 = assign103330_e155259_d_n8;
        locals.var_t0_dn9 = assign103330_e155259_d_n9;
        locals.var_t0_dn10 = assign103330_e155259_d_n10;
        locals.var_t0_dn11 = assign103330_e155259_d_n11;
        locals.var_t0_dn14 = assign103330_e155259_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign103340_e155269, assign103340_e155269_d_n0, assign103340_e155269_d_n2, assign103340_e155269_d_n4, assign103340_e155269_d_n5, assign103340_e155269_d_n6, assign103340_e155269_d_n7, assign103340_e155269_d_n8, assign103340_e155269_d_n9, assign103340_e155269_d_n10, assign103340_e155269_d_n11, assign103340_e155269_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 == 0.0)) {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103340_e155269;
        locals.var_gd_s_dn0 = assign103340_e155269_d_n0;
        locals.var_gd_s_dn2 = assign103340_e155269_d_n2;
        locals.var_gd_s_dn4 = assign103340_e155269_d_n4;
        locals.var_gd_s_dn5 = assign103340_e155269_d_n5;
        locals.var_gd_s_dn6 = assign103340_e155269_d_n6;
        locals.var_gd_s_dn7 = assign103340_e155269_d_n7;
        locals.var_gd_s_dn8 = assign103340_e155269_d_n8;
        locals.var_gd_s_dn9 = assign103340_e155269_d_n9;
        locals.var_gd_s_dn10 = assign103340_e155269_d_n10;
        locals.var_gd_s_dn11 = assign103340_e155269_d_n11;
        locals.var_gd_s_dn14 = assign103340_e155269_d_n14;
        locals.var_gd_s_rv = 0.0;

        let (assign103350_e155279, assign103350_e155279_d_n0, assign103350_e155279_d_n2, assign103350_e155279_d_n4, assign103350_e155279_d_n5, assign103350_e155279_d_n6, assign103350_e155279_d_n7, assign103350_e155279_d_n8, assign103350_e155279_d_n9, assign103350_e155279_d_n10, assign103350_e155279_d_n11, assign103350_e155279_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103350_e155279;
        locals.var_t0_dn0 = assign103350_e155279_d_n0;
        locals.var_t0_dn2 = assign103350_e155279_d_n2;
        locals.var_t0_dn4 = assign103350_e155279_d_n4;
        locals.var_t0_dn5 = assign103350_e155279_d_n5;
        locals.var_t0_dn6 = assign103350_e155279_d_n6;
        locals.var_t0_dn7 = assign103350_e155279_d_n7;
        locals.var_t0_dn8 = assign103350_e155279_d_n8;
        locals.var_t0_dn9 = assign103350_e155279_d_n9;
        locals.var_t0_dn10 = assign103350_e155279_d_n10;
        locals.var_t0_dn11 = assign103350_e155279_d_n11;
        locals.var_t0_dn14 = assign103350_e155279_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_396(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign103360_e155288, assign103360_e155288_d_n0, assign103360_e155288_d_n2, assign103360_e155288_d_n4, assign103360_e155288_d_n5, assign103360_e155288_d_n6, assign103360_e155288_d_n7, assign103360_e155288_d_n8, assign103360_e155288_d_n9, assign103360_e155288_d_n10, assign103360_e155288_d_n11, assign103360_e155288_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103360_e155286: f64 = (1.0 / locals.var_gd_s);
        (assign103360_e155286, (-(locals.var_gd_s_dn0 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn2 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn4 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn5 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn6 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn7 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn8 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn9 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn10 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn11 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn14 / (locals.var_gd_s * locals.var_gd_s))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103360_e155288;
        locals.var_rsd_dn0 = assign103360_e155288_d_n0;
        locals.var_rsd_dn2 = assign103360_e155288_d_n2;
        locals.var_rsd_dn4 = assign103360_e155288_d_n4;
        locals.var_rsd_dn5 = assign103360_e155288_d_n5;
        locals.var_rsd_dn6 = assign103360_e155288_d_n6;
        locals.var_rsd_dn7 = assign103360_e155288_d_n7;
        locals.var_rsd_dn8 = assign103360_e155288_d_n8;
        locals.var_rsd_dn9 = assign103360_e155288_d_n9;
        locals.var_rsd_dn10 = assign103360_e155288_d_n10;
        locals.var_rsd_dn11 = assign103360_e155288_d_n11;
        locals.var_rsd_dn14 = assign103360_e155288_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign103370_e155297, assign103370_e155297_d_n0, assign103370_e155297_d_n2, assign103370_e155297_d_n4, assign103370_e155297_d_n5, assign103370_e155297_d_n6, assign103370_e155297_d_n7, assign103370_e155297_d_n8, assign103370_e155297_d_n9, assign103370_e155297_d_n10, assign103370_e155297_d_n11, assign103370_e155297_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103370_e155295: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign103370_e155295, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn11 / locals.var_weffld_nf), (locals.var_rsd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103370_e155297;
        locals.var_rsd_dn0 = assign103370_e155297_d_n0;
        locals.var_rsd_dn2 = assign103370_e155297_d_n2;
        locals.var_rsd_dn4 = assign103370_e155297_d_n4;
        locals.var_rsd_dn5 = assign103370_e155297_d_n5;
        locals.var_rsd_dn6 = assign103370_e155297_d_n6;
        locals.var_rsd_dn7 = assign103370_e155297_d_n7;
        locals.var_rsd_dn8 = assign103370_e155297_d_n8;
        locals.var_rsd_dn9 = assign103370_e155297_d_n9;
        locals.var_rsd_dn10 = assign103370_e155297_d_n10;
        locals.var_rsd_dn11 = assign103370_e155297_d_n11;
        locals.var_rsd_dn14 = assign103370_e155297_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign103380_e155306, assign103380_e155306_d_n0, assign103380_e155306_d_n2, assign103380_e155306_d_n4, assign103380_e155306_d_n5, assign103380_e155306_d_n6, assign103380_e155306_d_n7, assign103380_e155306_d_n8, assign103380_e155306_d_n9, assign103380_e155306_d_n10, assign103380_e155306_d_n11, assign103380_e155306_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103380_e155304: f64 = (locals.var_rsd + locals.var_rs0);
        (assign103380_e155304, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103380_e155306;
        locals.var_rsd_dn0 = assign103380_e155306_d_n0;
        locals.var_rsd_dn2 = assign103380_e155306_d_n2;
        locals.var_rsd_dn4 = assign103380_e155306_d_n4;
        locals.var_rsd_dn5 = assign103380_e155306_d_n5;
        locals.var_rsd_dn6 = assign103380_e155306_d_n6;
        locals.var_rsd_dn7 = assign103380_e155306_d_n7;
        locals.var_rsd_dn8 = assign103380_e155306_d_n8;
        locals.var_rsd_dn9 = assign103380_e155306_d_n9;
        locals.var_rsd_dn10 = assign103380_e155306_d_n10;
        locals.var_rsd_dn11 = assign103380_e155306_d_n11;
        locals.var_rsd_dn14 = assign103380_e155306_d_n14;
        locals.var_rsd_rv = 0.0;

        let assign103420_e155337: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2355 = assign103420_e155337;
        locals.var_guard2355_rv = 0.0;

        let (assign103430_e155346, assign103430_e155346_d_n0, assign103430_e155346_d_n2, assign103430_e155346_d_n4, assign103430_e155346_d_n5, assign103430_e155346_d_n6, assign103430_e155346_d_n7, assign103430_e155346_d_n8, assign103430_e155346_d_n9, assign103430_e155346_d_n10, assign103430_e155346_d_n11, assign103430_e155346_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2355 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103430_e155346;
        locals.var_rsd_dn0 = assign103430_e155346_d_n0;
        locals.var_rsd_dn2 = assign103430_e155346_d_n2;
        locals.var_rsd_dn4 = assign103430_e155346_d_n4;
        locals.var_rsd_dn5 = assign103430_e155346_d_n5;
        locals.var_rsd_dn6 = assign103430_e155346_d_n6;
        locals.var_rsd_dn7 = assign103430_e155346_d_n7;
        locals.var_rsd_dn8 = assign103430_e155346_d_n8;
        locals.var_rsd_dn9 = assign103430_e155346_d_n9;
        locals.var_rsd_dn10 = assign103430_e155346_d_n10;
        locals.var_rsd_dn11 = assign103430_e155346_d_n11;
        locals.var_rsd_dn14 = assign103430_e155346_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign103440_e155355, assign103440_e155355_d_n0, assign103440_e155355_d_n2, assign103440_e155355_d_n4, assign103440_e155355_d_n5, assign103440_e155355_d_n6, assign103440_e155355_d_n7, assign103440_e155355_d_n8, assign103440_e155355_d_n9, assign103440_e155355_d_n10, assign103440_e155355_d_n11, assign103440_e155355_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103440_e155353: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign103440_e155353, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign103440_e155355;
        locals.var_rsde_dn0 = assign103440_e155355_d_n0;
        locals.var_rsde_dn2 = assign103440_e155355_d_n2;
        locals.var_rsde_dn4 = assign103440_e155355_d_n4;
        locals.var_rsde_dn5 = assign103440_e155355_d_n5;
        locals.var_rsde_dn6 = assign103440_e155355_d_n6;
        locals.var_rsde_dn7 = assign103440_e155355_d_n7;
        locals.var_rsde_dn8 = assign103440_e155355_d_n8;
        locals.var_rsde_dn9 = assign103440_e155355_d_n9;
        locals.var_rsde_dn10 = assign103440_e155355_d_n10;
        locals.var_rsde_dn11 = assign103440_e155355_d_n11;
        locals.var_rsde_dn14 = assign103440_e155355_d_n14;
        locals.var_rsde_rv = 0.0;

        let assign103450_e155358: f64 = if locals.var_flg_rd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2360 = assign103450_e155358;
        locals.var_guard2360_rv = 0.0;

        let (assign103460_e155365, assign103460_e155365_d_n6, assign103460_e155365_d_n8,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_vdsi, locals.var_vdsi_dn6, locals.var_vdsi_dn8,)
    } else {
        (locals.var_vds__blk2356, locals.var_vds__blk2356_dn6, locals.var_vds__blk2356_dn8,)
    }
};
        locals.var_vds__blk2356 = assign103460_e155365;
        locals.var_vds__blk2356_dn6 = assign103460_e155365_d_n6;
        locals.var_vds__blk2356_dn8 = assign103460_e155365_d_n8;
        locals.var_vds__blk2356_rv = 0.0;

        let (assign103470_e155372, assign103470_e155372_d_n8, assign103470_e155372_d_n9,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_vbsi, locals.var_vbsi_dn8, locals.var_vbsi_dn9,)
    } else {
        (locals.var_vbs__blk2357, locals.var_vbs__blk2357_dn8, locals.var_vbs__blk2357_dn9,)
    }
};
        locals.var_vbs__blk2357 = assign103470_e155372;
        locals.var_vbs__blk2357_dn8 = assign103470_e155372_d_n8;
        locals.var_vbs__blk2357_dn9 = assign103470_e155372_d_n9;
        locals.var_vbs__blk2357_rv = 0.0;

        let assign103480_e155379: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2361 = assign103480_e155379;
        locals.var_guard2361_rv = 0.0;

        let (assign103490_e155395, assign103490_e155395_d_n0, assign103490_e155395_d_n2, assign103490_e155395_d_n4, assign103490_e155395_d_n5, assign103490_e155395_d_n6, assign103490_e155395_d_n7, assign103490_e155395_d_n8, assign103490_e155395_d_n9, assign103490_e155395_d_n10, assign103490_e155395_d_n11, assign103490_e155395_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let (assign103490_e155393, assign103490_e155393_d_n0, assign103490_e155393_d_n2, assign103490_e155393_d_n4, assign103490_e155393_d_n5, assign103490_e155393_d_n6, assign103490_e155393_d_n7, assign103490_e155393_d_n8, assign103490_e155393_d_n9, assign103490_e155393_d_n10, assign103490_e155393_d_n11, assign103490_e155393_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103490_e155392: f64 = (locals.var_tratio).powf(p.p415);
                (assign103490_e155392, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn11)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn14)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign103490_e155393, assign103490_e155393_d_n0, assign103490_e155393_d_n2, assign103490_e155393_d_n4, assign103490_e155393_d_n5, assign103490_e155393_d_n6, assign103490_e155393_d_n7, assign103490_e155393_d_n8, assign103490_e155393_d_n9, assign103490_e155393_d_n10, assign103490_e155393_d_n11, assign103490_e155393_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103490_e155395;
        locals.var_t1_dn0 = assign103490_e155395_d_n0;
        locals.var_t1_dn2 = assign103490_e155395_d_n2;
        locals.var_t1_dn4 = assign103490_e155395_d_n4;
        locals.var_t1_dn5 = assign103490_e155395_d_n5;
        locals.var_t1_dn6 = assign103490_e155395_d_n6;
        locals.var_t1_dn7 = assign103490_e155395_d_n7;
        locals.var_t1_dn8 = assign103490_e155395_d_n8;
        locals.var_t1_dn9 = assign103490_e155395_d_n9;
        locals.var_t1_dn10 = assign103490_e155395_d_n10;
        locals.var_t1_dn11 = assign103490_e155395_d_n11;
        locals.var_t1_dn14 = assign103490_e155395_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign103500_e155406, assign103500_e155406_d_n0, assign103500_e155406_d_n2, assign103500_e155406_d_n4, assign103500_e155406_d_n5, assign103500_e155406_d_n6, assign103500_e155406_d_n7, assign103500_e155406_d_n8, assign103500_e155406_d_n9, assign103500_e155406_d_n10, assign103500_e155406_d_n11, assign103500_e155406_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103500_e155404: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign103500_e155404, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn11, locals.var_rrdrmue_dn14,)
    }
};
        locals.var_rrdrmue = assign103500_e155406;
        locals.var_rrdrmue_dn0 = assign103500_e155406_d_n0;
        locals.var_rrdrmue_dn2 = assign103500_e155406_d_n2;
        locals.var_rrdrmue_dn4 = assign103500_e155406_d_n4;
        locals.var_rrdrmue_dn5 = assign103500_e155406_d_n5;
        locals.var_rrdrmue_dn6 = assign103500_e155406_d_n6;
        locals.var_rrdrmue_dn7 = assign103500_e155406_d_n7;
        locals.var_rrdrmue_dn8 = assign103500_e155406_d_n8;
        locals.var_rrdrmue_dn9 = assign103500_e155406_d_n9;
        locals.var_rrdrmue_dn10 = assign103500_e155406_d_n10;
        locals.var_rrdrmue_dn11 = assign103500_e155406_d_n11;
        locals.var_rrdrmue_dn14 = assign103500_e155406_d_n14;
        locals.var_rrdrmue_rv = 0.0;

        let (assign103510_e155431, assign103510_e155431_d_n0, assign103510_e155431_d_n2, assign103510_e155431_d_n4, assign103510_e155431_d_n5, assign103510_e155431_d_n6, assign103510_e155431_d_n7, assign103510_e155431_d_n8, assign103510_e155431_d_n9, assign103510_e155431_d_n10, assign103510_e155431_d_n11, assign103510_e155431_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103510_e155416: f64 = (0.4 * locals.var_tratio);
        let assign103510_e155417: f64 = (1.8 + assign103510_e155416);
        let assign103510_e155420: f64 = (0.1 * locals.var_tratio);
        let assign103510_e155422: f64 = (assign103510_e155420 * locals.var_tratio);
        let assign103510_e155423: f64 = (assign103510_e155417 + assign103510_e155422);
        let assign103510_e155427: f64 = (1.0 - locals.var_tratio);
        let assign103510_e155428: f64 = (p.p417 * assign103510_e155427);
        let assign103510_e155429: f64 = (assign103510_e155423 - assign103510_e155428);
        (assign103510_e155429, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn11))) - (p.p417 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn14))) - (p.p417 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103510_e155431;
        locals.var_t0_dn0 = assign103510_e155431_d_n0;
        locals.var_t0_dn2 = assign103510_e155431_d_n2;
        locals.var_t0_dn4 = assign103510_e155431_d_n4;
        locals.var_t0_dn5 = assign103510_e155431_d_n5;
        locals.var_t0_dn6 = assign103510_e155431_d_n6;
        locals.var_t0_dn7 = assign103510_e155431_d_n7;
        locals.var_t0_dn8 = assign103510_e155431_d_n8;
        locals.var_t0_dn9 = assign103510_e155431_d_n9;
        locals.var_t0_dn10 = assign103510_e155431_d_n10;
        locals.var_t0_dn11 = assign103510_e155431_d_n11;
        locals.var_t0_dn14 = assign103510_e155431_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign103520_e155442, assign103520_e155442_d_n0, assign103520_e155442_d_n2, assign103520_e155442_d_n4, assign103520_e155442_d_n5, assign103520_e155442_d_n6, assign103520_e155442_d_n7, assign103520_e155442_d_n8, assign103520_e155442_d_n9, assign103520_e155442_d_n10, assign103520_e155442_d_n11, assign103520_e155442_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103520_e155440: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign103520_e155440, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn11, locals.var_rrdrvmax_dn14,)
    }
};
        locals.var_rrdrvmax = assign103520_e155442;
        locals.var_rrdrvmax_dn0 = assign103520_e155442_d_n0;
        locals.var_rrdrvmax_dn2 = assign103520_e155442_d_n2;
        locals.var_rrdrvmax_dn4 = assign103520_e155442_d_n4;
        locals.var_rrdrvmax_dn5 = assign103520_e155442_d_n5;
        locals.var_rrdrvmax_dn6 = assign103520_e155442_d_n6;
        locals.var_rrdrvmax_dn7 = assign103520_e155442_d_n7;
        locals.var_rrdrvmax_dn8 = assign103520_e155442_d_n8;
        locals.var_rrdrvmax_dn9 = assign103520_e155442_d_n9;
        locals.var_rrdrvmax_dn10 = assign103520_e155442_d_n10;
        locals.var_rrdrvmax_dn11 = assign103520_e155442_d_n11;
        locals.var_rrdrvmax_dn14 = assign103520_e155442_d_n14;
        locals.var_rrdrvmax_rv = 0.0;

        let (assign103530_e155457, assign103530_e155457_d_n0, assign103530_e155457_d_n2, assign103530_e155457_d_n4, assign103530_e155457_d_n5, assign103530_e155457_d_n6, assign103530_e155457_d_n7, assign103530_e155457_d_n8, assign103530_e155457_d_n9, assign103530_e155457_d_n10, assign103530_e155457_d_n11, assign103530_e155457_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103530_e155453: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign103530_e155454: f64 = (p.p438 * assign103530_e155453);
        let assign103530_e155455: f64 = (locals.var_uc_rdrbb + assign103530_e155454);
        (assign103530_e155455, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn11 + (p.p438 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_dn14 + (p.p438 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign103530_e155457;
        locals.var_uc_rdrbb_dn0 = assign103530_e155457_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103530_e155457_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103530_e155457_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103530_e155457_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103530_e155457_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103530_e155457_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103530_e155457_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103530_e155457_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103530_e155457_d_n10;
        locals.var_uc_rdrbb_dn11 = assign103530_e155457_d_n11;
        locals.var_uc_rdrbb_dn14 = assign103530_e155457_d_n14;
        locals.var_uc_rdrbb_rv = 0.0;

        let assign103550_e155465: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard2363 = assign103550_e155465;
        locals.var_guard2363_rv = 0.0;

        let (assign103560_e155476, assign103560_e155476_d_n0, assign103560_e155476_d_n2, assign103560_e155476_d_n4, assign103560_e155476_d_n5, assign103560_e155476_d_n6, assign103560_e155476_d_n7, assign103560_e155476_d_n8, assign103560_e155476_d_n9, assign103560_e155476_d_n10, assign103560_e155476_d_n11, assign103560_e155476_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) && (locals.var_guard2363 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign103560_e155476;
        locals.var_uc_rdrbb_dn0 = assign103560_e155476_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103560_e155476_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103560_e155476_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103560_e155476_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103560_e155476_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103560_e155476_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103560_e155476_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103560_e155476_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103560_e155476_d_n10;
        locals.var_uc_rdrbb_dn11 = assign103560_e155476_d_n11;
        locals.var_uc_rdrbb_dn14 = assign103560_e155476_d_n14;
        locals.var_uc_rdrbb_rv = 0.0;

        let (assign103570_e155488, assign103570_e155488_d_n0, assign103570_e155488_d_n2, assign103570_e155488_d_n4, assign103570_e155488_d_n5, assign103570_e155488_d_n6, assign103570_e155488_d_n7, assign103570_e155488_d_n8, assign103570_e155488_d_n9, assign103570_e155488_d_n10, assign103570_e155488_d_n11, assign103570_e155488_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 == 0.0)) {
        let assign103570_e155484: f64 = ctx_temp;
        let assign103570_e155486: f64 = (assign103570_e155484 + p.p11);
        (assign103570_e155486, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign103570_e155488;
        locals.var_ttemp_dn0 = assign103570_e155488_d_n0;
        locals.var_ttemp_dn2 = assign103570_e155488_d_n2;
        locals.var_ttemp_dn4 = assign103570_e155488_d_n4;
        locals.var_ttemp_dn5 = assign103570_e155488_d_n5;
        locals.var_ttemp_dn6 = assign103570_e155488_d_n6;
        locals.var_ttemp_dn7 = assign103570_e155488_d_n7;
        locals.var_ttemp_dn8 = assign103570_e155488_d_n8;
        locals.var_ttemp_dn9 = assign103570_e155488_d_n9;
        locals.var_ttemp_dn10 = assign103570_e155488_d_n10;
        locals.var_ttemp_dn11 = assign103570_e155488_d_n11;
        locals.var_ttemp_dn14 = assign103570_e155488_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign103580_e155497,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103580_e155495: f64 = (locals.var_weff_ld * p.p7);
        (assign103580_e155495,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign103580_e155497;
        locals.var_weffld_nf_rv = 0.0;

        let (assign103590_e155506,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103590_e155504: f64 = (p.p67 + p.p68);
        (assign103590_e155504,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign103590_e155506;
        locals.var_ldrifte_rv = 0.0;

        let (assign103600_e155515,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103600_e155513: f64 = (locals.var_uc_xldld + 1e-12);
        (assign103600_e155513,)
    } else {
        (locals.var_rd_xldld,)
    }
};
        locals.var_rd_xldld = assign103600_e155515;
        locals.var_rd_xldld_rv = 0.0;

        let (assign103610_e155522,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_noverd,)
    }
};
        locals.var_noverd = assign103610_e155522;
        locals.var_noverd_rv = 0.0;

        let (assign103620_e155537, assign103620_e155537_d_n0, assign103620_e155537_d_n2, assign103620_e155537_d_n4, assign103620_e155537_d_n5, assign103620_e155537_d_n6, assign103620_e155537_d_n7, assign103620_e155537_d_n8, assign103620_e155537_d_n9, assign103620_e155537_d_n10, assign103620_e155537_d_n11, assign103620_e155537_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103620_e155532: f64 = (p.p411 * locals.var_vbs__blk2357);
        let assign103620_e155533: f64 = (p.p410 - assign103620_e155532);
        let assign103620_e155534: f64 = (locals.var_vbs__blk2357 * assign103620_e155533);
        let assign103620_e155535: f64 = (1.0 + assign103620_e155534);
        (assign103620_e155535, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ((locals.var_vbs__blk2357_dn8 * assign103620_e155533) + (locals.var_vbs__blk2357 * (-(p.p411 * locals.var_vbs__blk2357_dn8)))), ((locals.var_vbs__blk2357_dn9 * assign103620_e155533) + (locals.var_vbs__blk2357 * (-(p.p411 * locals.var_vbs__blk2357_dn9)))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103620_e155537;
        locals.var_t1_dn0 = assign103620_e155537_d_n0;
        locals.var_t1_dn2 = assign103620_e155537_d_n2;
        locals.var_t1_dn4 = assign103620_e155537_d_n4;
        locals.var_t1_dn5 = assign103620_e155537_d_n5;
        locals.var_t1_dn6 = assign103620_e155537_d_n6;
        locals.var_t1_dn7 = assign103620_e155537_d_n7;
        locals.var_t1_dn8 = assign103620_e155537_d_n8;
        locals.var_t1_dn9 = assign103620_e155537_d_n9;
        locals.var_t1_dn10 = assign103620_e155537_d_n10;
        locals.var_t1_dn11 = assign103620_e155537_d_n11;
        locals.var_t1_dn14 = assign103620_e155537_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign103630_e155553, assign103630_e155553_d_n0, assign103630_e155553_d_n2, assign103630_e155553_d_n4, assign103630_e155553_d_n5, assign103630_e155553_d_n6, assign103630_e155553_d_n7, assign103630_e155553_d_n8, assign103630_e155553_d_n9, assign103630_e155553_d_n10, assign103630_e155553_d_n11, assign103630_e155553_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103630_e155544: f64 = (locals.var_t1 * locals.var_t1);
        let assign103630_e155547: f64 = (4.0 * 0.1);
        let assign103630_e155549: f64 = (assign103630_e155547 * 0.1);
        let assign103630_e155550: f64 = (assign103630_e155544 + assign103630_e155549);
        let assign103630_e155551: f64 = (assign103630_e155550).sqrt();
        (assign103630_e155551, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign103630_e155551)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103630_e155553;
        locals.var_tmf2_dn0 = assign103630_e155553_d_n0;
        locals.var_tmf2_dn2 = assign103630_e155553_d_n2;
        locals.var_tmf2_dn4 = assign103630_e155553_d_n4;
        locals.var_tmf2_dn5 = assign103630_e155553_d_n5;
        locals.var_tmf2_dn6 = assign103630_e155553_d_n6;
        locals.var_tmf2_dn7 = assign103630_e155553_d_n7;
        locals.var_tmf2_dn8 = assign103630_e155553_d_n8;
        locals.var_tmf2_dn9 = assign103630_e155553_d_n9;
        locals.var_tmf2_dn10 = assign103630_e155553_d_n10;
        locals.var_tmf2_dn11 = assign103630_e155553_d_n11;
        locals.var_tmf2_dn14 = assign103630_e155553_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign103640_e155566, assign103640_e155566_d_n0, assign103640_e155566_d_n2, assign103640_e155566_d_n4, assign103640_e155566_d_n5, assign103640_e155566_d_n6, assign103640_e155566_d_n7, assign103640_e155566_d_n8, assign103640_e155566_d_n9, assign103640_e155566_d_n10, assign103640_e155566_d_n11, assign103640_e155566_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103640_e155562: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign103640_e155563: f64 = (1.0 + assign103640_e155562);
        let assign103640_e155564: f64 = (0.5 * assign103640_e155563);
        (assign103640_e155564, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103640_e155566;
        locals.var_t2_dn0 = assign103640_e155566_d_n0;
        locals.var_t2_dn2 = assign103640_e155566_d_n2;
        locals.var_t2_dn4 = assign103640_e155566_d_n4;
        locals.var_t2_dn5 = assign103640_e155566_d_n5;
        locals.var_t2_dn6 = assign103640_e155566_d_n6;
        locals.var_t2_dn7 = assign103640_e155566_d_n7;
        locals.var_t2_dn8 = assign103640_e155566_d_n8;
        locals.var_t2_dn9 = assign103640_e155566_d_n9;
        locals.var_t2_dn10 = assign103640_e155566_d_n10;
        locals.var_t2_dn11 = assign103640_e155566_d_n11;
        locals.var_t2_dn14 = assign103640_e155566_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign103650_e155577, assign103650_e155577_d_n0, assign103650_e155577_d_n2, assign103650_e155577_d_n4, assign103650_e155577_d_n5, assign103650_e155577_d_n6, assign103650_e155577_d_n7, assign103650_e155577_d_n8, assign103650_e155577_d_n9, assign103650_e155577_d_n10, assign103650_e155577_d_n11, assign103650_e155577_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103650_e155574: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign103650_e155575: f64 = (0.5 * assign103650_e155574);
        (assign103650_e155575, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign103650_e155577;
        locals.var_rdrmuevbs_dn0 = assign103650_e155577_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103650_e155577_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103650_e155577_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103650_e155577_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103650_e155577_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103650_e155577_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103650_e155577_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103650_e155577_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103650_e155577_d_n10;
        locals.var_rdrmuevbs_dn11 = assign103650_e155577_d_n11;
        locals.var_rdrmuevbs_dn14 = assign103650_e155577_d_n14;
        locals.var_rdrmuevbs_rv = 0.0;

        let assign103660_e155580: f64 = if locals.var_rdrmuevbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2364 = assign103660_e155580;
        locals.var_guard2364_rv = 0.0;

        let (assign103670_e155589, assign103670_e155589_d_n0, assign103670_e155589_d_n2, assign103670_e155589_d_n4, assign103670_e155589_d_n5, assign103670_e155589_d_n6, assign103670_e155589_d_n7, assign103670_e155589_d_n8, assign103670_e155589_d_n9, assign103670_e155589_d_n10, assign103670_e155589_d_n11, assign103670_e155589_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2364 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign103670_e155589;
        locals.var_rdrmuevbs_dn0 = assign103670_e155589_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103670_e155589_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103670_e155589_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103670_e155589_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103670_e155589_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103670_e155589_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103670_e155589_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103670_e155589_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103670_e155589_d_n10;
        locals.var_rdrmuevbs_dn11 = assign103670_e155589_d_n11;
        locals.var_rdrmuevbs_dn14 = assign103670_e155589_d_n14;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign103680_e155598, assign103680_e155598_d_n0, assign103680_e155598_d_n2, assign103680_e155598_d_n4, assign103680_e155598_d_n5, assign103680_e155598_d_n6, assign103680_e155598_d_n7, assign103680_e155598_d_n8, assign103680_e155598_d_n9, assign103680_e155598_d_n10, assign103680_e155598_d_n11, assign103680_e155598_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2364 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103680_e155598;
        locals.var_t2_dn0 = assign103680_e155598_d_n0;
        locals.var_t2_dn2 = assign103680_e155598_d_n2;
        locals.var_t2_dn4 = assign103680_e155598_d_n4;
        locals.var_t2_dn5 = assign103680_e155598_d_n5;
        locals.var_t2_dn6 = assign103680_e155598_d_n6;
        locals.var_t2_dn7 = assign103680_e155598_d_n7;
        locals.var_t2_dn8 = assign103680_e155598_d_n8;
        locals.var_t2_dn9 = assign103680_e155598_d_n9;
        locals.var_t2_dn10 = assign103680_e155598_d_n10;
        locals.var_t2_dn11 = assign103680_e155598_d_n11;
        locals.var_t2_dn14 = assign103680_e155598_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_397(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103690_e155609, assign103690_e155609_d_n0, assign103690_e155609_d_n2, assign103690_e155609_d_n4, assign103690_e155609_d_n5, assign103690_e155609_d_n6, assign103690_e155609_d_n7, assign103690_e155609_d_n8, assign103690_e155609_d_n9, assign103690_e155609_d_n10, assign103690_e155609_d_n11, assign103690_e155609_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103690_e155605: f64 = (locals.var_rrdrmue * locals.var_rdrmuele);
        let assign103690_e155607: f64 = (assign103690_e155605 * locals.var_rdrmuevbs);
        (assign103690_e155607, (((locals.var_rrdrmue_dn0 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn0)), (((locals.var_rrdrmue_dn2 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn2)), (((locals.var_rrdrmue_dn4 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn4)), (((locals.var_rrdrmue_dn5 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn5)), (((locals.var_rrdrmue_dn6 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn6)), (((locals.var_rrdrmue_dn7 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn7)), (((locals.var_rrdrmue_dn8 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn8)), (((locals.var_rrdrmue_dn9 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn9)), (((locals.var_rrdrmue_dn10 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn10)), (((locals.var_rrdrmue_dn11 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn11)), (((locals.var_rrdrmue_dn14 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn14)),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn8, locals.var_mu0_dn9, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn14,)
    }
};
        locals.var_mu0 = assign103690_e155609;
        locals.var_mu0_dn0 = assign103690_e155609_d_n0;
        locals.var_mu0_dn2 = assign103690_e155609_d_n2;
        locals.var_mu0_dn4 = assign103690_e155609_d_n4;
        locals.var_mu0_dn5 = assign103690_e155609_d_n5;
        locals.var_mu0_dn6 = assign103690_e155609_d_n6;
        locals.var_mu0_dn7 = assign103690_e155609_d_n7;
        locals.var_mu0_dn8 = assign103690_e155609_d_n8;
        locals.var_mu0_dn9 = assign103690_e155609_d_n9;
        locals.var_mu0_dn10 = assign103690_e155609_d_n10;
        locals.var_mu0_dn11 = assign103690_e155609_d_n11;
        locals.var_mu0_dn14 = assign103690_e155609_d_n14;
        locals.var_mu0_rv = 0.0;

        let (assign103700_e155622, assign103700_e155622_d_n0, assign103700_e155622_d_n2, assign103700_e155622_d_n4, assign103700_e155622_d_n5, assign103700_e155622_d_n6, assign103700_e155622_d_n7, assign103700_e155622_d_n8, assign103700_e155622_d_n9, assign103700_e155622_d_n10, assign103700_e155622_d_n11, assign103700_e155622_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103700_e155616: f64 = (locals.var_rrdrvmax * locals.var_rdrvmaxwe);
        let assign103700_e155618: f64 = (assign103700_e155616 * locals.var_rdrvmaxle);
        let assign103700_e155620: f64 = (assign103700_e155618 + 1e-25);
        (assign103700_e155620, ((locals.var_rrdrvmax_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn14 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk2359, locals.var_vmaxe__blk2359_dn0, locals.var_vmaxe__blk2359_dn2, locals.var_vmaxe__blk2359_dn4, locals.var_vmaxe__blk2359_dn5, locals.var_vmaxe__blk2359_dn6, locals.var_vmaxe__blk2359_dn7, locals.var_vmaxe__blk2359_dn8, locals.var_vmaxe__blk2359_dn9, locals.var_vmaxe__blk2359_dn10, locals.var_vmaxe__blk2359_dn11, locals.var_vmaxe__blk2359_dn14,)
    }
};
        locals.var_vmaxe__blk2359 = assign103700_e155622;
        locals.var_vmaxe__blk2359_dn0 = assign103700_e155622_d_n0;
        locals.var_vmaxe__blk2359_dn2 = assign103700_e155622_d_n2;
        locals.var_vmaxe__blk2359_dn4 = assign103700_e155622_d_n4;
        locals.var_vmaxe__blk2359_dn5 = assign103700_e155622_d_n5;
        locals.var_vmaxe__blk2359_dn6 = assign103700_e155622_d_n6;
        locals.var_vmaxe__blk2359_dn7 = assign103700_e155622_d_n7;
        locals.var_vmaxe__blk2359_dn8 = assign103700_e155622_d_n8;
        locals.var_vmaxe__blk2359_dn9 = assign103700_e155622_d_n9;
        locals.var_vmaxe__blk2359_dn10 = assign103700_e155622_d_n10;
        locals.var_vmaxe__blk2359_dn11 = assign103700_e155622_d_n11;
        locals.var_vmaxe__blk2359_dn14 = assign103700_e155622_d_n14;
        locals.var_vmaxe__blk2359_rv = 0.0;

        let (assign103710_e155629,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_uc_rdrcx,)
    } else {
        (locals.var_cx,)
    }
};
        locals.var_cx = assign103710_e155629;
        locals.var_cx_rv = 0.0;

        let (assign103720_e155636,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (p.p421,)
    } else {
        (locals.var_car,)
    }
};
        locals.var_car = assign103720_e155636;
        locals.var_car_rv = 0.0;

        let (assign103730_e155645, assign103730_e155645_d_n0, assign103730_e155645_d_n2, assign103730_e155645_d_n4, assign103730_e155645_d_n5, assign103730_e155645_d_n6, assign103730_e155645_d_n7, assign103730_e155645_d_n8, assign103730_e155645_d_n9, assign103730_e155645_d_n10, assign103730_e155645_d_n11, assign103730_e155645_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103730_e155643: f64 = (locals.var_mu0 * 10000.0);
        (assign103730_e155643, (locals.var_mu0_dn0 * 10000.0), (locals.var_mu0_dn2 * 10000.0), (locals.var_mu0_dn4 * 10000.0), (locals.var_mu0_dn5 * 10000.0), (locals.var_mu0_dn6 * 10000.0), (locals.var_mu0_dn7 * 10000.0), (locals.var_mu0_dn8 * 10000.0), (locals.var_mu0_dn9 * 10000.0), (locals.var_mu0_dn10 * 10000.0), (locals.var_mu0_dn11 * 10000.0), (locals.var_mu0_dn14 * 10000.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103730_e155645;
        locals.var_t1_dn0 = assign103730_e155645_d_n0;
        locals.var_t1_dn2 = assign103730_e155645_d_n2;
        locals.var_t1_dn4 = assign103730_e155645_d_n4;
        locals.var_t1_dn5 = assign103730_e155645_d_n5;
        locals.var_t1_dn6 = assign103730_e155645_d_n6;
        locals.var_t1_dn7 = assign103730_e155645_d_n7;
        locals.var_t1_dn8 = assign103730_e155645_d_n8;
        locals.var_t1_dn9 = assign103730_e155645_d_n9;
        locals.var_t1_dn10 = assign103730_e155645_d_n10;
        locals.var_t1_dn11 = assign103730_e155645_d_n11;
        locals.var_t1_dn14 = assign103730_e155645_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign103740_e155654, assign103740_e155654_d_n0, assign103740_e155654_d_n2, assign103740_e155654_d_n4, assign103740_e155654_d_n5, assign103740_e155654_d_n6, assign103740_e155654_d_n7, assign103740_e155654_d_n8, assign103740_e155654_d_n9, assign103740_e155654_d_n10, assign103740_e155654_d_n11, assign103740_e155654_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103740_e155652: f64 = (locals.var_vmaxe__blk2359 * 100.0);
        (assign103740_e155652, (locals.var_vmaxe__blk2359_dn0 * 100.0), (locals.var_vmaxe__blk2359_dn2 * 100.0), (locals.var_vmaxe__blk2359_dn4 * 100.0), (locals.var_vmaxe__blk2359_dn5 * 100.0), (locals.var_vmaxe__blk2359_dn6 * 100.0), (locals.var_vmaxe__blk2359_dn7 * 100.0), (locals.var_vmaxe__blk2359_dn8 * 100.0), (locals.var_vmaxe__blk2359_dn9 * 100.0), (locals.var_vmaxe__blk2359_dn10 * 100.0), (locals.var_vmaxe__blk2359_dn11 * 100.0), (locals.var_vmaxe__blk2359_dn14 * 100.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103740_e155654;
        locals.var_t2_dn0 = assign103740_e155654_d_n0;
        locals.var_t2_dn2 = assign103740_e155654_d_n2;
        locals.var_t2_dn4 = assign103740_e155654_d_n4;
        locals.var_t2_dn5 = assign103740_e155654_d_n5;
        locals.var_t2_dn6 = assign103740_e155654_d_n6;
        locals.var_t2_dn7 = assign103740_e155654_d_n7;
        locals.var_t2_dn8 = assign103740_e155654_d_n8;
        locals.var_t2_dn9 = assign103740_e155654_d_n9;
        locals.var_t2_dn10 = assign103740_e155654_d_n10;
        locals.var_t2_dn11 = assign103740_e155654_d_n11;
        locals.var_t2_dn14 = assign103740_e155654_d_n14;
        locals.var_t2_rv = 0.0;

        let assign103770_e155675: f64 = if locals.var_vddp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2367 = assign103770_e155675;
        locals.var_guard2367_rv = 0.0;

        let (assign103780_e155691, assign103780_e155691_d_n0, assign103780_e155691_d_n2, assign103780_e155691_d_n4, assign103780_e155691_d_n5, assign103780_e155691_d_n6, assign103780_e155691_d_n7, assign103780_e155691_d_n8, assign103780_e155691_d_n9, assign103780_e155691_d_n10, assign103780_e155691_d_n11, assign103780_e155691_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103780_e155684: f64 = (-locals.var_vddp);
        let assign103780_e155686: f64 = (assign103780_e155684 / 2.0);
        let assign103780_e155687: f64 = (2.0 * assign103780_e155686);
        let assign103780_e155689: f64 = (assign103780_e155687 / p.p262);
        (assign103780_e155689, ((2.0 * ((-locals.var_vddp_dn0) / 2.0)) / p.p262), 0.0, 0.0, 0.0, ((2.0 * ((-locals.var_vddp_dn6) / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103780_e155691;
        locals.var_tmf1_dn0 = assign103780_e155691_d_n0;
        locals.var_tmf1_dn2 = assign103780_e155691_d_n2;
        locals.var_tmf1_dn4 = assign103780_e155691_d_n4;
        locals.var_tmf1_dn5 = assign103780_e155691_d_n5;
        locals.var_tmf1_dn6 = assign103780_e155691_d_n6;
        locals.var_tmf1_dn7 = assign103780_e155691_d_n7;
        locals.var_tmf1_dn8 = assign103780_e155691_d_n8;
        locals.var_tmf1_dn9 = assign103780_e155691_d_n9;
        locals.var_tmf1_dn10 = assign103780_e155691_d_n10;
        locals.var_tmf1_dn11 = assign103780_e155691_d_n11;
        locals.var_tmf1_dn14 = assign103780_e155691_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign103790_e155736, assign103790_e155736_d_n0, assign103790_e155736_d_n2, assign103790_e155736_d_n4, assign103790_e155736_d_n5, assign103790_e155736_d_n6, assign103790_e155736_d_n7, assign103790_e155736_d_n8, assign103790_e155736_d_n9, assign103790_e155736_d_n10, assign103790_e155736_d_n11, assign103790_e155736_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103790_e155702: f64 = (1.0 / 2.0);
        let assign103790_e155706: f64 = (1.0 / 6.0);
        let assign103790_e155710: f64 = (1.0 / 24.0);
        let assign103790_e155714: f64 = (1.0 / 120.0);
        let assign103790_e155718: f64 = (1.0 / 720.0);
        let assign103790_e155722: f64 = (1.0 / 5040.0);
        let assign103790_e155723: f64 = (locals.var_tmf1 * assign103790_e155722);
        let assign103790_e155724: f64 = (assign103790_e155718 + assign103790_e155723);
        let assign103790_e155725: f64 = (locals.var_tmf1 * assign103790_e155724);
        let assign103790_e155726: f64 = (assign103790_e155714 + assign103790_e155725);
        let assign103790_e155727: f64 = (locals.var_tmf1 * assign103790_e155726);
        let assign103790_e155728: f64 = (assign103790_e155710 + assign103790_e155727);
        let assign103790_e155729: f64 = (locals.var_tmf1 * assign103790_e155728);
        let assign103790_e155730: f64 = (assign103790_e155706 + assign103790_e155729);
        let assign103790_e155731: f64 = (locals.var_tmf1 * assign103790_e155730);
        let assign103790_e155732: f64 = (assign103790_e155702 + assign103790_e155731);
        let assign103790_e155733: f64 = (locals.var_tmf1 * assign103790_e155732);
        let assign103790_e155734: f64 = (1.0 + assign103790_e155733);
        (assign103790_e155734, ((locals.var_tmf1_dn0 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn2 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn4 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn5 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn6 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn7 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn8 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn9 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn10 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn11 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn14 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103790_e155722))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103790_e155736;
        locals.var_tmf2_dn0 = assign103790_e155736_d_n0;
        locals.var_tmf2_dn2 = assign103790_e155736_d_n2;
        locals.var_tmf2_dn4 = assign103790_e155736_d_n4;
        locals.var_tmf2_dn5 = assign103790_e155736_d_n5;
        locals.var_tmf2_dn6 = assign103790_e155736_d_n6;
        locals.var_tmf2_dn7 = assign103790_e155736_d_n7;
        locals.var_tmf2_dn8 = assign103790_e155736_d_n8;
        locals.var_tmf2_dn9 = assign103790_e155736_d_n9;
        locals.var_tmf2_dn10 = assign103790_e155736_d_n10;
        locals.var_tmf2_dn11 = assign103790_e155736_d_n11;
        locals.var_tmf2_dn14 = assign103790_e155736_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign103800_e155777, assign103800_e155777_d_n0, assign103800_e155777_d_n2, assign103800_e155777_d_n4, assign103800_e155777_d_n5, assign103800_e155777_d_n6, assign103800_e155777_d_n7, assign103800_e155777_d_n8, assign103800_e155777_d_n9, assign103800_e155777_d_n10, assign103800_e155777_d_n11, assign103800_e155777_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103800_e155745: f64 = (1.0 / 2.0);
        let assign103800_e155749: f64 = (1.0 / 3.0);
        let assign103800_e155753: f64 = (1.0 / 8.0);
        let assign103800_e155757: f64 = (1.0 / 30.0);
        let assign103800_e155761: f64 = (1.0 / 144.0);
        let assign103800_e155765: f64 = (1.0 / 840.0);
        let assign103800_e155766: f64 = (locals.var_tmf1 * assign103800_e155765);
        let assign103800_e155767: f64 = (assign103800_e155761 + assign103800_e155766);
        let assign103800_e155768: f64 = (locals.var_tmf1 * assign103800_e155767);
        let assign103800_e155769: f64 = (assign103800_e155757 + assign103800_e155768);
        let assign103800_e155770: f64 = (locals.var_tmf1 * assign103800_e155769);
        let assign103800_e155771: f64 = (assign103800_e155753 + assign103800_e155770);
        let assign103800_e155772: f64 = (locals.var_tmf1 * assign103800_e155771);
        let assign103800_e155773: f64 = (assign103800_e155749 + assign103800_e155772);
        let assign103800_e155774: f64 = (locals.var_tmf1 * assign103800_e155773);
        let assign103800_e155775: f64 = (assign103800_e155745 + assign103800_e155774);
        (assign103800_e155775, ((locals.var_tmf1_dn0 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103800_e155765))))))))), ((locals.var_tmf1_dn2 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103800_e155765))))))))), ((locals.var_tmf1_dn4 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103800_e155765))))))))), ((locals.var_tmf1_dn5 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103800_e155765))))))))), ((locals.var_tmf1_dn6 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103800_e155765))))))))), ((locals.var_tmf1_dn7 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103800_e155765))))))))), ((locals.var_tmf1_dn8 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103800_e155765))))))))), ((locals.var_tmf1_dn9 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103800_e155765))))))))), ((locals.var_tmf1_dn10 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103800_e155765))))))))), ((locals.var_tmf1_dn11 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103800_e155765))))))))), ((locals.var_tmf1_dn14 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103800_e155765))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign103800_e155777;
        locals.var_tmf3_dn0 = assign103800_e155777_d_n0;
        locals.var_tmf3_dn2 = assign103800_e155777_d_n2;
        locals.var_tmf3_dn4 = assign103800_e155777_d_n4;
        locals.var_tmf3_dn5 = assign103800_e155777_d_n5;
        locals.var_tmf3_dn6 = assign103800_e155777_d_n6;
        locals.var_tmf3_dn7 = assign103800_e155777_d_n7;
        locals.var_tmf3_dn8 = assign103800_e155777_d_n8;
        locals.var_tmf3_dn9 = assign103800_e155777_d_n9;
        locals.var_tmf3_dn10 = assign103800_e155777_d_n10;
        locals.var_tmf3_dn11 = assign103800_e155777_d_n11;
        locals.var_tmf3_dn14 = assign103800_e155777_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign103810_e155788, assign103810_e155788_d_n0, assign103810_e155788_d_n2, assign103810_e155788_d_n4, assign103810_e155788_d_n5, assign103810_e155788_d_n6, assign103810_e155788_d_n7, assign103810_e155788_d_n8, assign103810_e155788_d_n9, assign103810_e155788_d_n10, assign103810_e155788_d_n11, assign103810_e155788_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103810_e155786: f64 = (p.p262 / locals.var_tmf2);
        (assign103810_e155786, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103810_e155788;
        locals.var_vzadd_dn0 = assign103810_e155788_d_n0;
        locals.var_vzadd_dn2 = assign103810_e155788_d_n2;
        locals.var_vzadd_dn4 = assign103810_e155788_d_n4;
        locals.var_vzadd_dn5 = assign103810_e155788_d_n5;
        locals.var_vzadd_dn6 = assign103810_e155788_d_n6;
        locals.var_vzadd_dn7 = assign103810_e155788_d_n7;
        locals.var_vzadd_dn8 = assign103810_e155788_d_n8;
        locals.var_vzadd_dn9 = assign103810_e155788_d_n9;
        locals.var_vzadd_dn10 = assign103810_e155788_d_n10;
        locals.var_vzadd_dn11 = assign103810_e155788_d_n11;
        locals.var_vzadd_dn14 = assign103810_e155788_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign103820_e155804, assign103820_e155804_d_n0, assign103820_e155804_d_n2, assign103820_e155804_d_n4, assign103820_e155804_d_n5, assign103820_e155804_d_n6, assign103820_e155804_d_n7, assign103820_e155804_d_n8, assign103820_e155804_d_n9, assign103820_e155804_d_n10, assign103820_e155804_d_n11, assign103820_e155804_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103820_e155796: f64 = (-2.0);
        let assign103820_e155798: f64 = (assign103820_e155796 * locals.var_tmf3);
        let assign103820_e155801: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103820_e155802: f64 = (assign103820_e155798 / assign103820_e155801);
        (assign103820_e155802, ((((assign103820_e155796 * locals.var_tmf3_dn0) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn2) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn4) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn5) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn6) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn7) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn8) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn9) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn10) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn11) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn14) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign103820_e155801 * assign103820_e155801)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103820_e155804;
        locals.var_t2_dn0 = assign103820_e155804_d_n0;
        locals.var_t2_dn2 = assign103820_e155804_d_n2;
        locals.var_t2_dn4 = assign103820_e155804_d_n4;
        locals.var_t2_dn5 = assign103820_e155804_d_n5;
        locals.var_t2_dn6 = assign103820_e155804_d_n6;
        locals.var_t2_dn7 = assign103820_e155804_d_n7;
        locals.var_t2_dn8 = assign103820_e155804_d_n8;
        locals.var_t2_dn9 = assign103820_e155804_d_n9;
        locals.var_t2_dn10 = assign103820_e155804_d_n10;
        locals.var_t2_dn11 = assign103820_e155804_d_n11;
        locals.var_t2_dn14 = assign103820_e155804_d_n14;
        locals.var_t2_rv = 0.0;

        let assign103830_e155807: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2368 = assign103830_e155807;
        locals.var_guard2368_rv = 0.0;

        let (assign103840_e155818, assign103840_e155818_d_n0, assign103840_e155818_d_n2, assign103840_e155818_d_n4, assign103840_e155818_d_n5, assign103840_e155818_d_n6, assign103840_e155818_d_n7, assign103840_e155818_d_n8, assign103840_e155818_d_n9, assign103840_e155818_d_n10, assign103840_e155818_d_n11, assign103840_e155818_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) && (locals.var_guard2368 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103840_e155818;
        locals.var_vzadd_dn0 = assign103840_e155818_d_n0;
        locals.var_vzadd_dn2 = assign103840_e155818_d_n2;
        locals.var_vzadd_dn4 = assign103840_e155818_d_n4;
        locals.var_vzadd_dn5 = assign103840_e155818_d_n5;
        locals.var_vzadd_dn6 = assign103840_e155818_d_n6;
        locals.var_vzadd_dn7 = assign103840_e155818_d_n7;
        locals.var_vzadd_dn8 = assign103840_e155818_d_n8;
        locals.var_vzadd_dn9 = assign103840_e155818_d_n9;
        locals.var_vzadd_dn10 = assign103840_e155818_d_n10;
        locals.var_vzadd_dn11 = assign103840_e155818_d_n11;
        locals.var_vzadd_dn14 = assign103840_e155818_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign103850_e155831, assign103850_e155831_d_n0, assign103850_e155831_d_n2, assign103850_e155831_d_n4, assign103850_e155831_d_n5, assign103850_e155831_d_n6, assign103850_e155831_d_n7, assign103850_e155831_d_n8, assign103850_e155831_d_n9, assign103850_e155831_d_n10, assign103850_e155831_d_n11, assign103850_e155831_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103850_e155828: f64 = (2.0 * locals.var_vzadd);
        let assign103850_e155829: f64 = (locals.var_vddp - assign103850_e155828);
        (assign103850_e155829, (locals.var_vddp_dn0 - (2.0 * locals.var_vzadd_dn0)), (-(2.0 * locals.var_vzadd_dn2)), (-(2.0 * locals.var_vzadd_dn4)), (-(2.0 * locals.var_vzadd_dn5)), (locals.var_vddp_dn6 - (2.0 * locals.var_vzadd_dn6)), (-(2.0 * locals.var_vzadd_dn7)), (-(2.0 * locals.var_vzadd_dn8)), (-(2.0 * locals.var_vzadd_dn9)), (-(2.0 * locals.var_vzadd_dn10)), (-(2.0 * locals.var_vzadd_dn11)), (-(2.0 * locals.var_vzadd_dn14)),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn11, locals.var_vddpz_dn14,)
    }
};
        locals.var_vddpz = assign103850_e155831;
        locals.var_vddpz_dn0 = assign103850_e155831_d_n0;
        locals.var_vddpz_dn2 = assign103850_e155831_d_n2;
        locals.var_vddpz_dn4 = assign103850_e155831_d_n4;
        locals.var_vddpz_dn5 = assign103850_e155831_d_n5;
        locals.var_vddpz_dn6 = assign103850_e155831_d_n6;
        locals.var_vddpz_dn7 = assign103850_e155831_d_n7;
        locals.var_vddpz_dn8 = assign103850_e155831_d_n8;
        locals.var_vddpz_dn9 = assign103850_e155831_d_n9;
        locals.var_vddpz_dn10 = assign103850_e155831_d_n10;
        locals.var_vddpz_dn11 = assign103850_e155831_d_n11;
        locals.var_vddpz_dn14 = assign103850_e155831_d_n14;
        locals.var_vddpz_rv = 0.0;

        let (assign103860_e155847, assign103860_e155847_d_n0, assign103860_e155847_d_n2, assign103860_e155847_d_n4, assign103860_e155847_d_n5, assign103860_e155847_d_n6, assign103860_e155847_d_n7, assign103860_e155847_d_n8, assign103860_e155847_d_n9, assign103860_e155847_d_n10, assign103860_e155847_d_n11, assign103860_e155847_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103860_e155842: f64 = (locals.var_vddp / 2.0);
        let assign103860_e155843: f64 = (2.0 * assign103860_e155842);
        let assign103860_e155845: f64 = (assign103860_e155843 / p.p262);
        (assign103860_e155845, ((2.0 * (locals.var_vddp_dn0 / 2.0)) / p.p262), 0.0, 0.0, 0.0, ((2.0 * (locals.var_vddp_dn6 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103860_e155847;
        locals.var_tmf1_dn0 = assign103860_e155847_d_n0;
        locals.var_tmf1_dn2 = assign103860_e155847_d_n2;
        locals.var_tmf1_dn4 = assign103860_e155847_d_n4;
        locals.var_tmf1_dn5 = assign103860_e155847_d_n5;
        locals.var_tmf1_dn6 = assign103860_e155847_d_n6;
        locals.var_tmf1_dn7 = assign103860_e155847_d_n7;
        locals.var_tmf1_dn8 = assign103860_e155847_d_n8;
        locals.var_tmf1_dn9 = assign103860_e155847_d_n9;
        locals.var_tmf1_dn10 = assign103860_e155847_d_n10;
        locals.var_tmf1_dn11 = assign103860_e155847_d_n11;
        locals.var_tmf1_dn14 = assign103860_e155847_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign103870_e155893, assign103870_e155893_d_n0, assign103870_e155893_d_n2, assign103870_e155893_d_n4, assign103870_e155893_d_n5, assign103870_e155893_d_n6, assign103870_e155893_d_n7, assign103870_e155893_d_n8, assign103870_e155893_d_n9, assign103870_e155893_d_n10, assign103870_e155893_d_n11, assign103870_e155893_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103870_e155859: f64 = (1.0 / 2.0);
        let assign103870_e155863: f64 = (1.0 / 6.0);
        let assign103870_e155867: f64 = (1.0 / 24.0);
        let assign103870_e155871: f64 = (1.0 / 120.0);
        let assign103870_e155875: f64 = (1.0 / 720.0);
        let assign103870_e155879: f64 = (1.0 / 5040.0);
        let assign103870_e155880: f64 = (locals.var_tmf1 * assign103870_e155879);
        let assign103870_e155881: f64 = (assign103870_e155875 + assign103870_e155880);
        let assign103870_e155882: f64 = (locals.var_tmf1 * assign103870_e155881);
        let assign103870_e155883: f64 = (assign103870_e155871 + assign103870_e155882);
        let assign103870_e155884: f64 = (locals.var_tmf1 * assign103870_e155883);
        let assign103870_e155885: f64 = (assign103870_e155867 + assign103870_e155884);
        let assign103870_e155886: f64 = (locals.var_tmf1 * assign103870_e155885);
        let assign103870_e155887: f64 = (assign103870_e155863 + assign103870_e155886);
        let assign103870_e155888: f64 = (locals.var_tmf1 * assign103870_e155887);
        let assign103870_e155889: f64 = (assign103870_e155859 + assign103870_e155888);
        let assign103870_e155890: f64 = (locals.var_tmf1 * assign103870_e155889);
        let assign103870_e155891: f64 = (1.0 + assign103870_e155890);
        (assign103870_e155891, ((locals.var_tmf1_dn0 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn2 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn4 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn5 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn6 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn7 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn8 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn9 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn10 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn11 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn14 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103870_e155879))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103870_e155893;
        locals.var_tmf2_dn0 = assign103870_e155893_d_n0;
        locals.var_tmf2_dn2 = assign103870_e155893_d_n2;
        locals.var_tmf2_dn4 = assign103870_e155893_d_n4;
        locals.var_tmf2_dn5 = assign103870_e155893_d_n5;
        locals.var_tmf2_dn6 = assign103870_e155893_d_n6;
        locals.var_tmf2_dn7 = assign103870_e155893_d_n7;
        locals.var_tmf2_dn8 = assign103870_e155893_d_n8;
        locals.var_tmf2_dn9 = assign103870_e155893_d_n9;
        locals.var_tmf2_dn10 = assign103870_e155893_d_n10;
        locals.var_tmf2_dn11 = assign103870_e155893_d_n11;
        locals.var_tmf2_dn14 = assign103870_e155893_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign103880_e155935, assign103880_e155935_d_n0, assign103880_e155935_d_n2, assign103880_e155935_d_n4, assign103880_e155935_d_n5, assign103880_e155935_d_n6, assign103880_e155935_d_n7, assign103880_e155935_d_n8, assign103880_e155935_d_n9, assign103880_e155935_d_n10, assign103880_e155935_d_n11, assign103880_e155935_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103880_e155903: f64 = (1.0 / 2.0);
        let assign103880_e155907: f64 = (1.0 / 3.0);
        let assign103880_e155911: f64 = (1.0 / 8.0);
        let assign103880_e155915: f64 = (1.0 / 30.0);
        let assign103880_e155919: f64 = (1.0 / 144.0);
        let assign103880_e155923: f64 = (1.0 / 840.0);
        let assign103880_e155924: f64 = (locals.var_tmf1 * assign103880_e155923);
        let assign103880_e155925: f64 = (assign103880_e155919 + assign103880_e155924);
        let assign103880_e155926: f64 = (locals.var_tmf1 * assign103880_e155925);
        let assign103880_e155927: f64 = (assign103880_e155915 + assign103880_e155926);
        let assign103880_e155928: f64 = (locals.var_tmf1 * assign103880_e155927);
        let assign103880_e155929: f64 = (assign103880_e155911 + assign103880_e155928);
        let assign103880_e155930: f64 = (locals.var_tmf1 * assign103880_e155929);
        let assign103880_e155931: f64 = (assign103880_e155907 + assign103880_e155930);
        let assign103880_e155932: f64 = (locals.var_tmf1 * assign103880_e155931);
        let assign103880_e155933: f64 = (assign103880_e155903 + assign103880_e155932);
        (assign103880_e155933, ((locals.var_tmf1_dn0 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103880_e155923))))))))), ((locals.var_tmf1_dn2 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103880_e155923))))))))), ((locals.var_tmf1_dn4 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103880_e155923))))))))), ((locals.var_tmf1_dn5 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103880_e155923))))))))), ((locals.var_tmf1_dn6 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103880_e155923))))))))), ((locals.var_tmf1_dn7 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103880_e155923))))))))), ((locals.var_tmf1_dn8 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103880_e155923))))))))), ((locals.var_tmf1_dn9 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103880_e155923))))))))), ((locals.var_tmf1_dn10 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103880_e155923))))))))), ((locals.var_tmf1_dn11 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103880_e155923))))))))), ((locals.var_tmf1_dn14 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103880_e155923))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign103880_e155935;
        locals.var_tmf3_dn0 = assign103880_e155935_d_n0;
        locals.var_tmf3_dn2 = assign103880_e155935_d_n2;
        locals.var_tmf3_dn4 = assign103880_e155935_d_n4;
        locals.var_tmf3_dn5 = assign103880_e155935_d_n5;
        locals.var_tmf3_dn6 = assign103880_e155935_d_n6;
        locals.var_tmf3_dn7 = assign103880_e155935_d_n7;
        locals.var_tmf3_dn8 = assign103880_e155935_d_n8;
        locals.var_tmf3_dn9 = assign103880_e155935_d_n9;
        locals.var_tmf3_dn10 = assign103880_e155935_d_n10;
        locals.var_tmf3_dn11 = assign103880_e155935_d_n11;
        locals.var_tmf3_dn14 = assign103880_e155935_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign103890_e155947, assign103890_e155947_d_n0, assign103890_e155947_d_n2, assign103890_e155947_d_n4, assign103890_e155947_d_n5, assign103890_e155947_d_n6, assign103890_e155947_d_n7, assign103890_e155947_d_n8, assign103890_e155947_d_n9, assign103890_e155947_d_n10, assign103890_e155947_d_n11, assign103890_e155947_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103890_e155945: f64 = (p.p262 / locals.var_tmf2);
        (assign103890_e155945, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103890_e155947;
        locals.var_vzadd_dn0 = assign103890_e155947_d_n0;
        locals.var_vzadd_dn2 = assign103890_e155947_d_n2;
        locals.var_vzadd_dn4 = assign103890_e155947_d_n4;
        locals.var_vzadd_dn5 = assign103890_e155947_d_n5;
        locals.var_vzadd_dn6 = assign103890_e155947_d_n6;
        locals.var_vzadd_dn7 = assign103890_e155947_d_n7;
        locals.var_vzadd_dn8 = assign103890_e155947_d_n8;
        locals.var_vzadd_dn9 = assign103890_e155947_d_n9;
        locals.var_vzadd_dn10 = assign103890_e155947_d_n10;
        locals.var_vzadd_dn11 = assign103890_e155947_d_n11;
        locals.var_vzadd_dn14 = assign103890_e155947_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign103900_e155964, assign103900_e155964_d_n0, assign103900_e155964_d_n2, assign103900_e155964_d_n4, assign103900_e155964_d_n5, assign103900_e155964_d_n6, assign103900_e155964_d_n7, assign103900_e155964_d_n8, assign103900_e155964_d_n9, assign103900_e155964_d_n10, assign103900_e155964_d_n11, assign103900_e155964_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103900_e155956: f64 = (-2.0);
        let assign103900_e155958: f64 = (assign103900_e155956 * locals.var_tmf3);
        let assign103900_e155961: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103900_e155962: f64 = (assign103900_e155958 / assign103900_e155961);
        (assign103900_e155962, ((((assign103900_e155956 * locals.var_tmf3_dn0) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn2) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn4) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn5) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn6) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn7) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn8) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn9) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn10) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn11) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn14) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign103900_e155961 * assign103900_e155961)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103900_e155964;
        locals.var_t2_dn0 = assign103900_e155964_d_n0;
        locals.var_t2_dn2 = assign103900_e155964_d_n2;
        locals.var_t2_dn4 = assign103900_e155964_d_n4;
        locals.var_t2_dn5 = assign103900_e155964_d_n5;
        locals.var_t2_dn6 = assign103900_e155964_d_n6;
        locals.var_t2_dn7 = assign103900_e155964_d_n7;
        locals.var_t2_dn8 = assign103900_e155964_d_n8;
        locals.var_t2_dn9 = assign103900_e155964_d_n9;
        locals.var_t2_dn10 = assign103900_e155964_d_n10;
        locals.var_t2_dn11 = assign103900_e155964_d_n11;
        locals.var_t2_dn14 = assign103900_e155964_d_n14;
        locals.var_t2_rv = 0.0;

        let assign103910_e155967: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2369 = assign103910_e155967;
        locals.var_guard2369_rv = 0.0;

        let (assign103920_e155979, assign103920_e155979_d_n0, assign103920_e155979_d_n2, assign103920_e155979_d_n4, assign103920_e155979_d_n5, assign103920_e155979_d_n6, assign103920_e155979_d_n7, assign103920_e155979_d_n8, assign103920_e155979_d_n9, assign103920_e155979_d_n10, assign103920_e155979_d_n11, assign103920_e155979_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) && (locals.var_guard2369 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103920_e155979;
        locals.var_vzadd_dn0 = assign103920_e155979_d_n0;
        locals.var_vzadd_dn2 = assign103920_e155979_d_n2;
        locals.var_vzadd_dn4 = assign103920_e155979_d_n4;
        locals.var_vzadd_dn5 = assign103920_e155979_d_n5;
        locals.var_vzadd_dn6 = assign103920_e155979_d_n6;
        locals.var_vzadd_dn7 = assign103920_e155979_d_n7;
        locals.var_vzadd_dn8 = assign103920_e155979_d_n8;
        locals.var_vzadd_dn9 = assign103920_e155979_d_n9;
        locals.var_vzadd_dn10 = assign103920_e155979_d_n10;
        locals.var_vzadd_dn11 = assign103920_e155979_d_n11;
        locals.var_vzadd_dn14 = assign103920_e155979_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign103930_e155993, assign103930_e155993_d_n0, assign103930_e155993_d_n2, assign103930_e155993_d_n4, assign103930_e155993_d_n5, assign103930_e155993_d_n6, assign103930_e155993_d_n7, assign103930_e155993_d_n8, assign103930_e155993_d_n9, assign103930_e155993_d_n10, assign103930_e155993_d_n11, assign103930_e155993_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103930_e155990: f64 = (2.0 * locals.var_vzadd);
        let assign103930_e155991: f64 = (locals.var_vddp + assign103930_e155990);
        (assign103930_e155991, (locals.var_vddp_dn0 + (2.0 * locals.var_vzadd_dn0)), (2.0 * locals.var_vzadd_dn2), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (locals.var_vddp_dn6 + (2.0 * locals.var_vzadd_dn6)), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn11, locals.var_vddpz_dn14,)
    }
};
        locals.var_vddpz = assign103930_e155993;
        locals.var_vddpz_dn0 = assign103930_e155993_d_n0;
        locals.var_vddpz_dn2 = assign103930_e155993_d_n2;
        locals.var_vddpz_dn4 = assign103930_e155993_d_n4;
        locals.var_vddpz_dn5 = assign103930_e155993_d_n5;
        locals.var_vddpz_dn6 = assign103930_e155993_d_n6;
        locals.var_vddpz_dn7 = assign103930_e155993_d_n7;
        locals.var_vddpz_dn8 = assign103930_e155993_d_n8;
        locals.var_vddpz_dn9 = assign103930_e155993_d_n9;
        locals.var_vddpz_dn10 = assign103930_e155993_d_n10;
        locals.var_vddpz_dn11 = assign103930_e155993_d_n11;
        locals.var_vddpz_dn14 = assign103930_e155993_d_n14;
        locals.var_vddpz_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_398(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103940_e156002, assign103940_e156002_d_n0, assign103940_e156002_d_n2, assign103940_e156002_d_n4, assign103940_e156002_d_n5, assign103940_e156002_d_n6, assign103940_e156002_d_n7, assign103940_e156002_d_n8, assign103940_e156002_d_n9, assign103940_e156002_d_n10, assign103940_e156002_d_n11, assign103940_e156002_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103940_e156000: f64 = (locals.var_vddpz / locals.var_ldrifte);
        (assign103940_e156000, (locals.var_vddpz_dn0 / locals.var_ldrifte), (locals.var_vddpz_dn2 / locals.var_ldrifte), (locals.var_vddpz_dn4 / locals.var_ldrifte), (locals.var_vddpz_dn5 / locals.var_ldrifte), (locals.var_vddpz_dn6 / locals.var_ldrifte), (locals.var_vddpz_dn7 / locals.var_ldrifte), (locals.var_vddpz_dn8 / locals.var_ldrifte), (locals.var_vddpz_dn9 / locals.var_ldrifte), (locals.var_vddpz_dn10 / locals.var_ldrifte), (locals.var_vddpz_dn11 / locals.var_ldrifte), (locals.var_vddpz_dn14 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn4, locals.var_edri_dn5, locals.var_edri_dn6, locals.var_edri_dn7, locals.var_edri_dn8, locals.var_edri_dn9, locals.var_edri_dn10, locals.var_edri_dn11, locals.var_edri_dn14,)
    }
};
        locals.var_edri = assign103940_e156002;
        locals.var_edri_dn0 = assign103940_e156002_d_n0;
        locals.var_edri_dn2 = assign103940_e156002_d_n2;
        locals.var_edri_dn4 = assign103940_e156002_d_n4;
        locals.var_edri_dn5 = assign103940_e156002_d_n5;
        locals.var_edri_dn6 = assign103940_e156002_d_n6;
        locals.var_edri_dn7 = assign103940_e156002_d_n7;
        locals.var_edri_dn8 = assign103940_e156002_d_n8;
        locals.var_edri_dn9 = assign103940_e156002_d_n9;
        locals.var_edri_dn10 = assign103940_e156002_d_n10;
        locals.var_edri_dn11 = assign103940_e156002_d_n11;
        locals.var_edri_dn14 = assign103940_e156002_d_n14;
        locals.var_edri_rv = 0.0;

        let (assign103950_e156011, assign103950_e156011_d_n0, assign103950_e156011_d_n2, assign103950_e156011_d_n4, assign103950_e156011_d_n5, assign103950_e156011_d_n6, assign103950_e156011_d_n7, assign103950_e156011_d_n8, assign103950_e156011_d_n9, assign103950_e156011_d_n10, assign103950_e156011_d_n11, assign103950_e156011_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103950_e156009: f64 = (locals.var_mu0 * locals.var_edri);
        (assign103950_e156009, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn4 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn4)), ((locals.var_mu0_dn5 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn5)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), ((locals.var_mu0_dn8 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn8)), ((locals.var_mu0_dn9 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn9)), ((locals.var_mu0_dn10 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn10)), ((locals.var_mu0_dn11 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn11)), ((locals.var_mu0_dn14 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn14)),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn8, locals.var_vdri_dn9, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn14,)
    }
};
        locals.var_vdri = assign103950_e156011;
        locals.var_vdri_dn0 = assign103950_e156011_d_n0;
        locals.var_vdri_dn2 = assign103950_e156011_d_n2;
        locals.var_vdri_dn4 = assign103950_e156011_d_n4;
        locals.var_vdri_dn5 = assign103950_e156011_d_n5;
        locals.var_vdri_dn6 = assign103950_e156011_d_n6;
        locals.var_vdri_dn7 = assign103950_e156011_d_n7;
        locals.var_vdri_dn8 = assign103950_e156011_d_n8;
        locals.var_vdri_dn9 = assign103950_e156011_d_n9;
        locals.var_vdri_dn10 = assign103950_e156011_d_n10;
        locals.var_vdri_dn11 = assign103950_e156011_d_n11;
        locals.var_vdri_dn14 = assign103950_e156011_d_n14;
        locals.var_vdri_rv = 0.0;

        let assign103960_e156014: f64 = if locals.var_vddp >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2370 = assign103960_e156014;
        locals.var_guard2370_rv = 0.0;

        let (assign103970_e156025, assign103970_e156025_d_n0, assign103970_e156025_d_n2, assign103970_e156025_d_n4, assign103970_e156025_d_n5, assign103970_e156025_d_n6, assign103970_e156025_d_n7, assign103970_e156025_d_n8, assign103970_e156025_d_n9, assign103970_e156025_d_n10, assign103970_e156025_d_n11, assign103970_e156025_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2370 != 0.0)) {
        let assign103970_e156023: f64 = (locals.var_vdri / locals.var_vmaxe__blk2359);
        (assign103970_e156023, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn0)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn2)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn4)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn5)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn6)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn7)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn8)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn9 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn9)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn10)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn11)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn14 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn14)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103970_e156025;
        locals.var_t1_dn0 = assign103970_e156025_d_n0;
        locals.var_t1_dn2 = assign103970_e156025_d_n2;
        locals.var_t1_dn4 = assign103970_e156025_d_n4;
        locals.var_t1_dn5 = assign103970_e156025_d_n5;
        locals.var_t1_dn6 = assign103970_e156025_d_n6;
        locals.var_t1_dn7 = assign103970_e156025_d_n7;
        locals.var_t1_dn8 = assign103970_e156025_d_n8;
        locals.var_t1_dn9 = assign103970_e156025_d_n9;
        locals.var_t1_dn10 = assign103970_e156025_d_n10;
        locals.var_t1_dn11 = assign103970_e156025_d_n11;
        locals.var_t1_dn14 = assign103970_e156025_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign103980_e156038, assign103980_e156038_d_n0, assign103980_e156038_d_n2, assign103980_e156038_d_n4, assign103980_e156038_d_n5, assign103980_e156038_d_n6, assign103980_e156038_d_n7, assign103980_e156038_d_n8, assign103980_e156038_d_n9, assign103980_e156038_d_n10, assign103980_e156038_d_n11, assign103980_e156038_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2370 == 0.0)) {
        let assign103980_e156034: f64 = (-locals.var_vdri);
        let assign103980_e156036: f64 = (assign103980_e156034 / locals.var_vmaxe__blk2359);
        (assign103980_e156036, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn0)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn2)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn4)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn5)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn6)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn7)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn8)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn9) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn9)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn10)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn11)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn14) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn14)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103980_e156038;
        locals.var_t1_dn0 = assign103980_e156038_d_n0;
        locals.var_t1_dn2 = assign103980_e156038_d_n2;
        locals.var_t1_dn4 = assign103980_e156038_d_n4;
        locals.var_t1_dn5 = assign103980_e156038_d_n5;
        locals.var_t1_dn6 = assign103980_e156038_d_n6;
        locals.var_t1_dn7 = assign103980_e156038_d_n7;
        locals.var_t1_dn8 = assign103980_e156038_d_n8;
        locals.var_t1_dn9 = assign103980_e156038_d_n9;
        locals.var_t1_dn10 = assign103980_e156038_d_n10;
        locals.var_t1_dn11 = assign103980_e156038_d_n11;
        locals.var_t1_dn14 = assign103980_e156038_d_n14;
        locals.var_t1_rv = 0.0;

        let assign103990_e156042: f64 = (10.0 * 2.220446049250313e-16);
        let assign103990_e156043: f64 = (1.0 - assign103990_e156042);
        let assign103990_e156050: f64 = (10.0 * 2.220446049250313e-16);
        let assign103990_e156051: f64 = (1.0 + assign103990_e156050);
        let assign103990_e156053: f64 = if ((assign103990_e156043 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103990_e156051)) { 1.0 } else { 0.0 };
        locals.var_guard2371 = assign103990_e156053;
        locals.var_guard2371_rv = 0.0;

        let (assign104000_e156062, assign104000_e156062_d_n0, assign104000_e156062_d_n2, assign104000_e156062_d_n4, assign104000_e156062_d_n5, assign104000_e156062_d_n6, assign104000_e156062_d_n7, assign104000_e156062_d_n8, assign104000_e156062_d_n9, assign104000_e156062_d_n10, assign104000_e156062_d_n11, assign104000_e156062_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2371 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104000_e156062;
        locals.var_t3_dn0 = assign104000_e156062_d_n0;
        locals.var_t3_dn2 = assign104000_e156062_d_n2;
        locals.var_t3_dn4 = assign104000_e156062_d_n4;
        locals.var_t3_dn5 = assign104000_e156062_d_n5;
        locals.var_t3_dn6 = assign104000_e156062_d_n6;
        locals.var_t3_dn7 = assign104000_e156062_d_n7;
        locals.var_t3_dn8 = assign104000_e156062_d_n8;
        locals.var_t3_dn9 = assign104000_e156062_d_n9;
        locals.var_t3_dn10 = assign104000_e156062_d_n10;
        locals.var_t3_dn11 = assign104000_e156062_d_n11;
        locals.var_t3_dn14 = assign104000_e156062_d_n14;
        locals.var_t3_rv = 0.0;

        let assign104010_e156066: f64 = (10.0 * 2.220446049250313e-16);
        let assign104010_e156067: f64 = (2.0 - assign104010_e156066);
        let assign104010_e156074: f64 = (10.0 * 2.220446049250313e-16);
        let assign104010_e156075: f64 = (2.0 + assign104010_e156074);
        let assign104010_e156077: f64 = if ((assign104010_e156067 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104010_e156075)) { 1.0 } else { 0.0 };
        locals.var_guard2372 = assign104010_e156077;
        locals.var_guard2372_rv = 0.0;

        let (assign104020_e156089, assign104020_e156089_d_n0, assign104020_e156089_d_n2, assign104020_e156089_d_n4, assign104020_e156089_d_n5, assign104020_e156089_d_n6, assign104020_e156089_d_n7, assign104020_e156089_d_n8, assign104020_e156089_d_n9, assign104020_e156089_d_n10, assign104020_e156089_d_n11, assign104020_e156089_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104020_e156089;
        locals.var_t3_dn0 = assign104020_e156089_d_n0;
        locals.var_t3_dn2 = assign104020_e156089_d_n2;
        locals.var_t3_dn4 = assign104020_e156089_d_n4;
        locals.var_t3_dn5 = assign104020_e156089_d_n5;
        locals.var_t3_dn6 = assign104020_e156089_d_n6;
        locals.var_t3_dn7 = assign104020_e156089_d_n7;
        locals.var_t3_dn8 = assign104020_e156089_d_n8;
        locals.var_t3_dn9 = assign104020_e156089_d_n9;
        locals.var_t3_dn10 = assign104020_e156089_d_n10;
        locals.var_t3_dn11 = assign104020_e156089_d_n11;
        locals.var_t3_dn14 = assign104020_e156089_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign104030_e156106, assign104030_e156106_d_n0, assign104030_e156106_d_n2, assign104030_e156106_d_n4, assign104030_e156106_d_n5, assign104030_e156106_d_n6, assign104030_e156106_d_n7, assign104030_e156106_d_n8, assign104030_e156106_d_n9, assign104030_e156106_d_n10, assign104030_e156106_d_n11, assign104030_e156106_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 == 0.0)) {
        let assign104030_e156103: f64 = (locals.var_uc_rdrbb - 1.0);
        let assign104030_e156104: f64 = (locals.var_t1).powf(assign104030_e156103);
        (assign104030_e156104, if locals.var_uc_rdrbb_dn0 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn0)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn0 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn2 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn2)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn2 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn4 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn4)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn4 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn5 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn5)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn5 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn6 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn6)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn6 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn7 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn7)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn7 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn8 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn8)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn8 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn9 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn9)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn9 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn10 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn10)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn10 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn11 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn11)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn11 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn14 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn14)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn14 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn14 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104030_e156106;
        locals.var_t3_dn0 = assign104030_e156106_d_n0;
        locals.var_t3_dn2 = assign104030_e156106_d_n2;
        locals.var_t3_dn4 = assign104030_e156106_d_n4;
        locals.var_t3_dn5 = assign104030_e156106_d_n5;
        locals.var_t3_dn6 = assign104030_e156106_d_n6;
        locals.var_t3_dn7 = assign104030_e156106_d_n7;
        locals.var_t3_dn8 = assign104030_e156106_d_n8;
        locals.var_t3_dn9 = assign104030_e156106_d_n9;
        locals.var_t3_dn10 = assign104030_e156106_d_n10;
        locals.var_t3_dn11 = assign104030_e156106_d_n11;
        locals.var_t3_dn14 = assign104030_e156106_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign104040_e156115, assign104040_e156115_d_n0, assign104040_e156115_d_n2, assign104040_e156115_d_n4, assign104040_e156115_d_n5, assign104040_e156115_d_n6, assign104040_e156115_d_n7, assign104040_e156115_d_n8, assign104040_e156115_d_n9, assign104040_e156115_d_n10, assign104040_e156115_d_n11, assign104040_e156115_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104040_e156113: f64 = (locals.var_t1 * locals.var_t3);
        (assign104040_e156113, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104040_e156115;
        locals.var_t2_dn0 = assign104040_e156115_d_n0;
        locals.var_t2_dn2 = assign104040_e156115_d_n2;
        locals.var_t2_dn4 = assign104040_e156115_d_n4;
        locals.var_t2_dn5 = assign104040_e156115_d_n5;
        locals.var_t2_dn6 = assign104040_e156115_d_n6;
        locals.var_t2_dn7 = assign104040_e156115_d_n7;
        locals.var_t2_dn8 = assign104040_e156115_d_n8;
        locals.var_t2_dn9 = assign104040_e156115_d_n9;
        locals.var_t2_dn10 = assign104040_e156115_d_n10;
        locals.var_t2_dn11 = assign104040_e156115_d_n11;
        locals.var_t2_dn14 = assign104040_e156115_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign104050_e156124, assign104050_e156124_d_n0, assign104050_e156124_d_n2, assign104050_e156124_d_n4, assign104050_e156124_d_n5, assign104050_e156124_d_n6, assign104050_e156124_d_n7, assign104050_e156124_d_n8, assign104050_e156124_d_n9, assign104050_e156124_d_n10, assign104050_e156124_d_n11, assign104050_e156124_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104050_e156122: f64 = (1.0 + locals.var_t2);
        (assign104050_e156122, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104050_e156124;
        locals.var_t4_dn0 = assign104050_e156124_d_n0;
        locals.var_t4_dn2 = assign104050_e156124_d_n2;
        locals.var_t4_dn4 = assign104050_e156124_d_n4;
        locals.var_t4_dn5 = assign104050_e156124_d_n5;
        locals.var_t4_dn6 = assign104050_e156124_d_n6;
        locals.var_t4_dn7 = assign104050_e156124_d_n7;
        locals.var_t4_dn8 = assign104050_e156124_d_n8;
        locals.var_t4_dn9 = assign104050_e156124_d_n9;
        locals.var_t4_dn10 = assign104050_e156124_d_n10;
        locals.var_t4_dn11 = assign104050_e156124_d_n11;
        locals.var_t4_dn14 = assign104050_e156124_d_n14;
        locals.var_t4_rv = 0.0;

        let assign104060_e156128: f64 = (10.0 * 2.220446049250313e-16);
        let assign104060_e156129: f64 = (1.0 - assign104060_e156128);
        let assign104060_e156136: f64 = (10.0 * 2.220446049250313e-16);
        let assign104060_e156137: f64 = (1.0 + assign104060_e156136);
        let assign104060_e156139: f64 = if ((assign104060_e156129 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104060_e156137)) { 1.0 } else { 0.0 };
        locals.var_guard2373 = assign104060_e156139;
        locals.var_guard2373_rv = 0.0;

        let (assign104070_e156150, assign104070_e156150_d_n0, assign104070_e156150_d_n2, assign104070_e156150_d_n4, assign104070_e156150_d_n5, assign104070_e156150_d_n6, assign104070_e156150_d_n7, assign104070_e156150_d_n8, assign104070_e156150_d_n9, assign104070_e156150_d_n10, assign104070_e156150_d_n11, assign104070_e156150_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 != 0.0)) {
        let assign104070_e156148: f64 = (1.0 / locals.var_t4);
        (assign104070_e156148, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104070_e156150;
        locals.var_t5_dn0 = assign104070_e156150_d_n0;
        locals.var_t5_dn2 = assign104070_e156150_d_n2;
        locals.var_t5_dn4 = assign104070_e156150_d_n4;
        locals.var_t5_dn5 = assign104070_e156150_d_n5;
        locals.var_t5_dn6 = assign104070_e156150_d_n6;
        locals.var_t5_dn7 = assign104070_e156150_d_n7;
        locals.var_t5_dn8 = assign104070_e156150_d_n8;
        locals.var_t5_dn9 = assign104070_e156150_d_n9;
        locals.var_t5_dn10 = assign104070_e156150_d_n10;
        locals.var_t5_dn11 = assign104070_e156150_d_n11;
        locals.var_t5_dn14 = assign104070_e156150_d_n14;
        locals.var_t5_rv = 0.0;

        let assign104080_e156154: f64 = (10.0 * 2.220446049250313e-16);
        let assign104080_e156155: f64 = (2.0 - assign104080_e156154);
        let assign104080_e156162: f64 = (10.0 * 2.220446049250313e-16);
        let assign104080_e156163: f64 = (2.0 + assign104080_e156162);
        let assign104080_e156165: f64 = if ((assign104080_e156155 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104080_e156163)) { 1.0 } else { 0.0 };
        locals.var_guard2374 = assign104080_e156165;
        locals.var_guard2374_rv = 0.0;

        let (assign104090_e156180, assign104090_e156180_d_n0, assign104090_e156180_d_n2, assign104090_e156180_d_n4, assign104090_e156180_d_n5, assign104090_e156180_d_n6, assign104090_e156180_d_n7, assign104090_e156180_d_n8, assign104090_e156180_d_n9, assign104090_e156180_d_n10, assign104090_e156180_d_n11, assign104090_e156180_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 == 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104090_e156177: f64 = (locals.var_t4).sqrt();
        let assign104090_e156178: f64 = (1.0 / assign104090_e156177);
        (assign104090_e156178, (-((locals.var_t4_dn0 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn2 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn4 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn5 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn6 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn7 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn8 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn9 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn10 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn11 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn14 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104090_e156180;
        locals.var_t5_dn0 = assign104090_e156180_d_n0;
        locals.var_t5_dn2 = assign104090_e156180_d_n2;
        locals.var_t5_dn4 = assign104090_e156180_d_n4;
        locals.var_t5_dn5 = assign104090_e156180_d_n5;
        locals.var_t5_dn6 = assign104090_e156180_d_n6;
        locals.var_t5_dn7 = assign104090_e156180_d_n7;
        locals.var_t5_dn8 = assign104090_e156180_d_n8;
        locals.var_t5_dn9 = assign104090_e156180_d_n9;
        locals.var_t5_dn10 = assign104090_e156180_d_n10;
        locals.var_t5_dn11 = assign104090_e156180_d_n11;
        locals.var_t5_dn14 = assign104090_e156180_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign104100_e156205, assign104100_e156205_d_n0, assign104100_e156205_d_n2, assign104100_e156205_d_n4, assign104100_e156205_d_n5, assign104100_e156205_d_n6, assign104100_e156205_d_n7, assign104100_e156205_d_n8, assign104100_e156205_d_n9, assign104100_e156205_d_n10, assign104100_e156205_d_n11, assign104100_e156205_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 == 0.0)) && (locals.var_guard2374 == 0.0)) {
        let (assign104100_e156203, assign104100_e156203_d_n0, assign104100_e156203_d_n2, assign104100_e156203_d_n4, assign104100_e156203_d_n5, assign104100_e156203_d_n6, assign104100_e156203_d_n7, assign104100_e156203_d_n8, assign104100_e156203_d_n9, assign104100_e156203_d_n10, assign104100_e156203_d_n11, assign104100_e156203_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104100_e156197: f64 = (-1.0);
                let assign104100_e156199: f64 = (assign104100_e156197 / locals.var_uc_rdrbb);
                let assign104100_e156201: f64 = (assign104100_e156199 - 1.0);
                let assign104100_e156202: f64 = (locals.var_t4).powf(assign104100_e156201);
                (assign104100_e156202, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn0)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn2)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn4)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn5)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn6)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn7)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn8)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn9)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn10)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn11) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn11)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn11) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn11 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn14) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn14)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn14) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn14 / locals.var_t4)))) },)
            }
        };
        (assign104100_e156203, assign104100_e156203_d_n0, assign104100_e156203_d_n2, assign104100_e156203_d_n4, assign104100_e156203_d_n5, assign104100_e156203_d_n6, assign104100_e156203_d_n7, assign104100_e156203_d_n8, assign104100_e156203_d_n9, assign104100_e156203_d_n10, assign104100_e156203_d_n11, assign104100_e156203_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign104100_e156205;
        locals.var_t6_dn0 = assign104100_e156205_d_n0;
        locals.var_t6_dn2 = assign104100_e156205_d_n2;
        locals.var_t6_dn4 = assign104100_e156205_d_n4;
        locals.var_t6_dn5 = assign104100_e156205_d_n5;
        locals.var_t6_dn6 = assign104100_e156205_d_n6;
        locals.var_t6_dn7 = assign104100_e156205_d_n7;
        locals.var_t6_dn8 = assign104100_e156205_d_n8;
        locals.var_t6_dn9 = assign104100_e156205_d_n9;
        locals.var_t6_dn10 = assign104100_e156205_d_n10;
        locals.var_t6_dn11 = assign104100_e156205_d_n11;
        locals.var_t6_dn14 = assign104100_e156205_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign104110_e156220, assign104110_e156220_d_n0, assign104110_e156220_d_n2, assign104110_e156220_d_n4, assign104110_e156220_d_n5, assign104110_e156220_d_n6, assign104110_e156220_d_n7, assign104110_e156220_d_n8, assign104110_e156220_d_n9, assign104110_e156220_d_n10, assign104110_e156220_d_n11, assign104110_e156220_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 == 0.0)) && (locals.var_guard2374 == 0.0)) {
        let assign104110_e156218: f64 = (locals.var_t4 * locals.var_t6);
        (assign104110_e156218, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104110_e156220;
        locals.var_t5_dn0 = assign104110_e156220_d_n0;
        locals.var_t5_dn2 = assign104110_e156220_d_n2;
        locals.var_t5_dn4 = assign104110_e156220_d_n4;
        locals.var_t5_dn5 = assign104110_e156220_d_n5;
        locals.var_t5_dn6 = assign104110_e156220_d_n6;
        locals.var_t5_dn7 = assign104110_e156220_d_n7;
        locals.var_t5_dn8 = assign104110_e156220_d_n8;
        locals.var_t5_dn9 = assign104110_e156220_d_n9;
        locals.var_t5_dn10 = assign104110_e156220_d_n10;
        locals.var_t5_dn11 = assign104110_e156220_d_n11;
        locals.var_t5_dn14 = assign104110_e156220_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign104120_e156229, assign104120_e156229_d_n0, assign104120_e156229_d_n2, assign104120_e156229_d_n4, assign104120_e156229_d_n5, assign104120_e156229_d_n6, assign104120_e156229_d_n7, assign104120_e156229_d_n8, assign104120_e156229_d_n9, assign104120_e156229_d_n10, assign104120_e156229_d_n11, assign104120_e156229_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104120_e156227: f64 = (locals.var_mu0 * locals.var_t5);
        (assign104120_e156227, ((locals.var_mu0_dn0 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn0)), ((locals.var_mu0_dn2 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn2)), ((locals.var_mu0_dn4 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn4)), ((locals.var_mu0_dn5 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn5)), ((locals.var_mu0_dn6 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn6)), ((locals.var_mu0_dn7 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn7)), ((locals.var_mu0_dn8 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn8)), ((locals.var_mu0_dn9 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn9)), ((locals.var_mu0_dn10 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn10)), ((locals.var_mu0_dn11 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn11)), ((locals.var_mu0_dn14 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu__blk2358, locals.var_mu__blk2358_dn0, locals.var_mu__blk2358_dn2, locals.var_mu__blk2358_dn4, locals.var_mu__blk2358_dn5, locals.var_mu__blk2358_dn6, locals.var_mu__blk2358_dn7, locals.var_mu__blk2358_dn8, locals.var_mu__blk2358_dn9, locals.var_mu__blk2358_dn10, locals.var_mu__blk2358_dn11, locals.var_mu__blk2358_dn14,)
    }
};
        locals.var_mu__blk2358 = assign104120_e156229;
        locals.var_mu__blk2358_dn0 = assign104120_e156229_d_n0;
        locals.var_mu__blk2358_dn2 = assign104120_e156229_d_n2;
        locals.var_mu__blk2358_dn4 = assign104120_e156229_d_n4;
        locals.var_mu__blk2358_dn5 = assign104120_e156229_d_n5;
        locals.var_mu__blk2358_dn6 = assign104120_e156229_d_n6;
        locals.var_mu__blk2358_dn7 = assign104120_e156229_d_n7;
        locals.var_mu__blk2358_dn8 = assign104120_e156229_d_n8;
        locals.var_mu__blk2358_dn9 = assign104120_e156229_d_n9;
        locals.var_mu__blk2358_dn10 = assign104120_e156229_d_n10;
        locals.var_mu__blk2358_dn11 = assign104120_e156229_d_n11;
        locals.var_mu__blk2358_dn14 = assign104120_e156229_d_n14;
        locals.var_mu__blk2358_rv = 0.0;

        let (assign104130_e156238, assign104130_e156238_d_n0, assign104130_e156238_d_n2, assign104130_e156238_d_n4, assign104130_e156238_d_n5, assign104130_e156238_d_n6, assign104130_e156238_d_n7, assign104130_e156238_d_n8, assign104130_e156238_d_n9, assign104130_e156238_d_n10, assign104130_e156238_d_n11, assign104130_e156238_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104130_e156236: f64 = (1.0 + locals.var_t1);
        (assign104130_e156236, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104130_e156238;
        locals.var_t4_dn0 = assign104130_e156238_d_n0;
        locals.var_t4_dn2 = assign104130_e156238_d_n2;
        locals.var_t4_dn4 = assign104130_e156238_d_n4;
        locals.var_t4_dn5 = assign104130_e156238_d_n5;
        locals.var_t4_dn6 = assign104130_e156238_d_n6;
        locals.var_t4_dn7 = assign104130_e156238_d_n7;
        locals.var_t4_dn8 = assign104130_e156238_d_n8;
        locals.var_t4_dn9 = assign104130_e156238_d_n9;
        locals.var_t4_dn10 = assign104130_e156238_d_n10;
        locals.var_t4_dn11 = assign104130_e156238_d_n11;
        locals.var_t4_dn14 = assign104130_e156238_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign104140_e156247, assign104140_e156247_d_n0, assign104140_e156247_d_n2, assign104140_e156247_d_n4, assign104140_e156247_d_n5, assign104140_e156247_d_n6, assign104140_e156247_d_n7, assign104140_e156247_d_n8, assign104140_e156247_d_n9, assign104140_e156247_d_n10, assign104140_e156247_d_n11, assign104140_e156247_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104140_e156245: f64 = (1.0 / locals.var_t4);
        (assign104140_e156245, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104140_e156247;
        locals.var_t5_dn0 = assign104140_e156247_d_n0;
        locals.var_t5_dn2 = assign104140_e156247_d_n2;
        locals.var_t5_dn4 = assign104140_e156247_d_n4;
        locals.var_t5_dn5 = assign104140_e156247_d_n5;
        locals.var_t5_dn6 = assign104140_e156247_d_n6;
        locals.var_t5_dn7 = assign104140_e156247_d_n7;
        locals.var_t5_dn8 = assign104140_e156247_d_n8;
        locals.var_t5_dn9 = assign104140_e156247_d_n9;
        locals.var_t5_dn10 = assign104140_e156247_d_n10;
        locals.var_t5_dn11 = assign104140_e156247_d_n11;
        locals.var_t5_dn14 = assign104140_e156247_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign104150_e156266, assign104150_e156266_d_n0, assign104150_e156266_d_n2, assign104150_e156266_d_n4, assign104150_e156266_d_n5, assign104150_e156266_d_n6, assign104150_e156266_d_n7, assign104150_e156266_d_n8, assign104150_e156266_d_n9, assign104150_e156266_d_n10, assign104150_e156266_d_n11, assign104150_e156266_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104150_e156256: f64 = (1.0 - locals.var_t5);
        let assign104150_e156257: f64 = (locals.var_car * assign104150_e156256);
        let assign104150_e156259: f64 = (assign104150_e156257 * locals.var_vddpz);
        let assign104150_e156262: f64 = (locals.var_ldrifte - p.p423);
        let assign104150_e156263: f64 = (assign104150_e156259 / assign104150_e156262);
        let assign104150_e156264: f64 = (1.0 + assign104150_e156263);
        (assign104150_e156264, ((((locals.var_car * (-locals.var_t5_dn0)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn0)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn2)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn2)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn4)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn4)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn5)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn5)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn6)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn6)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn7)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn7)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn8)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn8)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn9)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn9)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn10)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn10)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn11)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn11)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn14)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn14)) / assign104150_e156262),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104150_e156266;
        locals.var_t4_dn0 = assign104150_e156266_d_n0;
        locals.var_t4_dn2 = assign104150_e156266_d_n2;
        locals.var_t4_dn4 = assign104150_e156266_d_n4;
        locals.var_t4_dn5 = assign104150_e156266_d_n5;
        locals.var_t4_dn6 = assign104150_e156266_d_n6;
        locals.var_t4_dn7 = assign104150_e156266_d_n7;
        locals.var_t4_dn8 = assign104150_e156266_d_n8;
        locals.var_t4_dn9 = assign104150_e156266_d_n9;
        locals.var_t4_dn10 = assign104150_e156266_d_n10;
        locals.var_t4_dn11 = assign104150_e156266_d_n11;
        locals.var_t4_dn14 = assign104150_e156266_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign104160_e156277, assign104160_e156277_d_n0, assign104160_e156277_d_n2, assign104160_e156277_d_n4, assign104160_e156277_d_n5, assign104160_e156277_d_n6, assign104160_e156277_d_n7, assign104160_e156277_d_n8, assign104160_e156277_d_n9, assign104160_e156277_d_n10, assign104160_e156277_d_n11, assign104160_e156277_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104160_e156273: f64 = locals.var_t4;
        let assign104160_e156275: f64 = (assign104160_e156273 - 0.001);
        (assign104160_e156275, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104160_e156277;
        locals.var_tmf1_dn0 = assign104160_e156277_d_n0;
        locals.var_tmf1_dn2 = assign104160_e156277_d_n2;
        locals.var_tmf1_dn4 = assign104160_e156277_d_n4;
        locals.var_tmf1_dn5 = assign104160_e156277_d_n5;
        locals.var_tmf1_dn6 = assign104160_e156277_d_n6;
        locals.var_tmf1_dn7 = assign104160_e156277_d_n7;
        locals.var_tmf1_dn8 = assign104160_e156277_d_n8;
        locals.var_tmf1_dn9 = assign104160_e156277_d_n9;
        locals.var_tmf1_dn10 = assign104160_e156277_d_n10;
        locals.var_tmf1_dn11 = assign104160_e156277_d_n11;
        locals.var_tmf1_dn14 = assign104160_e156277_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign104170_e156288, assign104170_e156288_d_n0, assign104170_e156288_d_n2, assign104170_e156288_d_n4, assign104170_e156288_d_n5, assign104170_e156288_d_n6, assign104170_e156288_d_n7, assign104170_e156288_d_n8, assign104170_e156288_d_n9, assign104170_e156288_d_n10, assign104170_e156288_d_n11, assign104170_e156288_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104170_e156288;
        locals.var_tmf2_dn0 = assign104170_e156288_d_n0;
        locals.var_tmf2_dn2 = assign104170_e156288_d_n2;
        locals.var_tmf2_dn4 = assign104170_e156288_d_n4;
        locals.var_tmf2_dn5 = assign104170_e156288_d_n5;
        locals.var_tmf2_dn6 = assign104170_e156288_d_n6;
        locals.var_tmf2_dn7 = assign104170_e156288_d_n7;
        locals.var_tmf2_dn8 = assign104170_e156288_d_n8;
        locals.var_tmf2_dn9 = assign104170_e156288_d_n9;
        locals.var_tmf2_dn10 = assign104170_e156288_d_n10;
        locals.var_tmf2_dn11 = assign104170_e156288_d_n11;
        locals.var_tmf2_dn14 = assign104170_e156288_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104180_e156301, assign104180_e156301_d_n0, assign104180_e156301_d_n2, assign104180_e156301_d_n4, assign104180_e156301_d_n5, assign104180_e156301_d_n6, assign104180_e156301_d_n7, assign104180_e156301_d_n8, assign104180_e156301_d_n9, assign104180_e156301_d_n10, assign104180_e156301_d_n11, assign104180_e156301_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let (assign104180_e156299, assign104180_e156299_d_n0, assign104180_e156299_d_n2, assign104180_e156299_d_n4, assign104180_e156299_d_n5, assign104180_e156299_d_n6, assign104180_e156299_d_n7, assign104180_e156299_d_n8, assign104180_e156299_d_n9, assign104180_e156299_d_n10, assign104180_e156299_d_n11, assign104180_e156299_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104180_e156298: f64 = (-locals.var_tmf2);
                (assign104180_e156298, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104180_e156299, assign104180_e156299_d_n0, assign104180_e156299_d_n2, assign104180_e156299_d_n4, assign104180_e156299_d_n5, assign104180_e156299_d_n6, assign104180_e156299_d_n7, assign104180_e156299_d_n8, assign104180_e156299_d_n9, assign104180_e156299_d_n10, assign104180_e156299_d_n11, assign104180_e156299_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104180_e156301;
        locals.var_tmf2_dn0 = assign104180_e156301_d_n0;
        locals.var_tmf2_dn2 = assign104180_e156301_d_n2;
        locals.var_tmf2_dn4 = assign104180_e156301_d_n4;
        locals.var_tmf2_dn5 = assign104180_e156301_d_n5;
        locals.var_tmf2_dn6 = assign104180_e156301_d_n6;
        locals.var_tmf2_dn7 = assign104180_e156301_d_n7;
        locals.var_tmf2_dn8 = assign104180_e156301_d_n8;
        locals.var_tmf2_dn9 = assign104180_e156301_d_n9;
        locals.var_tmf2_dn10 = assign104180_e156301_d_n10;
        locals.var_tmf2_dn11 = assign104180_e156301_d_n11;
        locals.var_tmf2_dn14 = assign104180_e156301_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_399(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104190_e156313, assign104190_e156313_d_n0, assign104190_e156313_d_n2, assign104190_e156313_d_n4, assign104190_e156313_d_n5, assign104190_e156313_d_n6, assign104190_e156313_d_n7, assign104190_e156313_d_n8, assign104190_e156313_d_n9, assign104190_e156313_d_n10, assign104190_e156313_d_n11, assign104190_e156313_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104190_e156308: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104190_e156310: f64 = (assign104190_e156308 + locals.var_tmf2);
        let assign104190_e156311: f64 = (assign104190_e156310).sqrt();
        (assign104190_e156311, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104190_e156311)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104190_e156313;
        locals.var_tmf2_dn0 = assign104190_e156313_d_n0;
        locals.var_tmf2_dn2 = assign104190_e156313_d_n2;
        locals.var_tmf2_dn4 = assign104190_e156313_d_n4;
        locals.var_tmf2_dn5 = assign104190_e156313_d_n5;
        locals.var_tmf2_dn6 = assign104190_e156313_d_n6;
        locals.var_tmf2_dn7 = assign104190_e156313_d_n7;
        locals.var_tmf2_dn8 = assign104190_e156313_d_n8;
        locals.var_tmf2_dn9 = assign104190_e156313_d_n9;
        locals.var_tmf2_dn10 = assign104190_e156313_d_n10;
        locals.var_tmf2_dn11 = assign104190_e156313_d_n11;
        locals.var_tmf2_dn14 = assign104190_e156313_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104200_e156326, assign104200_e156326_d_n0, assign104200_e156326_d_n2, assign104200_e156326_d_n4, assign104200_e156326_d_n5, assign104200_e156326_d_n6, assign104200_e156326_d_n7, assign104200_e156326_d_n8, assign104200_e156326_d_n9, assign104200_e156326_d_n10, assign104200_e156326_d_n11, assign104200_e156326_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104200_e156322: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104200_e156323: f64 = (1.0 + assign104200_e156322);
        let assign104200_e156324: f64 = (0.5 * assign104200_e156323);
        (assign104200_e156324, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104200_e156326;
        locals.var_t0_dn0 = assign104200_e156326_d_n0;
        locals.var_t0_dn2 = assign104200_e156326_d_n2;
        locals.var_t0_dn4 = assign104200_e156326_d_n4;
        locals.var_t0_dn5 = assign104200_e156326_d_n5;
        locals.var_t0_dn6 = assign104200_e156326_d_n6;
        locals.var_t0_dn7 = assign104200_e156326_d_n7;
        locals.var_t0_dn8 = assign104200_e156326_d_n8;
        locals.var_t0_dn9 = assign104200_e156326_d_n9;
        locals.var_t0_dn10 = assign104200_e156326_d_n10;
        locals.var_t0_dn11 = assign104200_e156326_d_n11;
        locals.var_t0_dn14 = assign104200_e156326_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104210_e156339, assign104210_e156339_d_n0, assign104210_e156339_d_n2, assign104210_e156339_d_n4, assign104210_e156339_d_n5, assign104210_e156339_d_n6, assign104210_e156339_d_n7, assign104210_e156339_d_n8, assign104210_e156339_d_n9, assign104210_e156339_d_n10, assign104210_e156339_d_n11, assign104210_e156339_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104210_e156335: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104210_e156336: f64 = (0.5 * assign104210_e156335);
        let assign104210_e156337: f64 = assign104210_e156336;
        (assign104210_e156337, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104210_e156339;
        locals.var_t5_dn0 = assign104210_e156339_d_n0;
        locals.var_t5_dn2 = assign104210_e156339_d_n2;
        locals.var_t5_dn4 = assign104210_e156339_d_n4;
        locals.var_t5_dn5 = assign104210_e156339_d_n5;
        locals.var_t5_dn6 = assign104210_e156339_d_n6;
        locals.var_t5_dn7 = assign104210_e156339_d_n7;
        locals.var_t5_dn8 = assign104210_e156339_d_n8;
        locals.var_t5_dn9 = assign104210_e156339_d_n9;
        locals.var_t5_dn10 = assign104210_e156339_d_n10;
        locals.var_t5_dn11 = assign104210_e156339_d_n11;
        locals.var_t5_dn14 = assign104210_e156339_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign104220_e156348, assign104220_e156348_d_n0, assign104220_e156348_d_n2, assign104220_e156348_d_n4, assign104220_e156348_d_n5, assign104220_e156348_d_n6, assign104220_e156348_d_n7, assign104220_e156348_d_n8, assign104220_e156348_d_n9, assign104220_e156348_d_n10, assign104220_e156348_d_n11, assign104220_e156348_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104220_e156346: f64 = (locals.var_noverd * locals.var_t5);
        (assign104220_e156346, (locals.var_noverd * locals.var_t5_dn0), (locals.var_noverd * locals.var_t5_dn2), (locals.var_noverd * locals.var_t5_dn4), (locals.var_noverd * locals.var_t5_dn5), (locals.var_noverd * locals.var_t5_dn6), (locals.var_noverd * locals.var_t5_dn7), (locals.var_noverd * locals.var_t5_dn8), (locals.var_noverd * locals.var_t5_dn9), (locals.var_noverd * locals.var_t5_dn10), (locals.var_noverd * locals.var_t5_dn11), (locals.var_noverd * locals.var_t5_dn14),)
    } else {
        (locals.var_carr1, locals.var_carr1_dn0, locals.var_carr1_dn2, locals.var_carr1_dn4, locals.var_carr1_dn5, locals.var_carr1_dn6, locals.var_carr1_dn7, locals.var_carr1_dn8, locals.var_carr1_dn9, locals.var_carr1_dn10, locals.var_carr1_dn11, locals.var_carr1_dn14,)
    }
};
        locals.var_carr1 = assign104220_e156348;
        locals.var_carr1_dn0 = assign104220_e156348_d_n0;
        locals.var_carr1_dn2 = assign104220_e156348_d_n2;
        locals.var_carr1_dn4 = assign104220_e156348_d_n4;
        locals.var_carr1_dn5 = assign104220_e156348_d_n5;
        locals.var_carr1_dn6 = assign104220_e156348_d_n6;
        locals.var_carr1_dn7 = assign104220_e156348_d_n7;
        locals.var_carr1_dn8 = assign104220_e156348_d_n8;
        locals.var_carr1_dn9 = assign104220_e156348_d_n9;
        locals.var_carr1_dn10 = assign104220_e156348_d_n10;
        locals.var_carr1_dn11 = assign104220_e156348_d_n11;
        locals.var_carr1_dn14 = assign104220_e156348_d_n14;
        locals.var_carr1_rv = 0.0;

        let (assign104230_e156359, assign104230_e156359_d_n0, assign104230_e156359_d_n2, assign104230_e156359_d_n4, assign104230_e156359_d_n5, assign104230_e156359_d_n6, assign104230_e156359_d_n7, assign104230_e156359_d_n8, assign104230_e156359_d_n9, assign104230_e156359_d_n10, assign104230_e156359_d_n11, assign104230_e156359_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104230_e156355: f64 = (locals.var_rd_qbuld / 1.6021918e-19);
        let assign104230_e156357: f64 = (assign104230_e156355 * p.p430);
        (assign104230_e156357, ((locals.var_rd_qbuld_dn0 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn2 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn4 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn5 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn6 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn7 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn8 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn9 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn10 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn11 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn14 / 1.6021918e-19) * p.p430),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn11, locals.var_carr2_dn14,)
    }
};
        locals.var_carr2 = assign104230_e156359;
        locals.var_carr2_dn0 = assign104230_e156359_d_n0;
        locals.var_carr2_dn2 = assign104230_e156359_d_n2;
        locals.var_carr2_dn4 = assign104230_e156359_d_n4;
        locals.var_carr2_dn5 = assign104230_e156359_d_n5;
        locals.var_carr2_dn6 = assign104230_e156359_d_n6;
        locals.var_carr2_dn7 = assign104230_e156359_d_n7;
        locals.var_carr2_dn8 = assign104230_e156359_d_n8;
        locals.var_carr2_dn9 = assign104230_e156359_d_n9;
        locals.var_carr2_dn10 = assign104230_e156359_d_n10;
        locals.var_carr2_dn11 = assign104230_e156359_d_n11;
        locals.var_carr2_dn14 = assign104230_e156359_d_n14;
        locals.var_carr2_rv = 0.0;

        let (assign104240_e156372, assign104240_e156372_d_n0, assign104240_e156372_d_n2, assign104240_e156372_d_n4, assign104240_e156372_d_n5, assign104240_e156372_d_n6, assign104240_e156372_d_n7, assign104240_e156372_d_n8, assign104240_e156372_d_n9, assign104240_e156372_d_n10, assign104240_e156372_d_n11, assign104240_e156372_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104240_e156366: f64 = (locals.var_carr1 - locals.var_carr2);
        let assign104240_e156369: f64 = (locals.var_carr1 * 0.001);
        let assign104240_e156370: f64 = (assign104240_e156366 - assign104240_e156369);
        (assign104240_e156370, ((locals.var_carr1_dn0 - locals.var_carr2_dn0) - (locals.var_carr1_dn0 * 0.001)), ((locals.var_carr1_dn2 - locals.var_carr2_dn2) - (locals.var_carr1_dn2 * 0.001)), ((locals.var_carr1_dn4 - locals.var_carr2_dn4) - (locals.var_carr1_dn4 * 0.001)), ((locals.var_carr1_dn5 - locals.var_carr2_dn5) - (locals.var_carr1_dn5 * 0.001)), ((locals.var_carr1_dn6 - locals.var_carr2_dn6) - (locals.var_carr1_dn6 * 0.001)), ((locals.var_carr1_dn7 - locals.var_carr2_dn7) - (locals.var_carr1_dn7 * 0.001)), ((locals.var_carr1_dn8 - locals.var_carr2_dn8) - (locals.var_carr1_dn8 * 0.001)), ((locals.var_carr1_dn9 - locals.var_carr2_dn9) - (locals.var_carr1_dn9 * 0.001)), ((locals.var_carr1_dn10 - locals.var_carr2_dn10) - (locals.var_carr1_dn10 * 0.001)), ((locals.var_carr1_dn11 - locals.var_carr2_dn11) - (locals.var_carr1_dn11 * 0.001)), ((locals.var_carr1_dn14 - locals.var_carr2_dn14) - (locals.var_carr1_dn14 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104240_e156372;
        locals.var_tmf1_dn0 = assign104240_e156372_d_n0;
        locals.var_tmf1_dn2 = assign104240_e156372_d_n2;
        locals.var_tmf1_dn4 = assign104240_e156372_d_n4;
        locals.var_tmf1_dn5 = assign104240_e156372_d_n5;
        locals.var_tmf1_dn6 = assign104240_e156372_d_n6;
        locals.var_tmf1_dn7 = assign104240_e156372_d_n7;
        locals.var_tmf1_dn8 = assign104240_e156372_d_n8;
        locals.var_tmf1_dn9 = assign104240_e156372_d_n9;
        locals.var_tmf1_dn10 = assign104240_e156372_d_n10;
        locals.var_tmf1_dn11 = assign104240_e156372_d_n11;
        locals.var_tmf1_dn14 = assign104240_e156372_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign104250_e156385, assign104250_e156385_d_n0, assign104250_e156385_d_n2, assign104250_e156385_d_n4, assign104250_e156385_d_n5, assign104250_e156385_d_n6, assign104250_e156385_d_n7, assign104250_e156385_d_n8, assign104250_e156385_d_n9, assign104250_e156385_d_n10, assign104250_e156385_d_n11, assign104250_e156385_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104250_e156379: f64 = (4.0 * locals.var_carr1);
        let assign104250_e156382: f64 = (locals.var_carr1 * 0.001);
        let assign104250_e156383: f64 = (assign104250_e156379 * assign104250_e156382);
        (assign104250_e156383, (((4.0 * locals.var_carr1_dn0) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn0 * 0.001))), (((4.0 * locals.var_carr1_dn2) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn2 * 0.001))), (((4.0 * locals.var_carr1_dn4) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn4 * 0.001))), (((4.0 * locals.var_carr1_dn5) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn5 * 0.001))), (((4.0 * locals.var_carr1_dn6) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn6 * 0.001))), (((4.0 * locals.var_carr1_dn7) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn7 * 0.001))), (((4.0 * locals.var_carr1_dn8) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn8 * 0.001))), (((4.0 * locals.var_carr1_dn9) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn9 * 0.001))), (((4.0 * locals.var_carr1_dn10) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn10 * 0.001))), (((4.0 * locals.var_carr1_dn11) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn11 * 0.001))), (((4.0 * locals.var_carr1_dn14) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn14 * 0.001))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104250_e156385;
        locals.var_tmf2_dn0 = assign104250_e156385_d_n0;
        locals.var_tmf2_dn2 = assign104250_e156385_d_n2;
        locals.var_tmf2_dn4 = assign104250_e156385_d_n4;
        locals.var_tmf2_dn5 = assign104250_e156385_d_n5;
        locals.var_tmf2_dn6 = assign104250_e156385_d_n6;
        locals.var_tmf2_dn7 = assign104250_e156385_d_n7;
        locals.var_tmf2_dn8 = assign104250_e156385_d_n8;
        locals.var_tmf2_dn9 = assign104250_e156385_d_n9;
        locals.var_tmf2_dn10 = assign104250_e156385_d_n10;
        locals.var_tmf2_dn11 = assign104250_e156385_d_n11;
        locals.var_tmf2_dn14 = assign104250_e156385_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104260_e156398, assign104260_e156398_d_n0, assign104260_e156398_d_n2, assign104260_e156398_d_n4, assign104260_e156398_d_n5, assign104260_e156398_d_n6, assign104260_e156398_d_n7, assign104260_e156398_d_n8, assign104260_e156398_d_n9, assign104260_e156398_d_n10, assign104260_e156398_d_n11, assign104260_e156398_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let (assign104260_e156396, assign104260_e156396_d_n0, assign104260_e156396_d_n2, assign104260_e156396_d_n4, assign104260_e156396_d_n5, assign104260_e156396_d_n6, assign104260_e156396_d_n7, assign104260_e156396_d_n8, assign104260_e156396_d_n9, assign104260_e156396_d_n10, assign104260_e156396_d_n11, assign104260_e156396_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104260_e156395: f64 = (-locals.var_tmf2);
                (assign104260_e156395, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104260_e156396, assign104260_e156396_d_n0, assign104260_e156396_d_n2, assign104260_e156396_d_n4, assign104260_e156396_d_n5, assign104260_e156396_d_n6, assign104260_e156396_d_n7, assign104260_e156396_d_n8, assign104260_e156396_d_n9, assign104260_e156396_d_n10, assign104260_e156396_d_n11, assign104260_e156396_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104260_e156398;
        locals.var_tmf2_dn0 = assign104260_e156398_d_n0;
        locals.var_tmf2_dn2 = assign104260_e156398_d_n2;
        locals.var_tmf2_dn4 = assign104260_e156398_d_n4;
        locals.var_tmf2_dn5 = assign104260_e156398_d_n5;
        locals.var_tmf2_dn6 = assign104260_e156398_d_n6;
        locals.var_tmf2_dn7 = assign104260_e156398_d_n7;
        locals.var_tmf2_dn8 = assign104260_e156398_d_n8;
        locals.var_tmf2_dn9 = assign104260_e156398_d_n9;
        locals.var_tmf2_dn10 = assign104260_e156398_d_n10;
        locals.var_tmf2_dn11 = assign104260_e156398_d_n11;
        locals.var_tmf2_dn14 = assign104260_e156398_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104270_e156410, assign104270_e156410_d_n0, assign104270_e156410_d_n2, assign104270_e156410_d_n4, assign104270_e156410_d_n5, assign104270_e156410_d_n6, assign104270_e156410_d_n7, assign104270_e156410_d_n8, assign104270_e156410_d_n9, assign104270_e156410_d_n10, assign104270_e156410_d_n11, assign104270_e156410_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104270_e156405: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104270_e156407: f64 = (assign104270_e156405 + locals.var_tmf2);
        let assign104270_e156408: f64 = (assign104270_e156407).sqrt();
        (assign104270_e156408, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104270_e156408)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104270_e156410;
        locals.var_tmf2_dn0 = assign104270_e156410_d_n0;
        locals.var_tmf2_dn2 = assign104270_e156410_d_n2;
        locals.var_tmf2_dn4 = assign104270_e156410_d_n4;
        locals.var_tmf2_dn5 = assign104270_e156410_d_n5;
        locals.var_tmf2_dn6 = assign104270_e156410_d_n6;
        locals.var_tmf2_dn7 = assign104270_e156410_d_n7;
        locals.var_tmf2_dn8 = assign104270_e156410_d_n8;
        locals.var_tmf2_dn9 = assign104270_e156410_d_n9;
        locals.var_tmf2_dn10 = assign104270_e156410_d_n10;
        locals.var_tmf2_dn11 = assign104270_e156410_d_n11;
        locals.var_tmf2_dn14 = assign104270_e156410_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104280_e156423, assign104280_e156423_d_n0, assign104280_e156423_d_n2, assign104280_e156423_d_n4, assign104280_e156423_d_n5, assign104280_e156423_d_n6, assign104280_e156423_d_n7, assign104280_e156423_d_n8, assign104280_e156423_d_n9, assign104280_e156423_d_n10, assign104280_e156423_d_n11, assign104280_e156423_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104280_e156419: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104280_e156420: f64 = (1.0 + assign104280_e156419);
        let assign104280_e156421: f64 = (0.5 * assign104280_e156420);
        (assign104280_e156421, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104280_e156423;
        locals.var_t0_dn0 = assign104280_e156423_d_n0;
        locals.var_t0_dn2 = assign104280_e156423_d_n2;
        locals.var_t0_dn4 = assign104280_e156423_d_n4;
        locals.var_t0_dn5 = assign104280_e156423_d_n5;
        locals.var_t0_dn6 = assign104280_e156423_d_n6;
        locals.var_t0_dn7 = assign104280_e156423_d_n7;
        locals.var_t0_dn8 = assign104280_e156423_d_n8;
        locals.var_t0_dn9 = assign104280_e156423_d_n9;
        locals.var_t0_dn10 = assign104280_e156423_d_n10;
        locals.var_t0_dn11 = assign104280_e156423_d_n11;
        locals.var_t0_dn14 = assign104280_e156423_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104290_e156436, assign104290_e156436_d_n0, assign104290_e156436_d_n2, assign104290_e156436_d_n4, assign104290_e156436_d_n5, assign104290_e156436_d_n6, assign104290_e156436_d_n7, assign104290_e156436_d_n8, assign104290_e156436_d_n9, assign104290_e156436_d_n10, assign104290_e156436_d_n11, assign104290_e156436_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104290_e156432: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104290_e156433: f64 = (0.5 * assign104290_e156432);
        let assign104290_e156434: f64 = (locals.var_carr1 - assign104290_e156433);
        (assign104290_e156434, (locals.var_carr1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_carr1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_carr1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_carr1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_carr1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_carr1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_carr1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_carr1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_carr1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_carr1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_carr1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn11, locals.var_carr2_dn14,)
    }
};
        locals.var_carr2 = assign104290_e156436;
        locals.var_carr2_dn0 = assign104290_e156436_d_n0;
        locals.var_carr2_dn2 = assign104290_e156436_d_n2;
        locals.var_carr2_dn4 = assign104290_e156436_d_n4;
        locals.var_carr2_dn5 = assign104290_e156436_d_n5;
        locals.var_carr2_dn6 = assign104290_e156436_d_n6;
        locals.var_carr2_dn7 = assign104290_e156436_d_n7;
        locals.var_carr2_dn8 = assign104290_e156436_d_n8;
        locals.var_carr2_dn9 = assign104290_e156436_d_n9;
        locals.var_carr2_dn10 = assign104290_e156436_d_n10;
        locals.var_carr2_dn11 = assign104290_e156436_d_n11;
        locals.var_carr2_dn14 = assign104290_e156436_d_n14;
        locals.var_carr2_rv = 0.0;

        let (assign104300_e156445, assign104300_e156445_d_n0, assign104300_e156445_d_n2, assign104300_e156445_d_n4, assign104300_e156445_d_n5, assign104300_e156445_d_n6, assign104300_e156445_d_n7, assign104300_e156445_d_n8, assign104300_e156445_d_n9, assign104300_e156445_d_n10, assign104300_e156445_d_n11, assign104300_e156445_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104300_e156443: f64 = (locals.var_carr1 - locals.var_carr2);
        (assign104300_e156443, (locals.var_carr1_dn0 - locals.var_carr2_dn0), (locals.var_carr1_dn2 - locals.var_carr2_dn2), (locals.var_carr1_dn4 - locals.var_carr2_dn4), (locals.var_carr1_dn5 - locals.var_carr2_dn5), (locals.var_carr1_dn6 - locals.var_carr2_dn6), (locals.var_carr1_dn7 - locals.var_carr2_dn7), (locals.var_carr1_dn8 - locals.var_carr2_dn8), (locals.var_carr1_dn9 - locals.var_carr2_dn9), (locals.var_carr1_dn10 - locals.var_carr2_dn10), (locals.var_carr1_dn11 - locals.var_carr2_dn11), (locals.var_carr1_dn14 - locals.var_carr2_dn14),)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    }
};
        locals.var_carr = assign104300_e156445;
        locals.var_carr_dn0 = assign104300_e156445_d_n0;
        locals.var_carr_dn2 = assign104300_e156445_d_n2;
        locals.var_carr_dn4 = assign104300_e156445_d_n4;
        locals.var_carr_dn5 = assign104300_e156445_d_n5;
        locals.var_carr_dn6 = assign104300_e156445_d_n6;
        locals.var_carr_dn7 = assign104300_e156445_d_n7;
        locals.var_carr_dn8 = assign104300_e156445_d_n8;
        locals.var_carr_dn9 = assign104300_e156445_d_n9;
        locals.var_carr_dn10 = assign104300_e156445_d_n10;
        locals.var_carr_dn11 = assign104300_e156445_d_n11;
        locals.var_carr_dn14 = assign104300_e156445_d_n14;
        locals.var_carr_rv = 0.0;

        let assign104310_e156452: f64 = if ((p.p441 > 0.0) && (p.p440 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2375 = assign104310_e156452;
        locals.var_guard2375_rv = 0.0;

        let assign104320_e156456: f64 = (locals.var_noverd * p.p440);
        let assign104320_e156459: f64 = (locals.var_noverd * p.p441);
        let assign104320_e156460: f64 = (assign104320_e156456 - assign104320_e156459);
        let assign104320_e156464: f64 = (locals.var_noverd * p.p441);
        let assign104320_e156467: f64 = if ((locals.var_carr > assign104320_e156460) && (assign104320_e156464 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2376 = assign104320_e156467;
        locals.var_guard2376_rv = 0.0;

        let (assign104330_e156486, assign104330_e156486_d_n0, assign104330_e156486_d_n2, assign104330_e156486_d_n4, assign104330_e156486_d_n5, assign104330_e156486_d_n6, assign104330_e156486_d_n7, assign104330_e156486_d_n8, assign104330_e156486_d_n9, assign104330_e156486_d_n10, assign104330_e156486_d_n11, assign104330_e156486_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104330_e156479: f64 = (locals.var_noverd * p.p440);
        let assign104330_e156480: f64 = (locals.var_carr - assign104330_e156479);
        let assign104330_e156483: f64 = (locals.var_noverd * p.p441);
        let assign104330_e156484: f64 = (assign104330_e156480 + assign104330_e156483);
        (assign104330_e156484, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104330_e156486;
        locals.var_tmf1_dn0 = assign104330_e156486_d_n0;
        locals.var_tmf1_dn2 = assign104330_e156486_d_n2;
        locals.var_tmf1_dn4 = assign104330_e156486_d_n4;
        locals.var_tmf1_dn5 = assign104330_e156486_d_n5;
        locals.var_tmf1_dn6 = assign104330_e156486_d_n6;
        locals.var_tmf1_dn7 = assign104330_e156486_d_n7;
        locals.var_tmf1_dn8 = assign104330_e156486_d_n8;
        locals.var_tmf1_dn9 = assign104330_e156486_d_n9;
        locals.var_tmf1_dn10 = assign104330_e156486_d_n10;
        locals.var_tmf1_dn11 = assign104330_e156486_d_n11;
        locals.var_tmf1_dn14 = assign104330_e156486_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign104340_e156499, assign104340_e156499_d_n0, assign104340_e156499_d_n2, assign104340_e156499_d_n4, assign104340_e156499_d_n5, assign104340_e156499_d_n6, assign104340_e156499_d_n7, assign104340_e156499_d_n8, assign104340_e156499_d_n9, assign104340_e156499_d_n10, assign104340_e156499_d_n11, assign104340_e156499_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104340_e156497: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign104340_e156497, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign104340_e156499;
        locals.var_x2_dn0 = assign104340_e156499_d_n0;
        locals.var_x2_dn2 = assign104340_e156499_d_n2;
        locals.var_x2_dn4 = assign104340_e156499_d_n4;
        locals.var_x2_dn5 = assign104340_e156499_d_n5;
        locals.var_x2_dn6 = assign104340_e156499_d_n6;
        locals.var_x2_dn7 = assign104340_e156499_d_n7;
        locals.var_x2_dn8 = assign104340_e156499_d_n8;
        locals.var_x2_dn9 = assign104340_e156499_d_n9;
        locals.var_x2_dn10 = assign104340_e156499_d_n10;
        locals.var_x2_dn11 = assign104340_e156499_d_n11;
        locals.var_x2_dn14 = assign104340_e156499_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign104350_e156516, assign104350_e156516_d_n0, assign104350_e156516_d_n2, assign104350_e156516_d_n4, assign104350_e156516_d_n5, assign104350_e156516_d_n6, assign104350_e156516_d_n7, assign104350_e156516_d_n8, assign104350_e156516_d_n9, assign104350_e156516_d_n10, assign104350_e156516_d_n11, assign104350_e156516_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104350_e156510: f64 = (locals.var_noverd * p.p441);
        let assign104350_e156513: f64 = (locals.var_noverd * p.p441);
        let assign104350_e156514: f64 = (assign104350_e156510 * assign104350_e156513);
        (assign104350_e156514, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign104350_e156516;
        locals.var_xmax2_dn0 = assign104350_e156516_d_n0;
        locals.var_xmax2_dn2 = assign104350_e156516_d_n2;
        locals.var_xmax2_dn4 = assign104350_e156516_d_n4;
        locals.var_xmax2_dn5 = assign104350_e156516_d_n5;
        locals.var_xmax2_dn6 = assign104350_e156516_d_n6;
        locals.var_xmax2_dn7 = assign104350_e156516_d_n7;
        locals.var_xmax2_dn8 = assign104350_e156516_d_n8;
        locals.var_xmax2_dn9 = assign104350_e156516_d_n9;
        locals.var_xmax2_dn10 = assign104350_e156516_d_n10;
        locals.var_xmax2_dn11 = assign104350_e156516_d_n11;
        locals.var_xmax2_dn14 = assign104350_e156516_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign104360_e156527, assign104360_e156527_d_n0, assign104360_e156527_d_n2, assign104360_e156527_d_n4, assign104360_e156527_d_n5, assign104360_e156527_d_n6, assign104360_e156527_d_n7, assign104360_e156527_d_n8, assign104360_e156527_d_n9, assign104360_e156527_d_n10, assign104360_e156527_d_n11, assign104360_e156527_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign104360_e156527;
        locals.var_xp_dn0 = assign104360_e156527_d_n0;
        locals.var_xp_dn2 = assign104360_e156527_d_n2;
        locals.var_xp_dn4 = assign104360_e156527_d_n4;
        locals.var_xp_dn5 = assign104360_e156527_d_n5;
        locals.var_xp_dn6 = assign104360_e156527_d_n6;
        locals.var_xp_dn7 = assign104360_e156527_d_n7;
        locals.var_xp_dn8 = assign104360_e156527_d_n8;
        locals.var_xp_dn9 = assign104360_e156527_d_n9;
        locals.var_xp_dn10 = assign104360_e156527_d_n10;
        locals.var_xp_dn11 = assign104360_e156527_d_n11;
        locals.var_xp_dn14 = assign104360_e156527_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign104370_e156538, assign104370_e156538_d_n0, assign104370_e156538_d_n2, assign104370_e156538_d_n4, assign104370_e156538_d_n5, assign104370_e156538_d_n6, assign104370_e156538_d_n7, assign104370_e156538_d_n8, assign104370_e156538_d_n9, assign104370_e156538_d_n10, assign104370_e156538_d_n11, assign104370_e156538_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign104370_e156538;
        locals.var_xmp_dn0 = assign104370_e156538_d_n0;
        locals.var_xmp_dn2 = assign104370_e156538_d_n2;
        locals.var_xmp_dn4 = assign104370_e156538_d_n4;
        locals.var_xmp_dn5 = assign104370_e156538_d_n5;
        locals.var_xmp_dn6 = assign104370_e156538_d_n6;
        locals.var_xmp_dn7 = assign104370_e156538_d_n7;
        locals.var_xmp_dn8 = assign104370_e156538_d_n8;
        locals.var_xmp_dn9 = assign104370_e156538_d_n9;
        locals.var_xmp_dn10 = assign104370_e156538_d_n10;
        locals.var_xmp_dn11 = assign104370_e156538_d_n11;
        locals.var_xmp_dn14 = assign104370_e156538_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign104380_e156549,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104380_e156549;
        locals.var_m0_rv = 0.0;

        let (assign104390_e156560,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104390_e156560;
        locals.var_mm_rv = 0.0;

        let (assign104400_e156571, assign104400_e156571_d_n0, assign104400_e156571_d_n2, assign104400_e156571_d_n4, assign104400_e156571_d_n5, assign104400_e156571_d_n6, assign104400_e156571_d_n7, assign104400_e156571_d_n8, assign104400_e156571_d_n9, assign104400_e156571_d_n10, assign104400_e156571_d_n11, assign104400_e156571_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign104400_e156571;
        locals.var_arg_dn0 = assign104400_e156571_d_n0;
        locals.var_arg_dn2 = assign104400_e156571_d_n2;
        locals.var_arg_dn4 = assign104400_e156571_d_n4;
        locals.var_arg_dn5 = assign104400_e156571_d_n5;
        locals.var_arg_dn6 = assign104400_e156571_d_n6;
        locals.var_arg_dn7 = assign104400_e156571_d_n7;
        locals.var_arg_dn8 = assign104400_e156571_d_n8;
        locals.var_arg_dn9 = assign104400_e156571_d_n9;
        locals.var_arg_dn10 = assign104400_e156571_d_n10;
        locals.var_arg_dn11 = assign104400_e156571_d_n11;
        locals.var_arg_dn14 = assign104400_e156571_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign104410_e156582, assign104410_e156582_d_n0, assign104410_e156582_d_n2, assign104410_e156582_d_n4, assign104410_e156582_d_n5, assign104410_e156582_d_n6, assign104410_e156582_d_n7, assign104410_e156582_d_n8, assign104410_e156582_d_n9, assign104410_e156582_d_n10, assign104410_e156582_d_n11, assign104410_e156582_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104410_e156582;
        locals.var_dnm_dn0 = assign104410_e156582_d_n0;
        locals.var_dnm_dn2 = assign104410_e156582_d_n2;
        locals.var_dnm_dn4 = assign104410_e156582_d_n4;
        locals.var_dnm_dn5 = assign104410_e156582_d_n5;
        locals.var_dnm_dn6 = assign104410_e156582_d_n6;
        locals.var_dnm_dn7 = assign104410_e156582_d_n7;
        locals.var_dnm_dn8 = assign104410_e156582_d_n8;
        locals.var_dnm_dn9 = assign104410_e156582_d_n9;
        locals.var_dnm_dn10 = assign104410_e156582_d_n10;
        locals.var_dnm_dn11 = assign104410_e156582_d_n11;
        locals.var_dnm_dn14 = assign104410_e156582_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign104420_e156593,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104420_e156593;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_400(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign104430_loop_guard: usize = 0;
        while {
            let assign104430_cond_e156605: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_m0 < p.p442)) { 1.0 } else { 0.0 };
            assign104430_cond_e156605 != 0.0
        } {
            assign104430_loop_guard += 1;
            assert!(assign104430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104430_body0_e156618, assign104430_body0_e156618_d_n0, assign104430_body0_e156618_d_n2, assign104430_body0_e156618_d_n4, assign104430_body0_e156618_d_n5, assign104430_body0_e156618_d_n6, assign104430_body0_e156618_d_n7, assign104430_body0_e156618_d_n8, assign104430_body0_e156618_d_n9, assign104430_body0_e156618_d_n10, assign104430_body0_e156618_d_n11, assign104430_body0_e156618_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104430_body0_e156616: f64 = (locals.var_xp * locals.var_x2);
        (assign104430_body0_e156616, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign104430_body0_e156618;
            locals.var_xp_dn0 = assign104430_body0_e156618_d_n0;
            locals.var_xp_dn2 = assign104430_body0_e156618_d_n2;
            locals.var_xp_dn4 = assign104430_body0_e156618_d_n4;
            locals.var_xp_dn5 = assign104430_body0_e156618_d_n5;
            locals.var_xp_dn6 = assign104430_body0_e156618_d_n6;
            locals.var_xp_dn7 = assign104430_body0_e156618_d_n7;
            locals.var_xp_dn8 = assign104430_body0_e156618_d_n8;
            locals.var_xp_dn9 = assign104430_body0_e156618_d_n9;
            locals.var_xp_dn10 = assign104430_body0_e156618_d_n10;
            locals.var_xp_dn11 = assign104430_body0_e156618_d_n11;
            locals.var_xp_dn14 = assign104430_body0_e156618_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign104430_body1_e156631, assign104430_body1_e156631_d_n0, assign104430_body1_e156631_d_n2, assign104430_body1_e156631_d_n4, assign104430_body1_e156631_d_n5, assign104430_body1_e156631_d_n6, assign104430_body1_e156631_d_n7, assign104430_body1_e156631_d_n8, assign104430_body1_e156631_d_n9, assign104430_body1_e156631_d_n10, assign104430_body1_e156631_d_n11, assign104430_body1_e156631_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104430_body1_e156629: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign104430_body1_e156629, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign104430_body1_e156631;
            locals.var_xmp_dn0 = assign104430_body1_e156631_d_n0;
            locals.var_xmp_dn2 = assign104430_body1_e156631_d_n2;
            locals.var_xmp_dn4 = assign104430_body1_e156631_d_n4;
            locals.var_xmp_dn5 = assign104430_body1_e156631_d_n5;
            locals.var_xmp_dn6 = assign104430_body1_e156631_d_n6;
            locals.var_xmp_dn7 = assign104430_body1_e156631_d_n7;
            locals.var_xmp_dn8 = assign104430_body1_e156631_d_n8;
            locals.var_xmp_dn9 = assign104430_body1_e156631_d_n9;
            locals.var_xmp_dn10 = assign104430_body1_e156631_d_n10;
            locals.var_xmp_dn11 = assign104430_body1_e156631_d_n11;
            locals.var_xmp_dn14 = assign104430_body1_e156631_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign104430_body2_e156644,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104430_body2_e156642: f64 = (locals.var_m0 + 1.0);
        (assign104430_body2_e156642,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104430_body2_e156644;
            locals.var_m0_rv = 0.0;
        }

        let (assign104440_e156657, assign104440_e156657_d_n0, assign104440_e156657_d_n2, assign104440_e156657_d_n4, assign104440_e156657_d_n5, assign104440_e156657_d_n6, assign104440_e156657_d_n7, assign104440_e156657_d_n8, assign104440_e156657_d_n9, assign104440_e156657_d_n10, assign104440_e156657_d_n11, assign104440_e156657_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104440_e156655: f64 = (locals.var_xp + locals.var_xmp);
        (assign104440_e156655, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign104440_e156657;
        locals.var_arg_dn0 = assign104440_e156657_d_n0;
        locals.var_arg_dn2 = assign104440_e156657_d_n2;
        locals.var_arg_dn4 = assign104440_e156657_d_n4;
        locals.var_arg_dn5 = assign104440_e156657_d_n5;
        locals.var_arg_dn6 = assign104440_e156657_d_n6;
        locals.var_arg_dn7 = assign104440_e156657_d_n7;
        locals.var_arg_dn8 = assign104440_e156657_d_n8;
        locals.var_arg_dn9 = assign104440_e156657_d_n9;
        locals.var_arg_dn10 = assign104440_e156657_d_n10;
        locals.var_arg_dn11 = assign104440_e156657_d_n11;
        locals.var_arg_dn14 = assign104440_e156657_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign104450_e156668, assign104450_e156668_d_n0, assign104450_e156668_d_n2, assign104450_e156668_d_n4, assign104450_e156668_d_n5, assign104450_e156668_d_n6, assign104450_e156668_d_n7, assign104450_e156668_d_n8, assign104450_e156668_d_n9, assign104450_e156668_d_n10, assign104450_e156668_d_n11, assign104450_e156668_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104450_e156668;
        locals.var_dnm_dn0 = assign104450_e156668_d_n0;
        locals.var_dnm_dn2 = assign104450_e156668_d_n2;
        locals.var_dnm_dn4 = assign104450_e156668_d_n4;
        locals.var_dnm_dn5 = assign104450_e156668_d_n5;
        locals.var_dnm_dn6 = assign104450_e156668_d_n6;
        locals.var_dnm_dn7 = assign104450_e156668_d_n7;
        locals.var_dnm_dn8 = assign104450_e156668_d_n8;
        locals.var_dnm_dn9 = assign104450_e156668_d_n9;
        locals.var_dnm_dn10 = assign104450_e156668_d_n10;
        locals.var_dnm_dn11 = assign104450_e156668_d_n11;
        locals.var_dnm_dn14 = assign104450_e156668_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign104460_e156683: f64 = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2377 = assign104460_e156683;
        locals.var_guard2377_rv = 0.0;

        let assign104470_e156686: f64 = if p.p442 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2378 = assign104470_e156686;
        locals.var_guard2378_rv = 0.0;

        let (assign104480_e156701,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104480_e156701;
        locals.var_mm_rv = 0.0;

        let assign104490_e156704: f64 = if p.p442 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2379 = assign104490_e156704;
        locals.var_guard2379_rv = 0.0;

        let (assign104500_e156722,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104500_e156722;
        locals.var_mm_rv = 0.0;

        let assign104510_e156725: f64 = if p.p442 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2380 = assign104510_e156725;
        locals.var_guard2380_rv = 0.0;

        let (assign104520_e156746,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104520_e156746;
        locals.var_mm_rv = 0.0;

        let assign104530_e156749: f64 = if p.p442 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2381 = assign104530_e156749;
        locals.var_guard2381_rv = 0.0;

        let (assign104540_e156773,) = {
    if (((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 == 0.0)) && (locals.var_guard2380 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104540_e156773;
        locals.var_mm_rv = 0.0;

        let (assign104550_e156786,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104550_e156786;
        locals.var_m0_rv = 0.0;

        let mut assign104560_loop_guard: usize = 0;
        while {
            let assign104560_cond_e156800: f64 = if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign104560_cond_e156800 != 0.0
        } {
            assign104560_loop_guard += 1;
            assert!(assign104560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104560_body0_e156814, assign104560_body0_e156814_d_n0, assign104560_body0_e156814_d_n2, assign104560_body0_e156814_d_n4, assign104560_body0_e156814_d_n5, assign104560_body0_e156814_d_n6, assign104560_body0_e156814_d_n7, assign104560_body0_e156814_d_n8, assign104560_body0_e156814_d_n9, assign104560_body0_e156814_d_n10, assign104560_body0_e156814_d_n11, assign104560_body0_e156814_d_n14,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) {
        let assign104560_body0_e156812: f64 = (locals.var_dnm).sqrt();
        (assign104560_body0_e156812, (locals.var_dnm_dn0 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn2 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn4 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn5 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn6 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn7 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn8 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn9 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn10 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn11 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn14 / (2.0 * assign104560_body0_e156812)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign104560_body0_e156814;
            locals.var_dnm_dn0 = assign104560_body0_e156814_d_n0;
            locals.var_dnm_dn2 = assign104560_body0_e156814_d_n2;
            locals.var_dnm_dn4 = assign104560_body0_e156814_d_n4;
            locals.var_dnm_dn5 = assign104560_body0_e156814_d_n5;
            locals.var_dnm_dn6 = assign104560_body0_e156814_d_n6;
            locals.var_dnm_dn7 = assign104560_body0_e156814_d_n7;
            locals.var_dnm_dn8 = assign104560_body0_e156814_d_n8;
            locals.var_dnm_dn9 = assign104560_body0_e156814_d_n9;
            locals.var_dnm_dn10 = assign104560_body0_e156814_d_n10;
            locals.var_dnm_dn11 = assign104560_body0_e156814_d_n11;
            locals.var_dnm_dn14 = assign104560_body0_e156814_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign104560_body1_e156829,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) {
        let assign104560_body1_e156827: f64 = (locals.var_m0 + 1.0);
        (assign104560_body1_e156827,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104560_body1_e156829;
            locals.var_m0_rv = 0.0;
        }

        let (assign104570_e156854, assign104570_e156854_d_n0, assign104570_e156854_d_n2, assign104570_e156854_d_n4, assign104570_e156854_d_n5, assign104570_e156854_d_n6, assign104570_e156854_d_n7, assign104570_e156854_d_n8, assign104570_e156854_d_n9, assign104570_e156854_d_n10, assign104570_e156854_d_n11, assign104570_e156854_d_n14,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 == 0.0)) {
        let (assign104570_e156852, assign104570_e156852_d_n0, assign104570_e156852_d_n2, assign104570_e156852_d_n4, assign104570_e156852_d_n5, assign104570_e156852_d_n6, assign104570_e156852_d_n7, assign104570_e156852_d_n8, assign104570_e156852_d_n9, assign104570_e156852_d_n10, assign104570_e156852_d_n11, assign104570_e156852_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104570_e156849: f64 = (2.0 * p.p442);
                let assign104570_e156850: f64 = (1.0 / assign104570_e156849);
                let assign104570_e156851: f64 = (locals.var_dnm).powf(assign104570_e156850);
                (assign104570_e156851, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn0)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn2)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn4)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn5)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn6)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn7)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn8)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn9)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn10)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn11)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((locals.var_dnm).powf(assign104570_e156850 - 1.0) * locals.var_dnm_dn14)) } } else { (assign104570_e156851 * (assign104570_e156850 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign104570_e156852, assign104570_e156852_d_n0, assign104570_e156852_d_n2, assign104570_e156852_d_n4, assign104570_e156852_d_n5, assign104570_e156852_d_n6, assign104570_e156852_d_n7, assign104570_e156852_d_n8, assign104570_e156852_d_n9, assign104570_e156852_d_n10, assign104570_e156852_d_n11, assign104570_e156852_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104570_e156854;
        locals.var_dnm_dn0 = assign104570_e156854_d_n0;
        locals.var_dnm_dn2 = assign104570_e156854_d_n2;
        locals.var_dnm_dn4 = assign104570_e156854_d_n4;
        locals.var_dnm_dn5 = assign104570_e156854_d_n5;
        locals.var_dnm_dn6 = assign104570_e156854_d_n6;
        locals.var_dnm_dn7 = assign104570_e156854_d_n7;
        locals.var_dnm_dn8 = assign104570_e156854_d_n8;
        locals.var_dnm_dn9 = assign104570_e156854_d_n9;
        locals.var_dnm_dn10 = assign104570_e156854_d_n10;
        locals.var_dnm_dn11 = assign104570_e156854_d_n11;
        locals.var_dnm_dn14 = assign104570_e156854_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign104580_e156867, assign104580_e156867_d_n0, assign104580_e156867_d_n2, assign104580_e156867_d_n4, assign104580_e156867_d_n5, assign104580_e156867_d_n6, assign104580_e156867_d_n7, assign104580_e156867_d_n8, assign104580_e156867_d_n9, assign104580_e156867_d_n10, assign104580_e156867_d_n11, assign104580_e156867_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104580_e156865: f64 = (1.0 / locals.var_dnm);
        (assign104580_e156865, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104580_e156867;
        locals.var_dnm_dn0 = assign104580_e156867_d_n0;
        locals.var_dnm_dn2 = assign104580_e156867_d_n2;
        locals.var_dnm_dn4 = assign104580_e156867_d_n4;
        locals.var_dnm_dn5 = assign104580_e156867_d_n5;
        locals.var_dnm_dn6 = assign104580_e156867_d_n6;
        locals.var_dnm_dn7 = assign104580_e156867_d_n7;
        locals.var_dnm_dn8 = assign104580_e156867_d_n8;
        locals.var_dnm_dn9 = assign104580_e156867_d_n9;
        locals.var_dnm_dn10 = assign104580_e156867_d_n10;
        locals.var_dnm_dn11 = assign104580_e156867_d_n11;
        locals.var_dnm_dn14 = assign104580_e156867_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign104590_e156884, assign104590_e156884_d_n0, assign104590_e156884_d_n2, assign104590_e156884_d_n4, assign104590_e156884_d_n5, assign104590_e156884_d_n6, assign104590_e156884_d_n7, assign104590_e156884_d_n8, assign104590_e156884_d_n9, assign104590_e156884_d_n10, assign104590_e156884_d_n11, assign104590_e156884_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104590_e156879: f64 = (locals.var_noverd * p.p441);
        let assign104590_e156880: f64 = (locals.var_tmf1 * assign104590_e156879);
        let assign104590_e156882: f64 = (assign104590_e156880 * locals.var_dnm);
        (assign104590_e156882, (((locals.var_tmf1_dn0 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign104590_e156879) * locals.var_dnm) + (assign104590_e156880 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign104590_e156884;
        locals.var_tmf0_dn0 = assign104590_e156884_d_n0;
        locals.var_tmf0_dn2 = assign104590_e156884_d_n2;
        locals.var_tmf0_dn4 = assign104590_e156884_d_n4;
        locals.var_tmf0_dn5 = assign104590_e156884_d_n5;
        locals.var_tmf0_dn6 = assign104590_e156884_d_n6;
        locals.var_tmf0_dn7 = assign104590_e156884_d_n7;
        locals.var_tmf0_dn8 = assign104590_e156884_d_n8;
        locals.var_tmf0_dn9 = assign104590_e156884_d_n9;
        locals.var_tmf0_dn10 = assign104590_e156884_d_n10;
        locals.var_tmf0_dn11 = assign104590_e156884_d_n11;
        locals.var_tmf0_dn14 = assign104590_e156884_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign104600_e156903, assign104600_e156903_d_n0, assign104600_e156903_d_n2, assign104600_e156903_d_n4, assign104600_e156903_d_n5, assign104600_e156903_d_n6, assign104600_e156903_d_n7, assign104600_e156903_d_n8, assign104600_e156903_d_n9, assign104600_e156903_d_n10, assign104600_e156903_d_n11, assign104600_e156903_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104600_e156895: f64 = (locals.var_noverd * p.p441);
        let assign104600_e156897: f64 = (assign104600_e156895 * locals.var_xmp);
        let assign104600_e156899: f64 = (assign104600_e156897 * locals.var_dnm);
        let assign104600_e156901: f64 = (assign104600_e156899 / locals.var_arg);
        (assign104600_e156901, ((((((assign104600_e156895 * locals.var_xmp_dn0) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn0)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn2) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn2)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn4) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn4)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn5) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn5)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn6) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn6)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn7) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn7)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn8) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn8)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn9) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn9)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn10) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn10)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn11) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn11)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign104600_e156895 * locals.var_xmp_dn14) * locals.var_dnm) + (assign104600_e156897 * locals.var_dnm_dn14)) * locals.var_arg) - (assign104600_e156899 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104600_e156903;
        locals.var_t0_dn0 = assign104600_e156903_d_n0;
        locals.var_t0_dn2 = assign104600_e156903_d_n2;
        locals.var_t0_dn4 = assign104600_e156903_d_n4;
        locals.var_t0_dn5 = assign104600_e156903_d_n5;
        locals.var_t0_dn6 = assign104600_e156903_d_n6;
        locals.var_t0_dn7 = assign104600_e156903_d_n7;
        locals.var_t0_dn8 = assign104600_e156903_d_n8;
        locals.var_t0_dn9 = assign104600_e156903_d_n9;
        locals.var_t0_dn10 = assign104600_e156903_d_n10;
        locals.var_t0_dn11 = assign104600_e156903_d_n11;
        locals.var_t0_dn14 = assign104600_e156903_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104610_e156922, assign104610_e156922_d_n0, assign104610_e156922_d_n2, assign104610_e156922_d_n4, assign104610_e156922_d_n5, assign104610_e156922_d_n6, assign104610_e156922_d_n7, assign104610_e156922_d_n8, assign104610_e156922_d_n9, assign104610_e156922_d_n10, assign104610_e156922_d_n11, assign104610_e156922_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104610_e156914: f64 = (locals.var_noverd * p.p440);
        let assign104610_e156917: f64 = (locals.var_noverd * p.p441);
        let assign104610_e156918: f64 = (assign104610_e156914 - assign104610_e156917);
        let assign104610_e156920: f64 = (assign104610_e156918 + locals.var_tmf0);
        (assign104610_e156920, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104610_e156922;
        locals.var_t2_dn0 = assign104610_e156922_d_n0;
        locals.var_t2_dn2 = assign104610_e156922_d_n2;
        locals.var_t2_dn4 = assign104610_e156922_d_n4;
        locals.var_t2_dn5 = assign104610_e156922_d_n5;
        locals.var_t2_dn6 = assign104610_e156922_d_n6;
        locals.var_t2_dn7 = assign104610_e156922_d_n7;
        locals.var_t2_dn8 = assign104610_e156922_d_n8;
        locals.var_t2_dn9 = assign104610_e156922_d_n9;
        locals.var_t2_dn10 = assign104610_e156922_d_n10;
        locals.var_t2_dn11 = assign104610_e156922_d_n11;
        locals.var_t2_dn14 = assign104610_e156922_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign104620_e156933, assign104620_e156933_d_n0, assign104620_e156933_d_n2, assign104620_e156933_d_n4, assign104620_e156933_d_n5, assign104620_e156933_d_n6, assign104620_e156933_d_n7, assign104620_e156933_d_n8, assign104620_e156933_d_n9, assign104620_e156933_d_n10, assign104620_e156933_d_n11, assign104620_e156933_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104620_e156933;
        locals.var_t0_dn0 = assign104620_e156933_d_n0;
        locals.var_t0_dn2 = assign104620_e156933_d_n2;
        locals.var_t0_dn4 = assign104620_e156933_d_n4;
        locals.var_t0_dn5 = assign104620_e156933_d_n5;
        locals.var_t0_dn6 = assign104620_e156933_d_n6;
        locals.var_t0_dn7 = assign104620_e156933_d_n7;
        locals.var_t0_dn8 = assign104620_e156933_d_n8;
        locals.var_t0_dn9 = assign104620_e156933_d_n9;
        locals.var_t0_dn10 = assign104620_e156933_d_n10;
        locals.var_t0_dn11 = assign104620_e156933_d_n11;
        locals.var_t0_dn14 = assign104620_e156933_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104630_e156945, assign104630_e156945_d_n0, assign104630_e156945_d_n2, assign104630_e156945_d_n4, assign104630_e156945_d_n5, assign104630_e156945_d_n6, assign104630_e156945_d_n7, assign104630_e156945_d_n8, assign104630_e156945_d_n9, assign104630_e156945_d_n10, assign104630_e156945_d_n11, assign104630_e156945_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 == 0.0)) {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104630_e156945;
        locals.var_t2_dn0 = assign104630_e156945_d_n0;
        locals.var_t2_dn2 = assign104630_e156945_d_n2;
        locals.var_t2_dn4 = assign104630_e156945_d_n4;
        locals.var_t2_dn5 = assign104630_e156945_d_n5;
        locals.var_t2_dn6 = assign104630_e156945_d_n6;
        locals.var_t2_dn7 = assign104630_e156945_d_n7;
        locals.var_t2_dn8 = assign104630_e156945_d_n8;
        locals.var_t2_dn9 = assign104630_e156945_d_n9;
        locals.var_t2_dn10 = assign104630_e156945_d_n10;
        locals.var_t2_dn11 = assign104630_e156945_d_n11;
        locals.var_t2_dn14 = assign104630_e156945_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign104640_e156957, assign104640_e156957_d_n0, assign104640_e156957_d_n2, assign104640_e156957_d_n4, assign104640_e156957_d_n5, assign104640_e156957_d_n6, assign104640_e156957_d_n7, assign104640_e156957_d_n8, assign104640_e156957_d_n9, assign104640_e156957_d_n10, assign104640_e156957_d_n11, assign104640_e156957_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104640_e156957;
        locals.var_t0_dn0 = assign104640_e156957_d_n0;
        locals.var_t0_dn2 = assign104640_e156957_d_n2;
        locals.var_t0_dn4 = assign104640_e156957_d_n4;
        locals.var_t0_dn5 = assign104640_e156957_d_n5;
        locals.var_t0_dn6 = assign104640_e156957_d_n6;
        locals.var_t0_dn7 = assign104640_e156957_d_n7;
        locals.var_t0_dn8 = assign104640_e156957_d_n8;
        locals.var_t0_dn9 = assign104640_e156957_d_n9;
        locals.var_t0_dn10 = assign104640_e156957_d_n10;
        locals.var_t0_dn11 = assign104640_e156957_d_n11;
        locals.var_t0_dn14 = assign104640_e156957_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104650_e156966, assign104650_e156966_d_n0, assign104650_e156966_d_n2, assign104650_e156966_d_n4, assign104650_e156966_d_n5, assign104650_e156966_d_n6, assign104650_e156966_d_n7, assign104650_e156966_d_n8, assign104650_e156966_d_n9, assign104650_e156966_d_n10, assign104650_e156966_d_n11, assign104650_e156966_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    }
};
        locals.var_carr = assign104650_e156966;
        locals.var_carr_dn0 = assign104650_e156966_d_n0;
        locals.var_carr_dn2 = assign104650_e156966_d_n2;
        locals.var_carr_dn4 = assign104650_e156966_d_n4;
        locals.var_carr_dn5 = assign104650_e156966_d_n5;
        locals.var_carr_dn6 = assign104650_e156966_d_n6;
        locals.var_carr_dn7 = assign104650_e156966_d_n7;
        locals.var_carr_dn8 = assign104650_e156966_d_n8;
        locals.var_carr_dn9 = assign104650_e156966_d_n9;
        locals.var_carr_dn10 = assign104650_e156966_d_n10;
        locals.var_carr_dn11 = assign104650_e156966_d_n11;
        locals.var_carr_dn14 = assign104650_e156966_d_n14;
        locals.var_carr_rv = 0.0;

        let (assign104660_e156974, assign104660_e156974_d_n0, assign104660_e156974_d_n2, assign104660_e156974_d_n4, assign104660_e156974_d_n5, assign104660_e156974_d_n6, assign104660_e156974_d_n7, assign104660_e156974_d_n8, assign104660_e156974_d_n9, assign104660_e156974_d_n10, assign104660_e156974_d_n11, assign104660_e156974_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104660_e156972: f64 = (-locals.var_rd_ps0ld);
        (assign104660_e156972, (-locals.var_rd_ps0ld_dn0), (-locals.var_rd_ps0ld_dn2), (-locals.var_rd_ps0ld_dn4), (-locals.var_rd_ps0ld_dn5), (-locals.var_rd_ps0ld_dn6), (-locals.var_rd_ps0ld_dn7), (-locals.var_rd_ps0ld_dn8), (-locals.var_rd_ps0ld_dn9), (-locals.var_rd_ps0ld_dn10), (-locals.var_rd_ps0ld_dn11), (-locals.var_rd_ps0ld_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104660_e156974;
        locals.var_t0_dn0 = assign104660_e156974_d_n0;
        locals.var_t0_dn2 = assign104660_e156974_d_n2;
        locals.var_t0_dn4 = assign104660_e156974_d_n4;
        locals.var_t0_dn5 = assign104660_e156974_d_n5;
        locals.var_t0_dn6 = assign104660_e156974_d_n6;
        locals.var_t0_dn7 = assign104660_e156974_d_n7;
        locals.var_t0_dn8 = assign104660_e156974_d_n8;
        locals.var_t0_dn9 = assign104660_e156974_d_n9;
        locals.var_t0_dn10 = assign104660_e156974_d_n10;
        locals.var_t0_dn11 = assign104660_e156974_d_n11;
        locals.var_t0_dn14 = assign104660_e156974_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104670_e156990, assign104670_e156990_d_n0, assign104670_e156990_d_n2, assign104670_e156990_d_n4, assign104670_e156990_d_n5, assign104670_e156990_d_n6, assign104670_e156990_d_n7, assign104670_e156990_d_n8, assign104670_e156990_d_n9, assign104670_e156990_d_n10, assign104670_e156990_d_n11, assign104670_e156990_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104670_e156981: f64 = (locals.var_t0 * locals.var_t0);
        let assign104670_e156984: f64 = (4.0 * 0.01);
        let assign104670_e156986: f64 = (assign104670_e156984 * 0.01);
        let assign104670_e156987: f64 = (assign104670_e156981 + assign104670_e156986);
        let assign104670_e156988: f64 = (assign104670_e156987).sqrt();
        (assign104670_e156988, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign104670_e156988)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign104670_e156988)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104670_e156990;
        locals.var_tmf2_dn0 = assign104670_e156990_d_n0;
        locals.var_tmf2_dn2 = assign104670_e156990_d_n2;
        locals.var_tmf2_dn4 = assign104670_e156990_d_n4;
        locals.var_tmf2_dn5 = assign104670_e156990_d_n5;
        locals.var_tmf2_dn6 = assign104670_e156990_d_n6;
        locals.var_tmf2_dn7 = assign104670_e156990_d_n7;
        locals.var_tmf2_dn8 = assign104670_e156990_d_n8;
        locals.var_tmf2_dn9 = assign104670_e156990_d_n9;
        locals.var_tmf2_dn10 = assign104670_e156990_d_n10;
        locals.var_tmf2_dn11 = assign104670_e156990_d_n11;
        locals.var_tmf2_dn14 = assign104670_e156990_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104680_e157003, assign104680_e157003_d_n0, assign104680_e157003_d_n2, assign104680_e157003_d_n4, assign104680_e157003_d_n5, assign104680_e157003_d_n6, assign104680_e157003_d_n7, assign104680_e157003_d_n8, assign104680_e157003_d_n9, assign104680_e157003_d_n10, assign104680_e157003_d_n11, assign104680_e157003_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104680_e156999: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign104680_e157000: f64 = (1.0 + assign104680_e156999);
        let assign104680_e157001: f64 = (0.5 * assign104680_e157000);
        (assign104680_e157001, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104680_e157003;
        locals.var_t9_dn0 = assign104680_e157003_d_n0;
        locals.var_t9_dn2 = assign104680_e157003_d_n2;
        locals.var_t9_dn4 = assign104680_e157003_d_n4;
        locals.var_t9_dn5 = assign104680_e157003_d_n5;
        locals.var_t9_dn6 = assign104680_e157003_d_n6;
        locals.var_t9_dn7 = assign104680_e157003_d_n7;
        locals.var_t9_dn8 = assign104680_e157003_d_n8;
        locals.var_t9_dn9 = assign104680_e157003_d_n9;
        locals.var_t9_dn10 = assign104680_e157003_d_n10;
        locals.var_t9_dn11 = assign104680_e157003_d_n11;
        locals.var_t9_dn14 = assign104680_e157003_d_n14;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_401(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104690_e157014, assign104690_e157014_d_n0, assign104690_e157014_d_n2, assign104690_e157014_d_n4, assign104690_e157014_d_n5, assign104690_e157014_d_n6, assign104690_e157014_d_n7, assign104690_e157014_d_n8, assign104690_e157014_d_n9, assign104690_e157014_d_n10, assign104690_e157014_d_n11, assign104690_e157014_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104690_e157011: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign104690_e157012: f64 = (0.5 * assign104690_e157011);
        (assign104690_e157012, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104690_e157014;
        locals.var_t0_dn0 = assign104690_e157014_d_n0;
        locals.var_t0_dn2 = assign104690_e157014_d_n2;
        locals.var_t0_dn4 = assign104690_e157014_d_n4;
        locals.var_t0_dn5 = assign104690_e157014_d_n5;
        locals.var_t0_dn6 = assign104690_e157014_d_n6;
        locals.var_t0_dn7 = assign104690_e157014_d_n7;
        locals.var_t0_dn8 = assign104690_e157014_d_n8;
        locals.var_t0_dn9 = assign104690_e157014_d_n9;
        locals.var_t0_dn10 = assign104690_e157014_d_n10;
        locals.var_t0_dn11 = assign104690_e157014_d_n11;
        locals.var_t0_dn14 = assign104690_e157014_d_n14;
        locals.var_t0_rv = 0.0;

        let assign104700_e157017: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2382 = assign104700_e157017;
        locals.var_guard2382_rv = 0.0;

        let (assign104710_e157026, assign104710_e157026_d_n0, assign104710_e157026_d_n2, assign104710_e157026_d_n4, assign104710_e157026_d_n5, assign104710_e157026_d_n6, assign104710_e157026_d_n7, assign104710_e157026_d_n8, assign104710_e157026_d_n9, assign104710_e157026_d_n10, assign104710_e157026_d_n11, assign104710_e157026_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104710_e157026;
        locals.var_t0_dn0 = assign104710_e157026_d_n0;
        locals.var_t0_dn2 = assign104710_e157026_d_n2;
        locals.var_t0_dn4 = assign104710_e157026_d_n4;
        locals.var_t0_dn5 = assign104710_e157026_d_n5;
        locals.var_t0_dn6 = assign104710_e157026_d_n6;
        locals.var_t0_dn7 = assign104710_e157026_d_n7;
        locals.var_t0_dn8 = assign104710_e157026_d_n8;
        locals.var_t0_dn9 = assign104710_e157026_d_n9;
        locals.var_t0_dn10 = assign104710_e157026_d_n10;
        locals.var_t0_dn11 = assign104710_e157026_d_n11;
        locals.var_t0_dn14 = assign104710_e157026_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104720_e157035, assign104720_e157035_d_n0, assign104720_e157035_d_n2, assign104720_e157035_d_n4, assign104720_e157035_d_n5, assign104720_e157035_d_n6, assign104720_e157035_d_n7, assign104720_e157035_d_n8, assign104720_e157035_d_n9, assign104720_e157035_d_n10, assign104720_e157035_d_n11, assign104720_e157035_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104720_e157035;
        locals.var_t9_dn0 = assign104720_e157035_d_n0;
        locals.var_t9_dn2 = assign104720_e157035_d_n2;
        locals.var_t9_dn4 = assign104720_e157035_d_n4;
        locals.var_t9_dn5 = assign104720_e157035_d_n5;
        locals.var_t9_dn6 = assign104720_e157035_d_n6;
        locals.var_t9_dn7 = assign104720_e157035_d_n7;
        locals.var_t9_dn8 = assign104720_e157035_d_n8;
        locals.var_t9_dn9 = assign104720_e157035_d_n9;
        locals.var_t9_dn10 = assign104720_e157035_d_n10;
        locals.var_t9_dn11 = assign104720_e157035_d_n11;
        locals.var_t9_dn14 = assign104720_e157035_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign104730_e157046, assign104730_e157046_d_n0, assign104730_e157046_d_n2, assign104730_e157046_d_n4, assign104730_e157046_d_n5, assign104730_e157046_d_n6, assign104730_e157046_d_n7, assign104730_e157046_d_n8, assign104730_e157046_d_n9, assign104730_e157046_d_n10, assign104730_e157046_d_n11, assign104730_e157046_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104730_e157043: f64 = (10.0 * 2.220446049250313e-16);
        let assign104730_e157044: f64 = (locals.var_t0 + assign104730_e157043);
        (assign104730_e157044, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104730_e157046;
        locals.var_t0_dn0 = assign104730_e157046_d_n0;
        locals.var_t0_dn2 = assign104730_e157046_d_n2;
        locals.var_t0_dn4 = assign104730_e157046_d_n4;
        locals.var_t0_dn5 = assign104730_e157046_d_n5;
        locals.var_t0_dn6 = assign104730_e157046_d_n6;
        locals.var_t0_dn7 = assign104730_e157046_d_n7;
        locals.var_t0_dn8 = assign104730_e157046_d_n8;
        locals.var_t0_dn9 = assign104730_e157046_d_n9;
        locals.var_t0_dn10 = assign104730_e157046_d_n10;
        locals.var_t0_dn11 = assign104730_e157046_d_n11;
        locals.var_t0_dn14 = assign104730_e157046_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104740_e157056, assign104740_e157056_d_n0, assign104740_e157056_d_n2, assign104740_e157056_d_n4, assign104740_e157056_d_n5, assign104740_e157056_d_n6, assign104740_e157056_d_n7, assign104740_e157056_d_n8, assign104740_e157056_d_n9, assign104740_e157056_d_n10, assign104740_e157056_d_n11, assign104740_e157056_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104740_e157053: f64 = (locals.var_kdep * locals.var_t0);
        let assign104740_e157054: f64 = (assign104740_e157053).sqrt();
        (assign104740_e157054, ((locals.var_kdep * locals.var_t0_dn0) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn2) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn4) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn5) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn6) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn7) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn8) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn9) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn10) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn11) / (2.0 * assign104740_e157054)), ((locals.var_kdep * locals.var_t0_dn14) / (2.0 * assign104740_e157054)),)
    } else {
        (locals.var_wdepl, locals.var_wdepl_dn0, locals.var_wdepl_dn2, locals.var_wdepl_dn4, locals.var_wdepl_dn5, locals.var_wdepl_dn6, locals.var_wdepl_dn7, locals.var_wdepl_dn8, locals.var_wdepl_dn9, locals.var_wdepl_dn10, locals.var_wdepl_dn11, locals.var_wdepl_dn14,)
    }
};
        locals.var_wdepl = assign104740_e157056;
        locals.var_wdepl_dn0 = assign104740_e157056_d_n0;
        locals.var_wdepl_dn2 = assign104740_e157056_d_n2;
        locals.var_wdepl_dn4 = assign104740_e157056_d_n4;
        locals.var_wdepl_dn5 = assign104740_e157056_d_n5;
        locals.var_wdepl_dn6 = assign104740_e157056_d_n6;
        locals.var_wdepl_dn7 = assign104740_e157056_d_n7;
        locals.var_wdepl_dn8 = assign104740_e157056_d_n8;
        locals.var_wdepl_dn9 = assign104740_e157056_d_n9;
        locals.var_wdepl_dn10 = assign104740_e157056_d_n10;
        locals.var_wdepl_dn11 = assign104740_e157056_d_n11;
        locals.var_wdepl_dn14 = assign104740_e157056_d_n14;
        locals.var_wdepl_rv = 0.0;

        let (assign104750_e157067, assign104750_e157067_d_n0, assign104750_e157067_d_n2, assign104750_e157067_d_n4, assign104750_e157067_d_n5, assign104750_e157067_d_n6, assign104750_e157067_d_n7, assign104750_e157067_d_n8, assign104750_e157067_d_n9, assign104750_e157067_d_n10, assign104750_e157067_d_n11, assign104750_e157067_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104750_e157063: f64 = (locals.var_vds__blk2356 - locals.var_vbs__blk2357);
        let assign104750_e157065: f64 = (assign104750_e157063 + p.p137);
        (assign104750_e157065, 0.0, 0.0, 0.0, 0.0, locals.var_vds__blk2356_dn6, 0.0, (locals.var_vds__blk2356_dn8 - locals.var_vbs__blk2357_dn8), (-locals.var_vbs__blk2357_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104750_e157067;
        locals.var_t2_dn0 = assign104750_e157067_d_n0;
        locals.var_t2_dn2 = assign104750_e157067_d_n2;
        locals.var_t2_dn4 = assign104750_e157067_d_n4;
        locals.var_t2_dn5 = assign104750_e157067_d_n5;
        locals.var_t2_dn6 = assign104750_e157067_d_n6;
        locals.var_t2_dn7 = assign104750_e157067_d_n7;
        locals.var_t2_dn8 = assign104750_e157067_d_n8;
        locals.var_t2_dn9 = assign104750_e157067_d_n9;
        locals.var_t2_dn10 = assign104750_e157067_d_n10;
        locals.var_t2_dn11 = assign104750_e157067_d_n11;
        locals.var_t2_dn14 = assign104750_e157067_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign104760_e157083, assign104760_e157083_d_n0, assign104760_e157083_d_n2, assign104760_e157083_d_n4, assign104760_e157083_d_n5, assign104760_e157083_d_n6, assign104760_e157083_d_n7, assign104760_e157083_d_n8, assign104760_e157083_d_n9, assign104760_e157083_d_n10, assign104760_e157083_d_n11, assign104760_e157083_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104760_e157074: f64 = (locals.var_t2 * locals.var_t2);
        let assign104760_e157077: f64 = (4.0 * 0.01);
        let assign104760_e157079: f64 = (assign104760_e157077 * 0.01);
        let assign104760_e157080: f64 = (assign104760_e157074 + assign104760_e157079);
        let assign104760_e157081: f64 = (assign104760_e157080).sqrt();
        (assign104760_e157081, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign104760_e157081)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign104760_e157081)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104760_e157083;
        locals.var_tmf2_dn0 = assign104760_e157083_d_n0;
        locals.var_tmf2_dn2 = assign104760_e157083_d_n2;
        locals.var_tmf2_dn4 = assign104760_e157083_d_n4;
        locals.var_tmf2_dn5 = assign104760_e157083_d_n5;
        locals.var_tmf2_dn6 = assign104760_e157083_d_n6;
        locals.var_tmf2_dn7 = assign104760_e157083_d_n7;
        locals.var_tmf2_dn8 = assign104760_e157083_d_n8;
        locals.var_tmf2_dn9 = assign104760_e157083_d_n9;
        locals.var_tmf2_dn10 = assign104760_e157083_d_n10;
        locals.var_tmf2_dn11 = assign104760_e157083_d_n11;
        locals.var_tmf2_dn14 = assign104760_e157083_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104770_e157096, assign104770_e157096_d_n0, assign104770_e157096_d_n2, assign104770_e157096_d_n4, assign104770_e157096_d_n5, assign104770_e157096_d_n6, assign104770_e157096_d_n7, assign104770_e157096_d_n8, assign104770_e157096_d_n9, assign104770_e157096_d_n10, assign104770_e157096_d_n11, assign104770_e157096_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104770_e157092: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign104770_e157093: f64 = (1.0 + assign104770_e157092);
        let assign104770_e157094: f64 = (0.5 * assign104770_e157093);
        (assign104770_e157094, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104770_e157096;
        locals.var_t9_dn0 = assign104770_e157096_d_n0;
        locals.var_t9_dn2 = assign104770_e157096_d_n2;
        locals.var_t9_dn4 = assign104770_e157096_d_n4;
        locals.var_t9_dn5 = assign104770_e157096_d_n5;
        locals.var_t9_dn6 = assign104770_e157096_d_n6;
        locals.var_t9_dn7 = assign104770_e157096_d_n7;
        locals.var_t9_dn8 = assign104770_e157096_d_n8;
        locals.var_t9_dn9 = assign104770_e157096_d_n9;
        locals.var_t9_dn10 = assign104770_e157096_d_n10;
        locals.var_t9_dn11 = assign104770_e157096_d_n11;
        locals.var_t9_dn14 = assign104770_e157096_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign104780_e157107, assign104780_e157107_d_n0, assign104780_e157107_d_n2, assign104780_e157107_d_n4, assign104780_e157107_d_n5, assign104780_e157107_d_n6, assign104780_e157107_d_n7, assign104780_e157107_d_n8, assign104780_e157107_d_n9, assign104780_e157107_d_n10, assign104780_e157107_d_n11, assign104780_e157107_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104780_e157104: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign104780_e157105: f64 = (0.5 * assign104780_e157104);
        (assign104780_e157105, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104780_e157107;
        locals.var_t2_dn0 = assign104780_e157107_d_n0;
        locals.var_t2_dn2 = assign104780_e157107_d_n2;
        locals.var_t2_dn4 = assign104780_e157107_d_n4;
        locals.var_t2_dn5 = assign104780_e157107_d_n5;
        locals.var_t2_dn6 = assign104780_e157107_d_n6;
        locals.var_t2_dn7 = assign104780_e157107_d_n7;
        locals.var_t2_dn8 = assign104780_e157107_d_n8;
        locals.var_t2_dn9 = assign104780_e157107_d_n9;
        locals.var_t2_dn10 = assign104780_e157107_d_n10;
        locals.var_t2_dn11 = assign104780_e157107_d_n11;
        locals.var_t2_dn14 = assign104780_e157107_d_n14;
        locals.var_t2_rv = 0.0;

        let assign104790_e157110: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2383 = assign104790_e157110;
        locals.var_guard2383_rv = 0.0;

        let (assign104800_e157119, assign104800_e157119_d_n0, assign104800_e157119_d_n2, assign104800_e157119_d_n4, assign104800_e157119_d_n5, assign104800_e157119_d_n6, assign104800_e157119_d_n7, assign104800_e157119_d_n8, assign104800_e157119_d_n9, assign104800_e157119_d_n10, assign104800_e157119_d_n11, assign104800_e157119_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104800_e157119;
        locals.var_t2_dn0 = assign104800_e157119_d_n0;
        locals.var_t2_dn2 = assign104800_e157119_d_n2;
        locals.var_t2_dn4 = assign104800_e157119_d_n4;
        locals.var_t2_dn5 = assign104800_e157119_d_n5;
        locals.var_t2_dn6 = assign104800_e157119_d_n6;
        locals.var_t2_dn7 = assign104800_e157119_d_n7;
        locals.var_t2_dn8 = assign104800_e157119_d_n8;
        locals.var_t2_dn9 = assign104800_e157119_d_n9;
        locals.var_t2_dn10 = assign104800_e157119_d_n10;
        locals.var_t2_dn11 = assign104800_e157119_d_n11;
        locals.var_t2_dn14 = assign104800_e157119_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign104810_e157128, assign104810_e157128_d_n0, assign104810_e157128_d_n2, assign104810_e157128_d_n4, assign104810_e157128_d_n5, assign104810_e157128_d_n6, assign104810_e157128_d_n7, assign104810_e157128_d_n8, assign104810_e157128_d_n9, assign104810_e157128_d_n10, assign104810_e157128_d_n11, assign104810_e157128_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104810_e157128;
        locals.var_t9_dn0 = assign104810_e157128_d_n0;
        locals.var_t9_dn2 = assign104810_e157128_d_n2;
        locals.var_t9_dn4 = assign104810_e157128_d_n4;
        locals.var_t9_dn5 = assign104810_e157128_d_n5;
        locals.var_t9_dn6 = assign104810_e157128_d_n6;
        locals.var_t9_dn7 = assign104810_e157128_d_n7;
        locals.var_t9_dn8 = assign104810_e157128_d_n8;
        locals.var_t9_dn9 = assign104810_e157128_d_n9;
        locals.var_t9_dn10 = assign104810_e157128_d_n10;
        locals.var_t9_dn11 = assign104810_e157128_d_n11;
        locals.var_t9_dn14 = assign104810_e157128_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign104820_e157139, assign104820_e157139_d_n0, assign104820_e157139_d_n2, assign104820_e157139_d_n4, assign104820_e157139_d_n5, assign104820_e157139_d_n6, assign104820_e157139_d_n7, assign104820_e157139_d_n8, assign104820_e157139_d_n9, assign104820_e157139_d_n10, assign104820_e157139_d_n11, assign104820_e157139_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104820_e157136: f64 = (10.0 * 2.220446049250313e-16);
        let assign104820_e157137: f64 = (locals.var_t2 + assign104820_e157136);
        (assign104820_e157137, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104820_e157139;
        locals.var_t2_dn0 = assign104820_e157139_d_n0;
        locals.var_t2_dn2 = assign104820_e157139_d_n2;
        locals.var_t2_dn4 = assign104820_e157139_d_n4;
        locals.var_t2_dn5 = assign104820_e157139_d_n5;
        locals.var_t2_dn6 = assign104820_e157139_d_n6;
        locals.var_t2_dn7 = assign104820_e157139_d_n7;
        locals.var_t2_dn8 = assign104820_e157139_d_n8;
        locals.var_t2_dn9 = assign104820_e157139_d_n9;
        locals.var_t2_dn10 = assign104820_e157139_d_n10;
        locals.var_t2_dn11 = assign104820_e157139_d_n11;
        locals.var_t2_dn14 = assign104820_e157139_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign104830_e157149, assign104830_e157149_d_n0, assign104830_e157149_d_n2, assign104830_e157149_d_n4, assign104830_e157149_d_n5, assign104830_e157149_d_n6, assign104830_e157149_d_n7, assign104830_e157149_d_n8, assign104830_e157149_d_n9, assign104830_e157149_d_n10, assign104830_e157149_d_n11, assign104830_e157149_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104830_e157146: f64 = (locals.var_kjunc * locals.var_t2);
        let assign104830_e157147: f64 = (assign104830_e157146).sqrt();
        (assign104830_e157147, (((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign104830_e157147)), (((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign104830_e157147)),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign104830_e157149;
        locals.var_wjunc0_dn0 = assign104830_e157149_d_n0;
        locals.var_wjunc0_dn2 = assign104830_e157149_d_n2;
        locals.var_wjunc0_dn4 = assign104830_e157149_d_n4;
        locals.var_wjunc0_dn5 = assign104830_e157149_d_n5;
        locals.var_wjunc0_dn6 = assign104830_e157149_d_n6;
        locals.var_wjunc0_dn7 = assign104830_e157149_d_n7;
        locals.var_wjunc0_dn8 = assign104830_e157149_d_n8;
        locals.var_wjunc0_dn9 = assign104830_e157149_d_n9;
        locals.var_wjunc0_dn10 = assign104830_e157149_d_n10;
        locals.var_wjunc0_dn11 = assign104830_e157149_d_n11;
        locals.var_wjunc0_dn14 = assign104830_e157149_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign104840_e157162, assign104840_e157162_d_n0, assign104840_e157162_d_n2, assign104840_e157162_d_n4, assign104840_e157162_d_n5, assign104840_e157162_d_n6, assign104840_e157162_d_n7, assign104840_e157162_d_n8, assign104840_e157162_d_n9, assign104840_e157162_d_n10, assign104840_e157162_d_n11, assign104840_e157162_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104840_e157156: f64 = (locals.var_rd_xldld - locals.var_wjunc0);
        let assign104840_e157159: f64 = (0.01 * locals.var_rd_xldld);
        let assign104840_e157160: f64 = (assign104840_e157156 - assign104840_e157159);
        (assign104840_e157160, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104840_e157162;
        locals.var_tmf1_dn0 = assign104840_e157162_d_n0;
        locals.var_tmf1_dn2 = assign104840_e157162_d_n2;
        locals.var_tmf1_dn4 = assign104840_e157162_d_n4;
        locals.var_tmf1_dn5 = assign104840_e157162_d_n5;
        locals.var_tmf1_dn6 = assign104840_e157162_d_n6;
        locals.var_tmf1_dn7 = assign104840_e157162_d_n7;
        locals.var_tmf1_dn8 = assign104840_e157162_d_n8;
        locals.var_tmf1_dn9 = assign104840_e157162_d_n9;
        locals.var_tmf1_dn10 = assign104840_e157162_d_n10;
        locals.var_tmf1_dn11 = assign104840_e157162_d_n11;
        locals.var_tmf1_dn14 = assign104840_e157162_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign104850_e157175, assign104850_e157175_d_n0, assign104850_e157175_d_n2, assign104850_e157175_d_n4, assign104850_e157175_d_n5, assign104850_e157175_d_n6, assign104850_e157175_d_n7, assign104850_e157175_d_n8, assign104850_e157175_d_n9, assign104850_e157175_d_n10, assign104850_e157175_d_n11, assign104850_e157175_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104850_e157169: f64 = (4.0 * locals.var_rd_xldld);
        let assign104850_e157172: f64 = (0.01 * locals.var_rd_xldld);
        let assign104850_e157173: f64 = (assign104850_e157169 * assign104850_e157172);
        (assign104850_e157173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104850_e157175;
        locals.var_tmf2_dn0 = assign104850_e157175_d_n0;
        locals.var_tmf2_dn2 = assign104850_e157175_d_n2;
        locals.var_tmf2_dn4 = assign104850_e157175_d_n4;
        locals.var_tmf2_dn5 = assign104850_e157175_d_n5;
        locals.var_tmf2_dn6 = assign104850_e157175_d_n6;
        locals.var_tmf2_dn7 = assign104850_e157175_d_n7;
        locals.var_tmf2_dn8 = assign104850_e157175_d_n8;
        locals.var_tmf2_dn9 = assign104850_e157175_d_n9;
        locals.var_tmf2_dn10 = assign104850_e157175_d_n10;
        locals.var_tmf2_dn11 = assign104850_e157175_d_n11;
        locals.var_tmf2_dn14 = assign104850_e157175_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104860_e157188, assign104860_e157188_d_n0, assign104860_e157188_d_n2, assign104860_e157188_d_n4, assign104860_e157188_d_n5, assign104860_e157188_d_n6, assign104860_e157188_d_n7, assign104860_e157188_d_n8, assign104860_e157188_d_n9, assign104860_e157188_d_n10, assign104860_e157188_d_n11, assign104860_e157188_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let (assign104860_e157186, assign104860_e157186_d_n0, assign104860_e157186_d_n2, assign104860_e157186_d_n4, assign104860_e157186_d_n5, assign104860_e157186_d_n6, assign104860_e157186_d_n7, assign104860_e157186_d_n8, assign104860_e157186_d_n9, assign104860_e157186_d_n10, assign104860_e157186_d_n11, assign104860_e157186_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104860_e157185: f64 = (-locals.var_tmf2);
                (assign104860_e157185, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104860_e157186, assign104860_e157186_d_n0, assign104860_e157186_d_n2, assign104860_e157186_d_n4, assign104860_e157186_d_n5, assign104860_e157186_d_n6, assign104860_e157186_d_n7, assign104860_e157186_d_n8, assign104860_e157186_d_n9, assign104860_e157186_d_n10, assign104860_e157186_d_n11, assign104860_e157186_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104860_e157188;
        locals.var_tmf2_dn0 = assign104860_e157188_d_n0;
        locals.var_tmf2_dn2 = assign104860_e157188_d_n2;
        locals.var_tmf2_dn4 = assign104860_e157188_d_n4;
        locals.var_tmf2_dn5 = assign104860_e157188_d_n5;
        locals.var_tmf2_dn6 = assign104860_e157188_d_n6;
        locals.var_tmf2_dn7 = assign104860_e157188_d_n7;
        locals.var_tmf2_dn8 = assign104860_e157188_d_n8;
        locals.var_tmf2_dn9 = assign104860_e157188_d_n9;
        locals.var_tmf2_dn10 = assign104860_e157188_d_n10;
        locals.var_tmf2_dn11 = assign104860_e157188_d_n11;
        locals.var_tmf2_dn14 = assign104860_e157188_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104870_e157200, assign104870_e157200_d_n0, assign104870_e157200_d_n2, assign104870_e157200_d_n4, assign104870_e157200_d_n5, assign104870_e157200_d_n6, assign104870_e157200_d_n7, assign104870_e157200_d_n8, assign104870_e157200_d_n9, assign104870_e157200_d_n10, assign104870_e157200_d_n11, assign104870_e157200_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104870_e157195: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104870_e157197: f64 = (assign104870_e157195 + locals.var_tmf2);
        let assign104870_e157198: f64 = (assign104870_e157197).sqrt();
        (assign104870_e157198, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104870_e157198)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104870_e157198)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104870_e157200;
        locals.var_tmf2_dn0 = assign104870_e157200_d_n0;
        locals.var_tmf2_dn2 = assign104870_e157200_d_n2;
        locals.var_tmf2_dn4 = assign104870_e157200_d_n4;
        locals.var_tmf2_dn5 = assign104870_e157200_d_n5;
        locals.var_tmf2_dn6 = assign104870_e157200_d_n6;
        locals.var_tmf2_dn7 = assign104870_e157200_d_n7;
        locals.var_tmf2_dn8 = assign104870_e157200_d_n8;
        locals.var_tmf2_dn9 = assign104870_e157200_d_n9;
        locals.var_tmf2_dn10 = assign104870_e157200_d_n10;
        locals.var_tmf2_dn11 = assign104870_e157200_d_n11;
        locals.var_tmf2_dn14 = assign104870_e157200_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104880_e157213, assign104880_e157213_d_n0, assign104880_e157213_d_n2, assign104880_e157213_d_n4, assign104880_e157213_d_n5, assign104880_e157213_d_n6, assign104880_e157213_d_n7, assign104880_e157213_d_n8, assign104880_e157213_d_n9, assign104880_e157213_d_n10, assign104880_e157213_d_n11, assign104880_e157213_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104880_e157209: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104880_e157210: f64 = (1.0 + assign104880_e157209);
        let assign104880_e157211: f64 = (0.5 * assign104880_e157210);
        (assign104880_e157211, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104880_e157213;
        locals.var_t0_dn0 = assign104880_e157213_d_n0;
        locals.var_t0_dn2 = assign104880_e157213_d_n2;
        locals.var_t0_dn4 = assign104880_e157213_d_n4;
        locals.var_t0_dn5 = assign104880_e157213_d_n5;
        locals.var_t0_dn6 = assign104880_e157213_d_n6;
        locals.var_t0_dn7 = assign104880_e157213_d_n7;
        locals.var_t0_dn8 = assign104880_e157213_d_n8;
        locals.var_t0_dn9 = assign104880_e157213_d_n9;
        locals.var_t0_dn10 = assign104880_e157213_d_n10;
        locals.var_t0_dn11 = assign104880_e157213_d_n11;
        locals.var_t0_dn14 = assign104880_e157213_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign104890_e157226, assign104890_e157226_d_n0, assign104890_e157226_d_n2, assign104890_e157226_d_n4, assign104890_e157226_d_n5, assign104890_e157226_d_n6, assign104890_e157226_d_n7, assign104890_e157226_d_n8, assign104890_e157226_d_n9, assign104890_e157226_d_n10, assign104890_e157226_d_n11, assign104890_e157226_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104890_e157222: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104890_e157223: f64 = (0.5 * assign104890_e157222);
        let assign104890_e157224: f64 = (locals.var_rd_xldld - assign104890_e157223);
        (assign104890_e157224, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjunc, locals.var_wjunc_dn0, locals.var_wjunc_dn2, locals.var_wjunc_dn4, locals.var_wjunc_dn5, locals.var_wjunc_dn6, locals.var_wjunc_dn7, locals.var_wjunc_dn8, locals.var_wjunc_dn9, locals.var_wjunc_dn10, locals.var_wjunc_dn11, locals.var_wjunc_dn14,)
    }
};
        locals.var_wjunc = assign104890_e157226;
        locals.var_wjunc_dn0 = assign104890_e157226_d_n0;
        locals.var_wjunc_dn2 = assign104890_e157226_d_n2;
        locals.var_wjunc_dn4 = assign104890_e157226_d_n4;
        locals.var_wjunc_dn5 = assign104890_e157226_d_n5;
        locals.var_wjunc_dn6 = assign104890_e157226_d_n6;
        locals.var_wjunc_dn7 = assign104890_e157226_d_n7;
        locals.var_wjunc_dn8 = assign104890_e157226_d_n8;
        locals.var_wjunc_dn9 = assign104890_e157226_d_n9;
        locals.var_wjunc_dn10 = assign104890_e157226_d_n10;
        locals.var_wjunc_dn11 = assign104890_e157226_d_n11;
        locals.var_wjunc_dn14 = assign104890_e157226_d_n14;
        locals.var_wjunc_rv = 0.0;

        let (assign104900_e157235,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104900_e157233: f64 = (p.p419 + 1e-25);
        (assign104900_e157233,)
    } else {
        (locals.var_wrdrdjunc,)
    }
};
        locals.var_wrdrdjunc = assign104900_e157235;
        locals.var_wrdrdjunc_rv = 0.0;

        let (assign104910_e157254, assign104910_e157254_d_n0, assign104910_e157254_d_n2, assign104910_e157254_d_n4, assign104910_e157254_d_n5, assign104910_e157254_d_n6, assign104910_e157254_d_n7, assign104910_e157254_d_n8, assign104910_e157254_d_n9, assign104910_e157254_d_n10, assign104910_e157254_d_n11, assign104910_e157254_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104910_e157245: f64 = (locals.var_wdepl / locals.var_wrdrdjunc);
        let assign104910_e157248: f64 = (locals.var_wjunc / locals.var_rd_xldld);
        let assign104910_e157249: f64 = (assign104910_e157245 + assign104910_e157248);
        let assign104910_e157250: f64 = (locals.var_cx * assign104910_e157249);
        let assign104910_e157251: f64 = (1.0 - assign104910_e157250);
        let assign104910_e157252: f64 = (locals.var_xmax * assign104910_e157251);
        (assign104910_e157252, (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn0 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn0 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn2 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn2 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn4 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn4 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn5 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn5 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn6 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn6 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn7 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn7 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn8 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn8 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn9 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn9 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn10 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn10 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn11 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn11 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn14 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn14 / locals.var_rd_xldld))))),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn11, locals.var_xov_dn14,)
    }
};
        locals.var_xov = assign104910_e157254;
        locals.var_xov_dn0 = assign104910_e157254_d_n0;
        locals.var_xov_dn2 = assign104910_e157254_d_n2;
        locals.var_xov_dn4 = assign104910_e157254_d_n4;
        locals.var_xov_dn5 = assign104910_e157254_d_n5;
        locals.var_xov_dn6 = assign104910_e157254_d_n6;
        locals.var_xov_dn7 = assign104910_e157254_d_n7;
        locals.var_xov_dn8 = assign104910_e157254_d_n8;
        locals.var_xov_dn9 = assign104910_e157254_d_n9;
        locals.var_xov_dn10 = assign104910_e157254_d_n10;
        locals.var_xov_dn11 = assign104910_e157254_d_n11;
        locals.var_xov_dn14 = assign104910_e157254_d_n14;
        locals.var_xov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_402(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104920_e157282, assign104920_e157282_d_n0, assign104920_e157282_d_n2, assign104920_e157282_d_n4, assign104920_e157282_d_n5, assign104920_e157282_d_n6, assign104920_e157282_d_n7, assign104920_e157282_d_n8, assign104920_e157282_d_n9, assign104920_e157282_d_n10, assign104920_e157282_d_n11, assign104920_e157282_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104920_e157261: f64 = (locals.var_xov * locals.var_xov);
        let assign104920_e157265: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104920_e157267: f64 = (assign104920_e157265 * locals.var_xmax);
        let assign104920_e157269: f64 = (assign104920_e157267 / 100.0);
        let assign104920_e157270: f64 = (4.0 * assign104920_e157269);
        let assign104920_e157273: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104920_e157275: f64 = (assign104920_e157273 * locals.var_xmax);
        let assign104920_e157277: f64 = (assign104920_e157275 / 100.0);
        let assign104920_e157278: f64 = (assign104920_e157270 * assign104920_e157277);
        let assign104920_e157279: f64 = (assign104920_e157261 + assign104920_e157278);
        let assign104920_e157280: f64 = (assign104920_e157279).sqrt();
        (assign104920_e157280, (((locals.var_xov_dn0 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn0)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn2 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn2)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn4 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn4)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn5 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn5)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn6 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn6)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn7 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn7)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn8 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn8)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn9 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn9)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn10 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn10)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn11 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn11)) / (2.0 * assign104920_e157280)), (((locals.var_xov_dn14 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn14)) / (2.0 * assign104920_e157280)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104920_e157282;
        locals.var_tmf2_dn0 = assign104920_e157282_d_n0;
        locals.var_tmf2_dn2 = assign104920_e157282_d_n2;
        locals.var_tmf2_dn4 = assign104920_e157282_d_n4;
        locals.var_tmf2_dn5 = assign104920_e157282_d_n5;
        locals.var_tmf2_dn6 = assign104920_e157282_d_n6;
        locals.var_tmf2_dn7 = assign104920_e157282_d_n7;
        locals.var_tmf2_dn8 = assign104920_e157282_d_n8;
        locals.var_tmf2_dn9 = assign104920_e157282_d_n9;
        locals.var_tmf2_dn10 = assign104920_e157282_d_n10;
        locals.var_tmf2_dn11 = assign104920_e157282_d_n11;
        locals.var_tmf2_dn14 = assign104920_e157282_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign104930_e157295, assign104930_e157295_d_n0, assign104930_e157295_d_n2, assign104930_e157295_d_n4, assign104930_e157295_d_n5, assign104930_e157295_d_n6, assign104930_e157295_d_n7, assign104930_e157295_d_n8, assign104930_e157295_d_n9, assign104930_e157295_d_n10, assign104930_e157295_d_n11, assign104930_e157295_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104930_e157291: f64 = (locals.var_xov / locals.var_tmf2);
        let assign104930_e157292: f64 = (1.0 + assign104930_e157291);
        let assign104930_e157293: f64 = (0.5 * assign104930_e157292);
        (assign104930_e157293, (0.5 * (((locals.var_xov_dn0 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn2 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn4 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn5 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn6 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn7 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn8 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn9 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn10 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn11 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn14 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104930_e157295;
        locals.var_t9_dn0 = assign104930_e157295_d_n0;
        locals.var_t9_dn2 = assign104930_e157295_d_n2;
        locals.var_t9_dn4 = assign104930_e157295_d_n4;
        locals.var_t9_dn5 = assign104930_e157295_d_n5;
        locals.var_t9_dn6 = assign104930_e157295_d_n6;
        locals.var_t9_dn7 = assign104930_e157295_d_n7;
        locals.var_t9_dn8 = assign104930_e157295_d_n8;
        locals.var_t9_dn9 = assign104930_e157295_d_n9;
        locals.var_t9_dn10 = assign104930_e157295_d_n10;
        locals.var_t9_dn11 = assign104930_e157295_d_n11;
        locals.var_t9_dn14 = assign104930_e157295_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign104940_e157306, assign104940_e157306_d_n0, assign104940_e157306_d_n2, assign104940_e157306_d_n4, assign104940_e157306_d_n5, assign104940_e157306_d_n6, assign104940_e157306_d_n7, assign104940_e157306_d_n8, assign104940_e157306_d_n9, assign104940_e157306_d_n10, assign104940_e157306_d_n11, assign104940_e157306_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104940_e157303: f64 = (locals.var_xov + locals.var_tmf2);
        let assign104940_e157304: f64 = (0.5 * assign104940_e157303);
        (assign104940_e157304, (0.5 * (locals.var_xov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_xov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_xov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_xov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_xov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_xov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_xov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_xov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_xov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_xov_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_xov_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn11, locals.var_xov_dn14,)
    }
};
        locals.var_xov = assign104940_e157306;
        locals.var_xov_dn0 = assign104940_e157306_d_n0;
        locals.var_xov_dn2 = assign104940_e157306_d_n2;
        locals.var_xov_dn4 = assign104940_e157306_d_n4;
        locals.var_xov_dn5 = assign104940_e157306_d_n5;
        locals.var_xov_dn6 = assign104940_e157306_d_n6;
        locals.var_xov_dn7 = assign104940_e157306_d_n7;
        locals.var_xov_dn8 = assign104940_e157306_d_n8;
        locals.var_xov_dn9 = assign104940_e157306_d_n9;
        locals.var_xov_dn10 = assign104940_e157306_d_n10;
        locals.var_xov_dn11 = assign104940_e157306_d_n11;
        locals.var_xov_dn14 = assign104940_e157306_d_n14;
        locals.var_xov_rv = 0.0;

        let assign104950_e157309: f64 = if locals.var_xov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2384 = assign104950_e157309;
        locals.var_guard2384_rv = 0.0;

        let (assign104960_e157318, assign104960_e157318_d_n0, assign104960_e157318_d_n2, assign104960_e157318_d_n4, assign104960_e157318_d_n5, assign104960_e157318_d_n6, assign104960_e157318_d_n7, assign104960_e157318_d_n8, assign104960_e157318_d_n9, assign104960_e157318_d_n10, assign104960_e157318_d_n11, assign104960_e157318_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn11, locals.var_xov_dn14,)
    }
};
        locals.var_xov = assign104960_e157318;
        locals.var_xov_dn0 = assign104960_e157318_d_n0;
        locals.var_xov_dn2 = assign104960_e157318_d_n2;
        locals.var_xov_dn4 = assign104960_e157318_d_n4;
        locals.var_xov_dn5 = assign104960_e157318_d_n5;
        locals.var_xov_dn6 = assign104960_e157318_d_n6;
        locals.var_xov_dn7 = assign104960_e157318_d_n7;
        locals.var_xov_dn8 = assign104960_e157318_d_n8;
        locals.var_xov_dn9 = assign104960_e157318_d_n9;
        locals.var_xov_dn10 = assign104960_e157318_d_n10;
        locals.var_xov_dn11 = assign104960_e157318_d_n11;
        locals.var_xov_dn14 = assign104960_e157318_d_n14;
        locals.var_xov_rv = 0.0;

        let (assign104970_e157327, assign104970_e157327_d_n0, assign104970_e157327_d_n2, assign104970_e157327_d_n4, assign104970_e157327_d_n5, assign104970_e157327_d_n6, assign104970_e157327_d_n7, assign104970_e157327_d_n8, assign104970_e157327_d_n9, assign104970_e157327_d_n10, assign104970_e157327_d_n11, assign104970_e157327_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign104970_e157327;
        locals.var_t9_dn0 = assign104970_e157327_d_n0;
        locals.var_t9_dn2 = assign104970_e157327_d_n2;
        locals.var_t9_dn4 = assign104970_e157327_d_n4;
        locals.var_t9_dn5 = assign104970_e157327_d_n5;
        locals.var_t9_dn6 = assign104970_e157327_d_n6;
        locals.var_t9_dn7 = assign104970_e157327_d_n7;
        locals.var_t9_dn8 = assign104970_e157327_d_n8;
        locals.var_t9_dn9 = assign104970_e157327_d_n9;
        locals.var_t9_dn10 = assign104970_e157327_d_n10;
        locals.var_t9_dn11 = assign104970_e157327_d_n11;
        locals.var_t9_dn14 = assign104970_e157327_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign104980_e157338, assign104980_e157338_d_n0, assign104980_e157338_d_n2, assign104980_e157338_d_n4, assign104980_e157338_d_n5, assign104980_e157338_d_n6, assign104980_e157338_d_n7, assign104980_e157338_d_n8, assign104980_e157338_d_n9, assign104980_e157338_d_n10, assign104980_e157338_d_n11, assign104980_e157338_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104980_e157335: f64 = (locals.var_ldrifte + p.p422);
        let assign104980_e157336: f64 = (1.6021918e-19 / assign104980_e157335);
        (assign104980_e157336, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign104980_e157338;
        locals.var_t1_dn0 = assign104980_e157338_d_n0;
        locals.var_t1_dn2 = assign104980_e157338_d_n2;
        locals.var_t1_dn4 = assign104980_e157338_d_n4;
        locals.var_t1_dn5 = assign104980_e157338_d_n5;
        locals.var_t1_dn6 = assign104980_e157338_d_n6;
        locals.var_t1_dn7 = assign104980_e157338_d_n7;
        locals.var_t1_dn8 = assign104980_e157338_d_n8;
        locals.var_t1_dn9 = assign104980_e157338_d_n9;
        locals.var_t1_dn10 = assign104980_e157338_d_n10;
        locals.var_t1_dn11 = assign104980_e157338_d_n11;
        locals.var_t1_dn14 = assign104980_e157338_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign104990_e157351, assign104990_e157351_d_n0, assign104990_e157351_d_n2, assign104990_e157351_d_n4, assign104990_e157351_d_n5, assign104990_e157351_d_n6, assign104990_e157351_d_n7, assign104990_e157351_d_n8, assign104990_e157351_d_n9, assign104990_e157351_d_n10, assign104990_e157351_d_n11, assign104990_e157351_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104990_e157345: f64 = (locals.var_t1 * locals.var_xov);
        let assign104990_e157347: f64 = (assign104990_e157345 * locals.var_mu__blk2358);
        let assign104990_e157349: f64 = (assign104990_e157347 * locals.var_carr);
        (assign104990_e157349, ((((((locals.var_t1_dn0 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn0)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn0)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn0)), ((((((locals.var_t1_dn2 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn2)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn2)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn2)), ((((((locals.var_t1_dn4 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn4)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn4)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn4)), ((((((locals.var_t1_dn5 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn5)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn5)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn5)), ((((((locals.var_t1_dn6 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn6)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn6)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn6)), ((((((locals.var_t1_dn7 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn7)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn7)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn7)), ((((((locals.var_t1_dn8 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn8)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn8)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn8)), ((((((locals.var_t1_dn9 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn9)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn9)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn9)), ((((((locals.var_t1_dn10 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn10)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn10)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn10)), ((((((locals.var_t1_dn11 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn11)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn11)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn11)), ((((((locals.var_t1_dn14 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn14)) * locals.var_mu__blk2358) + (assign104990_e157345 * locals.var_mu__blk2358_dn14)) * locals.var_carr) + (assign104990_e157347 * locals.var_carr_dn14)),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    }
};
        locals.var_gd = assign104990_e157351;
        locals.var_gd_dn0 = assign104990_e157351_d_n0;
        locals.var_gd_dn2 = assign104990_e157351_d_n2;
        locals.var_gd_dn4 = assign104990_e157351_d_n4;
        locals.var_gd_dn5 = assign104990_e157351_d_n5;
        locals.var_gd_dn6 = assign104990_e157351_d_n6;
        locals.var_gd_dn7 = assign104990_e157351_d_n7;
        locals.var_gd_dn8 = assign104990_e157351_d_n8;
        locals.var_gd_dn9 = assign104990_e157351_d_n9;
        locals.var_gd_dn10 = assign104990_e157351_d_n10;
        locals.var_gd_dn11 = assign104990_e157351_d_n11;
        locals.var_gd_dn14 = assign104990_e157351_d_n14;
        locals.var_gd_rv = 0.0;

        let assign105000_e157355: f64 = 1e-25;
        let assign105000_e157360: f64 = if ((locals.var_gd < assign105000_e157355) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2385 = assign105000_e157360;
        locals.var_guard2385_rv = 0.0;

        let (assign105010_e157373, assign105010_e157373_d_n0, assign105010_e157373_d_n2, assign105010_e157373_d_n4, assign105010_e157373_d_n5, assign105010_e157373_d_n6, assign105010_e157373_d_n7, assign105010_e157373_d_n8, assign105010_e157373_d_n9, assign105010_e157373_d_n10, assign105010_e157373_d_n11, assign105010_e157373_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105010_e157369: f64 = 1e-25;
        let assign105010_e157371: f64 = (assign105010_e157369 - locals.var_gd);
        (assign105010_e157371, (-locals.var_gd_dn0), (-locals.var_gd_dn2), (-locals.var_gd_dn4), (-locals.var_gd_dn5), (-locals.var_gd_dn6), (-locals.var_gd_dn7), (-locals.var_gd_dn8), (-locals.var_gd_dn9), (-locals.var_gd_dn10), (-locals.var_gd_dn11), (-locals.var_gd_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign105010_e157373;
        locals.var_tmf1_dn0 = assign105010_e157373_d_n0;
        locals.var_tmf1_dn2 = assign105010_e157373_d_n2;
        locals.var_tmf1_dn4 = assign105010_e157373_d_n4;
        locals.var_tmf1_dn5 = assign105010_e157373_d_n5;
        locals.var_tmf1_dn6 = assign105010_e157373_d_n6;
        locals.var_tmf1_dn7 = assign105010_e157373_d_n7;
        locals.var_tmf1_dn8 = assign105010_e157373_d_n8;
        locals.var_tmf1_dn9 = assign105010_e157373_d_n9;
        locals.var_tmf1_dn10 = assign105010_e157373_d_n10;
        locals.var_tmf1_dn11 = assign105010_e157373_d_n11;
        locals.var_tmf1_dn14 = assign105010_e157373_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign105020_e157384, assign105020_e157384_d_n0, assign105020_e157384_d_n2, assign105020_e157384_d_n4, assign105020_e157384_d_n5, assign105020_e157384_d_n6, assign105020_e157384_d_n7, assign105020_e157384_d_n8, assign105020_e157384_d_n9, assign105020_e157384_d_n10, assign105020_e157384_d_n11, assign105020_e157384_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105020_e157382: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign105020_e157382, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign105020_e157384;
        locals.var_x2_dn0 = assign105020_e157384_d_n0;
        locals.var_x2_dn2 = assign105020_e157384_d_n2;
        locals.var_x2_dn4 = assign105020_e157384_d_n4;
        locals.var_x2_dn5 = assign105020_e157384_d_n5;
        locals.var_x2_dn6 = assign105020_e157384_d_n6;
        locals.var_x2_dn7 = assign105020_e157384_d_n7;
        locals.var_x2_dn8 = assign105020_e157384_d_n8;
        locals.var_x2_dn9 = assign105020_e157384_d_n9;
        locals.var_x2_dn10 = assign105020_e157384_d_n10;
        locals.var_x2_dn11 = assign105020_e157384_d_n11;
        locals.var_x2_dn14 = assign105020_e157384_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign105030_e157395, assign105030_e157395_d_n0, assign105030_e157395_d_n2, assign105030_e157395_d_n4, assign105030_e157395_d_n5, assign105030_e157395_d_n6, assign105030_e157395_d_n7, assign105030_e157395_d_n8, assign105030_e157395_d_n9, assign105030_e157395_d_n10, assign105030_e157395_d_n11, assign105030_e157395_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105030_e157393: f64 = (1e-25 * 1e-25);
        (assign105030_e157393, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign105030_e157395;
        locals.var_xmax2_dn0 = assign105030_e157395_d_n0;
        locals.var_xmax2_dn2 = assign105030_e157395_d_n2;
        locals.var_xmax2_dn4 = assign105030_e157395_d_n4;
        locals.var_xmax2_dn5 = assign105030_e157395_d_n5;
        locals.var_xmax2_dn6 = assign105030_e157395_d_n6;
        locals.var_xmax2_dn7 = assign105030_e157395_d_n7;
        locals.var_xmax2_dn8 = assign105030_e157395_d_n8;
        locals.var_xmax2_dn9 = assign105030_e157395_d_n9;
        locals.var_xmax2_dn10 = assign105030_e157395_d_n10;
        locals.var_xmax2_dn11 = assign105030_e157395_d_n11;
        locals.var_xmax2_dn14 = assign105030_e157395_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign105040_e157404, assign105040_e157404_d_n0, assign105040_e157404_d_n2, assign105040_e157404_d_n4, assign105040_e157404_d_n5, assign105040_e157404_d_n6, assign105040_e157404_d_n7, assign105040_e157404_d_n8, assign105040_e157404_d_n9, assign105040_e157404_d_n10, assign105040_e157404_d_n11, assign105040_e157404_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105040_e157404;
        locals.var_xp_dn0 = assign105040_e157404_d_n0;
        locals.var_xp_dn2 = assign105040_e157404_d_n2;
        locals.var_xp_dn4 = assign105040_e157404_d_n4;
        locals.var_xp_dn5 = assign105040_e157404_d_n5;
        locals.var_xp_dn6 = assign105040_e157404_d_n6;
        locals.var_xp_dn7 = assign105040_e157404_d_n7;
        locals.var_xp_dn8 = assign105040_e157404_d_n8;
        locals.var_xp_dn9 = assign105040_e157404_d_n9;
        locals.var_xp_dn10 = assign105040_e157404_d_n10;
        locals.var_xp_dn11 = assign105040_e157404_d_n11;
        locals.var_xp_dn14 = assign105040_e157404_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign105050_e157413, assign105050_e157413_d_n0, assign105050_e157413_d_n2, assign105050_e157413_d_n4, assign105050_e157413_d_n5, assign105050_e157413_d_n6, assign105050_e157413_d_n7, assign105050_e157413_d_n8, assign105050_e157413_d_n9, assign105050_e157413_d_n10, assign105050_e157413_d_n11, assign105050_e157413_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105050_e157413;
        locals.var_xmp_dn0 = assign105050_e157413_d_n0;
        locals.var_xmp_dn2 = assign105050_e157413_d_n2;
        locals.var_xmp_dn4 = assign105050_e157413_d_n4;
        locals.var_xmp_dn5 = assign105050_e157413_d_n5;
        locals.var_xmp_dn6 = assign105050_e157413_d_n6;
        locals.var_xmp_dn7 = assign105050_e157413_d_n7;
        locals.var_xmp_dn8 = assign105050_e157413_d_n8;
        locals.var_xmp_dn9 = assign105050_e157413_d_n9;
        locals.var_xmp_dn10 = assign105050_e157413_d_n10;
        locals.var_xmp_dn11 = assign105050_e157413_d_n11;
        locals.var_xmp_dn14 = assign105050_e157413_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign105060_e157422,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105060_e157422;
        locals.var_m0_rv = 0.0;

        let (assign105070_e157431,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105070_e157431;
        locals.var_mm_rv = 0.0;

        let (assign105080_e157440, assign105080_e157440_d_n0, assign105080_e157440_d_n2, assign105080_e157440_d_n4, assign105080_e157440_d_n5, assign105080_e157440_d_n6, assign105080_e157440_d_n7, assign105080_e157440_d_n8, assign105080_e157440_d_n9, assign105080_e157440_d_n10, assign105080_e157440_d_n11, assign105080_e157440_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105080_e157440;
        locals.var_arg_dn0 = assign105080_e157440_d_n0;
        locals.var_arg_dn2 = assign105080_e157440_d_n2;
        locals.var_arg_dn4 = assign105080_e157440_d_n4;
        locals.var_arg_dn5 = assign105080_e157440_d_n5;
        locals.var_arg_dn6 = assign105080_e157440_d_n6;
        locals.var_arg_dn7 = assign105080_e157440_d_n7;
        locals.var_arg_dn8 = assign105080_e157440_d_n8;
        locals.var_arg_dn9 = assign105080_e157440_d_n9;
        locals.var_arg_dn10 = assign105080_e157440_d_n10;
        locals.var_arg_dn11 = assign105080_e157440_d_n11;
        locals.var_arg_dn14 = assign105080_e157440_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign105090_e157449, assign105090_e157449_d_n0, assign105090_e157449_d_n2, assign105090_e157449_d_n4, assign105090_e157449_d_n5, assign105090_e157449_d_n6, assign105090_e157449_d_n7, assign105090_e157449_d_n8, assign105090_e157449_d_n9, assign105090_e157449_d_n10, assign105090_e157449_d_n11, assign105090_e157449_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105090_e157449;
        locals.var_dnm_dn0 = assign105090_e157449_d_n0;
        locals.var_dnm_dn2 = assign105090_e157449_d_n2;
        locals.var_dnm_dn4 = assign105090_e157449_d_n4;
        locals.var_dnm_dn5 = assign105090_e157449_d_n5;
        locals.var_dnm_dn6 = assign105090_e157449_d_n6;
        locals.var_dnm_dn7 = assign105090_e157449_d_n7;
        locals.var_dnm_dn8 = assign105090_e157449_d_n8;
        locals.var_dnm_dn9 = assign105090_e157449_d_n9;
        locals.var_dnm_dn10 = assign105090_e157449_d_n10;
        locals.var_dnm_dn11 = assign105090_e157449_d_n11;
        locals.var_dnm_dn14 = assign105090_e157449_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign105100_e157460, assign105100_e157460_d_n0, assign105100_e157460_d_n2, assign105100_e157460_d_n4, assign105100_e157460_d_n5, assign105100_e157460_d_n6, assign105100_e157460_d_n7, assign105100_e157460_d_n8, assign105100_e157460_d_n9, assign105100_e157460_d_n10, assign105100_e157460_d_n11, assign105100_e157460_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105100_e157458: f64 = (locals.var_xp * locals.var_x2);
        (assign105100_e157458, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105100_e157460;
        locals.var_xp_dn0 = assign105100_e157460_d_n0;
        locals.var_xp_dn2 = assign105100_e157460_d_n2;
        locals.var_xp_dn4 = assign105100_e157460_d_n4;
        locals.var_xp_dn5 = assign105100_e157460_d_n5;
        locals.var_xp_dn6 = assign105100_e157460_d_n6;
        locals.var_xp_dn7 = assign105100_e157460_d_n7;
        locals.var_xp_dn8 = assign105100_e157460_d_n8;
        locals.var_xp_dn9 = assign105100_e157460_d_n9;
        locals.var_xp_dn10 = assign105100_e157460_d_n10;
        locals.var_xp_dn11 = assign105100_e157460_d_n11;
        locals.var_xp_dn14 = assign105100_e157460_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign105110_e157471, assign105110_e157471_d_n0, assign105110_e157471_d_n2, assign105110_e157471_d_n4, assign105110_e157471_d_n5, assign105110_e157471_d_n6, assign105110_e157471_d_n7, assign105110_e157471_d_n8, assign105110_e157471_d_n9, assign105110_e157471_d_n10, assign105110_e157471_d_n11, assign105110_e157471_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105110_e157469: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105110_e157469, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105110_e157471;
        locals.var_xmp_dn0 = assign105110_e157471_d_n0;
        locals.var_xmp_dn2 = assign105110_e157471_d_n2;
        locals.var_xmp_dn4 = assign105110_e157471_d_n4;
        locals.var_xmp_dn5 = assign105110_e157471_d_n5;
        locals.var_xmp_dn6 = assign105110_e157471_d_n6;
        locals.var_xmp_dn7 = assign105110_e157471_d_n7;
        locals.var_xmp_dn8 = assign105110_e157471_d_n8;
        locals.var_xmp_dn9 = assign105110_e157471_d_n9;
        locals.var_xmp_dn10 = assign105110_e157471_d_n10;
        locals.var_xmp_dn11 = assign105110_e157471_d_n11;
        locals.var_xmp_dn14 = assign105110_e157471_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign105120_e157482, assign105120_e157482_d_n0, assign105120_e157482_d_n2, assign105120_e157482_d_n4, assign105120_e157482_d_n5, assign105120_e157482_d_n6, assign105120_e157482_d_n7, assign105120_e157482_d_n8, assign105120_e157482_d_n9, assign105120_e157482_d_n10, assign105120_e157482_d_n11, assign105120_e157482_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105120_e157480: f64 = (locals.var_xp * locals.var_x2);
        (assign105120_e157480, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105120_e157482;
        locals.var_xp_dn0 = assign105120_e157482_d_n0;
        locals.var_xp_dn2 = assign105120_e157482_d_n2;
        locals.var_xp_dn4 = assign105120_e157482_d_n4;
        locals.var_xp_dn5 = assign105120_e157482_d_n5;
        locals.var_xp_dn6 = assign105120_e157482_d_n6;
        locals.var_xp_dn7 = assign105120_e157482_d_n7;
        locals.var_xp_dn8 = assign105120_e157482_d_n8;
        locals.var_xp_dn9 = assign105120_e157482_d_n9;
        locals.var_xp_dn10 = assign105120_e157482_d_n10;
        locals.var_xp_dn11 = assign105120_e157482_d_n11;
        locals.var_xp_dn14 = assign105120_e157482_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign105130_e157493, assign105130_e157493_d_n0, assign105130_e157493_d_n2, assign105130_e157493_d_n4, assign105130_e157493_d_n5, assign105130_e157493_d_n6, assign105130_e157493_d_n7, assign105130_e157493_d_n8, assign105130_e157493_d_n9, assign105130_e157493_d_n10, assign105130_e157493_d_n11, assign105130_e157493_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105130_e157491: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105130_e157491, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105130_e157493;
        locals.var_xmp_dn0 = assign105130_e157493_d_n0;
        locals.var_xmp_dn2 = assign105130_e157493_d_n2;
        locals.var_xmp_dn4 = assign105130_e157493_d_n4;
        locals.var_xmp_dn5 = assign105130_e157493_d_n5;
        locals.var_xmp_dn6 = assign105130_e157493_d_n6;
        locals.var_xmp_dn7 = assign105130_e157493_d_n7;
        locals.var_xmp_dn8 = assign105130_e157493_d_n8;
        locals.var_xmp_dn9 = assign105130_e157493_d_n9;
        locals.var_xmp_dn10 = assign105130_e157493_d_n10;
        locals.var_xmp_dn11 = assign105130_e157493_d_n11;
        locals.var_xmp_dn14 = assign105130_e157493_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign105140_e157504, assign105140_e157504_d_n0, assign105140_e157504_d_n2, assign105140_e157504_d_n4, assign105140_e157504_d_n5, assign105140_e157504_d_n6, assign105140_e157504_d_n7, assign105140_e157504_d_n8, assign105140_e157504_d_n9, assign105140_e157504_d_n10, assign105140_e157504_d_n11, assign105140_e157504_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105140_e157502: f64 = (locals.var_xp + locals.var_xmp);
        (assign105140_e157502, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105140_e157504;
        locals.var_arg_dn0 = assign105140_e157504_d_n0;
        locals.var_arg_dn2 = assign105140_e157504_d_n2;
        locals.var_arg_dn4 = assign105140_e157504_d_n4;
        locals.var_arg_dn5 = assign105140_e157504_d_n5;
        locals.var_arg_dn6 = assign105140_e157504_d_n6;
        locals.var_arg_dn7 = assign105140_e157504_d_n7;
        locals.var_arg_dn8 = assign105140_e157504_d_n8;
        locals.var_arg_dn9 = assign105140_e157504_d_n9;
        locals.var_arg_dn10 = assign105140_e157504_d_n10;
        locals.var_arg_dn11 = assign105140_e157504_d_n11;
        locals.var_arg_dn14 = assign105140_e157504_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign105150_e157513, assign105150_e157513_d_n0, assign105150_e157513_d_n2, assign105150_e157513_d_n4, assign105150_e157513_d_n5, assign105150_e157513_d_n6, assign105150_e157513_d_n7, assign105150_e157513_d_n8, assign105150_e157513_d_n9, assign105150_e157513_d_n10, assign105150_e157513_d_n11, assign105150_e157513_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105150_e157513;
        locals.var_dnm_dn0 = assign105150_e157513_d_n0;
        locals.var_dnm_dn2 = assign105150_e157513_d_n2;
        locals.var_dnm_dn4 = assign105150_e157513_d_n4;
        locals.var_dnm_dn5 = assign105150_e157513_d_n5;
        locals.var_dnm_dn6 = assign105150_e157513_d_n6;
        locals.var_dnm_dn7 = assign105150_e157513_d_n7;
        locals.var_dnm_dn8 = assign105150_e157513_d_n8;
        locals.var_dnm_dn9 = assign105150_e157513_d_n9;
        locals.var_dnm_dn10 = assign105150_e157513_d_n10;
        locals.var_dnm_dn11 = assign105150_e157513_d_n11;
        locals.var_dnm_dn14 = assign105150_e157513_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign105160_e157528: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2386 = assign105160_e157528;
        locals.var_guard2386_rv = 0.0;

        let assign105170_e157531: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2387 = assign105170_e157531;
        locals.var_guard2387_rv = 0.0;

        let (assign105180_e157544,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105180_e157544;
        locals.var_mm_rv = 0.0;

        let assign105190_e157547: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2388 = assign105190_e157547;
        locals.var_guard2388_rv = 0.0;

        let (assign105200_e157563,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105200_e157563;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_403(
        locals: &mut StampLocals,
    ) {
        let assign105210_e157566: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2389 = assign105210_e157566;
        locals.var_guard2389_rv = 0.0;

        let (assign105220_e157585,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 == 0.0)) && (locals.var_guard2389 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105220_e157585;
        locals.var_mm_rv = 0.0;

        let assign105230_e157588: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2390 = assign105230_e157588;
        locals.var_guard2390_rv = 0.0;

        let (assign105240_e157610,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_guard2387 == 0.0)) && (locals.var_guard2388 == 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105240_e157610;
        locals.var_mm_rv = 0.0;

        let (assign105250_e157621,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105250_e157621;
        locals.var_m0_rv = 0.0;

        let mut assign105260_loop_guard: usize = 0;
        while {
            let assign105260_cond_e157633: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105260_cond_e157633 != 0.0
        } {
            assign105260_loop_guard += 1;
            assert!(assign105260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105260_body0_e157645, assign105260_body0_e157645_d_n0, assign105260_body0_e157645_d_n2, assign105260_body0_e157645_d_n4, assign105260_body0_e157645_d_n5, assign105260_body0_e157645_d_n6, assign105260_body0_e157645_d_n7, assign105260_body0_e157645_d_n8, assign105260_body0_e157645_d_n9, assign105260_body0_e157645_d_n10, assign105260_body0_e157645_d_n11, assign105260_body0_e157645_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) {
        let assign105260_body0_e157643: f64 = (locals.var_dnm).sqrt();
        (assign105260_body0_e157643, (locals.var_dnm_dn0 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn2 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn4 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn5 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn6 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn7 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn8 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn9 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn10 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn11 / (2.0 * assign105260_body0_e157643)), (locals.var_dnm_dn14 / (2.0 * assign105260_body0_e157643)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign105260_body0_e157645;
            locals.var_dnm_dn0 = assign105260_body0_e157645_d_n0;
            locals.var_dnm_dn2 = assign105260_body0_e157645_d_n2;
            locals.var_dnm_dn4 = assign105260_body0_e157645_d_n4;
            locals.var_dnm_dn5 = assign105260_body0_e157645_d_n5;
            locals.var_dnm_dn6 = assign105260_body0_e157645_d_n6;
            locals.var_dnm_dn7 = assign105260_body0_e157645_d_n7;
            locals.var_dnm_dn8 = assign105260_body0_e157645_d_n8;
            locals.var_dnm_dn9 = assign105260_body0_e157645_d_n9;
            locals.var_dnm_dn10 = assign105260_body0_e157645_d_n10;
            locals.var_dnm_dn11 = assign105260_body0_e157645_d_n11;
            locals.var_dnm_dn14 = assign105260_body0_e157645_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign105260_body1_e157658,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 != 0.0)) {
        let assign105260_body1_e157656: f64 = (locals.var_m0 + 1.0);
        (assign105260_body1_e157656,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105260_body1_e157658;
            locals.var_m0_rv = 0.0;
        }

        let (assign105270_e157681, assign105270_e157681_d_n0, assign105270_e157681_d_n2, assign105270_e157681_d_n4, assign105270_e157681_d_n5, assign105270_e157681_d_n6, assign105270_e157681_d_n7, assign105270_e157681_d_n8, assign105270_e157681_d_n9, assign105270_e157681_d_n10, assign105270_e157681_d_n11, assign105270_e157681_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) && (locals.var_guard2386 == 0.0)) {
        let (assign105270_e157679, assign105270_e157679_d_n0, assign105270_e157679_d_n2, assign105270_e157679_d_n4, assign105270_e157679_d_n5, assign105270_e157679_d_n6, assign105270_e157679_d_n7, assign105270_e157679_d_n8, assign105270_e157679_d_n9, assign105270_e157679_d_n10, assign105270_e157679_d_n11, assign105270_e157679_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105270_e157676: f64 = (2.0 * 2.0);
                let assign105270_e157677: f64 = (1.0 / assign105270_e157676);
                let assign105270_e157678: f64 = (locals.var_dnm).powf(assign105270_e157677);
                (assign105270_e157678, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn11)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((locals.var_dnm).powf(assign105270_e157677 - 1.0) * locals.var_dnm_dn14)) } } else { (assign105270_e157678 * (assign105270_e157677 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign105270_e157679, assign105270_e157679_d_n0, assign105270_e157679_d_n2, assign105270_e157679_d_n4, assign105270_e157679_d_n5, assign105270_e157679_d_n6, assign105270_e157679_d_n7, assign105270_e157679_d_n8, assign105270_e157679_d_n9, assign105270_e157679_d_n10, assign105270_e157679_d_n11, assign105270_e157679_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105270_e157681;
        locals.var_dnm_dn0 = assign105270_e157681_d_n0;
        locals.var_dnm_dn2 = assign105270_e157681_d_n2;
        locals.var_dnm_dn4 = assign105270_e157681_d_n4;
        locals.var_dnm_dn5 = assign105270_e157681_d_n5;
        locals.var_dnm_dn6 = assign105270_e157681_d_n6;
        locals.var_dnm_dn7 = assign105270_e157681_d_n7;
        locals.var_dnm_dn8 = assign105270_e157681_d_n8;
        locals.var_dnm_dn9 = assign105270_e157681_d_n9;
        locals.var_dnm_dn10 = assign105270_e157681_d_n10;
        locals.var_dnm_dn11 = assign105270_e157681_d_n11;
        locals.var_dnm_dn14 = assign105270_e157681_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign105280_e157692, assign105280_e157692_d_n0, assign105280_e157692_d_n2, assign105280_e157692_d_n4, assign105280_e157692_d_n5, assign105280_e157692_d_n6, assign105280_e157692_d_n7, assign105280_e157692_d_n8, assign105280_e157692_d_n9, assign105280_e157692_d_n10, assign105280_e157692_d_n11, assign105280_e157692_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105280_e157690: f64 = (1.0 / locals.var_dnm);
        (assign105280_e157690, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105280_e157692;
        locals.var_dnm_dn0 = assign105280_e157692_d_n0;
        locals.var_dnm_dn2 = assign105280_e157692_d_n2;
        locals.var_dnm_dn4 = assign105280_e157692_d_n4;
        locals.var_dnm_dn5 = assign105280_e157692_d_n5;
        locals.var_dnm_dn6 = assign105280_e157692_d_n6;
        locals.var_dnm_dn7 = assign105280_e157692_d_n7;
        locals.var_dnm_dn8 = assign105280_e157692_d_n8;
        locals.var_dnm_dn9 = assign105280_e157692_d_n9;
        locals.var_dnm_dn10 = assign105280_e157692_d_n10;
        locals.var_dnm_dn11 = assign105280_e157692_d_n11;
        locals.var_dnm_dn14 = assign105280_e157692_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign105290_e157705, assign105290_e157705_d_n0, assign105290_e157705_d_n2, assign105290_e157705_d_n4, assign105290_e157705_d_n5, assign105290_e157705_d_n6, assign105290_e157705_d_n7, assign105290_e157705_d_n8, assign105290_e157705_d_n9, assign105290_e157705_d_n10, assign105290_e157705_d_n11, assign105290_e157705_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105290_e157701: f64 = (locals.var_tmf1 * 1e-25);
        let assign105290_e157703: f64 = (assign105290_e157701 * locals.var_dnm);
        (assign105290_e157703, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-25) * locals.var_dnm) + (assign105290_e157701 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign105290_e157705;
        locals.var_tmf0_dn0 = assign105290_e157705_d_n0;
        locals.var_tmf0_dn2 = assign105290_e157705_d_n2;
        locals.var_tmf0_dn4 = assign105290_e157705_d_n4;
        locals.var_tmf0_dn5 = assign105290_e157705_d_n5;
        locals.var_tmf0_dn6 = assign105290_e157705_d_n6;
        locals.var_tmf0_dn7 = assign105290_e157705_d_n7;
        locals.var_tmf0_dn8 = assign105290_e157705_d_n8;
        locals.var_tmf0_dn9 = assign105290_e157705_d_n9;
        locals.var_tmf0_dn10 = assign105290_e157705_d_n10;
        locals.var_tmf0_dn11 = assign105290_e157705_d_n11;
        locals.var_tmf0_dn14 = assign105290_e157705_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign105300_e157720, assign105300_e157720_d_n0, assign105300_e157720_d_n2, assign105300_e157720_d_n4, assign105300_e157720_d_n5, assign105300_e157720_d_n6, assign105300_e157720_d_n7, assign105300_e157720_d_n8, assign105300_e157720_d_n9, assign105300_e157720_d_n10, assign105300_e157720_d_n11, assign105300_e157720_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105300_e157714: f64 = (1e-25 * locals.var_xmp);
        let assign105300_e157716: f64 = (assign105300_e157714 * locals.var_dnm);
        let assign105300_e157718: f64 = (assign105300_e157716 / locals.var_arg);
        (assign105300_e157718, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn11) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn11)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn14) * locals.var_dnm) + (assign105300_e157714 * locals.var_dnm_dn14)) * locals.var_arg) - (assign105300_e157716 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105300_e157720;
        locals.var_t0_dn0 = assign105300_e157720_d_n0;
        locals.var_t0_dn2 = assign105300_e157720_d_n2;
        locals.var_t0_dn4 = assign105300_e157720_d_n4;
        locals.var_t0_dn5 = assign105300_e157720_d_n5;
        locals.var_t0_dn6 = assign105300_e157720_d_n6;
        locals.var_t0_dn7 = assign105300_e157720_d_n7;
        locals.var_t0_dn8 = assign105300_e157720_d_n8;
        locals.var_t0_dn9 = assign105300_e157720_d_n9;
        locals.var_t0_dn10 = assign105300_e157720_d_n10;
        locals.var_t0_dn11 = assign105300_e157720_d_n11;
        locals.var_t0_dn14 = assign105300_e157720_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign105310_e157733, assign105310_e157733_d_n0, assign105310_e157733_d_n2, assign105310_e157733_d_n4, assign105310_e157733_d_n5, assign105310_e157733_d_n6, assign105310_e157733_d_n7, assign105310_e157733_d_n8, assign105310_e157733_d_n9, assign105310_e157733_d_n10, assign105310_e157733_d_n11, assign105310_e157733_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        let assign105310_e157729: f64 = 1e-25;
        let assign105310_e157731: f64 = (assign105310_e157729 - locals.var_tmf0);
        (assign105310_e157731, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    }
};
        locals.var_gd = assign105310_e157733;
        locals.var_gd_dn0 = assign105310_e157733_d_n0;
        locals.var_gd_dn2 = assign105310_e157733_d_n2;
        locals.var_gd_dn4 = assign105310_e157733_d_n4;
        locals.var_gd_dn5 = assign105310_e157733_d_n5;
        locals.var_gd_dn6 = assign105310_e157733_d_n6;
        locals.var_gd_dn7 = assign105310_e157733_d_n7;
        locals.var_gd_dn8 = assign105310_e157733_d_n8;
        locals.var_gd_dn9 = assign105310_e157733_d_n9;
        locals.var_gd_dn10 = assign105310_e157733_d_n10;
        locals.var_gd_dn11 = assign105310_e157733_d_n11;
        locals.var_gd_dn14 = assign105310_e157733_d_n14;
        locals.var_gd_rv = 0.0;

        let (assign105320_e157742, assign105320_e157742_d_n0, assign105320_e157742_d_n2, assign105320_e157742_d_n4, assign105320_e157742_d_n5, assign105320_e157742_d_n6, assign105320_e157742_d_n7, assign105320_e157742_d_n8, assign105320_e157742_d_n9, assign105320_e157742_d_n10, assign105320_e157742_d_n11, assign105320_e157742_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105320_e157742;
        locals.var_t0_dn0 = assign105320_e157742_d_n0;
        locals.var_t0_dn2 = assign105320_e157742_d_n2;
        locals.var_t0_dn4 = assign105320_e157742_d_n4;
        locals.var_t0_dn5 = assign105320_e157742_d_n5;
        locals.var_t0_dn6 = assign105320_e157742_d_n6;
        locals.var_t0_dn7 = assign105320_e157742_d_n7;
        locals.var_t0_dn8 = assign105320_e157742_d_n8;
        locals.var_t0_dn9 = assign105320_e157742_d_n9;
        locals.var_t0_dn10 = assign105320_e157742_d_n10;
        locals.var_t0_dn11 = assign105320_e157742_d_n11;
        locals.var_t0_dn14 = assign105320_e157742_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign105330_e157752, assign105330_e157752_d_n0, assign105330_e157752_d_n2, assign105330_e157752_d_n4, assign105330_e157752_d_n5, assign105330_e157752_d_n6, assign105330_e157752_d_n7, assign105330_e157752_d_n8, assign105330_e157752_d_n9, assign105330_e157752_d_n10, assign105330_e157752_d_n11, assign105330_e157752_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 == 0.0)) {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn11, locals.var_gd_dn14,)
    }
};
        locals.var_gd = assign105330_e157752;
        locals.var_gd_dn0 = assign105330_e157752_d_n0;
        locals.var_gd_dn2 = assign105330_e157752_d_n2;
        locals.var_gd_dn4 = assign105330_e157752_d_n4;
        locals.var_gd_dn5 = assign105330_e157752_d_n5;
        locals.var_gd_dn6 = assign105330_e157752_d_n6;
        locals.var_gd_dn7 = assign105330_e157752_d_n7;
        locals.var_gd_dn8 = assign105330_e157752_d_n8;
        locals.var_gd_dn9 = assign105330_e157752_d_n9;
        locals.var_gd_dn10 = assign105330_e157752_d_n10;
        locals.var_gd_dn11 = assign105330_e157752_d_n11;
        locals.var_gd_dn14 = assign105330_e157752_d_n14;
        locals.var_gd_rv = 0.0;

        let (assign105340_e157762, assign105340_e157762_d_n0, assign105340_e157762_d_n2, assign105340_e157762_d_n4, assign105340_e157762_d_n5, assign105340_e157762_d_n6, assign105340_e157762_d_n7, assign105340_e157762_d_n8, assign105340_e157762_d_n9, assign105340_e157762_d_n10, assign105340_e157762_d_n11, assign105340_e157762_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2385 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105340_e157762;
        locals.var_t0_dn0 = assign105340_e157762_d_n0;
        locals.var_t0_dn2 = assign105340_e157762_d_n2;
        locals.var_t0_dn4 = assign105340_e157762_d_n4;
        locals.var_t0_dn5 = assign105340_e157762_d_n5;
        locals.var_t0_dn6 = assign105340_e157762_d_n6;
        locals.var_t0_dn7 = assign105340_e157762_d_n7;
        locals.var_t0_dn8 = assign105340_e157762_d_n8;
        locals.var_t0_dn9 = assign105340_e157762_d_n9;
        locals.var_t0_dn10 = assign105340_e157762_d_n10;
        locals.var_t0_dn11 = assign105340_e157762_d_n11;
        locals.var_t0_dn14 = assign105340_e157762_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign105350_e157771, assign105350_e157771_d_n0, assign105350_e157771_d_n2, assign105350_e157771_d_n4, assign105350_e157771_d_n5, assign105350_e157771_d_n6, assign105350_e157771_d_n7, assign105350_e157771_d_n8, assign105350_e157771_d_n9, assign105350_e157771_d_n10, assign105350_e157771_d_n11, assign105350_e157771_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105350_e157769: f64 = (1.0 / locals.var_gd);
        (assign105350_e157769, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn4 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn5 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn8 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn9 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn11 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn14 / (locals.var_gd * locals.var_gd))),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105350_e157771;
        locals.var_rdd_dn0 = assign105350_e157771_d_n0;
        locals.var_rdd_dn2 = assign105350_e157771_d_n2;
        locals.var_rdd_dn4 = assign105350_e157771_d_n4;
        locals.var_rdd_dn5 = assign105350_e157771_d_n5;
        locals.var_rdd_dn6 = assign105350_e157771_d_n6;
        locals.var_rdd_dn7 = assign105350_e157771_d_n7;
        locals.var_rdd_dn8 = assign105350_e157771_d_n8;
        locals.var_rdd_dn9 = assign105350_e157771_d_n9;
        locals.var_rdd_dn10 = assign105350_e157771_d_n10;
        locals.var_rdd_dn11 = assign105350_e157771_d_n11;
        locals.var_rdd_dn14 = assign105350_e157771_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign105360_e157780, assign105360_e157780_d_n0, assign105360_e157780_d_n2, assign105360_e157780_d_n4, assign105360_e157780_d_n5, assign105360_e157780_d_n6, assign105360_e157780_d_n7, assign105360_e157780_d_n8, assign105360_e157780_d_n9, assign105360_e157780_d_n10, assign105360_e157780_d_n11, assign105360_e157780_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105360_e157778: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign105360_e157778, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn11 / locals.var_weffld_nf), (locals.var_rdd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105360_e157780;
        locals.var_rdd_dn0 = assign105360_e157780_d_n0;
        locals.var_rdd_dn2 = assign105360_e157780_d_n2;
        locals.var_rdd_dn4 = assign105360_e157780_d_n4;
        locals.var_rdd_dn5 = assign105360_e157780_d_n5;
        locals.var_rdd_dn6 = assign105360_e157780_d_n6;
        locals.var_rdd_dn7 = assign105360_e157780_d_n7;
        locals.var_rdd_dn8 = assign105360_e157780_d_n8;
        locals.var_rdd_dn9 = assign105360_e157780_d_n9;
        locals.var_rdd_dn10 = assign105360_e157780_d_n10;
        locals.var_rdd_dn11 = assign105360_e157780_d_n11;
        locals.var_rdd_dn14 = assign105360_e157780_d_n14;
        locals.var_rdd_rv = 0.0;

        let assign105370_e157784: f64 = (1000000.0 - 1000.0);
        let assign105370_e157789: f64 = if ((locals.var_rdd > assign105370_e157784) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2391 = assign105370_e157789;
        locals.var_guard2391_rv = 0.0;

        let (assign105380_e157802, assign105380_e157802_d_n0, assign105380_e157802_d_n2, assign105380_e157802_d_n4, assign105380_e157802_d_n5, assign105380_e157802_d_n6, assign105380_e157802_d_n7, assign105380_e157802_d_n8, assign105380_e157802_d_n9, assign105380_e157802_d_n10, assign105380_e157802_d_n11, assign105380_e157802_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105380_e157798: f64 = (locals.var_rdd - 1000000.0);
        let assign105380_e157800: f64 = (assign105380_e157798 + 1000.0);
        (assign105380_e157800, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign105380_e157802;
        locals.var_tmf1_dn0 = assign105380_e157802_d_n0;
        locals.var_tmf1_dn2 = assign105380_e157802_d_n2;
        locals.var_tmf1_dn4 = assign105380_e157802_d_n4;
        locals.var_tmf1_dn5 = assign105380_e157802_d_n5;
        locals.var_tmf1_dn6 = assign105380_e157802_d_n6;
        locals.var_tmf1_dn7 = assign105380_e157802_d_n7;
        locals.var_tmf1_dn8 = assign105380_e157802_d_n8;
        locals.var_tmf1_dn9 = assign105380_e157802_d_n9;
        locals.var_tmf1_dn10 = assign105380_e157802_d_n10;
        locals.var_tmf1_dn11 = assign105380_e157802_d_n11;
        locals.var_tmf1_dn14 = assign105380_e157802_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign105390_e157813, assign105390_e157813_d_n0, assign105390_e157813_d_n2, assign105390_e157813_d_n4, assign105390_e157813_d_n5, assign105390_e157813_d_n6, assign105390_e157813_d_n7, assign105390_e157813_d_n8, assign105390_e157813_d_n9, assign105390_e157813_d_n10, assign105390_e157813_d_n11, assign105390_e157813_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105390_e157811: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign105390_e157811, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign105390_e157813;
        locals.var_x2_dn0 = assign105390_e157813_d_n0;
        locals.var_x2_dn2 = assign105390_e157813_d_n2;
        locals.var_x2_dn4 = assign105390_e157813_d_n4;
        locals.var_x2_dn5 = assign105390_e157813_d_n5;
        locals.var_x2_dn6 = assign105390_e157813_d_n6;
        locals.var_x2_dn7 = assign105390_e157813_d_n7;
        locals.var_x2_dn8 = assign105390_e157813_d_n8;
        locals.var_x2_dn9 = assign105390_e157813_d_n9;
        locals.var_x2_dn10 = assign105390_e157813_d_n10;
        locals.var_x2_dn11 = assign105390_e157813_d_n11;
        locals.var_x2_dn14 = assign105390_e157813_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign105400_e157824, assign105400_e157824_d_n0, assign105400_e157824_d_n2, assign105400_e157824_d_n4, assign105400_e157824_d_n5, assign105400_e157824_d_n6, assign105400_e157824_d_n7, assign105400_e157824_d_n8, assign105400_e157824_d_n9, assign105400_e157824_d_n10, assign105400_e157824_d_n11, assign105400_e157824_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105400_e157822: f64 = (1000.0 * 1000.0);
        (assign105400_e157822, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign105400_e157824;
        locals.var_xmax2_dn0 = assign105400_e157824_d_n0;
        locals.var_xmax2_dn2 = assign105400_e157824_d_n2;
        locals.var_xmax2_dn4 = assign105400_e157824_d_n4;
        locals.var_xmax2_dn5 = assign105400_e157824_d_n5;
        locals.var_xmax2_dn6 = assign105400_e157824_d_n6;
        locals.var_xmax2_dn7 = assign105400_e157824_d_n7;
        locals.var_xmax2_dn8 = assign105400_e157824_d_n8;
        locals.var_xmax2_dn9 = assign105400_e157824_d_n9;
        locals.var_xmax2_dn10 = assign105400_e157824_d_n10;
        locals.var_xmax2_dn11 = assign105400_e157824_d_n11;
        locals.var_xmax2_dn14 = assign105400_e157824_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign105410_e157833, assign105410_e157833_d_n0, assign105410_e157833_d_n2, assign105410_e157833_d_n4, assign105410_e157833_d_n5, assign105410_e157833_d_n6, assign105410_e157833_d_n7, assign105410_e157833_d_n8, assign105410_e157833_d_n9, assign105410_e157833_d_n10, assign105410_e157833_d_n11, assign105410_e157833_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105410_e157833;
        locals.var_xp_dn0 = assign105410_e157833_d_n0;
        locals.var_xp_dn2 = assign105410_e157833_d_n2;
        locals.var_xp_dn4 = assign105410_e157833_d_n4;
        locals.var_xp_dn5 = assign105410_e157833_d_n5;
        locals.var_xp_dn6 = assign105410_e157833_d_n6;
        locals.var_xp_dn7 = assign105410_e157833_d_n7;
        locals.var_xp_dn8 = assign105410_e157833_d_n8;
        locals.var_xp_dn9 = assign105410_e157833_d_n9;
        locals.var_xp_dn10 = assign105410_e157833_d_n10;
        locals.var_xp_dn11 = assign105410_e157833_d_n11;
        locals.var_xp_dn14 = assign105410_e157833_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign105420_e157842, assign105420_e157842_d_n0, assign105420_e157842_d_n2, assign105420_e157842_d_n4, assign105420_e157842_d_n5, assign105420_e157842_d_n6, assign105420_e157842_d_n7, assign105420_e157842_d_n8, assign105420_e157842_d_n9, assign105420_e157842_d_n10, assign105420_e157842_d_n11, assign105420_e157842_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105420_e157842;
        locals.var_xmp_dn0 = assign105420_e157842_d_n0;
        locals.var_xmp_dn2 = assign105420_e157842_d_n2;
        locals.var_xmp_dn4 = assign105420_e157842_d_n4;
        locals.var_xmp_dn5 = assign105420_e157842_d_n5;
        locals.var_xmp_dn6 = assign105420_e157842_d_n6;
        locals.var_xmp_dn7 = assign105420_e157842_d_n7;
        locals.var_xmp_dn8 = assign105420_e157842_d_n8;
        locals.var_xmp_dn9 = assign105420_e157842_d_n9;
        locals.var_xmp_dn10 = assign105420_e157842_d_n10;
        locals.var_xmp_dn11 = assign105420_e157842_d_n11;
        locals.var_xmp_dn14 = assign105420_e157842_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign105430_e157851,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105430_e157851;
        locals.var_m0_rv = 0.0;

        let (assign105440_e157860,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105440_e157860;
        locals.var_mm_rv = 0.0;

        let (assign105450_e157869, assign105450_e157869_d_n0, assign105450_e157869_d_n2, assign105450_e157869_d_n4, assign105450_e157869_d_n5, assign105450_e157869_d_n6, assign105450_e157869_d_n7, assign105450_e157869_d_n8, assign105450_e157869_d_n9, assign105450_e157869_d_n10, assign105450_e157869_d_n11, assign105450_e157869_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105450_e157869;
        locals.var_arg_dn0 = assign105450_e157869_d_n0;
        locals.var_arg_dn2 = assign105450_e157869_d_n2;
        locals.var_arg_dn4 = assign105450_e157869_d_n4;
        locals.var_arg_dn5 = assign105450_e157869_d_n5;
        locals.var_arg_dn6 = assign105450_e157869_d_n6;
        locals.var_arg_dn7 = assign105450_e157869_d_n7;
        locals.var_arg_dn8 = assign105450_e157869_d_n8;
        locals.var_arg_dn9 = assign105450_e157869_d_n9;
        locals.var_arg_dn10 = assign105450_e157869_d_n10;
        locals.var_arg_dn11 = assign105450_e157869_d_n11;
        locals.var_arg_dn14 = assign105450_e157869_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign105460_e157878, assign105460_e157878_d_n0, assign105460_e157878_d_n2, assign105460_e157878_d_n4, assign105460_e157878_d_n5, assign105460_e157878_d_n6, assign105460_e157878_d_n7, assign105460_e157878_d_n8, assign105460_e157878_d_n9, assign105460_e157878_d_n10, assign105460_e157878_d_n11, assign105460_e157878_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105460_e157878;
        locals.var_dnm_dn0 = assign105460_e157878_d_n0;
        locals.var_dnm_dn2 = assign105460_e157878_d_n2;
        locals.var_dnm_dn4 = assign105460_e157878_d_n4;
        locals.var_dnm_dn5 = assign105460_e157878_d_n5;
        locals.var_dnm_dn6 = assign105460_e157878_d_n6;
        locals.var_dnm_dn7 = assign105460_e157878_d_n7;
        locals.var_dnm_dn8 = assign105460_e157878_d_n8;
        locals.var_dnm_dn9 = assign105460_e157878_d_n9;
        locals.var_dnm_dn10 = assign105460_e157878_d_n10;
        locals.var_dnm_dn11 = assign105460_e157878_d_n11;
        locals.var_dnm_dn14 = assign105460_e157878_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign105470_e157889, assign105470_e157889_d_n0, assign105470_e157889_d_n2, assign105470_e157889_d_n4, assign105470_e157889_d_n5, assign105470_e157889_d_n6, assign105470_e157889_d_n7, assign105470_e157889_d_n8, assign105470_e157889_d_n9, assign105470_e157889_d_n10, assign105470_e157889_d_n11, assign105470_e157889_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105470_e157887: f64 = (locals.var_xp * locals.var_x2);
        (assign105470_e157887, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105470_e157889;
        locals.var_xp_dn0 = assign105470_e157889_d_n0;
        locals.var_xp_dn2 = assign105470_e157889_d_n2;
        locals.var_xp_dn4 = assign105470_e157889_d_n4;
        locals.var_xp_dn5 = assign105470_e157889_d_n5;
        locals.var_xp_dn6 = assign105470_e157889_d_n6;
        locals.var_xp_dn7 = assign105470_e157889_d_n7;
        locals.var_xp_dn8 = assign105470_e157889_d_n8;
        locals.var_xp_dn9 = assign105470_e157889_d_n9;
        locals.var_xp_dn10 = assign105470_e157889_d_n10;
        locals.var_xp_dn11 = assign105470_e157889_d_n11;
        locals.var_xp_dn14 = assign105470_e157889_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_404(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign105480_e157900, assign105480_e157900_d_n0, assign105480_e157900_d_n2, assign105480_e157900_d_n4, assign105480_e157900_d_n5, assign105480_e157900_d_n6, assign105480_e157900_d_n7, assign105480_e157900_d_n8, assign105480_e157900_d_n9, assign105480_e157900_d_n10, assign105480_e157900_d_n11, assign105480_e157900_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105480_e157898: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105480_e157898, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105480_e157900;
        locals.var_xmp_dn0 = assign105480_e157900_d_n0;
        locals.var_xmp_dn2 = assign105480_e157900_d_n2;
        locals.var_xmp_dn4 = assign105480_e157900_d_n4;
        locals.var_xmp_dn5 = assign105480_e157900_d_n5;
        locals.var_xmp_dn6 = assign105480_e157900_d_n6;
        locals.var_xmp_dn7 = assign105480_e157900_d_n7;
        locals.var_xmp_dn8 = assign105480_e157900_d_n8;
        locals.var_xmp_dn9 = assign105480_e157900_d_n9;
        locals.var_xmp_dn10 = assign105480_e157900_d_n10;
        locals.var_xmp_dn11 = assign105480_e157900_d_n11;
        locals.var_xmp_dn14 = assign105480_e157900_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign105490_e157911, assign105490_e157911_d_n0, assign105490_e157911_d_n2, assign105490_e157911_d_n4, assign105490_e157911_d_n5, assign105490_e157911_d_n6, assign105490_e157911_d_n7, assign105490_e157911_d_n8, assign105490_e157911_d_n9, assign105490_e157911_d_n10, assign105490_e157911_d_n11, assign105490_e157911_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105490_e157909: f64 = (locals.var_xp * locals.var_x2);
        (assign105490_e157909, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign105490_e157911;
        locals.var_xp_dn0 = assign105490_e157911_d_n0;
        locals.var_xp_dn2 = assign105490_e157911_d_n2;
        locals.var_xp_dn4 = assign105490_e157911_d_n4;
        locals.var_xp_dn5 = assign105490_e157911_d_n5;
        locals.var_xp_dn6 = assign105490_e157911_d_n6;
        locals.var_xp_dn7 = assign105490_e157911_d_n7;
        locals.var_xp_dn8 = assign105490_e157911_d_n8;
        locals.var_xp_dn9 = assign105490_e157911_d_n9;
        locals.var_xp_dn10 = assign105490_e157911_d_n10;
        locals.var_xp_dn11 = assign105490_e157911_d_n11;
        locals.var_xp_dn14 = assign105490_e157911_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign105500_e157922, assign105500_e157922_d_n0, assign105500_e157922_d_n2, assign105500_e157922_d_n4, assign105500_e157922_d_n5, assign105500_e157922_d_n6, assign105500_e157922_d_n7, assign105500_e157922_d_n8, assign105500_e157922_d_n9, assign105500_e157922_d_n10, assign105500_e157922_d_n11, assign105500_e157922_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105500_e157920: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105500_e157920, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign105500_e157922;
        locals.var_xmp_dn0 = assign105500_e157922_d_n0;
        locals.var_xmp_dn2 = assign105500_e157922_d_n2;
        locals.var_xmp_dn4 = assign105500_e157922_d_n4;
        locals.var_xmp_dn5 = assign105500_e157922_d_n5;
        locals.var_xmp_dn6 = assign105500_e157922_d_n6;
        locals.var_xmp_dn7 = assign105500_e157922_d_n7;
        locals.var_xmp_dn8 = assign105500_e157922_d_n8;
        locals.var_xmp_dn9 = assign105500_e157922_d_n9;
        locals.var_xmp_dn10 = assign105500_e157922_d_n10;
        locals.var_xmp_dn11 = assign105500_e157922_d_n11;
        locals.var_xmp_dn14 = assign105500_e157922_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign105510_e157933, assign105510_e157933_d_n0, assign105510_e157933_d_n2, assign105510_e157933_d_n4, assign105510_e157933_d_n5, assign105510_e157933_d_n6, assign105510_e157933_d_n7, assign105510_e157933_d_n8, assign105510_e157933_d_n9, assign105510_e157933_d_n10, assign105510_e157933_d_n11, assign105510_e157933_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105510_e157931: f64 = (locals.var_xp + locals.var_xmp);
        (assign105510_e157931, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign105510_e157933;
        locals.var_arg_dn0 = assign105510_e157933_d_n0;
        locals.var_arg_dn2 = assign105510_e157933_d_n2;
        locals.var_arg_dn4 = assign105510_e157933_d_n4;
        locals.var_arg_dn5 = assign105510_e157933_d_n5;
        locals.var_arg_dn6 = assign105510_e157933_d_n6;
        locals.var_arg_dn7 = assign105510_e157933_d_n7;
        locals.var_arg_dn8 = assign105510_e157933_d_n8;
        locals.var_arg_dn9 = assign105510_e157933_d_n9;
        locals.var_arg_dn10 = assign105510_e157933_d_n10;
        locals.var_arg_dn11 = assign105510_e157933_d_n11;
        locals.var_arg_dn14 = assign105510_e157933_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign105520_e157942, assign105520_e157942_d_n0, assign105520_e157942_d_n2, assign105520_e157942_d_n4, assign105520_e157942_d_n5, assign105520_e157942_d_n6, assign105520_e157942_d_n7, assign105520_e157942_d_n8, assign105520_e157942_d_n9, assign105520_e157942_d_n10, assign105520_e157942_d_n11, assign105520_e157942_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105520_e157942;
        locals.var_dnm_dn0 = assign105520_e157942_d_n0;
        locals.var_dnm_dn2 = assign105520_e157942_d_n2;
        locals.var_dnm_dn4 = assign105520_e157942_d_n4;
        locals.var_dnm_dn5 = assign105520_e157942_d_n5;
        locals.var_dnm_dn6 = assign105520_e157942_d_n6;
        locals.var_dnm_dn7 = assign105520_e157942_d_n7;
        locals.var_dnm_dn8 = assign105520_e157942_d_n8;
        locals.var_dnm_dn9 = assign105520_e157942_d_n9;
        locals.var_dnm_dn10 = assign105520_e157942_d_n10;
        locals.var_dnm_dn11 = assign105520_e157942_d_n11;
        locals.var_dnm_dn14 = assign105520_e157942_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign105530_e157957: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2392 = assign105530_e157957;
        locals.var_guard2392_rv = 0.0;

        let assign105540_e157960: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2393 = assign105540_e157960;
        locals.var_guard2393_rv = 0.0;

        let (assign105550_e157973,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105550_e157973;
        locals.var_mm_rv = 0.0;

        let assign105560_e157976: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2394 = assign105560_e157976;
        locals.var_guard2394_rv = 0.0;

        let (assign105570_e157992,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105570_e157992;
        locals.var_mm_rv = 0.0;

        let assign105580_e157995: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2395 = assign105580_e157995;
        locals.var_guard2395_rv = 0.0;

        let (assign105590_e158014,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 == 0.0)) && (locals.var_guard2395 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105590_e158014;
        locals.var_mm_rv = 0.0;

        let assign105600_e158017: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2396 = assign105600_e158017;
        locals.var_guard2396_rv = 0.0;

        let (assign105610_e158039,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_guard2393 == 0.0)) && (locals.var_guard2394 == 0.0)) && (locals.var_guard2395 == 0.0)) && (locals.var_guard2396 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105610_e158039;
        locals.var_mm_rv = 0.0;

        let (assign105620_e158050,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105620_e158050;
        locals.var_m0_rv = 0.0;

        let mut assign105630_loop_guard: usize = 0;
        while {
            let assign105630_cond_e158062: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105630_cond_e158062 != 0.0
        } {
            assign105630_loop_guard += 1;
            assert!(assign105630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105630_body0_e158074, assign105630_body0_e158074_d_n0, assign105630_body0_e158074_d_n2, assign105630_body0_e158074_d_n4, assign105630_body0_e158074_d_n5, assign105630_body0_e158074_d_n6, assign105630_body0_e158074_d_n7, assign105630_body0_e158074_d_n8, assign105630_body0_e158074_d_n9, assign105630_body0_e158074_d_n10, assign105630_body0_e158074_d_n11, assign105630_body0_e158074_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) {
        let assign105630_body0_e158072: f64 = (locals.var_dnm).sqrt();
        (assign105630_body0_e158072, (locals.var_dnm_dn0 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn2 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn4 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn5 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn6 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn7 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn8 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn9 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn10 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn11 / (2.0 * assign105630_body0_e158072)), (locals.var_dnm_dn14 / (2.0 * assign105630_body0_e158072)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign105630_body0_e158074;
            locals.var_dnm_dn0 = assign105630_body0_e158074_d_n0;
            locals.var_dnm_dn2 = assign105630_body0_e158074_d_n2;
            locals.var_dnm_dn4 = assign105630_body0_e158074_d_n4;
            locals.var_dnm_dn5 = assign105630_body0_e158074_d_n5;
            locals.var_dnm_dn6 = assign105630_body0_e158074_d_n6;
            locals.var_dnm_dn7 = assign105630_body0_e158074_d_n7;
            locals.var_dnm_dn8 = assign105630_body0_e158074_d_n8;
            locals.var_dnm_dn9 = assign105630_body0_e158074_d_n9;
            locals.var_dnm_dn10 = assign105630_body0_e158074_d_n10;
            locals.var_dnm_dn11 = assign105630_body0_e158074_d_n11;
            locals.var_dnm_dn14 = assign105630_body0_e158074_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign105630_body1_e158087,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 != 0.0)) {
        let assign105630_body1_e158085: f64 = (locals.var_m0 + 1.0);
        (assign105630_body1_e158085,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105630_body1_e158087;
            locals.var_m0_rv = 0.0;
        }

        let (assign105640_e158110, assign105640_e158110_d_n0, assign105640_e158110_d_n2, assign105640_e158110_d_n4, assign105640_e158110_d_n5, assign105640_e158110_d_n6, assign105640_e158110_d_n7, assign105640_e158110_d_n8, assign105640_e158110_d_n9, assign105640_e158110_d_n10, assign105640_e158110_d_n11, assign105640_e158110_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) && (locals.var_guard2392 == 0.0)) {
        let (assign105640_e158108, assign105640_e158108_d_n0, assign105640_e158108_d_n2, assign105640_e158108_d_n4, assign105640_e158108_d_n5, assign105640_e158108_d_n6, assign105640_e158108_d_n7, assign105640_e158108_d_n8, assign105640_e158108_d_n9, assign105640_e158108_d_n10, assign105640_e158108_d_n11, assign105640_e158108_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105640_e158105: f64 = (2.0 * 2.0);
                let assign105640_e158106: f64 = (1.0 / assign105640_e158105);
                let assign105640_e158107: f64 = (locals.var_dnm).powf(assign105640_e158106);
                (assign105640_e158107, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn11)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((locals.var_dnm).powf(assign105640_e158106 - 1.0) * locals.var_dnm_dn14)) } } else { (assign105640_e158107 * (assign105640_e158106 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign105640_e158108, assign105640_e158108_d_n0, assign105640_e158108_d_n2, assign105640_e158108_d_n4, assign105640_e158108_d_n5, assign105640_e158108_d_n6, assign105640_e158108_d_n7, assign105640_e158108_d_n8, assign105640_e158108_d_n9, assign105640_e158108_d_n10, assign105640_e158108_d_n11, assign105640_e158108_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105640_e158110;
        locals.var_dnm_dn0 = assign105640_e158110_d_n0;
        locals.var_dnm_dn2 = assign105640_e158110_d_n2;
        locals.var_dnm_dn4 = assign105640_e158110_d_n4;
        locals.var_dnm_dn5 = assign105640_e158110_d_n5;
        locals.var_dnm_dn6 = assign105640_e158110_d_n6;
        locals.var_dnm_dn7 = assign105640_e158110_d_n7;
        locals.var_dnm_dn8 = assign105640_e158110_d_n8;
        locals.var_dnm_dn9 = assign105640_e158110_d_n9;
        locals.var_dnm_dn10 = assign105640_e158110_d_n10;
        locals.var_dnm_dn11 = assign105640_e158110_d_n11;
        locals.var_dnm_dn14 = assign105640_e158110_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign105650_e158121, assign105650_e158121_d_n0, assign105650_e158121_d_n2, assign105650_e158121_d_n4, assign105650_e158121_d_n5, assign105650_e158121_d_n6, assign105650_e158121_d_n7, assign105650_e158121_d_n8, assign105650_e158121_d_n9, assign105650_e158121_d_n10, assign105650_e158121_d_n11, assign105650_e158121_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105650_e158119: f64 = (1.0 / locals.var_dnm);
        (assign105650_e158119, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign105650_e158121;
        locals.var_dnm_dn0 = assign105650_e158121_d_n0;
        locals.var_dnm_dn2 = assign105650_e158121_d_n2;
        locals.var_dnm_dn4 = assign105650_e158121_d_n4;
        locals.var_dnm_dn5 = assign105650_e158121_d_n5;
        locals.var_dnm_dn6 = assign105650_e158121_d_n6;
        locals.var_dnm_dn7 = assign105650_e158121_d_n7;
        locals.var_dnm_dn8 = assign105650_e158121_d_n8;
        locals.var_dnm_dn9 = assign105650_e158121_d_n9;
        locals.var_dnm_dn10 = assign105650_e158121_d_n10;
        locals.var_dnm_dn11 = assign105650_e158121_d_n11;
        locals.var_dnm_dn14 = assign105650_e158121_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign105660_e158134, assign105660_e158134_d_n0, assign105660_e158134_d_n2, assign105660_e158134_d_n4, assign105660_e158134_d_n5, assign105660_e158134_d_n6, assign105660_e158134_d_n7, assign105660_e158134_d_n8, assign105660_e158134_d_n9, assign105660_e158134_d_n10, assign105660_e158134_d_n11, assign105660_e158134_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105660_e158130: f64 = (locals.var_tmf1 * 1000.0);
        let assign105660_e158132: f64 = (assign105660_e158130 * locals.var_dnm);
        (assign105660_e158132, (((locals.var_tmf1_dn0 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1000.0) * locals.var_dnm) + (assign105660_e158130 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign105660_e158134;
        locals.var_tmf0_dn0 = assign105660_e158134_d_n0;
        locals.var_tmf0_dn2 = assign105660_e158134_d_n2;
        locals.var_tmf0_dn4 = assign105660_e158134_d_n4;
        locals.var_tmf0_dn5 = assign105660_e158134_d_n5;
        locals.var_tmf0_dn6 = assign105660_e158134_d_n6;
        locals.var_tmf0_dn7 = assign105660_e158134_d_n7;
        locals.var_tmf0_dn8 = assign105660_e158134_d_n8;
        locals.var_tmf0_dn9 = assign105660_e158134_d_n9;
        locals.var_tmf0_dn10 = assign105660_e158134_d_n10;
        locals.var_tmf0_dn11 = assign105660_e158134_d_n11;
        locals.var_tmf0_dn14 = assign105660_e158134_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign105670_e158149, assign105670_e158149_d_n0, assign105670_e158149_d_n2, assign105670_e158149_d_n4, assign105670_e158149_d_n5, assign105670_e158149_d_n6, assign105670_e158149_d_n7, assign105670_e158149_d_n8, assign105670_e158149_d_n9, assign105670_e158149_d_n10, assign105670_e158149_d_n11, assign105670_e158149_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105670_e158143: f64 = (1000.0 * locals.var_xmp);
        let assign105670_e158145: f64 = (assign105670_e158143 * locals.var_dnm);
        let assign105670_e158147: f64 = (assign105670_e158145 / locals.var_arg);
        (assign105670_e158147, ((((((1000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn11)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign105670_e158143 * locals.var_dnm_dn14)) * locals.var_arg) - (assign105670_e158145 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105670_e158149;
        locals.var_t0_dn0 = assign105670_e158149_d_n0;
        locals.var_t0_dn2 = assign105670_e158149_d_n2;
        locals.var_t0_dn4 = assign105670_e158149_d_n4;
        locals.var_t0_dn5 = assign105670_e158149_d_n5;
        locals.var_t0_dn6 = assign105670_e158149_d_n6;
        locals.var_t0_dn7 = assign105670_e158149_d_n7;
        locals.var_t0_dn8 = assign105670_e158149_d_n8;
        locals.var_t0_dn9 = assign105670_e158149_d_n9;
        locals.var_t0_dn10 = assign105670_e158149_d_n10;
        locals.var_t0_dn11 = assign105670_e158149_d_n11;
        locals.var_t0_dn14 = assign105670_e158149_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign105680_e158162, assign105680_e158162_d_n0, assign105680_e158162_d_n2, assign105680_e158162_d_n4, assign105680_e158162_d_n5, assign105680_e158162_d_n6, assign105680_e158162_d_n7, assign105680_e158162_d_n8, assign105680_e158162_d_n9, assign105680_e158162_d_n10, assign105680_e158162_d_n11, assign105680_e158162_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        let assign105680_e158158: f64 = (1000000.0 - 1000.0);
        let assign105680_e158160: f64 = (assign105680_e158158 + locals.var_tmf0);
        (assign105680_e158160, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105680_e158162;
        locals.var_rdd_dn0 = assign105680_e158162_d_n0;
        locals.var_rdd_dn2 = assign105680_e158162_d_n2;
        locals.var_rdd_dn4 = assign105680_e158162_d_n4;
        locals.var_rdd_dn5 = assign105680_e158162_d_n5;
        locals.var_rdd_dn6 = assign105680_e158162_d_n6;
        locals.var_rdd_dn7 = assign105680_e158162_d_n7;
        locals.var_rdd_dn8 = assign105680_e158162_d_n8;
        locals.var_rdd_dn9 = assign105680_e158162_d_n9;
        locals.var_rdd_dn10 = assign105680_e158162_d_n10;
        locals.var_rdd_dn11 = assign105680_e158162_d_n11;
        locals.var_rdd_dn14 = assign105680_e158162_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign105690_e158171, assign105690_e158171_d_n0, assign105690_e158171_d_n2, assign105690_e158171_d_n4, assign105690_e158171_d_n5, assign105690_e158171_d_n6, assign105690_e158171_d_n7, assign105690_e158171_d_n8, assign105690_e158171_d_n9, assign105690_e158171_d_n10, assign105690_e158171_d_n11, assign105690_e158171_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105690_e158171;
        locals.var_t0_dn0 = assign105690_e158171_d_n0;
        locals.var_t0_dn2 = assign105690_e158171_d_n2;
        locals.var_t0_dn4 = assign105690_e158171_d_n4;
        locals.var_t0_dn5 = assign105690_e158171_d_n5;
        locals.var_t0_dn6 = assign105690_e158171_d_n6;
        locals.var_t0_dn7 = assign105690_e158171_d_n7;
        locals.var_t0_dn8 = assign105690_e158171_d_n8;
        locals.var_t0_dn9 = assign105690_e158171_d_n9;
        locals.var_t0_dn10 = assign105690_e158171_d_n10;
        locals.var_t0_dn11 = assign105690_e158171_d_n11;
        locals.var_t0_dn14 = assign105690_e158171_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign105700_e158181, assign105700_e158181_d_n0, assign105700_e158181_d_n2, assign105700_e158181_d_n4, assign105700_e158181_d_n5, assign105700_e158181_d_n6, assign105700_e158181_d_n7, assign105700_e158181_d_n8, assign105700_e158181_d_n9, assign105700_e158181_d_n10, assign105700_e158181_d_n11, assign105700_e158181_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 == 0.0)) {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105700_e158181;
        locals.var_rdd_dn0 = assign105700_e158181_d_n0;
        locals.var_rdd_dn2 = assign105700_e158181_d_n2;
        locals.var_rdd_dn4 = assign105700_e158181_d_n4;
        locals.var_rdd_dn5 = assign105700_e158181_d_n5;
        locals.var_rdd_dn6 = assign105700_e158181_d_n6;
        locals.var_rdd_dn7 = assign105700_e158181_d_n7;
        locals.var_rdd_dn8 = assign105700_e158181_d_n8;
        locals.var_rdd_dn9 = assign105700_e158181_d_n9;
        locals.var_rdd_dn10 = assign105700_e158181_d_n10;
        locals.var_rdd_dn11 = assign105700_e158181_d_n11;
        locals.var_rdd_dn14 = assign105700_e158181_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign105710_e158191, assign105710_e158191_d_n0, assign105710_e158191_d_n2, assign105710_e158191_d_n4, assign105710_e158191_d_n5, assign105710_e158191_d_n6, assign105710_e158191_d_n7, assign105710_e158191_d_n8, assign105710_e158191_d_n9, assign105710_e158191_d_n10, assign105710_e158191_d_n11, assign105710_e158191_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2391 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign105710_e158191;
        locals.var_t0_dn0 = assign105710_e158191_d_n0;
        locals.var_t0_dn2 = assign105710_e158191_d_n2;
        locals.var_t0_dn4 = assign105710_e158191_d_n4;
        locals.var_t0_dn5 = assign105710_e158191_d_n5;
        locals.var_t0_dn6 = assign105710_e158191_d_n6;
        locals.var_t0_dn7 = assign105710_e158191_d_n7;
        locals.var_t0_dn8 = assign105710_e158191_d_n8;
        locals.var_t0_dn9 = assign105710_e158191_d_n9;
        locals.var_t0_dn10 = assign105710_e158191_d_n10;
        locals.var_t0_dn11 = assign105710_e158191_d_n11;
        locals.var_t0_dn14 = assign105710_e158191_d_n14;
        locals.var_t0_rv = 0.0;

        let assign105720_e158198: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign105720_e158199: f64 = (locals.var_uc_nover * assign105720_e158198);
        let assign105720_e158202: f64 = if ((p.p54 == 1.0) && (assign105720_e158199 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2397 = assign105720_e158202;
        locals.var_guard2397_rv = 0.0;

        let (assign105730_e158213, assign105730_e158213_d_n0, assign105730_e158213_d_n2, assign105730_e158213_d_n4, assign105730_e158213_d_n5, assign105730_e158213_d_n6, assign105730_e158213_d_n7, assign105730_e158213_d_n8, assign105730_e158213_d_n9, assign105730_e158213_d_n10, assign105730_e158213_d_n11, assign105730_e158213_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2397 != 0.0)) {
        let assign105730_e158211: f64 = (p.p334 - locals.var_wdep);
        (assign105730_e158211, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_ddriftld, locals.var_ddriftld_dn0, locals.var_ddriftld_dn2, locals.var_ddriftld_dn4, locals.var_ddriftld_dn5, locals.var_ddriftld_dn6, locals.var_ddriftld_dn7, locals.var_ddriftld_dn8, locals.var_ddriftld_dn9, locals.var_ddriftld_dn10, locals.var_ddriftld_dn11, locals.var_ddriftld_dn14,)
    }
};
        locals.var_ddriftld = assign105730_e158213;
        locals.var_ddriftld_dn0 = assign105730_e158213_d_n0;
        locals.var_ddriftld_dn2 = assign105730_e158213_d_n2;
        locals.var_ddriftld_dn4 = assign105730_e158213_d_n4;
        locals.var_ddriftld_dn5 = assign105730_e158213_d_n5;
        locals.var_ddriftld_dn6 = assign105730_e158213_d_n6;
        locals.var_ddriftld_dn7 = assign105730_e158213_d_n7;
        locals.var_ddriftld_dn8 = assign105730_e158213_d_n8;
        locals.var_ddriftld_dn9 = assign105730_e158213_d_n9;
        locals.var_ddriftld_dn10 = assign105730_e158213_d_n10;
        locals.var_ddriftld_dn11 = assign105730_e158213_d_n11;
        locals.var_ddriftld_dn14 = assign105730_e158213_d_n14;
        locals.var_ddriftld_rv = 0.0;

        let (assign105740_e158226, assign105740_e158226_d_n0, assign105740_e158226_d_n2, assign105740_e158226_d_n4, assign105740_e158226_d_n5, assign105740_e158226_d_n6, assign105740_e158226_d_n7, assign105740_e158226_d_n8, assign105740_e158226_d_n9, assign105740_e158226_d_n10, assign105740_e158226_d_n11, assign105740_e158226_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2397 != 0.0)) {
        let assign105740_e158222: f64 = (locals.var_rdd * locals.var_ldrift0);
        let assign105740_e158224: f64 = (assign105740_e158222 / locals.var_ddriftld);
        (assign105740_e158224, ((((locals.var_rdd_dn0 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn0)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn2 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn2)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn4 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn4)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn5 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn5)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn6 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn6)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn7 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn7)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn8 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn8)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn9 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn9)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn10 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn10)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn11 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn11)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn14 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105740_e158222 * locals.var_ddriftld_dn14)) / (locals.var_ddriftld * locals.var_ddriftld)),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105740_e158226;
        locals.var_rdd_dn0 = assign105740_e158226_d_n0;
        locals.var_rdd_dn2 = assign105740_e158226_d_n2;
        locals.var_rdd_dn4 = assign105740_e158226_d_n4;
        locals.var_rdd_dn5 = assign105740_e158226_d_n5;
        locals.var_rdd_dn6 = assign105740_e158226_d_n6;
        locals.var_rdd_dn7 = assign105740_e158226_d_n7;
        locals.var_rdd_dn8 = assign105740_e158226_d_n8;
        locals.var_rdd_dn9 = assign105740_e158226_d_n9;
        locals.var_rdd_dn10 = assign105740_e158226_d_n10;
        locals.var_rdd_dn11 = assign105740_e158226_d_n11;
        locals.var_rdd_dn14 = assign105740_e158226_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign105750_e158235, assign105750_e158235_d_n0, assign105750_e158235_d_n2, assign105750_e158235_d_n4, assign105750_e158235_d_n5, assign105750_e158235_d_n6, assign105750_e158235_d_n7, assign105750_e158235_d_n8, assign105750_e158235_d_n9, assign105750_e158235_d_n10, assign105750_e158235_d_n11, assign105750_e158235_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105750_e158233: f64 = (locals.var_rdd + locals.var_rd0);
        (assign105750_e158233, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105750_e158235;
        locals.var_rdd_dn0 = assign105750_e158235_d_n0;
        locals.var_rdd_dn2 = assign105750_e158235_d_n2;
        locals.var_rdd_dn4 = assign105750_e158235_d_n4;
        locals.var_rdd_dn5 = assign105750_e158235_d_n5;
        locals.var_rdd_dn6 = assign105750_e158235_d_n6;
        locals.var_rdd_dn7 = assign105750_e158235_d_n7;
        locals.var_rdd_dn8 = assign105750_e158235_d_n8;
        locals.var_rdd_dn9 = assign105750_e158235_d_n9;
        locals.var_rdd_dn10 = assign105750_e158235_d_n10;
        locals.var_rdd_dn11 = assign105750_e158235_d_n11;
        locals.var_rdd_dn14 = assign105750_e158235_d_n14;
        locals.var_rdd_rv = 0.0;

        let assign105790_e158266: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2399 = assign105790_e158266;
        locals.var_guard2399_rv = 0.0;

        let (assign105800_e158275, assign105800_e158275_d_n0, assign105800_e158275_d_n2, assign105800_e158275_d_n4, assign105800_e158275_d_n5, assign105800_e158275_d_n6, assign105800_e158275_d_n7, assign105800_e158275_d_n8, assign105800_e158275_d_n9, assign105800_e158275_d_n10, assign105800_e158275_d_n11, assign105800_e158275_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2399 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105800_e158275;
        locals.var_rdd_dn0 = assign105800_e158275_d_n0;
        locals.var_rdd_dn2 = assign105800_e158275_d_n2;
        locals.var_rdd_dn4 = assign105800_e158275_d_n4;
        locals.var_rdd_dn5 = assign105800_e158275_d_n5;
        locals.var_rdd_dn6 = assign105800_e158275_d_n6;
        locals.var_rdd_dn7 = assign105800_e158275_d_n7;
        locals.var_rdd_dn8 = assign105800_e158275_d_n8;
        locals.var_rdd_dn9 = assign105800_e158275_d_n9;
        locals.var_rdd_dn10 = assign105800_e158275_d_n10;
        locals.var_rdd_dn11 = assign105800_e158275_d_n11;
        locals.var_rdd_dn14 = assign105800_e158275_d_n14;
        locals.var_rdd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_405(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign105810_e158284, assign105810_e158284_d_n0, assign105810_e158284_d_n2, assign105810_e158284_d_n4, assign105810_e158284_d_n5, assign105810_e158284_d_n6, assign105810_e158284_d_n7, assign105810_e158284_d_n8, assign105810_e158284_d_n9, assign105810_e158284_d_n10, assign105810_e158284_d_n11, assign105810_e158284_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign105810_e158282: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105810_e158282, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14,)
    }
};
        locals.var_rdde = assign105810_e158284;
        locals.var_rdde_dn0 = assign105810_e158284_d_n0;
        locals.var_rdde_dn2 = assign105810_e158284_d_n2;
        locals.var_rdde_dn4 = assign105810_e158284_d_n4;
        locals.var_rdde_dn5 = assign105810_e158284_d_n5;
        locals.var_rdde_dn6 = assign105810_e158284_d_n6;
        locals.var_rdde_dn7 = assign105810_e158284_d_n7;
        locals.var_rdde_dn8 = assign105810_e158284_d_n8;
        locals.var_rdde_dn9 = assign105810_e158284_d_n9;
        locals.var_rdde_dn10 = assign105810_e158284_d_n10;
        locals.var_rdde_dn11 = assign105810_e158284_d_n11;
        locals.var_rdde_dn14 = assign105810_e158284_d_n14;
        locals.var_rdde_rv = 0.0;

        let assign105820_e158287: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2400 = assign105820_e158287;
        locals.var_guard2400_rv = 0.0;

        let (assign105830_e158294, assign105830_e158294_d_n0, assign105830_e158294_d_n2, assign105830_e158294_d_n4, assign105830_e158294_d_n5, assign105830_e158294_d_n6, assign105830_e158294_d_n7, assign105830_e158294_d_n8, assign105830_e158294_d_n9, assign105830_e158294_d_n10, assign105830_e158294_d_n11, assign105830_e158294_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2400 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign105830_e158294;
        locals.var_rdd_dn0 = assign105830_e158294_d_n0;
        locals.var_rdd_dn2 = assign105830_e158294_d_n2;
        locals.var_rdd_dn4 = assign105830_e158294_d_n4;
        locals.var_rdd_dn5 = assign105830_e158294_d_n5;
        locals.var_rdd_dn6 = assign105830_e158294_d_n6;
        locals.var_rdd_dn7 = assign105830_e158294_d_n7;
        locals.var_rdd_dn8 = assign105830_e158294_d_n8;
        locals.var_rdd_dn9 = assign105830_e158294_d_n9;
        locals.var_rdd_dn10 = assign105830_e158294_d_n10;
        locals.var_rdd_dn11 = assign105830_e158294_d_n11;
        locals.var_rdd_dn14 = assign105830_e158294_d_n14;
        locals.var_rdd_rv = 0.0;

        let assign105840_e158297: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2401 = assign105840_e158297;
        locals.var_guard2401_rv = 0.0;

        let (assign105850_e158304, assign105850_e158304_d_n0, assign105850_e158304_d_n2, assign105850_e158304_d_n4, assign105850_e158304_d_n5, assign105850_e158304_d_n6, assign105850_e158304_d_n7, assign105850_e158304_d_n8, assign105850_e158304_d_n9, assign105850_e158304_d_n10, assign105850_e158304_d_n11, assign105850_e158304_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2401 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign105850_e158304;
        locals.var_rsd_dn0 = assign105850_e158304_d_n0;
        locals.var_rsd_dn2 = assign105850_e158304_d_n2;
        locals.var_rsd_dn4 = assign105850_e158304_d_n4;
        locals.var_rsd_dn5 = assign105850_e158304_d_n5;
        locals.var_rsd_dn6 = assign105850_e158304_d_n6;
        locals.var_rsd_dn7 = assign105850_e158304_d_n7;
        locals.var_rsd_dn8 = assign105850_e158304_d_n8;
        locals.var_rsd_dn9 = assign105850_e158304_d_n9;
        locals.var_rsd_dn10 = assign105850_e158304_d_n10;
        locals.var_rsd_dn11 = assign105850_e158304_d_n11;
        locals.var_rsd_dn14 = assign105850_e158304_d_n14;
        locals.var_rsd_rv = 0.0;

        let assign105860_e158307: f64 = if locals.var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2402 = assign105860_e158307;
        locals.var_guard2402_rv = 0.0;

        let (assign105870_e158316, assign105870_e158316_d_n0, assign105870_e158316_d_n2, assign105870_e158316_d_n4, assign105870_e158316_d_n5, assign105870_e158316_d_n6, assign105870_e158316_d_n7, assign105870_e158316_d_n8, assign105870_e158316_d_n9, assign105870_e158316_d_n10, assign105870_e158316_d_n11, assign105870_e158316_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 != 0.0)) {
        let assign105870_e158314: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105870_e158314, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14,)
    }
};
        locals.var_rdde = assign105870_e158316;
        locals.var_rdde_dn0 = assign105870_e158316_d_n0;
        locals.var_rdde_dn2 = assign105870_e158316_d_n2;
        locals.var_rdde_dn4 = assign105870_e158316_d_n4;
        locals.var_rdde_dn5 = assign105870_e158316_d_n5;
        locals.var_rdde_dn6 = assign105870_e158316_d_n6;
        locals.var_rdde_dn7 = assign105870_e158316_d_n7;
        locals.var_rdde_dn8 = assign105870_e158316_d_n8;
        locals.var_rdde_dn9 = assign105870_e158316_d_n9;
        locals.var_rdde_dn10 = assign105870_e158316_d_n10;
        locals.var_rdde_dn11 = assign105870_e158316_d_n11;
        locals.var_rdde_dn14 = assign105870_e158316_d_n14;
        locals.var_rdde_rv = 0.0;

        let (assign105880_e158325, assign105880_e158325_d_n0, assign105880_e158325_d_n2, assign105880_e158325_d_n4, assign105880_e158325_d_n5, assign105880_e158325_d_n6, assign105880_e158325_d_n7, assign105880_e158325_d_n8, assign105880_e158325_d_n9, assign105880_e158325_d_n10, assign105880_e158325_d_n11, assign105880_e158325_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 != 0.0)) {
        let assign105880_e158323: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign105880_e158323, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign105880_e158325;
        locals.var_rsde_dn0 = assign105880_e158325_d_n0;
        locals.var_rsde_dn2 = assign105880_e158325_d_n2;
        locals.var_rsde_dn4 = assign105880_e158325_d_n4;
        locals.var_rsde_dn5 = assign105880_e158325_d_n5;
        locals.var_rsde_dn6 = assign105880_e158325_d_n6;
        locals.var_rsde_dn7 = assign105880_e158325_d_n7;
        locals.var_rsde_dn8 = assign105880_e158325_d_n8;
        locals.var_rsde_dn9 = assign105880_e158325_d_n9;
        locals.var_rsde_dn10 = assign105880_e158325_d_n10;
        locals.var_rsde_dn11 = assign105880_e158325_d_n11;
        locals.var_rsde_dn14 = assign105880_e158325_d_n14;
        locals.var_rsde_rv = 0.0;

        let (assign105890_e158335, assign105890_e158335_d_n0, assign105890_e158335_d_n2, assign105890_e158335_d_n4, assign105890_e158335_d_n5, assign105890_e158335_d_n6, assign105890_e158335_d_n7, assign105890_e158335_d_n8, assign105890_e158335_d_n9, assign105890_e158335_d_n10, assign105890_e158335_d_n11, assign105890_e158335_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 == 0.0)) {
        let assign105890_e158333: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign105890_e158333, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn11, locals.var_rdde_dn14,)
    }
};
        locals.var_rdde = assign105890_e158335;
        locals.var_rdde_dn0 = assign105890_e158335_d_n0;
        locals.var_rdde_dn2 = assign105890_e158335_d_n2;
        locals.var_rdde_dn4 = assign105890_e158335_d_n4;
        locals.var_rdde_dn5 = assign105890_e158335_d_n5;
        locals.var_rdde_dn6 = assign105890_e158335_d_n6;
        locals.var_rdde_dn7 = assign105890_e158335_d_n7;
        locals.var_rdde_dn8 = assign105890_e158335_d_n8;
        locals.var_rdde_dn9 = assign105890_e158335_d_n9;
        locals.var_rdde_dn10 = assign105890_e158335_d_n10;
        locals.var_rdde_dn11 = assign105890_e158335_d_n11;
        locals.var_rdde_dn14 = assign105890_e158335_d_n14;
        locals.var_rdde_rv = 0.0;

        let (assign105900_e158345, assign105900_e158345_d_n0, assign105900_e158345_d_n2, assign105900_e158345_d_n4, assign105900_e158345_d_n5, assign105900_e158345_d_n6, assign105900_e158345_d_n7, assign105900_e158345_d_n8, assign105900_e158345_d_n9, assign105900_e158345_d_n10, assign105900_e158345_d_n11, assign105900_e158345_d_n14,) = {
    if ((locals.var_guard2340 == 0.0) && (locals.var_guard2402 == 0.0)) {
        let assign105900_e158343: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105900_e158343, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn11 / locals.var_mfactor), (locals.var_rdd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign105900_e158345;
        locals.var_rsde_dn0 = assign105900_e158345_d_n0;
        locals.var_rsde_dn2 = assign105900_e158345_d_n2;
        locals.var_rsde_dn4 = assign105900_e158345_d_n4;
        locals.var_rsde_dn5 = assign105900_e158345_d_n5;
        locals.var_rsde_dn6 = assign105900_e158345_d_n6;
        locals.var_rsde_dn7 = assign105900_e158345_d_n7;
        locals.var_rsde_dn8 = assign105900_e158345_d_n8;
        locals.var_rsde_dn9 = assign105900_e158345_d_n9;
        locals.var_rsde_dn10 = assign105900_e158345_d_n10;
        locals.var_rsde_dn11 = assign105900_e158345_d_n11;
        locals.var_rsde_dn14 = assign105900_e158345_d_n14;
        locals.var_rsde_rv = 0.0;

        locals.var_rdd = locals.var_rdde;
        locals.var_rdd_dn0 = locals.var_rdde_dn0;
        locals.var_rdd_dn2 = locals.var_rdde_dn2;
        locals.var_rdd_dn4 = locals.var_rdde_dn4;
        locals.var_rdd_dn5 = locals.var_rdde_dn5;
        locals.var_rdd_dn6 = locals.var_rdde_dn6;
        locals.var_rdd_dn7 = locals.var_rdde_dn7;
        locals.var_rdd_dn8 = locals.var_rdde_dn8;
        locals.var_rdd_dn9 = locals.var_rdde_dn9;
        locals.var_rdd_dn10 = locals.var_rdde_dn10;
        locals.var_rdd_dn11 = locals.var_rdde_dn11;
        locals.var_rdd_dn14 = locals.var_rdde_dn14;
        locals.var_rdd_rv = 0.0;

        locals.var_rsd = locals.var_rsde;
        locals.var_rsd_dn0 = locals.var_rsde_dn0;
        locals.var_rsd_dn2 = locals.var_rsde_dn2;
        locals.var_rsd_dn4 = locals.var_rsde_dn4;
        locals.var_rsd_dn5 = locals.var_rsde_dn5;
        locals.var_rsd_dn6 = locals.var_rsde_dn6;
        locals.var_rsd_dn7 = locals.var_rsde_dn7;
        locals.var_rsd_dn8 = locals.var_rsde_dn8;
        locals.var_rsd_dn9 = locals.var_rsde_dn9;
        locals.var_rsd_dn10 = locals.var_rsde_dn10;
        locals.var_rsd_dn11 = locals.var_rsde_dn11;
        locals.var_rsd_dn14 = locals.var_rsde_dn14;
        locals.var_rsd_rv = 0.0;

        let assign105960_e158353: f64 = if locals.var_mode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2403 = assign105960_e158353;
        locals.var_guard2403_rv = 0.0;

        let (assign105970_e158357, assign105970_e158357_d_n0, assign105970_e158357_d_n2, assign105970_e158357_d_n4, assign105970_e158357_d_n5, assign105970_e158357_d_n6, assign105970_e158357_d_n7, assign105970_e158357_d_n8, assign105970_e158357_d_n9, assign105970_e158357_d_n10, assign105970_e158357_d_n11, assign105970_e158357_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign105970_e158357;
        locals.var_ids_dn0 = assign105970_e158357_d_n0;
        locals.var_ids_dn2 = assign105970_e158357_d_n2;
        locals.var_ids_dn4 = assign105970_e158357_d_n4;
        locals.var_ids_dn5 = assign105970_e158357_d_n5;
        locals.var_ids_dn6 = assign105970_e158357_d_n6;
        locals.var_ids_dn7 = assign105970_e158357_d_n7;
        locals.var_ids_dn8 = assign105970_e158357_d_n8;
        locals.var_ids_dn9 = assign105970_e158357_d_n9;
        locals.var_ids_dn10 = assign105970_e158357_d_n10;
        locals.var_ids_dn11 = assign105970_e158357_d_n11;
        locals.var_ids_dn14 = assign105970_e158357_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign105980_e158361, assign105980_e158361_d_n0, assign105980_e158361_d_n2, assign105980_e158361_d_n4, assign105980_e158361_d_n5, assign105980_e158361_d_n6, assign105980_e158361_d_n7, assign105980_e158361_d_n8, assign105980_e158361_d_n9, assign105980_e158361_d_n10, assign105980_e158361_d_n11, assign105980_e158361_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14,)
    }
};
        locals.var_qd = assign105980_e158361;
        locals.var_qd_dn0 = assign105980_e158361_d_n0;
        locals.var_qd_dn2 = assign105980_e158361_d_n2;
        locals.var_qd_dn4 = assign105980_e158361_d_n4;
        locals.var_qd_dn5 = assign105980_e158361_d_n5;
        locals.var_qd_dn6 = assign105980_e158361_d_n6;
        locals.var_qd_dn7 = assign105980_e158361_d_n7;
        locals.var_qd_dn8 = assign105980_e158361_d_n8;
        locals.var_qd_dn9 = assign105980_e158361_d_n9;
        locals.var_qd_dn10 = assign105980_e158361_d_n10;
        locals.var_qd_dn11 = assign105980_e158361_d_n11;
        locals.var_qd_dn14 = assign105980_e158361_d_n14;
        locals.var_qd_rv = 0.0;

        let (assign105990_e158365, assign105990_e158365_d_n0, assign105990_e158365_d_n2, assign105990_e158365_d_n4, assign105990_e158365_d_n5, assign105990_e158365_d_n6, assign105990_e158365_d_n7, assign105990_e158365_d_n8, assign105990_e158365_d_n9, assign105990_e158365_d_n10, assign105990_e158365_d_n11, assign105990_e158365_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14,)
    }
};
        locals.var_qg = assign105990_e158365;
        locals.var_qg_dn0 = assign105990_e158365_d_n0;
        locals.var_qg_dn2 = assign105990_e158365_d_n2;
        locals.var_qg_dn4 = assign105990_e158365_d_n4;
        locals.var_qg_dn5 = assign105990_e158365_d_n5;
        locals.var_qg_dn6 = assign105990_e158365_d_n6;
        locals.var_qg_dn7 = assign105990_e158365_d_n7;
        locals.var_qg_dn8 = assign105990_e158365_d_n8;
        locals.var_qg_dn9 = assign105990_e158365_d_n9;
        locals.var_qg_dn10 = assign105990_e158365_d_n10;
        locals.var_qg_dn11 = assign105990_e158365_d_n11;
        locals.var_qg_dn14 = assign105990_e158365_d_n14;
        locals.var_qg_rv = 0.0;

        let (assign106000_e158369, assign106000_e158369_d_n0, assign106000_e158369_d_n2, assign106000_e158369_d_n4, assign106000_e158369_d_n5, assign106000_e158369_d_n6, assign106000_e158369_d_n7, assign106000_e158369_d_n8, assign106000_e158369_d_n9, assign106000_e158369_d_n10, assign106000_e158369_d_n11, assign106000_e158369_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    } else {
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14,)
    }
};
        locals.var_qs = assign106000_e158369;
        locals.var_qs_dn0 = assign106000_e158369_d_n0;
        locals.var_qs_dn2 = assign106000_e158369_d_n2;
        locals.var_qs_dn4 = assign106000_e158369_d_n4;
        locals.var_qs_dn5 = assign106000_e158369_d_n5;
        locals.var_qs_dn6 = assign106000_e158369_d_n6;
        locals.var_qs_dn7 = assign106000_e158369_d_n7;
        locals.var_qs_dn8 = assign106000_e158369_d_n8;
        locals.var_qs_dn9 = assign106000_e158369_d_n9;
        locals.var_qs_dn10 = assign106000_e158369_d_n10;
        locals.var_qs_dn11 = assign106000_e158369_d_n11;
        locals.var_qs_dn14 = assign106000_e158369_d_n14;
        locals.var_qs_rv = 0.0;

        let (assign106010_e158378, assign106010_e158378_d_n0, assign106010_e158378_d_n2, assign106010_e158378_d_n4, assign106010_e158378_d_n5, assign106010_e158378_d_n6, assign106010_e158378_d_n7, assign106010_e158378_d_n8, assign106010_e158378_d_n9, assign106010_e158378_d_n10, assign106010_e158378_d_n11, assign106010_e158378_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        let assign106010_e158373: f64 = (locals.var_qge + locals.var_qde);
        let assign106010_e158375: f64 = (assign106010_e158373 + locals.var_qse);
        let assign106010_e158376: f64 = (-assign106010_e158375);
        (assign106010_e158376, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)),)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14,)
    }
};
        locals.var_qb = assign106010_e158378;
        locals.var_qb_dn0 = assign106010_e158378_d_n0;
        locals.var_qb_dn2 = assign106010_e158378_d_n2;
        locals.var_qb_dn4 = assign106010_e158378_d_n4;
        locals.var_qb_dn5 = assign106010_e158378_d_n5;
        locals.var_qb_dn6 = assign106010_e158378_d_n6;
        locals.var_qb_dn7 = assign106010_e158378_d_n7;
        locals.var_qb_dn8 = assign106010_e158378_d_n8;
        locals.var_qb_dn9 = assign106010_e158378_d_n9;
        locals.var_qb_dn10 = assign106010_e158378_d_n10;
        locals.var_qb_dn11 = assign106010_e158378_d_n11;
        locals.var_qb_dn14 = assign106010_e158378_d_n14;
        locals.var_qb_rv = 0.0;

        let (assign106020_e158382, assign106020_e158382_d_n0, assign106020_e158382_d_n2, assign106020_e158382_d_n4, assign106020_e158382_d_n5, assign106020_e158382_d_n6, assign106020_e158382_d_n7, assign106020_e158382_d_n8, assign106020_e158382_d_n9, assign106020_e158382_d_n10, assign106020_e158382_d_n11, assign106020_e158382_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn8, locals.var_isube_dn9, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn14,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14,)
    }
};
        locals.var_isub = assign106020_e158382;
        locals.var_isub_dn0 = assign106020_e158382_d_n0;
        locals.var_isub_dn2 = assign106020_e158382_d_n2;
        locals.var_isub_dn4 = assign106020_e158382_d_n4;
        locals.var_isub_dn5 = assign106020_e158382_d_n5;
        locals.var_isub_dn6 = assign106020_e158382_d_n6;
        locals.var_isub_dn7 = assign106020_e158382_d_n7;
        locals.var_isub_dn8 = assign106020_e158382_d_n8;
        locals.var_isub_dn9 = assign106020_e158382_d_n9;
        locals.var_isub_dn10 = assign106020_e158382_d_n10;
        locals.var_isub_dn11 = assign106020_e158382_d_n11;
        locals.var_isub_dn14 = assign106020_e158382_d_n14;
        locals.var_isub_rv = 0.0;

        let (assign106040_e158390, assign106040_e158390_d_n0, assign106040_e158390_d_n2, assign106040_e158390_d_n4, assign106040_e158390_d_n5, assign106040_e158390_d_n6, assign106040_e158390_d_n7, assign106040_e158390_d_n8, assign106040_e158390_d_n9, assign106040_e158390_d_n10, assign106040_e158390_d_n11, assign106040_e158390_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_isublde, locals.var_isublde_dn0, locals.var_isublde_dn2, locals.var_isublde_dn4, locals.var_isublde_dn5, locals.var_isublde_dn6, locals.var_isublde_dn7, locals.var_isublde_dn8, locals.var_isublde_dn9, locals.var_isublde_dn10, locals.var_isublde_dn11, locals.var_isublde_dn14,)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14,)
    }
};
        locals.var_isubld = assign106040_e158390;
        locals.var_isubld_dn0 = assign106040_e158390_d_n0;
        locals.var_isubld_dn2 = assign106040_e158390_d_n2;
        locals.var_isubld_dn4 = assign106040_e158390_d_n4;
        locals.var_isubld_dn5 = assign106040_e158390_d_n5;
        locals.var_isubld_dn6 = assign106040_e158390_d_n6;
        locals.var_isubld_dn7 = assign106040_e158390_d_n7;
        locals.var_isubld_dn8 = assign106040_e158390_d_n8;
        locals.var_isubld_dn9 = assign106040_e158390_d_n9;
        locals.var_isubld_dn10 = assign106040_e158390_d_n10;
        locals.var_isubld_dn11 = assign106040_e158390_d_n11;
        locals.var_isubld_dn14 = assign106040_e158390_d_n14;
        locals.var_isubld_rv = 0.0;

        let (assign106060_e158398, assign106060_e158398_d_n0, assign106060_e158398_d_n2, assign106060_e158398_d_n4, assign106060_e158398_d_n5, assign106060_e158398_d_n6, assign106060_e158398_d_n7, assign106060_e158398_d_n8, assign106060_e158398_d_n9, assign106060_e158398_d_n10, assign106060_e158398_d_n11, assign106060_e158398_d_n14,) = {
    if (locals.var_guard2403 != 0.0) {
        (locals.var_idsibpce, locals.var_idsibpce_dn0, locals.var_idsibpce_dn2, locals.var_idsibpce_dn4, locals.var_idsibpce_dn5, locals.var_idsibpce_dn6, locals.var_idsibpce_dn7, locals.var_idsibpce_dn8, locals.var_idsibpce_dn9, locals.var_idsibpce_dn10, locals.var_idsibpce_dn11, locals.var_idsibpce_dn14,)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14,)
    }
};
        locals.var_idsibpc = assign106060_e158398;
        locals.var_idsibpc_dn0 = assign106060_e158398_d_n0;
        locals.var_idsibpc_dn2 = assign106060_e158398_d_n2;
        locals.var_idsibpc_dn4 = assign106060_e158398_d_n4;
        locals.var_idsibpc_dn5 = assign106060_e158398_d_n5;
        locals.var_idsibpc_dn6 = assign106060_e158398_d_n6;
        locals.var_idsibpc_dn7 = assign106060_e158398_d_n7;
        locals.var_idsibpc_dn8 = assign106060_e158398_d_n8;
        locals.var_idsibpc_dn9 = assign106060_e158398_d_n9;
        locals.var_idsibpc_dn10 = assign106060_e158398_d_n10;
        locals.var_idsibpc_dn11 = assign106060_e158398_d_n11;
        locals.var_idsibpc_dn14 = assign106060_e158398_d_n14;
        locals.var_idsibpc_rv = 0.0;

        let (assign106140_e158432, assign106140_e158432_d_n0, assign106140_e158432_d_n2, assign106140_e158432_d_n4, assign106140_e158432_d_n5, assign106140_e158432_d_n6, assign106140_e158432_d_n7, assign106140_e158432_d_n8, assign106140_e158432_d_n9, assign106140_e158432_d_n10, assign106140_e158432_d_n11, assign106140_e158432_d_n14,) = {
    if ((locals.var_guard2403 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign106140_e158432;
        locals.var_qdrat_dn0 = assign106140_e158432_d_n0;
        locals.var_qdrat_dn2 = assign106140_e158432_d_n2;
        locals.var_qdrat_dn4 = assign106140_e158432_d_n4;
        locals.var_qdrat_dn5 = assign106140_e158432_d_n5;
        locals.var_qdrat_dn6 = assign106140_e158432_d_n6;
        locals.var_qdrat_dn7 = assign106140_e158432_d_n7;
        locals.var_qdrat_dn8 = assign106140_e158432_d_n8;
        locals.var_qdrat_dn9 = assign106140_e158432_d_n9;
        locals.var_qdrat_dn10 = assign106140_e158432_d_n10;
        locals.var_qdrat_dn11 = assign106140_e158432_d_n11;
        locals.var_qdrat_dn14 = assign106140_e158432_d_n14;
        locals.var_qdrat_rv = 0.0;

        let (assign106150_e158438, assign106150_e158438_d_n0, assign106150_e158438_d_n2, assign106150_e158438_d_n4, assign106150_e158438_d_n5, assign106150_e158438_d_n6, assign106150_e158438_d_n7, assign106150_e158438_d_n8, assign106150_e158438_d_n9, assign106150_e158438_d_n10, assign106150_e158438_d_n11, assign106150_e158438_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        let assign106150_e158436: f64 = (-locals.var_idse);
        (assign106150_e158436, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn8), (-locals.var_idse_dn9), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign106150_e158438;
        locals.var_ids_dn0 = assign106150_e158438_d_n0;
        locals.var_ids_dn2 = assign106150_e158438_d_n2;
        locals.var_ids_dn4 = assign106150_e158438_d_n4;
        locals.var_ids_dn5 = assign106150_e158438_d_n5;
        locals.var_ids_dn6 = assign106150_e158438_d_n6;
        locals.var_ids_dn7 = assign106150_e158438_d_n7;
        locals.var_ids_dn8 = assign106150_e158438_d_n8;
        locals.var_ids_dn9 = assign106150_e158438_d_n9;
        locals.var_ids_dn10 = assign106150_e158438_d_n10;
        locals.var_ids_dn11 = assign106150_e158438_d_n11;
        locals.var_ids_dn14 = assign106150_e158438_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign106160_e158443, assign106160_e158443_d_n0, assign106160_e158443_d_n2, assign106160_e158443_d_n4, assign106160_e158443_d_n5, assign106160_e158443_d_n6, assign106160_e158443_d_n7, assign106160_e158443_d_n8, assign106160_e158443_d_n9, assign106160_e158443_d_n10, assign106160_e158443_d_n11, assign106160_e158443_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14,)
    }
};
        locals.var_qd = assign106160_e158443;
        locals.var_qd_dn0 = assign106160_e158443_d_n0;
        locals.var_qd_dn2 = assign106160_e158443_d_n2;
        locals.var_qd_dn4 = assign106160_e158443_d_n4;
        locals.var_qd_dn5 = assign106160_e158443_d_n5;
        locals.var_qd_dn6 = assign106160_e158443_d_n6;
        locals.var_qd_dn7 = assign106160_e158443_d_n7;
        locals.var_qd_dn8 = assign106160_e158443_d_n8;
        locals.var_qd_dn9 = assign106160_e158443_d_n9;
        locals.var_qd_dn10 = assign106160_e158443_d_n10;
        locals.var_qd_dn11 = assign106160_e158443_d_n11;
        locals.var_qd_dn14 = assign106160_e158443_d_n14;
        locals.var_qd_rv = 0.0;

        let (assign106170_e158448, assign106170_e158448_d_n0, assign106170_e158448_d_n2, assign106170_e158448_d_n4, assign106170_e158448_d_n5, assign106170_e158448_d_n6, assign106170_e158448_d_n7, assign106170_e158448_d_n8, assign106170_e158448_d_n9, assign106170_e158448_d_n10, assign106170_e158448_d_n11, assign106170_e158448_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14,)
    }
};
        locals.var_qg = assign106170_e158448;
        locals.var_qg_dn0 = assign106170_e158448_d_n0;
        locals.var_qg_dn2 = assign106170_e158448_d_n2;
        locals.var_qg_dn4 = assign106170_e158448_d_n4;
        locals.var_qg_dn5 = assign106170_e158448_d_n5;
        locals.var_qg_dn6 = assign106170_e158448_d_n6;
        locals.var_qg_dn7 = assign106170_e158448_d_n7;
        locals.var_qg_dn8 = assign106170_e158448_d_n8;
        locals.var_qg_dn9 = assign106170_e158448_d_n9;
        locals.var_qg_dn10 = assign106170_e158448_d_n10;
        locals.var_qg_dn11 = assign106170_e158448_d_n11;
        locals.var_qg_dn14 = assign106170_e158448_d_n14;
        locals.var_qg_rv = 0.0;

        let (assign106180_e158453, assign106180_e158453_d_n0, assign106180_e158453_d_n2, assign106180_e158453_d_n4, assign106180_e158453_d_n5, assign106180_e158453_d_n6, assign106180_e158453_d_n7, assign106180_e158453_d_n8, assign106180_e158453_d_n9, assign106180_e158453_d_n10, assign106180_e158453_d_n11, assign106180_e158453_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    } else {
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14,)
    }
};
        locals.var_qs = assign106180_e158453;
        locals.var_qs_dn0 = assign106180_e158453_d_n0;
        locals.var_qs_dn2 = assign106180_e158453_d_n2;
        locals.var_qs_dn4 = assign106180_e158453_d_n4;
        locals.var_qs_dn5 = assign106180_e158453_d_n5;
        locals.var_qs_dn6 = assign106180_e158453_d_n6;
        locals.var_qs_dn7 = assign106180_e158453_d_n7;
        locals.var_qs_dn8 = assign106180_e158453_d_n8;
        locals.var_qs_dn9 = assign106180_e158453_d_n9;
        locals.var_qs_dn10 = assign106180_e158453_d_n10;
        locals.var_qs_dn11 = assign106180_e158453_d_n11;
        locals.var_qs_dn14 = assign106180_e158453_d_n14;
        locals.var_qs_rv = 0.0;

        let (assign106190_e158463, assign106190_e158463_d_n0, assign106190_e158463_d_n2, assign106190_e158463_d_n4, assign106190_e158463_d_n5, assign106190_e158463_d_n6, assign106190_e158463_d_n7, assign106190_e158463_d_n8, assign106190_e158463_d_n9, assign106190_e158463_d_n10, assign106190_e158463_d_n11, assign106190_e158463_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        let assign106190_e158458: f64 = (locals.var_qge + locals.var_qde);
        let assign106190_e158460: f64 = (assign106190_e158458 + locals.var_qse);
        let assign106190_e158461: f64 = (-assign106190_e158460);
        (assign106190_e158461, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)),)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14,)
    }
};
        locals.var_qb = assign106190_e158463;
        locals.var_qb_dn0 = assign106190_e158463_d_n0;
        locals.var_qb_dn2 = assign106190_e158463_d_n2;
        locals.var_qb_dn4 = assign106190_e158463_d_n4;
        locals.var_qb_dn5 = assign106190_e158463_d_n5;
        locals.var_qb_dn6 = assign106190_e158463_d_n6;
        locals.var_qb_dn7 = assign106190_e158463_d_n7;
        locals.var_qb_dn8 = assign106190_e158463_d_n8;
        locals.var_qb_dn9 = assign106190_e158463_d_n9;
        locals.var_qb_dn10 = assign106190_e158463_d_n10;
        locals.var_qb_dn11 = assign106190_e158463_d_n11;
        locals.var_qb_dn14 = assign106190_e158463_d_n14;
        locals.var_qb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_406(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign106200_e158468, assign106200_e158468_d_n0, assign106200_e158468_d_n2, assign106200_e158468_d_n4, assign106200_e158468_d_n5, assign106200_e158468_d_n6, assign106200_e158468_d_n7, assign106200_e158468_d_n8, assign106200_e158468_d_n9, assign106200_e158468_d_n10, assign106200_e158468_d_n11, assign106200_e158468_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14,)
    }
};
        locals.var_isub = assign106200_e158468;
        locals.var_isub_dn0 = assign106200_e158468_d_n0;
        locals.var_isub_dn2 = assign106200_e158468_d_n2;
        locals.var_isub_dn4 = assign106200_e158468_d_n4;
        locals.var_isub_dn5 = assign106200_e158468_d_n5;
        locals.var_isub_dn6 = assign106200_e158468_d_n6;
        locals.var_isub_dn7 = assign106200_e158468_d_n7;
        locals.var_isub_dn8 = assign106200_e158468_d_n8;
        locals.var_isub_dn9 = assign106200_e158468_d_n9;
        locals.var_isub_dn10 = assign106200_e158468_d_n10;
        locals.var_isub_dn11 = assign106200_e158468_d_n11;
        locals.var_isub_dn14 = assign106200_e158468_d_n14;
        locals.var_isub_rv = 0.0;

        let (assign106220_e158478, assign106220_e158478_d_n0, assign106220_e158478_d_n2, assign106220_e158478_d_n4, assign106220_e158478_d_n5, assign106220_e158478_d_n6, assign106220_e158478_d_n7, assign106220_e158478_d_n8, assign106220_e158478_d_n9, assign106220_e158478_d_n10, assign106220_e158478_d_n11, assign106220_e158478_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14,)
    }
};
        locals.var_isubld = assign106220_e158478;
        locals.var_isubld_dn0 = assign106220_e158478_d_n0;
        locals.var_isubld_dn2 = assign106220_e158478_d_n2;
        locals.var_isubld_dn4 = assign106220_e158478_d_n4;
        locals.var_isubld_dn5 = assign106220_e158478_d_n5;
        locals.var_isubld_dn6 = assign106220_e158478_d_n6;
        locals.var_isubld_dn7 = assign106220_e158478_d_n7;
        locals.var_isubld_dn8 = assign106220_e158478_d_n8;
        locals.var_isubld_dn9 = assign106220_e158478_d_n9;
        locals.var_isubld_dn10 = assign106220_e158478_d_n10;
        locals.var_isubld_dn11 = assign106220_e158478_d_n11;
        locals.var_isubld_dn14 = assign106220_e158478_d_n14;
        locals.var_isubld_rv = 0.0;

        let (assign106240_e158488, assign106240_e158488_d_n0, assign106240_e158488_d_n2, assign106240_e158488_d_n4, assign106240_e158488_d_n5, assign106240_e158488_d_n6, assign106240_e158488_d_n7, assign106240_e158488_d_n8, assign106240_e158488_d_n9, assign106240_e158488_d_n10, assign106240_e158488_d_n11, assign106240_e158488_d_n14,) = {
    if (locals.var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14,)
    }
};
        locals.var_idsibpc = assign106240_e158488;
        locals.var_idsibpc_dn0 = assign106240_e158488_d_n0;
        locals.var_idsibpc_dn2 = assign106240_e158488_d_n2;
        locals.var_idsibpc_dn4 = assign106240_e158488_d_n4;
        locals.var_idsibpc_dn5 = assign106240_e158488_d_n5;
        locals.var_idsibpc_dn6 = assign106240_e158488_d_n6;
        locals.var_idsibpc_dn7 = assign106240_e158488_d_n7;
        locals.var_idsibpc_dn8 = assign106240_e158488_d_n8;
        locals.var_idsibpc_dn9 = assign106240_e158488_d_n9;
        locals.var_idsibpc_dn10 = assign106240_e158488_d_n10;
        locals.var_idsibpc_dn11 = assign106240_e158488_d_n11;
        locals.var_idsibpc_dn14 = assign106240_e158488_d_n14;
        locals.var_idsibpc_rv = 0.0;

        let (assign106320_e158532, assign106320_e158532_d_n0, assign106320_e158532_d_n2, assign106320_e158532_d_n4, assign106320_e158532_d_n5, assign106320_e158532_d_n6, assign106320_e158532_d_n7, assign106320_e158532_d_n8, assign106320_e158532_d_n9, assign106320_e158532_d_n10, assign106320_e158532_d_n11, assign106320_e158532_d_n14,) = {
    if ((locals.var_guard2403 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign106320_e158530: f64 = (1.0 - locals.var_xd);
        (assign106320_e158530, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn4), (-locals.var_xd_dn5), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn8), (-locals.var_xd_dn9), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn14),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign106320_e158532;
        locals.var_qdrat_dn0 = assign106320_e158532_d_n0;
        locals.var_qdrat_dn2 = assign106320_e158532_d_n2;
        locals.var_qdrat_dn4 = assign106320_e158532_d_n4;
        locals.var_qdrat_dn5 = assign106320_e158532_d_n5;
        locals.var_qdrat_dn6 = assign106320_e158532_d_n6;
        locals.var_qdrat_dn7 = assign106320_e158532_d_n7;
        locals.var_qdrat_dn8 = assign106320_e158532_d_n8;
        locals.var_qdrat_dn9 = assign106320_e158532_d_n9;
        locals.var_qdrat_dn10 = assign106320_e158532_d_n10;
        locals.var_qdrat_dn11 = assign106320_e158532_d_n11;
        locals.var_qdrat_dn14 = assign106320_e158532_d_n14;
        locals.var_qdrat_rv = 0.0;

        let assign106330_e158535: f64 = (locals.var_qg + locals.var_qgov);
        locals.var_qg = assign106330_e158535;
        locals.var_qg_dn0 = (locals.var_qg_dn0 + locals.var_qgov_dn0);
        locals.var_qg_dn2 = (locals.var_qg_dn2 + locals.var_qgov_dn2);
        locals.var_qg_dn4 = (locals.var_qg_dn4 + locals.var_qgov_dn4);
        locals.var_qg_dn5 = (locals.var_qg_dn5 + locals.var_qgov_dn5);
        locals.var_qg_dn6 = (locals.var_qg_dn6 + locals.var_qgov_dn6);
        locals.var_qg_dn7 = (locals.var_qg_dn7 + locals.var_qgov_dn7);
        locals.var_qg_dn8 = (locals.var_qg_dn8 + locals.var_qgov_dn8);
        locals.var_qg_dn9 = (locals.var_qg_dn9 + locals.var_qgov_dn9);
        locals.var_qg_dn10 = (locals.var_qg_dn10 + locals.var_qgov_dn10);
        locals.var_qg_dn11 = (locals.var_qg_dn11 + locals.var_qgov_dn11);
        locals.var_qg_dn14 = (locals.var_qg_dn14 + locals.var_qgov_dn14);
        locals.var_qg_rv = 0.0;

        let assign106340_e158538: f64 = (locals.var_qd + locals.var_qdov);
        locals.var_qd = assign106340_e158538;
        locals.var_qd_dn0 = (locals.var_qd_dn0 + locals.var_qdov_dn0);
        locals.var_qd_dn2 = (locals.var_qd_dn2 + locals.var_qdov_dn2);
        locals.var_qd_dn4 = (locals.var_qd_dn4 + locals.var_qdov_dn4);
        locals.var_qd_dn5 = (locals.var_qd_dn5 + locals.var_qdov_dn5);
        locals.var_qd_dn6 = (locals.var_qd_dn6 + locals.var_qdov_dn6);
        locals.var_qd_dn7 = (locals.var_qd_dn7 + locals.var_qdov_dn7);
        locals.var_qd_dn8 = (locals.var_qd_dn8 + locals.var_qdov_dn8);
        locals.var_qd_dn9 = (locals.var_qd_dn9 + locals.var_qdov_dn9);
        locals.var_qd_dn10 = (locals.var_qd_dn10 + locals.var_qdov_dn10);
        locals.var_qd_dn11 = (locals.var_qd_dn11 + locals.var_qdov_dn11);
        locals.var_qd_dn14 = (locals.var_qd_dn14 + locals.var_qdov_dn14);
        locals.var_qd_rv = 0.0;

        let assign106350_e158541: f64 = (locals.var_qs + locals.var_qsov);
        locals.var_qs = assign106350_e158541;
        locals.var_qs_dn0 = (locals.var_qs_dn0 + locals.var_qsov_dn0);
        locals.var_qs_dn2 = (locals.var_qs_dn2 + locals.var_qsov_dn2);
        locals.var_qs_dn4 = (locals.var_qs_dn4 + locals.var_qsov_dn4);
        locals.var_qs_dn5 = (locals.var_qs_dn5 + locals.var_qsov_dn5);
        locals.var_qs_dn6 = (locals.var_qs_dn6 + locals.var_qsov_dn6);
        locals.var_qs_dn7 = (locals.var_qs_dn7 + locals.var_qsov_dn7);
        locals.var_qs_dn8 = (locals.var_qs_dn8 + locals.var_qsov_dn8);
        locals.var_qs_dn9 = (locals.var_qs_dn9 + locals.var_qsov_dn9);
        locals.var_qs_dn10 = (locals.var_qs_dn10 + locals.var_qsov_dn10);
        locals.var_qs_dn11 = (locals.var_qs_dn11 + locals.var_qsov_dn11);
        locals.var_qs_dn14 = (locals.var_qs_dn14 + locals.var_qsov_dn14);
        locals.var_qs_rv = 0.0;

        let assign106360_e158544: f64 = (locals.var_qg + locals.var_qd);
        let assign106360_e158546: f64 = (assign106360_e158544 + locals.var_qs);
        let assign106360_e158547: f64 = (-assign106360_e158546);
        locals.var_qb = assign106360_e158547;
        locals.var_qb_dn0 = (-((locals.var_qg_dn0 + locals.var_qd_dn0) + locals.var_qs_dn0));
        locals.var_qb_dn2 = (-((locals.var_qg_dn2 + locals.var_qd_dn2) + locals.var_qs_dn2));
        locals.var_qb_dn4 = (-((locals.var_qg_dn4 + locals.var_qd_dn4) + locals.var_qs_dn4));
        locals.var_qb_dn5 = (-((locals.var_qg_dn5 + locals.var_qd_dn5) + locals.var_qs_dn5));
        locals.var_qb_dn6 = (-((locals.var_qg_dn6 + locals.var_qd_dn6) + locals.var_qs_dn6));
        locals.var_qb_dn7 = (-((locals.var_qg_dn7 + locals.var_qd_dn7) + locals.var_qs_dn7));
        locals.var_qb_dn8 = (-((locals.var_qg_dn8 + locals.var_qd_dn8) + locals.var_qs_dn8));
        locals.var_qb_dn9 = (-((locals.var_qg_dn9 + locals.var_qd_dn9) + locals.var_qs_dn9));
        locals.var_qb_dn10 = (-((locals.var_qg_dn10 + locals.var_qd_dn10) + locals.var_qs_dn10));
        locals.var_qb_dn11 = (-((locals.var_qg_dn11 + locals.var_qd_dn11) + locals.var_qs_dn11));
        locals.var_qb_dn14 = (-((locals.var_qg_dn14 + locals.var_qd_dn14) + locals.var_qs_dn14));
        locals.var_qb_rv = 0.0;

        locals.var_qfd = locals.var_qdp;
        locals.var_qfd_dn0 = locals.var_qdp_dn0;
        locals.var_qfd_dn2 = locals.var_qdp_dn2;
        locals.var_qfd_dn7 = locals.var_qdp_dn7;
        locals.var_qfd_rv = 0.0;

        locals.var_qfs = locals.var_qsp;
        locals.var_qfs_dn2 = locals.var_qsp_dn2;
        locals.var_qfs_dn7 = locals.var_qsp_dn7;
        locals.var_qfs_rv = 0.0;

        locals.var_qdext = locals.var_qdexte;
        locals.var_qdext_dn0 = locals.var_qdexte_dn0;
        locals.var_qdext_dn2 = locals.var_qdexte_dn2;
        locals.var_qdext_dn4 = locals.var_qdexte_dn4;
        locals.var_qdext_dn5 = locals.var_qdexte_dn5;
        locals.var_qdext_dn6 = locals.var_qdexte_dn6;
        locals.var_qdext_dn7 = locals.var_qdexte_dn7;
        locals.var_qdext_dn8 = locals.var_qdexte_dn8;
        locals.var_qdext_dn9 = locals.var_qdexte_dn9;
        locals.var_qdext_dn10 = locals.var_qdexte_dn10;
        locals.var_qdext_dn11 = locals.var_qdexte_dn11;
        locals.var_qdext_dn14 = locals.var_qdexte_dn14;
        locals.var_qdext_rv = 0.0;

        locals.var_qgext = locals.var_qgexte;
        locals.var_qgext_dn0 = locals.var_qgexte_dn0;
        locals.var_qgext_dn2 = locals.var_qgexte_dn2;
        locals.var_qgext_dn4 = locals.var_qgexte_dn4;
        locals.var_qgext_dn5 = locals.var_qgexte_dn5;
        locals.var_qgext_dn6 = locals.var_qgexte_dn6;
        locals.var_qgext_dn7 = locals.var_qgexte_dn7;
        locals.var_qgext_dn8 = locals.var_qgexte_dn8;
        locals.var_qgext_dn9 = locals.var_qgexte_dn9;
        locals.var_qgext_dn10 = locals.var_qgexte_dn10;
        locals.var_qgext_dn11 = locals.var_qgexte_dn11;
        locals.var_qgext_dn14 = locals.var_qgexte_dn14;
        locals.var_qgext_rv = 0.0;

        let assign106410_e158554: f64 = (locals.var_qgexte + locals.var_qdexte);
        let assign106410_e158556: f64 = (assign106410_e158554 + locals.var_qsexte);
        let assign106410_e158557: f64 = (-assign106410_e158556);
        locals.var_qbext = assign106410_e158557;
        locals.var_qbext_dn0 = (-((locals.var_qgexte_dn0 + locals.var_qdexte_dn0) + locals.var_qsexte_dn0));
        locals.var_qbext_dn2 = (-((locals.var_qgexte_dn2 + locals.var_qdexte_dn2) + locals.var_qsexte_dn2));
        locals.var_qbext_dn4 = (-((locals.var_qgexte_dn4 + locals.var_qdexte_dn4) + locals.var_qsexte_dn4));
        locals.var_qbext_dn5 = (-((locals.var_qgexte_dn5 + locals.var_qdexte_dn5) + locals.var_qsexte_dn5));
        locals.var_qbext_dn6 = (-((locals.var_qgexte_dn6 + locals.var_qdexte_dn6) + locals.var_qsexte_dn6));
        locals.var_qbext_dn7 = (-((locals.var_qgexte_dn7 + locals.var_qdexte_dn7) + locals.var_qsexte_dn7));
        locals.var_qbext_dn8 = (-((locals.var_qgexte_dn8 + locals.var_qdexte_dn8) + locals.var_qsexte_dn8));
        locals.var_qbext_dn9 = (-((locals.var_qgexte_dn9 + locals.var_qdexte_dn9) + locals.var_qsexte_dn9));
        locals.var_qbext_dn10 = (-((locals.var_qgexte_dn10 + locals.var_qdexte_dn10) + locals.var_qsexte_dn10));
        locals.var_qbext_dn11 = (-((locals.var_qgexte_dn11 + locals.var_qdexte_dn11) + locals.var_qsexte_dn11));
        locals.var_qbext_dn14 = (-((locals.var_qgexte_dn14 + locals.var_qdexte_dn14) + locals.var_qsexte_dn14));
        locals.var_qbext_rv = 0.0;

        let assign106420_e158560: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2404 = assign106420_e158560;
        locals.var_guard2404_rv = 0.0;

        let assign106430_e158563: f64 = if locals.var_rth > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard2405 = assign106430_e158563;
        locals.var_guard2405_rv = 0.0;

        let (assign106440_e158571, assign106440_e158571_d_n0, assign106440_e158571_d_n2, assign106440_e158571_d_n4, assign106440_e158571_d_n5, assign106440_e158571_d_n6, assign106440_e158571_d_n7, assign106440_e158571_d_n8, assign106440_e158571_d_n9, assign106440_e158571_d_n10, assign106440_e158571_d_n11, assign106440_e158571_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2405 != 0.0)) {
        let assign106440_e158569: f64 = (1.0 / locals.var_rth);
        (assign106440_e158569, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn7 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn9 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn11 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn14 / (locals.var_rth * locals.var_rth))),)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14,)
    }
};
        locals.var_gth = assign106440_e158571;
        locals.var_gth_dn0 = assign106440_e158571_d_n0;
        locals.var_gth_dn2 = assign106440_e158571_d_n2;
        locals.var_gth_dn4 = assign106440_e158571_d_n4;
        locals.var_gth_dn5 = assign106440_e158571_d_n5;
        locals.var_gth_dn6 = assign106440_e158571_d_n6;
        locals.var_gth_dn7 = assign106440_e158571_d_n7;
        locals.var_gth_dn8 = assign106440_e158571_d_n8;
        locals.var_gth_dn9 = assign106440_e158571_d_n9;
        locals.var_gth_dn10 = assign106440_e158571_d_n10;
        locals.var_gth_dn11 = assign106440_e158571_d_n11;
        locals.var_gth_dn14 = assign106440_e158571_d_n14;
        locals.var_gth_rv = 0.0;

        let (assign106450_e158580, assign106450_e158580_d_n0, assign106450_e158580_d_n2, assign106450_e158580_d_n4, assign106450_e158580_d_n5, assign106450_e158580_d_n6, assign106450_e158580_d_n7, assign106450_e158580_d_n8, assign106450_e158580_d_n9, assign106450_e158580_d_n10, assign106450_e158580_d_n11, assign106450_e158580_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2405 == 0.0)) {
        let assign106450_e158578: f64 = (1.0 / 0.0001);
        (assign106450_e158578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14,)
    }
};
        locals.var_gth = assign106450_e158580;
        locals.var_gth_dn0 = assign106450_e158580_d_n0;
        locals.var_gth_dn2 = assign106450_e158580_d_n2;
        locals.var_gth_dn4 = assign106450_e158580_d_n4;
        locals.var_gth_dn5 = assign106450_e158580_d_n5;
        locals.var_gth_dn6 = assign106450_e158580_d_n6;
        locals.var_gth_dn7 = assign106450_e158580_d_n7;
        locals.var_gth_dn8 = assign106450_e158580_d_n8;
        locals.var_gth_dn9 = assign106450_e158580_d_n9;
        locals.var_gth_dn10 = assign106450_e158580_d_n10;
        locals.var_gth_dn11 = assign106450_e158580_d_n11;
        locals.var_gth_dn14 = assign106450_e158580_d_n14;
        locals.var_gth_rv = 0.0;

        let assign106460_e158584: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106460_e158585: f64 = (locals.var_vdsi * assign106460_e158584);
        let assign106460_e158587: f64 = if assign106460_e158585 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2406 = assign106460_e158587;
        locals.var_guard2406_rv = 0.0;

        let assign106470_e158590: f64 = if locals.var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2407 = assign106470_e158590;
        locals.var_guard2407_rv = 0.0;

        let (assign106480_e158598, assign106480_e158598_d_n0, assign106480_e158598_d_n2, assign106480_e158598_d_n4, assign106480_e158598_d_n5, assign106480_e158598_d_n6, assign106480_e158598_d_n7, assign106480_e158598_d_n8, assign106480_e158598_d_n9, assign106480_e158598_d_n10, assign106480_e158598_d_n11, assign106480_e158598_d_n14,) = {
    if (((locals.var_guard2404 != 0.0) && (locals.var_guard2406 != 0.0)) && (locals.var_guard2407 != 0.0)) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14,)
    }
};
        locals.var_veffpower = assign106480_e158598;
        locals.var_veffpower_dn0 = assign106480_e158598_d_n0;
        locals.var_veffpower_dn2 = assign106480_e158598_d_n2;
        locals.var_veffpower_dn4 = assign106480_e158598_d_n4;
        locals.var_veffpower_dn5 = assign106480_e158598_d_n5;
        locals.var_veffpower_dn6 = assign106480_e158598_d_n6;
        locals.var_veffpower_dn7 = assign106480_e158598_d_n7;
        locals.var_veffpower_dn8 = assign106480_e158598_d_n8;
        locals.var_veffpower_dn9 = assign106480_e158598_d_n9;
        locals.var_veffpower_dn10 = assign106480_e158598_d_n10;
        locals.var_veffpower_dn11 = assign106480_e158598_d_n11;
        locals.var_veffpower_dn14 = assign106480_e158598_d_n14;
        locals.var_veffpower_rv = 0.0;

        let (assign106490_e158613, assign106490_e158613_d_n0, assign106490_e158613_d_n2, assign106490_e158613_d_n4, assign106490_e158613_d_n5, assign106490_e158613_d_n6, assign106490_e158613_d_n7, assign106490_e158613_d_n8, assign106490_e158613_d_n9, assign106490_e158613_d_n10, assign106490_e158613_d_n11, assign106490_e158613_d_n14,) = {
    if (((locals.var_guard2404 != 0.0) && (locals.var_guard2406 != 0.0)) && (locals.var_guard2407 == 0.0)) {
        let assign106490_e158609: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106490_e158610: f64 = (locals.var_powratio * assign106490_e158609);
        let assign106490_e158611: f64 = (locals.var_vdsi + assign106490_e158610);
        (assign106490_e158611, ((locals.var_powratio_dn0 * assign106490_e158609) + (locals.var_powratio * locals.var_vdsei_dn0)), ((locals.var_powratio_dn2 * assign106490_e158609) + (locals.var_powratio * locals.var_vdsei_dn2)), (locals.var_powratio_dn4 * assign106490_e158609), (locals.var_powratio_dn5 * assign106490_e158609), (locals.var_vdsi_dn6 + ((locals.var_powratio_dn6 * assign106490_e158609) + (locals.var_powratio * (-locals.var_vdsi_dn6)))), (locals.var_powratio_dn7 * assign106490_e158609), (locals.var_vdsi_dn8 + ((locals.var_powratio_dn8 * assign106490_e158609) + (locals.var_powratio * (-locals.var_vdsi_dn8)))), (locals.var_powratio_dn9 * assign106490_e158609), (locals.var_powratio_dn10 * assign106490_e158609), (locals.var_powratio_dn11 * assign106490_e158609), (locals.var_powratio_dn14 * assign106490_e158609),)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14,)
    }
};
        locals.var_veffpower = assign106490_e158613;
        locals.var_veffpower_dn0 = assign106490_e158613_d_n0;
        locals.var_veffpower_dn2 = assign106490_e158613_d_n2;
        locals.var_veffpower_dn4 = assign106490_e158613_d_n4;
        locals.var_veffpower_dn5 = assign106490_e158613_d_n5;
        locals.var_veffpower_dn6 = assign106490_e158613_d_n6;
        locals.var_veffpower_dn7 = assign106490_e158613_d_n7;
        locals.var_veffpower_dn8 = assign106490_e158613_d_n8;
        locals.var_veffpower_dn9 = assign106490_e158613_d_n9;
        locals.var_veffpower_dn10 = assign106490_e158613_d_n10;
        locals.var_veffpower_dn11 = assign106490_e158613_d_n11;
        locals.var_veffpower_dn14 = assign106490_e158613_d_n14;
        locals.var_veffpower_rv = 0.0;

        let (assign106500_e158620, assign106500_e158620_d_n0, assign106500_e158620_d_n2, assign106500_e158620_d_n4, assign106500_e158620_d_n5, assign106500_e158620_d_n6, assign106500_e158620_d_n7, assign106500_e158620_d_n8, assign106500_e158620_d_n9, assign106500_e158620_d_n10, assign106500_e158620_d_n11, assign106500_e158620_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2406 == 0.0)) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, locals.var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14,)
    }
};
        locals.var_veffpower = assign106500_e158620;
        locals.var_veffpower_dn0 = assign106500_e158620_d_n0;
        locals.var_veffpower_dn2 = assign106500_e158620_d_n2;
        locals.var_veffpower_dn4 = assign106500_e158620_d_n4;
        locals.var_veffpower_dn5 = assign106500_e158620_d_n5;
        locals.var_veffpower_dn6 = assign106500_e158620_d_n6;
        locals.var_veffpower_dn7 = assign106500_e158620_d_n7;
        locals.var_veffpower_dn8 = assign106500_e158620_d_n8;
        locals.var_veffpower_dn9 = assign106500_e158620_d_n9;
        locals.var_veffpower_dn10 = assign106500_e158620_d_n10;
        locals.var_veffpower_dn11 = assign106500_e158620_d_n11;
        locals.var_veffpower_dn14 = assign106500_e158620_d_n14;
        locals.var_veffpower_rv = 0.0;

        let (assign106510_e158626, assign106510_e158626_d_n0, assign106510_e158626_d_n2, assign106510_e158626_d_n4, assign106510_e158626_d_n5, assign106510_e158626_d_n6, assign106510_e158626_d_n7, assign106510_e158626_d_n8, assign106510_e158626_d_n9, assign106510_e158626_d_n10, assign106510_e158626_d_n11, assign106510_e158626_d_n14,) = {
    if (locals.var_guard2404 != 0.0) {
        let assign106510_e158624: f64 = (locals.var_ids * locals.var_veffpower);
        (assign106510_e158624, ((locals.var_ids_dn0 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn0)), ((locals.var_ids_dn2 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn2)), ((locals.var_ids_dn4 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn4)), ((locals.var_ids_dn5 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn5)), ((locals.var_ids_dn6 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn6)), ((locals.var_ids_dn7 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn7)), ((locals.var_ids_dn8 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn8)), ((locals.var_ids_dn9 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn9)), ((locals.var_ids_dn10 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn10)), ((locals.var_ids_dn11 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn11)), ((locals.var_ids_dn14 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn14)),)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14,)
    }
};
        locals.var_p = assign106510_e158626;
        locals.var_p_dn0 = assign106510_e158626_d_n0;
        locals.var_p_dn2 = assign106510_e158626_d_n2;
        locals.var_p_dn4 = assign106510_e158626_d_n4;
        locals.var_p_dn5 = assign106510_e158626_d_n5;
        locals.var_p_dn6 = assign106510_e158626_d_n6;
        locals.var_p_dn7 = assign106510_e158626_d_n7;
        locals.var_p_dn8 = assign106510_e158626_d_n8;
        locals.var_p_dn9 = assign106510_e158626_d_n9;
        locals.var_p_dn10 = assign106510_e158626_d_n10;
        locals.var_p_dn11 = assign106510_e158626_d_n11;
        locals.var_p_dn14 = assign106510_e158626_d_n14;
        locals.var_p_rv = 0.0;

        let assign106520_e158629: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2408 = assign106520_e158629;
        locals.var_guard2408_rv = 0.0;

        let (assign106530_e158637, assign106530_e158637_d_n0, assign106530_e158637_d_n2, assign106530_e158637_d_n4, assign106530_e158637_d_n5, assign106530_e158637_d_n6, assign106530_e158637_d_n7, assign106530_e158637_d_n8, assign106530_e158637_d_n9, assign106530_e158637_d_n10, assign106530_e158637_d_n11, assign106530_e158637_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106530_e158635: f64 = (p.p433 * locals.var_gth);
        (assign106530_e158635, (p.p433 * locals.var_gth_dn0), (p.p433 * locals.var_gth_dn2), (p.p433 * locals.var_gth_dn4), (p.p433 * locals.var_gth_dn5), (p.p433 * locals.var_gth_dn6), (p.p433 * locals.var_gth_dn7), (p.p433 * locals.var_gth_dn8), (p.p433 * locals.var_gth_dn9), (p.p433 * locals.var_gth_dn10), (p.p433 * locals.var_gth_dn11), (p.p433 * locals.var_gth_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign106530_e158637;
        locals.var_t1_dn0 = assign106530_e158637_d_n0;
        locals.var_t1_dn2 = assign106530_e158637_d_n2;
        locals.var_t1_dn4 = assign106530_e158637_d_n4;
        locals.var_t1_dn5 = assign106530_e158637_d_n5;
        locals.var_t1_dn6 = assign106530_e158637_d_n6;
        locals.var_t1_dn7 = assign106530_e158637_d_n7;
        locals.var_t1_dn8 = assign106530_e158637_d_n8;
        locals.var_t1_dn9 = assign106530_e158637_d_n9;
        locals.var_t1_dn10 = assign106530_e158637_d_n10;
        locals.var_t1_dn11 = assign106530_e158637_d_n11;
        locals.var_t1_dn14 = assign106530_e158637_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign106540_e158649, assign106540_e158649_d_n0, assign106540_e158649_d_n2, assign106540_e158649_d_n4, assign106540_e158649_d_n5, assign106540_e158649_d_n6, assign106540_e158649_d_n7, assign106540_e158649_d_n8, assign106540_e158649_d_n9, assign106540_e158649_d_n10, assign106540_e158649_d_n11, assign106540_e158649_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106540_e158643: f64 = (locals.var_t1 - locals.var_p);
        let assign106540_e158646: f64 = (p.p337 * locals.var_gth);
        let assign106540_e158647: f64 = (assign106540_e158643 - assign106540_e158646);
        (assign106540_e158647, ((locals.var_t1_dn0 - locals.var_p_dn0) - (p.p337 * locals.var_gth_dn0)), ((locals.var_t1_dn2 - locals.var_p_dn2) - (p.p337 * locals.var_gth_dn2)), ((locals.var_t1_dn4 - locals.var_p_dn4) - (p.p337 * locals.var_gth_dn4)), ((locals.var_t1_dn5 - locals.var_p_dn5) - (p.p337 * locals.var_gth_dn5)), ((locals.var_t1_dn6 - locals.var_p_dn6) - (p.p337 * locals.var_gth_dn6)), ((locals.var_t1_dn7 - locals.var_p_dn7) - (p.p337 * locals.var_gth_dn7)), ((locals.var_t1_dn8 - locals.var_p_dn8) - (p.p337 * locals.var_gth_dn8)), ((locals.var_t1_dn9 - locals.var_p_dn9) - (p.p337 * locals.var_gth_dn9)), ((locals.var_t1_dn10 - locals.var_p_dn10) - (p.p337 * locals.var_gth_dn10)), ((locals.var_t1_dn11 - locals.var_p_dn11) - (p.p337 * locals.var_gth_dn11)), ((locals.var_t1_dn14 - locals.var_p_dn14) - (p.p337 * locals.var_gth_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign106540_e158649;
        locals.var_tmf1_dn0 = assign106540_e158649_d_n0;
        locals.var_tmf1_dn2 = assign106540_e158649_d_n2;
        locals.var_tmf1_dn4 = assign106540_e158649_d_n4;
        locals.var_tmf1_dn5 = assign106540_e158649_d_n5;
        locals.var_tmf1_dn6 = assign106540_e158649_d_n6;
        locals.var_tmf1_dn7 = assign106540_e158649_d_n7;
        locals.var_tmf1_dn8 = assign106540_e158649_d_n8;
        locals.var_tmf1_dn9 = assign106540_e158649_d_n9;
        locals.var_tmf1_dn10 = assign106540_e158649_d_n10;
        locals.var_tmf1_dn11 = assign106540_e158649_d_n11;
        locals.var_tmf1_dn14 = assign106540_e158649_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign106550_e158661, assign106550_e158661_d_n0, assign106550_e158661_d_n2, assign106550_e158661_d_n4, assign106550_e158661_d_n5, assign106550_e158661_d_n6, assign106550_e158661_d_n7, assign106550_e158661_d_n8, assign106550_e158661_d_n9, assign106550_e158661_d_n10, assign106550_e158661_d_n11, assign106550_e158661_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106550_e158655: f64 = (4.0 * locals.var_t1);
        let assign106550_e158658: f64 = (p.p337 * locals.var_gth);
        let assign106550_e158659: f64 = (assign106550_e158655 * assign106550_e158658);
        (assign106550_e158659, (((4.0 * locals.var_t1_dn0) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn0))), (((4.0 * locals.var_t1_dn2) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn2))), (((4.0 * locals.var_t1_dn4) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn4))), (((4.0 * locals.var_t1_dn5) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn5))), (((4.0 * locals.var_t1_dn6) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn6))), (((4.0 * locals.var_t1_dn7) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn7))), (((4.0 * locals.var_t1_dn8) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn8))), (((4.0 * locals.var_t1_dn9) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn9))), (((4.0 * locals.var_t1_dn10) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn10))), (((4.0 * locals.var_t1_dn11) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn11))), (((4.0 * locals.var_t1_dn14) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign106550_e158661;
        locals.var_tmf2_dn0 = assign106550_e158661_d_n0;
        locals.var_tmf2_dn2 = assign106550_e158661_d_n2;
        locals.var_tmf2_dn4 = assign106550_e158661_d_n4;
        locals.var_tmf2_dn5 = assign106550_e158661_d_n5;
        locals.var_tmf2_dn6 = assign106550_e158661_d_n6;
        locals.var_tmf2_dn7 = assign106550_e158661_d_n7;
        locals.var_tmf2_dn8 = assign106550_e158661_d_n8;
        locals.var_tmf2_dn9 = assign106550_e158661_d_n9;
        locals.var_tmf2_dn10 = assign106550_e158661_d_n10;
        locals.var_tmf2_dn11 = assign106550_e158661_d_n11;
        locals.var_tmf2_dn14 = assign106550_e158661_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign106560_e158673, assign106560_e158673_d_n0, assign106560_e158673_d_n2, assign106560_e158673_d_n4, assign106560_e158673_d_n5, assign106560_e158673_d_n6, assign106560_e158673_d_n7, assign106560_e158673_d_n8, assign106560_e158673_d_n9, assign106560_e158673_d_n10, assign106560_e158673_d_n11, assign106560_e158673_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign106560_e158670: f64 = (-locals.var_tmf2);
                (assign106560_e158670, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign106560_e158673;
        locals.var_tmf2_dn0 = assign106560_e158673_d_n0;
        locals.var_tmf2_dn2 = assign106560_e158673_d_n2;
        locals.var_tmf2_dn4 = assign106560_e158673_d_n4;
        locals.var_tmf2_dn5 = assign106560_e158673_d_n5;
        locals.var_tmf2_dn6 = assign106560_e158673_d_n6;
        locals.var_tmf2_dn7 = assign106560_e158673_d_n7;
        locals.var_tmf2_dn8 = assign106560_e158673_d_n8;
        locals.var_tmf2_dn9 = assign106560_e158673_d_n9;
        locals.var_tmf2_dn10 = assign106560_e158673_d_n10;
        locals.var_tmf2_dn11 = assign106560_e158673_d_n11;
        locals.var_tmf2_dn14 = assign106560_e158673_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign106570_e158684, assign106570_e158684_d_n0, assign106570_e158684_d_n2, assign106570_e158684_d_n4, assign106570_e158684_d_n5, assign106570_e158684_d_n6, assign106570_e158684_d_n7, assign106570_e158684_d_n8, assign106570_e158684_d_n9, assign106570_e158684_d_n10, assign106570_e158684_d_n11, assign106570_e158684_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106570_e158679: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign106570_e158681: f64 = (assign106570_e158679 + locals.var_tmf2);
        let assign106570_e158682: f64 = (assign106570_e158681).sqrt();
        (assign106570_e158682, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign106570_e158682)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign106570_e158684;
        locals.var_tmf2_dn0 = assign106570_e158684_d_n0;
        locals.var_tmf2_dn2 = assign106570_e158684_d_n2;
        locals.var_tmf2_dn4 = assign106570_e158684_d_n4;
        locals.var_tmf2_dn5 = assign106570_e158684_d_n5;
        locals.var_tmf2_dn6 = assign106570_e158684_d_n6;
        locals.var_tmf2_dn7 = assign106570_e158684_d_n7;
        locals.var_tmf2_dn8 = assign106570_e158684_d_n8;
        locals.var_tmf2_dn9 = assign106570_e158684_d_n9;
        locals.var_tmf2_dn10 = assign106570_e158684_d_n10;
        locals.var_tmf2_dn11 = assign106570_e158684_d_n11;
        locals.var_tmf2_dn14 = assign106570_e158684_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign106580_e158696, assign106580_e158696_d_n0, assign106580_e158696_d_n2, assign106580_e158696_d_n4, assign106580_e158696_d_n5, assign106580_e158696_d_n6, assign106580_e158696_d_n7, assign106580_e158696_d_n8, assign106580_e158696_d_n9, assign106580_e158696_d_n10, assign106580_e158696_d_n11, assign106580_e158696_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106580_e158692: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign106580_e158693: f64 = (1.0 + assign106580_e158692);
        let assign106580_e158694: f64 = (0.5 * assign106580_e158693);
        (assign106580_e158694, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign106580_e158696;
        locals.var_t0_dn0 = assign106580_e158696_d_n0;
        locals.var_t0_dn2 = assign106580_e158696_d_n2;
        locals.var_t0_dn4 = assign106580_e158696_d_n4;
        locals.var_t0_dn5 = assign106580_e158696_d_n5;
        locals.var_t0_dn6 = assign106580_e158696_d_n6;
        locals.var_t0_dn7 = assign106580_e158696_d_n7;
        locals.var_t0_dn8 = assign106580_e158696_d_n8;
        locals.var_t0_dn9 = assign106580_e158696_d_n9;
        locals.var_t0_dn10 = assign106580_e158696_d_n10;
        locals.var_t0_dn11 = assign106580_e158696_d_n11;
        locals.var_t0_dn14 = assign106580_e158696_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_407(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign106590_e158708, assign106590_e158708_d_n0, assign106590_e158708_d_n2, assign106590_e158708_d_n4, assign106590_e158708_d_n5, assign106590_e158708_d_n6, assign106590_e158708_d_n7, assign106590_e158708_d_n8, assign106590_e158708_d_n9, assign106590_e158708_d_n10, assign106590_e158708_d_n11, assign106590_e158708_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        let assign106590_e158704: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign106590_e158705: f64 = (0.5 * assign106590_e158704);
        let assign106590_e158706: f64 = (locals.var_t1 - assign106590_e158705);
        (assign106590_e158706, (locals.var_t1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign106590_e158708;
        locals.var_t2_dn0 = assign106590_e158708_d_n0;
        locals.var_t2_dn2 = assign106590_e158708_d_n2;
        locals.var_t2_dn4 = assign106590_e158708_d_n4;
        locals.var_t2_dn5 = assign106590_e158708_d_n5;
        locals.var_t2_dn6 = assign106590_e158708_d_n6;
        locals.var_t2_dn7 = assign106590_e158708_d_n7;
        locals.var_t2_dn8 = assign106590_e158708_d_n8;
        locals.var_t2_dn9 = assign106590_e158708_d_n9;
        locals.var_t2_dn10 = assign106590_e158708_d_n10;
        locals.var_t2_dn11 = assign106590_e158708_d_n11;
        locals.var_t2_dn14 = assign106590_e158708_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign106600_e158714, assign106600_e158714_d_n0, assign106600_e158714_d_n2, assign106600_e158714_d_n4, assign106600_e158714_d_n5, assign106600_e158714_d_n6, assign106600_e158714_d_n7, assign106600_e158714_d_n8, assign106600_e158714_d_n9, assign106600_e158714_d_n10, assign106600_e158714_d_n11, assign106600_e158714_d_n14,) = {
    if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14,)
    }
};
        locals.var_p = assign106600_e158714;
        locals.var_p_dn0 = assign106600_e158714_d_n0;
        locals.var_p_dn2 = assign106600_e158714_d_n2;
        locals.var_p_dn4 = assign106600_e158714_d_n4;
        locals.var_p_dn5 = assign106600_e158714_d_n5;
        locals.var_p_dn6 = assign106600_e158714_d_n6;
        locals.var_p_dn7 = assign106600_e158714_d_n7;
        locals.var_p_dn8 = assign106600_e158714_d_n8;
        locals.var_p_dn9 = assign106600_e158714_d_n9;
        locals.var_p_dn10 = assign106600_e158714_d_n10;
        locals.var_p_dn11 = assign106600_e158714_d_n11;
        locals.var_p_dn14 = assign106600_e158714_d_n14;
        locals.var_p_rv = 0.0;

        let (assign106610_e158719, assign106610_e158719_d_n0, assign106610_e158719_d_n2, assign106610_e158719_d_n4, assign106610_e158719_d_n5, assign106610_e158719_d_n6, assign106610_e158719_d_n7, assign106610_e158719_d_n8, assign106610_e158719_d_n9, assign106610_e158719_d_n10, assign106610_e158719_d_n11, assign106610_e158719_d_n14,) = {
    if (locals.var_guard2404 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14,)
    }
};
        locals.var_gth = assign106610_e158719;
        locals.var_gth_dn0 = assign106610_e158719_d_n0;
        locals.var_gth_dn2 = assign106610_e158719_d_n2;
        locals.var_gth_dn4 = assign106610_e158719_d_n4;
        locals.var_gth_dn5 = assign106610_e158719_d_n5;
        locals.var_gth_dn6 = assign106610_e158719_d_n6;
        locals.var_gth_dn7 = assign106610_e158719_d_n7;
        locals.var_gth_dn8 = assign106610_e158719_d_n8;
        locals.var_gth_dn9 = assign106610_e158719_d_n9;
        locals.var_gth_dn10 = assign106610_e158719_d_n10;
        locals.var_gth_dn11 = assign106610_e158719_d_n11;
        locals.var_gth_dn14 = assign106610_e158719_d_n14;
        locals.var_gth_rv = 0.0;

        let (assign106620_e158724, assign106620_e158724_d_n0, assign106620_e158724_d_n2, assign106620_e158724_d_n4, assign106620_e158724_d_n5, assign106620_e158724_d_n6, assign106620_e158724_d_n7, assign106620_e158724_d_n8, assign106620_e158724_d_n9, assign106620_e158724_d_n10, assign106620_e158724_d_n11, assign106620_e158724_d_n14,) = {
    if (locals.var_guard2404 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14,)
    }
};
        locals.var_p = assign106620_e158724;
        locals.var_p_dn0 = assign106620_e158724_d_n0;
        locals.var_p_dn2 = assign106620_e158724_d_n2;
        locals.var_p_dn4 = assign106620_e158724_d_n4;
        locals.var_p_dn5 = assign106620_e158724_d_n5;
        locals.var_p_dn6 = assign106620_e158724_d_n6;
        locals.var_p_dn7 = assign106620_e158724_d_n7;
        locals.var_p_dn8 = assign106620_e158724_d_n8;
        locals.var_p_dn9 = assign106620_e158724_d_n9;
        locals.var_p_dn10 = assign106620_e158724_d_n10;
        locals.var_p_dn11 = assign106620_e158724_d_n11;
        locals.var_p_dn14 = assign106620_e158724_d_n14;
        locals.var_p_rv = 0.0;

        let (assign106690_e158764, assign106690_e158764_d_n0, assign106690_e158764_d_n2, assign106690_e158764_d_n4, assign106690_e158764_d_n5, assign106690_e158764_d_n6, assign106690_e158764_d_n7, assign106690_e158764_d_n8, assign106690_e158764_d_n9, assign106690_e158764_d_n10, assign106690_e158764_d_n11, assign106690_e158764_d_n12, assign106690_e158764_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106690_e158762: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        (assign106690_e158762, (locals.var_qi_nqs * locals.var_qdrat_dn0), (locals.var_qi_nqs * locals.var_qdrat_dn2), (locals.var_qi_nqs * locals.var_qdrat_dn4), (locals.var_qi_nqs * locals.var_qdrat_dn5), (locals.var_qi_nqs * locals.var_qdrat_dn6), (locals.var_qi_nqs * locals.var_qdrat_dn7), (locals.var_qi_nqs * locals.var_qdrat_dn8), (locals.var_qi_nqs * locals.var_qdrat_dn9), (locals.var_qi_nqs * locals.var_qdrat_dn10), (locals.var_qi_nqs * locals.var_qdrat_dn11), (locals.var_qi_nqs_dn12 * locals.var_qdrat), (locals.var_qi_nqs * locals.var_qdrat_dn14),)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14,)
    }
};
        locals.var_qd_nqs = assign106690_e158764;
        locals.var_qd_nqs_dn0 = assign106690_e158764_d_n0;
        locals.var_qd_nqs_dn2 = assign106690_e158764_d_n2;
        locals.var_qd_nqs_dn4 = assign106690_e158764_d_n4;
        locals.var_qd_nqs_dn5 = assign106690_e158764_d_n5;
        locals.var_qd_nqs_dn6 = assign106690_e158764_d_n6;
        locals.var_qd_nqs_dn7 = assign106690_e158764_d_n7;
        locals.var_qd_nqs_dn8 = assign106690_e158764_d_n8;
        locals.var_qd_nqs_dn9 = assign106690_e158764_d_n9;
        locals.var_qd_nqs_dn10 = assign106690_e158764_d_n10;
        locals.var_qd_nqs_dn11 = assign106690_e158764_d_n11;
        locals.var_qd_nqs_dn12 = assign106690_e158764_d_n12;
        locals.var_qd_nqs_dn14 = assign106690_e158764_d_n14;
        locals.var_qd_nqs_rv = 0.0;

        let (assign106700_e158771, assign106700_e158771_d_n12, assign106700_e158771_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106700_e158767: f64 = (-locals.var_qi_nqs);
        let assign106700_e158769: f64 = (assign106700_e158767 - locals.var_qb_nqs);
        (assign106700_e158769, (-locals.var_qi_nqs_dn12), (-locals.var_qb_nqs_dn13),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13,)
    }
};
        locals.var_qg_nqs = assign106700_e158771;
        locals.var_qg_nqs_dn12 = assign106700_e158771_d_n12;
        locals.var_qg_nqs_dn13 = assign106700_e158771_d_n13;
        locals.var_qg_nqs_rv = 0.0;

        let (assign106710_e158779, assign106710_e158779_d_n0, assign106710_e158779_d_n2, assign106710_e158779_d_n4, assign106710_e158779_d_n5, assign106710_e158779_d_n6, assign106710_e158779_d_n7, assign106710_e158779_d_n8, assign106710_e158779_d_n9, assign106710_e158779_d_n10, assign106710_e158779_d_n11, assign106710_e158779_d_n12, assign106710_e158779_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106710_e158776: f64 = (1.0 - locals.var_qdrat);
        let assign106710_e158777: f64 = (locals.var_qi_nqs * assign106710_e158776);
        (assign106710_e158777, (locals.var_qi_nqs * (-locals.var_qdrat_dn0)), (locals.var_qi_nqs * (-locals.var_qdrat_dn2)), (locals.var_qi_nqs * (-locals.var_qdrat_dn4)), (locals.var_qi_nqs * (-locals.var_qdrat_dn5)), (locals.var_qi_nqs * (-locals.var_qdrat_dn6)), (locals.var_qi_nqs * (-locals.var_qdrat_dn7)), (locals.var_qi_nqs * (-locals.var_qdrat_dn8)), (locals.var_qi_nqs * (-locals.var_qdrat_dn9)), (locals.var_qi_nqs * (-locals.var_qdrat_dn10)), (locals.var_qi_nqs * (-locals.var_qdrat_dn11)), (locals.var_qi_nqs_dn12 * assign106710_e158776), (locals.var_qi_nqs * (-locals.var_qdrat_dn14)),)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14,)
    }
};
        locals.var_qs_nqs = assign106710_e158779;
        locals.var_qs_nqs_dn0 = assign106710_e158779_d_n0;
        locals.var_qs_nqs_dn2 = assign106710_e158779_d_n2;
        locals.var_qs_nqs_dn4 = assign106710_e158779_d_n4;
        locals.var_qs_nqs_dn5 = assign106710_e158779_d_n5;
        locals.var_qs_nqs_dn6 = assign106710_e158779_d_n6;
        locals.var_qs_nqs_dn7 = assign106710_e158779_d_n7;
        locals.var_qs_nqs_dn8 = assign106710_e158779_d_n8;
        locals.var_qs_nqs_dn9 = assign106710_e158779_d_n9;
        locals.var_qs_nqs_dn10 = assign106710_e158779_d_n10;
        locals.var_qs_nqs_dn11 = assign106710_e158779_d_n11;
        locals.var_qs_nqs_dn12 = assign106710_e158779_d_n12;
        locals.var_qs_nqs_dn14 = assign106710_e158779_d_n14;
        locals.var_qs_nqs_rv = 0.0;

        let (assign106740_e158794, assign106740_e158794_d_n0, assign106740_e158794_d_n2, assign106740_e158794_d_n4, assign106740_e158794_d_n5, assign106740_e158794_d_n6, assign106740_e158794_d_n7, assign106740_e158794_d_n8, assign106740_e158794_d_n9, assign106740_e158794_d_n10, assign106740_e158794_d_n11, assign106740_e158794_d_n12, assign106740_e158794_d_n14,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14,)
    }
};
        locals.var_qd_nqs = assign106740_e158794;
        locals.var_qd_nqs_dn0 = assign106740_e158794_d_n0;
        locals.var_qd_nqs_dn2 = assign106740_e158794_d_n2;
        locals.var_qd_nqs_dn4 = assign106740_e158794_d_n4;
        locals.var_qd_nqs_dn5 = assign106740_e158794_d_n5;
        locals.var_qd_nqs_dn6 = assign106740_e158794_d_n6;
        locals.var_qd_nqs_dn7 = assign106740_e158794_d_n7;
        locals.var_qd_nqs_dn8 = assign106740_e158794_d_n8;
        locals.var_qd_nqs_dn9 = assign106740_e158794_d_n9;
        locals.var_qd_nqs_dn10 = assign106740_e158794_d_n10;
        locals.var_qd_nqs_dn11 = assign106740_e158794_d_n11;
        locals.var_qd_nqs_dn12 = assign106740_e158794_d_n12;
        locals.var_qd_nqs_dn14 = assign106740_e158794_d_n14;
        locals.var_qd_nqs_rv = 0.0;

        let (assign106750_e158799, assign106750_e158799_d_n12, assign106750_e158799_d_n13,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13,)
    }
};
        locals.var_qg_nqs = assign106750_e158799;
        locals.var_qg_nqs_dn12 = assign106750_e158799_d_n12;
        locals.var_qg_nqs_dn13 = assign106750_e158799_d_n13;
        locals.var_qg_nqs_rv = 0.0;

        let (assign106760_e158804, assign106760_e158804_d_n0, assign106760_e158804_d_n2, assign106760_e158804_d_n4, assign106760_e158804_d_n5, assign106760_e158804_d_n6, assign106760_e158804_d_n7, assign106760_e158804_d_n8, assign106760_e158804_d_n9, assign106760_e158804_d_n10, assign106760_e158804_d_n11, assign106760_e158804_d_n12, assign106760_e158804_d_n14,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14,)
    }
};
        locals.var_qs_nqs = assign106760_e158804;
        locals.var_qs_nqs_dn0 = assign106760_e158804_d_n0;
        locals.var_qs_nqs_dn2 = assign106760_e158804_d_n2;
        locals.var_qs_nqs_dn4 = assign106760_e158804_d_n4;
        locals.var_qs_nqs_dn5 = assign106760_e158804_d_n5;
        locals.var_qs_nqs_dn6 = assign106760_e158804_d_n6;
        locals.var_qs_nqs_dn7 = assign106760_e158804_d_n7;
        locals.var_qs_nqs_dn8 = assign106760_e158804_d_n8;
        locals.var_qs_nqs_dn9 = assign106760_e158804_d_n9;
        locals.var_qs_nqs_dn10 = assign106760_e158804_d_n10;
        locals.var_qs_nqs_dn11 = assign106760_e158804_d_n11;
        locals.var_qs_nqs_dn12 = assign106760_e158804_d_n12;
        locals.var_qs_nqs_dn14 = assign106760_e158804_d_n14;
        locals.var_qs_nqs_rv = 0.0;

        let assign106770_e158807: f64 = (p.p87 * locals.var_mode);
        let assign106770_e158809: f64 = (assign106770_e158807 * locals.var_ids);
        locals.var_idse = assign106770_e158809;
        locals.var_idse_dn0 = (assign106770_e158807 * locals.var_ids_dn0);
        locals.var_idse_dn2 = (assign106770_e158807 * locals.var_ids_dn2);
        locals.var_idse_dn4 = (assign106770_e158807 * locals.var_ids_dn4);
        locals.var_idse_dn5 = (assign106770_e158807 * locals.var_ids_dn5);
        locals.var_idse_dn6 = (assign106770_e158807 * locals.var_ids_dn6);
        locals.var_idse_dn7 = (assign106770_e158807 * locals.var_ids_dn7);
        locals.var_idse_dn8 = (assign106770_e158807 * locals.var_ids_dn8);
        locals.var_idse_dn9 = (assign106770_e158807 * locals.var_ids_dn9);
        locals.var_idse_dn10 = (assign106770_e158807 * locals.var_ids_dn10);
        locals.var_idse_dn11 = (assign106770_e158807 * locals.var_ids_dn11);
        locals.var_idse_dn14 = (assign106770_e158807 * locals.var_ids_dn14);
        locals.var_idse_rv = 0.0;

        let assign106930_e158857: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign106930_e158857;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn14 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign106940_e158860: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign106940_e158860;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p87 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn14 = (p.p87 * locals.var_cgdbd_dn14);
        locals.var_cgdbd_rv = 0.0;

        let assign106950_e158863: f64 = locals.var_qg_dn8;
        locals.var_cgsbd = assign106950_e158863;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn14 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign106960_e158866: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign106960_e158866;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p87 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn14 = (p.p87 * locals.var_cgsbd_dn14);
        locals.var_cgsbd_rv = 0.0;

        let assign107330_e158981: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2411 = assign107330_e158981;
        locals.var_guard2411_rv = 0.0;

        let (assign107360_e158993, assign107360_e158993_d_n0, assign107360_e158993_d_n2, assign107360_e158993_d_n4, assign107360_e158993_d_n5, assign107360_e158993_d_n6, assign107360_e158993_d_n7, assign107360_e158993_d_n8, assign107360_e158993_d_n9, assign107360_e158993_d_n10, assign107360_e158993_d_n11, assign107360_e158993_d_n14,) = {
    if (locals.var_guard2411 != 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14,)
    }
};
        locals.var_cgsb = assign107360_e158993;
        locals.var_cgsb_dn0 = assign107360_e158993_d_n0;
        locals.var_cgsb_dn2 = assign107360_e158993_d_n2;
        locals.var_cgsb_dn4 = assign107360_e158993_d_n4;
        locals.var_cgsb_dn5 = assign107360_e158993_d_n5;
        locals.var_cgsb_dn6 = assign107360_e158993_d_n6;
        locals.var_cgsb_dn7 = assign107360_e158993_d_n7;
        locals.var_cgsb_dn8 = assign107360_e158993_d_n8;
        locals.var_cgsb_dn9 = assign107360_e158993_d_n9;
        locals.var_cgsb_dn10 = assign107360_e158993_d_n10;
        locals.var_cgsb_dn11 = assign107360_e158993_d_n11;
        locals.var_cgsb_dn14 = assign107360_e158993_d_n14;
        locals.var_cgsb_rv = 0.0;

        let (assign107460_e159037, assign107460_e159037_d_n0, assign107460_e159037_d_n2, assign107460_e159037_d_n4, assign107460_e159037_d_n5, assign107460_e159037_d_n6, assign107460_e159037_d_n7, assign107460_e159037_d_n8, assign107460_e159037_d_n9, assign107460_e159037_d_n10, assign107460_e159037_d_n11, assign107460_e159037_d_n14,) = {
    if (locals.var_guard2411 == 0.0) {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    } else {
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14,)
    }
};
        locals.var_cgsb = assign107460_e159037;
        locals.var_cgsb_dn0 = assign107460_e159037_d_n0;
        locals.var_cgsb_dn2 = assign107460_e159037_d_n2;
        locals.var_cgsb_dn4 = assign107460_e159037_d_n4;
        locals.var_cgsb_dn5 = assign107460_e159037_d_n5;
        locals.var_cgsb_dn6 = assign107460_e159037_d_n6;
        locals.var_cgsb_dn7 = assign107460_e159037_d_n7;
        locals.var_cgsb_dn8 = assign107460_e159037_d_n8;
        locals.var_cgsb_dn9 = assign107460_e159037_d_n9;
        locals.var_cgsb_dn10 = assign107460_e159037_d_n10;
        locals.var_cgsb_dn11 = assign107460_e159037_d_n11;
        locals.var_cgsb_dn14 = assign107460_e159037_d_n14;
        locals.var_cgsb_rv = 0.0;

        let assign107690_e159100: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2413 = assign107690_e159100;
        locals.var_guard2413_rv = 0.0;

        let (assign107780_e159145,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (locals.var_cqi,)
    }
};
        locals.var_cqi = assign107780_e159145;
        locals.var_cqi_rv = 0.0;

        let (assign107790_e159149,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (locals.var_cqb,)
    }
};
        locals.var_cqb = assign107790_e159149;
        locals.var_cqb_rv = 0.0;

    }
}
