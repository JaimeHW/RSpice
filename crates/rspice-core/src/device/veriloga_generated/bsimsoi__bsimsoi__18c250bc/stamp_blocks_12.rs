#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq11_e1314, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let eq11_e1308: f64 = (p.p32 * (nv13 - 0.0));
        let eq11_e1310: f64 = (eq11_e1308 * locals.var_sf);
        let eq11_e1310_d_n3: f64 = (eq11_e1308 * locals.var_sf_dn3);
        let eq11_e1310_d_n4: f64 = (eq11_e1308 * locals.var_sf_dn4);
        let eq11_e1310_d_n5: f64 = (eq11_e1308 * locals.var_sf_dn5);
        let eq11_e1310_d_n6: f64 = (eq11_e1308 * locals.var_sf_dn6);
        let eq11_e1310_d_n7: f64 = (eq11_e1308 * locals.var_sf_dn7);
        let eq11_e1310_d_n8: f64 = (eq11_e1308 * locals.var_sf_dn8);
        let eq11_e1310_d_n9: f64 = (eq11_e1308 * locals.var_sf_dn9);
        let eq11_e1310_d_n10: f64 = (eq11_e1308 * locals.var_sf_dn10);
        let eq11_e1310_d_n11: f64 = (eq11_e1308 * locals.var_sf_dn11);
        let eq11_e1310_d_n12: f64 = (eq11_e1308 * locals.var_sf_dn12);
        let eq11_e1310_d_n13: f64 = (p.p32 * locals.var_sf);
        let eq11_e1312: f64 = (eq11_e1310 * p.p226);
        let eq11_e1312_d_n3: f64 = (eq11_e1310_d_n3 * p.p226);
        let eq11_e1312_d_n4: f64 = (eq11_e1310_d_n4 * p.p226);
        let eq11_e1312_d_n5: f64 = (eq11_e1310_d_n5 * p.p226);
        let eq11_e1312_d_n6: f64 = (eq11_e1310_d_n6 * p.p226);
        let eq11_e1312_d_n7: f64 = (eq11_e1310_d_n7 * p.p226);
        let eq11_e1312_d_n8: f64 = (eq11_e1310_d_n8 * p.p226);
        let eq11_e1312_d_n9: f64 = (eq11_e1310_d_n9 * p.p226);
        let eq11_e1312_d_n10: f64 = (eq11_e1310_d_n10 * p.p226);
        let eq11_e1312_d_n11: f64 = (eq11_e1310_d_n11 * p.p226);
        let eq11_e1312_d_n12: f64 = (eq11_e1310_d_n12 * p.p226);
        let eq11_e1312_d_n13: f64 = (eq11_e1310_d_n13 * p.p226);
        (eq11_e1312, eq11_e1312_d_n3, eq11_e1312_d_n4, eq11_e1312_d_n5, eq11_e1312_d_n6, eq11_e1312_d_n7, eq11_e1312_d_n8, eq11_e1312_d_n9, eq11_e1312_d_n10, eq11_e1312_d_n11, eq11_e1312_d_n12, eq11_e1312_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1314;
        let eq11_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq11_node_derivatives: [f64; 11] = [eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e1356, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let eq13_e1348: f64 = (p.p32 * locals.var_ctnoi);
        let eq13_e1348_d_n3: f64 = (p.p32 * locals.var_ctnoi_dn3);
        let eq13_e1348_d_n4: f64 = (p.p32 * locals.var_ctnoi_dn4);
        let eq13_e1348_d_n5: f64 = (p.p32 * locals.var_ctnoi_dn5);
        let eq13_e1348_d_n6: f64 = (p.p32 * locals.var_ctnoi_dn6);
        let eq13_e1348_d_n7: f64 = (p.p32 * locals.var_ctnoi_dn7);
        let eq13_e1348_d_n8: f64 = (p.p32 * locals.var_ctnoi_dn8);
        let eq13_e1348_d_n9: f64 = (p.p32 * locals.var_ctnoi_dn9);
        let eq13_e1348_d_n10: f64 = (p.p32 * locals.var_ctnoi_dn10);
        let eq13_e1348_d_n11: f64 = (p.p32 * locals.var_ctnoi_dn11);
        let eq13_e1348_d_n12: f64 = (p.p32 * locals.var_ctnoi_dn12);
        let eq13_e1350: f64 = (eq13_e1348 * (nv13 - 0.0));
        let eq13_e1350_d_n3: f64 = (eq13_e1348_d_n3 * (nv13 - 0.0));
        let eq13_e1350_d_n4: f64 = (eq13_e1348_d_n4 * (nv13 - 0.0));
        let eq13_e1350_d_n5: f64 = (eq13_e1348_d_n5 * (nv13 - 0.0));
        let eq13_e1350_d_n6: f64 = (eq13_e1348_d_n6 * (nv13 - 0.0));
        let eq13_e1350_d_n7: f64 = (eq13_e1348_d_n7 * (nv13 - 0.0));
        let eq13_e1350_d_n8: f64 = (eq13_e1348_d_n8 * (nv13 - 0.0));
        let eq13_e1350_d_n9: f64 = (eq13_e1348_d_n9 * (nv13 - 0.0));
        let eq13_e1350_d_n10: f64 = (eq13_e1348_d_n10 * (nv13 - 0.0));
        let eq13_e1350_d_n11: f64 = (eq13_e1348_d_n11 * (nv13 - 0.0));
        let eq13_e1350_d_n12: f64 = (eq13_e1348_d_n12 * (nv13 - 0.0));
        let eq13_e1352: f64 = (eq13_e1350 * locals.var_sf);
        let eq13_e1352_d_n3: f64 = ((eq13_e1350_d_n3 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn3));
        let eq13_e1352_d_n4: f64 = ((eq13_e1350_d_n4 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn4));
        let eq13_e1352_d_n5: f64 = ((eq13_e1350_d_n5 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn5));
        let eq13_e1352_d_n6: f64 = ((eq13_e1350_d_n6 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn6));
        let eq13_e1352_d_n7: f64 = ((eq13_e1350_d_n7 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn7));
        let eq13_e1352_d_n8: f64 = ((eq13_e1350_d_n8 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn8));
        let eq13_e1352_d_n9: f64 = ((eq13_e1350_d_n9 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn9));
        let eq13_e1352_d_n10: f64 = ((eq13_e1350_d_n10 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn10));
        let eq13_e1352_d_n11: f64 = ((eq13_e1350_d_n11 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn11));
        let eq13_e1352_d_n12: f64 = ((eq13_e1350_d_n12 * locals.var_sf) + (eq13_e1350 * locals.var_sf_dn12));
        let eq13_e1352_d_n13: f64 = (eq13_e1348 * locals.var_sf);
        let eq13_e1354: f64 = (eq13_e1352 * p.p226);
        let eq13_e1354_d_n3: f64 = (eq13_e1352_d_n3 * p.p226);
        let eq13_e1354_d_n4: f64 = (eq13_e1352_d_n4 * p.p226);
        let eq13_e1354_d_n5: f64 = (eq13_e1352_d_n5 * p.p226);
        let eq13_e1354_d_n6: f64 = (eq13_e1352_d_n6 * p.p226);
        let eq13_e1354_d_n7: f64 = (eq13_e1352_d_n7 * p.p226);
        let eq13_e1354_d_n8: f64 = (eq13_e1352_d_n8 * p.p226);
        let eq13_e1354_d_n9: f64 = (eq13_e1352_d_n9 * p.p226);
        let eq13_e1354_d_n10: f64 = (eq13_e1352_d_n10 * p.p226);
        let eq13_e1354_d_n11: f64 = (eq13_e1352_d_n11 * p.p226);
        let eq13_e1354_d_n12: f64 = (eq13_e1352_d_n12 * p.p226);
        let eq13_e1354_d_n13: f64 = (eq13_e1352_d_n13 * p.p226);
        (eq13_e1354, eq13_e1354_d_n3, eq13_e1354_d_n4, eq13_e1354_d_n5, eq13_e1354_d_n6, eq13_e1354_d_n7, eq13_e1354_d_n8, eq13_e1354_d_n9, eq13_e1354_d_n10, eq13_e1354_d_n11, eq13_e1354_d_n12, eq13_e1354_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1356;
        let eq13_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq13_node_derivatives: [f64; 11] = [eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq14_e1376, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * locals.var_c0);
        let eq14_e1369_d_n3: f64 = (eq14_e1367 * locals.var_c0_dn3);
        let eq14_e1369_d_n4: f64 = (eq14_e1367 * locals.var_c0_dn4);
        let eq14_e1369_d_n5: f64 = (eq14_e1367 * locals.var_c0_dn5);
        let eq14_e1369_d_n6: f64 = (eq14_e1367 * locals.var_c0_dn6);
        let eq14_e1369_d_n7: f64 = (eq14_e1367 * locals.var_c0_dn7);
        let eq14_e1369_d_n8: f64 = (eq14_e1367 * locals.var_c0_dn8);
        let eq14_e1369_d_n9: f64 = (eq14_e1367 * locals.var_c0_dn9);
        let eq14_e1369_d_n10: f64 = (eq14_e1367 * locals.var_c0_dn10);
        let eq14_e1369_d_n11: f64 = (eq14_e1367 * locals.var_c0_dn11);
        let eq14_e1369_d_n12: f64 = (eq14_e1367 * locals.var_c0_dn12);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n3: f64 = (eq14_e1369_d_n3 * p.p226);
        let eq14_e1371_d_n4: f64 = (eq14_e1369_d_n4 * p.p226);
        let eq14_e1371_d_n5: f64 = (eq14_e1369_d_n5 * p.p226);
        let eq14_e1371_d_n6: f64 = (eq14_e1369_d_n6 * p.p226);
        let eq14_e1371_d_n7: f64 = (eq14_e1369_d_n7 * p.p226);
        let eq14_e1371_d_n8: f64 = (eq14_e1369_d_n8 * p.p226);
        let eq14_e1371_d_n9: f64 = (eq14_e1369_d_n9 * p.p226);
        let eq14_e1371_d_n10: f64 = (eq14_e1369_d_n10 * p.p226);
        let eq14_e1371_d_n11: f64 = (eq14_e1369_d_n11 * p.p226);
        let eq14_e1371_d_n12: f64 = (eq14_e1369_d_n12 * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq14_e1373);
        (eq14_e1374, (eq14_e1373_d_n3 * ddt_scale), (eq14_e1373_d_n4 * ddt_scale), (eq14_e1373_d_n5 * ddt_scale), (eq14_e1373_d_n6 * ddt_scale), (eq14_e1373_d_n7 * ddt_scale), (eq14_e1373_d_n8 * ddt_scale), (eq14_e1373_d_n9 * ddt_scale), (eq14_e1373_d_n10 * ddt_scale), (eq14_e1373_d_n11 * ddt_scale), (eq14_e1373_d_n12 * ddt_scale), (eq14_e1371 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e1376;
        let eq14_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq14_node_derivatives: [f64; 11] = [eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_branch_derivative_indices: [usize; 0] = [];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1396, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * locals.var_c0);
        let eq15_e1389_d_n3: f64 = (eq15_e1387 * locals.var_c0_dn3);
        let eq15_e1389_d_n4: f64 = (eq15_e1387 * locals.var_c0_dn4);
        let eq15_e1389_d_n5: f64 = (eq15_e1387 * locals.var_c0_dn5);
        let eq15_e1389_d_n6: f64 = (eq15_e1387 * locals.var_c0_dn6);
        let eq15_e1389_d_n7: f64 = (eq15_e1387 * locals.var_c0_dn7);
        let eq15_e1389_d_n8: f64 = (eq15_e1387 * locals.var_c0_dn8);
        let eq15_e1389_d_n9: f64 = (eq15_e1387 * locals.var_c0_dn9);
        let eq15_e1389_d_n10: f64 = (eq15_e1387 * locals.var_c0_dn10);
        let eq15_e1389_d_n11: f64 = (eq15_e1387 * locals.var_c0_dn11);
        let eq15_e1389_d_n12: f64 = (eq15_e1387 * locals.var_c0_dn12);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n3: f64 = (eq15_e1389_d_n3 * p.p226);
        let eq15_e1391_d_n4: f64 = (eq15_e1389_d_n4 * p.p226);
        let eq15_e1391_d_n5: f64 = (eq15_e1389_d_n5 * p.p226);
        let eq15_e1391_d_n6: f64 = (eq15_e1389_d_n6 * p.p226);
        let eq15_e1391_d_n7: f64 = (eq15_e1389_d_n7 * p.p226);
        let eq15_e1391_d_n8: f64 = (eq15_e1389_d_n8 * p.p226);
        let eq15_e1391_d_n9: f64 = (eq15_e1389_d_n9 * p.p226);
        let eq15_e1391_d_n10: f64 = (eq15_e1389_d_n10 * p.p226);
        let eq15_e1391_d_n11: f64 = (eq15_e1389_d_n11 * p.p226);
        let eq15_e1391_d_n12: f64 = (eq15_e1389_d_n12 * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq15_e1393);
        (eq15_e1394, (eq15_e1393_d_n3 * ddt_scale), (eq15_e1393_d_n4 * ddt_scale), (eq15_e1393_d_n5 * ddt_scale), (eq15_e1393_d_n6 * ddt_scale), (eq15_e1393_d_n7 * ddt_scale), (eq15_e1393_d_n8 * ddt_scale), (eq15_e1393_d_n9 * ddt_scale), (eq15_e1393_d_n10 * ddt_scale), (eq15_e1393_d_n11 * ddt_scale), (eq15_e1393_d_n12 * ddt_scale), (eq15_e1391 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1396;
        let eq15_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq15_node_derivatives: [f64; 11] = [eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1414, eq18_e1414_d_n0, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12,) = {
    if (locals.var_guard1511 != 0.0) {
        let eq18_e1410: f64 = (p.p32 * (nv0 - nv7));
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_rd;
        let eq18_e1412: f64 = (eq18_e1410 * __rspice_inv_cse_0);
        let eq18_e1412_d_n0: f64 = (p.p32 * __rspice_inv_cse_0);
        let eq18_e1412_d_n3: f64 = (-((eq18_e1410 * locals.var_rd_dn3) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n4: f64 = (-((eq18_e1410 * locals.var_rd_dn4) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n5: f64 = (-((eq18_e1410 * locals.var_rd_dn5) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n6: f64 = (-((eq18_e1410 * locals.var_rd_dn6) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n7: f64 = ((((-p.p32) * locals.var_rd) - (eq18_e1410 * locals.var_rd_dn7)) / (locals.var_rd * locals.var_rd));
        let eq18_e1412_d_n8: f64 = (-((eq18_e1410 * locals.var_rd_dn8) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n9: f64 = (-((eq18_e1410 * locals.var_rd_dn9) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n10: f64 = (-((eq18_e1410 * locals.var_rd_dn10) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n11: f64 = (-((eq18_e1410 * locals.var_rd_dn11) / (locals.var_rd * locals.var_rd)));
        let eq18_e1412_d_n12: f64 = (-((eq18_e1410 * locals.var_rd_dn12) / (locals.var_rd * locals.var_rd)));
        (eq18_e1412, eq18_e1412_d_n0, eq18_e1412_d_n3, eq18_e1412_d_n4, eq18_e1412_d_n5, eq18_e1412_d_n6, eq18_e1412_d_n7, eq18_e1412_d_n8, eq18_e1412_d_n9, eq18_e1412_d_n10, eq18_e1412_d_n11, eq18_e1412_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1414;
        let eq18_node_derivative_indices: [usize; 11] = [0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq18_node_derivatives: [f64; 11] = [eq18_e1414_d_n0, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1438, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12,) = {
    if (locals.var_guard1512 != 0.0) {
        let eq21_e1434: f64 = (p.p32 * (nv2 - nv8));
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_rs;
        let eq21_e1436: f64 = (eq21_e1434 * __rspice_inv_cse_1);
        let eq21_e1436_d_n2: f64 = (p.p32 * __rspice_inv_cse_1);
        let eq21_e1436_d_n3: f64 = (-((eq21_e1434 * locals.var_rs_dn3) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n4: f64 = (-((eq21_e1434 * locals.var_rs_dn4) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n5: f64 = (-((eq21_e1434 * locals.var_rs_dn5) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n6: f64 = (-((eq21_e1434 * locals.var_rs_dn6) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n7: f64 = (-((eq21_e1434 * locals.var_rs_dn7) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n8: f64 = ((((-p.p32) * locals.var_rs) - (eq21_e1434 * locals.var_rs_dn8)) / (locals.var_rs * locals.var_rs));
        let eq21_e1436_d_n9: f64 = (-((eq21_e1434 * locals.var_rs_dn9) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n10: f64 = (-((eq21_e1434 * locals.var_rs_dn10) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n11: f64 = (-((eq21_e1434 * locals.var_rs_dn11) / (locals.var_rs * locals.var_rs)));
        let eq21_e1436_d_n12: f64 = (-((eq21_e1434 * locals.var_rs_dn12) / (locals.var_rs * locals.var_rs)));
        (eq21_e1436, eq21_e1436_d_n2, eq21_e1436_d_n3, eq21_e1436_d_n4, eq21_e1436_d_n5, eq21_e1436_d_n6, eq21_e1436_d_n7, eq21_e1436_d_n8, eq21_e1436_d_n9, eq21_e1436_d_n10, eq21_e1436_d_n11, eq21_e1436_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1438;
        let eq21_node_derivative_indices: [usize; 11] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq21_node_derivatives: [f64; 11] = [eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1472, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12,) = {
    if (locals.var_guard1513 != 0.0) {
        let eq24_e1458: f64 = (p.p37 * p.p32);
        let eq24_e1461: f64 = (locals.var_ids_1 + locals.var_ic_1);
        let eq24_e1461_d_n3: f64 = (locals.var_ids_1_dn3 + locals.var_ic_1_dn3);
        let eq24_e1461_d_n4: f64 = (locals.var_ids_1_dn4 + locals.var_ic_1_dn4);
        let eq24_e1461_d_n5: f64 = (locals.var_ids_1_dn5 + locals.var_ic_1_dn5);
        let eq24_e1461_d_n6: f64 = (locals.var_ids_1_dn6 + locals.var_ic_1_dn6);
        let eq24_e1461_d_n7: f64 = (locals.var_ids_1_dn7 + locals.var_ic_1_dn7);
        let eq24_e1461_d_n8: f64 = (locals.var_ids_1_dn8 + locals.var_ic_1_dn8);
        let eq24_e1461_d_n9: f64 = (locals.var_ids_1_dn9 + locals.var_ic_1_dn9);
        let eq24_e1461_d_n10: f64 = (locals.var_ids_1_dn10 + locals.var_ic_1_dn10);
        let eq24_e1461_d_n11: f64 = (locals.var_ids_1_dn11 + locals.var_ic_1_dn11);
        let eq24_e1461_d_n12: f64 = (locals.var_ids_1_dn12 + locals.var_ic_1_dn12);
        let eq24_e1462: f64 = (eq24_e1458 * eq24_e1461);
        let eq24_e1462_d_n3: f64 = (eq24_e1458 * eq24_e1461_d_n3);
        let eq24_e1462_d_n4: f64 = (eq24_e1458 * eq24_e1461_d_n4);
        let eq24_e1462_d_n5: f64 = (eq24_e1458 * eq24_e1461_d_n5);
        let eq24_e1462_d_n6: f64 = (eq24_e1458 * eq24_e1461_d_n6);
        let eq24_e1462_d_n7: f64 = (eq24_e1458 * eq24_e1461_d_n7);
        let eq24_e1462_d_n8: f64 = (eq24_e1458 * eq24_e1461_d_n8);
        let eq24_e1462_d_n9: f64 = (eq24_e1458 * eq24_e1461_d_n9);
        let eq24_e1462_d_n10: f64 = (eq24_e1458 * eq24_e1461_d_n10);
        let eq24_e1462_d_n11: f64 = (eq24_e1458 * eq24_e1461_d_n11);
        let eq24_e1462_d_n12: f64 = (eq24_e1458 * eq24_e1461_d_n12);
        let eq24_e1466: f64 = 0.0;
        let eq24_e1468: f64 = (eq24_e1466 * (nv7 - nv8));
        let eq24_e1469: f64 = (p.p32 * eq24_e1468);
        let eq24_e1469_d_n7: f64 = (p.p32 * eq24_e1466);
        let eq24_e1469_d_n8: f64 = (p.p32 * (-eq24_e1466));
        let eq24_e1470: f64 = (eq24_e1462 + eq24_e1469);
        let eq24_e1470_d_n7: f64 = (eq24_e1462_d_n7 + eq24_e1469_d_n7);
        let eq24_e1470_d_n8: f64 = (eq24_e1462_d_n8 + eq24_e1469_d_n8);
        (eq24_e1470, eq24_e1462_d_n3, eq24_e1462_d_n4, eq24_e1462_d_n5, eq24_e1462_d_n6, eq24_e1470_d_n7, eq24_e1470_d_n8, eq24_e1462_d_n9, eq24_e1462_d_n10, eq24_e1462_d_n11, eq24_e1462_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1472;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq24_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq24_e1472_d_n3), multiplicity * (eq24_e1472_d_n4), multiplicity * (eq24_e1472_d_n5), multiplicity * (eq24_e1472_d_n6), multiplicity * (eq24_e1472_d_n7), multiplicity * (eq24_e1472_d_n8), multiplicity * (eq24_e1472_d_n9), multiplicity * (eq24_e1472_d_n10), multiplicity * (eq24_e1472_d_n11), multiplicity * (eq24_e1472_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq25_e1480, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12,) = {
    if (locals.var_guard1513 != 0.0) {
        let eq25_e1476: f64 = (p.p37 * p.p32);
        let eq25_e1478: f64 = (eq25_e1476 * locals.var_iii);
        let eq25_e1478_d_n3: f64 = (eq25_e1476 * locals.var_iii_dn3);
        let eq25_e1478_d_n4: f64 = (eq25_e1476 * locals.var_iii_dn4);
        let eq25_e1478_d_n5: f64 = (eq25_e1476 * locals.var_iii_dn5);
        let eq25_e1478_d_n6: f64 = (eq25_e1476 * locals.var_iii_dn6);
        let eq25_e1478_d_n7: f64 = (eq25_e1476 * locals.var_iii_dn7);
        let eq25_e1478_d_n8: f64 = (eq25_e1476 * locals.var_iii_dn8);
        let eq25_e1478_d_n9: f64 = (eq25_e1476 * locals.var_iii_dn9);
        let eq25_e1478_d_n10: f64 = (eq25_e1476 * locals.var_iii_dn10);
        let eq25_e1478_d_n11: f64 = (eq25_e1476 * locals.var_iii_dn11);
        let eq25_e1478_d_n12: f64 = (eq25_e1476 * locals.var_iii_dn12);
        (eq25_e1478, eq25_e1478_d_n3, eq25_e1478_d_n4, eq25_e1478_d_n5, eq25_e1478_d_n6, eq25_e1478_d_n7, eq25_e1478_d_n8, eq25_e1478_d_n9, eq25_e1478_d_n10, eq25_e1478_d_n11, eq25_e1478_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1480;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq25_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq25_e1480_d_n3), multiplicity * (eq25_e1480_d_n4), multiplicity * (eq25_e1480_d_n5), multiplicity * (eq25_e1480_d_n6), multiplicity * (eq25_e1480_d_n7), multiplicity * (eq25_e1480_d_n8), multiplicity * (eq25_e1480_d_n9), multiplicity * (eq25_e1480_d_n10), multiplicity * (eq25_e1480_d_n11), multiplicity * (eq25_e1480_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq26_e1499, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12,) = {
    if (locals.var_guard1513 == 0.0) {
        let eq26_e1485: f64 = (p.p37 * p.p32);
        let eq26_e1488: f64 = (locals.var_ids_1 - locals.var_ic_1);
        let eq26_e1488_d_n3: f64 = (locals.var_ids_1_dn3 - locals.var_ic_1_dn3);
        let eq26_e1488_d_n4: f64 = (locals.var_ids_1_dn4 - locals.var_ic_1_dn4);
        let eq26_e1488_d_n5: f64 = (locals.var_ids_1_dn5 - locals.var_ic_1_dn5);
        let eq26_e1488_d_n6: f64 = (locals.var_ids_1_dn6 - locals.var_ic_1_dn6);
        let eq26_e1488_d_n7: f64 = (locals.var_ids_1_dn7 - locals.var_ic_1_dn7);
        let eq26_e1488_d_n8: f64 = (locals.var_ids_1_dn8 - locals.var_ic_1_dn8);
        let eq26_e1488_d_n9: f64 = (locals.var_ids_1_dn9 - locals.var_ic_1_dn9);
        let eq26_e1488_d_n10: f64 = (locals.var_ids_1_dn10 - locals.var_ic_1_dn10);
        let eq26_e1488_d_n11: f64 = (locals.var_ids_1_dn11 - locals.var_ic_1_dn11);
        let eq26_e1488_d_n12: f64 = (locals.var_ids_1_dn12 - locals.var_ic_1_dn12);
        let eq26_e1489: f64 = (eq26_e1485 * eq26_e1488);
        let eq26_e1489_d_n3: f64 = (eq26_e1485 * eq26_e1488_d_n3);
        let eq26_e1489_d_n4: f64 = (eq26_e1485 * eq26_e1488_d_n4);
        let eq26_e1489_d_n5: f64 = (eq26_e1485 * eq26_e1488_d_n5);
        let eq26_e1489_d_n6: f64 = (eq26_e1485 * eq26_e1488_d_n6);
        let eq26_e1489_d_n7: f64 = (eq26_e1485 * eq26_e1488_d_n7);
        let eq26_e1489_d_n8: f64 = (eq26_e1485 * eq26_e1488_d_n8);
        let eq26_e1489_d_n9: f64 = (eq26_e1485 * eq26_e1488_d_n9);
        let eq26_e1489_d_n10: f64 = (eq26_e1485 * eq26_e1488_d_n10);
        let eq26_e1489_d_n11: f64 = (eq26_e1485 * eq26_e1488_d_n11);
        let eq26_e1489_d_n12: f64 = (eq26_e1485 * eq26_e1488_d_n12);
        let eq26_e1493: f64 = 0.0;
        let eq26_e1495: f64 = (eq26_e1493 * (nv8 - nv7));
        let eq26_e1496: f64 = (p.p32 * eq26_e1495);
        let eq26_e1496_d_n7: f64 = (p.p32 * (-eq26_e1493));
        let eq26_e1496_d_n8: f64 = (p.p32 * eq26_e1493);
        let eq26_e1497: f64 = (eq26_e1489 + eq26_e1496);
        let eq26_e1497_d_n7: f64 = (eq26_e1489_d_n7 + eq26_e1496_d_n7);
        let eq26_e1497_d_n8: f64 = (eq26_e1489_d_n8 + eq26_e1496_d_n8);
        (eq26_e1497, eq26_e1489_d_n3, eq26_e1489_d_n4, eq26_e1489_d_n5, eq26_e1489_d_n6, eq26_e1497_d_n7, eq26_e1497_d_n8, eq26_e1489_d_n9, eq26_e1489_d_n10, eq26_e1489_d_n11, eq26_e1489_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1499;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq26_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq26_e1499_d_n3), multiplicity * (eq26_e1499_d_n4), multiplicity * (eq26_e1499_d_n5), multiplicity * (eq26_e1499_d_n6), multiplicity * (eq26_e1499_d_n7), multiplicity * (eq26_e1499_d_n8), multiplicity * (eq26_e1499_d_n9), multiplicity * (eq26_e1499_d_n10), multiplicity * (eq26_e1499_d_n11), multiplicity * (eq26_e1499_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq27_e1508, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12,) = {
    if (locals.var_guard1513 == 0.0) {
        let eq27_e1504: f64 = (p.p37 * p.p32);
        let eq27_e1506: f64 = (eq27_e1504 * locals.var_iii);
        let eq27_e1506_d_n3: f64 = (eq27_e1504 * locals.var_iii_dn3);
        let eq27_e1506_d_n4: f64 = (eq27_e1504 * locals.var_iii_dn4);
        let eq27_e1506_d_n5: f64 = (eq27_e1504 * locals.var_iii_dn5);
        let eq27_e1506_d_n6: f64 = (eq27_e1504 * locals.var_iii_dn6);
        let eq27_e1506_d_n7: f64 = (eq27_e1504 * locals.var_iii_dn7);
        let eq27_e1506_d_n8: f64 = (eq27_e1504 * locals.var_iii_dn8);
        let eq27_e1506_d_n9: f64 = (eq27_e1504 * locals.var_iii_dn9);
        let eq27_e1506_d_n10: f64 = (eq27_e1504 * locals.var_iii_dn10);
        let eq27_e1506_d_n11: f64 = (eq27_e1504 * locals.var_iii_dn11);
        let eq27_e1506_d_n12: f64 = (eq27_e1504 * locals.var_iii_dn12);
        (eq27_e1506, eq27_e1506_d_n3, eq27_e1506_d_n4, eq27_e1506_d_n5, eq27_e1506_d_n6, eq27_e1506_d_n7, eq27_e1506_d_n8, eq27_e1506_d_n9, eq27_e1506_d_n10, eq27_e1506_d_n11, eq27_e1506_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1508;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq27_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq27_e1508_d_n3), multiplicity * (eq27_e1508_d_n4), multiplicity * (eq27_e1508_d_n5), multiplicity * (eq27_e1508_d_n6), multiplicity * (eq27_e1508_d_n7), multiplicity * (eq27_e1508_d_n8), multiplicity * (eq27_e1508_d_n9), multiplicity * (eq27_e1508_d_n10), multiplicity * (eq27_e1508_d_n11), multiplicity * (eq27_e1508_d_n12)],
            [],
            [],
            1.0,
        );
        let eq28_e1511: f64 = (p.p32 * locals.var_b4soiigidl);
        let eq28_e1511_d_n3: f64 = (p.p32 * locals.var_b4soiigidl_dn3);
        let eq28_e1511_d_n4: f64 = (p.p32 * locals.var_b4soiigidl_dn4);
        let eq28_e1511_d_n5: f64 = (p.p32 * locals.var_b4soiigidl_dn5);
        let eq28_e1511_d_n6: f64 = (p.p32 * locals.var_b4soiigidl_dn6);
        let eq28_e1511_d_n7: f64 = (p.p32 * locals.var_b4soiigidl_dn7);
        let eq28_e1511_d_n8: f64 = (p.p32 * locals.var_b4soiigidl_dn8);
        let eq28_e1511_d_n9: f64 = (p.p32 * locals.var_b4soiigidl_dn9);
        let eq28_e1511_d_n10: f64 = (p.p32 * locals.var_b4soiigidl_dn10);
        let eq28_e1511_d_n11: f64 = (p.p32 * locals.var_b4soiigidl_dn11);
        let eq28_e1511_d_n12: f64 = (p.p32 * locals.var_b4soiigidl_dn12);
        let eq28_value: f64 = eq28_e1511;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq28_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq28_e1511_d_n3), multiplicity * (eq28_e1511_d_n4), multiplicity * (eq28_e1511_d_n5), multiplicity * (eq28_e1511_d_n6), multiplicity * (eq28_e1511_d_n7), multiplicity * (eq28_e1511_d_n8), multiplicity * (eq28_e1511_d_n9), multiplicity * (eq28_e1511_d_n10), multiplicity * (eq28_e1511_d_n11), multiplicity * (eq28_e1511_d_n12)],
            [],
            [],
            1.0,
        );
        let eq29_e1514: f64 = (p.p32 * locals.var_b4soiigisl);
        let eq29_e1514_d_n3: f64 = (p.p32 * locals.var_b4soiigisl_dn3);
        let eq29_e1514_d_n4: f64 = (p.p32 * locals.var_b4soiigisl_dn4);
        let eq29_e1514_d_n5: f64 = (p.p32 * locals.var_b4soiigisl_dn5);
        let eq29_e1514_d_n6: f64 = (p.p32 * locals.var_b4soiigisl_dn6);
        let eq29_e1514_d_n7: f64 = (p.p32 * locals.var_b4soiigisl_dn7);
        let eq29_e1514_d_n8: f64 = (p.p32 * locals.var_b4soiigisl_dn8);
        let eq29_e1514_d_n9: f64 = (p.p32 * locals.var_b4soiigisl_dn9);
        let eq29_e1514_d_n10: f64 = (p.p32 * locals.var_b4soiigisl_dn10);
        let eq29_e1514_d_n11: f64 = (p.p32 * locals.var_b4soiigisl_dn11);
        let eq29_e1514_d_n12: f64 = (p.p32 * locals.var_b4soiigisl_dn12);
        let eq29_value: f64 = eq29_e1514;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq29_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq29_e1514_d_n3), multiplicity * (eq29_e1514_d_n4), multiplicity * (eq29_e1514_d_n5), multiplicity * (eq29_e1514_d_n6), multiplicity * (eq29_e1514_d_n7), multiplicity * (eq29_e1514_d_n8), multiplicity * (eq29_e1514_d_n9), multiplicity * (eq29_e1514_d_n10), multiplicity * (eq29_e1514_d_n11), multiplicity * (eq29_e1514_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq30_e1517: f64 = (p.p37 * p.p32);
        let eq30_e1519: f64 = (eq30_e1517 * locals.var_ibd_1);
        let eq30_e1519_d_n3: f64 = (eq30_e1517 * locals.var_ibd_1_dn3);
        let eq30_e1519_d_n4: f64 = (eq30_e1517 * locals.var_ibd_1_dn4);
        let eq30_e1519_d_n5: f64 = (eq30_e1517 * locals.var_ibd_1_dn5);
        let eq30_e1519_d_n6: f64 = (eq30_e1517 * locals.var_ibd_1_dn6);
        let eq30_e1519_d_n7: f64 = (eq30_e1517 * locals.var_ibd_1_dn7);
        let eq30_e1519_d_n8: f64 = (eq30_e1517 * locals.var_ibd_1_dn8);
        let eq30_e1519_d_n9: f64 = (eq30_e1517 * locals.var_ibd_1_dn9);
        let eq30_e1519_d_n10: f64 = (eq30_e1517 * locals.var_ibd_1_dn10);
        let eq30_e1519_d_n11: f64 = (eq30_e1517 * locals.var_ibd_1_dn11);
        let eq30_e1519_d_n12: f64 = (eq30_e1517 * locals.var_ibd_1_dn12);
        let eq30_value: f64 = eq30_e1519;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            Some(7),
            multiplicity * (eq30_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq30_e1519_d_n3), multiplicity * (eq30_e1519_d_n4), multiplicity * (eq30_e1519_d_n5), multiplicity * (eq30_e1519_d_n6), multiplicity * (eq30_e1519_d_n7), multiplicity * (eq30_e1519_d_n8), multiplicity * (eq30_e1519_d_n9), multiplicity * (eq30_e1519_d_n10), multiplicity * (eq30_e1519_d_n11), multiplicity * (eq30_e1519_d_n12)],
            [],
            [],
            1.0,
        );
        let eq31_e1522: f64 = (p.p37 * p.p32);
        let eq31_e1524: f64 = (eq31_e1522 * locals.var_ibs_1);
        let eq31_e1524_d_n3: f64 = (eq31_e1522 * locals.var_ibs_1_dn3);
        let eq31_e1524_d_n4: f64 = (eq31_e1522 * locals.var_ibs_1_dn4);
        let eq31_e1524_d_n5: f64 = (eq31_e1522 * locals.var_ibs_1_dn5);
        let eq31_e1524_d_n6: f64 = (eq31_e1522 * locals.var_ibs_1_dn6);
        let eq31_e1524_d_n7: f64 = (eq31_e1522 * locals.var_ibs_1_dn7);
        let eq31_e1524_d_n8: f64 = (eq31_e1522 * locals.var_ibs_1_dn8);
        let eq31_e1524_d_n9: f64 = (eq31_e1522 * locals.var_ibs_1_dn9);
        let eq31_e1524_d_n10: f64 = (eq31_e1522 * locals.var_ibs_1_dn10);
        let eq31_e1524_d_n11: f64 = (eq31_e1522 * locals.var_ibs_1_dn11);
        let eq31_e1524_d_n12: f64 = (eq31_e1522 * locals.var_ibs_1_dn12);
        let eq31_value: f64 = eq31_e1524;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq31_e1524_d_n3), multiplicity * (eq31_e1524_d_n4), multiplicity * (eq31_e1524_d_n5), multiplicity * (eq31_e1524_d_n6), multiplicity * (eq31_e1524_d_n7), multiplicity * (eq31_e1524_d_n8), multiplicity * (eq31_e1524_d_n9), multiplicity * (eq31_e1524_d_n10), multiplicity * (eq31_e1524_d_n11), multiplicity * (eq31_e1524_d_n12)],
            [],
            [],
            1.0,
        );
        let eq32_e1528: f64 = (locals.var_b4soiigd + locals.var_b4soiigcd);
        let eq32_e1528_d_n3: f64 = (locals.var_b4soiigd_dn3 + locals.var_b4soiigcd_dn3);
        let eq32_e1528_d_n4: f64 = (locals.var_b4soiigd_dn4 + locals.var_b4soiigcd_dn4);
        let eq32_e1528_d_n5: f64 = (locals.var_b4soiigd_dn5 + locals.var_b4soiigcd_dn5);
        let eq32_e1528_d_n6: f64 = (locals.var_b4soiigd_dn6 + locals.var_b4soiigcd_dn6);
        let eq32_e1528_d_n7: f64 = (locals.var_b4soiigd_dn7 + locals.var_b4soiigcd_dn7);
        let eq32_e1528_d_n8: f64 = (locals.var_b4soiigd_dn8 + locals.var_b4soiigcd_dn8);
        let eq32_e1528_d_n9: f64 = (locals.var_b4soiigd_dn9 + locals.var_b4soiigcd_dn9);
        let eq32_e1528_d_n10: f64 = (locals.var_b4soiigd_dn10 + locals.var_b4soiigcd_dn10);
        let eq32_e1528_d_n11: f64 = (locals.var_b4soiigd_dn11 + locals.var_b4soiigcd_dn11);
        let eq32_e1528_d_n12: f64 = (locals.var_b4soiigd_dn12 + locals.var_b4soiigcd_dn12);
        let eq32_e1529: f64 = (p.p32 * eq32_e1528);
        let eq32_e1529_d_n3: f64 = (p.p32 * eq32_e1528_d_n3);
        let eq32_e1529_d_n4: f64 = (p.p32 * eq32_e1528_d_n4);
        let eq32_e1529_d_n5: f64 = (p.p32 * eq32_e1528_d_n5);
        let eq32_e1529_d_n6: f64 = (p.p32 * eq32_e1528_d_n6);
        let eq32_e1529_d_n7: f64 = (p.p32 * eq32_e1528_d_n7);
        let eq32_e1529_d_n8: f64 = (p.p32 * eq32_e1528_d_n8);
        let eq32_e1529_d_n9: f64 = (p.p32 * eq32_e1528_d_n9);
        let eq32_e1529_d_n10: f64 = (p.p32 * eq32_e1528_d_n10);
        let eq32_e1529_d_n11: f64 = (p.p32 * eq32_e1528_d_n11);
        let eq32_e1529_d_n12: f64 = (p.p32 * eq32_e1528_d_n12);
        let eq32_e1533: f64 = 0.0;
        let eq32_e1535: f64 = (eq32_e1533 * (nv9 - nv7));
        let eq32_e1536: f64 = (p.p32 * eq32_e1535);
        let eq32_e1536_d_n7: f64 = (p.p32 * (-eq32_e1533));
        let eq32_e1536_d_n9: f64 = (p.p32 * eq32_e1533);
        let eq32_e1537: f64 = (eq32_e1529 + eq32_e1536);
        let eq32_e1537_d_n7: f64 = (eq32_e1529_d_n7 + eq32_e1536_d_n7);
        let eq32_e1537_d_n9: f64 = (eq32_e1529_d_n9 + eq32_e1536_d_n9);
        let eq32_value: f64 = eq32_e1537;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq32_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq32_e1529_d_n3), multiplicity * (eq32_e1529_d_n4), multiplicity * (eq32_e1529_d_n5), multiplicity * (eq32_e1529_d_n6), multiplicity * (eq32_e1537_d_n7), multiplicity * (eq32_e1529_d_n8), multiplicity * (eq32_e1537_d_n9), multiplicity * (eq32_e1529_d_n10), multiplicity * (eq32_e1529_d_n11), multiplicity * (eq32_e1529_d_n12)],
            [],
            [],
            1.0,
        );
        let eq33_e1541: f64 = (locals.var_b4soiigs + locals.var_b4soiigcs);
        let eq33_e1541_d_n3: f64 = (locals.var_b4soiigs_dn3 + locals.var_b4soiigcs_dn3);
        let eq33_e1541_d_n4: f64 = (locals.var_b4soiigs_dn4 + locals.var_b4soiigcs_dn4);
        let eq33_e1541_d_n5: f64 = (locals.var_b4soiigs_dn5 + locals.var_b4soiigcs_dn5);
        let eq33_e1541_d_n6: f64 = (locals.var_b4soiigs_dn6 + locals.var_b4soiigcs_dn6);
        let eq33_e1541_d_n7: f64 = (locals.var_b4soiigs_dn7 + locals.var_b4soiigcs_dn7);
        let eq33_e1541_d_n8: f64 = (locals.var_b4soiigs_dn8 + locals.var_b4soiigcs_dn8);
        let eq33_e1541_d_n9: f64 = (locals.var_b4soiigs_dn9 + locals.var_b4soiigcs_dn9);
        let eq33_e1541_d_n10: f64 = (locals.var_b4soiigs_dn10 + locals.var_b4soiigcs_dn10);
        let eq33_e1541_d_n11: f64 = (locals.var_b4soiigs_dn11 + locals.var_b4soiigcs_dn11);
        let eq33_e1541_d_n12: f64 = (locals.var_b4soiigs_dn12 + locals.var_b4soiigcs_dn12);
        let eq33_e1542: f64 = (p.p32 * eq33_e1541);
        let eq33_e1542_d_n3: f64 = (p.p32 * eq33_e1541_d_n3);
        let eq33_e1542_d_n4: f64 = (p.p32 * eq33_e1541_d_n4);
        let eq33_e1542_d_n5: f64 = (p.p32 * eq33_e1541_d_n5);
        let eq33_e1542_d_n6: f64 = (p.p32 * eq33_e1541_d_n6);
        let eq33_e1542_d_n7: f64 = (p.p32 * eq33_e1541_d_n7);
        let eq33_e1542_d_n8: f64 = (p.p32 * eq33_e1541_d_n8);
        let eq33_e1542_d_n9: f64 = (p.p32 * eq33_e1541_d_n9);
        let eq33_e1542_d_n10: f64 = (p.p32 * eq33_e1541_d_n10);
        let eq33_e1542_d_n11: f64 = (p.p32 * eq33_e1541_d_n11);
        let eq33_e1542_d_n12: f64 = (p.p32 * eq33_e1541_d_n12);
        let eq33_e1546: f64 = 0.0;
        let eq33_e1548: f64 = (eq33_e1546 * (nv9 - nv8));
        let eq33_e1549: f64 = (p.p32 * eq33_e1548);
        let eq33_e1549_d_n8: f64 = (p.p32 * (-eq33_e1546));
        let eq33_e1549_d_n9: f64 = (p.p32 * eq33_e1546);
        let eq33_e1550: f64 = (eq33_e1542 + eq33_e1549);
        let eq33_e1550_d_n8: f64 = (eq33_e1542_d_n8 + eq33_e1549_d_n8);
        let eq33_e1550_d_n9: f64 = (eq33_e1542_d_n9 + eq33_e1549_d_n9);
        let eq33_value: f64 = eq33_e1550;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq33_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e1542_d_n3), multiplicity * (eq33_e1542_d_n4), multiplicity * (eq33_e1542_d_n5), multiplicity * (eq33_e1542_d_n6), multiplicity * (eq33_e1542_d_n7), multiplicity * (eq33_e1550_d_n8), multiplicity * (eq33_e1550_d_n9), multiplicity * (eq33_e1542_d_n10), multiplicity * (eq33_e1542_d_n11), multiplicity * (eq33_e1542_d_n12)],
            [],
            [],
            1.0,
        );
        let eq34_e1553: f64 = (p.p32 * locals.var_b4soiig);
        let eq34_e1553_d_n3: f64 = (p.p32 * locals.var_b4soiig_dn3);
        let eq34_e1553_d_n4: f64 = (p.p32 * locals.var_b4soiig_dn4);
        let eq34_e1553_d_n5: f64 = (p.p32 * locals.var_b4soiig_dn5);
        let eq34_e1553_d_n6: f64 = (p.p32 * locals.var_b4soiig_dn6);
        let eq34_e1553_d_n7: f64 = (p.p32 * locals.var_b4soiig_dn7);
        let eq34_e1553_d_n8: f64 = (p.p32 * locals.var_b4soiig_dn8);
        let eq34_e1553_d_n9: f64 = (p.p32 * locals.var_b4soiig_dn9);
        let eq34_e1553_d_n10: f64 = (p.p32 * locals.var_b4soiig_dn10);
        let eq34_e1553_d_n11: f64 = (p.p32 * locals.var_b4soiig_dn11);
        let eq34_e1553_d_n12: f64 = (p.p32 * locals.var_b4soiig_dn12);
        let eq34_value: f64 = eq34_e1553;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq34_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq34_e1553_d_n3), multiplicity * (eq34_e1553_d_n4), multiplicity * (eq34_e1553_d_n5), multiplicity * (eq34_e1553_d_n6), multiplicity * (eq34_e1553_d_n7), multiplicity * (eq34_e1553_d_n8), multiplicity * (eq34_e1553_d_n9), multiplicity * (eq34_e1553_d_n10), multiplicity * (eq34_e1553_d_n11), multiplicity * (eq34_e1553_d_n12)],
            [],
            [],
            1.0,
        );
        let eq35_e1556: f64 = (p.p32 * locals.var_b4soiigp);
        let eq35_e1556_d_n3: f64 = (p.p32 * locals.var_b4soiigp_dn3);
        let eq35_e1556_d_n4: f64 = (p.p32 * locals.var_b4soiigp_dn4);
        let eq35_e1556_d_n5: f64 = (p.p32 * locals.var_b4soiigp_dn5);
        let eq35_e1556_d_n6: f64 = (p.p32 * locals.var_b4soiigp_dn6);
        let eq35_e1556_d_n7: f64 = (p.p32 * locals.var_b4soiigp_dn7);
        let eq35_e1556_d_n8: f64 = (p.p32 * locals.var_b4soiigp_dn8);
        let eq35_e1556_d_n9: f64 = (p.p32 * locals.var_b4soiigp_dn9);
        let eq35_e1556_d_n10: f64 = (p.p32 * locals.var_b4soiigp_dn10);
        let eq35_e1556_d_n11: f64 = (p.p32 * locals.var_b4soiigp_dn11);
        let eq35_e1556_d_n12: f64 = (p.p32 * locals.var_b4soiigp_dn12);
        let eq35_value: f64 = eq35_e1556;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(4),
            multiplicity * (eq35_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq35_e1556_d_n3), multiplicity * (eq35_e1556_d_n4), multiplicity * (eq35_e1556_d_n5), multiplicity * (eq35_e1556_d_n6), multiplicity * (eq35_e1556_d_n7), multiplicity * (eq35_e1556_d_n8), multiplicity * (eq35_e1556_d_n9), multiplicity * (eq35_e1556_d_n10), multiplicity * (eq35_e1556_d_n11), multiplicity * (eq35_e1556_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq37_e1569, eq37_e1569_d_n3, eq37_e1569_d_n4, eq37_e1569_d_n5, eq37_e1569_d_n6, eq37_e1569_d_n7, eq37_e1569_d_n8, eq37_e1569_d_n9, eq37_e1569_d_n10, eq37_e1569_d_n11, eq37_e1569_d_n12,) = {
    if (locals.var_guard1517 == 0.0) {
        let eq37_e1565: f64 = (p.p37 * p.p32);
        let eq37_e1567: f64 = (eq37_e1565 * locals.var_ibp);
        let eq37_e1567_d_n3: f64 = (eq37_e1565 * locals.var_ibp_dn3);
        let eq37_e1567_d_n4: f64 = (eq37_e1565 * locals.var_ibp_dn4);
        let eq37_e1567_d_n5: f64 = (eq37_e1565 * locals.var_ibp_dn5);
        let eq37_e1567_d_n6: f64 = (eq37_e1565 * locals.var_ibp_dn6);
        let eq37_e1567_d_n7: f64 = (eq37_e1565 * locals.var_ibp_dn7);
        let eq37_e1567_d_n8: f64 = (eq37_e1565 * locals.var_ibp_dn8);
        let eq37_e1567_d_n9: f64 = (eq37_e1565 * locals.var_ibp_dn9);
        let eq37_e1567_d_n10: f64 = (eq37_e1565 * locals.var_ibp_dn10);
        let eq37_e1567_d_n11: f64 = (eq37_e1565 * locals.var_ibp_dn11);
        let eq37_e1567_d_n12: f64 = (eq37_e1565 * locals.var_ibp_dn12);
        (eq37_e1567, eq37_e1567_d_n3, eq37_e1567_d_n4, eq37_e1567_d_n5, eq37_e1567_d_n6, eq37_e1567_d_n7, eq37_e1567_d_n8, eq37_e1567_d_n9, eq37_e1567_d_n10, eq37_e1567_d_n11, eq37_e1567_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1569;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq37_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq37_e1569_d_n3), multiplicity * (eq37_e1569_d_n4), multiplicity * (eq37_e1569_d_n5), multiplicity * (eq37_e1569_d_n6), multiplicity * (eq37_e1569_d_n7), multiplicity * (eq37_e1569_d_n8), multiplicity * (eq37_e1569_d_n9), multiplicity * (eq37_e1569_d_n10), multiplicity * (eq37_e1569_d_n11), multiplicity * (eq37_e1569_d_n12)],
            [],
            [],
            1.0,
        );
        let eq44_e1647: f64 = (p.p33 * locals.var_b4soiqdrn);
        let eq44_e1647_d_n3: f64 = (p.p33 * locals.var_b4soiqdrn_dn3);
        let eq44_e1647_d_n4: f64 = (p.p33 * locals.var_b4soiqdrn_dn4);
        let eq44_e1647_d_n5: f64 = (p.p33 * locals.var_b4soiqdrn_dn5);
        let eq44_e1647_d_n6: f64 = (p.p33 * locals.var_b4soiqdrn_dn6);
        let eq44_e1647_d_n7: f64 = (p.p33 * locals.var_b4soiqdrn_dn7);
        let eq44_e1647_d_n8: f64 = (p.p33 * locals.var_b4soiqdrn_dn8);
        let eq44_e1647_d_n9: f64 = (p.p33 * locals.var_b4soiqdrn_dn9);
        let eq44_e1647_d_n10: f64 = (p.p33 * locals.var_b4soiqdrn_dn10);
        let eq44_e1647_d_n11: f64 = (p.p33 * locals.var_b4soiqdrn_dn11);
        let eq44_e1647_d_n12: f64 = (p.p33 * locals.var_b4soiqdrn_dn12);
        let eq44_e1648: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq44_e1647);
        let eq44_value: f64 = eq44_e1648;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq44_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq44_e1647_d_n3 * ddt_scale)), multiplicity * ((eq44_e1647_d_n4 * ddt_scale)), multiplicity * ((eq44_e1647_d_n5 * ddt_scale)), multiplicity * ((eq44_e1647_d_n6 * ddt_scale)), multiplicity * ((eq44_e1647_d_n7 * ddt_scale)), multiplicity * ((eq44_e1647_d_n8 * ddt_scale)), multiplicity * ((eq44_e1647_d_n9 * ddt_scale)), multiplicity * ((eq44_e1647_d_n10 * ddt_scale)), multiplicity * ((eq44_e1647_d_n11 * ddt_scale)), multiplicity * ((eq44_e1647_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq45_e1651: f64 = (p.p33 * locals.var_b4soiqsrc);
        let eq45_e1651_d_n3: f64 = (p.p33 * locals.var_b4soiqsrc_dn3);
        let eq45_e1651_d_n4: f64 = (p.p33 * locals.var_b4soiqsrc_dn4);
        let eq45_e1651_d_n5: f64 = (p.p33 * locals.var_b4soiqsrc_dn5);
        let eq45_e1651_d_n6: f64 = (p.p33 * locals.var_b4soiqsrc_dn6);
        let eq45_e1651_d_n7: f64 = (p.p33 * locals.var_b4soiqsrc_dn7);
        let eq45_e1651_d_n8: f64 = (p.p33 * locals.var_b4soiqsrc_dn8);
        let eq45_e1651_d_n9: f64 = (p.p33 * locals.var_b4soiqsrc_dn9);
        let eq45_e1651_d_n10: f64 = (p.p33 * locals.var_b4soiqsrc_dn10);
        let eq45_e1651_d_n11: f64 = (p.p33 * locals.var_b4soiqsrc_dn11);
        let eq45_e1651_d_n12: f64 = (p.p33 * locals.var_b4soiqsrc_dn12);
        let eq45_e1652: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq45_e1651);
        let eq45_value: f64 = eq45_e1652;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq45_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq45_e1651_d_n3 * ddt_scale)), multiplicity * ((eq45_e1651_d_n4 * ddt_scale)), multiplicity * ((eq45_e1651_d_n5 * ddt_scale)), multiplicity * ((eq45_e1651_d_n6 * ddt_scale)), multiplicity * ((eq45_e1651_d_n7 * ddt_scale)), multiplicity * ((eq45_e1651_d_n8 * ddt_scale)), multiplicity * ((eq45_e1651_d_n9 * ddt_scale)), multiplicity * ((eq45_e1651_d_n10 * ddt_scale)), multiplicity * ((eq45_e1651_d_n11 * ddt_scale)), multiplicity * ((eq45_e1651_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq46_e1656: f64 = (p.p33 * locals.var_qgate);
        let eq46_e1656_d_n3: f64 = (p.p33 * locals.var_qgate_dn3);
        let eq46_e1656_d_n4: f64 = (p.p33 * locals.var_qgate_dn4);
        let eq46_e1656_d_n5: f64 = (p.p33 * locals.var_qgate_dn5);
        let eq46_e1656_d_n6: f64 = (p.p33 * locals.var_qgate_dn6);
        let eq46_e1656_d_n7: f64 = (p.p33 * locals.var_qgate_dn7);
        let eq46_e1656_d_n8: f64 = (p.p33 * locals.var_qgate_dn8);
        let eq46_e1656_d_n9: f64 = (p.p33 * locals.var_qgate_dn9);
        let eq46_e1656_d_n10: f64 = (p.p33 * locals.var_qgate_dn10);
        let eq46_e1656_d_n11: f64 = (p.p33 * locals.var_qgate_dn11);
        let eq46_e1656_d_n12: f64 = (p.p33 * locals.var_qgate_dn12);
        let eq46_e1657: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq46_e1656);
        let eq46_e1658: f64 = (p.p37 * eq46_e1657);
        let eq46_e1658_d_n3: f64 = (p.p37 * (eq46_e1656_d_n3 * ddt_scale));
        let eq46_e1658_d_n4: f64 = (p.p37 * (eq46_e1656_d_n4 * ddt_scale));
        let eq46_e1658_d_n5: f64 = (p.p37 * (eq46_e1656_d_n5 * ddt_scale));
        let eq46_e1658_d_n6: f64 = (p.p37 * (eq46_e1656_d_n6 * ddt_scale));
        let eq46_e1658_d_n7: f64 = (p.p37 * (eq46_e1656_d_n7 * ddt_scale));
        let eq46_e1658_d_n8: f64 = (p.p37 * (eq46_e1656_d_n8 * ddt_scale));
        let eq46_e1658_d_n9: f64 = (p.p37 * (eq46_e1656_d_n9 * ddt_scale));
        let eq46_e1658_d_n10: f64 = (p.p37 * (eq46_e1656_d_n10 * ddt_scale));
        let eq46_e1658_d_n11: f64 = (p.p37 * (eq46_e1656_d_n11 * ddt_scale));
        let eq46_e1658_d_n12: f64 = (p.p37 * (eq46_e1656_d_n12 * ddt_scale));
        let eq46_value: f64 = eq46_e1658;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq46_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq46_e1658_d_n3), multiplicity * (eq46_e1658_d_n4), multiplicity * (eq46_e1658_d_n5), multiplicity * (eq46_e1658_d_n6), multiplicity * (eq46_e1658_d_n7), multiplicity * (eq46_e1658_d_n8), multiplicity * (eq46_e1658_d_n9), multiplicity * (eq46_e1658_d_n10), multiplicity * (eq46_e1658_d_n11), multiplicity * (eq46_e1658_d_n12)],
            [],
            [],
            1.0,
        );
        let eq47_e1662: f64 = (p.p33 * locals.var_qsub);
        let eq47_e1662_d_n3: f64 = (p.p33 * locals.var_qsub_dn3);
        let eq47_e1662_d_n4: f64 = (p.p33 * locals.var_qsub_dn4);
        let eq47_e1662_d_n5: f64 = (p.p33 * locals.var_qsub_dn5);
        let eq47_e1662_d_n6: f64 = (p.p33 * locals.var_qsub_dn6);
        let eq47_e1662_d_n7: f64 = (p.p33 * locals.var_qsub_dn7);
        let eq47_e1662_d_n8: f64 = (p.p33 * locals.var_qsub_dn8);
        let eq47_e1662_d_n9: f64 = (p.p33 * locals.var_qsub_dn9);
        let eq47_e1662_d_n10: f64 = (p.p33 * locals.var_qsub_dn10);
        let eq47_e1662_d_n11: f64 = (p.p33 * locals.var_qsub_dn11);
        let eq47_e1662_d_n12: f64 = (p.p33 * locals.var_qsub_dn12);
        let eq47_e1663: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq47_e1662);
        let eq47_e1664: f64 = (p.p37 * eq47_e1663);
        let eq47_e1664_d_n3: f64 = (p.p37 * (eq47_e1662_d_n3 * ddt_scale));
        let eq47_e1664_d_n4: f64 = (p.p37 * (eq47_e1662_d_n4 * ddt_scale));
        let eq47_e1664_d_n5: f64 = (p.p37 * (eq47_e1662_d_n5 * ddt_scale));
        let eq47_e1664_d_n6: f64 = (p.p37 * (eq47_e1662_d_n6 * ddt_scale));
        let eq47_e1664_d_n7: f64 = (p.p37 * (eq47_e1662_d_n7 * ddt_scale));
        let eq47_e1664_d_n8: f64 = (p.p37 * (eq47_e1662_d_n8 * ddt_scale));
        let eq47_e1664_d_n9: f64 = (p.p37 * (eq47_e1662_d_n9 * ddt_scale));
        let eq47_e1664_d_n10: f64 = (p.p37 * (eq47_e1662_d_n10 * ddt_scale));
        let eq47_e1664_d_n11: f64 = (p.p37 * (eq47_e1662_d_n11 * ddt_scale));
        let eq47_e1664_d_n12: f64 = (p.p37 * (eq47_e1662_d_n12 * ddt_scale));
        let eq47_value: f64 = eq47_e1664;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(3),
            Some(5),
            multiplicity * (eq47_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq47_e1664_d_n3), multiplicity * (eq47_e1664_d_n4), multiplicity * (eq47_e1664_d_n5), multiplicity * (eq47_e1664_d_n6), multiplicity * (eq47_e1664_d_n7), multiplicity * (eq47_e1664_d_n8), multiplicity * (eq47_e1664_d_n9), multiplicity * (eq47_e1664_d_n10), multiplicity * (eq47_e1664_d_n11), multiplicity * (eq47_e1664_d_n12)],
            [],
            [],
            1.0,
        );
        let eq48_e1668: f64 = (p.p33 * locals.var_qjd_1);
        let eq48_e1668_d_n3: f64 = (p.p33 * locals.var_qjd_1_dn3);
        let eq48_e1668_d_n4: f64 = (p.p33 * locals.var_qjd_1_dn4);
        let eq48_e1668_d_n5: f64 = (p.p33 * locals.var_qjd_1_dn5);
        let eq48_e1668_d_n6: f64 = (p.p33 * locals.var_qjd_1_dn6);
        let eq48_e1668_d_n7: f64 = (p.p33 * locals.var_qjd_1_dn7);
        let eq48_e1668_d_n8: f64 = (p.p33 * locals.var_qjd_1_dn8);
        let eq48_e1668_d_n9: f64 = (p.p33 * locals.var_qjd_1_dn9);
        let eq48_e1668_d_n10: f64 = (p.p33 * locals.var_qjd_1_dn10);
        let eq48_e1668_d_n11: f64 = (p.p33 * locals.var_qjd_1_dn11);
        let eq48_e1668_d_n12: f64 = (p.p33 * locals.var_qjd_1_dn12);
        let eq48_e1669: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq48_e1668);
        let eq48_e1670: f64 = (p.p37 * eq48_e1669);
        let eq48_e1670_d_n3: f64 = (p.p37 * (eq48_e1668_d_n3 * ddt_scale));
        let eq48_e1670_d_n4: f64 = (p.p37 * (eq48_e1668_d_n4 * ddt_scale));
        let eq48_e1670_d_n5: f64 = (p.p37 * (eq48_e1668_d_n5 * ddt_scale));
        let eq48_e1670_d_n6: f64 = (p.p37 * (eq48_e1668_d_n6 * ddt_scale));
        let eq48_e1670_d_n7: f64 = (p.p37 * (eq48_e1668_d_n7 * ddt_scale));
        let eq48_e1670_d_n8: f64 = (p.p37 * (eq48_e1668_d_n8 * ddt_scale));
        let eq48_e1670_d_n9: f64 = (p.p37 * (eq48_e1668_d_n9 * ddt_scale));
        let eq48_e1670_d_n10: f64 = (p.p37 * (eq48_e1668_d_n10 * ddt_scale));
        let eq48_e1670_d_n11: f64 = (p.p37 * (eq48_e1668_d_n11 * ddt_scale));
        let eq48_e1670_d_n12: f64 = (p.p37 * (eq48_e1668_d_n12 * ddt_scale));
        let eq48_value: f64 = eq48_e1670;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            Some(7),
            multiplicity * (eq48_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq48_e1670_d_n3), multiplicity * (eq48_e1670_d_n4), multiplicity * (eq48_e1670_d_n5), multiplicity * (eq48_e1670_d_n6), multiplicity * (eq48_e1670_d_n7), multiplicity * (eq48_e1670_d_n8), multiplicity * (eq48_e1670_d_n9), multiplicity * (eq48_e1670_d_n10), multiplicity * (eq48_e1670_d_n11), multiplicity * (eq48_e1670_d_n12)],
            [],
            [],
            1.0,
        );
        let eq49_e1674: f64 = (p.p33 * locals.var_qjs_1);
        let eq49_e1674_d_n3: f64 = (p.p33 * locals.var_qjs_1_dn3);
        let eq49_e1674_d_n4: f64 = (p.p33 * locals.var_qjs_1_dn4);
        let eq49_e1674_d_n5: f64 = (p.p33 * locals.var_qjs_1_dn5);
        let eq49_e1674_d_n6: f64 = (p.p33 * locals.var_qjs_1_dn6);
        let eq49_e1674_d_n7: f64 = (p.p33 * locals.var_qjs_1_dn7);
        let eq49_e1674_d_n8: f64 = (p.p33 * locals.var_qjs_1_dn8);
        let eq49_e1674_d_n9: f64 = (p.p33 * locals.var_qjs_1_dn9);
        let eq49_e1674_d_n10: f64 = (p.p33 * locals.var_qjs_1_dn10);
        let eq49_e1674_d_n11: f64 = (p.p33 * locals.var_qjs_1_dn11);
        let eq49_e1674_d_n12: f64 = (p.p33 * locals.var_qjs_1_dn12);
        let eq49_e1675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq49_e1674);
        let eq49_e1676: f64 = (p.p37 * eq49_e1675);
        let eq49_e1676_d_n3: f64 = (p.p37 * (eq49_e1674_d_n3 * ddt_scale));
        let eq49_e1676_d_n4: f64 = (p.p37 * (eq49_e1674_d_n4 * ddt_scale));
        let eq49_e1676_d_n5: f64 = (p.p37 * (eq49_e1674_d_n5 * ddt_scale));
        let eq49_e1676_d_n6: f64 = (p.p37 * (eq49_e1674_d_n6 * ddt_scale));
        let eq49_e1676_d_n7: f64 = (p.p37 * (eq49_e1674_d_n7 * ddt_scale));
        let eq49_e1676_d_n8: f64 = (p.p37 * (eq49_e1674_d_n8 * ddt_scale));
        let eq49_e1676_d_n9: f64 = (p.p37 * (eq49_e1674_d_n9 * ddt_scale));
        let eq49_e1676_d_n10: f64 = (p.p37 * (eq49_e1674_d_n10 * ddt_scale));
        let eq49_e1676_d_n11: f64 = (p.p37 * (eq49_e1674_d_n11 * ddt_scale));
        let eq49_e1676_d_n12: f64 = (p.p37 * (eq49_e1674_d_n12 * ddt_scale));
        let eq49_value: f64 = eq49_e1676;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq49_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq49_e1676_d_n3), multiplicity * (eq49_e1676_d_n4), multiplicity * (eq49_e1676_d_n5), multiplicity * (eq49_e1676_d_n6), multiplicity * (eq49_e1676_d_n7), multiplicity * (eq49_e1676_d_n8), multiplicity * (eq49_e1676_d_n9), multiplicity * (eq49_e1676_d_n10), multiplicity * (eq49_e1676_d_n11), multiplicity * (eq49_e1676_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq50_e1685, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12,) = {
    if (locals.var_guard1518 != 0.0) {
        let eq50_e1681: f64 = (p.p33 * locals.var_qgdo);
        let eq50_e1681_d_n3: f64 = (p.p33 * locals.var_qgdo_dn3);
        let eq50_e1681_d_n4: f64 = (p.p33 * locals.var_qgdo_dn4);
        let eq50_e1681_d_n5: f64 = (p.p33 * locals.var_qgdo_dn5);
        let eq50_e1681_d_n6: f64 = (p.p33 * locals.var_qgdo_dn6);
        let eq50_e1681_d_n7: f64 = (p.p33 * locals.var_qgdo_dn7);
        let eq50_e1681_d_n8: f64 = (p.p33 * locals.var_qgdo_dn8);
        let eq50_e1681_d_n9: f64 = (p.p33 * locals.var_qgdo_dn9);
        let eq50_e1681_d_n10: f64 = (p.p33 * locals.var_qgdo_dn10);
        let eq50_e1681_d_n11: f64 = (p.p33 * locals.var_qgdo_dn11);
        let eq50_e1681_d_n12: f64 = (p.p33 * locals.var_qgdo_dn12);
        let eq50_e1682: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq50_e1681);
        let eq50_e1683: f64 = (p.p37 * eq50_e1682);
        let eq50_e1683_d_n3: f64 = (p.p37 * (eq50_e1681_d_n3 * ddt_scale));
        let eq50_e1683_d_n4: f64 = (p.p37 * (eq50_e1681_d_n4 * ddt_scale));
        let eq50_e1683_d_n5: f64 = (p.p37 * (eq50_e1681_d_n5 * ddt_scale));
        let eq50_e1683_d_n6: f64 = (p.p37 * (eq50_e1681_d_n6 * ddt_scale));
        let eq50_e1683_d_n7: f64 = (p.p37 * (eq50_e1681_d_n7 * ddt_scale));
        let eq50_e1683_d_n8: f64 = (p.p37 * (eq50_e1681_d_n8 * ddt_scale));
        let eq50_e1683_d_n9: f64 = (p.p37 * (eq50_e1681_d_n9 * ddt_scale));
        let eq50_e1683_d_n10: f64 = (p.p37 * (eq50_e1681_d_n10 * ddt_scale));
        let eq50_e1683_d_n11: f64 = (p.p37 * (eq50_e1681_d_n11 * ddt_scale));
        let eq50_e1683_d_n12: f64 = (p.p37 * (eq50_e1681_d_n12 * ddt_scale));
        (eq50_e1683, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1685;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq50_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq50_e1685_d_n3), multiplicity * (eq50_e1685_d_n4), multiplicity * (eq50_e1685_d_n5), multiplicity * (eq50_e1685_d_n6), multiplicity * (eq50_e1685_d_n7), multiplicity * (eq50_e1685_d_n8), multiplicity * (eq50_e1685_d_n9), multiplicity * (eq50_e1685_d_n10), multiplicity * (eq50_e1685_d_n11), multiplicity * (eq50_e1685_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq51_e1694, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12,) = {
    if (locals.var_guard1518 != 0.0) {
        let eq51_e1690: f64 = (p.p33 * locals.var_qgso);
        let eq51_e1690_d_n3: f64 = (p.p33 * locals.var_qgso_dn3);
        let eq51_e1690_d_n4: f64 = (p.p33 * locals.var_qgso_dn4);
        let eq51_e1690_d_n5: f64 = (p.p33 * locals.var_qgso_dn5);
        let eq51_e1690_d_n6: f64 = (p.p33 * locals.var_qgso_dn6);
        let eq51_e1690_d_n7: f64 = (p.p33 * locals.var_qgso_dn7);
        let eq51_e1690_d_n8: f64 = (p.p33 * locals.var_qgso_dn8);
        let eq51_e1690_d_n9: f64 = (p.p33 * locals.var_qgso_dn9);
        let eq51_e1690_d_n10: f64 = (p.p33 * locals.var_qgso_dn10);
        let eq51_e1690_d_n11: f64 = (p.p33 * locals.var_qgso_dn11);
        let eq51_e1690_d_n12: f64 = (p.p33 * locals.var_qgso_dn12);
        let eq51_e1691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1690);
        let eq51_e1692: f64 = (p.p37 * eq51_e1691);
        let eq51_e1692_d_n3: f64 = (p.p37 * (eq51_e1690_d_n3 * ddt_scale));
        let eq51_e1692_d_n4: f64 = (p.p37 * (eq51_e1690_d_n4 * ddt_scale));
        let eq51_e1692_d_n5: f64 = (p.p37 * (eq51_e1690_d_n5 * ddt_scale));
        let eq51_e1692_d_n6: f64 = (p.p37 * (eq51_e1690_d_n6 * ddt_scale));
        let eq51_e1692_d_n7: f64 = (p.p37 * (eq51_e1690_d_n7 * ddt_scale));
        let eq51_e1692_d_n8: f64 = (p.p37 * (eq51_e1690_d_n8 * ddt_scale));
        let eq51_e1692_d_n9: f64 = (p.p37 * (eq51_e1690_d_n9 * ddt_scale));
        let eq51_e1692_d_n10: f64 = (p.p37 * (eq51_e1690_d_n10 * ddt_scale));
        let eq51_e1692_d_n11: f64 = (p.p37 * (eq51_e1690_d_n11 * ddt_scale));
        let eq51_e1692_d_n12: f64 = (p.p37 * (eq51_e1690_d_n12 * ddt_scale));
        (eq51_e1692, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1694;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(8),
            multiplicity * (eq51_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq51_e1694_d_n3), multiplicity * (eq51_e1694_d_n4), multiplicity * (eq51_e1694_d_n5), multiplicity * (eq51_e1694_d_n6), multiplicity * (eq51_e1694_d_n7), multiplicity * (eq51_e1694_d_n8), multiplicity * (eq51_e1694_d_n9), multiplicity * (eq51_e1694_d_n10), multiplicity * (eq51_e1694_d_n11), multiplicity * (eq51_e1694_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq52_e1703, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12,) = {
    if (locals.var_guard1518 != 0.0) {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1700: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo);
        let eq52_e1700_d_n3: f64 = (((-p.p33) * locals.var_pparam_b4soicgeo) + (eq52_e1698 * locals.var_pparam_b4soicgeo_dn3));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn4);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn5);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn6);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn7);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn8);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn9);
        let eq52_e1700_d_n10: f64 = ((p.p33 * locals.var_pparam_b4soicgeo) + (eq52_e1698 * locals.var_pparam_b4soicgeo_dn10));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn11);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn12);
        let eq52_e1701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1700);
        (eq52_e1701, (eq52_e1700_d_n3 * ddt_scale), (eq52_e1700_d_n4 * ddt_scale), (eq52_e1700_d_n5 * ddt_scale), (eq52_e1700_d_n6 * ddt_scale), (eq52_e1700_d_n7 * ddt_scale), (eq52_e1700_d_n8 * ddt_scale), (eq52_e1700_d_n9 * ddt_scale), (eq52_e1700_d_n10 * ddt_scale), (eq52_e1700_d_n11 * ddt_scale), (eq52_e1700_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e1703;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(3),
            multiplicity * (eq52_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq52_e1703_d_n3), multiplicity * (eq52_e1703_d_n4), multiplicity * (eq52_e1703_d_n5), multiplicity * (eq52_e1703_d_n6), multiplicity * (eq52_e1703_d_n7), multiplicity * (eq52_e1703_d_n8), multiplicity * (eq52_e1703_d_n9), multiplicity * (eq52_e1703_d_n10), multiplicity * (eq52_e1703_d_n11), multiplicity * (eq52_e1703_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq53_e1713, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12,) = {
    if (locals.var_guard1518 == 0.0) {
        let eq53_e1709: f64 = (p.p33 * locals.var_qgdo);
        let eq53_e1709_d_n3: f64 = (p.p33 * locals.var_qgdo_dn3);
        let eq53_e1709_d_n4: f64 = (p.p33 * locals.var_qgdo_dn4);
        let eq53_e1709_d_n5: f64 = (p.p33 * locals.var_qgdo_dn5);
        let eq53_e1709_d_n6: f64 = (p.p33 * locals.var_qgdo_dn6);
        let eq53_e1709_d_n7: f64 = (p.p33 * locals.var_qgdo_dn7);
        let eq53_e1709_d_n8: f64 = (p.p33 * locals.var_qgdo_dn8);
        let eq53_e1709_d_n9: f64 = (p.p33 * locals.var_qgdo_dn9);
        let eq53_e1709_d_n10: f64 = (p.p33 * locals.var_qgdo_dn10);
        let eq53_e1709_d_n11: f64 = (p.p33 * locals.var_qgdo_dn11);
        let eq53_e1709_d_n12: f64 = (p.p33 * locals.var_qgdo_dn12);
        let eq53_e1710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1709);
        let eq53_e1711: f64 = (p.p37 * eq53_e1710);
        let eq53_e1711_d_n3: f64 = (p.p37 * (eq53_e1709_d_n3 * ddt_scale));
        let eq53_e1711_d_n4: f64 = (p.p37 * (eq53_e1709_d_n4 * ddt_scale));
        let eq53_e1711_d_n5: f64 = (p.p37 * (eq53_e1709_d_n5 * ddt_scale));
        let eq53_e1711_d_n6: f64 = (p.p37 * (eq53_e1709_d_n6 * ddt_scale));
        let eq53_e1711_d_n7: f64 = (p.p37 * (eq53_e1709_d_n7 * ddt_scale));
        let eq53_e1711_d_n8: f64 = (p.p37 * (eq53_e1709_d_n8 * ddt_scale));
        let eq53_e1711_d_n9: f64 = (p.p37 * (eq53_e1709_d_n9 * ddt_scale));
        let eq53_e1711_d_n10: f64 = (p.p37 * (eq53_e1709_d_n10 * ddt_scale));
        let eq53_e1711_d_n11: f64 = (p.p37 * (eq53_e1709_d_n11 * ddt_scale));
        let eq53_e1711_d_n12: f64 = (p.p37 * (eq53_e1709_d_n12 * ddt_scale));
        (eq53_e1711, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1713;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq53_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq53_e1713_d_n3), multiplicity * (eq53_e1713_d_n4), multiplicity * (eq53_e1713_d_n5), multiplicity * (eq53_e1713_d_n6), multiplicity * (eq53_e1713_d_n7), multiplicity * (eq53_e1713_d_n8), multiplicity * (eq53_e1713_d_n9), multiplicity * (eq53_e1713_d_n10), multiplicity * (eq53_e1713_d_n11), multiplicity * (eq53_e1713_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq54_e1723, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12,) = {
    if (locals.var_guard1518 == 0.0) {
        let eq54_e1719: f64 = (p.p33 * locals.var_qgso);
        let eq54_e1719_d_n3: f64 = (p.p33 * locals.var_qgso_dn3);
        let eq54_e1719_d_n4: f64 = (p.p33 * locals.var_qgso_dn4);
        let eq54_e1719_d_n5: f64 = (p.p33 * locals.var_qgso_dn5);
        let eq54_e1719_d_n6: f64 = (p.p33 * locals.var_qgso_dn6);
        let eq54_e1719_d_n7: f64 = (p.p33 * locals.var_qgso_dn7);
        let eq54_e1719_d_n8: f64 = (p.p33 * locals.var_qgso_dn8);
        let eq54_e1719_d_n9: f64 = (p.p33 * locals.var_qgso_dn9);
        let eq54_e1719_d_n10: f64 = (p.p33 * locals.var_qgso_dn10);
        let eq54_e1719_d_n11: f64 = (p.p33 * locals.var_qgso_dn11);
        let eq54_e1719_d_n12: f64 = (p.p33 * locals.var_qgso_dn12);
        let eq54_e1720: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq54_e1719);
        let eq54_e1721: f64 = (p.p37 * eq54_e1720);
        let eq54_e1721_d_n3: f64 = (p.p37 * (eq54_e1719_d_n3 * ddt_scale));
        let eq54_e1721_d_n4: f64 = (p.p37 * (eq54_e1719_d_n4 * ddt_scale));
        let eq54_e1721_d_n5: f64 = (p.p37 * (eq54_e1719_d_n5 * ddt_scale));
        let eq54_e1721_d_n6: f64 = (p.p37 * (eq54_e1719_d_n6 * ddt_scale));
        let eq54_e1721_d_n7: f64 = (p.p37 * (eq54_e1719_d_n7 * ddt_scale));
        let eq54_e1721_d_n8: f64 = (p.p37 * (eq54_e1719_d_n8 * ddt_scale));
        let eq54_e1721_d_n9: f64 = (p.p37 * (eq54_e1719_d_n9 * ddt_scale));
        let eq54_e1721_d_n10: f64 = (p.p37 * (eq54_e1719_d_n10 * ddt_scale));
        let eq54_e1721_d_n11: f64 = (p.p37 * (eq54_e1719_d_n11 * ddt_scale));
        let eq54_e1721_d_n12: f64 = (p.p37 * (eq54_e1719_d_n12 * ddt_scale));
        (eq54_e1721, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e1723;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq54_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq54_e1723_d_n3), multiplicity * (eq54_e1723_d_n4), multiplicity * (eq54_e1723_d_n5), multiplicity * (eq54_e1723_d_n6), multiplicity * (eq54_e1723_d_n7), multiplicity * (eq54_e1723_d_n8), multiplicity * (eq54_e1723_d_n9), multiplicity * (eq54_e1723_d_n10), multiplicity * (eq54_e1723_d_n11), multiplicity * (eq54_e1723_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq55_e1733, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12,) = {
    if (locals.var_guard1518 == 0.0) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1730: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo);
        let eq55_e1730_d_n3: f64 = (((-p.p33) * locals.var_pparam_b4soicgeo) + (eq55_e1728 * locals.var_pparam_b4soicgeo_dn3));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn4);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn5);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn6);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn7);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn8);
        let eq55_e1730_d_n9: f64 = ((p.p33 * locals.var_pparam_b4soicgeo) + (eq55_e1728 * locals.var_pparam_b4soicgeo_dn9));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn10);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn11);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn12);
        let eq55_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq55_e1730);
        (eq55_e1731, (eq55_e1730_d_n3 * ddt_scale), (eq55_e1730_d_n4 * ddt_scale), (eq55_e1730_d_n5 * ddt_scale), (eq55_e1730_d_n6 * ddt_scale), (eq55_e1730_d_n7 * ddt_scale), (eq55_e1730_d_n8 * ddt_scale), (eq55_e1730_d_n9 * ddt_scale), (eq55_e1730_d_n10 * ddt_scale), (eq55_e1730_d_n11 * ddt_scale), (eq55_e1730_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1733;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(3),
            multiplicity * (eq55_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq55_e1733_d_n3), multiplicity * (eq55_e1733_d_n4), multiplicity * (eq55_e1733_d_n5), multiplicity * (eq55_e1733_d_n6), multiplicity * (eq55_e1733_d_n7), multiplicity * (eq55_e1733_d_n8), multiplicity * (eq55_e1733_d_n9), multiplicity * (eq55_e1733_d_n10), multiplicity * (eq55_e1733_d_n11), multiplicity * (eq55_e1733_d_n12)],
            [],
            [],
            1.0,
        );
        let eq56_e1736: f64 = (p.p33 * locals.var_b4soiqde);
        let eq56_e1736_d_n3: f64 = (p.p33 * locals.var_b4soiqde_dn3);
        let eq56_e1736_d_n4: f64 = (p.p33 * locals.var_b4soiqde_dn4);
        let eq56_e1736_d_n5: f64 = (p.p33 * locals.var_b4soiqde_dn5);
        let eq56_e1736_d_n6: f64 = (p.p33 * locals.var_b4soiqde_dn6);
        let eq56_e1736_d_n7: f64 = (p.p33 * locals.var_b4soiqde_dn7);
        let eq56_e1736_d_n8: f64 = (p.p33 * locals.var_b4soiqde_dn8);
        let eq56_e1736_d_n9: f64 = (p.p33 * locals.var_b4soiqde_dn9);
        let eq56_e1736_d_n10: f64 = (p.p33 * locals.var_b4soiqde_dn10);
        let eq56_e1736_d_n11: f64 = (p.p33 * locals.var_b4soiqde_dn11);
        let eq56_e1736_d_n12: f64 = (p.p33 * locals.var_b4soiqde_dn12);
        let eq56_e1737: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq56_e1736);
        let eq56_value: f64 = eq56_e1737;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq56_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq56_e1736_d_n3 * ddt_scale)), multiplicity * ((eq56_e1736_d_n4 * ddt_scale)), multiplicity * ((eq56_e1736_d_n5 * ddt_scale)), multiplicity * ((eq56_e1736_d_n6 * ddt_scale)), multiplicity * ((eq56_e1736_d_n7 * ddt_scale)), multiplicity * ((eq56_e1736_d_n8 * ddt_scale)), multiplicity * ((eq56_e1736_d_n9 * ddt_scale)), multiplicity * ((eq56_e1736_d_n10 * ddt_scale)), multiplicity * ((eq56_e1736_d_n11 * ddt_scale)), multiplicity * ((eq56_e1736_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq57_e1740: f64 = (p.p33 * locals.var_b4soiqse);
        let eq57_e1740_d_n3: f64 = (p.p33 * locals.var_b4soiqse_dn3);
        let eq57_e1740_d_n4: f64 = (p.p33 * locals.var_b4soiqse_dn4);
        let eq57_e1740_d_n5: f64 = (p.p33 * locals.var_b4soiqse_dn5);
        let eq57_e1740_d_n6: f64 = (p.p33 * locals.var_b4soiqse_dn6);
        let eq57_e1740_d_n7: f64 = (p.p33 * locals.var_b4soiqse_dn7);
        let eq57_e1740_d_n8: f64 = (p.p33 * locals.var_b4soiqse_dn8);
        let eq57_e1740_d_n9: f64 = (p.p33 * locals.var_b4soiqse_dn9);
        let eq57_e1740_d_n10: f64 = (p.p33 * locals.var_b4soiqse_dn10);
        let eq57_e1740_d_n11: f64 = (p.p33 * locals.var_b4soiqse_dn11);
        let eq57_e1740_d_n12: f64 = (p.p33 * locals.var_b4soiqse_dn12);
        let eq57_e1741: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq57_e1740);
        let eq57_value: f64 = eq57_e1741;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(3),
            multiplicity * (eq57_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq57_e1740_d_n3 * ddt_scale)), multiplicity * ((eq57_e1740_d_n4 * ddt_scale)), multiplicity * ((eq57_e1740_d_n5 * ddt_scale)), multiplicity * ((eq57_e1740_d_n6 * ddt_scale)), multiplicity * ((eq57_e1740_d_n7 * ddt_scale)), multiplicity * ((eq57_e1740_d_n8 * ddt_scale)), multiplicity * ((eq57_e1740_d_n9 * ddt_scale)), multiplicity * ((eq57_e1740_d_n10 * ddt_scale)), multiplicity * ((eq57_e1740_d_n11 * ddt_scale)), multiplicity * ((eq57_e1740_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq62_e1779, eq62_e1779_d_n3, eq62_e1779_d_n4, eq62_e1779_d_n5, eq62_e1779_d_n6, eq62_e1779_d_n7, eq62_e1779_d_n8, eq62_e1779_d_n9, eq62_e1779_d_n10, eq62_e1779_d_n11, eq62_e1779_d_n12,) = {
    if (locals.var_guard1520 == 0.0) {
        let eq62_e1775: f64 = (p.p32 * (nv10 - nv9));
        let eq62_e1777: f64 = (eq62_e1775 * locals.var_b4soigcrg);
        let eq62_e1777_d_n3: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn3);
        let eq62_e1777_d_n4: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn4);
        let eq62_e1777_d_n5: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn5);
        let eq62_e1777_d_n6: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn6);
        let eq62_e1777_d_n7: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn7);
        let eq62_e1777_d_n8: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn8);
        let eq62_e1777_d_n9: f64 = (((-p.p32) * locals.var_b4soigcrg) + (eq62_e1775 * locals.var_b4soigcrg_dn9));
        let eq62_e1777_d_n10: f64 = ((p.p32 * locals.var_b4soigcrg) + (eq62_e1775 * locals.var_b4soigcrg_dn10));
        let eq62_e1777_d_n11: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn11);
        let eq62_e1777_d_n12: f64 = (eq62_e1775 * locals.var_b4soigcrg_dn12);
        (eq62_e1777, eq62_e1777_d_n3, eq62_e1777_d_n4, eq62_e1777_d_n5, eq62_e1777_d_n6, eq62_e1777_d_n7, eq62_e1777_d_n8, eq62_e1777_d_n9, eq62_e1777_d_n10, eq62_e1777_d_n11, eq62_e1777_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1779;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(9),
            multiplicity * (eq62_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq62_e1779_d_n3), multiplicity * (eq62_e1779_d_n4), multiplicity * (eq62_e1779_d_n5), multiplicity * (eq62_e1779_d_n6), multiplicity * (eq62_e1779_d_n7), multiplicity * (eq62_e1779_d_n8), multiplicity * (eq62_e1779_d_n9), multiplicity * (eq62_e1779_d_n10), multiplicity * (eq62_e1779_d_n11), multiplicity * (eq62_e1779_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq71_e1869, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12,) = {
    if (((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 != 0.0)) {
        let eq71_e1856: f64 = (-locals.var_ids_1);
        let eq71_e1858: f64 = (eq71_e1856 * locals.var_vds_1);
        let eq71_e1858_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq71_e1858_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq71_e1858_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq71_e1858_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq71_e1858_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq71_e1856 * locals.var_vds_1_dn7));
        let eq71_e1858_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq71_e1856 * locals.var_vds_1_dn8));
        let eq71_e1858_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq71_e1858_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq71_e1858_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq71_e1858_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq71_e1861: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq71_e1861_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq71_e1861_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq71_e1861_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq71_e1861_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq71_e1861_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq71_e1861_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq71_e1861_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq71_e1861_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq71_e1861_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq71_e1861_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq71_e1862: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq71_e1861);
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1862);
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + (eq71_e1861_d_n3 * ddt_scale));
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + (eq71_e1861_d_n4 * ddt_scale));
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + (eq71_e1861_d_n5 * ddt_scale));
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + (eq71_e1861_d_n6 * ddt_scale));
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + (eq71_e1861_d_n7 * ddt_scale));
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + (eq71_e1861_d_n8 * ddt_scale));
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + (eq71_e1861_d_n9 * ddt_scale));
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + (eq71_e1861_d_n10 * ddt_scale));
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + (eq71_e1861_d_n11 * ddt_scale));
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + (eq71_e1861_d_n12 * ddt_scale));
        let eq71_e1866: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq71_e1866_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_0: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq71_e1866_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        (eq71_e1867, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1869;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            None,
            multiplicity * (eq71_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq71_e1869_d_n3), multiplicity * (eq71_e1869_d_n4), multiplicity * (eq71_e1869_d_n5), multiplicity * (eq71_e1869_d_n6), multiplicity * (eq71_e1869_d_n7), multiplicity * (eq71_e1869_d_n8), multiplicity * (eq71_e1869_d_n9), multiplicity * (eq71_e1869_d_n10), multiplicity * (eq71_e1869_d_n11), multiplicity * (eq71_e1869_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq72_e1892, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12,) = {
    if ((((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 == 0.0)) && (locals.var_guard1527 != 0.0)) {
        let eq72_e1879: f64 = (-locals.var_ids_1);
        let eq72_e1881: f64 = (eq72_e1879 * locals.var_vds_1);
        let eq72_e1881_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq72_e1881_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq72_e1881_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq72_e1881_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq72_e1881_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq72_e1879 * locals.var_vds_1_dn7));
        let eq72_e1881_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq72_e1879 * locals.var_vds_1_dn8));
        let eq72_e1881_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq72_e1881_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq72_e1881_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq72_e1881_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq72_e1884: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq72_e1884_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq72_e1884_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq72_e1884_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq72_e1884_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq72_e1884_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq72_e1884_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq72_e1884_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq72_e1884_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq72_e1884_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq72_e1884_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq72_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq72_e1884);
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1885);
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + (eq72_e1884_d_n3 * ddt_scale));
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + (eq72_e1884_d_n4 * ddt_scale));
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + (eq72_e1884_d_n5 * ddt_scale));
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + (eq72_e1884_d_n6 * ddt_scale));
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + (eq72_e1884_d_n7 * ddt_scale));
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + (eq72_e1884_d_n8 * ddt_scale));
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + (eq72_e1884_d_n9 * ddt_scale));
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + (eq72_e1884_d_n10 * ddt_scale));
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + (eq72_e1884_d_n11 * ddt_scale));
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + (eq72_e1884_d_n12 * ddt_scale));
        let eq72_e1889: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq72_e1889_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_1: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq72_e1889_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        (eq72_e1890, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1892;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(4),
            None,
            multiplicity * (eq72_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq72_e1892_d_n3), multiplicity * (eq72_e1892_d_n4), multiplicity * (eq72_e1892_d_n5), multiplicity * (eq72_e1892_d_n6), multiplicity * (eq72_e1892_d_n7), multiplicity * (eq72_e1892_d_n8), multiplicity * (eq72_e1892_d_n9), multiplicity * (eq72_e1892_d_n10), multiplicity * (eq72_e1892_d_n11), multiplicity * (eq72_e1892_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq73_e1920, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12,) = {
    if (((((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 == 0.0)) && (locals.var_guard1527 == 0.0)) && (locals.var_guard1528 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p30;
        let eq73_e1906: f64 = (locals.var_ids_1 * __rspice_inv_cse_2);
        let eq73_e1906_d_n3: f64 = (locals.var_ids_1_dn3 * __rspice_inv_cse_2);
        let eq73_e1906_d_n4: f64 = (locals.var_ids_1_dn4 * __rspice_inv_cse_2);
        let eq73_e1906_d_n5: f64 = (locals.var_ids_1_dn5 * __rspice_inv_cse_2);
        let eq73_e1906_d_n6: f64 = (locals.var_ids_1_dn6 * __rspice_inv_cse_2);
        let eq73_e1906_d_n7: f64 = (locals.var_ids_1_dn7 * __rspice_inv_cse_2);
        let eq73_e1906_d_n8: f64 = (locals.var_ids_1_dn8 * __rspice_inv_cse_2);
        let eq73_e1906_d_n9: f64 = (locals.var_ids_1_dn9 * __rspice_inv_cse_2);
        let eq73_e1906_d_n10: f64 = (locals.var_ids_1_dn10 * __rspice_inv_cse_2);
        let eq73_e1906_d_n11: f64 = (locals.var_ids_1_dn11 * __rspice_inv_cse_2);
        let eq73_e1906_d_n12: f64 = (locals.var_ids_1_dn12 * __rspice_inv_cse_2);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1909: f64 = (eq73_e1907 * locals.var_vds_1);
        let eq73_e1909_d_n3: f64 = ((-eq73_e1906_d_n3) * locals.var_vds_1);
        let eq73_e1909_d_n4: f64 = ((-eq73_e1906_d_n4) * locals.var_vds_1);
        let eq73_e1909_d_n5: f64 = ((-eq73_e1906_d_n5) * locals.var_vds_1);
        let eq73_e1909_d_n6: f64 = ((-eq73_e1906_d_n6) * locals.var_vds_1);
        let eq73_e1909_d_n7: f64 = (((-eq73_e1906_d_n7) * locals.var_vds_1) + (eq73_e1907 * locals.var_vds_1_dn7));
        let eq73_e1909_d_n8: f64 = (((-eq73_e1906_d_n8) * locals.var_vds_1) + (eq73_e1907 * locals.var_vds_1_dn8));
        let eq73_e1909_d_n9: f64 = ((-eq73_e1906_d_n9) * locals.var_vds_1);
        let eq73_e1909_d_n10: f64 = ((-eq73_e1906_d_n10) * locals.var_vds_1);
        let eq73_e1909_d_n11: f64 = ((-eq73_e1906_d_n11) * locals.var_vds_1);
        let eq73_e1909_d_n12: f64 = ((-eq73_e1906_d_n12) * locals.var_vds_1);
        let eq73_e1912: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq73_e1912_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq73_e1912_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq73_e1912_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq73_e1912_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq73_e1912_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq73_e1912_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq73_e1912_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq73_e1912_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq73_e1912_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq73_e1912_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq73_e1913: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq73_e1912);
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1913);
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + (eq73_e1912_d_n3 * ddt_scale));
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + (eq73_e1912_d_n4 * ddt_scale));
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + (eq73_e1912_d_n5 * ddt_scale));
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + (eq73_e1912_d_n6 * ddt_scale));
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + (eq73_e1912_d_n7 * ddt_scale));
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + (eq73_e1912_d_n8 * ddt_scale));
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + (eq73_e1912_d_n9 * ddt_scale));
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + (eq73_e1912_d_n10 * ddt_scale));
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + (eq73_e1912_d_n11 * ddt_scale));
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + (eq73_e1912_d_n12 * ddt_scale));
        let eq73_e1917: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq73_e1917_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_3: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq73_e1917_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        (eq73_e1918, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1920;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq73_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq73_e1920_d_n3), multiplicity * (eq73_e1920_d_n4), multiplicity * (eq73_e1920_d_n5), multiplicity * (eq73_e1920_d_n6), multiplicity * (eq73_e1920_d_n7), multiplicity * (eq73_e1920_d_n8), multiplicity * (eq73_e1920_d_n9), multiplicity * (eq73_e1920_d_n10), multiplicity * (eq73_e1920_d_n11), multiplicity * (eq73_e1920_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq74_e1947, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12,) = {
    if (((((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 == 0.0)) && (locals.var_guard1527 == 0.0)) && (locals.var_guard1528 == 0.0)) {
        let eq74_e1934: f64 = (-locals.var_ids_1);
        let eq74_e1936: f64 = (eq74_e1934 * locals.var_vds_1);
        let eq74_e1936_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq74_e1936_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq74_e1936_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq74_e1936_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq74_e1936_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq74_e1934 * locals.var_vds_1_dn7));
        let eq74_e1936_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq74_e1934 * locals.var_vds_1_dn8));
        let eq74_e1936_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq74_e1936_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq74_e1936_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq74_e1936_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq74_e1939: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq74_e1939_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq74_e1939_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq74_e1939_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq74_e1939_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq74_e1939_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq74_e1939_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq74_e1939_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq74_e1939_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq74_e1939_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq74_e1939_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq74_e1940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq74_e1939);
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1940);
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + (eq74_e1939_d_n3 * ddt_scale));
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + (eq74_e1939_d_n4 * ddt_scale));
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + (eq74_e1939_d_n5 * ddt_scale));
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + (eq74_e1939_d_n6 * ddt_scale));
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + (eq74_e1939_d_n7 * ddt_scale));
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + (eq74_e1939_d_n8 * ddt_scale));
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + (eq74_e1939_d_n9 * ddt_scale));
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + (eq74_e1939_d_n10 * ddt_scale));
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + (eq74_e1939_d_n11 * ddt_scale));
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + (eq74_e1939_d_n12 * ddt_scale));
        let eq74_e1944: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq74_e1944_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_4: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq74_e1944_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        (eq74_e1945, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1947;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq74_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq74_e1947_d_n3), multiplicity * (eq74_e1947_d_n4), multiplicity * (eq74_e1947_d_n5), multiplicity * (eq74_e1947_d_n6), multiplicity * (eq74_e1947_d_n7), multiplicity * (eq74_e1947_d_n8), multiplicity * (eq74_e1947_d_n9), multiplicity * (eq74_e1947_d_n10), multiplicity * (eq74_e1947_d_n11), multiplicity * (eq74_e1947_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        locals: &mut StampLocals,
    ) {
        let (eq75_e1970, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12,) = {
    if (((locals.var_guard1524 != 0.0) && (locals.var_guard1525 == 0.0)) && (locals.var_guard1529 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq75_e1956: f64 = (locals.var_ids_1 * __rspice_inv_cse_0);
        let eq75_e1956_d_n3: f64 = (locals.var_ids_1_dn3 * __rspice_inv_cse_0);
        let eq75_e1956_d_n4: f64 = (locals.var_ids_1_dn4 * __rspice_inv_cse_0);
        let eq75_e1956_d_n5: f64 = (locals.var_ids_1_dn5 * __rspice_inv_cse_0);
        let eq75_e1956_d_n6: f64 = (locals.var_ids_1_dn6 * __rspice_inv_cse_0);
        let eq75_e1956_d_n7: f64 = (locals.var_ids_1_dn7 * __rspice_inv_cse_0);
        let eq75_e1956_d_n8: f64 = (locals.var_ids_1_dn8 * __rspice_inv_cse_0);
        let eq75_e1956_d_n9: f64 = (locals.var_ids_1_dn9 * __rspice_inv_cse_0);
        let eq75_e1956_d_n10: f64 = (locals.var_ids_1_dn10 * __rspice_inv_cse_0);
        let eq75_e1956_d_n11: f64 = (locals.var_ids_1_dn11 * __rspice_inv_cse_0);
        let eq75_e1956_d_n12: f64 = (locals.var_ids_1_dn12 * __rspice_inv_cse_0);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1959: f64 = (eq75_e1957 * locals.var_vds_1);
        let eq75_e1959_d_n3: f64 = ((-eq75_e1956_d_n3) * locals.var_vds_1);
        let eq75_e1959_d_n4: f64 = ((-eq75_e1956_d_n4) * locals.var_vds_1);
        let eq75_e1959_d_n5: f64 = ((-eq75_e1956_d_n5) * locals.var_vds_1);
        let eq75_e1959_d_n6: f64 = ((-eq75_e1956_d_n6) * locals.var_vds_1);
        let eq75_e1959_d_n7: f64 = (((-eq75_e1956_d_n7) * locals.var_vds_1) + (eq75_e1957 * locals.var_vds_1_dn7));
        let eq75_e1959_d_n8: f64 = (((-eq75_e1956_d_n8) * locals.var_vds_1) + (eq75_e1957 * locals.var_vds_1_dn8));
        let eq75_e1959_d_n9: f64 = ((-eq75_e1956_d_n9) * locals.var_vds_1);
        let eq75_e1959_d_n10: f64 = ((-eq75_e1956_d_n10) * locals.var_vds_1);
        let eq75_e1959_d_n11: f64 = ((-eq75_e1956_d_n11) * locals.var_vds_1);
        let eq75_e1959_d_n12: f64 = ((-eq75_e1956_d_n12) * locals.var_vds_1);
        let eq75_e1962: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq75_e1962_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq75_e1962_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq75_e1962_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq75_e1962_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq75_e1962_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq75_e1962_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq75_e1962_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq75_e1962_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq75_e1962_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq75_e1962_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq75_e1963: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq75_e1962);
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1963);
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + (eq75_e1962_d_n3 * ddt_scale));
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + (eq75_e1962_d_n4 * ddt_scale));
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + (eq75_e1962_d_n5 * ddt_scale));
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + (eq75_e1962_d_n6 * ddt_scale));
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + (eq75_e1962_d_n7 * ddt_scale));
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + (eq75_e1962_d_n8 * ddt_scale));
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + (eq75_e1962_d_n9 * ddt_scale));
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + (eq75_e1962_d_n10 * ddt_scale));
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + (eq75_e1962_d_n11 * ddt_scale));
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + (eq75_e1962_d_n12 * ddt_scale));
        let eq75_e1967: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq75_e1967_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_1: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq75_e1967_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_1);
        let eq75_e1967_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_1);
        let eq75_e1967_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_1);
        let eq75_e1967_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        (eq75_e1968, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1970;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq75_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq75_e1970_d_n3), multiplicity * (eq75_e1970_d_n4), multiplicity * (eq75_e1970_d_n5), multiplicity * (eq75_e1970_d_n6), multiplicity * (eq75_e1970_d_n7), multiplicity * (eq75_e1970_d_n8), multiplicity * (eq75_e1970_d_n9), multiplicity * (eq75_e1970_d_n10), multiplicity * (eq75_e1970_d_n11), multiplicity * (eq75_e1970_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq76_e1992, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12,) = {
    if (((locals.var_guard1524 != 0.0) && (locals.var_guard1525 == 0.0)) && (locals.var_guard1529 == 0.0)) {
        let eq76_e1979: f64 = (-locals.var_ids_1);
        let eq76_e1981: f64 = (eq76_e1979 * locals.var_vds_1);
        let eq76_e1981_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq76_e1981_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq76_e1981_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq76_e1981_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq76_e1981_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq76_e1979 * locals.var_vds_1_dn7));
        let eq76_e1981_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq76_e1979 * locals.var_vds_1_dn8));
        let eq76_e1981_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq76_e1981_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq76_e1981_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq76_e1981_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq76_e1984: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq76_e1984_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq76_e1984_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq76_e1984_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq76_e1984_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq76_e1984_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq76_e1984_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq76_e1984_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq76_e1984_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq76_e1984_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq76_e1984_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq76_e1985: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq76_e1984);
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1985);
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + (eq76_e1984_d_n3 * ddt_scale));
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + (eq76_e1984_d_n4 * ddt_scale));
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + (eq76_e1984_d_n5 * ddt_scale));
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + (eq76_e1984_d_n6 * ddt_scale));
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + (eq76_e1984_d_n7 * ddt_scale));
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + (eq76_e1984_d_n8 * ddt_scale));
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + (eq76_e1984_d_n9 * ddt_scale));
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + (eq76_e1984_d_n10 * ddt_scale));
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + (eq76_e1984_d_n11 * ddt_scale));
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + (eq76_e1984_d_n12 * ddt_scale));
        let eq76_e1989: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq76_e1989_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_2: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq76_e1989_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_2);
        let eq76_e1989_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_2);
        let eq76_e1989_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_2);
        let eq76_e1989_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        (eq76_e1990, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1992;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq76_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq76_e1992_d_n3), multiplicity * (eq76_e1992_d_n4), multiplicity * (eq76_e1992_d_n5), multiplicity * (eq76_e1992_d_n6), multiplicity * (eq76_e1992_d_n7), multiplicity * (eq76_e1992_d_n8), multiplicity * (eq76_e1992_d_n9), multiplicity * (eq76_e1992_d_n10), multiplicity * (eq76_e1992_d_n11), multiplicity * (eq76_e1992_d_n12)],
            [],
            [],
            1.0,
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq14_e1376, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13, eq14_e1376_q,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * locals.var_c0);
        let eq14_e1369_d_n3: f64 = (eq14_e1367 * locals.var_c0_dn3);
        let eq14_e1369_d_n4: f64 = (eq14_e1367 * locals.var_c0_dn4);
        let eq14_e1369_d_n5: f64 = (eq14_e1367 * locals.var_c0_dn5);
        let eq14_e1369_d_n6: f64 = (eq14_e1367 * locals.var_c0_dn6);
        let eq14_e1369_d_n7: f64 = (eq14_e1367 * locals.var_c0_dn7);
        let eq14_e1369_d_n8: f64 = (eq14_e1367 * locals.var_c0_dn8);
        let eq14_e1369_d_n9: f64 = (eq14_e1367 * locals.var_c0_dn9);
        let eq14_e1369_d_n10: f64 = (eq14_e1367 * locals.var_c0_dn10);
        let eq14_e1369_d_n11: f64 = (eq14_e1367 * locals.var_c0_dn11);
        let eq14_e1369_d_n12: f64 = (eq14_e1367 * locals.var_c0_dn12);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n3: f64 = (eq14_e1369_d_n3 * p.p226);
        let eq14_e1371_d_n4: f64 = (eq14_e1369_d_n4 * p.p226);
        let eq14_e1371_d_n5: f64 = (eq14_e1369_d_n5 * p.p226);
        let eq14_e1371_d_n6: f64 = (eq14_e1369_d_n6 * p.p226);
        let eq14_e1371_d_n7: f64 = (eq14_e1369_d_n7 * p.p226);
        let eq14_e1371_d_n8: f64 = (eq14_e1369_d_n8 * p.p226);
        let eq14_e1371_d_n9: f64 = (eq14_e1369_d_n9 * p.p226);
        let eq14_e1371_d_n10: f64 = (eq14_e1369_d_n10 * p.p226);
        let eq14_e1371_d_n11: f64 = (eq14_e1369_d_n11 * p.p226);
        let eq14_e1371_d_n12: f64 = (eq14_e1369_d_n12 * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1374_q: f64 = eq14_e1373;
        (eq14_e1373, eq14_e1373_d_n3, eq14_e1373_d_n4, eq14_e1373_d_n5, eq14_e1373_d_n6, eq14_e1373_d_n7, eq14_e1373_d_n8, eq14_e1373_d_n9, eq14_e1373_d_n10, eq14_e1373_d_n11, eq14_e1373_d_n12, eq14_e1371, eq14_e1374_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1396, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13, eq15_e1396_q,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * locals.var_c0);
        let eq15_e1389_d_n3: f64 = (eq15_e1387 * locals.var_c0_dn3);
        let eq15_e1389_d_n4: f64 = (eq15_e1387 * locals.var_c0_dn4);
        let eq15_e1389_d_n5: f64 = (eq15_e1387 * locals.var_c0_dn5);
        let eq15_e1389_d_n6: f64 = (eq15_e1387 * locals.var_c0_dn6);
        let eq15_e1389_d_n7: f64 = (eq15_e1387 * locals.var_c0_dn7);
        let eq15_e1389_d_n8: f64 = (eq15_e1387 * locals.var_c0_dn8);
        let eq15_e1389_d_n9: f64 = (eq15_e1387 * locals.var_c0_dn9);
        let eq15_e1389_d_n10: f64 = (eq15_e1387 * locals.var_c0_dn10);
        let eq15_e1389_d_n11: f64 = (eq15_e1387 * locals.var_c0_dn11);
        let eq15_e1389_d_n12: f64 = (eq15_e1387 * locals.var_c0_dn12);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n3: f64 = (eq15_e1389_d_n3 * p.p226);
        let eq15_e1391_d_n4: f64 = (eq15_e1389_d_n4 * p.p226);
        let eq15_e1391_d_n5: f64 = (eq15_e1389_d_n5 * p.p226);
        let eq15_e1391_d_n6: f64 = (eq15_e1389_d_n6 * p.p226);
        let eq15_e1391_d_n7: f64 = (eq15_e1389_d_n7 * p.p226);
        let eq15_e1391_d_n8: f64 = (eq15_e1389_d_n8 * p.p226);
        let eq15_e1391_d_n9: f64 = (eq15_e1389_d_n9 * p.p226);
        let eq15_e1391_d_n10: f64 = (eq15_e1389_d_n10 * p.p226);
        let eq15_e1391_d_n11: f64 = (eq15_e1389_d_n11 * p.p226);
        let eq15_e1391_d_n12: f64 = (eq15_e1389_d_n12 * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1394_q: f64 = eq15_e1393;
        (eq15_e1393, eq15_e1393_d_n3, eq15_e1393_d_n4, eq15_e1393_d_n5, eq15_e1393_d_n6, eq15_e1393_d_n7, eq15_e1393_d_n8, eq15_e1393_d_n9, eq15_e1393_d_n10, eq15_e1393_d_n11, eq15_e1393_d_n12, eq15_e1391, eq15_e1394_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1647: f64 = (p.p33 * locals.var_b4soiqdrn);
        let eq44_e1647_d_n3: f64 = (p.p33 * locals.var_b4soiqdrn_dn3);
        let eq44_e1647_d_n4: f64 = (p.p33 * locals.var_b4soiqdrn_dn4);
        let eq44_e1647_d_n5: f64 = (p.p33 * locals.var_b4soiqdrn_dn5);
        let eq44_e1647_d_n6: f64 = (p.p33 * locals.var_b4soiqdrn_dn6);
        let eq44_e1647_d_n7: f64 = (p.p33 * locals.var_b4soiqdrn_dn7);
        let eq44_e1647_d_n8: f64 = (p.p33 * locals.var_b4soiqdrn_dn8);
        let eq44_e1647_d_n9: f64 = (p.p33 * locals.var_b4soiqdrn_dn9);
        let eq44_e1647_d_n10: f64 = (p.p33 * locals.var_b4soiqdrn_dn10);
        let eq44_e1647_d_n11: f64 = (p.p33 * locals.var_b4soiqdrn_dn11);
        let eq44_e1647_d_n12: f64 = (p.p33 * locals.var_b4soiqdrn_dn12);
        let eq44_e1648_q: f64 = eq44_e1647;
        let eq44_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq44_e1647_d_n3, eq44_e1647_d_n4, eq44_e1647_d_n5, eq44_e1647_d_n6, eq44_e1647_d_n7, eq44_e1647_d_n8, eq44_e1647_d_n9, eq44_e1647_d_n10, eq44_e1647_d_n11, eq44_e1647_d_n12, 0.0];
        let eq44_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1651: f64 = (p.p33 * locals.var_b4soiqsrc);
        let eq45_e1651_d_n3: f64 = (p.p33 * locals.var_b4soiqsrc_dn3);
        let eq45_e1651_d_n4: f64 = (p.p33 * locals.var_b4soiqsrc_dn4);
        let eq45_e1651_d_n5: f64 = (p.p33 * locals.var_b4soiqsrc_dn5);
        let eq45_e1651_d_n6: f64 = (p.p33 * locals.var_b4soiqsrc_dn6);
        let eq45_e1651_d_n7: f64 = (p.p33 * locals.var_b4soiqsrc_dn7);
        let eq45_e1651_d_n8: f64 = (p.p33 * locals.var_b4soiqsrc_dn8);
        let eq45_e1651_d_n9: f64 = (p.p33 * locals.var_b4soiqsrc_dn9);
        let eq45_e1651_d_n10: f64 = (p.p33 * locals.var_b4soiqsrc_dn10);
        let eq45_e1651_d_n11: f64 = (p.p33 * locals.var_b4soiqsrc_dn11);
        let eq45_e1651_d_n12: f64 = (p.p33 * locals.var_b4soiqsrc_dn12);
        let eq45_e1652_q: f64 = eq45_e1651;
        let eq45_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq45_e1651_d_n3, eq45_e1651_d_n4, eq45_e1651_d_n5, eq45_e1651_d_n6, eq45_e1651_d_n7, eq45_e1651_d_n8, eq45_e1651_d_n9, eq45_e1651_d_n10, eq45_e1651_d_n11, eq45_e1651_d_n12, 0.0];
        let eq45_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1656: f64 = (p.p33 * locals.var_qgate);
        let eq46_e1656_d_n3: f64 = (p.p33 * locals.var_qgate_dn3);
        let eq46_e1656_d_n4: f64 = (p.p33 * locals.var_qgate_dn4);
        let eq46_e1656_d_n5: f64 = (p.p33 * locals.var_qgate_dn5);
        let eq46_e1656_d_n6: f64 = (p.p33 * locals.var_qgate_dn6);
        let eq46_e1656_d_n7: f64 = (p.p33 * locals.var_qgate_dn7);
        let eq46_e1656_d_n8: f64 = (p.p33 * locals.var_qgate_dn8);
        let eq46_e1656_d_n9: f64 = (p.p33 * locals.var_qgate_dn9);
        let eq46_e1656_d_n10: f64 = (p.p33 * locals.var_qgate_dn10);
        let eq46_e1656_d_n11: f64 = (p.p33 * locals.var_qgate_dn11);
        let eq46_e1656_d_n12: f64 = (p.p33 * locals.var_qgate_dn12);
        let eq46_e1657_q: f64 = eq46_e1656;
        let eq46_e1658: f64 = (p.p37 * eq46_e1656);
        let eq46_e1658_d_n3: f64 = (p.p37 * eq46_e1656_d_n3);
        let eq46_e1658_d_n4: f64 = (p.p37 * eq46_e1656_d_n4);
        let eq46_e1658_d_n5: f64 = (p.p37 * eq46_e1656_d_n5);
        let eq46_e1658_d_n6: f64 = (p.p37 * eq46_e1656_d_n6);
        let eq46_e1658_d_n7: f64 = (p.p37 * eq46_e1656_d_n7);
        let eq46_e1658_d_n8: f64 = (p.p37 * eq46_e1656_d_n8);
        let eq46_e1658_d_n9: f64 = (p.p37 * eq46_e1656_d_n9);
        let eq46_e1658_d_n10: f64 = (p.p37 * eq46_e1656_d_n10);
        let eq46_e1658_d_n11: f64 = (p.p37 * eq46_e1656_d_n11);
        let eq46_e1658_d_n12: f64 = (p.p37 * eq46_e1656_d_n12);
        let eq46_e1658_q: f64 = (p.p37 * eq46_e1657_q);
        let eq46_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq46_e1658_d_n3, eq46_e1658_d_n4, eq46_e1658_d_n5, eq46_e1658_d_n6, eq46_e1658_d_n7, eq46_e1658_d_n8, eq46_e1658_d_n9, eq46_e1658_d_n10, eq46_e1658_d_n11, eq46_e1658_d_n12, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1662: f64 = (p.p33 * locals.var_qsub);
        let eq47_e1662_d_n3: f64 = (p.p33 * locals.var_qsub_dn3);
        let eq47_e1662_d_n4: f64 = (p.p33 * locals.var_qsub_dn4);
        let eq47_e1662_d_n5: f64 = (p.p33 * locals.var_qsub_dn5);
        let eq47_e1662_d_n6: f64 = (p.p33 * locals.var_qsub_dn6);
        let eq47_e1662_d_n7: f64 = (p.p33 * locals.var_qsub_dn7);
        let eq47_e1662_d_n8: f64 = (p.p33 * locals.var_qsub_dn8);
        let eq47_e1662_d_n9: f64 = (p.p33 * locals.var_qsub_dn9);
        let eq47_e1662_d_n10: f64 = (p.p33 * locals.var_qsub_dn10);
        let eq47_e1662_d_n11: f64 = (p.p33 * locals.var_qsub_dn11);
        let eq47_e1662_d_n12: f64 = (p.p33 * locals.var_qsub_dn12);
        let eq47_e1663_q: f64 = eq47_e1662;
        let eq47_e1664: f64 = (p.p37 * eq47_e1662);
        let eq47_e1664_d_n3: f64 = (p.p37 * eq47_e1662_d_n3);
        let eq47_e1664_d_n4: f64 = (p.p37 * eq47_e1662_d_n4);
        let eq47_e1664_d_n5: f64 = (p.p37 * eq47_e1662_d_n5);
        let eq47_e1664_d_n6: f64 = (p.p37 * eq47_e1662_d_n6);
        let eq47_e1664_d_n7: f64 = (p.p37 * eq47_e1662_d_n7);
        let eq47_e1664_d_n8: f64 = (p.p37 * eq47_e1662_d_n8);
        let eq47_e1664_d_n9: f64 = (p.p37 * eq47_e1662_d_n9);
        let eq47_e1664_d_n10: f64 = (p.p37 * eq47_e1662_d_n10);
        let eq47_e1664_d_n11: f64 = (p.p37 * eq47_e1662_d_n11);
        let eq47_e1664_d_n12: f64 = (p.p37 * eq47_e1662_d_n12);
        let eq47_e1664_q: f64 = (p.p37 * eq47_e1663_q);
        let eq47_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq47_e1664_d_n3, eq47_e1664_d_n4, eq47_e1664_d_n5, eq47_e1664_d_n6, eq47_e1664_d_n7, eq47_e1664_d_n8, eq47_e1664_d_n9, eq47_e1664_d_n10, eq47_e1664_d_n11, eq47_e1664_d_n12, 0.0];
        let eq47_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1668: f64 = (p.p33 * locals.var_qjd_1);
        let eq48_e1668_d_n3: f64 = (p.p33 * locals.var_qjd_1_dn3);
        let eq48_e1668_d_n4: f64 = (p.p33 * locals.var_qjd_1_dn4);
        let eq48_e1668_d_n5: f64 = (p.p33 * locals.var_qjd_1_dn5);
        let eq48_e1668_d_n6: f64 = (p.p33 * locals.var_qjd_1_dn6);
        let eq48_e1668_d_n7: f64 = (p.p33 * locals.var_qjd_1_dn7);
        let eq48_e1668_d_n8: f64 = (p.p33 * locals.var_qjd_1_dn8);
        let eq48_e1668_d_n9: f64 = (p.p33 * locals.var_qjd_1_dn9);
        let eq48_e1668_d_n10: f64 = (p.p33 * locals.var_qjd_1_dn10);
        let eq48_e1668_d_n11: f64 = (p.p33 * locals.var_qjd_1_dn11);
        let eq48_e1668_d_n12: f64 = (p.p33 * locals.var_qjd_1_dn12);
        let eq48_e1669_q: f64 = eq48_e1668;
        let eq48_e1670: f64 = (p.p37 * eq48_e1668);
        let eq48_e1670_d_n3: f64 = (p.p37 * eq48_e1668_d_n3);
        let eq48_e1670_d_n4: f64 = (p.p37 * eq48_e1668_d_n4);
        let eq48_e1670_d_n5: f64 = (p.p37 * eq48_e1668_d_n5);
        let eq48_e1670_d_n6: f64 = (p.p37 * eq48_e1668_d_n6);
        let eq48_e1670_d_n7: f64 = (p.p37 * eq48_e1668_d_n7);
        let eq48_e1670_d_n8: f64 = (p.p37 * eq48_e1668_d_n8);
        let eq48_e1670_d_n9: f64 = (p.p37 * eq48_e1668_d_n9);
        let eq48_e1670_d_n10: f64 = (p.p37 * eq48_e1668_d_n10);
        let eq48_e1670_d_n11: f64 = (p.p37 * eq48_e1668_d_n11);
        let eq48_e1670_d_n12: f64 = (p.p37 * eq48_e1668_d_n12);
        let eq48_e1670_q: f64 = (p.p37 * eq48_e1669_q);
        let eq48_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq48_e1670_d_n3, eq48_e1670_d_n4, eq48_e1670_d_n5, eq48_e1670_d_n6, eq48_e1670_d_n7, eq48_e1670_d_n8, eq48_e1670_d_n9, eq48_e1670_d_n10, eq48_e1670_d_n11, eq48_e1670_d_n12, 0.0];
        let eq48_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq49_e1674: f64 = (p.p33 * locals.var_qjs_1);
        let eq49_e1674_d_n3: f64 = (p.p33 * locals.var_qjs_1_dn3);
        let eq49_e1674_d_n4: f64 = (p.p33 * locals.var_qjs_1_dn4);
        let eq49_e1674_d_n5: f64 = (p.p33 * locals.var_qjs_1_dn5);
        let eq49_e1674_d_n6: f64 = (p.p33 * locals.var_qjs_1_dn6);
        let eq49_e1674_d_n7: f64 = (p.p33 * locals.var_qjs_1_dn7);
        let eq49_e1674_d_n8: f64 = (p.p33 * locals.var_qjs_1_dn8);
        let eq49_e1674_d_n9: f64 = (p.p33 * locals.var_qjs_1_dn9);
        let eq49_e1674_d_n10: f64 = (p.p33 * locals.var_qjs_1_dn10);
        let eq49_e1674_d_n11: f64 = (p.p33 * locals.var_qjs_1_dn11);
        let eq49_e1674_d_n12: f64 = (p.p33 * locals.var_qjs_1_dn12);
        let eq49_e1675_q: f64 = eq49_e1674;
        let eq49_e1676: f64 = (p.p37 * eq49_e1674);
        let eq49_e1676_d_n3: f64 = (p.p37 * eq49_e1674_d_n3);
        let eq49_e1676_d_n4: f64 = (p.p37 * eq49_e1674_d_n4);
        let eq49_e1676_d_n5: f64 = (p.p37 * eq49_e1674_d_n5);
        let eq49_e1676_d_n6: f64 = (p.p37 * eq49_e1674_d_n6);
        let eq49_e1676_d_n7: f64 = (p.p37 * eq49_e1674_d_n7);
        let eq49_e1676_d_n8: f64 = (p.p37 * eq49_e1674_d_n8);
        let eq49_e1676_d_n9: f64 = (p.p37 * eq49_e1674_d_n9);
        let eq49_e1676_d_n10: f64 = (p.p37 * eq49_e1674_d_n10);
        let eq49_e1676_d_n11: f64 = (p.p37 * eq49_e1674_d_n11);
        let eq49_e1676_d_n12: f64 = (p.p37 * eq49_e1674_d_n12);
        let eq49_e1676_q: f64 = (p.p37 * eq49_e1675_q);
        let eq49_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq49_e1676_d_n3, eq49_e1676_d_n4, eq49_e1676_d_n5, eq49_e1676_d_n6, eq49_e1676_d_n7, eq49_e1676_d_n8, eq49_e1676_d_n9, eq49_e1676_d_n10, eq49_e1676_d_n11, eq49_e1676_d_n12, 0.0];
        let eq49_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1685, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_q,) = {
    if (locals.var_guard1518 != 0.0) {
        let eq50_e1681: f64 = (p.p33 * locals.var_qgdo);
        let eq50_e1681_d_n3: f64 = (p.p33 * locals.var_qgdo_dn3);
        let eq50_e1681_d_n4: f64 = (p.p33 * locals.var_qgdo_dn4);
        let eq50_e1681_d_n5: f64 = (p.p33 * locals.var_qgdo_dn5);
        let eq50_e1681_d_n6: f64 = (p.p33 * locals.var_qgdo_dn6);
        let eq50_e1681_d_n7: f64 = (p.p33 * locals.var_qgdo_dn7);
        let eq50_e1681_d_n8: f64 = (p.p33 * locals.var_qgdo_dn8);
        let eq50_e1681_d_n9: f64 = (p.p33 * locals.var_qgdo_dn9);
        let eq50_e1681_d_n10: f64 = (p.p33 * locals.var_qgdo_dn10);
        let eq50_e1681_d_n11: f64 = (p.p33 * locals.var_qgdo_dn11);
        let eq50_e1681_d_n12: f64 = (p.p33 * locals.var_qgdo_dn12);
        let eq50_e1682_q: f64 = eq50_e1681;
        let eq50_e1683: f64 = (p.p37 * eq50_e1681);
        let eq50_e1683_d_n3: f64 = (p.p37 * eq50_e1681_d_n3);
        let eq50_e1683_d_n4: f64 = (p.p37 * eq50_e1681_d_n4);
        let eq50_e1683_d_n5: f64 = (p.p37 * eq50_e1681_d_n5);
        let eq50_e1683_d_n6: f64 = (p.p37 * eq50_e1681_d_n6);
        let eq50_e1683_d_n7: f64 = (p.p37 * eq50_e1681_d_n7);
        let eq50_e1683_d_n8: f64 = (p.p37 * eq50_e1681_d_n8);
        let eq50_e1683_d_n9: f64 = (p.p37 * eq50_e1681_d_n9);
        let eq50_e1683_d_n10: f64 = (p.p37 * eq50_e1681_d_n10);
        let eq50_e1683_d_n11: f64 = (p.p37 * eq50_e1681_d_n11);
        let eq50_e1683_d_n12: f64 = (p.p37 * eq50_e1681_d_n12);
        let eq50_e1683_q: f64 = (p.p37 * eq50_e1682_q);
        (eq50_e1683, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12, eq50_e1683_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, 0.0];
        let eq50_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1694, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_q,) = {
    if (locals.var_guard1518 != 0.0) {
        let eq51_e1690: f64 = (p.p33 * locals.var_qgso);
        let eq51_e1690_d_n3: f64 = (p.p33 * locals.var_qgso_dn3);
        let eq51_e1690_d_n4: f64 = (p.p33 * locals.var_qgso_dn4);
        let eq51_e1690_d_n5: f64 = (p.p33 * locals.var_qgso_dn5);
        let eq51_e1690_d_n6: f64 = (p.p33 * locals.var_qgso_dn6);
        let eq51_e1690_d_n7: f64 = (p.p33 * locals.var_qgso_dn7);
        let eq51_e1690_d_n8: f64 = (p.p33 * locals.var_qgso_dn8);
        let eq51_e1690_d_n9: f64 = (p.p33 * locals.var_qgso_dn9);
        let eq51_e1690_d_n10: f64 = (p.p33 * locals.var_qgso_dn10);
        let eq51_e1690_d_n11: f64 = (p.p33 * locals.var_qgso_dn11);
        let eq51_e1690_d_n12: f64 = (p.p33 * locals.var_qgso_dn12);
        let eq51_e1691_q: f64 = eq51_e1690;
        let eq51_e1692: f64 = (p.p37 * eq51_e1690);
        let eq51_e1692_d_n3: f64 = (p.p37 * eq51_e1690_d_n3);
        let eq51_e1692_d_n4: f64 = (p.p37 * eq51_e1690_d_n4);
        let eq51_e1692_d_n5: f64 = (p.p37 * eq51_e1690_d_n5);
        let eq51_e1692_d_n6: f64 = (p.p37 * eq51_e1690_d_n6);
        let eq51_e1692_d_n7: f64 = (p.p37 * eq51_e1690_d_n7);
        let eq51_e1692_d_n8: f64 = (p.p37 * eq51_e1690_d_n8);
        let eq51_e1692_d_n9: f64 = (p.p37 * eq51_e1690_d_n9);
        let eq51_e1692_d_n10: f64 = (p.p37 * eq51_e1690_d_n10);
        let eq51_e1692_d_n11: f64 = (p.p37 * eq51_e1690_d_n11);
        let eq51_e1692_d_n12: f64 = (p.p37 * eq51_e1690_d_n12);
        let eq51_e1692_q: f64 = (p.p37 * eq51_e1691_q);
        (eq51_e1692, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12, eq51_e1692_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e1703, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_q,) = {
    if (locals.var_guard1518 != 0.0) {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1700: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo);
        let eq52_e1700_d_n3: f64 = (((-p.p33) * locals.var_pparam_b4soicgeo) + (eq52_e1698 * locals.var_pparam_b4soicgeo_dn3));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn4);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn5);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn6);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn7);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn8);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn9);
        let eq52_e1700_d_n10: f64 = ((p.p33 * locals.var_pparam_b4soicgeo) + (eq52_e1698 * locals.var_pparam_b4soicgeo_dn10));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn11);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * locals.var_pparam_b4soicgeo_dn12);
        let eq52_e1701_q: f64 = eq52_e1700;
        (eq52_e1700, eq52_e1700_d_n3, eq52_e1700_d_n4, eq52_e1700_d_n5, eq52_e1700_d_n6, eq52_e1700_d_n7, eq52_e1700_d_n8, eq52_e1700_d_n9, eq52_e1700_d_n10, eq52_e1700_d_n11, eq52_e1700_d_n12, eq52_e1701_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, 0.0];
        let eq52_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1713, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_q,) = {
    if (locals.var_guard1518 == 0.0) {
        let eq53_e1709: f64 = (p.p33 * locals.var_qgdo);
        let eq53_e1709_d_n3: f64 = (p.p33 * locals.var_qgdo_dn3);
        let eq53_e1709_d_n4: f64 = (p.p33 * locals.var_qgdo_dn4);
        let eq53_e1709_d_n5: f64 = (p.p33 * locals.var_qgdo_dn5);
        let eq53_e1709_d_n6: f64 = (p.p33 * locals.var_qgdo_dn6);
        let eq53_e1709_d_n7: f64 = (p.p33 * locals.var_qgdo_dn7);
        let eq53_e1709_d_n8: f64 = (p.p33 * locals.var_qgdo_dn8);
        let eq53_e1709_d_n9: f64 = (p.p33 * locals.var_qgdo_dn9);
        let eq53_e1709_d_n10: f64 = (p.p33 * locals.var_qgdo_dn10);
        let eq53_e1709_d_n11: f64 = (p.p33 * locals.var_qgdo_dn11);
        let eq53_e1709_d_n12: f64 = (p.p33 * locals.var_qgdo_dn12);
        let eq53_e1710_q: f64 = eq53_e1709;
        let eq53_e1711: f64 = (p.p37 * eq53_e1709);
        let eq53_e1711_d_n3: f64 = (p.p37 * eq53_e1709_d_n3);
        let eq53_e1711_d_n4: f64 = (p.p37 * eq53_e1709_d_n4);
        let eq53_e1711_d_n5: f64 = (p.p37 * eq53_e1709_d_n5);
        let eq53_e1711_d_n6: f64 = (p.p37 * eq53_e1709_d_n6);
        let eq53_e1711_d_n7: f64 = (p.p37 * eq53_e1709_d_n7);
        let eq53_e1711_d_n8: f64 = (p.p37 * eq53_e1709_d_n8);
        let eq53_e1711_d_n9: f64 = (p.p37 * eq53_e1709_d_n9);
        let eq53_e1711_d_n10: f64 = (p.p37 * eq53_e1709_d_n10);
        let eq53_e1711_d_n11: f64 = (p.p37 * eq53_e1709_d_n11);
        let eq53_e1711_d_n12: f64 = (p.p37 * eq53_e1709_d_n12);
        let eq53_e1711_q: f64 = (p.p37 * eq53_e1710_q);
        (eq53_e1711, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e1723, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_q,) = {
    if (locals.var_guard1518 == 0.0) {
        let eq54_e1719: f64 = (p.p33 * locals.var_qgso);
        let eq54_e1719_d_n3: f64 = (p.p33 * locals.var_qgso_dn3);
        let eq54_e1719_d_n4: f64 = (p.p33 * locals.var_qgso_dn4);
        let eq54_e1719_d_n5: f64 = (p.p33 * locals.var_qgso_dn5);
        let eq54_e1719_d_n6: f64 = (p.p33 * locals.var_qgso_dn6);
        let eq54_e1719_d_n7: f64 = (p.p33 * locals.var_qgso_dn7);
        let eq54_e1719_d_n8: f64 = (p.p33 * locals.var_qgso_dn8);
        let eq54_e1719_d_n9: f64 = (p.p33 * locals.var_qgso_dn9);
        let eq54_e1719_d_n10: f64 = (p.p33 * locals.var_qgso_dn10);
        let eq54_e1719_d_n11: f64 = (p.p33 * locals.var_qgso_dn11);
        let eq54_e1719_d_n12: f64 = (p.p33 * locals.var_qgso_dn12);
        let eq54_e1720_q: f64 = eq54_e1719;
        let eq54_e1721: f64 = (p.p37 * eq54_e1719);
        let eq54_e1721_d_n3: f64 = (p.p37 * eq54_e1719_d_n3);
        let eq54_e1721_d_n4: f64 = (p.p37 * eq54_e1719_d_n4);
        let eq54_e1721_d_n5: f64 = (p.p37 * eq54_e1719_d_n5);
        let eq54_e1721_d_n6: f64 = (p.p37 * eq54_e1719_d_n6);
        let eq54_e1721_d_n7: f64 = (p.p37 * eq54_e1719_d_n7);
        let eq54_e1721_d_n8: f64 = (p.p37 * eq54_e1719_d_n8);
        let eq54_e1721_d_n9: f64 = (p.p37 * eq54_e1719_d_n9);
        let eq54_e1721_d_n10: f64 = (p.p37 * eq54_e1719_d_n10);
        let eq54_e1721_d_n11: f64 = (p.p37 * eq54_e1719_d_n11);
        let eq54_e1721_d_n12: f64 = (p.p37 * eq54_e1719_d_n12);
        let eq54_e1721_q: f64 = (p.p37 * eq54_e1720_q);
        (eq54_e1721, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, 0.0];
        let eq54_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq55_e1733, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_q,) = {
    if (locals.var_guard1518 == 0.0) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1730: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo);
        let eq55_e1730_d_n3: f64 = (((-p.p33) * locals.var_pparam_b4soicgeo) + (eq55_e1728 * locals.var_pparam_b4soicgeo_dn3));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn4);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn5);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn6);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn7);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn8);
        let eq55_e1730_d_n9: f64 = ((p.p33 * locals.var_pparam_b4soicgeo) + (eq55_e1728 * locals.var_pparam_b4soicgeo_dn9));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn10);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn11);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * locals.var_pparam_b4soicgeo_dn12);
        let eq55_e1731_q: f64 = eq55_e1730;
        (eq55_e1730, eq55_e1730_d_n3, eq55_e1730_d_n4, eq55_e1730_d_n5, eq55_e1730_d_n6, eq55_e1730_d_n7, eq55_e1730_d_n8, eq55_e1730_d_n9, eq55_e1730_d_n10, eq55_e1730_d_n11, eq55_e1730_d_n12, eq55_e1731_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, 0.0];
        let eq55_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e1736: f64 = (p.p33 * locals.var_b4soiqde);
        let eq56_e1736_d_n3: f64 = (p.p33 * locals.var_b4soiqde_dn3);
        let eq56_e1736_d_n4: f64 = (p.p33 * locals.var_b4soiqde_dn4);
        let eq56_e1736_d_n5: f64 = (p.p33 * locals.var_b4soiqde_dn5);
        let eq56_e1736_d_n6: f64 = (p.p33 * locals.var_b4soiqde_dn6);
        let eq56_e1736_d_n7: f64 = (p.p33 * locals.var_b4soiqde_dn7);
        let eq56_e1736_d_n8: f64 = (p.p33 * locals.var_b4soiqde_dn8);
        let eq56_e1736_d_n9: f64 = (p.p33 * locals.var_b4soiqde_dn9);
        let eq56_e1736_d_n10: f64 = (p.p33 * locals.var_b4soiqde_dn10);
        let eq56_e1736_d_n11: f64 = (p.p33 * locals.var_b4soiqde_dn11);
        let eq56_e1736_d_n12: f64 = (p.p33 * locals.var_b4soiqde_dn12);
        let eq56_e1737_q: f64 = eq56_e1736;
        let eq56_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq56_e1736_d_n3, eq56_e1736_d_n4, eq56_e1736_d_n5, eq56_e1736_d_n6, eq56_e1736_d_n7, eq56_e1736_d_n8, eq56_e1736_d_n9, eq56_e1736_d_n10, eq56_e1736_d_n11, eq56_e1736_d_n12, 0.0];
        let eq56_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let eq57_e1740: f64 = (p.p33 * locals.var_b4soiqse);
        let eq57_e1740_d_n3: f64 = (p.p33 * locals.var_b4soiqse_dn3);
        let eq57_e1740_d_n4: f64 = (p.p33 * locals.var_b4soiqse_dn4);
        let eq57_e1740_d_n5: f64 = (p.p33 * locals.var_b4soiqse_dn5);
        let eq57_e1740_d_n6: f64 = (p.p33 * locals.var_b4soiqse_dn6);
        let eq57_e1740_d_n7: f64 = (p.p33 * locals.var_b4soiqse_dn7);
        let eq57_e1740_d_n8: f64 = (p.p33 * locals.var_b4soiqse_dn8);
        let eq57_e1740_d_n9: f64 = (p.p33 * locals.var_b4soiqse_dn9);
        let eq57_e1740_d_n10: f64 = (p.p33 * locals.var_b4soiqse_dn10);
        let eq57_e1740_d_n11: f64 = (p.p33 * locals.var_b4soiqse_dn11);
        let eq57_e1740_d_n12: f64 = (p.p33 * locals.var_b4soiqse_dn12);
        let eq57_e1741_q: f64 = eq57_e1740;
        let eq57_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq57_e1740_d_n3, eq57_e1740_d_n4, eq57_e1740_d_n5, eq57_e1740_d_n6, eq57_e1740_d_n7, eq57_e1740_d_n8, eq57_e1740_d_n9, eq57_e1740_d_n10, eq57_e1740_d_n11, eq57_e1740_d_n12, 0.0];
        let eq57_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &eq57_reactive_node_derivatives,
            branches,
            &eq57_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1869, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_q, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12,) = {
    if (((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 != 0.0)) {
        let eq71_e1856: f64 = (-locals.var_ids_1);
        let eq71_e1858: f64 = (eq71_e1856 * locals.var_vds_1);
        let eq71_e1858_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq71_e1858_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq71_e1858_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq71_e1858_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq71_e1858_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq71_e1856 * locals.var_vds_1_dn7));
        let eq71_e1858_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq71_e1856 * locals.var_vds_1_dn8));
        let eq71_e1858_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq71_e1858_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq71_e1858_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq71_e1858_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq71_e1861: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq71_e1861_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq71_e1861_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq71_e1861_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq71_e1861_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq71_e1861_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq71_e1861_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq71_e1861_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq71_e1861_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq71_e1861_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq71_e1861_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq71_e1862_q: f64 = eq71_e1861;
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1861);
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + eq71_e1861_d_n3);
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + eq71_e1861_d_n4);
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + eq71_e1861_d_n5);
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + eq71_e1861_d_n6);
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + eq71_e1861_d_n7);
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + eq71_e1861_d_n8);
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + eq71_e1861_d_n9);
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + eq71_e1861_d_n10);
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + eq71_e1861_d_n11);
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + eq71_e1861_d_n12);
        let eq71_e1863_q: f64 = eq71_e1862_q;
        let eq71_e1866: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq71_e1866_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_0: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq71_e1866_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1866_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        let eq71_e1867_q: f64 = eq71_e1863_q;
        (eq71_e1867, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_q, eq71_e1861_d_n3, eq71_e1861_d_n4, eq71_e1861_d_n5, eq71_e1861_d_n6, eq71_e1861_d_n7, eq71_e1861_d_n8, eq71_e1861_d_n9, eq71_e1861_d_n10, eq71_e1861_d_n11, eq71_e1861_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, 0.0];
        let eq71_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1892, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_q, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12,) = {
    if ((((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 == 0.0)) && (locals.var_guard1527 != 0.0)) {
        let eq72_e1879: f64 = (-locals.var_ids_1);
        let eq72_e1881: f64 = (eq72_e1879 * locals.var_vds_1);
        let eq72_e1881_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq72_e1881_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq72_e1881_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq72_e1881_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq72_e1881_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq72_e1879 * locals.var_vds_1_dn7));
        let eq72_e1881_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq72_e1879 * locals.var_vds_1_dn8));
        let eq72_e1881_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq72_e1881_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq72_e1881_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq72_e1881_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq72_e1884: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq72_e1884_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq72_e1884_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq72_e1884_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq72_e1884_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq72_e1884_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq72_e1884_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq72_e1884_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq72_e1884_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq72_e1884_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq72_e1884_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq72_e1885_q: f64 = eq72_e1884;
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1884);
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + eq72_e1884_d_n3);
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + eq72_e1884_d_n4);
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + eq72_e1884_d_n5);
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + eq72_e1884_d_n6);
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + eq72_e1884_d_n7);
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + eq72_e1884_d_n8);
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + eq72_e1884_d_n9);
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + eq72_e1884_d_n10);
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + eq72_e1884_d_n11);
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + eq72_e1884_d_n12);
        let eq72_e1886_q: f64 = eq72_e1885_q;
        let eq72_e1889: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq72_e1889_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_1: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq72_e1889_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1889_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        let eq72_e1890_q: f64 = eq72_e1886_q;
        (eq72_e1890, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_q, eq72_e1884_d_n3, eq72_e1884_d_n4, eq72_e1884_d_n5, eq72_e1884_d_n6, eq72_e1884_d_n7, eq72_e1884_d_n8, eq72_e1884_d_n9, eq72_e1884_d_n10, eq72_e1884_d_n11, eq72_e1884_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, 0.0];
        let eq72_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1920, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_q, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12,) = {
    if (((((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 == 0.0)) && (locals.var_guard1527 == 0.0)) && (locals.var_guard1528 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p30;
        let eq73_e1906: f64 = (locals.var_ids_1 * __rspice_inv_cse_2);
        let eq73_e1906_d_n3: f64 = (locals.var_ids_1_dn3 * __rspice_inv_cse_2);
        let eq73_e1906_d_n4: f64 = (locals.var_ids_1_dn4 * __rspice_inv_cse_2);
        let eq73_e1906_d_n5: f64 = (locals.var_ids_1_dn5 * __rspice_inv_cse_2);
        let eq73_e1906_d_n6: f64 = (locals.var_ids_1_dn6 * __rspice_inv_cse_2);
        let eq73_e1906_d_n7: f64 = (locals.var_ids_1_dn7 * __rspice_inv_cse_2);
        let eq73_e1906_d_n8: f64 = (locals.var_ids_1_dn8 * __rspice_inv_cse_2);
        let eq73_e1906_d_n9: f64 = (locals.var_ids_1_dn9 * __rspice_inv_cse_2);
        let eq73_e1906_d_n10: f64 = (locals.var_ids_1_dn10 * __rspice_inv_cse_2);
        let eq73_e1906_d_n11: f64 = (locals.var_ids_1_dn11 * __rspice_inv_cse_2);
        let eq73_e1906_d_n12: f64 = (locals.var_ids_1_dn12 * __rspice_inv_cse_2);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1909: f64 = (eq73_e1907 * locals.var_vds_1);
        let eq73_e1909_d_n3: f64 = ((-eq73_e1906_d_n3) * locals.var_vds_1);
        let eq73_e1909_d_n4: f64 = ((-eq73_e1906_d_n4) * locals.var_vds_1);
        let eq73_e1909_d_n5: f64 = ((-eq73_e1906_d_n5) * locals.var_vds_1);
        let eq73_e1909_d_n6: f64 = ((-eq73_e1906_d_n6) * locals.var_vds_1);
        let eq73_e1909_d_n7: f64 = (((-eq73_e1906_d_n7) * locals.var_vds_1) + (eq73_e1907 * locals.var_vds_1_dn7));
        let eq73_e1909_d_n8: f64 = (((-eq73_e1906_d_n8) * locals.var_vds_1) + (eq73_e1907 * locals.var_vds_1_dn8));
        let eq73_e1909_d_n9: f64 = ((-eq73_e1906_d_n9) * locals.var_vds_1);
        let eq73_e1909_d_n10: f64 = ((-eq73_e1906_d_n10) * locals.var_vds_1);
        let eq73_e1909_d_n11: f64 = ((-eq73_e1906_d_n11) * locals.var_vds_1);
        let eq73_e1909_d_n12: f64 = ((-eq73_e1906_d_n12) * locals.var_vds_1);
        let eq73_e1912: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq73_e1912_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq73_e1912_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq73_e1912_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq73_e1912_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq73_e1912_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq73_e1912_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq73_e1912_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq73_e1912_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq73_e1912_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq73_e1912_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq73_e1913_q: f64 = eq73_e1912;
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1912);
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + eq73_e1912_d_n3);
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + eq73_e1912_d_n4);
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + eq73_e1912_d_n5);
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + eq73_e1912_d_n6);
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + eq73_e1912_d_n7);
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + eq73_e1912_d_n8);
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + eq73_e1912_d_n9);
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + eq73_e1912_d_n10);
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + eq73_e1912_d_n11);
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + eq73_e1912_d_n12);
        let eq73_e1914_q: f64 = eq73_e1913_q;
        let eq73_e1917: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq73_e1917_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_3: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq73_e1917_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1917_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        let eq73_e1918_q: f64 = eq73_e1914_q;
        (eq73_e1918, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_q, eq73_e1912_d_n3, eq73_e1912_d_n4, eq73_e1912_d_n5, eq73_e1912_d_n6, eq73_e1912_d_n7, eq73_e1912_d_n8, eq73_e1912_d_n9, eq73_e1912_d_n10, eq73_e1912_d_n11, eq73_e1912_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, 0.0];
        let eq73_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1947, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_q, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12,) = {
    if (((((locals.var_guard1524 != 0.0) && (locals.var_guard1525 != 0.0)) && (locals.var_guard1526 == 0.0)) && (locals.var_guard1527 == 0.0)) && (locals.var_guard1528 == 0.0)) {
        let eq74_e1934: f64 = (-locals.var_ids_1);
        let eq74_e1936: f64 = (eq74_e1934 * locals.var_vds_1);
        let eq74_e1936_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq74_e1936_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq74_e1936_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq74_e1936_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq74_e1936_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq74_e1934 * locals.var_vds_1_dn7));
        let eq74_e1936_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq74_e1934 * locals.var_vds_1_dn8));
        let eq74_e1936_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq74_e1936_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq74_e1936_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq74_e1936_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq74_e1939: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq74_e1939_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq74_e1939_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq74_e1939_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq74_e1939_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq74_e1939_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq74_e1939_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq74_e1939_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq74_e1939_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq74_e1939_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq74_e1939_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq74_e1940_q: f64 = eq74_e1939;
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1939);
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + eq74_e1939_d_n3);
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + eq74_e1939_d_n4);
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + eq74_e1939_d_n5);
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + eq74_e1939_d_n6);
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + eq74_e1939_d_n7);
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + eq74_e1939_d_n8);
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + eq74_e1939_d_n9);
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + eq74_e1939_d_n10);
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + eq74_e1939_d_n11);
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + eq74_e1939_d_n12);
        let eq74_e1941_q: f64 = eq74_e1940_q;
        let eq74_e1944: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq74_e1944_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_4: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq74_e1944_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1944_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        let eq74_e1945_q: f64 = eq74_e1941_q;
        (eq74_e1945, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_q, eq74_e1939_d_n3, eq74_e1939_d_n4, eq74_e1939_d_n5, eq74_e1939_d_n6, eq74_e1939_d_n7, eq74_e1939_d_n8, eq74_e1939_d_n9, eq74_e1939_d_n10, eq74_e1939_d_n11, eq74_e1939_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, 0.0];
        let eq74_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1970, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_q, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12,) = {
    if (((locals.var_guard1524 != 0.0) && (locals.var_guard1525 == 0.0)) && (locals.var_guard1529 != 0.0)) {
        let __rspice_inv_cse_5: f64 = 1.0 / p.p30;
        let eq75_e1956: f64 = (locals.var_ids_1 * __rspice_inv_cse_5);
        let eq75_e1956_d_n3: f64 = (locals.var_ids_1_dn3 * __rspice_inv_cse_5);
        let eq75_e1956_d_n4: f64 = (locals.var_ids_1_dn4 * __rspice_inv_cse_5);
        let eq75_e1956_d_n5: f64 = (locals.var_ids_1_dn5 * __rspice_inv_cse_5);
        let eq75_e1956_d_n6: f64 = (locals.var_ids_1_dn6 * __rspice_inv_cse_5);
        let eq75_e1956_d_n7: f64 = (locals.var_ids_1_dn7 * __rspice_inv_cse_5);
        let eq75_e1956_d_n8: f64 = (locals.var_ids_1_dn8 * __rspice_inv_cse_5);
        let eq75_e1956_d_n9: f64 = (locals.var_ids_1_dn9 * __rspice_inv_cse_5);
        let eq75_e1956_d_n10: f64 = (locals.var_ids_1_dn10 * __rspice_inv_cse_5);
        let eq75_e1956_d_n11: f64 = (locals.var_ids_1_dn11 * __rspice_inv_cse_5);
        let eq75_e1956_d_n12: f64 = (locals.var_ids_1_dn12 * __rspice_inv_cse_5);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1959: f64 = (eq75_e1957 * locals.var_vds_1);
        let eq75_e1959_d_n3: f64 = ((-eq75_e1956_d_n3) * locals.var_vds_1);
        let eq75_e1959_d_n4: f64 = ((-eq75_e1956_d_n4) * locals.var_vds_1);
        let eq75_e1959_d_n5: f64 = ((-eq75_e1956_d_n5) * locals.var_vds_1);
        let eq75_e1959_d_n6: f64 = ((-eq75_e1956_d_n6) * locals.var_vds_1);
        let eq75_e1959_d_n7: f64 = (((-eq75_e1956_d_n7) * locals.var_vds_1) + (eq75_e1957 * locals.var_vds_1_dn7));
        let eq75_e1959_d_n8: f64 = (((-eq75_e1956_d_n8) * locals.var_vds_1) + (eq75_e1957 * locals.var_vds_1_dn8));
        let eq75_e1959_d_n9: f64 = ((-eq75_e1956_d_n9) * locals.var_vds_1);
        let eq75_e1959_d_n10: f64 = ((-eq75_e1956_d_n10) * locals.var_vds_1);
        let eq75_e1959_d_n11: f64 = ((-eq75_e1956_d_n11) * locals.var_vds_1);
        let eq75_e1959_d_n12: f64 = ((-eq75_e1956_d_n12) * locals.var_vds_1);
        let eq75_e1962: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq75_e1962_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq75_e1962_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq75_e1962_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq75_e1962_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq75_e1962_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq75_e1962_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq75_e1962_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq75_e1962_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq75_e1962_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq75_e1962_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq75_e1963_q: f64 = eq75_e1962;
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1962);
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + eq75_e1962_d_n3);
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + eq75_e1962_d_n4);
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + eq75_e1962_d_n5);
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + eq75_e1962_d_n6);
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + eq75_e1962_d_n7);
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + eq75_e1962_d_n8);
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + eq75_e1962_d_n9);
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + eq75_e1962_d_n10);
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + eq75_e1962_d_n11);
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + eq75_e1962_d_n12);
        let eq75_e1964_q: f64 = eq75_e1963_q;
        let eq75_e1967: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq75_e1967_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_6: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq75_e1967_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_6);
        let eq75_e1967_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_6);
        let eq75_e1967_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_6);
        let eq75_e1967_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1967_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        let eq75_e1968_q: f64 = eq75_e1964_q;
        (eq75_e1968, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_q, eq75_e1962_d_n3, eq75_e1962_d_n4, eq75_e1962_d_n5, eq75_e1962_d_n6, eq75_e1962_d_n7, eq75_e1962_d_n8, eq75_e1962_d_n9, eq75_e1962_d_n10, eq75_e1962_d_n11, eq75_e1962_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, 0.0];
        let eq75_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq75_reactive_node_derivatives,
            branches,
            &eq75_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq76_e1992, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_q, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12,) = {
    if (((locals.var_guard1524 != 0.0) && (locals.var_guard1525 == 0.0)) && (locals.var_guard1529 == 0.0)) {
        let eq76_e1979: f64 = (-locals.var_ids_1);
        let eq76_e1981: f64 = (eq76_e1979 * locals.var_vds_1);
        let eq76_e1981_d_n3: f64 = ((-locals.var_ids_1_dn3) * locals.var_vds_1);
        let eq76_e1981_d_n4: f64 = ((-locals.var_ids_1_dn4) * locals.var_vds_1);
        let eq76_e1981_d_n5: f64 = ((-locals.var_ids_1_dn5) * locals.var_vds_1);
        let eq76_e1981_d_n6: f64 = ((-locals.var_ids_1_dn6) * locals.var_vds_1);
        let eq76_e1981_d_n7: f64 = (((-locals.var_ids_1_dn7) * locals.var_vds_1) + (eq76_e1979 * locals.var_vds_1_dn7));
        let eq76_e1981_d_n8: f64 = (((-locals.var_ids_1_dn8) * locals.var_vds_1) + (eq76_e1979 * locals.var_vds_1_dn8));
        let eq76_e1981_d_n9: f64 = ((-locals.var_ids_1_dn9) * locals.var_vds_1);
        let eq76_e1981_d_n10: f64 = ((-locals.var_ids_1_dn10) * locals.var_vds_1);
        let eq76_e1981_d_n11: f64 = ((-locals.var_ids_1_dn11) * locals.var_vds_1);
        let eq76_e1981_d_n12: f64 = ((-locals.var_ids_1_dn12) * locals.var_vds_1);
        let eq76_e1984: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth);
        let eq76_e1984_d_n3: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn3);
        let eq76_e1984_d_n4: f64 = ((locals.var_deltemp_dn4 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn4));
        let eq76_e1984_d_n5: f64 = ((locals.var_deltemp_dn5 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn5));
        let eq76_e1984_d_n6: f64 = ((locals.var_deltemp_dn6 * locals.var_pparam_b4soicth) + (locals.var_deltemp * locals.var_pparam_b4soicth_dn6));
        let eq76_e1984_d_n7: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn7);
        let eq76_e1984_d_n8: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn8);
        let eq76_e1984_d_n9: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn9);
        let eq76_e1984_d_n10: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn10);
        let eq76_e1984_d_n11: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn11);
        let eq76_e1984_d_n12: f64 = (locals.var_deltemp * locals.var_pparam_b4soicth_dn12);
        let eq76_e1985_q: f64 = eq76_e1984;
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1984);
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + eq76_e1984_d_n3);
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + eq76_e1984_d_n4);
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + eq76_e1984_d_n5);
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + eq76_e1984_d_n6);
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + eq76_e1984_d_n7);
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + eq76_e1984_d_n8);
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + eq76_e1984_d_n9);
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + eq76_e1984_d_n10);
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + eq76_e1984_d_n11);
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + eq76_e1984_d_n12);
        let eq76_e1986_q: f64 = eq76_e1985_q;
        let eq76_e1989: f64 = (locals.var_deltemp / locals.var_pparam_b4soirth);
        let eq76_e1989_d_n3: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn3) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let __rspice_inv_cse_0: f64 = 1.0 / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth);
        let eq76_e1989_d_n4: f64 = (((locals.var_deltemp_dn4 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn4)) * __rspice_inv_cse_0);
        let eq76_e1989_d_n5: f64 = (((locals.var_deltemp_dn5 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn5)) * __rspice_inv_cse_0);
        let eq76_e1989_d_n6: f64 = (((locals.var_deltemp_dn6 * locals.var_pparam_b4soirth) - (locals.var_deltemp * locals.var_pparam_b4soirth_dn6)) * __rspice_inv_cse_0);
        let eq76_e1989_d_n7: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn7) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n8: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn8) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n9: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn9) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n10: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn10) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n11: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn11) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1989_d_n12: f64 = (-((locals.var_deltemp * locals.var_pparam_b4soirth_dn12) / (locals.var_pparam_b4soirth * locals.var_pparam_b4soirth)));
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        let eq76_e1990_q: f64 = eq76_e1986_q;
        (eq76_e1990, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_q, eq76_e1984_d_n3, eq76_e1984_d_n4, eq76_e1984_d_n5, eq76_e1984_d_n6, eq76_e1984_d_n7, eq76_e1984_d_n8, eq76_e1984_d_n9, eq76_e1984_d_n10, eq76_e1984_d_n11, eq76_e1984_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, 0.0];
        let eq76_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
