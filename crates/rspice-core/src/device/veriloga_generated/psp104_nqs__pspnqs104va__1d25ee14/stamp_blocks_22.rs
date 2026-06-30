#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
        var_chnl_type: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjund: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1925: f64,
        var_guard1926: f64,
        var_guard1927: f64,
        var_guard1928: f64,
        var_guard1929: f64,
        var_guard1930: f64,
        var_guard1931: f64,
        var_gwell: f64,
        var_ijun_d: f64,
        var_ijun_d_db0: f64,
        var_ijun_d_db1: f64,
        var_ijun_d_db10: f64,
        var_ijun_d_db11: f64,
        var_ijun_d_db12: f64,
        var_ijun_d_db13: f64,
        var_ijun_d_db14: f64,
        var_ijun_d_db15: f64,
        var_ijun_d_db16: f64,
        var_ijun_d_db17: f64,
        var_ijun_d_db18: f64,
        var_ijun_d_db19: f64,
        var_ijun_d_db2: f64,
        var_ijun_d_db20: f64,
        var_ijun_d_db21: f64,
        var_ijun_d_db22: f64,
        var_ijun_d_db23: f64,
        var_ijun_d_db24: f64,
        var_ijun_d_db3: f64,
        var_ijun_d_db4: f64,
        var_ijun_d_db5: f64,
        var_ijun_d_db6: f64,
        var_ijun_d_db7: f64,
        var_ijun_d_db8: f64,
        var_ijun_d_db9: f64,
        var_ijun_d_dn0: f64,
        var_ijun_d_dn1: f64,
        var_ijun_d_dn10: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn12: f64,
        var_ijun_d_dn13: f64,
        var_ijun_d_dn14: f64,
        var_ijun_d_dn15: f64,
        var_ijun_d_dn16: f64,
        var_ijun_d_dn17: f64,
        var_ijun_d_dn18: f64,
        var_ijun_d_dn19: f64,
        var_ijun_d_dn2: f64,
        var_ijun_d_dn20: f64,
        var_ijun_d_dn3: f64,
        var_ijun_d_dn4: f64,
        var_ijun_d_dn5: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_ijun_d_dn9: f64,
        var_mult_inst: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq14_e1078: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1080: f64 = (eq14_e1078 * p.p32);
        let eq14_e1082: f64 = (eq14_e1080 * var_ijun_d);
        let eq14_e1082_d_n0: f64 = (eq14_e1080 * var_ijun_d_dn0);
        let eq14_e1082_d_n1: f64 = (eq14_e1080 * var_ijun_d_dn1);
        let eq14_e1082_d_n2: f64 = (eq14_e1080 * var_ijun_d_dn2);
        let eq14_e1082_d_n3: f64 = (eq14_e1080 * var_ijun_d_dn3);
        let eq14_e1082_d_n4: f64 = (eq14_e1080 * var_ijun_d_dn4);
        let eq14_e1082_d_n5: f64 = (eq14_e1080 * var_ijun_d_dn5);
        let eq14_e1082_d_n6: f64 = (eq14_e1080 * var_ijun_d_dn6);
        let eq14_e1082_d_n7: f64 = (eq14_e1080 * var_ijun_d_dn7);
        let eq14_e1082_d_n8: f64 = (eq14_e1080 * var_ijun_d_dn8);
        let eq14_e1082_d_n9: f64 = (eq14_e1080 * var_ijun_d_dn9);
        let eq14_e1082_d_n10: f64 = (eq14_e1080 * var_ijun_d_dn10);
        let eq14_e1082_d_n11: f64 = (eq14_e1080 * var_ijun_d_dn11);
        let eq14_e1082_d_n12: f64 = (eq14_e1080 * var_ijun_d_dn12);
        let eq14_e1082_d_n13: f64 = (eq14_e1080 * var_ijun_d_dn13);
        let eq14_e1082_d_n14: f64 = (eq14_e1080 * var_ijun_d_dn14);
        let eq14_e1082_d_n15: f64 = (eq14_e1080 * var_ijun_d_dn15);
        let eq14_e1082_d_n16: f64 = (eq14_e1080 * var_ijun_d_dn16);
        let eq14_e1082_d_n17: f64 = (eq14_e1080 * var_ijun_d_dn17);
        let eq14_e1082_d_n18: f64 = (eq14_e1080 * var_ijun_d_dn18);
        let eq14_e1082_d_n19: f64 = (eq14_e1080 * var_ijun_d_dn19);
        let eq14_e1082_d_n20: f64 = (eq14_e1080 * var_ijun_d_dn20);
        let eq14_e1082_d_b0: f64 = (eq14_e1080 * var_ijun_d_db0);
        let eq14_e1082_d_b1: f64 = (eq14_e1080 * var_ijun_d_db1);
        let eq14_e1082_d_b2: f64 = (eq14_e1080 * var_ijun_d_db2);
        let eq14_e1082_d_b3: f64 = (eq14_e1080 * var_ijun_d_db3);
        let eq14_e1082_d_b4: f64 = (eq14_e1080 * var_ijun_d_db4);
        let eq14_e1082_d_b5: f64 = (eq14_e1080 * var_ijun_d_db5);
        let eq14_e1082_d_b6: f64 = (eq14_e1080 * var_ijun_d_db6);
        let eq14_e1082_d_b7: f64 = (eq14_e1080 * var_ijun_d_db7);
        let eq14_e1082_d_b8: f64 = (eq14_e1080 * var_ijun_d_db8);
        let eq14_e1082_d_b9: f64 = (eq14_e1080 * var_ijun_d_db9);
        let eq14_e1082_d_b10: f64 = (eq14_e1080 * var_ijun_d_db10);
        let eq14_e1082_d_b11: f64 = (eq14_e1080 * var_ijun_d_db11);
        let eq14_e1082_d_b12: f64 = (eq14_e1080 * var_ijun_d_db12);
        let eq14_e1082_d_b13: f64 = (eq14_e1080 * var_ijun_d_db13);
        let eq14_e1082_d_b14: f64 = (eq14_e1080 * var_ijun_d_db14);
        let eq14_e1082_d_b15: f64 = (eq14_e1080 * var_ijun_d_db15);
        let eq14_e1082_d_b16: f64 = (eq14_e1080 * var_ijun_d_db16);
        let eq14_e1082_d_b17: f64 = (eq14_e1080 * var_ijun_d_db17);
        let eq14_e1082_d_b18: f64 = (eq14_e1080 * var_ijun_d_db18);
        let eq14_e1082_d_b19: f64 = (eq14_e1080 * var_ijun_d_db19);
        let eq14_e1082_d_b20: f64 = (eq14_e1080 * var_ijun_d_db20);
        let eq14_e1082_d_b21: f64 = (eq14_e1080 * var_ijun_d_db21);
        let eq14_e1082_d_b22: f64 = (eq14_e1080 * var_ijun_d_db22);
        let eq14_e1082_d_b23: f64 = (eq14_e1080 * var_ijun_d_db23);
        let eq14_e1082_d_b24: f64 = (eq14_e1080 * var_ijun_d_db24);
        let eq14_value: f64 = eq14_e1082;
        let eq14_node_derivatives: [f64; 21] = [eq14_e1082_d_n0, eq14_e1082_d_n1, eq14_e1082_d_n2, eq14_e1082_d_n3, eq14_e1082_d_n4, eq14_e1082_d_n5, eq14_e1082_d_n6, eq14_e1082_d_n7, eq14_e1082_d_n8, eq14_e1082_d_n9, eq14_e1082_d_n10, eq14_e1082_d_n11, eq14_e1082_d_n12, eq14_e1082_d_n13, eq14_e1082_d_n14, eq14_e1082_d_n15, eq14_e1082_d_n16, eq14_e1082_d_n17, eq14_e1082_d_n18, eq14_e1082_d_n19, eq14_e1082_d_n20];
        let eq14_branch_derivatives: [f64; 25] = [eq14_e1082_d_b0, eq14_e1082_d_b1, eq14_e1082_d_b2, eq14_e1082_d_b3, eq14_e1082_d_b4, eq14_e1082_d_b5, eq14_e1082_d_b6, eq14_e1082_d_b7, eq14_e1082_d_b8, eq14_e1082_d_b9, eq14_e1082_d_b10, eq14_e1082_d_b11, eq14_e1082_d_b12, eq14_e1082_d_b13, eq14_e1082_d_b14, eq14_e1082_d_b15, eq14_e1082_d_b16, eq14_e1082_d_b17, eq14_e1082_d_b18, eq14_e1082_d_b19, eq14_e1082_d_b20, eq14_e1082_d_b21, eq14_e1082_d_b22, eq14_e1082_d_b23, eq14_e1082_d_b24];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1092, eq15_e1092_d_n1, eq15_e1092_d_n5,) = {
    if (var_guard1925 != 0.0) {
        let eq15_e1086: f64 = (var_mult_inst * p.p32);
        let eq15_e1088: f64 = (eq15_e1086 * var_ggate);
        let eq15_e1090: f64 = (eq15_e1088 * (nv1 - nv5));
        (eq15_e1090, eq15_e1088, (-eq15_e1088),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1092;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1092_d_n1),
            5,
            multiplicity * (eq15_e1092_d_n5),
        );
        let (eq17_e1107,) = {
    if (var_guard1925 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1107;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1117, eq18_e1117_d_n2, eq18_e1117_d_n6,) = {
    if (var_guard1926 != 0.0) {
        let eq18_e1111: f64 = (var_mult_inst * p.p32);
        let eq18_e1113: f64 = (eq18_e1111 * var_gsource);
        let eq18_e1115: f64 = (eq18_e1113 * (nv2 - nv6));
        (eq18_e1115, eq18_e1113, (-eq18_e1113),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1117;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1117_d_n2),
            6,
            multiplicity * (eq18_e1117_d_n6),
        );
        let (eq20_e1132,) = {
    if (var_guard1926 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1132;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1142, eq21_e1142_d_n0, eq21_e1142_d_n7,) = {
    if (var_guard1927 != 0.0) {
        let eq21_e1136: f64 = (var_mult_inst * p.p32);
        let eq21_e1138: f64 = (eq21_e1136 * var_gdrain);
        let eq21_e1140: f64 = (eq21_e1138 * (nv0 - nv7));
        (eq21_e1140, eq21_e1138, (-eq21_e1138),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1142;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1142_d_n0),
            7,
            multiplicity * (eq21_e1142_d_n7),
        );
        let (eq23_e1157,) = {
    if (var_guard1927 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1157;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1167, eq24_e1167_d_n8, eq24_e1167_d_n9,) = {
    if (var_guard1928 != 0.0) {
        let eq24_e1161: f64 = (var_mult_inst * p.p32);
        let eq24_e1163: f64 = (eq24_e1161 * var_gbulk);
        let eq24_e1165: f64 = (eq24_e1163 * (nv8 - nv9));
        (eq24_e1165, eq24_e1163, (-eq24_e1163),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1167;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            8,
            multiplicity * (eq24_e1167_d_n8),
            9,
            multiplicity * (eq24_e1167_d_n9),
        );
        let (eq26_e1182,) = {
    if (var_guard1928 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1182;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1192, eq27_e1192_d_n9, eq27_e1192_d_n10,) = {
    if (var_guard1929 != 0.0) {
        let eq27_e1186: f64 = (var_mult_inst * p.p32);
        let eq27_e1188: f64 = (eq27_e1186 * var_gjuns);
        let eq27_e1190: f64 = (eq27_e1188 * (nv10 - nv9));
        (eq27_e1190, (-eq27_e1188), eq27_e1188,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1192;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            9,
            multiplicity * (eq27_e1192_d_n9),
            10,
            multiplicity * (eq27_e1192_d_n10),
        );
        let (eq29_e1207,) = {
    if (var_guard1929 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1207;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1217, eq30_e1217_d_n9, eq30_e1217_d_n11,) = {
    if (var_guard1930 != 0.0) {
        let eq30_e1211: f64 = (var_mult_inst * p.p32);
        let eq30_e1213: f64 = (eq30_e1211 * var_gjund);
        let eq30_e1215: f64 = (eq30_e1213 * (nv11 - nv9));
        (eq30_e1215, (-eq30_e1213), eq30_e1213,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1217;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            9,
            multiplicity * (eq30_e1217_d_n9),
            11,
            multiplicity * (eq30_e1217_d_n11),
        );
        let (eq32_e1232,) = {
    if (var_guard1930 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1232;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1242, eq33_e1242_d_n3, eq33_e1242_d_n9,) = {
    if (var_guard1931 != 0.0) {
        let eq33_e1236: f64 = (var_mult_inst * p.p32);
        let eq33_e1238: f64 = (eq33_e1236 * var_gwell);
        let eq33_e1240: f64 = (eq33_e1238 * (nv3 - nv9));
        (eq33_e1240, eq33_e1238, (-eq33_e1238),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1242;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1242_d_n3),
            9,
            multiplicity * (eq33_e1242_d_n9),
        );
        let (eq35_e1257,) = {
    if (var_guard1931 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1257;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq39_e1275: f64 = (-s.v[1995]);
        let eq39_e1277: f64 = (eq39_e1275 * s.v[1951]);
        let eq39_e1277_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][0]));
        let eq39_e1277_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][1]));
        let eq39_e1277_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][2]));
        let eq39_e1277_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][3]));
        let eq39_e1277_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][4]));
        let eq39_e1277_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][5]));
        let eq39_e1277_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][6]));
        let eq39_e1277_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][7]));
        let eq39_e1277_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][8]));
        let eq39_e1277_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][9]));
        let eq39_e1277_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][10]));
        let eq39_e1277_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][11]));
        let eq39_e1277_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][12]));
        let eq39_e1277_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][13]));
        let eq39_e1277_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][14]));
        let eq39_e1277_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][15]));
        let eq39_e1277_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][16]));
        let eq39_e1277_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][17]));
        let eq39_e1277_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][18]));
        let eq39_e1277_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][19]));
        let eq39_e1277_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1951]) + (eq39_e1275 * s.dn[1951][20]));
        let eq39_e1277_d_b0: f64 = (((-s.db[1995][0]) * s.v[1951]) + (eq39_e1275 * s.db[1951][0]));
        let eq39_e1277_d_b1: f64 = (((-s.db[1995][1]) * s.v[1951]) + (eq39_e1275 * s.db[1951][1]));
        let eq39_e1277_d_b2: f64 = (((-s.db[1995][2]) * s.v[1951]) + (eq39_e1275 * s.db[1951][2]));
        let eq39_e1277_d_b3: f64 = (((-s.db[1995][3]) * s.v[1951]) + (eq39_e1275 * s.db[1951][3]));
        let eq39_e1277_d_b4: f64 = (((-s.db[1995][4]) * s.v[1951]) + (eq39_e1275 * s.db[1951][4]));
        let eq39_e1277_d_b5: f64 = (((-s.db[1995][5]) * s.v[1951]) + (eq39_e1275 * s.db[1951][5]));
        let eq39_e1277_d_b6: f64 = (((-s.db[1995][6]) * s.v[1951]) + (eq39_e1275 * s.db[1951][6]));
        let eq39_e1277_d_b7: f64 = (((-s.db[1995][7]) * s.v[1951]) + (eq39_e1275 * s.db[1951][7]));
        let eq39_e1277_d_b8: f64 = (((-s.db[1995][8]) * s.v[1951]) + (eq39_e1275 * s.db[1951][8]));
        let eq39_e1277_d_b9: f64 = (((-s.db[1995][9]) * s.v[1951]) + (eq39_e1275 * s.db[1951][9]));
        let eq39_e1277_d_b10: f64 = (((-s.db[1995][10]) * s.v[1951]) + (eq39_e1275 * s.db[1951][10]));
        let eq39_e1277_d_b11: f64 = (((-s.db[1995][11]) * s.v[1951]) + (eq39_e1275 * s.db[1951][11]));
        let eq39_e1277_d_b12: f64 = (((-s.db[1995][12]) * s.v[1951]) + (eq39_e1275 * s.db[1951][12]));
        let eq39_e1277_d_b13: f64 = (((-s.db[1995][13]) * s.v[1951]) + (eq39_e1275 * s.db[1951][13]));
        let eq39_e1277_d_b14: f64 = (((-s.db[1995][14]) * s.v[1951]) + (eq39_e1275 * s.db[1951][14]));
        let eq39_e1277_d_b15: f64 = (((-s.db[1995][15]) * s.v[1951]) + (eq39_e1275 * s.db[1951][15]));
        let eq39_e1277_d_b16: f64 = (((-s.db[1995][16]) * s.v[1951]) + (eq39_e1275 * s.db[1951][16]));
        let eq39_e1277_d_b17: f64 = (((-s.db[1995][17]) * s.v[1951]) + (eq39_e1275 * s.db[1951][17]));
        let eq39_e1277_d_b18: f64 = (((-s.db[1995][18]) * s.v[1951]) + (eq39_e1275 * s.db[1951][18]));
        let eq39_e1277_d_b19: f64 = (((-s.db[1995][19]) * s.v[1951]) + (eq39_e1275 * s.db[1951][19]));
        let eq39_e1277_d_b20: f64 = (((-s.db[1995][20]) * s.v[1951]) + (eq39_e1275 * s.db[1951][20]));
        let eq39_e1277_d_b21: f64 = (((-s.db[1995][21]) * s.v[1951]) + (eq39_e1275 * s.db[1951][21]));
        let eq39_e1277_d_b22: f64 = (((-s.db[1995][22]) * s.v[1951]) + (eq39_e1275 * s.db[1951][22]));
        let eq39_e1277_d_b23: f64 = (((-s.db[1995][23]) * s.v[1951]) + (eq39_e1275 * s.db[1951][23]));
        let eq39_e1277_d_b24: f64 = (((-s.db[1995][24]) * s.v[1951]) + (eq39_e1275 * s.db[1951][24]));
        let eq39_e1279: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 0, eq39_e1277, s.v[1942]);
        let eq39_e1280: f64 = (s.v[4] * eq39_e1279);
        let eq39_e1280_d_n0: f64 = (s.v[4] * (eq39_e1277_d_n0 * idt_scale));
        let eq39_e1280_d_n1: f64 = (s.v[4] * (eq39_e1277_d_n1 * idt_scale));
        let eq39_e1280_d_n2: f64 = (s.v[4] * (eq39_e1277_d_n2 * idt_scale));
        let eq39_e1280_d_n3: f64 = (s.v[4] * (eq39_e1277_d_n3 * idt_scale));
        let eq39_e1280_d_n4: f64 = (s.v[4] * (eq39_e1277_d_n4 * idt_scale));
        let eq39_e1280_d_n5: f64 = (s.v[4] * (eq39_e1277_d_n5 * idt_scale));
        let eq39_e1280_d_n6: f64 = (s.v[4] * (eq39_e1277_d_n6 * idt_scale));
        let eq39_e1280_d_n7: f64 = (s.v[4] * (eq39_e1277_d_n7 * idt_scale));
        let eq39_e1280_d_n8: f64 = (s.v[4] * (eq39_e1277_d_n8 * idt_scale));
        let eq39_e1280_d_n9: f64 = (s.v[4] * (eq39_e1277_d_n9 * idt_scale));
        let eq39_e1280_d_n10: f64 = (s.v[4] * (eq39_e1277_d_n10 * idt_scale));
        let eq39_e1280_d_n11: f64 = (s.v[4] * (eq39_e1277_d_n11 * idt_scale));
        let eq39_e1280_d_n12: f64 = (s.v[4] * (eq39_e1277_d_n12 * idt_scale));
        let eq39_e1280_d_n13: f64 = (s.v[4] * (eq39_e1277_d_n13 * idt_scale));
        let eq39_e1280_d_n14: f64 = (s.v[4] * (eq39_e1277_d_n14 * idt_scale));
        let eq39_e1280_d_n15: f64 = (s.v[4] * (eq39_e1277_d_n15 * idt_scale));
        let eq39_e1280_d_n16: f64 = (s.v[4] * (eq39_e1277_d_n16 * idt_scale));
        let eq39_e1280_d_n17: f64 = (s.v[4] * (eq39_e1277_d_n17 * idt_scale));
        let eq39_e1280_d_n18: f64 = (s.v[4] * (eq39_e1277_d_n18 * idt_scale));
        let eq39_e1280_d_n19: f64 = (s.v[4] * (eq39_e1277_d_n19 * idt_scale));
        let eq39_e1280_d_n20: f64 = (s.v[4] * (eq39_e1277_d_n20 * idt_scale));
        let eq39_e1280_d_b0: f64 = (s.v[4] * (eq39_e1277_d_b0 * idt_scale));
        let eq39_e1280_d_b1: f64 = (s.v[4] * (eq39_e1277_d_b1 * idt_scale));
        let eq39_e1280_d_b2: f64 = (s.v[4] * (eq39_e1277_d_b2 * idt_scale));
        let eq39_e1280_d_b3: f64 = (s.v[4] * (eq39_e1277_d_b3 * idt_scale));
        let eq39_e1280_d_b4: f64 = (s.v[4] * (eq39_e1277_d_b4 * idt_scale));
        let eq39_e1280_d_b5: f64 = (s.v[4] * (eq39_e1277_d_b5 * idt_scale));
        let eq39_e1280_d_b6: f64 = (s.v[4] * (eq39_e1277_d_b6 * idt_scale));
        let eq39_e1280_d_b7: f64 = (s.v[4] * (eq39_e1277_d_b7 * idt_scale));
        let eq39_e1280_d_b8: f64 = (s.v[4] * (eq39_e1277_d_b8 * idt_scale));
        let eq39_e1280_d_b9: f64 = (s.v[4] * (eq39_e1277_d_b9 * idt_scale));
        let eq39_e1280_d_b10: f64 = (s.v[4] * (eq39_e1277_d_b10 * idt_scale));
        let eq39_e1280_d_b11: f64 = (s.v[4] * (eq39_e1277_d_b11 * idt_scale));
        let eq39_e1280_d_b12: f64 = (s.v[4] * (eq39_e1277_d_b12 * idt_scale));
        let eq39_e1280_d_b13: f64 = (s.v[4] * (eq39_e1277_d_b13 * idt_scale));
        let eq39_e1280_d_b14: f64 = (s.v[4] * (eq39_e1277_d_b14 * idt_scale));
        let eq39_e1280_d_b15: f64 = (s.v[4] * (eq39_e1277_d_b15 * idt_scale));
        let eq39_e1280_d_b16: f64 = (s.v[4] * (eq39_e1277_d_b16 * idt_scale));
        let eq39_e1280_d_b17: f64 = (s.v[4] * (eq39_e1277_d_b17 * idt_scale));
        let eq39_e1280_d_b18: f64 = (s.v[4] * (eq39_e1277_d_b18 * idt_scale));
        let eq39_e1280_d_b19: f64 = (s.v[4] * (eq39_e1277_d_b19 * idt_scale));
        let eq39_e1280_d_b20: f64 = (s.v[4] * (eq39_e1277_d_b20 * idt_scale));
        let eq39_e1280_d_b21: f64 = (s.v[4] * (eq39_e1277_d_b21 * idt_scale));
        let eq39_e1280_d_b22: f64 = (s.v[4] * (eq39_e1277_d_b22 * idt_scale));
        let eq39_e1280_d_b23: f64 = (s.v[4] * (eq39_e1277_d_b23 * idt_scale));
        let eq39_e1280_d_b24: f64 = (s.v[4] * (eq39_e1277_d_b24 * idt_scale));
        let eq39_value: f64 = eq39_e1280;
        let eq39_node_derivatives: [f64; 21] = [eq39_e1280_d_n0, eq39_e1280_d_n1, eq39_e1280_d_n2, eq39_e1280_d_n3, eq39_e1280_d_n4, eq39_e1280_d_n5, eq39_e1280_d_n6, eq39_e1280_d_n7, eq39_e1280_d_n8, eq39_e1280_d_n9, eq39_e1280_d_n10, eq39_e1280_d_n11, eq39_e1280_d_n12, eq39_e1280_d_n13, eq39_e1280_d_n14, eq39_e1280_d_n15, eq39_e1280_d_n16, eq39_e1280_d_n17, eq39_e1280_d_n18, eq39_e1280_d_n19, eq39_e1280_d_n20];
        let eq39_branch_derivatives: [f64; 25] = [eq39_e1280_d_b0, eq39_e1280_d_b1, eq39_e1280_d_b2, eq39_e1280_d_b3, eq39_e1280_d_b4, eq39_e1280_d_b5, eq39_e1280_d_b6, eq39_e1280_d_b7, eq39_e1280_d_b8, eq39_e1280_d_b9, eq39_e1280_d_b10, eq39_e1280_d_b11, eq39_e1280_d_b12, eq39_e1280_d_b13, eq39_e1280_d_b14, eq39_e1280_d_b15, eq39_e1280_d_b16, eq39_e1280_d_b17, eq39_e1280_d_b18, eq39_e1280_d_b19, eq39_e1280_d_b20, eq39_e1280_d_b21, eq39_e1280_d_b22, eq39_e1280_d_b23, eq39_e1280_d_b24];
        stamper.stamp_potential_dense_local(
            8,
            eq39_value,
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
        );
        let eq41_e1288: f64 = (-s.v[1995]);
        let eq41_e1290: f64 = (eq41_e1288 * s.v[1952]);
        let eq41_e1290_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][0]));
        let eq41_e1290_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][1]));
        let eq41_e1290_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][2]));
        let eq41_e1290_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][3]));
        let eq41_e1290_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][4]));
        let eq41_e1290_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][5]));
        let eq41_e1290_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][6]));
        let eq41_e1290_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][7]));
        let eq41_e1290_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][8]));
        let eq41_e1290_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][9]));
        let eq41_e1290_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][10]));
        let eq41_e1290_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][11]));
        let eq41_e1290_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][12]));
        let eq41_e1290_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][13]));
        let eq41_e1290_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][14]));
        let eq41_e1290_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][15]));
        let eq41_e1290_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][16]));
        let eq41_e1290_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][17]));
        let eq41_e1290_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][18]));
        let eq41_e1290_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][19]));
        let eq41_e1290_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1952]) + (eq41_e1288 * s.dn[1952][20]));
        let eq41_e1290_d_b0: f64 = (((-s.db[1995][0]) * s.v[1952]) + (eq41_e1288 * s.db[1952][0]));
        let eq41_e1290_d_b1: f64 = (((-s.db[1995][1]) * s.v[1952]) + (eq41_e1288 * s.db[1952][1]));
        let eq41_e1290_d_b2: f64 = (((-s.db[1995][2]) * s.v[1952]) + (eq41_e1288 * s.db[1952][2]));
        let eq41_e1290_d_b3: f64 = (((-s.db[1995][3]) * s.v[1952]) + (eq41_e1288 * s.db[1952][3]));
        let eq41_e1290_d_b4: f64 = (((-s.db[1995][4]) * s.v[1952]) + (eq41_e1288 * s.db[1952][4]));
        let eq41_e1290_d_b5: f64 = (((-s.db[1995][5]) * s.v[1952]) + (eq41_e1288 * s.db[1952][5]));
        let eq41_e1290_d_b6: f64 = (((-s.db[1995][6]) * s.v[1952]) + (eq41_e1288 * s.db[1952][6]));
        let eq41_e1290_d_b7: f64 = (((-s.db[1995][7]) * s.v[1952]) + (eq41_e1288 * s.db[1952][7]));
        let eq41_e1290_d_b8: f64 = (((-s.db[1995][8]) * s.v[1952]) + (eq41_e1288 * s.db[1952][8]));
        let eq41_e1290_d_b9: f64 = (((-s.db[1995][9]) * s.v[1952]) + (eq41_e1288 * s.db[1952][9]));
        let eq41_e1290_d_b10: f64 = (((-s.db[1995][10]) * s.v[1952]) + (eq41_e1288 * s.db[1952][10]));
        let eq41_e1290_d_b11: f64 = (((-s.db[1995][11]) * s.v[1952]) + (eq41_e1288 * s.db[1952][11]));
        let eq41_e1290_d_b12: f64 = (((-s.db[1995][12]) * s.v[1952]) + (eq41_e1288 * s.db[1952][12]));
        let eq41_e1290_d_b13: f64 = (((-s.db[1995][13]) * s.v[1952]) + (eq41_e1288 * s.db[1952][13]));
        let eq41_e1290_d_b14: f64 = (((-s.db[1995][14]) * s.v[1952]) + (eq41_e1288 * s.db[1952][14]));
        let eq41_e1290_d_b15: f64 = (((-s.db[1995][15]) * s.v[1952]) + (eq41_e1288 * s.db[1952][15]));
        let eq41_e1290_d_b16: f64 = (((-s.db[1995][16]) * s.v[1952]) + (eq41_e1288 * s.db[1952][16]));
        let eq41_e1290_d_b17: f64 = (((-s.db[1995][17]) * s.v[1952]) + (eq41_e1288 * s.db[1952][17]));
        let eq41_e1290_d_b18: f64 = (((-s.db[1995][18]) * s.v[1952]) + (eq41_e1288 * s.db[1952][18]));
        let eq41_e1290_d_b19: f64 = (((-s.db[1995][19]) * s.v[1952]) + (eq41_e1288 * s.db[1952][19]));
        let eq41_e1290_d_b20: f64 = (((-s.db[1995][20]) * s.v[1952]) + (eq41_e1288 * s.db[1952][20]));
        let eq41_e1290_d_b21: f64 = (((-s.db[1995][21]) * s.v[1952]) + (eq41_e1288 * s.db[1952][21]));
        let eq41_e1290_d_b22: f64 = (((-s.db[1995][22]) * s.v[1952]) + (eq41_e1288 * s.db[1952][22]));
        let eq41_e1290_d_b23: f64 = (((-s.db[1995][23]) * s.v[1952]) + (eq41_e1288 * s.db[1952][23]));
        let eq41_e1290_d_b24: f64 = (((-s.db[1995][24]) * s.v[1952]) + (eq41_e1288 * s.db[1952][24]));
        let eq41_e1292: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 1, eq41_e1290, s.v[1943]);
        let eq41_e1293: f64 = (s.v[4] * eq41_e1292);
        let eq41_e1293_d_n0: f64 = (s.v[4] * (eq41_e1290_d_n0 * idt_scale));
        let eq41_e1293_d_n1: f64 = (s.v[4] * (eq41_e1290_d_n1 * idt_scale));
        let eq41_e1293_d_n2: f64 = (s.v[4] * (eq41_e1290_d_n2 * idt_scale));
        let eq41_e1293_d_n3: f64 = (s.v[4] * (eq41_e1290_d_n3 * idt_scale));
        let eq41_e1293_d_n4: f64 = (s.v[4] * (eq41_e1290_d_n4 * idt_scale));
        let eq41_e1293_d_n5: f64 = (s.v[4] * (eq41_e1290_d_n5 * idt_scale));
        let eq41_e1293_d_n6: f64 = (s.v[4] * (eq41_e1290_d_n6 * idt_scale));
        let eq41_e1293_d_n7: f64 = (s.v[4] * (eq41_e1290_d_n7 * idt_scale));
        let eq41_e1293_d_n8: f64 = (s.v[4] * (eq41_e1290_d_n8 * idt_scale));
        let eq41_e1293_d_n9: f64 = (s.v[4] * (eq41_e1290_d_n9 * idt_scale));
        let eq41_e1293_d_n10: f64 = (s.v[4] * (eq41_e1290_d_n10 * idt_scale));
        let eq41_e1293_d_n11: f64 = (s.v[4] * (eq41_e1290_d_n11 * idt_scale));
        let eq41_e1293_d_n12: f64 = (s.v[4] * (eq41_e1290_d_n12 * idt_scale));
        let eq41_e1293_d_n13: f64 = (s.v[4] * (eq41_e1290_d_n13 * idt_scale));
        let eq41_e1293_d_n14: f64 = (s.v[4] * (eq41_e1290_d_n14 * idt_scale));
        let eq41_e1293_d_n15: f64 = (s.v[4] * (eq41_e1290_d_n15 * idt_scale));
        let eq41_e1293_d_n16: f64 = (s.v[4] * (eq41_e1290_d_n16 * idt_scale));
        let eq41_e1293_d_n17: f64 = (s.v[4] * (eq41_e1290_d_n17 * idt_scale));
        let eq41_e1293_d_n18: f64 = (s.v[4] * (eq41_e1290_d_n18 * idt_scale));
        let eq41_e1293_d_n19: f64 = (s.v[4] * (eq41_e1290_d_n19 * idt_scale));
        let eq41_e1293_d_n20: f64 = (s.v[4] * (eq41_e1290_d_n20 * idt_scale));
        let eq41_e1293_d_b0: f64 = (s.v[4] * (eq41_e1290_d_b0 * idt_scale));
        let eq41_e1293_d_b1: f64 = (s.v[4] * (eq41_e1290_d_b1 * idt_scale));
        let eq41_e1293_d_b2: f64 = (s.v[4] * (eq41_e1290_d_b2 * idt_scale));
        let eq41_e1293_d_b3: f64 = (s.v[4] * (eq41_e1290_d_b3 * idt_scale));
        let eq41_e1293_d_b4: f64 = (s.v[4] * (eq41_e1290_d_b4 * idt_scale));
        let eq41_e1293_d_b5: f64 = (s.v[4] * (eq41_e1290_d_b5 * idt_scale));
        let eq41_e1293_d_b6: f64 = (s.v[4] * (eq41_e1290_d_b6 * idt_scale));
        let eq41_e1293_d_b7: f64 = (s.v[4] * (eq41_e1290_d_b7 * idt_scale));
        let eq41_e1293_d_b8: f64 = (s.v[4] * (eq41_e1290_d_b8 * idt_scale));
        let eq41_e1293_d_b9: f64 = (s.v[4] * (eq41_e1290_d_b9 * idt_scale));
        let eq41_e1293_d_b10: f64 = (s.v[4] * (eq41_e1290_d_b10 * idt_scale));
        let eq41_e1293_d_b11: f64 = (s.v[4] * (eq41_e1290_d_b11 * idt_scale));
        let eq41_e1293_d_b12: f64 = (s.v[4] * (eq41_e1290_d_b12 * idt_scale));
        let eq41_e1293_d_b13: f64 = (s.v[4] * (eq41_e1290_d_b13 * idt_scale));
        let eq41_e1293_d_b14: f64 = (s.v[4] * (eq41_e1290_d_b14 * idt_scale));
        let eq41_e1293_d_b15: f64 = (s.v[4] * (eq41_e1290_d_b15 * idt_scale));
        let eq41_e1293_d_b16: f64 = (s.v[4] * (eq41_e1290_d_b16 * idt_scale));
        let eq41_e1293_d_b17: f64 = (s.v[4] * (eq41_e1290_d_b17 * idt_scale));
        let eq41_e1293_d_b18: f64 = (s.v[4] * (eq41_e1290_d_b18 * idt_scale));
        let eq41_e1293_d_b19: f64 = (s.v[4] * (eq41_e1290_d_b19 * idt_scale));
        let eq41_e1293_d_b20: f64 = (s.v[4] * (eq41_e1290_d_b20 * idt_scale));
        let eq41_e1293_d_b21: f64 = (s.v[4] * (eq41_e1290_d_b21 * idt_scale));
        let eq41_e1293_d_b22: f64 = (s.v[4] * (eq41_e1290_d_b22 * idt_scale));
        let eq41_e1293_d_b23: f64 = (s.v[4] * (eq41_e1290_d_b23 * idt_scale));
        let eq41_e1293_d_b24: f64 = (s.v[4] * (eq41_e1290_d_b24 * idt_scale));
        let eq41_value: f64 = eq41_e1293;
        let eq41_node_derivatives: [f64; 21] = [eq41_e1293_d_n0, eq41_e1293_d_n1, eq41_e1293_d_n2, eq41_e1293_d_n3, eq41_e1293_d_n4, eq41_e1293_d_n5, eq41_e1293_d_n6, eq41_e1293_d_n7, eq41_e1293_d_n8, eq41_e1293_d_n9, eq41_e1293_d_n10, eq41_e1293_d_n11, eq41_e1293_d_n12, eq41_e1293_d_n13, eq41_e1293_d_n14, eq41_e1293_d_n15, eq41_e1293_d_n16, eq41_e1293_d_n17, eq41_e1293_d_n18, eq41_e1293_d_n19, eq41_e1293_d_n20];
        let eq41_branch_derivatives: [f64; 25] = [eq41_e1293_d_b0, eq41_e1293_d_b1, eq41_e1293_d_b2, eq41_e1293_d_b3, eq41_e1293_d_b4, eq41_e1293_d_b5, eq41_e1293_d_b6, eq41_e1293_d_b7, eq41_e1293_d_b8, eq41_e1293_d_b9, eq41_e1293_d_b10, eq41_e1293_d_b11, eq41_e1293_d_b12, eq41_e1293_d_b13, eq41_e1293_d_b14, eq41_e1293_d_b15, eq41_e1293_d_b16, eq41_e1293_d_b17, eq41_e1293_d_b18, eq41_e1293_d_b19, eq41_e1293_d_b20, eq41_e1293_d_b21, eq41_e1293_d_b22, eq41_e1293_d_b23, eq41_e1293_d_b24];
        stamper.stamp_potential_dense_local(
            10,
            eq41_value,
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
    ) {
        let eq43_e1301: f64 = (-s.v[1995]);
        let eq43_e1303: f64 = (eq43_e1301 * s.v[1953]);
        let eq43_e1303_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][0]));
        let eq43_e1303_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][1]));
        let eq43_e1303_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][2]));
        let eq43_e1303_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][3]));
        let eq43_e1303_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][4]));
        let eq43_e1303_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][5]));
        let eq43_e1303_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][6]));
        let eq43_e1303_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][7]));
        let eq43_e1303_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][8]));
        let eq43_e1303_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][9]));
        let eq43_e1303_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][10]));
        let eq43_e1303_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][11]));
        let eq43_e1303_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][12]));
        let eq43_e1303_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][13]));
        let eq43_e1303_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][14]));
        let eq43_e1303_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][15]));
        let eq43_e1303_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][16]));
        let eq43_e1303_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][17]));
        let eq43_e1303_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][18]));
        let eq43_e1303_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][19]));
        let eq43_e1303_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1953]) + (eq43_e1301 * s.dn[1953][20]));
        let eq43_e1303_d_b0: f64 = (((-s.db[1995][0]) * s.v[1953]) + (eq43_e1301 * s.db[1953][0]));
        let eq43_e1303_d_b1: f64 = (((-s.db[1995][1]) * s.v[1953]) + (eq43_e1301 * s.db[1953][1]));
        let eq43_e1303_d_b2: f64 = (((-s.db[1995][2]) * s.v[1953]) + (eq43_e1301 * s.db[1953][2]));
        let eq43_e1303_d_b3: f64 = (((-s.db[1995][3]) * s.v[1953]) + (eq43_e1301 * s.db[1953][3]));
        let eq43_e1303_d_b4: f64 = (((-s.db[1995][4]) * s.v[1953]) + (eq43_e1301 * s.db[1953][4]));
        let eq43_e1303_d_b5: f64 = (((-s.db[1995][5]) * s.v[1953]) + (eq43_e1301 * s.db[1953][5]));
        let eq43_e1303_d_b6: f64 = (((-s.db[1995][6]) * s.v[1953]) + (eq43_e1301 * s.db[1953][6]));
        let eq43_e1303_d_b7: f64 = (((-s.db[1995][7]) * s.v[1953]) + (eq43_e1301 * s.db[1953][7]));
        let eq43_e1303_d_b8: f64 = (((-s.db[1995][8]) * s.v[1953]) + (eq43_e1301 * s.db[1953][8]));
        let eq43_e1303_d_b9: f64 = (((-s.db[1995][9]) * s.v[1953]) + (eq43_e1301 * s.db[1953][9]));
        let eq43_e1303_d_b10: f64 = (((-s.db[1995][10]) * s.v[1953]) + (eq43_e1301 * s.db[1953][10]));
        let eq43_e1303_d_b11: f64 = (((-s.db[1995][11]) * s.v[1953]) + (eq43_e1301 * s.db[1953][11]));
        let eq43_e1303_d_b12: f64 = (((-s.db[1995][12]) * s.v[1953]) + (eq43_e1301 * s.db[1953][12]));
        let eq43_e1303_d_b13: f64 = (((-s.db[1995][13]) * s.v[1953]) + (eq43_e1301 * s.db[1953][13]));
        let eq43_e1303_d_b14: f64 = (((-s.db[1995][14]) * s.v[1953]) + (eq43_e1301 * s.db[1953][14]));
        let eq43_e1303_d_b15: f64 = (((-s.db[1995][15]) * s.v[1953]) + (eq43_e1301 * s.db[1953][15]));
        let eq43_e1303_d_b16: f64 = (((-s.db[1995][16]) * s.v[1953]) + (eq43_e1301 * s.db[1953][16]));
        let eq43_e1303_d_b17: f64 = (((-s.db[1995][17]) * s.v[1953]) + (eq43_e1301 * s.db[1953][17]));
        let eq43_e1303_d_b18: f64 = (((-s.db[1995][18]) * s.v[1953]) + (eq43_e1301 * s.db[1953][18]));
        let eq43_e1303_d_b19: f64 = (((-s.db[1995][19]) * s.v[1953]) + (eq43_e1301 * s.db[1953][19]));
        let eq43_e1303_d_b20: f64 = (((-s.db[1995][20]) * s.v[1953]) + (eq43_e1301 * s.db[1953][20]));
        let eq43_e1303_d_b21: f64 = (((-s.db[1995][21]) * s.v[1953]) + (eq43_e1301 * s.db[1953][21]));
        let eq43_e1303_d_b22: f64 = (((-s.db[1995][22]) * s.v[1953]) + (eq43_e1301 * s.db[1953][22]));
        let eq43_e1303_d_b23: f64 = (((-s.db[1995][23]) * s.v[1953]) + (eq43_e1301 * s.db[1953][23]));
        let eq43_e1303_d_b24: f64 = (((-s.db[1995][24]) * s.v[1953]) + (eq43_e1301 * s.db[1953][24]));
        let eq43_e1305: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 2, eq43_e1303, s.v[1944]);
        let eq43_e1306: f64 = (s.v[4] * eq43_e1305);
        let eq43_e1306_d_n0: f64 = (s.v[4] * (eq43_e1303_d_n0 * idt_scale));
        let eq43_e1306_d_n1: f64 = (s.v[4] * (eq43_e1303_d_n1 * idt_scale));
        let eq43_e1306_d_n2: f64 = (s.v[4] * (eq43_e1303_d_n2 * idt_scale));
        let eq43_e1306_d_n3: f64 = (s.v[4] * (eq43_e1303_d_n3 * idt_scale));
        let eq43_e1306_d_n4: f64 = (s.v[4] * (eq43_e1303_d_n4 * idt_scale));
        let eq43_e1306_d_n5: f64 = (s.v[4] * (eq43_e1303_d_n5 * idt_scale));
        let eq43_e1306_d_n6: f64 = (s.v[4] * (eq43_e1303_d_n6 * idt_scale));
        let eq43_e1306_d_n7: f64 = (s.v[4] * (eq43_e1303_d_n7 * idt_scale));
        let eq43_e1306_d_n8: f64 = (s.v[4] * (eq43_e1303_d_n8 * idt_scale));
        let eq43_e1306_d_n9: f64 = (s.v[4] * (eq43_e1303_d_n9 * idt_scale));
        let eq43_e1306_d_n10: f64 = (s.v[4] * (eq43_e1303_d_n10 * idt_scale));
        let eq43_e1306_d_n11: f64 = (s.v[4] * (eq43_e1303_d_n11 * idt_scale));
        let eq43_e1306_d_n12: f64 = (s.v[4] * (eq43_e1303_d_n12 * idt_scale));
        let eq43_e1306_d_n13: f64 = (s.v[4] * (eq43_e1303_d_n13 * idt_scale));
        let eq43_e1306_d_n14: f64 = (s.v[4] * (eq43_e1303_d_n14 * idt_scale));
        let eq43_e1306_d_n15: f64 = (s.v[4] * (eq43_e1303_d_n15 * idt_scale));
        let eq43_e1306_d_n16: f64 = (s.v[4] * (eq43_e1303_d_n16 * idt_scale));
        let eq43_e1306_d_n17: f64 = (s.v[4] * (eq43_e1303_d_n17 * idt_scale));
        let eq43_e1306_d_n18: f64 = (s.v[4] * (eq43_e1303_d_n18 * idt_scale));
        let eq43_e1306_d_n19: f64 = (s.v[4] * (eq43_e1303_d_n19 * idt_scale));
        let eq43_e1306_d_n20: f64 = (s.v[4] * (eq43_e1303_d_n20 * idt_scale));
        let eq43_e1306_d_b0: f64 = (s.v[4] * (eq43_e1303_d_b0 * idt_scale));
        let eq43_e1306_d_b1: f64 = (s.v[4] * (eq43_e1303_d_b1 * idt_scale));
        let eq43_e1306_d_b2: f64 = (s.v[4] * (eq43_e1303_d_b2 * idt_scale));
        let eq43_e1306_d_b3: f64 = (s.v[4] * (eq43_e1303_d_b3 * idt_scale));
        let eq43_e1306_d_b4: f64 = (s.v[4] * (eq43_e1303_d_b4 * idt_scale));
        let eq43_e1306_d_b5: f64 = (s.v[4] * (eq43_e1303_d_b5 * idt_scale));
        let eq43_e1306_d_b6: f64 = (s.v[4] * (eq43_e1303_d_b6 * idt_scale));
        let eq43_e1306_d_b7: f64 = (s.v[4] * (eq43_e1303_d_b7 * idt_scale));
        let eq43_e1306_d_b8: f64 = (s.v[4] * (eq43_e1303_d_b8 * idt_scale));
        let eq43_e1306_d_b9: f64 = (s.v[4] * (eq43_e1303_d_b9 * idt_scale));
        let eq43_e1306_d_b10: f64 = (s.v[4] * (eq43_e1303_d_b10 * idt_scale));
        let eq43_e1306_d_b11: f64 = (s.v[4] * (eq43_e1303_d_b11 * idt_scale));
        let eq43_e1306_d_b12: f64 = (s.v[4] * (eq43_e1303_d_b12 * idt_scale));
        let eq43_e1306_d_b13: f64 = (s.v[4] * (eq43_e1303_d_b13 * idt_scale));
        let eq43_e1306_d_b14: f64 = (s.v[4] * (eq43_e1303_d_b14 * idt_scale));
        let eq43_e1306_d_b15: f64 = (s.v[4] * (eq43_e1303_d_b15 * idt_scale));
        let eq43_e1306_d_b16: f64 = (s.v[4] * (eq43_e1303_d_b16 * idt_scale));
        let eq43_e1306_d_b17: f64 = (s.v[4] * (eq43_e1303_d_b17 * idt_scale));
        let eq43_e1306_d_b18: f64 = (s.v[4] * (eq43_e1303_d_b18 * idt_scale));
        let eq43_e1306_d_b19: f64 = (s.v[4] * (eq43_e1303_d_b19 * idt_scale));
        let eq43_e1306_d_b20: f64 = (s.v[4] * (eq43_e1303_d_b20 * idt_scale));
        let eq43_e1306_d_b21: f64 = (s.v[4] * (eq43_e1303_d_b21 * idt_scale));
        let eq43_e1306_d_b22: f64 = (s.v[4] * (eq43_e1303_d_b22 * idt_scale));
        let eq43_e1306_d_b23: f64 = (s.v[4] * (eq43_e1303_d_b23 * idt_scale));
        let eq43_e1306_d_b24: f64 = (s.v[4] * (eq43_e1303_d_b24 * idt_scale));
        let eq43_value: f64 = eq43_e1306;
        let eq43_node_derivatives: [f64; 21] = [eq43_e1306_d_n0, eq43_e1306_d_n1, eq43_e1306_d_n2, eq43_e1306_d_n3, eq43_e1306_d_n4, eq43_e1306_d_n5, eq43_e1306_d_n6, eq43_e1306_d_n7, eq43_e1306_d_n8, eq43_e1306_d_n9, eq43_e1306_d_n10, eq43_e1306_d_n11, eq43_e1306_d_n12, eq43_e1306_d_n13, eq43_e1306_d_n14, eq43_e1306_d_n15, eq43_e1306_d_n16, eq43_e1306_d_n17, eq43_e1306_d_n18, eq43_e1306_d_n19, eq43_e1306_d_n20];
        let eq43_branch_derivatives: [f64; 25] = [eq43_e1306_d_b0, eq43_e1306_d_b1, eq43_e1306_d_b2, eq43_e1306_d_b3, eq43_e1306_d_b4, eq43_e1306_d_b5, eq43_e1306_d_b6, eq43_e1306_d_b7, eq43_e1306_d_b8, eq43_e1306_d_b9, eq43_e1306_d_b10, eq43_e1306_d_b11, eq43_e1306_d_b12, eq43_e1306_d_b13, eq43_e1306_d_b14, eq43_e1306_d_b15, eq43_e1306_d_b16, eq43_e1306_d_b17, eq43_e1306_d_b18, eq43_e1306_d_b19, eq43_e1306_d_b20, eq43_e1306_d_b21, eq43_e1306_d_b22, eq43_e1306_d_b23, eq43_e1306_d_b24];
        stamper.stamp_potential_dense_local(
            12,
            eq43_value,
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
        );
        let eq45_e1314: f64 = (-s.v[1995]);
        let eq45_e1316: f64 = (eq45_e1314 * s.v[1954]);
        let eq45_e1316_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][0]));
        let eq45_e1316_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][1]));
        let eq45_e1316_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][2]));
        let eq45_e1316_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][3]));
        let eq45_e1316_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][4]));
        let eq45_e1316_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][5]));
        let eq45_e1316_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][6]));
        let eq45_e1316_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][7]));
        let eq45_e1316_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][8]));
        let eq45_e1316_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][9]));
        let eq45_e1316_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][10]));
        let eq45_e1316_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][11]));
        let eq45_e1316_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][12]));
        let eq45_e1316_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][13]));
        let eq45_e1316_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][14]));
        let eq45_e1316_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][15]));
        let eq45_e1316_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][16]));
        let eq45_e1316_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][17]));
        let eq45_e1316_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][18]));
        let eq45_e1316_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][19]));
        let eq45_e1316_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1954]) + (eq45_e1314 * s.dn[1954][20]));
        let eq45_e1316_d_b0: f64 = (((-s.db[1995][0]) * s.v[1954]) + (eq45_e1314 * s.db[1954][0]));
        let eq45_e1316_d_b1: f64 = (((-s.db[1995][1]) * s.v[1954]) + (eq45_e1314 * s.db[1954][1]));
        let eq45_e1316_d_b2: f64 = (((-s.db[1995][2]) * s.v[1954]) + (eq45_e1314 * s.db[1954][2]));
        let eq45_e1316_d_b3: f64 = (((-s.db[1995][3]) * s.v[1954]) + (eq45_e1314 * s.db[1954][3]));
        let eq45_e1316_d_b4: f64 = (((-s.db[1995][4]) * s.v[1954]) + (eq45_e1314 * s.db[1954][4]));
        let eq45_e1316_d_b5: f64 = (((-s.db[1995][5]) * s.v[1954]) + (eq45_e1314 * s.db[1954][5]));
        let eq45_e1316_d_b6: f64 = (((-s.db[1995][6]) * s.v[1954]) + (eq45_e1314 * s.db[1954][6]));
        let eq45_e1316_d_b7: f64 = (((-s.db[1995][7]) * s.v[1954]) + (eq45_e1314 * s.db[1954][7]));
        let eq45_e1316_d_b8: f64 = (((-s.db[1995][8]) * s.v[1954]) + (eq45_e1314 * s.db[1954][8]));
        let eq45_e1316_d_b9: f64 = (((-s.db[1995][9]) * s.v[1954]) + (eq45_e1314 * s.db[1954][9]));
        let eq45_e1316_d_b10: f64 = (((-s.db[1995][10]) * s.v[1954]) + (eq45_e1314 * s.db[1954][10]));
        let eq45_e1316_d_b11: f64 = (((-s.db[1995][11]) * s.v[1954]) + (eq45_e1314 * s.db[1954][11]));
        let eq45_e1316_d_b12: f64 = (((-s.db[1995][12]) * s.v[1954]) + (eq45_e1314 * s.db[1954][12]));
        let eq45_e1316_d_b13: f64 = (((-s.db[1995][13]) * s.v[1954]) + (eq45_e1314 * s.db[1954][13]));
        let eq45_e1316_d_b14: f64 = (((-s.db[1995][14]) * s.v[1954]) + (eq45_e1314 * s.db[1954][14]));
        let eq45_e1316_d_b15: f64 = (((-s.db[1995][15]) * s.v[1954]) + (eq45_e1314 * s.db[1954][15]));
        let eq45_e1316_d_b16: f64 = (((-s.db[1995][16]) * s.v[1954]) + (eq45_e1314 * s.db[1954][16]));
        let eq45_e1316_d_b17: f64 = (((-s.db[1995][17]) * s.v[1954]) + (eq45_e1314 * s.db[1954][17]));
        let eq45_e1316_d_b18: f64 = (((-s.db[1995][18]) * s.v[1954]) + (eq45_e1314 * s.db[1954][18]));
        let eq45_e1316_d_b19: f64 = (((-s.db[1995][19]) * s.v[1954]) + (eq45_e1314 * s.db[1954][19]));
        let eq45_e1316_d_b20: f64 = (((-s.db[1995][20]) * s.v[1954]) + (eq45_e1314 * s.db[1954][20]));
        let eq45_e1316_d_b21: f64 = (((-s.db[1995][21]) * s.v[1954]) + (eq45_e1314 * s.db[1954][21]));
        let eq45_e1316_d_b22: f64 = (((-s.db[1995][22]) * s.v[1954]) + (eq45_e1314 * s.db[1954][22]));
        let eq45_e1316_d_b23: f64 = (((-s.db[1995][23]) * s.v[1954]) + (eq45_e1314 * s.db[1954][23]));
        let eq45_e1316_d_b24: f64 = (((-s.db[1995][24]) * s.v[1954]) + (eq45_e1314 * s.db[1954][24]));
        let eq45_e1318: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 3, eq45_e1316, s.v[1945]);
        let eq45_e1319: f64 = (s.v[4] * eq45_e1318);
        let eq45_e1319_d_n0: f64 = (s.v[4] * (eq45_e1316_d_n0 * idt_scale));
        let eq45_e1319_d_n1: f64 = (s.v[4] * (eq45_e1316_d_n1 * idt_scale));
        let eq45_e1319_d_n2: f64 = (s.v[4] * (eq45_e1316_d_n2 * idt_scale));
        let eq45_e1319_d_n3: f64 = (s.v[4] * (eq45_e1316_d_n3 * idt_scale));
        let eq45_e1319_d_n4: f64 = (s.v[4] * (eq45_e1316_d_n4 * idt_scale));
        let eq45_e1319_d_n5: f64 = (s.v[4] * (eq45_e1316_d_n5 * idt_scale));
        let eq45_e1319_d_n6: f64 = (s.v[4] * (eq45_e1316_d_n6 * idt_scale));
        let eq45_e1319_d_n7: f64 = (s.v[4] * (eq45_e1316_d_n7 * idt_scale));
        let eq45_e1319_d_n8: f64 = (s.v[4] * (eq45_e1316_d_n8 * idt_scale));
        let eq45_e1319_d_n9: f64 = (s.v[4] * (eq45_e1316_d_n9 * idt_scale));
        let eq45_e1319_d_n10: f64 = (s.v[4] * (eq45_e1316_d_n10 * idt_scale));
        let eq45_e1319_d_n11: f64 = (s.v[4] * (eq45_e1316_d_n11 * idt_scale));
        let eq45_e1319_d_n12: f64 = (s.v[4] * (eq45_e1316_d_n12 * idt_scale));
        let eq45_e1319_d_n13: f64 = (s.v[4] * (eq45_e1316_d_n13 * idt_scale));
        let eq45_e1319_d_n14: f64 = (s.v[4] * (eq45_e1316_d_n14 * idt_scale));
        let eq45_e1319_d_n15: f64 = (s.v[4] * (eq45_e1316_d_n15 * idt_scale));
        let eq45_e1319_d_n16: f64 = (s.v[4] * (eq45_e1316_d_n16 * idt_scale));
        let eq45_e1319_d_n17: f64 = (s.v[4] * (eq45_e1316_d_n17 * idt_scale));
        let eq45_e1319_d_n18: f64 = (s.v[4] * (eq45_e1316_d_n18 * idt_scale));
        let eq45_e1319_d_n19: f64 = (s.v[4] * (eq45_e1316_d_n19 * idt_scale));
        let eq45_e1319_d_n20: f64 = (s.v[4] * (eq45_e1316_d_n20 * idt_scale));
        let eq45_e1319_d_b0: f64 = (s.v[4] * (eq45_e1316_d_b0 * idt_scale));
        let eq45_e1319_d_b1: f64 = (s.v[4] * (eq45_e1316_d_b1 * idt_scale));
        let eq45_e1319_d_b2: f64 = (s.v[4] * (eq45_e1316_d_b2 * idt_scale));
        let eq45_e1319_d_b3: f64 = (s.v[4] * (eq45_e1316_d_b3 * idt_scale));
        let eq45_e1319_d_b4: f64 = (s.v[4] * (eq45_e1316_d_b4 * idt_scale));
        let eq45_e1319_d_b5: f64 = (s.v[4] * (eq45_e1316_d_b5 * idt_scale));
        let eq45_e1319_d_b6: f64 = (s.v[4] * (eq45_e1316_d_b6 * idt_scale));
        let eq45_e1319_d_b7: f64 = (s.v[4] * (eq45_e1316_d_b7 * idt_scale));
        let eq45_e1319_d_b8: f64 = (s.v[4] * (eq45_e1316_d_b8 * idt_scale));
        let eq45_e1319_d_b9: f64 = (s.v[4] * (eq45_e1316_d_b9 * idt_scale));
        let eq45_e1319_d_b10: f64 = (s.v[4] * (eq45_e1316_d_b10 * idt_scale));
        let eq45_e1319_d_b11: f64 = (s.v[4] * (eq45_e1316_d_b11 * idt_scale));
        let eq45_e1319_d_b12: f64 = (s.v[4] * (eq45_e1316_d_b12 * idt_scale));
        let eq45_e1319_d_b13: f64 = (s.v[4] * (eq45_e1316_d_b13 * idt_scale));
        let eq45_e1319_d_b14: f64 = (s.v[4] * (eq45_e1316_d_b14 * idt_scale));
        let eq45_e1319_d_b15: f64 = (s.v[4] * (eq45_e1316_d_b15 * idt_scale));
        let eq45_e1319_d_b16: f64 = (s.v[4] * (eq45_e1316_d_b16 * idt_scale));
        let eq45_e1319_d_b17: f64 = (s.v[4] * (eq45_e1316_d_b17 * idt_scale));
        let eq45_e1319_d_b18: f64 = (s.v[4] * (eq45_e1316_d_b18 * idt_scale));
        let eq45_e1319_d_b19: f64 = (s.v[4] * (eq45_e1316_d_b19 * idt_scale));
        let eq45_e1319_d_b20: f64 = (s.v[4] * (eq45_e1316_d_b20 * idt_scale));
        let eq45_e1319_d_b21: f64 = (s.v[4] * (eq45_e1316_d_b21 * idt_scale));
        let eq45_e1319_d_b22: f64 = (s.v[4] * (eq45_e1316_d_b22 * idt_scale));
        let eq45_e1319_d_b23: f64 = (s.v[4] * (eq45_e1316_d_b23 * idt_scale));
        let eq45_e1319_d_b24: f64 = (s.v[4] * (eq45_e1316_d_b24 * idt_scale));
        let eq45_value: f64 = eq45_e1319;
        let eq45_node_derivatives: [f64; 21] = [eq45_e1319_d_n0, eq45_e1319_d_n1, eq45_e1319_d_n2, eq45_e1319_d_n3, eq45_e1319_d_n4, eq45_e1319_d_n5, eq45_e1319_d_n6, eq45_e1319_d_n7, eq45_e1319_d_n8, eq45_e1319_d_n9, eq45_e1319_d_n10, eq45_e1319_d_n11, eq45_e1319_d_n12, eq45_e1319_d_n13, eq45_e1319_d_n14, eq45_e1319_d_n15, eq45_e1319_d_n16, eq45_e1319_d_n17, eq45_e1319_d_n18, eq45_e1319_d_n19, eq45_e1319_d_n20];
        let eq45_branch_derivatives: [f64; 25] = [eq45_e1319_d_b0, eq45_e1319_d_b1, eq45_e1319_d_b2, eq45_e1319_d_b3, eq45_e1319_d_b4, eq45_e1319_d_b5, eq45_e1319_d_b6, eq45_e1319_d_b7, eq45_e1319_d_b8, eq45_e1319_d_b9, eq45_e1319_d_b10, eq45_e1319_d_b11, eq45_e1319_d_b12, eq45_e1319_d_b13, eq45_e1319_d_b14, eq45_e1319_d_b15, eq45_e1319_d_b16, eq45_e1319_d_b17, eq45_e1319_d_b18, eq45_e1319_d_b19, eq45_e1319_d_b20, eq45_e1319_d_b21, eq45_e1319_d_b22, eq45_e1319_d_b23, eq45_e1319_d_b24];
        stamper.stamp_potential_dense_local(
            14,
            eq45_value,
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
        );
        let eq47_e1327: f64 = (-s.v[1995]);
        let eq47_e1329: f64 = (eq47_e1327 * s.v[1955]);
        let eq47_e1329_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][0]));
        let eq47_e1329_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][1]));
        let eq47_e1329_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][2]));
        let eq47_e1329_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][3]));
        let eq47_e1329_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][4]));
        let eq47_e1329_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][5]));
        let eq47_e1329_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][6]));
        let eq47_e1329_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][7]));
        let eq47_e1329_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][8]));
        let eq47_e1329_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][9]));
        let eq47_e1329_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][10]));
        let eq47_e1329_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][11]));
        let eq47_e1329_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][12]));
        let eq47_e1329_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][13]));
        let eq47_e1329_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][14]));
        let eq47_e1329_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][15]));
        let eq47_e1329_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][16]));
        let eq47_e1329_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][17]));
        let eq47_e1329_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][18]));
        let eq47_e1329_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][19]));
        let eq47_e1329_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1955]) + (eq47_e1327 * s.dn[1955][20]));
        let eq47_e1329_d_b0: f64 = (((-s.db[1995][0]) * s.v[1955]) + (eq47_e1327 * s.db[1955][0]));
        let eq47_e1329_d_b1: f64 = (((-s.db[1995][1]) * s.v[1955]) + (eq47_e1327 * s.db[1955][1]));
        let eq47_e1329_d_b2: f64 = (((-s.db[1995][2]) * s.v[1955]) + (eq47_e1327 * s.db[1955][2]));
        let eq47_e1329_d_b3: f64 = (((-s.db[1995][3]) * s.v[1955]) + (eq47_e1327 * s.db[1955][3]));
        let eq47_e1329_d_b4: f64 = (((-s.db[1995][4]) * s.v[1955]) + (eq47_e1327 * s.db[1955][4]));
        let eq47_e1329_d_b5: f64 = (((-s.db[1995][5]) * s.v[1955]) + (eq47_e1327 * s.db[1955][5]));
        let eq47_e1329_d_b6: f64 = (((-s.db[1995][6]) * s.v[1955]) + (eq47_e1327 * s.db[1955][6]));
        let eq47_e1329_d_b7: f64 = (((-s.db[1995][7]) * s.v[1955]) + (eq47_e1327 * s.db[1955][7]));
        let eq47_e1329_d_b8: f64 = (((-s.db[1995][8]) * s.v[1955]) + (eq47_e1327 * s.db[1955][8]));
        let eq47_e1329_d_b9: f64 = (((-s.db[1995][9]) * s.v[1955]) + (eq47_e1327 * s.db[1955][9]));
        let eq47_e1329_d_b10: f64 = (((-s.db[1995][10]) * s.v[1955]) + (eq47_e1327 * s.db[1955][10]));
        let eq47_e1329_d_b11: f64 = (((-s.db[1995][11]) * s.v[1955]) + (eq47_e1327 * s.db[1955][11]));
        let eq47_e1329_d_b12: f64 = (((-s.db[1995][12]) * s.v[1955]) + (eq47_e1327 * s.db[1955][12]));
        let eq47_e1329_d_b13: f64 = (((-s.db[1995][13]) * s.v[1955]) + (eq47_e1327 * s.db[1955][13]));
        let eq47_e1329_d_b14: f64 = (((-s.db[1995][14]) * s.v[1955]) + (eq47_e1327 * s.db[1955][14]));
        let eq47_e1329_d_b15: f64 = (((-s.db[1995][15]) * s.v[1955]) + (eq47_e1327 * s.db[1955][15]));
        let eq47_e1329_d_b16: f64 = (((-s.db[1995][16]) * s.v[1955]) + (eq47_e1327 * s.db[1955][16]));
        let eq47_e1329_d_b17: f64 = (((-s.db[1995][17]) * s.v[1955]) + (eq47_e1327 * s.db[1955][17]));
        let eq47_e1329_d_b18: f64 = (((-s.db[1995][18]) * s.v[1955]) + (eq47_e1327 * s.db[1955][18]));
        let eq47_e1329_d_b19: f64 = (((-s.db[1995][19]) * s.v[1955]) + (eq47_e1327 * s.db[1955][19]));
        let eq47_e1329_d_b20: f64 = (((-s.db[1995][20]) * s.v[1955]) + (eq47_e1327 * s.db[1955][20]));
        let eq47_e1329_d_b21: f64 = (((-s.db[1995][21]) * s.v[1955]) + (eq47_e1327 * s.db[1955][21]));
        let eq47_e1329_d_b22: f64 = (((-s.db[1995][22]) * s.v[1955]) + (eq47_e1327 * s.db[1955][22]));
        let eq47_e1329_d_b23: f64 = (((-s.db[1995][23]) * s.v[1955]) + (eq47_e1327 * s.db[1955][23]));
        let eq47_e1329_d_b24: f64 = (((-s.db[1995][24]) * s.v[1955]) + (eq47_e1327 * s.db[1955][24]));
        let eq47_e1331: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 4, eq47_e1329, s.v[1946]);
        let eq47_e1332: f64 = (s.v[4] * eq47_e1331);
        let eq47_e1332_d_n0: f64 = (s.v[4] * (eq47_e1329_d_n0 * idt_scale));
        let eq47_e1332_d_n1: f64 = (s.v[4] * (eq47_e1329_d_n1 * idt_scale));
        let eq47_e1332_d_n2: f64 = (s.v[4] * (eq47_e1329_d_n2 * idt_scale));
        let eq47_e1332_d_n3: f64 = (s.v[4] * (eq47_e1329_d_n3 * idt_scale));
        let eq47_e1332_d_n4: f64 = (s.v[4] * (eq47_e1329_d_n4 * idt_scale));
        let eq47_e1332_d_n5: f64 = (s.v[4] * (eq47_e1329_d_n5 * idt_scale));
        let eq47_e1332_d_n6: f64 = (s.v[4] * (eq47_e1329_d_n6 * idt_scale));
        let eq47_e1332_d_n7: f64 = (s.v[4] * (eq47_e1329_d_n7 * idt_scale));
        let eq47_e1332_d_n8: f64 = (s.v[4] * (eq47_e1329_d_n8 * idt_scale));
        let eq47_e1332_d_n9: f64 = (s.v[4] * (eq47_e1329_d_n9 * idt_scale));
        let eq47_e1332_d_n10: f64 = (s.v[4] * (eq47_e1329_d_n10 * idt_scale));
        let eq47_e1332_d_n11: f64 = (s.v[4] * (eq47_e1329_d_n11 * idt_scale));
        let eq47_e1332_d_n12: f64 = (s.v[4] * (eq47_e1329_d_n12 * idt_scale));
        let eq47_e1332_d_n13: f64 = (s.v[4] * (eq47_e1329_d_n13 * idt_scale));
        let eq47_e1332_d_n14: f64 = (s.v[4] * (eq47_e1329_d_n14 * idt_scale));
        let eq47_e1332_d_n15: f64 = (s.v[4] * (eq47_e1329_d_n15 * idt_scale));
        let eq47_e1332_d_n16: f64 = (s.v[4] * (eq47_e1329_d_n16 * idt_scale));
        let eq47_e1332_d_n17: f64 = (s.v[4] * (eq47_e1329_d_n17 * idt_scale));
        let eq47_e1332_d_n18: f64 = (s.v[4] * (eq47_e1329_d_n18 * idt_scale));
        let eq47_e1332_d_n19: f64 = (s.v[4] * (eq47_e1329_d_n19 * idt_scale));
        let eq47_e1332_d_n20: f64 = (s.v[4] * (eq47_e1329_d_n20 * idt_scale));
        let eq47_e1332_d_b0: f64 = (s.v[4] * (eq47_e1329_d_b0 * idt_scale));
        let eq47_e1332_d_b1: f64 = (s.v[4] * (eq47_e1329_d_b1 * idt_scale));
        let eq47_e1332_d_b2: f64 = (s.v[4] * (eq47_e1329_d_b2 * idt_scale));
        let eq47_e1332_d_b3: f64 = (s.v[4] * (eq47_e1329_d_b3 * idt_scale));
        let eq47_e1332_d_b4: f64 = (s.v[4] * (eq47_e1329_d_b4 * idt_scale));
        let eq47_e1332_d_b5: f64 = (s.v[4] * (eq47_e1329_d_b5 * idt_scale));
        let eq47_e1332_d_b6: f64 = (s.v[4] * (eq47_e1329_d_b6 * idt_scale));
        let eq47_e1332_d_b7: f64 = (s.v[4] * (eq47_e1329_d_b7 * idt_scale));
        let eq47_e1332_d_b8: f64 = (s.v[4] * (eq47_e1329_d_b8 * idt_scale));
        let eq47_e1332_d_b9: f64 = (s.v[4] * (eq47_e1329_d_b9 * idt_scale));
        let eq47_e1332_d_b10: f64 = (s.v[4] * (eq47_e1329_d_b10 * idt_scale));
        let eq47_e1332_d_b11: f64 = (s.v[4] * (eq47_e1329_d_b11 * idt_scale));
        let eq47_e1332_d_b12: f64 = (s.v[4] * (eq47_e1329_d_b12 * idt_scale));
        let eq47_e1332_d_b13: f64 = (s.v[4] * (eq47_e1329_d_b13 * idt_scale));
        let eq47_e1332_d_b14: f64 = (s.v[4] * (eq47_e1329_d_b14 * idt_scale));
        let eq47_e1332_d_b15: f64 = (s.v[4] * (eq47_e1329_d_b15 * idt_scale));
        let eq47_e1332_d_b16: f64 = (s.v[4] * (eq47_e1329_d_b16 * idt_scale));
        let eq47_e1332_d_b17: f64 = (s.v[4] * (eq47_e1329_d_b17 * idt_scale));
        let eq47_e1332_d_b18: f64 = (s.v[4] * (eq47_e1329_d_b18 * idt_scale));
        let eq47_e1332_d_b19: f64 = (s.v[4] * (eq47_e1329_d_b19 * idt_scale));
        let eq47_e1332_d_b20: f64 = (s.v[4] * (eq47_e1329_d_b20 * idt_scale));
        let eq47_e1332_d_b21: f64 = (s.v[4] * (eq47_e1329_d_b21 * idt_scale));
        let eq47_e1332_d_b22: f64 = (s.v[4] * (eq47_e1329_d_b22 * idt_scale));
        let eq47_e1332_d_b23: f64 = (s.v[4] * (eq47_e1329_d_b23 * idt_scale));
        let eq47_e1332_d_b24: f64 = (s.v[4] * (eq47_e1329_d_b24 * idt_scale));
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivatives: [f64; 21] = [eq47_e1332_d_n0, eq47_e1332_d_n1, eq47_e1332_d_n2, eq47_e1332_d_n3, eq47_e1332_d_n4, eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n9, eq47_e1332_d_n10, eq47_e1332_d_n11, eq47_e1332_d_n12, eq47_e1332_d_n13, eq47_e1332_d_n14, eq47_e1332_d_n15, eq47_e1332_d_n16, eq47_e1332_d_n17, eq47_e1332_d_n18, eq47_e1332_d_n19, eq47_e1332_d_n20];
        let eq47_branch_derivatives: [f64; 25] = [eq47_e1332_d_b0, eq47_e1332_d_b1, eq47_e1332_d_b2, eq47_e1332_d_b3, eq47_e1332_d_b4, eq47_e1332_d_b5, eq47_e1332_d_b6, eq47_e1332_d_b7, eq47_e1332_d_b8, eq47_e1332_d_b9, eq47_e1332_d_b10, eq47_e1332_d_b11, eq47_e1332_d_b12, eq47_e1332_d_b13, eq47_e1332_d_b14, eq47_e1332_d_b15, eq47_e1332_d_b16, eq47_e1332_d_b17, eq47_e1332_d_b18, eq47_e1332_d_b19, eq47_e1332_d_b20, eq47_e1332_d_b21, eq47_e1332_d_b22, eq47_e1332_d_b23, eq47_e1332_d_b24];
        stamper.stamp_potential_dense_local(
            16,
            eq47_value,
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
        );
        let eq49_e1340: f64 = (-s.v[1995]);
        let eq49_e1342: f64 = (eq49_e1340 * s.v[1956]);
        let eq49_e1342_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][0]));
        let eq49_e1342_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][1]));
        let eq49_e1342_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][2]));
        let eq49_e1342_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][3]));
        let eq49_e1342_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][4]));
        let eq49_e1342_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][5]));
        let eq49_e1342_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][6]));
        let eq49_e1342_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][7]));
        let eq49_e1342_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][8]));
        let eq49_e1342_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][9]));
        let eq49_e1342_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][10]));
        let eq49_e1342_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][11]));
        let eq49_e1342_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][12]));
        let eq49_e1342_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][13]));
        let eq49_e1342_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][14]));
        let eq49_e1342_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][15]));
        let eq49_e1342_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][16]));
        let eq49_e1342_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][17]));
        let eq49_e1342_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][18]));
        let eq49_e1342_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][19]));
        let eq49_e1342_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1956]) + (eq49_e1340 * s.dn[1956][20]));
        let eq49_e1342_d_b0: f64 = (((-s.db[1995][0]) * s.v[1956]) + (eq49_e1340 * s.db[1956][0]));
        let eq49_e1342_d_b1: f64 = (((-s.db[1995][1]) * s.v[1956]) + (eq49_e1340 * s.db[1956][1]));
        let eq49_e1342_d_b2: f64 = (((-s.db[1995][2]) * s.v[1956]) + (eq49_e1340 * s.db[1956][2]));
        let eq49_e1342_d_b3: f64 = (((-s.db[1995][3]) * s.v[1956]) + (eq49_e1340 * s.db[1956][3]));
        let eq49_e1342_d_b4: f64 = (((-s.db[1995][4]) * s.v[1956]) + (eq49_e1340 * s.db[1956][4]));
        let eq49_e1342_d_b5: f64 = (((-s.db[1995][5]) * s.v[1956]) + (eq49_e1340 * s.db[1956][5]));
        let eq49_e1342_d_b6: f64 = (((-s.db[1995][6]) * s.v[1956]) + (eq49_e1340 * s.db[1956][6]));
        let eq49_e1342_d_b7: f64 = (((-s.db[1995][7]) * s.v[1956]) + (eq49_e1340 * s.db[1956][7]));
        let eq49_e1342_d_b8: f64 = (((-s.db[1995][8]) * s.v[1956]) + (eq49_e1340 * s.db[1956][8]));
        let eq49_e1342_d_b9: f64 = (((-s.db[1995][9]) * s.v[1956]) + (eq49_e1340 * s.db[1956][9]));
        let eq49_e1342_d_b10: f64 = (((-s.db[1995][10]) * s.v[1956]) + (eq49_e1340 * s.db[1956][10]));
        let eq49_e1342_d_b11: f64 = (((-s.db[1995][11]) * s.v[1956]) + (eq49_e1340 * s.db[1956][11]));
        let eq49_e1342_d_b12: f64 = (((-s.db[1995][12]) * s.v[1956]) + (eq49_e1340 * s.db[1956][12]));
        let eq49_e1342_d_b13: f64 = (((-s.db[1995][13]) * s.v[1956]) + (eq49_e1340 * s.db[1956][13]));
        let eq49_e1342_d_b14: f64 = (((-s.db[1995][14]) * s.v[1956]) + (eq49_e1340 * s.db[1956][14]));
        let eq49_e1342_d_b15: f64 = (((-s.db[1995][15]) * s.v[1956]) + (eq49_e1340 * s.db[1956][15]));
        let eq49_e1342_d_b16: f64 = (((-s.db[1995][16]) * s.v[1956]) + (eq49_e1340 * s.db[1956][16]));
        let eq49_e1342_d_b17: f64 = (((-s.db[1995][17]) * s.v[1956]) + (eq49_e1340 * s.db[1956][17]));
        let eq49_e1342_d_b18: f64 = (((-s.db[1995][18]) * s.v[1956]) + (eq49_e1340 * s.db[1956][18]));
        let eq49_e1342_d_b19: f64 = (((-s.db[1995][19]) * s.v[1956]) + (eq49_e1340 * s.db[1956][19]));
        let eq49_e1342_d_b20: f64 = (((-s.db[1995][20]) * s.v[1956]) + (eq49_e1340 * s.db[1956][20]));
        let eq49_e1342_d_b21: f64 = (((-s.db[1995][21]) * s.v[1956]) + (eq49_e1340 * s.db[1956][21]));
        let eq49_e1342_d_b22: f64 = (((-s.db[1995][22]) * s.v[1956]) + (eq49_e1340 * s.db[1956][22]));
        let eq49_e1342_d_b23: f64 = (((-s.db[1995][23]) * s.v[1956]) + (eq49_e1340 * s.db[1956][23]));
        let eq49_e1342_d_b24: f64 = (((-s.db[1995][24]) * s.v[1956]) + (eq49_e1340 * s.db[1956][24]));
        let eq49_e1344: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 5, eq49_e1342, s.v[1947]);
        let eq49_e1345: f64 = (s.v[4] * eq49_e1344);
        let eq49_e1345_d_n0: f64 = (s.v[4] * (eq49_e1342_d_n0 * idt_scale));
        let eq49_e1345_d_n1: f64 = (s.v[4] * (eq49_e1342_d_n1 * idt_scale));
        let eq49_e1345_d_n2: f64 = (s.v[4] * (eq49_e1342_d_n2 * idt_scale));
        let eq49_e1345_d_n3: f64 = (s.v[4] * (eq49_e1342_d_n3 * idt_scale));
        let eq49_e1345_d_n4: f64 = (s.v[4] * (eq49_e1342_d_n4 * idt_scale));
        let eq49_e1345_d_n5: f64 = (s.v[4] * (eq49_e1342_d_n5 * idt_scale));
        let eq49_e1345_d_n6: f64 = (s.v[4] * (eq49_e1342_d_n6 * idt_scale));
        let eq49_e1345_d_n7: f64 = (s.v[4] * (eq49_e1342_d_n7 * idt_scale));
        let eq49_e1345_d_n8: f64 = (s.v[4] * (eq49_e1342_d_n8 * idt_scale));
        let eq49_e1345_d_n9: f64 = (s.v[4] * (eq49_e1342_d_n9 * idt_scale));
        let eq49_e1345_d_n10: f64 = (s.v[4] * (eq49_e1342_d_n10 * idt_scale));
        let eq49_e1345_d_n11: f64 = (s.v[4] * (eq49_e1342_d_n11 * idt_scale));
        let eq49_e1345_d_n12: f64 = (s.v[4] * (eq49_e1342_d_n12 * idt_scale));
        let eq49_e1345_d_n13: f64 = (s.v[4] * (eq49_e1342_d_n13 * idt_scale));
        let eq49_e1345_d_n14: f64 = (s.v[4] * (eq49_e1342_d_n14 * idt_scale));
        let eq49_e1345_d_n15: f64 = (s.v[4] * (eq49_e1342_d_n15 * idt_scale));
        let eq49_e1345_d_n16: f64 = (s.v[4] * (eq49_e1342_d_n16 * idt_scale));
        let eq49_e1345_d_n17: f64 = (s.v[4] * (eq49_e1342_d_n17 * idt_scale));
        let eq49_e1345_d_n18: f64 = (s.v[4] * (eq49_e1342_d_n18 * idt_scale));
        let eq49_e1345_d_n19: f64 = (s.v[4] * (eq49_e1342_d_n19 * idt_scale));
        let eq49_e1345_d_n20: f64 = (s.v[4] * (eq49_e1342_d_n20 * idt_scale));
        let eq49_e1345_d_b0: f64 = (s.v[4] * (eq49_e1342_d_b0 * idt_scale));
        let eq49_e1345_d_b1: f64 = (s.v[4] * (eq49_e1342_d_b1 * idt_scale));
        let eq49_e1345_d_b2: f64 = (s.v[4] * (eq49_e1342_d_b2 * idt_scale));
        let eq49_e1345_d_b3: f64 = (s.v[4] * (eq49_e1342_d_b3 * idt_scale));
        let eq49_e1345_d_b4: f64 = (s.v[4] * (eq49_e1342_d_b4 * idt_scale));
        let eq49_e1345_d_b5: f64 = (s.v[4] * (eq49_e1342_d_b5 * idt_scale));
        let eq49_e1345_d_b6: f64 = (s.v[4] * (eq49_e1342_d_b6 * idt_scale));
        let eq49_e1345_d_b7: f64 = (s.v[4] * (eq49_e1342_d_b7 * idt_scale));
        let eq49_e1345_d_b8: f64 = (s.v[4] * (eq49_e1342_d_b8 * idt_scale));
        let eq49_e1345_d_b9: f64 = (s.v[4] * (eq49_e1342_d_b9 * idt_scale));
        let eq49_e1345_d_b10: f64 = (s.v[4] * (eq49_e1342_d_b10 * idt_scale));
        let eq49_e1345_d_b11: f64 = (s.v[4] * (eq49_e1342_d_b11 * idt_scale));
        let eq49_e1345_d_b12: f64 = (s.v[4] * (eq49_e1342_d_b12 * idt_scale));
        let eq49_e1345_d_b13: f64 = (s.v[4] * (eq49_e1342_d_b13 * idt_scale));
        let eq49_e1345_d_b14: f64 = (s.v[4] * (eq49_e1342_d_b14 * idt_scale));
        let eq49_e1345_d_b15: f64 = (s.v[4] * (eq49_e1342_d_b15 * idt_scale));
        let eq49_e1345_d_b16: f64 = (s.v[4] * (eq49_e1342_d_b16 * idt_scale));
        let eq49_e1345_d_b17: f64 = (s.v[4] * (eq49_e1342_d_b17 * idt_scale));
        let eq49_e1345_d_b18: f64 = (s.v[4] * (eq49_e1342_d_b18 * idt_scale));
        let eq49_e1345_d_b19: f64 = (s.v[4] * (eq49_e1342_d_b19 * idt_scale));
        let eq49_e1345_d_b20: f64 = (s.v[4] * (eq49_e1342_d_b20 * idt_scale));
        let eq49_e1345_d_b21: f64 = (s.v[4] * (eq49_e1342_d_b21 * idt_scale));
        let eq49_e1345_d_b22: f64 = (s.v[4] * (eq49_e1342_d_b22 * idt_scale));
        let eq49_e1345_d_b23: f64 = (s.v[4] * (eq49_e1342_d_b23 * idt_scale));
        let eq49_e1345_d_b24: f64 = (s.v[4] * (eq49_e1342_d_b24 * idt_scale));
        let eq49_value: f64 = eq49_e1345;
        let eq49_node_derivatives: [f64; 21] = [eq49_e1345_d_n0, eq49_e1345_d_n1, eq49_e1345_d_n2, eq49_e1345_d_n3, eq49_e1345_d_n4, eq49_e1345_d_n5, eq49_e1345_d_n6, eq49_e1345_d_n7, eq49_e1345_d_n8, eq49_e1345_d_n9, eq49_e1345_d_n10, eq49_e1345_d_n11, eq49_e1345_d_n12, eq49_e1345_d_n13, eq49_e1345_d_n14, eq49_e1345_d_n15, eq49_e1345_d_n16, eq49_e1345_d_n17, eq49_e1345_d_n18, eq49_e1345_d_n19, eq49_e1345_d_n20];
        let eq49_branch_derivatives: [f64; 25] = [eq49_e1345_d_b0, eq49_e1345_d_b1, eq49_e1345_d_b2, eq49_e1345_d_b3, eq49_e1345_d_b4, eq49_e1345_d_b5, eq49_e1345_d_b6, eq49_e1345_d_b7, eq49_e1345_d_b8, eq49_e1345_d_b9, eq49_e1345_d_b10, eq49_e1345_d_b11, eq49_e1345_d_b12, eq49_e1345_d_b13, eq49_e1345_d_b14, eq49_e1345_d_b15, eq49_e1345_d_b16, eq49_e1345_d_b17, eq49_e1345_d_b18, eq49_e1345_d_b19, eq49_e1345_d_b20, eq49_e1345_d_b21, eq49_e1345_d_b22, eq49_e1345_d_b23, eq49_e1345_d_b24];
        stamper.stamp_potential_dense_local(
            18,
            eq49_value,
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        var_chnl_type: f64,
        var_mult_inst: f64,
    ) {
        let eq51_e1353: f64 = (-s.v[1995]);
        let eq51_e1355: f64 = (eq51_e1353 * s.v[1957]);
        let eq51_e1355_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][0]));
        let eq51_e1355_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][1]));
        let eq51_e1355_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][2]));
        let eq51_e1355_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][3]));
        let eq51_e1355_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][4]));
        let eq51_e1355_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][5]));
        let eq51_e1355_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][6]));
        let eq51_e1355_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][7]));
        let eq51_e1355_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][8]));
        let eq51_e1355_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][9]));
        let eq51_e1355_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][10]));
        let eq51_e1355_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][11]));
        let eq51_e1355_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][12]));
        let eq51_e1355_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][13]));
        let eq51_e1355_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][14]));
        let eq51_e1355_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][15]));
        let eq51_e1355_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][16]));
        let eq51_e1355_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][17]));
        let eq51_e1355_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][18]));
        let eq51_e1355_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][19]));
        let eq51_e1355_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1957]) + (eq51_e1353 * s.dn[1957][20]));
        let eq51_e1355_d_b0: f64 = (((-s.db[1995][0]) * s.v[1957]) + (eq51_e1353 * s.db[1957][0]));
        let eq51_e1355_d_b1: f64 = (((-s.db[1995][1]) * s.v[1957]) + (eq51_e1353 * s.db[1957][1]));
        let eq51_e1355_d_b2: f64 = (((-s.db[1995][2]) * s.v[1957]) + (eq51_e1353 * s.db[1957][2]));
        let eq51_e1355_d_b3: f64 = (((-s.db[1995][3]) * s.v[1957]) + (eq51_e1353 * s.db[1957][3]));
        let eq51_e1355_d_b4: f64 = (((-s.db[1995][4]) * s.v[1957]) + (eq51_e1353 * s.db[1957][4]));
        let eq51_e1355_d_b5: f64 = (((-s.db[1995][5]) * s.v[1957]) + (eq51_e1353 * s.db[1957][5]));
        let eq51_e1355_d_b6: f64 = (((-s.db[1995][6]) * s.v[1957]) + (eq51_e1353 * s.db[1957][6]));
        let eq51_e1355_d_b7: f64 = (((-s.db[1995][7]) * s.v[1957]) + (eq51_e1353 * s.db[1957][7]));
        let eq51_e1355_d_b8: f64 = (((-s.db[1995][8]) * s.v[1957]) + (eq51_e1353 * s.db[1957][8]));
        let eq51_e1355_d_b9: f64 = (((-s.db[1995][9]) * s.v[1957]) + (eq51_e1353 * s.db[1957][9]));
        let eq51_e1355_d_b10: f64 = (((-s.db[1995][10]) * s.v[1957]) + (eq51_e1353 * s.db[1957][10]));
        let eq51_e1355_d_b11: f64 = (((-s.db[1995][11]) * s.v[1957]) + (eq51_e1353 * s.db[1957][11]));
        let eq51_e1355_d_b12: f64 = (((-s.db[1995][12]) * s.v[1957]) + (eq51_e1353 * s.db[1957][12]));
        let eq51_e1355_d_b13: f64 = (((-s.db[1995][13]) * s.v[1957]) + (eq51_e1353 * s.db[1957][13]));
        let eq51_e1355_d_b14: f64 = (((-s.db[1995][14]) * s.v[1957]) + (eq51_e1353 * s.db[1957][14]));
        let eq51_e1355_d_b15: f64 = (((-s.db[1995][15]) * s.v[1957]) + (eq51_e1353 * s.db[1957][15]));
        let eq51_e1355_d_b16: f64 = (((-s.db[1995][16]) * s.v[1957]) + (eq51_e1353 * s.db[1957][16]));
        let eq51_e1355_d_b17: f64 = (((-s.db[1995][17]) * s.v[1957]) + (eq51_e1353 * s.db[1957][17]));
        let eq51_e1355_d_b18: f64 = (((-s.db[1995][18]) * s.v[1957]) + (eq51_e1353 * s.db[1957][18]));
        let eq51_e1355_d_b19: f64 = (((-s.db[1995][19]) * s.v[1957]) + (eq51_e1353 * s.db[1957][19]));
        let eq51_e1355_d_b20: f64 = (((-s.db[1995][20]) * s.v[1957]) + (eq51_e1353 * s.db[1957][20]));
        let eq51_e1355_d_b21: f64 = (((-s.db[1995][21]) * s.v[1957]) + (eq51_e1353 * s.db[1957][21]));
        let eq51_e1355_d_b22: f64 = (((-s.db[1995][22]) * s.v[1957]) + (eq51_e1353 * s.db[1957][22]));
        let eq51_e1355_d_b23: f64 = (((-s.db[1995][23]) * s.v[1957]) + (eq51_e1353 * s.db[1957][23]));
        let eq51_e1355_d_b24: f64 = (((-s.db[1995][24]) * s.v[1957]) + (eq51_e1353 * s.db[1957][24]));
        let eq51_e1357: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 6, eq51_e1355, s.v[1948]);
        let eq51_e1358: f64 = (s.v[4] * eq51_e1357);
        let eq51_e1358_d_n0: f64 = (s.v[4] * (eq51_e1355_d_n0 * idt_scale));
        let eq51_e1358_d_n1: f64 = (s.v[4] * (eq51_e1355_d_n1 * idt_scale));
        let eq51_e1358_d_n2: f64 = (s.v[4] * (eq51_e1355_d_n2 * idt_scale));
        let eq51_e1358_d_n3: f64 = (s.v[4] * (eq51_e1355_d_n3 * idt_scale));
        let eq51_e1358_d_n4: f64 = (s.v[4] * (eq51_e1355_d_n4 * idt_scale));
        let eq51_e1358_d_n5: f64 = (s.v[4] * (eq51_e1355_d_n5 * idt_scale));
        let eq51_e1358_d_n6: f64 = (s.v[4] * (eq51_e1355_d_n6 * idt_scale));
        let eq51_e1358_d_n7: f64 = (s.v[4] * (eq51_e1355_d_n7 * idt_scale));
        let eq51_e1358_d_n8: f64 = (s.v[4] * (eq51_e1355_d_n8 * idt_scale));
        let eq51_e1358_d_n9: f64 = (s.v[4] * (eq51_e1355_d_n9 * idt_scale));
        let eq51_e1358_d_n10: f64 = (s.v[4] * (eq51_e1355_d_n10 * idt_scale));
        let eq51_e1358_d_n11: f64 = (s.v[4] * (eq51_e1355_d_n11 * idt_scale));
        let eq51_e1358_d_n12: f64 = (s.v[4] * (eq51_e1355_d_n12 * idt_scale));
        let eq51_e1358_d_n13: f64 = (s.v[4] * (eq51_e1355_d_n13 * idt_scale));
        let eq51_e1358_d_n14: f64 = (s.v[4] * (eq51_e1355_d_n14 * idt_scale));
        let eq51_e1358_d_n15: f64 = (s.v[4] * (eq51_e1355_d_n15 * idt_scale));
        let eq51_e1358_d_n16: f64 = (s.v[4] * (eq51_e1355_d_n16 * idt_scale));
        let eq51_e1358_d_n17: f64 = (s.v[4] * (eq51_e1355_d_n17 * idt_scale));
        let eq51_e1358_d_n18: f64 = (s.v[4] * (eq51_e1355_d_n18 * idt_scale));
        let eq51_e1358_d_n19: f64 = (s.v[4] * (eq51_e1355_d_n19 * idt_scale));
        let eq51_e1358_d_n20: f64 = (s.v[4] * (eq51_e1355_d_n20 * idt_scale));
        let eq51_e1358_d_b0: f64 = (s.v[4] * (eq51_e1355_d_b0 * idt_scale));
        let eq51_e1358_d_b1: f64 = (s.v[4] * (eq51_e1355_d_b1 * idt_scale));
        let eq51_e1358_d_b2: f64 = (s.v[4] * (eq51_e1355_d_b2 * idt_scale));
        let eq51_e1358_d_b3: f64 = (s.v[4] * (eq51_e1355_d_b3 * idt_scale));
        let eq51_e1358_d_b4: f64 = (s.v[4] * (eq51_e1355_d_b4 * idt_scale));
        let eq51_e1358_d_b5: f64 = (s.v[4] * (eq51_e1355_d_b5 * idt_scale));
        let eq51_e1358_d_b6: f64 = (s.v[4] * (eq51_e1355_d_b6 * idt_scale));
        let eq51_e1358_d_b7: f64 = (s.v[4] * (eq51_e1355_d_b7 * idt_scale));
        let eq51_e1358_d_b8: f64 = (s.v[4] * (eq51_e1355_d_b8 * idt_scale));
        let eq51_e1358_d_b9: f64 = (s.v[4] * (eq51_e1355_d_b9 * idt_scale));
        let eq51_e1358_d_b10: f64 = (s.v[4] * (eq51_e1355_d_b10 * idt_scale));
        let eq51_e1358_d_b11: f64 = (s.v[4] * (eq51_e1355_d_b11 * idt_scale));
        let eq51_e1358_d_b12: f64 = (s.v[4] * (eq51_e1355_d_b12 * idt_scale));
        let eq51_e1358_d_b13: f64 = (s.v[4] * (eq51_e1355_d_b13 * idt_scale));
        let eq51_e1358_d_b14: f64 = (s.v[4] * (eq51_e1355_d_b14 * idt_scale));
        let eq51_e1358_d_b15: f64 = (s.v[4] * (eq51_e1355_d_b15 * idt_scale));
        let eq51_e1358_d_b16: f64 = (s.v[4] * (eq51_e1355_d_b16 * idt_scale));
        let eq51_e1358_d_b17: f64 = (s.v[4] * (eq51_e1355_d_b17 * idt_scale));
        let eq51_e1358_d_b18: f64 = (s.v[4] * (eq51_e1355_d_b18 * idt_scale));
        let eq51_e1358_d_b19: f64 = (s.v[4] * (eq51_e1355_d_b19 * idt_scale));
        let eq51_e1358_d_b20: f64 = (s.v[4] * (eq51_e1355_d_b20 * idt_scale));
        let eq51_e1358_d_b21: f64 = (s.v[4] * (eq51_e1355_d_b21 * idt_scale));
        let eq51_e1358_d_b22: f64 = (s.v[4] * (eq51_e1355_d_b22 * idt_scale));
        let eq51_e1358_d_b23: f64 = (s.v[4] * (eq51_e1355_d_b23 * idt_scale));
        let eq51_e1358_d_b24: f64 = (s.v[4] * (eq51_e1355_d_b24 * idt_scale));
        let eq51_value: f64 = eq51_e1358;
        let eq51_node_derivatives: [f64; 21] = [eq51_e1358_d_n0, eq51_e1358_d_n1, eq51_e1358_d_n2, eq51_e1358_d_n3, eq51_e1358_d_n4, eq51_e1358_d_n5, eq51_e1358_d_n6, eq51_e1358_d_n7, eq51_e1358_d_n8, eq51_e1358_d_n9, eq51_e1358_d_n10, eq51_e1358_d_n11, eq51_e1358_d_n12, eq51_e1358_d_n13, eq51_e1358_d_n14, eq51_e1358_d_n15, eq51_e1358_d_n16, eq51_e1358_d_n17, eq51_e1358_d_n18, eq51_e1358_d_n19, eq51_e1358_d_n20];
        let eq51_branch_derivatives: [f64; 25] = [eq51_e1358_d_b0, eq51_e1358_d_b1, eq51_e1358_d_b2, eq51_e1358_d_b3, eq51_e1358_d_b4, eq51_e1358_d_b5, eq51_e1358_d_b6, eq51_e1358_d_b7, eq51_e1358_d_b8, eq51_e1358_d_b9, eq51_e1358_d_b10, eq51_e1358_d_b11, eq51_e1358_d_b12, eq51_e1358_d_b13, eq51_e1358_d_b14, eq51_e1358_d_b15, eq51_e1358_d_b16, eq51_e1358_d_b17, eq51_e1358_d_b18, eq51_e1358_d_b19, eq51_e1358_d_b20, eq51_e1358_d_b21, eq51_e1358_d_b22, eq51_e1358_d_b23, eq51_e1358_d_b24];
        stamper.stamp_potential_dense_local(
            20,
            eq51_value,
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
        );
        let eq53_e1366: f64 = (-s.v[1995]);
        let eq53_e1368: f64 = (eq53_e1366 * s.v[1958]);
        let eq53_e1368_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][0]));
        let eq53_e1368_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][1]));
        let eq53_e1368_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][2]));
        let eq53_e1368_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][3]));
        let eq53_e1368_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][4]));
        let eq53_e1368_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][5]));
        let eq53_e1368_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][6]));
        let eq53_e1368_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][7]));
        let eq53_e1368_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][8]));
        let eq53_e1368_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][9]));
        let eq53_e1368_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][10]));
        let eq53_e1368_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][11]));
        let eq53_e1368_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][12]));
        let eq53_e1368_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][13]));
        let eq53_e1368_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][14]));
        let eq53_e1368_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][15]));
        let eq53_e1368_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][16]));
        let eq53_e1368_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][17]));
        let eq53_e1368_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][18]));
        let eq53_e1368_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][19]));
        let eq53_e1368_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1958]) + (eq53_e1366 * s.dn[1958][20]));
        let eq53_e1368_d_b0: f64 = (((-s.db[1995][0]) * s.v[1958]) + (eq53_e1366 * s.db[1958][0]));
        let eq53_e1368_d_b1: f64 = (((-s.db[1995][1]) * s.v[1958]) + (eq53_e1366 * s.db[1958][1]));
        let eq53_e1368_d_b2: f64 = (((-s.db[1995][2]) * s.v[1958]) + (eq53_e1366 * s.db[1958][2]));
        let eq53_e1368_d_b3: f64 = (((-s.db[1995][3]) * s.v[1958]) + (eq53_e1366 * s.db[1958][3]));
        let eq53_e1368_d_b4: f64 = (((-s.db[1995][4]) * s.v[1958]) + (eq53_e1366 * s.db[1958][4]));
        let eq53_e1368_d_b5: f64 = (((-s.db[1995][5]) * s.v[1958]) + (eq53_e1366 * s.db[1958][5]));
        let eq53_e1368_d_b6: f64 = (((-s.db[1995][6]) * s.v[1958]) + (eq53_e1366 * s.db[1958][6]));
        let eq53_e1368_d_b7: f64 = (((-s.db[1995][7]) * s.v[1958]) + (eq53_e1366 * s.db[1958][7]));
        let eq53_e1368_d_b8: f64 = (((-s.db[1995][8]) * s.v[1958]) + (eq53_e1366 * s.db[1958][8]));
        let eq53_e1368_d_b9: f64 = (((-s.db[1995][9]) * s.v[1958]) + (eq53_e1366 * s.db[1958][9]));
        let eq53_e1368_d_b10: f64 = (((-s.db[1995][10]) * s.v[1958]) + (eq53_e1366 * s.db[1958][10]));
        let eq53_e1368_d_b11: f64 = (((-s.db[1995][11]) * s.v[1958]) + (eq53_e1366 * s.db[1958][11]));
        let eq53_e1368_d_b12: f64 = (((-s.db[1995][12]) * s.v[1958]) + (eq53_e1366 * s.db[1958][12]));
        let eq53_e1368_d_b13: f64 = (((-s.db[1995][13]) * s.v[1958]) + (eq53_e1366 * s.db[1958][13]));
        let eq53_e1368_d_b14: f64 = (((-s.db[1995][14]) * s.v[1958]) + (eq53_e1366 * s.db[1958][14]));
        let eq53_e1368_d_b15: f64 = (((-s.db[1995][15]) * s.v[1958]) + (eq53_e1366 * s.db[1958][15]));
        let eq53_e1368_d_b16: f64 = (((-s.db[1995][16]) * s.v[1958]) + (eq53_e1366 * s.db[1958][16]));
        let eq53_e1368_d_b17: f64 = (((-s.db[1995][17]) * s.v[1958]) + (eq53_e1366 * s.db[1958][17]));
        let eq53_e1368_d_b18: f64 = (((-s.db[1995][18]) * s.v[1958]) + (eq53_e1366 * s.db[1958][18]));
        let eq53_e1368_d_b19: f64 = (((-s.db[1995][19]) * s.v[1958]) + (eq53_e1366 * s.db[1958][19]));
        let eq53_e1368_d_b20: f64 = (((-s.db[1995][20]) * s.v[1958]) + (eq53_e1366 * s.db[1958][20]));
        let eq53_e1368_d_b21: f64 = (((-s.db[1995][21]) * s.v[1958]) + (eq53_e1366 * s.db[1958][21]));
        let eq53_e1368_d_b22: f64 = (((-s.db[1995][22]) * s.v[1958]) + (eq53_e1366 * s.db[1958][22]));
        let eq53_e1368_d_b23: f64 = (((-s.db[1995][23]) * s.v[1958]) + (eq53_e1366 * s.db[1958][23]));
        let eq53_e1368_d_b24: f64 = (((-s.db[1995][24]) * s.v[1958]) + (eq53_e1366 * s.db[1958][24]));
        let eq53_e1370: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 7, eq53_e1368, s.v[1949]);
        let eq53_e1371: f64 = (s.v[4] * eq53_e1370);
        let eq53_e1371_d_n0: f64 = (s.v[4] * (eq53_e1368_d_n0 * idt_scale));
        let eq53_e1371_d_n1: f64 = (s.v[4] * (eq53_e1368_d_n1 * idt_scale));
        let eq53_e1371_d_n2: f64 = (s.v[4] * (eq53_e1368_d_n2 * idt_scale));
        let eq53_e1371_d_n3: f64 = (s.v[4] * (eq53_e1368_d_n3 * idt_scale));
        let eq53_e1371_d_n4: f64 = (s.v[4] * (eq53_e1368_d_n4 * idt_scale));
        let eq53_e1371_d_n5: f64 = (s.v[4] * (eq53_e1368_d_n5 * idt_scale));
        let eq53_e1371_d_n6: f64 = (s.v[4] * (eq53_e1368_d_n6 * idt_scale));
        let eq53_e1371_d_n7: f64 = (s.v[4] * (eq53_e1368_d_n7 * idt_scale));
        let eq53_e1371_d_n8: f64 = (s.v[4] * (eq53_e1368_d_n8 * idt_scale));
        let eq53_e1371_d_n9: f64 = (s.v[4] * (eq53_e1368_d_n9 * idt_scale));
        let eq53_e1371_d_n10: f64 = (s.v[4] * (eq53_e1368_d_n10 * idt_scale));
        let eq53_e1371_d_n11: f64 = (s.v[4] * (eq53_e1368_d_n11 * idt_scale));
        let eq53_e1371_d_n12: f64 = (s.v[4] * (eq53_e1368_d_n12 * idt_scale));
        let eq53_e1371_d_n13: f64 = (s.v[4] * (eq53_e1368_d_n13 * idt_scale));
        let eq53_e1371_d_n14: f64 = (s.v[4] * (eq53_e1368_d_n14 * idt_scale));
        let eq53_e1371_d_n15: f64 = (s.v[4] * (eq53_e1368_d_n15 * idt_scale));
        let eq53_e1371_d_n16: f64 = (s.v[4] * (eq53_e1368_d_n16 * idt_scale));
        let eq53_e1371_d_n17: f64 = (s.v[4] * (eq53_e1368_d_n17 * idt_scale));
        let eq53_e1371_d_n18: f64 = (s.v[4] * (eq53_e1368_d_n18 * idt_scale));
        let eq53_e1371_d_n19: f64 = (s.v[4] * (eq53_e1368_d_n19 * idt_scale));
        let eq53_e1371_d_n20: f64 = (s.v[4] * (eq53_e1368_d_n20 * idt_scale));
        let eq53_e1371_d_b0: f64 = (s.v[4] * (eq53_e1368_d_b0 * idt_scale));
        let eq53_e1371_d_b1: f64 = (s.v[4] * (eq53_e1368_d_b1 * idt_scale));
        let eq53_e1371_d_b2: f64 = (s.v[4] * (eq53_e1368_d_b2 * idt_scale));
        let eq53_e1371_d_b3: f64 = (s.v[4] * (eq53_e1368_d_b3 * idt_scale));
        let eq53_e1371_d_b4: f64 = (s.v[4] * (eq53_e1368_d_b4 * idt_scale));
        let eq53_e1371_d_b5: f64 = (s.v[4] * (eq53_e1368_d_b5 * idt_scale));
        let eq53_e1371_d_b6: f64 = (s.v[4] * (eq53_e1368_d_b6 * idt_scale));
        let eq53_e1371_d_b7: f64 = (s.v[4] * (eq53_e1368_d_b7 * idt_scale));
        let eq53_e1371_d_b8: f64 = (s.v[4] * (eq53_e1368_d_b8 * idt_scale));
        let eq53_e1371_d_b9: f64 = (s.v[4] * (eq53_e1368_d_b9 * idt_scale));
        let eq53_e1371_d_b10: f64 = (s.v[4] * (eq53_e1368_d_b10 * idt_scale));
        let eq53_e1371_d_b11: f64 = (s.v[4] * (eq53_e1368_d_b11 * idt_scale));
        let eq53_e1371_d_b12: f64 = (s.v[4] * (eq53_e1368_d_b12 * idt_scale));
        let eq53_e1371_d_b13: f64 = (s.v[4] * (eq53_e1368_d_b13 * idt_scale));
        let eq53_e1371_d_b14: f64 = (s.v[4] * (eq53_e1368_d_b14 * idt_scale));
        let eq53_e1371_d_b15: f64 = (s.v[4] * (eq53_e1368_d_b15 * idt_scale));
        let eq53_e1371_d_b16: f64 = (s.v[4] * (eq53_e1368_d_b16 * idt_scale));
        let eq53_e1371_d_b17: f64 = (s.v[4] * (eq53_e1368_d_b17 * idt_scale));
        let eq53_e1371_d_b18: f64 = (s.v[4] * (eq53_e1368_d_b18 * idt_scale));
        let eq53_e1371_d_b19: f64 = (s.v[4] * (eq53_e1368_d_b19 * idt_scale));
        let eq53_e1371_d_b20: f64 = (s.v[4] * (eq53_e1368_d_b20 * idt_scale));
        let eq53_e1371_d_b21: f64 = (s.v[4] * (eq53_e1368_d_b21 * idt_scale));
        let eq53_e1371_d_b22: f64 = (s.v[4] * (eq53_e1368_d_b22 * idt_scale));
        let eq53_e1371_d_b23: f64 = (s.v[4] * (eq53_e1368_d_b23 * idt_scale));
        let eq53_e1371_d_b24: f64 = (s.v[4] * (eq53_e1368_d_b24 * idt_scale));
        let eq53_value: f64 = eq53_e1371;
        let eq53_node_derivatives: [f64; 21] = [eq53_e1371_d_n0, eq53_e1371_d_n1, eq53_e1371_d_n2, eq53_e1371_d_n3, eq53_e1371_d_n4, eq53_e1371_d_n5, eq53_e1371_d_n6, eq53_e1371_d_n7, eq53_e1371_d_n8, eq53_e1371_d_n9, eq53_e1371_d_n10, eq53_e1371_d_n11, eq53_e1371_d_n12, eq53_e1371_d_n13, eq53_e1371_d_n14, eq53_e1371_d_n15, eq53_e1371_d_n16, eq53_e1371_d_n17, eq53_e1371_d_n18, eq53_e1371_d_n19, eq53_e1371_d_n20];
        let eq53_branch_derivatives: [f64; 25] = [eq53_e1371_d_b0, eq53_e1371_d_b1, eq53_e1371_d_b2, eq53_e1371_d_b3, eq53_e1371_d_b4, eq53_e1371_d_b5, eq53_e1371_d_b6, eq53_e1371_d_b7, eq53_e1371_d_b8, eq53_e1371_d_b9, eq53_e1371_d_b10, eq53_e1371_d_b11, eq53_e1371_d_b12, eq53_e1371_d_b13, eq53_e1371_d_b14, eq53_e1371_d_b15, eq53_e1371_d_b16, eq53_e1371_d_b17, eq53_e1371_d_b18, eq53_e1371_d_b19, eq53_e1371_d_b20, eq53_e1371_d_b21, eq53_e1371_d_b22, eq53_e1371_d_b23, eq53_e1371_d_b24];
        stamper.stamp_potential_dense_local(
            22,
            eq53_value,
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
        );
        let eq55_e1379: f64 = (-s.v[1995]);
        let eq55_e1381: f64 = (eq55_e1379 * s.v[1959]);
        let eq55_e1381_d_n0: f64 = (((-s.dn[1995][0]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][0]));
        let eq55_e1381_d_n1: f64 = (((-s.dn[1995][1]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][1]));
        let eq55_e1381_d_n2: f64 = (((-s.dn[1995][2]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][2]));
        let eq55_e1381_d_n3: f64 = (((-s.dn[1995][3]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][3]));
        let eq55_e1381_d_n4: f64 = (((-s.dn[1995][4]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][4]));
        let eq55_e1381_d_n5: f64 = (((-s.dn[1995][5]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][5]));
        let eq55_e1381_d_n6: f64 = (((-s.dn[1995][6]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][6]));
        let eq55_e1381_d_n7: f64 = (((-s.dn[1995][7]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][7]));
        let eq55_e1381_d_n8: f64 = (((-s.dn[1995][8]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][8]));
        let eq55_e1381_d_n9: f64 = (((-s.dn[1995][9]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][9]));
        let eq55_e1381_d_n10: f64 = (((-s.dn[1995][10]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][10]));
        let eq55_e1381_d_n11: f64 = (((-s.dn[1995][11]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][11]));
        let eq55_e1381_d_n12: f64 = (((-s.dn[1995][12]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][12]));
        let eq55_e1381_d_n13: f64 = (((-s.dn[1995][13]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][13]));
        let eq55_e1381_d_n14: f64 = (((-s.dn[1995][14]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][14]));
        let eq55_e1381_d_n15: f64 = (((-s.dn[1995][15]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][15]));
        let eq55_e1381_d_n16: f64 = (((-s.dn[1995][16]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][16]));
        let eq55_e1381_d_n17: f64 = (((-s.dn[1995][17]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][17]));
        let eq55_e1381_d_n18: f64 = (((-s.dn[1995][18]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][18]));
        let eq55_e1381_d_n19: f64 = (((-s.dn[1995][19]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][19]));
        let eq55_e1381_d_n20: f64 = (((-s.dn[1995][20]) * s.v[1959]) + (eq55_e1379 * s.dn[1959][20]));
        let eq55_e1381_d_b0: f64 = (((-s.db[1995][0]) * s.v[1959]) + (eq55_e1379 * s.db[1959][0]));
        let eq55_e1381_d_b1: f64 = (((-s.db[1995][1]) * s.v[1959]) + (eq55_e1379 * s.db[1959][1]));
        let eq55_e1381_d_b2: f64 = (((-s.db[1995][2]) * s.v[1959]) + (eq55_e1379 * s.db[1959][2]));
        let eq55_e1381_d_b3: f64 = (((-s.db[1995][3]) * s.v[1959]) + (eq55_e1379 * s.db[1959][3]));
        let eq55_e1381_d_b4: f64 = (((-s.db[1995][4]) * s.v[1959]) + (eq55_e1379 * s.db[1959][4]));
        let eq55_e1381_d_b5: f64 = (((-s.db[1995][5]) * s.v[1959]) + (eq55_e1379 * s.db[1959][5]));
        let eq55_e1381_d_b6: f64 = (((-s.db[1995][6]) * s.v[1959]) + (eq55_e1379 * s.db[1959][6]));
        let eq55_e1381_d_b7: f64 = (((-s.db[1995][7]) * s.v[1959]) + (eq55_e1379 * s.db[1959][7]));
        let eq55_e1381_d_b8: f64 = (((-s.db[1995][8]) * s.v[1959]) + (eq55_e1379 * s.db[1959][8]));
        let eq55_e1381_d_b9: f64 = (((-s.db[1995][9]) * s.v[1959]) + (eq55_e1379 * s.db[1959][9]));
        let eq55_e1381_d_b10: f64 = (((-s.db[1995][10]) * s.v[1959]) + (eq55_e1379 * s.db[1959][10]));
        let eq55_e1381_d_b11: f64 = (((-s.db[1995][11]) * s.v[1959]) + (eq55_e1379 * s.db[1959][11]));
        let eq55_e1381_d_b12: f64 = (((-s.db[1995][12]) * s.v[1959]) + (eq55_e1379 * s.db[1959][12]));
        let eq55_e1381_d_b13: f64 = (((-s.db[1995][13]) * s.v[1959]) + (eq55_e1379 * s.db[1959][13]));
        let eq55_e1381_d_b14: f64 = (((-s.db[1995][14]) * s.v[1959]) + (eq55_e1379 * s.db[1959][14]));
        let eq55_e1381_d_b15: f64 = (((-s.db[1995][15]) * s.v[1959]) + (eq55_e1379 * s.db[1959][15]));
        let eq55_e1381_d_b16: f64 = (((-s.db[1995][16]) * s.v[1959]) + (eq55_e1379 * s.db[1959][16]));
        let eq55_e1381_d_b17: f64 = (((-s.db[1995][17]) * s.v[1959]) + (eq55_e1379 * s.db[1959][17]));
        let eq55_e1381_d_b18: f64 = (((-s.db[1995][18]) * s.v[1959]) + (eq55_e1379 * s.db[1959][18]));
        let eq55_e1381_d_b19: f64 = (((-s.db[1995][19]) * s.v[1959]) + (eq55_e1379 * s.db[1959][19]));
        let eq55_e1381_d_b20: f64 = (((-s.db[1995][20]) * s.v[1959]) + (eq55_e1379 * s.db[1959][20]));
        let eq55_e1381_d_b21: f64 = (((-s.db[1995][21]) * s.v[1959]) + (eq55_e1379 * s.db[1959][21]));
        let eq55_e1381_d_b22: f64 = (((-s.db[1995][22]) * s.v[1959]) + (eq55_e1379 * s.db[1959][22]));
        let eq55_e1381_d_b23: f64 = (((-s.db[1995][23]) * s.v[1959]) + (eq55_e1379 * s.db[1959][23]));
        let eq55_e1381_d_b24: f64 = (((-s.db[1995][24]) * s.v[1959]) + (eq55_e1379 * s.db[1959][24]));
        let eq55_e1383: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 8, eq55_e1381, s.v[1950]);
        let eq55_e1384: f64 = (s.v[4] * eq55_e1383);
        let eq55_e1384_d_n0: f64 = (s.v[4] * (eq55_e1381_d_n0 * idt_scale));
        let eq55_e1384_d_n1: f64 = (s.v[4] * (eq55_e1381_d_n1 * idt_scale));
        let eq55_e1384_d_n2: f64 = (s.v[4] * (eq55_e1381_d_n2 * idt_scale));
        let eq55_e1384_d_n3: f64 = (s.v[4] * (eq55_e1381_d_n3 * idt_scale));
        let eq55_e1384_d_n4: f64 = (s.v[4] * (eq55_e1381_d_n4 * idt_scale));
        let eq55_e1384_d_n5: f64 = (s.v[4] * (eq55_e1381_d_n5 * idt_scale));
        let eq55_e1384_d_n6: f64 = (s.v[4] * (eq55_e1381_d_n6 * idt_scale));
        let eq55_e1384_d_n7: f64 = (s.v[4] * (eq55_e1381_d_n7 * idt_scale));
        let eq55_e1384_d_n8: f64 = (s.v[4] * (eq55_e1381_d_n8 * idt_scale));
        let eq55_e1384_d_n9: f64 = (s.v[4] * (eq55_e1381_d_n9 * idt_scale));
        let eq55_e1384_d_n10: f64 = (s.v[4] * (eq55_e1381_d_n10 * idt_scale));
        let eq55_e1384_d_n11: f64 = (s.v[4] * (eq55_e1381_d_n11 * idt_scale));
        let eq55_e1384_d_n12: f64 = (s.v[4] * (eq55_e1381_d_n12 * idt_scale));
        let eq55_e1384_d_n13: f64 = (s.v[4] * (eq55_e1381_d_n13 * idt_scale));
        let eq55_e1384_d_n14: f64 = (s.v[4] * (eq55_e1381_d_n14 * idt_scale));
        let eq55_e1384_d_n15: f64 = (s.v[4] * (eq55_e1381_d_n15 * idt_scale));
        let eq55_e1384_d_n16: f64 = (s.v[4] * (eq55_e1381_d_n16 * idt_scale));
        let eq55_e1384_d_n17: f64 = (s.v[4] * (eq55_e1381_d_n17 * idt_scale));
        let eq55_e1384_d_n18: f64 = (s.v[4] * (eq55_e1381_d_n18 * idt_scale));
        let eq55_e1384_d_n19: f64 = (s.v[4] * (eq55_e1381_d_n19 * idt_scale));
        let eq55_e1384_d_n20: f64 = (s.v[4] * (eq55_e1381_d_n20 * idt_scale));
        let eq55_e1384_d_b0: f64 = (s.v[4] * (eq55_e1381_d_b0 * idt_scale));
        let eq55_e1384_d_b1: f64 = (s.v[4] * (eq55_e1381_d_b1 * idt_scale));
        let eq55_e1384_d_b2: f64 = (s.v[4] * (eq55_e1381_d_b2 * idt_scale));
        let eq55_e1384_d_b3: f64 = (s.v[4] * (eq55_e1381_d_b3 * idt_scale));
        let eq55_e1384_d_b4: f64 = (s.v[4] * (eq55_e1381_d_b4 * idt_scale));
        let eq55_e1384_d_b5: f64 = (s.v[4] * (eq55_e1381_d_b5 * idt_scale));
        let eq55_e1384_d_b6: f64 = (s.v[4] * (eq55_e1381_d_b6 * idt_scale));
        let eq55_e1384_d_b7: f64 = (s.v[4] * (eq55_e1381_d_b7 * idt_scale));
        let eq55_e1384_d_b8: f64 = (s.v[4] * (eq55_e1381_d_b8 * idt_scale));
        let eq55_e1384_d_b9: f64 = (s.v[4] * (eq55_e1381_d_b9 * idt_scale));
        let eq55_e1384_d_b10: f64 = (s.v[4] * (eq55_e1381_d_b10 * idt_scale));
        let eq55_e1384_d_b11: f64 = (s.v[4] * (eq55_e1381_d_b11 * idt_scale));
        let eq55_e1384_d_b12: f64 = (s.v[4] * (eq55_e1381_d_b12 * idt_scale));
        let eq55_e1384_d_b13: f64 = (s.v[4] * (eq55_e1381_d_b13 * idt_scale));
        let eq55_e1384_d_b14: f64 = (s.v[4] * (eq55_e1381_d_b14 * idt_scale));
        let eq55_e1384_d_b15: f64 = (s.v[4] * (eq55_e1381_d_b15 * idt_scale));
        let eq55_e1384_d_b16: f64 = (s.v[4] * (eq55_e1381_d_b16 * idt_scale));
        let eq55_e1384_d_b17: f64 = (s.v[4] * (eq55_e1381_d_b17 * idt_scale));
        let eq55_e1384_d_b18: f64 = (s.v[4] * (eq55_e1381_d_b18 * idt_scale));
        let eq55_e1384_d_b19: f64 = (s.v[4] * (eq55_e1381_d_b19 * idt_scale));
        let eq55_e1384_d_b20: f64 = (s.v[4] * (eq55_e1381_d_b20 * idt_scale));
        let eq55_e1384_d_b21: f64 = (s.v[4] * (eq55_e1381_d_b21 * idt_scale));
        let eq55_e1384_d_b22: f64 = (s.v[4] * (eq55_e1381_d_b22 * idt_scale));
        let eq55_e1384_d_b23: f64 = (s.v[4] * (eq55_e1381_d_b23 * idt_scale));
        let eq55_e1384_d_b24: f64 = (s.v[4] * (eq55_e1381_d_b24 * idt_scale));
        let eq55_value: f64 = eq55_e1384;
        let eq55_node_derivatives: [f64; 21] = [eq55_e1384_d_n0, eq55_e1384_d_n1, eq55_e1384_d_n2, eq55_e1384_d_n3, eq55_e1384_d_n4, eq55_e1384_d_n5, eq55_e1384_d_n6, eq55_e1384_d_n7, eq55_e1384_d_n8, eq55_e1384_d_n9, eq55_e1384_d_n10, eq55_e1384_d_n11, eq55_e1384_d_n12, eq55_e1384_d_n13, eq55_e1384_d_n14, eq55_e1384_d_n15, eq55_e1384_d_n16, eq55_e1384_d_n17, eq55_e1384_d_n18, eq55_e1384_d_n19, eq55_e1384_d_n20];
        let eq55_branch_derivatives: [f64; 25] = [eq55_e1384_d_b0, eq55_e1384_d_b1, eq55_e1384_d_b2, eq55_e1384_d_b3, eq55_e1384_d_b4, eq55_e1384_d_b5, eq55_e1384_d_b6, eq55_e1384_d_b7, eq55_e1384_d_b8, eq55_e1384_d_b9, eq55_e1384_d_b10, eq55_e1384_d_b11, eq55_e1384_d_b12, eq55_e1384_d_b13, eq55_e1384_d_b14, eq55_e1384_d_b15, eq55_e1384_d_b16, eq55_e1384_d_b17, eq55_e1384_d_b18, eq55_e1384_d_b19, eq55_e1384_d_b20, eq55_e1384_d_b21, eq55_e1384_d_b22, eq55_e1384_d_b23, eq55_e1384_d_b24];
        stamper.stamp_potential_dense_local(
            24,
            eq55_value,
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
        );
        let eq56_e1387: f64 = (var_chnl_type * var_mult_inst);
        let eq56_e1389: f64 = (eq56_e1387 * p.p33);
        let eq56_e1391: f64 = (eq56_e1389 * s.v[851]);
        let eq56_e1392: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq56_e1391);
        let eq56_e1392_d_n0: f64 = ((eq56_e1389 * s.dn[851][0]) * ddt_scale);
        let eq56_e1392_d_n1: f64 = ((eq56_e1389 * s.dn[851][1]) * ddt_scale);
        let eq56_e1392_d_n2: f64 = ((eq56_e1389 * s.dn[851][2]) * ddt_scale);
        let eq56_e1392_d_n3: f64 = ((eq56_e1389 * s.dn[851][3]) * ddt_scale);
        let eq56_e1392_d_n4: f64 = ((eq56_e1389 * s.dn[851][4]) * ddt_scale);
        let eq56_e1392_d_n5: f64 = ((eq56_e1389 * s.dn[851][5]) * ddt_scale);
        let eq56_e1392_d_n6: f64 = ((eq56_e1389 * s.dn[851][6]) * ddt_scale);
        let eq56_e1392_d_n7: f64 = ((eq56_e1389 * s.dn[851][7]) * ddt_scale);
        let eq56_e1392_d_n8: f64 = ((eq56_e1389 * s.dn[851][8]) * ddt_scale);
        let eq56_e1392_d_n9: f64 = ((eq56_e1389 * s.dn[851][9]) * ddt_scale);
        let eq56_e1392_d_n10: f64 = ((eq56_e1389 * s.dn[851][10]) * ddt_scale);
        let eq56_e1392_d_n11: f64 = ((eq56_e1389 * s.dn[851][11]) * ddt_scale);
        let eq56_e1392_d_n12: f64 = ((eq56_e1389 * s.dn[851][12]) * ddt_scale);
        let eq56_e1392_d_n13: f64 = ((eq56_e1389 * s.dn[851][13]) * ddt_scale);
        let eq56_e1392_d_n14: f64 = ((eq56_e1389 * s.dn[851][14]) * ddt_scale);
        let eq56_e1392_d_n15: f64 = ((eq56_e1389 * s.dn[851][15]) * ddt_scale);
        let eq56_e1392_d_n16: f64 = ((eq56_e1389 * s.dn[851][16]) * ddt_scale);
        let eq56_e1392_d_n17: f64 = ((eq56_e1389 * s.dn[851][17]) * ddt_scale);
        let eq56_e1392_d_n18: f64 = ((eq56_e1389 * s.dn[851][18]) * ddt_scale);
        let eq56_e1392_d_n19: f64 = ((eq56_e1389 * s.dn[851][19]) * ddt_scale);
        let eq56_e1392_d_n20: f64 = ((eq56_e1389 * s.dn[851][20]) * ddt_scale);
        let eq56_e1392_d_b0: f64 = ((eq56_e1389 * s.db[851][0]) * ddt_scale);
        let eq56_e1392_d_b1: f64 = ((eq56_e1389 * s.db[851][1]) * ddt_scale);
        let eq56_e1392_d_b2: f64 = ((eq56_e1389 * s.db[851][2]) * ddt_scale);
        let eq56_e1392_d_b3: f64 = ((eq56_e1389 * s.db[851][3]) * ddt_scale);
        let eq56_e1392_d_b4: f64 = ((eq56_e1389 * s.db[851][4]) * ddt_scale);
        let eq56_e1392_d_b5: f64 = ((eq56_e1389 * s.db[851][5]) * ddt_scale);
        let eq56_e1392_d_b6: f64 = ((eq56_e1389 * s.db[851][6]) * ddt_scale);
        let eq56_e1392_d_b7: f64 = ((eq56_e1389 * s.db[851][7]) * ddt_scale);
        let eq56_e1392_d_b8: f64 = ((eq56_e1389 * s.db[851][8]) * ddt_scale);
        let eq56_e1392_d_b9: f64 = ((eq56_e1389 * s.db[851][9]) * ddt_scale);
        let eq56_e1392_d_b10: f64 = ((eq56_e1389 * s.db[851][10]) * ddt_scale);
        let eq56_e1392_d_b11: f64 = ((eq56_e1389 * s.db[851][11]) * ddt_scale);
        let eq56_e1392_d_b12: f64 = ((eq56_e1389 * s.db[851][12]) * ddt_scale);
        let eq56_e1392_d_b13: f64 = ((eq56_e1389 * s.db[851][13]) * ddt_scale);
        let eq56_e1392_d_b14: f64 = ((eq56_e1389 * s.db[851][14]) * ddt_scale);
        let eq56_e1392_d_b15: f64 = ((eq56_e1389 * s.db[851][15]) * ddt_scale);
        let eq56_e1392_d_b16: f64 = ((eq56_e1389 * s.db[851][16]) * ddt_scale);
        let eq56_e1392_d_b17: f64 = ((eq56_e1389 * s.db[851][17]) * ddt_scale);
        let eq56_e1392_d_b18: f64 = ((eq56_e1389 * s.db[851][18]) * ddt_scale);
        let eq56_e1392_d_b19: f64 = ((eq56_e1389 * s.db[851][19]) * ddt_scale);
        let eq56_e1392_d_b20: f64 = ((eq56_e1389 * s.db[851][20]) * ddt_scale);
        let eq56_e1392_d_b21: f64 = ((eq56_e1389 * s.db[851][21]) * ddt_scale);
        let eq56_e1392_d_b22: f64 = ((eq56_e1389 * s.db[851][22]) * ddt_scale);
        let eq56_e1392_d_b23: f64 = ((eq56_e1389 * s.db[851][23]) * ddt_scale);
        let eq56_e1392_d_b24: f64 = ((eq56_e1389 * s.db[851][24]) * ddt_scale);
        let eq56_value: f64 = eq56_e1392;
        let eq56_node_derivatives: [f64; 21] = [eq56_e1392_d_n0, eq56_e1392_d_n1, eq56_e1392_d_n2, eq56_e1392_d_n3, eq56_e1392_d_n4, eq56_e1392_d_n5, eq56_e1392_d_n6, eq56_e1392_d_n7, eq56_e1392_d_n8, eq56_e1392_d_n9, eq56_e1392_d_n10, eq56_e1392_d_n11, eq56_e1392_d_n12, eq56_e1392_d_n13, eq56_e1392_d_n14, eq56_e1392_d_n15, eq56_e1392_d_n16, eq56_e1392_d_n17, eq56_e1392_d_n18, eq56_e1392_d_n19, eq56_e1392_d_n20];
        let eq56_branch_derivatives: [f64; 25] = [eq56_e1392_d_b0, eq56_e1392_d_b1, eq56_e1392_d_b2, eq56_e1392_d_b3, eq56_e1392_d_b4, eq56_e1392_d_b5, eq56_e1392_d_b6, eq56_e1392_d_b7, eq56_e1392_d_b8, eq56_e1392_d_b9, eq56_e1392_d_b10, eq56_e1392_d_b11, eq56_e1392_d_b12, eq56_e1392_d_b13, eq56_e1392_d_b14, eq56_e1392_d_b15, eq56_e1392_d_b16, eq56_e1392_d_b17, eq56_e1392_d_b18, eq56_e1392_d_b19, eq56_e1392_d_b20, eq56_e1392_d_b21, eq56_e1392_d_b22, eq56_e1392_d_b23, eq56_e1392_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let eq57_e1395: f64 = (var_chnl_type * var_mult_inst);
        let eq57_e1397: f64 = (eq57_e1395 * p.p33);
        let eq57_e1399: f64 = (eq57_e1397 * s.v[852]);
        let eq57_e1400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq57_e1399);
        let eq57_e1400_d_n0: f64 = ((eq57_e1397 * s.dn[852][0]) * ddt_scale);
        let eq57_e1400_d_n1: f64 = ((eq57_e1397 * s.dn[852][1]) * ddt_scale);
        let eq57_e1400_d_n2: f64 = ((eq57_e1397 * s.dn[852][2]) * ddt_scale);
        let eq57_e1400_d_n3: f64 = ((eq57_e1397 * s.dn[852][3]) * ddt_scale);
        let eq57_e1400_d_n4: f64 = ((eq57_e1397 * s.dn[852][4]) * ddt_scale);
        let eq57_e1400_d_n5: f64 = ((eq57_e1397 * s.dn[852][5]) * ddt_scale);
        let eq57_e1400_d_n6: f64 = ((eq57_e1397 * s.dn[852][6]) * ddt_scale);
        let eq57_e1400_d_n7: f64 = ((eq57_e1397 * s.dn[852][7]) * ddt_scale);
        let eq57_e1400_d_n8: f64 = ((eq57_e1397 * s.dn[852][8]) * ddt_scale);
        let eq57_e1400_d_n9: f64 = ((eq57_e1397 * s.dn[852][9]) * ddt_scale);
        let eq57_e1400_d_n10: f64 = ((eq57_e1397 * s.dn[852][10]) * ddt_scale);
        let eq57_e1400_d_n11: f64 = ((eq57_e1397 * s.dn[852][11]) * ddt_scale);
        let eq57_e1400_d_n12: f64 = ((eq57_e1397 * s.dn[852][12]) * ddt_scale);
        let eq57_e1400_d_n13: f64 = ((eq57_e1397 * s.dn[852][13]) * ddt_scale);
        let eq57_e1400_d_n14: f64 = ((eq57_e1397 * s.dn[852][14]) * ddt_scale);
        let eq57_e1400_d_n15: f64 = ((eq57_e1397 * s.dn[852][15]) * ddt_scale);
        let eq57_e1400_d_n16: f64 = ((eq57_e1397 * s.dn[852][16]) * ddt_scale);
        let eq57_e1400_d_n17: f64 = ((eq57_e1397 * s.dn[852][17]) * ddt_scale);
        let eq57_e1400_d_n18: f64 = ((eq57_e1397 * s.dn[852][18]) * ddt_scale);
        let eq57_e1400_d_n19: f64 = ((eq57_e1397 * s.dn[852][19]) * ddt_scale);
        let eq57_e1400_d_n20: f64 = ((eq57_e1397 * s.dn[852][20]) * ddt_scale);
        let eq57_e1400_d_b0: f64 = ((eq57_e1397 * s.db[852][0]) * ddt_scale);
        let eq57_e1400_d_b1: f64 = ((eq57_e1397 * s.db[852][1]) * ddt_scale);
        let eq57_e1400_d_b2: f64 = ((eq57_e1397 * s.db[852][2]) * ddt_scale);
        let eq57_e1400_d_b3: f64 = ((eq57_e1397 * s.db[852][3]) * ddt_scale);
        let eq57_e1400_d_b4: f64 = ((eq57_e1397 * s.db[852][4]) * ddt_scale);
        let eq57_e1400_d_b5: f64 = ((eq57_e1397 * s.db[852][5]) * ddt_scale);
        let eq57_e1400_d_b6: f64 = ((eq57_e1397 * s.db[852][6]) * ddt_scale);
        let eq57_e1400_d_b7: f64 = ((eq57_e1397 * s.db[852][7]) * ddt_scale);
        let eq57_e1400_d_b8: f64 = ((eq57_e1397 * s.db[852][8]) * ddt_scale);
        let eq57_e1400_d_b9: f64 = ((eq57_e1397 * s.db[852][9]) * ddt_scale);
        let eq57_e1400_d_b10: f64 = ((eq57_e1397 * s.db[852][10]) * ddt_scale);
        let eq57_e1400_d_b11: f64 = ((eq57_e1397 * s.db[852][11]) * ddt_scale);
        let eq57_e1400_d_b12: f64 = ((eq57_e1397 * s.db[852][12]) * ddt_scale);
        let eq57_e1400_d_b13: f64 = ((eq57_e1397 * s.db[852][13]) * ddt_scale);
        let eq57_e1400_d_b14: f64 = ((eq57_e1397 * s.db[852][14]) * ddt_scale);
        let eq57_e1400_d_b15: f64 = ((eq57_e1397 * s.db[852][15]) * ddt_scale);
        let eq57_e1400_d_b16: f64 = ((eq57_e1397 * s.db[852][16]) * ddt_scale);
        let eq57_e1400_d_b17: f64 = ((eq57_e1397 * s.db[852][17]) * ddt_scale);
        let eq57_e1400_d_b18: f64 = ((eq57_e1397 * s.db[852][18]) * ddt_scale);
        let eq57_e1400_d_b19: f64 = ((eq57_e1397 * s.db[852][19]) * ddt_scale);
        let eq57_e1400_d_b20: f64 = ((eq57_e1397 * s.db[852][20]) * ddt_scale);
        let eq57_e1400_d_b21: f64 = ((eq57_e1397 * s.db[852][21]) * ddt_scale);
        let eq57_e1400_d_b22: f64 = ((eq57_e1397 * s.db[852][22]) * ddt_scale);
        let eq57_e1400_d_b23: f64 = ((eq57_e1397 * s.db[852][23]) * ddt_scale);
        let eq57_e1400_d_b24: f64 = ((eq57_e1397 * s.db[852][24]) * ddt_scale);
        let eq57_value: f64 = eq57_e1400;
        let eq57_node_derivatives: [f64; 21] = [eq57_e1400_d_n0, eq57_e1400_d_n1, eq57_e1400_d_n2, eq57_e1400_d_n3, eq57_e1400_d_n4, eq57_e1400_d_n5, eq57_e1400_d_n6, eq57_e1400_d_n7, eq57_e1400_d_n8, eq57_e1400_d_n9, eq57_e1400_d_n10, eq57_e1400_d_n11, eq57_e1400_d_n12, eq57_e1400_d_n13, eq57_e1400_d_n14, eq57_e1400_d_n15, eq57_e1400_d_n16, eq57_e1400_d_n17, eq57_e1400_d_n18, eq57_e1400_d_n19, eq57_e1400_d_n20];
        let eq57_branch_derivatives: [f64; 25] = [eq57_e1400_d_b0, eq57_e1400_d_b1, eq57_e1400_d_b2, eq57_e1400_d_b3, eq57_e1400_d_b4, eq57_e1400_d_b5, eq57_e1400_d_b6, eq57_e1400_d_b7, eq57_e1400_d_b8, eq57_e1400_d_b9, eq57_e1400_d_b10, eq57_e1400_d_b11, eq57_e1400_d_b12, eq57_e1400_d_b13, eq57_e1400_d_b14, eq57_e1400_d_b15, eq57_e1400_d_b16, eq57_e1400_d_b17, eq57_e1400_d_b18, eq57_e1400_d_b19, eq57_e1400_d_b20, eq57_e1400_d_b21, eq57_e1400_d_b22, eq57_e1400_d_b23, eq57_e1400_d_b24];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let eq58_e1403: f64 = (var_chnl_type * var_mult_inst);
        let eq58_e1405: f64 = (eq58_e1403 * p.p33);
        let eq58_e1407: f64 = (eq58_e1405 * s.v[853]);
        let eq58_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq58_e1407);
        let eq58_e1408_d_n0: f64 = ((eq58_e1405 * s.dn[853][0]) * ddt_scale);
        let eq58_e1408_d_n1: f64 = ((eq58_e1405 * s.dn[853][1]) * ddt_scale);
        let eq58_e1408_d_n2: f64 = ((eq58_e1405 * s.dn[853][2]) * ddt_scale);
        let eq58_e1408_d_n3: f64 = ((eq58_e1405 * s.dn[853][3]) * ddt_scale);
        let eq58_e1408_d_n4: f64 = ((eq58_e1405 * s.dn[853][4]) * ddt_scale);
        let eq58_e1408_d_n5: f64 = ((eq58_e1405 * s.dn[853][5]) * ddt_scale);
        let eq58_e1408_d_n6: f64 = ((eq58_e1405 * s.dn[853][6]) * ddt_scale);
        let eq58_e1408_d_n7: f64 = ((eq58_e1405 * s.dn[853][7]) * ddt_scale);
        let eq58_e1408_d_n8: f64 = ((eq58_e1405 * s.dn[853][8]) * ddt_scale);
        let eq58_e1408_d_n9: f64 = ((eq58_e1405 * s.dn[853][9]) * ddt_scale);
        let eq58_e1408_d_n10: f64 = ((eq58_e1405 * s.dn[853][10]) * ddt_scale);
        let eq58_e1408_d_n11: f64 = ((eq58_e1405 * s.dn[853][11]) * ddt_scale);
        let eq58_e1408_d_n12: f64 = ((eq58_e1405 * s.dn[853][12]) * ddt_scale);
        let eq58_e1408_d_n13: f64 = ((eq58_e1405 * s.dn[853][13]) * ddt_scale);
        let eq58_e1408_d_n14: f64 = ((eq58_e1405 * s.dn[853][14]) * ddt_scale);
        let eq58_e1408_d_n15: f64 = ((eq58_e1405 * s.dn[853][15]) * ddt_scale);
        let eq58_e1408_d_n16: f64 = ((eq58_e1405 * s.dn[853][16]) * ddt_scale);
        let eq58_e1408_d_n17: f64 = ((eq58_e1405 * s.dn[853][17]) * ddt_scale);
        let eq58_e1408_d_n18: f64 = ((eq58_e1405 * s.dn[853][18]) * ddt_scale);
        let eq58_e1408_d_n19: f64 = ((eq58_e1405 * s.dn[853][19]) * ddt_scale);
        let eq58_e1408_d_n20: f64 = ((eq58_e1405 * s.dn[853][20]) * ddt_scale);
        let eq58_e1408_d_b0: f64 = ((eq58_e1405 * s.db[853][0]) * ddt_scale);
        let eq58_e1408_d_b1: f64 = ((eq58_e1405 * s.db[853][1]) * ddt_scale);
        let eq58_e1408_d_b2: f64 = ((eq58_e1405 * s.db[853][2]) * ddt_scale);
        let eq58_e1408_d_b3: f64 = ((eq58_e1405 * s.db[853][3]) * ddt_scale);
        let eq58_e1408_d_b4: f64 = ((eq58_e1405 * s.db[853][4]) * ddt_scale);
        let eq58_e1408_d_b5: f64 = ((eq58_e1405 * s.db[853][5]) * ddt_scale);
        let eq58_e1408_d_b6: f64 = ((eq58_e1405 * s.db[853][6]) * ddt_scale);
        let eq58_e1408_d_b7: f64 = ((eq58_e1405 * s.db[853][7]) * ddt_scale);
        let eq58_e1408_d_b8: f64 = ((eq58_e1405 * s.db[853][8]) * ddt_scale);
        let eq58_e1408_d_b9: f64 = ((eq58_e1405 * s.db[853][9]) * ddt_scale);
        let eq58_e1408_d_b10: f64 = ((eq58_e1405 * s.db[853][10]) * ddt_scale);
        let eq58_e1408_d_b11: f64 = ((eq58_e1405 * s.db[853][11]) * ddt_scale);
        let eq58_e1408_d_b12: f64 = ((eq58_e1405 * s.db[853][12]) * ddt_scale);
        let eq58_e1408_d_b13: f64 = ((eq58_e1405 * s.db[853][13]) * ddt_scale);
        let eq58_e1408_d_b14: f64 = ((eq58_e1405 * s.db[853][14]) * ddt_scale);
        let eq58_e1408_d_b15: f64 = ((eq58_e1405 * s.db[853][15]) * ddt_scale);
        let eq58_e1408_d_b16: f64 = ((eq58_e1405 * s.db[853][16]) * ddt_scale);
        let eq58_e1408_d_b17: f64 = ((eq58_e1405 * s.db[853][17]) * ddt_scale);
        let eq58_e1408_d_b18: f64 = ((eq58_e1405 * s.db[853][18]) * ddt_scale);
        let eq58_e1408_d_b19: f64 = ((eq58_e1405 * s.db[853][19]) * ddt_scale);
        let eq58_e1408_d_b20: f64 = ((eq58_e1405 * s.db[853][20]) * ddt_scale);
        let eq58_e1408_d_b21: f64 = ((eq58_e1405 * s.db[853][21]) * ddt_scale);
        let eq58_e1408_d_b22: f64 = ((eq58_e1405 * s.db[853][22]) * ddt_scale);
        let eq58_e1408_d_b23: f64 = ((eq58_e1405 * s.db[853][23]) * ddt_scale);
        let eq58_e1408_d_b24: f64 = ((eq58_e1405 * s.db[853][24]) * ddt_scale);
        let eq58_value: f64 = eq58_e1408;
        let eq58_node_derivatives: [f64; 21] = [eq58_e1408_d_n0, eq58_e1408_d_n1, eq58_e1408_d_n2, eq58_e1408_d_n3, eq58_e1408_d_n4, eq58_e1408_d_n5, eq58_e1408_d_n6, eq58_e1408_d_n7, eq58_e1408_d_n8, eq58_e1408_d_n9, eq58_e1408_d_n10, eq58_e1408_d_n11, eq58_e1408_d_n12, eq58_e1408_d_n13, eq58_e1408_d_n14, eq58_e1408_d_n15, eq58_e1408_d_n16, eq58_e1408_d_n17, eq58_e1408_d_n18, eq58_e1408_d_n19, eq58_e1408_d_n20];
        let eq58_branch_derivatives: [f64; 25] = [eq58_e1408_d_b0, eq58_e1408_d_b1, eq58_e1408_d_b2, eq58_e1408_d_b3, eq58_e1408_d_b4, eq58_e1408_d_b5, eq58_e1408_d_b6, eq58_e1408_d_b7, eq58_e1408_d_b8, eq58_e1408_d_b9, eq58_e1408_d_b10, eq58_e1408_d_b11, eq58_e1408_d_b12, eq58_e1408_d_b13, eq58_e1408_d_b14, eq58_e1408_d_b15, eq58_e1408_d_b16, eq58_e1408_d_b17, eq58_e1408_d_b18, eq58_e1408_d_b19, eq58_e1408_d_b20, eq58_e1408_d_b21, eq58_e1408_d_b22, eq58_e1408_d_b23, eq58_e1408_d_b24];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        var_chnl_type: f64,
        var_mult_inst: f64,
        var_qfgd: f64,
        var_qfgd_db0: f64,
        var_qfgd_db1: f64,
        var_qfgd_db10: f64,
        var_qfgd_db11: f64,
        var_qfgd_db12: f64,
        var_qfgd_db13: f64,
        var_qfgd_db14: f64,
        var_qfgd_db15: f64,
        var_qfgd_db16: f64,
        var_qfgd_db17: f64,
        var_qfgd_db18: f64,
        var_qfgd_db19: f64,
        var_qfgd_db2: f64,
        var_qfgd_db20: f64,
        var_qfgd_db21: f64,
        var_qfgd_db22: f64,
        var_qfgd_db23: f64,
        var_qfgd_db24: f64,
        var_qfgd_db3: f64,
        var_qfgd_db4: f64,
        var_qfgd_db5: f64,
        var_qfgd_db6: f64,
        var_qfgd_db7: f64,
        var_qfgd_db8: f64,
        var_qfgd_db9: f64,
        var_qfgd_dn0: f64,
        var_qfgd_dn1: f64,
        var_qfgd_dn10: f64,
        var_qfgd_dn11: f64,
        var_qfgd_dn12: f64,
        var_qfgd_dn13: f64,
        var_qfgd_dn14: f64,
        var_qfgd_dn15: f64,
        var_qfgd_dn16: f64,
        var_qfgd_dn17: f64,
        var_qfgd_dn18: f64,
        var_qfgd_dn19: f64,
        var_qfgd_dn2: f64,
        var_qfgd_dn20: f64,
        var_qfgd_dn3: f64,
        var_qfgd_dn4: f64,
        var_qfgd_dn5: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgd_dn8: f64,
        var_qfgd_dn9: f64,
        var_qfgs: f64,
        var_qfgs_db0: f64,
        var_qfgs_db1: f64,
        var_qfgs_db10: f64,
        var_qfgs_db11: f64,
        var_qfgs_db12: f64,
        var_qfgs_db13: f64,
        var_qfgs_db14: f64,
        var_qfgs_db15: f64,
        var_qfgs_db16: f64,
        var_qfgs_db17: f64,
        var_qfgs_db18: f64,
        var_qfgs_db19: f64,
        var_qfgs_db2: f64,
        var_qfgs_db20: f64,
        var_qfgs_db21: f64,
        var_qfgs_db22: f64,
        var_qfgs_db23: f64,
        var_qfgs_db24: f64,
        var_qfgs_db3: f64,
        var_qfgs_db4: f64,
        var_qfgs_db5: f64,
        var_qfgs_db6: f64,
        var_qfgs_db7: f64,
        var_qfgs_db8: f64,
        var_qfgs_db9: f64,
        var_qfgs_dn0: f64,
        var_qfgs_dn1: f64,
        var_qfgs_dn10: f64,
        var_qfgs_dn11: f64,
        var_qfgs_dn12: f64,
        var_qfgs_dn13: f64,
        var_qfgs_dn14: f64,
        var_qfgs_dn15: f64,
        var_qfgs_dn16: f64,
        var_qfgs_dn17: f64,
        var_qfgs_dn18: f64,
        var_qfgs_dn19: f64,
        var_qfgs_dn2: f64,
        var_qfgs_dn20: f64,
        var_qfgs_dn3: f64,
        var_qfgs_dn4: f64,
        var_qfgs_dn5: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qfgs_dn8: f64,
        var_qfgs_dn9: f64,
        var_qjun_d: f64,
        var_qjun_d_db0: f64,
        var_qjun_d_db1: f64,
        var_qjun_d_db10: f64,
        var_qjun_d_db11: f64,
        var_qjun_d_db12: f64,
        var_qjun_d_db13: f64,
        var_qjun_d_db14: f64,
        var_qjun_d_db15: f64,
        var_qjun_d_db16: f64,
        var_qjun_d_db17: f64,
        var_qjun_d_db18: f64,
        var_qjun_d_db19: f64,
        var_qjun_d_db2: f64,
        var_qjun_d_db20: f64,
        var_qjun_d_db21: f64,
        var_qjun_d_db22: f64,
        var_qjun_d_db23: f64,
        var_qjun_d_db24: f64,
        var_qjun_d_db3: f64,
        var_qjun_d_db4: f64,
        var_qjun_d_db5: f64,
        var_qjun_d_db6: f64,
        var_qjun_d_db7: f64,
        var_qjun_d_db8: f64,
        var_qjun_d_db9: f64,
        var_qjun_d_dn0: f64,
        var_qjun_d_dn1: f64,
        var_qjun_d_dn10: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn12: f64,
        var_qjun_d_dn13: f64,
        var_qjun_d_dn14: f64,
        var_qjun_d_dn15: f64,
        var_qjun_d_dn16: f64,
        var_qjun_d_dn17: f64,
        var_qjun_d_dn18: f64,
        var_qjun_d_dn19: f64,
        var_qjun_d_dn2: f64,
        var_qjun_d_dn20: f64,
        var_qjun_d_dn3: f64,
        var_qjun_d_dn4: f64,
        var_qjun_d_dn5: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_d_dn9: f64,
        var_qjun_s: f64,
        var_qjun_s_db0: f64,
        var_qjun_s_db1: f64,
        var_qjun_s_db10: f64,
        var_qjun_s_db11: f64,
        var_qjun_s_db12: f64,
        var_qjun_s_db13: f64,
        var_qjun_s_db14: f64,
        var_qjun_s_db15: f64,
        var_qjun_s_db16: f64,
        var_qjun_s_db17: f64,
        var_qjun_s_db18: f64,
        var_qjun_s_db19: f64,
        var_qjun_s_db2: f64,
        var_qjun_s_db20: f64,
        var_qjun_s_db21: f64,
        var_qjun_s_db22: f64,
        var_qjun_s_db23: f64,
        var_qjun_s_db24: f64,
        var_qjun_s_db3: f64,
        var_qjun_s_db4: f64,
        var_qjun_s_db5: f64,
        var_qjun_s_db6: f64,
        var_qjun_s_db7: f64,
        var_qjun_s_db8: f64,
        var_qjun_s_db9: f64,
        var_qjun_s_dn0: f64,
        var_qjun_s_dn1: f64,
        var_qjun_s_dn10: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn12: f64,
        var_qjun_s_dn13: f64,
        var_qjun_s_dn14: f64,
        var_qjun_s_dn15: f64,
        var_qjun_s_dn16: f64,
        var_qjun_s_dn17: f64,
        var_qjun_s_dn18: f64,
        var_qjun_s_dn19: f64,
        var_qjun_s_dn2: f64,
        var_qjun_s_dn20: f64,
        var_qjun_s_dn3: f64,
        var_qjun_s_dn4: f64,
        var_qjun_s_dn5: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
        var_qjun_s_dn9: f64,
    ) {
        let eq59_e1411: f64 = (var_chnl_type * var_mult_inst);
        let eq59_e1413: f64 = (eq59_e1411 * p.p33);
        let eq59_e1415: f64 = (eq59_e1413 * var_qfgs);
        let eq59_e1415_d_n0: f64 = (eq59_e1413 * var_qfgs_dn0);
        let eq59_e1415_d_n1: f64 = (eq59_e1413 * var_qfgs_dn1);
        let eq59_e1415_d_n2: f64 = (eq59_e1413 * var_qfgs_dn2);
        let eq59_e1415_d_n3: f64 = (eq59_e1413 * var_qfgs_dn3);
        let eq59_e1415_d_n4: f64 = (eq59_e1413 * var_qfgs_dn4);
        let eq59_e1415_d_n5: f64 = (eq59_e1413 * var_qfgs_dn5);
        let eq59_e1415_d_n6: f64 = (eq59_e1413 * var_qfgs_dn6);
        let eq59_e1415_d_n7: f64 = (eq59_e1413 * var_qfgs_dn7);
        let eq59_e1415_d_n8: f64 = (eq59_e1413 * var_qfgs_dn8);
        let eq59_e1415_d_n9: f64 = (eq59_e1413 * var_qfgs_dn9);
        let eq59_e1415_d_n10: f64 = (eq59_e1413 * var_qfgs_dn10);
        let eq59_e1415_d_n11: f64 = (eq59_e1413 * var_qfgs_dn11);
        let eq59_e1415_d_n12: f64 = (eq59_e1413 * var_qfgs_dn12);
        let eq59_e1415_d_n13: f64 = (eq59_e1413 * var_qfgs_dn13);
        let eq59_e1415_d_n14: f64 = (eq59_e1413 * var_qfgs_dn14);
        let eq59_e1415_d_n15: f64 = (eq59_e1413 * var_qfgs_dn15);
        let eq59_e1415_d_n16: f64 = (eq59_e1413 * var_qfgs_dn16);
        let eq59_e1415_d_n17: f64 = (eq59_e1413 * var_qfgs_dn17);
        let eq59_e1415_d_n18: f64 = (eq59_e1413 * var_qfgs_dn18);
        let eq59_e1415_d_n19: f64 = (eq59_e1413 * var_qfgs_dn19);
        let eq59_e1415_d_n20: f64 = (eq59_e1413 * var_qfgs_dn20);
        let eq59_e1415_d_b0: f64 = (eq59_e1413 * var_qfgs_db0);
        let eq59_e1415_d_b1: f64 = (eq59_e1413 * var_qfgs_db1);
        let eq59_e1415_d_b2: f64 = (eq59_e1413 * var_qfgs_db2);
        let eq59_e1415_d_b3: f64 = (eq59_e1413 * var_qfgs_db3);
        let eq59_e1415_d_b4: f64 = (eq59_e1413 * var_qfgs_db4);
        let eq59_e1415_d_b5: f64 = (eq59_e1413 * var_qfgs_db5);
        let eq59_e1415_d_b6: f64 = (eq59_e1413 * var_qfgs_db6);
        let eq59_e1415_d_b7: f64 = (eq59_e1413 * var_qfgs_db7);
        let eq59_e1415_d_b8: f64 = (eq59_e1413 * var_qfgs_db8);
        let eq59_e1415_d_b9: f64 = (eq59_e1413 * var_qfgs_db9);
        let eq59_e1415_d_b10: f64 = (eq59_e1413 * var_qfgs_db10);
        let eq59_e1415_d_b11: f64 = (eq59_e1413 * var_qfgs_db11);
        let eq59_e1415_d_b12: f64 = (eq59_e1413 * var_qfgs_db12);
        let eq59_e1415_d_b13: f64 = (eq59_e1413 * var_qfgs_db13);
        let eq59_e1415_d_b14: f64 = (eq59_e1413 * var_qfgs_db14);
        let eq59_e1415_d_b15: f64 = (eq59_e1413 * var_qfgs_db15);
        let eq59_e1415_d_b16: f64 = (eq59_e1413 * var_qfgs_db16);
        let eq59_e1415_d_b17: f64 = (eq59_e1413 * var_qfgs_db17);
        let eq59_e1415_d_b18: f64 = (eq59_e1413 * var_qfgs_db18);
        let eq59_e1415_d_b19: f64 = (eq59_e1413 * var_qfgs_db19);
        let eq59_e1415_d_b20: f64 = (eq59_e1413 * var_qfgs_db20);
        let eq59_e1415_d_b21: f64 = (eq59_e1413 * var_qfgs_db21);
        let eq59_e1415_d_b22: f64 = (eq59_e1413 * var_qfgs_db22);
        let eq59_e1415_d_b23: f64 = (eq59_e1413 * var_qfgs_db23);
        let eq59_e1415_d_b24: f64 = (eq59_e1413 * var_qfgs_db24);
        let eq59_e1416: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq59_e1415);
        let eq59_value: f64 = eq59_e1416;
        let eq59_node_derivatives: [f64; 21] = [(eq59_e1415_d_n0 * ddt_scale), (eq59_e1415_d_n1 * ddt_scale), (eq59_e1415_d_n2 * ddt_scale), (eq59_e1415_d_n3 * ddt_scale), (eq59_e1415_d_n4 * ddt_scale), (eq59_e1415_d_n5 * ddt_scale), (eq59_e1415_d_n6 * ddt_scale), (eq59_e1415_d_n7 * ddt_scale), (eq59_e1415_d_n8 * ddt_scale), (eq59_e1415_d_n9 * ddt_scale), (eq59_e1415_d_n10 * ddt_scale), (eq59_e1415_d_n11 * ddt_scale), (eq59_e1415_d_n12 * ddt_scale), (eq59_e1415_d_n13 * ddt_scale), (eq59_e1415_d_n14 * ddt_scale), (eq59_e1415_d_n15 * ddt_scale), (eq59_e1415_d_n16 * ddt_scale), (eq59_e1415_d_n17 * ddt_scale), (eq59_e1415_d_n18 * ddt_scale), (eq59_e1415_d_n19 * ddt_scale), (eq59_e1415_d_n20 * ddt_scale)];
        let eq59_branch_derivatives: [f64; 25] = [(eq59_e1415_d_b0 * ddt_scale), (eq59_e1415_d_b1 * ddt_scale), (eq59_e1415_d_b2 * ddt_scale), (eq59_e1415_d_b3 * ddt_scale), (eq59_e1415_d_b4 * ddt_scale), (eq59_e1415_d_b5 * ddt_scale), (eq59_e1415_d_b6 * ddt_scale), (eq59_e1415_d_b7 * ddt_scale), (eq59_e1415_d_b8 * ddt_scale), (eq59_e1415_d_b9 * ddt_scale), (eq59_e1415_d_b10 * ddt_scale), (eq59_e1415_d_b11 * ddt_scale), (eq59_e1415_d_b12 * ddt_scale), (eq59_e1415_d_b13 * ddt_scale), (eq59_e1415_d_b14 * ddt_scale), (eq59_e1415_d_b15 * ddt_scale), (eq59_e1415_d_b16 * ddt_scale), (eq59_e1415_d_b17 * ddt_scale), (eq59_e1415_d_b18 * ddt_scale), (eq59_e1415_d_b19 * ddt_scale), (eq59_e1415_d_b20 * ddt_scale), (eq59_e1415_d_b21 * ddt_scale), (eq59_e1415_d_b22 * ddt_scale), (eq59_e1415_d_b23 * ddt_scale), (eq59_e1415_d_b24 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let eq60_e1419: f64 = (var_chnl_type * var_mult_inst);
        let eq60_e1421: f64 = (eq60_e1419 * p.p33);
        let eq60_e1423: f64 = (eq60_e1421 * var_qfgd);
        let eq60_e1423_d_n0: f64 = (eq60_e1421 * var_qfgd_dn0);
        let eq60_e1423_d_n1: f64 = (eq60_e1421 * var_qfgd_dn1);
        let eq60_e1423_d_n2: f64 = (eq60_e1421 * var_qfgd_dn2);
        let eq60_e1423_d_n3: f64 = (eq60_e1421 * var_qfgd_dn3);
        let eq60_e1423_d_n4: f64 = (eq60_e1421 * var_qfgd_dn4);
        let eq60_e1423_d_n5: f64 = (eq60_e1421 * var_qfgd_dn5);
        let eq60_e1423_d_n6: f64 = (eq60_e1421 * var_qfgd_dn6);
        let eq60_e1423_d_n7: f64 = (eq60_e1421 * var_qfgd_dn7);
        let eq60_e1423_d_n8: f64 = (eq60_e1421 * var_qfgd_dn8);
        let eq60_e1423_d_n9: f64 = (eq60_e1421 * var_qfgd_dn9);
        let eq60_e1423_d_n10: f64 = (eq60_e1421 * var_qfgd_dn10);
        let eq60_e1423_d_n11: f64 = (eq60_e1421 * var_qfgd_dn11);
        let eq60_e1423_d_n12: f64 = (eq60_e1421 * var_qfgd_dn12);
        let eq60_e1423_d_n13: f64 = (eq60_e1421 * var_qfgd_dn13);
        let eq60_e1423_d_n14: f64 = (eq60_e1421 * var_qfgd_dn14);
        let eq60_e1423_d_n15: f64 = (eq60_e1421 * var_qfgd_dn15);
        let eq60_e1423_d_n16: f64 = (eq60_e1421 * var_qfgd_dn16);
        let eq60_e1423_d_n17: f64 = (eq60_e1421 * var_qfgd_dn17);
        let eq60_e1423_d_n18: f64 = (eq60_e1421 * var_qfgd_dn18);
        let eq60_e1423_d_n19: f64 = (eq60_e1421 * var_qfgd_dn19);
        let eq60_e1423_d_n20: f64 = (eq60_e1421 * var_qfgd_dn20);
        let eq60_e1423_d_b0: f64 = (eq60_e1421 * var_qfgd_db0);
        let eq60_e1423_d_b1: f64 = (eq60_e1421 * var_qfgd_db1);
        let eq60_e1423_d_b2: f64 = (eq60_e1421 * var_qfgd_db2);
        let eq60_e1423_d_b3: f64 = (eq60_e1421 * var_qfgd_db3);
        let eq60_e1423_d_b4: f64 = (eq60_e1421 * var_qfgd_db4);
        let eq60_e1423_d_b5: f64 = (eq60_e1421 * var_qfgd_db5);
        let eq60_e1423_d_b6: f64 = (eq60_e1421 * var_qfgd_db6);
        let eq60_e1423_d_b7: f64 = (eq60_e1421 * var_qfgd_db7);
        let eq60_e1423_d_b8: f64 = (eq60_e1421 * var_qfgd_db8);
        let eq60_e1423_d_b9: f64 = (eq60_e1421 * var_qfgd_db9);
        let eq60_e1423_d_b10: f64 = (eq60_e1421 * var_qfgd_db10);
        let eq60_e1423_d_b11: f64 = (eq60_e1421 * var_qfgd_db11);
        let eq60_e1423_d_b12: f64 = (eq60_e1421 * var_qfgd_db12);
        let eq60_e1423_d_b13: f64 = (eq60_e1421 * var_qfgd_db13);
        let eq60_e1423_d_b14: f64 = (eq60_e1421 * var_qfgd_db14);
        let eq60_e1423_d_b15: f64 = (eq60_e1421 * var_qfgd_db15);
        let eq60_e1423_d_b16: f64 = (eq60_e1421 * var_qfgd_db16);
        let eq60_e1423_d_b17: f64 = (eq60_e1421 * var_qfgd_db17);
        let eq60_e1423_d_b18: f64 = (eq60_e1421 * var_qfgd_db18);
        let eq60_e1423_d_b19: f64 = (eq60_e1421 * var_qfgd_db19);
        let eq60_e1423_d_b20: f64 = (eq60_e1421 * var_qfgd_db20);
        let eq60_e1423_d_b21: f64 = (eq60_e1421 * var_qfgd_db21);
        let eq60_e1423_d_b22: f64 = (eq60_e1421 * var_qfgd_db22);
        let eq60_e1423_d_b23: f64 = (eq60_e1421 * var_qfgd_db23);
        let eq60_e1423_d_b24: f64 = (eq60_e1421 * var_qfgd_db24);
        let eq60_e1424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq60_e1423);
        let eq60_value: f64 = eq60_e1424;
        let eq60_node_derivatives: [f64; 21] = [(eq60_e1423_d_n0 * ddt_scale), (eq60_e1423_d_n1 * ddt_scale), (eq60_e1423_d_n2 * ddt_scale), (eq60_e1423_d_n3 * ddt_scale), (eq60_e1423_d_n4 * ddt_scale), (eq60_e1423_d_n5 * ddt_scale), (eq60_e1423_d_n6 * ddt_scale), (eq60_e1423_d_n7 * ddt_scale), (eq60_e1423_d_n8 * ddt_scale), (eq60_e1423_d_n9 * ddt_scale), (eq60_e1423_d_n10 * ddt_scale), (eq60_e1423_d_n11 * ddt_scale), (eq60_e1423_d_n12 * ddt_scale), (eq60_e1423_d_n13 * ddt_scale), (eq60_e1423_d_n14 * ddt_scale), (eq60_e1423_d_n15 * ddt_scale), (eq60_e1423_d_n16 * ddt_scale), (eq60_e1423_d_n17 * ddt_scale), (eq60_e1423_d_n18 * ddt_scale), (eq60_e1423_d_n19 * ddt_scale), (eq60_e1423_d_n20 * ddt_scale)];
        let eq60_branch_derivatives: [f64; 25] = [(eq60_e1423_d_b0 * ddt_scale), (eq60_e1423_d_b1 * ddt_scale), (eq60_e1423_d_b2 * ddt_scale), (eq60_e1423_d_b3 * ddt_scale), (eq60_e1423_d_b4 * ddt_scale), (eq60_e1423_d_b5 * ddt_scale), (eq60_e1423_d_b6 * ddt_scale), (eq60_e1423_d_b7 * ddt_scale), (eq60_e1423_d_b8 * ddt_scale), (eq60_e1423_d_b9 * ddt_scale), (eq60_e1423_d_b10 * ddt_scale), (eq60_e1423_d_b11 * ddt_scale), (eq60_e1423_d_b12 * ddt_scale), (eq60_e1423_d_b13 * ddt_scale), (eq60_e1423_d_b14 * ddt_scale), (eq60_e1423_d_b15 * ddt_scale), (eq60_e1423_d_b16 * ddt_scale), (eq60_e1423_d_b17 * ddt_scale), (eq60_e1423_d_b18 * ddt_scale), (eq60_e1423_d_b19 * ddt_scale), (eq60_e1423_d_b20 * ddt_scale), (eq60_e1423_d_b21 * ddt_scale), (eq60_e1423_d_b22 * ddt_scale), (eq60_e1423_d_b23 * ddt_scale), (eq60_e1423_d_b24 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let eq61_e1427: f64 = (var_chnl_type * var_mult_inst);
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * s.v[856]);
        let eq61_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq61_e1431);
        let eq61_e1432_d_n0: f64 = ((eq61_e1429 * s.dn[856][0]) * ddt_scale);
        let eq61_e1432_d_n1: f64 = ((eq61_e1429 * s.dn[856][1]) * ddt_scale);
        let eq61_e1432_d_n2: f64 = ((eq61_e1429 * s.dn[856][2]) * ddt_scale);
        let eq61_e1432_d_n3: f64 = ((eq61_e1429 * s.dn[856][3]) * ddt_scale);
        let eq61_e1432_d_n4: f64 = ((eq61_e1429 * s.dn[856][4]) * ddt_scale);
        let eq61_e1432_d_n5: f64 = ((eq61_e1429 * s.dn[856][5]) * ddt_scale);
        let eq61_e1432_d_n6: f64 = ((eq61_e1429 * s.dn[856][6]) * ddt_scale);
        let eq61_e1432_d_n7: f64 = ((eq61_e1429 * s.dn[856][7]) * ddt_scale);
        let eq61_e1432_d_n8: f64 = ((eq61_e1429 * s.dn[856][8]) * ddt_scale);
        let eq61_e1432_d_n9: f64 = ((eq61_e1429 * s.dn[856][9]) * ddt_scale);
        let eq61_e1432_d_n10: f64 = ((eq61_e1429 * s.dn[856][10]) * ddt_scale);
        let eq61_e1432_d_n11: f64 = ((eq61_e1429 * s.dn[856][11]) * ddt_scale);
        let eq61_e1432_d_n12: f64 = ((eq61_e1429 * s.dn[856][12]) * ddt_scale);
        let eq61_e1432_d_n13: f64 = ((eq61_e1429 * s.dn[856][13]) * ddt_scale);
        let eq61_e1432_d_n14: f64 = ((eq61_e1429 * s.dn[856][14]) * ddt_scale);
        let eq61_e1432_d_n15: f64 = ((eq61_e1429 * s.dn[856][15]) * ddt_scale);
        let eq61_e1432_d_n16: f64 = ((eq61_e1429 * s.dn[856][16]) * ddt_scale);
        let eq61_e1432_d_n17: f64 = ((eq61_e1429 * s.dn[856][17]) * ddt_scale);
        let eq61_e1432_d_n18: f64 = ((eq61_e1429 * s.dn[856][18]) * ddt_scale);
        let eq61_e1432_d_n19: f64 = ((eq61_e1429 * s.dn[856][19]) * ddt_scale);
        let eq61_e1432_d_n20: f64 = ((eq61_e1429 * s.dn[856][20]) * ddt_scale);
        let eq61_e1432_d_b0: f64 = ((eq61_e1429 * s.db[856][0]) * ddt_scale);
        let eq61_e1432_d_b1: f64 = ((eq61_e1429 * s.db[856][1]) * ddt_scale);
        let eq61_e1432_d_b2: f64 = ((eq61_e1429 * s.db[856][2]) * ddt_scale);
        let eq61_e1432_d_b3: f64 = ((eq61_e1429 * s.db[856][3]) * ddt_scale);
        let eq61_e1432_d_b4: f64 = ((eq61_e1429 * s.db[856][4]) * ddt_scale);
        let eq61_e1432_d_b5: f64 = ((eq61_e1429 * s.db[856][5]) * ddt_scale);
        let eq61_e1432_d_b6: f64 = ((eq61_e1429 * s.db[856][6]) * ddt_scale);
        let eq61_e1432_d_b7: f64 = ((eq61_e1429 * s.db[856][7]) * ddt_scale);
        let eq61_e1432_d_b8: f64 = ((eq61_e1429 * s.db[856][8]) * ddt_scale);
        let eq61_e1432_d_b9: f64 = ((eq61_e1429 * s.db[856][9]) * ddt_scale);
        let eq61_e1432_d_b10: f64 = ((eq61_e1429 * s.db[856][10]) * ddt_scale);
        let eq61_e1432_d_b11: f64 = ((eq61_e1429 * s.db[856][11]) * ddt_scale);
        let eq61_e1432_d_b12: f64 = ((eq61_e1429 * s.db[856][12]) * ddt_scale);
        let eq61_e1432_d_b13: f64 = ((eq61_e1429 * s.db[856][13]) * ddt_scale);
        let eq61_e1432_d_b14: f64 = ((eq61_e1429 * s.db[856][14]) * ddt_scale);
        let eq61_e1432_d_b15: f64 = ((eq61_e1429 * s.db[856][15]) * ddt_scale);
        let eq61_e1432_d_b16: f64 = ((eq61_e1429 * s.db[856][16]) * ddt_scale);
        let eq61_e1432_d_b17: f64 = ((eq61_e1429 * s.db[856][17]) * ddt_scale);
        let eq61_e1432_d_b18: f64 = ((eq61_e1429 * s.db[856][18]) * ddt_scale);
        let eq61_e1432_d_b19: f64 = ((eq61_e1429 * s.db[856][19]) * ddt_scale);
        let eq61_e1432_d_b20: f64 = ((eq61_e1429 * s.db[856][20]) * ddt_scale);
        let eq61_e1432_d_b21: f64 = ((eq61_e1429 * s.db[856][21]) * ddt_scale);
        let eq61_e1432_d_b22: f64 = ((eq61_e1429 * s.db[856][22]) * ddt_scale);
        let eq61_e1432_d_b23: f64 = ((eq61_e1429 * s.db[856][23]) * ddt_scale);
        let eq61_e1432_d_b24: f64 = ((eq61_e1429 * s.db[856][24]) * ddt_scale);
        let eq61_value: f64 = eq61_e1432;
        let eq61_node_derivatives: [f64; 21] = [eq61_e1432_d_n0, eq61_e1432_d_n1, eq61_e1432_d_n2, eq61_e1432_d_n3, eq61_e1432_d_n4, eq61_e1432_d_n5, eq61_e1432_d_n6, eq61_e1432_d_n7, eq61_e1432_d_n8, eq61_e1432_d_n9, eq61_e1432_d_n10, eq61_e1432_d_n11, eq61_e1432_d_n12, eq61_e1432_d_n13, eq61_e1432_d_n14, eq61_e1432_d_n15, eq61_e1432_d_n16, eq61_e1432_d_n17, eq61_e1432_d_n18, eq61_e1432_d_n19, eq61_e1432_d_n20];
        let eq61_branch_derivatives: [f64; 25] = [eq61_e1432_d_b0, eq61_e1432_d_b1, eq61_e1432_d_b2, eq61_e1432_d_b3, eq61_e1432_d_b4, eq61_e1432_d_b5, eq61_e1432_d_b6, eq61_e1432_d_b7, eq61_e1432_d_b8, eq61_e1432_d_b9, eq61_e1432_d_b10, eq61_e1432_d_b11, eq61_e1432_d_b12, eq61_e1432_d_b13, eq61_e1432_d_b14, eq61_e1432_d_b15, eq61_e1432_d_b16, eq61_e1432_d_b17, eq61_e1432_d_b18, eq61_e1432_d_b19, eq61_e1432_d_b20, eq61_e1432_d_b21, eq61_e1432_d_b22, eq61_e1432_d_b23, eq61_e1432_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let eq62_e1435: f64 = (var_chnl_type * var_mult_inst);
        let eq62_e1437: f64 = (eq62_e1435 * p.p33);
        let eq62_e1439: f64 = (eq62_e1437 * var_qjun_s);
        let eq62_e1439_d_n0: f64 = (eq62_e1437 * var_qjun_s_dn0);
        let eq62_e1439_d_n1: f64 = (eq62_e1437 * var_qjun_s_dn1);
        let eq62_e1439_d_n2: f64 = (eq62_e1437 * var_qjun_s_dn2);
        let eq62_e1439_d_n3: f64 = (eq62_e1437 * var_qjun_s_dn3);
        let eq62_e1439_d_n4: f64 = (eq62_e1437 * var_qjun_s_dn4);
        let eq62_e1439_d_n5: f64 = (eq62_e1437 * var_qjun_s_dn5);
        let eq62_e1439_d_n6: f64 = (eq62_e1437 * var_qjun_s_dn6);
        let eq62_e1439_d_n7: f64 = (eq62_e1437 * var_qjun_s_dn7);
        let eq62_e1439_d_n8: f64 = (eq62_e1437 * var_qjun_s_dn8);
        let eq62_e1439_d_n9: f64 = (eq62_e1437 * var_qjun_s_dn9);
        let eq62_e1439_d_n10: f64 = (eq62_e1437 * var_qjun_s_dn10);
        let eq62_e1439_d_n11: f64 = (eq62_e1437 * var_qjun_s_dn11);
        let eq62_e1439_d_n12: f64 = (eq62_e1437 * var_qjun_s_dn12);
        let eq62_e1439_d_n13: f64 = (eq62_e1437 * var_qjun_s_dn13);
        let eq62_e1439_d_n14: f64 = (eq62_e1437 * var_qjun_s_dn14);
        let eq62_e1439_d_n15: f64 = (eq62_e1437 * var_qjun_s_dn15);
        let eq62_e1439_d_n16: f64 = (eq62_e1437 * var_qjun_s_dn16);
        let eq62_e1439_d_n17: f64 = (eq62_e1437 * var_qjun_s_dn17);
        let eq62_e1439_d_n18: f64 = (eq62_e1437 * var_qjun_s_dn18);
        let eq62_e1439_d_n19: f64 = (eq62_e1437 * var_qjun_s_dn19);
        let eq62_e1439_d_n20: f64 = (eq62_e1437 * var_qjun_s_dn20);
        let eq62_e1439_d_b0: f64 = (eq62_e1437 * var_qjun_s_db0);
        let eq62_e1439_d_b1: f64 = (eq62_e1437 * var_qjun_s_db1);
        let eq62_e1439_d_b2: f64 = (eq62_e1437 * var_qjun_s_db2);
        let eq62_e1439_d_b3: f64 = (eq62_e1437 * var_qjun_s_db3);
        let eq62_e1439_d_b4: f64 = (eq62_e1437 * var_qjun_s_db4);
        let eq62_e1439_d_b5: f64 = (eq62_e1437 * var_qjun_s_db5);
        let eq62_e1439_d_b6: f64 = (eq62_e1437 * var_qjun_s_db6);
        let eq62_e1439_d_b7: f64 = (eq62_e1437 * var_qjun_s_db7);
        let eq62_e1439_d_b8: f64 = (eq62_e1437 * var_qjun_s_db8);
        let eq62_e1439_d_b9: f64 = (eq62_e1437 * var_qjun_s_db9);
        let eq62_e1439_d_b10: f64 = (eq62_e1437 * var_qjun_s_db10);
        let eq62_e1439_d_b11: f64 = (eq62_e1437 * var_qjun_s_db11);
        let eq62_e1439_d_b12: f64 = (eq62_e1437 * var_qjun_s_db12);
        let eq62_e1439_d_b13: f64 = (eq62_e1437 * var_qjun_s_db13);
        let eq62_e1439_d_b14: f64 = (eq62_e1437 * var_qjun_s_db14);
        let eq62_e1439_d_b15: f64 = (eq62_e1437 * var_qjun_s_db15);
        let eq62_e1439_d_b16: f64 = (eq62_e1437 * var_qjun_s_db16);
        let eq62_e1439_d_b17: f64 = (eq62_e1437 * var_qjun_s_db17);
        let eq62_e1439_d_b18: f64 = (eq62_e1437 * var_qjun_s_db18);
        let eq62_e1439_d_b19: f64 = (eq62_e1437 * var_qjun_s_db19);
        let eq62_e1439_d_b20: f64 = (eq62_e1437 * var_qjun_s_db20);
        let eq62_e1439_d_b21: f64 = (eq62_e1437 * var_qjun_s_db21);
        let eq62_e1439_d_b22: f64 = (eq62_e1437 * var_qjun_s_db22);
        let eq62_e1439_d_b23: f64 = (eq62_e1437 * var_qjun_s_db23);
        let eq62_e1439_d_b24: f64 = (eq62_e1437 * var_qjun_s_db24);
        let eq62_e1440: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq62_e1439);
        let eq62_value: f64 = eq62_e1440;
        let eq62_node_derivatives: [f64; 21] = [(eq62_e1439_d_n0 * ddt_scale), (eq62_e1439_d_n1 * ddt_scale), (eq62_e1439_d_n2 * ddt_scale), (eq62_e1439_d_n3 * ddt_scale), (eq62_e1439_d_n4 * ddt_scale), (eq62_e1439_d_n5 * ddt_scale), (eq62_e1439_d_n6 * ddt_scale), (eq62_e1439_d_n7 * ddt_scale), (eq62_e1439_d_n8 * ddt_scale), (eq62_e1439_d_n9 * ddt_scale), (eq62_e1439_d_n10 * ddt_scale), (eq62_e1439_d_n11 * ddt_scale), (eq62_e1439_d_n12 * ddt_scale), (eq62_e1439_d_n13 * ddt_scale), (eq62_e1439_d_n14 * ddt_scale), (eq62_e1439_d_n15 * ddt_scale), (eq62_e1439_d_n16 * ddt_scale), (eq62_e1439_d_n17 * ddt_scale), (eq62_e1439_d_n18 * ddt_scale), (eq62_e1439_d_n19 * ddt_scale), (eq62_e1439_d_n20 * ddt_scale)];
        let eq62_branch_derivatives: [f64; 25] = [(eq62_e1439_d_b0 * ddt_scale), (eq62_e1439_d_b1 * ddt_scale), (eq62_e1439_d_b2 * ddt_scale), (eq62_e1439_d_b3 * ddt_scale), (eq62_e1439_d_b4 * ddt_scale), (eq62_e1439_d_b5 * ddt_scale), (eq62_e1439_d_b6 * ddt_scale), (eq62_e1439_d_b7 * ddt_scale), (eq62_e1439_d_b8 * ddt_scale), (eq62_e1439_d_b9 * ddt_scale), (eq62_e1439_d_b10 * ddt_scale), (eq62_e1439_d_b11 * ddt_scale), (eq62_e1439_d_b12 * ddt_scale), (eq62_e1439_d_b13 * ddt_scale), (eq62_e1439_d_b14 * ddt_scale), (eq62_e1439_d_b15 * ddt_scale), (eq62_e1439_d_b16 * ddt_scale), (eq62_e1439_d_b17 * ddt_scale), (eq62_e1439_d_b18 * ddt_scale), (eq62_e1439_d_b19 * ddt_scale), (eq62_e1439_d_b20 * ddt_scale), (eq62_e1439_d_b21 * ddt_scale), (eq62_e1439_d_b22 * ddt_scale), (eq62_e1439_d_b23 * ddt_scale), (eq62_e1439_d_b24 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let eq63_e1443: f64 = (var_chnl_type * var_mult_inst);
        let eq63_e1445: f64 = (eq63_e1443 * p.p33);
        let eq63_e1447: f64 = (eq63_e1445 * var_qjun_d);
        let eq63_e1447_d_n0: f64 = (eq63_e1445 * var_qjun_d_dn0);
        let eq63_e1447_d_n1: f64 = (eq63_e1445 * var_qjun_d_dn1);
        let eq63_e1447_d_n2: f64 = (eq63_e1445 * var_qjun_d_dn2);
        let eq63_e1447_d_n3: f64 = (eq63_e1445 * var_qjun_d_dn3);
        let eq63_e1447_d_n4: f64 = (eq63_e1445 * var_qjun_d_dn4);
        let eq63_e1447_d_n5: f64 = (eq63_e1445 * var_qjun_d_dn5);
        let eq63_e1447_d_n6: f64 = (eq63_e1445 * var_qjun_d_dn6);
        let eq63_e1447_d_n7: f64 = (eq63_e1445 * var_qjun_d_dn7);
        let eq63_e1447_d_n8: f64 = (eq63_e1445 * var_qjun_d_dn8);
        let eq63_e1447_d_n9: f64 = (eq63_e1445 * var_qjun_d_dn9);
        let eq63_e1447_d_n10: f64 = (eq63_e1445 * var_qjun_d_dn10);
        let eq63_e1447_d_n11: f64 = (eq63_e1445 * var_qjun_d_dn11);
        let eq63_e1447_d_n12: f64 = (eq63_e1445 * var_qjun_d_dn12);
        let eq63_e1447_d_n13: f64 = (eq63_e1445 * var_qjun_d_dn13);
        let eq63_e1447_d_n14: f64 = (eq63_e1445 * var_qjun_d_dn14);
        let eq63_e1447_d_n15: f64 = (eq63_e1445 * var_qjun_d_dn15);
        let eq63_e1447_d_n16: f64 = (eq63_e1445 * var_qjun_d_dn16);
        let eq63_e1447_d_n17: f64 = (eq63_e1445 * var_qjun_d_dn17);
        let eq63_e1447_d_n18: f64 = (eq63_e1445 * var_qjun_d_dn18);
        let eq63_e1447_d_n19: f64 = (eq63_e1445 * var_qjun_d_dn19);
        let eq63_e1447_d_n20: f64 = (eq63_e1445 * var_qjun_d_dn20);
        let eq63_e1447_d_b0: f64 = (eq63_e1445 * var_qjun_d_db0);
        let eq63_e1447_d_b1: f64 = (eq63_e1445 * var_qjun_d_db1);
        let eq63_e1447_d_b2: f64 = (eq63_e1445 * var_qjun_d_db2);
        let eq63_e1447_d_b3: f64 = (eq63_e1445 * var_qjun_d_db3);
        let eq63_e1447_d_b4: f64 = (eq63_e1445 * var_qjun_d_db4);
        let eq63_e1447_d_b5: f64 = (eq63_e1445 * var_qjun_d_db5);
        let eq63_e1447_d_b6: f64 = (eq63_e1445 * var_qjun_d_db6);
        let eq63_e1447_d_b7: f64 = (eq63_e1445 * var_qjun_d_db7);
        let eq63_e1447_d_b8: f64 = (eq63_e1445 * var_qjun_d_db8);
        let eq63_e1447_d_b9: f64 = (eq63_e1445 * var_qjun_d_db9);
        let eq63_e1447_d_b10: f64 = (eq63_e1445 * var_qjun_d_db10);
        let eq63_e1447_d_b11: f64 = (eq63_e1445 * var_qjun_d_db11);
        let eq63_e1447_d_b12: f64 = (eq63_e1445 * var_qjun_d_db12);
        let eq63_e1447_d_b13: f64 = (eq63_e1445 * var_qjun_d_db13);
        let eq63_e1447_d_b14: f64 = (eq63_e1445 * var_qjun_d_db14);
        let eq63_e1447_d_b15: f64 = (eq63_e1445 * var_qjun_d_db15);
        let eq63_e1447_d_b16: f64 = (eq63_e1445 * var_qjun_d_db16);
        let eq63_e1447_d_b17: f64 = (eq63_e1445 * var_qjun_d_db17);
        let eq63_e1447_d_b18: f64 = (eq63_e1445 * var_qjun_d_db18);
        let eq63_e1447_d_b19: f64 = (eq63_e1445 * var_qjun_d_db19);
        let eq63_e1447_d_b20: f64 = (eq63_e1445 * var_qjun_d_db20);
        let eq63_e1447_d_b21: f64 = (eq63_e1445 * var_qjun_d_db21);
        let eq63_e1447_d_b22: f64 = (eq63_e1445 * var_qjun_d_db22);
        let eq63_e1447_d_b23: f64 = (eq63_e1445 * var_qjun_d_db23);
        let eq63_e1447_d_b24: f64 = (eq63_e1445 * var_qjun_d_db24);
        let eq63_e1448: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq63_e1447);
        let eq63_value: f64 = eq63_e1448;
        let eq63_node_derivatives: [f64; 21] = [(eq63_e1447_d_n0 * ddt_scale), (eq63_e1447_d_n1 * ddt_scale), (eq63_e1447_d_n2 * ddt_scale), (eq63_e1447_d_n3 * ddt_scale), (eq63_e1447_d_n4 * ddt_scale), (eq63_e1447_d_n5 * ddt_scale), (eq63_e1447_d_n6 * ddt_scale), (eq63_e1447_d_n7 * ddt_scale), (eq63_e1447_d_n8 * ddt_scale), (eq63_e1447_d_n9 * ddt_scale), (eq63_e1447_d_n10 * ddt_scale), (eq63_e1447_d_n11 * ddt_scale), (eq63_e1447_d_n12 * ddt_scale), (eq63_e1447_d_n13 * ddt_scale), (eq63_e1447_d_n14 * ddt_scale), (eq63_e1447_d_n15 * ddt_scale), (eq63_e1447_d_n16 * ddt_scale), (eq63_e1447_d_n17 * ddt_scale), (eq63_e1447_d_n18 * ddt_scale), (eq63_e1447_d_n19 * ddt_scale), (eq63_e1447_d_n20 * ddt_scale)];
        let eq63_branch_derivatives: [f64; 25] = [(eq63_e1447_d_b0 * ddt_scale), (eq63_e1447_d_b1 * ddt_scale), (eq63_e1447_d_b2 * ddt_scale), (eq63_e1447_d_b3 * ddt_scale), (eq63_e1447_d_b4 * ddt_scale), (eq63_e1447_d_b5 * ddt_scale), (eq63_e1447_d_b6 * ddt_scale), (eq63_e1447_d_b7 * ddt_scale), (eq63_e1447_d_b8 * ddt_scale), (eq63_e1447_d_b9 * ddt_scale), (eq63_e1447_d_b10 * ddt_scale), (eq63_e1447_d_b11 * ddt_scale), (eq63_e1447_d_b12 * ddt_scale), (eq63_e1447_d_b13 * ddt_scale), (eq63_e1447_d_b14 * ddt_scale), (eq63_e1447_d_b15 * ddt_scale), (eq63_e1447_d_b16 * ddt_scale), (eq63_e1447_d_b17 * ddt_scale), (eq63_e1447_d_b18 * ddt_scale), (eq63_e1447_d_b19 * ddt_scale), (eq63_e1447_d_b20 * ddt_scale), (eq63_e1447_d_b21 * ddt_scale), (eq63_e1447_d_b22 * ddt_scale), (eq63_e1447_d_b23 * ddt_scale), (eq63_e1447_d_b24 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        var_mult_inst: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq65_e1456: f64 = ((nv4 - 0.0) / s.v[859]);
        let eq65_e1456_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[859][0]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[859][1]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[859][2]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[859][3]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n4: f64 = ((s.v[859] - ((nv4 - 0.0) * s.dn[859][4])) / (s.v[859] * s.v[859]));
        let eq65_e1456_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[859][5]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[859][6]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[859][7]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[859][8]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[859][9]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[859][10]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[859][11]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n12: f64 = (-(((nv4 - 0.0) * s.dn[859][12]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n13: f64 = (-(((nv4 - 0.0) * s.dn[859][13]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n14: f64 = (-(((nv4 - 0.0) * s.dn[859][14]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n15: f64 = (-(((nv4 - 0.0) * s.dn[859][15]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n16: f64 = (-(((nv4 - 0.0) * s.dn[859][16]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n17: f64 = (-(((nv4 - 0.0) * s.dn[859][17]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n18: f64 = (-(((nv4 - 0.0) * s.dn[859][18]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n19: f64 = (-(((nv4 - 0.0) * s.dn[859][19]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_n20: f64 = (-(((nv4 - 0.0) * s.dn[859][20]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b0: f64 = (-(((nv4 - 0.0) * s.db[859][0]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b1: f64 = (-(((nv4 - 0.0) * s.db[859][1]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b2: f64 = (-(((nv4 - 0.0) * s.db[859][2]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b3: f64 = (-(((nv4 - 0.0) * s.db[859][3]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b4: f64 = (-(((nv4 - 0.0) * s.db[859][4]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b5: f64 = (-(((nv4 - 0.0) * s.db[859][5]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b6: f64 = (-(((nv4 - 0.0) * s.db[859][6]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b7: f64 = (-(((nv4 - 0.0) * s.db[859][7]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b8: f64 = (-(((nv4 - 0.0) * s.db[859][8]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b9: f64 = (-(((nv4 - 0.0) * s.db[859][9]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b10: f64 = (-(((nv4 - 0.0) * s.db[859][10]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b11: f64 = (-(((nv4 - 0.0) * s.db[859][11]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b12: f64 = (-(((nv4 - 0.0) * s.db[859][12]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b13: f64 = (-(((nv4 - 0.0) * s.db[859][13]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b14: f64 = (-(((nv4 - 0.0) * s.db[859][14]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b15: f64 = (-(((nv4 - 0.0) * s.db[859][15]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b16: f64 = (-(((nv4 - 0.0) * s.db[859][16]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b17: f64 = (-(((nv4 - 0.0) * s.db[859][17]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b18: f64 = (-(((nv4 - 0.0) * s.db[859][18]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b19: f64 = (-(((nv4 - 0.0) * s.db[859][19]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b20: f64 = (-(((nv4 - 0.0) * s.db[859][20]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b21: f64 = (-(((nv4 - 0.0) * s.db[859][21]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b22: f64 = (-(((nv4 - 0.0) * s.db[859][22]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b23: f64 = (-(((nv4 - 0.0) * s.db[859][23]) / (s.v[859] * s.v[859])));
        let eq65_e1456_d_b24: f64 = (-(((nv4 - 0.0) * s.db[859][24]) / (s.v[859] * s.v[859])));
        let eq65_value: f64 = eq65_e1456;
        let eq65_node_derivatives: [f64; 21] = [eq65_e1456_d_n0, eq65_e1456_d_n1, eq65_e1456_d_n2, eq65_e1456_d_n3, eq65_e1456_d_n4, eq65_e1456_d_n5, eq65_e1456_d_n6, eq65_e1456_d_n7, eq65_e1456_d_n8, eq65_e1456_d_n9, eq65_e1456_d_n10, eq65_e1456_d_n11, eq65_e1456_d_n12, eq65_e1456_d_n13, eq65_e1456_d_n14, eq65_e1456_d_n15, eq65_e1456_d_n16, eq65_e1456_d_n17, eq65_e1456_d_n18, eq65_e1456_d_n19, eq65_e1456_d_n20];
        let eq65_branch_derivatives: [f64; 25] = [eq65_e1456_d_b0, eq65_e1456_d_b1, eq65_e1456_d_b2, eq65_e1456_d_b3, eq65_e1456_d_b4, eq65_e1456_d_b5, eq65_e1456_d_b6, eq65_e1456_d_b7, eq65_e1456_d_b8, eq65_e1456_d_b9, eq65_e1456_d_b10, eq65_e1456_d_b11, eq65_e1456_d_b12, eq65_e1456_d_b13, eq65_e1456_d_b14, eq65_e1456_d_b15, eq65_e1456_d_b16, eq65_e1456_d_b17, eq65_e1456_d_b18, eq65_e1456_d_b19, eq65_e1456_d_b20, eq65_e1456_d_b21, eq65_e1456_d_b22, eq65_e1456_d_b23, eq65_e1456_d_b24];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let eq66_e1459: f64 = (s.v[860] * (nv4 - 0.0));
        let eq66_e1459_d_n0: f64 = (s.dn[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_n1: f64 = (s.dn[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_n2: f64 = (s.dn[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_n3: f64 = (s.dn[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_n4: f64 = ((s.dn[860][4] * (nv4 - 0.0)) + s.v[860]);
        let eq66_e1459_d_n5: f64 = (s.dn[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (s.dn[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (s.dn[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (s.dn[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_n9: f64 = (s.dn[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_n10: f64 = (s.dn[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_n11: f64 = (s.dn[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (s.dn[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (s.dn[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (s.dn[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (s.dn[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (s.dn[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (s.dn[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (s.dn[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (s.dn[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (s.dn[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b0: f64 = (s.db[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_b1: f64 = (s.db[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_b2: f64 = (s.db[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_b3: f64 = (s.db[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_b4: f64 = (s.db[860][4] * (nv4 - 0.0));
        let eq66_e1459_d_b5: f64 = (s.db[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_b6: f64 = (s.db[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_b7: f64 = (s.db[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_b8: f64 = (s.db[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_b9: f64 = (s.db[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_b10: f64 = (s.db[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_b11: f64 = (s.db[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_b12: f64 = (s.db[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_b13: f64 = (s.db[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_b14: f64 = (s.db[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_b15: f64 = (s.db[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_b16: f64 = (s.db[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_b17: f64 = (s.db[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_b18: f64 = (s.db[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_b19: f64 = (s.db[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_b20: f64 = (s.db[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b21: f64 = (s.db[860][21] * (nv4 - 0.0));
        let eq66_e1459_d_b22: f64 = (s.db[860][22] * (nv4 - 0.0));
        let eq66_e1459_d_b23: f64 = (s.db[860][23] * (nv4 - 0.0));
        let eq66_e1459_d_b24: f64 = (s.db[860][24] * (nv4 - 0.0));
        let eq66_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq66_e1459);
        let eq66_value: f64 = eq66_e1460;
        let eq66_node_derivatives: [f64; 21] = [(eq66_e1459_d_n0 * ddt_scale), (eq66_e1459_d_n1 * ddt_scale), (eq66_e1459_d_n2 * ddt_scale), (eq66_e1459_d_n3 * ddt_scale), (eq66_e1459_d_n4 * ddt_scale), (eq66_e1459_d_n5 * ddt_scale), (eq66_e1459_d_n6 * ddt_scale), (eq66_e1459_d_n7 * ddt_scale), (eq66_e1459_d_n8 * ddt_scale), (eq66_e1459_d_n9 * ddt_scale), (eq66_e1459_d_n10 * ddt_scale), (eq66_e1459_d_n11 * ddt_scale), (eq66_e1459_d_n12 * ddt_scale), (eq66_e1459_d_n13 * ddt_scale), (eq66_e1459_d_n14 * ddt_scale), (eq66_e1459_d_n15 * ddt_scale), (eq66_e1459_d_n16 * ddt_scale), (eq66_e1459_d_n17 * ddt_scale), (eq66_e1459_d_n18 * ddt_scale), (eq66_e1459_d_n19 * ddt_scale), (eq66_e1459_d_n20 * ddt_scale)];
        let eq66_branch_derivatives: [f64; 25] = [(eq66_e1459_d_b0 * ddt_scale), (eq66_e1459_d_b1 * ddt_scale), (eq66_e1459_d_b2 * ddt_scale), (eq66_e1459_d_b3 * ddt_scale), (eq66_e1459_d_b4 * ddt_scale), (eq66_e1459_d_b5 * ddt_scale), (eq66_e1459_d_b6 * ddt_scale), (eq66_e1459_d_b7 * ddt_scale), (eq66_e1459_d_b8 * ddt_scale), (eq66_e1459_d_b9 * ddt_scale), (eq66_e1459_d_b10 * ddt_scale), (eq66_e1459_d_b11 * ddt_scale), (eq66_e1459_d_b12 * ddt_scale), (eq66_e1459_d_b13 * ddt_scale), (eq66_e1459_d_b14 * ddt_scale), (eq66_e1459_d_b15 * ddt_scale), (eq66_e1459_d_b16 * ddt_scale), (eq66_e1459_d_b17 * ddt_scale), (eq66_e1459_d_b18 * ddt_scale), (eq66_e1459_d_b19 * ddt_scale), (eq66_e1459_d_b20 * ddt_scale), (eq66_e1459_d_b21 * ddt_scale), (eq66_e1459_d_b22 * ddt_scale), (eq66_e1459_d_b23 * ddt_scale), (eq66_e1459_d_b24 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let eq67_e1463: f64 = (var_mult_inst * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * s.v[860]);
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n0: f64 = ((eq67_e1466 * s.dn[860][0]) * (nv4 - 0.0));
        let eq67_e1470_d_n1: f64 = ((eq67_e1466 * s.dn[860][1]) * (nv4 - 0.0));
        let eq67_e1470_d_n2: f64 = ((eq67_e1466 * s.dn[860][2]) * (nv4 - 0.0));
        let eq67_e1470_d_n3: f64 = ((eq67_e1466 * s.dn[860][3]) * (nv4 - 0.0));
        let eq67_e1470_d_n4: f64 = (((eq67_e1466 * s.dn[860][4]) * (nv4 - 0.0)) + eq67_e1468);
        let eq67_e1470_d_n5: f64 = ((eq67_e1466 * s.dn[860][5]) * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = ((eq67_e1466 * s.dn[860][6]) * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = ((eq67_e1466 * s.dn[860][7]) * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = ((eq67_e1466 * s.dn[860][8]) * (nv4 - 0.0));
        let eq67_e1470_d_n9: f64 = ((eq67_e1466 * s.dn[860][9]) * (nv4 - 0.0));
        let eq67_e1470_d_n10: f64 = ((eq67_e1466 * s.dn[860][10]) * (nv4 - 0.0));
        let eq67_e1470_d_n11: f64 = ((eq67_e1466 * s.dn[860][11]) * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = ((eq67_e1466 * s.dn[860][12]) * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = ((eq67_e1466 * s.dn[860][13]) * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = ((eq67_e1466 * s.dn[860][14]) * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = ((eq67_e1466 * s.dn[860][15]) * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = ((eq67_e1466 * s.dn[860][16]) * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = ((eq67_e1466 * s.dn[860][17]) * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = ((eq67_e1466 * s.dn[860][18]) * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = ((eq67_e1466 * s.dn[860][19]) * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = ((eq67_e1466 * s.dn[860][20]) * (nv4 - 0.0));
        let eq67_e1470_d_b0: f64 = ((eq67_e1466 * s.db[860][0]) * (nv4 - 0.0));
        let eq67_e1470_d_b1: f64 = ((eq67_e1466 * s.db[860][1]) * (nv4 - 0.0));
        let eq67_e1470_d_b2: f64 = ((eq67_e1466 * s.db[860][2]) * (nv4 - 0.0));
        let eq67_e1470_d_b3: f64 = ((eq67_e1466 * s.db[860][3]) * (nv4 - 0.0));
        let eq67_e1470_d_b4: f64 = ((eq67_e1466 * s.db[860][4]) * (nv4 - 0.0));
        let eq67_e1470_d_b5: f64 = ((eq67_e1466 * s.db[860][5]) * (nv4 - 0.0));
        let eq67_e1470_d_b6: f64 = ((eq67_e1466 * s.db[860][6]) * (nv4 - 0.0));
        let eq67_e1470_d_b7: f64 = ((eq67_e1466 * s.db[860][7]) * (nv4 - 0.0));
        let eq67_e1470_d_b8: f64 = ((eq67_e1466 * s.db[860][8]) * (nv4 - 0.0));
        let eq67_e1470_d_b9: f64 = ((eq67_e1466 * s.db[860][9]) * (nv4 - 0.0));
        let eq67_e1470_d_b10: f64 = ((eq67_e1466 * s.db[860][10]) * (nv4 - 0.0));
        let eq67_e1470_d_b11: f64 = ((eq67_e1466 * s.db[860][11]) * (nv4 - 0.0));
        let eq67_e1470_d_b12: f64 = ((eq67_e1466 * s.db[860][12]) * (nv4 - 0.0));
        let eq67_e1470_d_b13: f64 = ((eq67_e1466 * s.db[860][13]) * (nv4 - 0.0));
        let eq67_e1470_d_b14: f64 = ((eq67_e1466 * s.db[860][14]) * (nv4 - 0.0));
        let eq67_e1470_d_b15: f64 = ((eq67_e1466 * s.db[860][15]) * (nv4 - 0.0));
        let eq67_e1470_d_b16: f64 = ((eq67_e1466 * s.db[860][16]) * (nv4 - 0.0));
        let eq67_e1470_d_b17: f64 = ((eq67_e1466 * s.db[860][17]) * (nv4 - 0.0));
        let eq67_e1470_d_b18: f64 = ((eq67_e1466 * s.db[860][18]) * (nv4 - 0.0));
        let eq67_e1470_d_b19: f64 = ((eq67_e1466 * s.db[860][19]) * (nv4 - 0.0));
        let eq67_e1470_d_b20: f64 = ((eq67_e1466 * s.db[860][20]) * (nv4 - 0.0));
        let eq67_e1470_d_b21: f64 = ((eq67_e1466 * s.db[860][21]) * (nv4 - 0.0));
        let eq67_e1470_d_b22: f64 = ((eq67_e1466 * s.db[860][22]) * (nv4 - 0.0));
        let eq67_e1470_d_b23: f64 = ((eq67_e1466 * s.db[860][23]) * (nv4 - 0.0));
        let eq67_e1470_d_b24: f64 = ((eq67_e1466 * s.db[860][24]) * (nv4 - 0.0));
        let eq67_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq67_e1470);
        let eq67_e1472: f64 = (-eq67_e1471);
        let eq67_e1472_d_n0: f64 = (-(eq67_e1470_d_n0 * ddt_scale));
        let eq67_e1472_d_n1: f64 = (-(eq67_e1470_d_n1 * ddt_scale));
        let eq67_e1472_d_n2: f64 = (-(eq67_e1470_d_n2 * ddt_scale));
        let eq67_e1472_d_n3: f64 = (-(eq67_e1470_d_n3 * ddt_scale));
        let eq67_e1472_d_n4: f64 = (-(eq67_e1470_d_n4 * ddt_scale));
        let eq67_e1472_d_n5: f64 = (-(eq67_e1470_d_n5 * ddt_scale));
        let eq67_e1472_d_n6: f64 = (-(eq67_e1470_d_n6 * ddt_scale));
        let eq67_e1472_d_n7: f64 = (-(eq67_e1470_d_n7 * ddt_scale));
        let eq67_e1472_d_n8: f64 = (-(eq67_e1470_d_n8 * ddt_scale));
        let eq67_e1472_d_n9: f64 = (-(eq67_e1470_d_n9 * ddt_scale));
        let eq67_e1472_d_n10: f64 = (-(eq67_e1470_d_n10 * ddt_scale));
        let eq67_e1472_d_n11: f64 = (-(eq67_e1470_d_n11 * ddt_scale));
        let eq67_e1472_d_n12: f64 = (-(eq67_e1470_d_n12 * ddt_scale));
        let eq67_e1472_d_n13: f64 = (-(eq67_e1470_d_n13 * ddt_scale));
        let eq67_e1472_d_n14: f64 = (-(eq67_e1470_d_n14 * ddt_scale));
        let eq67_e1472_d_n15: f64 = (-(eq67_e1470_d_n15 * ddt_scale));
        let eq67_e1472_d_n16: f64 = (-(eq67_e1470_d_n16 * ddt_scale));
        let eq67_e1472_d_n17: f64 = (-(eq67_e1470_d_n17 * ddt_scale));
        let eq67_e1472_d_n18: f64 = (-(eq67_e1470_d_n18 * ddt_scale));
        let eq67_e1472_d_n19: f64 = (-(eq67_e1470_d_n19 * ddt_scale));
        let eq67_e1472_d_n20: f64 = (-(eq67_e1470_d_n20 * ddt_scale));
        let eq67_e1472_d_b0: f64 = (-(eq67_e1470_d_b0 * ddt_scale));
        let eq67_e1472_d_b1: f64 = (-(eq67_e1470_d_b1 * ddt_scale));
        let eq67_e1472_d_b2: f64 = (-(eq67_e1470_d_b2 * ddt_scale));
        let eq67_e1472_d_b3: f64 = (-(eq67_e1470_d_b3 * ddt_scale));
        let eq67_e1472_d_b4: f64 = (-(eq67_e1470_d_b4 * ddt_scale));
        let eq67_e1472_d_b5: f64 = (-(eq67_e1470_d_b5 * ddt_scale));
        let eq67_e1472_d_b6: f64 = (-(eq67_e1470_d_b6 * ddt_scale));
        let eq67_e1472_d_b7: f64 = (-(eq67_e1470_d_b7 * ddt_scale));
        let eq67_e1472_d_b8: f64 = (-(eq67_e1470_d_b8 * ddt_scale));
        let eq67_e1472_d_b9: f64 = (-(eq67_e1470_d_b9 * ddt_scale));
        let eq67_e1472_d_b10: f64 = (-(eq67_e1470_d_b10 * ddt_scale));
        let eq67_e1472_d_b11: f64 = (-(eq67_e1470_d_b11 * ddt_scale));
        let eq67_e1472_d_b12: f64 = (-(eq67_e1470_d_b12 * ddt_scale));
        let eq67_e1472_d_b13: f64 = (-(eq67_e1470_d_b13 * ddt_scale));
        let eq67_e1472_d_b14: f64 = (-(eq67_e1470_d_b14 * ddt_scale));
        let eq67_e1472_d_b15: f64 = (-(eq67_e1470_d_b15 * ddt_scale));
        let eq67_e1472_d_b16: f64 = (-(eq67_e1470_d_b16 * ddt_scale));
        let eq67_e1472_d_b17: f64 = (-(eq67_e1470_d_b17 * ddt_scale));
        let eq67_e1472_d_b18: f64 = (-(eq67_e1470_d_b18 * ddt_scale));
        let eq67_e1472_d_b19: f64 = (-(eq67_e1470_d_b19 * ddt_scale));
        let eq67_e1472_d_b20: f64 = (-(eq67_e1470_d_b20 * ddt_scale));
        let eq67_e1472_d_b21: f64 = (-(eq67_e1470_d_b21 * ddt_scale));
        let eq67_e1472_d_b22: f64 = (-(eq67_e1470_d_b22 * ddt_scale));
        let eq67_e1472_d_b23: f64 = (-(eq67_e1470_d_b23 * ddt_scale));
        let eq67_e1472_d_b24: f64 = (-(eq67_e1470_d_b24 * ddt_scale));
        let eq67_value: f64 = eq67_e1472;
        let eq67_node_derivatives: [f64; 21] = [eq67_e1472_d_n0, eq67_e1472_d_n1, eq67_e1472_d_n2, eq67_e1472_d_n3, eq67_e1472_d_n4, eq67_e1472_d_n5, eq67_e1472_d_n6, eq67_e1472_d_n7, eq67_e1472_d_n8, eq67_e1472_d_n9, eq67_e1472_d_n10, eq67_e1472_d_n11, eq67_e1472_d_n12, eq67_e1472_d_n13, eq67_e1472_d_n14, eq67_e1472_d_n15, eq67_e1472_d_n16, eq67_e1472_d_n17, eq67_e1472_d_n18, eq67_e1472_d_n19, eq67_e1472_d_n20];
        let eq67_branch_derivatives: [f64; 25] = [eq67_e1472_d_b0, eq67_e1472_d_b1, eq67_e1472_d_b2, eq67_e1472_d_b3, eq67_e1472_d_b4, eq67_e1472_d_b5, eq67_e1472_d_b6, eq67_e1472_d_b7, eq67_e1472_d_b8, eq67_e1472_d_b9, eq67_e1472_d_b10, eq67_e1472_d_b11, eq67_e1472_d_b12, eq67_e1472_d_b13, eq67_e1472_d_b14, eq67_e1472_d_b15, eq67_e1472_d_b16, eq67_e1472_d_b17, eq67_e1472_d_b18, eq67_e1472_d_b19, eq67_e1472_d_b20, eq67_e1472_d_b21, eq67_e1472_d_b22, eq67_e1472_d_b23, eq67_e1472_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq67_value),
            &eq67_node_derivatives,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (var_mult_inst * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * s.v[860]);
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n0: f64 = ((eq68_e1478 * s.dn[860][0]) * (nv4 - 0.0));
        let eq68_e1482_d_n1: f64 = ((eq68_e1478 * s.dn[860][1]) * (nv4 - 0.0));
        let eq68_e1482_d_n2: f64 = ((eq68_e1478 * s.dn[860][2]) * (nv4 - 0.0));
        let eq68_e1482_d_n3: f64 = ((eq68_e1478 * s.dn[860][3]) * (nv4 - 0.0));
        let eq68_e1482_d_n4: f64 = (((eq68_e1478 * s.dn[860][4]) * (nv4 - 0.0)) + eq68_e1480);
        let eq68_e1482_d_n5: f64 = ((eq68_e1478 * s.dn[860][5]) * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = ((eq68_e1478 * s.dn[860][6]) * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = ((eq68_e1478 * s.dn[860][7]) * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = ((eq68_e1478 * s.dn[860][8]) * (nv4 - 0.0));
        let eq68_e1482_d_n9: f64 = ((eq68_e1478 * s.dn[860][9]) * (nv4 - 0.0));
        let eq68_e1482_d_n10: f64 = ((eq68_e1478 * s.dn[860][10]) * (nv4 - 0.0));
        let eq68_e1482_d_n11: f64 = ((eq68_e1478 * s.dn[860][11]) * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = ((eq68_e1478 * s.dn[860][12]) * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = ((eq68_e1478 * s.dn[860][13]) * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = ((eq68_e1478 * s.dn[860][14]) * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = ((eq68_e1478 * s.dn[860][15]) * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = ((eq68_e1478 * s.dn[860][16]) * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = ((eq68_e1478 * s.dn[860][17]) * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = ((eq68_e1478 * s.dn[860][18]) * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = ((eq68_e1478 * s.dn[860][19]) * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = ((eq68_e1478 * s.dn[860][20]) * (nv4 - 0.0));
        let eq68_e1482_d_b0: f64 = ((eq68_e1478 * s.db[860][0]) * (nv4 - 0.0));
        let eq68_e1482_d_b1: f64 = ((eq68_e1478 * s.db[860][1]) * (nv4 - 0.0));
        let eq68_e1482_d_b2: f64 = ((eq68_e1478 * s.db[860][2]) * (nv4 - 0.0));
        let eq68_e1482_d_b3: f64 = ((eq68_e1478 * s.db[860][3]) * (nv4 - 0.0));
        let eq68_e1482_d_b4: f64 = ((eq68_e1478 * s.db[860][4]) * (nv4 - 0.0));
        let eq68_e1482_d_b5: f64 = ((eq68_e1478 * s.db[860][5]) * (nv4 - 0.0));
        let eq68_e1482_d_b6: f64 = ((eq68_e1478 * s.db[860][6]) * (nv4 - 0.0));
        let eq68_e1482_d_b7: f64 = ((eq68_e1478 * s.db[860][7]) * (nv4 - 0.0));
        let eq68_e1482_d_b8: f64 = ((eq68_e1478 * s.db[860][8]) * (nv4 - 0.0));
        let eq68_e1482_d_b9: f64 = ((eq68_e1478 * s.db[860][9]) * (nv4 - 0.0));
        let eq68_e1482_d_b10: f64 = ((eq68_e1478 * s.db[860][10]) * (nv4 - 0.0));
        let eq68_e1482_d_b11: f64 = ((eq68_e1478 * s.db[860][11]) * (nv4 - 0.0));
        let eq68_e1482_d_b12: f64 = ((eq68_e1478 * s.db[860][12]) * (nv4 - 0.0));
        let eq68_e1482_d_b13: f64 = ((eq68_e1478 * s.db[860][13]) * (nv4 - 0.0));
        let eq68_e1482_d_b14: f64 = ((eq68_e1478 * s.db[860][14]) * (nv4 - 0.0));
        let eq68_e1482_d_b15: f64 = ((eq68_e1478 * s.db[860][15]) * (nv4 - 0.0));
        let eq68_e1482_d_b16: f64 = ((eq68_e1478 * s.db[860][16]) * (nv4 - 0.0));
        let eq68_e1482_d_b17: f64 = ((eq68_e1478 * s.db[860][17]) * (nv4 - 0.0));
        let eq68_e1482_d_b18: f64 = ((eq68_e1478 * s.db[860][18]) * (nv4 - 0.0));
        let eq68_e1482_d_b19: f64 = ((eq68_e1478 * s.db[860][19]) * (nv4 - 0.0));
        let eq68_e1482_d_b20: f64 = ((eq68_e1478 * s.db[860][20]) * (nv4 - 0.0));
        let eq68_e1482_d_b21: f64 = ((eq68_e1478 * s.db[860][21]) * (nv4 - 0.0));
        let eq68_e1482_d_b22: f64 = ((eq68_e1478 * s.db[860][22]) * (nv4 - 0.0));
        let eq68_e1482_d_b23: f64 = ((eq68_e1478 * s.db[860][23]) * (nv4 - 0.0));
        let eq68_e1482_d_b24: f64 = ((eq68_e1478 * s.db[860][24]) * (nv4 - 0.0));
        let eq68_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq68_e1482);
        let eq68_e1484: f64 = (-eq68_e1483);
        let eq68_e1484_d_n0: f64 = (-(eq68_e1482_d_n0 * ddt_scale));
        let eq68_e1484_d_n1: f64 = (-(eq68_e1482_d_n1 * ddt_scale));
        let eq68_e1484_d_n2: f64 = (-(eq68_e1482_d_n2 * ddt_scale));
        let eq68_e1484_d_n3: f64 = (-(eq68_e1482_d_n3 * ddt_scale));
        let eq68_e1484_d_n4: f64 = (-(eq68_e1482_d_n4 * ddt_scale));
        let eq68_e1484_d_n5: f64 = (-(eq68_e1482_d_n5 * ddt_scale));
        let eq68_e1484_d_n6: f64 = (-(eq68_e1482_d_n6 * ddt_scale));
        let eq68_e1484_d_n7: f64 = (-(eq68_e1482_d_n7 * ddt_scale));
        let eq68_e1484_d_n8: f64 = (-(eq68_e1482_d_n8 * ddt_scale));
        let eq68_e1484_d_n9: f64 = (-(eq68_e1482_d_n9 * ddt_scale));
        let eq68_e1484_d_n10: f64 = (-(eq68_e1482_d_n10 * ddt_scale));
        let eq68_e1484_d_n11: f64 = (-(eq68_e1482_d_n11 * ddt_scale));
        let eq68_e1484_d_n12: f64 = (-(eq68_e1482_d_n12 * ddt_scale));
        let eq68_e1484_d_n13: f64 = (-(eq68_e1482_d_n13 * ddt_scale));
        let eq68_e1484_d_n14: f64 = (-(eq68_e1482_d_n14 * ddt_scale));
        let eq68_e1484_d_n15: f64 = (-(eq68_e1482_d_n15 * ddt_scale));
        let eq68_e1484_d_n16: f64 = (-(eq68_e1482_d_n16 * ddt_scale));
        let eq68_e1484_d_n17: f64 = (-(eq68_e1482_d_n17 * ddt_scale));
        let eq68_e1484_d_n18: f64 = (-(eq68_e1482_d_n18 * ddt_scale));
        let eq68_e1484_d_n19: f64 = (-(eq68_e1482_d_n19 * ddt_scale));
        let eq68_e1484_d_n20: f64 = (-(eq68_e1482_d_n20 * ddt_scale));
        let eq68_e1484_d_b0: f64 = (-(eq68_e1482_d_b0 * ddt_scale));
        let eq68_e1484_d_b1: f64 = (-(eq68_e1482_d_b1 * ddt_scale));
        let eq68_e1484_d_b2: f64 = (-(eq68_e1482_d_b2 * ddt_scale));
        let eq68_e1484_d_b3: f64 = (-(eq68_e1482_d_b3 * ddt_scale));
        let eq68_e1484_d_b4: f64 = (-(eq68_e1482_d_b4 * ddt_scale));
        let eq68_e1484_d_b5: f64 = (-(eq68_e1482_d_b5 * ddt_scale));
        let eq68_e1484_d_b6: f64 = (-(eq68_e1482_d_b6 * ddt_scale));
        let eq68_e1484_d_b7: f64 = (-(eq68_e1482_d_b7 * ddt_scale));
        let eq68_e1484_d_b8: f64 = (-(eq68_e1482_d_b8 * ddt_scale));
        let eq68_e1484_d_b9: f64 = (-(eq68_e1482_d_b9 * ddt_scale));
        let eq68_e1484_d_b10: f64 = (-(eq68_e1482_d_b10 * ddt_scale));
        let eq68_e1484_d_b11: f64 = (-(eq68_e1482_d_b11 * ddt_scale));
        let eq68_e1484_d_b12: f64 = (-(eq68_e1482_d_b12 * ddt_scale));
        let eq68_e1484_d_b13: f64 = (-(eq68_e1482_d_b13 * ddt_scale));
        let eq68_e1484_d_b14: f64 = (-(eq68_e1482_d_b14 * ddt_scale));
        let eq68_e1484_d_b15: f64 = (-(eq68_e1482_d_b15 * ddt_scale));
        let eq68_e1484_d_b16: f64 = (-(eq68_e1482_d_b16 * ddt_scale));
        let eq68_e1484_d_b17: f64 = (-(eq68_e1482_d_b17 * ddt_scale));
        let eq68_e1484_d_b18: f64 = (-(eq68_e1482_d_b18 * ddt_scale));
        let eq68_e1484_d_b19: f64 = (-(eq68_e1482_d_b19 * ddt_scale));
        let eq68_e1484_d_b20: f64 = (-(eq68_e1482_d_b20 * ddt_scale));
        let eq68_e1484_d_b21: f64 = (-(eq68_e1482_d_b21 * ddt_scale));
        let eq68_e1484_d_b22: f64 = (-(eq68_e1482_d_b22 * ddt_scale));
        let eq68_e1484_d_b23: f64 = (-(eq68_e1482_d_b23 * ddt_scale));
        let eq68_e1484_d_b24: f64 = (-(eq68_e1482_d_b24 * ddt_scale));
        let eq68_value: f64 = eq68_e1484;
        let eq68_node_derivatives: [f64; 21] = [eq68_e1484_d_n0, eq68_e1484_d_n1, eq68_e1484_d_n2, eq68_e1484_d_n3, eq68_e1484_d_n4, eq68_e1484_d_n5, eq68_e1484_d_n6, eq68_e1484_d_n7, eq68_e1484_d_n8, eq68_e1484_d_n9, eq68_e1484_d_n10, eq68_e1484_d_n11, eq68_e1484_d_n12, eq68_e1484_d_n13, eq68_e1484_d_n14, eq68_e1484_d_n15, eq68_e1484_d_n16, eq68_e1484_d_n17, eq68_e1484_d_n18, eq68_e1484_d_n19, eq68_e1484_d_n20];
        let eq68_branch_derivatives: [f64; 25] = [eq68_e1484_d_b0, eq68_e1484_d_b1, eq68_e1484_d_b2, eq68_e1484_d_b3, eq68_e1484_d_b4, eq68_e1484_d_b5, eq68_e1484_d_b6, eq68_e1484_d_b7, eq68_e1484_d_b8, eq68_e1484_d_b9, eq68_e1484_d_b10, eq68_e1484_d_b11, eq68_e1484_d_b12, eq68_e1484_d_b13, eq68_e1484_d_b14, eq68_e1484_d_b15, eq68_e1484_d_b16, eq68_e1484_d_b17, eq68_e1484_d_b18, eq68_e1484_d_b19, eq68_e1484_d_b20, eq68_e1484_d_b21, eq68_e1484_d_b22, eq68_e1484_d_b23, eq68_e1484_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq68_value),
            &eq68_node_derivatives,
            &eq68_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let __rspice_deriv_cse_17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let __rspice_deriv_cse_18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let __rspice_deriv_cse_19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let __rspice_deriv_cse_20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let __rspice_deriv_cse_21: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let __rspice_deriv_cse_22: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let __rspice_deriv_cse_23: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let __rspice_deriv_cse_24: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let __rspice_deriv_cse_25: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let __rspice_deriv_cse_26: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let __rspice_deriv_cse_27: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let __rspice_deriv_cse_28: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let __rspice_deriv_cse_29: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let __rspice_deriv_cse_30: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let __rspice_deriv_cse_31: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let __rspice_deriv_cse_32: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let __rspice_deriv_cse_33: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let __rspice_deriv_cse_34: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let __rspice_deriv_cse_35: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let __rspice_deriv_cse_36: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let __rspice_deriv_cse_37: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let __rspice_deriv_cse_38: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let __rspice_deriv_cse_39: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let __rspice_deriv_cse_40: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let __rspice_deriv_cse_41: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let __rspice_deriv_cse_42: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let __rspice_deriv_cse_43: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let __rspice_deriv_cse_44: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let __rspice_deriv_cse_45: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq56_e1387: f64 = (s.v[0] * s.v[19]);
        let eq56_e1389: f64 = (eq56_e1387 * p.p33);
        let eq56_e1389_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq56_e1389_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq56_e1389_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq56_e1389_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq56_e1389_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq56_e1389_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq56_e1389_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq56_e1389_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq56_e1389_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq56_e1389_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq56_e1389_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq56_e1389_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq56_e1389_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq56_e1389_d_n13: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq56_e1389_d_n14: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq56_e1389_d_n15: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq56_e1389_d_n16: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq56_e1389_d_n17: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq56_e1389_d_n18: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq56_e1389_d_n19: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq56_e1389_d_n20: f64 = (__rspice_deriv_cse_20 * p.p33);
        let eq56_e1389_d_b0: f64 = (__rspice_deriv_cse_21 * p.p33);
        let eq56_e1389_d_b1: f64 = (__rspice_deriv_cse_22 * p.p33);
        let eq56_e1389_d_b2: f64 = (__rspice_deriv_cse_23 * p.p33);
        let eq56_e1389_d_b3: f64 = (__rspice_deriv_cse_24 * p.p33);
        let eq56_e1389_d_b4: f64 = (__rspice_deriv_cse_25 * p.p33);
        let eq56_e1389_d_b5: f64 = (__rspice_deriv_cse_26 * p.p33);
        let eq56_e1389_d_b6: f64 = (__rspice_deriv_cse_27 * p.p33);
        let eq56_e1389_d_b7: f64 = (__rspice_deriv_cse_28 * p.p33);
        let eq56_e1389_d_b8: f64 = (__rspice_deriv_cse_29 * p.p33);
        let eq56_e1389_d_b9: f64 = (__rspice_deriv_cse_30 * p.p33);
        let eq56_e1389_d_b10: f64 = (__rspice_deriv_cse_31 * p.p33);
        let eq56_e1389_d_b11: f64 = (__rspice_deriv_cse_32 * p.p33);
        let eq56_e1389_d_b12: f64 = (__rspice_deriv_cse_33 * p.p33);
        let eq56_e1389_d_b13: f64 = (__rspice_deriv_cse_34 * p.p33);
        let eq56_e1389_d_b14: f64 = (__rspice_deriv_cse_35 * p.p33);
        let eq56_e1389_d_b15: f64 = (__rspice_deriv_cse_36 * p.p33);
        let eq56_e1389_d_b16: f64 = (__rspice_deriv_cse_37 * p.p33);
        let eq56_e1389_d_b17: f64 = (__rspice_deriv_cse_38 * p.p33);
        let eq56_e1389_d_b18: f64 = (__rspice_deriv_cse_39 * p.p33);
        let eq56_e1389_d_b19: f64 = (__rspice_deriv_cse_40 * p.p33);
        let eq56_e1389_d_b20: f64 = (__rspice_deriv_cse_41 * p.p33);
        let eq56_e1389_d_b21: f64 = (__rspice_deriv_cse_42 * p.p33);
        let eq56_e1389_d_b22: f64 = (__rspice_deriv_cse_43 * p.p33);
        let eq56_e1389_d_b23: f64 = (__rspice_deriv_cse_44 * p.p33);
        let eq56_e1389_d_b24: f64 = (__rspice_deriv_cse_45 * p.p33);
        let eq56_e1391: f64 = (eq56_e1389 * s.v[851]);
        let eq56_e1391_d_n0: f64 = ((eq56_e1389_d_n0 * s.v[851]) + (eq56_e1389 * s.dn[851][0]));
        let eq56_e1391_d_n1: f64 = ((eq56_e1389_d_n1 * s.v[851]) + (eq56_e1389 * s.dn[851][1]));
        let eq56_e1391_d_n2: f64 = ((eq56_e1389_d_n2 * s.v[851]) + (eq56_e1389 * s.dn[851][2]));
        let eq56_e1391_d_n3: f64 = ((eq56_e1389_d_n3 * s.v[851]) + (eq56_e1389 * s.dn[851][3]));
        let eq56_e1391_d_n4: f64 = ((eq56_e1389_d_n4 * s.v[851]) + (eq56_e1389 * s.dn[851][4]));
        let eq56_e1391_d_n5: f64 = ((eq56_e1389_d_n5 * s.v[851]) + (eq56_e1389 * s.dn[851][5]));
        let eq56_e1391_d_n6: f64 = ((eq56_e1389_d_n6 * s.v[851]) + (eq56_e1389 * s.dn[851][6]));
        let eq56_e1391_d_n7: f64 = ((eq56_e1389_d_n7 * s.v[851]) + (eq56_e1389 * s.dn[851][7]));
        let eq56_e1391_d_n8: f64 = ((eq56_e1389_d_n8 * s.v[851]) + (eq56_e1389 * s.dn[851][8]));
        let eq56_e1391_d_n9: f64 = ((eq56_e1389_d_n9 * s.v[851]) + (eq56_e1389 * s.dn[851][9]));
        let eq56_e1391_d_n10: f64 = ((eq56_e1389_d_n10 * s.v[851]) + (eq56_e1389 * s.dn[851][10]));
        let eq56_e1391_d_n11: f64 = ((eq56_e1389_d_n11 * s.v[851]) + (eq56_e1389 * s.dn[851][11]));
        let eq56_e1391_d_n12: f64 = ((eq56_e1389_d_n12 * s.v[851]) + (eq56_e1389 * s.dn[851][12]));
        let eq56_e1391_d_n13: f64 = ((eq56_e1389_d_n13 * s.v[851]) + (eq56_e1389 * s.dn[851][13]));
        let eq56_e1391_d_n14: f64 = ((eq56_e1389_d_n14 * s.v[851]) + (eq56_e1389 * s.dn[851][14]));
        let eq56_e1391_d_n15: f64 = ((eq56_e1389_d_n15 * s.v[851]) + (eq56_e1389 * s.dn[851][15]));
        let eq56_e1391_d_n16: f64 = ((eq56_e1389_d_n16 * s.v[851]) + (eq56_e1389 * s.dn[851][16]));
        let eq56_e1391_d_n17: f64 = ((eq56_e1389_d_n17 * s.v[851]) + (eq56_e1389 * s.dn[851][17]));
        let eq56_e1391_d_n18: f64 = ((eq56_e1389_d_n18 * s.v[851]) + (eq56_e1389 * s.dn[851][18]));
        let eq56_e1391_d_n19: f64 = ((eq56_e1389_d_n19 * s.v[851]) + (eq56_e1389 * s.dn[851][19]));
        let eq56_e1391_d_n20: f64 = ((eq56_e1389_d_n20 * s.v[851]) + (eq56_e1389 * s.dn[851][20]));
        let eq56_e1391_d_b0: f64 = ((eq56_e1389_d_b0 * s.v[851]) + (eq56_e1389 * s.db[851][0]));
        let eq56_e1391_d_b1: f64 = ((eq56_e1389_d_b1 * s.v[851]) + (eq56_e1389 * s.db[851][1]));
        let eq56_e1391_d_b2: f64 = ((eq56_e1389_d_b2 * s.v[851]) + (eq56_e1389 * s.db[851][2]));
        let eq56_e1391_d_b3: f64 = ((eq56_e1389_d_b3 * s.v[851]) + (eq56_e1389 * s.db[851][3]));
        let eq56_e1391_d_b4: f64 = ((eq56_e1389_d_b4 * s.v[851]) + (eq56_e1389 * s.db[851][4]));
        let eq56_e1391_d_b5: f64 = ((eq56_e1389_d_b5 * s.v[851]) + (eq56_e1389 * s.db[851][5]));
        let eq56_e1391_d_b6: f64 = ((eq56_e1389_d_b6 * s.v[851]) + (eq56_e1389 * s.db[851][6]));
        let eq56_e1391_d_b7: f64 = ((eq56_e1389_d_b7 * s.v[851]) + (eq56_e1389 * s.db[851][7]));
        let eq56_e1391_d_b8: f64 = ((eq56_e1389_d_b8 * s.v[851]) + (eq56_e1389 * s.db[851][8]));
        let eq56_e1391_d_b9: f64 = ((eq56_e1389_d_b9 * s.v[851]) + (eq56_e1389 * s.db[851][9]));
        let eq56_e1391_d_b10: f64 = ((eq56_e1389_d_b10 * s.v[851]) + (eq56_e1389 * s.db[851][10]));
        let eq56_e1391_d_b11: f64 = ((eq56_e1389_d_b11 * s.v[851]) + (eq56_e1389 * s.db[851][11]));
        let eq56_e1391_d_b12: f64 = ((eq56_e1389_d_b12 * s.v[851]) + (eq56_e1389 * s.db[851][12]));
        let eq56_e1391_d_b13: f64 = ((eq56_e1389_d_b13 * s.v[851]) + (eq56_e1389 * s.db[851][13]));
        let eq56_e1391_d_b14: f64 = ((eq56_e1389_d_b14 * s.v[851]) + (eq56_e1389 * s.db[851][14]));
        let eq56_e1391_d_b15: f64 = ((eq56_e1389_d_b15 * s.v[851]) + (eq56_e1389 * s.db[851][15]));
        let eq56_e1391_d_b16: f64 = ((eq56_e1389_d_b16 * s.v[851]) + (eq56_e1389 * s.db[851][16]));
        let eq56_e1391_d_b17: f64 = ((eq56_e1389_d_b17 * s.v[851]) + (eq56_e1389 * s.db[851][17]));
        let eq56_e1391_d_b18: f64 = ((eq56_e1389_d_b18 * s.v[851]) + (eq56_e1389 * s.db[851][18]));
        let eq56_e1391_d_b19: f64 = ((eq56_e1389_d_b19 * s.v[851]) + (eq56_e1389 * s.db[851][19]));
        let eq56_e1391_d_b20: f64 = ((eq56_e1389_d_b20 * s.v[851]) + (eq56_e1389 * s.db[851][20]));
        let eq56_e1391_d_b21: f64 = ((eq56_e1389_d_b21 * s.v[851]) + (eq56_e1389 * s.db[851][21]));
        let eq56_e1391_d_b22: f64 = ((eq56_e1389_d_b22 * s.v[851]) + (eq56_e1389 * s.db[851][22]));
        let eq56_e1391_d_b23: f64 = ((eq56_e1389_d_b23 * s.v[851]) + (eq56_e1389 * s.db[851][23]));
        let eq56_e1391_d_b24: f64 = ((eq56_e1389_d_b24 * s.v[851]) + (eq56_e1389 * s.db[851][24]));
        let eq56_e1392_q: f64 = eq56_e1391;
        let eq56_reactive_node_derivatives: [f64; 21] = [eq56_e1391_d_n0, eq56_e1391_d_n1, eq56_e1391_d_n2, eq56_e1391_d_n3, eq56_e1391_d_n4, eq56_e1391_d_n5, eq56_e1391_d_n6, eq56_e1391_d_n7, eq56_e1391_d_n8, eq56_e1391_d_n9, eq56_e1391_d_n10, eq56_e1391_d_n11, eq56_e1391_d_n12, eq56_e1391_d_n13, eq56_e1391_d_n14, eq56_e1391_d_n15, eq56_e1391_d_n16, eq56_e1391_d_n17, eq56_e1391_d_n18, eq56_e1391_d_n19, eq56_e1391_d_n20];
        let eq56_reactive_branch_derivatives: [f64; 25] = [eq56_e1391_d_b0, eq56_e1391_d_b1, eq56_e1391_d_b2, eq56_e1391_d_b3, eq56_e1391_d_b4, eq56_e1391_d_b5, eq56_e1391_d_b6, eq56_e1391_d_b7, eq56_e1391_d_b8, eq56_e1391_d_b9, eq56_e1391_d_b10, eq56_e1391_d_b11, eq56_e1391_d_b12, eq56_e1391_d_b13, eq56_e1391_d_b14, eq56_e1391_d_b15, eq56_e1391_d_b16, eq56_e1391_d_b17, eq56_e1391_d_b18, eq56_e1391_d_b19, eq56_e1391_d_b20, eq56_e1391_d_b21, eq56_e1391_d_b22, eq56_e1391_d_b23, eq56_e1391_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let eq57_e1395: f64 = (s.v[0] * s.v[19]);
        let eq57_e1397: f64 = (eq57_e1395 * p.p33);
        let eq57_e1399: f64 = (eq57_e1397 * s.v[852]);
        let eq57_e1399_d_n0: f64 = ((eq56_e1389_d_n0 * s.v[852]) + (eq57_e1397 * s.dn[852][0]));
        let eq57_e1399_d_n1: f64 = ((eq56_e1389_d_n1 * s.v[852]) + (eq57_e1397 * s.dn[852][1]));
        let eq57_e1399_d_n2: f64 = ((eq56_e1389_d_n2 * s.v[852]) + (eq57_e1397 * s.dn[852][2]));
        let eq57_e1399_d_n3: f64 = ((eq56_e1389_d_n3 * s.v[852]) + (eq57_e1397 * s.dn[852][3]));
        let eq57_e1399_d_n4: f64 = ((eq56_e1389_d_n4 * s.v[852]) + (eq57_e1397 * s.dn[852][4]));
        let eq57_e1399_d_n5: f64 = ((eq56_e1389_d_n5 * s.v[852]) + (eq57_e1397 * s.dn[852][5]));
        let eq57_e1399_d_n6: f64 = ((eq56_e1389_d_n6 * s.v[852]) + (eq57_e1397 * s.dn[852][6]));
        let eq57_e1399_d_n7: f64 = ((eq56_e1389_d_n7 * s.v[852]) + (eq57_e1397 * s.dn[852][7]));
        let eq57_e1399_d_n8: f64 = ((eq56_e1389_d_n8 * s.v[852]) + (eq57_e1397 * s.dn[852][8]));
        let eq57_e1399_d_n9: f64 = ((eq56_e1389_d_n9 * s.v[852]) + (eq57_e1397 * s.dn[852][9]));
        let eq57_e1399_d_n10: f64 = ((eq56_e1389_d_n10 * s.v[852]) + (eq57_e1397 * s.dn[852][10]));
        let eq57_e1399_d_n11: f64 = ((eq56_e1389_d_n11 * s.v[852]) + (eq57_e1397 * s.dn[852][11]));
        let eq57_e1399_d_n12: f64 = ((eq56_e1389_d_n12 * s.v[852]) + (eq57_e1397 * s.dn[852][12]));
        let eq57_e1399_d_n13: f64 = ((eq56_e1389_d_n13 * s.v[852]) + (eq57_e1397 * s.dn[852][13]));
        let eq57_e1399_d_n14: f64 = ((eq56_e1389_d_n14 * s.v[852]) + (eq57_e1397 * s.dn[852][14]));
        let eq57_e1399_d_n15: f64 = ((eq56_e1389_d_n15 * s.v[852]) + (eq57_e1397 * s.dn[852][15]));
        let eq57_e1399_d_n16: f64 = ((eq56_e1389_d_n16 * s.v[852]) + (eq57_e1397 * s.dn[852][16]));
        let eq57_e1399_d_n17: f64 = ((eq56_e1389_d_n17 * s.v[852]) + (eq57_e1397 * s.dn[852][17]));
        let eq57_e1399_d_n18: f64 = ((eq56_e1389_d_n18 * s.v[852]) + (eq57_e1397 * s.dn[852][18]));
        let eq57_e1399_d_n19: f64 = ((eq56_e1389_d_n19 * s.v[852]) + (eq57_e1397 * s.dn[852][19]));
        let eq57_e1399_d_n20: f64 = ((eq56_e1389_d_n20 * s.v[852]) + (eq57_e1397 * s.dn[852][20]));
        let eq57_e1399_d_b0: f64 = ((eq56_e1389_d_b0 * s.v[852]) + (eq57_e1397 * s.db[852][0]));
        let eq57_e1399_d_b1: f64 = ((eq56_e1389_d_b1 * s.v[852]) + (eq57_e1397 * s.db[852][1]));
        let eq57_e1399_d_b2: f64 = ((eq56_e1389_d_b2 * s.v[852]) + (eq57_e1397 * s.db[852][2]));
        let eq57_e1399_d_b3: f64 = ((eq56_e1389_d_b3 * s.v[852]) + (eq57_e1397 * s.db[852][3]));
        let eq57_e1399_d_b4: f64 = ((eq56_e1389_d_b4 * s.v[852]) + (eq57_e1397 * s.db[852][4]));
        let eq57_e1399_d_b5: f64 = ((eq56_e1389_d_b5 * s.v[852]) + (eq57_e1397 * s.db[852][5]));
        let eq57_e1399_d_b6: f64 = ((eq56_e1389_d_b6 * s.v[852]) + (eq57_e1397 * s.db[852][6]));
        let eq57_e1399_d_b7: f64 = ((eq56_e1389_d_b7 * s.v[852]) + (eq57_e1397 * s.db[852][7]));
        let eq57_e1399_d_b8: f64 = ((eq56_e1389_d_b8 * s.v[852]) + (eq57_e1397 * s.db[852][8]));
        let eq57_e1399_d_b9: f64 = ((eq56_e1389_d_b9 * s.v[852]) + (eq57_e1397 * s.db[852][9]));
        let eq57_e1399_d_b10: f64 = ((eq56_e1389_d_b10 * s.v[852]) + (eq57_e1397 * s.db[852][10]));
        let eq57_e1399_d_b11: f64 = ((eq56_e1389_d_b11 * s.v[852]) + (eq57_e1397 * s.db[852][11]));
        let eq57_e1399_d_b12: f64 = ((eq56_e1389_d_b12 * s.v[852]) + (eq57_e1397 * s.db[852][12]));
        let eq57_e1399_d_b13: f64 = ((eq56_e1389_d_b13 * s.v[852]) + (eq57_e1397 * s.db[852][13]));
        let eq57_e1399_d_b14: f64 = ((eq56_e1389_d_b14 * s.v[852]) + (eq57_e1397 * s.db[852][14]));
        let eq57_e1399_d_b15: f64 = ((eq56_e1389_d_b15 * s.v[852]) + (eq57_e1397 * s.db[852][15]));
        let eq57_e1399_d_b16: f64 = ((eq56_e1389_d_b16 * s.v[852]) + (eq57_e1397 * s.db[852][16]));
        let eq57_e1399_d_b17: f64 = ((eq56_e1389_d_b17 * s.v[852]) + (eq57_e1397 * s.db[852][17]));
        let eq57_e1399_d_b18: f64 = ((eq56_e1389_d_b18 * s.v[852]) + (eq57_e1397 * s.db[852][18]));
        let eq57_e1399_d_b19: f64 = ((eq56_e1389_d_b19 * s.v[852]) + (eq57_e1397 * s.db[852][19]));
        let eq57_e1399_d_b20: f64 = ((eq56_e1389_d_b20 * s.v[852]) + (eq57_e1397 * s.db[852][20]));
        let eq57_e1399_d_b21: f64 = ((eq56_e1389_d_b21 * s.v[852]) + (eq57_e1397 * s.db[852][21]));
        let eq57_e1399_d_b22: f64 = ((eq56_e1389_d_b22 * s.v[852]) + (eq57_e1397 * s.db[852][22]));
        let eq57_e1399_d_b23: f64 = ((eq56_e1389_d_b23 * s.v[852]) + (eq57_e1397 * s.db[852][23]));
        let eq57_e1399_d_b24: f64 = ((eq56_e1389_d_b24 * s.v[852]) + (eq57_e1397 * s.db[852][24]));
        let eq57_e1400_q: f64 = eq57_e1399;
        let eq57_reactive_node_derivatives: [f64; 21] = [eq57_e1399_d_n0, eq57_e1399_d_n1, eq57_e1399_d_n2, eq57_e1399_d_n3, eq57_e1399_d_n4, eq57_e1399_d_n5, eq57_e1399_d_n6, eq57_e1399_d_n7, eq57_e1399_d_n8, eq57_e1399_d_n9, eq57_e1399_d_n10, eq57_e1399_d_n11, eq57_e1399_d_n12, eq57_e1399_d_n13, eq57_e1399_d_n14, eq57_e1399_d_n15, eq57_e1399_d_n16, eq57_e1399_d_n17, eq57_e1399_d_n18, eq57_e1399_d_n19, eq57_e1399_d_n20];
        let eq57_reactive_branch_derivatives: [f64; 25] = [eq57_e1399_d_b0, eq57_e1399_d_b1, eq57_e1399_d_b2, eq57_e1399_d_b3, eq57_e1399_d_b4, eq57_e1399_d_b5, eq57_e1399_d_b6, eq57_e1399_d_b7, eq57_e1399_d_b8, eq57_e1399_d_b9, eq57_e1399_d_b10, eq57_e1399_d_b11, eq57_e1399_d_b12, eq57_e1399_d_b13, eq57_e1399_d_b14, eq57_e1399_d_b15, eq57_e1399_d_b16, eq57_e1399_d_b17, eq57_e1399_d_b18, eq57_e1399_d_b19, eq57_e1399_d_b20, eq57_e1399_d_b21, eq57_e1399_d_b22, eq57_e1399_d_b23, eq57_e1399_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq57_reactive_node_derivatives,
            branches,
            &eq57_reactive_branch_derivatives,
            multiplicity,
        );
        let eq58_e1403: f64 = (s.v[0] * s.v[19]);
        let eq58_e1405: f64 = (eq58_e1403 * p.p33);
        let eq58_e1407: f64 = (eq58_e1405 * s.v[853]);
        let eq58_e1407_d_n0: f64 = ((eq56_e1389_d_n0 * s.v[853]) + (eq58_e1405 * s.dn[853][0]));
        let eq58_e1407_d_n1: f64 = ((eq56_e1389_d_n1 * s.v[853]) + (eq58_e1405 * s.dn[853][1]));
        let eq58_e1407_d_n2: f64 = ((eq56_e1389_d_n2 * s.v[853]) + (eq58_e1405 * s.dn[853][2]));
        let eq58_e1407_d_n3: f64 = ((eq56_e1389_d_n3 * s.v[853]) + (eq58_e1405 * s.dn[853][3]));
        let eq58_e1407_d_n4: f64 = ((eq56_e1389_d_n4 * s.v[853]) + (eq58_e1405 * s.dn[853][4]));
        let eq58_e1407_d_n5: f64 = ((eq56_e1389_d_n5 * s.v[853]) + (eq58_e1405 * s.dn[853][5]));
        let eq58_e1407_d_n6: f64 = ((eq56_e1389_d_n6 * s.v[853]) + (eq58_e1405 * s.dn[853][6]));
        let eq58_e1407_d_n7: f64 = ((eq56_e1389_d_n7 * s.v[853]) + (eq58_e1405 * s.dn[853][7]));
        let eq58_e1407_d_n8: f64 = ((eq56_e1389_d_n8 * s.v[853]) + (eq58_e1405 * s.dn[853][8]));
        let eq58_e1407_d_n9: f64 = ((eq56_e1389_d_n9 * s.v[853]) + (eq58_e1405 * s.dn[853][9]));
        let eq58_e1407_d_n10: f64 = ((eq56_e1389_d_n10 * s.v[853]) + (eq58_e1405 * s.dn[853][10]));
        let eq58_e1407_d_n11: f64 = ((eq56_e1389_d_n11 * s.v[853]) + (eq58_e1405 * s.dn[853][11]));
        let eq58_e1407_d_n12: f64 = ((eq56_e1389_d_n12 * s.v[853]) + (eq58_e1405 * s.dn[853][12]));
        let eq58_e1407_d_n13: f64 = ((eq56_e1389_d_n13 * s.v[853]) + (eq58_e1405 * s.dn[853][13]));
        let eq58_e1407_d_n14: f64 = ((eq56_e1389_d_n14 * s.v[853]) + (eq58_e1405 * s.dn[853][14]));
        let eq58_e1407_d_n15: f64 = ((eq56_e1389_d_n15 * s.v[853]) + (eq58_e1405 * s.dn[853][15]));
        let eq58_e1407_d_n16: f64 = ((eq56_e1389_d_n16 * s.v[853]) + (eq58_e1405 * s.dn[853][16]));
        let eq58_e1407_d_n17: f64 = ((eq56_e1389_d_n17 * s.v[853]) + (eq58_e1405 * s.dn[853][17]));
        let eq58_e1407_d_n18: f64 = ((eq56_e1389_d_n18 * s.v[853]) + (eq58_e1405 * s.dn[853][18]));
        let eq58_e1407_d_n19: f64 = ((eq56_e1389_d_n19 * s.v[853]) + (eq58_e1405 * s.dn[853][19]));
        let eq58_e1407_d_n20: f64 = ((eq56_e1389_d_n20 * s.v[853]) + (eq58_e1405 * s.dn[853][20]));
        let eq58_e1407_d_b0: f64 = ((eq56_e1389_d_b0 * s.v[853]) + (eq58_e1405 * s.db[853][0]));
        let eq58_e1407_d_b1: f64 = ((eq56_e1389_d_b1 * s.v[853]) + (eq58_e1405 * s.db[853][1]));
        let eq58_e1407_d_b2: f64 = ((eq56_e1389_d_b2 * s.v[853]) + (eq58_e1405 * s.db[853][2]));
        let eq58_e1407_d_b3: f64 = ((eq56_e1389_d_b3 * s.v[853]) + (eq58_e1405 * s.db[853][3]));
        let eq58_e1407_d_b4: f64 = ((eq56_e1389_d_b4 * s.v[853]) + (eq58_e1405 * s.db[853][4]));
        let eq58_e1407_d_b5: f64 = ((eq56_e1389_d_b5 * s.v[853]) + (eq58_e1405 * s.db[853][5]));
        let eq58_e1407_d_b6: f64 = ((eq56_e1389_d_b6 * s.v[853]) + (eq58_e1405 * s.db[853][6]));
        let eq58_e1407_d_b7: f64 = ((eq56_e1389_d_b7 * s.v[853]) + (eq58_e1405 * s.db[853][7]));
        let eq58_e1407_d_b8: f64 = ((eq56_e1389_d_b8 * s.v[853]) + (eq58_e1405 * s.db[853][8]));
        let eq58_e1407_d_b9: f64 = ((eq56_e1389_d_b9 * s.v[853]) + (eq58_e1405 * s.db[853][9]));
        let eq58_e1407_d_b10: f64 = ((eq56_e1389_d_b10 * s.v[853]) + (eq58_e1405 * s.db[853][10]));
        let eq58_e1407_d_b11: f64 = ((eq56_e1389_d_b11 * s.v[853]) + (eq58_e1405 * s.db[853][11]));
        let eq58_e1407_d_b12: f64 = ((eq56_e1389_d_b12 * s.v[853]) + (eq58_e1405 * s.db[853][12]));
        let eq58_e1407_d_b13: f64 = ((eq56_e1389_d_b13 * s.v[853]) + (eq58_e1405 * s.db[853][13]));
        let eq58_e1407_d_b14: f64 = ((eq56_e1389_d_b14 * s.v[853]) + (eq58_e1405 * s.db[853][14]));
        let eq58_e1407_d_b15: f64 = ((eq56_e1389_d_b15 * s.v[853]) + (eq58_e1405 * s.db[853][15]));
        let eq58_e1407_d_b16: f64 = ((eq56_e1389_d_b16 * s.v[853]) + (eq58_e1405 * s.db[853][16]));
        let eq58_e1407_d_b17: f64 = ((eq56_e1389_d_b17 * s.v[853]) + (eq58_e1405 * s.db[853][17]));
        let eq58_e1407_d_b18: f64 = ((eq56_e1389_d_b18 * s.v[853]) + (eq58_e1405 * s.db[853][18]));
        let eq58_e1407_d_b19: f64 = ((eq56_e1389_d_b19 * s.v[853]) + (eq58_e1405 * s.db[853][19]));
        let eq58_e1407_d_b20: f64 = ((eq56_e1389_d_b20 * s.v[853]) + (eq58_e1405 * s.db[853][20]));
        let eq58_e1407_d_b21: f64 = ((eq56_e1389_d_b21 * s.v[853]) + (eq58_e1405 * s.db[853][21]));
        let eq58_e1407_d_b22: f64 = ((eq56_e1389_d_b22 * s.v[853]) + (eq58_e1405 * s.db[853][22]));
        let eq58_e1407_d_b23: f64 = ((eq56_e1389_d_b23 * s.v[853]) + (eq58_e1405 * s.db[853][23]));
        let eq58_e1407_d_b24: f64 = ((eq56_e1389_d_b24 * s.v[853]) + (eq58_e1405 * s.db[853][24]));
        let eq58_e1408_q: f64 = eq58_e1407;
        let eq58_reactive_node_derivatives: [f64; 21] = [eq58_e1407_d_n0, eq58_e1407_d_n1, eq58_e1407_d_n2, eq58_e1407_d_n3, eq58_e1407_d_n4, eq58_e1407_d_n5, eq58_e1407_d_n6, eq58_e1407_d_n7, eq58_e1407_d_n8, eq58_e1407_d_n9, eq58_e1407_d_n10, eq58_e1407_d_n11, eq58_e1407_d_n12, eq58_e1407_d_n13, eq58_e1407_d_n14, eq58_e1407_d_n15, eq58_e1407_d_n16, eq58_e1407_d_n17, eq58_e1407_d_n18, eq58_e1407_d_n19, eq58_e1407_d_n20];
        let eq58_reactive_branch_derivatives: [f64; 25] = [eq58_e1407_d_b0, eq58_e1407_d_b1, eq58_e1407_d_b2, eq58_e1407_d_b3, eq58_e1407_d_b4, eq58_e1407_d_b5, eq58_e1407_d_b6, eq58_e1407_d_b7, eq58_e1407_d_b8, eq58_e1407_d_b9, eq58_e1407_d_b10, eq58_e1407_d_b11, eq58_e1407_d_b12, eq58_e1407_d_b13, eq58_e1407_d_b14, eq58_e1407_d_b15, eq58_e1407_d_b16, eq58_e1407_d_b17, eq58_e1407_d_b18, eq58_e1407_d_b19, eq58_e1407_d_b20, eq58_e1407_d_b21, eq58_e1407_d_b22, eq58_e1407_d_b23, eq58_e1407_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let __rspice_deriv_cse_17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let __rspice_deriv_cse_18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let __rspice_deriv_cse_19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let __rspice_deriv_cse_20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let __rspice_deriv_cse_21: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let __rspice_deriv_cse_22: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let __rspice_deriv_cse_23: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let __rspice_deriv_cse_24: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let __rspice_deriv_cse_25: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let __rspice_deriv_cse_26: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let __rspice_deriv_cse_27: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let __rspice_deriv_cse_28: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let __rspice_deriv_cse_29: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let __rspice_deriv_cse_30: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let __rspice_deriv_cse_31: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let __rspice_deriv_cse_32: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let __rspice_deriv_cse_33: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let __rspice_deriv_cse_34: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let __rspice_deriv_cse_35: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let __rspice_deriv_cse_36: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let __rspice_deriv_cse_37: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let __rspice_deriv_cse_38: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let __rspice_deriv_cse_39: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let __rspice_deriv_cse_40: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let __rspice_deriv_cse_41: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let __rspice_deriv_cse_42: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let __rspice_deriv_cse_43: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let __rspice_deriv_cse_44: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let __rspice_deriv_cse_45: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq59_e1411: f64 = (s.v[0] * s.v[19]);
        let eq59_e1413: f64 = (eq59_e1411 * p.p33);
        let eq59_e1413_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq59_e1413_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq59_e1413_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq59_e1413_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq59_e1413_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq59_e1413_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq59_e1413_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq59_e1413_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq59_e1413_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq59_e1413_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq59_e1413_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq59_e1413_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq59_e1413_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq59_e1413_d_n13: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq59_e1413_d_n14: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq59_e1413_d_n15: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq59_e1413_d_n16: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq59_e1413_d_n17: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq59_e1413_d_n18: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq59_e1413_d_n19: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq59_e1413_d_n20: f64 = (__rspice_deriv_cse_20 * p.p33);
        let eq59_e1413_d_b0: f64 = (__rspice_deriv_cse_21 * p.p33);
        let eq59_e1413_d_b1: f64 = (__rspice_deriv_cse_22 * p.p33);
        let eq59_e1413_d_b2: f64 = (__rspice_deriv_cse_23 * p.p33);
        let eq59_e1413_d_b3: f64 = (__rspice_deriv_cse_24 * p.p33);
        let eq59_e1413_d_b4: f64 = (__rspice_deriv_cse_25 * p.p33);
        let eq59_e1413_d_b5: f64 = (__rspice_deriv_cse_26 * p.p33);
        let eq59_e1413_d_b6: f64 = (__rspice_deriv_cse_27 * p.p33);
        let eq59_e1413_d_b7: f64 = (__rspice_deriv_cse_28 * p.p33);
        let eq59_e1413_d_b8: f64 = (__rspice_deriv_cse_29 * p.p33);
        let eq59_e1413_d_b9: f64 = (__rspice_deriv_cse_30 * p.p33);
        let eq59_e1413_d_b10: f64 = (__rspice_deriv_cse_31 * p.p33);
        let eq59_e1413_d_b11: f64 = (__rspice_deriv_cse_32 * p.p33);
        let eq59_e1413_d_b12: f64 = (__rspice_deriv_cse_33 * p.p33);
        let eq59_e1413_d_b13: f64 = (__rspice_deriv_cse_34 * p.p33);
        let eq59_e1413_d_b14: f64 = (__rspice_deriv_cse_35 * p.p33);
        let eq59_e1413_d_b15: f64 = (__rspice_deriv_cse_36 * p.p33);
        let eq59_e1413_d_b16: f64 = (__rspice_deriv_cse_37 * p.p33);
        let eq59_e1413_d_b17: f64 = (__rspice_deriv_cse_38 * p.p33);
        let eq59_e1413_d_b18: f64 = (__rspice_deriv_cse_39 * p.p33);
        let eq59_e1413_d_b19: f64 = (__rspice_deriv_cse_40 * p.p33);
        let eq59_e1413_d_b20: f64 = (__rspice_deriv_cse_41 * p.p33);
        let eq59_e1413_d_b21: f64 = (__rspice_deriv_cse_42 * p.p33);
        let eq59_e1413_d_b22: f64 = (__rspice_deriv_cse_43 * p.p33);
        let eq59_e1413_d_b23: f64 = (__rspice_deriv_cse_44 * p.p33);
        let eq59_e1413_d_b24: f64 = (__rspice_deriv_cse_45 * p.p33);
        let eq59_e1415: f64 = (eq59_e1413 * s.v[854]);
        let eq59_e1415_d_n0: f64 = ((eq59_e1413_d_n0 * s.v[854]) + (eq59_e1413 * s.dn[854][0]));
        let eq59_e1415_d_n1: f64 = ((eq59_e1413_d_n1 * s.v[854]) + (eq59_e1413 * s.dn[854][1]));
        let eq59_e1415_d_n2: f64 = ((eq59_e1413_d_n2 * s.v[854]) + (eq59_e1413 * s.dn[854][2]));
        let eq59_e1415_d_n3: f64 = ((eq59_e1413_d_n3 * s.v[854]) + (eq59_e1413 * s.dn[854][3]));
        let eq59_e1415_d_n4: f64 = ((eq59_e1413_d_n4 * s.v[854]) + (eq59_e1413 * s.dn[854][4]));
        let eq59_e1415_d_n5: f64 = ((eq59_e1413_d_n5 * s.v[854]) + (eq59_e1413 * s.dn[854][5]));
        let eq59_e1415_d_n6: f64 = ((eq59_e1413_d_n6 * s.v[854]) + (eq59_e1413 * s.dn[854][6]));
        let eq59_e1415_d_n7: f64 = ((eq59_e1413_d_n7 * s.v[854]) + (eq59_e1413 * s.dn[854][7]));
        let eq59_e1415_d_n8: f64 = ((eq59_e1413_d_n8 * s.v[854]) + (eq59_e1413 * s.dn[854][8]));
        let eq59_e1415_d_n9: f64 = ((eq59_e1413_d_n9 * s.v[854]) + (eq59_e1413 * s.dn[854][9]));
        let eq59_e1415_d_n10: f64 = ((eq59_e1413_d_n10 * s.v[854]) + (eq59_e1413 * s.dn[854][10]));
        let eq59_e1415_d_n11: f64 = ((eq59_e1413_d_n11 * s.v[854]) + (eq59_e1413 * s.dn[854][11]));
        let eq59_e1415_d_n12: f64 = ((eq59_e1413_d_n12 * s.v[854]) + (eq59_e1413 * s.dn[854][12]));
        let eq59_e1415_d_n13: f64 = ((eq59_e1413_d_n13 * s.v[854]) + (eq59_e1413 * s.dn[854][13]));
        let eq59_e1415_d_n14: f64 = ((eq59_e1413_d_n14 * s.v[854]) + (eq59_e1413 * s.dn[854][14]));
        let eq59_e1415_d_n15: f64 = ((eq59_e1413_d_n15 * s.v[854]) + (eq59_e1413 * s.dn[854][15]));
        let eq59_e1415_d_n16: f64 = ((eq59_e1413_d_n16 * s.v[854]) + (eq59_e1413 * s.dn[854][16]));
        let eq59_e1415_d_n17: f64 = ((eq59_e1413_d_n17 * s.v[854]) + (eq59_e1413 * s.dn[854][17]));
        let eq59_e1415_d_n18: f64 = ((eq59_e1413_d_n18 * s.v[854]) + (eq59_e1413 * s.dn[854][18]));
        let eq59_e1415_d_n19: f64 = ((eq59_e1413_d_n19 * s.v[854]) + (eq59_e1413 * s.dn[854][19]));
        let eq59_e1415_d_n20: f64 = ((eq59_e1413_d_n20 * s.v[854]) + (eq59_e1413 * s.dn[854][20]));
        let eq59_e1415_d_b0: f64 = ((eq59_e1413_d_b0 * s.v[854]) + (eq59_e1413 * s.db[854][0]));
        let eq59_e1415_d_b1: f64 = ((eq59_e1413_d_b1 * s.v[854]) + (eq59_e1413 * s.db[854][1]));
        let eq59_e1415_d_b2: f64 = ((eq59_e1413_d_b2 * s.v[854]) + (eq59_e1413 * s.db[854][2]));
        let eq59_e1415_d_b3: f64 = ((eq59_e1413_d_b3 * s.v[854]) + (eq59_e1413 * s.db[854][3]));
        let eq59_e1415_d_b4: f64 = ((eq59_e1413_d_b4 * s.v[854]) + (eq59_e1413 * s.db[854][4]));
        let eq59_e1415_d_b5: f64 = ((eq59_e1413_d_b5 * s.v[854]) + (eq59_e1413 * s.db[854][5]));
        let eq59_e1415_d_b6: f64 = ((eq59_e1413_d_b6 * s.v[854]) + (eq59_e1413 * s.db[854][6]));
        let eq59_e1415_d_b7: f64 = ((eq59_e1413_d_b7 * s.v[854]) + (eq59_e1413 * s.db[854][7]));
        let eq59_e1415_d_b8: f64 = ((eq59_e1413_d_b8 * s.v[854]) + (eq59_e1413 * s.db[854][8]));
        let eq59_e1415_d_b9: f64 = ((eq59_e1413_d_b9 * s.v[854]) + (eq59_e1413 * s.db[854][9]));
        let eq59_e1415_d_b10: f64 = ((eq59_e1413_d_b10 * s.v[854]) + (eq59_e1413 * s.db[854][10]));
        let eq59_e1415_d_b11: f64 = ((eq59_e1413_d_b11 * s.v[854]) + (eq59_e1413 * s.db[854][11]));
        let eq59_e1415_d_b12: f64 = ((eq59_e1413_d_b12 * s.v[854]) + (eq59_e1413 * s.db[854][12]));
        let eq59_e1415_d_b13: f64 = ((eq59_e1413_d_b13 * s.v[854]) + (eq59_e1413 * s.db[854][13]));
        let eq59_e1415_d_b14: f64 = ((eq59_e1413_d_b14 * s.v[854]) + (eq59_e1413 * s.db[854][14]));
        let eq59_e1415_d_b15: f64 = ((eq59_e1413_d_b15 * s.v[854]) + (eq59_e1413 * s.db[854][15]));
        let eq59_e1415_d_b16: f64 = ((eq59_e1413_d_b16 * s.v[854]) + (eq59_e1413 * s.db[854][16]));
        let eq59_e1415_d_b17: f64 = ((eq59_e1413_d_b17 * s.v[854]) + (eq59_e1413 * s.db[854][17]));
        let eq59_e1415_d_b18: f64 = ((eq59_e1413_d_b18 * s.v[854]) + (eq59_e1413 * s.db[854][18]));
        let eq59_e1415_d_b19: f64 = ((eq59_e1413_d_b19 * s.v[854]) + (eq59_e1413 * s.db[854][19]));
        let eq59_e1415_d_b20: f64 = ((eq59_e1413_d_b20 * s.v[854]) + (eq59_e1413 * s.db[854][20]));
        let eq59_e1415_d_b21: f64 = ((eq59_e1413_d_b21 * s.v[854]) + (eq59_e1413 * s.db[854][21]));
        let eq59_e1415_d_b22: f64 = ((eq59_e1413_d_b22 * s.v[854]) + (eq59_e1413 * s.db[854][22]));
        let eq59_e1415_d_b23: f64 = ((eq59_e1413_d_b23 * s.v[854]) + (eq59_e1413 * s.db[854][23]));
        let eq59_e1415_d_b24: f64 = ((eq59_e1413_d_b24 * s.v[854]) + (eq59_e1413 * s.db[854][24]));
        let eq59_e1416_q: f64 = eq59_e1415;
        let eq59_reactive_node_derivatives: [f64; 21] = [eq59_e1415_d_n0, eq59_e1415_d_n1, eq59_e1415_d_n2, eq59_e1415_d_n3, eq59_e1415_d_n4, eq59_e1415_d_n5, eq59_e1415_d_n6, eq59_e1415_d_n7, eq59_e1415_d_n8, eq59_e1415_d_n9, eq59_e1415_d_n10, eq59_e1415_d_n11, eq59_e1415_d_n12, eq59_e1415_d_n13, eq59_e1415_d_n14, eq59_e1415_d_n15, eq59_e1415_d_n16, eq59_e1415_d_n17, eq59_e1415_d_n18, eq59_e1415_d_n19, eq59_e1415_d_n20];
        let eq59_reactive_branch_derivatives: [f64; 25] = [eq59_e1415_d_b0, eq59_e1415_d_b1, eq59_e1415_d_b2, eq59_e1415_d_b3, eq59_e1415_d_b4, eq59_e1415_d_b5, eq59_e1415_d_b6, eq59_e1415_d_b7, eq59_e1415_d_b8, eq59_e1415_d_b9, eq59_e1415_d_b10, eq59_e1415_d_b11, eq59_e1415_d_b12, eq59_e1415_d_b13, eq59_e1415_d_b14, eq59_e1415_d_b15, eq59_e1415_d_b16, eq59_e1415_d_b17, eq59_e1415_d_b18, eq59_e1415_d_b19, eq59_e1415_d_b20, eq59_e1415_d_b21, eq59_e1415_d_b22, eq59_e1415_d_b23, eq59_e1415_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq59_reactive_node_derivatives,
            branches,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
        let eq60_e1419: f64 = (s.v[0] * s.v[19]);
        let eq60_e1421: f64 = (eq60_e1419 * p.p33);
        let eq60_e1423: f64 = (eq60_e1421 * s.v[855]);
        let eq60_e1423_d_n0: f64 = ((eq59_e1413_d_n0 * s.v[855]) + (eq60_e1421 * s.dn[855][0]));
        let eq60_e1423_d_n1: f64 = ((eq59_e1413_d_n1 * s.v[855]) + (eq60_e1421 * s.dn[855][1]));
        let eq60_e1423_d_n2: f64 = ((eq59_e1413_d_n2 * s.v[855]) + (eq60_e1421 * s.dn[855][2]));
        let eq60_e1423_d_n3: f64 = ((eq59_e1413_d_n3 * s.v[855]) + (eq60_e1421 * s.dn[855][3]));
        let eq60_e1423_d_n4: f64 = ((eq59_e1413_d_n4 * s.v[855]) + (eq60_e1421 * s.dn[855][4]));
        let eq60_e1423_d_n5: f64 = ((eq59_e1413_d_n5 * s.v[855]) + (eq60_e1421 * s.dn[855][5]));
        let eq60_e1423_d_n6: f64 = ((eq59_e1413_d_n6 * s.v[855]) + (eq60_e1421 * s.dn[855][6]));
        let eq60_e1423_d_n7: f64 = ((eq59_e1413_d_n7 * s.v[855]) + (eq60_e1421 * s.dn[855][7]));
        let eq60_e1423_d_n8: f64 = ((eq59_e1413_d_n8 * s.v[855]) + (eq60_e1421 * s.dn[855][8]));
        let eq60_e1423_d_n9: f64 = ((eq59_e1413_d_n9 * s.v[855]) + (eq60_e1421 * s.dn[855][9]));
        let eq60_e1423_d_n10: f64 = ((eq59_e1413_d_n10 * s.v[855]) + (eq60_e1421 * s.dn[855][10]));
        let eq60_e1423_d_n11: f64 = ((eq59_e1413_d_n11 * s.v[855]) + (eq60_e1421 * s.dn[855][11]));
        let eq60_e1423_d_n12: f64 = ((eq59_e1413_d_n12 * s.v[855]) + (eq60_e1421 * s.dn[855][12]));
        let eq60_e1423_d_n13: f64 = ((eq59_e1413_d_n13 * s.v[855]) + (eq60_e1421 * s.dn[855][13]));
        let eq60_e1423_d_n14: f64 = ((eq59_e1413_d_n14 * s.v[855]) + (eq60_e1421 * s.dn[855][14]));
        let eq60_e1423_d_n15: f64 = ((eq59_e1413_d_n15 * s.v[855]) + (eq60_e1421 * s.dn[855][15]));
        let eq60_e1423_d_n16: f64 = ((eq59_e1413_d_n16 * s.v[855]) + (eq60_e1421 * s.dn[855][16]));
        let eq60_e1423_d_n17: f64 = ((eq59_e1413_d_n17 * s.v[855]) + (eq60_e1421 * s.dn[855][17]));
        let eq60_e1423_d_n18: f64 = ((eq59_e1413_d_n18 * s.v[855]) + (eq60_e1421 * s.dn[855][18]));
        let eq60_e1423_d_n19: f64 = ((eq59_e1413_d_n19 * s.v[855]) + (eq60_e1421 * s.dn[855][19]));
        let eq60_e1423_d_n20: f64 = ((eq59_e1413_d_n20 * s.v[855]) + (eq60_e1421 * s.dn[855][20]));
        let eq60_e1423_d_b0: f64 = ((eq59_e1413_d_b0 * s.v[855]) + (eq60_e1421 * s.db[855][0]));
        let eq60_e1423_d_b1: f64 = ((eq59_e1413_d_b1 * s.v[855]) + (eq60_e1421 * s.db[855][1]));
        let eq60_e1423_d_b2: f64 = ((eq59_e1413_d_b2 * s.v[855]) + (eq60_e1421 * s.db[855][2]));
        let eq60_e1423_d_b3: f64 = ((eq59_e1413_d_b3 * s.v[855]) + (eq60_e1421 * s.db[855][3]));
        let eq60_e1423_d_b4: f64 = ((eq59_e1413_d_b4 * s.v[855]) + (eq60_e1421 * s.db[855][4]));
        let eq60_e1423_d_b5: f64 = ((eq59_e1413_d_b5 * s.v[855]) + (eq60_e1421 * s.db[855][5]));
        let eq60_e1423_d_b6: f64 = ((eq59_e1413_d_b6 * s.v[855]) + (eq60_e1421 * s.db[855][6]));
        let eq60_e1423_d_b7: f64 = ((eq59_e1413_d_b7 * s.v[855]) + (eq60_e1421 * s.db[855][7]));
        let eq60_e1423_d_b8: f64 = ((eq59_e1413_d_b8 * s.v[855]) + (eq60_e1421 * s.db[855][8]));
        let eq60_e1423_d_b9: f64 = ((eq59_e1413_d_b9 * s.v[855]) + (eq60_e1421 * s.db[855][9]));
        let eq60_e1423_d_b10: f64 = ((eq59_e1413_d_b10 * s.v[855]) + (eq60_e1421 * s.db[855][10]));
        let eq60_e1423_d_b11: f64 = ((eq59_e1413_d_b11 * s.v[855]) + (eq60_e1421 * s.db[855][11]));
        let eq60_e1423_d_b12: f64 = ((eq59_e1413_d_b12 * s.v[855]) + (eq60_e1421 * s.db[855][12]));
        let eq60_e1423_d_b13: f64 = ((eq59_e1413_d_b13 * s.v[855]) + (eq60_e1421 * s.db[855][13]));
        let eq60_e1423_d_b14: f64 = ((eq59_e1413_d_b14 * s.v[855]) + (eq60_e1421 * s.db[855][14]));
        let eq60_e1423_d_b15: f64 = ((eq59_e1413_d_b15 * s.v[855]) + (eq60_e1421 * s.db[855][15]));
        let eq60_e1423_d_b16: f64 = ((eq59_e1413_d_b16 * s.v[855]) + (eq60_e1421 * s.db[855][16]));
        let eq60_e1423_d_b17: f64 = ((eq59_e1413_d_b17 * s.v[855]) + (eq60_e1421 * s.db[855][17]));
        let eq60_e1423_d_b18: f64 = ((eq59_e1413_d_b18 * s.v[855]) + (eq60_e1421 * s.db[855][18]));
        let eq60_e1423_d_b19: f64 = ((eq59_e1413_d_b19 * s.v[855]) + (eq60_e1421 * s.db[855][19]));
        let eq60_e1423_d_b20: f64 = ((eq59_e1413_d_b20 * s.v[855]) + (eq60_e1421 * s.db[855][20]));
        let eq60_e1423_d_b21: f64 = ((eq59_e1413_d_b21 * s.v[855]) + (eq60_e1421 * s.db[855][21]));
        let eq60_e1423_d_b22: f64 = ((eq59_e1413_d_b22 * s.v[855]) + (eq60_e1421 * s.db[855][22]));
        let eq60_e1423_d_b23: f64 = ((eq59_e1413_d_b23 * s.v[855]) + (eq60_e1421 * s.db[855][23]));
        let eq60_e1423_d_b24: f64 = ((eq59_e1413_d_b24 * s.v[855]) + (eq60_e1421 * s.db[855][24]));
        let eq60_e1424_q: f64 = eq60_e1423;
        let eq60_reactive_node_derivatives: [f64; 21] = [eq60_e1423_d_n0, eq60_e1423_d_n1, eq60_e1423_d_n2, eq60_e1423_d_n3, eq60_e1423_d_n4, eq60_e1423_d_n5, eq60_e1423_d_n6, eq60_e1423_d_n7, eq60_e1423_d_n8, eq60_e1423_d_n9, eq60_e1423_d_n10, eq60_e1423_d_n11, eq60_e1423_d_n12, eq60_e1423_d_n13, eq60_e1423_d_n14, eq60_e1423_d_n15, eq60_e1423_d_n16, eq60_e1423_d_n17, eq60_e1423_d_n18, eq60_e1423_d_n19, eq60_e1423_d_n20];
        let eq60_reactive_branch_derivatives: [f64; 25] = [eq60_e1423_d_b0, eq60_e1423_d_b1, eq60_e1423_d_b2, eq60_e1423_d_b3, eq60_e1423_d_b4, eq60_e1423_d_b5, eq60_e1423_d_b6, eq60_e1423_d_b7, eq60_e1423_d_b8, eq60_e1423_d_b9, eq60_e1423_d_b10, eq60_e1423_d_b11, eq60_e1423_d_b12, eq60_e1423_d_b13, eq60_e1423_d_b14, eq60_e1423_d_b15, eq60_e1423_d_b16, eq60_e1423_d_b17, eq60_e1423_d_b18, eq60_e1423_d_b19, eq60_e1423_d_b20, eq60_e1423_d_b21, eq60_e1423_d_b22, eq60_e1423_d_b23, eq60_e1423_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq60_reactive_node_derivatives,
            branches,
            &eq60_reactive_branch_derivatives,
            multiplicity,
        );
        let eq61_e1427: f64 = (s.v[0] * s.v[19]);
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * s.v[856]);
        let eq61_e1431_d_n0: f64 = ((eq59_e1413_d_n0 * s.v[856]) + (eq61_e1429 * s.dn[856][0]));
        let eq61_e1431_d_n1: f64 = ((eq59_e1413_d_n1 * s.v[856]) + (eq61_e1429 * s.dn[856][1]));
        let eq61_e1431_d_n2: f64 = ((eq59_e1413_d_n2 * s.v[856]) + (eq61_e1429 * s.dn[856][2]));
        let eq61_e1431_d_n3: f64 = ((eq59_e1413_d_n3 * s.v[856]) + (eq61_e1429 * s.dn[856][3]));
        let eq61_e1431_d_n4: f64 = ((eq59_e1413_d_n4 * s.v[856]) + (eq61_e1429 * s.dn[856][4]));
        let eq61_e1431_d_n5: f64 = ((eq59_e1413_d_n5 * s.v[856]) + (eq61_e1429 * s.dn[856][5]));
        let eq61_e1431_d_n6: f64 = ((eq59_e1413_d_n6 * s.v[856]) + (eq61_e1429 * s.dn[856][6]));
        let eq61_e1431_d_n7: f64 = ((eq59_e1413_d_n7 * s.v[856]) + (eq61_e1429 * s.dn[856][7]));
        let eq61_e1431_d_n8: f64 = ((eq59_e1413_d_n8 * s.v[856]) + (eq61_e1429 * s.dn[856][8]));
        let eq61_e1431_d_n9: f64 = ((eq59_e1413_d_n9 * s.v[856]) + (eq61_e1429 * s.dn[856][9]));
        let eq61_e1431_d_n10: f64 = ((eq59_e1413_d_n10 * s.v[856]) + (eq61_e1429 * s.dn[856][10]));
        let eq61_e1431_d_n11: f64 = ((eq59_e1413_d_n11 * s.v[856]) + (eq61_e1429 * s.dn[856][11]));
        let eq61_e1431_d_n12: f64 = ((eq59_e1413_d_n12 * s.v[856]) + (eq61_e1429 * s.dn[856][12]));
        let eq61_e1431_d_n13: f64 = ((eq59_e1413_d_n13 * s.v[856]) + (eq61_e1429 * s.dn[856][13]));
        let eq61_e1431_d_n14: f64 = ((eq59_e1413_d_n14 * s.v[856]) + (eq61_e1429 * s.dn[856][14]));
        let eq61_e1431_d_n15: f64 = ((eq59_e1413_d_n15 * s.v[856]) + (eq61_e1429 * s.dn[856][15]));
        let eq61_e1431_d_n16: f64 = ((eq59_e1413_d_n16 * s.v[856]) + (eq61_e1429 * s.dn[856][16]));
        let eq61_e1431_d_n17: f64 = ((eq59_e1413_d_n17 * s.v[856]) + (eq61_e1429 * s.dn[856][17]));
        let eq61_e1431_d_n18: f64 = ((eq59_e1413_d_n18 * s.v[856]) + (eq61_e1429 * s.dn[856][18]));
        let eq61_e1431_d_n19: f64 = ((eq59_e1413_d_n19 * s.v[856]) + (eq61_e1429 * s.dn[856][19]));
        let eq61_e1431_d_n20: f64 = ((eq59_e1413_d_n20 * s.v[856]) + (eq61_e1429 * s.dn[856][20]));
        let eq61_e1431_d_b0: f64 = ((eq59_e1413_d_b0 * s.v[856]) + (eq61_e1429 * s.db[856][0]));
        let eq61_e1431_d_b1: f64 = ((eq59_e1413_d_b1 * s.v[856]) + (eq61_e1429 * s.db[856][1]));
        let eq61_e1431_d_b2: f64 = ((eq59_e1413_d_b2 * s.v[856]) + (eq61_e1429 * s.db[856][2]));
        let eq61_e1431_d_b3: f64 = ((eq59_e1413_d_b3 * s.v[856]) + (eq61_e1429 * s.db[856][3]));
        let eq61_e1431_d_b4: f64 = ((eq59_e1413_d_b4 * s.v[856]) + (eq61_e1429 * s.db[856][4]));
        let eq61_e1431_d_b5: f64 = ((eq59_e1413_d_b5 * s.v[856]) + (eq61_e1429 * s.db[856][5]));
        let eq61_e1431_d_b6: f64 = ((eq59_e1413_d_b6 * s.v[856]) + (eq61_e1429 * s.db[856][6]));
        let eq61_e1431_d_b7: f64 = ((eq59_e1413_d_b7 * s.v[856]) + (eq61_e1429 * s.db[856][7]));
        let eq61_e1431_d_b8: f64 = ((eq59_e1413_d_b8 * s.v[856]) + (eq61_e1429 * s.db[856][8]));
        let eq61_e1431_d_b9: f64 = ((eq59_e1413_d_b9 * s.v[856]) + (eq61_e1429 * s.db[856][9]));
        let eq61_e1431_d_b10: f64 = ((eq59_e1413_d_b10 * s.v[856]) + (eq61_e1429 * s.db[856][10]));
        let eq61_e1431_d_b11: f64 = ((eq59_e1413_d_b11 * s.v[856]) + (eq61_e1429 * s.db[856][11]));
        let eq61_e1431_d_b12: f64 = ((eq59_e1413_d_b12 * s.v[856]) + (eq61_e1429 * s.db[856][12]));
        let eq61_e1431_d_b13: f64 = ((eq59_e1413_d_b13 * s.v[856]) + (eq61_e1429 * s.db[856][13]));
        let eq61_e1431_d_b14: f64 = ((eq59_e1413_d_b14 * s.v[856]) + (eq61_e1429 * s.db[856][14]));
        let eq61_e1431_d_b15: f64 = ((eq59_e1413_d_b15 * s.v[856]) + (eq61_e1429 * s.db[856][15]));
        let eq61_e1431_d_b16: f64 = ((eq59_e1413_d_b16 * s.v[856]) + (eq61_e1429 * s.db[856][16]));
        let eq61_e1431_d_b17: f64 = ((eq59_e1413_d_b17 * s.v[856]) + (eq61_e1429 * s.db[856][17]));
        let eq61_e1431_d_b18: f64 = ((eq59_e1413_d_b18 * s.v[856]) + (eq61_e1429 * s.db[856][18]));
        let eq61_e1431_d_b19: f64 = ((eq59_e1413_d_b19 * s.v[856]) + (eq61_e1429 * s.db[856][19]));
        let eq61_e1431_d_b20: f64 = ((eq59_e1413_d_b20 * s.v[856]) + (eq61_e1429 * s.db[856][20]));
        let eq61_e1431_d_b21: f64 = ((eq59_e1413_d_b21 * s.v[856]) + (eq61_e1429 * s.db[856][21]));
        let eq61_e1431_d_b22: f64 = ((eq59_e1413_d_b22 * s.v[856]) + (eq61_e1429 * s.db[856][22]));
        let eq61_e1431_d_b23: f64 = ((eq59_e1413_d_b23 * s.v[856]) + (eq61_e1429 * s.db[856][23]));
        let eq61_e1431_d_b24: f64 = ((eq59_e1413_d_b24 * s.v[856]) + (eq61_e1429 * s.db[856][24]));
        let eq61_e1432_q: f64 = eq61_e1431;
        let eq61_reactive_node_derivatives: [f64; 21] = [eq61_e1431_d_n0, eq61_e1431_d_n1, eq61_e1431_d_n2, eq61_e1431_d_n3, eq61_e1431_d_n4, eq61_e1431_d_n5, eq61_e1431_d_n6, eq61_e1431_d_n7, eq61_e1431_d_n8, eq61_e1431_d_n9, eq61_e1431_d_n10, eq61_e1431_d_n11, eq61_e1431_d_n12, eq61_e1431_d_n13, eq61_e1431_d_n14, eq61_e1431_d_n15, eq61_e1431_d_n16, eq61_e1431_d_n17, eq61_e1431_d_n18, eq61_e1431_d_n19, eq61_e1431_d_n20];
        let eq61_reactive_branch_derivatives: [f64; 25] = [eq61_e1431_d_b0, eq61_e1431_d_b1, eq61_e1431_d_b2, eq61_e1431_d_b3, eq61_e1431_d_b4, eq61_e1431_d_b5, eq61_e1431_d_b6, eq61_e1431_d_b7, eq61_e1431_d_b8, eq61_e1431_d_b9, eq61_e1431_d_b10, eq61_e1431_d_b11, eq61_e1431_d_b12, eq61_e1431_d_b13, eq61_e1431_d_b14, eq61_e1431_d_b15, eq61_e1431_d_b16, eq61_e1431_d_b17, eq61_e1431_d_b18, eq61_e1431_d_b19, eq61_e1431_d_b20, eq61_e1431_d_b21, eq61_e1431_d_b22, eq61_e1431_d_b23, eq61_e1431_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let __rspice_deriv_cse_17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let __rspice_deriv_cse_18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let __rspice_deriv_cse_19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let __rspice_deriv_cse_20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let __rspice_deriv_cse_21: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let __rspice_deriv_cse_22: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let __rspice_deriv_cse_23: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let __rspice_deriv_cse_24: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let __rspice_deriv_cse_25: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let __rspice_deriv_cse_26: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let __rspice_deriv_cse_27: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let __rspice_deriv_cse_28: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let __rspice_deriv_cse_29: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let __rspice_deriv_cse_30: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let __rspice_deriv_cse_31: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let __rspice_deriv_cse_32: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let __rspice_deriv_cse_33: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let __rspice_deriv_cse_34: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let __rspice_deriv_cse_35: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let __rspice_deriv_cse_36: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let __rspice_deriv_cse_37: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let __rspice_deriv_cse_38: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let __rspice_deriv_cse_39: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let __rspice_deriv_cse_40: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let __rspice_deriv_cse_41: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let __rspice_deriv_cse_42: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let __rspice_deriv_cse_43: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let __rspice_deriv_cse_44: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let __rspice_deriv_cse_45: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq62_e1435: f64 = (s.v[0] * s.v[19]);
        let eq62_e1437: f64 = (eq62_e1435 * p.p33);
        let eq62_e1437_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq62_e1437_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq62_e1437_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq62_e1437_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq62_e1437_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq62_e1437_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq62_e1437_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq62_e1437_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq62_e1437_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq62_e1437_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq62_e1437_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq62_e1437_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq62_e1437_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq62_e1437_d_n13: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq62_e1437_d_n14: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq62_e1437_d_n15: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq62_e1437_d_n16: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq62_e1437_d_n17: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq62_e1437_d_n18: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq62_e1437_d_n19: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq62_e1437_d_n20: f64 = (__rspice_deriv_cse_20 * p.p33);
        let eq62_e1437_d_b0: f64 = (__rspice_deriv_cse_21 * p.p33);
        let eq62_e1437_d_b1: f64 = (__rspice_deriv_cse_22 * p.p33);
        let eq62_e1437_d_b2: f64 = (__rspice_deriv_cse_23 * p.p33);
        let eq62_e1437_d_b3: f64 = (__rspice_deriv_cse_24 * p.p33);
        let eq62_e1437_d_b4: f64 = (__rspice_deriv_cse_25 * p.p33);
        let eq62_e1437_d_b5: f64 = (__rspice_deriv_cse_26 * p.p33);
        let eq62_e1437_d_b6: f64 = (__rspice_deriv_cse_27 * p.p33);
        let eq62_e1437_d_b7: f64 = (__rspice_deriv_cse_28 * p.p33);
        let eq62_e1437_d_b8: f64 = (__rspice_deriv_cse_29 * p.p33);
        let eq62_e1437_d_b9: f64 = (__rspice_deriv_cse_30 * p.p33);
        let eq62_e1437_d_b10: f64 = (__rspice_deriv_cse_31 * p.p33);
        let eq62_e1437_d_b11: f64 = (__rspice_deriv_cse_32 * p.p33);
        let eq62_e1437_d_b12: f64 = (__rspice_deriv_cse_33 * p.p33);
        let eq62_e1437_d_b13: f64 = (__rspice_deriv_cse_34 * p.p33);
        let eq62_e1437_d_b14: f64 = (__rspice_deriv_cse_35 * p.p33);
        let eq62_e1437_d_b15: f64 = (__rspice_deriv_cse_36 * p.p33);
        let eq62_e1437_d_b16: f64 = (__rspice_deriv_cse_37 * p.p33);
        let eq62_e1437_d_b17: f64 = (__rspice_deriv_cse_38 * p.p33);
        let eq62_e1437_d_b18: f64 = (__rspice_deriv_cse_39 * p.p33);
        let eq62_e1437_d_b19: f64 = (__rspice_deriv_cse_40 * p.p33);
        let eq62_e1437_d_b20: f64 = (__rspice_deriv_cse_41 * p.p33);
        let eq62_e1437_d_b21: f64 = (__rspice_deriv_cse_42 * p.p33);
        let eq62_e1437_d_b22: f64 = (__rspice_deriv_cse_43 * p.p33);
        let eq62_e1437_d_b23: f64 = (__rspice_deriv_cse_44 * p.p33);
        let eq62_e1437_d_b24: f64 = (__rspice_deriv_cse_45 * p.p33);
        let eq62_e1439: f64 = (eq62_e1437 * s.v[857]);
        let eq62_e1439_d_n0: f64 = ((eq62_e1437_d_n0 * s.v[857]) + (eq62_e1437 * s.dn[857][0]));
        let eq62_e1439_d_n1: f64 = ((eq62_e1437_d_n1 * s.v[857]) + (eq62_e1437 * s.dn[857][1]));
        let eq62_e1439_d_n2: f64 = ((eq62_e1437_d_n2 * s.v[857]) + (eq62_e1437 * s.dn[857][2]));
        let eq62_e1439_d_n3: f64 = ((eq62_e1437_d_n3 * s.v[857]) + (eq62_e1437 * s.dn[857][3]));
        let eq62_e1439_d_n4: f64 = ((eq62_e1437_d_n4 * s.v[857]) + (eq62_e1437 * s.dn[857][4]));
        let eq62_e1439_d_n5: f64 = ((eq62_e1437_d_n5 * s.v[857]) + (eq62_e1437 * s.dn[857][5]));
        let eq62_e1439_d_n6: f64 = ((eq62_e1437_d_n6 * s.v[857]) + (eq62_e1437 * s.dn[857][6]));
        let eq62_e1439_d_n7: f64 = ((eq62_e1437_d_n7 * s.v[857]) + (eq62_e1437 * s.dn[857][7]));
        let eq62_e1439_d_n8: f64 = ((eq62_e1437_d_n8 * s.v[857]) + (eq62_e1437 * s.dn[857][8]));
        let eq62_e1439_d_n9: f64 = ((eq62_e1437_d_n9 * s.v[857]) + (eq62_e1437 * s.dn[857][9]));
        let eq62_e1439_d_n10: f64 = ((eq62_e1437_d_n10 * s.v[857]) + (eq62_e1437 * s.dn[857][10]));
        let eq62_e1439_d_n11: f64 = ((eq62_e1437_d_n11 * s.v[857]) + (eq62_e1437 * s.dn[857][11]));
        let eq62_e1439_d_n12: f64 = ((eq62_e1437_d_n12 * s.v[857]) + (eq62_e1437 * s.dn[857][12]));
        let eq62_e1439_d_n13: f64 = ((eq62_e1437_d_n13 * s.v[857]) + (eq62_e1437 * s.dn[857][13]));
        let eq62_e1439_d_n14: f64 = ((eq62_e1437_d_n14 * s.v[857]) + (eq62_e1437 * s.dn[857][14]));
        let eq62_e1439_d_n15: f64 = ((eq62_e1437_d_n15 * s.v[857]) + (eq62_e1437 * s.dn[857][15]));
        let eq62_e1439_d_n16: f64 = ((eq62_e1437_d_n16 * s.v[857]) + (eq62_e1437 * s.dn[857][16]));
        let eq62_e1439_d_n17: f64 = ((eq62_e1437_d_n17 * s.v[857]) + (eq62_e1437 * s.dn[857][17]));
        let eq62_e1439_d_n18: f64 = ((eq62_e1437_d_n18 * s.v[857]) + (eq62_e1437 * s.dn[857][18]));
        let eq62_e1439_d_n19: f64 = ((eq62_e1437_d_n19 * s.v[857]) + (eq62_e1437 * s.dn[857][19]));
        let eq62_e1439_d_n20: f64 = ((eq62_e1437_d_n20 * s.v[857]) + (eq62_e1437 * s.dn[857][20]));
        let eq62_e1439_d_b0: f64 = ((eq62_e1437_d_b0 * s.v[857]) + (eq62_e1437 * s.db[857][0]));
        let eq62_e1439_d_b1: f64 = ((eq62_e1437_d_b1 * s.v[857]) + (eq62_e1437 * s.db[857][1]));
        let eq62_e1439_d_b2: f64 = ((eq62_e1437_d_b2 * s.v[857]) + (eq62_e1437 * s.db[857][2]));
        let eq62_e1439_d_b3: f64 = ((eq62_e1437_d_b3 * s.v[857]) + (eq62_e1437 * s.db[857][3]));
        let eq62_e1439_d_b4: f64 = ((eq62_e1437_d_b4 * s.v[857]) + (eq62_e1437 * s.db[857][4]));
        let eq62_e1439_d_b5: f64 = ((eq62_e1437_d_b5 * s.v[857]) + (eq62_e1437 * s.db[857][5]));
        let eq62_e1439_d_b6: f64 = ((eq62_e1437_d_b6 * s.v[857]) + (eq62_e1437 * s.db[857][6]));
        let eq62_e1439_d_b7: f64 = ((eq62_e1437_d_b7 * s.v[857]) + (eq62_e1437 * s.db[857][7]));
        let eq62_e1439_d_b8: f64 = ((eq62_e1437_d_b8 * s.v[857]) + (eq62_e1437 * s.db[857][8]));
        let eq62_e1439_d_b9: f64 = ((eq62_e1437_d_b9 * s.v[857]) + (eq62_e1437 * s.db[857][9]));
        let eq62_e1439_d_b10: f64 = ((eq62_e1437_d_b10 * s.v[857]) + (eq62_e1437 * s.db[857][10]));
        let eq62_e1439_d_b11: f64 = ((eq62_e1437_d_b11 * s.v[857]) + (eq62_e1437 * s.db[857][11]));
        let eq62_e1439_d_b12: f64 = ((eq62_e1437_d_b12 * s.v[857]) + (eq62_e1437 * s.db[857][12]));
        let eq62_e1439_d_b13: f64 = ((eq62_e1437_d_b13 * s.v[857]) + (eq62_e1437 * s.db[857][13]));
        let eq62_e1439_d_b14: f64 = ((eq62_e1437_d_b14 * s.v[857]) + (eq62_e1437 * s.db[857][14]));
        let eq62_e1439_d_b15: f64 = ((eq62_e1437_d_b15 * s.v[857]) + (eq62_e1437 * s.db[857][15]));
        let eq62_e1439_d_b16: f64 = ((eq62_e1437_d_b16 * s.v[857]) + (eq62_e1437 * s.db[857][16]));
        let eq62_e1439_d_b17: f64 = ((eq62_e1437_d_b17 * s.v[857]) + (eq62_e1437 * s.db[857][17]));
        let eq62_e1439_d_b18: f64 = ((eq62_e1437_d_b18 * s.v[857]) + (eq62_e1437 * s.db[857][18]));
        let eq62_e1439_d_b19: f64 = ((eq62_e1437_d_b19 * s.v[857]) + (eq62_e1437 * s.db[857][19]));
        let eq62_e1439_d_b20: f64 = ((eq62_e1437_d_b20 * s.v[857]) + (eq62_e1437 * s.db[857][20]));
        let eq62_e1439_d_b21: f64 = ((eq62_e1437_d_b21 * s.v[857]) + (eq62_e1437 * s.db[857][21]));
        let eq62_e1439_d_b22: f64 = ((eq62_e1437_d_b22 * s.v[857]) + (eq62_e1437 * s.db[857][22]));
        let eq62_e1439_d_b23: f64 = ((eq62_e1437_d_b23 * s.v[857]) + (eq62_e1437 * s.db[857][23]));
        let eq62_e1439_d_b24: f64 = ((eq62_e1437_d_b24 * s.v[857]) + (eq62_e1437 * s.db[857][24]));
        let eq62_e1440_q: f64 = eq62_e1439;
        let eq62_reactive_node_derivatives: [f64; 21] = [eq62_e1439_d_n0, eq62_e1439_d_n1, eq62_e1439_d_n2, eq62_e1439_d_n3, eq62_e1439_d_n4, eq62_e1439_d_n5, eq62_e1439_d_n6, eq62_e1439_d_n7, eq62_e1439_d_n8, eq62_e1439_d_n9, eq62_e1439_d_n10, eq62_e1439_d_n11, eq62_e1439_d_n12, eq62_e1439_d_n13, eq62_e1439_d_n14, eq62_e1439_d_n15, eq62_e1439_d_n16, eq62_e1439_d_n17, eq62_e1439_d_n18, eq62_e1439_d_n19, eq62_e1439_d_n20];
        let eq62_reactive_branch_derivatives: [f64; 25] = [eq62_e1439_d_b0, eq62_e1439_d_b1, eq62_e1439_d_b2, eq62_e1439_d_b3, eq62_e1439_d_b4, eq62_e1439_d_b5, eq62_e1439_d_b6, eq62_e1439_d_b7, eq62_e1439_d_b8, eq62_e1439_d_b9, eq62_e1439_d_b10, eq62_e1439_d_b11, eq62_e1439_d_b12, eq62_e1439_d_b13, eq62_e1439_d_b14, eq62_e1439_d_b15, eq62_e1439_d_b16, eq62_e1439_d_b17, eq62_e1439_d_b18, eq62_e1439_d_b19, eq62_e1439_d_b20, eq62_e1439_d_b21, eq62_e1439_d_b22, eq62_e1439_d_b23, eq62_e1439_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq62_reactive_node_derivatives,
            branches,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let eq63_e1443: f64 = (s.v[0] * s.v[19]);
        let eq63_e1445: f64 = (eq63_e1443 * p.p33);
        let eq63_e1447: f64 = (eq63_e1445 * s.v[858]);
        let eq63_e1447_d_n0: f64 = ((eq62_e1437_d_n0 * s.v[858]) + (eq63_e1445 * s.dn[858][0]));
        let eq63_e1447_d_n1: f64 = ((eq62_e1437_d_n1 * s.v[858]) + (eq63_e1445 * s.dn[858][1]));
        let eq63_e1447_d_n2: f64 = ((eq62_e1437_d_n2 * s.v[858]) + (eq63_e1445 * s.dn[858][2]));
        let eq63_e1447_d_n3: f64 = ((eq62_e1437_d_n3 * s.v[858]) + (eq63_e1445 * s.dn[858][3]));
        let eq63_e1447_d_n4: f64 = ((eq62_e1437_d_n4 * s.v[858]) + (eq63_e1445 * s.dn[858][4]));
        let eq63_e1447_d_n5: f64 = ((eq62_e1437_d_n5 * s.v[858]) + (eq63_e1445 * s.dn[858][5]));
        let eq63_e1447_d_n6: f64 = ((eq62_e1437_d_n6 * s.v[858]) + (eq63_e1445 * s.dn[858][6]));
        let eq63_e1447_d_n7: f64 = ((eq62_e1437_d_n7 * s.v[858]) + (eq63_e1445 * s.dn[858][7]));
        let eq63_e1447_d_n8: f64 = ((eq62_e1437_d_n8 * s.v[858]) + (eq63_e1445 * s.dn[858][8]));
        let eq63_e1447_d_n9: f64 = ((eq62_e1437_d_n9 * s.v[858]) + (eq63_e1445 * s.dn[858][9]));
        let eq63_e1447_d_n10: f64 = ((eq62_e1437_d_n10 * s.v[858]) + (eq63_e1445 * s.dn[858][10]));
        let eq63_e1447_d_n11: f64 = ((eq62_e1437_d_n11 * s.v[858]) + (eq63_e1445 * s.dn[858][11]));
        let eq63_e1447_d_n12: f64 = ((eq62_e1437_d_n12 * s.v[858]) + (eq63_e1445 * s.dn[858][12]));
        let eq63_e1447_d_n13: f64 = ((eq62_e1437_d_n13 * s.v[858]) + (eq63_e1445 * s.dn[858][13]));
        let eq63_e1447_d_n14: f64 = ((eq62_e1437_d_n14 * s.v[858]) + (eq63_e1445 * s.dn[858][14]));
        let eq63_e1447_d_n15: f64 = ((eq62_e1437_d_n15 * s.v[858]) + (eq63_e1445 * s.dn[858][15]));
        let eq63_e1447_d_n16: f64 = ((eq62_e1437_d_n16 * s.v[858]) + (eq63_e1445 * s.dn[858][16]));
        let eq63_e1447_d_n17: f64 = ((eq62_e1437_d_n17 * s.v[858]) + (eq63_e1445 * s.dn[858][17]));
        let eq63_e1447_d_n18: f64 = ((eq62_e1437_d_n18 * s.v[858]) + (eq63_e1445 * s.dn[858][18]));
        let eq63_e1447_d_n19: f64 = ((eq62_e1437_d_n19 * s.v[858]) + (eq63_e1445 * s.dn[858][19]));
        let eq63_e1447_d_n20: f64 = ((eq62_e1437_d_n20 * s.v[858]) + (eq63_e1445 * s.dn[858][20]));
        let eq63_e1447_d_b0: f64 = ((eq62_e1437_d_b0 * s.v[858]) + (eq63_e1445 * s.db[858][0]));
        let eq63_e1447_d_b1: f64 = ((eq62_e1437_d_b1 * s.v[858]) + (eq63_e1445 * s.db[858][1]));
        let eq63_e1447_d_b2: f64 = ((eq62_e1437_d_b2 * s.v[858]) + (eq63_e1445 * s.db[858][2]));
        let eq63_e1447_d_b3: f64 = ((eq62_e1437_d_b3 * s.v[858]) + (eq63_e1445 * s.db[858][3]));
        let eq63_e1447_d_b4: f64 = ((eq62_e1437_d_b4 * s.v[858]) + (eq63_e1445 * s.db[858][4]));
        let eq63_e1447_d_b5: f64 = ((eq62_e1437_d_b5 * s.v[858]) + (eq63_e1445 * s.db[858][5]));
        let eq63_e1447_d_b6: f64 = ((eq62_e1437_d_b6 * s.v[858]) + (eq63_e1445 * s.db[858][6]));
        let eq63_e1447_d_b7: f64 = ((eq62_e1437_d_b7 * s.v[858]) + (eq63_e1445 * s.db[858][7]));
        let eq63_e1447_d_b8: f64 = ((eq62_e1437_d_b8 * s.v[858]) + (eq63_e1445 * s.db[858][8]));
        let eq63_e1447_d_b9: f64 = ((eq62_e1437_d_b9 * s.v[858]) + (eq63_e1445 * s.db[858][9]));
        let eq63_e1447_d_b10: f64 = ((eq62_e1437_d_b10 * s.v[858]) + (eq63_e1445 * s.db[858][10]));
        let eq63_e1447_d_b11: f64 = ((eq62_e1437_d_b11 * s.v[858]) + (eq63_e1445 * s.db[858][11]));
        let eq63_e1447_d_b12: f64 = ((eq62_e1437_d_b12 * s.v[858]) + (eq63_e1445 * s.db[858][12]));
        let eq63_e1447_d_b13: f64 = ((eq62_e1437_d_b13 * s.v[858]) + (eq63_e1445 * s.db[858][13]));
        let eq63_e1447_d_b14: f64 = ((eq62_e1437_d_b14 * s.v[858]) + (eq63_e1445 * s.db[858][14]));
        let eq63_e1447_d_b15: f64 = ((eq62_e1437_d_b15 * s.v[858]) + (eq63_e1445 * s.db[858][15]));
        let eq63_e1447_d_b16: f64 = ((eq62_e1437_d_b16 * s.v[858]) + (eq63_e1445 * s.db[858][16]));
        let eq63_e1447_d_b17: f64 = ((eq62_e1437_d_b17 * s.v[858]) + (eq63_e1445 * s.db[858][17]));
        let eq63_e1447_d_b18: f64 = ((eq62_e1437_d_b18 * s.v[858]) + (eq63_e1445 * s.db[858][18]));
        let eq63_e1447_d_b19: f64 = ((eq62_e1437_d_b19 * s.v[858]) + (eq63_e1445 * s.db[858][19]));
        let eq63_e1447_d_b20: f64 = ((eq62_e1437_d_b20 * s.v[858]) + (eq63_e1445 * s.db[858][20]));
        let eq63_e1447_d_b21: f64 = ((eq62_e1437_d_b21 * s.v[858]) + (eq63_e1445 * s.db[858][21]));
        let eq63_e1447_d_b22: f64 = ((eq62_e1437_d_b22 * s.v[858]) + (eq63_e1445 * s.db[858][22]));
        let eq63_e1447_d_b23: f64 = ((eq62_e1437_d_b23 * s.v[858]) + (eq63_e1445 * s.db[858][23]));
        let eq63_e1447_d_b24: f64 = ((eq62_e1437_d_b24 * s.v[858]) + (eq63_e1445 * s.db[858][24]));
        let eq63_e1448_q: f64 = eq63_e1447;
        let eq63_reactive_node_derivatives: [f64; 21] = [eq63_e1447_d_n0, eq63_e1447_d_n1, eq63_e1447_d_n2, eq63_e1447_d_n3, eq63_e1447_d_n4, eq63_e1447_d_n5, eq63_e1447_d_n6, eq63_e1447_d_n7, eq63_e1447_d_n8, eq63_e1447_d_n9, eq63_e1447_d_n10, eq63_e1447_d_n11, eq63_e1447_d_n12, eq63_e1447_d_n13, eq63_e1447_d_n14, eq63_e1447_d_n15, eq63_e1447_d_n16, eq63_e1447_d_n17, eq63_e1447_d_n18, eq63_e1447_d_n19, eq63_e1447_d_n20];
        let eq63_reactive_branch_derivatives: [f64; 25] = [eq63_e1447_d_b0, eq63_e1447_d_b1, eq63_e1447_d_b2, eq63_e1447_d_b3, eq63_e1447_d_b4, eq63_e1447_d_b5, eq63_e1447_d_b6, eq63_e1447_d_b7, eq63_e1447_d_b8, eq63_e1447_d_b9, eq63_e1447_d_b10, eq63_e1447_d_b11, eq63_e1447_d_b12, eq63_e1447_d_b13, eq63_e1447_d_b14, eq63_e1447_d_b15, eq63_e1447_d_b16, eq63_e1447_d_b17, eq63_e1447_d_b18, eq63_e1447_d_b19, eq63_e1447_d_b20, eq63_e1447_d_b21, eq63_e1447_d_b22, eq63_e1447_d_b23, eq63_e1447_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq63_reactive_node_derivatives,
            branches,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let eq66_e1459: f64 = (s.v[860] * (nv4 - 0.0));
        let eq66_e1459_d_n0: f64 = (s.dn[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_n1: f64 = (s.dn[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_n2: f64 = (s.dn[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_n3: f64 = (s.dn[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_n4: f64 = ((s.dn[860][4] * (nv4 - 0.0)) + s.v[860]);
        let eq66_e1459_d_n5: f64 = (s.dn[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (s.dn[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (s.dn[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (s.dn[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_n9: f64 = (s.dn[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_n10: f64 = (s.dn[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_n11: f64 = (s.dn[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (s.dn[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (s.dn[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (s.dn[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (s.dn[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (s.dn[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (s.dn[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (s.dn[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (s.dn[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (s.dn[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b0: f64 = (s.db[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_b1: f64 = (s.db[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_b2: f64 = (s.db[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_b3: f64 = (s.db[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_b4: f64 = (s.db[860][4] * (nv4 - 0.0));
        let eq66_e1459_d_b5: f64 = (s.db[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_b6: f64 = (s.db[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_b7: f64 = (s.db[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_b8: f64 = (s.db[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_b9: f64 = (s.db[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_b10: f64 = (s.db[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_b11: f64 = (s.db[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_b12: f64 = (s.db[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_b13: f64 = (s.db[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_b14: f64 = (s.db[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_b15: f64 = (s.db[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_b16: f64 = (s.db[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_b17: f64 = (s.db[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_b18: f64 = (s.db[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_b19: f64 = (s.db[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_b20: f64 = (s.db[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b21: f64 = (s.db[860][21] * (nv4 - 0.0));
        let eq66_e1459_d_b22: f64 = (s.db[860][22] * (nv4 - 0.0));
        let eq66_e1459_d_b23: f64 = (s.db[860][23] * (nv4 - 0.0));
        let eq66_e1459_d_b24: f64 = (s.db[860][24] * (nv4 - 0.0));
        let eq66_e1460_q: f64 = eq66_e1459;
        let eq66_reactive_node_derivatives: [f64; 21] = [eq66_e1459_d_n0, eq66_e1459_d_n1, eq66_e1459_d_n2, eq66_e1459_d_n3, eq66_e1459_d_n4, eq66_e1459_d_n5, eq66_e1459_d_n6, eq66_e1459_d_n7, eq66_e1459_d_n8, eq66_e1459_d_n9, eq66_e1459_d_n10, eq66_e1459_d_n11, eq66_e1459_d_n12, eq66_e1459_d_n13, eq66_e1459_d_n14, eq66_e1459_d_n15, eq66_e1459_d_n16, eq66_e1459_d_n17, eq66_e1459_d_n18, eq66_e1459_d_n19, eq66_e1459_d_n20];
        let eq66_reactive_branch_derivatives: [f64; 25] = [eq66_e1459_d_b0, eq66_e1459_d_b1, eq66_e1459_d_b2, eq66_e1459_d_b3, eq66_e1459_d_b4, eq66_e1459_d_b5, eq66_e1459_d_b6, eq66_e1459_d_b7, eq66_e1459_d_b8, eq66_e1459_d_b9, eq66_e1459_d_b10, eq66_e1459_d_b11, eq66_e1459_d_b12, eq66_e1459_d_b13, eq66_e1459_d_b14, eq66_e1459_d_b15, eq66_e1459_d_b16, eq66_e1459_d_b17, eq66_e1459_d_b18, eq66_e1459_d_b19, eq66_e1459_d_b20, eq66_e1459_d_b21, eq66_e1459_d_b22, eq66_e1459_d_b23, eq66_e1459_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq67_e1463: f64 = (s.v[19] * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq67_e1464);
        let eq67_e1464_d_n0: f64 = ((s.dn[19][0] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n1: f64 = ((s.dn[19][1] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n2: f64 = ((s.dn[19][2] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n3: f64 = ((s.dn[19][3] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n4: f64 = ((s.dn[19][4] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n5: f64 = ((s.dn[19][5] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n6: f64 = ((s.dn[19][6] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n7: f64 = ((s.dn[19][7] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n8: f64 = ((s.dn[19][8] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n9: f64 = ((s.dn[19][9] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n10: f64 = ((s.dn[19][10] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n11: f64 = ((s.dn[19][11] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n12: f64 = ((s.dn[19][12] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n13: f64 = ((s.dn[19][13] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n14: f64 = ((s.dn[19][14] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n15: f64 = ((s.dn[19][15] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n16: f64 = ((s.dn[19][16] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n17: f64 = ((s.dn[19][17] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n18: f64 = ((s.dn[19][18] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n19: f64 = ((s.dn[19][19] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n20: f64 = ((s.dn[19][20] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b0: f64 = ((s.db[19][0] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b1: f64 = ((s.db[19][1] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b2: f64 = ((s.db[19][2] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b3: f64 = ((s.db[19][3] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b4: f64 = ((s.db[19][4] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b5: f64 = ((s.db[19][5] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b6: f64 = ((s.db[19][6] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b7: f64 = ((s.db[19][7] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b8: f64 = ((s.db[19][8] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b9: f64 = ((s.db[19][9] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b10: f64 = ((s.db[19][10] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b11: f64 = ((s.db[19][11] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b12: f64 = ((s.db[19][12] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b13: f64 = ((s.db[19][13] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b14: f64 = ((s.db[19][14] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b15: f64 = ((s.db[19][15] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b16: f64 = ((s.db[19][16] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b17: f64 = ((s.db[19][17] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b18: f64 = ((s.db[19][18] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b19: f64 = ((s.db[19][19] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b20: f64 = ((s.db[19][20] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b21: f64 = ((s.db[19][21] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b22: f64 = ((s.db[19][22] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b23: f64 = ((s.db[19][23] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b24: f64 = ((s.db[19][24] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1466_d_n0: f64 = (eq67_e1464_d_n0 * 0.5);
        let eq67_e1466_d_n1: f64 = (eq67_e1464_d_n1 * 0.5);
        let eq67_e1466_d_n2: f64 = (eq67_e1464_d_n2 * 0.5);
        let eq67_e1466_d_n3: f64 = (eq67_e1464_d_n3 * 0.5);
        let eq67_e1466_d_n4: f64 = (eq67_e1464_d_n4 * 0.5);
        let eq67_e1466_d_n5: f64 = (eq67_e1464_d_n5 * 0.5);
        let eq67_e1466_d_n6: f64 = (eq67_e1464_d_n6 * 0.5);
        let eq67_e1466_d_n7: f64 = (eq67_e1464_d_n7 * 0.5);
        let eq67_e1466_d_n8: f64 = (eq67_e1464_d_n8 * 0.5);
        let eq67_e1466_d_n9: f64 = (eq67_e1464_d_n9 * 0.5);
        let eq67_e1466_d_n10: f64 = (eq67_e1464_d_n10 * 0.5);
        let eq67_e1466_d_n11: f64 = (eq67_e1464_d_n11 * 0.5);
        let eq67_e1466_d_n12: f64 = (eq67_e1464_d_n12 * 0.5);
        let eq67_e1466_d_n13: f64 = (eq67_e1464_d_n13 * 0.5);
        let eq67_e1466_d_n14: f64 = (eq67_e1464_d_n14 * 0.5);
        let eq67_e1466_d_n15: f64 = (eq67_e1464_d_n15 * 0.5);
        let eq67_e1466_d_n16: f64 = (eq67_e1464_d_n16 * 0.5);
        let eq67_e1466_d_n17: f64 = (eq67_e1464_d_n17 * 0.5);
        let eq67_e1466_d_n18: f64 = (eq67_e1464_d_n18 * 0.5);
        let eq67_e1466_d_n19: f64 = (eq67_e1464_d_n19 * 0.5);
        let eq67_e1466_d_n20: f64 = (eq67_e1464_d_n20 * 0.5);
        let eq67_e1466_d_b0: f64 = (eq67_e1464_d_b0 * 0.5);
        let eq67_e1466_d_b1: f64 = (eq67_e1464_d_b1 * 0.5);
        let eq67_e1466_d_b2: f64 = (eq67_e1464_d_b2 * 0.5);
        let eq67_e1466_d_b3: f64 = (eq67_e1464_d_b3 * 0.5);
        let eq67_e1466_d_b4: f64 = (eq67_e1464_d_b4 * 0.5);
        let eq67_e1466_d_b5: f64 = (eq67_e1464_d_b5 * 0.5);
        let eq67_e1466_d_b6: f64 = (eq67_e1464_d_b6 * 0.5);
        let eq67_e1466_d_b7: f64 = (eq67_e1464_d_b7 * 0.5);
        let eq67_e1466_d_b8: f64 = (eq67_e1464_d_b8 * 0.5);
        let eq67_e1466_d_b9: f64 = (eq67_e1464_d_b9 * 0.5);
        let eq67_e1466_d_b10: f64 = (eq67_e1464_d_b10 * 0.5);
        let eq67_e1466_d_b11: f64 = (eq67_e1464_d_b11 * 0.5);
        let eq67_e1466_d_b12: f64 = (eq67_e1464_d_b12 * 0.5);
        let eq67_e1466_d_b13: f64 = (eq67_e1464_d_b13 * 0.5);
        let eq67_e1466_d_b14: f64 = (eq67_e1464_d_b14 * 0.5);
        let eq67_e1466_d_b15: f64 = (eq67_e1464_d_b15 * 0.5);
        let eq67_e1466_d_b16: f64 = (eq67_e1464_d_b16 * 0.5);
        let eq67_e1466_d_b17: f64 = (eq67_e1464_d_b17 * 0.5);
        let eq67_e1466_d_b18: f64 = (eq67_e1464_d_b18 * 0.5);
        let eq67_e1466_d_b19: f64 = (eq67_e1464_d_b19 * 0.5);
        let eq67_e1466_d_b20: f64 = (eq67_e1464_d_b20 * 0.5);
        let eq67_e1466_d_b21: f64 = (eq67_e1464_d_b21 * 0.5);
        let eq67_e1466_d_b22: f64 = (eq67_e1464_d_b22 * 0.5);
        let eq67_e1466_d_b23: f64 = (eq67_e1464_d_b23 * 0.5);
        let eq67_e1466_d_b24: f64 = (eq67_e1464_d_b24 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * s.v[860]);
        let eq67_e1468_d_n0: f64 = ((eq67_e1466_d_n0 * s.v[860]) + (eq67_e1466 * s.dn[860][0]));
        let eq67_e1468_d_n1: f64 = ((eq67_e1466_d_n1 * s.v[860]) + (eq67_e1466 * s.dn[860][1]));
        let eq67_e1468_d_n2: f64 = ((eq67_e1466_d_n2 * s.v[860]) + (eq67_e1466 * s.dn[860][2]));
        let eq67_e1468_d_n3: f64 = ((eq67_e1466_d_n3 * s.v[860]) + (eq67_e1466 * s.dn[860][3]));
        let eq67_e1468_d_n4: f64 = ((eq67_e1466_d_n4 * s.v[860]) + (eq67_e1466 * s.dn[860][4]));
        let eq67_e1468_d_n5: f64 = ((eq67_e1466_d_n5 * s.v[860]) + (eq67_e1466 * s.dn[860][5]));
        let eq67_e1468_d_n6: f64 = ((eq67_e1466_d_n6 * s.v[860]) + (eq67_e1466 * s.dn[860][6]));
        let eq67_e1468_d_n7: f64 = ((eq67_e1466_d_n7 * s.v[860]) + (eq67_e1466 * s.dn[860][7]));
        let eq67_e1468_d_n8: f64 = ((eq67_e1466_d_n8 * s.v[860]) + (eq67_e1466 * s.dn[860][8]));
        let eq67_e1468_d_n9: f64 = ((eq67_e1466_d_n9 * s.v[860]) + (eq67_e1466 * s.dn[860][9]));
        let eq67_e1468_d_n10: f64 = ((eq67_e1466_d_n10 * s.v[860]) + (eq67_e1466 * s.dn[860][10]));
        let eq67_e1468_d_n11: f64 = ((eq67_e1466_d_n11 * s.v[860]) + (eq67_e1466 * s.dn[860][11]));
        let eq67_e1468_d_n12: f64 = ((eq67_e1466_d_n12 * s.v[860]) + (eq67_e1466 * s.dn[860][12]));
        let eq67_e1468_d_n13: f64 = ((eq67_e1466_d_n13 * s.v[860]) + (eq67_e1466 * s.dn[860][13]));
        let eq67_e1468_d_n14: f64 = ((eq67_e1466_d_n14 * s.v[860]) + (eq67_e1466 * s.dn[860][14]));
        let eq67_e1468_d_n15: f64 = ((eq67_e1466_d_n15 * s.v[860]) + (eq67_e1466 * s.dn[860][15]));
        let eq67_e1468_d_n16: f64 = ((eq67_e1466_d_n16 * s.v[860]) + (eq67_e1466 * s.dn[860][16]));
        let eq67_e1468_d_n17: f64 = ((eq67_e1466_d_n17 * s.v[860]) + (eq67_e1466 * s.dn[860][17]));
        let eq67_e1468_d_n18: f64 = ((eq67_e1466_d_n18 * s.v[860]) + (eq67_e1466 * s.dn[860][18]));
        let eq67_e1468_d_n19: f64 = ((eq67_e1466_d_n19 * s.v[860]) + (eq67_e1466 * s.dn[860][19]));
        let eq67_e1468_d_n20: f64 = ((eq67_e1466_d_n20 * s.v[860]) + (eq67_e1466 * s.dn[860][20]));
        let eq67_e1468_d_b0: f64 = ((eq67_e1466_d_b0 * s.v[860]) + (eq67_e1466 * s.db[860][0]));
        let eq67_e1468_d_b1: f64 = ((eq67_e1466_d_b1 * s.v[860]) + (eq67_e1466 * s.db[860][1]));
        let eq67_e1468_d_b2: f64 = ((eq67_e1466_d_b2 * s.v[860]) + (eq67_e1466 * s.db[860][2]));
        let eq67_e1468_d_b3: f64 = ((eq67_e1466_d_b3 * s.v[860]) + (eq67_e1466 * s.db[860][3]));
        let eq67_e1468_d_b4: f64 = ((eq67_e1466_d_b4 * s.v[860]) + (eq67_e1466 * s.db[860][4]));
        let eq67_e1468_d_b5: f64 = ((eq67_e1466_d_b5 * s.v[860]) + (eq67_e1466 * s.db[860][5]));
        let eq67_e1468_d_b6: f64 = ((eq67_e1466_d_b6 * s.v[860]) + (eq67_e1466 * s.db[860][6]));
        let eq67_e1468_d_b7: f64 = ((eq67_e1466_d_b7 * s.v[860]) + (eq67_e1466 * s.db[860][7]));
        let eq67_e1468_d_b8: f64 = ((eq67_e1466_d_b8 * s.v[860]) + (eq67_e1466 * s.db[860][8]));
        let eq67_e1468_d_b9: f64 = ((eq67_e1466_d_b9 * s.v[860]) + (eq67_e1466 * s.db[860][9]));
        let eq67_e1468_d_b10: f64 = ((eq67_e1466_d_b10 * s.v[860]) + (eq67_e1466 * s.db[860][10]));
        let eq67_e1468_d_b11: f64 = ((eq67_e1466_d_b11 * s.v[860]) + (eq67_e1466 * s.db[860][11]));
        let eq67_e1468_d_b12: f64 = ((eq67_e1466_d_b12 * s.v[860]) + (eq67_e1466 * s.db[860][12]));
        let eq67_e1468_d_b13: f64 = ((eq67_e1466_d_b13 * s.v[860]) + (eq67_e1466 * s.db[860][13]));
        let eq67_e1468_d_b14: f64 = ((eq67_e1466_d_b14 * s.v[860]) + (eq67_e1466 * s.db[860][14]));
        let eq67_e1468_d_b15: f64 = ((eq67_e1466_d_b15 * s.v[860]) + (eq67_e1466 * s.db[860][15]));
        let eq67_e1468_d_b16: f64 = ((eq67_e1466_d_b16 * s.v[860]) + (eq67_e1466 * s.db[860][16]));
        let eq67_e1468_d_b17: f64 = ((eq67_e1466_d_b17 * s.v[860]) + (eq67_e1466 * s.db[860][17]));
        let eq67_e1468_d_b18: f64 = ((eq67_e1466_d_b18 * s.v[860]) + (eq67_e1466 * s.db[860][18]));
        let eq67_e1468_d_b19: f64 = ((eq67_e1466_d_b19 * s.v[860]) + (eq67_e1466 * s.db[860][19]));
        let eq67_e1468_d_b20: f64 = ((eq67_e1466_d_b20 * s.v[860]) + (eq67_e1466 * s.db[860][20]));
        let eq67_e1468_d_b21: f64 = ((eq67_e1466_d_b21 * s.v[860]) + (eq67_e1466 * s.db[860][21]));
        let eq67_e1468_d_b22: f64 = ((eq67_e1466_d_b22 * s.v[860]) + (eq67_e1466 * s.db[860][22]));
        let eq67_e1468_d_b23: f64 = ((eq67_e1466_d_b23 * s.v[860]) + (eq67_e1466 * s.db[860][23]));
        let eq67_e1468_d_b24: f64 = ((eq67_e1466_d_b24 * s.v[860]) + (eq67_e1466 * s.db[860][24]));
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n0: f64 = (eq67_e1468_d_n0 * (nv4 - 0.0));
        let eq67_e1470_d_n1: f64 = (eq67_e1468_d_n1 * (nv4 - 0.0));
        let eq67_e1470_d_n2: f64 = (eq67_e1468_d_n2 * (nv4 - 0.0));
        let eq67_e1470_d_n3: f64 = (eq67_e1468_d_n3 * (nv4 - 0.0));
        let eq67_e1470_d_n4: f64 = ((eq67_e1468_d_n4 * (nv4 - 0.0)) + eq67_e1468);
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n9: f64 = (eq67_e1468_d_n9 * (nv4 - 0.0));
        let eq67_e1470_d_n10: f64 = (eq67_e1468_d_n10 * (nv4 - 0.0));
        let eq67_e1470_d_n11: f64 = (eq67_e1468_d_n11 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1470_d_b0: f64 = (eq67_e1468_d_b0 * (nv4 - 0.0));
        let eq67_e1470_d_b1: f64 = (eq67_e1468_d_b1 * (nv4 - 0.0));
        let eq67_e1470_d_b2: f64 = (eq67_e1468_d_b2 * (nv4 - 0.0));
        let eq67_e1470_d_b3: f64 = (eq67_e1468_d_b3 * (nv4 - 0.0));
        let eq67_e1470_d_b4: f64 = (eq67_e1468_d_b4 * (nv4 - 0.0));
        let eq67_e1470_d_b5: f64 = (eq67_e1468_d_b5 * (nv4 - 0.0));
        let eq67_e1470_d_b6: f64 = (eq67_e1468_d_b6 * (nv4 - 0.0));
        let eq67_e1470_d_b7: f64 = (eq67_e1468_d_b7 * (nv4 - 0.0));
        let eq67_e1470_d_b8: f64 = (eq67_e1468_d_b8 * (nv4 - 0.0));
        let eq67_e1470_d_b9: f64 = (eq67_e1468_d_b9 * (nv4 - 0.0));
        let eq67_e1470_d_b10: f64 = (eq67_e1468_d_b10 * (nv4 - 0.0));
        let eq67_e1470_d_b11: f64 = (eq67_e1468_d_b11 * (nv4 - 0.0));
        let eq67_e1470_d_b12: f64 = (eq67_e1468_d_b12 * (nv4 - 0.0));
        let eq67_e1470_d_b13: f64 = (eq67_e1468_d_b13 * (nv4 - 0.0));
        let eq67_e1470_d_b14: f64 = (eq67_e1468_d_b14 * (nv4 - 0.0));
        let eq67_e1470_d_b15: f64 = (eq67_e1468_d_b15 * (nv4 - 0.0));
        let eq67_e1470_d_b16: f64 = (eq67_e1468_d_b16 * (nv4 - 0.0));
        let eq67_e1470_d_b17: f64 = (eq67_e1468_d_b17 * (nv4 - 0.0));
        let eq67_e1470_d_b18: f64 = (eq67_e1468_d_b18 * (nv4 - 0.0));
        let eq67_e1470_d_b19: f64 = (eq67_e1468_d_b19 * (nv4 - 0.0));
        let eq67_e1470_d_b20: f64 = (eq67_e1468_d_b20 * (nv4 - 0.0));
        let eq67_e1470_d_b21: f64 = (eq67_e1468_d_b21 * (nv4 - 0.0));
        let eq67_e1470_d_b22: f64 = (eq67_e1468_d_b22 * (nv4 - 0.0));
        let eq67_e1470_d_b23: f64 = (eq67_e1468_d_b23 * (nv4 - 0.0));
        let eq67_e1470_d_b24: f64 = (eq67_e1468_d_b24 * (nv4 - 0.0));
        let eq67_e1471_q: f64 = eq67_e1470;
        let eq67_e1472: f64 = (-eq67_e1470);
        let eq67_e1472_q: f64 = (-eq67_e1471_q);
        let eq67_reactive_node_derivatives: [f64; 21] = [(-eq67_e1470_d_n0), (-eq67_e1470_d_n1), (-eq67_e1470_d_n2), (-eq67_e1470_d_n3), (-eq67_e1470_d_n4), (-eq67_e1470_d_n5), (-eq67_e1470_d_n6), (-eq67_e1470_d_n7), (-eq67_e1470_d_n8), (-eq67_e1470_d_n9), (-eq67_e1470_d_n10), (-eq67_e1470_d_n11), (-eq67_e1470_d_n12), (-eq67_e1470_d_n13), (-eq67_e1470_d_n14), (-eq67_e1470_d_n15), (-eq67_e1470_d_n16), (-eq67_e1470_d_n17), (-eq67_e1470_d_n18), (-eq67_e1470_d_n19), (-eq67_e1470_d_n20)];
        let eq67_reactive_branch_derivatives: [f64; 25] = [(-eq67_e1470_d_b0), (-eq67_e1470_d_b1), (-eq67_e1470_d_b2), (-eq67_e1470_d_b3), (-eq67_e1470_d_b4), (-eq67_e1470_d_b5), (-eq67_e1470_d_b6), (-eq67_e1470_d_b7), (-eq67_e1470_d_b8), (-eq67_e1470_d_b9), (-eq67_e1470_d_b10), (-eq67_e1470_d_b11), (-eq67_e1470_d_b12), (-eq67_e1470_d_b13), (-eq67_e1470_d_b14), (-eq67_e1470_d_b15), (-eq67_e1470_d_b16), (-eq67_e1470_d_b17), (-eq67_e1470_d_b18), (-eq67_e1470_d_b19), (-eq67_e1470_d_b20), (-eq67_e1470_d_b21), (-eq67_e1470_d_b22), (-eq67_e1470_d_b23), (-eq67_e1470_d_b24)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (s.v[19] * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let __rspice_inv_cse_1: f64 = 1.0 / (2.0 * eq68_e1476);
        let eq68_e1476_d_n0: f64 = ((s.dn[19][0] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n1: f64 = ((s.dn[19][1] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n2: f64 = ((s.dn[19][2] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n3: f64 = ((s.dn[19][3] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n4: f64 = ((s.dn[19][4] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n5: f64 = ((s.dn[19][5] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n6: f64 = ((s.dn[19][6] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n7: f64 = ((s.dn[19][7] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n8: f64 = ((s.dn[19][8] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n9: f64 = ((s.dn[19][9] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n10: f64 = ((s.dn[19][10] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n11: f64 = ((s.dn[19][11] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n12: f64 = ((s.dn[19][12] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n13: f64 = ((s.dn[19][13] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n14: f64 = ((s.dn[19][14] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n15: f64 = ((s.dn[19][15] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n16: f64 = ((s.dn[19][16] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n17: f64 = ((s.dn[19][17] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n18: f64 = ((s.dn[19][18] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n19: f64 = ((s.dn[19][19] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n20: f64 = ((s.dn[19][20] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b0: f64 = ((s.db[19][0] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b1: f64 = ((s.db[19][1] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b2: f64 = ((s.db[19][2] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b3: f64 = ((s.db[19][3] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b4: f64 = ((s.db[19][4] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b5: f64 = ((s.db[19][5] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b6: f64 = ((s.db[19][6] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b7: f64 = ((s.db[19][7] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b8: f64 = ((s.db[19][8] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b9: f64 = ((s.db[19][9] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b10: f64 = ((s.db[19][10] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b11: f64 = ((s.db[19][11] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b12: f64 = ((s.db[19][12] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b13: f64 = ((s.db[19][13] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b14: f64 = ((s.db[19][14] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b15: f64 = ((s.db[19][15] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b16: f64 = ((s.db[19][16] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b17: f64 = ((s.db[19][17] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b18: f64 = ((s.db[19][18] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b19: f64 = ((s.db[19][19] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b20: f64 = ((s.db[19][20] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b21: f64 = ((s.db[19][21] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b22: f64 = ((s.db[19][22] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b23: f64 = ((s.db[19][23] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b24: f64 = ((s.db[19][24] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1478_d_n0: f64 = (eq68_e1476_d_n0 * 0.5);
        let eq68_e1478_d_n1: f64 = (eq68_e1476_d_n1 * 0.5);
        let eq68_e1478_d_n2: f64 = (eq68_e1476_d_n2 * 0.5);
        let eq68_e1478_d_n3: f64 = (eq68_e1476_d_n3 * 0.5);
        let eq68_e1478_d_n4: f64 = (eq68_e1476_d_n4 * 0.5);
        let eq68_e1478_d_n5: f64 = (eq68_e1476_d_n5 * 0.5);
        let eq68_e1478_d_n6: f64 = (eq68_e1476_d_n6 * 0.5);
        let eq68_e1478_d_n7: f64 = (eq68_e1476_d_n7 * 0.5);
        let eq68_e1478_d_n8: f64 = (eq68_e1476_d_n8 * 0.5);
        let eq68_e1478_d_n9: f64 = (eq68_e1476_d_n9 * 0.5);
        let eq68_e1478_d_n10: f64 = (eq68_e1476_d_n10 * 0.5);
        let eq68_e1478_d_n11: f64 = (eq68_e1476_d_n11 * 0.5);
        let eq68_e1478_d_n12: f64 = (eq68_e1476_d_n12 * 0.5);
        let eq68_e1478_d_n13: f64 = (eq68_e1476_d_n13 * 0.5);
        let eq68_e1478_d_n14: f64 = (eq68_e1476_d_n14 * 0.5);
        let eq68_e1478_d_n15: f64 = (eq68_e1476_d_n15 * 0.5);
        let eq68_e1478_d_n16: f64 = (eq68_e1476_d_n16 * 0.5);
        let eq68_e1478_d_n17: f64 = (eq68_e1476_d_n17 * 0.5);
        let eq68_e1478_d_n18: f64 = (eq68_e1476_d_n18 * 0.5);
        let eq68_e1478_d_n19: f64 = (eq68_e1476_d_n19 * 0.5);
        let eq68_e1478_d_n20: f64 = (eq68_e1476_d_n20 * 0.5);
        let eq68_e1478_d_b0: f64 = (eq68_e1476_d_b0 * 0.5);
        let eq68_e1478_d_b1: f64 = (eq68_e1476_d_b1 * 0.5);
        let eq68_e1478_d_b2: f64 = (eq68_e1476_d_b2 * 0.5);
        let eq68_e1478_d_b3: f64 = (eq68_e1476_d_b3 * 0.5);
        let eq68_e1478_d_b4: f64 = (eq68_e1476_d_b4 * 0.5);
        let eq68_e1478_d_b5: f64 = (eq68_e1476_d_b5 * 0.5);
        let eq68_e1478_d_b6: f64 = (eq68_e1476_d_b6 * 0.5);
        let eq68_e1478_d_b7: f64 = (eq68_e1476_d_b7 * 0.5);
        let eq68_e1478_d_b8: f64 = (eq68_e1476_d_b8 * 0.5);
        let eq68_e1478_d_b9: f64 = (eq68_e1476_d_b9 * 0.5);
        let eq68_e1478_d_b10: f64 = (eq68_e1476_d_b10 * 0.5);
        let eq68_e1478_d_b11: f64 = (eq68_e1476_d_b11 * 0.5);
        let eq68_e1478_d_b12: f64 = (eq68_e1476_d_b12 * 0.5);
        let eq68_e1478_d_b13: f64 = (eq68_e1476_d_b13 * 0.5);
        let eq68_e1478_d_b14: f64 = (eq68_e1476_d_b14 * 0.5);
        let eq68_e1478_d_b15: f64 = (eq68_e1476_d_b15 * 0.5);
        let eq68_e1478_d_b16: f64 = (eq68_e1476_d_b16 * 0.5);
        let eq68_e1478_d_b17: f64 = (eq68_e1476_d_b17 * 0.5);
        let eq68_e1478_d_b18: f64 = (eq68_e1476_d_b18 * 0.5);
        let eq68_e1478_d_b19: f64 = (eq68_e1476_d_b19 * 0.5);
        let eq68_e1478_d_b20: f64 = (eq68_e1476_d_b20 * 0.5);
        let eq68_e1478_d_b21: f64 = (eq68_e1476_d_b21 * 0.5);
        let eq68_e1478_d_b22: f64 = (eq68_e1476_d_b22 * 0.5);
        let eq68_e1478_d_b23: f64 = (eq68_e1476_d_b23 * 0.5);
        let eq68_e1478_d_b24: f64 = (eq68_e1476_d_b24 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * s.v[860]);
        let eq68_e1480_d_n0: f64 = ((eq68_e1478_d_n0 * s.v[860]) + (eq68_e1478 * s.dn[860][0]));
        let eq68_e1480_d_n1: f64 = ((eq68_e1478_d_n1 * s.v[860]) + (eq68_e1478 * s.dn[860][1]));
        let eq68_e1480_d_n2: f64 = ((eq68_e1478_d_n2 * s.v[860]) + (eq68_e1478 * s.dn[860][2]));
        let eq68_e1480_d_n3: f64 = ((eq68_e1478_d_n3 * s.v[860]) + (eq68_e1478 * s.dn[860][3]));
        let eq68_e1480_d_n4: f64 = ((eq68_e1478_d_n4 * s.v[860]) + (eq68_e1478 * s.dn[860][4]));
        let eq68_e1480_d_n5: f64 = ((eq68_e1478_d_n5 * s.v[860]) + (eq68_e1478 * s.dn[860][5]));
        let eq68_e1480_d_n6: f64 = ((eq68_e1478_d_n6 * s.v[860]) + (eq68_e1478 * s.dn[860][6]));
        let eq68_e1480_d_n7: f64 = ((eq68_e1478_d_n7 * s.v[860]) + (eq68_e1478 * s.dn[860][7]));
        let eq68_e1480_d_n8: f64 = ((eq68_e1478_d_n8 * s.v[860]) + (eq68_e1478 * s.dn[860][8]));
        let eq68_e1480_d_n9: f64 = ((eq68_e1478_d_n9 * s.v[860]) + (eq68_e1478 * s.dn[860][9]));
        let eq68_e1480_d_n10: f64 = ((eq68_e1478_d_n10 * s.v[860]) + (eq68_e1478 * s.dn[860][10]));
        let eq68_e1480_d_n11: f64 = ((eq68_e1478_d_n11 * s.v[860]) + (eq68_e1478 * s.dn[860][11]));
        let eq68_e1480_d_n12: f64 = ((eq68_e1478_d_n12 * s.v[860]) + (eq68_e1478 * s.dn[860][12]));
        let eq68_e1480_d_n13: f64 = ((eq68_e1478_d_n13 * s.v[860]) + (eq68_e1478 * s.dn[860][13]));
        let eq68_e1480_d_n14: f64 = ((eq68_e1478_d_n14 * s.v[860]) + (eq68_e1478 * s.dn[860][14]));
        let eq68_e1480_d_n15: f64 = ((eq68_e1478_d_n15 * s.v[860]) + (eq68_e1478 * s.dn[860][15]));
        let eq68_e1480_d_n16: f64 = ((eq68_e1478_d_n16 * s.v[860]) + (eq68_e1478 * s.dn[860][16]));
        let eq68_e1480_d_n17: f64 = ((eq68_e1478_d_n17 * s.v[860]) + (eq68_e1478 * s.dn[860][17]));
        let eq68_e1480_d_n18: f64 = ((eq68_e1478_d_n18 * s.v[860]) + (eq68_e1478 * s.dn[860][18]));
        let eq68_e1480_d_n19: f64 = ((eq68_e1478_d_n19 * s.v[860]) + (eq68_e1478 * s.dn[860][19]));
        let eq68_e1480_d_n20: f64 = ((eq68_e1478_d_n20 * s.v[860]) + (eq68_e1478 * s.dn[860][20]));
        let eq68_e1480_d_b0: f64 = ((eq68_e1478_d_b0 * s.v[860]) + (eq68_e1478 * s.db[860][0]));
        let eq68_e1480_d_b1: f64 = ((eq68_e1478_d_b1 * s.v[860]) + (eq68_e1478 * s.db[860][1]));
        let eq68_e1480_d_b2: f64 = ((eq68_e1478_d_b2 * s.v[860]) + (eq68_e1478 * s.db[860][2]));
        let eq68_e1480_d_b3: f64 = ((eq68_e1478_d_b3 * s.v[860]) + (eq68_e1478 * s.db[860][3]));
        let eq68_e1480_d_b4: f64 = ((eq68_e1478_d_b4 * s.v[860]) + (eq68_e1478 * s.db[860][4]));
        let eq68_e1480_d_b5: f64 = ((eq68_e1478_d_b5 * s.v[860]) + (eq68_e1478 * s.db[860][5]));
        let eq68_e1480_d_b6: f64 = ((eq68_e1478_d_b6 * s.v[860]) + (eq68_e1478 * s.db[860][6]));
        let eq68_e1480_d_b7: f64 = ((eq68_e1478_d_b7 * s.v[860]) + (eq68_e1478 * s.db[860][7]));
        let eq68_e1480_d_b8: f64 = ((eq68_e1478_d_b8 * s.v[860]) + (eq68_e1478 * s.db[860][8]));
        let eq68_e1480_d_b9: f64 = ((eq68_e1478_d_b9 * s.v[860]) + (eq68_e1478 * s.db[860][9]));
        let eq68_e1480_d_b10: f64 = ((eq68_e1478_d_b10 * s.v[860]) + (eq68_e1478 * s.db[860][10]));
        let eq68_e1480_d_b11: f64 = ((eq68_e1478_d_b11 * s.v[860]) + (eq68_e1478 * s.db[860][11]));
        let eq68_e1480_d_b12: f64 = ((eq68_e1478_d_b12 * s.v[860]) + (eq68_e1478 * s.db[860][12]));
        let eq68_e1480_d_b13: f64 = ((eq68_e1478_d_b13 * s.v[860]) + (eq68_e1478 * s.db[860][13]));
        let eq68_e1480_d_b14: f64 = ((eq68_e1478_d_b14 * s.v[860]) + (eq68_e1478 * s.db[860][14]));
        let eq68_e1480_d_b15: f64 = ((eq68_e1478_d_b15 * s.v[860]) + (eq68_e1478 * s.db[860][15]));
        let eq68_e1480_d_b16: f64 = ((eq68_e1478_d_b16 * s.v[860]) + (eq68_e1478 * s.db[860][16]));
        let eq68_e1480_d_b17: f64 = ((eq68_e1478_d_b17 * s.v[860]) + (eq68_e1478 * s.db[860][17]));
        let eq68_e1480_d_b18: f64 = ((eq68_e1478_d_b18 * s.v[860]) + (eq68_e1478 * s.db[860][18]));
        let eq68_e1480_d_b19: f64 = ((eq68_e1478_d_b19 * s.v[860]) + (eq68_e1478 * s.db[860][19]));
        let eq68_e1480_d_b20: f64 = ((eq68_e1478_d_b20 * s.v[860]) + (eq68_e1478 * s.db[860][20]));
        let eq68_e1480_d_b21: f64 = ((eq68_e1478_d_b21 * s.v[860]) + (eq68_e1478 * s.db[860][21]));
        let eq68_e1480_d_b22: f64 = ((eq68_e1478_d_b22 * s.v[860]) + (eq68_e1478 * s.db[860][22]));
        let eq68_e1480_d_b23: f64 = ((eq68_e1478_d_b23 * s.v[860]) + (eq68_e1478 * s.db[860][23]));
        let eq68_e1480_d_b24: f64 = ((eq68_e1478_d_b24 * s.v[860]) + (eq68_e1478 * s.db[860][24]));
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n0: f64 = (eq68_e1480_d_n0 * (nv4 - 0.0));
        let eq68_e1482_d_n1: f64 = (eq68_e1480_d_n1 * (nv4 - 0.0));
        let eq68_e1482_d_n2: f64 = (eq68_e1480_d_n2 * (nv4 - 0.0));
        let eq68_e1482_d_n3: f64 = (eq68_e1480_d_n3 * (nv4 - 0.0));
        let eq68_e1482_d_n4: f64 = ((eq68_e1480_d_n4 * (nv4 - 0.0)) + eq68_e1480);
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n9: f64 = (eq68_e1480_d_n9 * (nv4 - 0.0));
        let eq68_e1482_d_n10: f64 = (eq68_e1480_d_n10 * (nv4 - 0.0));
        let eq68_e1482_d_n11: f64 = (eq68_e1480_d_n11 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1482_d_b0: f64 = (eq68_e1480_d_b0 * (nv4 - 0.0));
        let eq68_e1482_d_b1: f64 = (eq68_e1480_d_b1 * (nv4 - 0.0));
        let eq68_e1482_d_b2: f64 = (eq68_e1480_d_b2 * (nv4 - 0.0));
        let eq68_e1482_d_b3: f64 = (eq68_e1480_d_b3 * (nv4 - 0.0));
        let eq68_e1482_d_b4: f64 = (eq68_e1480_d_b4 * (nv4 - 0.0));
        let eq68_e1482_d_b5: f64 = (eq68_e1480_d_b5 * (nv4 - 0.0));
        let eq68_e1482_d_b6: f64 = (eq68_e1480_d_b6 * (nv4 - 0.0));
        let eq68_e1482_d_b7: f64 = (eq68_e1480_d_b7 * (nv4 - 0.0));
        let eq68_e1482_d_b8: f64 = (eq68_e1480_d_b8 * (nv4 - 0.0));
        let eq68_e1482_d_b9: f64 = (eq68_e1480_d_b9 * (nv4 - 0.0));
        let eq68_e1482_d_b10: f64 = (eq68_e1480_d_b10 * (nv4 - 0.0));
        let eq68_e1482_d_b11: f64 = (eq68_e1480_d_b11 * (nv4 - 0.0));
        let eq68_e1482_d_b12: f64 = (eq68_e1480_d_b12 * (nv4 - 0.0));
        let eq68_e1482_d_b13: f64 = (eq68_e1480_d_b13 * (nv4 - 0.0));
        let eq68_e1482_d_b14: f64 = (eq68_e1480_d_b14 * (nv4 - 0.0));
        let eq68_e1482_d_b15: f64 = (eq68_e1480_d_b15 * (nv4 - 0.0));
        let eq68_e1482_d_b16: f64 = (eq68_e1480_d_b16 * (nv4 - 0.0));
        let eq68_e1482_d_b17: f64 = (eq68_e1480_d_b17 * (nv4 - 0.0));
        let eq68_e1482_d_b18: f64 = (eq68_e1480_d_b18 * (nv4 - 0.0));
        let eq68_e1482_d_b19: f64 = (eq68_e1480_d_b19 * (nv4 - 0.0));
        let eq68_e1482_d_b20: f64 = (eq68_e1480_d_b20 * (nv4 - 0.0));
        let eq68_e1482_d_b21: f64 = (eq68_e1480_d_b21 * (nv4 - 0.0));
        let eq68_e1482_d_b22: f64 = (eq68_e1480_d_b22 * (nv4 - 0.0));
        let eq68_e1482_d_b23: f64 = (eq68_e1480_d_b23 * (nv4 - 0.0));
        let eq68_e1482_d_b24: f64 = (eq68_e1480_d_b24 * (nv4 - 0.0));
        let eq68_e1483_q: f64 = eq68_e1482;
        let eq68_e1484: f64 = (-eq68_e1482);
        let eq68_e1484_q: f64 = (-eq68_e1483_q);
        let eq68_reactive_node_derivatives: [f64; 21] = [(-eq68_e1482_d_n0), (-eq68_e1482_d_n1), (-eq68_e1482_d_n2), (-eq68_e1482_d_n3), (-eq68_e1482_d_n4), (-eq68_e1482_d_n5), (-eq68_e1482_d_n6), (-eq68_e1482_d_n7), (-eq68_e1482_d_n8), (-eq68_e1482_d_n9), (-eq68_e1482_d_n10), (-eq68_e1482_d_n11), (-eq68_e1482_d_n12), (-eq68_e1482_d_n13), (-eq68_e1482_d_n14), (-eq68_e1482_d_n15), (-eq68_e1482_d_n16), (-eq68_e1482_d_n17), (-eq68_e1482_d_n18), (-eq68_e1482_d_n19), (-eq68_e1482_d_n20)];
        let eq68_reactive_branch_derivatives: [f64; 25] = [(-eq68_e1482_d_b0), (-eq68_e1482_d_b1), (-eq68_e1482_d_b2), (-eq68_e1482_d_b3), (-eq68_e1482_d_b4), (-eq68_e1482_d_b5), (-eq68_e1482_d_b6), (-eq68_e1482_d_b7), (-eq68_e1482_d_b8), (-eq68_e1482_d_b9), (-eq68_e1482_d_b10), (-eq68_e1482_d_b11), (-eq68_e1482_d_b12), (-eq68_e1482_d_b13), (-eq68_e1482_d_b14), (-eq68_e1482_d_b15), (-eq68_e1482_d_b16), (-eq68_e1482_d_b17), (-eq68_e1482_d_b18), (-eq68_e1482_d_b19), (-eq68_e1482_d_b20), (-eq68_e1482_d_b21), (-eq68_e1482_d_b22), (-eq68_e1482_d_b23), (-eq68_e1482_d_b24)];
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
