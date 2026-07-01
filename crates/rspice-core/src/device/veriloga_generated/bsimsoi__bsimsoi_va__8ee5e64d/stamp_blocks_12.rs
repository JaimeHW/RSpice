#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_b4soiqde: f64,
        var_b4soiqde_dn10: f64,
        var_b4soiqde_dn11: f64,
        var_b4soiqde_dn12: f64,
        var_b4soiqde_dn3: f64,
        var_b4soiqde_dn4: f64,
        var_b4soiqde_dn5: f64,
        var_b4soiqde_dn6: f64,
        var_b4soiqde_dn7: f64,
        var_b4soiqde_dn8: f64,
        var_b4soiqde_dn9: f64,
        var_b4soiqdrn: f64,
        var_b4soiqdrn_dn10: f64,
        var_b4soiqdrn_dn11: f64,
        var_b4soiqdrn_dn12: f64,
        var_b4soiqdrn_dn3: f64,
        var_b4soiqdrn_dn4: f64,
        var_b4soiqdrn_dn5: f64,
        var_b4soiqdrn_dn6: f64,
        var_b4soiqdrn_dn7: f64,
        var_b4soiqdrn_dn8: f64,
        var_b4soiqdrn_dn9: f64,
        var_b4soiqse: f64,
        var_b4soiqse_dn10: f64,
        var_b4soiqse_dn11: f64,
        var_b4soiqse_dn12: f64,
        var_b4soiqse_dn3: f64,
        var_b4soiqse_dn4: f64,
        var_b4soiqse_dn5: f64,
        var_b4soiqse_dn6: f64,
        var_b4soiqse_dn7: f64,
        var_b4soiqse_dn8: f64,
        var_b4soiqse_dn9: f64,
        var_b4soiqsrc: f64,
        var_b4soiqsrc_dn10: f64,
        var_b4soiqsrc_dn11: f64,
        var_b4soiqsrc_dn12: f64,
        var_b4soiqsrc_dn3: f64,
        var_b4soiqsrc_dn4: f64,
        var_b4soiqsrc_dn5: f64,
        var_b4soiqsrc_dn6: f64,
        var_b4soiqsrc_dn7: f64,
        var_b4soiqsrc_dn8: f64,
        var_b4soiqsrc_dn9: f64,
        var_b4soitype: f64,
        var_deltemp: f64,
        var_deltemp_dn6: f64,
        var_guard1828: f64,
        var_guard1834: f64,
        var_pparam_b4soicgeo: f64,
        var_pparam_b4soicgeo_dn10: f64,
        var_pparam_b4soicgeo_dn11: f64,
        var_pparam_b4soicgeo_dn12: f64,
        var_pparam_b4soicgeo_dn3: f64,
        var_pparam_b4soicgeo_dn4: f64,
        var_pparam_b4soicgeo_dn5: f64,
        var_pparam_b4soicgeo_dn6: f64,
        var_pparam_b4soicgeo_dn7: f64,
        var_pparam_b4soicgeo_dn8: f64,
        var_pparam_b4soicgeo_dn9: f64,
        var_pparam_b4soicth: f64,
        var_pparam_b4soicth_dn10: f64,
        var_pparam_b4soicth_dn11: f64,
        var_pparam_b4soicth_dn12: f64,
        var_pparam_b4soicth_dn3: f64,
        var_pparam_b4soicth_dn4: f64,
        var_pparam_b4soicth_dn5: f64,
        var_pparam_b4soicth_dn6: f64,
        var_pparam_b4soicth_dn7: f64,
        var_pparam_b4soicth_dn8: f64,
        var_pparam_b4soicth_dn9: f64,
        var_qgate: f64,
        var_qgate_dn10: f64,
        var_qgate_dn11: f64,
        var_qgate_dn12: f64,
        var_qgate_dn3: f64,
        var_qgate_dn4: f64,
        var_qgate_dn5: f64,
        var_qgate_dn6: f64,
        var_qgate_dn7: f64,
        var_qgate_dn8: f64,
        var_qgate_dn9: f64,
        var_qgdo: f64,
        var_qgdo_dn10: f64,
        var_qgdo_dn11: f64,
        var_qgdo_dn12: f64,
        var_qgdo_dn3: f64,
        var_qgdo_dn4: f64,
        var_qgdo_dn5: f64,
        var_qgdo_dn6: f64,
        var_qgdo_dn7: f64,
        var_qgdo_dn8: f64,
        var_qgdo_dn9: f64,
        var_qgso: f64,
        var_qgso_dn10: f64,
        var_qgso_dn11: f64,
        var_qgso_dn12: f64,
        var_qgso_dn3: f64,
        var_qgso_dn4: f64,
        var_qgso_dn5: f64,
        var_qgso_dn6: f64,
        var_qgso_dn7: f64,
        var_qgso_dn8: f64,
        var_qgso_dn9: f64,
        var_qjd_1: f64,
        var_qjd_1_dn10: f64,
        var_qjd_1_dn11: f64,
        var_qjd_1_dn12: f64,
        var_qjd_1_dn3: f64,
        var_qjd_1_dn4: f64,
        var_qjd_1_dn5: f64,
        var_qjd_1_dn6: f64,
        var_qjd_1_dn7: f64,
        var_qjd_1_dn8: f64,
        var_qjd_1_dn9: f64,
        var_qjs_1: f64,
        var_qjs_1_dn10: f64,
        var_qjs_1_dn11: f64,
        var_qjs_1_dn12: f64,
        var_qjs_1_dn3: f64,
        var_qjs_1_dn4: f64,
        var_qjs_1_dn5: f64,
        var_qjs_1_dn6: f64,
        var_qjs_1_dn7: f64,
        var_qjs_1_dn8: f64,
        var_qjs_1_dn9: f64,
        var_qsub: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn12: f64,
        var_qsub_dn3: f64,
        var_qsub_dn4: f64,
        var_qsub_dn5: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_qsub_dn8: f64,
        var_qsub_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq30_e1299_q: f64 = var_b4soiqdrn;
        let eq30_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, var_b4soiqdrn_dn3, var_b4soiqdrn_dn4, var_b4soiqdrn_dn5, var_b4soiqdrn_dn6, var_b4soiqdrn_dn7, var_b4soiqdrn_dn8, var_b4soiqdrn_dn9, var_b4soiqdrn_dn10, var_b4soiqdrn_dn11, var_b4soiqdrn_dn12];
        let eq30_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1301_q: f64 = var_b4soiqsrc;
        let eq31_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, var_b4soiqsrc_dn3, var_b4soiqsrc_dn4, var_b4soiqsrc_dn5, var_b4soiqsrc_dn6, var_b4soiqsrc_dn7, var_b4soiqsrc_dn8, var_b4soiqsrc_dn9, var_b4soiqsrc_dn10, var_b4soiqsrc_dn11, var_b4soiqsrc_dn12];
        let eq31_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1304_q: f64 = var_qgate;
        let eq32_e1305: f64 = (var_b4soitype * var_qgate);
        let eq32_e1305_d_n3: f64 = (var_b4soitype * var_qgate_dn3);
        let eq32_e1305_d_n4: f64 = (var_b4soitype * var_qgate_dn4);
        let eq32_e1305_d_n5: f64 = (var_b4soitype * var_qgate_dn5);
        let eq32_e1305_d_n6: f64 = (var_b4soitype * var_qgate_dn6);
        let eq32_e1305_d_n7: f64 = (var_b4soitype * var_qgate_dn7);
        let eq32_e1305_d_n8: f64 = (var_b4soitype * var_qgate_dn8);
        let eq32_e1305_d_n9: f64 = (var_b4soitype * var_qgate_dn9);
        let eq32_e1305_d_n10: f64 = (var_b4soitype * var_qgate_dn10);
        let eq32_e1305_d_n11: f64 = (var_b4soitype * var_qgate_dn11);
        let eq32_e1305_d_n12: f64 = (var_b4soitype * var_qgate_dn12);
        let eq32_e1305_q: f64 = (var_b4soitype * eq32_e1304_q);
        let eq32_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq32_e1305_d_n3, eq32_e1305_d_n4, eq32_e1305_d_n5, eq32_e1305_d_n6, eq32_e1305_d_n7, eq32_e1305_d_n8, eq32_e1305_d_n9, eq32_e1305_d_n10, eq32_e1305_d_n11, eq32_e1305_d_n12];
        let eq32_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1308_q: f64 = var_qsub;
        let eq33_e1309: f64 = (var_b4soitype * var_qsub);
        let eq33_e1309_d_n3: f64 = (var_b4soitype * var_qsub_dn3);
        let eq33_e1309_d_n4: f64 = (var_b4soitype * var_qsub_dn4);
        let eq33_e1309_d_n5: f64 = (var_b4soitype * var_qsub_dn5);
        let eq33_e1309_d_n6: f64 = (var_b4soitype * var_qsub_dn6);
        let eq33_e1309_d_n7: f64 = (var_b4soitype * var_qsub_dn7);
        let eq33_e1309_d_n8: f64 = (var_b4soitype * var_qsub_dn8);
        let eq33_e1309_d_n9: f64 = (var_b4soitype * var_qsub_dn9);
        let eq33_e1309_d_n10: f64 = (var_b4soitype * var_qsub_dn10);
        let eq33_e1309_d_n11: f64 = (var_b4soitype * var_qsub_dn11);
        let eq33_e1309_d_n12: f64 = (var_b4soitype * var_qsub_dn12);
        let eq33_e1309_q: f64 = (var_b4soitype * eq33_e1308_q);
        let eq33_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq33_e1309_d_n3, eq33_e1309_d_n4, eq33_e1309_d_n5, eq33_e1309_d_n6, eq33_e1309_d_n7, eq33_e1309_d_n8, eq33_e1309_d_n9, eq33_e1309_d_n10, eq33_e1309_d_n11, eq33_e1309_d_n12];
        let eq33_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1312_q: f64 = var_qjd_1;
        let eq34_e1313: f64 = (var_b4soitype * var_qjd_1);
        let eq34_e1313_d_n3: f64 = (var_b4soitype * var_qjd_1_dn3);
        let eq34_e1313_d_n4: f64 = (var_b4soitype * var_qjd_1_dn4);
        let eq34_e1313_d_n5: f64 = (var_b4soitype * var_qjd_1_dn5);
        let eq34_e1313_d_n6: f64 = (var_b4soitype * var_qjd_1_dn6);
        let eq34_e1313_d_n7: f64 = (var_b4soitype * var_qjd_1_dn7);
        let eq34_e1313_d_n8: f64 = (var_b4soitype * var_qjd_1_dn8);
        let eq34_e1313_d_n9: f64 = (var_b4soitype * var_qjd_1_dn9);
        let eq34_e1313_d_n10: f64 = (var_b4soitype * var_qjd_1_dn10);
        let eq34_e1313_d_n11: f64 = (var_b4soitype * var_qjd_1_dn11);
        let eq34_e1313_d_n12: f64 = (var_b4soitype * var_qjd_1_dn12);
        let eq34_e1313_q: f64 = (var_b4soitype * eq34_e1312_q);
        let eq34_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq34_e1313_d_n3, eq34_e1313_d_n4, eq34_e1313_d_n5, eq34_e1313_d_n6, eq34_e1313_d_n7, eq34_e1313_d_n8, eq34_e1313_d_n9, eq34_e1313_d_n10, eq34_e1313_d_n11, eq34_e1313_d_n12];
        let eq34_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e1316_q: f64 = var_qjs_1;
        let eq35_e1317: f64 = (var_b4soitype * var_qjs_1);
        let eq35_e1317_d_n3: f64 = (var_b4soitype * var_qjs_1_dn3);
        let eq35_e1317_d_n4: f64 = (var_b4soitype * var_qjs_1_dn4);
        let eq35_e1317_d_n5: f64 = (var_b4soitype * var_qjs_1_dn5);
        let eq35_e1317_d_n6: f64 = (var_b4soitype * var_qjs_1_dn6);
        let eq35_e1317_d_n7: f64 = (var_b4soitype * var_qjs_1_dn7);
        let eq35_e1317_d_n8: f64 = (var_b4soitype * var_qjs_1_dn8);
        let eq35_e1317_d_n9: f64 = (var_b4soitype * var_qjs_1_dn9);
        let eq35_e1317_d_n10: f64 = (var_b4soitype * var_qjs_1_dn10);
        let eq35_e1317_d_n11: f64 = (var_b4soitype * var_qjs_1_dn11);
        let eq35_e1317_d_n12: f64 = (var_b4soitype * var_qjs_1_dn12);
        let eq35_e1317_q: f64 = (var_b4soitype * eq35_e1316_q);
        let eq35_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq35_e1317_d_n3, eq35_e1317_d_n4, eq35_e1317_d_n5, eq35_e1317_d_n6, eq35_e1317_d_n7, eq35_e1317_d_n8, eq35_e1317_d_n9, eq35_e1317_d_n10, eq35_e1317_d_n11, eq35_e1317_d_n12];
        let eq35_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1324, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_q,) = {
    if (var_guard1828 != 0.0) {
        let eq36_e1321_q: f64 = var_qgdo;
        let eq36_e1322: f64 = (var_b4soitype * var_qgdo);
        let eq36_e1322_d_n3: f64 = (var_b4soitype * var_qgdo_dn3);
        let eq36_e1322_d_n4: f64 = (var_b4soitype * var_qgdo_dn4);
        let eq36_e1322_d_n5: f64 = (var_b4soitype * var_qgdo_dn5);
        let eq36_e1322_d_n6: f64 = (var_b4soitype * var_qgdo_dn6);
        let eq36_e1322_d_n7: f64 = (var_b4soitype * var_qgdo_dn7);
        let eq36_e1322_d_n8: f64 = (var_b4soitype * var_qgdo_dn8);
        let eq36_e1322_d_n9: f64 = (var_b4soitype * var_qgdo_dn9);
        let eq36_e1322_d_n10: f64 = (var_b4soitype * var_qgdo_dn10);
        let eq36_e1322_d_n11: f64 = (var_b4soitype * var_qgdo_dn11);
        let eq36_e1322_d_n12: f64 = (var_b4soitype * var_qgdo_dn12);
        let eq36_e1322_q: f64 = (var_b4soitype * eq36_e1321_q);
        (eq36_e1322, eq36_e1322_d_n3, eq36_e1322_d_n4, eq36_e1322_d_n5, eq36_e1322_d_n6, eq36_e1322_d_n7, eq36_e1322_d_n8, eq36_e1322_d_n9, eq36_e1322_d_n10, eq36_e1322_d_n11, eq36_e1322_d_n12, eq36_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];
        let eq36_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_q,) = {
    if (var_guard1828 != 0.0) {
        let eq37_e1328_q: f64 = var_qgso;
        let eq37_e1329: f64 = (var_b4soitype * var_qgso);
        let eq37_e1329_d_n3: f64 = (var_b4soitype * var_qgso_dn3);
        let eq37_e1329_d_n4: f64 = (var_b4soitype * var_qgso_dn4);
        let eq37_e1329_d_n5: f64 = (var_b4soitype * var_qgso_dn5);
        let eq37_e1329_d_n6: f64 = (var_b4soitype * var_qgso_dn6);
        let eq37_e1329_d_n7: f64 = (var_b4soitype * var_qgso_dn7);
        let eq37_e1329_d_n8: f64 = (var_b4soitype * var_qgso_dn8);
        let eq37_e1329_d_n9: f64 = (var_b4soitype * var_qgso_dn9);
        let eq37_e1329_d_n10: f64 = (var_b4soitype * var_qgso_dn10);
        let eq37_e1329_d_n11: f64 = (var_b4soitype * var_qgso_dn11);
        let eq37_e1329_d_n12: f64 = (var_b4soitype * var_qgso_dn12);
        let eq37_e1329_q: f64 = (var_b4soitype * eq37_e1328_q);
        (eq37_e1329, eq37_e1329_d_n3, eq37_e1329_d_n4, eq37_e1329_d_n5, eq37_e1329_d_n6, eq37_e1329_d_n7, eq37_e1329_d_n8, eq37_e1329_d_n9, eq37_e1329_d_n10, eq37_e1329_d_n11, eq37_e1329_d_n12, eq37_e1329_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];
        let eq37_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_q,) = {
    if (var_guard1828 != 0.0) {
        let eq38_e1335: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo);
        let eq38_e1335_d_n3: f64 = ((-var_pparam_b4soicgeo) + ((nv10 - nv3) * var_pparam_b4soicgeo_dn3));
        let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn4);
        let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn5);
        let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn6);
        let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn7);
        let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn8);
        let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn9);
        let eq38_e1335_d_n10: f64 = (var_pparam_b4soicgeo + ((nv10 - nv3) * var_pparam_b4soicgeo_dn10));
        let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn11);
        let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * var_pparam_b4soicgeo_dn12);
        let eq38_e1336_q: f64 = eq38_e1335;
        (eq38_e1335, eq38_e1335_d_n3, eq38_e1335_d_n4, eq38_e1335_d_n5, eq38_e1335_d_n6, eq38_e1335_d_n7, eq38_e1335_d_n8, eq38_e1335_d_n9, eq38_e1335_d_n10, eq38_e1335_d_n11, eq38_e1335_d_n12, eq38_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];
        let eq38_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_q,) = {
    if (var_guard1828 == 0.0) {
        let eq39_e1343_q: f64 = var_qgdo;
        let eq39_e1344: f64 = (var_b4soitype * var_qgdo);
        let eq39_e1344_d_n3: f64 = (var_b4soitype * var_qgdo_dn3);
        let eq39_e1344_d_n4: f64 = (var_b4soitype * var_qgdo_dn4);
        let eq39_e1344_d_n5: f64 = (var_b4soitype * var_qgdo_dn5);
        let eq39_e1344_d_n6: f64 = (var_b4soitype * var_qgdo_dn6);
        let eq39_e1344_d_n7: f64 = (var_b4soitype * var_qgdo_dn7);
        let eq39_e1344_d_n8: f64 = (var_b4soitype * var_qgdo_dn8);
        let eq39_e1344_d_n9: f64 = (var_b4soitype * var_qgdo_dn9);
        let eq39_e1344_d_n10: f64 = (var_b4soitype * var_qgdo_dn10);
        let eq39_e1344_d_n11: f64 = (var_b4soitype * var_qgdo_dn11);
        let eq39_e1344_d_n12: f64 = (var_b4soitype * var_qgdo_dn12);
        let eq39_e1344_q: f64 = (var_b4soitype * eq39_e1343_q);
        (eq39_e1344, eq39_e1344_d_n3, eq39_e1344_d_n4, eq39_e1344_d_n5, eq39_e1344_d_n6, eq39_e1344_d_n7, eq39_e1344_d_n8, eq39_e1344_d_n9, eq39_e1344_d_n10, eq39_e1344_d_n11, eq39_e1344_d_n12, eq39_e1344_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_q,) = {
    if (var_guard1828 == 0.0) {
        let eq40_e1351_q: f64 = var_qgso;
        let eq40_e1352: f64 = (var_b4soitype * var_qgso);
        let eq40_e1352_d_n3: f64 = (var_b4soitype * var_qgso_dn3);
        let eq40_e1352_d_n4: f64 = (var_b4soitype * var_qgso_dn4);
        let eq40_e1352_d_n5: f64 = (var_b4soitype * var_qgso_dn5);
        let eq40_e1352_d_n6: f64 = (var_b4soitype * var_qgso_dn6);
        let eq40_e1352_d_n7: f64 = (var_b4soitype * var_qgso_dn7);
        let eq40_e1352_d_n8: f64 = (var_b4soitype * var_qgso_dn8);
        let eq40_e1352_d_n9: f64 = (var_b4soitype * var_qgso_dn9);
        let eq40_e1352_d_n10: f64 = (var_b4soitype * var_qgso_dn10);
        let eq40_e1352_d_n11: f64 = (var_b4soitype * var_qgso_dn11);
        let eq40_e1352_d_n12: f64 = (var_b4soitype * var_qgso_dn12);
        let eq40_e1352_q: f64 = (var_b4soitype * eq40_e1351_q);
        (eq40_e1352, eq40_e1352_d_n3, eq40_e1352_d_n4, eq40_e1352_d_n5, eq40_e1352_d_n6, eq40_e1352_d_n7, eq40_e1352_d_n8, eq40_e1352_d_n9, eq40_e1352_d_n10, eq40_e1352_d_n11, eq40_e1352_d_n12, eq40_e1352_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];
        let eq40_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1362, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_q,) = {
    if (var_guard1828 == 0.0) {
        let eq41_e1359: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo);
        let eq41_e1359_d_n3: f64 = ((-var_pparam_b4soicgeo) + ((nv9 - nv3) * var_pparam_b4soicgeo_dn3));
        let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn4);
        let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn5);
        let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn6);
        let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn7);
        let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn8);
        let eq41_e1359_d_n9: f64 = (var_pparam_b4soicgeo + ((nv9 - nv3) * var_pparam_b4soicgeo_dn9));
        let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn10);
        let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn11);
        let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * var_pparam_b4soicgeo_dn12);
        let eq41_e1360_q: f64 = eq41_e1359;
        (eq41_e1359, eq41_e1359_d_n3, eq41_e1359_d_n4, eq41_e1359_d_n5, eq41_e1359_d_n6, eq41_e1359_d_n7, eq41_e1359_d_n8, eq41_e1359_d_n9, eq41_e1359_d_n10, eq41_e1359_d_n11, eq41_e1359_d_n12, eq41_e1360_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1364_q: f64 = var_b4soiqde;
        let eq42_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, var_b4soiqde_dn3, var_b4soiqde_dn4, var_b4soiqde_dn5, var_b4soiqde_dn6, var_b4soiqde_dn7, var_b4soiqde_dn8, var_b4soiqde_dn9, var_b4soiqde_dn10, var_b4soiqde_dn11, var_b4soiqde_dn12];
        let eq42_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1366_q: f64 = var_b4soiqse;
        let eq43_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, var_b4soiqse_dn3, var_b4soiqse_dn4, var_b4soiqse_dn5, var_b4soiqse_dn6, var_b4soiqse_dn7, var_b4soiqse_dn8, var_b4soiqse_dn9, var_b4soiqse_dn10, var_b4soiqse_dn11, var_b4soiqse_dn12];
        let eq43_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_q,) = {
    if (var_guard1834 != 0.0) {
        let eq58_e1473: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq58_e1473_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq58_e1473_d_n4: f64 = (var_deltemp * var_pparam_b4soicth_dn4);
        let eq58_e1473_d_n5: f64 = (var_deltemp * var_pparam_b4soicth_dn5);
        let eq58_e1473_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq58_e1473_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq58_e1473_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq58_e1473_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq58_e1473_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq58_e1473_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq58_e1473_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq58_e1474_q: f64 = eq58_e1473;
        (eq58_e1473, eq58_e1473_d_n3, eq58_e1473_d_n4, eq58_e1473_d_n5, eq58_e1473_d_n6, eq58_e1473_d_n7, eq58_e1473_d_n8, eq58_e1473_d_n9, eq58_e1473_d_n10, eq58_e1473_d_n11, eq58_e1473_d_n12, eq58_e1474_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];
        let eq58_reactive_branch_derivatives: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
