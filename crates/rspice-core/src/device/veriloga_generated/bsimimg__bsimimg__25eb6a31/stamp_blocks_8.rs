#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let t0: f64 = if l.f603 > 0.0 { 1.0 } else { 0.0 };l.f26a = t0;l.f26b = 0.0;
        if (l.f26a != 0.0) {let t1: f64 = (l.f226 * l.f52d);let t2: f64 = (l.f5c1 + t1);let t3: f64 = (t2 / l.f601);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (t3, (l.f5ca / l.f601), (l.f5cb / l.f601), (l.f5cc / l.f601), (l.f5cd / l.f601), (l.f5ce / l.f601), (l.f5cf / l.f601), );l.f73f = 0.0;}
        if (l.f26a != 0.0) {let t4: f64 = (l.f730).powf(l.f4f6);let t5: f64 = (1.0 + t4);(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (t5, if 0.0 == 0.0 && ((l.f4f6) as f64).is_finite() && ((l.f4f6) as f64).fract() == 0.0 { if l.f4f6 == 0.0 { 0.0 } else { (l.f4f6 * ((l.f730).powf(l.f4f6 - 1.0) * l.f739)) } } else { (t4 * (l.f4f6 * (l.f739 / l.f730))) }, if 0.0 == 0.0 && ((l.f4f6) as f64).is_finite() && ((l.f4f6) as f64).fract() == 0.0 { if l.f4f6 == 0.0 { 0.0 } else { (l.f4f6 * ((l.f730).powf(l.f4f6 - 1.0) * l.f73a)) } } else { (t4 * (l.f4f6 * (l.f73a / l.f730))) }, if 0.0 == 0.0 && ((l.f4f6) as f64).is_finite() && ((l.f4f6) as f64).fract() == 0.0 { if l.f4f6 == 0.0 { 0.0 } else { (l.f4f6 * ((l.f730).powf(l.f4f6 - 1.0) * l.f73b)) } } else { (t4 * (l.f4f6 * (l.f73b / l.f730))) }, if 0.0 == 0.0 && ((l.f4f6) as f64).is_finite() && ((l.f4f6) as f64).fract() == 0.0 { if l.f4f6 == 0.0 { 0.0 } else { (l.f4f6 * ((l.f730).powf(l.f4f6 - 1.0) * l.f73c)) } } else { (t4 * (l.f4f6 * (l.f73c / l.f730))) }, if 0.0 == 0.0 && ((l.f4f6) as f64).is_finite() && ((l.f4f6) as f64).fract() == 0.0 { if l.f4f6 == 0.0 { 0.0 } else { (l.f4f6 * ((l.f730).powf(l.f4f6 - 1.0) * l.f73d)) } } else { (t4 * (l.f4f6 * (l.f73d / l.f730))) }, if 0.0 == 0.0 && ((l.f4f6) as f64).is_finite() && ((l.f4f6) as f64).fract() == 0.0 { if l.f4f6 == 0.0 { 0.0 } else { (l.f4f6 * ((l.f730).powf(l.f4f6 - 1.0) * l.f73e)) } } else { (t4 * (l.f4f6 * (l.f73e / l.f730))) }, );l.f747 = 0.0;}
        if (l.f26a != 0.0) {l.f781 = p.p49;l.f782 = 0.0;let t6: f64 = (l.f781 / l.f740);(l.f780, l.f783, l.f784, l.f785, l.f786, l.f787, l.f788, ) = (t6, (-((l.f781 * l.f741) / (l.f740 * l.f740))), (-((l.f781 * l.f742) / (l.f740 * l.f740))), (-((l.f781 * l.f743) / (l.f740 * l.f740))), (-((l.f781 * l.f744) / (l.f740 * l.f740))), (-((l.f781 * l.f745) / (l.f740 * l.f740))), (-((l.f781 * l.f746) / (l.f740 * l.f740))), );l.f789 = 0.0;let t7: f64 = (3.9 * 8.85418e-12);let t8: f64 = (l.f3ae * 3.9);let t9: f64 = (t8 / p.p60);let ta: f64 = (l.f780 * l.f603);let tb: f64 = (ta / l.f1d7);let tc: f64 = (t9 + tb);let td: f64 = (t7 / tc);(l.f88, l.f89, l.f8a, l.f8b, l.f8c, l.f8d, l.f8e, ) = (td, (-((t7 * ((l.f783 * l.f603) / l.f1d7)) / (tc * tc))), (-((t7 * ((l.f784 * l.f603) / l.f1d7)) / (tc * tc))), (-((t7 * ((l.f785 * l.f603) / l.f1d7)) / (tc * tc))), (-((t7 * ((l.f786 * l.f603) / l.f1d7)) / (tc * tc))), (-((t7 * ((l.f787 * l.f603) / l.f1d7)) / (tc * tc))), (-((t7 * ((l.f788 * l.f603) / l.f1d7)) / (tc * tc))), );l.f8f = 0.0;}
        if (l.f26a == 0.0) {(l.f88, l.f89, l.f8a, l.f8b, l.f8c, l.f8d, l.f8e, ) = (l.f84, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f8f = 0.0;}
        let te: f64 = (l.f950 * l.f3f7);let tf: f64 = (te / l.f41d);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (tf, (-((te * l.f41e) / (l.f41d * l.f41d))), (-((te * l.f41f) / (l.f41d * l.f41d))), (-((te * l.f420) / (l.f41d * l.f41d))), (-((te * l.f421) / (l.f41d * l.f41d))), (-((te * l.f422) / (l.f41d * l.f41d))), (-((te * l.f423) / (l.f41d * l.f41d))), );l.f6a7 = 0.0;let t10: f64 = (l.f579 * l.f6a0);(l.f579, l.f57a, l.f57b, l.f57c, l.f57d, l.f57e, l.f57f, ) = (t10, ((l.f57a * l.f6a0) + (l.f579 * l.f6a1)), ((l.f57b * l.f6a0) + (l.f579 * l.f6a2)), ((l.f57c * l.f6a0) + (l.f579 * l.f6a3)), ((l.f57d * l.f6a0) + (l.f579 * l.f6a4)), ((l.f57e * l.f6a0) + (l.f579 * l.f6a5)), ((l.f57f * l.f6a0) + (l.f579 * l.f6a6)), );l.f580 = 0.0;let t11: f64 = (-l.f561);let t12: f64 = (t11 * l.f6a0);(l.f561, l.f562, l.f563, l.f564, l.f565, l.f566, l.f567, ) = (t12, (((-l.f562) * l.f6a0) + (t11 * l.f6a1)), (((-l.f563) * l.f6a0) + (t11 * l.f6a2)), (((-l.f564) * l.f6a0) + (t11 * l.f6a3)), (((-l.f565) * l.f6a0) + (t11 * l.f6a4)), (((-l.f566) * l.f6a0) + (t11 * l.f6a5)), (((-l.f567) * l.f6a0) + (t11 * l.f6a6)), );l.f568 = 0.0;let t13: f64 = (l.f53f * l.f6a0);(l.f53f, l.f540, l.f541, l.f542, l.f543, l.f544, l.f545, ) = (t13, ((l.f540 * l.f6a0) + (l.f53f * l.f6a1)), ((l.f541 * l.f6a0) + (l.f53f * l.f6a2)), ((l.f542 * l.f6a0) + (l.f53f * l.f6a3)), ((l.f543 * l.f6a0) + (l.f53f * l.f6a4)), ((l.f544 * l.f6a0) + (l.f53f * l.f6a5)), ((l.f545 * l.f6a0) + (l.f53f * l.f6a6)), );l.f546 = 0.0;let t14: f64 = (-l.f605);let t15: f64 = (t14 * l.f6a0);(l.f605, l.f606, l.f607, l.f608, l.f609, l.f60a, l.f60b, ) = (t15, (((-l.f606) * l.f6a0) + (t14 * l.f6a1)), (((-l.f607) * l.f6a0) + (t14 * l.f6a2)), (((-l.f608) * l.f6a0) + (t14 * l.f6a3)), (((-l.f609) * l.f6a0) + (t14 * l.f6a4)), (((-l.f60a) * l.f6a0) + (t14 * l.f6a5)), (((-l.f60b) * l.f6a0) + (t14 * l.f6a6)), );l.f60c = 0.0;let t16: f64 = (l.f950 * l.f40d);let t17: f64 = (t16 * l.f82);let t18: f64 = (t17 * (nv7 - nv6));(l.f5a1, l.f5a2, l.f5a3, l.f5a4, l.f5a5, l.f5a6, l.f5a7, ) = (t18, 0.0, 0.0, 0.0, (-t17), t17, 0.0, );l.f5a8 = 0.0;let t19: f64 = (l.f950 * l.f40b);let t1a: f64 = (t19 * l.f82);let t1b: f64 = (t1a * (nv7 - nv5));(l.f585, l.f586, l.f587, l.f588, l.f589, l.f58a, l.f58b, ) = (t1b, 0.0, 0.0, (-t1a), 0.0, t1a, 0.0, );l.f58c = 0.0;let t1c: f64 = (l.f4b5 - l.f4c2);let t1d: f64 = (l.fc6 * t1c);(l.f883, l.f884, l.f885, l.f886, l.f887, l.f888, l.f889, ) = (t1d, (l.fc6 * (l.f4b6 - l.f4c3)), (l.fc6 * (l.f4b7 - l.f4c4)), (l.fc6 * (l.f4b8 - l.f4c5)), (l.fc6 * (l.f4b9 - l.f4c6)), (l.fc6 * (l.f4ba - l.f4c7)), (l.fc6 * (l.f4bb - l.f4c8)), );l.f88a = 0.0;let t1e: f64 = (l.f8f9 - l.f882);let t1f: f64 = (t1e + 0.02);let t20: f64 = (p.p45 / p.p46);let t21: f64 = (l.f847 - l.f883);let t22: f64 = (t21 - p.p268);let t23: f64 = (t20 * t22);let t24: f64 = (t23 * p.p269);let t25: f64 = (t1f + t24);
        (l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t25, ((-l.f88b) + ((t20 * (l.f848 - l.f884)) * p.p269)), ((-l.f88c) + ((t20 * (-l.f885)) * p.p269)), ((-l.f88d) + ((t20 * (-l.f886)) * p.p269)), ((l.f8fa - l.f88e) + ((t20 * (l.f849 - l.f887)) * p.p269)), ((l.f8fb - l.f88f) + ((t20 * (-l.f888)) * p.p269)), ((-l.f890) + ((t20 * (-l.f889)) * p.p269)), );l.f6a7 = 0.0;let t26: f64 = (l.f6a0 * l.f6a0);let t27: f64 = (4.0 * 0.02);let t28: f64 = (t26 + t27);let t29: f64 = (t28).sqrt();let t2a: f64 = (l.f6a0 - t29);let t2b: f64 = (0.5 * t2a);(l.f8b6, l.f8b7, l.f8b8, l.f8b9, l.f8ba, l.f8bb, l.f8bc, ) = (t2b, (0.5 * (l.f6a1 - (((l.f6a1 * l.f6a0) + (l.f6a0 * l.f6a1)) / (2.0 * t29)))), (0.5 * (l.f6a2 - (((l.f6a2 * l.f6a0) + (l.f6a0 * l.f6a2)) / (2.0 * t29)))), (0.5 * (l.f6a3 - (((l.f6a3 * l.f6a0) + (l.f6a0 * l.f6a3)) / (2.0 * t29)))), (0.5 * (l.f6a4 - (((l.f6a4 * l.f6a0) + (l.f6a0 * l.f6a4)) / (2.0 * t29)))), (0.5 * (l.f6a5 - (((l.f6a5 * l.f6a0) + (l.f6a0 * l.f6a5)) / (2.0 * t29)))), (0.5 * (l.f6a6 - (((l.f6a6 * l.f6a0) + (l.f6a0 * l.f6a6)) / (2.0 * t29)))), );l.f8bd = 0.0;let t2c: f64 = (l.f8f9 - l.f882);let t2d: f64 = (t2c - l.f8b6);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t2d, ((-l.f88b) - l.f8b7), ((-l.f88c) - l.f8b8), ((-l.f88d) - l.f8b9), ((l.f8fa - l.f88e) - l.f8ba), ((l.f8fb - l.f88f) - l.f8bb), ((-l.f890) - l.f8bc), );l.f6cf = 0.0;let t2e: f64 = (l.fc6 * l.f950);let t2f: f64 = (t2e * p.p263);let t30: f64 = (0.5 * p.p265);let t31: f64 = (4.0 * l.f8b6);let t32: f64 = (t31 / p.p265);let t33: f64 = (1.0 - t32);let t34: f64 = (t33).sqrt();let t35: f64 = (t34 - 1.0);let t36: f64 = (t30 * t35);let t37: f64 = (l.f6a8 - t36);let t38: f64 = (t2f * t37);let t39: f64 = (l.f5a1 + t38);(l.f5a1, l.f5a2, l.f5a3, l.f5a4, l.f5a5, l.f5a6, l.f5a7, ) = (t39, (l.f5a2 + (t2f * (l.f6c1 - (t30 * ((-((4.0 * l.f8b7) / p.p265)) / (2.0 * t34)))))), (l.f5a3 + (t2f * (l.f6c2 - (t30 * ((-((4.0 * l.f8b8) / p.p265)) / (2.0 * t34)))))), (l.f5a4 + (t2f * (l.f6c3 - (t30 * ((-((4.0 * l.f8b9) / p.p265)) / (2.0 * t34)))))), (l.f5a5 + (t2f * (l.f6c4 - (t30 * ((-((4.0 * l.f8ba) / p.p265)) / (2.0 * t34)))))), (l.f5a6 + (t2f * (l.f6c5 - (t30 * ((-((4.0 * l.f8bb) / p.p265)) / (2.0 * t34)))))), (l.f5a7 + (t2f * (l.f6c6 - (t30 * ((-((4.0 * l.f8bc) / p.p265)) / (2.0 * t34)))))), );l.f5a8 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let t3a: f64 = (l.f8cf - l.f882);let t3b: f64 = (t3a + 0.02);let t3c: f64 = (p.p45 / p.p46);let t3d: f64 = (l.f83a - l.f883);let t3e: f64 = (t3d - p.p270);let t3f: f64 = (t3c * t3e);let t40: f64 = (t3f * p.p271);let t41: f64 = (t3b + t40);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t41, ((-l.f88b) + ((t3c * (l.f83b - l.f884)) * p.p271)), ((-l.f88c) + ((t3c * (-l.f885)) * p.p271)), ((l.f8d0 - l.f88d) + ((t3c * (l.f83c - l.f886)) * p.p271)), ((-l.f88e) + ((t3c * (-l.f887)) * p.p271)), ((l.f8d1 - l.f88f) + ((t3c * (-l.f888)) * p.p271)), ((-l.f890) + ((t3c * (-l.f889)) * p.p271)), );l.f6a7 = 0.0;let t42: f64 = (l.f6a0 * l.f6a0);let t43: f64 = (4.0 * 0.02);let t44: f64 = (t42 + t43);let t45: f64 = (t44).sqrt();let t46: f64 = (l.f6a0 - t45);let t47: f64 = (0.5 * t46);(l.f8a2, l.f8a3, l.f8a4, l.f8a5, l.f8a6, l.f8a7, l.f8a8, ) = (t47, (0.5 * (l.f6a1 - (((l.f6a1 * l.f6a0) + (l.f6a0 * l.f6a1)) / (2.0 * t45)))), (0.5 * (l.f6a2 - (((l.f6a2 * l.f6a0) + (l.f6a0 * l.f6a2)) / (2.0 * t45)))), (0.5 * (l.f6a3 - (((l.f6a3 * l.f6a0) + (l.f6a0 * l.f6a3)) / (2.0 * t45)))), (0.5 * (l.f6a4 - (((l.f6a4 * l.f6a0) + (l.f6a0 * l.f6a4)) / (2.0 * t45)))), (0.5 * (l.f6a5 - (((l.f6a5 * l.f6a0) + (l.f6a0 * l.f6a5)) / (2.0 * t45)))), (0.5 * (l.f6a6 - (((l.f6a6 * l.f6a0) + (l.f6a0 * l.f6a6)) / (2.0 * t45)))), );l.f8a9 = 0.0;let t48: f64 = (l.f8cf - l.f882);let t49: f64 = (t48 - l.f8a2);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t49, ((-l.f88b) - l.f8a3), ((-l.f88c) - l.f8a4), ((l.f8d0 - l.f88d) - l.f8a5), ((-l.f88e) - l.f8a6), ((l.f8d1 - l.f88f) - l.f8a7), ((-l.f890) - l.f8a8), );l.f6cf = 0.0;let t4a: f64 = (l.fc6 * l.f950);let t4b: f64 = (t4a * p.p264);let t4c: f64 = (0.5 * p.p266);let t4d: f64 = (4.0 * l.f8a2);let t4e: f64 = (t4d / p.p266);let t4f: f64 = (1.0 - t4e);let t50: f64 = (t4f).sqrt();let t51: f64 = (t50 - 1.0);let t52: f64 = (t4c * t51);let t53: f64 = (l.f6a8 - t52);let t54: f64 = (t4b * t53);let t55: f64 = (l.f585 + t54);(l.f585, l.f586, l.f587, l.f588, l.f589, l.f58a, l.f58b, ) = (t55, (l.f586 + (t4b * (l.f6c1 - (t4c * ((-((4.0 * l.f8a3) / p.p266)) / (2.0 * t50)))))), (l.f587 + (t4b * (l.f6c2 - (t4c * ((-((4.0 * l.f8a4) / p.p266)) / (2.0 * t50)))))), (l.f588 + (t4b * (l.f6c3 - (t4c * ((-((4.0 * l.f8a5) / p.p266)) / (2.0 * t50)))))), (l.f589 + (t4b * (l.f6c4 - (t4c * ((-((4.0 * l.f8a6) / p.p266)) / (2.0 * t50)))))), (l.f58a + (t4b * (l.f6c5 - (t4c * ((-((4.0 * l.f8a7) / p.p266)) / (2.0 * t50)))))), (l.f58b + (t4b * (l.f6c6 - (t4c * ((-((4.0 * l.f8a8) / p.p266)) / (2.0 * t50)))))), );l.f58c = 0.0;let t56: f64 = (l.f950 * l.f6c);
        let t57: f64 = (t56 * (nv7 - nv6));(l.f59d, l.f59e, l.f59f, ) = (t57, (-t56), t56, );l.f5a0 = 0.0;let t58: f64 = (l.f950 * l.f6a);let t59: f64 = (t58 * (nv7 - nv5));(l.f581, l.f582, l.f583, ) = (t59, (-t58), t58, );l.f584 = 0.0;let t5a: f64 = (l.f5a1 + l.f59d);(l.f5a9, l.f5aa, l.f5ab, l.f5ac, l.f5ad, l.f5ae, l.f5af, ) = (t5a, l.f5a2, l.f5a3, l.f5a4, (l.f5a5 + l.f59e), (l.f5a6 + l.f59f), l.f5a7, );l.f5b0 = 0.0;let t5b: f64 = (l.f585 + l.f581);(l.f58d, l.f58e, l.f58f, l.f590, l.f591, l.f592, l.f593, ) = (t5b, l.f586, l.f587, (l.f588 + l.f582), l.f589, (l.f58a + l.f583), l.f58b, );l.f594 = 0.0;let t5c: f64 = (l.fc6 * l.f90);let t5d: f64 = (t5c * (nv6 - nv3));(l.f60d, l.f60e, l.f60f, l.f610, l.f611, l.f612, l.f613, ) = (t5d, (((l.fc6 * l.f91) * (nv6 - nv3)) + (-t5c)), ((l.fc6 * l.f92) * (nv6 - nv3)), ((l.fc6 * l.f93) * (nv6 - nv3)), (((l.fc6 * l.f94) * (nv6 - nv3)) + t5c), ((l.fc6 * l.f95) * (nv6 - nv3)), ((l.fc6 * l.f96) * (nv6 - nv3)), );l.f614 = 0.0;let t5e: f64 = (l.fc6 * l.f5e);let t5f: f64 = (t5e * (nv5 - nv3));(l.f569, l.f56a, l.f56b, l.f56c, l.f56d, l.f56e, l.f56f, ) = (t5f, (((l.fc6 * l.f5f) * (nv5 - nv3)) + (-t5e)), ((l.fc6 * l.f60) * (nv5 - nv3)), (((l.fc6 * l.f61) * (nv5 - nv3)) + t5e), ((l.fc6 * l.f62) * (nv5 - nv3)), ((l.fc6 * l.f63) * (nv5 - nv3)), ((l.fc6 * l.f64) * (nv5 - nv3)), );l.f570 = 0.0;let t60: f64 = (l.f21 * l.f3f5);let t61: f64 = (l.f1f + t60);let t62: f64 = (t61 / l.f3f5);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t62, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f6a7 = 0.0;let t63: f64 = if ((l.f6a0 <= 0.0) || (l.f36 <= 0.0)) { 1.0 } else { 0.0 };l.f26c = t63;l.f26d = 0.0;let t64: f64 = (l.f36 / 80.0);let t65: f64 = if l.feb > t64 { 1.0 } else { 0.0 };l.f26e = t65;l.f26f = 0.0;
        if ((l.f26c == 0.0) && (l.f26e != 0.0)) {let t66: f64 = (-l.f36);let t67: f64 = (t66 / l.feb);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t67, (-((t66 * l.fec) / (l.feb * l.feb))), ((((-l.f37) * l.feb) - (t66 * l.fed)) / (l.feb * l.feb)), (-((t66 * l.fee) / (l.feb * l.feb))), (-((t66 * l.fef) / (l.feb * l.feb))), (-((t66 * l.ff0) / (l.feb * l.feb))), (-((t66 * l.ff1) / (l.feb * l.feb))), );l.f6cf = 0.0;}
        let t68: f64 = if p.p17 != 0.0 { 1.0 } else { 0.0 };l.f270 = t68;l.f271 = 0.0;
        if (l.f270 != 0.0) {let t69: f64 = (l.f5c1 - l.f1d5);let t6a: f64 = (t69 / l.f450);let t6b: f64 = (t6a / l.f937);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t6b, ((l.f5ca / l.f450) / l.f937), ((((l.f5cb / l.f450) * l.f937) - (t6a * l.f938)) / (l.f937 * l.f937)), ((l.f5cc / l.f450) / l.f937), ((l.f5cd / l.f450) / l.f937), ((l.f5ce / l.f450) / l.f937), ((l.f5cf / l.f450) / l.f937), );l.f6cf = 0.0;let t6c: f64 = (l.f4c * l.f5c1);let t6d: f64 = (l.f17 - t6c);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (t6d, (-(l.f4c * l.f5ca)), (-(l.f4c * l.f5cb)), (-(l.f4c * l.f5cc)), (-(l.f4c * l.f5cd)), (-(l.f4c * l.f5ce)), (-(l.f4c * l.f5cf)), );l.f6ff = 0.0;let t6e: f64 = (l.f70 * l.f5c1);let t6f: f64 = (1.0 + t6e);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (t6f, (l.f70 * l.f5ca), (l.f70 * l.f5cb), (l.f70 * l.f5cc), (l.f70 * l.f5cd), (l.f70 * l.f5ce), (l.f70 * l.f5cf), );l.f72f = 0.0;let t70: f64 = (-982222000000.0);let t71: f64 = (t70 * p.p99);let t72: f64 = (t71 * l.f6d0);let t73: f64 = (t72 * l.f700);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (t73, (((t71 * l.f6f9) * l.f700) + (t72 * l.f729)), (((t71 * l.f6fa) * l.f700) + (t72 * l.f72a)), (((t71 * l.f6fb) * l.f700) + (t72 * l.f72b)), (((t71 * l.f6fc) * l.f700) + (t72 * l.f72c)), (((t71 * l.f6fd) * l.f700) + (t72 * l.f72d)), (((t71 * l.f6fe) * l.f700) + (t72 * l.f72e)), );l.f73f = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f270 != 0.0) {let t74: f64 = { let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (t74, ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f739), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73a), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73b), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73c), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73d), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73e), );l.f747 = 0.0;}
        if (l.f270 != 0.0) {(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (3.75956e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f74f = 0.0;let t75: f64 = (l.fb8 - l.f4a3);(l.f892, l.f893, l.f894, l.f895, l.f896, l.f897, l.f898, ) = (t75, (-l.f4a4), (l.fb9 - l.f4a5), (-l.f4a6), (-l.f4a7), (-l.f4a8), (-l.f4a9), );l.f899 = 0.0;let t76: f64 = (l.f892 - l.f8bf);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t76, (l.f893 - l.f8c0), l.f894, l.f895, l.f896, l.f897, (l.f898 - l.f8c1), );l.f6a7 = 0.0;let t77: f64 = (l.f6a0 / l.f44e);let t78: f64 = (t77 / l.f937);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t78, ((l.f6a1 / l.f44e) / l.f937), ((((l.f6a2 / l.f44e) * l.f937) - (t77 * l.f938)) / (l.f937 * l.f937)), ((l.f6a3 / l.f44e) / l.f937), ((l.f6a4 / l.f44e) / l.f937), ((l.f6a5 / l.f44e) / l.f937), ((l.f6a6 / l.f44e) / l.f937), );l.f6cf = 0.0;}
        let t79: f64 = if l.f892 <= 0.0 { 1.0 } else { 0.0 };l.f272 = t79;l.f273 = 0.0;
        if ((l.f270 != 0.0) && (l.f272 != 0.0)) {let t7a: f64 = (l.f6a0 - 0.02);let t7b: f64 = (l.f6a0 - 0.02);let t7c: f64 = (l.f6a0 - 0.02);let t7d: f64 = (t7b * t7c);let t7e: f64 = (0.08 * l.f892);let t7f: f64 = (t7d - t7e);let t80: f64 = (t7f).sqrt();let t81: f64 = (t7a + t80);let t82: f64 = (0.5 * t81);(l.f911, l.f912, l.f913, l.f914, l.f915, l.f916, l.f917, ) = (t82, (0.5 * (l.f6a1 + ((((l.f6a1 * t7c) + (t7b * l.f6a1)) - (0.08 * l.f893)) / (2.0 * t80)))), (0.5 * (l.f6a2 + ((((l.f6a2 * t7c) + (t7b * l.f6a2)) - (0.08 * l.f894)) / (2.0 * t80)))), (0.5 * (l.f6a3 + ((((l.f6a3 * t7c) + (t7b * l.f6a3)) - (0.08 * l.f895)) / (2.0 * t80)))), (0.5 * (l.f6a4 + ((((l.f6a4 * t7c) + (t7b * l.f6a4)) - (0.08 * l.f896)) / (2.0 * t80)))), (0.5 * (l.f6a5 + ((((l.f6a5 * t7c) + (t7b * l.f6a5)) - (0.08 * l.f897)) / (2.0 * t80)))), (0.5 * (l.f6a6 + ((((l.f6a6 * t7c) + (t7b * l.f6a6)) - (0.08 * l.f898)) / (2.0 * t80)))), );l.f918 = 0.0;}
        if ((l.f270 != 0.0) && (l.f272 == 0.0)) {let t83: f64 = (l.f6a0 - 0.02);let t84: f64 = (l.f6a0 - 0.02);let t85: f64 = (l.f6a0 - 0.02);let t86: f64 = (t84 * t85);let t87: f64 = (0.08 * l.f892);let t88: f64 = (t86 + t87);let t89: f64 = (t88).sqrt();let t8a: f64 = (t83 + t89);let t8b: f64 = (0.5 * t8a);(l.f911, l.f912, l.f913, l.f914, l.f915, l.f916, l.f917, ) = (t8b, (0.5 * (l.f6a1 + ((((l.f6a1 * t85) + (t84 * l.f6a1)) + (0.08 * l.f893)) / (2.0 * t89)))), (0.5 * (l.f6a2 + ((((l.f6a2 * t85) + (t84 * l.f6a2)) + (0.08 * l.f894)) / (2.0 * t89)))), (0.5 * (l.f6a3 + ((((l.f6a3 * t85) + (t84 * l.f6a3)) + (0.08 * l.f895)) / (2.0 * t89)))), (0.5 * (l.f6a4 + ((((l.f6a4 * t85) + (t84 * l.f6a4)) + (0.08 * l.f896)) / (2.0 * t89)))), (0.5 * (l.f6a5 + ((((l.f6a5 * t85) + (t84 * l.f6a5)) + (0.08 * l.f897)) / (2.0 * t89)))), (0.5 * (l.f6a6 + ((((l.f6a6 * t85) + (t84 * l.f6a6)) + (0.08 * l.f898)) / (2.0 * t89)))), );l.f918 = 0.0;}
        if (l.f270 != 0.0) {let t8c: f64 = (l.f4a * l.f911);let t8d: f64 = (l.f15 - t8c);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (t8d, (-(l.f4a * l.f912)), (-(l.f4a * l.f913)), (-(l.f4a * l.f914)), (-(l.f4a * l.f915)), (-(l.f4a * l.f916)), (-(l.f4a * l.f917)), );l.f6ff = 0.0;let t8e: f64 = (l.f6e * l.f911);let t8f: f64 = (1.0 + t8e);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (t8f, (l.f6e * l.f912), (l.f6e * l.f913), (l.f6e * l.f914), (l.f6e * l.f915), (l.f6e * l.f916), (l.f6e * l.f917), );l.f72f = 0.0;let t90: f64 = (-745669000000.0);let t91: f64 = (t90 * p.p99);let t92: f64 = (t91 * l.f6d0);let t93: f64 = (t92 * l.f700);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (t93, (((t91 * l.f6f9) * l.f700) + (t92 * l.f729)), (((t91 * l.f6fa) * l.f700) + (t92 * l.f72a)), (((t91 * l.f6fb) * l.f700) + (t92 * l.f72b)), (((t91 * l.f6fc) * l.f700) + (t92 * l.f72c)), (((t91 * l.f6fd) * l.f700) + (t92 * l.f72d)), (((t91 * l.f6fe) * l.f700) + (t92 * l.f72e)), );l.f73f = 0.0;}
        if (l.f270 != 0.0) {let t94: f64 = { let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (t94, ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f739), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73a), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73b), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73c), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73d), ({ let limited_exp_arg = l.f730; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f73e), );l.f747 = 0.0;}
        if (l.f270 != 0.0) {(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (4.97232e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f74f = 0.0;}
        let t95: f64 = (0.6 * l.f861);let t96: f64 = (t95 / l.f937);let t97: f64 = (t96).tanh();(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t97, 0.0, ((-((t95 * l.f938) / (l.f937 * l.f937))) / ((t96).cosh() * (t96).cosh())), (((0.6 * l.f862) / l.f937) / ((t96).cosh() * (t96).cosh())), (((0.6 * l.f863) / l.f937) / ((t96).cosh() * (t96).cosh())), 0.0, 0.0, );l.f6a7 = 0.0;let t98: f64 = if p.p16 != 0.0 { 1.0 } else { 0.0 };l.f274 = t98;l.f275 = 0.0;
        if (l.f274 != 0.0) {let t99: f64 = (l.ff3 * l.f4ab);let t9a: f64 = (l.f8d3 - t99);let t9b: f64 = (l.f4e * t9a);let t9c: f64 = (l.f19 - t9b);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t9c, (-(l.f4e * (-(l.ff3 * l.f4ac)))), (-(l.f4e * (l.f8d4 - (l.ff3 * l.f4ad)))), (-(l.f4e * (l.f8d5 - (l.ff3 * l.f4ae)))), (-(l.f4e * (l.f8d6 - (l.ff3 * l.f4af)))), (-(l.f4e * (-(l.ff3 * l.f4b0)))), (-(l.f4e * (l.f8d7 - (l.ff3 * l.f4b1)))), );l.f6cf = 0.0;let t9d: f64 = (l.ff3 * l.f4ab);let t9e: f64 = (l.f8d3 - t9d);let t9f: f64 = (l.f72 * t9e);let ta0: f64 = (1.0 + t9f);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (ta0, (l.f72 * (-(l.ff3 * l.f4ac))), (l.f72 * (l.f8d4 - (l.ff3 * l.f4ad))), (l.f72 * (l.f8d5 - (l.ff3 * l.f4ae))), (l.f72 * (l.f8d6 - (l.ff3 * l.f4af))), (l.f72 * (-(l.ff3 * l.f4b0))), (l.f72 * (l.f8d7 - (l.ff3 * l.f4b1))), );l.f6ff = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f274 != 0.0) {let ta1: f64 = (-l.f31);let ta2: f64 = (ta1 * p.p99);let ta3: f64 = (ta2 * l.f6a8);let ta4: f64 = (ta3 * l.f6d0);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (ta4, (((ta2 * l.f6c1) * l.f6d0) + (ta3 * l.f6f9)), (((ta2 * l.f6c2) * l.f6d0) + (ta3 * l.f6fa)), (((ta2 * l.f6c3) * l.f6d0) + (ta3 * l.f6fb)), (((ta2 * l.f6c4) * l.f6d0) + (ta3 * l.f6fc)), (((ta2 * l.f6c5) * l.f6d0) + (ta3 * l.f6fd)), (((ta2 * l.f6c6) * l.f6d0) + (ta3 * l.f6fe)), );l.f72f = 0.0;}
        if (l.f274 != 0.0) {let ta5: f64 = { let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let ta6: f64 = (l.f5c1 * ta5);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (ta6, ((l.f5ca * ta5) + (l.f5c1 * ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f729))), ((l.f5cb * ta5) + (l.f5c1 * ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72a))), ((l.f5cc * ta5) + (l.f5c1 * ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72b))), ((l.f5cd * ta5) + (l.f5c1 * ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72c))), ((l.f5ce * ta5) + (l.f5c1 * ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72d))), ((l.f5cf * ta5) + (l.f5c1 * ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72e))), );l.f73f = 0.0;}
        if (l.f274 != 0.0) {let ta7: f64 = (0.5 * l.f87e);let ta8: f64 = (l.f8bf + ta7);let ta9: f64 = (l.f847 + l.f83a);let taa: f64 = (0.5 * ta9);let tab: f64 = (ta8 + taa);(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (tab, (l.f8c0 + (0.5 * (l.f848 + l.f83b))), 0.0, ((0.5 * l.f87f) + (0.5 * l.f83c)), ((0.5 * l.f880) + (0.5 * l.f849)), 0.0, l.f8c1, );l.f747 = 0.0;let tac: f64 = (l.f86e * l.f86e);let tad: f64 = (tac + 0.01);let tae: f64 = (tad).sqrt();let taf: f64 = (tae - 0.1);(l.f876, l.f877, l.f878, l.f879, l.f87a, l.f87b, l.f87c, ) = (taf, (((l.f86f * l.f86e) + (l.f86e * l.f86f)) / (2.0 * tae)), (((l.f870 * l.f86e) + (l.f86e * l.f870)) / (2.0 * tae)), (((l.f871 * l.f86e) + (l.f86e * l.f871)) / (2.0 * tae)), (((l.f872 * l.f86e) + (l.f86e * l.f872)) / (2.0 * tae)), (((l.f873 * l.f86e) + (l.f86e * l.f873)) / (2.0 * tae)), (((l.f874 * l.f86e) + (l.f86e * l.f874)) / (2.0 * tae)), );l.f87d = 0.0;let tb0: f64 = (l.f4f2 * l.f876);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (tb0, (l.f4f2 * l.f877), (l.f4f2 * l.f878), (l.f4f2 * l.f879), (l.f4f2 * l.f87a), (l.f4f2 * l.f87b), (l.f4f2 * l.f87c), );l.f6cf = 0.0;}
        if (l.f274 != 0.0) {let tb1: f64 = (-l.f6a8);let tb2: f64 = { let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f6c7, l.f6c8, l.f6c9, l.f6ca, l.f6cb, l.f6cc, l.f6cd, ) = (tb2, ({ let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6c1)), ({ let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6c2)), ({ let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6c3)), ({ let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6c4)), ({ let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6c5)), ({ let limited_exp_arg = tb1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6c6)), );l.f6ce = 0.0;}
        if (l.f274 != 0.0) {let tb3: f64 = (l.f6a8 + l.f6c7);let tb4: f64 = (tb3 - 1.0);let tb5: f64 = (tb4 + 0.0001);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (tb5, (l.f6c1 + l.f6c8), (l.f6c2 + l.f6c9), (l.f6c3 + l.f6ca), (l.f6c4 + l.f6cb), (l.f6c5 + l.f6cc), (l.f6c6 + l.f6cd), );l.f72f = 0.0;let tb6: f64 = (l.f6a8 + 1.0);let tb7: f64 = (tb6 * l.f6c7);let tb8: f64 = (1.0 - tb7);let tb9: f64 = (tb8 + 0.0001);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (tb9, (-((l.f6c1 * l.f6c7) + (tb6 * l.f6c8))), (-((l.f6c2 * l.f6c7) + (tb6 * l.f6c9))), (-((l.f6c3 * l.f6c7) + (tb6 * l.f6ca))), (-((l.f6c4 * l.f6c7) + (tb6 * l.f6cb))), (-((l.f6c5 * l.f6c7) + (tb6 * l.f6cc))), (-((l.f6c6 * l.f6c7) + (tb6 * l.f6cd))), );l.f73f = 0.0;let tba: f64 = (l.f6a8 * l.f6a8);let tbb: f64 = (tba + 0.0002);(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (tbb, ((l.f6c1 * l.f6a8) + (l.f6a8 * l.f6c1)), ((l.f6c2 * l.f6a8) + (l.f6a8 * l.f6c2)), ((l.f6c3 * l.f6a8) + (l.f6a8 * l.f6c3)), ((l.f6c4 * l.f6a8) + (l.f6a8 * l.f6c4)), ((l.f6c5 * l.f6a8) + (l.f6a8 * l.f6c5)), ((l.f6c6 * l.f6a8) + (l.f6a8 * l.f6c6)), );l.f747 = 0.0;let tbc: f64 = (l.f8f5 - l.f882);let tbd: f64 = (l.ff7 * l.f240);let tbe: f64 = (l.f843 - l.f883);let tbf: f64 = (tbd * tbe);let tc0: f64 = (tbc + tbf);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (tc0, ((-l.f88b) + (tbd * (l.f844 - l.f884))), ((-l.f88c) + (tbd * (-l.f885))), ((-l.f88d) + (tbd * (l.f845 - l.f886))), ((l.f8f6 - l.f88e) + (tbd * (l.f846 - l.f887))), ((-l.f88f) + (tbd * (-l.f888))), ((l.f8f7 - l.f890) + (tbd * (-l.f889))), );l.f6a7 = 0.0;let tc1: f64 = (l.f6a0 * l.f6a0);let tc2: f64 = (tc1 + 0.0001);let tc3: f64 = (tc2).sqrt();(l.f8ae, l.f8af, l.f8b0, l.f8b1, l.f8b2, l.f8b3, l.f8b4, ) = (tc3, (((l.f6a1 * l.f6a0) + (l.f6a0 * l.f6a1)) / (2.0 * tc3)), (((l.f6a2 * l.f6a0) + (l.f6a0 * l.f6a2)) / (2.0 * tc3)), (((l.f6a3 * l.f6a0) + (l.f6a0 * l.f6a3)) / (2.0 * tc3)), (((l.f6a4 * l.f6a0) + (l.f6a0 * l.f6a4)) / (2.0 * tc3)), (((l.f6a5 * l.f6a0) + (l.f6a0 * l.f6a5)) / (2.0 * tc3)), (((l.f6a6 * l.f6a0) + (l.f6a0 * l.f6a6)) / (2.0 * tc3)), );l.f8b5 = 0.0;let tc4: f64 = (l.f52 * l.f8ae);let tc5: f64 = (l.f1d - tc4);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (tc5, (-(l.f52 * l.f8af)), (-(l.f52 * l.f8b0)), (-(l.f52 * l.f8b1)), (-(l.f52 * l.f8b2)), (-(l.f52 * l.f8b3)), (-(l.f52 * l.f8b4)), );l.f6cf = 0.0;let tc6: f64 = (l.f76 * l.f8ae);let tc7: f64 = (1.0 + tc6);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (tc7, (l.f76 * l.f8af), (l.f76 * l.f8b0), (l.f76 * l.f8b1), (l.f76 * l.f8b2), (l.f76 * l.f8b3), (l.f76 * l.f8b4), );l.f6ff = 0.0;}
        if (l.f274 != 0.0) {let tc8: f64 = (-l.f31);let tc9: f64 = (tc8 * p.p99);let tca: f64 = (tc9 * l.f4f4);let tcb: f64 = (tca * l.f6a8);let tcc: f64 = (tcb * l.f6d0);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (tcc, (((tca * l.f6c1) * l.f6d0) + (tcb * l.f6f9)), (((tca * l.f6c2) * l.f6d0) + (tcb * l.f6fa)), (((tca * l.f6c3) * l.f6d0) + (tcb * l.f6fb)), (((tca * l.f6c4) * l.f6d0) + (tcb * l.f6fc)), (((tca * l.f6c5) * l.f6d0) + (tcb * l.f6fd)), (((tca * l.f6c6) * l.f6d0) + (tcb * l.f6fe)), );l.f72f = 0.0;}
        if (l.f274 != 0.0) {let tcd: f64 = { let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (tcd, ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f729), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72a), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72b), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72c), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72d), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72e), );l.f73f = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f274 != 0.0) {let tce: f64 = (l.f8cb - l.f882);let tcf: f64 = (l.ff5 * l.f240);let td0: f64 = (l.f843 - l.f883);let td1: f64 = (tcf * td0);let td2: f64 = (tce + td1);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (td2, ((-l.f88b) + (tcf * (l.f844 - l.f884))), ((-l.f88c) + (tcf * (-l.f885))), ((l.f8cc - l.f88d) + (tcf * (l.f845 - l.f886))), ((-l.f88e) + (tcf * (l.f846 - l.f887))), ((-l.f88f) + (tcf * (-l.f888))), ((l.f8cd - l.f890) + (tcf * (-l.f889))), );l.f6a7 = 0.0;let td3: f64 = (l.f6a0 * l.f6a0);let td4: f64 = (td3 + 0.0001);let td5: f64 = (td4).sqrt();(l.f89a, l.f89b, l.f89c, l.f89d, l.f89e, l.f89f, l.f8a0, ) = (td5, (((l.f6a1 * l.f6a0) + (l.f6a0 * l.f6a1)) / (2.0 * td5)), (((l.f6a2 * l.f6a0) + (l.f6a0 * l.f6a2)) / (2.0 * td5)), (((l.f6a3 * l.f6a0) + (l.f6a0 * l.f6a3)) / (2.0 * td5)), (((l.f6a4 * l.f6a0) + (l.f6a0 * l.f6a4)) / (2.0 * td5)), (((l.f6a5 * l.f6a0) + (l.f6a0 * l.f6a5)) / (2.0 * td5)), (((l.f6a6 * l.f6a0) + (l.f6a0 * l.f6a6)) / (2.0 * td5)), );l.f8a1 = 0.0;let td6: f64 = (l.f50 * l.f89a);let td7: f64 = (l.f1b - td6);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (td7, (-(l.f50 * l.f89b)), (-(l.f50 * l.f89c)), (-(l.f50 * l.f89d)), (-(l.f50 * l.f89e)), (-(l.f50 * l.f89f)), (-(l.f50 * l.f8a0)), );l.f6cf = 0.0;let td8: f64 = (l.f74 * l.f89a);let td9: f64 = (1.0 + td8);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (td9, (l.f74 * l.f89b), (l.f74 * l.f89c), (l.f74 * l.f89d), (l.f74 * l.f89e), (l.f74 * l.f89f), (l.f74 * l.f8a0), );l.f6ff = 0.0;let tda: f64 = (-l.f31);let tdb: f64 = (tda * p.p99);let tdc: f64 = (tdb * l.f4f4);let tdd: f64 = (tdc * l.f6a8);let tde: f64 = (tdd * l.f6d0);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (tde, (((tdc * l.f6c1) * l.f6d0) + (tdd * l.f6f9)), (((tdc * l.f6c2) * l.f6d0) + (tdd * l.f6fa)), (((tdc * l.f6c3) * l.f6d0) + (tdd * l.f6fb)), (((tdc * l.f6c4) * l.f6d0) + (tdd * l.f6fc)), (((tdc * l.f6c5) * l.f6d0) + (tdd * l.f6fd)), (((tdc * l.f6c6) * l.f6d0) + (tdd * l.f6fe)), );l.f72f = 0.0;}
        if (l.f274 != 0.0) {let tdf: f64 = { let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (tdf, ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f729), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72a), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72b), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72c), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72d), ({ let limited_exp_arg = l.f700; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * l.f72e), );l.f73f = 0.0;}
        let te0: f64 = if p.p15 != 0.0 { 1.0 } else { 0.0 };l.f278 = te0;l.f279 = 0.0;
        if (l.f278 != 0.0) {let te1: f64 = (l.f1d7 * p.p45);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (te1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f6a7 = 0.0;}
        let te2: f64 = if ((l.f11 <= 0.0) || (l.f42 <= 0.0)) { 1.0 } else { 0.0 };l.f27a = te2;l.f27b = 0.0;
        if ((l.f278 != 0.0) && (l.f27a != 0.0)) {(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f74f = 0.0;}
        if ((l.f278 != 0.0) && (l.f27a == 0.0)) {let te3: f64 = (-l.f8cb);let te4: f64 = (te3 - l.f1d1);let te5: f64 = (te4 + l.f882);let te6: f64 = (l.f83f * l.f240);let te7: f64 = (l.f843 - l.f883);let te8: f64 = (te7 - l.f832);let te9: f64 = (te6 * te8);let tea: f64 = (te5 + te9);let teb: f64 = (tea / l.f6a0);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (teb, ((((l.f88b + (te6 * (l.f844 - l.f884))) * l.f6a0) - (tea * l.f6a1)) / (l.f6a0 * l.f6a0)), ((((l.f88c + (te6 * (-l.f885))) * l.f6a0) - (tea * l.f6a2)) / (l.f6a0 * l.f6a0)), ((((((-l.f8cc) + l.f88d) + (te6 * (l.f845 - l.f886))) * l.f6a0) - (tea * l.f6a3)) / (l.f6a0 * l.f6a0)), ((((l.f88e + (te6 * (l.f846 - l.f887))) * l.f6a0) - (tea * l.f6a4)) / (l.f6a0 * l.f6a0)), ((((l.f88f + (te6 * (-l.f888))) * l.f6a0) - (tea * l.f6a5)) / (l.f6a0 * l.f6a0)), ((((((-l.f8cd) + l.f890) + (te6 * (-l.f889))) * l.f6a0) - (tea * l.f6a6)) / (l.f6a0 * l.f6a0)), );l.f6cf = 0.0;let tec: f64 = (l.f6a8 * l.f6a8);let ted: f64 = (4.0 * 0.01);let tee: f64 = (ted * 0.01);let tef: f64 = (tec + tee);let tf0: f64 = (tef).sqrt();let tf1: f64 = (l.f6a8 + tf0);let tf2: f64 = (0.5 * tf1);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (tf2, (0.5 * (l.f6c1 + (((l.f6c1 * l.f6a8) + (l.f6a8 * l.f6c1)) / (2.0 * tf0)))), (0.5 * (l.f6c2 + (((l.f6c2 * l.f6a8) + (l.f6a8 * l.f6c2)) / (2.0 * tf0)))), (0.5 * (l.f6c3 + (((l.f6c3 * l.f6a8) + (l.f6a8 * l.f6c3)) / (2.0 * tf0)))), (0.5 * (l.f6c4 + (((l.f6c4 * l.f6a8) + (l.f6a8 * l.f6c4)) / (2.0 * tf0)))), (0.5 * (l.f6c5 + (((l.f6c5 * l.f6a8) + (l.f6a8 * l.f6c5)) / (2.0 * tf0)))), (0.5 * (l.f6c6 + (((l.f6c6 * l.f6a8) + (l.f6a8 * l.f6c6)) / (2.0 * tf0)))), );l.f6cf = 0.0;let tf3: f64 = (l.f6a8 + 0.001);let tf4: f64 = (l.f42 / tf3);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (tf4, (-((l.f42 * l.f6c1) / (tf3 * tf3))), (((l.f43 * tf3) - (l.f42 * l.f6c2)) / (tf3 * tf3)), (-((l.f42 * l.f6c3) / (tf3 * tf3))), (-((l.f42 * l.f6c4) / (tf3 * tf3))), (-((l.f42 * l.f6c5) / (tf3 * tf3))), (-((l.f42 * l.f6c6) / (tf3 * tf3))), );l.f6ff = 0.0;}
        if ((l.f278 != 0.0) && (l.f27a == 0.0)) {let tf5: f64 = (l.f6a8).max(1e-38);let tf6: f64 = (tf5).ln();let tf7: f64 = (l.f47f * tf6);let tf8: f64 = { let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (tf8, ({ let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f47f * (if l.f6a8 >= 1e-38 { l.f6c1 } else { 0.0 } / tf5))), ({ let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f47f * (if l.f6a8 >= 1e-38 { l.f6c2 } else { 0.0 } / tf5))), ({ let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f47f * (if l.f6a8 >= 1e-38 { l.f6c3 } else { 0.0 } / tf5))), ({ let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f47f * (if l.f6a8 >= 1e-38 { l.f6c4 } else { 0.0 } / tf5))), ({ let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f47f * (if l.f6a8 >= 1e-38 { l.f6c5 } else { 0.0 } / tf5))), ({ let limited_exp_arg = tf7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f47f * (if l.f6a8 >= 1e-38 { l.f6c6 } else { 0.0 } / tf5))), );l.f72f = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f278 != 0.0) && (l.f27a == 0.0)) {let tf9: f64 = (l.f11 * l.f94e);let tfa: f64 = (tf9 * l.f700);let tfb: f64 = (-l.f6d0);let tfc: f64 = { let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let tfd: f64 = (tfa * tfc);let tfe: f64 = (tfd * l.f861);(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (tfe, ((((tf9 * l.f729) * tfc) + (tfa * ({ let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6f9)))) * l.f861), ((((tf9 * l.f72a) * tfc) + (tfa * ({ let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fa)))) * l.f861), (((((tf9 * l.f72b) * tfc) + (tfa * ({ let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fb)))) * l.f861) + (tfd * l.f862)), (((((tf9 * l.f72c) * tfc) + (tfa * ({ let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fc)))) * l.f861) + (tfd * l.f863)), ((((tf9 * l.f72d) * tfc) + (tfa * ({ let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fd)))) * l.f861), ((((tf9 * l.f72e) * tfc) + (tfa * ({ let limited_exp_arg = tfb; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fe)))) * l.f861), );l.f74f = 0.0;}
        let tff: f64 = if ((l.f13 <= 0.0) || (l.f47 <= 0.0)) { 1.0 } else { 0.0 };l.f27d = tff;l.f27e = 0.0;
        if ((l.f278 != 0.0) && (l.f27d != 0.0)) {(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f74f = 0.0;}
        if ((l.f278 != 0.0) && (l.f27d == 0.0)) {let t100: f64 = (-l.f8f5);let t101: f64 = (t100 - l.f1d3);let t102: f64 = (t101 + l.f882);let t103: f64 = (l.f841 * l.f240);let t104: f64 = (l.f843 - l.f883);let t105: f64 = (t104 - l.f834);let t106: f64 = (t103 * t105);let t107: f64 = (t102 + t106);let t108: f64 = (t107 / l.f6a0);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t108, ((((l.f88b + (t103 * (l.f844 - l.f884))) * l.f6a0) - (t107 * l.f6a1)) / (l.f6a0 * l.f6a0)), ((((l.f88c + (t103 * (-l.f885))) * l.f6a0) - (t107 * l.f6a2)) / (l.f6a0 * l.f6a0)), ((((l.f88d + (t103 * (l.f845 - l.f886))) * l.f6a0) - (t107 * l.f6a3)) / (l.f6a0 * l.f6a0)), ((((((-l.f8f6) + l.f88e) + (t103 * (l.f846 - l.f887))) * l.f6a0) - (t107 * l.f6a4)) / (l.f6a0 * l.f6a0)), ((((l.f88f + (t103 * (-l.f888))) * l.f6a0) - (t107 * l.f6a5)) / (l.f6a0 * l.f6a0)), ((((((-l.f8f7) + l.f890) + (t103 * (-l.f889))) * l.f6a0) - (t107 * l.f6a6)) / (l.f6a0 * l.f6a0)), );l.f6cf = 0.0;let t109: f64 = (l.f6a8 * l.f6a8);let t10a: f64 = (4.0 * 0.01);let t10b: f64 = (t10a * 0.01);let t10c: f64 = (t109 + t10b);let t10d: f64 = (t10c).sqrt();let t10e: f64 = (l.f6a8 + t10d);let t10f: f64 = (0.5 * t10e);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t10f, (0.5 * (l.f6c1 + (((l.f6c1 * l.f6a8) + (l.f6a8 * l.f6c1)) / (2.0 * t10d)))), (0.5 * (l.f6c2 + (((l.f6c2 * l.f6a8) + (l.f6a8 * l.f6c2)) / (2.0 * t10d)))), (0.5 * (l.f6c3 + (((l.f6c3 * l.f6a8) + (l.f6a8 * l.f6c3)) / (2.0 * t10d)))), (0.5 * (l.f6c4 + (((l.f6c4 * l.f6a8) + (l.f6a8 * l.f6c4)) / (2.0 * t10d)))), (0.5 * (l.f6c5 + (((l.f6c5 * l.f6a8) + (l.f6a8 * l.f6c5)) / (2.0 * t10d)))), (0.5 * (l.f6c6 + (((l.f6c6 * l.f6a8) + (l.f6a8 * l.f6c6)) / (2.0 * t10d)))), );l.f6cf = 0.0;let t110: f64 = (l.f6a8 + 0.001);let t111: f64 = (l.f47 / t110);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (t111, (-((l.f47 * l.f6c1) / (t110 * t110))), (((l.f48 * t110) - (l.f47 * l.f6c2)) / (t110 * t110)), (-((l.f47 * l.f6c3) / (t110 * t110))), (-((l.f47 * l.f6c4) / (t110 * t110))), (-((l.f47 * l.f6c5) / (t110 * t110))), (-((l.f47 * l.f6c6) / (t110 * t110))), );l.f6ff = 0.0;}
        if ((l.f278 != 0.0) && (l.f27d == 0.0)) {let t112: f64 = (l.f6a8).max(1e-38);let t113: f64 = (t112).ln();let t114: f64 = (l.f481 * t113);let t115: f64 = { let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (t115, ({ let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f481 * (if l.f6a8 >= 1e-38 { l.f6c1 } else { 0.0 } / t112))), ({ let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f481 * (if l.f6a8 >= 1e-38 { l.f6c2 } else { 0.0 } / t112))), ({ let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f481 * (if l.f6a8 >= 1e-38 { l.f6c3 } else { 0.0 } / t112))), ({ let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f481 * (if l.f6a8 >= 1e-38 { l.f6c4 } else { 0.0 } / t112))), ({ let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f481 * (if l.f6a8 >= 1e-38 { l.f6c5 } else { 0.0 } / t112))), ({ let limited_exp_arg = t114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (l.f481 * (if l.f6a8 >= 1e-38 { l.f6c6 } else { 0.0 } / t112))), );l.f72f = 0.0;}
        if ((l.f278 != 0.0) && (l.f27d == 0.0)) {let t116: f64 = (-l.f861);let t117: f64 = (t116 * l.f13);let t118: f64 = (t117 * l.f94e);let t119: f64 = (t118 * l.f700);let t11a: f64 = (-l.f6d0);let t11b: f64 = { let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let t11c: f64 = (t119 * t11b);(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (t11c, (((t118 * l.f729) * t11b) + (t119 * ({ let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6f9)))), (((t118 * l.f72a) * t11b) + (t119 * ({ let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fa)))), (((((((-l.f862) * l.f13) * l.f94e) * l.f700) + (t118 * l.f72b)) * t11b) + (t119 * ({ let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fb)))), (((((((-l.f863) * l.f13) * l.f94e) * l.f700) + (t118 * l.f72c)) * t11b) + (t119 * ({ let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fc)))), (((t118 * l.f72d) * t11b) + (t119 * ({ let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fd)))), (((t118 * l.f72e) * t11b) + (t119 * ({ let limited_exp_arg = t11a; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-l.f6fe)))), );l.f74f = 0.0;}
        let t11d: f64 = (2.0 * l.f920);let t11e: f64 = (t11d / l.f814);(l.f20b, l.f20c, l.f20d, l.f20e, l.f20f, l.f210, l.f211, ) = (t11e, (-((t11d * l.f815) / (l.f814 * l.f814))), ((((2.0 * l.f921) * l.f814) - (t11d * l.f816)) / (l.f814 * l.f814)), (-((t11d * l.f817) / (l.f814 * l.f814))), (-((t11d * l.f818) / (l.f814 * l.f814))), (-((t11d * l.f819) / (l.f814 * l.f814))), (-((t11d * l.f81a) / (l.f814 * l.f814))), );l.f212 = 0.0;let t11f: f64 = if (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0)) { 1.0 } else { 0.0 };l.f280 = t11f;l.f281 = 0.0;
        if (l.f280 != 0.0) {let t120: f64 = (2.0 * l.f3fd);let t121: f64 = (l.f3f5 - t120);l.f3f9 = t121;l.f3fa = 0.0;let t122: f64 = (l.f3f9 * l.f3f9);l.f3fb = t122;l.f3fc = 0.0;}
        let t123: f64 = if p.p287 <= 0.0 { 1.0 } else { 0.0 };l.f282 = t123;l.f283 = 0.0;
        if ((l.f280 != 0.0) && (l.f282 != 0.0)) {(l.fa8, l.fa9, l.faa, l.fab, l.fac, l.fad, l.fae, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.faf = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f280 != 0.0) && (l.f282 == 0.0)) {let t124: f64 = (l.feb / l.f3ff);let t125: f64 = (t124 + p.p287);let t126: f64 = (t125 / l.f20b);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t126, ((((l.fec / l.f3ff) * l.f20b) - (t125 * l.f20c)) / (l.f20b * l.f20b)), ((((l.fed / l.f3ff) * l.f20b) - (t125 * l.f20d)) / (l.f20b * l.f20b)), ((((l.fee / l.f3ff) * l.f20b) - (t125 * l.f20e)) / (l.f20b * l.f20b)), ((((l.fef / l.f3ff) * l.f20b) - (t125 * l.f20f)) / (l.f20b * l.f20b)), ((((l.ff0 / l.f3ff) * l.f20b) - (t125 * l.f210)) / (l.f20b * l.f20b)), ((((l.ff1 / l.f3ff) * l.f20b) - (t125 * l.f211)) / (l.f20b * l.f20b)), );l.f6a7 = 0.0;}
        if ((l.f280 != 0.0) && (l.f282 == 0.0)) {let t127: f64 = (l.f6a0).max(1e-38);let t128: f64 = (t127).ln();let t129: f64 = (l.f3ff * t128);(l.fa8, l.fa9, l.faa, l.fab, l.fac, l.fad, l.fae, ) = (t129, (l.f3ff * (if l.f6a0 >= 1e-38 { l.f6a1 } else { 0.0 } / t127)), (l.f3ff * (if l.f6a0 >= 1e-38 { l.f6a2 } else { 0.0 } / t127)), (l.f3ff * (if l.f6a0 >= 1e-38 { l.f6a3 } else { 0.0 } / t127)), (l.f3ff * (if l.f6a0 >= 1e-38 { l.f6a4 } else { 0.0 } / t127)), (l.f3ff * (if l.f6a0 >= 1e-38 { l.f6a5 } else { 0.0 } / t127)), (l.f3ff * (if l.f6a0 >= 1e-38 { l.f6a6 } else { 0.0 } / t127)), );l.faf = 0.0;}
        let t12a: f64 = if l.fa8 < 0.0 { 1.0 } else { 0.0 };l.f284 = t12a;l.f285 = 0.0;
        if (((l.f280 != 0.0) && (l.f282 == 0.0)) && (l.f284 != 0.0)) {(l.fa8, l.fa9, l.faa, l.fab, l.fac, l.fad, l.fae, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.faf = 0.0;}
        let t12b: f64 = if p.p22 == 1.0 { 1.0 } else { 0.0 };l.f286 = t12b;l.f287 = 0.0;
        if ((l.f280 != 0.0) && (l.f286 != 0.0)) {let t12c: f64 = (l.f5c2 / l.f635);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t12c, (l.f5c3 / l.f635), (l.f5c4 / l.f635), (l.f5c5 / l.f635), (l.f5c6 / l.f635), (l.f5c7 / l.f635), (l.f5c8 / l.f635), );l.f6cf = 0.0;}
        if ((l.f280 != 0.0) && (l.f286 != 0.0)) {let t12d: f64 = (l.f6a8).powf(l.f43a);let t12e: f64 = (1.0 + t12d);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (t12e, if 0.0 == 0.0 && ((l.f43a) as f64).is_finite() && ((l.f43a) as f64).fract() == 0.0 { if l.f43a == 0.0 { 0.0 } else { (l.f43a * ((l.f6a8).powf(l.f43a - 1.0) * l.f6c1)) } } else { (t12d * (l.f43a * (l.f6c1 / l.f6a8))) }, if 0.0 == 0.0 && ((l.f43a) as f64).is_finite() && ((l.f43a) as f64).fract() == 0.0 { if l.f43a == 0.0 { 0.0 } else { (l.f43a * ((l.f6a8).powf(l.f43a - 1.0) * l.f6c2)) } } else { (t12d * (l.f43a * (l.f6c2 / l.f6a8))) }, if 0.0 == 0.0 && ((l.f43a) as f64).is_finite() && ((l.f43a) as f64).fract() == 0.0 { if l.f43a == 0.0 { 0.0 } else { (l.f43a * ((l.f6a8).powf(l.f43a - 1.0) * l.f6c3)) } } else { (t12d * (l.f43a * (l.f6c3 / l.f6a8))) }, if 0.0 == 0.0 && ((l.f43a) as f64).is_finite() && ((l.f43a) as f64).fract() == 0.0 { if l.f43a == 0.0 { 0.0 } else { (l.f43a * ((l.f6a8).powf(l.f43a - 1.0) * l.f6c4)) } } else { (t12d * (l.f43a * (l.f6c4 / l.f6a8))) }, if 0.0 == 0.0 && ((l.f43a) as f64).is_finite() && ((l.f43a) as f64).fract() == 0.0 { if l.f43a == 0.0 { 0.0 } else { (l.f43a * ((l.f6a8).powf(l.f43a - 1.0) * l.f6c5)) } } else { (t12d * (l.f43a * (l.f6c5 / l.f6a8))) }, if 0.0 == 0.0 && ((l.f43a) as f64).is_finite() && ((l.f43a) as f64).fract() == 0.0 { if l.f43a == 0.0 { 0.0 } else { (l.f43a * ((l.f6a8).powf(l.f43a - 1.0) * l.f6c6)) } } else { (t12d * (l.f43a * (l.f6c6 / l.f6a8))) }, );l.f6ff = 0.0;}
        if ((l.f280 != 0.0) && (l.f286 != 0.0)) {let t12f: f64 = (l.f45a / l.f6d0);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (t12f, (-((l.f45a * l.f6f9) / (l.f6d0 * l.f6d0))), (-((l.f45a * l.f6fa) / (l.f6d0 * l.f6d0))), (-((l.f45a * l.f6fb) / (l.f6d0 * l.f6d0))), (-((l.f45a * l.f6fc) / (l.f6d0 * l.f6d0))), (-((l.f45a * l.f6fd) / (l.f6d0 * l.f6d0))), (-((l.f45a * l.f6fe) / (l.f6d0 * l.f6d0))), );l.f72f = 0.0;let t130: f64 = (l.f700 / p.p288);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (t130, (l.f729 / p.p288), (l.f72a / p.p288), (l.f72b / p.p288), (l.f72c / p.p288), (l.f72d / p.p288), (l.f72e / p.p288), );l.f73f = 0.0;let t131: f64 = (l.f730 + 1.0);let t132: f64 = (l.f730 - 1.0);let t133: f64 = (l.f730 - 1.0);let t134: f64 = (t132 * t133);let t135: f64 = (0.25 * p.p292);let t136: f64 = (t135 * p.p292);let t137: f64 = (t134 + t136);let t138: f64 = (t137).sqrt();let t139: f64 = (t131 + t138);let t13a: f64 = (0.5 * t139);(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (t13a, (0.5 * (l.f739 + (((l.f739 * t133) + (t132 * l.f739)) / (2.0 * t138)))), (0.5 * (l.f73a + (((l.f73a * t133) + (t132 * l.f73a)) / (2.0 * t138)))), (0.5 * (l.f73b + (((l.f73b * t133) + (t132 * l.f73b)) / (2.0 * t138)))), (0.5 * (l.f73c + (((l.f73c * t133) + (t132 * l.f73c)) / (2.0 * t138)))), (0.5 * (l.f73d + (((l.f73d * t133) + (t132 * l.f73d)) / (2.0 * t138)))), (0.5 * (l.f73e + (((l.f73e * t133) + (t132 * l.f73e)) / (2.0 * t138)))), );l.f747 = 0.0;let t13b: f64 = (p.p288 * l.f740);(l.f45c, l.f45d, l.f45e, l.f45f, l.f460, l.f461, l.f462, ) = (t13b, (p.p288 * l.f741), (p.p288 * l.f742), (p.p288 * l.f743), (p.p288 * l.f744), (p.p288 * l.f745), (p.p288 * l.f746), );l.f463 = 0.0;}
        if ((l.f280 != 0.0) && (l.f286 == 0.0)) {(l.f45c, l.f45d, l.f45e, l.f45f, l.f460, l.f461, l.f462, ) = (p.p288, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f463 = 0.0;}
        if (l.f280 != 0.0) {let t13c: f64 = (1.60219e-19 * 1.60219e-19);let t13d: f64 = (t13c * 1.60219e-19);let t13e: f64 = (t13d * l.f937);let t13f: f64 = (l.f328).abs();let t140: f64 = (t13e * t13f);let t141: f64 = (t140 * l.f814);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t141, (((t13e * if l.f328 >= 0.0 { l.f339 } else { (-l.f339) }) * l.f814) + (t140 * l.f815)), (((((t13d * l.f938) * t13f) + (t13e * if l.f328 >= 0.0 { l.f33a } else { (-l.f33a) })) * l.f814) + (t140 * l.f816)), (((t13e * if l.f328 >= 0.0 { l.f33b } else { (-l.f33b) }) * l.f814) + (t140 * l.f817)), (((t13e * if l.f328 >= 0.0 { l.f33c } else { (-l.f33c) }) * l.f814) + (t140 * l.f818)), (((t13e * if l.f328 >= 0.0 { l.f33d } else { (-l.f33d) }) * l.f814) + (t140 * l.f819)), (((t13e * if l.f328 >= 0.0 { l.f33e } else { (-l.f33e) }) * l.f814) + (t140 * l.f81a)), );l.f6cf = 0.0;}
        if (l.f280 != 0.0) {let t142: f64 = (10000000000.0 * l.f88);let t143: f64 = (t142 * l.f3fb);(l.f6d0, l.f6f9, l.f6fa, l.f6fb, l.f6fc, l.f6fd, l.f6fe, ) = (t143, ((10000000000.0 * l.f89) * l.f3fb), ((10000000000.0 * l.f8a) * l.f3fb), ((10000000000.0 * l.f8b) * l.f3fb), ((10000000000.0 * l.f8c) * l.f3fb), ((10000000000.0 * l.f8d) * l.f3fb), ((10000000000.0 * l.f8e) * l.f3fb), );l.f6ff = 0.0;let t144: f64 = (l.f88 * l.f5f9);let t145: f64 = (t144 / 1.60219e-19);(l.f43c, l.f43d, l.f43e, l.f43f, l.f440, l.f441, l.f442, ) = (t145, (((l.f89 * l.f5f9) + (l.f88 * l.f5fa)) / 1.60219e-19), (((l.f8a * l.f5f9) + (l.f88 * l.f5fb)) / 1.60219e-19), (((l.f8b * l.f5f9) + (l.f88 * l.f5fc)) / 1.60219e-19), (((l.f8c * l.f5f9) + (l.f88 * l.f5fd)) / 1.60219e-19), (((l.f8d * l.f5f9) + (l.f88 * l.f5fe)) / 1.60219e-19), (((l.f8e * l.f5f9) + (l.f88 * l.f5ff)) / 1.60219e-19), );l.f443 = 0.0;let t146: f64 = (l.f88 * l.f5e9);let t147: f64 = (t146 / 1.60219e-19);(l.f452, l.f453, l.f454, l.f455, l.f456, l.f457, l.f458, ) = (t147, (((l.f89 * l.f5e9) + (l.f88 * l.f5ea)) / 1.60219e-19), (((l.f8a * l.f5e9) + (l.f88 * l.f5eb)) / 1.60219e-19), (((l.f8b * l.f5e9) + (l.f88 * l.f5ec)) / 1.60219e-19), (((l.f8c * l.f5e9) + (l.f88 * l.f5ed)) / 1.60219e-19), (((l.f8d * l.f5e9) + (l.f88 * l.f5ee)) / 1.60219e-19), (((l.f8e * l.f5e9) + (l.f88 * l.f5ef)) / 1.60219e-19), );l.f459 = 0.0;let t148: f64 = (l.f937 / 1.60219e-19);let t149: f64 = (l.f88 + l.f78);let t14a: f64 = (t148 * t149);(l.f466, l.f467, l.f468, l.f469, l.f46a, l.f46b, l.f46c, ) = (t14a, (t148 * l.f89), (((l.f938 / 1.60219e-19) * t149) + (t148 * l.f8a)), (t148 * l.f8b), (t148 * l.f8c), (t148 * l.f8d), (t148 * l.f8e), );l.f46d = 0.0;}
        if (l.f280 != 0.0) {let t14b: f64 = (l.f43c + l.f466);let t14c: f64 = (l.f452 + l.f466);let t14d: f64 = (t14b / t14c);let t14e: f64 = (t14d).max(1e-38);let t14f: f64 = (t14e).ln();let t150: f64 = (l.f45c * t14f);(l.f700, l.f729, l.f72a, l.f72b, l.f72c, l.f72d, l.f72e, ) = (t150, ((l.f45d * t14f) + (l.f45c * (if t14d >= 1e-38 { ((((l.f43d + l.f467) * t14c) - (t14b * (l.f453 + l.f467))) / (t14c * t14c)) } else { 0.0 } / t14e))), ((l.f45e * t14f) + (l.f45c * (if t14d >= 1e-38 { ((((l.f43e + l.f468) * t14c) - (t14b * (l.f454 + l.f468))) / (t14c * t14c)) } else { 0.0 } / t14e))), ((l.f45f * t14f) + (l.f45c * (if t14d >= 1e-38 { ((((l.f43f + l.f469) * t14c) - (t14b * (l.f455 + l.f469))) / (t14c * t14c)) } else { 0.0 } / t14e))), ((l.f460 * t14f) + (l.f45c * (if t14d >= 1e-38 { ((((l.f440 + l.f46a) * t14c) - (t14b * (l.f456 + l.f46a))) / (t14c * t14c)) } else { 0.0 } / t14e))), ((l.f461 * t14f) + (l.f45c * (if t14d >= 1e-38 { ((((l.f441 + l.f46b) * t14c) - (t14b * (l.f457 + l.f46b))) / (t14c * t14c)) } else { 0.0 } / t14e))), ((l.f462 * t14f) + (l.f45c * (if t14d >= 1e-38 { ((((l.f442 + l.f46c) * t14c) - (t14b * (l.f458 + l.f46c))) / (t14c * t14c)) } else { 0.0 } / t14e))), );l.f72f = 0.0;}
        if (l.f280 != 0.0) {let t151: f64 = (l.f43c - l.f452);let t152: f64 = (p.p289 * t151);(l.f730, l.f739, l.f73a, l.f73b, l.f73c, l.f73d, l.f73e, ) = (t152, (p.p289 * (l.f43d - l.f453)), (p.p289 * (l.f43e - l.f454)), (p.p289 * (l.f43f - l.f455)), (p.p289 * (l.f440 - l.f456)), (p.p289 * (l.f441 - l.f457)), (p.p289 * (l.f442 - l.f458)), );l.f73f = 0.0;let t153: f64 = (0.5 * p.p290);let t154: f64 = (l.f43c * l.f43c);let t155: f64 = (l.f452 * l.f452);let t156: f64 = (t154 - t155);let t157: f64 = (t153 * t156);(l.f740, l.f741, l.f742, l.f743, l.f744, l.f745, l.f746, ) = (t157, (t153 * (((l.f43d * l.f43c) + (l.f43c * l.f43d)) - ((l.f453 * l.f452) + (l.f452 * l.f453)))), (t153 * (((l.f43e * l.f43c) + (l.f43c * l.f43e)) - ((l.f454 * l.f452) + (l.f452 * l.f454)))), (t153 * (((l.f43f * l.f43c) + (l.f43c * l.f43f)) - ((l.f455 * l.f452) + (l.f452 * l.f455)))), (t153 * (((l.f440 * l.f43c) + (l.f43c * l.f440)) - ((l.f456 * l.f452) + (l.f452 * l.f456)))), (t153 * (((l.f441 * l.f43c) + (l.f43c * l.f441)) - ((l.f457 * l.f452) + (l.f452 * l.f457)))), (t153 * (((l.f442 * l.f43c) + (l.f43c * l.f442)) - ((l.f458 * l.f452) + (l.f452 * l.f458)))), );l.f747 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f280 != 0.0) {let t158: f64 = (1.60219e-19 * l.f937);let t159: f64 = (t158 * l.f328);let t15a: f64 = (t159 * l.f328);(l.f748, l.f749, l.f74a, l.f74b, l.f74c, l.f74d, l.f74e, ) = (t15a, (((t158 * l.f339) * l.f328) + (t159 * l.f339)), (((((1.60219e-19 * l.f938) * l.f328) + (t158 * l.f33a)) * l.f328) + (t159 * l.f33a)), (((t158 * l.f33b) * l.f328) + (t159 * l.f33b)), (((t158 * l.f33c) * l.f328) + (t159 * l.f33c)), (((t158 * l.f33d) * l.f328) + (t159 * l.f33d)), (((t158 * l.f33e) * l.f328) + (t159 * l.f33e)), );l.f74f = 0.0;let t15b: f64 = (10000000000.0 * l.f3fb);let t15c: f64 = (t15b * l.f94e);let t15d: f64 = (t15c * p.p2);(l.f750, l.f751, l.f752, l.f753, l.f754, l.f755, l.f756, ) = (t15d, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f757 = 0.0;let t15e: f64 = (p.p289 * l.f452);let t15f: f64 = (l.f45c + t15e);let t160: f64 = (p.p290 * l.f452);let t161: f64 = (t160 * l.f452);let t162: f64 = (t15f + t161);(l.f758, l.f759, l.f75a, l.f75b, l.f75c, l.f75d, l.f75e, ) = (t162, ((l.f45d + (p.p289 * l.f453)) + (((p.p290 * l.f453) * l.f452) + (t160 * l.f453))), ((l.f45e + (p.p289 * l.f454)) + (((p.p290 * l.f454) * l.f452) + (t160 * l.f454))), ((l.f45f + (p.p289 * l.f455)) + (((p.p290 * l.f455) * l.f452) + (t160 * l.f455))), ((l.f460 + (p.p289 * l.f456)) + (((p.p290 * l.f456) * l.f452) + (t160 * l.f456))), ((l.f461 + (p.p289 * l.f457)) + (((p.p290 * l.f457) * l.f452) + (t160 * l.f457))), ((l.f462 + (p.p289 * l.f458)) + (((p.p290 * l.f458) * l.f452) + (t160 * l.f458))), );l.f75f = 0.0;let t163: f64 = (l.f452 + l.f466);let t164: f64 = (l.f452 + l.f466);let t165: f64 = (t163 * t164);(l.f760, l.f761, l.f762, l.f763, l.f764, l.f765, l.f766, ) = (t165, (((l.f453 + l.f467) * t164) + (t163 * (l.f453 + l.f467))), (((l.f454 + l.f468) * t164) + (t163 * (l.f454 + l.f468))), (((l.f455 + l.f469) * t164) + (t163 * (l.f455 + l.f469))), (((l.f456 + l.f46a) * t164) + (t163 * (l.f456 + l.f46a))), (((l.f457 + l.f46b) * t164) + (t163 * (l.f457 + l.f46b))), (((l.f458 + l.f46c) * t164) + (t163 * (l.f458 + l.f46c))), );l.f767 = 0.0;}
        if (l.f280 != 0.0) {let t166: f64 = (l.f6a8 / l.f6d0);let t167: f64 = (l.f700 + l.f730);let t168: f64 = (t167 + l.f740);let t169: f64 = (t166 * t168);let t16a: f64 = (l.f748 / l.f750);let t16b: f64 = (t16a * l.fa8);let t16c: f64 = (t16b * l.f758);let t16d: f64 = (t16c / l.f760);let t16e: f64 = (t169 + t16d);(l.f68c, l.f68d, l.f68e, l.f68f, l.f690, l.f691, l.f692, ) = (t16e, ((((((l.f6c1 * l.f6d0) - (l.f6a8 * l.f6f9)) / (l.f6d0 * l.f6d0)) * t168) + (t166 * ((l.f729 + l.f739) + l.f741))) + ((((((((((l.f749 * l.f750) - (l.f748 * l.f751)) / (l.f750 * l.f750)) * l.fa8) + (t16a * l.fa9)) * l.f758) + (t16b * l.f759)) * l.f760) - (t16c * l.f761)) / (l.f760 * l.f760))), ((((((l.f6c2 * l.f6d0) - (l.f6a8 * l.f6fa)) / (l.f6d0 * l.f6d0)) * t168) + (t166 * ((l.f72a + l.f73a) + l.f742))) + ((((((((((l.f74a * l.f750) - (l.f748 * l.f752)) / (l.f750 * l.f750)) * l.fa8) + (t16a * l.faa)) * l.f758) + (t16b * l.f75a)) * l.f760) - (t16c * l.f762)) / (l.f760 * l.f760))), ((((((l.f6c3 * l.f6d0) - (l.f6a8 * l.f6fb)) / (l.f6d0 * l.f6d0)) * t168) + (t166 * ((l.f72b + l.f73b) + l.f743))) + ((((((((((l.f74b * l.f750) - (l.f748 * l.f753)) / (l.f750 * l.f750)) * l.fa8) + (t16a * l.fab)) * l.f758) + (t16b * l.f75b)) * l.f760) - (t16c * l.f763)) / (l.f760 * l.f760))), ((((((l.f6c4 * l.f6d0) - (l.f6a8 * l.f6fc)) / (l.f6d0 * l.f6d0)) * t168) + (t166 * ((l.f72c + l.f73c) + l.f744))) + ((((((((((l.f74c * l.f750) - (l.f748 * l.f754)) / (l.f750 * l.f750)) * l.fa8) + (t16a * l.fac)) * l.f758) + (t16b * l.f75c)) * l.f760) - (t16c * l.f764)) / (l.f760 * l.f760))), ((((((l.f6c5 * l.f6d0) - (l.f6a8 * l.f6fd)) / (l.f6d0 * l.f6d0)) * t168) + (t166 * ((l.f72d + l.f73d) + l.f745))) + ((((((((((l.f74d * l.f750) - (l.f748 * l.f755)) / (l.f750 * l.f750)) * l.fa8) + (t16a * l.fad)) * l.f758) + (t16b * l.f75d)) * l.f760) - (t16c * l.f765)) / (l.f760 * l.f760))), ((((((l.f6c6 * l.f6d0) - (l.f6a8 * l.f6fe)) / (l.f6d0 * l.f6d0)) * t168) + (t166 * ((l.f72e + l.f73e) + l.f746))) + ((((((((((l.f74e * l.f750) - (l.f748 * l.f756)) / (l.f750 * l.f750)) * l.fa8) + (t16a * l.fae)) * l.f758) + (t16b * l.f75e)) * l.f760) - (t16c * l.f766)) / (l.f760 * l.f760))), );l.f693 = 0.0;let t16f: f64 = (l.f45c * 1.60219e-19);let t170: f64 = (t16f * l.f937);(l.f6a9, l.f6aa, l.f6ab, l.f6ac, l.f6ad, l.f6ae, l.f6af, ) = (t170, ((l.f45d * 1.60219e-19) * l.f937), (((l.f45e * 1.60219e-19) * l.f937) + (t16f * l.f938)), ((l.f45f * 1.60219e-19) * l.f937), ((l.f460 * 1.60219e-19) * l.f937), ((l.f461 * 1.60219e-19) * l.f937), ((l.f462 * 1.60219e-19) * l.f937), );l.f6b0 = 0.0;}
        if (l.f280 != 0.0) {let t171: f64 = (l.f94e * p.p2);let t172: f64 = (t171 * l.f3f9);let t173: f64 = (t172 * 10000000000.0);let t174: f64 = (t173 * l.f466);let t175: f64 = (t174 * l.f466);(l.f6b1, l.f6b2, l.f6b3, l.f6b4, l.f6b5, l.f6b6, l.f6b7, ) = (t175, (((t173 * l.f467) * l.f466) + (t174 * l.f467)), (((t173 * l.f468) * l.f466) + (t174 * l.f468)), (((t173 * l.f469) * l.f466) + (t174 * l.f469)), (((t173 * l.f46a) * l.f466) + (t174 * l.f46a)), (((t173 * l.f46b) * l.f466) + (t174 * l.f46b)), (((t173 * l.f46c) * l.f466) + (t174 * l.f46c)), );l.f6b8 = 0.0;let t176: f64 = (l.f6a9 / l.f6b1);let t177: f64 = (t176 * l.f328);let t178: f64 = (t177 * l.f328);(l.f694, l.f695, l.f696, l.f697, l.f698, l.f699, l.f69a, ) = (t178, (((((((l.f6aa * l.f6b1) - (l.f6a9 * l.f6b2)) / (l.f6b1 * l.f6b1)) * l.f328) + (t176 * l.f339)) * l.f328) + (t177 * l.f339)), (((((((l.f6ab * l.f6b1) - (l.f6a9 * l.f6b3)) / (l.f6b1 * l.f6b1)) * l.f328) + (t176 * l.f33a)) * l.f328) + (t177 * l.f33a)), (((((((l.f6ac * l.f6b1) - (l.f6a9 * l.f6b4)) / (l.f6b1 * l.f6b1)) * l.f328) + (t176 * l.f33b)) * l.f328) + (t177 * l.f33b)), (((((((l.f6ad * l.f6b1) - (l.f6a9 * l.f6b5)) / (l.f6b1 * l.f6b1)) * l.f328) + (t176 * l.f33c)) * l.f328) + (t177 * l.f33c)), (((((((l.f6ae * l.f6b1) - (l.f6a9 * l.f6b6)) / (l.f6b1 * l.f6b1)) * l.f328) + (t176 * l.f33d)) * l.f328) + (t177 * l.f33d)), (((((((l.f6af * l.f6b1) - (l.f6a9 * l.f6b7)) / (l.f6b1 * l.f6b1)) * l.f328) + (t176 * l.f33e)) * l.f328) + (t177 * l.f33e)), );l.f69b = 0.0;let t179: f64 = (l.f694 + l.f68c);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t179, (l.f695 + l.f68d), (l.f696 + l.f68e), (l.f697 + l.f68f), (l.f698 + l.f690), (l.f699 + l.f691), (l.f69a + l.f692), );l.f6cf = 0.0;}
        let t17a: f64 = (l.fc6 * p.p2);let t17b: f64 = (t17a * l.f579);(l.f595, l.f596, l.f597, l.f598, l.f599, l.f59a, l.f59b, ) = (t17b, (t17a * l.f57a), (t17a * l.f57b), (t17a * l.f57c), (t17a * l.f57d), (t17a * l.f57e), (t17a * l.f57f), );l.f59c = 0.0;let t17c: f64 = (p.p2 * l.f53f);(l.f547, l.f548, l.f549, l.f54a, l.f54b, l.f54c, l.f54d, ) = (t17c, (p.p2 * l.f540), (p.p2 * l.f541), (p.p2 * l.f542), (p.p2 * l.f543), (p.p2 * l.f544), (p.p2 * l.f545), );l.f54e = 0.0;let t17d: f64 = if l.f68a > 0.0 { 1.0 } else { 0.0 };l.f288 = t17d;l.f289 = 0.0;
        if (l.f288 != 0.0) {let t17e: f64 = (p.p2 * l.f605);(l.f615, l.f616, l.f617, l.f618, l.f619, l.f61a, l.f61b, ) = (t17e, (p.p2 * l.f606), (p.p2 * l.f607), (p.p2 * l.f608), (p.p2 * l.f609), (p.p2 * l.f60a), (p.p2 * l.f60b), );l.f61c = 0.0;let t17f: f64 = (p.p2 * l.f561);(l.f571, l.f572, l.f573, l.f574, l.f575, l.f576, l.f577, ) = (t17f, (p.p2 * l.f562), (p.p2 * l.f563), (p.p2 * l.f564), (p.p2 * l.f565), (p.p2 * l.f566), (p.p2 * l.f567), );l.f578 = 0.0;let t180: f64 = (l.f605 - l.f5a9);let t181: f64 = (p.p2 * t180);let t182: f64 = (t181 + l.f60d);(l.f605, l.f606, l.f607, l.f608, l.f609, l.f60a, l.f60b, ) = (t182, ((p.p2 * (l.f606 - l.f5aa)) + l.f60e), ((p.p2 * (l.f607 - l.f5ab)) + l.f60f), ((p.p2 * (l.f608 - l.f5ac)) + l.f610), ((p.p2 * (l.f609 - l.f5ad)) + l.f611), ((p.p2 * (l.f60a - l.f5ae)) + l.f612), ((p.p2 * (l.f60b - l.f5af)) + l.f613), );l.f60c = 0.0;let t183: f64 = (l.f561 - l.f58d);let t184: f64 = (p.p2 * t183);let t185: f64 = (t184 + l.f569);(l.f561, l.f562, l.f563, l.f564, l.f565, l.f566, l.f567, ) = (t185, ((p.p2 * (l.f562 - l.f58e)) + l.f56a), ((p.p2 * (l.f563 - l.f58f)) + l.f56b), ((p.p2 * (l.f564 - l.f590)) + l.f56c), ((p.p2 * (l.f565 - l.f591)) + l.f56d), ((p.p2 * (l.f566 - l.f592)) + l.f56e), ((p.p2 * (l.f567 - l.f593)) + l.f56f), );l.f568 = 0.0;}
        if (l.f288 == 0.0) {let t186: f64 = (p.p2 * l.f561);(l.f615, l.f616, l.f617, l.f618, l.f619, l.f61a, l.f61b, ) = (t186, (p.p2 * l.f562), (p.p2 * l.f563), (p.p2 * l.f564), (p.p2 * l.f565), (p.p2 * l.f566), (p.p2 * l.f567), );l.f61c = 0.0;let t187: f64 = (p.p2 * l.f605);(l.f571, l.f572, l.f573, l.f574, l.f575, l.f576, l.f577, ) = (t187, (p.p2 * l.f606), (p.p2 * l.f607), (p.p2 * l.f608), (p.p2 * l.f609), (p.p2 * l.f60a), (p.p2 * l.f60b), );l.f578 = 0.0;let t188: f64 = (l.f561 - l.f5a9);let t189: f64 = (p.p2 * t188);let t18a: f64 = (t189 + l.f60d);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t18a, ((p.p2 * (l.f562 - l.f5aa)) + l.f60e), ((p.p2 * (l.f563 - l.f5ab)) + l.f60f), ((p.p2 * (l.f564 - l.f5ac)) + l.f610), ((p.p2 * (l.f565 - l.f5ad)) + l.f611), ((p.p2 * (l.f566 - l.f5ae)) + l.f612), ((p.p2 * (l.f567 - l.f5af)) + l.f613), );l.f6a7 = 0.0;let t18b: f64 = (l.f605 - l.f58d);let t18c: f64 = (p.p2 * t18b);let t18d: f64 = (t18c + l.f569);(l.f561, l.f562, l.f563, l.f564, l.f565, l.f566, l.f567, ) = (t18d, ((p.p2 * (l.f606 - l.f58e)) + l.f56a), ((p.p2 * (l.f607 - l.f58f)) + l.f56b), ((p.p2 * (l.f608 - l.f590)) + l.f56c), ((p.p2 * (l.f609 - l.f591)) + l.f56d), ((p.p2 * (l.f60a - l.f592)) + l.f56e), ((p.p2 * (l.f60b - l.f593)) + l.f56f), );l.f568 = 0.0;(l.f605, l.f606, l.f607, l.f608, l.f609, l.f60a, l.f60b, ) = (l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, );l.f60c = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t18e: f64 = (l.f5a9 + l.f58d);let t18f: f64 = (p.p2 * t18e);let t190: f64 = (l.f595 + t18f);(l.f579, l.f57a, l.f57b, l.f57c, l.f57d, l.f57e, l.f57f, ) = (t190, (l.f596 + (p.p2 * (l.f5aa + l.f58e))), (l.f597 + (p.p2 * (l.f5ab + l.f58f))), (l.f598 + (p.p2 * (l.f5ac + l.f590))), (l.f599 + (p.p2 * (l.f5ad + l.f591))), (l.f59a + (p.p2 * (l.f5ae + l.f592))), (l.f59b + (p.p2 * (l.f5af + l.f593))), );l.f580 = 0.0;let t191: f64 = (p.p2 * l.f53f);let t192: f64 = (t191 - l.f60d);let t193: f64 = (t192 - l.f569);(l.f53f, l.f540, l.f541, l.f542, l.f543, l.f544, l.f545, ) = (t193, (((p.p2 * l.f540) - l.f60e) - l.f56a), (((p.p2 * l.f541) - l.f60f) - l.f56b), (((p.p2 * l.f542) - l.f610) - l.f56c), (((p.p2 * l.f543) - l.f611) - l.f56d), (((p.p2 * l.f544) - l.f612) - l.f56e), (((p.p2 * l.f545) - l.f613) - l.f56f), );l.f546 = 0.0;let t194: f64 = (p.p2 * l.f5a9);(l.f5a9, l.f5aa, l.f5ab, l.f5ac, l.f5ad, l.f5ae, l.f5af, ) = (t194, (p.p2 * l.f5aa), (p.p2 * l.f5ab), (p.p2 * l.f5ac), (p.p2 * l.f5ad), (p.p2 * l.f5ae), (p.p2 * l.f5af), );l.f5b0 = 0.0;let t195: f64 = (p.p2 * l.f58d);(l.f58d, l.f58e, l.f58f, l.f590, l.f591, l.f592, l.f593, ) = (t195, (p.p2 * l.f58e), (p.p2 * l.f58f), (p.p2 * l.f590), (p.p2 * l.f591), (p.p2 * l.f592), (p.p2 * l.f593), );l.f594 = 0.0;let t196: f64 = (l.f615 + l.f571);let t197: f64 = (-t196);(l.f5f1, l.f5f2, l.f5f3, l.f5f4, l.f5f5, l.f5f6, l.f5f7, ) = (t197, (-(l.f616 + l.f572)), (-(l.f617 + l.f573)), (-(l.f618 + l.f574)), (-(l.f619 + l.f575)), (-(l.f61a + l.f576)), (-(l.f61b + l.f577)), );l.f5f8 = 0.0;let t198: f64 = (l.f814 * l.f5f1);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t198, ((l.f815 * l.f5f1) + (l.f814 * l.f5f2)), ((l.f816 * l.f5f1) + (l.f814 * l.f5f3)), ((l.f817 * l.f5f1) + (l.f814 * l.f5f4)), ((l.f818 * l.f5f1) + (l.f814 * l.f5f5)), ((l.f819 * l.f5f1) + (l.f814 * l.f5f6)), ((l.f81a * l.f5f1) + (l.f814 * l.f5f7)), );l.f6a7 = 0.0;let t199: f64 = (l.f6a0 * l.f65a);let t19a: f64 = (l.f3f5 * l.f3f5);let t19b: f64 = (t199 + t19a);(l.f6a8, l.f6c1, l.f6c2, l.f6c3, l.f6c4, l.f6c5, l.f6c6, ) = (t19b, ((l.f6a1 * l.f65a) + (l.f6a0 * l.f65b)), ((l.f6a2 * l.f65a) + (l.f6a0 * l.f65c)), ((l.f6a3 * l.f65a) + (l.f6a0 * l.f65d)), ((l.f6a4 * l.f65a) + (l.f6a0 * l.f65e)), ((l.f6a5 * l.f65a) + (l.f6a0 * l.f65f)), ((l.f6a6 * l.f65a) + (l.f6a0 * l.f660)), );l.f6cf = 0.0;let t19c: f64 = if ((p.p20 == 1.0) && (l.f97b != 0.0)) { 1.0 } else { 0.0 };l.f28a = t19c;l.f28b = 0.0;
        if (l.f28a != 0.0) {let t19d: f64 = (l.f814 * l.f82);let t19e: f64 = (t19d * l.f94e);let t19f: f64 = (t19e / l.f3f5);(l.f6a0, l.f6a1, l.f6a2, l.f6a3, l.f6a4, l.f6a5, l.f6a6, ) = (t19f, (((l.f815 * l.f82) * l.f94e) / l.f3f5), (((l.f816 * l.f82) * l.f94e) / l.f3f5), (((l.f817 * l.f82) * l.f94e) / l.f3f5), (((l.f818 * l.f82) * l.f94e) / l.f3f5), (((l.f819 * l.f82) * l.f94e) / l.f3f5), (((l.f81a * l.f82) * l.f94e) / l.f3f5), );l.f6a7 = 0.0;}
        let t1a0: f64 = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };l.f294 = t1a0;l.f295 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq0_e790, eq0_e790_d_n3, eq0_e790_d_n4, eq0_e790_d_n5, eq0_e790_d_n6, eq0_e790_d_n7, eq0_e790_d_n8,) = {
    if (l.f28c != 0.0) {
        let eq0_e782: f64 = (l.fc6 * l.f328);let eq0_e782_d_n3: f64 = (l.fc6 * l.f339);let eq0_e782_d_n4: f64 = (l.fc6 * l.f33a);let eq0_e782_d_n5: f64 = (l.fc6 * l.f33b);let eq0_e782_d_n6: f64 = (l.fc6 * l.f33c);let eq0_e782_d_n7: f64 = (l.fc6 * l.f33d);let eq0_e782_d_n8: f64 = (l.fc6 * l.f33e);let eq0_e785: f64 = 1e-12;let eq0_e787: f64 = (eq0_e785 * (nv5 - nv6));let eq0_e788: f64 = (eq0_e782 + eq0_e787);let eq0_e788_d_n5: f64 = (eq0_e782_d_n5 + eq0_e785);let eq0_e788_d_n6: f64 = (eq0_e782_d_n6 + (-eq0_e785));
        (eq0_e788, eq0_e782_d_n3, eq0_e782_d_n4, eq0_e788_d_n5, eq0_e788_d_n6, eq0_e782_d_n7, eq0_e782_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e790;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq0_e790_d_n3), multiplicity * (eq0_e790_d_n4), multiplicity * (eq0_e790_d_n5), multiplicity * (eq0_e790_d_n6), multiplicity * (eq0_e790_d_n7), multiplicity * (eq0_e790_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq1_e798, eq1_e798_d_n3, eq1_e798_d_n4, eq1_e798_d_n5, eq1_e798_d_n6, eq1_e798_d_n7, eq1_e798_d_n8,) = {
    if (l.f28c != 0.0) {
        let eq1_e795: f64 = (l.f37f + l.f3a5);let eq1_e795_d_n3: f64 = (l.f380 + l.f3a6);let eq1_e795_d_n4: f64 = (l.f381 + l.f3a7);let eq1_e795_d_n5: f64 = (l.f382 + l.f3a8);let eq1_e795_d_n6: f64 = (l.f383 + l.f3a9);let eq1_e795_d_n7: f64 = (l.f384 + l.f3aa);let eq1_e795_d_n8: f64 = (l.f385 + l.f3ab);let eq1_e796: f64 = (l.fc6 * eq1_e795);let eq1_e796_d_n3: f64 = (l.fc6 * eq1_e795_d_n3);let eq1_e796_d_n4: f64 = (l.fc6 * eq1_e795_d_n4);let eq1_e796_d_n5: f64 = (l.fc6 * eq1_e795_d_n5);let eq1_e796_d_n6: f64 = (l.fc6 * eq1_e795_d_n6);let eq1_e796_d_n7: f64 = (l.fc6 * eq1_e795_d_n7);let eq1_e796_d_n8: f64 = (l.fc6 * eq1_e795_d_n8);
        (eq1_e796, eq1_e796_d_n3, eq1_e796_d_n4, eq1_e796_d_n5, eq1_e796_d_n6, eq1_e796_d_n7, eq1_e796_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e798;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq1_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq1_e798_d_n3), multiplicity * (eq1_e798_d_n4), multiplicity * (eq1_e798_d_n5), multiplicity * (eq1_e798_d_n6), multiplicity * (eq1_e798_d_n7), multiplicity * (eq1_e798_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq2_e804, eq2_e804_d_n3, eq2_e804_d_n4, eq2_e804_d_n5, eq2_e804_d_n6, eq2_e804_d_n7, eq2_e804_d_n8,) = {
    if (l.f28c != 0.0) {
        let eq2_e802: f64 = (l.fc6 * l.f386);let eq2_e802_d_n3: f64 = (l.fc6 * l.f387);let eq2_e802_d_n4: f64 = (l.fc6 * l.f388);let eq2_e802_d_n5: f64 = (l.fc6 * l.f389);let eq2_e802_d_n6: f64 = (l.fc6 * l.f38a);let eq2_e802_d_n7: f64 = (l.fc6 * l.f38b);let eq2_e802_d_n8: f64 = (l.fc6 * l.f38c);
        (eq2_e802, eq2_e802_d_n3, eq2_e802_d_n4, eq2_e802_d_n5, eq2_e802_d_n6, eq2_e802_d_n7, eq2_e802_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e804;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq2_e804_d_n3), multiplicity * (eq2_e804_d_n4), multiplicity * (eq2_e804_d_n5), multiplicity * (eq2_e804_d_n6), multiplicity * (eq2_e804_d_n7), multiplicity * (eq2_e804_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq3_e812, eq3_e812_d_n3, eq3_e812_d_n4, eq3_e812_d_n5, eq3_e812_d_n6, eq3_e812_d_n7, eq3_e812_d_n8,) = {
    if (l.f28c != 0.0) {
        let eq3_e809: f64 = (l.f371 + l.f38d);let eq3_e809_d_n3: f64 = (l.f372 + l.f38e);let eq3_e809_d_n4: f64 = (l.f373 + l.f38f);let eq3_e809_d_n5: f64 = (l.f374 + l.f390);let eq3_e809_d_n6: f64 = (l.f375 + l.f391);let eq3_e809_d_n7: f64 = (l.f376 + l.f392);let eq3_e809_d_n8: f64 = (l.f377 + l.f393);let eq3_e810: f64 = (l.fc6 * eq3_e809);let eq3_e810_d_n3: f64 = (l.fc6 * eq3_e809_d_n3);let eq3_e810_d_n4: f64 = (l.fc6 * eq3_e809_d_n4);let eq3_e810_d_n5: f64 = (l.fc6 * eq3_e809_d_n5);let eq3_e810_d_n6: f64 = (l.fc6 * eq3_e809_d_n6);let eq3_e810_d_n7: f64 = (l.fc6 * eq3_e809_d_n7);let eq3_e810_d_n8: f64 = (l.fc6 * eq3_e809_d_n8);
        (eq3_e810, eq3_e810_d_n3, eq3_e810_d_n4, eq3_e810_d_n5, eq3_e810_d_n6, eq3_e810_d_n7, eq3_e810_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e812;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq3_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq3_e812_d_n3), multiplicity * (eq3_e812_d_n4), multiplicity * (eq3_e812_d_n5), multiplicity * (eq3_e812_d_n6), multiplicity * (eq3_e812_d_n7), multiplicity * (eq3_e812_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq4_e820, eq4_e820_d_n3, eq4_e820_d_n4, eq4_e820_d_n5, eq4_e820_d_n6, eq4_e820_d_n7, eq4_e820_d_n8,) = {
    if (l.f28c != 0.0) {
        let eq4_e817: f64 = (l.f36a + l.f378);let eq4_e817_d_n3: f64 = (l.f36b + l.f379);let eq4_e817_d_n4: f64 = (l.f36c + l.f37a);let eq4_e817_d_n5: f64 = (l.f36d + l.f37b);let eq4_e817_d_n6: f64 = (l.f36e + l.f37c);let eq4_e817_d_n7: f64 = (l.f36f + l.f37d);let eq4_e817_d_n8: f64 = (l.f370 + l.f37e);let eq4_e818: f64 = (l.fc6 * eq4_e817);let eq4_e818_d_n3: f64 = (l.fc6 * eq4_e817_d_n3);let eq4_e818_d_n4: f64 = (l.fc6 * eq4_e817_d_n4);let eq4_e818_d_n5: f64 = (l.fc6 * eq4_e817_d_n5);let eq4_e818_d_n6: f64 = (l.fc6 * eq4_e817_d_n6);let eq4_e818_d_n7: f64 = (l.fc6 * eq4_e817_d_n7);let eq4_e818_d_n8: f64 = (l.fc6 * eq4_e817_d_n8);
        (eq4_e818, eq4_e818_d_n3, eq4_e818_d_n4, eq4_e818_d_n5, eq4_e818_d_n6, eq4_e818_d_n7, eq4_e818_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e820;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq4_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq4_e820_d_n3), multiplicity * (eq4_e820_d_n4), multiplicity * (eq4_e820_d_n5), multiplicity * (eq4_e820_d_n6), multiplicity * (eq4_e820_d_n7), multiplicity * (eq4_e820_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq5_e833, eq5_e833_d_n3, eq5_e833_d_n4, eq5_e833_d_n5, eq5_e833_d_n6, eq5_e833_d_n7, eq5_e833_d_n8,) = {
    if (l.f28c == 0.0) {
        let eq5_e825: f64 = (l.fc6 * l.f328);let eq5_e825_d_n3: f64 = (l.fc6 * l.f339);let eq5_e825_d_n4: f64 = (l.fc6 * l.f33a);let eq5_e825_d_n5: f64 = (l.fc6 * l.f33b);let eq5_e825_d_n6: f64 = (l.fc6 * l.f33c);let eq5_e825_d_n7: f64 = (l.fc6 * l.f33d);let eq5_e825_d_n8: f64 = (l.fc6 * l.f33e);let eq5_e828: f64 = 1e-12;let eq5_e830: f64 = (eq5_e828 * (nv6 - nv5));let eq5_e831: f64 = (eq5_e825 + eq5_e830);let eq5_e831_d_n5: f64 = (eq5_e825_d_n5 + (-eq5_e828));let eq5_e831_d_n6: f64 = (eq5_e825_d_n6 + eq5_e828);
        (eq5_e831, eq5_e825_d_n3, eq5_e825_d_n4, eq5_e831_d_n5, eq5_e831_d_n6, eq5_e825_d_n7, eq5_e825_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e833;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq5_e833_d_n3), multiplicity * (eq5_e833_d_n4), multiplicity * (eq5_e833_d_n5), multiplicity * (eq5_e833_d_n6), multiplicity * (eq5_e833_d_n7), multiplicity * (eq5_e833_d_n8)],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
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
        let (eq6_e842, eq6_e842_d_n3, eq6_e842_d_n4, eq6_e842_d_n5, eq6_e842_d_n6, eq6_e842_d_n7, eq6_e842_d_n8,) = {
    if (l.f28c == 0.0) {
        let eq6_e839: f64 = (l.f37f + l.f3a5);let eq6_e839_d_n3: f64 = (l.f380 + l.f3a6);let eq6_e839_d_n4: f64 = (l.f381 + l.f3a7);let eq6_e839_d_n5: f64 = (l.f382 + l.f3a8);let eq6_e839_d_n6: f64 = (l.f383 + l.f3a9);let eq6_e839_d_n7: f64 = (l.f384 + l.f3aa);let eq6_e839_d_n8: f64 = (l.f385 + l.f3ab);let eq6_e840: f64 = (l.fc6 * eq6_e839);let eq6_e840_d_n3: f64 = (l.fc6 * eq6_e839_d_n3);let eq6_e840_d_n4: f64 = (l.fc6 * eq6_e839_d_n4);let eq6_e840_d_n5: f64 = (l.fc6 * eq6_e839_d_n5);let eq6_e840_d_n6: f64 = (l.fc6 * eq6_e839_d_n6);let eq6_e840_d_n7: f64 = (l.fc6 * eq6_e839_d_n7);let eq6_e840_d_n8: f64 = (l.fc6 * eq6_e839_d_n8);
        (eq6_e840, eq6_e840_d_n3, eq6_e840_d_n4, eq6_e840_d_n5, eq6_e840_d_n6, eq6_e840_d_n7, eq6_e840_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e842;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq6_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq6_e842_d_n3), multiplicity * (eq6_e842_d_n4), multiplicity * (eq6_e842_d_n5), multiplicity * (eq6_e842_d_n6), multiplicity * (eq6_e842_d_n7), multiplicity * (eq6_e842_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq7_e849, eq7_e849_d_n3, eq7_e849_d_n4, eq7_e849_d_n5, eq7_e849_d_n6, eq7_e849_d_n7, eq7_e849_d_n8,) = {
    if (l.f28c == 0.0) {
        let eq7_e847: f64 = (l.fc6 * l.f386);let eq7_e847_d_n3: f64 = (l.fc6 * l.f387);let eq7_e847_d_n4: f64 = (l.fc6 * l.f388);let eq7_e847_d_n5: f64 = (l.fc6 * l.f389);let eq7_e847_d_n6: f64 = (l.fc6 * l.f38a);let eq7_e847_d_n7: f64 = (l.fc6 * l.f38b);let eq7_e847_d_n8: f64 = (l.fc6 * l.f38c);
        (eq7_e847, eq7_e847_d_n3, eq7_e847_d_n4, eq7_e847_d_n5, eq7_e847_d_n6, eq7_e847_d_n7, eq7_e847_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e849;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq7_e849_d_n3), multiplicity * (eq7_e849_d_n4), multiplicity * (eq7_e849_d_n5), multiplicity * (eq7_e849_d_n6), multiplicity * (eq7_e849_d_n7), multiplicity * (eq7_e849_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq8_e858, eq8_e858_d_n3, eq8_e858_d_n4, eq8_e858_d_n5, eq8_e858_d_n6, eq8_e858_d_n7, eq8_e858_d_n8,) = {
    if (l.f28c == 0.0) {
        let eq8_e855: f64 = (l.f371 + l.f38d);let eq8_e855_d_n3: f64 = (l.f372 + l.f38e);let eq8_e855_d_n4: f64 = (l.f373 + l.f38f);let eq8_e855_d_n5: f64 = (l.f374 + l.f390);let eq8_e855_d_n6: f64 = (l.f375 + l.f391);let eq8_e855_d_n7: f64 = (l.f376 + l.f392);let eq8_e855_d_n8: f64 = (l.f377 + l.f393);let eq8_e856: f64 = (l.fc6 * eq8_e855);let eq8_e856_d_n3: f64 = (l.fc6 * eq8_e855_d_n3);let eq8_e856_d_n4: f64 = (l.fc6 * eq8_e855_d_n4);let eq8_e856_d_n5: f64 = (l.fc6 * eq8_e855_d_n5);let eq8_e856_d_n6: f64 = (l.fc6 * eq8_e855_d_n6);let eq8_e856_d_n7: f64 = (l.fc6 * eq8_e855_d_n7);let eq8_e856_d_n8: f64 = (l.fc6 * eq8_e855_d_n8);
        (eq8_e856, eq8_e856_d_n3, eq8_e856_d_n4, eq8_e856_d_n5, eq8_e856_d_n6, eq8_e856_d_n7, eq8_e856_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e858;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq8_e858_d_n3), multiplicity * (eq8_e858_d_n4), multiplicity * (eq8_e858_d_n5), multiplicity * (eq8_e858_d_n6), multiplicity * (eq8_e858_d_n7), multiplicity * (eq8_e858_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq9_e867, eq9_e867_d_n3, eq9_e867_d_n4, eq9_e867_d_n5, eq9_e867_d_n6, eq9_e867_d_n7, eq9_e867_d_n8,) = {
    if (l.f28c == 0.0) {
        let eq9_e864: f64 = (l.f36a + l.f378);let eq9_e864_d_n3: f64 = (l.f36b + l.f379);let eq9_e864_d_n4: f64 = (l.f36c + l.f37a);let eq9_e864_d_n5: f64 = (l.f36d + l.f37b);let eq9_e864_d_n6: f64 = (l.f36e + l.f37c);let eq9_e864_d_n7: f64 = (l.f36f + l.f37d);let eq9_e864_d_n8: f64 = (l.f370 + l.f37e);let eq9_e865: f64 = (l.fc6 * eq9_e864);let eq9_e865_d_n3: f64 = (l.fc6 * eq9_e864_d_n3);let eq9_e865_d_n4: f64 = (l.fc6 * eq9_e864_d_n4);let eq9_e865_d_n5: f64 = (l.fc6 * eq9_e864_d_n5);let eq9_e865_d_n6: f64 = (l.fc6 * eq9_e864_d_n6);let eq9_e865_d_n7: f64 = (l.fc6 * eq9_e864_d_n7);let eq9_e865_d_n8: f64 = (l.fc6 * eq9_e864_d_n8);
        (eq9_e865, eq9_e865_d_n3, eq9_e865_d_n4, eq9_e865_d_n5, eq9_e865_d_n6, eq9_e865_d_n7, eq9_e865_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e867;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq9_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq9_e867_d_n3), multiplicity * (eq9_e867_d_n4), multiplicity * (eq9_e867_d_n5), multiplicity * (eq9_e867_d_n6), multiplicity * (eq9_e867_d_n7), multiplicity * (eq9_e867_d_n8)],
            [],
            [],
            1.0,
        );let eq10_e870: f64 = (l.fc6 * l.f35c);let eq10_e870_d_n3: f64 = (l.fc6 * l.f35d);let eq10_e870_d_n4: f64 = (l.fc6 * l.f35e);let eq10_e870_d_n5: f64 = (l.fc6 * l.f35f);let eq10_e870_d_n6: f64 = (l.fc6 * l.f360);let eq10_e870_d_n7: f64 = (l.fc6 * l.f361);let eq10_e870_d_n8: f64 = (l.fc6 * l.f362);let eq10_value: f64 = eq10_e870;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq10_e870_d_n3), multiplicity * (eq10_e870_d_n4), multiplicity * (eq10_e870_d_n5), multiplicity * (eq10_e870_d_n6), multiplicity * (eq10_e870_d_n7), multiplicity * (eq10_e870_d_n8)],
            [],
            [],
            1.0,
        );let eq11_e873: f64 = (l.fc6 * l.f34e);let eq11_e873_d_n3: f64 = (l.fc6 * l.f34f);let eq11_e873_d_n4: f64 = (l.fc6 * l.f350);let eq11_e873_d_n5: f64 = (l.fc6 * l.f351);let eq11_e873_d_n6: f64 = (l.fc6 * l.f352);let eq11_e873_d_n7: f64 = (l.fc6 * l.f353);let eq11_e873_d_n8: f64 = (l.fc6 * l.f354);let eq11_value: f64 = eq11_e873;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq11_e873_d_n3), multiplicity * (eq11_e873_d_n4), multiplicity * (eq11_e873_d_n5), multiplicity * (eq11_e873_d_n6), multiplicity * (eq11_e873_d_n7), multiplicity * (eq11_e873_d_n8)],
            [],
            [],
            1.0,
        );let eq12_e876: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, l.f571);let eq12_e877: f64 = (l.fc6 * eq12_e876);let eq12_e877_d_n3: f64 = (l.fc6 * (l.f572 * ddt_scale));let eq12_e877_d_n4: f64 = (l.fc6 * (l.f573 * ddt_scale));let eq12_e877_d_n5: f64 = (l.fc6 * (l.f574 * ddt_scale));let eq12_e877_d_n6: f64 = (l.fc6 * (l.f575 * ddt_scale));let eq12_e877_d_n7: f64 = (l.fc6 * (l.f576 * ddt_scale));let eq12_e877_d_n8: f64 = (l.fc6 * (l.f577 * ddt_scale));let eq12_value: f64 = eq12_e877;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq12_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq12_e877_d_n3), multiplicity * (eq12_e877_d_n4), multiplicity * (eq12_e877_d_n5), multiplicity * (eq12_e877_d_n6), multiplicity * (eq12_e877_d_n7), multiplicity * (eq12_e877_d_n8)],
            [],
            [],
            1.0,
        );let eq13_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, l.f595);let eq13_value: f64 = eq13_e879;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq13_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * ((l.f596 * ddt_scale)), multiplicity * ((l.f597 * ddt_scale)), multiplicity * ((l.f598 * ddt_scale)), multiplicity * ((l.f599 * ddt_scale)), multiplicity * ((l.f59a * ddt_scale)), multiplicity * ((l.f59b * ddt_scale))],
            [],
            [],
            1.0,
        );
    }
    #[inline(never)]
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
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let eq14_e882: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, l.f547);let eq14_e883: f64 = (l.fc6 * eq14_e882);let eq14_e883_d_n3: f64 = (l.fc6 * (l.f548 * ddt_scale));let eq14_e883_d_n4: f64 = (l.fc6 * (l.f549 * ddt_scale));let eq14_e883_d_n5: f64 = (l.fc6 * (l.f54a * ddt_scale));let eq14_e883_d_n6: f64 = (l.fc6 * (l.f54b * ddt_scale));let eq14_e883_d_n7: f64 = (l.fc6 * (l.f54c * ddt_scale));let eq14_e883_d_n8: f64 = (l.fc6 * (l.f54d * ddt_scale));let eq14_value: f64 = eq14_e883;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(6),
            multiplicity * (eq14_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq14_e883_d_n3), multiplicity * (eq14_e883_d_n4), multiplicity * (eq14_e883_d_n5), multiplicity * (eq14_e883_d_n6), multiplicity * (eq14_e883_d_n7), multiplicity * (eq14_e883_d_n8)],
            [],
            [],
            1.0,
        );let eq15_e885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, l.f5a9);let eq15_value: f64 = eq15_e885;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq15_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * ((l.f5aa * ddt_scale)), multiplicity * ((l.f5ab * ddt_scale)), multiplicity * ((l.f5ac * ddt_scale)), multiplicity * ((l.f5ad * ddt_scale)), multiplicity * ((l.f5ae * ddt_scale)), multiplicity * ((l.f5af * ddt_scale))],
            [],
            [],
            1.0,
        );let eq16_e887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, l.f58d);let eq16_value: f64 = eq16_e887;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq16_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * ((l.f58e * ddt_scale)), multiplicity * ((l.f58f * ddt_scale)), multiplicity * ((l.f590 * ddt_scale)), multiplicity * ((l.f591 * ddt_scale)), multiplicity * ((l.f592 * ddt_scale)), multiplicity * ((l.f593 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq17_e890: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, l.f60d);let eq17_e891: f64 = (l.fc6 * eq17_e890);let eq17_e891_d_n3: f64 = (l.fc6 * (l.f60e * ddt_scale));let eq17_e891_d_n4: f64 = (l.fc6 * (l.f60f * ddt_scale));let eq17_e891_d_n5: f64 = (l.fc6 * (l.f610 * ddt_scale));let eq17_e891_d_n6: f64 = (l.fc6 * (l.f611 * ddt_scale));let eq17_e891_d_n7: f64 = (l.fc6 * (l.f612 * ddt_scale));let eq17_e891_d_n8: f64 = (l.fc6 * (l.f613 * ddt_scale));let eq17_value: f64 = eq17_e891;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(3),
            multiplicity * (eq17_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq17_e891_d_n3), multiplicity * (eq17_e891_d_n4), multiplicity * (eq17_e891_d_n5), multiplicity * (eq17_e891_d_n6), multiplicity * (eq17_e891_d_n7), multiplicity * (eq17_e891_d_n8)],
            [],
            [],
            1.0,
        );let eq18_e894: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, l.f569);let eq18_e895: f64 = (l.fc6 * eq18_e894);let eq18_e895_d_n3: f64 = (l.fc6 * (l.f56a * ddt_scale));let eq18_e895_d_n4: f64 = (l.fc6 * (l.f56b * ddt_scale));let eq18_e895_d_n5: f64 = (l.fc6 * (l.f56c * ddt_scale));let eq18_e895_d_n6: f64 = (l.fc6 * (l.f56d * ddt_scale));let eq18_e895_d_n7: f64 = (l.fc6 * (l.f56e * ddt_scale));let eq18_e895_d_n8: f64 = (l.fc6 * (l.f56f * ddt_scale));let eq18_value: f64 = eq18_e895;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (eq18_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq18_e895_d_n3), multiplicity * (eq18_e895_d_n4), multiplicity * (eq18_e895_d_n5), multiplicity * (eq18_e895_d_n6), multiplicity * (eq18_e895_d_n7), multiplicity * (eq18_e895_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq19_e899,) = {
    if (l.f28d != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e899;
        stamper.stamp_potential_const_local(
            0,
            eq19_value,
        );
        let (eq20_e903,) = {
    if (l.f28d != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e903;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e910, eq21_e910_d_n0, eq21_e910_d_n3, eq21_e910_d_n4, eq21_e910_d_n5, eq21_e910_d_n6, eq21_e910_d_n7, eq21_e910_d_n8,) = {
    if (l.f28d == 0.0) {
        let eq21_e908: f64 = ((nv0 - nv5) * l.f249);let eq21_e908_d_n3: f64 = ((nv0 - nv5) * l.f24a);let eq21_e908_d_n4: f64 = ((nv0 - nv5) * l.f24b);let eq21_e908_d_n5: f64 = ((-l.f249) + ((nv0 - nv5) * l.f24c));let eq21_e908_d_n6: f64 = ((nv0 - nv5) * l.f24d);let eq21_e908_d_n7: f64 = ((nv0 - nv5) * l.f24e);let eq21_e908_d_n8: f64 = ((nv0 - nv5) * l.f24f);
        (eq21_e908, l.f249, eq21_e908_d_n3, eq21_e908_d_n4, eq21_e908_d_n5, eq21_e908_d_n6, eq21_e908_d_n7, eq21_e908_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e910;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(0),
            Some(5),
            multiplicity * (eq21_value),
            [0, 3, 4, 5, 6, 7, 8],
            [multiplicity * (eq21_e910_d_n0), multiplicity * (eq21_e910_d_n3), multiplicity * (eq21_e910_d_n4), multiplicity * (eq21_e910_d_n5), multiplicity * (eq21_e910_d_n6), multiplicity * (eq21_e910_d_n7), multiplicity * (eq21_e910_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq22_e917, eq22_e917_d_n2, eq22_e917_d_n3, eq22_e917_d_n4, eq22_e917_d_n5, eq22_e917_d_n6, eq22_e917_d_n7, eq22_e917_d_n8,) = {
    if (l.f28d == 0.0) {
        let eq22_e915: f64 = ((nv2 - nv6) * l.f252);let eq22_e915_d_n3: f64 = ((nv2 - nv6) * l.f253);let eq22_e915_d_n4: f64 = ((nv2 - nv6) * l.f254);let eq22_e915_d_n5: f64 = ((nv2 - nv6) * l.f255);let eq22_e915_d_n6: f64 = ((-l.f252) + ((nv2 - nv6) * l.f256));let eq22_e915_d_n7: f64 = ((nv2 - nv6) * l.f257);let eq22_e915_d_n8: f64 = ((nv2 - nv6) * l.f258);
        (eq22_e915, l.f252, eq22_e915_d_n3, eq22_e915_d_n4, eq22_e915_d_n5, eq22_e915_d_n6, eq22_e915_d_n7, eq22_e915_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e917;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(2),
            Some(6),
            multiplicity * (eq22_value),
            [2, 3, 4, 5, 6, 7, 8],
            [multiplicity * (eq22_e917_d_n2), multiplicity * (eq22_e917_d_n3), multiplicity * (eq22_e917_d_n4), multiplicity * (eq22_e917_d_n5), multiplicity * (eq22_e917_d_n6), multiplicity * (eq22_e917_d_n7), multiplicity * (eq22_e917_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq25_e941, eq25_e941_d_n3, eq25_e941_d_n4, eq25_e941_d_n5, eq25_e941_d_n6, eq25_e941_d_n7, eq25_e941_d_n8,) = {
    if (l.f28f != 0.0) {
        let eq25_e939: f64 = ((nv7 - nv8) * l.f242);let eq25_e939_d_n3: f64 = ((nv7 - nv8) * l.f243);let eq25_e939_d_n4: f64 = ((nv7 - nv8) * l.f244);let eq25_e939_d_n5: f64 = ((nv7 - nv8) * l.f245);let eq25_e939_d_n6: f64 = ((nv7 - nv8) * l.f246);let eq25_e939_d_n7: f64 = (l.f242 + ((nv7 - nv8) * l.f247));let eq25_e939_d_n8: f64 = ((-l.f242) + ((nv7 - nv8) * l.f248));
        (eq25_e939, eq25_e939_d_n3, eq25_e939_d_n4, eq25_e939_d_n5, eq25_e939_d_n6, eq25_e939_d_n7, eq25_e939_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e941;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq25_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq25_e941_d_n3), multiplicity * (eq25_e941_d_n4), multiplicity * (eq25_e941_d_n5), multiplicity * (eq25_e941_d_n6), multiplicity * (eq25_e941_d_n7), multiplicity * (eq25_e941_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq26_e946,) = {
    if (l.f28f == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e946;
        stamper.stamp_potential_const_local(
            2,
            eq26_value,
        );
        let (eq27_e950,) = {
    if (l.f290 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e950;
        stamper.stamp_potential_const_local(
            3,
            eq27_value,
        );
        let (eq28_e957, eq28_e957_d_n1, eq28_e957_d_n7,) = {
    if (l.f290 == 0.0) {
        let eq28_e955: f64 = ((nv1 - nv7) * l.f250);
        (eq28_e955, l.f250, (-l.f250),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e957;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(7),
            multiplicity * (eq28_value),
            1,
            multiplicity * (eq28_e957_d_n1),
            7,
            multiplicity * (eq28_e957_d_n7),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
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
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv4 = ctx.node_voltage(nodes[4]);let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq38_e1082, eq38_e1082_d_n0, eq38_e1082_d_n2, eq38_e1082_d_n3, eq38_e1082_d_n4, eq38_e1082_d_n5, eq38_e1082_d_n6, eq38_e1082_d_n7, eq38_e1082_d_n8,) = {
    if ((l.f294 != 0.0) && (l.f296 != 0.0)) {
        let eq38_e1063: f64 = (l.fc6 * l.f68a);let eq38_e1065: f64 = (eq38_e1063 * (nv5 - nv6));let eq38_e1067: f64 = (eq38_e1065 * l.f328);let eq38_e1067_d_n3: f64 = (eq38_e1065 * l.f339);let eq38_e1067_d_n4: f64 = (eq38_e1065 * l.f33a);let eq38_e1067_d_n5: f64 = ((eq38_e1063 * l.f328) + (eq38_e1065 * l.f33b));let eq38_e1067_d_n6: f64 = (((-eq38_e1063) * l.f328) + (eq38_e1065 * l.f33c));let eq38_e1067_d_n7: f64 = (eq38_e1065 * l.f33d);let eq38_e1067_d_n8: f64 = (eq38_e1065 * l.f33e);let eq38_e1070: f64 = ((nv0 - nv5) * (nv0 - nv5));let eq38_e1070_d_n0: f64 = ((nv0 - nv5) + (nv0 - nv5));let eq38_e1070_d_n5: f64 = ((-(nv0 - nv5)) + (-(nv0 - nv5)));let __rspice_inv_cse_0: f64 = 1.0 / l.f651;let eq38_e1072: f64 = (eq38_e1070 * __rspice_inv_cse_0);let eq38_e1072_d_n0: f64 = (eq38_e1070_d_n0 * __rspice_inv_cse_0);let eq38_e1072_d_n3: f64 = (-((eq38_e1070 * l.f652) / (l.f651 * l.f651)));let eq38_e1072_d_n4: f64 = (-((eq38_e1070 * l.f653) / (l.f651 * l.f651)));let eq38_e1072_d_n5: f64 = (((eq38_e1070_d_n5 * l.f651) - (eq38_e1070 * l.f654)) / (l.f651 * l.f651));let eq38_e1072_d_n6: f64 = (-((eq38_e1070 * l.f655) / (l.f651 * l.f651)));let eq38_e1072_d_n7: f64 = (-((eq38_e1070 * l.f656) / (l.f651 * l.f651)));let eq38_e1072_d_n8: f64 = (-((eq38_e1070 * l.f657) / (l.f651 * l.f651)));let eq38_e1073: f64 = (eq38_e1067 + eq38_e1072);let eq38_e1073_d_n3: f64 = (eq38_e1067_d_n3 + eq38_e1072_d_n3);let eq38_e1073_d_n4: f64 = (eq38_e1067_d_n4 + eq38_e1072_d_n4);let eq38_e1073_d_n5: f64 = (eq38_e1067_d_n5 + eq38_e1072_d_n5);let eq38_e1073_d_n6: f64 = (eq38_e1067_d_n6 + eq38_e1072_d_n6);let eq38_e1073_d_n7: f64 = (eq38_e1067_d_n7 + eq38_e1072_d_n7);let eq38_e1073_d_n8: f64 = (eq38_e1067_d_n8 + eq38_e1072_d_n8);let eq38_e1076: f64 = ((nv2 - nv6) * (nv2 - nv6));let eq38_e1076_d_n2: f64 = ((nv2 - nv6) + (nv2 - nv6));let eq38_e1076_d_n6: f64 = ((-(nv2 - nv6)) + (-(nv2 - nv6)));let __rspice_inv_cse_1: f64 = 1.0 / l.f673;let eq38_e1078: f64 = (eq38_e1076 * __rspice_inv_cse_1);let eq38_e1078_d_n2: f64 = (eq38_e1076_d_n2 * __rspice_inv_cse_1);let eq38_e1078_d_n3: f64 = (-((eq38_e1076 * l.f674) / (l.f673 * l.f673)));let eq38_e1078_d_n4: f64 = (-((eq38_e1076 * l.f675) / (l.f673 * l.f673)));let eq38_e1078_d_n5: f64 = (-((eq38_e1076 * l.f676) / (l.f673 * l.f673)));let eq38_e1078_d_n6: f64 = (((eq38_e1076_d_n6 * l.f673) - (eq38_e1076 * l.f677)) / (l.f673 * l.f673));let eq38_e1078_d_n7: f64 = (-((eq38_e1076 * l.f678) / (l.f673 * l.f673)));let eq38_e1078_d_n8: f64 = (-((eq38_e1076 * l.f679) / (l.f673 * l.f673)));let eq38_e1079: f64 = (eq38_e1073 + eq38_e1078);let eq38_e1079_d_n3: f64 = (eq38_e1073_d_n3 + eq38_e1078_d_n3);let eq38_e1079_d_n4: f64 = (eq38_e1073_d_n4 + eq38_e1078_d_n4);let eq38_e1079_d_n5: f64 = (eq38_e1073_d_n5 + eq38_e1078_d_n5);let eq38_e1079_d_n6: f64 = (eq38_e1073_d_n6 + eq38_e1078_d_n6);let eq38_e1079_d_n7: f64 = (eq38_e1073_d_n7 + eq38_e1078_d_n7);let eq38_e1079_d_n8: f64 = (eq38_e1073_d_n8 + eq38_e1078_d_n8);let eq38_e1080: f64 = (-eq38_e1079);
        (eq38_e1080, (-eq38_e1072_d_n0), (-eq38_e1078_d_n2), (-eq38_e1079_d_n3), (-eq38_e1079_d_n4), (-eq38_e1079_d_n5), (-eq38_e1079_d_n6), (-eq38_e1079_d_n7), (-eq38_e1079_d_n8),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1082;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 3, 4, 5, 6, 7, 8],
            [multiplicity * (eq38_e1082_d_n0), multiplicity * (eq38_e1082_d_n2), multiplicity * (eq38_e1082_d_n3), multiplicity * (eq38_e1082_d_n4), multiplicity * (eq38_e1082_d_n5), multiplicity * (eq38_e1082_d_n6), multiplicity * (eq38_e1082_d_n7), multiplicity * (eq38_e1082_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq39_e1096, eq39_e1096_d_n3, eq39_e1096_d_n4, eq39_e1096_d_n5, eq39_e1096_d_n6, eq39_e1096_d_n7, eq39_e1096_d_n8,) = {
    if ((l.f294 != 0.0) && (l.f296 == 0.0)) {
        let eq39_e1089: f64 = (l.fc6 * l.f68a);let eq39_e1091: f64 = (eq39_e1089 * (nv5 - nv6));let eq39_e1093: f64 = (eq39_e1091 * l.f328);let eq39_e1093_d_n3: f64 = (eq39_e1091 * l.f339);let eq39_e1093_d_n4: f64 = (eq39_e1091 * l.f33a);let eq39_e1093_d_n5: f64 = ((eq39_e1089 * l.f328) + (eq39_e1091 * l.f33b));let eq39_e1093_d_n6: f64 = (((-eq39_e1089) * l.f328) + (eq39_e1091 * l.f33c));let eq39_e1093_d_n7: f64 = (eq39_e1091 * l.f33d);let eq39_e1093_d_n8: f64 = (eq39_e1091 * l.f33e);let eq39_e1094: f64 = (-eq39_e1093);
        (eq39_e1094, (-eq39_e1093_d_n3), (-eq39_e1093_d_n4), (-eq39_e1093_d_n5), (-eq39_e1093_d_n6), (-eq39_e1093_d_n7), (-eq39_e1093_d_n8),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1096;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            None,
            multiplicity * (eq39_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq39_e1096_d_n3), multiplicity * (eq39_e1096_d_n4), multiplicity * (eq39_e1096_d_n5), multiplicity * (eq39_e1096_d_n6), multiplicity * (eq39_e1096_d_n7), multiplicity * (eq39_e1096_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq40_e1102, eq40_e1102_d_n4,) = {
    if (l.f294 != 0.0) {
        let eq40_e1100: f64 = ((nv4 - 0.0) * l.f259);
        (eq40_e1100, l.f259,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1102;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1102_d_n4),
        );
        let (eq41_e1109, eq41_e1109_d_n4,) = {
    if (l.f294 != 0.0) {
        let eq41_e1106: f64 = ((nv4 - 0.0) * l.fa2);let eq41_e1107: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq41_e1106);
        (eq41_e1107, (l.fa2 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1109;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq41_value),
            4,
            multiplicity * (eq41_e1109_d_n4),
        );
        let (eq42_e1114,) = {
    if (l.f294 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e1114;
        stamper.stamp_potential_const_local(
            4,
            eq42_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let eq12_e876_q: f64 = l.f571;let eq12_e877: f64 = (l.fc6 * l.f571);let eq12_e877_d_n3: f64 = (l.fc6 * l.f572);let eq12_e877_d_n4: f64 = (l.fc6 * l.f573);let eq12_e877_d_n5: f64 = (l.fc6 * l.f574);let eq12_e877_d_n6: f64 = (l.fc6 * l.f575);let eq12_e877_d_n7: f64 = (l.fc6 * l.f576);let eq12_e877_d_n8: f64 = (l.fc6 * l.f577);let eq12_e877_q: f64 = (l.fc6 * eq12_e876_q);let eq12_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq12_e877_d_n3, eq12_e877_d_n4, eq12_e877_d_n5, eq12_e877_d_n6, eq12_e877_d_n7, eq12_e877_d_n8];let eq12_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(6),
            &eq12_reactive_node_derivatives,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );let eq13_e879_q: f64 = l.f595;let eq13_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, l.f596, l.f597, l.f598, l.f599, l.f59a, l.f59b];let eq13_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq13_reactive_node_derivatives,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );let eq14_e882_q: f64 = l.f547;let eq14_e883: f64 = (l.fc6 * l.f547);let eq14_e883_d_n3: f64 = (l.fc6 * l.f548);let eq14_e883_d_n4: f64 = (l.fc6 * l.f549);let eq14_e883_d_n5: f64 = (l.fc6 * l.f54a);let eq14_e883_d_n6: f64 = (l.fc6 * l.f54b);let eq14_e883_d_n7: f64 = (l.fc6 * l.f54c);let eq14_e883_d_n8: f64 = (l.fc6 * l.f54d);let eq14_e883_q: f64 = (l.fc6 * eq14_e882_q);let eq14_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq14_e883_d_n3, eq14_e883_d_n4, eq14_e883_d_n5, eq14_e883_d_n6, eq14_e883_d_n7, eq14_e883_d_n8];let eq14_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(3),
            Some(6),
            &eq14_reactive_node_derivatives,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );let eq15_e885_q: f64 = l.f5a9;let eq15_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, l.f5aa, l.f5ab, l.f5ac, l.f5ad, l.f5ae, l.f5af];let eq15_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(6),
            &eq15_reactive_node_derivatives,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );let eq16_e887_q: f64 = l.f58d;let eq16_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, l.f58e, l.f58f, l.f590, l.f591, l.f592, l.f593];let eq16_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(7),
            Some(5),
            &eq16_reactive_node_derivatives,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );let eq17_e890_q: f64 = l.f60d;let eq17_e891: f64 = (l.fc6 * l.f60d);let eq17_e891_d_n3: f64 = (l.fc6 * l.f60e);let eq17_e891_d_n4: f64 = (l.fc6 * l.f60f);let eq17_e891_d_n5: f64 = (l.fc6 * l.f610);let eq17_e891_d_n6: f64 = (l.fc6 * l.f611);let eq17_e891_d_n7: f64 = (l.fc6 * l.f612);let eq17_e891_d_n8: f64 = (l.fc6 * l.f613);let eq17_e891_q: f64 = (l.fc6 * eq17_e890_q);let eq17_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq17_e891_d_n3, eq17_e891_d_n4, eq17_e891_d_n5, eq17_e891_d_n6, eq17_e891_d_n7, eq17_e891_d_n8];let eq17_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(3),
            &eq17_reactive_node_derivatives,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );let eq18_e894_q: f64 = l.f569;let eq18_e895: f64 = (l.fc6 * l.f569);let eq18_e895_d_n3: f64 = (l.fc6 * l.f56a);let eq18_e895_d_n4: f64 = (l.fc6 * l.f56b);let eq18_e895_d_n5: f64 = (l.fc6 * l.f56c);let eq18_e895_d_n6: f64 = (l.fc6 * l.f56d);let eq18_e895_d_n7: f64 = (l.fc6 * l.f56e);let eq18_e895_d_n8: f64 = (l.fc6 * l.f56f);let eq18_e895_q: f64 = (l.fc6 * eq18_e894_q);let eq18_reactive_node_derivatives: [f64; 9] = [0.0, 0.0, 0.0, eq18_e895_d_n3, eq18_e895_d_n4, eq18_e895_d_n5, eq18_e895_d_n6, eq18_e895_d_n7, eq18_e895_d_n8];let eq18_reactive_branch_derivatives: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(3),
            &eq18_reactive_node_derivatives,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1109, eq41_e1109_d_n4, eq41_e1109_q,) = {
    if (l.f294 != 0.0) {
        let eq41_e1106: f64 = ((nv4 - 0.0) * l.fa2);let eq41_e1107_q: f64 = eq41_e1106;
        (eq41_e1106, l.fa2, eq41_e1107_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (eq41_e1109_d_n4),
        );
    }
}
