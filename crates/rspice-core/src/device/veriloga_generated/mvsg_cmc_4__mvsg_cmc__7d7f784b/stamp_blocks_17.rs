#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv6 = ctx.node_voltage(nodes[6]);
        if ((l.f2033 == 0.0) && (l.f2038 != 0.0)) {let t0: f64 = (p.p0 * p.p2);let t1: f64 = (l.f5a * (nv6 - nv0));let t2: f64 = (l.f59 * p.p28);let t3: f64 = ((nv6 - nv0) - p.p27);let t4: f64 = (t3 / p.p28);let t5: f64 = (t4).exp();let t6: f64 = (t2 * t5);let t7: f64 = (t1 + t6);let t8: f64 = (t0 * t7);(l.f2262, l.f2263, l.f2264, l.f2265, ) = (t8, (t0 * ((-l.f5a) + (t2 * (t5 * (-1.0 / p.p28))))), (t0 * ((l.f5b * (nv6 - nv0)) + ((l.f5d * p.p28) * t5))), (t0 * (l.f5a + (t2 * (t5 * (1.0 / p.p28))))), );l.f2266 = 0.0;}
        if ((l.f2033 == 0.0) && (l.f2038 == 0.0)) {let t9: f64 = (p.p0 * p.p2);let ta: f64 = (l.f5a * (nv6 - nv0));let tb: f64 = (l.f59 * p.p28);let tc: f64 = ((nv6 - nv0) - p.p27);let td: f64 = (tc / p.p28);let te: f64 = (td).exp();let tf: f64 = (1.0 + te);let t10: f64 = (tf).ln();let t11: f64 = (tb * t10);let t12: f64 = (ta + t11);let t13: f64 = (t9 * t12);(l.f2262, l.f2263, l.f2264, l.f2265, ) = (t13, (t9 * ((-l.f5a) + (tb * ((te * (-1.0 / p.p28)) / tf)))), (t9 * ((l.f5b * (nv6 - nv0)) + ((l.f5d * p.p28) * t10))), (t9 * (l.f5a + (tb * ((te * (1.0 / p.p28)) / tf)))), );l.f2266 = 0.0;}
        let t14: f64 = ((nv2 - nv0) - p.p27);let t15: f64 = (t14 / p.p28);let t16: f64 = if t15 > 50.0 { 1.0 } else { 0.0 };l.f203a = t16;l.f203b = 0.0;
        if (l.f203a != 0.0) {let t17: f64 = (p.p0 * p.p2);let t18: f64 = (l.f60 * (nv2 - nv0));let t19: f64 = ((nv2 - nv0) - p.p27);let t1a: f64 = (l.f5f * t19);let t1b: f64 = (t18 + t1a);let t1c: f64 = (t17 * t1b);(l.f2267, l.f2268, l.f2269, l.f226a, ) = (t1c, (t17 * ((-l.f60) + (-l.f5f))), (t17 * (l.f60 + l.f5f)), (t17 * ((l.f61 * (nv2 - nv0)) + (l.f63 * t19))), );l.f226b = 0.0;}
        let t1d: f64 = ((nv2 - nv0) - p.p27);let t1e: f64 = (t1d / p.p28);let t1f: f64 = (-50.0);let t20: f64 = if t1e < t1f { 1.0 } else { 0.0 };l.f203c = t20;l.f203d = 0.0;
        if ((l.f203a == 0.0) && (l.f203c != 0.0)) {let t21: f64 = (p.p0 * p.p2);let t22: f64 = (l.f60 * (nv2 - nv0));let t23: f64 = (l.f5f * p.p28);let t24: f64 = ((nv2 - nv0) - p.p27);let t25: f64 = (t24 / p.p28);let t26: f64 = (t25).exp();let t27: f64 = (t23 * t26);let t28: f64 = (t22 + t27);let t29: f64 = (t21 * t28);(l.f2267, l.f2268, l.f2269, l.f226a, ) = (t29, (t21 * ((-l.f60) + (t23 * (t26 * (-1.0 / p.p28))))), (t21 * (l.f60 + (t23 * (t26 * (1.0 / p.p28))))), (t21 * ((l.f61 * (nv2 - nv0)) + ((l.f63 * p.p28) * t26))), );l.f226b = 0.0;}
        if ((l.f203a == 0.0) && (l.f203c == 0.0)) {let t2a: f64 = (p.p0 * p.p2);let t2b: f64 = (l.f60 * (nv2 - nv0));let t2c: f64 = (l.f5f * p.p28);let t2d: f64 = ((nv2 - nv0) - p.p27);let t2e: f64 = (t2d / p.p28);let t2f: f64 = (t2e).exp();let t30: f64 = (1.0 + t2f);let t31: f64 = (t30).ln();let t32: f64 = (t2c * t31);let t33: f64 = (t2b + t32);let t34: f64 = (t2a * t33);(l.f2267, l.f2268, l.f2269, l.f226a, ) = (t34, (t2a * ((-l.f60) + (t2c * ((t2f * (-1.0 / p.p28)) / t30)))), (t2a * (l.f60 + (t2c * ((t2f * (1.0 / p.p28)) / t30)))), (t2a * ((l.f61 * (nv2 - nv0)) + ((l.f63 * p.p28) * t31))), );l.f226b = 0.0;}
        let t35: f64 = ((nv3 - nv2) - p.p27);let t36: f64 = (t35 / p.p28);let t37: f64 = if t36 > 50.0 { 1.0 } else { 0.0 };l.f203e = t37;l.f203f = 0.0;
        if (l.f203e != 0.0) {let t38: f64 = (p.p0 * p.p2);let t39: f64 = (l.f78 * (nv3 - nv2));let t3a: f64 = ((nv3 - nv2) - p.p27);let t3b: f64 = (l.f77 * t3a);let t3c: f64 = (t39 + t3b);let t3d: f64 = (t38 * t3c);(l.f227b, l.f227c, l.f227d, l.f227e, ) = (t3d, (t38 * ((-l.f78) + (-l.f77))), (t38 * (l.f78 + l.f77)), (t38 * ((l.f79 * (nv3 - nv2)) + (l.f7b * t3a))), );l.f227f = 0.0;}
        let t3e: f64 = ((nv3 - nv2) - p.p27);let t3f: f64 = (t3e / p.p28);let t40: f64 = (-50.0);let t41: f64 = if t3f < t40 { 1.0 } else { 0.0 };l.f2040 = t41;l.f2041 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_118(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv6 = ctx.node_voltage(nodes[6]);
        if ((l.f203e == 0.0) && (l.f2040 != 0.0)) {let t42: f64 = (p.p0 * p.p2);let t43: f64 = (l.f78 * (nv3 - nv2));let t44: f64 = (l.f77 * p.p28);let t45: f64 = ((nv3 - nv2) - p.p27);let t46: f64 = (t45 / p.p28);let t47: f64 = (t46).exp();let t48: f64 = (t44 * t47);let t49: f64 = (t43 + t48);let t4a: f64 = (t42 * t49);(l.f227b, l.f227c, l.f227d, l.f227e, ) = (t4a, (t42 * ((-l.f78) + (t44 * (t47 * (-1.0 / p.p28))))), (t42 * (l.f78 + (t44 * (t47 * (1.0 / p.p28))))), (t42 * ((l.f79 * (nv3 - nv2)) + ((l.f7b * p.p28) * t47))), );l.f227f = 0.0;}
        if ((l.f203e == 0.0) && (l.f2040 == 0.0)) {let t4b: f64 = (p.p0 * p.p2);let t4c: f64 = (l.f78 * (nv3 - nv2));let t4d: f64 = (l.f77 * p.p28);let t4e: f64 = ((nv3 - nv2) - p.p27);let t4f: f64 = (t4e / p.p28);let t50: f64 = (t4f).exp();let t51: f64 = (1.0 + t50);let t52: f64 = (t51).ln();let t53: f64 = (t4d * t52);let t54: f64 = (t4c + t53);let t55: f64 = (t4b * t54);(l.f227b, l.f227c, l.f227d, l.f227e, ) = (t55, (t4b * ((-l.f78) + (t4d * ((t50 * (-1.0 / p.p28)) / t51)))), (t4b * (l.f78 + (t4d * ((t50 * (1.0 / p.p28)) / t51)))), (t4b * ((l.f79 * (nv3 - nv2)) + ((l.f7b * p.p28) * t52))), );l.f227f = 0.0;}
        let t56: f64 = ((nv3 - nv0) - p.p27);let t57: f64 = (t56 / p.p28);let t58: f64 = if t57 > 50.0 { 1.0 } else { 0.0 };l.f2042 = t58;l.f2043 = 0.0;
        if (l.f2042 != 0.0) {let t59: f64 = (p.p0 * p.p2);let t5a: f64 = (l.f66 * (nv3 - nv0));let t5b: f64 = ((nv3 - nv0) - p.p27);let t5c: f64 = (l.f65 * t5b);let t5d: f64 = (t5a + t5c);let t5e: f64 = (t59 * t5d);(l.f226c, l.f226d, l.f226e, l.f226f, ) = (t5e, (t59 * ((-l.f66) + (-l.f65))), (t59 * (l.f66 + l.f65)), (t59 * ((l.f67 * (nv3 - nv0)) + (l.f69 * t5b))), );l.f2270 = 0.0;}
        let t5f: f64 = ((nv3 - nv0) - p.p27);let t60: f64 = (t5f / p.p28);let t61: f64 = (-50.0);let t62: f64 = if t60 < t61 { 1.0 } else { 0.0 };l.f2044 = t62;l.f2045 = 0.0;
        if ((l.f2042 == 0.0) && (l.f2044 != 0.0)) {let t63: f64 = (p.p0 * p.p2);let t64: f64 = (l.f66 * (nv3 - nv0));let t65: f64 = (l.f65 * p.p28);let t66: f64 = ((nv3 - nv0) - p.p27);let t67: f64 = (t66 / p.p28);let t68: f64 = (t67).exp();let t69: f64 = (t65 * t68);let t6a: f64 = (t64 + t69);let t6b: f64 = (t63 * t6a);(l.f226c, l.f226d, l.f226e, l.f226f, ) = (t6b, (t63 * ((-l.f66) + (t65 * (t68 * (-1.0 / p.p28))))), (t63 * (l.f66 + (t65 * (t68 * (1.0 / p.p28))))), (t63 * ((l.f67 * (nv3 - nv0)) + ((l.f69 * p.p28) * t68))), );l.f2270 = 0.0;}
        if ((l.f2042 == 0.0) && (l.f2044 == 0.0)) {let t6c: f64 = (p.p0 * p.p2);let t6d: f64 = (l.f66 * (nv3 - nv0));let t6e: f64 = (l.f65 * p.p28);let t6f: f64 = ((nv3 - nv0) - p.p27);let t70: f64 = (t6f / p.p28);let t71: f64 = (t70).exp();let t72: f64 = (1.0 + t71);let t73: f64 = (t72).ln();let t74: f64 = (t6e * t73);let t75: f64 = (t6d + t74);let t76: f64 = (t6c * t75);(l.f226c, l.f226d, l.f226e, l.f226f, ) = (t76, (t6c * ((-l.f66) + (t6e * ((t71 * (-1.0 / p.p28)) / t72)))), (t6c * (l.f66 + (t6e * ((t71 * (1.0 / p.p28)) / t72)))), (t6c * ((l.f67 * (nv3 - nv0)) + ((l.f69 * p.p28) * t73))), );l.f2270 = 0.0;}
        let t77: f64 = ((nv6 - nv3) - p.p27);let t78: f64 = (t77 / p.p28);let t79: f64 = if t78 > 50.0 { 1.0 } else { 0.0 };l.f2046 = t79;l.f2047 = 0.0;
        if (l.f2046 != 0.0) {let t7a: f64 = (p.p0 * p.p2);let t7b: f64 = (l.f6c * (nv6 - nv3));let t7c: f64 = ((nv6 - nv3) - p.p27);let t7d: f64 = (l.f6b * t7c);let t7e: f64 = (t7b + t7d);let t7f: f64 = (t7a * t7e);(l.f2271, l.f2272, l.f2273, l.f2274, ) = (t7f, (t7a * ((-l.f6c) + (-l.f6b))), (t7a * ((l.f6d * (nv6 - nv3)) + (l.f6f * t7c))), (t7a * (l.f6c + l.f6b)), );l.f2275 = 0.0;}
        let t80: f64 = ((nv6 - nv3) - p.p27);let t81: f64 = (t80 / p.p28);let t82: f64 = (-50.0);let t83: f64 = if t81 < t82 { 1.0 } else { 0.0 };l.f2048 = t83;l.f2049 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_119(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv6 = ctx.node_voltage(nodes[6]);
        if ((l.f2046 == 0.0) && (l.f2048 != 0.0)) {let t84: f64 = (p.p0 * p.p2);let t85: f64 = (l.f6c * (nv6 - nv3));let t86: f64 = (l.f6b * p.p28);let t87: f64 = ((nv6 - nv3) - p.p27);let t88: f64 = (t87 / p.p28);let t89: f64 = (t88).exp();let t8a: f64 = (t86 * t89);let t8b: f64 = (t85 + t8a);let t8c: f64 = (t84 * t8b);(l.f2271, l.f2272, l.f2273, l.f2274, ) = (t8c, (t84 * ((-l.f6c) + (t86 * (t89 * (-1.0 / p.p28))))), (t84 * ((l.f6d * (nv6 - nv3)) + ((l.f6f * p.p28) * t89))), (t84 * (l.f6c + (t86 * (t89 * (1.0 / p.p28))))), );l.f2275 = 0.0;}
        if ((l.f2046 == 0.0) && (l.f2048 == 0.0)) {let t8d: f64 = (p.p0 * p.p2);let t8e: f64 = (l.f6c * (nv6 - nv3));let t8f: f64 = (l.f6b * p.p28);let t90: f64 = ((nv6 - nv3) - p.p27);let t91: f64 = (t90 / p.p28);let t92: f64 = (t91).exp();let t93: f64 = (1.0 + t92);let t94: f64 = (t93).ln();let t95: f64 = (t8f * t94);let t96: f64 = (t8e + t95);let t97: f64 = (t8d * t96);(l.f2271, l.f2272, l.f2273, l.f2274, ) = (t97, (t8d * ((-l.f6c) + (t8f * ((t92 * (-1.0 / p.p28)) / t93)))), (t8d * ((l.f6d * (nv6 - nv3)) + ((l.f6f * p.p28) * t94))), (t8d * (l.f6c + (t8f * ((t92 * (1.0 / p.p28)) / t93)))), );l.f2275 = 0.0;}
        let t98: f64 = if p.p320 > 0.0 { 1.0 } else { 0.0 };l.f205b = t98;l.f205c = 0.0;
    }
    #[inline(never)]
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
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv20 = ctx.node_voltage(nodes[20]);let nv21 = ctx.node_voltage(nodes[21]);
        let (eq0_e383,) = {
    if (l.f1db2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e383;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e387,) = {
    if (l.f1db2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e387;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let (eq2_e391,) = {
    if (l.f1db2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e391;
        stamper.stamp_potential_const_local(
            2,
            eq2_value,
        );
        let (eq3_e395,) = {
    if (l.f1db2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e395;
        stamper.stamp_potential_const_local(
            3,
            eq3_value,
        );
        let (eq4_e399,) = {
    if (l.f1db2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e399;
        stamper.stamp_potential_const_local(
            4,
            eq4_value,
        );
        let (eq5_e403,) = {
    if (l.f1db2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e403;
        stamper.stamp_potential_const_local(
            5,
            eq5_value,
        );
        let (eq6_e408, eq6_e408_d_n0, eq6_e408_d_n1, eq6_e408_d_n21,) = {
    if (l.f1db2 != 0.0) {
        let eq6_e406: f64 = (-l.f23e0);
        (eq6_e406, (-l.f23e1), (-l.f23e2), (-l.f23e3),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e408;
        stamper.stamp_current_node3_local(
            Some(21),
            None,
            multiplicity * (eq6_value),
            0,
            multiplicity * (eq6_e408_d_n0),
            1,
            multiplicity * (eq6_e408_d_n1),
            21,
            multiplicity * (eq6_e408_d_n21),
        );
        let (eq7_e414, eq7_e414_d_n21,) = {
    if (l.f1db2 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p329;let eq7_e412: f64 = ((nv21 - 0.0) * __rspice_inv_cse_0);let eq7_e412_d_n21: f64 = (1.0 * __rspice_inv_cse_0);
        (eq7_e412, eq7_e412_d_n21,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e414;
        stamper.stamp_current_node1_local(
            Some(21),
            None,
            multiplicity * (eq7_value),
            21,
            multiplicity * (eq7_e414_d_n21),
        );
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21,) = {
    if (l.f1db2 != 0.0) {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));let eq8_e419: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq8_e418);
        (eq8_e419, ((-p.p330) * ddt_scale), (p.p330 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e421;
        stamper.stamp_current_node2_local(
            Some(21),
            Some(20),
            multiplicity * (eq8_value),
            20,
            multiplicity * (eq8_e421_d_n20),
            21,
            multiplicity * (eq8_e421_d_n21),
        );
        let (eq9_e428, eq9_e428_d_n20,) = {
    if (l.f1db2 != 0.0) {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));let eq9_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq9_e425);
        (eq9_e426, (p.p332 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e428;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (eq9_value),
            20,
            multiplicity * (eq9_e428_d_n20),
        );
        let (eq10_e432, eq10_e432_d_n20,) = {
    if (l.f1db2 != 0.0) {
        ((nv20 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e432;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (eq10_value),
            20,
            multiplicity * (eq10_e432_d_n20),
        );
        let (eq11_e439,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e439;
        stamper.stamp_potential_const_local(
            6,
            eq11_value,
        );
        let (eq12_e446,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e446;
        stamper.stamp_potential_const_local(
            7,
            eq12_value,
        );
        let (eq13_e455, eq13_e455_d_n0, eq13_e455_d_n2,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq13_e453: f64 = (p.p6 * (nv0 - nv2));
        (eq13_e453, p.p6, (-p.p6),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e455;
        stamper.stamp_potential_node2_local(
            8,
            eq13_value,
            0,
            eq13_e455_d_n0,
            2,
            eq13_e455_d_n2,
        );
    }
    #[inline(never)]
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
        l: &mut StampLocals,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv22 = ctx.node_voltage(nodes[22]);let nv23 = ctx.node_voltage(nodes[23]);let nv24 = ctx.node_voltage(nodes[24]);
        let (eq14_e518, eq14_e518_d_n4, eq14_e518_d_n23, eq14_e518_d_n24,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq14_e463: f64 = ((nv24 - nv23) / l.f215b);let eq14_e463_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_0: f64 = 1.0 / l.f215b;let eq14_e463_d_n23: f64 = ((-1.0) * __rspice_inv_cse_0);let eq14_e463_d_n24: f64 = (1.0 * __rspice_inv_cse_0);let eq14_e469: f64 = ((nv24 - nv23) * __rspice_inv_cse_0);let eq14_e469_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_1: f64 = 1.0 / l.f215b;let eq14_e469_d_n23: f64 = ((-1.0) * __rspice_inv_cse_1);let eq14_e469_d_n24: f64 = (1.0 * __rspice_inv_cse_1);let eq14_e471: f64 = (-50.0);
        let (eq14_e513, eq14_e513_d_n4, eq14_e513_d_n23, eq14_e513_d_n24,) = {
            if ((!(eq14_e463 > 50.0)) && (!(eq14_e469 < eq14_e471))) {
                let eq14_e477: f64 = ((nv24 - nv23) / l.f215b);let eq14_e477_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_2: f64 = 1.0 / l.f215b;let eq14_e477_d_n23: f64 = ((-1.0) * __rspice_inv_cse_2);let eq14_e477_d_n24: f64 = (1.0 * __rspice_inv_cse_2);let eq14_e478: f64 = (eq14_e477).exp();let eq14_e478_d_n4: f64 = (eq14_e478 * eq14_e477_d_n4);let eq14_e478_d_n23: f64 = (eq14_e478 * eq14_e477_d_n23);let eq14_e478_d_n24: f64 = (eq14_e478 * eq14_e477_d_n24);
                (eq14_e478, eq14_e478_d_n4, eq14_e478_d_n23, eq14_e478_d_n24,)
            } else {
                let eq14_e481: f64 = ((nv24 - nv23) / l.f215b);let eq14_e481_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_3: f64 = 1.0 / l.f215b;let eq14_e481_d_n23: f64 = ((-1.0) * __rspice_inv_cse_3);let eq14_e481_d_n24: f64 = (1.0 * __rspice_inv_cse_3);let eq14_e487: f64 = ((nv24 - nv23) * __rspice_inv_cse_3);let eq14_e487_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_4: f64 = 1.0 / l.f215b;let eq14_e487_d_n23: f64 = ((-1.0) * __rspice_inv_cse_4);let eq14_e487_d_n24: f64 = (1.0 * __rspice_inv_cse_4);let eq14_e489: f64 = (-50.0);
                let (eq14_e512, eq14_e512_d_n4, eq14_e512_d_n23, eq14_e512_d_n24,) = {
                    if ((!(eq14_e481 > 50.0)) && (eq14_e487 < eq14_e489)) {
                        let eq14_e493: f64 = (-50.0);let eq14_e494: f64 = (eq14_e493).exp();
                        (eq14_e494, 0.0, 0.0, 0.0,)
                    } else {
                        let eq14_e497: f64 = ((nv24 - nv23) / l.f215b);let eq14_e497_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_5: f64 = 1.0 / l.f215b;let eq14_e497_d_n23: f64 = ((-1.0) * __rspice_inv_cse_5);let eq14_e497_d_n24: f64 = (1.0 * __rspice_inv_cse_5);
                        let (eq14_e511, eq14_e511_d_n4, eq14_e511_d_n23, eq14_e511_d_n24,) = {
                            if (eq14_e497 > 50.0) {
                                let eq14_e501: f64 = (50.0_f64).exp();let eq14_e505: f64 = ((nv24 - nv23) / l.f215b);let eq14_e505_d_n4: f64 = (-(((nv24 - nv23) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_6: f64 = 1.0 / l.f215b;let eq14_e505_d_n23: f64 = ((-1.0) * __rspice_inv_cse_6);let eq14_e505_d_n24: f64 = (1.0 * __rspice_inv_cse_6);let eq14_e507: f64 = (eq14_e505 - 50.0);let eq14_e508: f64 = (1.0 + eq14_e507);let eq14_e509: f64 = (eq14_e501 * eq14_e508);let eq14_e509_d_n4: f64 = (eq14_e501 * eq14_e505_d_n4);let eq14_e509_d_n23: f64 = (eq14_e501 * eq14_e505_d_n23);let eq14_e509_d_n24: f64 = (eq14_e501 * eq14_e505_d_n24);
                                (eq14_e509, eq14_e509_d_n4, eq14_e509_d_n23, eq14_e509_d_n24,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (eq14_e511, eq14_e511_d_n4, eq14_e511_d_n23, eq14_e511_d_n24,)
                    }
                };
                (eq14_e512, eq14_e512_d_n4, eq14_e512_d_n23, eq14_e512_d_n24,)
            }
        };let eq14_e515: f64 = (eq14_e513 - 1.0);let eq14_e516: f64 = (p.p346 * eq14_e515);let eq14_e516_d_n4: f64 = (p.p346 * eq14_e513_d_n4);let eq14_e516_d_n23: f64 = (p.p346 * eq14_e513_d_n23);let eq14_e516_d_n24: f64 = (p.p346 * eq14_e513_d_n24);
        (eq14_e516, eq14_e516_d_n4, eq14_e516_d_n23, eq14_e516_d_n24,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e518;
        stamper.stamp_current_node3_local(
            Some(24),
            Some(23),
            multiplicity * (eq14_value),
            4,
            multiplicity * (eq14_e518_d_n4),
            23,
            multiplicity * (eq14_e518_d_n23),
            24,
            multiplicity * (eq14_e518_d_n24),
        );
        let (eq15_e527, eq15_e527_d_n22, eq15_e527_d_n24,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let __rspice_inv_cse_7: f64 = 1.0 / p.p340;let eq15_e525: f64 = ((nv22 - nv24) * __rspice_inv_cse_7);let eq15_e525_d_n22: f64 = (1.0 * __rspice_inv_cse_7);let eq15_e525_d_n24: f64 = ((-1.0) * __rspice_inv_cse_7);
        (eq15_e525, eq15_e525_d_n22, eq15_e525_d_n24,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e527;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(24),
            multiplicity * (eq15_value),
            22,
            multiplicity * (eq15_e527_d_n22),
            24,
            multiplicity * (eq15_e527_d_n24),
        );
        let (eq16_e536, eq16_e536_d_n22, eq16_e536_d_n23,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let __rspice_inv_cse_8: f64 = 1.0 / p.p339;let eq16_e534: f64 = ((nv22 - nv23) * __rspice_inv_cse_8);let eq16_e534_d_n22: f64 = (1.0 * __rspice_inv_cse_8);let eq16_e534_d_n23: f64 = ((-1.0) * __rspice_inv_cse_8);
        (eq16_e534, eq16_e534_d_n22, eq16_e534_d_n23,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e536;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(23),
            multiplicity * (eq16_value),
            22,
            multiplicity * (eq16_e536_d_n22),
            23,
            multiplicity * (eq16_e536_d_n23),
        );
        let (eq17_e564, eq17_e564_d_n4, eq17_e564_d_n23,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq17_e543: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, l.f234a);let eq17_e544: f64 = (p.p341 * eq17_e543);let eq17_e544_d_n23: f64 = (p.p341 * (l.f234b * ddt_scale));let eq17_e549: f64 = (l.f22ef - l.f22f4);let eq17_e550: f64 = (p.p342 * eq17_e549);let eq17_e550_d_n4: f64 = (p.p342 * l.f22f0);let eq17_e551: f64 = (1.0 + eq17_e550);let eq17_e555: f64 = (l.f22ef - l.f22f4);let eq17_e556: f64 = (p.p344 * eq17_e555);let eq17_e556_d_n4: f64 = (p.p344 * l.f22f0);let eq17_e559: f64 = (l.f22ef - l.f22f4);let eq17_e560: f64 = (eq17_e556 * eq17_e559);let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * l.f22f0));let eq17_e561: f64 = (eq17_e551 + eq17_e560);let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);let eq17_e562: f64 = (eq17_e544 * eq17_e561);let eq17_e562_d_n4: f64 = (eq17_e544 * eq17_e561_d_n4);let eq17_e562_d_n23: f64 = (eq17_e544_d_n23 * eq17_e561);
        (eq17_e562, eq17_e562_d_n4, eq17_e562_d_n23,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e564;
        stamper.stamp_current_node2_local(
            Some(23),
            None,
            multiplicity * (eq17_value),
            4,
            multiplicity * (eq17_e564_d_n4),
            23,
            multiplicity * (eq17_e564_d_n23),
        );
        let (eq18_e573, eq18_e573_d_n1, eq18_e573_d_n2,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq18_e571: f64 = (p.p6 * (nv1 - nv2));
        (eq18_e571, p.p6, (-p.p6),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e573;
        stamper.stamp_potential_node2_local(
            9,
            eq18_value,
            1,
            eq18_e573_d_n1,
            2,
            eq18_e573_d_n2,
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
        let nv25 = ctx.node_voltage(nodes[25]);let nv26 = ctx.node_voltage(nodes[26]);let nv27 = ctx.node_voltage(nodes[27]);
        let (eq19_e636, eq19_e636_d_n4, eq19_e636_d_n26, eq19_e636_d_n27,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq19_e581: f64 = ((nv26 - nv27) / l.f215b);let eq19_e581_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_0: f64 = 1.0 / l.f215b;let eq19_e581_d_n26: f64 = (1.0 * __rspice_inv_cse_0);let eq19_e581_d_n27: f64 = ((-1.0) * __rspice_inv_cse_0);let eq19_e587: f64 = ((nv26 - nv27) * __rspice_inv_cse_0);let eq19_e587_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_1: f64 = 1.0 / l.f215b;let eq19_e587_d_n26: f64 = (1.0 * __rspice_inv_cse_1);let eq19_e587_d_n27: f64 = ((-1.0) * __rspice_inv_cse_1);let eq19_e589: f64 = (-50.0);
        let (eq19_e631, eq19_e631_d_n4, eq19_e631_d_n26, eq19_e631_d_n27,) = {
            if ((!(eq19_e581 > 50.0)) && (!(eq19_e587 < eq19_e589))) {
                let eq19_e595: f64 = ((nv26 - nv27) / l.f215b);let eq19_e595_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_2: f64 = 1.0 / l.f215b;let eq19_e595_d_n26: f64 = (1.0 * __rspice_inv_cse_2);let eq19_e595_d_n27: f64 = ((-1.0) * __rspice_inv_cse_2);let eq19_e596: f64 = (eq19_e595).exp();let eq19_e596_d_n4: f64 = (eq19_e596 * eq19_e595_d_n4);let eq19_e596_d_n26: f64 = (eq19_e596 * eq19_e595_d_n26);let eq19_e596_d_n27: f64 = (eq19_e596 * eq19_e595_d_n27);
                (eq19_e596, eq19_e596_d_n4, eq19_e596_d_n26, eq19_e596_d_n27,)
            } else {
                let eq19_e599: f64 = ((nv26 - nv27) / l.f215b);let eq19_e599_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_3: f64 = 1.0 / l.f215b;let eq19_e599_d_n26: f64 = (1.0 * __rspice_inv_cse_3);let eq19_e599_d_n27: f64 = ((-1.0) * __rspice_inv_cse_3);let eq19_e605: f64 = ((nv26 - nv27) * __rspice_inv_cse_3);let eq19_e605_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_4: f64 = 1.0 / l.f215b;let eq19_e605_d_n26: f64 = (1.0 * __rspice_inv_cse_4);let eq19_e605_d_n27: f64 = ((-1.0) * __rspice_inv_cse_4);let eq19_e607: f64 = (-50.0);
                let (eq19_e630, eq19_e630_d_n4, eq19_e630_d_n26, eq19_e630_d_n27,) = {
                    if ((!(eq19_e599 > 50.0)) && (eq19_e605 < eq19_e607)) {
                        let eq19_e611: f64 = (-50.0);let eq19_e612: f64 = (eq19_e611).exp();
                        (eq19_e612, 0.0, 0.0, 0.0,)
                    } else {
                        let eq19_e615: f64 = ((nv26 - nv27) / l.f215b);let eq19_e615_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_5: f64 = 1.0 / l.f215b;let eq19_e615_d_n26: f64 = (1.0 * __rspice_inv_cse_5);let eq19_e615_d_n27: f64 = ((-1.0) * __rspice_inv_cse_5);
                        let (eq19_e629, eq19_e629_d_n4, eq19_e629_d_n26, eq19_e629_d_n27,) = {
                            if (eq19_e615 > 50.0) {
                                let eq19_e619: f64 = (50.0_f64).exp();let eq19_e623: f64 = ((nv26 - nv27) / l.f215b);let eq19_e623_d_n4: f64 = (-(((nv26 - nv27) * l.f215c) / (l.f215b * l.f215b)));let __rspice_inv_cse_6: f64 = 1.0 / l.f215b;let eq19_e623_d_n26: f64 = (1.0 * __rspice_inv_cse_6);let eq19_e623_d_n27: f64 = ((-1.0) * __rspice_inv_cse_6);let eq19_e625: f64 = (eq19_e623 - 50.0);let eq19_e626: f64 = (1.0 + eq19_e625);let eq19_e627: f64 = (eq19_e619 * eq19_e626);let eq19_e627_d_n4: f64 = (eq19_e619 * eq19_e623_d_n4);let eq19_e627_d_n26: f64 = (eq19_e619 * eq19_e623_d_n26);let eq19_e627_d_n27: f64 = (eq19_e619 * eq19_e623_d_n27);
                                (eq19_e627, eq19_e627_d_n4, eq19_e627_d_n26, eq19_e627_d_n27,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (eq19_e629, eq19_e629_d_n4, eq19_e629_d_n26, eq19_e629_d_n27,)
                    }
                };
                (eq19_e630, eq19_e630_d_n4, eq19_e630_d_n26, eq19_e630_d_n27,)
            }
        };let eq19_e633: f64 = (eq19_e631 - 1.0);let eq19_e634: f64 = (p.p346 * eq19_e633);let eq19_e634_d_n4: f64 = (p.p346 * eq19_e631_d_n4);let eq19_e634_d_n26: f64 = (p.p346 * eq19_e631_d_n26);let eq19_e634_d_n27: f64 = (p.p346 * eq19_e631_d_n27);
        (eq19_e634, eq19_e634_d_n4, eq19_e634_d_n26, eq19_e634_d_n27,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e636;
        stamper.stamp_current_node3_local(
            Some(26),
            Some(27),
            multiplicity * (eq19_value),
            4,
            multiplicity * (eq19_e636_d_n4),
            26,
            multiplicity * (eq19_e636_d_n26),
            27,
            multiplicity * (eq19_e636_d_n27),
        );
        let (eq20_e645, eq20_e645_d_n25, eq20_e645_d_n27,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let __rspice_inv_cse_7: f64 = 1.0 / p.p340;let eq20_e643: f64 = ((nv25 - nv27) * __rspice_inv_cse_7);let eq20_e643_d_n25: f64 = (1.0 * __rspice_inv_cse_7);let eq20_e643_d_n27: f64 = ((-1.0) * __rspice_inv_cse_7);
        (eq20_e643, eq20_e643_d_n25, eq20_e643_d_n27,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e645;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(27),
            multiplicity * (eq20_value),
            25,
            multiplicity * (eq20_e645_d_n25),
            27,
            multiplicity * (eq20_e645_d_n27),
        );
        let (eq21_e654, eq21_e654_d_n25, eq21_e654_d_n26,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let __rspice_inv_cse_8: f64 = 1.0 / p.p339;let eq21_e652: f64 = ((nv25 - nv26) * __rspice_inv_cse_8);let eq21_e652_d_n25: f64 = (1.0 * __rspice_inv_cse_8);let eq21_e652_d_n26: f64 = ((-1.0) * __rspice_inv_cse_8);
        (eq21_e652, eq21_e652_d_n25, eq21_e652_d_n26,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e654;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(26),
            multiplicity * (eq21_value),
            25,
            multiplicity * (eq21_e654_d_n25),
            26,
            multiplicity * (eq21_e654_d_n26),
        );
        let (eq22_e682, eq22_e682_d_n4, eq22_e682_d_n26,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {
        let eq22_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, l.f237a);let eq22_e662: f64 = (p.p341 * eq22_e661);let eq22_e662_d_n26: f64 = (p.p341 * (l.f237b * ddt_scale));let eq22_e667: f64 = (l.f22ef - l.f22f4);let eq22_e668: f64 = (p.p343 * eq22_e667);let eq22_e668_d_n4: f64 = (p.p343 * l.f22f0);let eq22_e669: f64 = (1.0 + eq22_e668);let eq22_e673: f64 = (l.f22ef - l.f22f4);let eq22_e674: f64 = (p.p345 * eq22_e673);let eq22_e674_d_n4: f64 = (p.p345 * l.f22f0);let eq22_e677: f64 = (l.f22ef - l.f22f4);let eq22_e678: f64 = (eq22_e674 * eq22_e677);let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * l.f22f0));let eq22_e679: f64 = (eq22_e669 + eq22_e678);let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);let eq22_e680: f64 = (eq22_e662 * eq22_e679);let eq22_e680_d_n4: f64 = (eq22_e662 * eq22_e679_d_n4);let eq22_e680_d_n26: f64 = (eq22_e662_d_n26 * eq22_e679);
        (eq22_e680, eq22_e680_d_n4, eq22_e680_d_n26,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e682;
        stamper.stamp_current_node2_local(
            Some(26),
            None,
            multiplicity * (eq22_value),
            4,
            multiplicity * (eq22_e682_d_n4),
            26,
            multiplicity * (eq22_e682_d_n26),
        );
        let (eq23_e690,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e690;
        stamper.stamp_potential_const_local(
            10,
            eq23_value,
        );
        let (eq24_e698,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e698;
        stamper.stamp_potential_const_local(
            11,
            eq24_value,
        );
        let (eq25_e706,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e706;
        stamper.stamp_potential_const_local(
            12,
            eq25_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);
        let (eq26_e714,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e714;
        stamper.stamp_potential_const_local(
            13,
            eq26_value,
        );
        let (eq27_e722,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e722;
        stamper.stamp_potential_const_local(
            14,
            eq27_value,
        );
        let (eq28_e730,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e730;
        stamper.stamp_potential_const_local(
            15,
            eq28_value,
        );
        let (eq29_e738,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e738;
        stamper.stamp_potential_const_local(
            16,
            eq29_value,
        );
        let (eq30_e746,) = {
    if ((l.f1db2 == 0.0) && (l.f1dc8 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e746;
        stamper.stamp_potential_const_local(
            17,
            eq30_value,
        );
        let (eq31_e754, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n7, eq31_e754_d_n16, eq31_e754_d_n17,) = {
    if (l.f1eb3 != 0.0) {
        let eq31_e751: f64 = (l.f1d87 * (nv17 - nv16));let eq31_e752: f64 = (l.f20e3 + eq31_e751);let eq31_e752_d_n16: f64 = (l.f20e4 + (-l.f1d87));let eq31_e752_d_n17: f64 = (l.f20e5 + l.f1d87);
        (eq31_e752, l.f20e6, l.f20e7, l.f20e8, l.f20e9, eq31_e752_d_n16, eq31_e752_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e754;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(17),
            Some(16),
            multiplicity * (eq31_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * (eq31_e754_d_n2), multiplicity * (eq31_e754_d_n3), multiplicity * (eq31_e754_d_n4), multiplicity * (eq31_e754_d_n7), multiplicity * (eq31_e754_d_n16), multiplicity * (eq31_e754_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq32_e759,) = {
    if (l.f1eb3 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e759;
        stamper.stamp_potential_const_local(
            18,
            eq32_value,
        );
        let (eq33_e769, eq33_e769_d_n2, eq33_e769_d_n4, eq33_e769_d_n7, eq33_e769_d_n16, eq33_e769_d_n17,) = {
    if (l.f206a != 0.0) {
        let eq33_e762: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, l.f223f);let eq33_e765: f64 = (p.p355 * (nv7 - nv16));let eq33_e766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq33_e765);let eq33_e767: f64 = (eq33_e762 + eq33_e766);let eq33_e767_d_n7: f64 = ((l.f2244 * ddt_scale) + (p.p355 * ddt_scale));let eq33_e767_d_n16: f64 = ((l.f2240 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq33_e767, (l.f2242 * ddt_scale), (l.f2243 * ddt_scale), eq33_e767_d_n7, eq33_e767_d_n16, (l.f2241 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e769;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(16),
            multiplicity * (eq33_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq33_e769_d_n2), multiplicity * (eq33_e769_d_n4), multiplicity * (eq33_e769_d_n7), multiplicity * (eq33_e769_d_n16), multiplicity * (eq33_e769_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e779, eq34_e779_d_n2, eq34_e779_d_n4, eq34_e779_d_n7, eq34_e779_d_n16, eq34_e779_d_n17,) = {
    if (l.f206a != 0.0) {
        let eq34_e772: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, l.f21fd);let eq34_e775: f64 = (p.p355 * (nv7 - nv17));let eq34_e776: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq34_e775);let eq34_e777: f64 = (eq34_e772 + eq34_e776);let eq34_e777_d_n7: f64 = ((l.f2202 * ddt_scale) + (p.p355 * ddt_scale));let eq34_e777_d_n17: f64 = ((l.f21ff * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq34_e777, (l.f2200 * ddt_scale), (l.f2201 * ddt_scale), eq34_e777_d_n7, (l.f21fe * ddt_scale), eq34_e777_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e779;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(17),
            multiplicity * (eq34_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq34_e779_d_n2), multiplicity * (eq34_e779_d_n4), multiplicity * (eq34_e779_d_n7), multiplicity * (eq34_e779_d_n16), multiplicity * (eq34_e779_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e789, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n7, eq35_e789_d_n16, eq35_e789_d_n17,) = {
    if (l.f206a != 0.0) {
        let eq35_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, l.f21b6);let eq35_e785: f64 = (p.p355 * (nv2 - nv16));let eq35_e786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq35_e785);let eq35_e787: f64 = (eq35_e782 + eq35_e786);let eq35_e787_d_n2: f64 = ((l.f21b9 * ddt_scale) + (p.p355 * ddt_scale));let eq35_e787_d_n16: f64 = ((l.f21b7 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq35_e787, eq35_e787_d_n2, (l.f21ba * ddt_scale), (l.f21bb * ddt_scale), (l.f21bc * ddt_scale), eq35_e787_d_n16, (l.f21b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e789;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(16),
            multiplicity * (eq35_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * (eq35_e789_d_n2), multiplicity * (eq35_e789_d_n3), multiplicity * (eq35_e789_d_n4), multiplicity * (eq35_e789_d_n7), multiplicity * (eq35_e789_d_n16), multiplicity * (eq35_e789_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq37_e803, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n7, eq37_e803_d_n9, eq37_e803_d_n16, eq37_e803_d_n17,) = {
    if (l.f206a != 0.0) {
        let eq37_e796: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, l.f22bc);let eq37_e799: f64 = (p.p355 * (nv7 - nv9));let eq37_e800: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq37_e799);let eq37_e801: f64 = (eq37_e796 + eq37_e800);let eq37_e801_d_n7: f64 = ((l.f22c2 * ddt_scale) + (p.p355 * ddt_scale));
        (eq37_e801, (l.f22bf * ddt_scale), (l.f22c0 * ddt_scale), (l.f22c1 * ddt_scale), eq37_e801_d_n7, ((-p.p355) * ddt_scale), (l.f22bd * ddt_scale), (l.f22be * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e803;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq37_value),
            [2, 3, 4, 7, 9, 16, 17],
            [multiplicity * (eq37_e803_d_n2), multiplicity * (eq37_e803_d_n3), multiplicity * (eq37_e803_d_n4), multiplicity * (eq37_e803_d_n7), multiplicity * (eq37_e803_d_n9), multiplicity * (eq37_e803_d_n16), multiplicity * (eq37_e803_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq38_e814, eq38_e814_d_n2, eq38_e814_d_n4, eq38_e814_d_n7, eq38_e814_d_n16, eq38_e814_d_n17,) = {
    if (l.f206a == 0.0) {
        let eq38_e807: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, l.f223f);let eq38_e810: f64 = (p.p355 * (nv2 - nv16));let eq38_e811: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq38_e810);let eq38_e812: f64 = (eq38_e807 + eq38_e811);let eq38_e812_d_n2: f64 = ((l.f2242 * ddt_scale) + (p.p355 * ddt_scale));let eq38_e812_d_n16: f64 = ((l.f2240 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq38_e812, eq38_e812_d_n2, (l.f2243 * ddt_scale), (l.f2244 * ddt_scale), eq38_e812_d_n16, (l.f2241 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e814;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(16),
            multiplicity * (eq38_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq38_e814_d_n2), multiplicity * (eq38_e814_d_n4), multiplicity * (eq38_e814_d_n7), multiplicity * (eq38_e814_d_n16), multiplicity * (eq38_e814_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq39_e825, eq39_e825_d_n2, eq39_e825_d_n4, eq39_e825_d_n7, eq39_e825_d_n16, eq39_e825_d_n17,) = {
    if (l.f206a == 0.0) {
        let eq39_e818: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, l.f21fd);let eq39_e821: f64 = (p.p355 * (nv2 - nv17));let eq39_e822: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq39_e821);let eq39_e823: f64 = (eq39_e818 + eq39_e822);let eq39_e823_d_n2: f64 = ((l.f2200 * ddt_scale) + (p.p355 * ddt_scale));let eq39_e823_d_n17: f64 = ((l.f21ff * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq39_e823, eq39_e823_d_n2, (l.f2201 * ddt_scale), (l.f2202 * ddt_scale), (l.f21fe * ddt_scale), eq39_e823_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e825;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(17),
            multiplicity * (eq39_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq39_e825_d_n2), multiplicity * (eq39_e825_d_n4), multiplicity * (eq39_e825_d_n7), multiplicity * (eq39_e825_d_n16), multiplicity * (eq39_e825_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq40_e836, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n7, eq40_e836_d_n16, eq40_e836_d_n17,) = {
    if (l.f206a == 0.0) {
        let eq40_e829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, l.f21b6);let eq40_e832: f64 = (p.p355 * (nv7 - nv16));let eq40_e833: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq40_e832);let eq40_e834: f64 = (eq40_e829 + eq40_e833);let eq40_e834_d_n7: f64 = ((l.f21bc * ddt_scale) + (p.p355 * ddt_scale));let eq40_e834_d_n16: f64 = ((l.f21b7 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq40_e834, (l.f21b9 * ddt_scale), (l.f21ba * ddt_scale), (l.f21bb * ddt_scale), eq40_e834_d_n7, eq40_e834_d_n16, (l.f21b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e836;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(16),
            multiplicity * (eq40_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * (eq40_e836_d_n2), multiplicity * (eq40_e836_d_n3), multiplicity * (eq40_e836_d_n4), multiplicity * (eq40_e836_d_n7), multiplicity * (eq40_e836_d_n16), multiplicity * (eq40_e836_d_n17)],
            [],
            [],
            1.0,
        );let eq43_e848: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, l.f2176);let eq43_e851: f64 = (p.p355 * (nv3 - nv16));let eq43_e852: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq43_e851);let eq43_e853: f64 = (eq43_e848 + eq43_e852);let eq43_e853_d_n3: f64 = ((l.f217a * ddt_scale) + (p.p355 * ddt_scale));let eq43_e853_d_n16: f64 = ((l.f2177 * ddt_scale) + ((-p.p355) * ddt_scale));let eq43_value: f64 = eq43_e853;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(16),
            multiplicity * (eq43_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * ((l.f2179 * ddt_scale)), multiplicity * (eq43_e853_d_n3), multiplicity * ((l.f217b * ddt_scale)), multiplicity * ((l.f217c * ddt_scale)), multiplicity * (eq43_e853_d_n16), multiplicity * ((l.f2178 * ddt_scale))],
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);
        let (eq44_e861, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n7, eq44_e861_d_n15, eq44_e861_d_n16,) = {
    if (l.f206d != 0.0) {
        let eq44_e858: f64 = (l.f1d87 * (nv16 - nv15));let eq44_e859: f64 = (l.f20dc + eq44_e858);let eq44_e859_d_n15: f64 = (l.f20dd + (-l.f1d87));let eq44_e859_d_n16: f64 = (l.f20de + l.f1d87);
        (eq44_e859, l.f20df, l.f20e0, l.f20e1, l.f20e2, eq44_e859_d_n15, eq44_e859_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e861;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(16),
            Some(15),
            multiplicity * (eq44_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * (eq44_e861_d_n2), multiplicity * (eq44_e861_d_n3), multiplicity * (eq44_e861_d_n4), multiplicity * (eq44_e861_d_n7), multiplicity * (eq44_e861_d_n15), multiplicity * (eq44_e861_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq45_e866,) = {
    if (l.f206d == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e866;
        stamper.stamp_potential_const_local(
            19,
            eq45_value,
        );
        let (eq46_e876, eq46_e876_d_n2, eq46_e876_d_n4, eq46_e876_d_n7, eq46_e876_d_n15, eq46_e876_d_n16,) = {
    if (l.f20b4 != 0.0) {
        let eq46_e869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, l.f2238);let eq46_e872: f64 = (p.p355 * (nv7 - nv15));let eq46_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq46_e872);let eq46_e874: f64 = (eq46_e869 + eq46_e873);let eq46_e874_d_n7: f64 = ((l.f223d * ddt_scale) + (p.p355 * ddt_scale));let eq46_e874_d_n15: f64 = ((l.f2239 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq46_e874, (l.f223b * ddt_scale), (l.f223c * ddt_scale), eq46_e874_d_n7, eq46_e874_d_n15, (l.f223a * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e876;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(15),
            multiplicity * (eq46_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq46_e876_d_n2), multiplicity * (eq46_e876_d_n4), multiplicity * (eq46_e876_d_n7), multiplicity * (eq46_e876_d_n15), multiplicity * (eq46_e876_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq47_e886, eq47_e886_d_n2, eq47_e886_d_n4, eq47_e886_d_n7, eq47_e886_d_n15, eq47_e886_d_n16,) = {
    if (l.f20b4 != 0.0) {
        let eq47_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, l.f21f6);let eq47_e882: f64 = (p.p355 * (nv7 - nv16));let eq47_e883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq47_e882);let eq47_e884: f64 = (eq47_e879 + eq47_e883);let eq47_e884_d_n7: f64 = ((l.f21fb * ddt_scale) + (p.p355 * ddt_scale));let eq47_e884_d_n16: f64 = ((l.f21f8 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq47_e884, (l.f21f9 * ddt_scale), (l.f21fa * ddt_scale), eq47_e884_d_n7, (l.f21f7 * ddt_scale), eq47_e884_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e886;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(16),
            multiplicity * (eq47_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq47_e886_d_n2), multiplicity * (eq47_e886_d_n4), multiplicity * (eq47_e886_d_n7), multiplicity * (eq47_e886_d_n15), multiplicity * (eq47_e886_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq48_e896, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n7, eq48_e896_d_n15, eq48_e896_d_n16,) = {
    if (l.f20b4 != 0.0) {
        let eq48_e889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, l.f21ae);let eq48_e892: f64 = (p.p355 * (nv2 - nv15));let eq48_e893: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, eq48_e892);let eq48_e894: f64 = (eq48_e889 + eq48_e893);let eq48_e894_d_n2: f64 = ((l.f21b1 * ddt_scale) + (p.p355 * ddt_scale));let eq48_e894_d_n15: f64 = ((l.f21af * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq48_e894, eq48_e894_d_n2, (l.f21b2 * ddt_scale), (l.f21b3 * ddt_scale), (l.f21b4 * ddt_scale), eq48_e894_d_n15, (l.f21b0 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e896;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(15),
            multiplicity * (eq48_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * (eq48_e896_d_n2), multiplicity * (eq48_e896_d_n3), multiplicity * (eq48_e896_d_n4), multiplicity * (eq48_e896_d_n7), multiplicity * (eq48_e896_d_n15), multiplicity * (eq48_e896_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq50_e910, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n7, eq50_e910_d_n9, eq50_e910_d_n15, eq50_e910_d_n16,) = {
    if (l.f20b4 != 0.0) {
        let eq50_e903: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, l.f22b4);let eq50_e906: f64 = (p.p355 * (nv7 - nv9));let eq50_e907: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, eq50_e906);let eq50_e908: f64 = (eq50_e903 + eq50_e907);let eq50_e908_d_n7: f64 = ((l.f22ba * ddt_scale) + (p.p355 * ddt_scale));
        (eq50_e908, (l.f22b7 * ddt_scale), (l.f22b8 * ddt_scale), (l.f22b9 * ddt_scale), eq50_e908_d_n7, ((-p.p355) * ddt_scale), (l.f22b5 * ddt_scale), (l.f22b6 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e910;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq50_value),
            [2, 3, 4, 7, 9, 15, 16],
            [multiplicity * (eq50_e910_d_n2), multiplicity * (eq50_e910_d_n3), multiplicity * (eq50_e910_d_n4), multiplicity * (eq50_e910_d_n7), multiplicity * (eq50_e910_d_n9), multiplicity * (eq50_e910_d_n15), multiplicity * (eq50_e910_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq51_e921, eq51_e921_d_n2, eq51_e921_d_n4, eq51_e921_d_n7, eq51_e921_d_n15, eq51_e921_d_n16,) = {
    if (l.f20b4 == 0.0) {
        let eq51_e914: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 28, l.f2238);let eq51_e917: f64 = (p.p355 * (nv2 - nv15));let eq51_e918: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 29, eq51_e917);let eq51_e919: f64 = (eq51_e914 + eq51_e918);let eq51_e919_d_n2: f64 = ((l.f223b * ddt_scale) + (p.p355 * ddt_scale));let eq51_e919_d_n15: f64 = ((l.f2239 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq51_e919, eq51_e919_d_n2, (l.f223c * ddt_scale), (l.f223d * ddt_scale), eq51_e919_d_n15, (l.f223a * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e921;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(15),
            multiplicity * (eq51_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq51_e921_d_n2), multiplicity * (eq51_e921_d_n4), multiplicity * (eq51_e921_d_n7), multiplicity * (eq51_e921_d_n15), multiplicity * (eq51_e921_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq52_e932, eq52_e932_d_n2, eq52_e932_d_n4, eq52_e932_d_n7, eq52_e932_d_n15, eq52_e932_d_n16,) = {
    if (l.f20b4 == 0.0) {
        let eq52_e925: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 30, l.f21f6);let eq52_e928: f64 = (p.p355 * (nv2 - nv16));let eq52_e929: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 31, eq52_e928);let eq52_e930: f64 = (eq52_e925 + eq52_e929);let eq52_e930_d_n2: f64 = ((l.f21f9 * ddt_scale) + (p.p355 * ddt_scale));let eq52_e930_d_n16: f64 = ((l.f21f8 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq52_e930, eq52_e930_d_n2, (l.f21fa * ddt_scale), (l.f21fb * ddt_scale), (l.f21f7 * ddt_scale), eq52_e930_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e932;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(16),
            multiplicity * (eq52_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq52_e932_d_n2), multiplicity * (eq52_e932_d_n4), multiplicity * (eq52_e932_d_n7), multiplicity * (eq52_e932_d_n15), multiplicity * (eq52_e932_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq53_e943, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n7, eq53_e943_d_n15, eq53_e943_d_n16,) = {
    if (l.f20b4 == 0.0) {
        let eq53_e936: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 32, l.f21ae);let eq53_e939: f64 = (p.p355 * (nv7 - nv15));let eq53_e940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 33, eq53_e939);let eq53_e941: f64 = (eq53_e936 + eq53_e940);let eq53_e941_d_n7: f64 = ((l.f21b4 * ddt_scale) + (p.p355 * ddt_scale));let eq53_e941_d_n15: f64 = ((l.f21af * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq53_e941, (l.f21b1 * ddt_scale), (l.f21b2 * ddt_scale), (l.f21b3 * ddt_scale), eq53_e941_d_n7, eq53_e941_d_n15, (l.f21b0 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e943;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(15),
            multiplicity * (eq53_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * (eq53_e943_d_n2), multiplicity * (eq53_e943_d_n3), multiplicity * (eq53_e943_d_n4), multiplicity * (eq53_e943_d_n7), multiplicity * (eq53_e943_d_n15), multiplicity * (eq53_e943_d_n16)],
            [],
            [],
            1.0,
        );let eq56_e955: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 34, l.f216e);let eq56_e958: f64 = (p.p355 * (nv3 - nv15));let eq56_e959: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 35, eq56_e958);let eq56_e960: f64 = (eq56_e955 + eq56_e959);let eq56_e960_d_n3: f64 = ((l.f2172 * ddt_scale) + (p.p355 * ddt_scale));let eq56_e960_d_n15: f64 = ((l.f216f * ddt_scale) + ((-p.p355) * ddt_scale));let eq56_value: f64 = eq56_e960;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(15),
            multiplicity * (eq56_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * ((l.f2171 * ddt_scale)), multiplicity * (eq56_e960_d_n3), multiplicity * ((l.f2173 * ddt_scale)), multiplicity * ((l.f2174 * ddt_scale)), multiplicity * (eq56_e960_d_n15), multiplicity * ((l.f2170 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq57_e968, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n7, eq57_e968_d_n14, eq57_e968_d_n15,) = {
    if (l.f20b6 != 0.0) {
        let eq57_e965: f64 = (l.f1d87 * (nv15 - nv14));let eq57_e966: f64 = (l.f20d5 + eq57_e965);let eq57_e966_d_n14: f64 = (l.f20d6 + (-l.f1d87));let eq57_e966_d_n15: f64 = (l.f20d7 + l.f1d87);
        (eq57_e966, l.f20d8, l.f20d9, l.f20da, l.f20db, eq57_e966_d_n14, eq57_e966_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e968;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(15),
            Some(14),
            multiplicity * (eq57_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * (eq57_e968_d_n2), multiplicity * (eq57_e968_d_n3), multiplicity * (eq57_e968_d_n4), multiplicity * (eq57_e968_d_n7), multiplicity * (eq57_e968_d_n14), multiplicity * (eq57_e968_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq58_e973,) = {
    if (l.f20b6 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e973;
        stamper.stamp_potential_const_local(
            20,
            eq58_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);
        let (eq59_e983, eq59_e983_d_n2, eq59_e983_d_n4, eq59_e983_d_n7, eq59_e983_d_n14, eq59_e983_d_n15,) = {
    if (l.f1dcb != 0.0) {
        let eq59_e976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 36, l.f2231);let eq59_e979: f64 = (p.p355 * (nv7 - nv14));let eq59_e980: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 37, eq59_e979);let eq59_e981: f64 = (eq59_e976 + eq59_e980);let eq59_e981_d_n7: f64 = ((l.f2236 * ddt_scale) + (p.p355 * ddt_scale));let eq59_e981_d_n14: f64 = ((l.f2232 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq59_e981, (l.f2234 * ddt_scale), (l.f2235 * ddt_scale), eq59_e981_d_n7, eq59_e981_d_n14, (l.f2233 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e983;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(14),
            multiplicity * (eq59_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq59_e983_d_n2), multiplicity * (eq59_e983_d_n4), multiplicity * (eq59_e983_d_n7), multiplicity * (eq59_e983_d_n14), multiplicity * (eq59_e983_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq60_e993, eq60_e993_d_n2, eq60_e993_d_n4, eq60_e993_d_n7, eq60_e993_d_n14, eq60_e993_d_n15,) = {
    if (l.f1dcb != 0.0) {
        let eq60_e986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 38, l.f21ef);let eq60_e989: f64 = (p.p355 * (nv7 - nv15));let eq60_e990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 39, eq60_e989);let eq60_e991: f64 = (eq60_e986 + eq60_e990);let eq60_e991_d_n7: f64 = ((l.f21f4 * ddt_scale) + (p.p355 * ddt_scale));let eq60_e991_d_n15: f64 = ((l.f21f1 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq60_e991, (l.f21f2 * ddt_scale), (l.f21f3 * ddt_scale), eq60_e991_d_n7, (l.f21f0 * ddt_scale), eq60_e991_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e993;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(15),
            multiplicity * (eq60_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq60_e993_d_n2), multiplicity * (eq60_e993_d_n4), multiplicity * (eq60_e993_d_n7), multiplicity * (eq60_e993_d_n14), multiplicity * (eq60_e993_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq61_e1003, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n7, eq61_e1003_d_n14, eq61_e1003_d_n15,) = {
    if (l.f1dcb != 0.0) {
        let eq61_e996: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 40, l.f21a6);let eq61_e999: f64 = (p.p355 * (nv2 - nv14));let eq61_e1000: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 41, eq61_e999);let eq61_e1001: f64 = (eq61_e996 + eq61_e1000);let eq61_e1001_d_n2: f64 = ((l.f21a9 * ddt_scale) + (p.p355 * ddt_scale));let eq61_e1001_d_n14: f64 = ((l.f21a7 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq61_e1001, eq61_e1001_d_n2, (l.f21aa * ddt_scale), (l.f21ab * ddt_scale), (l.f21ac * ddt_scale), eq61_e1001_d_n14, (l.f21a8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1003;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(14),
            multiplicity * (eq61_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * (eq61_e1003_d_n2), multiplicity * (eq61_e1003_d_n3), multiplicity * (eq61_e1003_d_n4), multiplicity * (eq61_e1003_d_n7), multiplicity * (eq61_e1003_d_n14), multiplicity * (eq61_e1003_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq63_e1017, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n7, eq63_e1017_d_n9, eq63_e1017_d_n14, eq63_e1017_d_n15,) = {
    if (l.f1dcb != 0.0) {
        let eq63_e1010: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 42, l.f22ac);let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));let eq63_e1014: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 43, eq63_e1013);let eq63_e1015: f64 = (eq63_e1010 + eq63_e1014);let eq63_e1015_d_n7: f64 = ((l.f22b2 * ddt_scale) + (p.p355 * ddt_scale));
        (eq63_e1015, (l.f22af * ddt_scale), (l.f22b0 * ddt_scale), (l.f22b1 * ddt_scale), eq63_e1015_d_n7, ((-p.p355) * ddt_scale), (l.f22ad * ddt_scale), (l.f22ae * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1017;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq63_value),
            [2, 3, 4, 7, 9, 14, 15],
            [multiplicity * (eq63_e1017_d_n2), multiplicity * (eq63_e1017_d_n3), multiplicity * (eq63_e1017_d_n4), multiplicity * (eq63_e1017_d_n7), multiplicity * (eq63_e1017_d_n9), multiplicity * (eq63_e1017_d_n14), multiplicity * (eq63_e1017_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq64_e1028, eq64_e1028_d_n2, eq64_e1028_d_n4, eq64_e1028_d_n7, eq64_e1028_d_n14, eq64_e1028_d_n15,) = {
    if (l.f1dcb == 0.0) {
        let eq64_e1021: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 44, l.f2231);let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));let eq64_e1025: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 45, eq64_e1024);let eq64_e1026: f64 = (eq64_e1021 + eq64_e1025);let eq64_e1026_d_n2: f64 = ((l.f2234 * ddt_scale) + (p.p355 * ddt_scale));let eq64_e1026_d_n14: f64 = ((l.f2232 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq64_e1026, eq64_e1026_d_n2, (l.f2235 * ddt_scale), (l.f2236 * ddt_scale), eq64_e1026_d_n14, (l.f2233 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e1028;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(14),
            multiplicity * (eq64_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq64_e1028_d_n2), multiplicity * (eq64_e1028_d_n4), multiplicity * (eq64_e1028_d_n7), multiplicity * (eq64_e1028_d_n14), multiplicity * (eq64_e1028_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq65_e1039, eq65_e1039_d_n2, eq65_e1039_d_n4, eq65_e1039_d_n7, eq65_e1039_d_n14, eq65_e1039_d_n15,) = {
    if (l.f1dcb == 0.0) {
        let eq65_e1032: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 46, l.f21ef);let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));let eq65_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 47, eq65_e1035);let eq65_e1037: f64 = (eq65_e1032 + eq65_e1036);let eq65_e1037_d_n2: f64 = ((l.f21f2 * ddt_scale) + (p.p355 * ddt_scale));let eq65_e1037_d_n15: f64 = ((l.f21f1 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq65_e1037, eq65_e1037_d_n2, (l.f21f3 * ddt_scale), (l.f21f4 * ddt_scale), (l.f21f0 * ddt_scale), eq65_e1037_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1039;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(15),
            multiplicity * (eq65_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq65_e1039_d_n2), multiplicity * (eq65_e1039_d_n4), multiplicity * (eq65_e1039_d_n7), multiplicity * (eq65_e1039_d_n14), multiplicity * (eq65_e1039_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq66_e1050, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n7, eq66_e1050_d_n14, eq66_e1050_d_n15,) = {
    if (l.f1dcb == 0.0) {
        let eq66_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 48, l.f21a6);let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));let eq66_e1047: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 49, eq66_e1046);let eq66_e1048: f64 = (eq66_e1043 + eq66_e1047);let eq66_e1048_d_n7: f64 = ((l.f21ac * ddt_scale) + (p.p355 * ddt_scale));let eq66_e1048_d_n14: f64 = ((l.f21a7 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq66_e1048, (l.f21a9 * ddt_scale), (l.f21aa * ddt_scale), (l.f21ab * ddt_scale), eq66_e1048_d_n7, eq66_e1048_d_n14, (l.f21a8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1050;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(14),
            multiplicity * (eq66_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * (eq66_e1050_d_n2), multiplicity * (eq66_e1050_d_n3), multiplicity * (eq66_e1050_d_n4), multiplicity * (eq66_e1050_d_n7), multiplicity * (eq66_e1050_d_n14), multiplicity * (eq66_e1050_d_n15)],
            [],
            [],
            1.0,
        );let eq69_e1062: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 50, l.f2166);let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));let eq69_e1066: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 51, eq69_e1065);let eq69_e1067: f64 = (eq69_e1062 + eq69_e1066);let eq69_e1067_d_n3: f64 = ((l.f216a * ddt_scale) + (p.p355 * ddt_scale));let eq69_e1067_d_n14: f64 = ((l.f2167 * ddt_scale) + ((-p.p355) * ddt_scale));let eq69_value: f64 = eq69_e1067;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(14),
            multiplicity * (eq69_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * ((l.f2169 * ddt_scale)), multiplicity * (eq69_e1067_d_n3), multiplicity * ((l.f216b * ddt_scale)), multiplicity * ((l.f216c * ddt_scale)), multiplicity * (eq69_e1067_d_n14), multiplicity * ((l.f2168 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq70_e1075, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n7, eq70_e1075_d_n14,) = {
    if (l.f1dcd != 0.0) {
        let eq70_e1072: f64 = (l.f1d87 * (nv14 - nv5));let eq70_e1073: f64 = (l.f20ce + eq70_e1072);let eq70_e1073_d_n5: f64 = (l.f20d3 + (-l.f1d87));let eq70_e1073_d_n14: f64 = (l.f20cf + l.f1d87);
        (eq70_e1073, l.f20d0, l.f20d1, l.f20d2, eq70_e1073_d_n5, l.f20d4, eq70_e1073_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1075;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(14),
            Some(5),
            multiplicity * (eq70_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * (eq70_e1075_d_n2), multiplicity * (eq70_e1075_d_n3), multiplicity * (eq70_e1075_d_n4), multiplicity * (eq70_e1075_d_n5), multiplicity * (eq70_e1075_d_n7), multiplicity * (eq70_e1075_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq71_e1080,) = {
    if (l.f1dcd == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1080;
        stamper.stamp_potential_const_local(
            21,
            eq71_value,
        );
        let (eq72_e1090, eq72_e1090_d_n2, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n7, eq72_e1090_d_n14,) = {
    if (l.f1e15 != 0.0) {
        let eq72_e1083: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 52, l.f222a);let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));let eq72_e1087: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 53, eq72_e1086);let eq72_e1088: f64 = (eq72_e1083 + eq72_e1087);let eq72_e1088_d_n5: f64 = ((l.f222e * ddt_scale) + ((-p.p355) * ddt_scale));let eq72_e1088_d_n7: f64 = ((l.f222f * ddt_scale) + (p.p355 * ddt_scale));
        (eq72_e1088, (l.f222c * ddt_scale), (l.f222d * ddt_scale), eq72_e1088_d_n5, eq72_e1088_d_n7, (l.f222b * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1090;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq72_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq72_e1090_d_n2), multiplicity * (eq72_e1090_d_n4), multiplicity * (eq72_e1090_d_n5), multiplicity * (eq72_e1090_d_n7), multiplicity * (eq72_e1090_d_n14)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv14 = ctx.node_voltage(nodes[14]);
        let (eq73_e1100, eq73_e1100_d_n2, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n7, eq73_e1100_d_n14,) = {
    if (l.f1e15 != 0.0) {
        let eq73_e1093: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 54, l.f21e8);let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));let eq73_e1097: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 55, eq73_e1096);let eq73_e1098: f64 = (eq73_e1093 + eq73_e1097);let eq73_e1098_d_n7: f64 = ((l.f21ed * ddt_scale) + (p.p355 * ddt_scale));let eq73_e1098_d_n14: f64 = ((l.f21e9 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq73_e1098, (l.f21ea * ddt_scale), (l.f21eb * ddt_scale), (l.f21ec * ddt_scale), eq73_e1098_d_n7, eq73_e1098_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1100;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(14),
            multiplicity * (eq73_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq73_e1100_d_n2), multiplicity * (eq73_e1100_d_n4), multiplicity * (eq73_e1100_d_n5), multiplicity * (eq73_e1100_d_n7), multiplicity * (eq73_e1100_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq74_e1110, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n7, eq74_e1110_d_n14,) = {
    if (l.f1e15 != 0.0) {
        let eq74_e1103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 56, l.f219e);let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));let eq74_e1107: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 57, eq74_e1106);let eq74_e1108: f64 = (eq74_e1103 + eq74_e1107);let eq74_e1108_d_n2: f64 = ((l.f21a0 * ddt_scale) + (p.p355 * ddt_scale));let eq74_e1108_d_n5: f64 = ((l.f21a3 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq74_e1108, eq74_e1108_d_n2, (l.f21a1 * ddt_scale), (l.f21a2 * ddt_scale), eq74_e1108_d_n5, (l.f21a4 * ddt_scale), (l.f219f * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1110;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(5),
            multiplicity * (eq74_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * (eq74_e1110_d_n2), multiplicity * (eq74_e1110_d_n3), multiplicity * (eq74_e1110_d_n4), multiplicity * (eq74_e1110_d_n5), multiplicity * (eq74_e1110_d_n7), multiplicity * (eq74_e1110_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq76_e1124, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n7, eq76_e1124_d_n9, eq76_e1124_d_n14,) = {
    if (l.f1e15 != 0.0) {
        let eq76_e1117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 58, l.f22a4);let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));let eq76_e1121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 59, eq76_e1120);let eq76_e1122: f64 = (eq76_e1117 + eq76_e1121);let eq76_e1122_d_n7: f64 = ((l.f22aa * ddt_scale) + (p.p355 * ddt_scale));
        (eq76_e1122, (l.f22a6 * ddt_scale), (l.f22a7 * ddt_scale), (l.f22a8 * ddt_scale), (l.f22a9 * ddt_scale), eq76_e1122_d_n7, ((-p.p355) * ddt_scale), (l.f22a5 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1124;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq76_value),
            [2, 3, 4, 5, 7, 9, 14],
            [multiplicity * (eq76_e1124_d_n2), multiplicity * (eq76_e1124_d_n3), multiplicity * (eq76_e1124_d_n4), multiplicity * (eq76_e1124_d_n5), multiplicity * (eq76_e1124_d_n7), multiplicity * (eq76_e1124_d_n9), multiplicity * (eq76_e1124_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq77_e1135, eq77_e1135_d_n2, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n7, eq77_e1135_d_n14,) = {
    if (l.f1e15 == 0.0) {
        let eq77_e1128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 60, l.f222a);let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));let eq77_e1132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 61, eq77_e1131);let eq77_e1133: f64 = (eq77_e1128 + eq77_e1132);let eq77_e1133_d_n2: f64 = ((l.f222c * ddt_scale) + (p.p355 * ddt_scale));let eq77_e1133_d_n5: f64 = ((l.f222e * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq77_e1133, eq77_e1133_d_n2, (l.f222d * ddt_scale), eq77_e1133_d_n5, (l.f222f * ddt_scale), (l.f222b * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1135;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(5),
            multiplicity * (eq77_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq77_e1135_d_n2), multiplicity * (eq77_e1135_d_n4), multiplicity * (eq77_e1135_d_n5), multiplicity * (eq77_e1135_d_n7), multiplicity * (eq77_e1135_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq78_e1146, eq78_e1146_d_n2, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n7, eq78_e1146_d_n14,) = {
    if (l.f1e15 == 0.0) {
        let eq78_e1139: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 62, l.f21e8);let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));let eq78_e1143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 63, eq78_e1142);let eq78_e1144: f64 = (eq78_e1139 + eq78_e1143);let eq78_e1144_d_n2: f64 = ((l.f21ea * ddt_scale) + (p.p355 * ddt_scale));let eq78_e1144_d_n14: f64 = ((l.f21e9 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq78_e1144, eq78_e1144_d_n2, (l.f21eb * ddt_scale), (l.f21ec * ddt_scale), (l.f21ed * ddt_scale), eq78_e1144_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1146;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(14),
            multiplicity * (eq78_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq78_e1146_d_n2), multiplicity * (eq78_e1146_d_n4), multiplicity * (eq78_e1146_d_n5), multiplicity * (eq78_e1146_d_n7), multiplicity * (eq78_e1146_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq79_e1157, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n7, eq79_e1157_d_n14,) = {
    if (l.f1e15 == 0.0) {
        let eq79_e1150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 64, l.f219e);let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));let eq79_e1154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 65, eq79_e1153);let eq79_e1155: f64 = (eq79_e1150 + eq79_e1154);let eq79_e1155_d_n5: f64 = ((l.f21a3 * ddt_scale) + ((-p.p355) * ddt_scale));let eq79_e1155_d_n7: f64 = ((l.f21a4 * ddt_scale) + (p.p355 * ddt_scale));
        (eq79_e1155, (l.f21a0 * ddt_scale), (l.f21a1 * ddt_scale), (l.f21a2 * ddt_scale), eq79_e1155_d_n5, eq79_e1155_d_n7, (l.f219f * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1157;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq79_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * (eq79_e1157_d_n2), multiplicity * (eq79_e1157_d_n3), multiplicity * (eq79_e1157_d_n4), multiplicity * (eq79_e1157_d_n5), multiplicity * (eq79_e1157_d_n7), multiplicity * (eq79_e1157_d_n14)],
            [],
            [],
            1.0,
        );let eq82_e1169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 66, l.f215e);let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));let eq82_e1173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 67, eq82_e1172);let eq82_e1174: f64 = (eq82_e1169 + eq82_e1173);let eq82_e1174_d_n3: f64 = ((l.f2161 * ddt_scale) + (p.p355 * ddt_scale));let eq82_e1174_d_n5: f64 = ((l.f2163 * ddt_scale) + ((-p.p355) * ddt_scale));let eq82_value: f64 = eq82_e1174;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(5),
            multiplicity * (eq82_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * ((l.f2160 * ddt_scale)), multiplicity * (eq82_e1174_d_n3), multiplicity * ((l.f2162 * ddt_scale)), multiplicity * (eq82_e1174_d_n5), multiplicity * ((l.f2164 * ddt_scale)), multiplicity * ((l.f215f * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq83_e1182, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n7, eq83_e1182_d_n9, eq83_e1182_d_n10,) = {
    if (l.f1e17 != 0.0) {
        let eq83_e1179: f64 = (l.f1d87 * (nv9 - nv10));let eq83_e1180: f64 = (l.f20ea + eq83_e1179);let eq83_e1180_d_n9: f64 = (l.f20f0 + l.f1d87);let eq83_e1180_d_n10: f64 = (l.f20eb + (-l.f1d87));
        (eq83_e1180, l.f20ec, l.f20ed, l.f20ee, l.f20ef, eq83_e1180_d_n9, eq83_e1180_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1182;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(10),
            multiplicity * (eq83_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq83_e1182_d_n2), multiplicity * (eq83_e1182_d_n3), multiplicity * (eq83_e1182_d_n4), multiplicity * (eq83_e1182_d_n7), multiplicity * (eq83_e1182_d_n9), multiplicity * (eq83_e1182_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq84_e1187,) = {
    if (l.f1e17 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1187;
        stamper.stamp_potential_const_local(
            22,
            eq84_value,
        );
        let (eq85_e1197, eq85_e1197_d_n2, eq85_e1197_d_n4, eq85_e1197_d_n7, eq85_e1197_d_n9, eq85_e1197_d_n10,) = {
    if (l.f1e64 != 0.0) {
        let eq85_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 68, l.f2246);let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));let eq85_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 69, eq85_e1193);let eq85_e1195: f64 = (eq85_e1190 + eq85_e1194);let eq85_e1195_d_n7: f64 = ((l.f224a * ddt_scale) + (p.p355 * ddt_scale));let eq85_e1195_d_n10: f64 = ((l.f2247 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq85_e1195, (l.f2248 * ddt_scale), (l.f2249 * ddt_scale), eq85_e1195_d_n7, (l.f224b * ddt_scale), eq85_e1195_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_value: f64 = eq85_e1197;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq85_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq85_e1197_d_n2), multiplicity * (eq85_e1197_d_n4), multiplicity * (eq85_e1197_d_n7), multiplicity * (eq85_e1197_d_n9), multiplicity * (eq85_e1197_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq86_e1207, eq86_e1207_d_n2, eq86_e1207_d_n4, eq86_e1207_d_n7, eq86_e1207_d_n9, eq86_e1207_d_n10,) = {
    if (l.f1e64 != 0.0) {
        let eq86_e1200: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 70, l.f2204);let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));let eq86_e1204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 71, eq86_e1203);let eq86_e1205: f64 = (eq86_e1200 + eq86_e1204);let eq86_e1205_d_n7: f64 = ((l.f2208 * ddt_scale) + (p.p355 * ddt_scale));let eq86_e1205_d_n9: f64 = ((l.f2209 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq86_e1205, (l.f2206 * ddt_scale), (l.f2207 * ddt_scale), eq86_e1205_d_n7, eq86_e1205_d_n9, (l.f2205 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1207;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq86_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq86_e1207_d_n2), multiplicity * (eq86_e1207_d_n4), multiplicity * (eq86_e1207_d_n7), multiplicity * (eq86_e1207_d_n9), multiplicity * (eq86_e1207_d_n10)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq87_e1217, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n7, eq87_e1217_d_n9, eq87_e1217_d_n10,) = {
    if (l.f1e64 != 0.0) {
        let eq87_e1210: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 72, l.f21be);let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));let eq87_e1214: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 73, eq87_e1213);let eq87_e1215: f64 = (eq87_e1210 + eq87_e1214);let eq87_e1215_d_n2: f64 = ((l.f21c0 * ddt_scale) + (p.p355 * ddt_scale));let eq87_e1215_d_n10: f64 = ((l.f21bf * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq87_e1215, eq87_e1215_d_n2, (l.f21c1 * ddt_scale), (l.f21c2 * ddt_scale), (l.f21c3 * ddt_scale), (l.f21c4 * ddt_scale), eq87_e1215_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_value: f64 = eq87_e1217;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(10),
            multiplicity * (eq87_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq87_e1217_d_n2), multiplicity * (eq87_e1217_d_n3), multiplicity * (eq87_e1217_d_n4), multiplicity * (eq87_e1217_d_n7), multiplicity * (eq87_e1217_d_n9), multiplicity * (eq87_e1217_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq89_e1231, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n7, eq89_e1231_d_n9, eq89_e1231_d_n10,) = {
    if (l.f1e64 != 0.0) {
        let eq89_e1224: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 74, l.f22c4);let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));let eq89_e1228: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 75, eq89_e1227);let eq89_e1229: f64 = (eq89_e1224 + eq89_e1228);let eq89_e1229_d_n7: f64 = ((l.f22c9 * ddt_scale) + (p.p355 * ddt_scale));let eq89_e1229_d_n9: f64 = ((l.f22ca * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq89_e1229, (l.f22c6 * ddt_scale), (l.f22c7 * ddt_scale), (l.f22c8 * ddt_scale), eq89_e1229_d_n7, eq89_e1229_d_n9, (l.f22c5 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1231;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq89_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq89_e1231_d_n2), multiplicity * (eq89_e1231_d_n3), multiplicity * (eq89_e1231_d_n4), multiplicity * (eq89_e1231_d_n7), multiplicity * (eq89_e1231_d_n9), multiplicity * (eq89_e1231_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq90_e1242, eq90_e1242_d_n2, eq90_e1242_d_n4, eq90_e1242_d_n7, eq90_e1242_d_n9, eq90_e1242_d_n10,) = {
    if (l.f1e64 == 0.0) {
        let eq90_e1235: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 76, l.f2246);let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));let eq90_e1239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 77, eq90_e1238);let eq90_e1240: f64 = (eq90_e1235 + eq90_e1239);let eq90_e1240_d_n2: f64 = ((l.f2248 * ddt_scale) + (p.p355 * ddt_scale));let eq90_e1240_d_n10: f64 = ((l.f2247 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq90_e1240, eq90_e1240_d_n2, (l.f2249 * ddt_scale), (l.f224a * ddt_scale), (l.f224b * ddt_scale), eq90_e1240_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_value: f64 = eq90_e1242;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(10),
            multiplicity * (eq90_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq90_e1242_d_n2), multiplicity * (eq90_e1242_d_n4), multiplicity * (eq90_e1242_d_n7), multiplicity * (eq90_e1242_d_n9), multiplicity * (eq90_e1242_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq91_e1253, eq91_e1253_d_n2, eq91_e1253_d_n4, eq91_e1253_d_n7, eq91_e1253_d_n9, eq91_e1253_d_n10,) = {
    if (l.f1e64 == 0.0) {
        let eq91_e1246: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 78, l.f2204);let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));let eq91_e1250: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 79, eq91_e1249);let eq91_e1251: f64 = (eq91_e1246 + eq91_e1250);let eq91_e1251_d_n2: f64 = ((l.f2206 * ddt_scale) + (p.p355 * ddt_scale));let eq91_e1251_d_n9: f64 = ((l.f2209 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq91_e1251, eq91_e1251_d_n2, (l.f2207 * ddt_scale), (l.f2208 * ddt_scale), eq91_e1251_d_n9, (l.f2205 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_value: f64 = eq91_e1253;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(9),
            multiplicity * (eq91_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq91_e1253_d_n2), multiplicity * (eq91_e1253_d_n4), multiplicity * (eq91_e1253_d_n7), multiplicity * (eq91_e1253_d_n9), multiplicity * (eq91_e1253_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq92_e1264, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n7, eq92_e1264_d_n9, eq92_e1264_d_n10,) = {
    if (l.f1e64 == 0.0) {
        let eq92_e1257: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 80, l.f21be);let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));let eq92_e1261: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 81, eq92_e1260);let eq92_e1262: f64 = (eq92_e1257 + eq92_e1261);let eq92_e1262_d_n7: f64 = ((l.f21c3 * ddt_scale) + (p.p355 * ddt_scale));let eq92_e1262_d_n10: f64 = ((l.f21bf * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq92_e1262, (l.f21c0 * ddt_scale), (l.f21c1 * ddt_scale), (l.f21c2 * ddt_scale), eq92_e1262_d_n7, (l.f21c4 * ddt_scale), eq92_e1262_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e1264;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq92_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq92_e1264_d_n2), multiplicity * (eq92_e1264_d_n3), multiplicity * (eq92_e1264_d_n4), multiplicity * (eq92_e1264_d_n7), multiplicity * (eq92_e1264_d_n9), multiplicity * (eq92_e1264_d_n10)],
            [],
            [],
            1.0,
        );let eq95_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 82, l.f217e);let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));let eq95_e1280: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 83, eq95_e1279);let eq95_e1281: f64 = (eq95_e1276 + eq95_e1280);let eq95_e1281_d_n3: f64 = ((l.f2181 * ddt_scale) + (p.p355 * ddt_scale));let eq95_e1281_d_n10: f64 = ((l.f217f * ddt_scale) + ((-p.p355) * ddt_scale));let eq95_value: f64 = eq95_e1281;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(10),
            multiplicity * (eq95_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * ((l.f2180 * ddt_scale)), multiplicity * (eq95_e1281_d_n3), multiplicity * ((l.f2182 * ddt_scale)), multiplicity * ((l.f2183 * ddt_scale)), multiplicity * ((l.f2184 * ddt_scale)), multiplicity * (eq95_e1281_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq96_e1289, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n7, eq96_e1289_d_n10, eq96_e1289_d_n11,) = {
    if (l.f1e66 != 0.0) {
        let eq96_e1286: f64 = (l.f1d87 * (nv10 - nv11));let eq96_e1287: f64 = (l.f20f1 + eq96_e1286);let eq96_e1287_d_n10: f64 = (l.f20f2 + l.f1d87);let eq96_e1287_d_n11: f64 = (l.f20f3 + (-l.f1d87));
        (eq96_e1287, l.f20f4, l.f20f5, l.f20f6, l.f20f7, eq96_e1287_d_n10, eq96_e1287_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1289;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(11),
            multiplicity * (eq96_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * (eq96_e1289_d_n2), multiplicity * (eq96_e1289_d_n3), multiplicity * (eq96_e1289_d_n4), multiplicity * (eq96_e1289_d_n7), multiplicity * (eq96_e1289_d_n10), multiplicity * (eq96_e1289_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq97_e1294,) = {
    if (l.f1e66 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1294;
        stamper.stamp_potential_const_local(
            23,
            eq97_value,
        );
        let (eq98_e1304, eq98_e1304_d_n2, eq98_e1304_d_n4, eq98_e1304_d_n7, eq98_e1304_d_n10, eq98_e1304_d_n11,) = {
    if (l.f1eb0 != 0.0) {
        let eq98_e1297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 84, l.f224d);let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));let eq98_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 85, eq98_e1300);let eq98_e1302: f64 = (eq98_e1297 + eq98_e1301);let eq98_e1302_d_n7: f64 = ((l.f2252 * ddt_scale) + (p.p355 * ddt_scale));let eq98_e1302_d_n11: f64 = ((l.f224f * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq98_e1302, (l.f2250 * ddt_scale), (l.f2251 * ddt_scale), eq98_e1302_d_n7, (l.f224e * ddt_scale), eq98_e1302_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_value: f64 = eq98_e1304;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq98_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq98_e1304_d_n2), multiplicity * (eq98_e1304_d_n4), multiplicity * (eq98_e1304_d_n7), multiplicity * (eq98_e1304_d_n10), multiplicity * (eq98_e1304_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq99_e1314, eq99_e1314_d_n2, eq99_e1314_d_n4, eq99_e1314_d_n7, eq99_e1314_d_n10, eq99_e1314_d_n11,) = {
    if (l.f1eb0 != 0.0) {
        let eq99_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 86, l.f220b);let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));let eq99_e1311: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 87, eq99_e1310);let eq99_e1312: f64 = (eq99_e1307 + eq99_e1311);let eq99_e1312_d_n7: f64 = ((l.f2210 * ddt_scale) + (p.p355 * ddt_scale));let eq99_e1312_d_n10: f64 = ((l.f220c * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq99_e1312, (l.f220e * ddt_scale), (l.f220f * ddt_scale), eq99_e1312_d_n7, eq99_e1312_d_n10, (l.f220d * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_value: f64 = eq99_e1314;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq99_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq99_e1314_d_n2), multiplicity * (eq99_e1314_d_n4), multiplicity * (eq99_e1314_d_n7), multiplicity * (eq99_e1314_d_n10), multiplicity * (eq99_e1314_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq100_e1324, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n7, eq100_e1324_d_n10, eq100_e1324_d_n11,) = {
    if (l.f1eb0 != 0.0) {
        let eq100_e1317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 88, l.f21c6);let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));let eq100_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 89, eq100_e1320);let eq100_e1322: f64 = (eq100_e1317 + eq100_e1321);let eq100_e1322_d_n2: f64 = ((l.f21c9 * ddt_scale) + (p.p355 * ddt_scale));let eq100_e1322_d_n11: f64 = ((l.f21c8 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq100_e1322, eq100_e1322_d_n2, (l.f21ca * ddt_scale), (l.f21cb * ddt_scale), (l.f21cc * ddt_scale), (l.f21c7 * ddt_scale), eq100_e1322_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1324;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(11),
            multiplicity * (eq100_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * (eq100_e1324_d_n2), multiplicity * (eq100_e1324_d_n3), multiplicity * (eq100_e1324_d_n4), multiplicity * (eq100_e1324_d_n7), multiplicity * (eq100_e1324_d_n10), multiplicity * (eq100_e1324_d_n11)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq102_e1338, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n7, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11,) = {
    if (l.f1eb0 != 0.0) {
        let eq102_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 90, l.f22cc);let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));let eq102_e1335: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 91, eq102_e1334);let eq102_e1336: f64 = (eq102_e1331 + eq102_e1335);let eq102_e1336_d_n7: f64 = ((l.f22d2 * ddt_scale) + (p.p355 * ddt_scale));
        (eq102_e1336, (l.f22cf * ddt_scale), (l.f22d0 * ddt_scale), (l.f22d1 * ddt_scale), eq102_e1336_d_n7, ((-p.p355) * ddt_scale), (l.f22cd * ddt_scale), (l.f22ce * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1338;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq102_value),
            [2, 3, 4, 7, 9, 10, 11],
            [multiplicity * (eq102_e1338_d_n2), multiplicity * (eq102_e1338_d_n3), multiplicity * (eq102_e1338_d_n4), multiplicity * (eq102_e1338_d_n7), multiplicity * (eq102_e1338_d_n9), multiplicity * (eq102_e1338_d_n10), multiplicity * (eq102_e1338_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq103_e1349, eq103_e1349_d_n2, eq103_e1349_d_n4, eq103_e1349_d_n7, eq103_e1349_d_n10, eq103_e1349_d_n11,) = {
    if (l.f1eb0 == 0.0) {
        let eq103_e1342: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 92, l.f224d);let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));let eq103_e1346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 93, eq103_e1345);let eq103_e1347: f64 = (eq103_e1342 + eq103_e1346);let eq103_e1347_d_n2: f64 = ((l.f2250 * ddt_scale) + (p.p355 * ddt_scale));let eq103_e1347_d_n11: f64 = ((l.f224f * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq103_e1347, eq103_e1347_d_n2, (l.f2251 * ddt_scale), (l.f2252 * ddt_scale), (l.f224e * ddt_scale), eq103_e1347_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1349;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(11),
            multiplicity * (eq103_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq103_e1349_d_n2), multiplicity * (eq103_e1349_d_n4), multiplicity * (eq103_e1349_d_n7), multiplicity * (eq103_e1349_d_n10), multiplicity * (eq103_e1349_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq104_e1360, eq104_e1360_d_n2, eq104_e1360_d_n4, eq104_e1360_d_n7, eq104_e1360_d_n10, eq104_e1360_d_n11,) = {
    if (l.f1eb0 == 0.0) {
        let eq104_e1353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 94, l.f220b);let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));let eq104_e1357: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 95, eq104_e1356);let eq104_e1358: f64 = (eq104_e1353 + eq104_e1357);let eq104_e1358_d_n2: f64 = ((l.f220e * ddt_scale) + (p.p355 * ddt_scale));let eq104_e1358_d_n10: f64 = ((l.f220c * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq104_e1358, eq104_e1358_d_n2, (l.f220f * ddt_scale), (l.f2210 * ddt_scale), eq104_e1358_d_n10, (l.f220d * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_value: f64 = eq104_e1360;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(10),
            multiplicity * (eq104_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq104_e1360_d_n2), multiplicity * (eq104_e1360_d_n4), multiplicity * (eq104_e1360_d_n7), multiplicity * (eq104_e1360_d_n10), multiplicity * (eq104_e1360_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq105_e1371, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n7, eq105_e1371_d_n10, eq105_e1371_d_n11,) = {
    if (l.f1eb0 == 0.0) {
        let eq105_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 96, l.f21c6);let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));let eq105_e1368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 97, eq105_e1367);let eq105_e1369: f64 = (eq105_e1364 + eq105_e1368);let eq105_e1369_d_n7: f64 = ((l.f21cc * ddt_scale) + (p.p355 * ddt_scale));let eq105_e1369_d_n11: f64 = ((l.f21c8 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq105_e1369, (l.f21c9 * ddt_scale), (l.f21ca * ddt_scale), (l.f21cb * ddt_scale), eq105_e1369_d_n7, (l.f21c7 * ddt_scale), eq105_e1369_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq105_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * (eq105_e1371_d_n2), multiplicity * (eq105_e1371_d_n3), multiplicity * (eq105_e1371_d_n4), multiplicity * (eq105_e1371_d_n7), multiplicity * (eq105_e1371_d_n10), multiplicity * (eq105_e1371_d_n11)],
            [],
            [],
            1.0,
        );let eq108_e1383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 98, l.f2186);let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));let eq108_e1387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 99, eq108_e1386);let eq108_e1388: f64 = (eq108_e1383 + eq108_e1387);let eq108_e1388_d_n3: f64 = ((l.f218a * ddt_scale) + (p.p355 * ddt_scale));let eq108_e1388_d_n11: f64 = ((l.f2188 * ddt_scale) + ((-p.p355) * ddt_scale));let eq108_value: f64 = eq108_e1388;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(11),
            multiplicity * (eq108_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * ((l.f2189 * ddt_scale)), multiplicity * (eq108_e1388_d_n3), multiplicity * ((l.f218b * ddt_scale)), multiplicity * ((l.f218c * ddt_scale)), multiplicity * ((l.f2187 * ddt_scale)), multiplicity * (eq108_e1388_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq109_e1396, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n7, eq109_e1396_d_n11, eq109_e1396_d_n12,) = {
    if (l.f1eb4 != 0.0) {
        let eq109_e1393: f64 = (l.f1d87 * (nv11 - nv12));let eq109_e1394: f64 = (l.f20f8 + eq109_e1393);let eq109_e1394_d_n11: f64 = (l.f20f9 + l.f1d87);let eq109_e1394_d_n12: f64 = (l.f20fa + (-l.f1d87));
        (eq109_e1394, l.f20fb, l.f20fc, l.f20fd, l.f20fe, eq109_e1394_d_n11, eq109_e1394_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e1396;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq109_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * (eq109_e1396_d_n2), multiplicity * (eq109_e1396_d_n3), multiplicity * (eq109_e1396_d_n4), multiplicity * (eq109_e1396_d_n7), multiplicity * (eq109_e1396_d_n11), multiplicity * (eq109_e1396_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq110_e1401,) = {
    if (l.f1eb4 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq110_value: f64 = eq110_e1401;
        stamper.stamp_potential_const_local(
            24,
            eq110_value,
        );
        let (eq111_e1411, eq111_e1411_d_n2, eq111_e1411_d_n4, eq111_e1411_d_n7, eq111_e1411_d_n11, eq111_e1411_d_n12,) = {
    if (l.f1efc != 0.0) {
        let eq111_e1404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 100, l.f2254);let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));let eq111_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 101, eq111_e1407);let eq111_e1409: f64 = (eq111_e1404 + eq111_e1408);let eq111_e1409_d_n7: f64 = ((l.f2259 * ddt_scale) + (p.p355 * ddt_scale));let eq111_e1409_d_n12: f64 = ((l.f2256 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq111_e1409, (l.f2257 * ddt_scale), (l.f2258 * ddt_scale), eq111_e1409_d_n7, (l.f2255 * ddt_scale), eq111_e1409_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1411;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq111_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq111_e1411_d_n2), multiplicity * (eq111_e1411_d_n4), multiplicity * (eq111_e1411_d_n7), multiplicity * (eq111_e1411_d_n11), multiplicity * (eq111_e1411_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq112_e1421, eq112_e1421_d_n2, eq112_e1421_d_n4, eq112_e1421_d_n7, eq112_e1421_d_n11, eq112_e1421_d_n12,) = {
    if (l.f1efc != 0.0) {
        let eq112_e1414: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 102, l.f2212);let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));let eq112_e1418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 103, eq112_e1417);let eq112_e1419: f64 = (eq112_e1414 + eq112_e1418);let eq112_e1419_d_n7: f64 = ((l.f2217 * ddt_scale) + (p.p355 * ddt_scale));let eq112_e1419_d_n11: f64 = ((l.f2213 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq112_e1419, (l.f2215 * ddt_scale), (l.f2216 * ddt_scale), eq112_e1419_d_n7, eq112_e1419_d_n11, (l.f2214 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1421;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq112_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq112_e1421_d_n2), multiplicity * (eq112_e1421_d_n4), multiplicity * (eq112_e1421_d_n7), multiplicity * (eq112_e1421_d_n11), multiplicity * (eq112_e1421_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq113_e1431, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n7, eq113_e1431_d_n11, eq113_e1431_d_n12,) = {
    if (l.f1efc != 0.0) {
        let eq113_e1424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 104, l.f21ce);let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));let eq113_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 105, eq113_e1427);let eq113_e1429: f64 = (eq113_e1424 + eq113_e1428);let eq113_e1429_d_n2: f64 = ((l.f21d1 * ddt_scale) + (p.p355 * ddt_scale));let eq113_e1429_d_n12: f64 = ((l.f21d0 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq113_e1429, eq113_e1429_d_n2, (l.f21d2 * ddt_scale), (l.f21d3 * ddt_scale), (l.f21d4 * ddt_scale), (l.f21cf * ddt_scale), eq113_e1429_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1431;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq113_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * (eq113_e1431_d_n2), multiplicity * (eq113_e1431_d_n3), multiplicity * (eq113_e1431_d_n4), multiplicity * (eq113_e1431_d_n7), multiplicity * (eq113_e1431_d_n11), multiplicity * (eq113_e1431_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq115_e1445, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n7, eq115_e1445_d_n9, eq115_e1445_d_n11, eq115_e1445_d_n12,) = {
    if (l.f1efc != 0.0) {
        let eq115_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 106, l.f22d4);let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));let eq115_e1442: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 107, eq115_e1441);let eq115_e1443: f64 = (eq115_e1438 + eq115_e1442);let eq115_e1443_d_n7: f64 = ((l.f22da * ddt_scale) + (p.p355 * ddt_scale));
        (eq115_e1443, (l.f22d7 * ddt_scale), (l.f22d8 * ddt_scale), (l.f22d9 * ddt_scale), eq115_e1443_d_n7, ((-p.p355) * ddt_scale), (l.f22d5 * ddt_scale), (l.f22d6 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_value: f64 = eq115_e1445;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq115_value),
            [2, 3, 4, 7, 9, 11, 12],
            [multiplicity * (eq115_e1445_d_n2), multiplicity * (eq115_e1445_d_n3), multiplicity * (eq115_e1445_d_n4), multiplicity * (eq115_e1445_d_n7), multiplicity * (eq115_e1445_d_n9), multiplicity * (eq115_e1445_d_n11), multiplicity * (eq115_e1445_d_n12)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq116_e1456, eq116_e1456_d_n2, eq116_e1456_d_n4, eq116_e1456_d_n7, eq116_e1456_d_n11, eq116_e1456_d_n12,) = {
    if (l.f1efc == 0.0) {
        let eq116_e1449: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 108, l.f2254);let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));let eq116_e1453: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 109, eq116_e1452);let eq116_e1454: f64 = (eq116_e1449 + eq116_e1453);let eq116_e1454_d_n2: f64 = ((l.f2257 * ddt_scale) + (p.p355 * ddt_scale));let eq116_e1454_d_n12: f64 = ((l.f2256 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq116_e1454, eq116_e1454_d_n2, (l.f2258 * ddt_scale), (l.f2259 * ddt_scale), (l.f2255 * ddt_scale), eq116_e1454_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_value: f64 = eq116_e1456;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq116_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq116_e1456_d_n2), multiplicity * (eq116_e1456_d_n4), multiplicity * (eq116_e1456_d_n7), multiplicity * (eq116_e1456_d_n11), multiplicity * (eq116_e1456_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq117_e1467, eq117_e1467_d_n2, eq117_e1467_d_n4, eq117_e1467_d_n7, eq117_e1467_d_n11, eq117_e1467_d_n12,) = {
    if (l.f1efc == 0.0) {
        let eq117_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 110, l.f2212);let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));let eq117_e1464: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 111, eq117_e1463);let eq117_e1465: f64 = (eq117_e1460 + eq117_e1464);let eq117_e1465_d_n2: f64 = ((l.f2215 * ddt_scale) + (p.p355 * ddt_scale));let eq117_e1465_d_n11: f64 = ((l.f2213 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq117_e1465, eq117_e1465_d_n2, (l.f2216 * ddt_scale), (l.f2217 * ddt_scale), eq117_e1465_d_n11, (l.f2214 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_value: f64 = eq117_e1467;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(11),
            multiplicity * (eq117_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq117_e1467_d_n2), multiplicity * (eq117_e1467_d_n4), multiplicity * (eq117_e1467_d_n7), multiplicity * (eq117_e1467_d_n11), multiplicity * (eq117_e1467_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq118_e1478, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n7, eq118_e1478_d_n11, eq118_e1478_d_n12,) = {
    if (l.f1efc == 0.0) {
        let eq118_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 112, l.f21ce);let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));let eq118_e1475: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 113, eq118_e1474);let eq118_e1476: f64 = (eq118_e1471 + eq118_e1475);let eq118_e1476_d_n7: f64 = ((l.f21d4 * ddt_scale) + (p.p355 * ddt_scale));let eq118_e1476_d_n12: f64 = ((l.f21d0 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq118_e1476, (l.f21d1 * ddt_scale), (l.f21d2 * ddt_scale), (l.f21d3 * ddt_scale), eq118_e1476_d_n7, (l.f21cf * ddt_scale), eq118_e1476_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_value: f64 = eq118_e1478;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq118_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * (eq118_e1478_d_n2), multiplicity * (eq118_e1478_d_n3), multiplicity * (eq118_e1478_d_n4), multiplicity * (eq118_e1478_d_n7), multiplicity * (eq118_e1478_d_n11), multiplicity * (eq118_e1478_d_n12)],
            [],
            [],
            1.0,
        );let eq121_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 114, l.f218e);let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));let eq121_e1494: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 115, eq121_e1493);let eq121_e1495: f64 = (eq121_e1490 + eq121_e1494);let eq121_e1495_d_n3: f64 = ((l.f2192 * ddt_scale) + (p.p355 * ddt_scale));let eq121_e1495_d_n12: f64 = ((l.f2190 * ddt_scale) + ((-p.p355) * ddt_scale));let eq121_value: f64 = eq121_e1495;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(12),
            multiplicity * (eq121_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * ((l.f2191 * ddt_scale)), multiplicity * (eq121_e1495_d_n3), multiplicity * ((l.f2193 * ddt_scale)), multiplicity * ((l.f2194 * ddt_scale)), multiplicity * ((l.f218f * ddt_scale)), multiplicity * (eq121_e1495_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq122_e1503, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n7, eq122_e1503_d_n12, eq122_e1503_d_n13,) = {
    if (l.f1efe != 0.0) {
        let eq122_e1500: f64 = (l.f1d87 * (nv12 - nv13));let eq122_e1501: f64 = (l.f20ff + eq122_e1500);let eq122_e1501_d_n12: f64 = (l.f2100 + l.f1d87);let eq122_e1501_d_n13: f64 = (l.f2101 + (-l.f1d87));
        (eq122_e1501, l.f2102, l.f2103, l.f2104, l.f2105, eq122_e1501_d_n12, eq122_e1501_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1503;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq122_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq122_e1503_d_n2), multiplicity * (eq122_e1503_d_n3), multiplicity * (eq122_e1503_d_n4), multiplicity * (eq122_e1503_d_n7), multiplicity * (eq122_e1503_d_n12), multiplicity * (eq122_e1503_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq123_e1508,) = {
    if (l.f1efe == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq123_value: f64 = eq123_e1508;
        stamper.stamp_potential_const_local(
            25,
            eq123_value,
        );
        let (eq124_e1518, eq124_e1518_d_n2, eq124_e1518_d_n4, eq124_e1518_d_n7, eq124_e1518_d_n12, eq124_e1518_d_n13,) = {
    if (l.f1f4c != 0.0) {
        let eq124_e1511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 116, l.f225b);let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));let eq124_e1515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 117, eq124_e1514);let eq124_e1516: f64 = (eq124_e1511 + eq124_e1515);let eq124_e1516_d_n7: f64 = ((l.f2260 * ddt_scale) + (p.p355 * ddt_scale));let eq124_e1516_d_n13: f64 = ((l.f225d * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq124_e1516, (l.f225e * ddt_scale), (l.f225f * ddt_scale), eq124_e1516_d_n7, (l.f225c * ddt_scale), eq124_e1516_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1518;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(13),
            multiplicity * (eq124_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq124_e1518_d_n2), multiplicity * (eq124_e1518_d_n4), multiplicity * (eq124_e1518_d_n7), multiplicity * (eq124_e1518_d_n12), multiplicity * (eq124_e1518_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq125_e1528, eq125_e1528_d_n2, eq125_e1528_d_n4, eq125_e1528_d_n7, eq125_e1528_d_n12, eq125_e1528_d_n13,) = {
    if (l.f1f4c != 0.0) {
        let eq125_e1521: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 118, l.f2219);let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));let eq125_e1525: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 119, eq125_e1524);let eq125_e1526: f64 = (eq125_e1521 + eq125_e1525);let eq125_e1526_d_n7: f64 = ((l.f221e * ddt_scale) + (p.p355 * ddt_scale));let eq125_e1526_d_n12: f64 = ((l.f221a * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq125_e1526, (l.f221c * ddt_scale), (l.f221d * ddt_scale), eq125_e1526_d_n7, eq125_e1526_d_n12, (l.f221b * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1528;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq125_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq125_e1528_d_n2), multiplicity * (eq125_e1528_d_n4), multiplicity * (eq125_e1528_d_n7), multiplicity * (eq125_e1528_d_n12), multiplicity * (eq125_e1528_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq126_e1538, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n7, eq126_e1538_d_n12, eq126_e1538_d_n13,) = {
    if (l.f1f4c != 0.0) {
        let eq126_e1531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 120, l.f21d6);let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));let eq126_e1535: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 121, eq126_e1534);let eq126_e1536: f64 = (eq126_e1531 + eq126_e1535);let eq126_e1536_d_n2: f64 = ((l.f21d9 * ddt_scale) + (p.p355 * ddt_scale));let eq126_e1536_d_n13: f64 = ((l.f21d8 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq126_e1536, eq126_e1536_d_n2, (l.f21da * ddt_scale), (l.f21db * ddt_scale), (l.f21dc * ddt_scale), (l.f21d7 * ddt_scale), eq126_e1536_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1538;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(13),
            multiplicity * (eq126_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq126_e1538_d_n2), multiplicity * (eq126_e1538_d_n3), multiplicity * (eq126_e1538_d_n4), multiplicity * (eq126_e1538_d_n7), multiplicity * (eq126_e1538_d_n12), multiplicity * (eq126_e1538_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq128_e1552, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n7, eq128_e1552_d_n9, eq128_e1552_d_n12, eq128_e1552_d_n13,) = {
    if (l.f1f4c != 0.0) {
        let eq128_e1545: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 122, l.f22dc);let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));let eq128_e1549: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 123, eq128_e1548);let eq128_e1550: f64 = (eq128_e1545 + eq128_e1549);let eq128_e1550_d_n7: f64 = ((l.f22e2 * ddt_scale) + (p.p355 * ddt_scale));
        (eq128_e1550, (l.f22df * ddt_scale), (l.f22e0 * ddt_scale), (l.f22e1 * ddt_scale), eq128_e1550_d_n7, ((-p.p355) * ddt_scale), (l.f22dd * ddt_scale), (l.f22de * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1552;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq128_value),
            [2, 3, 4, 7, 9, 12, 13],
            [multiplicity * (eq128_e1552_d_n2), multiplicity * (eq128_e1552_d_n3), multiplicity * (eq128_e1552_d_n4), multiplicity * (eq128_e1552_d_n7), multiplicity * (eq128_e1552_d_n9), multiplicity * (eq128_e1552_d_n12), multiplicity * (eq128_e1552_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq129_e1563, eq129_e1563_d_n2, eq129_e1563_d_n4, eq129_e1563_d_n7, eq129_e1563_d_n12, eq129_e1563_d_n13,) = {
    if (l.f1f4c == 0.0) {
        let eq129_e1556: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 124, l.f225b);let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));let eq129_e1560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 125, eq129_e1559);let eq129_e1561: f64 = (eq129_e1556 + eq129_e1560);let eq129_e1561_d_n2: f64 = ((l.f225e * ddt_scale) + (p.p355 * ddt_scale));let eq129_e1561_d_n13: f64 = ((l.f225d * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq129_e1561, eq129_e1561_d_n2, (l.f225f * ddt_scale), (l.f2260 * ddt_scale), (l.f225c * ddt_scale), eq129_e1561_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1563;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(13),
            multiplicity * (eq129_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq129_e1563_d_n2), multiplicity * (eq129_e1563_d_n4), multiplicity * (eq129_e1563_d_n7), multiplicity * (eq129_e1563_d_n12), multiplicity * (eq129_e1563_d_n13)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv17 = ctx.node_voltage(nodes[17]);let nv18 = ctx.node_voltage(nodes[18]);let nv19 = ctx.node_voltage(nodes[19]);let nv28 = ctx.node_voltage(nodes[28]);let nv29 = ctx.node_voltage(nodes[29]);
        let (eq130_e1574, eq130_e1574_d_n2, eq130_e1574_d_n4, eq130_e1574_d_n7, eq130_e1574_d_n12, eq130_e1574_d_n13,) = {
    if (l.f1f4c == 0.0) {
        let eq130_e1567: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 126, l.f2219);let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));let eq130_e1571: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 127, eq130_e1570);let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);let eq130_e1572_d_n2: f64 = ((l.f221c * ddt_scale) + (p.p355 * ddt_scale));let eq130_e1572_d_n12: f64 = ((l.f221a * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq130_e1572, eq130_e1572_d_n2, (l.f221d * ddt_scale), (l.f221e * ddt_scale), eq130_e1572_d_n12, (l.f221b * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1574;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq130_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq130_e1574_d_n2), multiplicity * (eq130_e1574_d_n4), multiplicity * (eq130_e1574_d_n7), multiplicity * (eq130_e1574_d_n12), multiplicity * (eq130_e1574_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq131_e1585, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n7, eq131_e1585_d_n12, eq131_e1585_d_n13,) = {
    if (l.f1f4c == 0.0) {
        let eq131_e1578: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 128, l.f21d6);let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));let eq131_e1582: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 129, eq131_e1581);let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);let eq131_e1583_d_n7: f64 = ((l.f21dc * ddt_scale) + (p.p355 * ddt_scale));let eq131_e1583_d_n13: f64 = ((l.f21d8 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq131_e1583, (l.f21d9 * ddt_scale), (l.f21da * ddt_scale), (l.f21db * ddt_scale), eq131_e1583_d_n7, (l.f21d7 * ddt_scale), eq131_e1583_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1585;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(13),
            multiplicity * (eq131_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq131_e1585_d_n2), multiplicity * (eq131_e1585_d_n3), multiplicity * (eq131_e1585_d_n4), multiplicity * (eq131_e1585_d_n7), multiplicity * (eq131_e1585_d_n12), multiplicity * (eq131_e1585_d_n13)],
            [],
            [],
            1.0,
        );let eq134_e1597: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 130, l.f2196);let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));let eq134_e1601: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 131, eq134_e1600);let eq134_e1602: f64 = (eq134_e1597 + eq134_e1601);let eq134_e1602_d_n3: f64 = ((l.f219a * ddt_scale) + (p.p355 * ddt_scale));let eq134_e1602_d_n13: f64 = ((l.f2198 * ddt_scale) + ((-p.p355) * ddt_scale));let eq134_value: f64 = eq134_e1602;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(13),
            multiplicity * (eq134_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * ((l.f2199 * ddt_scale)), multiplicity * (eq134_e1602_d_n3), multiplicity * ((l.f219b * ddt_scale)), multiplicity * ((l.f219c * ddt_scale)), multiplicity * ((l.f2197 * ddt_scale)), multiplicity * (eq134_e1602_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq135_e1610, eq135_e1610_d_n0, eq135_e1610_d_n2, eq135_e1610_d_n4, eq135_e1610_d_n13, eq135_e1610_d_n19,) = {
    if (l.f1f4e != 0.0) {
        let eq135_e1607: f64 = (l.f1d87 * (nv13 - nv19));let eq135_e1608: f64 = (l.f210f + eq135_e1607);let eq135_e1608_d_n13: f64 = (l.f2111 + l.f1d87);let eq135_e1608_d_n19: f64 = (l.f2112 + (-l.f1d87));
        (eq135_e1608, l.f2110, l.f2113, l.f2114, eq135_e1608_d_n13, eq135_e1608_d_n19,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1610;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(13),
            Some(19),
            multiplicity * (eq135_value),
            [0, 2, 4, 13, 19],
            [multiplicity * (eq135_e1610_d_n0), multiplicity * (eq135_e1610_d_n2), multiplicity * (eq135_e1610_d_n4), multiplicity * (eq135_e1610_d_n13), multiplicity * (eq135_e1610_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq136_e1615,) = {
    if (l.f1f4e == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq136_value: f64 = eq136_e1615;
        stamper.stamp_potential_const_local(
            26,
            eq136_value,
        );
        let (eq137_e1623, eq137_e1623_d_n0, eq137_e1623_d_n2, eq137_e1623_d_n4, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n20,) = {
    if (l.f1f6a != 0.0) {
        let eq137_e1620: f64 = (l.f1d87 * (nv18 - nv17));let eq137_e1621: f64 = (l.f2106 + eq137_e1620);let eq137_e1621_d_n17: f64 = (l.f2108 + (-l.f1d87));let eq137_e1621_d_n18: f64 = (l.f2109 + l.f1d87);
        (eq137_e1621, l.f2107, l.f210a, l.f210c, eq137_e1621_d_n17, eq137_e1621_d_n18, l.f210b,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1623;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(18),
            Some(17),
            multiplicity * (eq137_value),
            [0, 2, 4, 17, 18, 20],
            [multiplicity * (eq137_e1623_d_n0), multiplicity * (eq137_e1623_d_n2), multiplicity * (eq137_e1623_d_n4), multiplicity * (eq137_e1623_d_n17), multiplicity * (eq137_e1623_d_n18), multiplicity * (eq137_e1623_d_n20)],
            [],
            [],
            1.0,
        );
        let (eq138_e1628,) = {
    if (l.f1f6a == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq138_value: f64 = eq138_e1628;
        stamper.stamp_potential_const_local(
            27,
            eq138_value,
        );
        let (eq139_e1632,) = {
    if (l.f1fd2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq139_value: f64 = eq139_e1632;
        stamper.stamp_potential_const_local(
            28,
            eq139_value,
        );
        let (eq140_e1636,) = {
    if (l.f1fd2 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq140_value: f64 = eq140_e1636;
        stamper.stamp_potential_const_local(
            29,
            eq140_value,
        );
        let (eq141_e1644, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n25, eq141_e1644_d_n26,) = {
    if (l.f1fd2 != 0.0) {
        let eq141_e1641: f64 = (l.f1d87 * (nv5 - nv9));let eq141_e1642: f64 = (l.f20bc + eq141_e1641);let eq141_e1642_d_n5: f64 = (l.f20c2 + l.f1d87);let eq141_e1642_d_n9: f64 = (l.f20c4 + (-l.f1d87));
        (eq141_e1642, l.f20c1, eq141_e1642_d_n5, l.f20c3, eq141_e1642_d_n9, l.f20bd, l.f20be, l.f20bf, l.f20c0,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1644;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(5),
            Some(9),
            multiplicity * (eq141_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * (eq141_e1644_d_n4), multiplicity * (eq141_e1644_d_n5), multiplicity * (eq141_e1644_d_n8), multiplicity * (eq141_e1644_d_n9), multiplicity * (eq141_e1644_d_n22), multiplicity * (eq141_e1644_d_n23), multiplicity * (eq141_e1644_d_n25), multiplicity * (eq141_e1644_d_n26)],
            [],
            [],
            1.0,
        );
        let (eq142_e1656, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n28, eq142_e1656_d_n29,) = {
    if (l.f1fd2 == 0.0) {
        let eq142_e1649: f64 = (l.f20bc - (nv29 - 0.0));let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));let eq142_e1653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 132, eq142_e1652);let eq142_e1654: f64 = (eq142_e1649 - eq142_e1653);let eq142_e1654_d_n28: f64 = (-(p.p323 * ddt_scale));
        (eq142_e1654, l.f20c1, l.f20c2, l.f20c3, l.f20c4, l.f20bd, l.f20be, l.f20bf, l.f20c0, eq142_e1654_d_n28, (-1.0),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1656;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(28),
            None,
            multiplicity * (eq142_value),
            [4, 5, 8, 9, 22, 23, 25, 26, 28, 29],
            [multiplicity * (eq142_e1656_d_n4), multiplicity * (eq142_e1656_d_n5), multiplicity * (eq142_e1656_d_n8), multiplicity * (eq142_e1656_d_n9), multiplicity * (eq142_e1656_d_n22), multiplicity * (eq142_e1656_d_n23), multiplicity * (eq142_e1656_d_n25), multiplicity * (eq142_e1656_d_n26), multiplicity * (eq142_e1656_d_n28), multiplicity * (eq142_e1656_d_n29)],
            [],
            [],
            1.0,
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29,) = {
    if (l.f1fd2 == 0.0) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));let eq143_e1664: f64 = (p.p323 / 3.0);let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));let eq143_e1667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 133, eq143_e1666);let eq143_e1668: f64 = (eq143_e1661 - eq143_e1667);let eq143_e1668_d_n29: f64 = ((-1.0) - (eq143_e1664 * ddt_scale));
        (eq143_e1668, 1.0, eq143_e1668_d_n29,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1670;
        stamper.stamp_current_node2_local(
            Some(29),
            None,
            multiplicity * (eq143_value),
            28,
            multiplicity * (eq143_e1670_d_n28),
            29,
            multiplicity * (eq143_e1670_d_n29),
        );
        let (eq144_e1679, eq144_e1679_d_n5, eq144_e1679_d_n9, eq144_e1679_d_n29,) = {
    if (l.f1fd2 == 0.0) {
        let eq144_e1676: f64 = (l.f1d87 * (nv5 - nv9));let eq144_e1677: f64 = (l.f210d + eq144_e1676);
        (eq144_e1677, l.f1d87, (-l.f1d87), l.f210e,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1679;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(9),
            multiplicity * (eq144_value),
            5,
            multiplicity * (eq144_e1679_d_n5),
            9,
            multiplicity * (eq144_e1679_d_n9),
            29,
            multiplicity * (eq144_e1679_d_n29),
        );let eq145_e1681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 134, l.f2220);let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));let eq145_e1685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 135, eq145_e1684);let eq145_e1686: f64 = (eq145_e1681 + eq145_e1685);let eq145_e1686_d_n8: f64 = ((l.f2227 * ddt_scale) + (p.p355 * ddt_scale));let eq145_e1686_d_n9: f64 = ((l.f2228 * ddt_scale) + ((-p.p355) * ddt_scale));let eq145_value: f64 = eq145_e1686;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq145_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * ((l.f2225 * ddt_scale)), multiplicity * ((l.f2226 * ddt_scale)), multiplicity * (eq145_e1686_d_n8), multiplicity * (eq145_e1686_d_n9), multiplicity * ((l.f2221 * ddt_scale)), multiplicity * ((l.f2222 * ddt_scale)), multiplicity * ((l.f2223 * ddt_scale)), multiplicity * ((l.f2224 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
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
        let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv13 = ctx.node_voltage(nodes[13]);let nv17 = ctx.node_voltage(nodes[17]);let eq146_e1688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 136, l.f21de);let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));let eq146_e1692: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 137, eq146_e1691);let eq146_e1693: f64 = (eq146_e1688 + eq146_e1692);let eq146_e1693_d_n5: f64 = ((l.f21e4 * ddt_scale) + ((-p.p355) * ddt_scale));let eq146_e1693_d_n8: f64 = ((l.f21e5 * ddt_scale) + (p.p355 * ddt_scale));let eq146_value: f64 = eq146_e1693;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq146_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * ((l.f21e3 * ddt_scale)), multiplicity * (eq146_e1693_d_n5), multiplicity * (eq146_e1693_d_n8), multiplicity * ((l.f21e6 * ddt_scale)), multiplicity * ((l.f21df * ddt_scale)), multiplicity * ((l.f21e0 * ddt_scale)), multiplicity * ((l.f21e1 * ddt_scale)), multiplicity * ((l.f21e2 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq147_e1701, eq147_e1701_d_n4, eq147_e1701_d_n8, eq147_e1701_d_n13,) = {
    if (l.f1fd4 != 0.0) {
        let eq147_e1698: f64 = (l.f1d87 * (nv8 - nv13));let eq147_e1699: f64 = (l.f2133 + eq147_e1698);let eq147_e1699_d_n8: f64 = (l.f213e + l.f1d87);let eq147_e1699_d_n13: f64 = (l.f213c + (-l.f1d87));
        (eq147_e1699, l.f213d, eq147_e1699_d_n8, eq147_e1699_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1701;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(13),
            multiplicity * (eq147_value),
            4,
            multiplicity * (eq147_e1701_d_n4),
            8,
            multiplicity * (eq147_e1701_d_n8),
            13,
            multiplicity * (eq147_e1701_d_n13),
        );
        let (eq148_e1709, eq148_e1709_d_n4, eq148_e1709_d_n8, eq148_e1709_d_n17,) = {
    if (l.f1fd4 != 0.0) {
        let eq148_e1706: f64 = (l.f1d87 * (nv8 - nv17));let eq148_e1707: f64 = (l.f211c + eq148_e1706);let eq148_e1707_d_n8: f64 = (l.f2127 + l.f1d87);let eq148_e1707_d_n17: f64 = (l.f2125 + (-l.f1d87));
        (eq148_e1707, l.f2126, eq148_e1707_d_n8, eq148_e1707_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1709;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(17),
            multiplicity * (eq148_value),
            4,
            multiplicity * (eq148_e1709_d_n4),
            8,
            multiplicity * (eq148_e1709_d_n8),
            17,
            multiplicity * (eq148_e1709_d_n17),
        );
        let (eq149_e1719, eq149_e1719_d_n4, eq149_e1719_d_n8, eq149_e1719_d_n13,) = {
    if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
        let eq149_e1716: f64 = (l.f1d87 * (nv8 - nv13));let eq149_e1717: f64 = (l.f2134 + eq149_e1716);let eq149_e1717_d_n8: f64 = (l.f2137 + l.f1d87);let eq149_e1717_d_n13: f64 = (l.f2135 + (-l.f1d87));
        (eq149_e1717, l.f2136, eq149_e1717_d_n8, eq149_e1717_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1719;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(13),
            multiplicity * (eq149_value),
            4,
            multiplicity * (eq149_e1719_d_n4),
            8,
            multiplicity * (eq149_e1719_d_n8),
            13,
            multiplicity * (eq149_e1719_d_n13),
        );
        let (eq150_e1729, eq150_e1729_d_n4, eq150_e1729_d_n8, eq150_e1729_d_n17,) = {
    if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
        let eq150_e1726: f64 = (l.f1d87 * (nv8 - nv17));let eq150_e1727: f64 = (l.f211d + eq150_e1726);let eq150_e1727_d_n8: f64 = (l.f2120 + l.f1d87);let eq150_e1727_d_n17: f64 = (l.f211e + (-l.f1d87));
        (eq150_e1727, l.f211f, eq150_e1727_d_n8, eq150_e1727_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1729;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(17),
            multiplicity * (eq150_value),
            4,
            multiplicity * (eq150_e1729_d_n4),
            8,
            multiplicity * (eq150_e1729_d_n8),
            17,
            multiplicity * (eq150_e1729_d_n17),
        );
        let (eq151_e1739, eq151_e1739_d_n4, eq151_e1739_d_n8, eq151_e1739_d_n9,) = {
    if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
        let eq151_e1736: f64 = (l.f1d87 * (nv8 - nv9));let eq151_e1737: f64 = (l.f213f + eq151_e1736);let eq151_e1737_d_n8: f64 = (l.f2141 + l.f1d87);let eq151_e1737_d_n9: f64 = (l.f2142 + (-l.f1d87));
        (eq151_e1737, l.f2140, eq151_e1737_d_n8, eq151_e1737_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1739;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(9),
            multiplicity * (eq151_value),
            4,
            multiplicity * (eq151_e1739_d_n4),
            8,
            multiplicity * (eq151_e1739_d_n8),
            9,
            multiplicity * (eq151_e1739_d_n9),
        );
        let (eq152_e1749, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n8,) = {
    if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
        let eq152_e1746: f64 = (l.f1d87 * (nv8 - nv5));let eq152_e1747: f64 = (l.f2128 + eq152_e1746);let eq152_e1747_d_n5: f64 = (l.f212a + (-l.f1d87));let eq152_e1747_d_n8: f64 = (l.f212b + l.f1d87);
        (eq152_e1747, l.f2129, eq152_e1747_d_n5, eq152_e1747_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1749;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (eq152_value),
            4,
            multiplicity * (eq152_e1749_d_n4),
            5,
            multiplicity * (eq152_e1749_d_n5),
            8,
            multiplicity * (eq152_e1749_d_n8),
        );
        let (eq153_e1761, eq153_e1761_d_n4, eq153_e1761_d_n8, eq153_e1761_d_n9,) = {
    if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
        let eq153_e1758: f64 = (l.f1d87 * (nv8 - nv9));let eq153_e1759: f64 = (l.f2138 + eq153_e1758);let eq153_e1759_d_n8: f64 = (l.f213a + l.f1d87);let eq153_e1759_d_n9: f64 = (l.f213b + (-l.f1d87));
        (eq153_e1759, l.f2139, eq153_e1759_d_n8, eq153_e1759_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1761;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(9),
            multiplicity * (eq153_value),
            4,
            multiplicity * (eq153_e1761_d_n4),
            8,
            multiplicity * (eq153_e1761_d_n8),
            9,
            multiplicity * (eq153_e1761_d_n9),
        );
        let (eq154_e1773, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n8,) = {
    if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
        let eq154_e1770: f64 = (l.f1d87 * (nv8 - nv5));let eq154_e1771: f64 = (l.f2121 + eq154_e1770);let eq154_e1771_d_n5: f64 = (l.f2123 + (-l.f1d87));let eq154_e1771_d_n8: f64 = (l.f2124 + l.f1d87);
        (eq154_e1771, l.f2122, eq154_e1771_d_n5, eq154_e1771_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1773;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (eq154_value),
            4,
            multiplicity * (eq154_e1773_d_n4),
            5,
            multiplicity * (eq154_e1773_d_n5),
            8,
            multiplicity * (eq154_e1773_d_n8),
        );
        let (eq155_e1781, eq155_e1781_d_n4, eq155_e1781_d_n7, eq155_e1781_d_n8,) = {
    if (l.f2002 != 0.0) {
        let eq155_e1778: f64 = (l.f1d87 * (nv8 - nv7));let eq155_e1779: f64 = (l.f20c6 + eq155_e1778);let eq155_e1779_d_n7: f64 = (l.f20cc + (-l.f1d87));let eq155_e1779_d_n8: f64 = (l.f20cd + l.f1d87);
        (eq155_e1779, l.f20cb, eq155_e1779_d_n7, eq155_e1779_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1781;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(7),
            multiplicity * (eq155_value),
            4,
            multiplicity * (eq155_e1781_d_n4),
            7,
            multiplicity * (eq155_e1781_d_n7),
            8,
            multiplicity * (eq155_e1781_d_n8),
        );
        let (eq156_e1791, eq156_e1791_d_n4, eq156_e1791_d_n7, eq156_e1791_d_n8,) = {
    if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
        let eq156_e1788: f64 = (l.f1d87 * (nv8 - nv7));let eq156_e1789: f64 = (l.f20c7 + eq156_e1788);let eq156_e1789_d_n7: f64 = (l.f20c9 + (-l.f1d87));let eq156_e1789_d_n8: f64 = (l.f20ca + l.f1d87);
        (eq156_e1789, l.f20c8, eq156_e1789_d_n7, eq156_e1789_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1791;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(7),
            multiplicity * (eq156_value),
            4,
            multiplicity * (eq156_e1791_d_n4),
            7,
            multiplicity * (eq156_e1791_d_n7),
            8,
            multiplicity * (eq156_e1791_d_n8),
        );
        let (eq157_e1796, eq157_e1796_d_n7, eq157_e1796_d_n8,) = {
    if (l.f2002 != 0.0) {
        let eq157_e1794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 138, l.f2280);
        (eq157_e1794, (l.f22a1 * ddt_scale), (l.f22a2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1796;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(7),
            multiplicity * (eq157_value),
            7,
            multiplicity * (eq157_e1796_d_n7),
            8,
            multiplicity * (eq157_e1796_d_n8),
        );
        let (eq158_e1804, eq158_e1804_d_n7, eq158_e1804_d_n8,) = {
    if ((l.f2002 != 0.0) && (l.f201b != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / l.f22ea;let eq158_e1802: f64 = ((nv8 - nv7) * __rspice_inv_cse_0);let eq158_e1802_d_n7: f64 = ((-1.0) * __rspice_inv_cse_0);let eq158_e1802_d_n8: f64 = (1.0 * __rspice_inv_cse_0);
        (eq158_e1802, eq158_e1802_d_n7, eq158_e1802_d_n8,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e1804;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(7),
            multiplicity * (eq158_value),
            7,
            multiplicity * (eq158_e1804_d_n7),
            8,
            multiplicity * (eq158_e1804_d_n8),
        );
        let (eq159_e1809,) = {
    if (l.f2002 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq159_value: f64 = eq159_e1809;
        stamper.stamp_potential_const_local(
            30,
            eq159_value,
        );
        let (eq160_e1815, eq160_e1815_d_n0, eq160_e1815_d_n2, eq160_e1815_d_n4, eq160_e1815_d_n8, eq160_e1815_d_n18, eq160_e1815_d_n19,) = {
    if ((l.f201e != 0.0) && (l.f202a != 0.0)) {
        (l.f212c, l.f212d, l.f2130, l.f2131, l.f2132, l.f212e, l.f212f,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e1815;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(0),
            multiplicity * (eq160_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq160_e1815_d_n0), multiplicity * (eq160_e1815_d_n2), multiplicity * (eq160_e1815_d_n4), multiplicity * (eq160_e1815_d_n8), multiplicity * (eq160_e1815_d_n18), multiplicity * (eq160_e1815_d_n19)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv4 = ctx.node_voltage(nodes[4]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv18 = ctx.node_voltage(nodes[18]);let nv19 = ctx.node_voltage(nodes[19]);
        let (eq161_e1821, eq161_e1821_d_n0, eq161_e1821_d_n2, eq161_e1821_d_n4, eq161_e1821_d_n8, eq161_e1821_d_n18, eq161_e1821_d_n19,) = {
    if ((l.f201e != 0.0) && (l.f202a != 0.0)) {
        (l.f2115, l.f2116, l.f2119, l.f211a, l.f211b, l.f2117, l.f2118,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e1821;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(0),
            Some(2),
            multiplicity * (eq161_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq161_e1821_d_n0), multiplicity * (eq161_e1821_d_n2), multiplicity * (eq161_e1821_d_n4), multiplicity * (eq161_e1821_d_n8), multiplicity * (eq161_e1821_d_n18), multiplicity * (eq161_e1821_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq162_e1828, eq162_e1828_d_n0, eq162_e1828_d_n2, eq162_e1828_d_n4, eq162_e1828_d_n8, eq162_e1828_d_n18, eq162_e1828_d_n19,) = {
    if ((l.f201e != 0.0) && (l.f202a == 0.0)) {
        (l.f212c, l.f212d, l.f2130, l.f2131, l.f2132, l.f212e, l.f212f,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e1828;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(19),
            Some(18),
            multiplicity * (eq162_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq162_e1828_d_n0), multiplicity * (eq162_e1828_d_n2), multiplicity * (eq162_e1828_d_n4), multiplicity * (eq162_e1828_d_n8), multiplicity * (eq162_e1828_d_n18), multiplicity * (eq162_e1828_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq163_e1835, eq163_e1835_d_n0, eq163_e1835_d_n2, eq163_e1835_d_n4, eq163_e1835_d_n8, eq163_e1835_d_n18, eq163_e1835_d_n19,) = {
    if ((l.f201e != 0.0) && (l.f202a == 0.0)) {
        (l.f2115, l.f2116, l.f2119, l.f211a, l.f211b, l.f2117, l.f2118,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e1835;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(18),
            Some(19),
            multiplicity * (eq163_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq163_e1835_d_n0), multiplicity * (eq163_e1835_d_n2), multiplicity * (eq163_e1835_d_n4), multiplicity * (eq163_e1835_d_n8), multiplicity * (eq163_e1835_d_n18), multiplicity * (eq163_e1835_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq164_e1841, eq164_e1841_d_n0, eq164_e1841_d_n4, eq164_e1841_d_n18,) = {
    if (l.f202b != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / l.f22e6;let eq164_e1839: f64 = ((nv0 - nv18) * __rspice_inv_cse_0);let eq164_e1839_d_n0: f64 = (1.0 * __rspice_inv_cse_0);let eq164_e1839_d_n4: f64 = (-(((nv0 - nv18) * l.f22e7) / (l.f22e6 * l.f22e6)));let eq164_e1839_d_n18: f64 = (-1.0 / l.f22e6);
        (eq164_e1839, eq164_e1839_d_n0, eq164_e1839_d_n4, eq164_e1839_d_n18,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e1841;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(18),
            multiplicity * (eq164_value),
            0,
            multiplicity * (eq164_e1841_d_n0),
            4,
            multiplicity * (eq164_e1841_d_n4),
            18,
            multiplicity * (eq164_e1841_d_n18),
        );
        let (eq165_e1846,) = {
    if (l.f202b == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq165_value: f64 = eq165_e1846;
        stamper.stamp_potential_const_local(
            31,
            eq165_value,
        );
        let (eq166_e1852, eq166_e1852_d_n2, eq166_e1852_d_n4, eq166_e1852_d_n19,) = {
    if (l.f202c != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / l.f22eb;let eq166_e1850: f64 = ((nv19 - nv2) * __rspice_inv_cse_1);let eq166_e1850_d_n2: f64 = ((-1.0) * __rspice_inv_cse_1);let eq166_e1850_d_n4: f64 = (-(((nv19 - nv2) * l.f22ec) / (l.f22eb * l.f22eb)));let eq166_e1850_d_n19: f64 = (1.0 / l.f22eb);
        (eq166_e1850, eq166_e1850_d_n2, eq166_e1850_d_n4, eq166_e1850_d_n19,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e1852;
        stamper.stamp_current_node3_local(
            Some(19),
            Some(2),
            multiplicity * (eq166_value),
            2,
            multiplicity * (eq166_e1852_d_n2),
            4,
            multiplicity * (eq166_e1852_d_n4),
            19,
            multiplicity * (eq166_e1852_d_n19),
        );
        let (eq167_e1857,) = {
    if (l.f202c == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq167_value: f64 = eq167_e1857;
        stamper.stamp_potential_const_local(
            32,
            eq167_value,
        );
        let (eq168_e1863, eq168_e1863_d_n1, eq168_e1863_d_n6,) = {
    if (l.f202d != 0.0) {
        let __rspice_inv_cse_2: f64 = 1.0 / l.f22e8;let eq168_e1861: f64 = ((nv1 - nv6) * __rspice_inv_cse_2);let eq168_e1861_d_n1: f64 = (1.0 * __rspice_inv_cse_2);let eq168_e1861_d_n6: f64 = ((-1.0) * __rspice_inv_cse_2);
        (eq168_e1861, eq168_e1861_d_n1, eq168_e1861_d_n6,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e1863;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (eq168_value),
            1,
            multiplicity * (eq168_e1863_d_n1),
            6,
            multiplicity * (eq168_e1863_d_n6),
        );
        let (eq169_e1868,) = {
    if (l.f202d == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq169_value: f64 = eq169_e1868;
        stamper.stamp_potential_const_local(
            33,
            eq169_value,
        );
        let (eq170_e1874, eq170_e1874_d_n6, eq170_e1874_d_n7,) = {
    if (l.f202e != 0.0) {
        let __rspice_inv_cse_3: f64 = 1.0 / l.f22e9;let eq170_e1872: f64 = ((nv6 - nv7) * __rspice_inv_cse_3);let eq170_e1872_d_n6: f64 = (1.0 * __rspice_inv_cse_3);let eq170_e1872_d_n7: f64 = ((-1.0) * __rspice_inv_cse_3);
        (eq170_e1872, eq170_e1872_d_n6, eq170_e1872_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e1874;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(7),
            multiplicity * (eq170_value),
            6,
            multiplicity * (eq170_e1874_d_n6),
            7,
            multiplicity * (eq170_e1874_d_n7),
        );
        let (eq171_e1879,) = {
    if (l.f202e == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq171_value: f64 = eq171_e1879;
        stamper.stamp_potential_const_local(
            34,
            eq171_value,
        );let eq172_e1881: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 139, l.f2276);let eq172_value: f64 = eq172_e1881;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(2),
            multiplicity * (eq172_value),
            2,
            multiplicity * ((l.f2277 * ddt_scale)),
            4,
            multiplicity * ((l.f2278 * ddt_scale)),
            6,
            multiplicity * ((l.f2279 * ddt_scale)),
        );let eq173_e1883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 140, l.f2262);let eq173_value: f64 = eq173_e1883;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(0),
            multiplicity * (eq173_value),
            0,
            multiplicity * ((l.f2263 * ddt_scale)),
            4,
            multiplicity * ((l.f2264 * ddt_scale)),
            6,
            multiplicity * ((l.f2265 * ddt_scale)),
        );let eq174_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 141, l.f2267);let eq174_value: f64 = eq174_e1885;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(0),
            multiplicity * (eq174_value),
            0,
            multiplicity * ((l.f2268 * ddt_scale)),
            2,
            multiplicity * ((l.f2269 * ddt_scale)),
            4,
            multiplicity * ((l.f226a * ddt_scale)),
        );let eq175_e1887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 142, l.f227b);let eq175_value: f64 = eq175_e1887;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(2),
            multiplicity * (eq175_value),
            2,
            multiplicity * ((l.f227c * ddt_scale)),
            3,
            multiplicity * ((l.f227d * ddt_scale)),
            4,
            multiplicity * ((l.f227e * ddt_scale)),
        );let eq176_e1889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 143, l.f226c);let eq176_value: f64 = eq176_e1889;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(0),
            multiplicity * (eq176_value),
            0,
            multiplicity * ((l.f226d * ddt_scale)),
            3,
            multiplicity * ((l.f226e * ddt_scale)),
            4,
            multiplicity * ((l.f226f * ddt_scale)),
        );let eq177_e1891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 144, l.f2271);let eq177_value: f64 = eq177_e1891;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(3),
            multiplicity * (eq177_value),
            3,
            multiplicity * ((l.f2272 * ddt_scale)),
            4,
            multiplicity * ((l.f2273 * ddt_scale)),
            6,
            multiplicity * ((l.f2274 * ddt_scale)),
        );
        let (eq194_e2167, eq194_e2167_d_n4,) = {
    if (l.f205b != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));let eq194_e2165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 145, eq194_e2164);
        (eq194_e2165, (p.p321 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2167;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq194_value),
            4,
            multiplicity * (eq194_e2167_d_n4),
        );
        let (eq195_e2172, eq195_e2172_d_n0, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n25, eq195_e2172_d_n26,) = {
    if (l.f205b != 0.0) {
        let eq195_e2170: f64 = (-l.f2143);
        (eq195_e2170, (-l.f2144), (-l.f214f), (-l.f2155), (-l.f2156), (-l.f2157), (-l.f2158), (-l.f2159), (-l.f215a), (-l.f2145), (-l.f2146), (-l.f2147), (-l.f2148), (-l.f2149), (-l.f214a), (-l.f214b), (-l.f214c), (-l.f214d), (-l.f214e), (-l.f2150), (-l.f2151), (-l.f2152), (-l.f2153), (-l.f2154),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2172;let eq195_node_derivative_indices: [usize; 23] = [0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 25, 26];let eq195_node_derivatives: [f64; 23] = [eq195_e2172_d_n0, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n25, eq195_e2172_d_n26];let eq195_branch_derivative_indices: [usize; 0] = [];let eq195_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq195_value),
            &eq195_node_derivative_indices,
            &eq195_node_derivatives,
            &eq195_branch_derivative_indices,
            &eq195_branch_derivatives,
            multiplicity,
        );
        let (eq196_e2178, eq196_e2178_d_n4,) = {
    if (l.f205b != 0.0) {
        let __rspice_inv_cse_4: f64 = 1.0 / p.p320;let eq196_e2176: f64 = ((nv4 - 0.0) * __rspice_inv_cse_4);let eq196_e2176_d_n4: f64 = (1.0 * __rspice_inv_cse_4);
        (eq196_e2176, eq196_e2176_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2178;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq196_value),
            4,
            multiplicity * (eq196_e2178_d_n4),
        );
        let (eq197_e2183,) = {
    if (l.f205b == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq197_value: f64 = eq197_e2183;
        stamper.stamp_potential_const_local(
            35,
            eq197_value,
        );
    }
}
