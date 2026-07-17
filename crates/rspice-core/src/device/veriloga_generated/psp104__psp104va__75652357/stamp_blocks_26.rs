#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
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
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq24_e1160, eq24_e1160_d_n8, eq24_e1160_d_n9,) = {
    if (l.f8ae != 0.0) {
        let eq24_e1154: f64 = (l.febf * p.p32);let eq24_e1156: f64 = (eq24_e1154 * l.f506);let eq24_e1158: f64 = (eq24_e1156 * (nv8 - nv9));
        (eq24_e1158, eq24_e1156, (-eq24_e1156),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1160;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            8,
            multiplicity * (eq24_e1160_d_n8),
            9,
            multiplicity * (eq24_e1160_d_n9),
        );
        let (eq26_e1175,) = {
    if (l.f8ae == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1175;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1185, eq27_e1185_d_n9, eq27_e1185_d_n10,) = {
    if (l.f8af != 0.0) {
        let eq27_e1179: f64 = (l.febf * p.p32);let eq27_e1181: f64 = (eq27_e1179 * l.f56f);let eq27_e1183: f64 = (eq27_e1181 * (nv10 - nv9));
        (eq27_e1183, (-eq27_e1181), eq27_e1181,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1185;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            9,
            multiplicity * (eq27_e1185_d_n9),
            10,
            multiplicity * (eq27_e1185_d_n10),
        );
        let (eq29_e1200,) = {
    if (l.f8af == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1200;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1210, eq30_e1210_d_n9, eq30_e1210_d_n11,) = {
    if (l.f8b0 != 0.0) {
        let eq30_e1204: f64 = (l.febf * p.p32);let eq30_e1206: f64 = (eq30_e1204 * l.f56e);let eq30_e1208: f64 = (eq30_e1206 * (nv11 - nv9));
        (eq30_e1208, (-eq30_e1206), eq30_e1206,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1210;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            9,
            multiplicity * (eq30_e1210_d_n9),
            11,
            multiplicity * (eq30_e1210_d_n11),
        );
        let (eq32_e1225,) = {
    if (l.f8b0 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1225;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1235, eq33_e1235_d_n3, eq33_e1235_d_n9,) = {
    if (l.f8b1 != 0.0) {
        let eq33_e1229: f64 = (l.febf * p.p32);let eq33_e1231: f64 = (eq33_e1229 * l.fbeb);let eq33_e1233: f64 = (eq33_e1231 * (nv3 - nv9));
        (eq33_e1233, eq33_e1231, (-eq33_e1231),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1235;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1235_d_n3),
            9,
            multiplicity * (eq33_e1235_d_n9),
        );
        let (eq35_e1250,) = {
    if (l.f8b1 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1250;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );let eq36_e1253: f64 = (p.p32 * l.f570);let eq36_e1255: f64 = (eq36_e1253 * (nv7 - nv8));let eq36_value: f64 = eq36_e1255;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(8),
            multiplicity * (eq36_value),
            7,
            multiplicity * (eq36_e1253),
            8,
            multiplicity * ((-eq36_e1253)),
        );let eq37_e1258: f64 = (p.p32 * l.f570);let eq37_e1260: f64 = (eq37_e1258 * (nv6 - nv8));let eq37_value: f64 = eq37_e1260;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(8),
            multiplicity * (eq37_value),
            6,
            multiplicity * (eq37_e1258),
            8,
            multiplicity * ((-eq37_e1258)),
        );let eq38_e1263: f64 = (l.f193 * l.febf);let eq38_e1265: f64 = (eq38_e1263 * p.p33);let eq38_e1267: f64 = (eq38_e1265 * l.f115e);let eq38_e1267_d_n5: f64 = (eq38_e1265 * l.f1165);let eq38_e1267_d_n6: f64 = (eq38_e1265 * l.f1166);let eq38_e1267_d_n7: f64 = (eq38_e1265 * l.f1167);let eq38_e1267_d_n8: f64 = (eq38_e1265 * l.f1168);let eq38_e1268: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq38_e1267);let eq38_value: f64 = eq38_e1268;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq38_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq38_e1267_d_n5 * ddt_scale)), multiplicity * ((eq38_e1267_d_n6 * ddt_scale)), multiplicity * ((eq38_e1267_d_n7 * ddt_scale)), multiplicity * ((eq38_e1267_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq39_e1271: f64 = (l.f193 * l.febf);let eq39_e1273: f64 = (eq39_e1271 * p.p33);let eq39_e1275: f64 = (eq39_e1273 * l.f1098);let eq39_e1275_d_n5: f64 = (eq39_e1273 * l.f10a1);let eq39_e1275_d_n6: f64 = (eq39_e1273 * l.f10a2);let eq39_e1275_d_n7: f64 = (eq39_e1273 * l.f10a3);let eq39_e1275_d_n8: f64 = (eq39_e1273 * l.f10a4);let eq39_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq39_e1275);let eq39_value: f64 = eq39_e1276;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq39_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq39_e1275_d_n5 * ddt_scale)), multiplicity * ((eq39_e1275_d_n6 * ddt_scale)), multiplicity * ((eq39_e1275_d_n7 * ddt_scale)), multiplicity * ((eq39_e1275_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq40_e1279: f64 = (l.f193 * l.febf);let eq40_e1281: f64 = (eq40_e1279 * p.p33);let eq40_e1283: f64 = (eq40_e1281 * l.f1112);let eq40_e1283_d_n5: f64 = (eq40_e1281 * l.f1119);let eq40_e1283_d_n6: f64 = (eq40_e1281 * l.f111a);let eq40_e1283_d_n7: f64 = (eq40_e1281 * l.f111b);let eq40_e1283_d_n8: f64 = (eq40_e1281 * l.f111c);let eq40_e1284: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq40_e1283);let eq40_value: f64 = eq40_e1284;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq40_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq40_e1283_d_n5 * ddt_scale)), multiplicity * ((eq40_e1283_d_n6 * ddt_scale)), multiplicity * ((eq40_e1283_d_n7 * ddt_scale)), multiplicity * ((eq40_e1283_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq41_e1287: f64 = (l.f193 * l.febf);let eq41_e1289: f64 = (eq41_e1287 * p.p33);let eq41_e1291: f64 = (eq41_e1289 * l.f1159);let eq41_e1291_d_n5: f64 = (eq41_e1289 * l.f115a);let eq41_e1291_d_n6: f64 = (eq41_e1289 * l.f115b);let eq41_e1291_d_n7: f64 = (eq41_e1289 * l.f115c);let eq41_e1292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq41_e1291);let eq41_value: f64 = eq41_e1292;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (eq41_value),
            5,
            multiplicity * ((eq41_e1291_d_n5 * ddt_scale)),
            6,
            multiplicity * ((eq41_e1291_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq41_e1291_d_n7 * ddt_scale)),
        );let eq42_e1295: f64 = (l.f193 * l.febf);let eq42_e1297: f64 = (eq42_e1295 * p.p33);let eq42_e1299: f64 = (eq42_e1297 * l.f1154);let eq42_e1299_d_n5: f64 = (eq42_e1297 * l.f1155);let eq42_e1299_d_n6: f64 = (eq42_e1297 * l.f1156);let eq42_e1299_d_n7: f64 = (eq42_e1297 * l.f1157);let eq42_e1300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq42_e1299);let eq42_value: f64 = eq42_e1300;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(7),
            multiplicity * (eq42_value),
            5,
            multiplicity * ((eq42_e1299_d_n5 * ddt_scale)),
            6,
            multiplicity * ((eq42_e1299_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq42_e1299_d_n7 * ddt_scale)),
        );
    }
    #[inline(never)]
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
        l: &mut StampLocals,
    ) {
        let eq43_e1303: f64 = (l.f193 * l.febf);let eq43_e1305: f64 = (eq43_e1303 * p.p33);let eq43_e1307: f64 = (eq43_e1305 * l.f117c);let eq43_e1307_d_n5: f64 = (eq43_e1305 * l.f117d);let eq43_e1307_d_n6: f64 = (eq43_e1305 * l.f117e);let eq43_e1307_d_n7: f64 = (eq43_e1305 * l.f117f);let eq43_e1307_d_n8: f64 = (eq43_e1305 * l.f1180);let eq43_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq43_e1307);let eq43_value: f64 = eq43_e1308;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(8),
            multiplicity * (eq43_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq43_e1307_d_n5 * ddt_scale)), multiplicity * ((eq43_e1307_d_n6 * ddt_scale)), multiplicity * ((eq43_e1307_d_n7 * ddt_scale)), multiplicity * ((eq43_e1307_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq44_e1311: f64 = (l.f193 * l.febf);let eq44_e1313: f64 = (eq44_e1311 * p.p33);let eq44_e1315: f64 = (eq44_e1313 * l.f1212);let eq44_e1315_d_n5: f64 = (eq44_e1313 * l.f1215);let eq44_e1315_d_n6: f64 = (eq44_e1313 * l.f1216);let eq44_e1315_d_n7: f64 = (eq44_e1313 * l.f1217);let eq44_e1315_d_n8: f64 = (eq44_e1313 * l.f1218);let eq44_e1315_d_n10: f64 = (eq44_e1313 * l.f1213);let eq44_e1315_d_n11: f64 = (eq44_e1313 * l.f1214);let eq44_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq44_e1315);let eq44_value: f64 = eq44_e1316;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq44_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * ((eq44_e1315_d_n5 * ddt_scale)), multiplicity * ((eq44_e1315_d_n6 * ddt_scale)), multiplicity * ((eq44_e1315_d_n7 * ddt_scale)), multiplicity * ((eq44_e1315_d_n8 * ddt_scale)), multiplicity * ((eq44_e1315_d_n10 * ddt_scale)), multiplicity * ((eq44_e1315_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq45_e1319: f64 = (l.f193 * l.febf);let eq45_e1321: f64 = (eq45_e1319 * p.p33);let eq45_e1323: f64 = (eq45_e1321 * l.f120a);let eq45_e1323_d_n5: f64 = (eq45_e1321 * l.f120d);let eq45_e1323_d_n6: f64 = (eq45_e1321 * l.f120e);let eq45_e1323_d_n7: f64 = (eq45_e1321 * l.f120f);let eq45_e1323_d_n8: f64 = (eq45_e1321 * l.f1210);let eq45_e1323_d_n10: f64 = (eq45_e1321 * l.f120b);let eq45_e1323_d_n11: f64 = (eq45_e1321 * l.f120c);let eq45_e1324: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq45_e1323);let eq45_value: f64 = eq45_e1324;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq45_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * ((eq45_e1323_d_n5 * ddt_scale)), multiplicity * ((eq45_e1323_d_n6 * ddt_scale)), multiplicity * ((eq45_e1323_d_n7 * ddt_scale)), multiplicity * ((eq45_e1323_d_n8 * ddt_scale)), multiplicity * ((eq45_e1323_d_n10 * ddt_scale)), multiplicity * ((eq45_e1323_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
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
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let __rspice_inv_cse_0: f64 = 1.0 / l.fe93;let eq47_e1332: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);let eq47_e1332_d_n4: f64 = (1.0 * __rspice_inv_cse_0);let eq47_e1332_d_n5: f64 = (-(((nv4 - 0.0) * l.fe94) / (l.fe93 * l.fe93)));let eq47_e1332_d_n6: f64 = (-(((nv4 - 0.0) * l.fe95) / (l.fe93 * l.fe93)));let eq47_e1332_d_n7: f64 = (-(((nv4 - 0.0) * l.fe96) / (l.fe93 * l.fe93)));let eq47_e1332_d_n8: f64 = (-(((nv4 - 0.0) * l.fe97) / (l.fe93 * l.fe93)));let eq47_value: f64 = eq47_e1332;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq47_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq47_e1332_d_n4), multiplicity * (eq47_e1332_d_n5), multiplicity * (eq47_e1332_d_n6), multiplicity * (eq47_e1332_d_n7), multiplicity * (eq47_e1332_d_n8)],
            [],
            [],
            1.0,
        );let eq48_e1335: f64 = (l.f175 * (nv4 - 0.0));let eq48_e1335_d_n5: f64 = (l.f176 * (nv4 - 0.0));let eq48_e1335_d_n6: f64 = (l.f177 * (nv4 - 0.0));let eq48_e1335_d_n7: f64 = (l.f178 * (nv4 - 0.0));let eq48_e1335_d_n8: f64 = (l.f179 * (nv4 - 0.0));let eq48_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1335);let eq48_value: f64 = eq48_e1336;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq48_value),
            [4, 5, 6, 7, 8],
            [multiplicity * ((l.f175 * ddt_scale)), multiplicity * ((eq48_e1335_d_n5 * ddt_scale)), multiplicity * ((eq48_e1335_d_n6 * ddt_scale)), multiplicity * ((eq48_e1335_d_n7 * ddt_scale)), multiplicity * ((eq48_e1335_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq49_e1339: f64 = (l.febf * p.p32);let eq49_e1340: f64 = (eq49_e1339).sqrt();let eq49_e1342: f64 = (eq49_e1340 * 0.5);let eq49_e1344: f64 = (eq49_e1342 * l.f175);let eq49_e1344_d_n5: f64 = (eq49_e1342 * l.f176);let eq49_e1344_d_n6: f64 = (eq49_e1342 * l.f177);let eq49_e1344_d_n7: f64 = (eq49_e1342 * l.f178);let eq49_e1344_d_n8: f64 = (eq49_e1342 * l.f179);let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));let eq49_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq49_e1346);let eq49_e1348: f64 = (-eq49_e1347);let eq49_e1348_d_n4: f64 = (-(eq49_e1344 * ddt_scale));let eq49_e1348_d_n5: f64 = (-(eq49_e1346_d_n5 * ddt_scale));let eq49_e1348_d_n6: f64 = (-(eq49_e1346_d_n6 * ddt_scale));let eq49_e1348_d_n7: f64 = (-(eq49_e1346_d_n7 * ddt_scale));let eq49_e1348_d_n8: f64 = (-(eq49_e1346_d_n8 * ddt_scale));let eq49_value: f64 = eq49_e1348;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq49_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq49_e1348_d_n4), multiplicity * (eq49_e1348_d_n5), multiplicity * (eq49_e1348_d_n6), multiplicity * (eq49_e1348_d_n7), multiplicity * (eq49_e1348_d_n8)],
            [],
            [],
            1.0,
        );let eq50_e1351: f64 = (l.febf * p.p32);let eq50_e1352: f64 = (eq50_e1351).sqrt();let eq50_e1354: f64 = (eq50_e1352 * 0.5);let eq50_e1356: f64 = (eq50_e1354 * l.f175);let eq50_e1356_d_n5: f64 = (eq50_e1354 * l.f176);let eq50_e1356_d_n6: f64 = (eq50_e1354 * l.f177);let eq50_e1356_d_n7: f64 = (eq50_e1354 * l.f178);let eq50_e1356_d_n8: f64 = (eq50_e1354 * l.f179);let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));let eq50_e1359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq50_e1358);let eq50_e1360: f64 = (-eq50_e1359);let eq50_e1360_d_n4: f64 = (-(eq50_e1356 * ddt_scale));let eq50_e1360_d_n5: f64 = (-(eq50_e1358_d_n5 * ddt_scale));let eq50_e1360_d_n6: f64 = (-(eq50_e1358_d_n6 * ddt_scale));let eq50_e1360_d_n7: f64 = (-(eq50_e1358_d_n7 * ddt_scale));let eq50_e1360_d_n8: f64 = (-(eq50_e1358_d_n8 * ddt_scale));let eq50_value: f64 = eq50_e1360;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq50_e1360_d_n4), multiplicity * (eq50_e1360_d_n5), multiplicity * (eq50_e1360_d_n6), multiplicity * (eq50_e1360_d_n7), multiplicity * (eq50_e1360_d_n8)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let eq38_e1263: f64 = (l.f193 * l.febf);let eq38_e1265: f64 = (eq38_e1263 * p.p33);let eq38_e1267: f64 = (eq38_e1265 * l.f115e);let eq38_e1267_d_n5: f64 = (eq38_e1265 * l.f1165);let eq38_e1267_d_n6: f64 = (eq38_e1265 * l.f1166);let eq38_e1267_d_n7: f64 = (eq38_e1265 * l.f1167);let eq38_e1267_d_n8: f64 = (eq38_e1265 * l.f1168);let eq38_e1268_q: f64 = eq38_e1267;
        stamper.stamp_current_reactive_local(
            Some(5),
            Some(6),
            &[
                GeneratedDerivative::node(5, multiplicity * (eq38_e1267_d_n5)),
                GeneratedDerivative::node(6, multiplicity * (eq38_e1267_d_n6)),
                GeneratedDerivative::node(7, multiplicity * (eq38_e1267_d_n7)),
                GeneratedDerivative::node(8, multiplicity * (eq38_e1267_d_n8)),
            ],
        );let eq39_e1271: f64 = (l.f193 * l.febf);let eq39_e1273: f64 = (eq39_e1271 * p.p33);let eq39_e1275: f64 = (eq39_e1273 * l.f1098);let eq39_e1275_d_n5: f64 = (eq39_e1273 * l.f10a1);let eq39_e1275_d_n6: f64 = (eq39_e1273 * l.f10a2);let eq39_e1275_d_n7: f64 = (eq39_e1273 * l.f10a3);let eq39_e1275_d_n8: f64 = (eq39_e1273 * l.f10a4);let eq39_e1276_q: f64 = eq39_e1275;
        stamper.stamp_current_reactive_local(
            Some(8),
            Some(6),
            &[
                GeneratedDerivative::node(5, multiplicity * (eq39_e1275_d_n5)),
                GeneratedDerivative::node(6, multiplicity * (eq39_e1275_d_n6)),
                GeneratedDerivative::node(7, multiplicity * (eq39_e1275_d_n7)),
                GeneratedDerivative::node(8, multiplicity * (eq39_e1275_d_n8)),
            ],
        );let eq40_e1279: f64 = (l.f193 * l.febf);let eq40_e1281: f64 = (eq40_e1279 * p.p33);let eq40_e1283: f64 = (eq40_e1281 * l.f1112);let eq40_e1283_d_n5: f64 = (eq40_e1281 * l.f1119);let eq40_e1283_d_n6: f64 = (eq40_e1281 * l.f111a);let eq40_e1283_d_n7: f64 = (eq40_e1281 * l.f111b);let eq40_e1283_d_n8: f64 = (eq40_e1281 * l.f111c);let eq40_e1284_q: f64 = eq40_e1283;
        stamper.stamp_current_reactive_local(
            Some(7),
            Some(6),
            &[
                GeneratedDerivative::node(5, multiplicity * (eq40_e1283_d_n5)),
                GeneratedDerivative::node(6, multiplicity * (eq40_e1283_d_n6)),
                GeneratedDerivative::node(7, multiplicity * (eq40_e1283_d_n7)),
                GeneratedDerivative::node(8, multiplicity * (eq40_e1283_d_n8)),
            ],
        );let eq41_e1287: f64 = (l.f193 * l.febf);let eq41_e1289: f64 = (eq41_e1287 * p.p33);let eq41_e1291: f64 = (eq41_e1289 * l.f1159);let eq41_e1291_d_n5: f64 = (eq41_e1289 * l.f115a);let eq41_e1291_d_n6: f64 = (eq41_e1289 * l.f115b);let eq41_e1291_d_n7: f64 = (eq41_e1289 * l.f115c);let eq41_e1292_q: f64 = eq41_e1291;
        stamper.stamp_current_reactive_node3_local(
            Some(5),
            Some(6),
            5,
            multiplicity * (eq41_e1291_d_n5),
            6,
            multiplicity * (eq41_e1291_d_n6),
            7,
            multiplicity * (eq41_e1291_d_n7),
        );let eq42_e1295: f64 = (l.f193 * l.febf);let eq42_e1297: f64 = (eq42_e1295 * p.p33);let eq42_e1299: f64 = (eq42_e1297 * l.f1154);let eq42_e1299_d_n5: f64 = (eq42_e1297 * l.f1155);let eq42_e1299_d_n6: f64 = (eq42_e1297 * l.f1156);let eq42_e1299_d_n7: f64 = (eq42_e1297 * l.f1157);let eq42_e1300_q: f64 = eq42_e1299;
        stamper.stamp_current_reactive_node3_local(
            Some(5),
            Some(7),
            5,
            multiplicity * (eq42_e1299_d_n5),
            6,
            multiplicity * (eq42_e1299_d_n6),
            7,
            multiplicity * (eq42_e1299_d_n7),
        );let eq43_e1303: f64 = (l.f193 * l.febf);let eq43_e1305: f64 = (eq43_e1303 * p.p33);let eq43_e1307: f64 = (eq43_e1305 * l.f117c);let eq43_e1307_d_n5: f64 = (eq43_e1305 * l.f117d);let eq43_e1307_d_n6: f64 = (eq43_e1305 * l.f117e);let eq43_e1307_d_n7: f64 = (eq43_e1305 * l.f117f);let eq43_e1307_d_n8: f64 = (eq43_e1305 * l.f1180);let eq43_e1308_q: f64 = eq43_e1307;
        stamper.stamp_current_reactive_local(
            Some(5),
            Some(8),
            &[
                GeneratedDerivative::node(5, multiplicity * (eq43_e1307_d_n5)),
                GeneratedDerivative::node(6, multiplicity * (eq43_e1307_d_n6)),
                GeneratedDerivative::node(7, multiplicity * (eq43_e1307_d_n7)),
                GeneratedDerivative::node(8, multiplicity * (eq43_e1307_d_n8)),
            ],
        );let eq44_e1311: f64 = (l.f193 * l.febf);let eq44_e1313: f64 = (eq44_e1311 * p.p33);let eq44_e1315: f64 = (eq44_e1313 * l.f1212);let eq44_e1315_d_n5: f64 = (eq44_e1313 * l.f1215);let eq44_e1315_d_n6: f64 = (eq44_e1313 * l.f1216);let eq44_e1315_d_n7: f64 = (eq44_e1313 * l.f1217);let eq44_e1315_d_n8: f64 = (eq44_e1313 * l.f1218);let eq44_e1315_d_n10: f64 = (eq44_e1313 * l.f1213);let eq44_e1315_d_n11: f64 = (eq44_e1313 * l.f1214);let eq44_e1316_q: f64 = eq44_e1315;let eq44_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, eq44_e1315_d_n5, eq44_e1315_d_n6, eq44_e1315_d_n7, eq44_e1315_d_n8, 0.0, eq44_e1315_d_n10, eq44_e1315_d_n11];let eq44_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            Some(6),
            &eq44_reactive_node_derivatives,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );let eq45_e1319: f64 = (l.f193 * l.febf);let eq45_e1321: f64 = (eq45_e1319 * p.p33);let eq45_e1323: f64 = (eq45_e1321 * l.f120a);let eq45_e1323_d_n5: f64 = (eq45_e1321 * l.f120d);let eq45_e1323_d_n6: f64 = (eq45_e1321 * l.f120e);let eq45_e1323_d_n7: f64 = (eq45_e1321 * l.f120f);let eq45_e1323_d_n8: f64 = (eq45_e1321 * l.f1210);let eq45_e1323_d_n10: f64 = (eq45_e1321 * l.f120b);let eq45_e1323_d_n11: f64 = (eq45_e1321 * l.f120c);let eq45_e1324_q: f64 = eq45_e1323;let eq45_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, eq45_e1323_d_n5, eq45_e1323_d_n6, eq45_e1323_d_n7, eq45_e1323_d_n8, 0.0, eq45_e1323_d_n10, eq45_e1323_d_n11];let eq45_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(7),
            &eq45_reactive_node_derivatives,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );let eq48_e1335: f64 = (l.f175 * (nv4 - 0.0));let eq48_e1335_d_n5: f64 = (l.f176 * (nv4 - 0.0));let eq48_e1335_d_n6: f64 = (l.f177 * (nv4 - 0.0));let eq48_e1335_d_n7: f64 = (l.f178 * (nv4 - 0.0));let eq48_e1335_d_n8: f64 = (l.f179 * (nv4 - 0.0));let eq48_e1336_q: f64 = eq48_e1335;let eq48_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, l.f175, eq48_e1335_d_n5, eq48_e1335_d_n6, eq48_e1335_d_n7, eq48_e1335_d_n8, 0.0, 0.0, 0.0];let eq48_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &eq48_reactive_node_derivatives,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let eq49_e1339: f64 = (l.febf * p.p32);let eq49_e1340: f64 = (eq49_e1339).sqrt();let eq49_e1342: f64 = (eq49_e1340 * 0.5);let eq49_e1344: f64 = (eq49_e1342 * l.f175);let eq49_e1344_d_n5: f64 = (eq49_e1342 * l.f176);let eq49_e1344_d_n6: f64 = (eq49_e1342 * l.f177);let eq49_e1344_d_n7: f64 = (eq49_e1342 * l.f178);let eq49_e1344_d_n8: f64 = (eq49_e1342 * l.f179);let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));let eq49_e1347_q: f64 = eq49_e1346;let eq49_e1348: f64 = (-eq49_e1346);let eq49_e1348_q: f64 = (-eq49_e1347_q);let eq49_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, (-eq49_e1344), (-eq49_e1346_d_n5), (-eq49_e1346_d_n6), (-eq49_e1346_d_n7), (-eq49_e1346_d_n8), 0.0, 0.0, 0.0];let eq49_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(6),
            &eq49_reactive_node_derivatives,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );let eq50_e1351: f64 = (l.febf * p.p32);let eq50_e1352: f64 = (eq50_e1351).sqrt();let eq50_e1354: f64 = (eq50_e1352 * 0.5);let eq50_e1356: f64 = (eq50_e1354 * l.f175);let eq50_e1356_d_n5: f64 = (eq50_e1354 * l.f176);let eq50_e1356_d_n6: f64 = (eq50_e1354 * l.f177);let eq50_e1356_d_n7: f64 = (eq50_e1354 * l.f178);let eq50_e1356_d_n8: f64 = (eq50_e1354 * l.f179);let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));let eq50_e1359_q: f64 = eq50_e1358;let eq50_e1360: f64 = (-eq50_e1358);let eq50_e1360_q: f64 = (-eq50_e1359_q);let eq50_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, (-eq50_e1356), (-eq50_e1358_d_n5), (-eq50_e1358_d_n6), (-eq50_e1358_d_n7), (-eq50_e1358_d_n8), 0.0, 0.0, 0.0];let eq50_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(7),
            &eq50_reactive_node_derivatives,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
