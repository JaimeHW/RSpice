#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq7_e1050, eq7_e1050_d_n4, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9,) = {
    if (l.f952 == 0.0) {
        let eq7_e1044: f64 = (l.f1b5 * l.ff9e);let eq7_e1046: f64 = (eq7_e1044 * p.p32);let eq7_e1048: f64 = (eq7_e1046 * l.fd0d);let eq7_e1048_d_n4: f64 = (eq7_e1046 * l.fd0e);let eq7_e1048_d_n6: f64 = (eq7_e1046 * l.fd0f);let eq7_e1048_d_n7: f64 = (eq7_e1046 * l.fd10);let eq7_e1048_d_n8: f64 = (eq7_e1046 * l.fd11);let eq7_e1048_d_n9: f64 = (eq7_e1046 * l.fd12);
        (eq7_e1048, eq7_e1048_d_n4, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq7_e1050_d_n4), multiplicity * (eq7_e1050_d_n6), multiplicity * (eq7_e1050_d_n7), multiplicity * (eq7_e1050_d_n8), multiplicity * (eq7_e1050_d_n9)],
            [],
            [],
            1.0,
        );let eq8_e1053: f64 = (l.f1b5 * l.ff9e);let eq8_e1055: f64 = (eq8_e1053 * p.p32);let eq8_e1057: f64 = (eq8_e1055 * l.fd07);let eq8_e1057_d_n4: f64 = (eq8_e1055 * l.fd08);let eq8_e1057_d_n6: f64 = (eq8_e1055 * l.fd09);let eq8_e1057_d_n7: f64 = (eq8_e1055 * l.fd0a);let eq8_e1057_d_n8: f64 = (eq8_e1055 * l.fd0b);let eq8_e1057_d_n9: f64 = (eq8_e1055 * l.fd0c);let eq8_value: f64 = eq8_e1057;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq8_e1057_d_n4), multiplicity * (eq8_e1057_d_n6), multiplicity * (eq8_e1057_d_n7), multiplicity * (eq8_e1057_d_n8), multiplicity * (eq8_e1057_d_n9)],
            [],
            [],
            1.0,
        );let eq9_e1060: f64 = (l.f1b5 * l.ff9e);let eq9_e1062: f64 = (eq9_e1060 * p.p32);let eq9_e1064: f64 = (eq9_e1062 * l.fd7c);let eq9_e1064_d_n4: f64 = (eq9_e1062 * l.fd7d);let eq9_e1064_d_n6: f64 = (eq9_e1062 * l.fd7e);let eq9_e1064_d_n7: f64 = (eq9_e1062 * l.fd7f);let eq9_e1064_d_n8: f64 = (eq9_e1062 * l.fd80);let eq9_e1064_d_n9: f64 = (eq9_e1062 * l.fd81);let eq9_value: f64 = eq9_e1064;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq9_e1064_d_n4), multiplicity * (eq9_e1064_d_n6), multiplicity * (eq9_e1064_d_n7), multiplicity * (eq9_e1064_d_n8), multiplicity * (eq9_e1064_d_n9)],
            [],
            [],
            1.0,
        );let eq10_e1067: f64 = (l.f1b5 * l.ff9e);let eq10_e1069: f64 = (eq10_e1067 * p.p32);let eq10_e1071: f64 = (eq10_e1069 * l.fd6a);let eq10_e1071_d_n4: f64 = (eq10_e1069 * l.fd6b);let eq10_e1071_d_n6: f64 = (eq10_e1069 * l.fd6c);let eq10_e1071_d_n7: f64 = (eq10_e1069 * l.fd6d);let eq10_e1071_d_n8: f64 = (eq10_e1069 * l.fd6e);let eq10_e1071_d_n9: f64 = (eq10_e1069 * l.fd6f);let eq10_value: f64 = eq10_e1071;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq10_e1071_d_n4), multiplicity * (eq10_e1071_d_n6), multiplicity * (eq10_e1071_d_n7), multiplicity * (eq10_e1071_d_n8), multiplicity * (eq10_e1071_d_n9)],
            [],
            [],
            1.0,
        );let eq11_e1074: f64 = (l.f1b5 * l.ff9e);let eq11_e1076: f64 = (eq11_e1074 * p.p32);let eq11_e1078: f64 = (eq11_e1076 * l.fd1f);let eq11_e1078_d_n4: f64 = (eq11_e1076 * l.fd20);let eq11_e1078_d_n6: f64 = (eq11_e1076 * l.fd21);let eq11_e1078_d_n7: f64 = (eq11_e1076 * l.fd22);let eq11_e1078_d_n8: f64 = (eq11_e1076 * l.fd23);let eq11_e1078_d_n9: f64 = (eq11_e1076 * l.fd24);let eq11_value: f64 = eq11_e1078;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq11_e1078_d_n4), multiplicity * (eq11_e1078_d_n6), multiplicity * (eq11_e1078_d_n7), multiplicity * (eq11_e1078_d_n8), multiplicity * (eq11_e1078_d_n9)],
            [],
            [],
            1.0,
        );let eq12_e1081: f64 = (l.f1b5 * l.ff9e);let eq12_e1083: f64 = (eq12_e1081 * p.p32);let eq12_e1085: f64 = (eq12_e1083 * l.fd19);let eq12_e1085_d_n4: f64 = (eq12_e1083 * l.fd1a);let eq12_e1085_d_n6: f64 = (eq12_e1083 * l.fd1b);let eq12_e1085_d_n7: f64 = (eq12_e1083 * l.fd1c);let eq12_e1085_d_n8: f64 = (eq12_e1083 * l.fd1d);let eq12_e1085_d_n9: f64 = (eq12_e1083 * l.fd1e);let eq12_value: f64 = eq12_e1085;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq12_e1085_d_n4), multiplicity * (eq12_e1085_d_n6), multiplicity * (eq12_e1085_d_n7), multiplicity * (eq12_e1085_d_n8), multiplicity * (eq12_e1085_d_n9)],
            [],
            [],
            1.0,
        );let eq13_e1088: f64 = (l.f1b5 * l.ff9e);let eq13_e1090: f64 = (eq13_e1088 * p.p32);let eq13_e1092: f64 = (eq13_e1090 * l.fd9a);let eq13_e1092_d_n6: f64 = (eq13_e1090 * l.fd9d);let eq13_e1092_d_n7: f64 = (eq13_e1090 * l.fd9e);let eq13_e1092_d_n8: f64 = (eq13_e1090 * l.fd9f);let eq13_e1092_d_n9: f64 = (eq13_e1090 * l.fda0);let eq13_e1092_d_n11: f64 = (eq13_e1090 * l.fd9b);let eq13_e1092_d_n12: f64 = (eq13_e1090 * l.fd9c);let eq13_value: f64 = eq13_e1092;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq13_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * (eq13_e1092_d_n6), multiplicity * (eq13_e1092_d_n7), multiplicity * (eq13_e1092_d_n8), multiplicity * (eq13_e1092_d_n9), multiplicity * (eq13_e1092_d_n11), multiplicity * (eq13_e1092_d_n12)],
            [],
            [],
            1.0,
        );let eq14_e1095: f64 = (l.f1b5 * l.ff9e);let eq14_e1097: f64 = (eq14_e1095 * p.p32);let eq14_e1099: f64 = (eq14_e1097 * l.fd93);let eq14_e1099_d_n6: f64 = (eq14_e1097 * l.fd96);let eq14_e1099_d_n7: f64 = (eq14_e1097 * l.fd97);let eq14_e1099_d_n8: f64 = (eq14_e1097 * l.fd98);let eq14_e1099_d_n9: f64 = (eq14_e1097 * l.fd99);let eq14_e1099_d_n11: f64 = (eq14_e1097 * l.fd94);let eq14_e1099_d_n12: f64 = (eq14_e1097 * l.fd95);let eq14_value: f64 = eq14_e1099;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(8),
            multiplicity * (eq14_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * (eq14_e1099_d_n6), multiplicity * (eq14_e1099_d_n7), multiplicity * (eq14_e1099_d_n8), multiplicity * (eq14_e1099_d_n9), multiplicity * (eq14_e1099_d_n11), multiplicity * (eq14_e1099_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq15_e1109, eq15_e1109_d_n1, eq15_e1109_d_n6,) = {
    if (l.f953 != 0.0) {
        let eq15_e1103: f64 = (l.ff9e * p.p32);let eq15_e1105: f64 = (eq15_e1103 * l.f5fc);let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));
        (eq15_e1107, eq15_e1105, (-eq15_e1105),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1109_d_n1),
            6,
            multiplicity * (eq15_e1109_d_n6),
        );
        let (eq17_e1124,) = {
    if (l.f953 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
    }
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv4 = ctx.node_voltage(nodes[4]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq18_e1134, eq18_e1134_d_n2, eq18_e1134_d_n7,) = {
    if (l.f954 != 0.0) {
        let eq18_e1128: f64 = (l.ff9e * p.p32);let eq18_e1130: f64 = (eq18_e1128 * l.f683);let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));
        (eq18_e1132, eq18_e1130, (-eq18_e1130),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(7),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1134_d_n2),
            7,
            multiplicity * (eq18_e1134_d_n7),
        );
        let (eq20_e1149,) = {
    if (l.f954 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n8,) = {
    if (l.f955 != 0.0) {
        let eq21_e1153: f64 = (l.ff9e * p.p32);let eq21_e1155: f64 = (eq21_e1153 * l.f5ba);let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));
        (eq21_e1157, eq21_e1155, (-eq21_e1155),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(8),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1159_d_n0),
            8,
            multiplicity * (eq21_e1159_d_n8),
        );
        let (eq23_e1174,) = {
    if (l.f955 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1184, eq24_e1184_d_n9, eq24_e1184_d_n10,) = {
    if (l.f956 != 0.0) {
        let eq24_e1178: f64 = (l.ff9e * p.p32);let eq24_e1180: f64 = (eq24_e1178 * l.f589);let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));
        (eq24_e1182, eq24_e1180, (-eq24_e1180),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            9,
            multiplicity * (eq24_e1184_d_n9),
            10,
            multiplicity * (eq24_e1184_d_n10),
        );
        let (eq26_e1199,) = {
    if (l.f956 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1209, eq27_e1209_d_n10, eq27_e1209_d_n11,) = {
    if (l.f959 != 0.0) {
        let eq27_e1203: f64 = (l.ff9e * p.p32);let eq27_e1205: f64 = (eq27_e1203 * l.f5fe);let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));
        (eq27_e1207, (-eq27_e1205), eq27_e1205,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e1209_d_n10),
            11,
            multiplicity * (eq27_e1209_d_n11),
        );
        let (eq29_e1224,) = {
    if (l.f959 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1234, eq30_e1234_d_n10, eq30_e1234_d_n12,) = {
    if (l.f95a != 0.0) {
        let eq30_e1228: f64 = (l.ff9e * p.p32);let eq30_e1230: f64 = (eq30_e1228 * l.f5fd);let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));
        (eq30_e1232, (-eq30_e1230), eq30_e1230,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(10),
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e1234_d_n10),
            12,
            multiplicity * (eq30_e1234_d_n12),
        );
        let (eq32_e1249,) = {
    if (l.f95a == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1259, eq33_e1259_d_n3, eq33_e1259_d_n10,) = {
    if (l.f95b != 0.0) {
        let eq33_e1253: f64 = (l.ff9e * p.p32);let eq33_e1255: f64 = (eq33_e1253 * l.fc96);let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));
        (eq33_e1257, eq33_e1255, (-eq33_e1255),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1259_d_n3),
            10,
            multiplicity * (eq33_e1259_d_n10),
        );
        let (eq35_e1274,) = {
    if (l.f95b == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );let eq36_e1277: f64 = (p.p32 * l.f5ff);let eq36_e1279: f64 = (eq36_e1277 * (nv8 - nv9));let eq36_value: f64 = eq36_e1279;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq36_value),
            8,
            multiplicity * (eq36_e1277),
            9,
            multiplicity * ((-eq36_e1277)),
        );let eq37_e1282: f64 = (p.p32 * l.f5ff);let eq37_e1284: f64 = (eq37_e1282 * (nv7 - nv9));let eq37_value: f64 = eq37_e1284;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(9),
            multiplicity * (eq37_value),
            7,
            multiplicity * (eq37_e1282),
            9,
            multiplicity * ((-eq37_e1282)),
        );let eq38_e1286: f64 = (-l.ff9e);let eq38_e1288: f64 = (eq38_e1286 * l.f104c);let eq38_e1288_d_n0: f64 = (eq38_e1286 * l.f104d);let eq38_e1288_d_n2: f64 = (eq38_e1286 * l.f104e);let eq38_e1288_d_n4: f64 = (eq38_e1286 * l.f104f);let eq38_e1288_d_n6: f64 = (eq38_e1286 * l.f1050);let eq38_e1288_d_n7: f64 = (eq38_e1286 * l.f1051);let eq38_e1288_d_n8: f64 = (eq38_e1286 * l.f1052);let eq38_e1288_d_n9: f64 = (eq38_e1286 * l.f1053);let eq38_value: f64 = eq38_e1288;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 4, 6, 7, 8, 9],
            [multiplicity * (eq38_e1288_d_n0), multiplicity * (eq38_e1288_d_n2), multiplicity * (eq38_e1288_d_n4), multiplicity * (eq38_e1288_d_n6), multiplicity * (eq38_e1288_d_n7), multiplicity * (eq38_e1288_d_n8), multiplicity * (eq38_e1288_d_n9)],
            [],
            [],
            1.0,
        );let eq39_e1291: f64 = (l.ff9e * l.f223);let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq39_e1293);let eq39_value: f64 = eq39_e1294;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * ((eq39_e1291 * ddt_scale)),
        );let eq40_e1297: f64 = (l.ff9e * (nv4 - 0.0));let __rspice_inv_cse_0: f64 = 1.0 / l.f144c;let eq40_e1299: f64 = (eq40_e1297 * __rspice_inv_cse_0);let eq40_e1299_d_n4: f64 = (l.ff9e * __rspice_inv_cse_0);let eq40_value: f64 = eq40_e1299;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1299_d_n4),
        );let eq41_e1302: f64 = (l.f1b5 * l.ff9e);let eq41_e1304: f64 = (eq41_e1302 * p.p33);let eq41_e1306: f64 = (eq41_e1304 * l.f12a7);let eq41_e1306_d_n4: f64 = (eq41_e1304 * l.f12af);let eq41_e1306_d_n6: f64 = (eq41_e1304 * l.f12b0);let eq41_e1306_d_n7: f64 = (eq41_e1304 * l.f12b1);let eq41_e1306_d_n8: f64 = (eq41_e1304 * l.f12b2);let eq41_e1306_d_n9: f64 = (eq41_e1304 * l.f12b3);let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);let eq41_value: f64 = eq41_e1307;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq41_e1306_d_n4 * ddt_scale)), multiplicity * ((eq41_e1306_d_n6 * ddt_scale)), multiplicity * ((eq41_e1306_d_n7 * ddt_scale)), multiplicity * ((eq41_e1306_d_n8 * ddt_scale)), multiplicity * ((eq41_e1306_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
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
        let eq42_e1310: f64 = (l.f1b5 * l.ff9e);let eq42_e1312: f64 = (eq42_e1310 * p.p33);let eq42_e1314: f64 = (eq42_e1312 * l.f11c1);let eq42_e1314_d_n4: f64 = (eq42_e1312 * l.f11cc);let eq42_e1314_d_n6: f64 = (eq42_e1312 * l.f11cd);let eq42_e1314_d_n7: f64 = (eq42_e1312 * l.f11ce);let eq42_e1314_d_n8: f64 = (eq42_e1312 * l.f11cf);let eq42_e1314_d_n9: f64 = (eq42_e1312 * l.f11d0);let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq42_e1314);let eq42_value: f64 = eq42_e1315;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq42_e1314_d_n4 * ddt_scale)), multiplicity * ((eq42_e1314_d_n6 * ddt_scale)), multiplicity * ((eq42_e1314_d_n7 * ddt_scale)), multiplicity * ((eq42_e1314_d_n8 * ddt_scale)), multiplicity * ((eq42_e1314_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq43_e1318: f64 = (l.f1b5 * l.ff9e);let eq43_e1320: f64 = (eq43_e1318 * p.p33);let eq43_e1322: f64 = (eq43_e1320 * l.f1250);let eq43_e1322_d_n4: f64 = (eq43_e1320 * l.f1258);let eq43_e1322_d_n6: f64 = (eq43_e1320 * l.f1259);let eq43_e1322_d_n7: f64 = (eq43_e1320 * l.f125a);let eq43_e1322_d_n8: f64 = (eq43_e1320 * l.f125b);let eq43_e1322_d_n9: f64 = (eq43_e1320 * l.f125c);let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq43_e1322);let eq43_value: f64 = eq43_e1323;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq43_e1322_d_n4 * ddt_scale)), multiplicity * ((eq43_e1322_d_n6 * ddt_scale)), multiplicity * ((eq43_e1322_d_n7 * ddt_scale)), multiplicity * ((eq43_e1322_d_n8 * ddt_scale)), multiplicity * ((eq43_e1322_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq44_e1326: f64 = (l.f1b5 * l.ff9e);let eq44_e1328: f64 = (eq44_e1326 * p.p33);let eq44_e1330: f64 = (eq44_e1328 * l.f12a2);let eq44_e1330_d_n6: f64 = (eq44_e1328 * l.f12a3);let eq44_e1330_d_n7: f64 = (eq44_e1328 * l.f12a4);let eq44_e1330_d_n8: f64 = (eq44_e1328 * l.f12a5);let eq44_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq44_e1330);let eq44_value: f64 = eq44_e1331;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * (eq44_value),
            6,
            multiplicity * ((eq44_e1330_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq44_e1330_d_n7 * ddt_scale)),
            8,
            multiplicity * ((eq44_e1330_d_n8 * ddt_scale)),
        );let eq45_e1334: f64 = (l.f1b5 * l.ff9e);let eq45_e1336: f64 = (eq45_e1334 * p.p33);let eq45_e1338: f64 = (eq45_e1336 * l.f129d);let eq45_e1338_d_n6: f64 = (eq45_e1336 * l.f129e);let eq45_e1338_d_n7: f64 = (eq45_e1336 * l.f129f);let eq45_e1338_d_n8: f64 = (eq45_e1336 * l.f12a0);let eq45_e1339: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq45_e1338);let eq45_value: f64 = eq45_e1339;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(8),
            multiplicity * (eq45_value),
            6,
            multiplicity * ((eq45_e1338_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq45_e1338_d_n7 * ddt_scale)),
            8,
            multiplicity * ((eq45_e1338_d_n8 * ddt_scale)),
        );let eq46_e1342: f64 = (l.f1b5 * l.ff9e);let eq46_e1344: f64 = (eq46_e1342 * p.p33);let eq46_e1346: f64 = (eq46_e1344 * l.f12ca);let eq46_e1346_d_n4: f64 = (eq46_e1344 * l.f12cb);let eq46_e1346_d_n6: f64 = (eq46_e1344 * l.f12cc);let eq46_e1346_d_n7: f64 = (eq46_e1344 * l.f12cd);let eq46_e1346_d_n8: f64 = (eq46_e1344 * l.f12ce);let eq46_e1346_d_n9: f64 = (eq46_e1344 * l.f12cf);let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq46_e1346);let eq46_value: f64 = eq46_e1347;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq46_e1346_d_n4 * ddt_scale)), multiplicity * ((eq46_e1346_d_n6 * ddt_scale)), multiplicity * ((eq46_e1346_d_n7 * ddt_scale)), multiplicity * ((eq46_e1346_d_n8 * ddt_scale)), multiplicity * ((eq46_e1346_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq47_e1350: f64 = (l.f1b5 * l.ff9e);let eq47_e1352: f64 = (eq47_e1350 * p.p33);let eq47_e1354: f64 = (eq47_e1352 * l.f1376);let eq47_e1354_d_n6: f64 = (eq47_e1352 * l.f1379);let eq47_e1354_d_n7: f64 = (eq47_e1352 * l.f137a);let eq47_e1354_d_n8: f64 = (eq47_e1352 * l.f137b);let eq47_e1354_d_n9: f64 = (eq47_e1352 * l.f137c);let eq47_e1354_d_n11: f64 = (eq47_e1352 * l.f1377);let eq47_e1354_d_n12: f64 = (eq47_e1352 * l.f1378);let eq47_e1355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq47_e1354);let eq47_value: f64 = eq47_e1355;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq47_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * ((eq47_e1354_d_n6 * ddt_scale)), multiplicity * ((eq47_e1354_d_n7 * ddt_scale)), multiplicity * ((eq47_e1354_d_n8 * ddt_scale)), multiplicity * ((eq47_e1354_d_n9 * ddt_scale)), multiplicity * ((eq47_e1354_d_n11 * ddt_scale)), multiplicity * ((eq47_e1354_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq48_e1358: f64 = (l.f1b5 * l.ff9e);let eq48_e1360: f64 = (eq48_e1358 * p.p33);let eq48_e1362: f64 = (eq48_e1360 * l.f136e);let eq48_e1362_d_n6: f64 = (eq48_e1360 * l.f1371);let eq48_e1362_d_n7: f64 = (eq48_e1360 * l.f1372);let eq48_e1362_d_n8: f64 = (eq48_e1360 * l.f1373);let eq48_e1362_d_n9: f64 = (eq48_e1360 * l.f1374);let eq48_e1362_d_n11: f64 = (eq48_e1360 * l.f136f);let eq48_e1362_d_n12: f64 = (eq48_e1360 * l.f1370);let eq48_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1362);let eq48_value: f64 = eq48_e1363;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(8),
            multiplicity * (eq48_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * ((eq48_e1362_d_n6 * ddt_scale)), multiplicity * ((eq48_e1362_d_n7 * ddt_scale)), multiplicity * ((eq48_e1362_d_n8 * ddt_scale)), multiplicity * ((eq48_e1362_d_n9 * ddt_scale)), multiplicity * ((eq48_e1362_d_n11 * ddt_scale)), multiplicity * ((eq48_e1362_d_n12 * ddt_scale))],
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
        let nv5 = ctx.node_voltage(nodes[5]);let eq50_e1371: f64 = ((nv5 - 0.0) / l.ff6e);let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * l.ff6f) / (l.ff6e * l.ff6e)));let eq50_e1371_d_n5: f64 = (1.0 / l.ff6e);let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * l.ff70) / (l.ff6e * l.ff6e)));let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * l.ff71) / (l.ff6e * l.ff6e)));let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * l.ff72) / (l.ff6e * l.ff6e)));let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * l.ff73) / (l.ff6e * l.ff6e)));let eq50_value: f64 = eq50_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq50_e1371_d_n4), multiplicity * (eq50_e1371_d_n5), multiplicity * (eq50_e1371_d_n6), multiplicity * (eq50_e1371_d_n7), multiplicity * (eq50_e1371_d_n8), multiplicity * (eq50_e1371_d_n9)],
            [],
            [],
            1.0,
        );let eq51_e1374: f64 = (l.f196 * (nv5 - 0.0));let eq51_e1374_d_n4: f64 = (l.f197 * (nv5 - 0.0));let eq51_e1374_d_n6: f64 = (l.f198 * (nv5 - 0.0));let eq51_e1374_d_n7: f64 = (l.f199 * (nv5 - 0.0));let eq51_e1374_d_n8: f64 = (l.f19a * (nv5 - 0.0));let eq51_e1374_d_n9: f64 = (l.f19b * (nv5 - 0.0));let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1374);let eq51_value: f64 = eq51_e1375;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq51_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq51_e1374_d_n4 * ddt_scale)), multiplicity * ((l.f196 * ddt_scale)), multiplicity * ((eq51_e1374_d_n6 * ddt_scale)), multiplicity * ((eq51_e1374_d_n7 * ddt_scale)), multiplicity * ((eq51_e1374_d_n8 * ddt_scale)), multiplicity * ((eq51_e1374_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq52_e1378: f64 = (l.ff9e * p.p32);let eq52_e1379: f64 = (eq52_e1378).sqrt();let eq52_e1381: f64 = (eq52_e1379 * 0.5);let eq52_e1383: f64 = (eq52_e1381 * l.f196);let eq52_e1383_d_n4: f64 = (eq52_e1381 * l.f197);let eq52_e1383_d_n6: f64 = (eq52_e1381 * l.f198);let eq52_e1383_d_n7: f64 = (eq52_e1381 * l.f199);let eq52_e1383_d_n8: f64 = (eq52_e1381 * l.f19a);let eq52_e1383_d_n9: f64 = (eq52_e1381 * l.f19b);let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1385);let eq52_e1387: f64 = (-eq52_e1386);let eq52_e1387_d_n4: f64 = (-(eq52_e1385_d_n4 * ddt_scale));let eq52_e1387_d_n5: f64 = (-(eq52_e1383 * ddt_scale));let eq52_e1387_d_n6: f64 = (-(eq52_e1385_d_n6 * ddt_scale));let eq52_e1387_d_n7: f64 = (-(eq52_e1385_d_n7 * ddt_scale));let eq52_e1387_d_n8: f64 = (-(eq52_e1385_d_n8 * ddt_scale));let eq52_e1387_d_n9: f64 = (-(eq52_e1385_d_n9 * ddt_scale));let eq52_value: f64 = eq52_e1387;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq52_e1387_d_n4), multiplicity * (eq52_e1387_d_n5), multiplicity * (eq52_e1387_d_n6), multiplicity * (eq52_e1387_d_n7), multiplicity * (eq52_e1387_d_n8), multiplicity * (eq52_e1387_d_n9)],
            [],
            [],
            1.0,
        );let eq53_e1390: f64 = (l.ff9e * p.p32);let eq53_e1391: f64 = (eq53_e1390).sqrt();let eq53_e1393: f64 = (eq53_e1391 * 0.5);let eq53_e1395: f64 = (eq53_e1393 * l.f196);let eq53_e1395_d_n4: f64 = (eq53_e1393 * l.f197);let eq53_e1395_d_n6: f64 = (eq53_e1393 * l.f198);let eq53_e1395_d_n7: f64 = (eq53_e1393 * l.f199);let eq53_e1395_d_n8: f64 = (eq53_e1393 * l.f19a);let eq53_e1395_d_n9: f64 = (eq53_e1393 * l.f19b);let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1397);let eq53_e1399: f64 = (-eq53_e1398);let eq53_e1399_d_n4: f64 = (-(eq53_e1397_d_n4 * ddt_scale));let eq53_e1399_d_n5: f64 = (-(eq53_e1395 * ddt_scale));let eq53_e1399_d_n6: f64 = (-(eq53_e1397_d_n6 * ddt_scale));let eq53_e1399_d_n7: f64 = (-(eq53_e1397_d_n7 * ddt_scale));let eq53_e1399_d_n8: f64 = (-(eq53_e1397_d_n8 * ddt_scale));let eq53_e1399_d_n9: f64 = (-(eq53_e1397_d_n9 * ddt_scale));let eq53_value: f64 = eq53_e1399;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq53_e1399_d_n4), multiplicity * (eq53_e1399_d_n5), multiplicity * (eq53_e1399_d_n6), multiplicity * (eq53_e1399_d_n7), multiplicity * (eq53_e1399_d_n8), multiplicity * (eq53_e1399_d_n9)],
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
        let nv4 = ctx.node_voltage(nodes[4]);let eq39_e1291: f64 = (l.ff9e * l.f223);let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));let eq39_e1294_q: f64 = eq39_e1293;
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (eq39_e1291),
        );let eq41_e1302: f64 = (l.f1b5 * l.ff9e);let eq41_e1304: f64 = (eq41_e1302 * p.p33);let eq41_e1306: f64 = (eq41_e1304 * l.f12a7);let eq41_e1306_d_n4: f64 = (eq41_e1304 * l.f12af);let eq41_e1306_d_n6: f64 = (eq41_e1304 * l.f12b0);let eq41_e1306_d_n7: f64 = (eq41_e1304 * l.f12b1);let eq41_e1306_d_n8: f64 = (eq41_e1304 * l.f12b2);let eq41_e1306_d_n9: f64 = (eq41_e1304 * l.f12b3);let eq41_e1307_q: f64 = eq41_e1306;let eq41_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq41_e1306_d_n4, 0.0, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, 0.0, 0.0, 0.0];let eq41_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(7),
            &eq41_reactive_node_derivatives,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );let eq42_e1310: f64 = (l.f1b5 * l.ff9e);let eq42_e1312: f64 = (eq42_e1310 * p.p33);let eq42_e1314: f64 = (eq42_e1312 * l.f11c1);let eq42_e1314_d_n4: f64 = (eq42_e1312 * l.f11cc);let eq42_e1314_d_n6: f64 = (eq42_e1312 * l.f11cd);let eq42_e1314_d_n7: f64 = (eq42_e1312 * l.f11ce);let eq42_e1314_d_n8: f64 = (eq42_e1312 * l.f11cf);let eq42_e1314_d_n9: f64 = (eq42_e1312 * l.f11d0);let eq42_e1315_q: f64 = eq42_e1314;let eq42_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq42_e1314_d_n4, 0.0, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, 0.0, 0.0, 0.0];let eq42_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq42_reactive_node_derivatives,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );let eq43_e1318: f64 = (l.f1b5 * l.ff9e);let eq43_e1320: f64 = (eq43_e1318 * p.p33);let eq43_e1322: f64 = (eq43_e1320 * l.f1250);let eq43_e1322_d_n4: f64 = (eq43_e1320 * l.f1258);let eq43_e1322_d_n6: f64 = (eq43_e1320 * l.f1259);let eq43_e1322_d_n7: f64 = (eq43_e1320 * l.f125a);let eq43_e1322_d_n8: f64 = (eq43_e1320 * l.f125b);let eq43_e1322_d_n9: f64 = (eq43_e1320 * l.f125c);let eq43_e1323_q: f64 = eq43_e1322;let eq43_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq43_e1322_d_n4, 0.0, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, 0.0, 0.0, 0.0];let eq43_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(7),
            &eq43_reactive_node_derivatives,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );let eq44_e1326: f64 = (l.f1b5 * l.ff9e);let eq44_e1328: f64 = (eq44_e1326 * p.p33);let eq44_e1330: f64 = (eq44_e1328 * l.f12a2);let eq44_e1330_d_n6: f64 = (eq44_e1328 * l.f12a3);let eq44_e1330_d_n7: f64 = (eq44_e1328 * l.f12a4);let eq44_e1330_d_n8: f64 = (eq44_e1328 * l.f12a5);let eq44_e1331_q: f64 = eq44_e1330;
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(7),
            6,
            multiplicity * (eq44_e1330_d_n6),
            7,
            multiplicity * (eq44_e1330_d_n7),
            8,
            multiplicity * (eq44_e1330_d_n8),
        );let eq45_e1334: f64 = (l.f1b5 * l.ff9e);let eq45_e1336: f64 = (eq45_e1334 * p.p33);let eq45_e1338: f64 = (eq45_e1336 * l.f129d);let eq45_e1338_d_n6: f64 = (eq45_e1336 * l.f129e);let eq45_e1338_d_n7: f64 = (eq45_e1336 * l.f129f);let eq45_e1338_d_n8: f64 = (eq45_e1336 * l.f12a0);let eq45_e1339_q: f64 = eq45_e1338;
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(8),
            6,
            multiplicity * (eq45_e1338_d_n6),
            7,
            multiplicity * (eq45_e1338_d_n7),
            8,
            multiplicity * (eq45_e1338_d_n8),
        );let eq46_e1342: f64 = (l.f1b5 * l.ff9e);let eq46_e1344: f64 = (eq46_e1342 * p.p33);let eq46_e1346: f64 = (eq46_e1344 * l.f12ca);let eq46_e1346_d_n4: f64 = (eq46_e1344 * l.f12cb);let eq46_e1346_d_n6: f64 = (eq46_e1344 * l.f12cc);let eq46_e1346_d_n7: f64 = (eq46_e1344 * l.f12cd);let eq46_e1346_d_n8: f64 = (eq46_e1344 * l.f12ce);let eq46_e1346_d_n9: f64 = (eq46_e1344 * l.f12cf);let eq46_e1347_q: f64 = eq46_e1346;let eq46_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq46_e1346_d_n4, 0.0, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, 0.0, 0.0, 0.0];let eq46_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(9),
            &eq46_reactive_node_derivatives,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );let eq47_e1350: f64 = (l.f1b5 * l.ff9e);let eq47_e1352: f64 = (eq47_e1350 * p.p33);let eq47_e1354: f64 = (eq47_e1352 * l.f1376);let eq47_e1354_d_n6: f64 = (eq47_e1352 * l.f1379);let eq47_e1354_d_n7: f64 = (eq47_e1352 * l.f137a);let eq47_e1354_d_n8: f64 = (eq47_e1352 * l.f137b);let eq47_e1354_d_n9: f64 = (eq47_e1352 * l.f137c);let eq47_e1354_d_n11: f64 = (eq47_e1352 * l.f1377);let eq47_e1354_d_n12: f64 = (eq47_e1352 * l.f1378);let eq47_e1355_q: f64 = eq47_e1354;let eq47_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq47_e1354_d_n6, eq47_e1354_d_n7, eq47_e1354_d_n8, eq47_e1354_d_n9, 0.0, eq47_e1354_d_n11, eq47_e1354_d_n12];let eq47_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(7),
            &eq47_reactive_node_derivatives,
            &eq47_reactive_branch_derivatives,
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
        let nv5 = ctx.node_voltage(nodes[5]);let eq48_e1358: f64 = (l.f1b5 * l.ff9e);let eq48_e1360: f64 = (eq48_e1358 * p.p33);let eq48_e1362: f64 = (eq48_e1360 * l.f136e);let eq48_e1362_d_n6: f64 = (eq48_e1360 * l.f1371);let eq48_e1362_d_n7: f64 = (eq48_e1360 * l.f1372);let eq48_e1362_d_n8: f64 = (eq48_e1360 * l.f1373);let eq48_e1362_d_n9: f64 = (eq48_e1360 * l.f1374);let eq48_e1362_d_n11: f64 = (eq48_e1360 * l.f136f);let eq48_e1362_d_n12: f64 = (eq48_e1360 * l.f1370);let eq48_e1363_q: f64 = eq48_e1362;let eq48_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq48_e1362_d_n6, eq48_e1362_d_n7, eq48_e1362_d_n8, eq48_e1362_d_n9, 0.0, eq48_e1362_d_n11, eq48_e1362_d_n12];let eq48_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            Some(8),
            &eq48_reactive_node_derivatives,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );let eq51_e1374: f64 = (l.f196 * (nv5 - 0.0));let eq51_e1374_d_n4: f64 = (l.f197 * (nv5 - 0.0));let eq51_e1374_d_n6: f64 = (l.f198 * (nv5 - 0.0));let eq51_e1374_d_n7: f64 = (l.f199 * (nv5 - 0.0));let eq51_e1374_d_n8: f64 = (l.f19a * (nv5 - 0.0));let eq51_e1374_d_n9: f64 = (l.f19b * (nv5 - 0.0));let eq51_e1375_q: f64 = eq51_e1374;let eq51_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq51_e1374_d_n4, l.f196, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, 0.0, 0.0, 0.0];let eq51_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            None,
            &eq51_reactive_node_derivatives,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );let eq52_e1378: f64 = (l.ff9e * p.p32);let eq52_e1379: f64 = (eq52_e1378).sqrt();let eq52_e1381: f64 = (eq52_e1379 * 0.5);let eq52_e1383: f64 = (eq52_e1381 * l.f196);let eq52_e1383_d_n4: f64 = (eq52_e1381 * l.f197);let eq52_e1383_d_n6: f64 = (eq52_e1381 * l.f198);let eq52_e1383_d_n7: f64 = (eq52_e1381 * l.f199);let eq52_e1383_d_n8: f64 = (eq52_e1381 * l.f19a);let eq52_e1383_d_n9: f64 = (eq52_e1381 * l.f19b);let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));let eq52_e1386_q: f64 = eq52_e1385;let eq52_e1387: f64 = (-eq52_e1385);let eq52_e1387_q: f64 = (-eq52_e1386_q);let eq52_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq52_e1385_d_n4), (-eq52_e1383), (-eq52_e1385_d_n6), (-eq52_e1385_d_n7), (-eq52_e1385_d_n8), (-eq52_e1385_d_n9), 0.0, 0.0, 0.0];let eq52_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(7),
            &eq52_reactive_node_derivatives,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );let eq53_e1390: f64 = (l.ff9e * p.p32);let eq53_e1391: f64 = (eq53_e1390).sqrt();let eq53_e1393: f64 = (eq53_e1391 * 0.5);let eq53_e1395: f64 = (eq53_e1393 * l.f196);let eq53_e1395_d_n4: f64 = (eq53_e1393 * l.f197);let eq53_e1395_d_n6: f64 = (eq53_e1393 * l.f198);let eq53_e1395_d_n7: f64 = (eq53_e1393 * l.f199);let eq53_e1395_d_n8: f64 = (eq53_e1393 * l.f19a);let eq53_e1395_d_n9: f64 = (eq53_e1393 * l.f19b);let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));let eq53_e1398_q: f64 = eq53_e1397;let eq53_e1399: f64 = (-eq53_e1397);let eq53_e1399_q: f64 = (-eq53_e1398_q);let eq53_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq53_e1397_d_n4), (-eq53_e1395), (-eq53_e1397_d_n6), (-eq53_e1397_d_n7), (-eq53_e1397_d_n8), (-eq53_e1397_d_n9), 0.0, 0.0, 0.0];let eq53_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(8),
            &eq53_reactive_node_derivatives,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
