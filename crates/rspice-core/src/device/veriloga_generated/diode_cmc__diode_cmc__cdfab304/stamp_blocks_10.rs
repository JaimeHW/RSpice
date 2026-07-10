#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_160(
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t1c: f64 = (l.fe * l.f719);let t1d: f64 = (t1c * l.f6c0);let t1e: f64 = (l.fe * l.f725);let t1f: f64 = (t1d - t1e);let t20: f64 = (l.f18 * l.f731);let t21: f64 = (0.5 * t20);let t22: f64 = (t1f + t21);(l.f5d3, l.f5d6, l.f5d7, ) = (t22, (((((l.fe * l.f71c) * l.f6c0) + (t1c * l.f6c3)) - (l.fe * l.f728)) + (0.5 * ((l.f1b * l.f731) + (l.f18 * l.f734)))), (((((l.fe * l.f71d) * l.f6c0) + (t1c * l.f6c4)) - (l.fe * l.f729)) + (0.5 * ((l.f1c * l.f731) + (l.f18 * l.f735)))), );let t23: f64 = (l.f5b3 - 1.0);let t24: f64 = (t23 * l.f5a7);(l.f7fa, l.f7fd, l.f7fe, ) = (t24, ((l.f5b6 * l.f5a7) + (t23 * l.f5aa)), ((l.f5b7 * l.f5a7) + (t23 * l.f5ab)), );let t25: f64 = (l.f7fa * l.f7fa);(l.f810, l.f813, l.f814, ) = (t25, ((l.f7fd * l.f7fa) + (l.f7fa * l.f7fd)), ((l.f7fe * l.f7fa) + (l.f7fa * l.f7fe)), );}
        let t26: f64 = if l.f7fa > 0.0 { 1.0 } else { 0.0 };l.f42e = t26;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f42e != 0.0)) {let t27: f64 = (l.f62b * l.f7fa);let t28: f64 = (1.0 + t27);let t29: f64 = (1.0 / t28);(l.f6e1, l.f6e4, l.f6e5, ) = (t29, (-((l.f62b * l.f7fd) / (t28 * t28))), (-((l.f62b * l.f7fe) / (t28 * t28))), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f42e == 0.0)) {let t2a: f64 = (l.f62b * l.f7fa);let t2b: f64 = (1.0 - t2a);let t2c: f64 = (1.0 / t2b);(l.f6e1, l.f6e4, l.f6e5, ) = (t2c, (-((-(l.f62b * l.f7fd)) / (t2b * t2b))), (-((-(l.f62b * l.f7fe)) / (t2b * t2b))), );}
        let t2d: f64 = (-l.f810);let t2e: f64 = (t2d + l.f5d3);let t2f: f64 = (-230.25850929940458);let t30: f64 = if t2e > t2f { 1.0 } else { 0.0 };l.f430 = t30;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f430 != 0.0)) {let t31: f64 = (-l.f810);let t32: f64 = (t31 + l.f5d3);let t33: f64 = (t32).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t33, (t33 * ((-l.f813) + l.f5d6)), (t33 * ((-l.f814) + l.f5d7)), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f430 == 0.0)) {let t34: f64 = (-230.25850929940458);let t35: f64 = (-l.f810);let t36: f64 = (t35 + l.f5d3);let t37: f64 = (t34 - t36);let t38: f64 = (-230.25850929940458);let t39: f64 = (-l.f810);let t3a: f64 = (t39 + l.f5d3);let t3b: f64 = (t38 - t3a);let t3c: f64 = (-230.25850929940458);let t3d: f64 = (-l.f810);let t3e: f64 = (t3d + l.f5d3);let t3f: f64 = (t3c - t3e);let t40: f64 = (t3f * 0.3333333333333333);let t41: f64 = (1.0 + t40);let t42: f64 = (t3b * t41);let t43: f64 = (0.5 * t42);let t44: f64 = (1.0 + t43);let t45: f64 = (t37 * t44);let t46: f64 = (1.0 + t45);let t47: f64 = (1e-100 / t46);(l.f6fb, l.f6fe, l.f6ff, ) = (t47, (-((1e-100 * (((-((-l.f813) + l.f5d6)) * t44) + (t37 * (0.5 * (((-((-l.f813) + l.f5d6)) * t41) + (t3b * ((-((-l.f813) + l.f5d6)) * 0.3333333333333333))))))) / (t46 * t46))), (-((1e-100 * (((-((-l.f814) + l.f5d7)) * t44) + (t37 * (0.5 * (((-((-l.f814) + l.f5d7)) * t41) + (t3b * ((-((-l.f814) + l.f5d7)) * 0.3333333333333333))))))) / (t46 * t46))), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t48: f64 = (0.29214664 * l.f6e1);let t49: f64 = (l.f6e1 * l.f6e1);let t4a: f64 = (l.f16 * t49);let t4b: f64 = (t48 + t4a);let t4c: f64 = (l.f6e1 * l.f6e1);let t4d: f64 = (t4c * l.f6e1);let t4e: f64 = (l.f2a * t4d);let t4f: f64 = (t4b + t4e);let t50: f64 = (t4f * l.f6fb);(l.f6d, l.f70, l.f71, ) = (t50, (((((0.29214664 * l.f6e4) + (l.f16 * ((l.f6e4 * l.f6e1) + (l.f6e1 * l.f6e4)))) + (l.f2a * ((((l.f6e4 * l.f6e1) + (l.f6e1 * l.f6e4)) * l.f6e1) + (t4c * l.f6e4)))) * l.f6fb) + (t4f * l.f6fe)), (((((0.29214664 * l.f6e5) + (l.f16 * ((l.f6e5 * l.f6e1) + (l.f6e1 * l.f6e5)))) + (l.f2a * ((((l.f6e5 * l.f6e1) + (l.f6e1 * l.f6e5)) * l.f6e1) + (t4c * l.f6e5)))) * l.f6fb) + (t4f * l.f6ff)), );}
        let t51: f64 = if l.f7fa > 0.0 { 1.0 } else { 0.0 };l.f432 = t51;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f432 != 0.0)) {(l.f73, l.f76, l.f77, ) = (l.f6d, l.f70, l.f71, );}
        let t52: f64 = (-230.25850929940458);let t53: f64 = if l.f5d3 > t52 { 1.0 } else { 0.0 };l.f434 = t53;
        if (((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f432 == 0.0)) && (l.f434 != 0.0)) {let t54: f64 = (l.f5d3).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t54, (t54 * l.f5d6), (t54 * l.f5d7), );}
        if (((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f432 == 0.0)) && (l.f434 == 0.0)) {let t55: f64 = (-230.25850929940458);let t56: f64 = (t55 - l.f5d3);let t57: f64 = (-230.25850929940458);let t58: f64 = (t57 - l.f5d3);let t59: f64 = (-230.25850929940458);let t5a: f64 = (t59 - l.f5d3);let t5b: f64 = (t5a * 0.3333333333333333);let t5c: f64 = (1.0 + t5b);let t5d: f64 = (t58 * t5c);let t5e: f64 = (0.5 * t5d);let t5f: f64 = (1.0 + t5e);let t60: f64 = (t56 * t5f);let t61: f64 = (1.0 + t60);let t62: f64 = (1e-100 / t61);(l.f6fb, l.f6fe, l.f6ff, ) = (t62, (-((1e-100 * (((-l.f5d6) * t5f) + (t56 * (0.5 * (((-l.f5d6) * t5c) + (t58 * ((-l.f5d6) * 0.3333333333333333))))))) / (t61 * t61))), (-((1e-100 * (((-l.f5d7) * t5f) + (t56 * (0.5 * (((-l.f5d7) * t5c) + (t58 * ((-l.f5d7) * 0.3333333333333333))))))) / (t61 * t61))), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) && (l.f432 == 0.0)) {let t63: f64 = (2.0 * l.f6fb);let t64: f64 = (t63 - l.f6d);(l.f73, l.f76, l.f77, ) = (t64, ((2.0 * l.f6fe) - l.f70), ((2.0 * l.f6ff) - l.f71), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t65: f64 = (1.772453850905516 * 0.5);let t66: f64 = (l.fe * l.f73);let t67: f64 = (t66 / l.f5a7);let t68: f64 = (t65 * t67);(l.fd5, l.fd8, l.fd9, ) = (t68, (t65 * ((((l.fe * l.f76) * l.f5a7) - (t66 * l.f5aa)) / (l.f5a7 * l.f5a7))), (t65 * ((((l.fe * l.f77) * l.f5a7) - (t66 * l.f5ab)) / (l.f5a7 * l.f5a7))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_161(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f42a == 0.0)) {let t69: f64 = (l.f8 * l.fd5);let t6a: f64 = (t69 * l.f7f4);let t6b: f64 = (l.f3f * t6a);(l.f598, l.f59b, l.f59c, ) = (t6b, (l.f3f * ((((l.fb * l.fd5) + (l.f8 * l.fd8)) * l.f7f4) + (t69 * l.f7f7))), (l.f3f * ((((l.fc * l.fd5) + (l.f8 * l.fd9)) * l.f7f4) + (t69 * l.f7f8))), );}
        let t6c: f64 = if l.f24 == 0.0 { 1.0 } else { 0.0 };l.f436 = t6c;
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 != 0.0)) {(l.f528, l.f52b, l.f52c, ) = (0.0, 0.0, 0.0, );}
        let t6d: f64 = if l.f623 == 0.5 { 1.0 } else { 0.0 };l.f438 = t6d;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) && (l.f438 != 0.0)) {let t6e: f64 = (l.f771 - l.f74f);let t6f: f64 = (t6e * l.f773);let t70: f64 = (t6f).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t70, (((-l.f752) * l.f773) / (2.0 * t70)), (((-l.f753) * l.f773) / (2.0 * t70)), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) && (l.f438 == 0.0)) {let t71: f64 = (l.f771 - l.f74f);let t72: f64 = (t71 * l.f773);let t73: f64 = (t72).powf(l.f623);(l.f6fb, l.f6fe, l.f6ff, ) = (t73, if 0.0 == 0.0 && ((l.f623) as f64).is_finite() && ((l.f623) as f64).fract() == 0.0 { if l.f623 == 0.0 { 0.0 } else { (l.f623 * ((t72).powf(l.f623 - 1.0) * ((-l.f752) * l.f773))) } } else { (t73 * (l.f623 * (((-l.f752) * l.f773) / t72))) }, if 0.0 == 0.0 && ((l.f623) as f64).is_finite() && ((l.f623) as f64).fract() == 0.0 { if l.f623 == 0.0 { 0.0 } else { (l.f623 * ((t72).powf(l.f623 - 1.0) * ((-l.f753) * l.f773))) } } else { (t73 * (l.f623 * (((-l.f753) * l.f773) / t72))) }, );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) {let t74: f64 = (l.f771 - l.f74f);let t75: f64 = (t74 * l.f7da);let t76: f64 = (t75 / l.f6fb);let t77: f64 = (l.f611 * t76);(l.fb5, l.fb8, l.fb9, ) = (t77, (l.f611 * (((((-l.f752) * l.f7da) * l.f6fb) - (t75 * l.f6fe)) / (l.f6fb * l.f6fb))), (l.f611 * (((((-l.f753) * l.f7da) * l.f6fb) - (t75 * l.f6ff)) / (l.f6fb * l.f6fb))), );}
        let t78: f64 = (-l.fa1);let t79: f64 = (t78 / l.fb5);let t7a: f64 = (t79).abs();let t7b: f64 = if t7a < 230.25850929940458 { 1.0 } else { 0.0 };l.f43a = t7b;
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) && (l.f43a != 0.0)) {let t7c: f64 = (-l.fa1);let t7d: f64 = (t7c / l.fb5);let t7e: f64 = (t7d).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t7e, (t7e * (-((t7c * l.fb8) / (l.fb5 * l.fb5)))), (t7e * (-((t7c * l.fb9) / (l.fb5 * l.fb5)))), );}
        let t7f: f64 = (-l.fa1);let t80: f64 = (t7f / l.fb5);let t81: f64 = (-230.25850929940458);let t82: f64 = if t80 < t81 { 1.0 } else { 0.0 };l.f43c = t82;
        if (((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) && (l.f43a == 0.0)) && (l.f43c != 0.0)) {let t83: f64 = (-230.25850929940458);let t84: f64 = (-l.fa1);let t85: f64 = (t84 / l.fb5);let t86: f64 = (t83 - t85);let t87: f64 = (-230.25850929940458);let t88: f64 = (-l.fa1);let t89: f64 = (t88 / l.fb5);let t8a: f64 = (t87 - t89);let t8b: f64 = (-230.25850929940458);let t8c: f64 = (-l.fa1);let t8d: f64 = (t8c / l.fb5);let t8e: f64 = (t8b - t8d);let t8f: f64 = (t8e * 0.3333333333333333);let t90: f64 = (1.0 + t8f);let t91: f64 = (t8a * t90);let t92: f64 = (0.5 * t91);let t93: f64 = (1.0 + t92);let t94: f64 = (t86 * t93);let t95: f64 = (1.0 + t94);let t96: f64 = (1e-100 / t95);(l.f6fb, l.f6fe, l.f6ff, ) = (t96, (-((1e-100 * (((-(-((t84 * l.fb8) / (l.fb5 * l.fb5)))) * t93) + (t86 * (0.5 * (((-(-((t88 * l.fb8) / (l.fb5 * l.fb5)))) * t90) + (t8a * ((-(-((t8c * l.fb8) / (l.fb5 * l.fb5)))) * 0.3333333333333333))))))) / (t95 * t95))), (-((1e-100 * (((-(-((t84 * l.fb9) / (l.fb5 * l.fb5)))) * t93) + (t86 * (0.5 * (((-(-((t88 * l.fb9) / (l.fb5 * l.fb5)))) * t90) + (t8a * ((-(-((t8c * l.fb9) / (l.fb5 * l.fb5)))) * 0.3333333333333333))))))) / (t95 * t95))), );}
        if (((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) && (l.f43a == 0.0)) && (l.f43c == 0.0)) {let t97: f64 = (-l.fa1);let t98: f64 = (t97 / l.fb5);let t99: f64 = (t98 - 230.25850929940458);let t9a: f64 = (-l.fa1);let t9b: f64 = (t9a / l.fb5);let t9c: f64 = (t9b - 230.25850929940458);let t9d: f64 = (-l.fa1);let t9e: f64 = (t9d / l.fb5);let t9f: f64 = (t9e - 230.25850929940458);let ta0: f64 = (t9f * 0.3333333333333333);let ta1: f64 = (1.0 + ta0);let ta2: f64 = (t9c * ta1);let ta3: f64 = (0.5 * ta2);let ta4: f64 = (1.0 + ta3);let ta5: f64 = (t99 * ta4);let ta6: f64 = (1.0 + ta5);let ta7: f64 = (1e100 * ta6);(l.f6fb, l.f6fe, l.f6ff, ) = (ta7, (1e100 * (((-((t97 * l.fb8) / (l.fb5 * l.fb5))) * ta4) + (t99 * (0.5 * (((-((t9a * l.fb8) / (l.fb5 * l.fb5))) * ta1) + (t9c * ((-((t9d * l.fb8) / (l.fb5 * l.fb5))) * 0.3333333333333333))))))), (1e100 * (((-((t97 * l.fb9) / (l.fb5 * l.fb5))) * ta4) + (t99 * (0.5 * (((-((t9a * l.fb9) / (l.fb5 * l.fb5))) * ta1) + (t9c * ((-((t9d * l.fb9) / (l.fb5 * l.fb5))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f436 == 0.0)) {let ta8: f64 = (l.f745 * l.fb5);let ta9: f64 = (ta8 * l.fb5);let taa: f64 = (ta9 * l.f6fb);let tab: f64 = (l.f24 * taa);(l.f528, l.f52b, l.f52c, ) = (tab, (l.f24 * ((((((l.f746 * l.fb5) + (l.f745 * l.fb8)) * l.fb5) + (ta8 * l.fb8)) * l.f6fb) + (ta9 * l.f6fe))), (l.f24 * ((((((l.f747 * l.fb5) + (l.f745 * l.fb9)) * l.fb5) + (ta8 * l.fb9)) * l.f6fb) + (ta9 * l.f6ff))), );}
        let tac: f64 = if ((l.f783 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f43e = tac;
        if (((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f43e != 0.0)) {(l.fad, l.fb0, l.fb1, ) = (1.0, 0.0, 0.0, );}
        let tad: f64 = (-l.f2);let tae: f64 = (tad * l.f783);let taf: f64 = if l.f749 > tae { 1.0 } else { 0.0 };l.f440 = taf;let tb0: f64 = if l.f625 == 4.0 { 1.0 } else { 0.0 };l.f442 = tb0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_162(
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f43e == 0.0)) && (l.f440 != 0.0)) && (l.f442 != 0.0)) {let tb1: f64 = (l.f749 * l.f787);let tb2: f64 = (tb1).abs();let tb3: f64 = (l.f749 * l.f787);let tb4: f64 = (tb3).abs();let tb5: f64 = (tb2 * tb4);let tb6: f64 = (l.f749 * l.f787);let tb7: f64 = (tb6).abs();let tb8: f64 = (tb5 * tb7);let tb9: f64 = (l.f749 * l.f787);let tba: f64 = (tb9).abs();let tbb: f64 = (tb8 * tba);(l.f6fb, l.f6fe, l.f6ff, ) = (tbb, ((((((if tb1 >= 0.0 { (l.f74c * l.f787) } else { (-(l.f74c * l.f787)) } * tb4) + (tb2 * if tb3 >= 0.0 { (l.f74c * l.f787) } else { (-(l.f74c * l.f787)) })) * tb7) + (tb5 * if tb6 >= 0.0 { (l.f74c * l.f787) } else { (-(l.f74c * l.f787)) })) * tba) + (tb8 * if tb9 >= 0.0 { (l.f74c * l.f787) } else { (-(l.f74c * l.f787)) })), ((((((if tb1 >= 0.0 { (l.f74d * l.f787) } else { (-(l.f74d * l.f787)) } * tb4) + (tb2 * if tb3 >= 0.0 { (l.f74d * l.f787) } else { (-(l.f74d * l.f787)) })) * tb7) + (tb5 * if tb6 >= 0.0 { (l.f74d * l.f787) } else { (-(l.f74d * l.f787)) })) * tba) + (tb8 * if tb9 >= 0.0 { (l.f74d * l.f787) } else { (-(l.f74d * l.f787)) })), );}
        if (((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f43e == 0.0)) && (l.f440 != 0.0)) && (l.f442 == 0.0)) {let tbc: f64 = (l.f749 * l.f787);let tbd: f64 = (tbc).abs();let tbe: f64 = (tbd).powf(l.f625);(l.f6fb, l.f6fe, l.f6ff, ) = (tbe, if 0.0 == 0.0 && ((l.f625) as f64).is_finite() && ((l.f625) as f64).fract() == 0.0 { if l.f625 == 0.0 { 0.0 } else { (l.f625 * ((tbd).powf(l.f625 - 1.0) * if tbc >= 0.0 { (l.f74c * l.f787) } else { (-(l.f74c * l.f787)) })) } } else { (tbe * (l.f625 * (if tbc >= 0.0 { (l.f74c * l.f787) } else { (-(l.f74c * l.f787)) } / tbd))) }, if 0.0 == 0.0 && ((l.f625) as f64).is_finite() && ((l.f625) as f64).fract() == 0.0 { if l.f625 == 0.0 { 0.0 } else { (l.f625 * ((tbd).powf(l.f625 - 1.0) * if tbc >= 0.0 { (l.f74d * l.f787) } else { (-(l.f74d * l.f787)) })) } } else { (tbe * (l.f625 * (if tbc >= 0.0 { (l.f74d * l.f787) } else { (-(l.f74d * l.f787)) } / tbd))) }, );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f43e == 0.0)) && (l.f440 != 0.0)) {let tbf: f64 = (1.0 - l.f6fb);let tc0: f64 = (1.0 / tbf);(l.fad, l.fb0, l.fb1, ) = (tc0, (-((-l.f6fe) / (tbf * tbf))), (-((-l.f6ff) / (tbf * tbf))), );}
        if ((((l.f3e0 == 0.0) && (l.f420 == 0.0)) && (l.f43e == 0.0)) && (l.f440 == 0.0)) {let tc1: f64 = (l.f2 * l.f783);let tc2: f64 = (l.f749 + tc1);let tc3: f64 = (tc2 * l.f6ba);let tc4: f64 = (l.fc3 + tc3);(l.fad, l.fb0, l.fb1, ) = (tc4, (l.f74c * l.f6ba), (l.f74d * l.f6ba), );}
        if ((l.f3e0 == 0.0) && (l.f420 == 0.0)) {let tc5: f64 = (l.f52e + l.f592);let tc6: f64 = (tc5 + l.f598);let tc7: f64 = (tc6 + l.f528);let tc8: f64 = (tc7 * l.fad);(l.f562, l.f563, l.f564, ) = (tc8, (((((l.f533 + l.f595) + l.f59b) + l.f52b) * l.fad) + (tc7 * l.fb0)), (((((l.f534 + l.f596) + l.f59c) + l.f52c) * l.fad) + (tc7 * l.fb1)), );let tc9: f64 = (l.f592 + l.f598);let tca: f64 = (tc9 + l.f528);let tcb: f64 = (tca * l.fad);(l.f552, l.f553, l.f554, ) = (tcb, ((((l.f595 + l.f59b) + l.f52b) * l.fad) + (tca * l.fb0)), ((((l.f596 + l.f59c) + l.f52c) * l.fad) + (tca * l.fb1)), );}
        let tcc: f64 = if l.f5b1 == 0.0 { 1.0 } else { 0.0 };l.f444 = tcc;
        if ((l.f3e0 == 0.0) && (l.f444 != 0.0)) {(l.f576, l.f577, l.f578, ) = (0.0, 0.0, 0.0, );(l.f55a, l.f55b, l.f55c, ) = (0.0, 0.0, 0.0, );(l.f694, l.f695, l.f696, ) = (0.0, 0.0, 0.0, );}
        let tcd: f64 = if l.f60f == 0.5 { 1.0 } else { 0.0 };l.f446 = tcd;
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f446 != 0.0)) {let tce: f64 = (l.f795 * l.f76d);let tcf: f64 = (1.0 - tce);let td0: f64 = (tcf).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (td0, ((-(l.f798 * l.f76d)) / (2.0 * td0)), ((-(l.f799 * l.f76d)) / (2.0 * td0)), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f446 == 0.0)) {let td1: f64 = (l.f795 * l.f76d);let td2: f64 = (1.0 - td1);let td3: f64 = (td2).powf(l.f60f);(l.f6fb, l.f6fe, l.f6ff, ) = (td3, if 0.0 == 0.0 && ((l.f60f) as f64).is_finite() && ((l.f60f) as f64).fract() == 0.0 { if l.f60f == 0.0 { 0.0 } else { (l.f60f * ((td2).powf(l.f60f - 1.0) * (-(l.f798 * l.f76d)))) } } else { (td3 * (l.f60f * ((-(l.f798 * l.f76d)) / td2))) }, if 0.0 == 0.0 && ((l.f60f) as f64).is_finite() && ((l.f60f) as f64).fract() == 0.0 { if l.f60f == 0.0 { 0.0 } else { (l.f60f * ((td2).powf(l.f60f - 1.0) * (-(l.f799 * l.f76d)))) } } else { (td3 * (l.f60f * ((-(l.f799 * l.f76d)) / td2))) }, );}
        if ((l.f3e0 == 0.0) && (l.f444 == 0.0)) {let td4: f64 = (1.0 - l.f6fb);let td5: f64 = (l.f6a2 * td4);let td6: f64 = (l.f745 - l.f795);let td7: f64 = (l.f69c * td6);let td8: f64 = (td5 + td7);(l.f694, l.f695, l.f696, ) = (td8, ((l.f6a2 * (-l.f6fe)) + (l.f69c * (l.f746 - l.f798))), ((l.f6a2 * (-l.f6ff)) + (l.f69c * (l.f747 - l.f799))), );let td9: f64 = (l.f54c * l.f53e);(l.f52e, l.f533, l.f534, ) = (td9, (l.f54c * l.f53f), (l.f54c * l.f540), );}
        let tda: f64 = if ((l.f3d == 0.0) && (l.f43 == 0.0)) { 1.0 } else { 0.0 };l.f448 = tda;
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 != 0.0)) {(l.f757, l.f75a, l.f75b, ) = (0.0, 0.0, 0.0, );(l.f7e8, l.f7eb, l.f7ec, ) = (0.0, 0.0, 0.0, );(l.f7d0, l.f7d3, l.f7d4, ) = (0.0, 0.0, 0.0, );(l.f8, l.fb, l.fc, ) = (0.0, 0.0, 0.0, );(l.f592, l.f595, l.f596, ) = (0.0, 0.0, 0.0, );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) {let tdb: f64 = (l.f77d - l.f7a1);(l.f757, l.f75a, l.f75b, ) = (tdb, (-l.f7a4), (-l.f7a5), );let tdc: f64 = (l.f713 / l.f757);let tdd: f64 = (1.0 - tdc);let tde: f64 = (tdd).sqrt();let tdf: f64 = (1.0 - tde);(l.f7ee, l.f7f1, l.f7f2, ) = (tdf, (-((-(((l.f716 * l.f757) - (l.f713 * l.f75a)) / (l.f757 * l.f757))) / (2.0 * tde))), (-((-(((l.f717 * l.f757) - (l.f713 * l.f75b)) / (l.f757 * l.f757))) / (2.0 * tde))), );}
        let te0: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f44a = te0;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) && (l.f44a != 0.0)) {(l.f65, l.f68, l.f69, ) = (0.0, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) && (l.f44a == 0.0)) {let te1: f64 = (l.f7ee * l.f7ee);let te2: f64 = (l.f7ee).ln();let te3: f64 = (te1 * te2);let te4: f64 = (1.0 - l.f7ee);let te5: f64 = (te3 / te4);let te6: f64 = (te5 + l.f7ee);let te7: f64 = (2.0 * l.f653);let te8: f64 = (1.0 - te7);let te9: f64 = (te6 * te8);(l.f65, l.f68, l.f69, ) = (te9, (((((((((l.f7f1 * l.f7ee) + (l.f7ee * l.f7f1)) * te2) + (te1 * (l.f7f1 / l.f7ee))) * te4) - (te3 * (-l.f7f1))) / (te4 * te4)) + l.f7f1) * te8), (((((((((l.f7f2 * l.f7ee) + (l.f7ee * l.f7f2)) * te2) + (te1 * (l.f7f2 / l.f7ee))) * te4) - (te3 * (-l.f7f2))) / (te4 * te4)) + l.f7f2) * te8), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) {let tea: f64 = (l.f7ee + l.f65);(l.f7e8, l.f7eb, l.f7ec, ) = (tea, (l.f7f1 + l.f68), (l.f7f2 + l.f69), );}
        let teb: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f44c = teb;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) && (l.f44c != 0.0)) {let tec: f64 = (l.f757 * l.f77b);let ted: f64 = (tec).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (ted, ((l.f75a * l.f77b) / (2.0 * ted)), ((l.f75b * l.f77b) / (2.0 * ted)), );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) && (l.f44c == 0.0)) {let tee: f64 = (l.f757 * l.f77b);let tef: f64 = (tee).powf(l.f653);(l.f6fb, l.f6fe, l.f6ff, ) = (tef, if 0.0 == 0.0 && ((l.f653) as f64).is_finite() && ((l.f653) as f64).fract() == 0.0 { if l.f653 == 0.0 { 0.0 } else { (l.f653 * ((tee).powf(l.f653 - 1.0) * (l.f75a * l.f77b))) } } else { (tef * (l.f653 * ((l.f75a * l.f77b) / tee))) }, if 0.0 == 0.0 && ((l.f653) as f64).is_finite() && ((l.f653) as f64).fract() == 0.0 { if l.f653 == 0.0 { 0.0 } else { (l.f653 * ((tee).powf(l.f653 - 1.0) * (l.f75b * l.f77b))) } } else { (tef * (l.f653 * ((l.f75b * l.f77b) / tee))) }, );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) {let tf0: f64 = (l.f7e0 * l.f6fb);(l.f7d0, l.f7d3, l.f7d4, ) = (tf0, (l.f7e0 * l.f6fe), (l.f7e0 * l.f6ff), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_163(
        l: &mut StampLocals,
    ) {
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f448 == 0.0)) {let tf1: f64 = (l.f824 - 1.0);let tf2: f64 = (tf1 * l.f7d0);let tf3: f64 = (l.fd1 * tf2);(l.f8, l.fb, l.fc, ) = (tf3, (l.fd1 * ((l.f827 * l.f7d0) + (tf1 * l.f7d3))), (l.fd1 * ((l.f828 * l.f7d0) + (tf1 * l.f7d4))), );let tf4: f64 = (l.f8 * l.f7e8);let tf5: f64 = (l.f3d * tf4);(l.f592, l.f595, l.f596, ) = (tf5, (l.f3d * ((l.fb * l.f7e8) + (l.f8 * l.f7eb))), (l.f3d * ((l.fc * l.f7e8) + (l.f8 * l.f7ec))), );}
        let tf6: f64 = if l.f43 == 0.0 { 1.0 } else { 0.0 };l.f44e = tf6;
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e != 0.0)) {(l.f598, l.f59b, l.f59c, ) = (0.0, 0.0, 0.0, );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) {let tf7: f64 = (l.f7d0 * l.f60f);let tf8: f64 = (tf7 / l.f757);let tf9: f64 = (l.f22 * tf8);(l.f18, l.f1b, l.f1c, ) = (tf9, (l.f22 * ((((l.f7d3 * l.f60f) * l.f757) - (tf7 * l.f75a)) / (l.f757 * l.f757))), (l.f22 * ((((l.f7d4 * l.f60f) * l.f757) - (tf7 * l.f75b)) / (l.f757 * l.f757))), );let tfa: f64 = (0.666666666666667 * l.f12);let tfb: f64 = (tfa / l.f18);(l.f719, l.f71c, l.f71d, ) = (tfb, (-((tfa * l.f1b) / (l.f18 * l.f18))), (-((tfa * l.f1c) / (l.f18 * l.f18))), );let tfc: f64 = (l.f719 * l.f719);(l.f72b, l.f72e, l.f72f, ) = (tfc, ((l.f71c * l.f719) + (l.f719 * l.f71c)), ((l.f71d * l.f719) + (l.f719 * l.f71d)), );let tfd: f64 = (l.f72b * l.f72b);let tfe: f64 = (l.f72b * l.f72b);let tff: f64 = (tfe + 1.0);let t100: f64 = (tfd / tff);let t101: f64 = (t100).sqrt();(l.f725, l.f728, l.f729, ) = (t101, ((((((l.f72e * l.f72b) + (l.f72b * l.f72e)) * tff) - (tfd * ((l.f72e * l.f72b) + (l.f72b * l.f72e)))) / (tff * tff)) / (2.0 * t101)), ((((((l.f72f * l.f72b) + (l.f72b * l.f72f)) * tff) - (tfd * ((l.f72f * l.f72b) + (l.f72b * l.f72f)))) / (tff * tff)) / (2.0 * t101)), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) {let t102: f64 = (l.f725).abs();let t103: f64 = (t102).sqrt();(l.f6c0, l.f6c3, l.f6c4, ) = (t103, (if l.f725 >= 0.0 { l.f728 } else { (-l.f728) } / (2.0 * t103)), (if l.f725 >= 0.0 { l.f729 } else { (-l.f729) } / (2.0 * t103)), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) {let t104: f64 = (l.f725 * l.f6c0);(l.f731, l.f734, l.f735, ) = (t104, ((l.f728 * l.f6c0) + (l.f725 * l.f6c3)), ((l.f729 * l.f6c0) + (l.f725 * l.f6c4)), );}
        let t105: f64 = (-l.f653);let t106: f64 = (t105 * l.f615);let t107: f64 = (-1.0);let t108: f64 = if t106 == t107 { 1.0 } else { 0.0 };l.f450 = t108;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f450 != 0.0)) {let t109: f64 = (l.f18 * l.f731);let t10a: f64 = (1.0 + t109);let t10b: f64 = (1.0 / t10a);(l.f7e2, l.f7e5, l.f7e6, ) = (t10b, (-(((l.f1b * l.f731) + (l.f18 * l.f734)) / (t10a * t10a))), (-(((l.f1c * l.f731) + (l.f18 * l.f735)) / (t10a * t10a))), );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f450 == 0.0)) {let t10c: f64 = (l.f18 * l.f731);let t10d: f64 = (1.0 + t10c);let t10e: f64 = (-l.f653);let t10f: f64 = (t10e * l.f615);let t110: f64 = (t10d).powf(t10f);(l.f7e2, l.f7e5, l.f7e6, ) = (t110, if 0.0 == 0.0 && ((t10f) as f64).is_finite() && ((t10f) as f64).fract() == 0.0 { if t10f == 0.0 { 0.0 } else { (t10f * ((t10d).powf(t10f - 1.0) * ((l.f1b * l.f731) + (l.f18 * l.f734)))) } } else { (t110 * (t10f * (((l.f1b * l.f731) + (l.f18 * l.f734)) / t10d))) }, if 0.0 == 0.0 && ((t10f) as f64).is_finite() && ((t10f) as f64).fract() == 0.0 { if t10f == 0.0 { 0.0 } else { (t10f * ((t10d).powf(t10f - 1.0) * ((l.f1c * l.f731) + (l.f18 * l.f735)))) } } else { (t110 * (t10f * (((l.f1c * l.f731) + (l.f18 * l.f735)) / t10d))) }, );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) {let t111: f64 = (l.f7e8 * l.f7e2);let t112: f64 = (l.f7e8 + l.f7e2);let t113: f64 = (t111 / t112);(l.f7f4, l.f7f7, l.f7f8, ) = (t113, (((((l.f7eb * l.f7e2) + (l.f7e8 * l.f7e5)) * t112) - (t111 * (l.f7eb + l.f7e5))) / (t112 * t112)), (((((l.f7ec * l.f7e2) + (l.f7e8 * l.f7e6)) * t112) - (t111 * (l.f7ec + l.f7e6))) / (t112 * t112)), );let t114: f64 = (l.f18 / l.f6c0);let t115: f64 = (0.375 * t114);let t116: f64 = (t115).sqrt();(l.f5a7, l.f5aa, l.f5ab, ) = (t116, ((0.375 * (((l.f1b * l.f6c0) - (l.f18 * l.f6c3)) / (l.f6c0 * l.f6c0))) / (2.0 * t116)), ((0.375 * (((l.f1c * l.f6c0) - (l.f18 * l.f6c4)) / (l.f6c0 * l.f6c0))) / (2.0 * t116)), );let t117: f64 = (l.f719 * l.f6c0);let t118: f64 = (2.0 * t117);let t119: f64 = (t118 - l.f725);(l.f5b3, l.f5b6, l.f5b7, ) = (t119, ((2.0 * ((l.f71c * l.f6c0) + (l.f719 * l.f6c3))) - l.f728), ((2.0 * ((l.f71d * l.f6c0) + (l.f719 * l.f6c4))) - l.f729), );let t11a: f64 = (l.f12 * l.f719);let t11b: f64 = (t11a * l.f6c0);let t11c: f64 = (l.f12 * l.f725);let t11d: f64 = (t11b - t11c);let t11e: f64 = (l.f18 * l.f731);let t11f: f64 = (0.5 * t11e);let t120: f64 = (t11d + t11f);(l.f5d3, l.f5d6, l.f5d7, ) = (t120, (((((l.f12 * l.f71c) * l.f6c0) + (t11a * l.f6c3)) - (l.f12 * l.f728)) + (0.5 * ((l.f1b * l.f731) + (l.f18 * l.f734)))), (((((l.f12 * l.f71d) * l.f6c0) + (t11a * l.f6c4)) - (l.f12 * l.f729)) + (0.5 * ((l.f1c * l.f731) + (l.f18 * l.f735)))), );let t121: f64 = (l.f5b3 - 1.0);let t122: f64 = (t121 * l.f5a7);(l.f7fa, l.f7fd, l.f7fe, ) = (t122, ((l.f5b6 * l.f5a7) + (t121 * l.f5aa)), ((l.f5b7 * l.f5a7) + (t121 * l.f5ab)), );let t123: f64 = (l.f7fa * l.f7fa);(l.f810, l.f813, l.f814, ) = (t123, ((l.f7fd * l.f7fa) + (l.f7fa * l.f7fd)), ((l.f7fe * l.f7fa) + (l.f7fa * l.f7fe)), );}
        let t124: f64 = if l.f7fa > 0.0 { 1.0 } else { 0.0 };l.f452 = t124;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f452 != 0.0)) {let t125: f64 = (l.f62b * l.f7fa);let t126: f64 = (1.0 + t125);let t127: f64 = (1.0 / t126);(l.f6e1, l.f6e4, l.f6e5, ) = (t127, (-((l.f62b * l.f7fd) / (t126 * t126))), (-((l.f62b * l.f7fe) / (t126 * t126))), );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f452 == 0.0)) {let t128: f64 = (l.f62b * l.f7fa);let t129: f64 = (1.0 - t128);let t12a: f64 = (1.0 / t129);(l.f6e1, l.f6e4, l.f6e5, ) = (t12a, (-((-(l.f62b * l.f7fd)) / (t129 * t129))), (-((-(l.f62b * l.f7fe)) / (t129 * t129))), );}
        let t12b: f64 = (-l.f810);let t12c: f64 = (t12b + l.f5d3);let t12d: f64 = (-230.25850929940458);let t12e: f64 = if t12c > t12d { 1.0 } else { 0.0 };l.f454 = t12e;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f454 != 0.0)) {let t12f: f64 = (-l.f810);let t130: f64 = (t12f + l.f5d3);let t131: f64 = (t130).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t131, (t131 * ((-l.f813) + l.f5d6)), (t131 * ((-l.f814) + l.f5d7)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_164(
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f454 == 0.0)) {let t132: f64 = (-230.25850929940458);let t133: f64 = (-l.f810);let t134: f64 = (t133 + l.f5d3);let t135: f64 = (t132 - t134);let t136: f64 = (-230.25850929940458);let t137: f64 = (-l.f810);let t138: f64 = (t137 + l.f5d3);let t139: f64 = (t136 - t138);let t13a: f64 = (-230.25850929940458);let t13b: f64 = (-l.f810);let t13c: f64 = (t13b + l.f5d3);let t13d: f64 = (t13a - t13c);let t13e: f64 = (t13d * 0.3333333333333333);let t13f: f64 = (1.0 + t13e);let t140: f64 = (t139 * t13f);let t141: f64 = (0.5 * t140);let t142: f64 = (1.0 + t141);let t143: f64 = (t135 * t142);let t144: f64 = (1.0 + t143);let t145: f64 = (1e-100 / t144);(l.f6fb, l.f6fe, l.f6ff, ) = (t145, (-((1e-100 * (((-((-l.f813) + l.f5d6)) * t142) + (t135 * (0.5 * (((-((-l.f813) + l.f5d6)) * t13f) + (t139 * ((-((-l.f813) + l.f5d6)) * 0.3333333333333333))))))) / (t144 * t144))), (-((1e-100 * (((-((-l.f814) + l.f5d7)) * t142) + (t135 * (0.5 * (((-((-l.f814) + l.f5d7)) * t13f) + (t139 * ((-((-l.f814) + l.f5d7)) * 0.3333333333333333))))))) / (t144 * t144))), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) {let t146: f64 = (0.29214664 * l.f6e1);let t147: f64 = (l.f6e1 * l.f6e1);let t148: f64 = (l.f16 * t147);let t149: f64 = (t146 + t148);let t14a: f64 = (l.f6e1 * l.f6e1);let t14b: f64 = (t14a * l.f6e1);let t14c: f64 = (l.f2a * t14b);let t14d: f64 = (t149 + t14c);let t14e: f64 = (t14d * l.f6fb);(l.f6d, l.f70, l.f71, ) = (t14e, (((((0.29214664 * l.f6e4) + (l.f16 * ((l.f6e4 * l.f6e1) + (l.f6e1 * l.f6e4)))) + (l.f2a * ((((l.f6e4 * l.f6e1) + (l.f6e1 * l.f6e4)) * l.f6e1) + (t14a * l.f6e4)))) * l.f6fb) + (t14d * l.f6fe)), (((((0.29214664 * l.f6e5) + (l.f16 * ((l.f6e5 * l.f6e1) + (l.f6e1 * l.f6e5)))) + (l.f2a * ((((l.f6e5 * l.f6e1) + (l.f6e1 * l.f6e5)) * l.f6e1) + (t14a * l.f6e5)))) * l.f6fb) + (t14d * l.f6ff)), );}
        let t14f: f64 = if l.f7fa > 0.0 { 1.0 } else { 0.0 };l.f456 = t14f;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f456 != 0.0)) {(l.f73, l.f76, l.f77, ) = (l.f6d, l.f70, l.f71, );}
        let t150: f64 = (-230.25850929940458);let t151: f64 = if l.f5d3 > t150 { 1.0 } else { 0.0 };l.f458 = t151;
        if (((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f456 == 0.0)) && (l.f458 != 0.0)) {let t152: f64 = (l.f5d3).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t152, (t152 * l.f5d6), (t152 * l.f5d7), );}
        if (((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f456 == 0.0)) && (l.f458 == 0.0)) {let t153: f64 = (-230.25850929940458);let t154: f64 = (t153 - l.f5d3);let t155: f64 = (-230.25850929940458);let t156: f64 = (t155 - l.f5d3);let t157: f64 = (-230.25850929940458);let t158: f64 = (t157 - l.f5d3);let t159: f64 = (t158 * 0.3333333333333333);let t15a: f64 = (1.0 + t159);let t15b: f64 = (t156 * t15a);let t15c: f64 = (0.5 * t15b);let t15d: f64 = (1.0 + t15c);let t15e: f64 = (t154 * t15d);let t15f: f64 = (1.0 + t15e);let t160: f64 = (1e-100 / t15f);(l.f6fb, l.f6fe, l.f6ff, ) = (t160, (-((1e-100 * (((-l.f5d6) * t15d) + (t154 * (0.5 * (((-l.f5d6) * t15a) + (t156 * ((-l.f5d6) * 0.3333333333333333))))))) / (t15f * t15f))), (-((1e-100 * (((-l.f5d7) * t15d) + (t154 * (0.5 * (((-l.f5d7) * t15a) + (t156 * ((-l.f5d7) * 0.3333333333333333))))))) / (t15f * t15f))), );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) && (l.f456 == 0.0)) {let t161: f64 = (2.0 * l.f6fb);let t162: f64 = (t161 - l.f6d);(l.f73, l.f76, l.f77, ) = (t162, ((2.0 * l.f6fe) - l.f70), ((2.0 * l.f6ff) - l.f71), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f44e == 0.0)) {let t163: f64 = (1.772453850905516 * 0.5);let t164: f64 = (l.f12 * l.f73);let t165: f64 = (t164 / l.f5a7);let t166: f64 = (t163 * t165);(l.fd5, l.fd8, l.fd9, ) = (t166, (t163 * ((((l.f12 * l.f76) * l.f5a7) - (t164 * l.f5aa)) / (l.f5a7 * l.f5a7))), (t163 * ((((l.f12 * l.f77) * l.f5a7) - (t164 * l.f5ab)) / (l.f5a7 * l.f5a7))), );let t167: f64 = (l.f8 * l.fd5);let t168: f64 = (t167 * l.f7f4);let t169: f64 = (l.f43 * t168);(l.f598, l.f59b, l.f59c, ) = (t169, (l.f43 * ((((l.fb * l.fd5) + (l.f8 * l.fd8)) * l.f7f4) + (t167 * l.f7f7))), (l.f43 * ((((l.fc * l.fd5) + (l.f8 * l.fd9)) * l.f7f4) + (t167 * l.f7f8))), );}
        let t16a: f64 = if l.f28 == 0.0 { 1.0 } else { 0.0 };l.f45a = t16a;
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a != 0.0)) {(l.f528, l.f52b, l.f52c, ) = (0.0, 0.0, 0.0, );}
        let t16b: f64 = if l.f653 == 0.5 { 1.0 } else { 0.0 };l.f45c = t16b;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) && (l.f45c != 0.0)) {let t16c: f64 = (l.f779 - l.f74f);let t16d: f64 = (t16c * l.f77b);let t16e: f64 = (t16d).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t16e, (((-l.f752) * l.f77b) / (2.0 * t16e)), (((-l.f753) * l.f77b) / (2.0 * t16e)), );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) && (l.f45c == 0.0)) {let t16f: f64 = (l.f779 - l.f74f);let t170: f64 = (t16f * l.f77b);let t171: f64 = (t170).powf(l.f653);(l.f6fb, l.f6fe, l.f6ff, ) = (t171, if 0.0 == 0.0 && ((l.f653) as f64).is_finite() && ((l.f653) as f64).fract() == 0.0 { if l.f653 == 0.0 { 0.0 } else { (l.f653 * ((t170).powf(l.f653 - 1.0) * ((-l.f752) * l.f77b))) } } else { (t171 * (l.f653 * (((-l.f752) * l.f77b) / t170))) }, if 0.0 == 0.0 && ((l.f653) as f64).is_finite() && ((l.f653) as f64).fract() == 0.0 { if l.f653 == 0.0 { 0.0 } else { (l.f653 * ((t170).powf(l.f653 - 1.0) * ((-l.f753) * l.f77b))) } } else { (t171 * (l.f653 * (((-l.f753) * l.f77b) / t170))) }, );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) {let t172: f64 = (l.f779 - l.f74f);let t173: f64 = (t172 * l.f7de);let t174: f64 = (t173 / l.f6fb);let t175: f64 = (l.f615 * t174);(l.fb5, l.fb8, l.fb9, ) = (t175, (l.f615 * (((((-l.f752) * l.f7de) * l.f6fb) - (t173 * l.f6fe)) / (l.f6fb * l.f6fb))), (l.f615 * (((((-l.f753) * l.f7de) * l.f6fb) - (t173 * l.f6ff)) / (l.f6fb * l.f6fb))), );}
        let t176: f64 = (-l.fab);let t177: f64 = (t176 / l.fb5);let t178: f64 = (t177).abs();let t179: f64 = if t178 < 230.25850929940458 { 1.0 } else { 0.0 };l.f45e = t179;
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) && (l.f45e != 0.0)) {let t17a: f64 = (-l.fab);let t17b: f64 = (t17a / l.fb5);let t17c: f64 = (t17b).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t17c, (t17c * (-((t17a * l.fb8) / (l.fb5 * l.fb5)))), (t17c * (-((t17a * l.fb9) / (l.fb5 * l.fb5)))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_165(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t17d: f64 = (-l.fab);let t17e: f64 = (t17d / l.fb5);let t17f: f64 = (-230.25850929940458);let t180: f64 = if t17e < t17f { 1.0 } else { 0.0 };l.f460 = t180;
        if (((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) && (l.f45e == 0.0)) && (l.f460 != 0.0)) {let t181: f64 = (-230.25850929940458);let t182: f64 = (-l.fab);let t183: f64 = (t182 / l.fb5);let t184: f64 = (t181 - t183);let t185: f64 = (-230.25850929940458);let t186: f64 = (-l.fab);let t187: f64 = (t186 / l.fb5);let t188: f64 = (t185 - t187);let t189: f64 = (-230.25850929940458);let t18a: f64 = (-l.fab);let t18b: f64 = (t18a / l.fb5);let t18c: f64 = (t189 - t18b);let t18d: f64 = (t18c * 0.3333333333333333);let t18e: f64 = (1.0 + t18d);let t18f: f64 = (t188 * t18e);let t190: f64 = (0.5 * t18f);let t191: f64 = (1.0 + t190);let t192: f64 = (t184 * t191);let t193: f64 = (1.0 + t192);let t194: f64 = (1e-100 / t193);(l.f6fb, l.f6fe, l.f6ff, ) = (t194, (-((1e-100 * (((-(-((t182 * l.fb8) / (l.fb5 * l.fb5)))) * t191) + (t184 * (0.5 * (((-(-((t186 * l.fb8) / (l.fb5 * l.fb5)))) * t18e) + (t188 * ((-(-((t18a * l.fb8) / (l.fb5 * l.fb5)))) * 0.3333333333333333))))))) / (t193 * t193))), (-((1e-100 * (((-(-((t182 * l.fb9) / (l.fb5 * l.fb5)))) * t191) + (t184 * (0.5 * (((-(-((t186 * l.fb9) / (l.fb5 * l.fb5)))) * t18e) + (t188 * ((-(-((t18a * l.fb9) / (l.fb5 * l.fb5)))) * 0.3333333333333333))))))) / (t193 * t193))), );}
        if (((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) && (l.f45e == 0.0)) && (l.f460 == 0.0)) {let t195: f64 = (-l.fab);let t196: f64 = (t195 / l.fb5);let t197: f64 = (t196 - 230.25850929940458);let t198: f64 = (-l.fab);let t199: f64 = (t198 / l.fb5);let t19a: f64 = (t199 - 230.25850929940458);let t19b: f64 = (-l.fab);let t19c: f64 = (t19b / l.fb5);let t19d: f64 = (t19c - 230.25850929940458);let t19e: f64 = (t19d * 0.3333333333333333);let t19f: f64 = (1.0 + t19e);let t1a0: f64 = (t19a * t19f);let t1a1: f64 = (0.5 * t1a0);let t1a2: f64 = (1.0 + t1a1);let t1a3: f64 = (t197 * t1a2);let t1a4: f64 = (1.0 + t1a3);let t1a5: f64 = (1e100 * t1a4);(l.f6fb, l.f6fe, l.f6ff, ) = (t1a5, (1e100 * (((-((t195 * l.fb8) / (l.fb5 * l.fb5))) * t1a2) + (t197 * (0.5 * (((-((t198 * l.fb8) / (l.fb5 * l.fb5))) * t19f) + (t19a * ((-((t19b * l.fb8) / (l.fb5 * l.fb5))) * 0.3333333333333333))))))), (1e100 * (((-((t195 * l.fb9) / (l.fb5 * l.fb5))) * t1a2) + (t197 * (0.5 * (((-((t198 * l.fb9) / (l.fb5 * l.fb5))) * t19f) + (t19a * ((-((t19b * l.fb9) / (l.fb5 * l.fb5))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f45a == 0.0)) {let t1a6: f64 = (l.f745 * l.fb5);let t1a7: f64 = (t1a6 * l.fb5);let t1a8: f64 = (t1a7 * l.f6fb);let t1a9: f64 = (l.f28 * t1a8);(l.f528, l.f52b, l.f52c, ) = (t1a9, (l.f28 * ((((((l.f746 * l.fb5) + (l.f745 * l.fb8)) * l.fb5) + (t1a6 * l.fb8)) * l.f6fb) + (t1a7 * l.f6fe))), (l.f28 * ((((((l.f747 * l.fb5) + (l.f745 * l.fb9)) * l.fb5) + (t1a6 * l.fb9)) * l.f6fb) + (t1a7 * l.f6ff))), );}
        let t1aa: f64 = if ((l.f78d > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f462 = t1aa;
        if (((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f462 != 0.0)) {(l.fad, l.fb0, l.fb1, ) = (1.0, 0.0, 0.0, );}
        let t1ab: f64 = (-l.f2);let t1ac: f64 = (t1ab * l.f78d);let t1ad: f64 = if l.f749 > t1ac { 1.0 } else { 0.0 };l.f464 = t1ad;let t1ae: f64 = if l.f629 == 4.0 { 1.0 } else { 0.0 };l.f466 = t1ae;
        if (((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f462 == 0.0)) && (l.f464 != 0.0)) && (l.f466 != 0.0)) {let t1af: f64 = (l.f749 * l.f78b);let t1b0: f64 = (t1af).abs();let t1b1: f64 = (l.f749 * l.f78b);let t1b2: f64 = (t1b1).abs();let t1b3: f64 = (t1b0 * t1b2);let t1b4: f64 = (l.f749 * l.f78b);let t1b5: f64 = (t1b4).abs();let t1b6: f64 = (t1b3 * t1b5);let t1b7: f64 = (l.f749 * l.f78b);let t1b8: f64 = (t1b7).abs();let t1b9: f64 = (t1b6 * t1b8);(l.f6fb, l.f6fe, l.f6ff, ) = (t1b9, ((((((if t1af >= 0.0 { (l.f74c * l.f78b) } else { (-(l.f74c * l.f78b)) } * t1b2) + (t1b0 * if t1b1 >= 0.0 { (l.f74c * l.f78b) } else { (-(l.f74c * l.f78b)) })) * t1b5) + (t1b3 * if t1b4 >= 0.0 { (l.f74c * l.f78b) } else { (-(l.f74c * l.f78b)) })) * t1b8) + (t1b6 * if t1b7 >= 0.0 { (l.f74c * l.f78b) } else { (-(l.f74c * l.f78b)) })), ((((((if t1af >= 0.0 { (l.f74d * l.f78b) } else { (-(l.f74d * l.f78b)) } * t1b2) + (t1b0 * if t1b1 >= 0.0 { (l.f74d * l.f78b) } else { (-(l.f74d * l.f78b)) })) * t1b5) + (t1b3 * if t1b4 >= 0.0 { (l.f74d * l.f78b) } else { (-(l.f74d * l.f78b)) })) * t1b8) + (t1b6 * if t1b7 >= 0.0 { (l.f74d * l.f78b) } else { (-(l.f74d * l.f78b)) })), );}
        if (((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f462 == 0.0)) && (l.f464 != 0.0)) && (l.f466 == 0.0)) {let t1ba: f64 = (l.f749 * l.f78b);let t1bb: f64 = (t1ba).abs();let t1bc: f64 = (t1bb).powf(l.f629);(l.f6fb, l.f6fe, l.f6ff, ) = (t1bc, if 0.0 == 0.0 && ((l.f629) as f64).is_finite() && ((l.f629) as f64).fract() == 0.0 { if l.f629 == 0.0 { 0.0 } else { (l.f629 * ((t1bb).powf(l.f629 - 1.0) * if t1ba >= 0.0 { (l.f74c * l.f78b) } else { (-(l.f74c * l.f78b)) })) } } else { (t1bc * (l.f629 * (if t1ba >= 0.0 { (l.f74c * l.f78b) } else { (-(l.f74c * l.f78b)) } / t1bb))) }, if 0.0 == 0.0 && ((l.f629) as f64).is_finite() && ((l.f629) as f64).fract() == 0.0 { if l.f629 == 0.0 { 0.0 } else { (l.f629 * ((t1bb).powf(l.f629 - 1.0) * if t1ba >= 0.0 { (l.f74d * l.f78b) } else { (-(l.f74d * l.f78b)) })) } } else { (t1bc * (l.f629 * (if t1ba >= 0.0 { (l.f74d * l.f78b) } else { (-(l.f74d * l.f78b)) } / t1bb))) }, );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f462 == 0.0)) && (l.f464 != 0.0)) {let t1bd: f64 = (1.0 - l.f6fb);let t1be: f64 = (1.0 / t1bd);(l.fad, l.fb0, l.fb1, ) = (t1be, (-((-l.f6fe) / (t1bd * t1bd))), (-((-l.f6ff) / (t1bd * t1bd))), );}
        if ((((l.f3e0 == 0.0) && (l.f444 == 0.0)) && (l.f462 == 0.0)) && (l.f464 == 0.0)) {let t1bf: f64 = (l.f2 * l.f78d);let t1c0: f64 = (l.f749 + t1bf);let t1c1: f64 = (t1c0 * l.f6be);let t1c2: f64 = (l.fc7 + t1c1);(l.fad, l.fb0, l.fb1, ) = (t1c2, (l.f74c * l.f6be), (l.f74d * l.f6be), );}
        if ((l.f3e0 == 0.0) && (l.f444 == 0.0)) {let t1c3: f64 = (l.f52e + l.f592);let t1c4: f64 = (t1c3 + l.f598);let t1c5: f64 = (t1c4 + l.f528);let t1c6: f64 = (t1c5 * l.fad);(l.f576, l.f577, l.f578, ) = (t1c6, (((((l.f533 + l.f595) + l.f59b) + l.f52b) * l.fad) + (t1c5 * l.fb0)), (((((l.f534 + l.f596) + l.f59c) + l.f52c) * l.fad) + (t1c5 * l.fb1)), );let t1c7: f64 = (l.f592 + l.f598);let t1c8: f64 = (t1c7 + l.f528);let t1c9: f64 = (t1c8 * l.fad);(l.f55a, l.f55b, l.f55c, ) = (t1c9, ((((l.f595 + l.f59b) + l.f52b) * l.fad) + (t1c8 * l.fb0)), ((((l.f596 + l.f59c) + l.f52c) * l.fad) + (t1c8 * l.fb1)), );}
        let t1ca: f64 = if l.f5af == 0.0 { 1.0 } else { 0.0 };l.f468 = t1ca;
        if ((l.f3e0 == 0.0) && (l.f468 != 0.0)) {(l.f56e, l.f56f, l.f570, ) = (0.0, 0.0, 0.0, );(l.f556, l.f557, l.f558, ) = (0.0, 0.0, 0.0, );(l.f690, l.f691, l.f692, ) = (0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_166(
        l: &mut StampLocals,
    ) {
        let t1cb: f64 = if l.f60d == 0.5 { 1.0 } else { 0.0 };l.f46a = t1cb;
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46a != 0.0)) {let t1cc: f64 = (l.f795 * l.f76b);let t1cd: f64 = (1.0 - t1cc);let t1ce: f64 = (t1cd).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t1ce, ((-(l.f798 * l.f76b)) / (2.0 * t1ce)), ((-(l.f799 * l.f76b)) / (2.0 * t1ce)), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46a == 0.0)) {let t1cf: f64 = (l.f795 * l.f76b);let t1d0: f64 = (1.0 - t1cf);let t1d1: f64 = (t1d0).powf(l.f60d);(l.f6fb, l.f6fe, l.f6ff, ) = (t1d1, if 0.0 == 0.0 && ((l.f60d) as f64).is_finite() && ((l.f60d) as f64).fract() == 0.0 { if l.f60d == 0.0 { 0.0 } else { (l.f60d * ((t1d0).powf(l.f60d - 1.0) * (-(l.f798 * l.f76b)))) } } else { (t1d1 * (l.f60d * ((-(l.f798 * l.f76b)) / t1d0))) }, if 0.0 == 0.0 && ((l.f60d) as f64).is_finite() && ((l.f60d) as f64).fract() == 0.0 { if l.f60d == 0.0 { 0.0 } else { (l.f60d * ((t1d0).powf(l.f60d - 1.0) * (-(l.f799 * l.f76b)))) } } else { (t1d1 * (l.f60d * ((-(l.f799 * l.f76b)) / t1d0))) }, );}
        if ((l.f3e0 == 0.0) && (l.f468 == 0.0)) {let t1d2: f64 = (1.0 - l.f6fb);let t1d3: f64 = (l.f6a0 * t1d2);let t1d4: f64 = (l.f745 - l.f795);let t1d5: f64 = (l.f69a * t1d4);let t1d6: f64 = (t1d3 + t1d5);(l.f690, l.f691, l.f692, ) = (t1d6, ((l.f6a0 * (-l.f6fe)) + (l.f69a * (l.f746 - l.f798))), ((l.f6a0 * (-l.f6ff)) + (l.f69a * (l.f747 - l.f799))), );let t1d7: f64 = (l.f544 * l.f53a);(l.f52e, l.f533, l.f534, ) = (t1d7, (l.f544 * l.f53b), (l.f544 * l.f53c), );}
        let t1d8: f64 = if ((l.f3b == 0.0) && (l.f41 == 0.0)) { 1.0 } else { 0.0 };l.f46c = t1d8;
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c != 0.0)) {(l.f757, l.f75a, l.f75b, ) = (0.0, 0.0, 0.0, );(l.f7e8, l.f7eb, l.f7ec, ) = (0.0, 0.0, 0.0, );(l.f7d0, l.f7d3, l.f7d4, ) = (0.0, 0.0, 0.0, );(l.f8, l.fb, l.fc, ) = (0.0, 0.0, 0.0, );(l.f592, l.f595, l.f596, ) = (0.0, 0.0, 0.0, );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) {let t1d9: f64 = (l.f763 - l.f7a1);(l.f757, l.f75a, l.f75b, ) = (t1d9, (-l.f7a4), (-l.f7a5), );let t1da: f64 = (l.f713 / l.f757);let t1db: f64 = (1.0 - t1da);let t1dc: f64 = (t1db).sqrt();let t1dd: f64 = (1.0 - t1dc);(l.f7ee, l.f7f1, l.f7f2, ) = (t1dd, (-((-(((l.f716 * l.f757) - (l.f713 * l.f75a)) / (l.f757 * l.f757))) / (2.0 * t1dc))), (-((-(((l.f717 * l.f757) - (l.f713 * l.f75b)) / (l.f757 * l.f757))) / (2.0 * t1dc))), );}
        let t1de: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f46e = t1de;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) && (l.f46e != 0.0)) {(l.f65, l.f68, l.f69, ) = (0.0, 0.0, 0.0, );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) && (l.f46e == 0.0)) {let t1df: f64 = (l.f7ee * l.f7ee);let t1e0: f64 = (l.f7ee).ln();let t1e1: f64 = (t1df * t1e0);let t1e2: f64 = (1.0 - l.f7ee);let t1e3: f64 = (t1e1 / t1e2);let t1e4: f64 = (t1e3 + l.f7ee);let t1e5: f64 = (2.0 * l.f62f);let t1e6: f64 = (1.0 - t1e5);let t1e7: f64 = (t1e4 * t1e6);(l.f65, l.f68, l.f69, ) = (t1e7, (((((((((l.f7f1 * l.f7ee) + (l.f7ee * l.f7f1)) * t1e0) + (t1df * (l.f7f1 / l.f7ee))) * t1e2) - (t1e1 * (-l.f7f1))) / (t1e2 * t1e2)) + l.f7f1) * t1e6), (((((((((l.f7f2 * l.f7ee) + (l.f7ee * l.f7f2)) * t1e0) + (t1df * (l.f7f2 / l.f7ee))) * t1e2) - (t1e1 * (-l.f7f2))) / (t1e2 * t1e2)) + l.f7f2) * t1e6), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) {let t1e8: f64 = (l.f7ee + l.f65);(l.f7e8, l.f7eb, l.f7ec, ) = (t1e8, (l.f7f1 + l.f68), (l.f7f2 + l.f69), );}
        let t1e9: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f470 = t1e9;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) && (l.f470 != 0.0)) {let t1ea: f64 = (l.f757 * l.f777);let t1eb: f64 = (t1ea).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t1eb, ((l.f75a * l.f777) / (2.0 * t1eb)), ((l.f75b * l.f777) / (2.0 * t1eb)), );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) && (l.f470 == 0.0)) {let t1ec: f64 = (l.f757 * l.f777);let t1ed: f64 = (t1ec).powf(l.f62f);(l.f6fb, l.f6fe, l.f6ff, ) = (t1ed, if 0.0 == 0.0 && ((l.f62f) as f64).is_finite() && ((l.f62f) as f64).fract() == 0.0 { if l.f62f == 0.0 { 0.0 } else { (l.f62f * ((t1ec).powf(l.f62f - 1.0) * (l.f75a * l.f777))) } } else { (t1ed * (l.f62f * ((l.f75a * l.f777) / t1ec))) }, if 0.0 == 0.0 && ((l.f62f) as f64).is_finite() && ((l.f62f) as f64).fract() == 0.0 { if l.f62f == 0.0 { 0.0 } else { (l.f62f * ((t1ec).powf(l.f62f - 1.0) * (l.f75b * l.f777))) } } else { (t1ed * (l.f62f * ((l.f75b * l.f777) / t1ec))) }, );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f46c == 0.0)) {let t1ee: f64 = (l.f7d8 * l.f6fb);(l.f7d0, l.f7d3, l.f7d4, ) = (t1ee, (l.f7d8 * l.f6fe), (l.f7d8 * l.f6ff), );let t1ef: f64 = (l.f824 - 1.0);let t1f0: f64 = (t1ef * l.f7d0);let t1f1: f64 = (l.fcd * t1f0);(l.f8, l.fb, l.fc, ) = (t1f1, (l.fcd * ((l.f827 * l.f7d0) + (t1ef * l.f7d3))), (l.fcd * ((l.f828 * l.f7d0) + (t1ef * l.f7d4))), );let t1f2: f64 = (l.f8 * l.f7e8);let t1f3: f64 = (l.f3b * t1f2);(l.f592, l.f595, l.f596, ) = (t1f3, (l.f3b * ((l.fb * l.f7e8) + (l.f8 * l.f7eb))), (l.f3b * ((l.fc * l.f7e8) + (l.f8 * l.f7ec))), );}
        let t1f4: f64 = if l.f41 == 0.0 { 1.0 } else { 0.0 };l.f472 = t1f4;
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 != 0.0)) {(l.f598, l.f59b, l.f59c, ) = (0.0, 0.0, 0.0, );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) {let t1f5: f64 = (l.f7d0 * l.f60d);let t1f6: f64 = (t1f5 / l.f757);let t1f7: f64 = (l.f20 * t1f6);(l.f18, l.f1b, l.f1c, ) = (t1f7, (l.f20 * ((((l.f7d3 * l.f60d) * l.f757) - (t1f5 * l.f75a)) / (l.f757 * l.f757))), (l.f20 * ((((l.f7d4 * l.f60d) * l.f757) - (t1f5 * l.f75b)) / (l.f757 * l.f757))), );let t1f8: f64 = (0.666666666666667 * l.f10);let t1f9: f64 = (t1f8 / l.f18);(l.f719, l.f71c, l.f71d, ) = (t1f9, (-((t1f8 * l.f1b) / (l.f18 * l.f18))), (-((t1f8 * l.f1c) / (l.f18 * l.f18))), );let t1fa: f64 = (l.f719 * l.f719);(l.f72b, l.f72e, l.f72f, ) = (t1fa, ((l.f71c * l.f719) + (l.f719 * l.f71c)), ((l.f71d * l.f719) + (l.f719 * l.f71d)), );let t1fb: f64 = (l.f72b * l.f72b);let t1fc: f64 = (l.f72b * l.f72b);let t1fd: f64 = (t1fc + 1.0);let t1fe: f64 = (t1fb / t1fd);let t1ff: f64 = (t1fe).sqrt();(l.f725, l.f728, l.f729, ) = (t1ff, ((((((l.f72e * l.f72b) + (l.f72b * l.f72e)) * t1fd) - (t1fb * ((l.f72e * l.f72b) + (l.f72b * l.f72e)))) / (t1fd * t1fd)) / (2.0 * t1ff)), ((((((l.f72f * l.f72b) + (l.f72b * l.f72f)) * t1fd) - (t1fb * ((l.f72f * l.f72b) + (l.f72b * l.f72f)))) / (t1fd * t1fd)) / (2.0 * t1ff)), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) {let t200: f64 = (l.f725).abs();let t201: f64 = (t200).sqrt();(l.f6c0, l.f6c3, l.f6c4, ) = (t201, (if l.f725 >= 0.0 { l.f728 } else { (-l.f728) } / (2.0 * t201)), (if l.f725 >= 0.0 { l.f729 } else { (-l.f729) } / (2.0 * t201)), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) {let t202: f64 = (l.f725 * l.f6c0);(l.f731, l.f734, l.f735, ) = (t202, ((l.f728 * l.f6c0) + (l.f725 * l.f6c3)), ((l.f729 * l.f6c0) + (l.f725 * l.f6c4)), );}
        let t203: f64 = (-l.f62f);let t204: f64 = (t203 * l.f613);let t205: f64 = (-1.0);let t206: f64 = if t204 == t205 { 1.0 } else { 0.0 };l.f474 = t206;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f474 != 0.0)) {let t207: f64 = (l.f18 * l.f731);let t208: f64 = (1.0 + t207);let t209: f64 = (1.0 / t208);(l.f7e2, l.f7e5, l.f7e6, ) = (t209, (-(((l.f1b * l.f731) + (l.f18 * l.f734)) / (t208 * t208))), (-(((l.f1c * l.f731) + (l.f18 * l.f735)) / (t208 * t208))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_167(
        l: &mut StampLocals,
    ) {
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f474 == 0.0)) {let t20a: f64 = (l.f18 * l.f731);let t20b: f64 = (1.0 + t20a);let t20c: f64 = (-l.f62f);let t20d: f64 = (t20c * l.f613);let t20e: f64 = (t20b).powf(t20d);(l.f7e2, l.f7e5, l.f7e6, ) = (t20e, if 0.0 == 0.0 && ((t20d) as f64).is_finite() && ((t20d) as f64).fract() == 0.0 { if t20d == 0.0 { 0.0 } else { (t20d * ((t20b).powf(t20d - 1.0) * ((l.f1b * l.f731) + (l.f18 * l.f734)))) } } else { (t20e * (t20d * (((l.f1b * l.f731) + (l.f18 * l.f734)) / t20b))) }, if 0.0 == 0.0 && ((t20d) as f64).is_finite() && ((t20d) as f64).fract() == 0.0 { if t20d == 0.0 { 0.0 } else { (t20d * ((t20b).powf(t20d - 1.0) * ((l.f1c * l.f731) + (l.f18 * l.f735)))) } } else { (t20e * (t20d * (((l.f1c * l.f731) + (l.f18 * l.f735)) / t20b))) }, );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) {let t20f: f64 = (l.f7e8 * l.f7e2);let t210: f64 = (l.f7e8 + l.f7e2);let t211: f64 = (t20f / t210);(l.f7f4, l.f7f7, l.f7f8, ) = (t211, (((((l.f7eb * l.f7e2) + (l.f7e8 * l.f7e5)) * t210) - (t20f * (l.f7eb + l.f7e5))) / (t210 * t210)), (((((l.f7ec * l.f7e2) + (l.f7e8 * l.f7e6)) * t210) - (t20f * (l.f7ec + l.f7e6))) / (t210 * t210)), );let t212: f64 = (l.f18 / l.f6c0);let t213: f64 = (0.375 * t212);let t214: f64 = (t213).sqrt();(l.f5a7, l.f5aa, l.f5ab, ) = (t214, ((0.375 * (((l.f1b * l.f6c0) - (l.f18 * l.f6c3)) / (l.f6c0 * l.f6c0))) / (2.0 * t214)), ((0.375 * (((l.f1c * l.f6c0) - (l.f18 * l.f6c4)) / (l.f6c0 * l.f6c0))) / (2.0 * t214)), );let t215: f64 = (l.f719 * l.f6c0);let t216: f64 = (2.0 * t215);let t217: f64 = (t216 - l.f725);(l.f5b3, l.f5b6, l.f5b7, ) = (t217, ((2.0 * ((l.f71c * l.f6c0) + (l.f719 * l.f6c3))) - l.f728), ((2.0 * ((l.f71d * l.f6c0) + (l.f719 * l.f6c4))) - l.f729), );let t218: f64 = (l.f10 * l.f719);let t219: f64 = (t218 * l.f6c0);let t21a: f64 = (l.f10 * l.f725);let t21b: f64 = (t219 - t21a);let t21c: f64 = (l.f18 * l.f731);let t21d: f64 = (0.5 * t21c);let t21e: f64 = (t21b + t21d);(l.f5d3, l.f5d6, l.f5d7, ) = (t21e, (((((l.f10 * l.f71c) * l.f6c0) + (t218 * l.f6c3)) - (l.f10 * l.f728)) + (0.5 * ((l.f1b * l.f731) + (l.f18 * l.f734)))), (((((l.f10 * l.f71d) * l.f6c0) + (t218 * l.f6c4)) - (l.f10 * l.f729)) + (0.5 * ((l.f1c * l.f731) + (l.f18 * l.f735)))), );let t21f: f64 = (l.f5b3 - 1.0);let t220: f64 = (t21f * l.f5a7);(l.f7fa, l.f7fd, l.f7fe, ) = (t220, ((l.f5b6 * l.f5a7) + (t21f * l.f5aa)), ((l.f5b7 * l.f5a7) + (t21f * l.f5ab)), );let t221: f64 = (l.f7fa * l.f7fa);(l.f810, l.f813, l.f814, ) = (t221, ((l.f7fd * l.f7fa) + (l.f7fa * l.f7fd)), ((l.f7fe * l.f7fa) + (l.f7fa * l.f7fe)), );}
        let t222: f64 = if l.f7fa > 0.0 { 1.0 } else { 0.0 };l.f476 = t222;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f476 != 0.0)) {let t223: f64 = (l.f62b * l.f7fa);let t224: f64 = (1.0 + t223);let t225: f64 = (1.0 / t224);(l.f6e1, l.f6e4, l.f6e5, ) = (t225, (-((l.f62b * l.f7fd) / (t224 * t224))), (-((l.f62b * l.f7fe) / (t224 * t224))), );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f476 == 0.0)) {let t226: f64 = (l.f62b * l.f7fa);let t227: f64 = (1.0 - t226);let t228: f64 = (1.0 / t227);(l.f6e1, l.f6e4, l.f6e5, ) = (t228, (-((-(l.f62b * l.f7fd)) / (t227 * t227))), (-((-(l.f62b * l.f7fe)) / (t227 * t227))), );}
        let t229: f64 = (-l.f810);let t22a: f64 = (t229 + l.f5d3);let t22b: f64 = (-230.25850929940458);let t22c: f64 = if t22a > t22b { 1.0 } else { 0.0 };l.f478 = t22c;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f478 != 0.0)) {let t22d: f64 = (-l.f810);let t22e: f64 = (t22d + l.f5d3);let t22f: f64 = (t22e).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t22f, (t22f * ((-l.f813) + l.f5d6)), (t22f * ((-l.f814) + l.f5d7)), );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f478 == 0.0)) {let t230: f64 = (-230.25850929940458);let t231: f64 = (-l.f810);let t232: f64 = (t231 + l.f5d3);let t233: f64 = (t230 - t232);let t234: f64 = (-230.25850929940458);let t235: f64 = (-l.f810);let t236: f64 = (t235 + l.f5d3);let t237: f64 = (t234 - t236);let t238: f64 = (-230.25850929940458);let t239: f64 = (-l.f810);let t23a: f64 = (t239 + l.f5d3);let t23b: f64 = (t238 - t23a);let t23c: f64 = (t23b * 0.3333333333333333);let t23d: f64 = (1.0 + t23c);let t23e: f64 = (t237 * t23d);let t23f: f64 = (0.5 * t23e);let t240: f64 = (1.0 + t23f);let t241: f64 = (t233 * t240);let t242: f64 = (1.0 + t241);let t243: f64 = (1e-100 / t242);(l.f6fb, l.f6fe, l.f6ff, ) = (t243, (-((1e-100 * (((-((-l.f813) + l.f5d6)) * t240) + (t233 * (0.5 * (((-((-l.f813) + l.f5d6)) * t23d) + (t237 * ((-((-l.f813) + l.f5d6)) * 0.3333333333333333))))))) / (t242 * t242))), (-((1e-100 * (((-((-l.f814) + l.f5d7)) * t240) + (t233 * (0.5 * (((-((-l.f814) + l.f5d7)) * t23d) + (t237 * ((-((-l.f814) + l.f5d7)) * 0.3333333333333333))))))) / (t242 * t242))), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) {let t244: f64 = (0.29214664 * l.f6e1);let t245: f64 = (l.f6e1 * l.f6e1);let t246: f64 = (l.f16 * t245);let t247: f64 = (t244 + t246);let t248: f64 = (l.f6e1 * l.f6e1);let t249: f64 = (t248 * l.f6e1);let t24a: f64 = (l.f2a * t249);let t24b: f64 = (t247 + t24a);let t24c: f64 = (t24b * l.f6fb);(l.f6d, l.f70, l.f71, ) = (t24c, (((((0.29214664 * l.f6e4) + (l.f16 * ((l.f6e4 * l.f6e1) + (l.f6e1 * l.f6e4)))) + (l.f2a * ((((l.f6e4 * l.f6e1) + (l.f6e1 * l.f6e4)) * l.f6e1) + (t248 * l.f6e4)))) * l.f6fb) + (t24b * l.f6fe)), (((((0.29214664 * l.f6e5) + (l.f16 * ((l.f6e5 * l.f6e1) + (l.f6e1 * l.f6e5)))) + (l.f2a * ((((l.f6e5 * l.f6e1) + (l.f6e1 * l.f6e5)) * l.f6e1) + (t248 * l.f6e5)))) * l.f6fb) + (t24b * l.f6ff)), );}
        let t24d: f64 = if l.f7fa > 0.0 { 1.0 } else { 0.0 };l.f47a = t24d;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f47a != 0.0)) {(l.f73, l.f76, l.f77, ) = (l.f6d, l.f70, l.f71, );}
        let t24e: f64 = (-230.25850929940458);let t24f: f64 = if l.f5d3 > t24e { 1.0 } else { 0.0 };l.f47c = t24f;
        if (((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f47a == 0.0)) && (l.f47c != 0.0)) {let t250: f64 = (l.f5d3).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t250, (t250 * l.f5d6), (t250 * l.f5d7), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_168(
        l: &mut StampLocals,
    ) {
        if (((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f47a == 0.0)) && (l.f47c == 0.0)) {let t251: f64 = (-230.25850929940458);let t252: f64 = (t251 - l.f5d3);let t253: f64 = (-230.25850929940458);let t254: f64 = (t253 - l.f5d3);let t255: f64 = (-230.25850929940458);let t256: f64 = (t255 - l.f5d3);let t257: f64 = (t256 * 0.3333333333333333);let t258: f64 = (1.0 + t257);let t259: f64 = (t254 * t258);let t25a: f64 = (0.5 * t259);let t25b: f64 = (1.0 + t25a);let t25c: f64 = (t252 * t25b);let t25d: f64 = (1.0 + t25c);let t25e: f64 = (1e-100 / t25d);(l.f6fb, l.f6fe, l.f6ff, ) = (t25e, (-((1e-100 * (((-l.f5d6) * t25b) + (t252 * (0.5 * (((-l.f5d6) * t258) + (t254 * ((-l.f5d6) * 0.3333333333333333))))))) / (t25d * t25d))), (-((1e-100 * (((-l.f5d7) * t25b) + (t252 * (0.5 * (((-l.f5d7) * t258) + (t254 * ((-l.f5d7) * 0.3333333333333333))))))) / (t25d * t25d))), );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) && (l.f47a == 0.0)) {let t25f: f64 = (2.0 * l.f6fb);let t260: f64 = (t25f - l.f6d);(l.f73, l.f76, l.f77, ) = (t260, ((2.0 * l.f6fe) - l.f70), ((2.0 * l.f6ff) - l.f71), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f472 == 0.0)) {let t261: f64 = (1.772453850905516 * 0.5);let t262: f64 = (l.f10 * l.f73);let t263: f64 = (t262 / l.f5a7);let t264: f64 = (t261 * t263);(l.fd5, l.fd8, l.fd9, ) = (t264, (t261 * ((((l.f10 * l.f76) * l.f5a7) - (t262 * l.f5aa)) / (l.f5a7 * l.f5a7))), (t261 * ((((l.f10 * l.f77) * l.f5a7) - (t262 * l.f5ab)) / (l.f5a7 * l.f5a7))), );let t265: f64 = (l.f8 * l.fd5);let t266: f64 = (t265 * l.f7f4);let t267: f64 = (l.f41 * t266);(l.f598, l.f59b, l.f59c, ) = (t267, (l.f41 * ((((l.fb * l.fd5) + (l.f8 * l.fd8)) * l.f7f4) + (t265 * l.f7f7))), (l.f41 * ((((l.fc * l.fd5) + (l.f8 * l.fd9)) * l.f7f4) + (t265 * l.f7f8))), );}
        let t268: f64 = if l.f26 == 0.0 { 1.0 } else { 0.0 };l.f47e = t268;
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e != 0.0)) {(l.f528, l.f52b, l.f52c, ) = (0.0, 0.0, 0.0, );}
        let t269: f64 = if l.f62f == 0.5 { 1.0 } else { 0.0 };l.f480 = t269;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) && (l.f480 != 0.0)) {let t26a: f64 = (l.f775 - l.f74f);let t26b: f64 = (t26a * l.f777);let t26c: f64 = (t26b).sqrt();(l.f6fb, l.f6fe, l.f6ff, ) = (t26c, (((-l.f752) * l.f777) / (2.0 * t26c)), (((-l.f753) * l.f777) / (2.0 * t26c)), );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) && (l.f480 == 0.0)) {let t26d: f64 = (l.f775 - l.f74f);let t26e: f64 = (t26d * l.f777);let t26f: f64 = (t26e).powf(l.f62f);(l.f6fb, l.f6fe, l.f6ff, ) = (t26f, if 0.0 == 0.0 && ((l.f62f) as f64).is_finite() && ((l.f62f) as f64).fract() == 0.0 { if l.f62f == 0.0 { 0.0 } else { (l.f62f * ((t26e).powf(l.f62f - 1.0) * ((-l.f752) * l.f777))) } } else { (t26f * (l.f62f * (((-l.f752) * l.f777) / t26e))) }, if 0.0 == 0.0 && ((l.f62f) as f64).is_finite() && ((l.f62f) as f64).fract() == 0.0 { if l.f62f == 0.0 { 0.0 } else { (l.f62f * ((t26e).powf(l.f62f - 1.0) * ((-l.f753) * l.f777))) } } else { (t26f * (l.f62f * (((-l.f753) * l.f777) / t26e))) }, );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) {let t270: f64 = (l.f775 - l.f74f);let t271: f64 = (t270 * l.f7dc);let t272: f64 = (t271 / l.f6fb);let t273: f64 = (l.f613 * t272);(l.fb5, l.fb8, l.fb9, ) = (t273, (l.f613 * (((((-l.f752) * l.f7dc) * l.f6fb) - (t271 * l.f6fe)) / (l.f6fb * l.f6fb))), (l.f613 * (((((-l.f753) * l.f7dc) * l.f6fb) - (t271 * l.f6ff)) / (l.f6fb * l.f6fb))), );}
        let t274: f64 = (-l.fa3);let t275: f64 = (t274 / l.fb5);let t276: f64 = (t275).abs();let t277: f64 = if t276 < 230.25850929940458 { 1.0 } else { 0.0 };l.f482 = t277;
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) && (l.f482 != 0.0)) {let t278: f64 = (-l.fa3);let t279: f64 = (t278 / l.fb5);let t27a: f64 = (t279).exp();(l.f6fb, l.f6fe, l.f6ff, ) = (t27a, (t27a * (-((t278 * l.fb8) / (l.fb5 * l.fb5)))), (t27a * (-((t278 * l.fb9) / (l.fb5 * l.fb5)))), );}
        let t27b: f64 = (-l.fa3);let t27c: f64 = (t27b / l.fb5);let t27d: f64 = (-230.25850929940458);let t27e: f64 = if t27c < t27d { 1.0 } else { 0.0 };l.f484 = t27e;
        if (((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) && (l.f482 == 0.0)) && (l.f484 != 0.0)) {let t27f: f64 = (-230.25850929940458);let t280: f64 = (-l.fa3);let t281: f64 = (t280 / l.fb5);let t282: f64 = (t27f - t281);let t283: f64 = (-230.25850929940458);let t284: f64 = (-l.fa3);let t285: f64 = (t284 / l.fb5);let t286: f64 = (t283 - t285);let t287: f64 = (-230.25850929940458);let t288: f64 = (-l.fa3);let t289: f64 = (t288 / l.fb5);let t28a: f64 = (t287 - t289);let t28b: f64 = (t28a * 0.3333333333333333);let t28c: f64 = (1.0 + t28b);let t28d: f64 = (t286 * t28c);let t28e: f64 = (0.5 * t28d);let t28f: f64 = (1.0 + t28e);let t290: f64 = (t282 * t28f);let t291: f64 = (1.0 + t290);let t292: f64 = (1e-100 / t291);(l.f6fb, l.f6fe, l.f6ff, ) = (t292, (-((1e-100 * (((-(-((t280 * l.fb8) / (l.fb5 * l.fb5)))) * t28f) + (t282 * (0.5 * (((-(-((t284 * l.fb8) / (l.fb5 * l.fb5)))) * t28c) + (t286 * ((-(-((t288 * l.fb8) / (l.fb5 * l.fb5)))) * 0.3333333333333333))))))) / (t291 * t291))), (-((1e-100 * (((-(-((t280 * l.fb9) / (l.fb5 * l.fb5)))) * t28f) + (t282 * (0.5 * (((-(-((t284 * l.fb9) / (l.fb5 * l.fb5)))) * t28c) + (t286 * ((-(-((t288 * l.fb9) / (l.fb5 * l.fb5)))) * 0.3333333333333333))))))) / (t291 * t291))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_169(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv2 = ctx.node_voltage(nodes[2]);
        if (((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) && (l.f482 == 0.0)) && (l.f484 == 0.0)) {let t293: f64 = (-l.fa3);let t294: f64 = (t293 / l.fb5);let t295: f64 = (t294 - 230.25850929940458);let t296: f64 = (-l.fa3);let t297: f64 = (t296 / l.fb5);let t298: f64 = (t297 - 230.25850929940458);let t299: f64 = (-l.fa3);let t29a: f64 = (t299 / l.fb5);let t29b: f64 = (t29a - 230.25850929940458);let t29c: f64 = (t29b * 0.3333333333333333);let t29d: f64 = (1.0 + t29c);let t29e: f64 = (t298 * t29d);let t29f: f64 = (0.5 * t29e);let t2a0: f64 = (1.0 + t29f);let t2a1: f64 = (t295 * t2a0);let t2a2: f64 = (1.0 + t2a1);let t2a3: f64 = (1e100 * t2a2);(l.f6fb, l.f6fe, l.f6ff, ) = (t2a3, (1e100 * (((-((t293 * l.fb8) / (l.fb5 * l.fb5))) * t2a0) + (t295 * (0.5 * (((-((t296 * l.fb8) / (l.fb5 * l.fb5))) * t29d) + (t298 * ((-((t299 * l.fb8) / (l.fb5 * l.fb5))) * 0.3333333333333333))))))), (1e100 * (((-((t293 * l.fb9) / (l.fb5 * l.fb5))) * t2a0) + (t295 * (0.5 * (((-((t296 * l.fb9) / (l.fb5 * l.fb5))) * t29d) + (t298 * ((-((t299 * l.fb9) / (l.fb5 * l.fb5))) * 0.3333333333333333))))))), );}
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f47e == 0.0)) {let t2a4: f64 = (l.f745 * l.fb5);let t2a5: f64 = (t2a4 * l.fb5);let t2a6: f64 = (t2a5 * l.f6fb);let t2a7: f64 = (l.f26 * t2a6);(l.f528, l.f52b, l.f52c, ) = (t2a7, (l.f26 * ((((((l.f746 * l.fb5) + (l.f745 * l.fb8)) * l.fb5) + (t2a4 * l.fb8)) * l.f6fb) + (t2a5 * l.f6fe))), (l.f26 * ((((((l.f747 * l.fb5) + (l.f745 * l.fb9)) * l.fb5) + (t2a4 * l.fb9)) * l.f6fb) + (t2a5 * l.f6ff))), );}
        let t2a8: f64 = if ((l.f785 > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };l.f486 = t2a8;
        if (((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f486 != 0.0)) {(l.fad, l.fb0, l.fb1, ) = (1.0, 0.0, 0.0, );}
        let t2a9: f64 = (-l.f2);let t2aa: f64 = (t2a9 * l.f785);let t2ab: f64 = if l.f749 > t2aa { 1.0 } else { 0.0 };l.f488 = t2ab;let t2ac: f64 = if l.f627 == 4.0 { 1.0 } else { 0.0 };l.f48a = t2ac;
        if (((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f486 == 0.0)) && (l.f488 != 0.0)) && (l.f48a != 0.0)) {let t2ad: f64 = (l.f749 * l.f789);let t2ae: f64 = (t2ad).abs();let t2af: f64 = (l.f749 * l.f789);let t2b0: f64 = (t2af).abs();let t2b1: f64 = (t2ae * t2b0);let t2b2: f64 = (l.f749 * l.f789);let t2b3: f64 = (t2b2).abs();let t2b4: f64 = (t2b1 * t2b3);let t2b5: f64 = (l.f749 * l.f789);let t2b6: f64 = (t2b5).abs();let t2b7: f64 = (t2b4 * t2b6);(l.f6fb, l.f6fe, l.f6ff, ) = (t2b7, ((((((if t2ad >= 0.0 { (l.f74c * l.f789) } else { (-(l.f74c * l.f789)) } * t2b0) + (t2ae * if t2af >= 0.0 { (l.f74c * l.f789) } else { (-(l.f74c * l.f789)) })) * t2b3) + (t2b1 * if t2b2 >= 0.0 { (l.f74c * l.f789) } else { (-(l.f74c * l.f789)) })) * t2b6) + (t2b4 * if t2b5 >= 0.0 { (l.f74c * l.f789) } else { (-(l.f74c * l.f789)) })), ((((((if t2ad >= 0.0 { (l.f74d * l.f789) } else { (-(l.f74d * l.f789)) } * t2b0) + (t2ae * if t2af >= 0.0 { (l.f74d * l.f789) } else { (-(l.f74d * l.f789)) })) * t2b3) + (t2b1 * if t2b2 >= 0.0 { (l.f74d * l.f789) } else { (-(l.f74d * l.f789)) })) * t2b6) + (t2b4 * if t2b5 >= 0.0 { (l.f74d * l.f789) } else { (-(l.f74d * l.f789)) })), );}
        if (((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f486 == 0.0)) && (l.f488 != 0.0)) && (l.f48a == 0.0)) {let t2b8: f64 = (l.f749 * l.f789);let t2b9: f64 = (t2b8).abs();let t2ba: f64 = (t2b9).powf(l.f627);(l.f6fb, l.f6fe, l.f6ff, ) = (t2ba, if 0.0 == 0.0 && ((l.f627) as f64).is_finite() && ((l.f627) as f64).fract() == 0.0 { if l.f627 == 0.0 { 0.0 } else { (l.f627 * ((t2b9).powf(l.f627 - 1.0) * if t2b8 >= 0.0 { (l.f74c * l.f789) } else { (-(l.f74c * l.f789)) })) } } else { (t2ba * (l.f627 * (if t2b8 >= 0.0 { (l.f74c * l.f789) } else { (-(l.f74c * l.f789)) } / t2b9))) }, if 0.0 == 0.0 && ((l.f627) as f64).is_finite() && ((l.f627) as f64).fract() == 0.0 { if l.f627 == 0.0 { 0.0 } else { (l.f627 * ((t2b9).powf(l.f627 - 1.0) * if t2b8 >= 0.0 { (l.f74d * l.f789) } else { (-(l.f74d * l.f789)) })) } } else { (t2ba * (l.f627 * (if t2b8 >= 0.0 { (l.f74d * l.f789) } else { (-(l.f74d * l.f789)) } / t2b9))) }, );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f486 == 0.0)) && (l.f488 != 0.0)) {let t2bb: f64 = (1.0 - l.f6fb);let t2bc: f64 = (1.0 / t2bb);(l.fad, l.fb0, l.fb1, ) = (t2bc, (-((-l.f6fe) / (t2bb * t2bb))), (-((-l.f6ff) / (t2bb * t2bb))), );}
        if ((((l.f3e0 == 0.0) && (l.f468 == 0.0)) && (l.f486 == 0.0)) && (l.f488 == 0.0)) {let t2bd: f64 = (l.f2 * l.f785);let t2be: f64 = (l.f749 + t2bd);let t2bf: f64 = (t2be * l.f6bc);let t2c0: f64 = (l.fc5 + t2bf);(l.fad, l.fb0, l.fb1, ) = (t2c0, (l.f74c * l.f6bc), (l.f74d * l.f6bc), );}
        if ((l.f3e0 == 0.0) && (l.f468 == 0.0)) {let t2c1: f64 = (l.f52e + l.f592);let t2c2: f64 = (t2c1 + l.f598);let t2c3: f64 = (t2c2 + l.f528);let t2c4: f64 = (t2c3 * l.fad);(l.f56e, l.f56f, l.f570, ) = (t2c4, (((((l.f533 + l.f595) + l.f59b) + l.f52b) * l.fad) + (t2c3 * l.fb0)), (((((l.f534 + l.f596) + l.f59c) + l.f52c) * l.fad) + (t2c3 * l.fb1)), );let t2c5: f64 = (l.f592 + l.f598);let t2c6: f64 = (t2c5 + l.f528);let t2c7: f64 = (t2c6 * l.fad);(l.f556, l.f557, l.f558, ) = (t2c7, ((((l.f595 + l.f59b) + l.f52b) * l.fad) + (t2c6 * l.fb0)), ((((l.f596 + l.f59c) + l.f52c) * l.fad) + (t2c6 * l.fb1)), );}
        if (l.f3e0 == 0.0) {let t2c8: f64 = (l.f0 * l.f562);let t2c9: f64 = (l.f5b1 * l.f576);let t2ca: f64 = (t2c8 + t2c9);let t2cb: f64 = (l.f5af * l.f56e);let t2cc: f64 = (t2ca + t2cb);(l.f55e, l.f55f, l.f560, ) = (t2cc, (((l.f0 * l.f563) + (l.f5b1 * l.f577)) + (l.f5af * l.f56f)), (((l.f0 * l.f564) + (l.f5b1 * l.f578)) + (l.f5af * l.f570)), );let t2cd: f64 = (l.f0 * l.f552);let t2ce: f64 = (l.f5b1 * l.f55a);let t2cf: f64 = (t2cd + t2ce);let t2d0: f64 = (l.f5af * l.f556);let t2d1: f64 = (t2cf + t2d0);(l.f54e, l.f54f, l.f550, ) = (t2d1, (((l.f0 * l.f553) + (l.f5b1 * l.f55b)) + (l.f5af * l.f557)), (((l.f0 * l.f554) + (l.f5b1 * l.f55c)) + (l.f5af * l.f558)), );}
        let t2d2: f64 = (l.f0 * l.f68c);let t2d3: f64 = (l.f5b1 * l.f694);let t2d4: f64 = (t2d2 + t2d3);let t2d5: f64 = (l.f5af * l.f690);let t2d6: f64 = (t2d4 + t2d5);(l.f685, l.f686, l.f687, l.f688, l.f689, l.f68a, ) = (t2d6, (((l.f0 * l.f68d) + (l.f5b1 * l.f695)) + (l.f5af * l.f691)), (((l.f0 * l.f68e) + (l.f5b1 * l.f696)) + (l.f5af * l.f692)), 0.0, 0.0, 0.0, );(l.f7b9, l.f7ba, l.f7bb, ) = ((nv2 - nv1), -1.0, 1.0, );let t2d7: f64 = if p.p84 > 0.0 { 1.0 } else { 0.0 };l.f48c = t2d7;let t2d8: f64 = if l.f609 < p.p85 { 1.0 } else { 0.0 };l.f48e = t2d8;
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t2d9: f64 = (l.f745 - l.f743);let t2da: f64 = (p.p86 * t2d9);let t2db: f64 = (t2da + l.f609);(l.f5f6, l.f5f7, l.f5f8, ) = (t2db, (p.p86 * l.f746), (p.p86 * l.f747), );let t2dc: f64 = (p.p86 * l.f743);let t2dd: f64 = (l.f609 - t2dc);(l.f5ed, l.f5ee, l.f5ef, ) = (t2dd, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_170(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t2de: f64 = (p.p85 - l.f5f6);let t2df: f64 = (t2de - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2df, (-l.f5f7), (-l.f5f8), );let t2e0: f64 = (4.0 * p.p85);let t2e1: f64 = (t2e0 * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2e1, 0.0, 0.0, );}
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {
            let (t2e3, t2e4, t2e5,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2e2: f64 = (-l.f6f7);
        (t2e2, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2e3, t2e4, t2e5, );
        }
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t2e6: f64 = (l.f6f3 * l.f6f3);let t2e7: f64 = (t2e6 + l.f6f7);let t2e8: f64 = (t2e7).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2e8, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2e8)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2e8)), );let t2e9: f64 = (l.f6f3 + l.f6f7);let t2ea: f64 = (0.5 * t2e9);let t2eb: f64 = (p.p85 - t2ea);(l.f5fa, l.f5fb, l.f5fc, ) = (t2eb, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t2ec: f64 = (l.f5fa - l.f609);let t2ed: f64 = (t2ec - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2ed, l.f5fb, l.f5fc, );let t2ee: f64 = (4.0 * l.f609);let t2ef: f64 = (t2ee * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2ef, 0.0, 0.0, );}
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {
            let (t2f1, t2f2, t2f3,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2f0: f64 = (-l.f6f7);
        (t2f0, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2f1, t2f2, t2f3, );
        }
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t2f4: f64 = (l.f6f3 * l.f6f3);let t2f5: f64 = (t2f4 + l.f6f7);let t2f6: f64 = (t2f5).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t2f6, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t2f6)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t2f6)), );let t2f7: f64 = (l.f6f3 + l.f6f7);let t2f8: f64 = (0.5 * t2f7);let t2f9: f64 = (l.f609 + t2f8);(l.f5f5, l.f5fe, l.f5ff, ) = (t2f9, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );let t2fa: f64 = (p.p85 - l.f5ed);let t2fb: f64 = (t2fa - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t2fb, (-l.f5ee), (-l.f5ef), );let t2fc: f64 = (4.0 * p.p85);let t2fd: f64 = (t2fc * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t2fd, 0.0, 0.0, );}
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {
            let (t2ff, t300, t301,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t2fe: f64 = (-l.f6f7);
        (t2fe, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t2ff, t300, t301, );
        }
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t302: f64 = (l.f6f3 * l.f6f3);let t303: f64 = (t302 + l.f6f7);let t304: f64 = (t303).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t304, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t304)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t304)), );let t305: f64 = (l.f6f3 + l.f6f7);let t306: f64 = (0.5 * t305);let t307: f64 = (p.p85 - t306);(l.f5ed, l.f5ee, l.f5ef, ) = (t307, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );let t308: f64 = (l.f5ed - l.f609);let t309: f64 = (t308 - 0.01);(l.f6f3, l.f6f4, l.f6f5, ) = (t309, l.f5ee, l.f5ef, );let t30a: f64 = (4.0 * l.f609);let t30b: f64 = (t30a * 0.01);(l.f6f7, l.f6f8, l.f6f9, ) = (t30b, 0.0, 0.0, );}
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {
            let (t30d, t30e, t30f,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t30c: f64 = (-l.f6f7);
        (t30c, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t30d, t30e, t30f, );
        }
        if ((l.f48c != 0.0) && (l.f48e != 0.0)) {let t310: f64 = (l.f6f3 * l.f6f3);let t311: f64 = (t310 + l.f6f7);let t312: f64 = (t311).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t312, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t312)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t312)), );let t313: f64 = (l.f6f3 + l.f6f7);let t314: f64 = (0.5 * t313);let t315: f64 = (l.f609 + t314);(l.f5ed, l.f5ee, l.f5ef, ) = (t315, (0.5 * (l.f6f4 + l.f6f8)), (0.5 * (l.f6f5 + l.f6f9)), );}
        if ((l.f48c != 0.0) && (l.f48e == 0.0)) {(l.f5f5, l.f5fe, l.f5ff, ) = (l.f609, 0.0, 0.0, );(l.f5ed, l.f5ee, l.f5ef, ) = (l.f609, 0.0, 0.0, );}
        if (l.f48c != 0.0) {(l.f79, l.f7e, l.f7f, ) = (l.f536, l.f537, l.f538, );}
        let t316: f64 = (l.f743 - l.f741);let t317: f64 = (l.f745 - t316);let t318: f64 = if t317 > 0.0 { 1.0 } else { 0.0 };l.f490 = t318;let t319: f64 = (l.f745 / l.f5f5);let t31a: f64 = (l.f743 - l.f741);let t31b: f64 = (t31a / l.f5f5);let t31c: f64 = (t319 - t31b);let t31d: f64 = (l.f5f5 - l.f5ed);let t31e: f64 = (l.f743 * t31d);let t31f: f64 = (l.f5ed * p.p85);let t320: f64 = (t31e / t31f);let t321: f64 = (t31c + t320);let t322: f64 = (l.f645 * t321);let t323: f64 = (t322).abs();let t324: f64 = if t323 < 230.25850929940458 { 1.0 } else { 0.0 };l.f492 = t324;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_171(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f48c != 0.0) && (l.f490 != 0.0)) && (l.f492 != 0.0)) {let t325: f64 = (l.f745 / l.f5f5);let t326: f64 = (l.f743 - l.f741);let t327: f64 = (t326 / l.f5f5);let t328: f64 = (t325 - t327);let t329: f64 = (l.f5f5 - l.f5ed);let t32a: f64 = (l.f743 * t329);let t32b: f64 = (l.f5ed * p.p85);let t32c: f64 = (t32a / t32b);let t32d: f64 = (t328 + t32c);let t32e: f64 = (l.f645 * t32d);let t32f: f64 = (t32e).exp();(l.f81, l.f86, l.f87, ) = (t32f, (t32f * (l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t326 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t32b) - (t32a * (l.f5ee * p.p85))) / (t32b * t32b))))), (t32f * (l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t326 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t32b) - (t32a * (l.f5ef * p.p85))) / (t32b * t32b))))), );}
        let t330: f64 = (l.f745 / l.f5f5);let t331: f64 = (l.f743 - l.f741);let t332: f64 = (t331 / l.f5f5);let t333: f64 = (t330 - t332);let t334: f64 = (l.f5f5 - l.f5ed);let t335: f64 = (l.f743 * t334);let t336: f64 = (l.f5ed * p.p85);let t337: f64 = (t335 / t336);let t338: f64 = (t333 + t337);let t339: f64 = (l.f645 * t338);let t33a: f64 = (-230.25850929940458);let t33b: f64 = if t339 < t33a { 1.0 } else { 0.0 };l.f494 = t33b;
        if ((((l.f48c != 0.0) && (l.f490 != 0.0)) && (l.f492 == 0.0)) && (l.f494 != 0.0)) {
            let t33c: f64 = (-230.25850929940458);let t33d: f64 = (l.f745 / l.f5f5);let t33e: f64 = (l.f743 - l.f741);let t33f: f64 = (t33e / l.f5f5);let t340: f64 = (t33d - t33f);let t341: f64 = (l.f5f5 - l.f5ed);let t342: f64 = (l.f743 * t341);let t343: f64 = (l.f5ed * p.p85);let t344: f64 = (t342 / t343);let t345: f64 = (t340 + t344);let t346: f64 = (l.f645 * t345);let t347: f64 = (t33c - t346);let t348: f64 = (-230.25850929940458);let t349: f64 = (l.f745 / l.f5f5);let t34a: f64 = (l.f743 - l.f741);let t34b: f64 = (t34a / l.f5f5);let t34c: f64 = (t349 - t34b);let t34d: f64 = (l.f5f5 - l.f5ed);let t34e: f64 = (l.f743 * t34d);let t34f: f64 = (l.f5ed * p.p85);let t350: f64 = (t34e / t34f);let t351: f64 = (t34c + t350);let t352: f64 = (l.f645 * t351);let t353: f64 = (t348 - t352);let t354: f64 = (-230.25850929940458);let t355: f64 = (l.f745 / l.f5f5);let t356: f64 = (l.f743 - l.f741);let t357: f64 = (t356 / l.f5f5);let t358: f64 = (t355 - t357);let t359: f64 = (l.f5f5 - l.f5ed);let t35a: f64 = (l.f743 * t359);let t35b: f64 = (l.f5ed * p.p85);let t35c: f64 = (t35a / t35b);let t35d: f64 = (t358 + t35c);let t35e: f64 = (l.f645 * t35d);let t35f: f64 = (t354 - t35e);let t360: f64 = (t35f * 0.3333333333333333);let t361: f64 = (1.0 + t360);let t362: f64 = (t353 * t361);let t363: f64 = (0.5 * t362);let t364: f64 = (1.0 + t363);let t365: f64 = (t347 * t364);let t366: f64 = (1.0 + t365);let t367: f64 = (1e-100 / t366);
            (l.f81, l.f86, l.f87, ) = (t367, (-((1e-100 * (((-(l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t33e * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t343) - (t342 * (l.f5ee * p.p85))) / (t343 * t343))))) * t364) + (t347 * (0.5 * (((-(l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t34a * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t34f) - (t34e * (l.f5ee * p.p85))) / (t34f * t34f))))) * t361) + (t353 * ((-(l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t356 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t35b) - (t35a * (l.f5ee * p.p85))) / (t35b * t35b))))) * 0.3333333333333333))))))) / (t366 * t366))), (-((1e-100 * (((-(l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t33e * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t343) - (t342 * (l.f5ef * p.p85))) / (t343 * t343))))) * t364) + (t347 * (0.5 * (((-(l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t34a * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t34f) - (t34e * (l.f5ef * p.p85))) / (t34f * t34f))))) * t361) + (t353 * ((-(l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t356 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t35b) - (t35a * (l.f5ef * p.p85))) / (t35b * t35b))))) * 0.3333333333333333))))))) / (t366 * t366))), );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_172(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        if ((((l.f48c != 0.0) && (l.f490 != 0.0)) && (l.f492 == 0.0)) && (l.f494 == 0.0)) {
            let t368: f64 = (l.f745 / l.f5f5);let t369: f64 = (l.f743 - l.f741);let t36a: f64 = (t369 / l.f5f5);let t36b: f64 = (t368 - t36a);let t36c: f64 = (l.f5f5 - l.f5ed);let t36d: f64 = (l.f743 * t36c);let t36e: f64 = (l.f5ed * p.p85);let t36f: f64 = (t36d / t36e);let t370: f64 = (t36b + t36f);let t371: f64 = (l.f645 * t370);let t372: f64 = (t371 - 230.25850929940458);let t373: f64 = (l.f745 / l.f5f5);let t374: f64 = (l.f743 - l.f741);let t375: f64 = (t374 / l.f5f5);let t376: f64 = (t373 - t375);let t377: f64 = (l.f5f5 - l.f5ed);let t378: f64 = (l.f743 * t377);let t379: f64 = (l.f5ed * p.p85);let t37a: f64 = (t378 / t379);let t37b: f64 = (t376 + t37a);let t37c: f64 = (l.f645 * t37b);let t37d: f64 = (t37c - 230.25850929940458);let t37e: f64 = (l.f745 / l.f5f5);let t37f: f64 = (l.f743 - l.f741);let t380: f64 = (t37f / l.f5f5);let t381: f64 = (t37e - t380);let t382: f64 = (l.f5f5 - l.f5ed);let t383: f64 = (l.f743 * t382);let t384: f64 = (l.f5ed * p.p85);let t385: f64 = (t383 / t384);let t386: f64 = (t381 + t385);let t387: f64 = (l.f645 * t386);let t388: f64 = (t387 - 230.25850929940458);let t389: f64 = (t388 * 0.3333333333333333);let t38a: f64 = (1.0 + t389);let t38b: f64 = (t37d * t38a);let t38c: f64 = (0.5 * t38b);let t38d: f64 = (1.0 + t38c);let t38e: f64 = (t372 * t38d);let t38f: f64 = (1.0 + t38e);let t390: f64 = (1e100 * t38f);
            (l.f81, l.f86, l.f87, ) = (t390, (1e100 * (((l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t369 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t36e) - (t36d * (l.f5ee * p.p85))) / (t36e * t36e)))) * t38d) + (t372 * (0.5 * (((l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t374 * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t379) - (t378 * (l.f5ee * p.p85))) / (t379 * t379)))) * t38a) + (t37d * ((l.f645 * (((((l.f746 * l.f5f5) - (l.f745 * l.f5fe)) / (l.f5f5 * l.f5f5)) - (-((t37f * l.f5fe) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5fe - l.f5ee)) * t384) - (t383 * (l.f5ee * p.p85))) / (t384 * t384)))) * 0.3333333333333333))))))), (1e100 * (((l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t369 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t36e) - (t36d * (l.f5ef * p.p85))) / (t36e * t36e)))) * t38d) + (t372 * (0.5 * (((l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t374 * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t379) - (t378 * (l.f5ef * p.p85))) / (t379 * t379)))) * t38a) + (t37d * ((l.f645 * (((((l.f747 * l.f5f5) - (l.f745 * l.f5ff)) / (l.f5f5 * l.f5f5)) - (-((t37f * l.f5ff) / (l.f5f5 * l.f5f5)))) + ((((l.f743 * (l.f5ff - l.f5ef)) * t384) - (t383 * (l.f5ef * p.p85))) / (t384 * t384)))) * 0.3333333333333333))))))), );
        }
        if ((l.f48c != 0.0) && (l.f490 == 0.0)) {(l.f81, l.f86, l.f87, ) = (1.0, 0.0, 0.0, );}
        let t391: f64 = if ((p.p91 == 0.0) || (l.f745 < l.f741)) { 1.0 } else { 0.0 };l.f496 = t391;
        if ((l.f48c != 0.0) && (l.f496 != 0.0)) {let t392: f64 = (l.f79 * p.p90);(l.f7a, l.f7b, l.f7c, ) = (t392, (l.f7e * p.p90), (l.f7f * p.p90), );}
        if ((l.f48c != 0.0) && (l.f496 == 0.0)) {let t393: f64 = (l.f79 * p.p90);let t394: f64 = (-p.p91);let t395: f64 = (l.f745 - l.f741);let t396: f64 = (t394 * t395);let t397: f64 = (l.f745 - l.f741);let t398: f64 = (t396 * t397);let t399: f64 = (l.f6e9 / l.f6e7);let t39a: f64 = (t399).ln();let t39b: f64 = (p.p98 * t39a);let t39c: f64 = (t39b).exp();let t39d: f64 = (t398 * t39c);let t39e: f64 = (t39d).exp();let t39f: f64 = (t393 * t39e);(l.f7a, l.f7b, l.f7c, ) = (t39f, (((l.f7e * p.p90) * t39e) + (t393 * (t39e * ((((t394 * l.f746) * t397) + (t396 * l.f746)) * t39c)))), (((l.f7f * p.p90) * t39e) + (t393 * (t39e * ((((t394 * l.f747) * t397) + (t396 * l.f747)) * t39c)))), );}
        if (l.f48c != 0.0) {
            let (t3a0, t3a1, t3a2,) = {
    if (l.f7a > p.p79) {
        (p.p79, 0.0, 0.0,)
    } else {
        (l.f7a, l.f7b, l.f7c,)
    }
};
            (l.f7a, l.f7b, l.f7c, ) = (t3a0, t3a1, t3a2, );
        }
        if (l.f48c != 0.0) {let t3a3: f64 = (l.f64d * l.f7a);(l.f617, l.f618, l.f619, ) = (t3a3, (l.f64d * l.f7b), (l.f64d * l.f7c), );let t3a4: f64 = (1.6021918e-19 * l.f0);let t3a5: f64 = (l.f617 - l.f64d);let t3a6: f64 = (t3a4 * t3a5);(l.f66b, l.f66c, l.f66d, ) = (t3a6, (t3a4 * l.f618), (t3a4 * l.f619), );}
        let t3a7: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };l.f498 = t3a7;
        if ((l.f48c != 0.0) && (l.f498 != 0.0)) {let t3a8: f64 = (1e-23 / l.f669);let t3a9: f64 = (l.f66b * t3a8);(l.f67d, l.f67e, l.f67f, ) = (t3a9, (l.f66c * t3a8), (l.f66d * t3a8), );let t3aa: f64 = (nv3 - 0.0);(l.f663, l.f664, ) = (t3aa, 1.0, );let t3ab: f64 = (l.f663 - l.f67d);let t3ac: f64 = (t3ab / p.p92);(l.f57c, l.f57d, l.f57e, l.f57f, ) = (t3ac, ((-l.f67e) / p.p92), ((-l.f67f) / p.p92), (l.f664 / p.p92), );let t3ad: f64 = (1e-23 / l.f669);let t3ae: f64 = (l.f663 / t3ad);(l.f66e, l.f66f, l.f670, l.f671, ) = (t3ae, 0.0, 0.0, (l.f664 / t3ad), );}
        if ((l.f48c != 0.0) && (l.f498 == 0.0)) {(l.f67d, l.f67e, l.f67f, ) = (l.f66b, l.f66c, l.f66d, );(l.f66e, l.f66f, l.f670, l.f671, ) = (l.f67d, l.f67e, l.f67f, 0.0, );}
        let t3af: f64 = if ((p.p91 == 0.0) || (l.f745 < l.f743)) { 1.0 } else { 0.0 };l.f49a = t3af;
        if ((l.f48c != 0.0) && (l.f49a != 0.0)) {let t3b0: f64 = (l.f81 * p.p90);(l.f82, l.f83, l.f84, ) = (t3b0, (l.f86 * p.p90), (l.f87 * p.p90), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_173(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv5 = ctx.node_voltage(nodes[5]);
        if ((l.f48c != 0.0) && (l.f49a == 0.0)) {let t3b1: f64 = (l.f81 * p.p90);let t3b2: f64 = (-p.p91);let t3b3: f64 = (l.f745 - l.f743);let t3b4: f64 = (t3b2 * t3b3);let t3b5: f64 = (l.f745 - l.f743);let t3b6: f64 = (t3b4 * t3b5);let t3b7: f64 = (l.f6e9 / l.f6e7);let t3b8: f64 = (t3b7).ln();let t3b9: f64 = (p.p98 * t3b8);let t3ba: f64 = (t3b9).exp();let t3bb: f64 = (t3b6 * t3ba);let t3bc: f64 = (t3bb).exp();let t3bd: f64 = (t3b1 * t3bc);(l.f82, l.f83, l.f84, ) = (t3bd, (((l.f86 * p.p90) * t3bc) + (t3b1 * (t3bc * ((((t3b2 * l.f746) * t3b5) + (t3b4 * l.f746)) * t3ba)))), (((l.f87 * p.p90) * t3bc) + (t3b1 * (t3bc * ((((t3b2 * l.f747) * t3b5) + (t3b4 * l.f747)) * t3ba)))), );}
        if (l.f48c != 0.0) {
            let (t3be, t3bf, t3c0,) = {
    if (l.f82 > p.p79) {
        (p.p79, 0.0, 0.0,)
    } else {
        (l.f82, l.f83, l.f84,)
    }
};
            (l.f82, l.f83, l.f84, ) = (t3be, t3bf, t3c0, );
        }
        if (l.f48c != 0.0) {let t3c1: f64 = (l.f64d * l.f82);(l.f61b, l.f61c, l.f61d, ) = (t3c1, (l.f64d * l.f83), (l.f64d * l.f84), );let t3c2: f64 = (1.6021918e-19 * l.f0);let t3c3: f64 = (l.f61b - l.f64d);let t3c4: f64 = (t3c2 * t3c3);(l.f674, l.f675, l.f676, ) = (t3c4, (t3c2 * l.f61c), (t3c2 * l.f61d), );}
        let t3c5: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };l.f49c = t3c5;
        if ((l.f48c != 0.0) && (l.f49c != 0.0)) {let t3c6: f64 = (1e-23 / l.f669);let t3c7: f64 = (l.f674 * t3c6);(l.f681, l.f682, l.f683, ) = (t3c7, (l.f675 * t3c6), (l.f676 * t3c6), );let t3c8: f64 = (nv4 - 0.0);(l.f666, l.f667, ) = (t3c8, 1.0, );let t3c9: f64 = (l.f666 - l.f681);let t3ca: f64 = (t3c9 / p.p92);(l.f581, l.f582, l.f583, l.f584, ) = (t3ca, ((-l.f682) / p.p92), ((-l.f683) / p.p92), (l.f667 / p.p92), );let t3cb: f64 = (1e-23 / l.f669);let t3cc: f64 = (l.f666 / t3cb);(l.f677, l.f678, l.f679, l.f67a, ) = (t3cc, 0.0, 0.0, (l.f667 / t3cb), );}
        if ((l.f48c != 0.0) && (l.f49c == 0.0)) {(l.f681, l.f682, l.f683, ) = (l.f674, l.f675, l.f676, );(l.f677, l.f678, l.f679, l.f67a, ) = (l.f681, l.f682, l.f683, 0.0, );}
        if (l.f48c != 0.0) {let t3cd: f64 = (l.f61f - l.f745);(l.f7a7, l.f7a8, l.f7a9, ) = (t3cd, (-l.f746), (-l.f747), );let t3ce: f64 = (l.f7a7 * l.f7a7);let t3cf: f64 = (4.0 * l.f5a3);let t3d0: f64 = (t3cf * l.f5a3);let t3d1: f64 = (t3ce + t3d0);let t3d2: f64 = (t3d1).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3d2, (((l.f7a8 * l.f7a7) + (l.f7a7 * l.f7a8)) / (2.0 * t3d2)), (((l.f7a9 * l.f7a7) + (l.f7a7 * l.f7a9)) / (2.0 * t3d2)), );let t3d3: f64 = (l.f7a7 + l.f6f7);let t3d4: f64 = (0.5 * t3d3);(l.f7a7, l.f7a8, l.f7a9, ) = (t3d4, (0.5 * (l.f7a8 + l.f6f8)), (0.5 * (l.f7a9 + l.f6f9)), );}
        let t3d5: f64 = if l.f7a7 < 0.0 { 1.0 } else { 0.0 };l.f49e = t3d5;
        if ((l.f48c != 0.0) && (l.f49e != 0.0)) {(l.f7a7, l.f7a8, l.f7a9, ) = (0.0, 0.0, 0.0, );}
        if (l.f48c != 0.0) {let t3d6: f64 = (2.0 * l.f6b);let t3d7: f64 = (t3d6 * l.f7a7);let t3d8: f64 = (1.6021918e-19 * l.f5dd);let t3d9: f64 = (t3d7 / t3d8);let t3da: f64 = (t3d9).sqrt();(l.f7bc, l.f7c1, l.f7c2, ) = (t3da, (((t3d6 * l.f7a8) / t3d8) / (2.0 * t3da)), (((t3d6 * l.f7a9) / t3d8) / (2.0 * t3da)), );let t3db: f64 = (p.p94 - l.f7bc);let t3dc: f64 = (t3db - 1e-7);(l.f6f3, l.f6f4, l.f6f5, ) = (t3dc, (-l.f7c1), (-l.f7c2), );let t3dd: f64 = (4.0 * p.p94);let t3de: f64 = (t3dd * 1e-7);(l.f6f7, l.f6f8, l.f6f9, ) = (t3de, 0.0, 0.0, );}
        if (l.f48c != 0.0) {
            let (t3e0, t3e1, t3e2,) = {
    if (l.f6f7 > 0.0) {
        (l.f6f7, l.f6f8, l.f6f9,)
    } else {
        let t3df: f64 = (-l.f6f7);
        (t3df, (-l.f6f8), (-l.f6f9),)
    }
};
            (l.f6f7, l.f6f8, l.f6f9, ) = (t3e0, t3e1, t3e2, );
        }
        if (l.f48c != 0.0) {let t3e3: f64 = (l.f6f3 * l.f6f3);let t3e4: f64 = (t3e3 + l.f6f7);let t3e5: f64 = (t3e4).sqrt();(l.f6f7, l.f6f8, l.f6f9, ) = (t3e5, ((((l.f6f4 * l.f6f3) + (l.f6f3 * l.f6f4)) + l.f6f8) / (2.0 * t3e5)), ((((l.f6f5 * l.f6f3) + (l.f6f3 * l.f6f5)) + l.f6f9) / (2.0 * t3e5)), );let t3e6: f64 = (l.f6f3 + l.f6f7);let t3e7: f64 = (0.5 * t3e6);let t3e8: f64 = (p.p94 - t3e7);(l.f7bc, l.f7c1, l.f7c2, ) = (t3e8, (-(0.5 * (l.f6f4 + l.f6f8))), (-(0.5 * (l.f6f5 + l.f6f9))), );}
        let t3e9: f64 = if p.p95 > 0.0 { 1.0 } else { 0.0 };l.f4a0 = t3e9;
        if ((l.f48c != 0.0) && (l.f4a0 != 0.0)) {let t3ea: f64 = (1.0 / l.f7bd);let t3eb: f64 = (l.f7bc * t3ea);(l.f7cc, l.f7cd, l.f7ce, ) = (t3eb, ((l.f7c1 * t3ea) + (l.f7bc * (-(l.f7be / (l.f7bd * l.f7bd))))), ((l.f7c2 * t3ea) + (l.f7bc * (-(l.f7bf / (l.f7bd * l.f7bd))))), );let t3ec: f64 = (nv5 - 0.0);(l.f7c9, l.f7ca, ) = (t3ec, 1.0, );let t3ed: f64 = (l.f7c9 - l.f7cc);let t3ee: f64 = (t3ed / p.p95);(l.f59e, l.f59f, l.f5a0, l.f5a1, ) = (t3ee, ((-l.f7cd) / p.p95), ((-l.f7ce) / p.p95), (l.f7ca / p.p95), );let t3ef: f64 = (1.0 / l.f7bd);let t3f0: f64 = (l.f7c9 / t3ef);(l.f7c3, l.f7c4, l.f7c5, l.f7c6, ) = (t3f0, (-((l.f7c9 * (-(l.f7be / (l.f7bd * l.f7bd)))) / (t3ef * t3ef))), (-((l.f7c9 * (-(l.f7bf / (l.f7bd * l.f7bd)))) / (t3ef * t3ef))), (l.f7ca / t3ef), );}
        if ((l.f48c != 0.0) && (l.f4a0 == 0.0)) {(l.f7cc, l.f7cd, l.f7ce, ) = (l.f7bc, l.f7c1, l.f7c2, );(l.f7c3, l.f7c4, l.f7c5, l.f7c6, ) = (l.f7cc, l.f7cd, l.f7ce, 0.0, );}
        if (l.f48c != 0.0) {let t3f1: f64 = (l.f5dd * l.f0);let t3f2: f64 = (t3f1 * 1.6021918e-19);let t3f3: f64 = (-t3f2);let t3f4: f64 = (t3f3 * p.p94);l.f655 = t3f4;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_174(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f48c != 0.0) {let t3f5: f64 = (l.f5ad * l.f66e);let t3f6: f64 = (-p.p94);let t3f7: f64 = (t3f6 / l.f5ad);let t3f8: f64 = (t3f7).exp();let t3f9: f64 = (-l.f7c3);let t3fa: f64 = (t3f9 / l.f5ad);let t3fb: f64 = (t3fa).exp();let t3fc: f64 = (t3f8 - t3fb);let t3fd: f64 = (t3f5 * t3fc);(l.f657, l.f658, l.f659, l.f65a, l.f65b, ) = (t3fd, (((l.f5ad * l.f66f) * t3fc) + (t3f5 * (-(t3fb * ((-l.f7c4) / l.f5ad))))), (((l.f5ad * l.f670) * t3fc) + (t3f5 * (-(t3fb * ((-l.f7c5) / l.f5ad))))), ((l.f5ad * l.f671) * t3fc), (t3f5 * (-(t3fb * ((-l.f7c6) / l.f5ad)))), );let t3fe: f64 = (l.f5ad * l.f677);let t3ff: f64 = (p.p94 - l.f7c3);let t400: f64 = (-t3ff);let t401: f64 = (t400 / l.f5ad);let t402: f64 = (t401).exp();let t403: f64 = (t402 - 1.0);let t404: f64 = (t3fe * t403);(l.f65d, l.f65e, l.f65f, l.f660, l.f661, ) = (t404, (((l.f5ad * l.f678) * t403) + (t3fe * (t402 * ((-(-l.f7c4)) / l.f5ad)))), (((l.f5ad * l.f679) * t403) + (t3fe * (t402 * ((-(-l.f7c5)) / l.f5ad)))), ((l.f5ad * l.f67a) * t403), (t3fe * (t402 * ((-(-l.f7c6)) / l.f5ad))), );let t405: f64 = (l.f655 + l.f657);let t406: f64 = (t405 + l.f65d);let t407: f64 = (-t406);(l.f6a4, l.f6a5, l.f6a6, l.f6a7, l.f6a8, l.f6a9, ) = (t407, (-(l.f658 + l.f65e)), (-(l.f659 + l.f65f)), (-l.f65a), (-l.f660), (-(l.f65b + l.f661)), );let t408: f64 = (l.f685 + l.f6a4);(l.f685, l.f686, l.f687, l.f688, l.f689, l.f68a, ) = (t408, (l.f686 + l.f6a5), (l.f687 + l.f6a6), (l.f688 + l.f6a7), (l.f689 + l.f6a8), (l.f68a + l.f6a9), );l.f711 = 0.0;}
        if (l.f48c == 0.0) {let t409: f64 = (l.f55e - l.f54e);let t40a: f64 = (l.f711 * t409);(l.f6a4, l.f6a5, l.f6a6, l.f6a7, l.f6a8, l.f6a9, ) = (t40a, (l.f711 * (l.f55f - l.f54f)), (l.f711 * (l.f560 - l.f550)), 0.0, 0.0, 0.0, );}
        let t40b: f64 = if ((l.f6af > 0.0) && (l.f6af >= p.p4)) { 1.0 } else { 0.0 };l.f4a2 = t40b;let t40c: f64 = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };l.f4a3 = t40c;let t40d: f64 = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };l.f4a5 = t40d;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let t0: f64 = (8.8541878176e-12 * 11.8);l.f6b = t0;l.f6c = 0.0;let t2: f64 = (-250.0);
        let (t4,) = {
    if (p.p6 > t2) {
        (p.p6,)
    } else {
        let t3: f64 = (-250.0);
        (t3,)
    }
};
        l.f705 = t4;l.f706 = 0.0;let tf: f64 = if ((!param_given[6]) && param_given[96]) { 1.0 } else { 0.0 };l.fdb = tf;l.f1ac = 0.0;
        if (l.fdb != 0.0) {
            let t19: f64 = (-250.0);
            let (t1b,) = {
    if (p.p96 > t19) {
        (p.p96,)
    } else {
        let t1a: f64 = (-250.0);
        (t1a,)
    }
};
            l.f705 = t1b;l.f706 = 0.0;
        }
        let (t40e,) = {
    if (p.p5 > 1e-12) {
        (p.p5,)
    } else {
        (1e-12,)
    }
};
        l.f57a = t40e;l.f57b = 0.0;
        let (t40f,) = {
    if (p.p8 > 1e-12) {
        (p.p8,)
    } else {
        (1e-12,)
    }
};
        l.f30 = t40f;l.f31 = 0.0;
        let (t410,) = {
    if (p.p9 > 1e-18) {
        (p.p9,)
    } else {
        (1e-18,)
    }
};
        l.f34 = t410;l.f35 = 0.0;
        let (t411,) = {
    if (p.p10 > 1e-18) {
        (p.p10,)
    } else {
        (1e-18,)
    }
};
        l.f32 = t411;l.f33 = 0.0;
        let (t412,) = {
    if (p.p11 > 0.05) {
        (p.p11,)
    } else {
        (0.05,)
    }
};
        l.f771 = t412;l.f772 = 0.0;
        let (t413,) = {
    if (p.p12 > 0.05) {
        (p.p12,)
    } else {
        (0.05,)
    }
};
        l.f779 = t413;l.f77a = 0.0;
        let (t1,) = {
    if (p.p13 > 0.05) {
        (p.p13,)
    } else {
        (0.05,)
    }
};
        l.f775 = t1;l.f776 = 0.0;
        let (t6,) = {
    if (p.p14 > 0.05) {
        let (t5,) = {
            if (p.p14 < 0.95) {
                (p.p14,)
            } else {
                (0.95,)
            }
        };
        (t5,)
    } else {
        (0.05,)
    }
};
        l.f623 = t6;l.f624 = 0.0;
        let (t8,) = {
    if (p.p15 > 0.05) {
        let (t7,) = {
            if (p.p15 < 0.95) {
                (p.p15,)
            } else {
                (0.95,)
            }
        };
        (t7,)
    } else {
        (0.05,)
    }
};
        l.f653 = t8;l.f654 = 0.0;
        let (ta,) = {
    if (p.p16 > 0.05) {
        let (t9,) = {
            if (p.p16 < 0.95) {
                (p.p16,)
            } else {
                (0.95,)
            }
        };
        (t9,)
    } else {
        (0.05,)
    }
};
        l.f62f = ta;l.f630 = 0.0;l.f631 = p.p17;l.f632 = 0.0;l.f641 = p.p18;l.f642 = 0.0;l.f639 = p.p19;l.f63a = 0.0;
        let (tb,) = {
    if (p.p20 > 0.0) {
        (p.p20,)
    } else {
        (0.0,)
    }
};
        l.f546 = tb;l.f547 = 0.0;
        let (tc,) = {
    if (p.p21 > 0.0) {
        (p.p21,)
    } else {
        (0.0,)
    }
};
        l.f54a = tc;l.f54b = 0.0;
        let (td,) = {
    if (p.p22 > 0.0) {
        (p.p22,)
    } else {
        (0.0,)
    }
};
        l.f548 = td;l.f549 = 0.0;
        let (te,) = {
    if (p.p23 > 0.0) {
        (p.p23,)
    } else {
        (0.0,)
    }
};
        l.f39 = te;l.f3a = 0.0;
        let (t10,) = {
    if (p.p24 > 0.0) {
        (p.p24,)
    } else {
        (0.0,)
    }
};
        l.f3d = t10;l.f3e = 0.0;
        let (t11,) = {
    if (p.p25 > 0.0) {
        (p.p25,)
    } else {
        (0.0,)
    }
};
        l.f3b = t11;l.f3c = 0.0;
        let (t12,) = {
    if (p.p26 > 1e-9) {
        (p.p26,)
    } else {
        (1e-9,)
    }
};
        l.f80c = t12;l.f80d = 0.0;
        let (t13,) = {
    if (p.p27 > 1e-9) {
        (p.p27,)
    } else {
        (1e-9,)
    }
};
        l.f80a = t13;l.f80b = 0.0;
        let (t14,) = {
    if (p.p28 > 0.0) {
        (p.p28,)
    } else {
        (0.0,)
    }
};
        l.f3f = t14;l.f40 = 0.0;
        let (t15,) = {
    if (p.p29 > 0.0) {
        (p.p29,)
    } else {
        (0.0,)
    }
};
        l.f43 = t15;l.f44 = 0.0;
        let (t16,) = {
    if (p.p30 > 0.0) {
        (p.p30,)
    } else {
        (0.0,)
    }
};
        l.f41 = t16;l.f42 = 0.0;
        let (t17,) = {
    if (p.p31 > 0.01) {
        (p.p31,)
    } else {
        (0.01,)
    }
};
        l.f5c3 = t17;l.f5c4 = 0.0;
        let (t18,) = {
    if (p.p32 > 0.01) {
        (p.p32,)
    } else {
        (0.01,)
    }
};
        l.f5c7 = t18;l.f5c8 = 0.0;
    }
}
