#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_cth: f64,
        var_deltemp1: f64,
        var_deltemp1_dn4: f64,
        var_deltemp1_dn5: f64,
        var_devsign: f64,
        var_gbody: f64,
        var_gbody_dn10: f64,
        var_gbody_dn11: f64,
        var_gbody_dn3: f64,
        var_gbody_dn4: f64,
        var_gbody_dn5: f64,
        var_gbody_dn6: f64,
        var_gbody_dn7: f64,
        var_gbody_dn8: f64,
        var_gbody_dn9: f64,
        var_gbodyagbcp2: f64,
        var_gbodyagbcp2_dn10: f64,
        var_gbodyagbcp2_dn11: f64,
        var_gbodyagbcp2_dn3: f64,
        var_gbodyagbcp2_dn4: f64,
        var_gbodyagbcp2_dn5: f64,
        var_gbodyagbcp2_dn6: f64,
        var_gbodyagbcp2_dn7: f64,
        var_gbodyagbcp2_dn8: f64,
        var_gbodyagbcp2_dn9: f64,
        var_gcrg: f64,
        var_gcrg_dn10: f64,
        var_gcrg_dn11: f64,
        var_gcrg_dn3: f64,
        var_gcrg_dn4: f64,
        var_gcrg_dn5: f64,
        var_gcrg_dn6: f64,
        var_gcrg_dn7: f64,
        var_gcrg_dn8: f64,
        var_gcrg_dn9: f64,
        var_gdpr: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_ggate: f64,
        var_ggate_dn10: f64,
        var_ggate_dn11: f64,
        var_ggate_dn3: f64,
        var_ggate_dn4: f64,
        var_ggate_dn5: f64,
        var_ggate_dn6: f64,
        var_ggate_dn7: f64,
        var_ggate_dn8: f64,
        var_ggate_dn9: f64,
        var_gmin: f64,
        var_gspr: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gth: f64,
        var_guard883: f64,
        var_guard884: f64,
        var_guard888: f64,
        var_guard890: f64,
        var_guard892: f64,
        var_guard893: f64,
        var_guard896: f64,
        var_guard897: f64,
        var_guard909: f64,
        var_guard910: f64,
        var_ibd: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn3: f64,
        var_ibd_dn4: f64,
        var_ibd_dn5: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibd_dn8: f64,
        var_ibd_dn9: f64,
        var_ibs: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn3: f64,
        var_ibs_dn4: f64,
        var_ibs_dn5: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ibs_dn8: f64,
        var_ibs_dn9: f64,
        var_igidl_1: f64,
        var_igidl_1_dn10: f64,
        var_igidl_1_dn11: f64,
        var_igidl_1_dn3: f64,
        var_igidl_1_dn4: f64,
        var_igidl_1_dn5: f64,
        var_igidl_1_dn6: f64,
        var_igidl_1_dn7: f64,
        var_igidl_1_dn8: f64,
        var_igidl_1_dn9: f64,
        var_igisl_1: f64,
        var_igisl_1_dn10: f64,
        var_igisl_1_dn11: f64,
        var_igisl_1_dn3: f64,
        var_igisl_1_dn4: f64,
        var_igisl_1_dn5: f64,
        var_igisl_1_dn6: f64,
        var_igisl_1_dn7: f64,
        var_igisl_1_dn8: f64,
        var_igisl_1_dn9: f64,
        var_isub: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_pdiss: f64,
        var_pdiss_dn0: f64,
        var_pdiss_dn10: f64,
        var_pdiss_dn11: f64,
        var_pdiss_dn2: f64,
        var_pdiss_dn3: f64,
        var_pdiss_dn4: f64,
        var_pdiss_dn5: f64,
        var_pdiss_dn6: f64,
        var_pdiss_dn7: f64,
        var_pdiss_dn8: f64,
        var_pdiss_dn9: f64,
        var_qbdj: f64,
        var_qbdj_dn10: f64,
        var_qbdj_dn11: f64,
        var_qbdj_dn3: f64,
        var_qbdj_dn4: f64,
        var_qbdj_dn5: f64,
        var_qbdj_dn6: f64,
        var_qbdj_dn7: f64,
        var_qbdj_dn8: f64,
        var_qbdj_dn9: f64,
        var_qbsj: f64,
        var_qbsj_dn10: f64,
        var_qbsj_dn11: f64,
        var_qbsj_dn3: f64,
        var_qbsj_dn4: f64,
        var_qbsj_dn5: f64,
        var_qbsj_dn6: f64,
        var_qbsj_dn7: f64,
        var_qbsj_dn8: f64,
        var_qbsj_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq54_e2010, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11,) = {
    if (var_guard883 == 0.0) {
        (var_igidl_1, var_igidl_1_dn3, var_igidl_1_dn4, var_igidl_1_dn5, var_igidl_1_dn6, var_igidl_1_dn7, var_igidl_1_dn8, var_igidl_1_dn9, var_igidl_1_dn10, var_igidl_1_dn11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2010;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(10),
            multiplicity * (eq54_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq54_e2010_d_n3), multiplicity * (eq54_e2010_d_n4), multiplicity * (eq54_e2010_d_n5), multiplicity * (eq54_e2010_d_n6), multiplicity * (eq54_e2010_d_n7), multiplicity * (eq54_e2010_d_n8), multiplicity * (eq54_e2010_d_n9), multiplicity * (eq54_e2010_d_n10), multiplicity * (eq54_e2010_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq55_e2017, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11,) = {
    if (var_guard883 == 0.0) {
        let eq55_e2015: f64 = (var_isub + var_igisl_1);
        let eq55_e2015_d_n3: f64 = (var_isub_dn3 + var_igisl_1_dn3);
        let eq55_e2015_d_n4: f64 = (var_isub_dn4 + var_igisl_1_dn4);
        let eq55_e2015_d_n5: f64 = (var_isub_dn5 + var_igisl_1_dn5);
        let eq55_e2015_d_n6: f64 = (var_isub_dn6 + var_igisl_1_dn6);
        let eq55_e2015_d_n7: f64 = (var_isub_dn7 + var_igisl_1_dn7);
        let eq55_e2015_d_n8: f64 = (var_isub_dn8 + var_igisl_1_dn8);
        let eq55_e2015_d_n9: f64 = (var_isub_dn9 + var_igisl_1_dn9);
        let eq55_e2015_d_n10: f64 = (var_isub_dn10 + var_igisl_1_dn10);
        let eq55_e2015_d_n11: f64 = (var_isub_dn11 + var_igisl_1_dn11);
        (eq55_e2015, eq55_e2015_d_n3, eq55_e2015_d_n4, eq55_e2015_d_n5, eq55_e2015_d_n6, eq55_e2015_d_n7, eq55_e2015_d_n8, eq55_e2015_d_n9, eq55_e2015_d_n10, eq55_e2015_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2017;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq55_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq55_e2017_d_n3), multiplicity * (eq55_e2017_d_n4), multiplicity * (eq55_e2017_d_n5), multiplicity * (eq55_e2017_d_n6), multiplicity * (eq55_e2017_d_n7), multiplicity * (eq55_e2017_d_n8), multiplicity * (eq55_e2017_d_n9), multiplicity * (eq55_e2017_d_n10), multiplicity * (eq55_e2017_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq57_e2028, eq57_e2028_d_n1, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11,) = {
    if (var_guard884 == 0.0) {
        let eq57_e2026: f64 = ((nv1 - nv9) * var_ggate);
        let eq57_e2026_d_n3: f64 = ((nv1 - nv9) * var_ggate_dn3);
        let eq57_e2026_d_n4: f64 = ((nv1 - nv9) * var_ggate_dn4);
        let eq57_e2026_d_n5: f64 = ((nv1 - nv9) * var_ggate_dn5);
        let eq57_e2026_d_n6: f64 = ((nv1 - nv9) * var_ggate_dn6);
        let eq57_e2026_d_n7: f64 = ((nv1 - nv9) * var_ggate_dn7);
        let eq57_e2026_d_n8: f64 = ((nv1 - nv9) * var_ggate_dn8);
        let eq57_e2026_d_n9: f64 = ((-var_ggate) + ((nv1 - nv9) * var_ggate_dn9));
        let eq57_e2026_d_n10: f64 = ((nv1 - nv9) * var_ggate_dn10);
        let eq57_e2026_d_n11: f64 = ((nv1 - nv9) * var_ggate_dn11);
        (eq57_e2026, var_ggate, eq57_e2026_d_n3, eq57_e2026_d_n4, eq57_e2026_d_n5, eq57_e2026_d_n6, eq57_e2026_d_n7, eq57_e2026_d_n8, eq57_e2026_d_n9, eq57_e2026_d_n10, eq57_e2026_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2028;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq57_value),
            [1, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq57_e2028_d_n1), multiplicity * (eq57_e2028_d_n3), multiplicity * (eq57_e2028_d_n4), multiplicity * (eq57_e2028_d_n5), multiplicity * (eq57_e2028_d_n6), multiplicity * (eq57_e2028_d_n7), multiplicity * (eq57_e2028_d_n8), multiplicity * (eq57_e2028_d_n9), multiplicity * (eq57_e2028_d_n10), multiplicity * (eq57_e2028_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq59_e2043, eq59_e2043_d_n0, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11,) = {
    if (var_guard888 != 0.0) {
        let eq59_e2041: f64 = ((nv0 - nv6) * var_gdpr);
        let eq59_e2041_d_n3: f64 = ((nv0 - nv6) * var_gdpr_dn3);
        let eq59_e2041_d_n4: f64 = ((nv0 - nv6) * var_gdpr_dn4);
        let eq59_e2041_d_n5: f64 = ((nv0 - nv6) * var_gdpr_dn5);
        let eq59_e2041_d_n6: f64 = ((-var_gdpr) + ((nv0 - nv6) * var_gdpr_dn6));
        let eq59_e2041_d_n7: f64 = ((nv0 - nv6) * var_gdpr_dn7);
        let eq59_e2041_d_n8: f64 = ((nv0 - nv6) * var_gdpr_dn8);
        let eq59_e2041_d_n9: f64 = ((nv0 - nv6) * var_gdpr_dn9);
        let eq59_e2041_d_n10: f64 = ((nv0 - nv6) * var_gdpr_dn10);
        let eq59_e2041_d_n11: f64 = ((nv0 - nv6) * var_gdpr_dn11);
        (eq59_e2041, var_gdpr, eq59_e2041_d_n3, eq59_e2041_d_n4, eq59_e2041_d_n5, eq59_e2041_d_n6, eq59_e2041_d_n7, eq59_e2041_d_n8, eq59_e2041_d_n9, eq59_e2041_d_n10, eq59_e2041_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2043;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq59_value),
            [0, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq59_e2043_d_n0), multiplicity * (eq59_e2043_d_n3), multiplicity * (eq59_e2043_d_n4), multiplicity * (eq59_e2043_d_n5), multiplicity * (eq59_e2043_d_n6), multiplicity * (eq59_e2043_d_n7), multiplicity * (eq59_e2043_d_n8), multiplicity * (eq59_e2043_d_n9), multiplicity * (eq59_e2043_d_n10), multiplicity * (eq59_e2043_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq60_e2048,) = {
    if (var_guard888 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2048;
        stamper.stamp_potential_const_local(
            3,
            eq60_value,
        );
        let (eq62_e2062, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11,) = {
    if (var_guard890 != 0.0) {
        let eq62_e2060: f64 = ((nv2 - nv7) * var_gspr);
        let eq62_e2060_d_n3: f64 = ((nv2 - nv7) * var_gspr_dn3);
        let eq62_e2060_d_n4: f64 = ((nv2 - nv7) * var_gspr_dn4);
        let eq62_e2060_d_n5: f64 = ((nv2 - nv7) * var_gspr_dn5);
        let eq62_e2060_d_n6: f64 = ((nv2 - nv7) * var_gspr_dn6);
        let eq62_e2060_d_n7: f64 = ((-var_gspr) + ((nv2 - nv7) * var_gspr_dn7));
        let eq62_e2060_d_n8: f64 = ((nv2 - nv7) * var_gspr_dn8);
        let eq62_e2060_d_n9: f64 = ((nv2 - nv7) * var_gspr_dn9);
        let eq62_e2060_d_n10: f64 = ((nv2 - nv7) * var_gspr_dn10);
        let eq62_e2060_d_n11: f64 = ((nv2 - nv7) * var_gspr_dn11);
        (eq62_e2060, var_gspr, eq62_e2060_d_n3, eq62_e2060_d_n4, eq62_e2060_d_n5, eq62_e2060_d_n6, eq62_e2060_d_n7, eq62_e2060_d_n8, eq62_e2060_d_n9, eq62_e2060_d_n10, eq62_e2060_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2062;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(2),
            Some(7),
            multiplicity * (eq62_value),
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq62_e2062_d_n2), multiplicity * (eq62_e2062_d_n3), multiplicity * (eq62_e2062_d_n4), multiplicity * (eq62_e2062_d_n5), multiplicity * (eq62_e2062_d_n6), multiplicity * (eq62_e2062_d_n7), multiplicity * (eq62_e2062_d_n8), multiplicity * (eq62_e2062_d_n9), multiplicity * (eq62_e2062_d_n10), multiplicity * (eq62_e2062_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq63_e2067,) = {
    if (var_guard890 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2067;
        stamper.stamp_potential_const_local(
            4,
            eq63_value,
        );
        let (eq65_e2081, eq65_e2081_d_n3, eq65_e2081_d_n4, eq65_e2081_d_n5, eq65_e2081_d_n6, eq65_e2081_d_n7, eq65_e2081_d_n8, eq65_e2081_d_n9, eq65_e2081_d_n10, eq65_e2081_d_n11,) = {
    if (var_guard892 != 0.0) {
        let eq65_e2079: f64 = ((nv9 - nv8) * var_gcrg);
        let eq65_e2079_d_n3: f64 = ((nv9 - nv8) * var_gcrg_dn3);
        let eq65_e2079_d_n4: f64 = ((nv9 - nv8) * var_gcrg_dn4);
        let eq65_e2079_d_n5: f64 = ((nv9 - nv8) * var_gcrg_dn5);
        let eq65_e2079_d_n6: f64 = ((nv9 - nv8) * var_gcrg_dn6);
        let eq65_e2079_d_n7: f64 = ((nv9 - nv8) * var_gcrg_dn7);
        let eq65_e2079_d_n8: f64 = ((-var_gcrg) + ((nv9 - nv8) * var_gcrg_dn8));
        let eq65_e2079_d_n9: f64 = (var_gcrg + ((nv9 - nv8) * var_gcrg_dn9));
        let eq65_e2079_d_n10: f64 = ((nv9 - nv8) * var_gcrg_dn10);
        let eq65_e2079_d_n11: f64 = ((nv9 - nv8) * var_gcrg_dn11);
        (eq65_e2079, eq65_e2079_d_n3, eq65_e2079_d_n4, eq65_e2079_d_n5, eq65_e2079_d_n6, eq65_e2079_d_n7, eq65_e2079_d_n8, eq65_e2079_d_n9, eq65_e2079_d_n10, eq65_e2079_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e2081;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq65_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq65_e2081_d_n3), multiplicity * (eq65_e2081_d_n4), multiplicity * (eq65_e2081_d_n5), multiplicity * (eq65_e2081_d_n6), multiplicity * (eq65_e2081_d_n7), multiplicity * (eq65_e2081_d_n8), multiplicity * (eq65_e2081_d_n9), multiplicity * (eq65_e2081_d_n10), multiplicity * (eq65_e2081_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11,) = {
    if (((var_guard893 != 0.0) && (var_guard896 != 0.0)) && (var_guard897 != 0.0)) {
        let eq67_e2094: f64 = (var_deltemp1 * var_gth);
        let eq67_e2094_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq67_e2094_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq67_e2097: f64 = (var_deltemp1 * var_cth);
        let eq67_e2097_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq67_e2097_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq67_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq67_e2097);
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2098);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + (eq67_e2097_d_n4 * ddt_scale));
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + (eq67_e2097_d_n5 * ddt_scale));
        let eq67_e2101: f64 = (eq67_e2099 - var_pdiss);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - var_pdiss_dn4);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - var_pdiss_dn5);
        (eq67_e2101, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq67_e2101_d_n4, eq67_e2101_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2103;
        let eq67_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq67_node_derivatives: [f64; 11] = [eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11,) = {
    if (((var_guard893 != 0.0) && (var_guard896 != 0.0)) && (var_guard897 == 0.0)) {
        let eq68_e2112: f64 = (var_deltemp1 * var_gth);
        let eq68_e2112_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq68_e2112_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq68_e2115: f64 = (var_deltemp1 * var_cth);
        let eq68_e2115_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq68_e2115_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq68_e2116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq68_e2115);
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2116);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + (eq68_e2115_d_n4 * ddt_scale));
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + (eq68_e2115_d_n5 * ddt_scale));
        let eq68_e2119: f64 = (eq68_e2117 - var_pdiss);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - var_pdiss_dn4);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - var_pdiss_dn5);
        (eq68_e2119, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq68_e2119_d_n4, eq68_e2119_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2121;
        let eq68_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq68_node_derivatives: [f64; 11] = [eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11];
        let eq68_branch_derivative_indices: [usize; 0] = [];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq68_value),
            &eq68_node_derivative_indices,
            &eq68_node_derivatives,
            &eq68_branch_derivative_indices,
            &eq68_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11,) = {
    if ((var_guard893 != 0.0) && (var_guard896 == 0.0)) {
        let eq69_e2128: f64 = (var_deltemp1 * var_gth);
        let eq69_e2128_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq69_e2128_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq69_e2131: f64 = (var_deltemp1 * var_cth);
        let eq69_e2131_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq69_e2131_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq69_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq69_e2131);
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2132);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + (eq69_e2131_d_n4 * ddt_scale));
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + (eq69_e2131_d_n5 * ddt_scale));
        let eq69_e2135: f64 = (eq69_e2133 - var_pdiss);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - var_pdiss_dn4);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - var_pdiss_dn5);
        (eq69_e2135, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq69_e2135_d_n4, eq69_e2135_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2137;
        let eq69_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq69_node_derivatives: [f64; 11] = [eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11];
        let eq69_branch_derivative_indices: [usize; 0] = [];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivative_indices,
            &eq69_node_derivatives,
            &eq69_branch_derivative_indices,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq76_e2187, eq76_e2187_d_n3, eq76_e2187_d_n4, eq76_e2187_d_n5, eq76_e2187_d_n6, eq76_e2187_d_n7, eq76_e2187_d_n8, eq76_e2187_d_n9, eq76_e2187_d_n10, eq76_e2187_d_n11,) = {
    if ((var_guard909 != 0.0) && (var_guard910 != 0.0)) {
        let eq76_e2185: f64 = ((nv4 - nv10) * var_gbody);
        let eq76_e2185_d_n3: f64 = ((nv4 - nv10) * var_gbody_dn3);
        let eq76_e2185_d_n4: f64 = (var_gbody + ((nv4 - nv10) * var_gbody_dn4));
        let eq76_e2185_d_n5: f64 = ((nv4 - nv10) * var_gbody_dn5);
        let eq76_e2185_d_n6: f64 = ((nv4 - nv10) * var_gbody_dn6);
        let eq76_e2185_d_n7: f64 = ((nv4 - nv10) * var_gbody_dn7);
        let eq76_e2185_d_n8: f64 = ((nv4 - nv10) * var_gbody_dn8);
        let eq76_e2185_d_n9: f64 = ((nv4 - nv10) * var_gbody_dn9);
        let eq76_e2185_d_n10: f64 = ((-var_gbody) + ((nv4 - nv10) * var_gbody_dn10));
        let eq76_e2185_d_n11: f64 = ((nv4 - nv10) * var_gbody_dn11);
        (eq76_e2185, eq76_e2185_d_n3, eq76_e2185_d_n4, eq76_e2185_d_n5, eq76_e2185_d_n6, eq76_e2185_d_n7, eq76_e2185_d_n8, eq76_e2185_d_n9, eq76_e2185_d_n10, eq76_e2185_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2187;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(10),
            multiplicity * (eq76_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq76_e2187_d_n3), multiplicity * (eq76_e2187_d_n4), multiplicity * (eq76_e2187_d_n5), multiplicity * (eq76_e2187_d_n6), multiplicity * (eq76_e2187_d_n7), multiplicity * (eq76_e2187_d_n8), multiplicity * (eq76_e2187_d_n9), multiplicity * (eq76_e2187_d_n10), multiplicity * (eq76_e2187_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq77_e2195, eq77_e2195_d_n3, eq77_e2195_d_n4, eq77_e2195_d_n5, eq77_e2195_d_n6, eq77_e2195_d_n7, eq77_e2195_d_n8, eq77_e2195_d_n9, eq77_e2195_d_n10, eq77_e2195_d_n11,) = {
    if ((var_guard909 != 0.0) && (var_guard910 != 0.0)) {
        let eq77_e2193: f64 = ((nv4 - nv11) * var_gbodyagbcp2);
        let eq77_e2193_d_n3: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn3);
        let eq77_e2193_d_n4: f64 = (var_gbodyagbcp2 + ((nv4 - nv11) * var_gbodyagbcp2_dn4));
        let eq77_e2193_d_n5: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn5);
        let eq77_e2193_d_n6: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn6);
        let eq77_e2193_d_n7: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn7);
        let eq77_e2193_d_n8: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn8);
        let eq77_e2193_d_n9: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn9);
        let eq77_e2193_d_n10: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn10);
        let eq77_e2193_d_n11: f64 = ((-var_gbodyagbcp2) + ((nv4 - nv11) * var_gbodyagbcp2_dn11));
        (eq77_e2193, eq77_e2193_d_n3, eq77_e2193_d_n4, eq77_e2193_d_n5, eq77_e2193_d_n6, eq77_e2193_d_n7, eq77_e2193_d_n8, eq77_e2193_d_n9, eq77_e2193_d_n10, eq77_e2193_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e2195;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(11),
            multiplicity * (eq77_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq77_e2195_d_n3), multiplicity * (eq77_e2195_d_n4), multiplicity * (eq77_e2195_d_n5), multiplicity * (eq77_e2195_d_n6), multiplicity * (eq77_e2195_d_n7), multiplicity * (eq77_e2195_d_n8), multiplicity * (eq77_e2195_d_n9), multiplicity * (eq77_e2195_d_n10), multiplicity * (eq77_e2195_d_n11)],
            [],
            [],
            1.0,
        );
        let eq78_e2198: f64 = (var_devsign * var_ibs);
        let eq78_e2198_d_n3: f64 = (var_devsign * var_ibs_dn3);
        let eq78_e2198_d_n4: f64 = (var_devsign * var_ibs_dn4);
        let eq78_e2198_d_n5: f64 = (var_devsign * var_ibs_dn5);
        let eq78_e2198_d_n6: f64 = (var_devsign * var_ibs_dn6);
        let eq78_e2198_d_n7: f64 = (var_devsign * var_ibs_dn7);
        let eq78_e2198_d_n8: f64 = (var_devsign * var_ibs_dn8);
        let eq78_e2198_d_n9: f64 = (var_devsign * var_ibs_dn9);
        let eq78_e2198_d_n10: f64 = (var_devsign * var_ibs_dn10);
        let eq78_e2198_d_n11: f64 = (var_devsign * var_ibs_dn11);
        let eq78_e2201: f64 = ((nv10 - nv7) * var_gmin);
        let eq78_e2202: f64 = (eq78_e2198 + eq78_e2201);
        let eq78_e2202_d_n7: f64 = (eq78_e2198_d_n7 + (-var_gmin));
        let eq78_e2202_d_n10: f64 = (eq78_e2198_d_n10 + var_gmin);
        let eq78_value: f64 = eq78_e2202;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq78_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq78_e2198_d_n3), multiplicity * (eq78_e2198_d_n4), multiplicity * (eq78_e2198_d_n5), multiplicity * (eq78_e2198_d_n6), multiplicity * (eq78_e2202_d_n7), multiplicity * (eq78_e2198_d_n8), multiplicity * (eq78_e2198_d_n9), multiplicity * (eq78_e2202_d_n10), multiplicity * (eq78_e2198_d_n11)],
            [],
            [],
            1.0,
        );
        let eq79_e2205: f64 = (var_devsign * var_ibd);
        let eq79_e2205_d_n3: f64 = (var_devsign * var_ibd_dn3);
        let eq79_e2205_d_n4: f64 = (var_devsign * var_ibd_dn4);
        let eq79_e2205_d_n5: f64 = (var_devsign * var_ibd_dn5);
        let eq79_e2205_d_n6: f64 = (var_devsign * var_ibd_dn6);
        let eq79_e2205_d_n7: f64 = (var_devsign * var_ibd_dn7);
        let eq79_e2205_d_n8: f64 = (var_devsign * var_ibd_dn8);
        let eq79_e2205_d_n9: f64 = (var_devsign * var_ibd_dn9);
        let eq79_e2205_d_n10: f64 = (var_devsign * var_ibd_dn10);
        let eq79_e2205_d_n11: f64 = (var_devsign * var_ibd_dn11);
        let eq79_e2208: f64 = ((nv10 - nv6) * var_gmin);
        let eq79_e2209: f64 = (eq79_e2205 + eq79_e2208);
        let eq79_e2209_d_n6: f64 = (eq79_e2205_d_n6 + (-var_gmin));
        let eq79_e2209_d_n10: f64 = (eq79_e2205_d_n10 + var_gmin);
        let eq79_value: f64 = eq79_e2209;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq79_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq79_e2205_d_n3), multiplicity * (eq79_e2205_d_n4), multiplicity * (eq79_e2205_d_n5), multiplicity * (eq79_e2209_d_n6), multiplicity * (eq79_e2205_d_n7), multiplicity * (eq79_e2205_d_n8), multiplicity * (eq79_e2205_d_n9), multiplicity * (eq79_e2209_d_n10), multiplicity * (eq79_e2205_d_n11)],
            [],
            [],
            1.0,
        );
        let eq80_e2212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, var_qbsj);
        let eq80_e2213: f64 = (var_devsign * eq80_e2212);
        let eq80_e2213_d_n3: f64 = (var_devsign * (var_qbsj_dn3 * ddt_scale));
        let eq80_e2213_d_n4: f64 = (var_devsign * (var_qbsj_dn4 * ddt_scale));
        let eq80_e2213_d_n5: f64 = (var_devsign * (var_qbsj_dn5 * ddt_scale));
        let eq80_e2213_d_n6: f64 = (var_devsign * (var_qbsj_dn6 * ddt_scale));
        let eq80_e2213_d_n7: f64 = (var_devsign * (var_qbsj_dn7 * ddt_scale));
        let eq80_e2213_d_n8: f64 = (var_devsign * (var_qbsj_dn8 * ddt_scale));
        let eq80_e2213_d_n9: f64 = (var_devsign * (var_qbsj_dn9 * ddt_scale));
        let eq80_e2213_d_n10: f64 = (var_devsign * (var_qbsj_dn10 * ddt_scale));
        let eq80_e2213_d_n11: f64 = (var_devsign * (var_qbsj_dn11 * ddt_scale));
        let eq80_value: f64 = eq80_e2213;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq80_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq80_e2213_d_n3), multiplicity * (eq80_e2213_d_n4), multiplicity * (eq80_e2213_d_n5), multiplicity * (eq80_e2213_d_n6), multiplicity * (eq80_e2213_d_n7), multiplicity * (eq80_e2213_d_n8), multiplicity * (eq80_e2213_d_n9), multiplicity * (eq80_e2213_d_n10), multiplicity * (eq80_e2213_d_n11)],
            [],
            [],
            1.0,
        );
        let eq81_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, var_qbdj);
        let eq81_e2217: f64 = (var_devsign * eq81_e2216);
        let eq81_e2217_d_n3: f64 = (var_devsign * (var_qbdj_dn3 * ddt_scale));
        let eq81_e2217_d_n4: f64 = (var_devsign * (var_qbdj_dn4 * ddt_scale));
        let eq81_e2217_d_n5: f64 = (var_devsign * (var_qbdj_dn5 * ddt_scale));
        let eq81_e2217_d_n6: f64 = (var_devsign * (var_qbdj_dn6 * ddt_scale));
        let eq81_e2217_d_n7: f64 = (var_devsign * (var_qbdj_dn7 * ddt_scale));
        let eq81_e2217_d_n8: f64 = (var_devsign * (var_qbdj_dn8 * ddt_scale));
        let eq81_e2217_d_n9: f64 = (var_devsign * (var_qbdj_dn9 * ddt_scale));
        let eq81_e2217_d_n10: f64 = (var_devsign * (var_qbdj_dn10 * ddt_scale));
        let eq81_e2217_d_n11: f64 = (var_devsign * (var_qbdj_dn11 * ddt_scale));
        let eq81_value: f64 = eq81_e2217;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq81_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq81_e2217_d_n3), multiplicity * (eq81_e2217_d_n4), multiplicity * (eq81_e2217_d_n5), multiplicity * (eq81_e2217_d_n6), multiplicity * (eq81_e2217_d_n7), multiplicity * (eq81_e2217_d_n8), multiplicity * (eq81_e2217_d_n9), multiplicity * (eq81_e2217_d_n10), multiplicity * (eq81_e2217_d_n11)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1546, eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13, eq7_e1546_d_b0, eq7_e1546_d_b1, eq7_e1546_d_b2, eq7_e1546_d_b3, eq7_e1546_d_b4, eq7_e1546_d_b5, eq7_e1546_d_b6, eq7_e1546_d_b7, eq7_e1546_d_b8, eq7_e1546_d_b9, eq7_e1546_d_b10, eq7_e1546_d_b11, eq7_e1546_q,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq7_e1535: f64 = (s.v[622] * s.v[199]);
        let eq7_e1537: f64 = (eq7_e1535 * s.v[183]);
        let eq7_e1537_d_n0: f64 = ((s.dn[622][0] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n1: f64 = ((s.dn[622][1] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n2: f64 = ((s.dn[622][2] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n3: f64 = ((s.dn[622][3] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n4: f64 = ((s.dn[622][4] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n5: f64 = ((s.dn[622][5] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n6: f64 = ((s.dn[622][6] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n7: f64 = ((s.dn[622][7] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n8: f64 = ((s.dn[622][8] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n9: f64 = ((s.dn[622][9] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n10: f64 = ((s.dn[622][10] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n11: f64 = ((s.dn[622][11] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n12: f64 = ((s.dn[622][12] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_n13: f64 = ((s.dn[622][13] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b0: f64 = ((s.db[622][0] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b1: f64 = ((s.db[622][1] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b2: f64 = ((s.db[622][2] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b3: f64 = ((s.db[622][3] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b4: f64 = ((s.db[622][4] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b5: f64 = ((s.db[622][5] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b6: f64 = ((s.db[622][6] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b7: f64 = ((s.db[622][7] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b8: f64 = ((s.db[622][8] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b9: f64 = ((s.db[622][9] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b10: f64 = ((s.db[622][10] * s.v[199]) * s.v[183]);
        let eq7_e1537_d_b11: f64 = ((s.db[622][11] * s.v[199]) * s.v[183]);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n0: f64 = (eq7_e1537_d_n0 * p.p2);
        let eq7_e1539_d_n1: f64 = (eq7_e1537_d_n1 * p.p2);
        let eq7_e1539_d_n2: f64 = (eq7_e1537_d_n2 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1539_d_n12: f64 = (eq7_e1537_d_n12 * p.p2);
        let eq7_e1539_d_n13: f64 = (eq7_e1537_d_n13 * p.p2);
        let eq7_e1539_d_b0: f64 = (eq7_e1537_d_b0 * p.p2);
        let eq7_e1539_d_b1: f64 = (eq7_e1537_d_b1 * p.p2);
        let eq7_e1539_d_b2: f64 = (eq7_e1537_d_b2 * p.p2);
        let eq7_e1539_d_b3: f64 = (eq7_e1537_d_b3 * p.p2);
        let eq7_e1539_d_b4: f64 = (eq7_e1537_d_b4 * p.p2);
        let eq7_e1539_d_b5: f64 = (eq7_e1537_d_b5 * p.p2);
        let eq7_e1539_d_b6: f64 = (eq7_e1537_d_b6 * p.p2);
        let eq7_e1539_d_b7: f64 = (eq7_e1537_d_b7 * p.p2);
        let eq7_e1539_d_b8: f64 = (eq7_e1537_d_b8 * p.p2);
        let eq7_e1539_d_b9: f64 = (eq7_e1537_d_b9 * p.p2);
        let eq7_e1539_d_b10: f64 = (eq7_e1537_d_b10 * p.p2);
        let eq7_e1539_d_b11: f64 = (eq7_e1537_d_b11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * s.v[184]);
        let eq7_e1541_d_n0: f64 = (eq7_e1539_d_n0 * s.v[184]);
        let eq7_e1541_d_n1: f64 = (eq7_e1539_d_n1 * s.v[184]);
        let eq7_e1541_d_n2: f64 = (eq7_e1539_d_n2 * s.v[184]);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * s.v[184]);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * s.v[184]);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * s.v[184]);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * s.v[184]);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * s.v[184]);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * s.v[184]);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * s.v[184]);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * s.v[184]);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * s.v[184]);
        let eq7_e1541_d_n12: f64 = (eq7_e1539_d_n12 * s.v[184]);
        let eq7_e1541_d_n13: f64 = (eq7_e1539_d_n13 * s.v[184]);
        let eq7_e1541_d_b0: f64 = (eq7_e1539_d_b0 * s.v[184]);
        let eq7_e1541_d_b1: f64 = (eq7_e1539_d_b1 * s.v[184]);
        let eq7_e1541_d_b2: f64 = (eq7_e1539_d_b2 * s.v[184]);
        let eq7_e1541_d_b3: f64 = (eq7_e1539_d_b3 * s.v[184]);
        let eq7_e1541_d_b4: f64 = (eq7_e1539_d_b4 * s.v[184]);
        let eq7_e1541_d_b5: f64 = (eq7_e1539_d_b5 * s.v[184]);
        let eq7_e1541_d_b6: f64 = (eq7_e1539_d_b6 * s.v[184]);
        let eq7_e1541_d_b7: f64 = (eq7_e1539_d_b7 * s.v[184]);
        let eq7_e1541_d_b8: f64 = (eq7_e1539_d_b8 * s.v[184]);
        let eq7_e1541_d_b9: f64 = (eq7_e1539_d_b9 * s.v[184]);
        let eq7_e1541_d_b10: f64 = (eq7_e1539_d_b10 * s.v[184]);
        let eq7_e1541_d_b11: f64 = (eq7_e1539_d_b11 * s.v[184]);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n0: f64 = (eq7_e1541_d_n0 * (nv12 - 0.0));
        let eq7_e1543_d_n1: f64 = (eq7_e1541_d_n1 * (nv12 - 0.0));
        let eq7_e1543_d_n2: f64 = (eq7_e1541_d_n2 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1543_d_n12: f64 = ((eq7_e1541_d_n12 * (nv12 - 0.0)) + eq7_e1541);
        let eq7_e1543_d_n13: f64 = (eq7_e1541_d_n13 * (nv12 - 0.0));
        let eq7_e1543_d_b0: f64 = (eq7_e1541_d_b0 * (nv12 - 0.0));
        let eq7_e1543_d_b1: f64 = (eq7_e1541_d_b1 * (nv12 - 0.0));
        let eq7_e1543_d_b2: f64 = (eq7_e1541_d_b2 * (nv12 - 0.0));
        let eq7_e1543_d_b3: f64 = (eq7_e1541_d_b3 * (nv12 - 0.0));
        let eq7_e1543_d_b4: f64 = (eq7_e1541_d_b4 * (nv12 - 0.0));
        let eq7_e1543_d_b5: f64 = (eq7_e1541_d_b5 * (nv12 - 0.0));
        let eq7_e1543_d_b6: f64 = (eq7_e1541_d_b6 * (nv12 - 0.0));
        let eq7_e1543_d_b7: f64 = (eq7_e1541_d_b7 * (nv12 - 0.0));
        let eq7_e1543_d_b8: f64 = (eq7_e1541_d_b8 * (nv12 - 0.0));
        let eq7_e1543_d_b9: f64 = (eq7_e1541_d_b9 * (nv12 - 0.0));
        let eq7_e1543_d_b10: f64 = (eq7_e1541_d_b10 * (nv12 - 0.0));
        let eq7_e1543_d_b11: f64 = (eq7_e1541_d_b11 * (nv12 - 0.0));
        let eq7_e1544_q: f64 = eq7_e1543;
        (eq7_e1543, eq7_e1543_d_n0, eq7_e1543_d_n1, eq7_e1543_d_n2, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1543_d_n12, eq7_e1543_d_n13, eq7_e1543_d_b0, eq7_e1543_d_b1, eq7_e1543_d_b2, eq7_e1543_d_b3, eq7_e1543_d_b4, eq7_e1543_d_b5, eq7_e1543_d_b6, eq7_e1543_d_b7, eq7_e1543_d_b8, eq7_e1543_d_b9, eq7_e1543_d_b10, eq7_e1543_d_b11, eq7_e1544_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 14] = [eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13];
        let eq7_reactive_branch_derivatives: [f64; 12] = [eq7_e1546_d_b0, eq7_e1546_d_b1, eq7_e1546_d_b2, eq7_e1546_d_b3, eq7_e1546_d_b4, eq7_e1546_d_b5, eq7_e1546_d_b6, eq7_e1546_d_b7, eq7_e1546_d_b8, eq7_e1546_d_b9, eq7_e1546_d_b10, eq7_e1546_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1600, eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13, eq10_e1600_d_b0, eq10_e1600_d_b1, eq10_e1600_d_b2, eq10_e1600_d_b3, eq10_e1600_d_b4, eq10_e1600_d_b5, eq10_e1600_d_b6, eq10_e1600_d_b7, eq10_e1600_d_b8, eq10_e1600_d_b9, eq10_e1600_d_b10, eq10_e1600_d_b11, eq10_e1600_q,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq10_e1584: f64 = (1.0 + s.v[211]);
        let eq10_e1586: f64 = (eq10_e1584 * s.v[622]);
        let eq10_e1586_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq10_e1584 * s.dn[622][0]));
        let eq10_e1586_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq10_e1584 * s.dn[622][1]));
        let eq10_e1586_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq10_e1584 * s.dn[622][2]));
        let eq10_e1586_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq10_e1584 * s.dn[622][3]));
        let eq10_e1586_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq10_e1584 * s.dn[622][4]));
        let eq10_e1586_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq10_e1584 * s.dn[622][5]));
        let eq10_e1586_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq10_e1584 * s.dn[622][6]));
        let eq10_e1586_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq10_e1584 * s.dn[622][7]));
        let eq10_e1586_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq10_e1584 * s.dn[622][8]));
        let eq10_e1586_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq10_e1584 * s.dn[622][9]));
        let eq10_e1586_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq10_e1584 * s.dn[622][10]));
        let eq10_e1586_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq10_e1584 * s.dn[622][11]));
        let eq10_e1586_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq10_e1584 * s.dn[622][12]));
        let eq10_e1586_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq10_e1584 * s.dn[622][13]));
        let eq10_e1586_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq10_e1584 * s.db[622][0]));
        let eq10_e1586_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq10_e1584 * s.db[622][1]));
        let eq10_e1586_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq10_e1584 * s.db[622][2]));
        let eq10_e1586_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq10_e1584 * s.db[622][3]));
        let eq10_e1586_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq10_e1584 * s.db[622][4]));
        let eq10_e1586_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq10_e1584 * s.db[622][5]));
        let eq10_e1586_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq10_e1584 * s.db[622][6]));
        let eq10_e1586_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq10_e1584 * s.db[622][7]));
        let eq10_e1586_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq10_e1584 * s.db[622][8]));
        let eq10_e1586_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq10_e1584 * s.db[622][9]));
        let eq10_e1586_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq10_e1584 * s.db[622][10]));
        let eq10_e1586_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq10_e1584 * s.db[622][11]));
        let eq10_e1588: f64 = (eq10_e1586 * s.v[199]);
        let eq10_e1588_d_n0: f64 = (eq10_e1586_d_n0 * s.v[199]);
        let eq10_e1588_d_n1: f64 = (eq10_e1586_d_n1 * s.v[199]);
        let eq10_e1588_d_n2: f64 = (eq10_e1586_d_n2 * s.v[199]);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * s.v[199]);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * s.v[199]);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * s.v[199]);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * s.v[199]);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * s.v[199]);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * s.v[199]);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * s.v[199]);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * s.v[199]);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * s.v[199]);
        let eq10_e1588_d_n12: f64 = (eq10_e1586_d_n12 * s.v[199]);
        let eq10_e1588_d_n13: f64 = (eq10_e1586_d_n13 * s.v[199]);
        let eq10_e1588_d_b0: f64 = (eq10_e1586_d_b0 * s.v[199]);
        let eq10_e1588_d_b1: f64 = (eq10_e1586_d_b1 * s.v[199]);
        let eq10_e1588_d_b2: f64 = (eq10_e1586_d_b2 * s.v[199]);
        let eq10_e1588_d_b3: f64 = (eq10_e1586_d_b3 * s.v[199]);
        let eq10_e1588_d_b4: f64 = (eq10_e1586_d_b4 * s.v[199]);
        let eq10_e1588_d_b5: f64 = (eq10_e1586_d_b5 * s.v[199]);
        let eq10_e1588_d_b6: f64 = (eq10_e1586_d_b6 * s.v[199]);
        let eq10_e1588_d_b7: f64 = (eq10_e1586_d_b7 * s.v[199]);
        let eq10_e1588_d_b8: f64 = (eq10_e1586_d_b8 * s.v[199]);
        let eq10_e1588_d_b9: f64 = (eq10_e1586_d_b9 * s.v[199]);
        let eq10_e1588_d_b10: f64 = (eq10_e1586_d_b10 * s.v[199]);
        let eq10_e1588_d_b11: f64 = (eq10_e1586_d_b11 * s.v[199]);
        let eq10_e1590: f64 = (eq10_e1588 * s.v[183]);
        let eq10_e1590_d_n0: f64 = (eq10_e1588_d_n0 * s.v[183]);
        let eq10_e1590_d_n1: f64 = (eq10_e1588_d_n1 * s.v[183]);
        let eq10_e1590_d_n2: f64 = (eq10_e1588_d_n2 * s.v[183]);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * s.v[183]);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * s.v[183]);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * s.v[183]);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * s.v[183]);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * s.v[183]);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * s.v[183]);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * s.v[183]);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * s.v[183]);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * s.v[183]);
        let eq10_e1590_d_n12: f64 = (eq10_e1588_d_n12 * s.v[183]);
        let eq10_e1590_d_n13: f64 = (eq10_e1588_d_n13 * s.v[183]);
        let eq10_e1590_d_b0: f64 = (eq10_e1588_d_b0 * s.v[183]);
        let eq10_e1590_d_b1: f64 = (eq10_e1588_d_b1 * s.v[183]);
        let eq10_e1590_d_b2: f64 = (eq10_e1588_d_b2 * s.v[183]);
        let eq10_e1590_d_b3: f64 = (eq10_e1588_d_b3 * s.v[183]);
        let eq10_e1590_d_b4: f64 = (eq10_e1588_d_b4 * s.v[183]);
        let eq10_e1590_d_b5: f64 = (eq10_e1588_d_b5 * s.v[183]);
        let eq10_e1590_d_b6: f64 = (eq10_e1588_d_b6 * s.v[183]);
        let eq10_e1590_d_b7: f64 = (eq10_e1588_d_b7 * s.v[183]);
        let eq10_e1590_d_b8: f64 = (eq10_e1588_d_b8 * s.v[183]);
        let eq10_e1590_d_b9: f64 = (eq10_e1588_d_b9 * s.v[183]);
        let eq10_e1590_d_b10: f64 = (eq10_e1588_d_b10 * s.v[183]);
        let eq10_e1590_d_b11: f64 = (eq10_e1588_d_b11 * s.v[183]);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n0: f64 = (eq10_e1590_d_n0 * p.p2);
        let eq10_e1592_d_n1: f64 = (eq10_e1590_d_n1 * p.p2);
        let eq10_e1592_d_n2: f64 = (eq10_e1590_d_n2 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1592_d_n12: f64 = (eq10_e1590_d_n12 * p.p2);
        let eq10_e1592_d_n13: f64 = (eq10_e1590_d_n13 * p.p2);
        let eq10_e1592_d_b0: f64 = (eq10_e1590_d_b0 * p.p2);
        let eq10_e1592_d_b1: f64 = (eq10_e1590_d_b1 * p.p2);
        let eq10_e1592_d_b2: f64 = (eq10_e1590_d_b2 * p.p2);
        let eq10_e1592_d_b3: f64 = (eq10_e1590_d_b3 * p.p2);
        let eq10_e1592_d_b4: f64 = (eq10_e1590_d_b4 * p.p2);
        let eq10_e1592_d_b5: f64 = (eq10_e1590_d_b5 * p.p2);
        let eq10_e1592_d_b6: f64 = (eq10_e1590_d_b6 * p.p2);
        let eq10_e1592_d_b7: f64 = (eq10_e1590_d_b7 * p.p2);
        let eq10_e1592_d_b8: f64 = (eq10_e1590_d_b8 * p.p2);
        let eq10_e1592_d_b9: f64 = (eq10_e1590_d_b9 * p.p2);
        let eq10_e1592_d_b10: f64 = (eq10_e1590_d_b10 * p.p2);
        let eq10_e1592_d_b11: f64 = (eq10_e1590_d_b11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * s.v[184]);
        let eq10_e1594_d_n0: f64 = (eq10_e1592_d_n0 * s.v[184]);
        let eq10_e1594_d_n1: f64 = (eq10_e1592_d_n1 * s.v[184]);
        let eq10_e1594_d_n2: f64 = (eq10_e1592_d_n2 * s.v[184]);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * s.v[184]);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * s.v[184]);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * s.v[184]);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * s.v[184]);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * s.v[184]);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * s.v[184]);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * s.v[184]);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * s.v[184]);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * s.v[184]);
        let eq10_e1594_d_n12: f64 = (eq10_e1592_d_n12 * s.v[184]);
        let eq10_e1594_d_n13: f64 = (eq10_e1592_d_n13 * s.v[184]);
        let eq10_e1594_d_b0: f64 = (eq10_e1592_d_b0 * s.v[184]);
        let eq10_e1594_d_b1: f64 = (eq10_e1592_d_b1 * s.v[184]);
        let eq10_e1594_d_b2: f64 = (eq10_e1592_d_b2 * s.v[184]);
        let eq10_e1594_d_b3: f64 = (eq10_e1592_d_b3 * s.v[184]);
        let eq10_e1594_d_b4: f64 = (eq10_e1592_d_b4 * s.v[184]);
        let eq10_e1594_d_b5: f64 = (eq10_e1592_d_b5 * s.v[184]);
        let eq10_e1594_d_b6: f64 = (eq10_e1592_d_b6 * s.v[184]);
        let eq10_e1594_d_b7: f64 = (eq10_e1592_d_b7 * s.v[184]);
        let eq10_e1594_d_b8: f64 = (eq10_e1592_d_b8 * s.v[184]);
        let eq10_e1594_d_b9: f64 = (eq10_e1592_d_b9 * s.v[184]);
        let eq10_e1594_d_b10: f64 = (eq10_e1592_d_b10 * s.v[184]);
        let eq10_e1594_d_b11: f64 = (eq10_e1592_d_b11 * s.v[184]);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n0: f64 = (eq10_e1594_d_n0 * (nv12 - 0.0));
        let eq10_e1596_d_n1: f64 = (eq10_e1594_d_n1 * (nv12 - 0.0));
        let eq10_e1596_d_n2: f64 = (eq10_e1594_d_n2 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1596_d_n12: f64 = ((eq10_e1594_d_n12 * (nv12 - 0.0)) + eq10_e1594);
        let eq10_e1596_d_n13: f64 = (eq10_e1594_d_n13 * (nv12 - 0.0));
        let eq10_e1596_d_b0: f64 = (eq10_e1594_d_b0 * (nv12 - 0.0));
        let eq10_e1596_d_b1: f64 = (eq10_e1594_d_b1 * (nv12 - 0.0));
        let eq10_e1596_d_b2: f64 = (eq10_e1594_d_b2 * (nv12 - 0.0));
        let eq10_e1596_d_b3: f64 = (eq10_e1594_d_b3 * (nv12 - 0.0));
        let eq10_e1596_d_b4: f64 = (eq10_e1594_d_b4 * (nv12 - 0.0));
        let eq10_e1596_d_b5: f64 = (eq10_e1594_d_b5 * (nv12 - 0.0));
        let eq10_e1596_d_b6: f64 = (eq10_e1594_d_b6 * (nv12 - 0.0));
        let eq10_e1596_d_b7: f64 = (eq10_e1594_d_b7 * (nv12 - 0.0));
        let eq10_e1596_d_b8: f64 = (eq10_e1594_d_b8 * (nv12 - 0.0));
        let eq10_e1596_d_b9: f64 = (eq10_e1594_d_b9 * (nv12 - 0.0));
        let eq10_e1596_d_b10: f64 = (eq10_e1594_d_b10 * (nv12 - 0.0));
        let eq10_e1596_d_b11: f64 = (eq10_e1594_d_b11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n0: f64 = (0.5 * eq10_e1596_d_n0);
        let eq10_e1597_d_n1: f64 = (0.5 * eq10_e1596_d_n1);
        let eq10_e1597_d_n2: f64 = (0.5 * eq10_e1596_d_n2);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1596_d_n12);
        let eq10_e1597_d_n13: f64 = (0.5 * eq10_e1596_d_n13);
        let eq10_e1597_d_b0: f64 = (0.5 * eq10_e1596_d_b0);
        let eq10_e1597_d_b1: f64 = (0.5 * eq10_e1596_d_b1);
        let eq10_e1597_d_b2: f64 = (0.5 * eq10_e1596_d_b2);
        let eq10_e1597_d_b3: f64 = (0.5 * eq10_e1596_d_b3);
        let eq10_e1597_d_b4: f64 = (0.5 * eq10_e1596_d_b4);
        let eq10_e1597_d_b5: f64 = (0.5 * eq10_e1596_d_b5);
        let eq10_e1597_d_b6: f64 = (0.5 * eq10_e1596_d_b6);
        let eq10_e1597_d_b7: f64 = (0.5 * eq10_e1596_d_b7);
        let eq10_e1597_d_b8: f64 = (0.5 * eq10_e1596_d_b8);
        let eq10_e1597_d_b9: f64 = (0.5 * eq10_e1596_d_b9);
        let eq10_e1597_d_b10: f64 = (0.5 * eq10_e1596_d_b10);
        let eq10_e1597_d_b11: f64 = (0.5 * eq10_e1596_d_b11);
        let eq10_e1598_q: f64 = eq10_e1597;
        (eq10_e1597, eq10_e1597_d_n0, eq10_e1597_d_n1, eq10_e1597_d_n2, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1597_d_n13, eq10_e1597_d_b0, eq10_e1597_d_b1, eq10_e1597_d_b2, eq10_e1597_d_b3, eq10_e1597_d_b4, eq10_e1597_d_b5, eq10_e1597_d_b6, eq10_e1597_d_b7, eq10_e1597_d_b8, eq10_e1597_d_b9, eq10_e1597_d_b10, eq10_e1597_d_b11, eq10_e1598_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 14] = [eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13];
        let eq10_reactive_branch_derivatives: [f64; 12] = [eq10_e1600_d_b0, eq10_e1600_d_b1, eq10_e1600_d_b2, eq10_e1600_d_b3, eq10_e1600_d_b4, eq10_e1600_d_b5, eq10_e1600_d_b6, eq10_e1600_d_b7, eq10_e1600_d_b8, eq10_e1600_d_b9, eq10_e1600_d_b10, eq10_e1600_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq11_e1626, eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13, eq11_e1626_d_b0, eq11_e1626_d_b1, eq11_e1626_d_b2, eq11_e1626_d_b3, eq11_e1626_d_b4, eq11_e1626_d_b5, eq11_e1626_d_b6, eq11_e1626_d_b7, eq11_e1626_d_b8, eq11_e1626_d_b9, eq11_e1626_d_b10, eq11_e1626_d_b11, eq11_e1626_q,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq11_e1610: f64 = (1.0 - s.v[211]);
        let eq11_e1612: f64 = (eq11_e1610 * s.v[622]);
        let eq11_e1612_d_n0: f64 = (((-s.dn[211][0]) * s.v[622]) + (eq11_e1610 * s.dn[622][0]));
        let eq11_e1612_d_n1: f64 = (((-s.dn[211][1]) * s.v[622]) + (eq11_e1610 * s.dn[622][1]));
        let eq11_e1612_d_n2: f64 = (((-s.dn[211][2]) * s.v[622]) + (eq11_e1610 * s.dn[622][2]));
        let eq11_e1612_d_n3: f64 = (((-s.dn[211][3]) * s.v[622]) + (eq11_e1610 * s.dn[622][3]));
        let eq11_e1612_d_n4: f64 = (((-s.dn[211][4]) * s.v[622]) + (eq11_e1610 * s.dn[622][4]));
        let eq11_e1612_d_n5: f64 = (((-s.dn[211][5]) * s.v[622]) + (eq11_e1610 * s.dn[622][5]));
        let eq11_e1612_d_n6: f64 = (((-s.dn[211][6]) * s.v[622]) + (eq11_e1610 * s.dn[622][6]));
        let eq11_e1612_d_n7: f64 = (((-s.dn[211][7]) * s.v[622]) + (eq11_e1610 * s.dn[622][7]));
        let eq11_e1612_d_n8: f64 = (((-s.dn[211][8]) * s.v[622]) + (eq11_e1610 * s.dn[622][8]));
        let eq11_e1612_d_n9: f64 = (((-s.dn[211][9]) * s.v[622]) + (eq11_e1610 * s.dn[622][9]));
        let eq11_e1612_d_n10: f64 = (((-s.dn[211][10]) * s.v[622]) + (eq11_e1610 * s.dn[622][10]));
        let eq11_e1612_d_n11: f64 = (((-s.dn[211][11]) * s.v[622]) + (eq11_e1610 * s.dn[622][11]));
        let eq11_e1612_d_n12: f64 = (((-s.dn[211][12]) * s.v[622]) + (eq11_e1610 * s.dn[622][12]));
        let eq11_e1612_d_n13: f64 = (((-s.dn[211][13]) * s.v[622]) + (eq11_e1610 * s.dn[622][13]));
        let eq11_e1612_d_b0: f64 = (((-s.db[211][0]) * s.v[622]) + (eq11_e1610 * s.db[622][0]));
        let eq11_e1612_d_b1: f64 = (((-s.db[211][1]) * s.v[622]) + (eq11_e1610 * s.db[622][1]));
        let eq11_e1612_d_b2: f64 = (((-s.db[211][2]) * s.v[622]) + (eq11_e1610 * s.db[622][2]));
        let eq11_e1612_d_b3: f64 = (((-s.db[211][3]) * s.v[622]) + (eq11_e1610 * s.db[622][3]));
        let eq11_e1612_d_b4: f64 = (((-s.db[211][4]) * s.v[622]) + (eq11_e1610 * s.db[622][4]));
        let eq11_e1612_d_b5: f64 = (((-s.db[211][5]) * s.v[622]) + (eq11_e1610 * s.db[622][5]));
        let eq11_e1612_d_b6: f64 = (((-s.db[211][6]) * s.v[622]) + (eq11_e1610 * s.db[622][6]));
        let eq11_e1612_d_b7: f64 = (((-s.db[211][7]) * s.v[622]) + (eq11_e1610 * s.db[622][7]));
        let eq11_e1612_d_b8: f64 = (((-s.db[211][8]) * s.v[622]) + (eq11_e1610 * s.db[622][8]));
        let eq11_e1612_d_b9: f64 = (((-s.db[211][9]) * s.v[622]) + (eq11_e1610 * s.db[622][9]));
        let eq11_e1612_d_b10: f64 = (((-s.db[211][10]) * s.v[622]) + (eq11_e1610 * s.db[622][10]));
        let eq11_e1612_d_b11: f64 = (((-s.db[211][11]) * s.v[622]) + (eq11_e1610 * s.db[622][11]));
        let eq11_e1614: f64 = (eq11_e1612 * s.v[199]);
        let eq11_e1614_d_n0: f64 = (eq11_e1612_d_n0 * s.v[199]);
        let eq11_e1614_d_n1: f64 = (eq11_e1612_d_n1 * s.v[199]);
        let eq11_e1614_d_n2: f64 = (eq11_e1612_d_n2 * s.v[199]);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * s.v[199]);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * s.v[199]);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * s.v[199]);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * s.v[199]);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * s.v[199]);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * s.v[199]);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * s.v[199]);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * s.v[199]);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * s.v[199]);
        let eq11_e1614_d_n12: f64 = (eq11_e1612_d_n12 * s.v[199]);
        let eq11_e1614_d_n13: f64 = (eq11_e1612_d_n13 * s.v[199]);
        let eq11_e1614_d_b0: f64 = (eq11_e1612_d_b0 * s.v[199]);
        let eq11_e1614_d_b1: f64 = (eq11_e1612_d_b1 * s.v[199]);
        let eq11_e1614_d_b2: f64 = (eq11_e1612_d_b2 * s.v[199]);
        let eq11_e1614_d_b3: f64 = (eq11_e1612_d_b3 * s.v[199]);
        let eq11_e1614_d_b4: f64 = (eq11_e1612_d_b4 * s.v[199]);
        let eq11_e1614_d_b5: f64 = (eq11_e1612_d_b5 * s.v[199]);
        let eq11_e1614_d_b6: f64 = (eq11_e1612_d_b6 * s.v[199]);
        let eq11_e1614_d_b7: f64 = (eq11_e1612_d_b7 * s.v[199]);
        let eq11_e1614_d_b8: f64 = (eq11_e1612_d_b8 * s.v[199]);
        let eq11_e1614_d_b9: f64 = (eq11_e1612_d_b9 * s.v[199]);
        let eq11_e1614_d_b10: f64 = (eq11_e1612_d_b10 * s.v[199]);
        let eq11_e1614_d_b11: f64 = (eq11_e1612_d_b11 * s.v[199]);
        let eq11_e1616: f64 = (eq11_e1614 * s.v[183]);
        let eq11_e1616_d_n0: f64 = (eq11_e1614_d_n0 * s.v[183]);
        let eq11_e1616_d_n1: f64 = (eq11_e1614_d_n1 * s.v[183]);
        let eq11_e1616_d_n2: f64 = (eq11_e1614_d_n2 * s.v[183]);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * s.v[183]);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * s.v[183]);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * s.v[183]);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * s.v[183]);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * s.v[183]);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * s.v[183]);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * s.v[183]);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * s.v[183]);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * s.v[183]);
        let eq11_e1616_d_n12: f64 = (eq11_e1614_d_n12 * s.v[183]);
        let eq11_e1616_d_n13: f64 = (eq11_e1614_d_n13 * s.v[183]);
        let eq11_e1616_d_b0: f64 = (eq11_e1614_d_b0 * s.v[183]);
        let eq11_e1616_d_b1: f64 = (eq11_e1614_d_b1 * s.v[183]);
        let eq11_e1616_d_b2: f64 = (eq11_e1614_d_b2 * s.v[183]);
        let eq11_e1616_d_b3: f64 = (eq11_e1614_d_b3 * s.v[183]);
        let eq11_e1616_d_b4: f64 = (eq11_e1614_d_b4 * s.v[183]);
        let eq11_e1616_d_b5: f64 = (eq11_e1614_d_b5 * s.v[183]);
        let eq11_e1616_d_b6: f64 = (eq11_e1614_d_b6 * s.v[183]);
        let eq11_e1616_d_b7: f64 = (eq11_e1614_d_b7 * s.v[183]);
        let eq11_e1616_d_b8: f64 = (eq11_e1614_d_b8 * s.v[183]);
        let eq11_e1616_d_b9: f64 = (eq11_e1614_d_b9 * s.v[183]);
        let eq11_e1616_d_b10: f64 = (eq11_e1614_d_b10 * s.v[183]);
        let eq11_e1616_d_b11: f64 = (eq11_e1614_d_b11 * s.v[183]);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n0: f64 = (eq11_e1616_d_n0 * p.p2);
        let eq11_e1618_d_n1: f64 = (eq11_e1616_d_n1 * p.p2);
        let eq11_e1618_d_n2: f64 = (eq11_e1616_d_n2 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1618_d_n12: f64 = (eq11_e1616_d_n12 * p.p2);
        let eq11_e1618_d_n13: f64 = (eq11_e1616_d_n13 * p.p2);
        let eq11_e1618_d_b0: f64 = (eq11_e1616_d_b0 * p.p2);
        let eq11_e1618_d_b1: f64 = (eq11_e1616_d_b1 * p.p2);
        let eq11_e1618_d_b2: f64 = (eq11_e1616_d_b2 * p.p2);
        let eq11_e1618_d_b3: f64 = (eq11_e1616_d_b3 * p.p2);
        let eq11_e1618_d_b4: f64 = (eq11_e1616_d_b4 * p.p2);
        let eq11_e1618_d_b5: f64 = (eq11_e1616_d_b5 * p.p2);
        let eq11_e1618_d_b6: f64 = (eq11_e1616_d_b6 * p.p2);
        let eq11_e1618_d_b7: f64 = (eq11_e1616_d_b7 * p.p2);
        let eq11_e1618_d_b8: f64 = (eq11_e1616_d_b8 * p.p2);
        let eq11_e1618_d_b9: f64 = (eq11_e1616_d_b9 * p.p2);
        let eq11_e1618_d_b10: f64 = (eq11_e1616_d_b10 * p.p2);
        let eq11_e1618_d_b11: f64 = (eq11_e1616_d_b11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * s.v[184]);
        let eq11_e1620_d_n0: f64 = (eq11_e1618_d_n0 * s.v[184]);
        let eq11_e1620_d_n1: f64 = (eq11_e1618_d_n1 * s.v[184]);
        let eq11_e1620_d_n2: f64 = (eq11_e1618_d_n2 * s.v[184]);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * s.v[184]);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * s.v[184]);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * s.v[184]);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * s.v[184]);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * s.v[184]);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * s.v[184]);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * s.v[184]);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * s.v[184]);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * s.v[184]);
        let eq11_e1620_d_n12: f64 = (eq11_e1618_d_n12 * s.v[184]);
        let eq11_e1620_d_n13: f64 = (eq11_e1618_d_n13 * s.v[184]);
        let eq11_e1620_d_b0: f64 = (eq11_e1618_d_b0 * s.v[184]);
        let eq11_e1620_d_b1: f64 = (eq11_e1618_d_b1 * s.v[184]);
        let eq11_e1620_d_b2: f64 = (eq11_e1618_d_b2 * s.v[184]);
        let eq11_e1620_d_b3: f64 = (eq11_e1618_d_b3 * s.v[184]);
        let eq11_e1620_d_b4: f64 = (eq11_e1618_d_b4 * s.v[184]);
        let eq11_e1620_d_b5: f64 = (eq11_e1618_d_b5 * s.v[184]);
        let eq11_e1620_d_b6: f64 = (eq11_e1618_d_b6 * s.v[184]);
        let eq11_e1620_d_b7: f64 = (eq11_e1618_d_b7 * s.v[184]);
        let eq11_e1620_d_b8: f64 = (eq11_e1618_d_b8 * s.v[184]);
        let eq11_e1620_d_b9: f64 = (eq11_e1618_d_b9 * s.v[184]);
        let eq11_e1620_d_b10: f64 = (eq11_e1618_d_b10 * s.v[184]);
        let eq11_e1620_d_b11: f64 = (eq11_e1618_d_b11 * s.v[184]);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n0: f64 = (eq11_e1620_d_n0 * (nv12 - 0.0));
        let eq11_e1622_d_n1: f64 = (eq11_e1620_d_n1 * (nv12 - 0.0));
        let eq11_e1622_d_n2: f64 = (eq11_e1620_d_n2 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1622_d_n12: f64 = ((eq11_e1620_d_n12 * (nv12 - 0.0)) + eq11_e1620);
        let eq11_e1622_d_n13: f64 = (eq11_e1620_d_n13 * (nv12 - 0.0));
        let eq11_e1622_d_b0: f64 = (eq11_e1620_d_b0 * (nv12 - 0.0));
        let eq11_e1622_d_b1: f64 = (eq11_e1620_d_b1 * (nv12 - 0.0));
        let eq11_e1622_d_b2: f64 = (eq11_e1620_d_b2 * (nv12 - 0.0));
        let eq11_e1622_d_b3: f64 = (eq11_e1620_d_b3 * (nv12 - 0.0));
        let eq11_e1622_d_b4: f64 = (eq11_e1620_d_b4 * (nv12 - 0.0));
        let eq11_e1622_d_b5: f64 = (eq11_e1620_d_b5 * (nv12 - 0.0));
        let eq11_e1622_d_b6: f64 = (eq11_e1620_d_b6 * (nv12 - 0.0));
        let eq11_e1622_d_b7: f64 = (eq11_e1620_d_b7 * (nv12 - 0.0));
        let eq11_e1622_d_b8: f64 = (eq11_e1620_d_b8 * (nv12 - 0.0));
        let eq11_e1622_d_b9: f64 = (eq11_e1620_d_b9 * (nv12 - 0.0));
        let eq11_e1622_d_b10: f64 = (eq11_e1620_d_b10 * (nv12 - 0.0));
        let eq11_e1622_d_b11: f64 = (eq11_e1620_d_b11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n0: f64 = (0.5 * eq11_e1622_d_n0);
        let eq11_e1623_d_n1: f64 = (0.5 * eq11_e1622_d_n1);
        let eq11_e1623_d_n2: f64 = (0.5 * eq11_e1622_d_n2);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1622_d_n12);
        let eq11_e1623_d_n13: f64 = (0.5 * eq11_e1622_d_n13);
        let eq11_e1623_d_b0: f64 = (0.5 * eq11_e1622_d_b0);
        let eq11_e1623_d_b1: f64 = (0.5 * eq11_e1622_d_b1);
        let eq11_e1623_d_b2: f64 = (0.5 * eq11_e1622_d_b2);
        let eq11_e1623_d_b3: f64 = (0.5 * eq11_e1622_d_b3);
        let eq11_e1623_d_b4: f64 = (0.5 * eq11_e1622_d_b4);
        let eq11_e1623_d_b5: f64 = (0.5 * eq11_e1622_d_b5);
        let eq11_e1623_d_b6: f64 = (0.5 * eq11_e1622_d_b6);
        let eq11_e1623_d_b7: f64 = (0.5 * eq11_e1622_d_b7);
        let eq11_e1623_d_b8: f64 = (0.5 * eq11_e1622_d_b8);
        let eq11_e1623_d_b9: f64 = (0.5 * eq11_e1622_d_b9);
        let eq11_e1623_d_b10: f64 = (0.5 * eq11_e1622_d_b10);
        let eq11_e1623_d_b11: f64 = (0.5 * eq11_e1622_d_b11);
        let eq11_e1624_q: f64 = eq11_e1623;
        (eq11_e1623, eq11_e1623_d_n0, eq11_e1623_d_n1, eq11_e1623_d_n2, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1623_d_n13, eq11_e1623_d_b0, eq11_e1623_d_b1, eq11_e1623_d_b2, eq11_e1623_d_b3, eq11_e1623_d_b4, eq11_e1623_d_b5, eq11_e1623_d_b6, eq11_e1623_d_b7, eq11_e1623_d_b8, eq11_e1623_d_b9, eq11_e1623_d_b10, eq11_e1623_d_b11, eq11_e1624_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 14] = [eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13];
        let eq11_reactive_branch_derivatives: [f64; 12] = [eq11_e1626_d_b0, eq11_e1626_d_b1, eq11_e1626_d_b2, eq11_e1626_d_b3, eq11_e1626_d_b4, eq11_e1626_d_b5, eq11_e1626_d_b6, eq11_e1626_d_b7, eq11_e1626_d_b8, eq11_e1626_d_b9, eq11_e1626_d_b10, eq11_e1626_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1784, eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13, eq24_e1784_d_b0, eq24_e1784_d_b1, eq24_e1784_d_b2, eq24_e1784_d_b3, eq24_e1784_d_b4, eq24_e1784_d_b5, eq24_e1784_d_b6, eq24_e1784_d_b7, eq24_e1784_d_b8, eq24_e1784_d_b9, eq24_e1784_d_b10, eq24_e1784_d_b11, eq24_e1784_q,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq24_e1773: f64 = (s.v[622] * s.v[199]);
        let eq24_e1775: f64 = (eq24_e1773 * s.v[183]);
        let eq24_e1775_d_n0: f64 = ((s.dn[622][0] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n1: f64 = ((s.dn[622][1] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n2: f64 = ((s.dn[622][2] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n3: f64 = ((s.dn[622][3] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n4: f64 = ((s.dn[622][4] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n5: f64 = ((s.dn[622][5] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n6: f64 = ((s.dn[622][6] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n7: f64 = ((s.dn[622][7] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n8: f64 = ((s.dn[622][8] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n9: f64 = ((s.dn[622][9] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n10: f64 = ((s.dn[622][10] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n11: f64 = ((s.dn[622][11] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n12: f64 = ((s.dn[622][12] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_n13: f64 = ((s.dn[622][13] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b0: f64 = ((s.db[622][0] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b1: f64 = ((s.db[622][1] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b2: f64 = ((s.db[622][2] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b3: f64 = ((s.db[622][3] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b4: f64 = ((s.db[622][4] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b5: f64 = ((s.db[622][5] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b6: f64 = ((s.db[622][6] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b7: f64 = ((s.db[622][7] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b8: f64 = ((s.db[622][8] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b9: f64 = ((s.db[622][9] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b10: f64 = ((s.db[622][10] * s.v[199]) * s.v[183]);
        let eq24_e1775_d_b11: f64 = ((s.db[622][11] * s.v[199]) * s.v[183]);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n0: f64 = (eq24_e1775_d_n0 * p.p2);
        let eq24_e1777_d_n1: f64 = (eq24_e1775_d_n1 * p.p2);
        let eq24_e1777_d_n2: f64 = (eq24_e1775_d_n2 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1777_d_n12: f64 = (eq24_e1775_d_n12 * p.p2);
        let eq24_e1777_d_n13: f64 = (eq24_e1775_d_n13 * p.p2);
        let eq24_e1777_d_b0: f64 = (eq24_e1775_d_b0 * p.p2);
        let eq24_e1777_d_b1: f64 = (eq24_e1775_d_b1 * p.p2);
        let eq24_e1777_d_b2: f64 = (eq24_e1775_d_b2 * p.p2);
        let eq24_e1777_d_b3: f64 = (eq24_e1775_d_b3 * p.p2);
        let eq24_e1777_d_b4: f64 = (eq24_e1775_d_b4 * p.p2);
        let eq24_e1777_d_b5: f64 = (eq24_e1775_d_b5 * p.p2);
        let eq24_e1777_d_b6: f64 = (eq24_e1775_d_b6 * p.p2);
        let eq24_e1777_d_b7: f64 = (eq24_e1775_d_b7 * p.p2);
        let eq24_e1777_d_b8: f64 = (eq24_e1775_d_b8 * p.p2);
        let eq24_e1777_d_b9: f64 = (eq24_e1775_d_b9 * p.p2);
        let eq24_e1777_d_b10: f64 = (eq24_e1775_d_b10 * p.p2);
        let eq24_e1777_d_b11: f64 = (eq24_e1775_d_b11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * s.v[184]);
        let eq24_e1779_d_n0: f64 = (eq24_e1777_d_n0 * s.v[184]);
        let eq24_e1779_d_n1: f64 = (eq24_e1777_d_n1 * s.v[184]);
        let eq24_e1779_d_n2: f64 = (eq24_e1777_d_n2 * s.v[184]);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * s.v[184]);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * s.v[184]);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * s.v[184]);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * s.v[184]);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * s.v[184]);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * s.v[184]);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * s.v[184]);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * s.v[184]);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * s.v[184]);
        let eq24_e1779_d_n12: f64 = (eq24_e1777_d_n12 * s.v[184]);
        let eq24_e1779_d_n13: f64 = (eq24_e1777_d_n13 * s.v[184]);
        let eq24_e1779_d_b0: f64 = (eq24_e1777_d_b0 * s.v[184]);
        let eq24_e1779_d_b1: f64 = (eq24_e1777_d_b1 * s.v[184]);
        let eq24_e1779_d_b2: f64 = (eq24_e1777_d_b2 * s.v[184]);
        let eq24_e1779_d_b3: f64 = (eq24_e1777_d_b3 * s.v[184]);
        let eq24_e1779_d_b4: f64 = (eq24_e1777_d_b4 * s.v[184]);
        let eq24_e1779_d_b5: f64 = (eq24_e1777_d_b5 * s.v[184]);
        let eq24_e1779_d_b6: f64 = (eq24_e1777_d_b6 * s.v[184]);
        let eq24_e1779_d_b7: f64 = (eq24_e1777_d_b7 * s.v[184]);
        let eq24_e1779_d_b8: f64 = (eq24_e1777_d_b8 * s.v[184]);
        let eq24_e1779_d_b9: f64 = (eq24_e1777_d_b9 * s.v[184]);
        let eq24_e1779_d_b10: f64 = (eq24_e1777_d_b10 * s.v[184]);
        let eq24_e1779_d_b11: f64 = (eq24_e1777_d_b11 * s.v[184]);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n0: f64 = (eq24_e1779_d_n0 * (nv12 - 0.0));
        let eq24_e1781_d_n1: f64 = (eq24_e1779_d_n1 * (nv12 - 0.0));
        let eq24_e1781_d_n2: f64 = (eq24_e1779_d_n2 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1781_d_n12: f64 = ((eq24_e1779_d_n12 * (nv12 - 0.0)) + eq24_e1779);
        let eq24_e1781_d_n13: f64 = (eq24_e1779_d_n13 * (nv12 - 0.0));
        let eq24_e1781_d_b0: f64 = (eq24_e1779_d_b0 * (nv12 - 0.0));
        let eq24_e1781_d_b1: f64 = (eq24_e1779_d_b1 * (nv12 - 0.0));
        let eq24_e1781_d_b2: f64 = (eq24_e1779_d_b2 * (nv12 - 0.0));
        let eq24_e1781_d_b3: f64 = (eq24_e1779_d_b3 * (nv12 - 0.0));
        let eq24_e1781_d_b4: f64 = (eq24_e1779_d_b4 * (nv12 - 0.0));
        let eq24_e1781_d_b5: f64 = (eq24_e1779_d_b5 * (nv12 - 0.0));
        let eq24_e1781_d_b6: f64 = (eq24_e1779_d_b6 * (nv12 - 0.0));
        let eq24_e1781_d_b7: f64 = (eq24_e1779_d_b7 * (nv12 - 0.0));
        let eq24_e1781_d_b8: f64 = (eq24_e1779_d_b8 * (nv12 - 0.0));
        let eq24_e1781_d_b9: f64 = (eq24_e1779_d_b9 * (nv12 - 0.0));
        let eq24_e1781_d_b10: f64 = (eq24_e1779_d_b10 * (nv12 - 0.0));
        let eq24_e1781_d_b11: f64 = (eq24_e1779_d_b11 * (nv12 - 0.0));
        let eq24_e1782_q: f64 = eq24_e1781;
        (eq24_e1781, eq24_e1781_d_n0, eq24_e1781_d_n1, eq24_e1781_d_n2, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1781_d_n12, eq24_e1781_d_n13, eq24_e1781_d_b0, eq24_e1781_d_b1, eq24_e1781_d_b2, eq24_e1781_d_b3, eq24_e1781_d_b4, eq24_e1781_d_b5, eq24_e1781_d_b6, eq24_e1781_d_b7, eq24_e1781_d_b8, eq24_e1781_d_b9, eq24_e1781_d_b10, eq24_e1781_d_b11, eq24_e1782_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_reactive_node_derivatives: [f64; 14] = [eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13];
        let eq24_reactive_branch_derivatives: [f64; 12] = [eq24_e1784_d_b0, eq24_e1784_d_b1, eq24_e1784_d_b2, eq24_e1784_d_b3, eq24_e1784_d_b4, eq24_e1784_d_b5, eq24_e1784_d_b6, eq24_e1784_d_b7, eq24_e1784_d_b8, eq24_e1784_d_b9, eq24_e1784_d_b10, eq24_e1784_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1841, eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13, eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11, eq27_e1841_q,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq27_e1825: f64 = (1.0 + s.v[211]);
        let eq27_e1827: f64 = (eq27_e1825 * s.v[622]);
        let eq27_e1827_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq27_e1825 * s.dn[622][0]));
        let eq27_e1827_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq27_e1825 * s.dn[622][1]));
        let eq27_e1827_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq27_e1825 * s.dn[622][2]));
        let eq27_e1827_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq27_e1825 * s.dn[622][3]));
        let eq27_e1827_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq27_e1825 * s.dn[622][4]));
        let eq27_e1827_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq27_e1825 * s.dn[622][5]));
        let eq27_e1827_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq27_e1825 * s.dn[622][6]));
        let eq27_e1827_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq27_e1825 * s.dn[622][7]));
        let eq27_e1827_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq27_e1825 * s.dn[622][8]));
        let eq27_e1827_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq27_e1825 * s.dn[622][9]));
        let eq27_e1827_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq27_e1825 * s.dn[622][10]));
        let eq27_e1827_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq27_e1825 * s.dn[622][11]));
        let eq27_e1827_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq27_e1825 * s.dn[622][12]));
        let eq27_e1827_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq27_e1825 * s.dn[622][13]));
        let eq27_e1827_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq27_e1825 * s.db[622][0]));
        let eq27_e1827_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq27_e1825 * s.db[622][1]));
        let eq27_e1827_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq27_e1825 * s.db[622][2]));
        let eq27_e1827_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq27_e1825 * s.db[622][3]));
        let eq27_e1827_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq27_e1825 * s.db[622][4]));
        let eq27_e1827_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq27_e1825 * s.db[622][5]));
        let eq27_e1827_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq27_e1825 * s.db[622][6]));
        let eq27_e1827_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq27_e1825 * s.db[622][7]));
        let eq27_e1827_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq27_e1825 * s.db[622][8]));
        let eq27_e1827_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq27_e1825 * s.db[622][9]));
        let eq27_e1827_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq27_e1825 * s.db[622][10]));
        let eq27_e1827_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq27_e1825 * s.db[622][11]));
        let eq27_e1829: f64 = (eq27_e1827 * s.v[199]);
        let eq27_e1829_d_n0: f64 = (eq27_e1827_d_n0 * s.v[199]);
        let eq27_e1829_d_n1: f64 = (eq27_e1827_d_n1 * s.v[199]);
        let eq27_e1829_d_n2: f64 = (eq27_e1827_d_n2 * s.v[199]);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * s.v[199]);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * s.v[199]);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * s.v[199]);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * s.v[199]);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * s.v[199]);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * s.v[199]);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * s.v[199]);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * s.v[199]);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * s.v[199]);
        let eq27_e1829_d_n12: f64 = (eq27_e1827_d_n12 * s.v[199]);
        let eq27_e1829_d_n13: f64 = (eq27_e1827_d_n13 * s.v[199]);
        let eq27_e1829_d_b0: f64 = (eq27_e1827_d_b0 * s.v[199]);
        let eq27_e1829_d_b1: f64 = (eq27_e1827_d_b1 * s.v[199]);
        let eq27_e1829_d_b2: f64 = (eq27_e1827_d_b2 * s.v[199]);
        let eq27_e1829_d_b3: f64 = (eq27_e1827_d_b3 * s.v[199]);
        let eq27_e1829_d_b4: f64 = (eq27_e1827_d_b4 * s.v[199]);
        let eq27_e1829_d_b5: f64 = (eq27_e1827_d_b5 * s.v[199]);
        let eq27_e1829_d_b6: f64 = (eq27_e1827_d_b6 * s.v[199]);
        let eq27_e1829_d_b7: f64 = (eq27_e1827_d_b7 * s.v[199]);
        let eq27_e1829_d_b8: f64 = (eq27_e1827_d_b8 * s.v[199]);
        let eq27_e1829_d_b9: f64 = (eq27_e1827_d_b9 * s.v[199]);
        let eq27_e1829_d_b10: f64 = (eq27_e1827_d_b10 * s.v[199]);
        let eq27_e1829_d_b11: f64 = (eq27_e1827_d_b11 * s.v[199]);
        let eq27_e1831: f64 = (eq27_e1829 * s.v[183]);
        let eq27_e1831_d_n0: f64 = (eq27_e1829_d_n0 * s.v[183]);
        let eq27_e1831_d_n1: f64 = (eq27_e1829_d_n1 * s.v[183]);
        let eq27_e1831_d_n2: f64 = (eq27_e1829_d_n2 * s.v[183]);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * s.v[183]);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * s.v[183]);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * s.v[183]);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * s.v[183]);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * s.v[183]);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * s.v[183]);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * s.v[183]);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * s.v[183]);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * s.v[183]);
        let eq27_e1831_d_n12: f64 = (eq27_e1829_d_n12 * s.v[183]);
        let eq27_e1831_d_n13: f64 = (eq27_e1829_d_n13 * s.v[183]);
        let eq27_e1831_d_b0: f64 = (eq27_e1829_d_b0 * s.v[183]);
        let eq27_e1831_d_b1: f64 = (eq27_e1829_d_b1 * s.v[183]);
        let eq27_e1831_d_b2: f64 = (eq27_e1829_d_b2 * s.v[183]);
        let eq27_e1831_d_b3: f64 = (eq27_e1829_d_b3 * s.v[183]);
        let eq27_e1831_d_b4: f64 = (eq27_e1829_d_b4 * s.v[183]);
        let eq27_e1831_d_b5: f64 = (eq27_e1829_d_b5 * s.v[183]);
        let eq27_e1831_d_b6: f64 = (eq27_e1829_d_b6 * s.v[183]);
        let eq27_e1831_d_b7: f64 = (eq27_e1829_d_b7 * s.v[183]);
        let eq27_e1831_d_b8: f64 = (eq27_e1829_d_b8 * s.v[183]);
        let eq27_e1831_d_b9: f64 = (eq27_e1829_d_b9 * s.v[183]);
        let eq27_e1831_d_b10: f64 = (eq27_e1829_d_b10 * s.v[183]);
        let eq27_e1831_d_b11: f64 = (eq27_e1829_d_b11 * s.v[183]);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n0: f64 = (eq27_e1831_d_n0 * p.p2);
        let eq27_e1833_d_n1: f64 = (eq27_e1831_d_n1 * p.p2);
        let eq27_e1833_d_n2: f64 = (eq27_e1831_d_n2 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1833_d_n12: f64 = (eq27_e1831_d_n12 * p.p2);
        let eq27_e1833_d_n13: f64 = (eq27_e1831_d_n13 * p.p2);
        let eq27_e1833_d_b0: f64 = (eq27_e1831_d_b0 * p.p2);
        let eq27_e1833_d_b1: f64 = (eq27_e1831_d_b1 * p.p2);
        let eq27_e1833_d_b2: f64 = (eq27_e1831_d_b2 * p.p2);
        let eq27_e1833_d_b3: f64 = (eq27_e1831_d_b3 * p.p2);
        let eq27_e1833_d_b4: f64 = (eq27_e1831_d_b4 * p.p2);
        let eq27_e1833_d_b5: f64 = (eq27_e1831_d_b5 * p.p2);
        let eq27_e1833_d_b6: f64 = (eq27_e1831_d_b6 * p.p2);
        let eq27_e1833_d_b7: f64 = (eq27_e1831_d_b7 * p.p2);
        let eq27_e1833_d_b8: f64 = (eq27_e1831_d_b8 * p.p2);
        let eq27_e1833_d_b9: f64 = (eq27_e1831_d_b9 * p.p2);
        let eq27_e1833_d_b10: f64 = (eq27_e1831_d_b10 * p.p2);
        let eq27_e1833_d_b11: f64 = (eq27_e1831_d_b11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * s.v[184]);
        let eq27_e1835_d_n0: f64 = (eq27_e1833_d_n0 * s.v[184]);
        let eq27_e1835_d_n1: f64 = (eq27_e1833_d_n1 * s.v[184]);
        let eq27_e1835_d_n2: f64 = (eq27_e1833_d_n2 * s.v[184]);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * s.v[184]);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * s.v[184]);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * s.v[184]);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * s.v[184]);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * s.v[184]);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * s.v[184]);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * s.v[184]);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * s.v[184]);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * s.v[184]);
        let eq27_e1835_d_n12: f64 = (eq27_e1833_d_n12 * s.v[184]);
        let eq27_e1835_d_n13: f64 = (eq27_e1833_d_n13 * s.v[184]);
        let eq27_e1835_d_b0: f64 = (eq27_e1833_d_b0 * s.v[184]);
        let eq27_e1835_d_b1: f64 = (eq27_e1833_d_b1 * s.v[184]);
        let eq27_e1835_d_b2: f64 = (eq27_e1833_d_b2 * s.v[184]);
        let eq27_e1835_d_b3: f64 = (eq27_e1833_d_b3 * s.v[184]);
        let eq27_e1835_d_b4: f64 = (eq27_e1833_d_b4 * s.v[184]);
        let eq27_e1835_d_b5: f64 = (eq27_e1833_d_b5 * s.v[184]);
        let eq27_e1835_d_b6: f64 = (eq27_e1833_d_b6 * s.v[184]);
        let eq27_e1835_d_b7: f64 = (eq27_e1833_d_b7 * s.v[184]);
        let eq27_e1835_d_b8: f64 = (eq27_e1833_d_b8 * s.v[184]);
        let eq27_e1835_d_b9: f64 = (eq27_e1833_d_b9 * s.v[184]);
        let eq27_e1835_d_b10: f64 = (eq27_e1833_d_b10 * s.v[184]);
        let eq27_e1835_d_b11: f64 = (eq27_e1833_d_b11 * s.v[184]);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n0: f64 = (eq27_e1835_d_n0 * (nv12 - 0.0));
        let eq27_e1837_d_n1: f64 = (eq27_e1835_d_n1 * (nv12 - 0.0));
        let eq27_e1837_d_n2: f64 = (eq27_e1835_d_n2 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1837_d_n12: f64 = ((eq27_e1835_d_n12 * (nv12 - 0.0)) + eq27_e1835);
        let eq27_e1837_d_n13: f64 = (eq27_e1835_d_n13 * (nv12 - 0.0));
        let eq27_e1837_d_b0: f64 = (eq27_e1835_d_b0 * (nv12 - 0.0));
        let eq27_e1837_d_b1: f64 = (eq27_e1835_d_b1 * (nv12 - 0.0));
        let eq27_e1837_d_b2: f64 = (eq27_e1835_d_b2 * (nv12 - 0.0));
        let eq27_e1837_d_b3: f64 = (eq27_e1835_d_b3 * (nv12 - 0.0));
        let eq27_e1837_d_b4: f64 = (eq27_e1835_d_b4 * (nv12 - 0.0));
        let eq27_e1837_d_b5: f64 = (eq27_e1835_d_b5 * (nv12 - 0.0));
        let eq27_e1837_d_b6: f64 = (eq27_e1835_d_b6 * (nv12 - 0.0));
        let eq27_e1837_d_b7: f64 = (eq27_e1835_d_b7 * (nv12 - 0.0));
        let eq27_e1837_d_b8: f64 = (eq27_e1835_d_b8 * (nv12 - 0.0));
        let eq27_e1837_d_b9: f64 = (eq27_e1835_d_b9 * (nv12 - 0.0));
        let eq27_e1837_d_b10: f64 = (eq27_e1835_d_b10 * (nv12 - 0.0));
        let eq27_e1837_d_b11: f64 = (eq27_e1835_d_b11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n0: f64 = (0.5 * eq27_e1837_d_n0);
        let eq27_e1838_d_n1: f64 = (0.5 * eq27_e1837_d_n1);
        let eq27_e1838_d_n2: f64 = (0.5 * eq27_e1837_d_n2);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1837_d_n12);
        let eq27_e1838_d_n13: f64 = (0.5 * eq27_e1837_d_n13);
        let eq27_e1838_d_b0: f64 = (0.5 * eq27_e1837_d_b0);
        let eq27_e1838_d_b1: f64 = (0.5 * eq27_e1837_d_b1);
        let eq27_e1838_d_b2: f64 = (0.5 * eq27_e1837_d_b2);
        let eq27_e1838_d_b3: f64 = (0.5 * eq27_e1837_d_b3);
        let eq27_e1838_d_b4: f64 = (0.5 * eq27_e1837_d_b4);
        let eq27_e1838_d_b5: f64 = (0.5 * eq27_e1837_d_b5);
        let eq27_e1838_d_b6: f64 = (0.5 * eq27_e1837_d_b6);
        let eq27_e1838_d_b7: f64 = (0.5 * eq27_e1837_d_b7);
        let eq27_e1838_d_b8: f64 = (0.5 * eq27_e1837_d_b8);
        let eq27_e1838_d_b9: f64 = (0.5 * eq27_e1837_d_b9);
        let eq27_e1838_d_b10: f64 = (0.5 * eq27_e1837_d_b10);
        let eq27_e1838_d_b11: f64 = (0.5 * eq27_e1837_d_b11);
        let eq27_e1839_q: f64 = eq27_e1838;
        (eq27_e1838, eq27_e1838_d_n0, eq27_e1838_d_n1, eq27_e1838_d_n2, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1838_d_n13, eq27_e1838_d_b0, eq27_e1838_d_b1, eq27_e1838_d_b2, eq27_e1838_d_b3, eq27_e1838_d_b4, eq27_e1838_d_b5, eq27_e1838_d_b6, eq27_e1838_d_b7, eq27_e1838_d_b8, eq27_e1838_d_b9, eq27_e1838_d_b10, eq27_e1838_d_b11, eq27_e1839_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 14] = [eq27_e1841_d_n0, eq27_e1841_d_n1, eq27_e1841_d_n2, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_d_n13];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1841_d_b0, eq27_e1841_d_b1, eq27_e1841_d_b2, eq27_e1841_d_b3, eq27_e1841_d_b4, eq27_e1841_d_b5, eq27_e1841_d_b6, eq27_e1841_d_b7, eq27_e1841_d_b8, eq27_e1841_d_b9, eq27_e1841_d_b10, eq27_e1841_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1868, eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13, eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11, eq28_e1868_q,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq28_e1852: f64 = (1.0 - s.v[211]);
        let eq28_e1854: f64 = (eq28_e1852 * s.v[622]);
        let eq28_e1854_d_n0: f64 = (((-s.dn[211][0]) * s.v[622]) + (eq28_e1852 * s.dn[622][0]));
        let eq28_e1854_d_n1: f64 = (((-s.dn[211][1]) * s.v[622]) + (eq28_e1852 * s.dn[622][1]));
        let eq28_e1854_d_n2: f64 = (((-s.dn[211][2]) * s.v[622]) + (eq28_e1852 * s.dn[622][2]));
        let eq28_e1854_d_n3: f64 = (((-s.dn[211][3]) * s.v[622]) + (eq28_e1852 * s.dn[622][3]));
        let eq28_e1854_d_n4: f64 = (((-s.dn[211][4]) * s.v[622]) + (eq28_e1852 * s.dn[622][4]));
        let eq28_e1854_d_n5: f64 = (((-s.dn[211][5]) * s.v[622]) + (eq28_e1852 * s.dn[622][5]));
        let eq28_e1854_d_n6: f64 = (((-s.dn[211][6]) * s.v[622]) + (eq28_e1852 * s.dn[622][6]));
        let eq28_e1854_d_n7: f64 = (((-s.dn[211][7]) * s.v[622]) + (eq28_e1852 * s.dn[622][7]));
        let eq28_e1854_d_n8: f64 = (((-s.dn[211][8]) * s.v[622]) + (eq28_e1852 * s.dn[622][8]));
        let eq28_e1854_d_n9: f64 = (((-s.dn[211][9]) * s.v[622]) + (eq28_e1852 * s.dn[622][9]));
        let eq28_e1854_d_n10: f64 = (((-s.dn[211][10]) * s.v[622]) + (eq28_e1852 * s.dn[622][10]));
        let eq28_e1854_d_n11: f64 = (((-s.dn[211][11]) * s.v[622]) + (eq28_e1852 * s.dn[622][11]));
        let eq28_e1854_d_n12: f64 = (((-s.dn[211][12]) * s.v[622]) + (eq28_e1852 * s.dn[622][12]));
        let eq28_e1854_d_n13: f64 = (((-s.dn[211][13]) * s.v[622]) + (eq28_e1852 * s.dn[622][13]));
        let eq28_e1854_d_b0: f64 = (((-s.db[211][0]) * s.v[622]) + (eq28_e1852 * s.db[622][0]));
        let eq28_e1854_d_b1: f64 = (((-s.db[211][1]) * s.v[622]) + (eq28_e1852 * s.db[622][1]));
        let eq28_e1854_d_b2: f64 = (((-s.db[211][2]) * s.v[622]) + (eq28_e1852 * s.db[622][2]));
        let eq28_e1854_d_b3: f64 = (((-s.db[211][3]) * s.v[622]) + (eq28_e1852 * s.db[622][3]));
        let eq28_e1854_d_b4: f64 = (((-s.db[211][4]) * s.v[622]) + (eq28_e1852 * s.db[622][4]));
        let eq28_e1854_d_b5: f64 = (((-s.db[211][5]) * s.v[622]) + (eq28_e1852 * s.db[622][5]));
        let eq28_e1854_d_b6: f64 = (((-s.db[211][6]) * s.v[622]) + (eq28_e1852 * s.db[622][6]));
        let eq28_e1854_d_b7: f64 = (((-s.db[211][7]) * s.v[622]) + (eq28_e1852 * s.db[622][7]));
        let eq28_e1854_d_b8: f64 = (((-s.db[211][8]) * s.v[622]) + (eq28_e1852 * s.db[622][8]));
        let eq28_e1854_d_b9: f64 = (((-s.db[211][9]) * s.v[622]) + (eq28_e1852 * s.db[622][9]));
        let eq28_e1854_d_b10: f64 = (((-s.db[211][10]) * s.v[622]) + (eq28_e1852 * s.db[622][10]));
        let eq28_e1854_d_b11: f64 = (((-s.db[211][11]) * s.v[622]) + (eq28_e1852 * s.db[622][11]));
        let eq28_e1856: f64 = (eq28_e1854 * s.v[199]);
        let eq28_e1856_d_n0: f64 = (eq28_e1854_d_n0 * s.v[199]);
        let eq28_e1856_d_n1: f64 = (eq28_e1854_d_n1 * s.v[199]);
        let eq28_e1856_d_n2: f64 = (eq28_e1854_d_n2 * s.v[199]);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * s.v[199]);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * s.v[199]);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * s.v[199]);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * s.v[199]);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * s.v[199]);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * s.v[199]);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * s.v[199]);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * s.v[199]);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * s.v[199]);
        let eq28_e1856_d_n12: f64 = (eq28_e1854_d_n12 * s.v[199]);
        let eq28_e1856_d_n13: f64 = (eq28_e1854_d_n13 * s.v[199]);
        let eq28_e1856_d_b0: f64 = (eq28_e1854_d_b0 * s.v[199]);
        let eq28_e1856_d_b1: f64 = (eq28_e1854_d_b1 * s.v[199]);
        let eq28_e1856_d_b2: f64 = (eq28_e1854_d_b2 * s.v[199]);
        let eq28_e1856_d_b3: f64 = (eq28_e1854_d_b3 * s.v[199]);
        let eq28_e1856_d_b4: f64 = (eq28_e1854_d_b4 * s.v[199]);
        let eq28_e1856_d_b5: f64 = (eq28_e1854_d_b5 * s.v[199]);
        let eq28_e1856_d_b6: f64 = (eq28_e1854_d_b6 * s.v[199]);
        let eq28_e1856_d_b7: f64 = (eq28_e1854_d_b7 * s.v[199]);
        let eq28_e1856_d_b8: f64 = (eq28_e1854_d_b8 * s.v[199]);
        let eq28_e1856_d_b9: f64 = (eq28_e1854_d_b9 * s.v[199]);
        let eq28_e1856_d_b10: f64 = (eq28_e1854_d_b10 * s.v[199]);
        let eq28_e1856_d_b11: f64 = (eq28_e1854_d_b11 * s.v[199]);
        let eq28_e1858: f64 = (eq28_e1856 * s.v[183]);
        let eq28_e1858_d_n0: f64 = (eq28_e1856_d_n0 * s.v[183]);
        let eq28_e1858_d_n1: f64 = (eq28_e1856_d_n1 * s.v[183]);
        let eq28_e1858_d_n2: f64 = (eq28_e1856_d_n2 * s.v[183]);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * s.v[183]);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * s.v[183]);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * s.v[183]);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * s.v[183]);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * s.v[183]);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * s.v[183]);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * s.v[183]);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * s.v[183]);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * s.v[183]);
        let eq28_e1858_d_n12: f64 = (eq28_e1856_d_n12 * s.v[183]);
        let eq28_e1858_d_n13: f64 = (eq28_e1856_d_n13 * s.v[183]);
        let eq28_e1858_d_b0: f64 = (eq28_e1856_d_b0 * s.v[183]);
        let eq28_e1858_d_b1: f64 = (eq28_e1856_d_b1 * s.v[183]);
        let eq28_e1858_d_b2: f64 = (eq28_e1856_d_b2 * s.v[183]);
        let eq28_e1858_d_b3: f64 = (eq28_e1856_d_b3 * s.v[183]);
        let eq28_e1858_d_b4: f64 = (eq28_e1856_d_b4 * s.v[183]);
        let eq28_e1858_d_b5: f64 = (eq28_e1856_d_b5 * s.v[183]);
        let eq28_e1858_d_b6: f64 = (eq28_e1856_d_b6 * s.v[183]);
        let eq28_e1858_d_b7: f64 = (eq28_e1856_d_b7 * s.v[183]);
        let eq28_e1858_d_b8: f64 = (eq28_e1856_d_b8 * s.v[183]);
        let eq28_e1858_d_b9: f64 = (eq28_e1856_d_b9 * s.v[183]);
        let eq28_e1858_d_b10: f64 = (eq28_e1856_d_b10 * s.v[183]);
        let eq28_e1858_d_b11: f64 = (eq28_e1856_d_b11 * s.v[183]);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n0: f64 = (eq28_e1858_d_n0 * p.p2);
        let eq28_e1860_d_n1: f64 = (eq28_e1858_d_n1 * p.p2);
        let eq28_e1860_d_n2: f64 = (eq28_e1858_d_n2 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1860_d_n12: f64 = (eq28_e1858_d_n12 * p.p2);
        let eq28_e1860_d_n13: f64 = (eq28_e1858_d_n13 * p.p2);
        let eq28_e1860_d_b0: f64 = (eq28_e1858_d_b0 * p.p2);
        let eq28_e1860_d_b1: f64 = (eq28_e1858_d_b1 * p.p2);
        let eq28_e1860_d_b2: f64 = (eq28_e1858_d_b2 * p.p2);
        let eq28_e1860_d_b3: f64 = (eq28_e1858_d_b3 * p.p2);
        let eq28_e1860_d_b4: f64 = (eq28_e1858_d_b4 * p.p2);
        let eq28_e1860_d_b5: f64 = (eq28_e1858_d_b5 * p.p2);
        let eq28_e1860_d_b6: f64 = (eq28_e1858_d_b6 * p.p2);
        let eq28_e1860_d_b7: f64 = (eq28_e1858_d_b7 * p.p2);
        let eq28_e1860_d_b8: f64 = (eq28_e1858_d_b8 * p.p2);
        let eq28_e1860_d_b9: f64 = (eq28_e1858_d_b9 * p.p2);
        let eq28_e1860_d_b10: f64 = (eq28_e1858_d_b10 * p.p2);
        let eq28_e1860_d_b11: f64 = (eq28_e1858_d_b11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * s.v[184]);
        let eq28_e1862_d_n0: f64 = (eq28_e1860_d_n0 * s.v[184]);
        let eq28_e1862_d_n1: f64 = (eq28_e1860_d_n1 * s.v[184]);
        let eq28_e1862_d_n2: f64 = (eq28_e1860_d_n2 * s.v[184]);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * s.v[184]);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * s.v[184]);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * s.v[184]);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * s.v[184]);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * s.v[184]);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * s.v[184]);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * s.v[184]);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * s.v[184]);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * s.v[184]);
        let eq28_e1862_d_n12: f64 = (eq28_e1860_d_n12 * s.v[184]);
        let eq28_e1862_d_n13: f64 = (eq28_e1860_d_n13 * s.v[184]);
        let eq28_e1862_d_b0: f64 = (eq28_e1860_d_b0 * s.v[184]);
        let eq28_e1862_d_b1: f64 = (eq28_e1860_d_b1 * s.v[184]);
        let eq28_e1862_d_b2: f64 = (eq28_e1860_d_b2 * s.v[184]);
        let eq28_e1862_d_b3: f64 = (eq28_e1860_d_b3 * s.v[184]);
        let eq28_e1862_d_b4: f64 = (eq28_e1860_d_b4 * s.v[184]);
        let eq28_e1862_d_b5: f64 = (eq28_e1860_d_b5 * s.v[184]);
        let eq28_e1862_d_b6: f64 = (eq28_e1860_d_b6 * s.v[184]);
        let eq28_e1862_d_b7: f64 = (eq28_e1860_d_b7 * s.v[184]);
        let eq28_e1862_d_b8: f64 = (eq28_e1860_d_b8 * s.v[184]);
        let eq28_e1862_d_b9: f64 = (eq28_e1860_d_b9 * s.v[184]);
        let eq28_e1862_d_b10: f64 = (eq28_e1860_d_b10 * s.v[184]);
        let eq28_e1862_d_b11: f64 = (eq28_e1860_d_b11 * s.v[184]);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n0: f64 = (eq28_e1862_d_n0 * (nv12 - 0.0));
        let eq28_e1864_d_n1: f64 = (eq28_e1862_d_n1 * (nv12 - 0.0));
        let eq28_e1864_d_n2: f64 = (eq28_e1862_d_n2 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1864_d_n12: f64 = ((eq28_e1862_d_n12 * (nv12 - 0.0)) + eq28_e1862);
        let eq28_e1864_d_n13: f64 = (eq28_e1862_d_n13 * (nv12 - 0.0));
        let eq28_e1864_d_b0: f64 = (eq28_e1862_d_b0 * (nv12 - 0.0));
        let eq28_e1864_d_b1: f64 = (eq28_e1862_d_b1 * (nv12 - 0.0));
        let eq28_e1864_d_b2: f64 = (eq28_e1862_d_b2 * (nv12 - 0.0));
        let eq28_e1864_d_b3: f64 = (eq28_e1862_d_b3 * (nv12 - 0.0));
        let eq28_e1864_d_b4: f64 = (eq28_e1862_d_b4 * (nv12 - 0.0));
        let eq28_e1864_d_b5: f64 = (eq28_e1862_d_b5 * (nv12 - 0.0));
        let eq28_e1864_d_b6: f64 = (eq28_e1862_d_b6 * (nv12 - 0.0));
        let eq28_e1864_d_b7: f64 = (eq28_e1862_d_b7 * (nv12 - 0.0));
        let eq28_e1864_d_b8: f64 = (eq28_e1862_d_b8 * (nv12 - 0.0));
        let eq28_e1864_d_b9: f64 = (eq28_e1862_d_b9 * (nv12 - 0.0));
        let eq28_e1864_d_b10: f64 = (eq28_e1862_d_b10 * (nv12 - 0.0));
        let eq28_e1864_d_b11: f64 = (eq28_e1862_d_b11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n0: f64 = (0.5 * eq28_e1864_d_n0);
        let eq28_e1865_d_n1: f64 = (0.5 * eq28_e1864_d_n1);
        let eq28_e1865_d_n2: f64 = (0.5 * eq28_e1864_d_n2);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1864_d_n12);
        let eq28_e1865_d_n13: f64 = (0.5 * eq28_e1864_d_n13);
        let eq28_e1865_d_b0: f64 = (0.5 * eq28_e1864_d_b0);
        let eq28_e1865_d_b1: f64 = (0.5 * eq28_e1864_d_b1);
        let eq28_e1865_d_b2: f64 = (0.5 * eq28_e1864_d_b2);
        let eq28_e1865_d_b3: f64 = (0.5 * eq28_e1864_d_b3);
        let eq28_e1865_d_b4: f64 = (0.5 * eq28_e1864_d_b4);
        let eq28_e1865_d_b5: f64 = (0.5 * eq28_e1864_d_b5);
        let eq28_e1865_d_b6: f64 = (0.5 * eq28_e1864_d_b6);
        let eq28_e1865_d_b7: f64 = (0.5 * eq28_e1864_d_b7);
        let eq28_e1865_d_b8: f64 = (0.5 * eq28_e1864_d_b8);
        let eq28_e1865_d_b9: f64 = (0.5 * eq28_e1864_d_b9);
        let eq28_e1865_d_b10: f64 = (0.5 * eq28_e1864_d_b10);
        let eq28_e1865_d_b11: f64 = (0.5 * eq28_e1864_d_b11);
        let eq28_e1866_q: f64 = eq28_e1865;
        (eq28_e1865, eq28_e1865_d_n0, eq28_e1865_d_n1, eq28_e1865_d_n2, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1865_d_n13, eq28_e1865_d_b0, eq28_e1865_d_b1, eq28_e1865_d_b2, eq28_e1865_d_b3, eq28_e1865_d_b4, eq28_e1865_d_b5, eq28_e1865_d_b6, eq28_e1865_d_b7, eq28_e1865_d_b8, eq28_e1865_d_b9, eq28_e1865_d_b10, eq28_e1865_d_b11, eq28_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 14] = [eq28_e1868_d_n0, eq28_e1868_d_n1, eq28_e1868_d_n2, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_d_n13];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1868_d_b0, eq28_e1868_d_b1, eq28_e1868_d_b2, eq28_e1868_d_b3, eq28_e1868_d_b4, eq28_e1868_d_b5, eq28_e1868_d_b6, eq28_e1868_d_b7, eq28_e1868_d_b8, eq28_e1868_d_b9, eq28_e1868_d_b10, eq28_e1868_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e1938_q: f64 = s.v[1057];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[10]),
            nodes,
            &s.dn[1057],
            branches,
            &s.db[1057],
            multiplicity,
        );
        let eq36_e1940_q: f64 = s.v[1058];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[11]),
            nodes,
            &s.dn[1058],
            branches,
            &s.db[1058],
            multiplicity,
        );
        let eq37_e1942_q: f64 = s.v[1051];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &s.dn[1051],
            branches,
            &s.db[1051],
            multiplicity,
        );
        let eq38_e1944_q: f64 = s.v[1052];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &s.dn[1052],
            branches,
            &s.db[1052],
            multiplicity,
        );
        let eq39_e1946_q: f64 = s.v[1054];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[10]),
            nodes,
            &s.dn[1054],
            branches,
            &s.db[1054],
            multiplicity,
        );
        let eq40_e1948_q: f64 = s.v[1055];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &s.dn[1055],
            branches,
            &s.db[1055],
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let __rspice_deriv_cse_14: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let __rspice_deriv_cse_15: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let __rspice_deriv_cse_16: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let __rspice_deriv_cse_17: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let __rspice_deriv_cse_18: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let __rspice_deriv_cse_19: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let __rspice_deriv_cse_20: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let __rspice_deriv_cse_21: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let __rspice_deriv_cse_22: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let __rspice_deriv_cse_23: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let __rspice_deriv_cse_24: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let __rspice_deriv_cse_25: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let __rspice_deriv_cse_26: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let __rspice_deriv_cse_27: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let __rspice_deriv_cse_28: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let __rspice_deriv_cse_29: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let __rspice_deriv_cse_30: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let __rspice_deriv_cse_31: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let __rspice_deriv_cse_32: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let __rspice_deriv_cse_33: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let __rspice_deriv_cse_34: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let __rspice_deriv_cse_35: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let __rspice_deriv_cse_36: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let __rspice_deriv_cse_37: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let __rspice_deriv_cse_38: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let __rspice_deriv_cse_39: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let __rspice_deriv_cse_40: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let __rspice_deriv_cse_41: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let __rspice_deriv_cse_42: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let __rspice_deriv_cse_43: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let __rspice_deriv_cse_44: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let __rspice_deriv_cse_45: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let __rspice_deriv_cse_46: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let __rspice_deriv_cse_47: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let __rspice_deriv_cse_48: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let __rspice_deriv_cse_49: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let __rspice_deriv_cse_50: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let __rspice_deriv_cse_51: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let __rspice_deriv_cse_52: f64 = (__rspice_deriv_cse_0 + __rspice_deriv_cse_26);
        let __rspice_deriv_cse_53: f64 = (__rspice_deriv_cse_1 + __rspice_deriv_cse_27);
        let __rspice_deriv_cse_54: f64 = (__rspice_deriv_cse_2 + __rspice_deriv_cse_28);
        let __rspice_deriv_cse_55: f64 = (__rspice_deriv_cse_3 + __rspice_deriv_cse_29);
        let __rspice_deriv_cse_56: f64 = (__rspice_deriv_cse_4 + __rspice_deriv_cse_30);
        let __rspice_deriv_cse_57: f64 = (__rspice_deriv_cse_5 + __rspice_deriv_cse_31);
        let __rspice_deriv_cse_58: f64 = (__rspice_deriv_cse_6 + __rspice_deriv_cse_32);
        let __rspice_deriv_cse_59: f64 = (__rspice_deriv_cse_7 + __rspice_deriv_cse_33);
        let __rspice_deriv_cse_60: f64 = (__rspice_deriv_cse_8 + __rspice_deriv_cse_34);
        let __rspice_deriv_cse_61: f64 = (__rspice_deriv_cse_9 + __rspice_deriv_cse_35);
        let __rspice_deriv_cse_62: f64 = (__rspice_deriv_cse_10 + __rspice_deriv_cse_36);
        let __rspice_deriv_cse_63: f64 = (__rspice_deriv_cse_11 + __rspice_deriv_cse_37);
        let __rspice_deriv_cse_64: f64 = (__rspice_deriv_cse_12 + __rspice_deriv_cse_38);
        let __rspice_deriv_cse_65: f64 = (__rspice_deriv_cse_13 + __rspice_deriv_cse_39);
        let __rspice_deriv_cse_66: f64 = (__rspice_deriv_cse_14 + __rspice_deriv_cse_40);
        let __rspice_deriv_cse_67: f64 = (__rspice_deriv_cse_15 + __rspice_deriv_cse_41);
        let __rspice_deriv_cse_68: f64 = (__rspice_deriv_cse_16 + __rspice_deriv_cse_42);
        let __rspice_deriv_cse_69: f64 = (__rspice_deriv_cse_17 + __rspice_deriv_cse_43);
        let __rspice_deriv_cse_70: f64 = (__rspice_deriv_cse_18 + __rspice_deriv_cse_44);
        let __rspice_deriv_cse_71: f64 = (__rspice_deriv_cse_19 + __rspice_deriv_cse_45);
        let __rspice_deriv_cse_72: f64 = (__rspice_deriv_cse_20 + __rspice_deriv_cse_46);
        let __rspice_deriv_cse_73: f64 = (__rspice_deriv_cse_21 + __rspice_deriv_cse_47);
        let __rspice_deriv_cse_74: f64 = (__rspice_deriv_cse_22 + __rspice_deriv_cse_48);
        let __rspice_deriv_cse_75: f64 = (__rspice_deriv_cse_23 + __rspice_deriv_cse_49);
        let __rspice_deriv_cse_76: f64 = (__rspice_deriv_cse_24 + __rspice_deriv_cse_50);
        let __rspice_deriv_cse_77: f64 = (__rspice_deriv_cse_25 + __rspice_deriv_cse_51);
        let __rspice_deriv_cse_78: f64 = (__rspice_deriv_cse_52 - s.dn[1017][0]);
        let __rspice_deriv_cse_79: f64 = (__rspice_deriv_cse_53 - s.dn[1017][1]);
        let __rspice_deriv_cse_80: f64 = (__rspice_deriv_cse_54 - s.dn[1017][2]);
        let __rspice_deriv_cse_81: f64 = (__rspice_deriv_cse_55 - s.dn[1017][3]);
        let __rspice_deriv_cse_82: f64 = (__rspice_deriv_cse_56 - s.dn[1017][4]);
        let __rspice_deriv_cse_83: f64 = (__rspice_deriv_cse_57 - s.dn[1017][5]);
        let __rspice_deriv_cse_84: f64 = (__rspice_deriv_cse_58 - s.dn[1017][6]);
        let __rspice_deriv_cse_85: f64 = (__rspice_deriv_cse_59 - s.dn[1017][7]);
        let __rspice_deriv_cse_86: f64 = (__rspice_deriv_cse_60 - s.dn[1017][8]);
        let __rspice_deriv_cse_87: f64 = (__rspice_deriv_cse_61 - s.dn[1017][9]);
        let __rspice_deriv_cse_88: f64 = (__rspice_deriv_cse_62 - s.dn[1017][10]);
        let __rspice_deriv_cse_89: f64 = (__rspice_deriv_cse_63 - s.dn[1017][11]);
        let __rspice_deriv_cse_90: f64 = (__rspice_deriv_cse_64 - s.dn[1017][12]);
        let __rspice_deriv_cse_91: f64 = (__rspice_deriv_cse_65 - s.dn[1017][13]);
        let __rspice_deriv_cse_92: f64 = (__rspice_deriv_cse_66 - s.db[1017][0]);
        let __rspice_deriv_cse_93: f64 = (__rspice_deriv_cse_67 - s.db[1017][1]);
        let __rspice_deriv_cse_94: f64 = (__rspice_deriv_cse_68 - s.db[1017][2]);
        let __rspice_deriv_cse_95: f64 = (__rspice_deriv_cse_69 - s.db[1017][3]);
        let __rspice_deriv_cse_96: f64 = (__rspice_deriv_cse_70 - s.db[1017][4]);
        let __rspice_deriv_cse_97: f64 = (__rspice_deriv_cse_71 - s.db[1017][5]);
        let __rspice_deriv_cse_98: f64 = (__rspice_deriv_cse_72 - s.db[1017][6]);
        let __rspice_deriv_cse_99: f64 = (__rspice_deriv_cse_73 - s.db[1017][7]);
        let __rspice_deriv_cse_100: f64 = (__rspice_deriv_cse_74 - s.db[1017][8]);
        let __rspice_deriv_cse_101: f64 = (__rspice_deriv_cse_75 - s.db[1017][9]);
        let __rspice_deriv_cse_102: f64 = (__rspice_deriv_cse_76 - s.db[1017][10]);
        let __rspice_deriv_cse_103: f64 = (__rspice_deriv_cse_77 - s.db[1017][11]);
        let eq41_e1950: f64 = (-s.v[379]);
        let eq41_e1952: f64 = (eq41_e1950 * s.v[423]);
        let eq41_e1952_d_n0: f64 = (((-s.dn[379][0]) * s.v[423]) + (eq41_e1950 * s.dn[423][0]));
        let eq41_e1952_d_n1: f64 = (((-s.dn[379][1]) * s.v[423]) + (eq41_e1950 * s.dn[423][1]));
        let eq41_e1952_d_n2: f64 = (((-s.dn[379][2]) * s.v[423]) + (eq41_e1950 * s.dn[423][2]));
        let eq41_e1952_d_n3: f64 = (((-s.dn[379][3]) * s.v[423]) + (eq41_e1950 * s.dn[423][3]));
        let eq41_e1952_d_n4: f64 = (((-s.dn[379][4]) * s.v[423]) + (eq41_e1950 * s.dn[423][4]));
        let eq41_e1952_d_n5: f64 = (((-s.dn[379][5]) * s.v[423]) + (eq41_e1950 * s.dn[423][5]));
        let eq41_e1952_d_n6: f64 = (((-s.dn[379][6]) * s.v[423]) + (eq41_e1950 * s.dn[423][6]));
        let eq41_e1952_d_n7: f64 = (((-s.dn[379][7]) * s.v[423]) + (eq41_e1950 * s.dn[423][7]));
        let eq41_e1952_d_n8: f64 = (((-s.dn[379][8]) * s.v[423]) + (eq41_e1950 * s.dn[423][8]));
        let eq41_e1952_d_n9: f64 = (((-s.dn[379][9]) * s.v[423]) + (eq41_e1950 * s.dn[423][9]));
        let eq41_e1952_d_n10: f64 = (((-s.dn[379][10]) * s.v[423]) + (eq41_e1950 * s.dn[423][10]));
        let eq41_e1952_d_n11: f64 = (((-s.dn[379][11]) * s.v[423]) + (eq41_e1950 * s.dn[423][11]));
        let eq41_e1952_d_n12: f64 = (((-s.dn[379][12]) * s.v[423]) + (eq41_e1950 * s.dn[423][12]));
        let eq41_e1952_d_n13: f64 = (((-s.dn[379][13]) * s.v[423]) + (eq41_e1950 * s.dn[423][13]));
        let eq41_e1952_d_b0: f64 = (((-s.db[379][0]) * s.v[423]) + (eq41_e1950 * s.db[423][0]));
        let eq41_e1952_d_b1: f64 = (((-s.db[379][1]) * s.v[423]) + (eq41_e1950 * s.db[423][1]));
        let eq41_e1952_d_b2: f64 = (((-s.db[379][2]) * s.v[423]) + (eq41_e1950 * s.db[423][2]));
        let eq41_e1952_d_b3: f64 = (((-s.db[379][3]) * s.v[423]) + (eq41_e1950 * s.db[423][3]));
        let eq41_e1952_d_b4: f64 = (((-s.db[379][4]) * s.v[423]) + (eq41_e1950 * s.db[423][4]));
        let eq41_e1952_d_b5: f64 = (((-s.db[379][5]) * s.v[423]) + (eq41_e1950 * s.db[423][5]));
        let eq41_e1952_d_b6: f64 = (((-s.db[379][6]) * s.v[423]) + (eq41_e1950 * s.db[423][6]));
        let eq41_e1952_d_b7: f64 = (((-s.db[379][7]) * s.v[423]) + (eq41_e1950 * s.db[423][7]));
        let eq41_e1952_d_b8: f64 = (((-s.db[379][8]) * s.v[423]) + (eq41_e1950 * s.db[423][8]));
        let eq41_e1952_d_b9: f64 = (((-s.db[379][9]) * s.v[423]) + (eq41_e1950 * s.db[423][9]));
        let eq41_e1952_d_b10: f64 = (((-s.db[379][10]) * s.v[423]) + (eq41_e1950 * s.db[423][10]));
        let eq41_e1952_d_b11: f64 = (((-s.db[379][11]) * s.v[423]) + (eq41_e1950 * s.db[423][11]));
        let eq41_e1953_q: f64 = eq41_e1952;
        let eq41_reactive_node_derivatives: [f64; 14] = [eq41_e1952_d_n0, eq41_e1952_d_n1, eq41_e1952_d_n2, eq41_e1952_d_n3, eq41_e1952_d_n4, eq41_e1952_d_n5, eq41_e1952_d_n6, eq41_e1952_d_n7, eq41_e1952_d_n8, eq41_e1952_d_n9, eq41_e1952_d_n10, eq41_e1952_d_n11, eq41_e1952_d_n12, eq41_e1952_d_n13];
        let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1952_d_b0, eq41_e1952_d_b1, eq41_e1952_d_b2, eq41_e1952_d_b3, eq41_e1952_d_b4, eq41_e1952_d_b5, eq41_e1952_d_b6, eq41_e1952_d_b7, eq41_e1952_d_b8, eq41_e1952_d_b9, eq41_e1952_d_b10, eq41_e1952_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1955: f64 = (-s.v[379]);
        let eq42_e1957: f64 = (eq42_e1955 * s.v[424]);
        let eq42_e1957_d_n0: f64 = (((-s.dn[379][0]) * s.v[424]) + (eq42_e1955 * s.dn[424][0]));
        let eq42_e1957_d_n1: f64 = (((-s.dn[379][1]) * s.v[424]) + (eq42_e1955 * s.dn[424][1]));
        let eq42_e1957_d_n2: f64 = (((-s.dn[379][2]) * s.v[424]) + (eq42_e1955 * s.dn[424][2]));
        let eq42_e1957_d_n3: f64 = (((-s.dn[379][3]) * s.v[424]) + (eq42_e1955 * s.dn[424][3]));
        let eq42_e1957_d_n4: f64 = (((-s.dn[379][4]) * s.v[424]) + (eq42_e1955 * s.dn[424][4]));
        let eq42_e1957_d_n5: f64 = (((-s.dn[379][5]) * s.v[424]) + (eq42_e1955 * s.dn[424][5]));
        let eq42_e1957_d_n6: f64 = (((-s.dn[379][6]) * s.v[424]) + (eq42_e1955 * s.dn[424][6]));
        let eq42_e1957_d_n7: f64 = (((-s.dn[379][7]) * s.v[424]) + (eq42_e1955 * s.dn[424][7]));
        let eq42_e1957_d_n8: f64 = (((-s.dn[379][8]) * s.v[424]) + (eq42_e1955 * s.dn[424][8]));
        let eq42_e1957_d_n9: f64 = (((-s.dn[379][9]) * s.v[424]) + (eq42_e1955 * s.dn[424][9]));
        let eq42_e1957_d_n10: f64 = (((-s.dn[379][10]) * s.v[424]) + (eq42_e1955 * s.dn[424][10]));
        let eq42_e1957_d_n11: f64 = (((-s.dn[379][11]) * s.v[424]) + (eq42_e1955 * s.dn[424][11]));
        let eq42_e1957_d_n12: f64 = (((-s.dn[379][12]) * s.v[424]) + (eq42_e1955 * s.dn[424][12]));
        let eq42_e1957_d_n13: f64 = (((-s.dn[379][13]) * s.v[424]) + (eq42_e1955 * s.dn[424][13]));
        let eq42_e1957_d_b0: f64 = (((-s.db[379][0]) * s.v[424]) + (eq42_e1955 * s.db[424][0]));
        let eq42_e1957_d_b1: f64 = (((-s.db[379][1]) * s.v[424]) + (eq42_e1955 * s.db[424][1]));
        let eq42_e1957_d_b2: f64 = (((-s.db[379][2]) * s.v[424]) + (eq42_e1955 * s.db[424][2]));
        let eq42_e1957_d_b3: f64 = (((-s.db[379][3]) * s.v[424]) + (eq42_e1955 * s.db[424][3]));
        let eq42_e1957_d_b4: f64 = (((-s.db[379][4]) * s.v[424]) + (eq42_e1955 * s.db[424][4]));
        let eq42_e1957_d_b5: f64 = (((-s.db[379][5]) * s.v[424]) + (eq42_e1955 * s.db[424][5]));
        let eq42_e1957_d_b6: f64 = (((-s.db[379][6]) * s.v[424]) + (eq42_e1955 * s.db[424][6]));
        let eq42_e1957_d_b7: f64 = (((-s.db[379][7]) * s.v[424]) + (eq42_e1955 * s.db[424][7]));
        let eq42_e1957_d_b8: f64 = (((-s.db[379][8]) * s.v[424]) + (eq42_e1955 * s.db[424][8]));
        let eq42_e1957_d_b9: f64 = (((-s.db[379][9]) * s.v[424]) + (eq42_e1955 * s.db[424][9]));
        let eq42_e1957_d_b10: f64 = (((-s.db[379][10]) * s.v[424]) + (eq42_e1955 * s.db[424][10]));
        let eq42_e1957_d_b11: f64 = (((-s.db[379][11]) * s.v[424]) + (eq42_e1955 * s.db[424][11]));
        let eq42_e1958_q: f64 = eq42_e1957;
        let eq42_reactive_node_derivatives: [f64; 14] = [eq42_e1957_d_n0, eq42_e1957_d_n1, eq42_e1957_d_n2, eq42_e1957_d_n3, eq42_e1957_d_n4, eq42_e1957_d_n5, eq42_e1957_d_n6, eq42_e1957_d_n7, eq42_e1957_d_n8, eq42_e1957_d_n9, eq42_e1957_d_n10, eq42_e1957_d_n11, eq42_e1957_d_n12, eq42_e1957_d_n13];
        let eq42_reactive_branch_derivatives: [f64; 12] = [eq42_e1957_d_b0, eq42_e1957_d_b1, eq42_e1957_d_b2, eq42_e1957_d_b3, eq42_e1957_d_b4, eq42_e1957_d_b5, eq42_e1957_d_b6, eq42_e1957_d_b7, eq42_e1957_d_b8, eq42_e1957_d_b9, eq42_e1957_d_b10, eq42_e1957_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1969_q: f64 = s.v[1039];
        let eq45_e1970: f64 = (s.v[379] * s.v[1039]);
        let eq45_e1970_d_n0: f64 = ((s.dn[379][0] * s.v[1039]) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_d_n1: f64 = ((s.dn[379][1] * s.v[1039]) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_d_n2: f64 = ((s.dn[379][2] * s.v[1039]) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_d_n3: f64 = ((s.dn[379][3] * s.v[1039]) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_d_n4: f64 = ((s.dn[379][4] * s.v[1039]) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_d_n5: f64 = ((s.dn[379][5] * s.v[1039]) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_d_n6: f64 = ((s.dn[379][6] * s.v[1039]) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_d_n7: f64 = ((s.dn[379][7] * s.v[1039]) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_d_n8: f64 = ((s.dn[379][8] * s.v[1039]) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_d_n9: f64 = ((s.dn[379][9] * s.v[1039]) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_d_n10: f64 = ((s.dn[379][10] * s.v[1039]) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_d_n11: f64 = ((s.dn[379][11] * s.v[1039]) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_d_n12: f64 = ((s.dn[379][12] * s.v[1039]) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_d_n13: f64 = ((s.dn[379][13] * s.v[1039]) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_d_b0: f64 = ((s.db[379][0] * s.v[1039]) + (s.v[379] * s.db[1039][0]));
        let eq45_e1970_d_b1: f64 = ((s.db[379][1] * s.v[1039]) + (s.v[379] * s.db[1039][1]));
        let eq45_e1970_d_b2: f64 = ((s.db[379][2] * s.v[1039]) + (s.v[379] * s.db[1039][2]));
        let eq45_e1970_d_b3: f64 = ((s.db[379][3] * s.v[1039]) + (s.v[379] * s.db[1039][3]));
        let eq45_e1970_d_b4: f64 = ((s.db[379][4] * s.v[1039]) + (s.v[379] * s.db[1039][4]));
        let eq45_e1970_d_b5: f64 = ((s.db[379][5] * s.v[1039]) + (s.v[379] * s.db[1039][5]));
        let eq45_e1970_d_b6: f64 = ((s.db[379][6] * s.v[1039]) + (s.v[379] * s.db[1039][6]));
        let eq45_e1970_d_b7: f64 = ((s.db[379][7] * s.v[1039]) + (s.v[379] * s.db[1039][7]));
        let eq45_e1970_d_b8: f64 = ((s.db[379][8] * s.v[1039]) + (s.v[379] * s.db[1039][8]));
        let eq45_e1970_d_b9: f64 = ((s.db[379][9] * s.v[1039]) + (s.v[379] * s.db[1039][9]));
        let eq45_e1970_d_b10: f64 = ((s.db[379][10] * s.v[1039]) + (s.v[379] * s.db[1039][10]));
        let eq45_e1970_d_b11: f64 = ((s.db[379][11] * s.v[1039]) + (s.v[379] * s.db[1039][11]));
        let eq45_e1970_q: f64 = (s.v[379] * eq45_e1969_q);
        let eq45_e1970_q_d_n0: f64 = ((s.dn[379][0] * eq45_e1969_q) + (s.v[379] * s.dn[1039][0]));
        let eq45_e1970_q_d_n1: f64 = ((s.dn[379][1] * eq45_e1969_q) + (s.v[379] * s.dn[1039][1]));
        let eq45_e1970_q_d_n2: f64 = ((s.dn[379][2] * eq45_e1969_q) + (s.v[379] * s.dn[1039][2]));
        let eq45_e1970_q_d_n3: f64 = ((s.dn[379][3] * eq45_e1969_q) + (s.v[379] * s.dn[1039][3]));
        let eq45_e1970_q_d_n4: f64 = ((s.dn[379][4] * eq45_e1969_q) + (s.v[379] * s.dn[1039][4]));
        let eq45_e1970_q_d_n5: f64 = ((s.dn[379][5] * eq45_e1969_q) + (s.v[379] * s.dn[1039][5]));
        let eq45_e1970_q_d_n6: f64 = ((s.dn[379][6] * eq45_e1969_q) + (s.v[379] * s.dn[1039][6]));
        let eq45_e1970_q_d_n7: f64 = ((s.dn[379][7] * eq45_e1969_q) + (s.v[379] * s.dn[1039][7]));
        let eq45_e1970_q_d_n8: f64 = ((s.dn[379][8] * eq45_e1969_q) + (s.v[379] * s.dn[1039][8]));
        let eq45_e1970_q_d_n9: f64 = ((s.dn[379][9] * eq45_e1969_q) + (s.v[379] * s.dn[1039][9]));
        let eq45_e1970_q_d_n10: f64 = ((s.dn[379][10] * eq45_e1969_q) + (s.v[379] * s.dn[1039][10]));
        let eq45_e1970_q_d_n11: f64 = ((s.dn[379][11] * eq45_e1969_q) + (s.v[379] * s.dn[1039][11]));
        let eq45_e1970_q_d_n12: f64 = ((s.dn[379][12] * eq45_e1969_q) + (s.v[379] * s.dn[1039][12]));
        let eq45_e1970_q_d_n13: f64 = ((s.dn[379][13] * eq45_e1969_q) + (s.v[379] * s.dn[1039][13]));
        let eq45_e1970_q_d_b0: f64 = ((s.db[379][0] * eq45_e1969_q) + (s.v[379] * s.db[1039][0]));
        let eq45_e1970_q_d_b1: f64 = ((s.db[379][1] * eq45_e1969_q) + (s.v[379] * s.db[1039][1]));
        let eq45_e1970_q_d_b2: f64 = ((s.db[379][2] * eq45_e1969_q) + (s.v[379] * s.db[1039][2]));
        let eq45_e1970_q_d_b3: f64 = ((s.db[379][3] * eq45_e1969_q) + (s.v[379] * s.db[1039][3]));
        let eq45_e1970_q_d_b4: f64 = ((s.db[379][4] * eq45_e1969_q) + (s.v[379] * s.db[1039][4]));
        let eq45_e1970_q_d_b5: f64 = ((s.db[379][5] * eq45_e1969_q) + (s.v[379] * s.db[1039][5]));
        let eq45_e1970_q_d_b6: f64 = ((s.db[379][6] * eq45_e1969_q) + (s.v[379] * s.db[1039][6]));
        let eq45_e1970_q_d_b7: f64 = ((s.db[379][7] * eq45_e1969_q) + (s.v[379] * s.db[1039][7]));
        let eq45_e1970_q_d_b8: f64 = ((s.db[379][8] * eq45_e1969_q) + (s.v[379] * s.db[1039][8]));
        let eq45_e1970_q_d_b9: f64 = ((s.db[379][9] * eq45_e1969_q) + (s.v[379] * s.db[1039][9]));
        let eq45_e1970_q_d_b10: f64 = ((s.db[379][10] * eq45_e1969_q) + (s.v[379] * s.db[1039][10]));
        let eq45_e1970_q_d_b11: f64 = ((s.db[379][11] * eq45_e1969_q) + (s.v[379] * s.db[1039][11]));
        let eq45_reactive_node_derivatives: [f64; 14] = [eq45_e1970_q_d_n0, eq45_e1970_q_d_n1, eq45_e1970_q_d_n2, eq45_e1970_q_d_n3, eq45_e1970_q_d_n4, eq45_e1970_q_d_n5, eq45_e1970_q_d_n6, eq45_e1970_q_d_n7, eq45_e1970_q_d_n8, eq45_e1970_q_d_n9, eq45_e1970_q_d_n10, eq45_e1970_q_d_n11, eq45_e1970_q_d_n12, eq45_e1970_q_d_n13];
        let eq45_reactive_branch_derivatives: [f64; 12] = [eq45_e1970_q_d_b0, eq45_e1970_q_d_b1, eq45_e1970_q_d_b2, eq45_e1970_q_d_b3, eq45_e1970_q_d_b4, eq45_e1970_q_d_b5, eq45_e1970_q_d_b6, eq45_e1970_q_d_b7, eq45_e1970_q_d_b8, eq45_e1970_q_d_b9, eq45_e1970_q_d_b10, eq45_e1970_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1972_q: f64 = s.v[1047];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &s.dn[1047],
            branches,
            &s.db[1047],
            multiplicity,
        );
        let eq47_e1974_q: f64 = s.v[1046];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &s.dn[1046],
            branches,
            &s.db[1046],
            multiplicity,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n1, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_d_n12, eq67_e2103_d_n13, eq67_e2103_d_b0, eq67_e2103_d_b1, eq67_e2103_d_b2, eq67_e2103_d_b3, eq67_e2103_d_b4, eq67_e2103_d_b5, eq67_e2103_d_b6, eq67_e2103_d_b7, eq67_e2103_d_b8, eq67_e2103_d_b9, eq67_e2103_d_b10, eq67_e2103_d_b11, eq67_e2103_q, eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13, eq67_e2103_q_d_b0, eq67_e2103_q_d_b1, eq67_e2103_q_d_b2, eq67_e2103_q_d_b3, eq67_e2103_q_d_b4, eq67_e2103_q_d_b5, eq67_e2103_q_d_b6, eq67_e2103_q_d_b7, eq67_e2103_q_d_b8, eq67_e2103_q_d_b9, eq67_e2103_q_d_b10, eq67_e2103_q_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && s.b[2025]) {
        let eq67_e2094: f64 = (s.v[634] * s.v[1015]);
        let eq67_e2097: f64 = (s.v[634] * s.v[1016]);
        let eq67_e2098_q: f64 = eq67_e2097;
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2097);
        let eq67_e2099_q: f64 = eq67_e2098_q;
        let eq67_e2101: f64 = (eq67_e2099 - s.v[1017]);
        let eq67_e2101_q: f64 = eq67_e2099_q;
        (eq67_e2101, __rspice_deriv_cse_78, __rspice_deriv_cse_79, __rspice_deriv_cse_80, __rspice_deriv_cse_81, __rspice_deriv_cse_82, __rspice_deriv_cse_83, __rspice_deriv_cse_84, __rspice_deriv_cse_85, __rspice_deriv_cse_86, __rspice_deriv_cse_87, __rspice_deriv_cse_88, __rspice_deriv_cse_89, __rspice_deriv_cse_90, __rspice_deriv_cse_91, __rspice_deriv_cse_92, __rspice_deriv_cse_93, __rspice_deriv_cse_94, __rspice_deriv_cse_95, __rspice_deriv_cse_96, __rspice_deriv_cse_97, __rspice_deriv_cse_98, __rspice_deriv_cse_99, __rspice_deriv_cse_100, __rspice_deriv_cse_101, __rspice_deriv_cse_102, __rspice_deriv_cse_103, eq67_e2101_q, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_reactive_node_derivatives: [f64; 14] = [eq67_e2103_q_d_n0, eq67_e2103_q_d_n1, eq67_e2103_q_d_n2, eq67_e2103_q_d_n3, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5, eq67_e2103_q_d_n6, eq67_e2103_q_d_n7, eq67_e2103_q_d_n8, eq67_e2103_q_d_n9, eq67_e2103_q_d_n10, eq67_e2103_q_d_n11, eq67_e2103_q_d_n12, eq67_e2103_q_d_n13];
        let eq67_reactive_branch_derivatives: [f64; 12] = [eq67_e2103_q_d_b0, eq67_e2103_q_d_b1, eq67_e2103_q_d_b2, eq67_e2103_q_d_b3, eq67_e2103_q_d_b4, eq67_e2103_q_d_b5, eq67_e2103_q_d_b6, eq67_e2103_q_d_b7, eq67_e2103_q_d_b8, eq67_e2103_q_d_b9, eq67_e2103_q_d_b10, eq67_e2103_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n1, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_d_n12, eq68_e2121_d_n13, eq68_e2121_d_b0, eq68_e2121_d_b1, eq68_e2121_d_b2, eq68_e2121_d_b3, eq68_e2121_d_b4, eq68_e2121_d_b5, eq68_e2121_d_b6, eq68_e2121_d_b7, eq68_e2121_d_b8, eq68_e2121_d_b9, eq68_e2121_d_b10, eq68_e2121_d_b11, eq68_e2121_q, eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13, eq68_e2121_q_d_b0, eq68_e2121_q_d_b1, eq68_e2121_q_d_b2, eq68_e2121_q_d_b3, eq68_e2121_q_d_b4, eq68_e2121_q_d_b5, eq68_e2121_q_d_b6, eq68_e2121_q_d_b7, eq68_e2121_q_d_b8, eq68_e2121_q_d_b9, eq68_e2121_q_d_b10, eq68_e2121_q_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && (!s.b[2025])) {
        let eq68_e2112: f64 = (s.v[634] * s.v[1015]);
        let eq68_e2115: f64 = (s.v[634] * s.v[1016]);
        let eq68_e2116_q: f64 = eq68_e2115;
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2115);
        let eq68_e2117_q: f64 = eq68_e2116_q;
        let eq68_e2119: f64 = (eq68_e2117 - s.v[1017]);
        let eq68_e2119_q: f64 = eq68_e2117_q;
        (eq68_e2119, __rspice_deriv_cse_78, __rspice_deriv_cse_79, __rspice_deriv_cse_80, __rspice_deriv_cse_81, __rspice_deriv_cse_82, __rspice_deriv_cse_83, __rspice_deriv_cse_84, __rspice_deriv_cse_85, __rspice_deriv_cse_86, __rspice_deriv_cse_87, __rspice_deriv_cse_88, __rspice_deriv_cse_89, __rspice_deriv_cse_90, __rspice_deriv_cse_91, __rspice_deriv_cse_92, __rspice_deriv_cse_93, __rspice_deriv_cse_94, __rspice_deriv_cse_95, __rspice_deriv_cse_96, __rspice_deriv_cse_97, __rspice_deriv_cse_98, __rspice_deriv_cse_99, __rspice_deriv_cse_100, __rspice_deriv_cse_101, __rspice_deriv_cse_102, __rspice_deriv_cse_103, eq68_e2119_q, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_reactive_node_derivatives: [f64; 14] = [eq68_e2121_q_d_n0, eq68_e2121_q_d_n1, eq68_e2121_q_d_n2, eq68_e2121_q_d_n3, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5, eq68_e2121_q_d_n6, eq68_e2121_q_d_n7, eq68_e2121_q_d_n8, eq68_e2121_q_d_n9, eq68_e2121_q_d_n10, eq68_e2121_q_d_n11, eq68_e2121_q_d_n12, eq68_e2121_q_d_n13];
        let eq68_reactive_branch_derivatives: [f64; 12] = [eq68_e2121_q_d_b0, eq68_e2121_q_d_b1, eq68_e2121_q_d_b2, eq68_e2121_q_d_b3, eq68_e2121_q_d_b4, eq68_e2121_q_d_b5, eq68_e2121_q_d_b6, eq68_e2121_q_d_b7, eq68_e2121_q_d_b8, eq68_e2121_q_d_b9, eq68_e2121_q_d_b10, eq68_e2121_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq68_reactive_node_derivatives,
            branches,
            &eq68_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n1, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_d_n12, eq69_e2137_d_n13, eq69_e2137_d_b0, eq69_e2137_d_b1, eq69_e2137_d_b2, eq69_e2137_d_b3, eq69_e2137_d_b4, eq69_e2137_d_b5, eq69_e2137_d_b6, eq69_e2137_d_b7, eq69_e2137_d_b8, eq69_e2137_d_b9, eq69_e2137_d_b10, eq69_e2137_d_b11, eq69_e2137_q, eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13, eq69_e2137_q_d_b0, eq69_e2137_q_d_b1, eq69_e2137_q_d_b2, eq69_e2137_q_d_b3, eq69_e2137_q_d_b4, eq69_e2137_q_d_b5, eq69_e2137_q_d_b6, eq69_e2137_q_d_b7, eq69_e2137_q_d_b8, eq69_e2137_q_d_b9, eq69_e2137_q_d_b10, eq69_e2137_q_d_b11,) = {
    if (s.b[2021] && (!s.b[2024])) {
        let eq69_e2128: f64 = (s.v[634] * s.v[1015]);
        let eq69_e2128_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));
        let eq69_e2128_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));
        let eq69_e2128_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));
        let eq69_e2128_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));
        let eq69_e2128_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));
        let eq69_e2128_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));
        let eq69_e2128_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));
        let eq69_e2128_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));
        let eq69_e2128_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));
        let eq69_e2128_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));
        let eq69_e2128_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));
        let eq69_e2128_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));
        let eq69_e2128_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));
        let eq69_e2128_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));
        let eq69_e2128_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));
        let eq69_e2128_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));
        let eq69_e2128_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));
        let eq69_e2128_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));
        let eq69_e2128_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));
        let eq69_e2128_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));
        let eq69_e2128_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));
        let eq69_e2128_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));
        let eq69_e2128_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));
        let eq69_e2128_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));
        let eq69_e2128_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));
        let eq69_e2128_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));
        let eq69_e2131: f64 = (s.v[634] * s.v[1016]);
        let eq69_e2131_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));
        let eq69_e2131_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));
        let eq69_e2131_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));
        let eq69_e2131_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));
        let eq69_e2131_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));
        let eq69_e2131_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));
        let eq69_e2131_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));
        let eq69_e2131_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));
        let eq69_e2131_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));
        let eq69_e2131_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));
        let eq69_e2131_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));
        let eq69_e2131_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));
        let eq69_e2131_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));
        let eq69_e2131_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));
        let eq69_e2131_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));
        let eq69_e2131_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));
        let eq69_e2131_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));
        let eq69_e2131_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));
        let eq69_e2131_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));
        let eq69_e2131_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq69_e2131_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));
        let eq69_e2131_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));
        let eq69_e2131_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));
        let eq69_e2131_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));
        let eq69_e2131_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));
        let eq69_e2131_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));
        let eq69_e2132_q: f64 = eq69_e2131;
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2131);
        let eq69_e2133_d_n0: f64 = (eq69_e2128_d_n0 + eq69_e2131_d_n0);
        let eq69_e2133_d_n1: f64 = (eq69_e2128_d_n1 + eq69_e2131_d_n1);
        let eq69_e2133_d_n2: f64 = (eq69_e2128_d_n2 + eq69_e2131_d_n2);
        let eq69_e2133_d_n3: f64 = (eq69_e2128_d_n3 + eq69_e2131_d_n3);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2131_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2131_d_n5);
        let eq69_e2133_d_n6: f64 = (eq69_e2128_d_n6 + eq69_e2131_d_n6);
        let eq69_e2133_d_n7: f64 = (eq69_e2128_d_n7 + eq69_e2131_d_n7);
        let eq69_e2133_d_n8: f64 = (eq69_e2128_d_n8 + eq69_e2131_d_n8);
        let eq69_e2133_d_n9: f64 = (eq69_e2128_d_n9 + eq69_e2131_d_n9);
        let eq69_e2133_d_n10: f64 = (eq69_e2128_d_n10 + eq69_e2131_d_n10);
        let eq69_e2133_d_n11: f64 = (eq69_e2128_d_n11 + eq69_e2131_d_n11);
        let eq69_e2133_d_n12: f64 = (eq69_e2128_d_n12 + eq69_e2131_d_n12);
        let eq69_e2133_d_n13: f64 = (eq69_e2128_d_n13 + eq69_e2131_d_n13);
        let eq69_e2133_d_b0: f64 = (eq69_e2128_d_b0 + eq69_e2131_d_b0);
        let eq69_e2133_d_b1: f64 = (eq69_e2128_d_b1 + eq69_e2131_d_b1);
        let eq69_e2133_d_b2: f64 = (eq69_e2128_d_b2 + eq69_e2131_d_b2);
        let eq69_e2133_d_b3: f64 = (eq69_e2128_d_b3 + eq69_e2131_d_b3);
        let eq69_e2133_d_b4: f64 = (eq69_e2128_d_b4 + eq69_e2131_d_b4);
        let eq69_e2133_d_b5: f64 = (eq69_e2128_d_b5 + eq69_e2131_d_b5);
        let eq69_e2133_d_b6: f64 = (eq69_e2128_d_b6 + eq69_e2131_d_b6);
        let eq69_e2133_d_b7: f64 = (eq69_e2128_d_b7 + eq69_e2131_d_b7);
        let eq69_e2133_d_b8: f64 = (eq69_e2128_d_b8 + eq69_e2131_d_b8);
        let eq69_e2133_d_b9: f64 = (eq69_e2128_d_b9 + eq69_e2131_d_b9);
        let eq69_e2133_d_b10: f64 = (eq69_e2128_d_b10 + eq69_e2131_d_b10);
        let eq69_e2133_d_b11: f64 = (eq69_e2128_d_b11 + eq69_e2131_d_b11);
        let eq69_e2133_q: f64 = eq69_e2132_q;
        let eq69_e2135: f64 = (eq69_e2133 - s.v[1017]);
        let eq69_e2135_d_n0: f64 = (eq69_e2133_d_n0 - s.dn[1017][0]);
        let eq69_e2135_d_n1: f64 = (eq69_e2133_d_n1 - s.dn[1017][1]);
        let eq69_e2135_d_n2: f64 = (eq69_e2133_d_n2 - s.dn[1017][2]);
        let eq69_e2135_d_n3: f64 = (eq69_e2133_d_n3 - s.dn[1017][3]);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - s.dn[1017][4]);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - s.dn[1017][5]);
        let eq69_e2135_d_n6: f64 = (eq69_e2133_d_n6 - s.dn[1017][6]);
        let eq69_e2135_d_n7: f64 = (eq69_e2133_d_n7 - s.dn[1017][7]);
        let eq69_e2135_d_n8: f64 = (eq69_e2133_d_n8 - s.dn[1017][8]);
        let eq69_e2135_d_n9: f64 = (eq69_e2133_d_n9 - s.dn[1017][9]);
        let eq69_e2135_d_n10: f64 = (eq69_e2133_d_n10 - s.dn[1017][10]);
        let eq69_e2135_d_n11: f64 = (eq69_e2133_d_n11 - s.dn[1017][11]);
        let eq69_e2135_d_n12: f64 = (eq69_e2133_d_n12 - s.dn[1017][12]);
        let eq69_e2135_d_n13: f64 = (eq69_e2133_d_n13 - s.dn[1017][13]);
        let eq69_e2135_d_b0: f64 = (eq69_e2133_d_b0 - s.db[1017][0]);
        let eq69_e2135_d_b1: f64 = (eq69_e2133_d_b1 - s.db[1017][1]);
        let eq69_e2135_d_b2: f64 = (eq69_e2133_d_b2 - s.db[1017][2]);
        let eq69_e2135_d_b3: f64 = (eq69_e2133_d_b3 - s.db[1017][3]);
        let eq69_e2135_d_b4: f64 = (eq69_e2133_d_b4 - s.db[1017][4]);
        let eq69_e2135_d_b5: f64 = (eq69_e2133_d_b5 - s.db[1017][5]);
        let eq69_e2135_d_b6: f64 = (eq69_e2133_d_b6 - s.db[1017][6]);
        let eq69_e2135_d_b7: f64 = (eq69_e2133_d_b7 - s.db[1017][7]);
        let eq69_e2135_d_b8: f64 = (eq69_e2133_d_b8 - s.db[1017][8]);
        let eq69_e2135_d_b9: f64 = (eq69_e2133_d_b9 - s.db[1017][9]);
        let eq69_e2135_d_b10: f64 = (eq69_e2133_d_b10 - s.db[1017][10]);
        let eq69_e2135_d_b11: f64 = (eq69_e2133_d_b11 - s.db[1017][11]);
        let eq69_e2135_q: f64 = eq69_e2133_q;
        (eq69_e2135, eq69_e2135_d_n0, eq69_e2135_d_n1, eq69_e2135_d_n2, eq69_e2135_d_n3, eq69_e2135_d_n4, eq69_e2135_d_n5, eq69_e2135_d_n6, eq69_e2135_d_n7, eq69_e2135_d_n8, eq69_e2135_d_n9, eq69_e2135_d_n10, eq69_e2135_d_n11, eq69_e2135_d_n12, eq69_e2135_d_n13, eq69_e2135_d_b0, eq69_e2135_d_b1, eq69_e2135_d_b2, eq69_e2135_d_b3, eq69_e2135_d_b4, eq69_e2135_d_b5, eq69_e2135_d_b6, eq69_e2135_d_b7, eq69_e2135_d_b8, eq69_e2135_d_b9, eq69_e2135_d_b10, eq69_e2135_d_b11, eq69_e2135_q, eq69_e2131_d_n0, eq69_e2131_d_n1, eq69_e2131_d_n2, eq69_e2131_d_n3, eq69_e2131_d_n4, eq69_e2131_d_n5, eq69_e2131_d_n6, eq69_e2131_d_n7, eq69_e2131_d_n8, eq69_e2131_d_n9, eq69_e2131_d_n10, eq69_e2131_d_n11, eq69_e2131_d_n12, eq69_e2131_d_n13, eq69_e2131_d_b0, eq69_e2131_d_b1, eq69_e2131_d_b2, eq69_e2131_d_b3, eq69_e2131_d_b4, eq69_e2131_d_b5, eq69_e2131_d_b6, eq69_e2131_d_b7, eq69_e2131_d_b8, eq69_e2131_d_b9, eq69_e2131_d_b10, eq69_e2131_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 14] = [eq69_e2137_q_d_n0, eq69_e2137_q_d_n1, eq69_e2137_q_d_n2, eq69_e2137_q_d_n3, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5, eq69_e2137_q_d_n6, eq69_e2137_q_d_n7, eq69_e2137_q_d_n8, eq69_e2137_q_d_n9, eq69_e2137_q_d_n10, eq69_e2137_q_d_n11, eq69_e2137_q_d_n12, eq69_e2137_q_d_n13];
        let eq69_reactive_branch_derivatives: [f64; 12] = [eq69_e2137_q_d_b0, eq69_e2137_q_d_b1, eq69_e2137_q_d_b2, eq69_e2137_q_d_b3, eq69_e2137_q_d_b4, eq69_e2137_q_d_b5, eq69_e2137_q_d_b6, eq69_e2137_q_d_b7, eq69_e2137_q_d_b8, eq69_e2137_q_d_b9, eq69_e2137_q_d_b10, eq69_e2137_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let eq80_e2212_q: f64 = s.v[520];
        let eq80_e2213: f64 = (s.v[379] * s.v[520]);
        let eq80_e2213_d_n0: f64 = ((s.dn[379][0] * s.v[520]) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_d_n1: f64 = ((s.dn[379][1] * s.v[520]) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_d_n2: f64 = ((s.dn[379][2] * s.v[520]) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_d_n3: f64 = ((s.dn[379][3] * s.v[520]) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_d_n4: f64 = ((s.dn[379][4] * s.v[520]) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_d_n5: f64 = ((s.dn[379][5] * s.v[520]) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_d_n6: f64 = ((s.dn[379][6] * s.v[520]) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_d_n7: f64 = ((s.dn[379][7] * s.v[520]) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_d_n8: f64 = ((s.dn[379][8] * s.v[520]) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_d_n9: f64 = ((s.dn[379][9] * s.v[520]) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_d_n10: f64 = ((s.dn[379][10] * s.v[520]) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_d_n11: f64 = ((s.dn[379][11] * s.v[520]) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_d_n12: f64 = ((s.dn[379][12] * s.v[520]) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_d_n13: f64 = ((s.dn[379][13] * s.v[520]) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_d_b0: f64 = ((s.db[379][0] * s.v[520]) + (s.v[379] * s.db[520][0]));
        let eq80_e2213_d_b1: f64 = ((s.db[379][1] * s.v[520]) + (s.v[379] * s.db[520][1]));
        let eq80_e2213_d_b2: f64 = ((s.db[379][2] * s.v[520]) + (s.v[379] * s.db[520][2]));
        let eq80_e2213_d_b3: f64 = ((s.db[379][3] * s.v[520]) + (s.v[379] * s.db[520][3]));
        let eq80_e2213_d_b4: f64 = ((s.db[379][4] * s.v[520]) + (s.v[379] * s.db[520][4]));
        let eq80_e2213_d_b5: f64 = ((s.db[379][5] * s.v[520]) + (s.v[379] * s.db[520][5]));
        let eq80_e2213_d_b6: f64 = ((s.db[379][6] * s.v[520]) + (s.v[379] * s.db[520][6]));
        let eq80_e2213_d_b7: f64 = ((s.db[379][7] * s.v[520]) + (s.v[379] * s.db[520][7]));
        let eq80_e2213_d_b8: f64 = ((s.db[379][8] * s.v[520]) + (s.v[379] * s.db[520][8]));
        let eq80_e2213_d_b9: f64 = ((s.db[379][9] * s.v[520]) + (s.v[379] * s.db[520][9]));
        let eq80_e2213_d_b10: f64 = ((s.db[379][10] * s.v[520]) + (s.v[379] * s.db[520][10]));
        let eq80_e2213_d_b11: f64 = ((s.db[379][11] * s.v[520]) + (s.v[379] * s.db[520][11]));
        let eq80_e2213_q: f64 = (s.v[379] * eq80_e2212_q);
        let eq80_e2213_q_d_n0: f64 = ((s.dn[379][0] * eq80_e2212_q) + (s.v[379] * s.dn[520][0]));
        let eq80_e2213_q_d_n1: f64 = ((s.dn[379][1] * eq80_e2212_q) + (s.v[379] * s.dn[520][1]));
        let eq80_e2213_q_d_n2: f64 = ((s.dn[379][2] * eq80_e2212_q) + (s.v[379] * s.dn[520][2]));
        let eq80_e2213_q_d_n3: f64 = ((s.dn[379][3] * eq80_e2212_q) + (s.v[379] * s.dn[520][3]));
        let eq80_e2213_q_d_n4: f64 = ((s.dn[379][4] * eq80_e2212_q) + (s.v[379] * s.dn[520][4]));
        let eq80_e2213_q_d_n5: f64 = ((s.dn[379][5] * eq80_e2212_q) + (s.v[379] * s.dn[520][5]));
        let eq80_e2213_q_d_n6: f64 = ((s.dn[379][6] * eq80_e2212_q) + (s.v[379] * s.dn[520][6]));
        let eq80_e2213_q_d_n7: f64 = ((s.dn[379][7] * eq80_e2212_q) + (s.v[379] * s.dn[520][7]));
        let eq80_e2213_q_d_n8: f64 = ((s.dn[379][8] * eq80_e2212_q) + (s.v[379] * s.dn[520][8]));
        let eq80_e2213_q_d_n9: f64 = ((s.dn[379][9] * eq80_e2212_q) + (s.v[379] * s.dn[520][9]));
        let eq80_e2213_q_d_n10: f64 = ((s.dn[379][10] * eq80_e2212_q) + (s.v[379] * s.dn[520][10]));
        let eq80_e2213_q_d_n11: f64 = ((s.dn[379][11] * eq80_e2212_q) + (s.v[379] * s.dn[520][11]));
        let eq80_e2213_q_d_n12: f64 = ((s.dn[379][12] * eq80_e2212_q) + (s.v[379] * s.dn[520][12]));
        let eq80_e2213_q_d_n13: f64 = ((s.dn[379][13] * eq80_e2212_q) + (s.v[379] * s.dn[520][13]));
        let eq80_e2213_q_d_b0: f64 = ((s.db[379][0] * eq80_e2212_q) + (s.v[379] * s.db[520][0]));
        let eq80_e2213_q_d_b1: f64 = ((s.db[379][1] * eq80_e2212_q) + (s.v[379] * s.db[520][1]));
        let eq80_e2213_q_d_b2: f64 = ((s.db[379][2] * eq80_e2212_q) + (s.v[379] * s.db[520][2]));
        let eq80_e2213_q_d_b3: f64 = ((s.db[379][3] * eq80_e2212_q) + (s.v[379] * s.db[520][3]));
        let eq80_e2213_q_d_b4: f64 = ((s.db[379][4] * eq80_e2212_q) + (s.v[379] * s.db[520][4]));
        let eq80_e2213_q_d_b5: f64 = ((s.db[379][5] * eq80_e2212_q) + (s.v[379] * s.db[520][5]));
        let eq80_e2213_q_d_b6: f64 = ((s.db[379][6] * eq80_e2212_q) + (s.v[379] * s.db[520][6]));
        let eq80_e2213_q_d_b7: f64 = ((s.db[379][7] * eq80_e2212_q) + (s.v[379] * s.db[520][7]));
        let eq80_e2213_q_d_b8: f64 = ((s.db[379][8] * eq80_e2212_q) + (s.v[379] * s.db[520][8]));
        let eq80_e2213_q_d_b9: f64 = ((s.db[379][9] * eq80_e2212_q) + (s.v[379] * s.db[520][9]));
        let eq80_e2213_q_d_b10: f64 = ((s.db[379][10] * eq80_e2212_q) + (s.v[379] * s.db[520][10]));
        let eq80_e2213_q_d_b11: f64 = ((s.db[379][11] * eq80_e2212_q) + (s.v[379] * s.db[520][11]));
        let eq80_reactive_node_derivatives: [f64; 14] = [eq80_e2213_q_d_n0, eq80_e2213_q_d_n1, eq80_e2213_q_d_n2, eq80_e2213_q_d_n3, eq80_e2213_q_d_n4, eq80_e2213_q_d_n5, eq80_e2213_q_d_n6, eq80_e2213_q_d_n7, eq80_e2213_q_d_n8, eq80_e2213_q_d_n9, eq80_e2213_q_d_n10, eq80_e2213_q_d_n11, eq80_e2213_q_d_n12, eq80_e2213_q_d_n13];
        let eq80_reactive_branch_derivatives: [f64; 12] = [eq80_e2213_q_d_b0, eq80_e2213_q_d_b1, eq80_e2213_q_d_b2, eq80_e2213_q_d_b3, eq80_e2213_q_d_b4, eq80_e2213_q_d_b5, eq80_e2213_q_d_b6, eq80_e2213_q_d_b7, eq80_e2213_q_d_b8, eq80_e2213_q_d_b9, eq80_e2213_q_d_b10, eq80_e2213_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq80_reactive_node_derivatives,
            branches,
            &eq80_reactive_branch_derivatives,
            multiplicity,
        );
        let eq81_e2216_q: f64 = s.v[525];
        let eq81_e2217: f64 = (s.v[379] * s.v[525]);
        let eq81_e2217_d_n0: f64 = ((s.dn[379][0] * s.v[525]) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_d_n1: f64 = ((s.dn[379][1] * s.v[525]) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_d_n2: f64 = ((s.dn[379][2] * s.v[525]) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_d_n3: f64 = ((s.dn[379][3] * s.v[525]) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_d_n4: f64 = ((s.dn[379][4] * s.v[525]) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_d_n5: f64 = ((s.dn[379][5] * s.v[525]) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_d_n6: f64 = ((s.dn[379][6] * s.v[525]) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_d_n7: f64 = ((s.dn[379][7] * s.v[525]) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_d_n8: f64 = ((s.dn[379][8] * s.v[525]) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_d_n9: f64 = ((s.dn[379][9] * s.v[525]) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_d_n10: f64 = ((s.dn[379][10] * s.v[525]) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_d_n11: f64 = ((s.dn[379][11] * s.v[525]) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_d_n12: f64 = ((s.dn[379][12] * s.v[525]) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_d_n13: f64 = ((s.dn[379][13] * s.v[525]) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_d_b0: f64 = ((s.db[379][0] * s.v[525]) + (s.v[379] * s.db[525][0]));
        let eq81_e2217_d_b1: f64 = ((s.db[379][1] * s.v[525]) + (s.v[379] * s.db[525][1]));
        let eq81_e2217_d_b2: f64 = ((s.db[379][2] * s.v[525]) + (s.v[379] * s.db[525][2]));
        let eq81_e2217_d_b3: f64 = ((s.db[379][3] * s.v[525]) + (s.v[379] * s.db[525][3]));
        let eq81_e2217_d_b4: f64 = ((s.db[379][4] * s.v[525]) + (s.v[379] * s.db[525][4]));
        let eq81_e2217_d_b5: f64 = ((s.db[379][5] * s.v[525]) + (s.v[379] * s.db[525][5]));
        let eq81_e2217_d_b6: f64 = ((s.db[379][6] * s.v[525]) + (s.v[379] * s.db[525][6]));
        let eq81_e2217_d_b7: f64 = ((s.db[379][7] * s.v[525]) + (s.v[379] * s.db[525][7]));
        let eq81_e2217_d_b8: f64 = ((s.db[379][8] * s.v[525]) + (s.v[379] * s.db[525][8]));
        let eq81_e2217_d_b9: f64 = ((s.db[379][9] * s.v[525]) + (s.v[379] * s.db[525][9]));
        let eq81_e2217_d_b10: f64 = ((s.db[379][10] * s.v[525]) + (s.v[379] * s.db[525][10]));
        let eq81_e2217_d_b11: f64 = ((s.db[379][11] * s.v[525]) + (s.v[379] * s.db[525][11]));
        let eq81_e2217_q: f64 = (s.v[379] * eq81_e2216_q);
        let eq81_e2217_q_d_n0: f64 = ((s.dn[379][0] * eq81_e2216_q) + (s.v[379] * s.dn[525][0]));
        let eq81_e2217_q_d_n1: f64 = ((s.dn[379][1] * eq81_e2216_q) + (s.v[379] * s.dn[525][1]));
        let eq81_e2217_q_d_n2: f64 = ((s.dn[379][2] * eq81_e2216_q) + (s.v[379] * s.dn[525][2]));
        let eq81_e2217_q_d_n3: f64 = ((s.dn[379][3] * eq81_e2216_q) + (s.v[379] * s.dn[525][3]));
        let eq81_e2217_q_d_n4: f64 = ((s.dn[379][4] * eq81_e2216_q) + (s.v[379] * s.dn[525][4]));
        let eq81_e2217_q_d_n5: f64 = ((s.dn[379][5] * eq81_e2216_q) + (s.v[379] * s.dn[525][5]));
        let eq81_e2217_q_d_n6: f64 = ((s.dn[379][6] * eq81_e2216_q) + (s.v[379] * s.dn[525][6]));
        let eq81_e2217_q_d_n7: f64 = ((s.dn[379][7] * eq81_e2216_q) + (s.v[379] * s.dn[525][7]));
        let eq81_e2217_q_d_n8: f64 = ((s.dn[379][8] * eq81_e2216_q) + (s.v[379] * s.dn[525][8]));
        let eq81_e2217_q_d_n9: f64 = ((s.dn[379][9] * eq81_e2216_q) + (s.v[379] * s.dn[525][9]));
        let eq81_e2217_q_d_n10: f64 = ((s.dn[379][10] * eq81_e2216_q) + (s.v[379] * s.dn[525][10]));
        let eq81_e2217_q_d_n11: f64 = ((s.dn[379][11] * eq81_e2216_q) + (s.v[379] * s.dn[525][11]));
        let eq81_e2217_q_d_n12: f64 = ((s.dn[379][12] * eq81_e2216_q) + (s.v[379] * s.dn[525][12]));
        let eq81_e2217_q_d_n13: f64 = ((s.dn[379][13] * eq81_e2216_q) + (s.v[379] * s.dn[525][13]));
        let eq81_e2217_q_d_b0: f64 = ((s.db[379][0] * eq81_e2216_q) + (s.v[379] * s.db[525][0]));
        let eq81_e2217_q_d_b1: f64 = ((s.db[379][1] * eq81_e2216_q) + (s.v[379] * s.db[525][1]));
        let eq81_e2217_q_d_b2: f64 = ((s.db[379][2] * eq81_e2216_q) + (s.v[379] * s.db[525][2]));
        let eq81_e2217_q_d_b3: f64 = ((s.db[379][3] * eq81_e2216_q) + (s.v[379] * s.db[525][3]));
        let eq81_e2217_q_d_b4: f64 = ((s.db[379][4] * eq81_e2216_q) + (s.v[379] * s.db[525][4]));
        let eq81_e2217_q_d_b5: f64 = ((s.db[379][5] * eq81_e2216_q) + (s.v[379] * s.db[525][5]));
        let eq81_e2217_q_d_b6: f64 = ((s.db[379][6] * eq81_e2216_q) + (s.v[379] * s.db[525][6]));
        let eq81_e2217_q_d_b7: f64 = ((s.db[379][7] * eq81_e2216_q) + (s.v[379] * s.db[525][7]));
        let eq81_e2217_q_d_b8: f64 = ((s.db[379][8] * eq81_e2216_q) + (s.v[379] * s.db[525][8]));
        let eq81_e2217_q_d_b9: f64 = ((s.db[379][9] * eq81_e2216_q) + (s.v[379] * s.db[525][9]));
        let eq81_e2217_q_d_b10: f64 = ((s.db[379][10] * eq81_e2216_q) + (s.v[379] * s.db[525][10]));
        let eq81_e2217_q_d_b11: f64 = ((s.db[379][11] * eq81_e2216_q) + (s.v[379] * s.db[525][11]));
        let eq81_reactive_node_derivatives: [f64; 14] = [eq81_e2217_q_d_n0, eq81_e2217_q_d_n1, eq81_e2217_q_d_n2, eq81_e2217_q_d_n3, eq81_e2217_q_d_n4, eq81_e2217_q_d_n5, eq81_e2217_q_d_n6, eq81_e2217_q_d_n7, eq81_e2217_q_d_n8, eq81_e2217_q_d_n9, eq81_e2217_q_d_n10, eq81_e2217_q_d_n11, eq81_e2217_q_d_n12, eq81_e2217_q_d_n13];
        let eq81_reactive_branch_derivatives: [f64; 12] = [eq81_e2217_q_d_b0, eq81_e2217_q_d_b1, eq81_e2217_q_d_b2, eq81_e2217_q_d_b3, eq81_e2217_q_d_b4, eq81_e2217_q_d_b5, eq81_e2217_q_d_b6, eq81_e2217_q_d_b7, eq81_e2217_q_d_b8, eq81_e2217_q_d_b9, eq81_e2217_q_d_b10, eq81_e2217_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq81_reactive_node_derivatives,
            branches,
            &eq81_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
