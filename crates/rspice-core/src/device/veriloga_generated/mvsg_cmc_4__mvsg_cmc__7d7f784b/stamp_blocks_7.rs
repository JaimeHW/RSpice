#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv17 = ctx.node_voltage(nodes[17]);
        if (l.f1fd4 != 0.0) {let t0: f64 = (-l.f12d6);let t1: f64 = (t0 * l.f12de);let t2: f64 = (t1 * l.f12c6);let t3: f64 = (t2 * l.f12c0);let t4: f64 = (t3 * l.f12d4);let t5: f64 = t4;(l.f12c3, l.f12c4, ) = (t5, (t3 * l.f12d5), );let t6: f64 = (l.f12cb / l.f12cc);let t7: f64 = (t6 * l.f129c);(l.f1291, l.f1293, l.f1294, l.f1292, ) = (t7, ((-((l.f12cb * l.f12cd) / (l.f12cc * l.f12cc))) * l.f129c), (t6 * l.f129e), (t6 * l.f129d), );}
        if (l.f1fd4 != 0.0) {
            let t8: f64 = (-50.0);
            let (t19, t1b, t1c, t1a,) = {
    if ((!(l.f1291 > 50.0)) && (!(l.f1291 < t8))) {
        let t9: f64 = (l.f1291).exp();
        (t9, (t9 * l.f1293), (t9 * l.f1294), (t9 * l.f1292),)
    } else {
        let ta: f64 = (-50.0);
        let (t15, t17, t18, t16,) = {
            if ((!(l.f1291 > 50.0)) && (l.f1291 < ta)) {
                let tb: f64 = (-50.0);let tc: f64 = (tb).exp();
                (tc, 0.0, 0.0, 0.0,)
            } else {
                let (t11, t13, t14, t12,) = {
                    if (l.f1291 > 50.0) {
                        let td: f64 = (50.0_f64).exp();let te: f64 = (l.f1291 - 50.0);let tf: f64 = (1.0 + te);let t10: f64 = (td * tf);
                        (t10, (td * l.f1293), (td * l.f1294), (td * l.f1292),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t11, t13, t14, t12,)
            }
        };
        (t15, t17, t18, t16,)
    }
};
            (l.f128d, l.f128f, l.f1290, l.f128e, ) = (t19, t1b, t1c, t1a, );
        }
        if (l.f1fd4 != 0.0) {let t1d: f64 = (l.f128d - 1.0);let t1e: f64 = (l.f12c3 * t1d);(l.f12b7, l.f12b9, l.f12ba, l.f12b8, ) = (t1e, ((l.f12c4 * t1d) + (l.f12c3 * l.f128f)), (l.f12c3 * l.f1290), (l.f12c3 * l.f128e), );let t1f: f64 = (l.f12a5 + l.f12b7);(l.f12bb, l.f12bd, l.f12be, l.f12bc, ) = (t1f, (l.f12a7 + l.f12b9), (l.f12a8 + l.f12ba), (l.f12a6 + l.f12b8), );(l.f12ce, l.f12d0, l.f12d1, l.f12cf, ) = (l.f12bb, l.f12bd, l.f12be, l.f12bc, );(l.f2133, l.f213d, l.f213e, l.f213c, ) = (l.f12ce, l.f12d0, l.f12d1, l.f12cf, );(l.f1350, l.f1352, l.f1353, l.f1351, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1343, l.f1344, ) = (0.0, 0.0, );(l.f1345, l.f1346, ) = (0.0, 0.0, );let t20: f64 = (p.p6 * (nv8 - nv17));(l.f135a, l.f135c, l.f135b, ) = (t20, p.p6, (-p.p6), );(l.f134e, l.f134f, ) = (l.f215b, l.f215c, );l.f135d = p.p265;l.f12e1 = p.p267;l.f131d = p.p266;l.f134b = p.p263;l.f1349 = p.p281;l.f1359 = p.p280;(l.f1356, l.f1357, ) = (l.f22f2, l.f22f3, );l.f1360 = p.p0;l.f1348 = p.p2;let t21: f64 = (1.0 - p.p255);let t22: f64 = (t21 * p.p264);l.f1341 = t22;l.f1347 = p.p279;l.f135e = p.p274;l.f12e2 = p.p275;let t23: f64 = (1.0 - p.p255);let t24: f64 = (t23 * p.p273);l.f1342 = t24;l.f134d = p.p272;l.f134a = p.p257;l.f135f = p.p256;l.f1358 = p.p6;(l.f133d, l.f133f, l.f1340, l.f133e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12df, l.f12e0, ) = (0.0, 0.0, );(l.f1354, l.f1355, ) = (0.0, 0.0, );(l.f1319, l.f131b, l.f131c, l.f131a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1321, l.f1323, l.f1324, l.f1322, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1327, l.f1329, l.f132a, l.f1328, ) = (0.0, 0.0, 0.0, 0.0, );(l.f131e, l.f1320, l.f131f, ) = (0.0, 0.0, 0.0, );(l.f1339, l.f133b, l.f133c, l.f133a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12eb, l.f12ed, l.f12ee, l.f12ec, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12f1, l.f12f2, ) = (0.0, 0.0, );(l.f12e3, l.f12e5, l.f12e6, l.f12e4, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12e9, l.f12ea, ) = (0.0, 0.0, );(l.f1317, l.f1318, ) = (0.0, 0.0, );(l.f12f3, l.f12f5, l.f12f6, l.f12f4, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1303, l.f1305, l.f1306, l.f1304, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12f7, l.f12f9, l.f12fa, l.f12f8, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1313, l.f1315, l.f1316, l.f1314, ) = (0.0, 0.0, 0.0, 0.0, );(l.f130f, l.f1311, l.f1312, l.f1310, ) = (0.0, 0.0, 0.0, 0.0, );l.f134c = 0.0;(l.f12ef, l.f12f0, ) = (0.0, 0.0, );(l.f12e7, l.f12e8, ) = (0.0, 0.0, );(l.f1325, l.f1326, ) = (0.0, 0.0, );(l.f130d, l.f130e, ) = (0.0, 0.0, );(l.f1301, l.f1302, ) = (0.0, 0.0, );(l.f1337, l.f1338, ) = (0.0, 0.0, );(l.f1333, l.f1335, l.f1336, l.f1334, ) = (0.0, 0.0, 0.0, 0.0, );(l.f130b, l.f130c, ) = (0.0, 0.0, );(l.f12ff, l.f1300, ) = (0.0, 0.0, );(l.f1331, l.f1332, ) = (0.0, 0.0, );(l.f1307, l.f1309, l.f130a, l.f1308, ) = (0.0, 0.0, 0.0, 0.0, );(l.f12fb, l.f12fd, l.f12fe, l.f12fc, ) = (0.0, 0.0, 0.0, 0.0, );(l.f132f, l.f1330, ) = (0.0, 0.0, );(l.f132b, l.f132d, l.f132e, l.f132c, ) = (0.0, 0.0, 0.0, 0.0, );let t25: f64 = (l.f134a / l.f134e);let t26: f64 = (-l.f135f);let t27: f64 = (t25 * t26);(l.f1317, l.f1318, ) = (t27, ((-((l.f134a * l.f134f) / (l.f134e * l.f134e))) * t26), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        l: &mut StampLocals,
    ) {
        if (l.f1fd4 != 0.0) {
            let t28: f64 = (-50.0);
            let (t35, t36,) = {
    if ((!(l.f1317 > 50.0)) && (!(l.f1317 < t28))) {
        let t29: f64 = (l.f1317).exp();
        (t29, (t29 * l.f1318),)
    } else {
        let t2a: f64 = (-50.0);
        let (t33, t34,) = {
            if ((!(l.f1317 > 50.0)) && (l.f1317 < t2a)) {
                let t2b: f64 = (-50.0);let t2c: f64 = (t2b).exp();
                (t2c, 0.0,)
            } else {
                let (t31, t32,) = {
                    if (l.f1317 > 50.0) {
                        let t2d: f64 = (50.0_f64).exp();let t2e: f64 = (l.f1317 - 50.0);let t2f: f64 = (1.0 + t2e);let t30: f64 = (t2d * t2f);
                        (t30, (t2d * l.f1318),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t31, t32,)
            }
        };
        (t33, t34,)
    }
};
            (l.f1354, l.f1355, ) = (t35, t36, );
        }
        if (l.f1fd4 != 0.0) {let t37: f64 = (-l.f135a);let t38: f64 = (t37 - l.f1359);let t39: f64 = (l.f1349 * t38);let t3a: f64 = (t39 + l.f1317);(l.f12eb, l.f12ed, l.f12ee, l.f12ec, ) = (t3a, l.f1318, (l.f1349 * (-l.f135c)), (l.f1349 * (-l.f135b)), );let t3b: f64 = (-l.f1349);let t3c: f64 = (t3b * l.f1359);let t3d: f64 = (t3c + l.f1317);(l.f12f1, l.f12f2, ) = (t3d, l.f1318, );}
        if (l.f1fd4 != 0.0) {
            let t3e: f64 = (-50.0);
            let (t4f, t51, t52, t50,) = {
    if ((!(l.f12eb > 50.0)) && (!(l.f12eb < t3e))) {
        let t3f: f64 = (l.f12eb).exp();
        (t3f, (t3f * l.f12ed), (t3f * l.f12ee), (t3f * l.f12ec),)
    } else {
        let t40: f64 = (-50.0);
        let (t4b, t4d, t4e, t4c,) = {
            if ((!(l.f12eb > 50.0)) && (l.f12eb < t40)) {
                let t41: f64 = (-50.0);let t42: f64 = (t41).exp();
                (t42, 0.0, 0.0, 0.0,)
            } else {
                let (t47, t49, t4a, t48,) = {
                    if (l.f12eb > 50.0) {
                        let t43: f64 = (50.0_f64).exp();let t44: f64 = (l.f12eb - 50.0);let t45: f64 = (1.0 + t44);let t46: f64 = (t43 * t45);
                        (t46, (t43 * l.f12ed), (t43 * l.f12ee), (t43 * l.f12ec),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t47, t49, t4a, t48,)
            }
        };
        (t4b, t4d, t4e, t4c,)
    }
};
            (l.f12e3, l.f12e5, l.f12e6, l.f12e4, ) = (t4f, t51, t52, t50, );
        }
        if (l.f1fd4 != 0.0) {
            let t53: f64 = (-50.0);
            let (t60, t61,) = {
    if ((!(l.f12f1 > 50.0)) && (!(l.f12f1 < t53))) {
        let t54: f64 = (l.f12f1).exp();
        (t54, (t54 * l.f12f2),)
    } else {
        let t55: f64 = (-50.0);
        let (t5e, t5f,) = {
            if ((!(l.f12f1 > 50.0)) && (l.f12f1 < t55)) {
                let t56: f64 = (-50.0);let t57: f64 = (t56).exp();
                (t57, 0.0,)
            } else {
                let (t5c, t5d,) = {
                    if (l.f12f1 > 50.0) {
                        let t58: f64 = (50.0_f64).exp();let t59: f64 = (l.f12f1 - 50.0);let t5a: f64 = (1.0 + t59);let t5b: f64 = (t58 * t5a);
                        (t5b, (t58 * l.f12f2),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t5c, t5d,)
            }
        };
        (t5e, t5f,)
    }
};
            (l.f12e9, l.f12ea, ) = (t60, t61, );
        }
        if (l.f1fd4 != 0.0) {let t62: f64 = (l.f12e3 - l.f12e9);(l.f1321, l.f1323, l.f1324, l.f1322, ) = (t62, (l.f12e5 - l.f12ea), l.f12e6, l.f12e4, );let t63: f64 = (l.f1358 * l.f1360);let t64: f64 = (t63 * l.f1348);let t65: f64 = (t64 * l.f1341);let t66: f64 = (t65 * l.f1356);(l.f1343, l.f1344, ) = (t66, (t65 * l.f1357), );let t67: f64 = (l.f134b / l.f134e);let t68: f64 = (t67 * l.f135a);let t69: f64 = (t68 + l.f1317);(l.f1303, l.f1305, l.f1306, l.f1304, ) = (t69, (((-((l.f134b * l.f134f) / (l.f134e * l.f134e))) * l.f135a) + l.f1318), (t67 * l.f135c), (t67 * l.f135b), );}
        if (l.f1fd4 != 0.0) {
            let t6a: f64 = (-50.0);
            let (t7b, t7d, t7e, t7c,) = {
    if ((!(l.f1303 > 50.0)) && (!(l.f1303 < t6a))) {
        let t6b: f64 = (l.f1303).exp();
        (t6b, (t6b * l.f1305), (t6b * l.f1306), (t6b * l.f1304),)
    } else {
        let t6c: f64 = (-50.0);
        let (t77, t79, t7a, t78,) = {
            if ((!(l.f1303 > 50.0)) && (l.f1303 < t6c)) {
                let t6d: f64 = (-50.0);let t6e: f64 = (t6d).exp();
                (t6e, 0.0, 0.0, 0.0,)
            } else {
                let (t73, t75, t76, t74,) = {
                    if (l.f1303 > 50.0) {
                        let t6f: f64 = (50.0_f64).exp();let t70: f64 = (l.f1303 - 50.0);let t71: f64 = (1.0 + t70);let t72: f64 = (t6f * t71);
                        (t72, (t6f * l.f1305), (t6f * l.f1306), (t6f * l.f1304),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t73, t75, t76, t74,)
            }
        };
        (t77, t79, t7a, t78,)
    }
};
            (l.f12f7, l.f12f9, l.f12fa, l.f12f8, ) = (t7b, t7d, t7e, t7c, );
        }
        let t7f: f64 = if l.f131d == 1.0 { 1.0 } else { 0.0 };l.f1fdb = t7f;
        if ((l.f1fd4 != 0.0) && (l.f1fdb != 0.0)) {let t80: f64 = (l.f1347 * l.f1321);let t81: f64 = (l.f12f7 - t80);let t82: f64 = (t81 - l.f1354);let t83: f64 = (l.f1343 * t82);(l.f1327, l.f1329, l.f132a, l.f1328, ) = (t83, ((l.f1344 * t82) + (l.f1343 * ((l.f12f9 - (l.f1347 * l.f1323)) - l.f1355))), (l.f1343 * (l.f12fa - (l.f1347 * l.f1324))), (l.f1343 * (l.f12f8 - (l.f1347 * l.f1322))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {let t84: f64 = (-l.f135d);let t85: f64 = (t84 - l.f1359);let t86: f64 = (l.f1349 * t85);let t87: f64 = (t86 + l.f1317);(l.f12ef, l.f12f0, ) = (t87, l.f1318, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {
            let t88: f64 = (-50.0);
            let (t95, t96,) = {
    if ((!(l.f12ef > 50.0)) && (!(l.f12ef < t88))) {
        let t89: f64 = (l.f12ef).exp();
        (t89, (t89 * l.f12f0),)
    } else {
        let t8a: f64 = (-50.0);
        let (t93, t94,) = {
            if ((!(l.f12ef > 50.0)) && (l.f12ef < t8a)) {
                let t8b: f64 = (-50.0);let t8c: f64 = (t8b).exp();
                (t8c, 0.0,)
            } else {
                let (t91, t92,) = {
                    if (l.f12ef > 50.0) {
                        let t8d: f64 = (50.0_f64).exp();let t8e: f64 = (l.f12ef - 50.0);let t8f: f64 = (1.0 + t8e);let t90: f64 = (t8d * t8f);
                        (t90, (t8d * l.f12f0),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t91, t92,)
            }
        };
        (t93, t94,)
    }
};
            (l.f12e7, l.f12e8, ) = (t95, t96, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {let t97: f64 = (l.f12e7 - l.f12e9);(l.f1325, l.f1326, ) = (t97, (l.f12e8 - l.f12ea), );let t98: f64 = (l.f134b / l.f134e);let t99: f64 = (t98 * l.f135d);let t9a: f64 = (t99 + l.f1317);(l.f130d, l.f130e, ) = (t9a, (((-((l.f134b * l.f134f) / (l.f134e * l.f134e))) * l.f135d) + l.f1318), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {
            let t9b: f64 = (-50.0);
            let (ta8, ta9,) = {
    if ((!(l.f130d > 50.0)) && (!(l.f130d < t9b))) {
        let t9c: f64 = (l.f130d).exp();
        (t9c, (t9c * l.f130e),)
    } else {
        let t9d: f64 = (-50.0);
        let (ta6, ta7,) = {
            if ((!(l.f130d > 50.0)) && (l.f130d < t9d)) {
                let t9e: f64 = (-50.0);let t9f: f64 = (t9e).exp();
                (t9f, 0.0,)
            } else {
                let (ta4, ta5,) = {
                    if (l.f130d > 50.0) {
                        let ta0: f64 = (50.0_f64).exp();let ta1: f64 = (l.f130d - 50.0);let ta2: f64 = (1.0 + ta1);let ta3: f64 = (ta0 * ta2);
                        (ta3, (ta0 * l.f130e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (ta4, ta5,)
            }
        };
        (ta6, ta7,)
    }
};
            (l.f1301, l.f1302, ) = (ta8, ta9, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {let taa: f64 = (l.f1347 * l.f1325);let tab: f64 = (l.f1301 - taa);let tac: f64 = (tab - l.f1354);(l.f1337, l.f1338, ) = (tac, ((l.f1302 - (l.f1347 * l.f1326)) - l.f1355), );let tad: f64 = (l.f1347 * l.f1321);let tae: f64 = (l.f12f7 - tad);let taf: f64 = (tae - l.f1354);let tb0: f64 = (l.f1343 * taf);(l.f1333, l.f1335, l.f1336, l.f1334, ) = (tb0, ((l.f1344 * taf) + (l.f1343 * ((l.f12f9 - (l.f1347 * l.f1323)) - l.f1355))), (l.f1343 * (l.f12fa - (l.f1347 * l.f1324))), (l.f1343 * (l.f12f8 - (l.f1347 * l.f1322))), );}
        let tb1: f64 = if l.f131d > 0.0 { 1.0 } else { 0.0 };l.f1fdc = tb1;
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdc != 0.0)) {let tb2: f64 = (l.f131d * l.f134b);l.f134c = tb2;let tb3: f64 = (l.f134c / l.f134e);let tb4: f64 = (tb3 * l.f135d);let tb5: f64 = (tb4 + l.f1317);(l.f130b, l.f130c, ) = (tb5, (((-((l.f134c * l.f134f) / (l.f134e * l.f134e))) * l.f135d) + l.f1318), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdc != 0.0)) {
            let tb6: f64 = (-50.0);
            let (tc3, tc4,) = {
    if ((!(l.f130b > 50.0)) && (!(l.f130b < tb6))) {
        let tb7: f64 = (l.f130b).exp();
        (tb7, (tb7 * l.f130c),)
    } else {
        let tb8: f64 = (-50.0);
        let (tc1, tc2,) = {
            if ((!(l.f130b > 50.0)) && (l.f130b < tb8)) {
                let tb9: f64 = (-50.0);let tba: f64 = (tb9).exp();
                (tba, 0.0,)
            } else {
                let (tbf, tc0,) = {
                    if (l.f130b > 50.0) {
                        let tbb: f64 = (50.0_f64).exp();let tbc: f64 = (l.f130b - 50.0);let tbd: f64 = (1.0 + tbc);let tbe: f64 = (tbb * tbd);
                        (tbe, (tbb * l.f130c),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (tbf, tc0,)
            }
        };
        (tc1, tc2,)
    }
};
            (l.f12ff, l.f1300, ) = (tc3, tc4, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdc != 0.0)) {let tc5: f64 = (l.f1347 * l.f1325);let tc6: f64 = (l.f12ff - tc5);let tc7: f64 = (tc6 - l.f1354);(l.f1331, l.f1332, ) = (tc7, ((l.f1300 - (l.f1347 * l.f1326)) - l.f1355), );let tc8: f64 = (l.f134c / l.f134e);let tc9: f64 = (tc8 * l.f135a);let tca: f64 = (tc9 + l.f1317);(l.f1307, l.f1309, l.f130a, l.f1308, ) = (tca, (((-((l.f134c * l.f134f) / (l.f134e * l.f134e))) * l.f135a) + l.f1318), (tc8 * l.f135c), (tc8 * l.f135b), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdc != 0.0)) {
            let tcb: f64 = (-50.0);
            let (tdc, tde, tdf, tdd,) = {
    if ((!(l.f1307 > 50.0)) && (!(l.f1307 < tcb))) {
        let tcc: f64 = (l.f1307).exp();
        (tcc, (tcc * l.f1309), (tcc * l.f130a), (tcc * l.f1308),)
    } else {
        let tcd: f64 = (-50.0);
        let (td8, tda, tdb, td9,) = {
            if ((!(l.f1307 > 50.0)) && (l.f1307 < tcd)) {
                let tce: f64 = (-50.0);let tcf: f64 = (tce).exp();
                (tcf, 0.0, 0.0, 0.0,)
            } else {
                let (td4, td6, td7, td5,) = {
                    if (l.f1307 > 50.0) {
                        let td0: f64 = (50.0_f64).exp();let td1: f64 = (l.f1307 - 50.0);let td2: f64 = (1.0 + td1);let td3: f64 = (td0 * td2);
                        (td3, (td0 * l.f1309), (td0 * l.f130a), (td0 * l.f1308),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (td4, td6, td7, td5,)
            }
        };
        (td8, tda, tdb, td9,)
    }
};
            (l.f12fb, l.f12fd, l.f12fe, l.f12fc, ) = (tdc, tde, tdf, tdd, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdc != 0.0)) {let te0: f64 = (l.f1343 * l.f1337);let te1: f64 = (te0 / l.f1331);(l.f132f, l.f1330, ) = (te1, (((((l.f1344 * l.f1337) + (l.f1343 * l.f1338)) * l.f1331) - (te0 * l.f1332)) / (l.f1331 * l.f1331)), );let te2: f64 = (l.f1347 * l.f1321);let te3: f64 = (l.f12fb - te2);let te4: f64 = (te3 - l.f1354);let te5: f64 = (l.f132f * te4);(l.f132b, l.f132d, l.f132e, l.f132c, ) = (te5, ((l.f1330 * te4) + (l.f132f * ((l.f12fd - (l.f1347 * l.f1323)) - l.f1355))), (l.f132f * (l.f12fe - (l.f1347 * l.f1324))), (l.f132f * (l.f12fc - (l.f1347 * l.f1322))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdc == 0.0)) {let te6: f64 = (l.f1343 * l.f1337);(l.f132b, l.f132d, l.f132e, l.f132c, ) = (te6, ((l.f1344 * l.f1337) + (l.f1343 * l.f1338)), 0.0, 0.0, );}
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {let te7: f64 = (l.f12e1 * l.f12e1);let te8: f64 = (te7 * l.f134e);(l.f12df, l.f12e0, ) = (te8, (te7 * l.f134f), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv13 = ctx.node_voltage(nodes[13]);
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {let te9: f64 = (l.f12df / 2.0);let tea: f64 = (l.f135d - te9);let teb: f64 = (l.f135a - tea);let tec: f64 = (teb / l.f12df);(l.f12f3, l.f12f5, l.f12f6, l.f12f4, ) = (tec, ((((-(-(l.f12e0 / 2.0))) * l.f12df) - (teb * l.f12e0)) / (l.f12df * l.f12df)), (l.f135c / l.f12df), (l.f135b / l.f12df), );}
        let ted: f64 = if l.f12f3 > 50.0 { 1.0 } else { 0.0 };l.f1fdd = ted;
        if (((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdd != 0.0)) {(l.f1319, l.f131b, l.f131c, l.f131a, ) = (0.0, 0.0, 0.0, 0.0, );}
        let tee: f64 = (-50.0);let tef: f64 = if l.f12f3 < tee { 1.0 } else { 0.0 };l.f1fde = tef;
        if ((((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdd == 0.0)) && (l.f1fde != 0.0)) {(l.f1319, l.f131b, l.f131c, l.f131a, ) = (1.0, 0.0, 0.0, 0.0, );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) && (l.f1fdd == 0.0)) && (l.f1fde == 0.0)) {let tf0: f64 = (l.f12f3).exp();let tf1: f64 = (1.0 + tf0);let tf2: f64 = (1.0 / tf1);(l.f1319, l.f131b, l.f131c, l.f131a, ) = (tf2, (-((tf0 * l.f12f5) / (tf1 * tf1))), (-((tf0 * l.f12f6) / (tf1 * tf1))), (-((tf0 * l.f12f4) / (tf1 * tf1))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdb == 0.0)) {let tf3: f64 = (l.f1319 * l.f1333);let tf4: f64 = (1.0 - l.f1319);let tf5: f64 = (tf4 * l.f132b);let tf6: f64 = (tf3 + tf5);(l.f1327, l.f1329, l.f132a, l.f1328, ) = (tf6, (((l.f131b * l.f1333) + (l.f1319 * l.f1335)) + (((-l.f131b) * l.f132b) + (tf4 * l.f132d))), (((l.f131c * l.f1333) + (l.f1319 * l.f1336)) + (((-l.f131c) * l.f132b) + (tf4 * l.f132e))), (((l.f131a * l.f1333) + (l.f1319 * l.f1334)) + (((-l.f131a) * l.f132b) + (tf4 * l.f132c))), );}
        if (l.f1fd4 != 0.0) {
            let tf7: f64 = (-l.f135a);
            let (t106, t108, t107,) = {
    if (p.p52 != 0.0) {
        let tf8: f64 = (l.f135a / l.f135e);let tf9: f64 = (0.001 / p.p53);let tfa: f64 = (l.f135a / l.f135e);let tfb: f64 = (tf9 * tfa);let tfc: f64 = (tfb).tanh();let tfd: f64 = (tf8 * tfc);
        (tfd, (((l.f135c / l.f135e) * tfc) + (tf8 * ((tf9 * (l.f135c / l.f135e)) / ((tfb).cosh() * (tfb).cosh())))), (((l.f135b / l.f135e) * tfc) + (tf8 * ((tf9 * (l.f135b / l.f135e)) / ((tfb).cosh() * (tfb).cosh())))),)
    } else {
        let (t103, t105, t104,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f135e;let tfe: f64 = (l.f135a * __rspice_inv_cse_0);let tff: f64 = (l.f135a * __rspice_inv_cse_0);let t100: f64 = (tfe * tff);let t101: f64 = (t100 + p.p53);let t102: f64 = (t101).sqrt();
                (t102, ((((l.f135c / l.f135e) * tff) + (tfe * (l.f135c / l.f135e))) / (2.0 * t102)), ((((l.f135b / l.f135e) * tff) + (tfe * (l.f135b / l.f135e))) / (2.0 * t102)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t103, t105, t104,)
    }
};
            let t109: f64 = (t106).powf(l.f12e2);let t10a: f64 = (1.0 + t109);let t10b: f64 = (1.0 / l.f12e2);let t10c: f64 = (t10a).powf(t10b);let t10d: f64 = (tf7 / t10c);(l.f131e, l.f1320, l.f131f, ) = (t10d, ((((-l.f135c) * t10c) - (tf7 * if 0.0 == 0.0 && ((t10b) as f64).is_finite() && ((t10b) as f64).fract() == 0.0 { if t10b == 0.0 { 0.0 } else { (t10b * ((t10a).powf(t10b - 1.0) * if 0.0 == 0.0 && ((l.f12e2) as f64).is_finite() && ((l.f12e2) as f64).fract() == 0.0 { if l.f12e2 == 0.0 { 0.0 } else { (l.f12e2 * ((t106).powf(l.f12e2 - 1.0) * t108)) } } else { (t109 * (l.f12e2 * (t108 / t106))) })) } } else { (t10c * (t10b * (if 0.0 == 0.0 && ((l.f12e2) as f64).is_finite() && ((l.f12e2) as f64).fract() == 0.0 { if l.f12e2 == 0.0 { 0.0 } else { (l.f12e2 * ((t106).powf(l.f12e2 - 1.0) * t108)) } } else { (t109 * (l.f12e2 * (t108 / t106))) } / t10a))) })) / (t10c * t10c)), ((((-l.f135b) * t10c) - (tf7 * if 0.0 == 0.0 && ((t10b) as f64).is_finite() && ((t10b) as f64).fract() == 0.0 { if t10b == 0.0 { 0.0 } else { (t10b * ((t10a).powf(t10b - 1.0) * if 0.0 == 0.0 && ((l.f12e2) as f64).is_finite() && ((l.f12e2) as f64).fract() == 0.0 { if l.f12e2 == 0.0 { 0.0 } else { (l.f12e2 * ((t106).powf(l.f12e2 - 1.0) * t107)) } } else { (t109 * (l.f12e2 * (t107 / t106))) })) } } else { (t10c * (t10b * (if 0.0 == 0.0 && ((l.f12e2) as f64).is_finite() && ((l.f12e2) as f64).fract() == 0.0 { if l.f12e2 == 0.0 { 0.0 } else { (l.f12e2 * ((t106).powf(l.f12e2 - 1.0) * t107)) } } else { (t109 * (l.f12e2 * (t107 / t106))) } / t10a))) })) / (t10c * t10c)), );
        }
        if (l.f1fd4 != 0.0) {let t10e: f64 = (-l.f1358);let t10f: f64 = (t10e * l.f1360);let t110: f64 = (t10f * l.f1348);let t111: f64 = (t110 * l.f1342);let t112: f64 = (t111 * l.f1356);let t113: f64 = t112;(l.f1345, l.f1346, ) = (t113, (t111 * l.f1357), );let t114: f64 = (l.f134d / l.f134e);let t115: f64 = (t114 * l.f131e);(l.f1313, l.f1315, l.f1316, l.f1314, ) = (t115, ((-((l.f134d * l.f134f) / (l.f134e * l.f134e))) * l.f131e), (t114 * l.f1320), (t114 * l.f131f), );}
        if (l.f1fd4 != 0.0) {
            let t116: f64 = (-50.0);
            let (t127, t129, t12a, t128,) = {
    if ((!(l.f1313 > 50.0)) && (!(l.f1313 < t116))) {
        let t117: f64 = (l.f1313).exp();
        (t117, (t117 * l.f1315), (t117 * l.f1316), (t117 * l.f1314),)
    } else {
        let t118: f64 = (-50.0);
        let (t123, t125, t126, t124,) = {
            if ((!(l.f1313 > 50.0)) && (l.f1313 < t118)) {
                let t119: f64 = (-50.0);let t11a: f64 = (t119).exp();
                (t11a, 0.0, 0.0, 0.0,)
            } else {
                let (t11f, t121, t122, t120,) = {
                    if (l.f1313 > 50.0) {
                        let t11b: f64 = (50.0_f64).exp();let t11c: f64 = (l.f1313 - 50.0);let t11d: f64 = (1.0 + t11c);let t11e: f64 = (t11b * t11d);
                        (t11e, (t11b * l.f1315), (t11b * l.f1316), (t11b * l.f1314),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t11f, t121, t122, t120,)
            }
        };
        (t123, t125, t126, t124,)
    }
};
            (l.f130f, l.f1311, l.f1312, l.f1310, ) = (t127, t129, t12a, t128, );
        }
        if (l.f1fd4 != 0.0) {let t12b: f64 = (l.f130f - 1.0);let t12c: f64 = (l.f1345 * t12b);(l.f1339, l.f133b, l.f133c, l.f133a, ) = (t12c, ((l.f1346 * t12b) + (l.f1345 * l.f1311)), (l.f1345 * l.f1312), (l.f1345 * l.f1310), );let t12d: f64 = (l.f1327 + l.f1339);(l.f133d, l.f133f, l.f1340, l.f133e, ) = (t12d, (l.f1329 + l.f133b), (l.f132a + l.f133c), (l.f1328 + l.f133a), );(l.f1350, l.f1352, l.f1353, l.f1351, ) = (l.f133d, l.f133f, l.f1340, l.f133e, );(l.f211c, l.f2126, l.f2127, l.f2125, ) = (l.f1350, l.f1352, l.f1353, l.f1351, );}
        let t12e: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };l.f1fdf = t12e;
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {(l.f13d2, l.f13d4, l.f13d5, l.f13d3, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13c5, l.f13c6, ) = (0.0, 0.0, );(l.f13c7, l.f13c8, ) = (0.0, 0.0, );let t12f: f64 = (p.p6 * (nv8 - nv13));(l.f13dc, l.f13de, l.f13dd, ) = (t12f, p.p6, (-p.p6), );(l.f13d0, l.f13d1, ) = (l.f215b, l.f215c, );l.f13df = p.p260;l.f1363 = p.p262;l.f139f = 1.0;l.f13cd = p.p258;l.f13cb = p.p278;l.f13db = p.p277;(l.f13d8, l.f13d9, ) = (l.f22f2, l.f22f3, );l.f13e2 = p.p0;l.f13ca = p.p2;l.f13c3 = 0.0;l.f13c9 = 0.0;l.f13e0 = p.p285;l.f1364 = p.p286;let t130: f64 = (1.0 - p.p255);let t131: f64 = (t130 * p.p284);l.f13c4 = t131;l.f13cf = p.p283;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {l.f13cc = p.p257;l.f13e1 = p.p256;l.f13da = p.p6;(l.f13bf, l.f13c1, l.f13c2, l.f13c0, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1361, l.f1362, ) = (0.0, 0.0, );(l.f13d6, l.f13d7, ) = (0.0, 0.0, );(l.f139b, l.f139d, l.f139e, l.f139c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13a3, l.f13a5, l.f13a6, l.f13a4, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13a9, l.f13ab, l.f13ac, l.f13aa, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13a0, l.f13a2, l.f13a1, ) = (0.0, 0.0, 0.0, );(l.f13bb, l.f13bd, l.f13be, l.f13bc, ) = (0.0, 0.0, 0.0, 0.0, );(l.f136d, l.f136f, l.f1370, l.f136e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1373, l.f1374, ) = (0.0, 0.0, );(l.f1365, l.f1367, l.f1368, l.f1366, ) = (0.0, 0.0, 0.0, 0.0, );(l.f136b, l.f136c, ) = (0.0, 0.0, );(l.f1399, l.f139a, ) = (0.0, 0.0, );(l.f1375, l.f1377, l.f1378, l.f1376, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1385, l.f1387, l.f1388, l.f1386, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1379, l.f137b, l.f137c, l.f137a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1395, l.f1397, l.f1398, l.f1396, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1391, l.f1393, l.f1394, l.f1392, ) = (0.0, 0.0, 0.0, 0.0, );l.f13ce = 0.0;(l.f1371, l.f1372, ) = (0.0, 0.0, );(l.f1369, l.f136a, ) = (0.0, 0.0, );(l.f13a7, l.f13a8, ) = (0.0, 0.0, );(l.f138f, l.f1390, ) = (0.0, 0.0, );(l.f1383, l.f1384, ) = (0.0, 0.0, );(l.f13b9, l.f13ba, ) = (0.0, 0.0, );(l.f13b5, l.f13b7, l.f13b8, l.f13b6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f138d, l.f138e, ) = (0.0, 0.0, );(l.f1381, l.f1382, ) = (0.0, 0.0, );(l.f13b3, l.f13b4, ) = (0.0, 0.0, );(l.f1389, l.f138b, l.f138c, l.f138a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f137d, l.f137f, l.f1380, l.f137e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13b1, l.f13b2, ) = (0.0, 0.0, );(l.f13ad, l.f13af, l.f13b0, l.f13ae, ) = (0.0, 0.0, 0.0, 0.0, );let t132: f64 = (l.f13cc / l.f13d0);let t133: f64 = (-l.f13e1);let t134: f64 = (t132 * t133);(l.f1399, l.f139a, ) = (t134, ((-((l.f13cc * l.f13d1) / (l.f13d0 * l.f13d0))) * t133), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t135: f64 = (-50.0);
            let (t142, t143,) = {
    if ((!(l.f1399 > 50.0)) && (!(l.f1399 < t135))) {
        let t136: f64 = (l.f1399).exp();
        (t136, (t136 * l.f139a),)
    } else {
        let t137: f64 = (-50.0);
        let (t140, t141,) = {
            if ((!(l.f1399 > 50.0)) && (l.f1399 < t137)) {
                let t138: f64 = (-50.0);let t139: f64 = (t138).exp();
                (t139, 0.0,)
            } else {
                let (t13e, t13f,) = {
                    if (l.f1399 > 50.0) {
                        let t13a: f64 = (50.0_f64).exp();let t13b: f64 = (l.f1399 - 50.0);let t13c: f64 = (1.0 + t13b);let t13d: f64 = (t13a * t13c);
                        (t13d, (t13a * l.f139a),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t13e, t13f,)
            }
        };
        (t140, t141,)
    }
};
            (l.f13d6, l.f13d7, ) = (t142, t143, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t144: f64 = (-l.f13dc);let t145: f64 = (t144 - l.f13db);let t146: f64 = (l.f13cb * t145);let t147: f64 = (t146 + l.f1399);(l.f136d, l.f136f, l.f1370, l.f136e, ) = (t147, l.f139a, (l.f13cb * (-l.f13de)), (l.f13cb * (-l.f13dd)), );let t148: f64 = (-l.f13cb);let t149: f64 = (t148 * l.f13db);let t14a: f64 = (t149 + l.f1399);(l.f1373, l.f1374, ) = (t14a, l.f139a, );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t14b: f64 = (-50.0);
            let (t15c, t15e, t15f, t15d,) = {
    if ((!(l.f136d > 50.0)) && (!(l.f136d < t14b))) {
        let t14c: f64 = (l.f136d).exp();
        (t14c, (t14c * l.f136f), (t14c * l.f1370), (t14c * l.f136e),)
    } else {
        let t14d: f64 = (-50.0);
        let (t158, t15a, t15b, t159,) = {
            if ((!(l.f136d > 50.0)) && (l.f136d < t14d)) {
                let t14e: f64 = (-50.0);let t14f: f64 = (t14e).exp();
                (t14f, 0.0, 0.0, 0.0,)
            } else {
                let (t154, t156, t157, t155,) = {
                    if (l.f136d > 50.0) {
                        let t150: f64 = (50.0_f64).exp();let t151: f64 = (l.f136d - 50.0);let t152: f64 = (1.0 + t151);let t153: f64 = (t150 * t152);
                        (t153, (t150 * l.f136f), (t150 * l.f1370), (t150 * l.f136e),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t154, t156, t157, t155,)
            }
        };
        (t158, t15a, t15b, t159,)
    }
};
            (l.f1365, l.f1367, l.f1368, l.f1366, ) = (t15c, t15e, t15f, t15d, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t160: f64 = (-50.0);
            let (t16d, t16e,) = {
    if ((!(l.f1373 > 50.0)) && (!(l.f1373 < t160))) {
        let t161: f64 = (l.f1373).exp();
        (t161, (t161 * l.f1374),)
    } else {
        let t162: f64 = (-50.0);
        let (t16b, t16c,) = {
            if ((!(l.f1373 > 50.0)) && (l.f1373 < t162)) {
                let t163: f64 = (-50.0);let t164: f64 = (t163).exp();
                (t164, 0.0,)
            } else {
                let (t169, t16a,) = {
                    if (l.f1373 > 50.0) {
                        let t165: f64 = (50.0_f64).exp();let t166: f64 = (l.f1373 - 50.0);let t167: f64 = (1.0 + t166);let t168: f64 = (t165 * t167);
                        (t168, (t165 * l.f1374),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t169, t16a,)
            }
        };
        (t16b, t16c,)
    }
};
            (l.f136b, l.f136c, ) = (t16d, t16e, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t16f: f64 = (l.f1365 - l.f136b);(l.f13a3, l.f13a5, l.f13a6, l.f13a4, ) = (t16f, (l.f1367 - l.f136c), l.f1368, l.f1366, );let t170: f64 = (l.f13da * l.f13e2);let t171: f64 = (t170 * l.f13ca);let t172: f64 = (t171 * l.f13c3);let t173: f64 = (t172 * l.f13d8);(l.f13c5, l.f13c6, ) = (t173, (t172 * l.f13d9), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t174: f64 = (l.f13cd / l.f13d0);let t175: f64 = (t174 * l.f13dc);let t176: f64 = (t175 + l.f1399);(l.f1385, l.f1387, l.f1388, l.f1386, ) = (t176, (((-((l.f13cd * l.f13d1) / (l.f13d0 * l.f13d0))) * l.f13dc) + l.f139a), (t174 * l.f13de), (t174 * l.f13dd), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t177: f64 = (-50.0);
            let (t188, t18a, t18b, t189,) = {
    if ((!(l.f1385 > 50.0)) && (!(l.f1385 < t177))) {
        let t178: f64 = (l.f1385).exp();
        (t178, (t178 * l.f1387), (t178 * l.f1388), (t178 * l.f1386),)
    } else {
        let t179: f64 = (-50.0);
        let (t184, t186, t187, t185,) = {
            if ((!(l.f1385 > 50.0)) && (l.f1385 < t179)) {
                let t17a: f64 = (-50.0);let t17b: f64 = (t17a).exp();
                (t17b, 0.0, 0.0, 0.0,)
            } else {
                let (t180, t182, t183, t181,) = {
                    if (l.f1385 > 50.0) {
                        let t17c: f64 = (50.0_f64).exp();let t17d: f64 = (l.f1385 - 50.0);let t17e: f64 = (1.0 + t17d);let t17f: f64 = (t17c * t17e);
                        (t17f, (t17c * l.f1387), (t17c * l.f1388), (t17c * l.f1386),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t180, t182, t183, t181,)
            }
        };
        (t184, t186, t187, t185,)
    }
};
            (l.f1379, l.f137b, l.f137c, l.f137a, ) = (t188, t18a, t18b, t189, );
        }
        let t18c: f64 = if l.f139f == 1.0 { 1.0 } else { 0.0 };l.f1fe2 = t18c;
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 != 0.0)) {let t18d: f64 = (l.f13c9 * l.f13a3);let t18e: f64 = (l.f1379 - t18d);let t18f: f64 = (t18e - l.f13d6);let t190: f64 = (l.f13c5 * t18f);(l.f13a9, l.f13ab, l.f13ac, l.f13aa, ) = (t190, ((l.f13c6 * t18f) + (l.f13c5 * ((l.f137b - (l.f13c9 * l.f13a5)) - l.f13d7))), (l.f13c5 * (l.f137c - (l.f13c9 * l.f13a6))), (l.f13c5 * (l.f137a - (l.f13c9 * l.f13a4))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {let t191: f64 = (-l.f13df);let t192: f64 = (t191 - l.f13db);let t193: f64 = (l.f13cb * t192);let t194: f64 = (t193 + l.f1399);(l.f1371, l.f1372, ) = (t194, l.f139a, );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {
            let t195: f64 = (-50.0);
            let (t1a2, t1a3,) = {
    if ((!(l.f1371 > 50.0)) && (!(l.f1371 < t195))) {
        let t196: f64 = (l.f1371).exp();
        (t196, (t196 * l.f1372),)
    } else {
        let t197: f64 = (-50.0);
        let (t1a0, t1a1,) = {
            if ((!(l.f1371 > 50.0)) && (l.f1371 < t197)) {
                let t198: f64 = (-50.0);let t199: f64 = (t198).exp();
                (t199, 0.0,)
            } else {
                let (t19e, t19f,) = {
                    if (l.f1371 > 50.0) {
                        let t19a: f64 = (50.0_f64).exp();let t19b: f64 = (l.f1371 - 50.0);let t19c: f64 = (1.0 + t19b);let t19d: f64 = (t19a * t19c);
                        (t19d, (t19a * l.f1372),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t19e, t19f,)
            }
        };
        (t1a0, t1a1,)
    }
};
            (l.f1369, l.f136a, ) = (t1a2, t1a3, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {let t1a4: f64 = (l.f1369 - l.f136b);(l.f13a7, l.f13a8, ) = (t1a4, (l.f136a - l.f136c), );let t1a5: f64 = (l.f13cd / l.f13d0);let t1a6: f64 = (t1a5 * l.f13df);let t1a7: f64 = (t1a6 + l.f1399);(l.f138f, l.f1390, ) = (t1a7, (((-((l.f13cd * l.f13d1) / (l.f13d0 * l.f13d0))) * l.f13df) + l.f139a), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {
            let t1a8: f64 = (-50.0);
            let (t1b5, t1b6,) = {
    if ((!(l.f138f > 50.0)) && (!(l.f138f < t1a8))) {
        let t1a9: f64 = (l.f138f).exp();
        (t1a9, (t1a9 * l.f1390),)
    } else {
        let t1aa: f64 = (-50.0);
        let (t1b3, t1b4,) = {
            if ((!(l.f138f > 50.0)) && (l.f138f < t1aa)) {
                let t1ab: f64 = (-50.0);let t1ac: f64 = (t1ab).exp();
                (t1ac, 0.0,)
            } else {
                let (t1b1, t1b2,) = {
                    if (l.f138f > 50.0) {
                        let t1ad: f64 = (50.0_f64).exp();let t1ae: f64 = (l.f138f - 50.0);let t1af: f64 = (1.0 + t1ae);let t1b0: f64 = (t1ad * t1af);
                        (t1b0, (t1ad * l.f1390),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t1b1, t1b2,)
            }
        };
        (t1b3, t1b4,)
    }
};
            (l.f1383, l.f1384, ) = (t1b5, t1b6, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {let t1b7: f64 = (l.f13c9 * l.f13a7);let t1b8: f64 = (l.f1383 - t1b7);let t1b9: f64 = (t1b8 - l.f13d6);(l.f13b9, l.f13ba, ) = (t1b9, ((l.f1384 - (l.f13c9 * l.f13a8)) - l.f13d7), );let t1ba: f64 = (l.f13c9 * l.f13a3);let t1bb: f64 = (l.f1379 - t1ba);let t1bc: f64 = (t1bb - l.f13d6);let t1bd: f64 = (l.f13c5 * t1bc);(l.f13b5, l.f13b7, l.f13b8, l.f13b6, ) = (t1bd, ((l.f13c6 * t1bc) + (l.f13c5 * ((l.f137b - (l.f13c9 * l.f13a5)) - l.f13d7))), (l.f13c5 * (l.f137c - (l.f13c9 * l.f13a6))), (l.f13c5 * (l.f137a - (l.f13c9 * l.f13a4))), );}
        let t1be: f64 = if l.f139f > 0.0 { 1.0 } else { 0.0 };l.f1fe3 = t1be;
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 != 0.0)) {let t1bf: f64 = (l.f139f * l.f13cd);l.f13ce = t1bf;let t1c0: f64 = (l.f13ce / l.f13d0);let t1c1: f64 = (t1c0 * l.f13df);let t1c2: f64 = (t1c1 + l.f1399);(l.f138d, l.f138e, ) = (t1c2, (((-((l.f13ce * l.f13d1) / (l.f13d0 * l.f13d0))) * l.f13df) + l.f139a), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 != 0.0)) {
            let t1c3: f64 = (-50.0);
            let (t1d0, t1d1,) = {
    if ((!(l.f138d > 50.0)) && (!(l.f138d < t1c3))) {
        let t1c4: f64 = (l.f138d).exp();
        (t1c4, (t1c4 * l.f138e),)
    } else {
        let t1c5: f64 = (-50.0);
        let (t1ce, t1cf,) = {
            if ((!(l.f138d > 50.0)) && (l.f138d < t1c5)) {
                let t1c6: f64 = (-50.0);let t1c7: f64 = (t1c6).exp();
                (t1c7, 0.0,)
            } else {
                let (t1cc, t1cd,) = {
                    if (l.f138d > 50.0) {
                        let t1c8: f64 = (50.0_f64).exp();let t1c9: f64 = (l.f138d - 50.0);let t1ca: f64 = (1.0 + t1c9);let t1cb: f64 = (t1c8 * t1ca);
                        (t1cb, (t1c8 * l.f138e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t1cc, t1cd,)
            }
        };
        (t1ce, t1cf,)
    }
};
            (l.f1381, l.f1382, ) = (t1d0, t1d1, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 != 0.0)) {let t1d2: f64 = (l.f13c9 * l.f13a7);let t1d3: f64 = (l.f1381 - t1d2);let t1d4: f64 = (t1d3 - l.f13d6);(l.f13b3, l.f13b4, ) = (t1d4, ((l.f1382 - (l.f13c9 * l.f13a8)) - l.f13d7), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 != 0.0)) {let t1d5: f64 = (l.f13ce / l.f13d0);let t1d6: f64 = (t1d5 * l.f13dc);let t1d7: f64 = (t1d6 + l.f1399);(l.f1389, l.f138b, l.f138c, l.f138a, ) = (t1d7, (((-((l.f13ce * l.f13d1) / (l.f13d0 * l.f13d0))) * l.f13dc) + l.f139a), (t1d5 * l.f13de), (t1d5 * l.f13dd), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 != 0.0)) {
            let t1d8: f64 = (-50.0);
            let (t1e9, t1eb, t1ec, t1ea,) = {
    if ((!(l.f1389 > 50.0)) && (!(l.f1389 < t1d8))) {
        let t1d9: f64 = (l.f1389).exp();
        (t1d9, (t1d9 * l.f138b), (t1d9 * l.f138c), (t1d9 * l.f138a),)
    } else {
        let t1da: f64 = (-50.0);
        let (t1e5, t1e7, t1e8, t1e6,) = {
            if ((!(l.f1389 > 50.0)) && (l.f1389 < t1da)) {
                let t1db: f64 = (-50.0);let t1dc: f64 = (t1db).exp();
                (t1dc, 0.0, 0.0, 0.0,)
            } else {
                let (t1e1, t1e3, t1e4, t1e2,) = {
                    if (l.f1389 > 50.0) {
                        let t1dd: f64 = (50.0_f64).exp();let t1de: f64 = (l.f1389 - 50.0);let t1df: f64 = (1.0 + t1de);let t1e0: f64 = (t1dd * t1df);
                        (t1e0, (t1dd * l.f138b), (t1dd * l.f138c), (t1dd * l.f138a),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t1e1, t1e3, t1e4, t1e2,)
            }
        };
        (t1e5, t1e7, t1e8, t1e6,)
    }
};
            (l.f137d, l.f137f, l.f1380, l.f137e, ) = (t1e9, t1eb, t1ec, t1ea, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 != 0.0)) {let t1ed: f64 = (l.f13c5 * l.f13b9);let t1ee: f64 = (t1ed / l.f13b3);(l.f13b1, l.f13b2, ) = (t1ee, (((((l.f13c6 * l.f13b9) + (l.f13c5 * l.f13ba)) * l.f13b3) - (t1ed * l.f13b4)) / (l.f13b3 * l.f13b3)), );let t1ef: f64 = (l.f13c9 * l.f13a3);let t1f0: f64 = (l.f137d - t1ef);let t1f1: f64 = (t1f0 - l.f13d6);let t1f2: f64 = (l.f13b1 * t1f1);(l.f13ad, l.f13af, l.f13b0, l.f13ae, ) = (t1f2, ((l.f13b2 * t1f1) + (l.f13b1 * ((l.f137f - (l.f13c9 * l.f13a5)) - l.f13d7))), (l.f13b1 * (l.f1380 - (l.f13c9 * l.f13a6))), (l.f13b1 * (l.f137e - (l.f13c9 * l.f13a4))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe3 == 0.0)) {let t1f3: f64 = (l.f13c5 * l.f13b9);(l.f13ad, l.f13af, l.f13b0, l.f13ae, ) = (t1f3, ((l.f13c6 * l.f13b9) + (l.f13c5 * l.f13ba)), 0.0, 0.0, );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {let t1f4: f64 = (l.f1363 * l.f1363);let t1f5: f64 = (t1f4 * l.f13d0);(l.f1361, l.f1362, ) = (t1f5, (t1f4 * l.f13d1), );let t1f6: f64 = (l.f1361 / 2.0);let t1f7: f64 = (l.f13df - t1f6);let t1f8: f64 = (l.f13dc - t1f7);let t1f9: f64 = (t1f8 / l.f1361);(l.f1375, l.f1377, l.f1378, l.f1376, ) = (t1f9, ((((-(-(l.f1362 / 2.0))) * l.f1361) - (t1f8 * l.f1362)) / (l.f1361 * l.f1361)), (l.f13de / l.f1361), (l.f13dd / l.f1361), );}
        let t1fa: f64 = if l.f1375 > 50.0 { 1.0 } else { 0.0 };l.f1fe4 = t1fa;
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe4 != 0.0)) {(l.f139b, l.f139d, l.f139e, l.f139c, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t1fb: f64 = (-50.0);let t1fc: f64 = if l.f1375 < t1fb { 1.0 } else { 0.0 };l.f1fe5 = t1fc;
        if (((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe4 == 0.0)) && (l.f1fe5 != 0.0)) {(l.f139b, l.f139d, l.f139e, l.f139c, ) = (1.0, 0.0, 0.0, 0.0, );}
        if (((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) && (l.f1fe4 == 0.0)) && (l.f1fe5 == 0.0)) {let t1fd: f64 = (l.f1375).exp();let t1fe: f64 = (1.0 + t1fd);let t1ff: f64 = (1.0 / t1fe);(l.f139b, l.f139d, l.f139e, l.f139c, ) = (t1ff, (-((t1fd * l.f1377) / (t1fe * t1fe))), (-((t1fd * l.f1378) / (t1fe * t1fe))), (-((t1fd * l.f1376) / (t1fe * t1fe))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe2 == 0.0)) {let t200: f64 = (l.f139b * l.f13b5);let t201: f64 = (1.0 - l.f139b);let t202: f64 = (t201 * l.f13ad);let t203: f64 = (t200 + t202);(l.f13a9, l.f13ab, l.f13ac, l.f13aa, ) = (t203, (((l.f139d * l.f13b5) + (l.f139b * l.f13b7)) + (((-l.f139d) * l.f13ad) + (t201 * l.f13af))), (((l.f139e * l.f13b5) + (l.f139b * l.f13b8)) + (((-l.f139e) * l.f13ad) + (t201 * l.f13b0))), (((l.f139c * l.f13b5) + (l.f139b * l.f13b6)) + (((-l.f139c) * l.f13ad) + (t201 * l.f13ae))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t204: f64 = (-l.f13dc);
            let (t213, t215, t214,) = {
    if (p.p52 != 0.0) {
        let t205: f64 = (l.f13dc / l.f13e0);let t206: f64 = (0.001 / p.p53);let t207: f64 = (l.f13dc / l.f13e0);let t208: f64 = (t206 * t207);let t209: f64 = (t208).tanh();let t20a: f64 = (t205 * t209);
        (t20a, (((l.f13de / l.f13e0) * t209) + (t205 * ((t206 * (l.f13de / l.f13e0)) / ((t208).cosh() * (t208).cosh())))), (((l.f13dd / l.f13e0) * t209) + (t205 * ((t206 * (l.f13dd / l.f13e0)) / ((t208).cosh() * (t208).cosh())))),)
    } else {
        let (t210, t212, t211,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f13e0;let t20b: f64 = (l.f13dc * __rspice_inv_cse_0);let t20c: f64 = (l.f13dc * __rspice_inv_cse_0);let t20d: f64 = (t20b * t20c);let t20e: f64 = (t20d + p.p53);let t20f: f64 = (t20e).sqrt();
                (t20f, ((((l.f13de / l.f13e0) * t20c) + (t20b * (l.f13de / l.f13e0))) / (2.0 * t20f)), ((((l.f13dd / l.f13e0) * t20c) + (t20b * (l.f13dd / l.f13e0))) / (2.0 * t20f)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t210, t212, t211,)
    }
};
            let t216: f64 = (t213).powf(l.f1364);let t217: f64 = (1.0 + t216);let t218: f64 = (1.0 / l.f1364);let t219: f64 = (t217).powf(t218);let t21a: f64 = (t204 / t219);(l.f13a0, l.f13a2, l.f13a1, ) = (t21a, ((((-l.f13de) * t219) - (t204 * if 0.0 == 0.0 && ((t218) as f64).is_finite() && ((t218) as f64).fract() == 0.0 { if t218 == 0.0 { 0.0 } else { (t218 * ((t217).powf(t218 - 1.0) * if 0.0 == 0.0 && ((l.f1364) as f64).is_finite() && ((l.f1364) as f64).fract() == 0.0 { if l.f1364 == 0.0 { 0.0 } else { (l.f1364 * ((t213).powf(l.f1364 - 1.0) * t215)) } } else { (t216 * (l.f1364 * (t215 / t213))) })) } } else { (t219 * (t218 * (if 0.0 == 0.0 && ((l.f1364) as f64).is_finite() && ((l.f1364) as f64).fract() == 0.0 { if l.f1364 == 0.0 { 0.0 } else { (l.f1364 * ((t213).powf(l.f1364 - 1.0) * t215)) } } else { (t216 * (l.f1364 * (t215 / t213))) } / t217))) })) / (t219 * t219)), ((((-l.f13dd) * t219) - (t204 * if 0.0 == 0.0 && ((t218) as f64).is_finite() && ((t218) as f64).fract() == 0.0 { if t218 == 0.0 { 0.0 } else { (t218 * ((t217).powf(t218 - 1.0) * if 0.0 == 0.0 && ((l.f1364) as f64).is_finite() && ((l.f1364) as f64).fract() == 0.0 { if l.f1364 == 0.0 { 0.0 } else { (l.f1364 * ((t213).powf(l.f1364 - 1.0) * t214)) } } else { (t216 * (l.f1364 * (t214 / t213))) })) } } else { (t219 * (t218 * (if 0.0 == 0.0 && ((l.f1364) as f64).is_finite() && ((l.f1364) as f64).fract() == 0.0 { if l.f1364 == 0.0 { 0.0 } else { (l.f1364 * ((t213).powf(l.f1364 - 1.0) * t214)) } } else { (t216 * (l.f1364 * (t214 / t213))) } / t217))) })) / (t219 * t219)), );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t21b: f64 = (-l.f13da);let t21c: f64 = (t21b * l.f13e2);let t21d: f64 = (t21c * l.f13ca);let t21e: f64 = (t21d * l.f13c4);let t21f: f64 = (t21e * l.f13d8);let t220: f64 = t21f;(l.f13c7, l.f13c8, ) = (t220, (t21e * l.f13d9), );let t221: f64 = (l.f13cf / l.f13d0);let t222: f64 = (t221 * l.f13a0);(l.f1395, l.f1397, l.f1398, l.f1396, ) = (t222, ((-((l.f13cf * l.f13d1) / (l.f13d0 * l.f13d0))) * l.f13a0), (t221 * l.f13a2), (t221 * l.f13a1), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t223: f64 = (-50.0);
            let (t234, t236, t237, t235,) = {
    if ((!(l.f1395 > 50.0)) && (!(l.f1395 < t223))) {
        let t224: f64 = (l.f1395).exp();
        (t224, (t224 * l.f1397), (t224 * l.f1398), (t224 * l.f1396),)
    } else {
        let t225: f64 = (-50.0);
        let (t230, t232, t233, t231,) = {
            if ((!(l.f1395 > 50.0)) && (l.f1395 < t225)) {
                let t226: f64 = (-50.0);let t227: f64 = (t226).exp();
                (t227, 0.0, 0.0, 0.0,)
            } else {
                let (t22c, t22e, t22f, t22d,) = {
                    if (l.f1395 > 50.0) {
                        let t228: f64 = (50.0_f64).exp();let t229: f64 = (l.f1395 - 50.0);let t22a: f64 = (1.0 + t229);let t22b: f64 = (t228 * t22a);
                        (t22b, (t228 * l.f1397), (t228 * l.f1398), (t228 * l.f1396),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t22c, t22e, t22f, t22d,)
            }
        };
        (t230, t232, t233, t231,)
    }
};
            (l.f1391, l.f1393, l.f1394, l.f1392, ) = (t234, t236, t237, t235, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv17 = ctx.node_voltage(nodes[17]);
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t238: f64 = (l.f1391 - 1.0);let t239: f64 = (l.f13c7 * t238);(l.f13bb, l.f13bd, l.f13be, l.f13bc, ) = (t239, ((l.f13c8 * t238) + (l.f13c7 * l.f1393)), (l.f13c7 * l.f1394), (l.f13c7 * l.f1392), );let t23a: f64 = (l.f13a9 + l.f13bb);(l.f13bf, l.f13c1, l.f13c2, l.f13c0, ) = (t23a, (l.f13ab + l.f13bd), (l.f13ac + l.f13be), (l.f13aa + l.f13bc), );(l.f13d2, l.f13d4, l.f13d5, l.f13d3, ) = (l.f13bf, l.f13c1, l.f13c2, l.f13c0, );(l.f2134, l.f2136, l.f2137, l.f2135, ) = (l.f13d2, l.f13d4, l.f13d5, l.f13d3, );(l.f1454, l.f1456, l.f1457, l.f1455, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1447, l.f1448, ) = (0.0, 0.0, );(l.f1449, l.f144a, ) = (0.0, 0.0, );let t23b: f64 = (p.p6 * (nv8 - nv17));(l.f145e, l.f1460, l.f145f, ) = (t23b, p.p6, (-p.p6), );(l.f1452, l.f1453, ) = (l.f215b, l.f215c, );l.f1461 = p.p265;l.f13e5 = p.p267;l.f1421 = 1.0;l.f144f = p.p263;l.f144d = p.p281;l.f145d = p.p280;(l.f145a, l.f145b, ) = (l.f22f2, l.f22f3, );l.f1464 = p.p0;l.f144c = p.p2;l.f1445 = 0.0;l.f144b = 0.0;l.f1462 = p.p289;l.f13e6 = p.p290;let t23c: f64 = (1.0 - p.p255);let t23d: f64 = (t23c * p.p288);l.f1446 = t23d;l.f1451 = p.p287;l.f144e = p.p257;l.f1463 = p.p256;l.f145c = p.p6;(l.f1441, l.f1443, l.f1444, l.f1442, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13e3, l.f13e4, ) = (0.0, 0.0, );(l.f1458, l.f1459, ) = (0.0, 0.0, );(l.f141d, l.f141f, l.f1420, l.f141e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1425, l.f1427, l.f1428, l.f1426, ) = (0.0, 0.0, 0.0, 0.0, );(l.f142b, l.f142d, l.f142e, l.f142c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1422, l.f1424, l.f1423, ) = (0.0, 0.0, 0.0, );(l.f143d, l.f143f, l.f1440, l.f143e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13ef, l.f13f1, l.f13f2, l.f13f0, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13f5, l.f13f6, ) = (0.0, 0.0, );(l.f13e7, l.f13e9, l.f13ea, l.f13e8, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13ed, l.f13ee, ) = (0.0, 0.0, );(l.f141b, l.f141c, ) = (0.0, 0.0, );(l.f13f7, l.f13f9, l.f13fa, l.f13f8, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1407, l.f1409, l.f140a, l.f1408, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13fb, l.f13fd, l.f13fe, l.f13fc, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1417, l.f1419, l.f141a, l.f1418, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1413, l.f1415, l.f1416, l.f1414, ) = (0.0, 0.0, 0.0, 0.0, );l.f1450 = 0.0;(l.f13f3, l.f13f4, ) = (0.0, 0.0, );(l.f13eb, l.f13ec, ) = (0.0, 0.0, );(l.f1429, l.f142a, ) = (0.0, 0.0, );(l.f1411, l.f1412, ) = (0.0, 0.0, );(l.f1405, l.f1406, ) = (0.0, 0.0, );(l.f143b, l.f143c, ) = (0.0, 0.0, );(l.f1437, l.f1439, l.f143a, l.f1438, ) = (0.0, 0.0, 0.0, 0.0, );(l.f140f, l.f1410, ) = (0.0, 0.0, );(l.f1403, l.f1404, ) = (0.0, 0.0, );(l.f1435, l.f1436, ) = (0.0, 0.0, );(l.f140b, l.f140d, l.f140e, l.f140c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f13ff, l.f1401, l.f1402, l.f1400, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1433, l.f1434, ) = (0.0, 0.0, );(l.f142f, l.f1431, l.f1432, l.f1430, ) = (0.0, 0.0, 0.0, 0.0, );let t23e: f64 = (l.f144e / l.f1452);let t23f: f64 = (-l.f1463);let t240: f64 = (t23e * t23f);(l.f141b, l.f141c, ) = (t240, ((-((l.f144e * l.f1453) / (l.f1452 * l.f1452))) * t23f), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t241: f64 = (-50.0);
            let (t24e, t24f,) = {
    if ((!(l.f141b > 50.0)) && (!(l.f141b < t241))) {
        let t242: f64 = (l.f141b).exp();
        (t242, (t242 * l.f141c),)
    } else {
        let t243: f64 = (-50.0);
        let (t24c, t24d,) = {
            if ((!(l.f141b > 50.0)) && (l.f141b < t243)) {
                let t244: f64 = (-50.0);let t245: f64 = (t244).exp();
                (t245, 0.0,)
            } else {
                let (t24a, t24b,) = {
                    if (l.f141b > 50.0) {
                        let t246: f64 = (50.0_f64).exp();let t247: f64 = (l.f141b - 50.0);let t248: f64 = (1.0 + t247);let t249: f64 = (t246 * t248);
                        (t249, (t246 * l.f141c),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t24a, t24b,)
            }
        };
        (t24c, t24d,)
    }
};
            (l.f1458, l.f1459, ) = (t24e, t24f, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t250: f64 = (-l.f145e);let t251: f64 = (t250 - l.f145d);let t252: f64 = (l.f144d * t251);let t253: f64 = (t252 + l.f141b);(l.f13ef, l.f13f1, l.f13f2, l.f13f0, ) = (t253, l.f141c, (l.f144d * (-l.f1460)), (l.f144d * (-l.f145f)), );let t254: f64 = (-l.f144d);let t255: f64 = (t254 * l.f145d);let t256: f64 = (t255 + l.f141b);(l.f13f5, l.f13f6, ) = (t256, l.f141c, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t257: f64 = (-50.0);
            let (t268, t26a, t26b, t269,) = {
    if ((!(l.f13ef > 50.0)) && (!(l.f13ef < t257))) {
        let t258: f64 = (l.f13ef).exp();
        (t258, (t258 * l.f13f1), (t258 * l.f13f2), (t258 * l.f13f0),)
    } else {
        let t259: f64 = (-50.0);
        let (t264, t266, t267, t265,) = {
            if ((!(l.f13ef > 50.0)) && (l.f13ef < t259)) {
                let t25a: f64 = (-50.0);let t25b: f64 = (t25a).exp();
                (t25b, 0.0, 0.0, 0.0,)
            } else {
                let (t260, t262, t263, t261,) = {
                    if (l.f13ef > 50.0) {
                        let t25c: f64 = (50.0_f64).exp();let t25d: f64 = (l.f13ef - 50.0);let t25e: f64 = (1.0 + t25d);let t25f: f64 = (t25c * t25e);
                        (t25f, (t25c * l.f13f1), (t25c * l.f13f2), (t25c * l.f13f0),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t260, t262, t263, t261,)
            }
        };
        (t264, t266, t267, t265,)
    }
};
            (l.f13e7, l.f13e9, l.f13ea, l.f13e8, ) = (t268, t26a, t26b, t269, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t26c: f64 = (-50.0);
            let (t279, t27a,) = {
    if ((!(l.f13f5 > 50.0)) && (!(l.f13f5 < t26c))) {
        let t26d: f64 = (l.f13f5).exp();
        (t26d, (t26d * l.f13f6),)
    } else {
        let t26e: f64 = (-50.0);
        let (t277, t278,) = {
            if ((!(l.f13f5 > 50.0)) && (l.f13f5 < t26e)) {
                let t26f: f64 = (-50.0);let t270: f64 = (t26f).exp();
                (t270, 0.0,)
            } else {
                let (t275, t276,) = {
                    if (l.f13f5 > 50.0) {
                        let t271: f64 = (50.0_f64).exp();let t272: f64 = (l.f13f5 - 50.0);let t273: f64 = (1.0 + t272);let t274: f64 = (t271 * t273);
                        (t274, (t271 * l.f13f6),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t275, t276,)
            }
        };
        (t277, t278,)
    }
};
            (l.f13ed, l.f13ee, ) = (t279, t27a, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t27b: f64 = (l.f13e7 - l.f13ed);(l.f1425, l.f1427, l.f1428, l.f1426, ) = (t27b, (l.f13e9 - l.f13ee), l.f13ea, l.f13e8, );let t27c: f64 = (l.f145c * l.f1464);let t27d: f64 = (t27c * l.f144c);let t27e: f64 = (t27d * l.f1445);let t27f: f64 = (t27e * l.f145a);(l.f1447, l.f1448, ) = (t27f, (t27e * l.f145b), );let t280: f64 = (l.f144f / l.f1452);let t281: f64 = (t280 * l.f145e);let t282: f64 = (t281 + l.f141b);(l.f1407, l.f1409, l.f140a, l.f1408, ) = (t282, (((-((l.f144f * l.f1453) / (l.f1452 * l.f1452))) * l.f145e) + l.f141c), (t280 * l.f1460), (t280 * l.f145f), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t283: f64 = (-50.0);
            let (t294, t296, t297, t295,) = {
    if ((!(l.f1407 > 50.0)) && (!(l.f1407 < t283))) {
        let t284: f64 = (l.f1407).exp();
        (t284, (t284 * l.f1409), (t284 * l.f140a), (t284 * l.f1408),)
    } else {
        let t285: f64 = (-50.0);
        let (t290, t292, t293, t291,) = {
            if ((!(l.f1407 > 50.0)) && (l.f1407 < t285)) {
                let t286: f64 = (-50.0);let t287: f64 = (t286).exp();
                (t287, 0.0, 0.0, 0.0,)
            } else {
                let (t28c, t28e, t28f, t28d,) = {
                    if (l.f1407 > 50.0) {
                        let t288: f64 = (50.0_f64).exp();let t289: f64 = (l.f1407 - 50.0);let t28a: f64 = (1.0 + t289);let t28b: f64 = (t288 * t28a);
                        (t28b, (t288 * l.f1409), (t288 * l.f140a), (t288 * l.f1408),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t28c, t28e, t28f, t28d,)
            }
        };
        (t290, t292, t293, t291,)
    }
};
            (l.f13fb, l.f13fd, l.f13fe, l.f13fc, ) = (t294, t296, t297, t295, );
        }
        let t298: f64 = if l.f1421 == 1.0 { 1.0 } else { 0.0 };l.f1fe6 = t298;
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 != 0.0)) {let t299: f64 = (l.f144b * l.f1425);let t29a: f64 = (l.f13fb - t299);let t29b: f64 = (t29a - l.f1458);let t29c: f64 = (l.f1447 * t29b);(l.f142b, l.f142d, l.f142e, l.f142c, ) = (t29c, ((l.f1448 * t29b) + (l.f1447 * ((l.f13fd - (l.f144b * l.f1427)) - l.f1459))), (l.f1447 * (l.f13fe - (l.f144b * l.f1428))), (l.f1447 * (l.f13fc - (l.f144b * l.f1426))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {let t29d: f64 = (-l.f1461);let t29e: f64 = (t29d - l.f145d);let t29f: f64 = (l.f144d * t29e);let t2a0: f64 = (t29f + l.f141b);(l.f13f3, l.f13f4, ) = (t2a0, l.f141c, );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {
            let t2a1: f64 = (-50.0);
            let (t2ae, t2af,) = {
    if ((!(l.f13f3 > 50.0)) && (!(l.f13f3 < t2a1))) {
        let t2a2: f64 = (l.f13f3).exp();
        (t2a2, (t2a2 * l.f13f4),)
    } else {
        let t2a3: f64 = (-50.0);
        let (t2ac, t2ad,) = {
            if ((!(l.f13f3 > 50.0)) && (l.f13f3 < t2a3)) {
                let t2a4: f64 = (-50.0);let t2a5: f64 = (t2a4).exp();
                (t2a5, 0.0,)
            } else {
                let (t2aa, t2ab,) = {
                    if (l.f13f3 > 50.0) {
                        let t2a6: f64 = (50.0_f64).exp();let t2a7: f64 = (l.f13f3 - 50.0);let t2a8: f64 = (1.0 + t2a7);let t2a9: f64 = (t2a6 * t2a8);
                        (t2a9, (t2a6 * l.f13f4),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t2aa, t2ab,)
            }
        };
        (t2ac, t2ad,)
    }
};
            (l.f13eb, l.f13ec, ) = (t2ae, t2af, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {let t2b0: f64 = (l.f13eb - l.f13ed);(l.f1429, l.f142a, ) = (t2b0, (l.f13ec - l.f13ee), );let t2b1: f64 = (l.f144f / l.f1452);let t2b2: f64 = (t2b1 * l.f1461);let t2b3: f64 = (t2b2 + l.f141b);(l.f1411, l.f1412, ) = (t2b3, (((-((l.f144f * l.f1453) / (l.f1452 * l.f1452))) * l.f1461) + l.f141c), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {
            let t2b4: f64 = (-50.0);
            let (t2c1, t2c2,) = {
    if ((!(l.f1411 > 50.0)) && (!(l.f1411 < t2b4))) {
        let t2b5: f64 = (l.f1411).exp();
        (t2b5, (t2b5 * l.f1412),)
    } else {
        let t2b6: f64 = (-50.0);
        let (t2bf, t2c0,) = {
            if ((!(l.f1411 > 50.0)) && (l.f1411 < t2b6)) {
                let t2b7: f64 = (-50.0);let t2b8: f64 = (t2b7).exp();
                (t2b8, 0.0,)
            } else {
                let (t2bd, t2be,) = {
                    if (l.f1411 > 50.0) {
                        let t2b9: f64 = (50.0_f64).exp();let t2ba: f64 = (l.f1411 - 50.0);let t2bb: f64 = (1.0 + t2ba);let t2bc: f64 = (t2b9 * t2bb);
                        (t2bc, (t2b9 * l.f1412),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t2bd, t2be,)
            }
        };
        (t2bf, t2c0,)
    }
};
            (l.f1405, l.f1406, ) = (t2c1, t2c2, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        l: &mut StampLocals,
    ) {
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {let t2c3: f64 = (l.f144b * l.f1429);let t2c4: f64 = (l.f1405 - t2c3);let t2c5: f64 = (t2c4 - l.f1458);(l.f143b, l.f143c, ) = (t2c5, ((l.f1406 - (l.f144b * l.f142a)) - l.f1459), );let t2c6: f64 = (l.f144b * l.f1425);let t2c7: f64 = (l.f13fb - t2c6);let t2c8: f64 = (t2c7 - l.f1458);let t2c9: f64 = (l.f1447 * t2c8);(l.f1437, l.f1439, l.f143a, l.f1438, ) = (t2c9, ((l.f1448 * t2c8) + (l.f1447 * ((l.f13fd - (l.f144b * l.f1427)) - l.f1459))), (l.f1447 * (l.f13fe - (l.f144b * l.f1428))), (l.f1447 * (l.f13fc - (l.f144b * l.f1426))), );}
        let t2ca: f64 = if l.f1421 > 0.0 { 1.0 } else { 0.0 };l.f1fe7 = t2ca;
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe7 != 0.0)) {let t2cb: f64 = (l.f1421 * l.f144f);l.f1450 = t2cb;let t2cc: f64 = (l.f1450 / l.f1452);let t2cd: f64 = (t2cc * l.f1461);let t2ce: f64 = (t2cd + l.f141b);(l.f140f, l.f1410, ) = (t2ce, (((-((l.f1450 * l.f1453) / (l.f1452 * l.f1452))) * l.f1461) + l.f141c), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe7 != 0.0)) {
            let t2cf: f64 = (-50.0);
            let (t2dc, t2dd,) = {
    if ((!(l.f140f > 50.0)) && (!(l.f140f < t2cf))) {
        let t2d0: f64 = (l.f140f).exp();
        (t2d0, (t2d0 * l.f1410),)
    } else {
        let t2d1: f64 = (-50.0);
        let (t2da, t2db,) = {
            if ((!(l.f140f > 50.0)) && (l.f140f < t2d1)) {
                let t2d2: f64 = (-50.0);let t2d3: f64 = (t2d2).exp();
                (t2d3, 0.0,)
            } else {
                let (t2d8, t2d9,) = {
                    if (l.f140f > 50.0) {
                        let t2d4: f64 = (50.0_f64).exp();let t2d5: f64 = (l.f140f - 50.0);let t2d6: f64 = (1.0 + t2d5);let t2d7: f64 = (t2d4 * t2d6);
                        (t2d7, (t2d4 * l.f1410),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t2d8, t2d9,)
            }
        };
        (t2da, t2db,)
    }
};
            (l.f1403, l.f1404, ) = (t2dc, t2dd, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe7 != 0.0)) {let t2de: f64 = (l.f144b * l.f1429);let t2df: f64 = (l.f1403 - t2de);let t2e0: f64 = (t2df - l.f1458);(l.f1435, l.f1436, ) = (t2e0, ((l.f1404 - (l.f144b * l.f142a)) - l.f1459), );let t2e1: f64 = (l.f1450 / l.f1452);let t2e2: f64 = (t2e1 * l.f145e);let t2e3: f64 = (t2e2 + l.f141b);(l.f140b, l.f140d, l.f140e, l.f140c, ) = (t2e3, (((-((l.f1450 * l.f1453) / (l.f1452 * l.f1452))) * l.f145e) + l.f141c), (t2e1 * l.f1460), (t2e1 * l.f145f), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe7 != 0.0)) {
            let t2e4: f64 = (-50.0);
            let (t2f5, t2f7, t2f8, t2f6,) = {
    if ((!(l.f140b > 50.0)) && (!(l.f140b < t2e4))) {
        let t2e5: f64 = (l.f140b).exp();
        (t2e5, (t2e5 * l.f140d), (t2e5 * l.f140e), (t2e5 * l.f140c),)
    } else {
        let t2e6: f64 = (-50.0);
        let (t2f1, t2f3, t2f4, t2f2,) = {
            if ((!(l.f140b > 50.0)) && (l.f140b < t2e6)) {
                let t2e7: f64 = (-50.0);let t2e8: f64 = (t2e7).exp();
                (t2e8, 0.0, 0.0, 0.0,)
            } else {
                let (t2ed, t2ef, t2f0, t2ee,) = {
                    if (l.f140b > 50.0) {
                        let t2e9: f64 = (50.0_f64).exp();let t2ea: f64 = (l.f140b - 50.0);let t2eb: f64 = (1.0 + t2ea);let t2ec: f64 = (t2e9 * t2eb);
                        (t2ec, (t2e9 * l.f140d), (t2e9 * l.f140e), (t2e9 * l.f140c),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t2ed, t2ef, t2f0, t2ee,)
            }
        };
        (t2f1, t2f3, t2f4, t2f2,)
    }
};
            (l.f13ff, l.f1401, l.f1402, l.f1400, ) = (t2f5, t2f7, t2f8, t2f6, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe7 != 0.0)) {let t2f9: f64 = (l.f1447 * l.f143b);let t2fa: f64 = (t2f9 / l.f1435);(l.f1433, l.f1434, ) = (t2fa, (((((l.f1448 * l.f143b) + (l.f1447 * l.f143c)) * l.f1435) - (t2f9 * l.f1436)) / (l.f1435 * l.f1435)), );let t2fb: f64 = (l.f144b * l.f1425);let t2fc: f64 = (l.f13ff - t2fb);let t2fd: f64 = (t2fc - l.f1458);let t2fe: f64 = (l.f1433 * t2fd);(l.f142f, l.f1431, l.f1432, l.f1430, ) = (t2fe, ((l.f1434 * t2fd) + (l.f1433 * ((l.f1401 - (l.f144b * l.f1427)) - l.f1459))), (l.f1433 * (l.f1402 - (l.f144b * l.f1428))), (l.f1433 * (l.f1400 - (l.f144b * l.f1426))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe7 == 0.0)) {let t2ff: f64 = (l.f1447 * l.f143b);(l.f142f, l.f1431, l.f1432, l.f1430, ) = (t2ff, ((l.f1448 * l.f143b) + (l.f1447 * l.f143c)), 0.0, 0.0, );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {let t300: f64 = (l.f13e5 * l.f13e5);let t301: f64 = (t300 * l.f1452);(l.f13e3, l.f13e4, ) = (t301, (t300 * l.f1453), );let t302: f64 = (l.f13e3 / 2.0);let t303: f64 = (l.f1461 - t302);let t304: f64 = (l.f145e - t303);let t305: f64 = (t304 / l.f13e3);(l.f13f7, l.f13f9, l.f13fa, l.f13f8, ) = (t305, ((((-(-(l.f13e4 / 2.0))) * l.f13e3) - (t304 * l.f13e4)) / (l.f13e3 * l.f13e3)), (l.f1460 / l.f13e3), (l.f145f / l.f13e3), );}
        let t306: f64 = if l.f13f7 > 50.0 { 1.0 } else { 0.0 };l.f1fe8 = t306;
        if ((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe8 != 0.0)) {(l.f141d, l.f141f, l.f1420, l.f141e, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t307: f64 = (-50.0);let t308: f64 = if l.f13f7 < t307 { 1.0 } else { 0.0 };l.f1fe9 = t308;
        if (((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe8 == 0.0)) && (l.f1fe9 != 0.0)) {(l.f141d, l.f141f, l.f1420, l.f141e, ) = (1.0, 0.0, 0.0, 0.0, );}
        if (((((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) && (l.f1fe8 == 0.0)) && (l.f1fe9 == 0.0)) {let t309: f64 = (l.f13f7).exp();let t30a: f64 = (1.0 + t309);let t30b: f64 = (1.0 / t30a);(l.f141d, l.f141f, l.f1420, l.f141e, ) = (t30b, (-((t309 * l.f13f9) / (t30a * t30a))), (-((t309 * l.f13fa) / (t30a * t30a))), (-((t309 * l.f13f8) / (t30a * t30a))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) && (l.f1fe6 == 0.0)) {let t30c: f64 = (l.f141d * l.f1437);let t30d: f64 = (1.0 - l.f141d);let t30e: f64 = (t30d * l.f142f);let t30f: f64 = (t30c + t30e);(l.f142b, l.f142d, l.f142e, l.f142c, ) = (t30f, (((l.f141f * l.f1437) + (l.f141d * l.f1439)) + (((-l.f141f) * l.f142f) + (t30d * l.f1431))), (((l.f1420 * l.f1437) + (l.f141d * l.f143a)) + (((-l.f1420) * l.f142f) + (t30d * l.f1432))), (((l.f141e * l.f1437) + (l.f141d * l.f1438)) + (((-l.f141e) * l.f142f) + (t30d * l.f1430))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t310: f64 = (-l.f145e);
            let (t31f, t321, t320,) = {
    if (p.p52 != 0.0) {
        let t311: f64 = (l.f145e / l.f1462);let t312: f64 = (0.001 / p.p53);let t313: f64 = (l.f145e / l.f1462);let t314: f64 = (t312 * t313);let t315: f64 = (t314).tanh();let t316: f64 = (t311 * t315);
        (t316, (((l.f1460 / l.f1462) * t315) + (t311 * ((t312 * (l.f1460 / l.f1462)) / ((t314).cosh() * (t314).cosh())))), (((l.f145f / l.f1462) * t315) + (t311 * ((t312 * (l.f145f / l.f1462)) / ((t314).cosh() * (t314).cosh())))),)
    } else {
        let (t31c, t31e, t31d,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f1462;let t317: f64 = (l.f145e * __rspice_inv_cse_0);let t318: f64 = (l.f145e * __rspice_inv_cse_0);let t319: f64 = (t317 * t318);let t31a: f64 = (t319 + p.p53);let t31b: f64 = (t31a).sqrt();
                (t31b, ((((l.f1460 / l.f1462) * t318) + (t317 * (l.f1460 / l.f1462))) / (2.0 * t31b)), ((((l.f145f / l.f1462) * t318) + (t317 * (l.f145f / l.f1462))) / (2.0 * t31b)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t31c, t31e, t31d,)
    }
};
            let t322: f64 = (t31f).powf(l.f13e6);let t323: f64 = (1.0 + t322);let t324: f64 = (1.0 / l.f13e6);let t325: f64 = (t323).powf(t324);let t326: f64 = (t310 / t325);(l.f1422, l.f1424, l.f1423, ) = (t326, ((((-l.f1460) * t325) - (t310 * if 0.0 == 0.0 && ((t324) as f64).is_finite() && ((t324) as f64).fract() == 0.0 { if t324 == 0.0 { 0.0 } else { (t324 * ((t323).powf(t324 - 1.0) * if 0.0 == 0.0 && ((l.f13e6) as f64).is_finite() && ((l.f13e6) as f64).fract() == 0.0 { if l.f13e6 == 0.0 { 0.0 } else { (l.f13e6 * ((t31f).powf(l.f13e6 - 1.0) * t321)) } } else { (t322 * (l.f13e6 * (t321 / t31f))) })) } } else { (t325 * (t324 * (if 0.0 == 0.0 && ((l.f13e6) as f64).is_finite() && ((l.f13e6) as f64).fract() == 0.0 { if l.f13e6 == 0.0 { 0.0 } else { (l.f13e6 * ((t31f).powf(l.f13e6 - 1.0) * t321)) } } else { (t322 * (l.f13e6 * (t321 / t31f))) } / t323))) })) / (t325 * t325)), ((((-l.f145f) * t325) - (t310 * if 0.0 == 0.0 && ((t324) as f64).is_finite() && ((t324) as f64).fract() == 0.0 { if t324 == 0.0 { 0.0 } else { (t324 * ((t323).powf(t324 - 1.0) * if 0.0 == 0.0 && ((l.f13e6) as f64).is_finite() && ((l.f13e6) as f64).fract() == 0.0 { if l.f13e6 == 0.0 { 0.0 } else { (l.f13e6 * ((t31f).powf(l.f13e6 - 1.0) * t320)) } } else { (t322 * (l.f13e6 * (t320 / t31f))) })) } } else { (t325 * (t324 * (if 0.0 == 0.0 && ((l.f13e6) as f64).is_finite() && ((l.f13e6) as f64).fract() == 0.0 { if l.f13e6 == 0.0 { 0.0 } else { (l.f13e6 * ((t31f).powf(l.f13e6 - 1.0) * t320)) } } else { (t322 * (l.f13e6 * (t320 / t31f))) } / t323))) })) / (t325 * t325)), );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t327: f64 = (-l.f145c);let t328: f64 = (t327 * l.f1464);let t329: f64 = (t328 * l.f144c);let t32a: f64 = (t329 * l.f1446);let t32b: f64 = (t32a * l.f145a);let t32c: f64 = t32b;(l.f1449, l.f144a, ) = (t32c, (t32a * l.f145b), );let t32d: f64 = (l.f1451 / l.f1452);let t32e: f64 = (t32d * l.f1422);(l.f1417, l.f1419, l.f141a, l.f1418, ) = (t32e, ((-((l.f1451 * l.f1453) / (l.f1452 * l.f1452))) * l.f1422), (t32d * l.f1424), (t32d * l.f1423), );}
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {
            let t32f: f64 = (-50.0);
            let (t340, t342, t343, t341,) = {
    if ((!(l.f1417 > 50.0)) && (!(l.f1417 < t32f))) {
        let t330: f64 = (l.f1417).exp();
        (t330, (t330 * l.f1419), (t330 * l.f141a), (t330 * l.f1418),)
    } else {
        let t331: f64 = (-50.0);
        let (t33c, t33e, t33f, t33d,) = {
            if ((!(l.f1417 > 50.0)) && (l.f1417 < t331)) {
                let t332: f64 = (-50.0);let t333: f64 = (t332).exp();
                (t333, 0.0, 0.0, 0.0,)
            } else {
                let (t338, t33a, t33b, t339,) = {
                    if (l.f1417 > 50.0) {
                        let t334: f64 = (50.0_f64).exp();let t335: f64 = (l.f1417 - 50.0);let t336: f64 = (1.0 + t335);let t337: f64 = (t334 * t336);
                        (t337, (t334 * l.f1419), (t334 * l.f141a), (t334 * l.f1418),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t338, t33a, t33b, t339,)
            }
        };
        (t33c, t33e, t33f, t33d,)
    }
};
            (l.f1413, l.f1415, l.f1416, l.f1414, ) = (t340, t342, t343, t341, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fdf != 0.0)) {let t344: f64 = (l.f1413 - 1.0);let t345: f64 = (l.f1449 * t344);(l.f143d, l.f143f, l.f1440, l.f143e, ) = (t345, ((l.f144a * t344) + (l.f1449 * l.f1415)), (l.f1449 * l.f1416), (l.f1449 * l.f1414), );let t346: f64 = (l.f142b + l.f143d);(l.f1441, l.f1443, l.f1444, l.f1442, ) = (t346, (l.f142d + l.f143f), (l.f142e + l.f1440), (l.f142c + l.f143e), );(l.f1454, l.f1456, l.f1457, l.f1455, ) = (l.f1441, l.f1443, l.f1444, l.f1442, );(l.f211d, l.f211f, l.f2120, l.f211e, ) = (l.f1454, l.f1456, l.f1457, l.f1455, );}
        let t347: f64 = if p.p255 != 0.0 { 1.0 } else { 0.0 };l.f1fea = t347;
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {(l.f14d6, l.f14d7, l.f14d8, l.f14d9, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14c9, l.f14ca, ) = (0.0, 0.0, );(l.f14cb, l.f14cc, ) = (0.0, 0.0, );let t348: f64 = (p.p6 * (nv8 - nv9));(l.f14e0, l.f14e1, l.f14e2, ) = (t348, p.p6, (-p.p6), );(l.f14d4, l.f14d5, ) = (l.f215b, l.f215c, );l.f14e3 = p.p260;l.f1467 = p.p262;l.f14a3 = p.p261;l.f14d1 = p.p258;l.f14cf = p.p278;l.f14df = p.p277;(l.f14dc, l.f14dd, ) = (l.f22f2, l.f22f3, );l.f14e6 = p.p0;l.f14ce = p.p2;let t349: f64 = (p.p255 * p.p259);l.f14c7 = t349;l.f14cd = p.p276;l.f14e4 = p.p270;l.f1468 = p.p271;let t34a: f64 = (p.p255 * p.p269);l.f14c8 = t34a;l.f14d3 = p.p268;l.f14d0 = p.p257;l.f14e5 = p.p256;l.f14de = p.p6;(l.f14c3, l.f14c4, l.f14c5, l.f14c6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1465, l.f1466, ) = (0.0, 0.0, );(l.f14da, l.f14db, ) = (0.0, 0.0, );(l.f149f, l.f14a0, l.f14a1, l.f14a2, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14a7, l.f14a8, l.f14a9, l.f14aa, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14ad, l.f14ae, l.f14af, l.f14b0, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14a4, l.f14a5, l.f14a6, ) = (0.0, 0.0, 0.0, );(l.f14bf, l.f14c0, l.f14c1, l.f14c2, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1471, l.f1472, l.f1473, l.f1474, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1477, l.f1478, ) = (0.0, 0.0, );(l.f1469, l.f146a, l.f146b, l.f146c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f146f, l.f1470, ) = (0.0, 0.0, );(l.f149d, l.f149e, ) = (0.0, 0.0, );(l.f1479, l.f147a, l.f147b, l.f147c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1489, l.f148a, l.f148b, l.f148c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f147d, l.f147e, l.f147f, l.f1480, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1499, l.f149a, l.f149b, l.f149c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1495, l.f1496, l.f1497, l.f1498, ) = (0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {l.f14d2 = 0.0;(l.f1475, l.f1476, ) = (0.0, 0.0, );(l.f146d, l.f146e, ) = (0.0, 0.0, );(l.f14ab, l.f14ac, ) = (0.0, 0.0, );(l.f1493, l.f1494, ) = (0.0, 0.0, );(l.f1487, l.f1488, ) = (0.0, 0.0, );(l.f14bd, l.f14be, ) = (0.0, 0.0, );(l.f14b9, l.f14ba, l.f14bb, l.f14bc, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1491, l.f1492, ) = (0.0, 0.0, );(l.f1485, l.f1486, ) = (0.0, 0.0, );(l.f14b7, l.f14b8, ) = (0.0, 0.0, );(l.f148d, l.f148e, l.f148f, l.f1490, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1481, l.f1482, l.f1483, l.f1484, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14b5, l.f14b6, ) = (0.0, 0.0, );(l.f14b1, l.f14b2, l.f14b3, l.f14b4, ) = (0.0, 0.0, 0.0, 0.0, );let t34b: f64 = (l.f14d0 / l.f14d4);let t34c: f64 = (-l.f14e5);let t34d: f64 = (t34b * t34c);(l.f149d, l.f149e, ) = (t34d, ((-((l.f14d0 * l.f14d5) / (l.f14d4 * l.f14d4))) * t34c), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t34e: f64 = (-50.0);
            let (t35b, t35c,) = {
    if ((!(l.f149d > 50.0)) && (!(l.f149d < t34e))) {
        let t34f: f64 = (l.f149d).exp();
        (t34f, (t34f * l.f149e),)
    } else {
        let t350: f64 = (-50.0);
        let (t359, t35a,) = {
            if ((!(l.f149d > 50.0)) && (l.f149d < t350)) {
                let t351: f64 = (-50.0);let t352: f64 = (t351).exp();
                (t352, 0.0,)
            } else {
                let (t357, t358,) = {
                    if (l.f149d > 50.0) {
                        let t353: f64 = (50.0_f64).exp();let t354: f64 = (l.f149d - 50.0);let t355: f64 = (1.0 + t354);let t356: f64 = (t353 * t355);
                        (t356, (t353 * l.f149e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t357, t358,)
            }
        };
        (t359, t35a,)
    }
};
            (l.f14da, l.f14db, ) = (t35b, t35c, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t35d: f64 = (-l.f14e0);let t35e: f64 = (t35d - l.f14df);let t35f: f64 = (l.f14cf * t35e);let t360: f64 = (t35f + l.f149d);(l.f1471, l.f1472, l.f1473, l.f1474, ) = (t360, l.f149e, (l.f14cf * (-l.f14e1)), (l.f14cf * (-l.f14e2)), );let t361: f64 = (-l.f14cf);let t362: f64 = (t361 * l.f14df);let t363: f64 = (t362 + l.f149d);(l.f1477, l.f1478, ) = (t363, l.f149e, );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t364: f64 = (-50.0);
            let (t375, t376, t377, t378,) = {
    if ((!(l.f1471 > 50.0)) && (!(l.f1471 < t364))) {
        let t365: f64 = (l.f1471).exp();
        (t365, (t365 * l.f1472), (t365 * l.f1473), (t365 * l.f1474),)
    } else {
        let t366: f64 = (-50.0);
        let (t371, t372, t373, t374,) = {
            if ((!(l.f1471 > 50.0)) && (l.f1471 < t366)) {
                let t367: f64 = (-50.0);let t368: f64 = (t367).exp();
                (t368, 0.0, 0.0, 0.0,)
            } else {
                let (t36d, t36e, t36f, t370,) = {
                    if (l.f1471 > 50.0) {
                        let t369: f64 = (50.0_f64).exp();let t36a: f64 = (l.f1471 - 50.0);let t36b: f64 = (1.0 + t36a);let t36c: f64 = (t369 * t36b);
                        (t36c, (t369 * l.f1472), (t369 * l.f1473), (t369 * l.f1474),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t36d, t36e, t36f, t370,)
            }
        };
        (t371, t372, t373, t374,)
    }
};
            (l.f1469, l.f146a, l.f146b, l.f146c, ) = (t375, t376, t377, t378, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t379: f64 = (-50.0);
            let (t386, t387,) = {
    if ((!(l.f1477 > 50.0)) && (!(l.f1477 < t379))) {
        let t37a: f64 = (l.f1477).exp();
        (t37a, (t37a * l.f1478),)
    } else {
        let t37b: f64 = (-50.0);
        let (t384, t385,) = {
            if ((!(l.f1477 > 50.0)) && (l.f1477 < t37b)) {
                let t37c: f64 = (-50.0);let t37d: f64 = (t37c).exp();
                (t37d, 0.0,)
            } else {
                let (t382, t383,) = {
                    if (l.f1477 > 50.0) {
                        let t37e: f64 = (50.0_f64).exp();let t37f: f64 = (l.f1477 - 50.0);let t380: f64 = (1.0 + t37f);let t381: f64 = (t37e * t380);
                        (t381, (t37e * l.f1478),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t382, t383,)
            }
        };
        (t384, t385,)
    }
};
            (l.f146f, l.f1470, ) = (t386, t387, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t388: f64 = (l.f1469 - l.f146f);(l.f14a7, l.f14a8, l.f14a9, l.f14aa, ) = (t388, (l.f146a - l.f1470), l.f146b, l.f146c, );let t389: f64 = (l.f14de * l.f14e6);let t38a: f64 = (t389 * l.f14ce);let t38b: f64 = (t38a * l.f14c7);let t38c: f64 = (t38b * l.f14dc);(l.f14c9, l.f14ca, ) = (t38c, (t38b * l.f14dd), );let t38d: f64 = (l.f14d1 / l.f14d4);let t38e: f64 = (t38d * l.f14e0);let t38f: f64 = (t38e + l.f149d);(l.f1489, l.f148a, l.f148b, l.f148c, ) = (t38f, (((-((l.f14d1 * l.f14d5) / (l.f14d4 * l.f14d4))) * l.f14e0) + l.f149e), (t38d * l.f14e1), (t38d * l.f14e2), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t390: f64 = (-50.0);
            let (t3a1, t3a2, t3a3, t3a4,) = {
    if ((!(l.f1489 > 50.0)) && (!(l.f1489 < t390))) {
        let t391: f64 = (l.f1489).exp();
        (t391, (t391 * l.f148a), (t391 * l.f148b), (t391 * l.f148c),)
    } else {
        let t392: f64 = (-50.0);
        let (t39d, t39e, t39f, t3a0,) = {
            if ((!(l.f1489 > 50.0)) && (l.f1489 < t392)) {
                let t393: f64 = (-50.0);let t394: f64 = (t393).exp();
                (t394, 0.0, 0.0, 0.0,)
            } else {
                let (t399, t39a, t39b, t39c,) = {
                    if (l.f1489 > 50.0) {
                        let t395: f64 = (50.0_f64).exp();let t396: f64 = (l.f1489 - 50.0);let t397: f64 = (1.0 + t396);let t398: f64 = (t395 * t397);
                        (t398, (t395 * l.f148a), (t395 * l.f148b), (t395 * l.f148c),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t399, t39a, t39b, t39c,)
            }
        };
        (t39d, t39e, t39f, t3a0,)
    }
};
            (l.f147d, l.f147e, l.f147f, l.f1480, ) = (t3a1, t3a2, t3a3, t3a4, );
        }
        let t3a5: f64 = if l.f14a3 == 1.0 { 1.0 } else { 0.0 };l.f1fed = t3a5;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        l: &mut StampLocals,
    ) {
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed != 0.0)) {let t3a6: f64 = (l.f14cd * l.f14a7);let t3a7: f64 = (l.f147d - t3a6);let t3a8: f64 = (t3a7 - l.f14da);let t3a9: f64 = (l.f14c9 * t3a8);(l.f14ad, l.f14ae, l.f14af, l.f14b0, ) = (t3a9, ((l.f14ca * t3a8) + (l.f14c9 * ((l.f147e - (l.f14cd * l.f14a8)) - l.f14db))), (l.f14c9 * (l.f147f - (l.f14cd * l.f14a9))), (l.f14c9 * (l.f1480 - (l.f14cd * l.f14aa))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {let t3aa: f64 = (-l.f14e3);let t3ab: f64 = (t3aa - l.f14df);let t3ac: f64 = (l.f14cf * t3ab);let t3ad: f64 = (t3ac + l.f149d);(l.f1475, l.f1476, ) = (t3ad, l.f149e, );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {
            let t3ae: f64 = (-50.0);
            let (t3bb, t3bc,) = {
    if ((!(l.f1475 > 50.0)) && (!(l.f1475 < t3ae))) {
        let t3af: f64 = (l.f1475).exp();
        (t3af, (t3af * l.f1476),)
    } else {
        let t3b0: f64 = (-50.0);
        let (t3b9, t3ba,) = {
            if ((!(l.f1475 > 50.0)) && (l.f1475 < t3b0)) {
                let t3b1: f64 = (-50.0);let t3b2: f64 = (t3b1).exp();
                (t3b2, 0.0,)
            } else {
                let (t3b7, t3b8,) = {
                    if (l.f1475 > 50.0) {
                        let t3b3: f64 = (50.0_f64).exp();let t3b4: f64 = (l.f1475 - 50.0);let t3b5: f64 = (1.0 + t3b4);let t3b6: f64 = (t3b3 * t3b5);
                        (t3b6, (t3b3 * l.f1476),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3b7, t3b8,)
            }
        };
        (t3b9, t3ba,)
    }
};
            (l.f146d, l.f146e, ) = (t3bb, t3bc, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {let t3bd: f64 = (l.f146d - l.f146f);(l.f14ab, l.f14ac, ) = (t3bd, (l.f146e - l.f1470), );let t3be: f64 = (l.f14d1 / l.f14d4);let t3bf: f64 = (t3be * l.f14e3);let t3c0: f64 = (t3bf + l.f149d);(l.f1493, l.f1494, ) = (t3c0, (((-((l.f14d1 * l.f14d5) / (l.f14d4 * l.f14d4))) * l.f14e3) + l.f149e), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {
            let t3c1: f64 = (-50.0);
            let (t3ce, t3cf,) = {
    if ((!(l.f1493 > 50.0)) && (!(l.f1493 < t3c1))) {
        let t3c2: f64 = (l.f1493).exp();
        (t3c2, (t3c2 * l.f1494),)
    } else {
        let t3c3: f64 = (-50.0);
        let (t3cc, t3cd,) = {
            if ((!(l.f1493 > 50.0)) && (l.f1493 < t3c3)) {
                let t3c4: f64 = (-50.0);let t3c5: f64 = (t3c4).exp();
                (t3c5, 0.0,)
            } else {
                let (t3ca, t3cb,) = {
                    if (l.f1493 > 50.0) {
                        let t3c6: f64 = (50.0_f64).exp();let t3c7: f64 = (l.f1493 - 50.0);let t3c8: f64 = (1.0 + t3c7);let t3c9: f64 = (t3c6 * t3c8);
                        (t3c9, (t3c6 * l.f1494),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3ca, t3cb,)
            }
        };
        (t3cc, t3cd,)
    }
};
            (l.f1487, l.f1488, ) = (t3ce, t3cf, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {let t3d0: f64 = (l.f14cd * l.f14ab);let t3d1: f64 = (l.f1487 - t3d0);let t3d2: f64 = (t3d1 - l.f14da);(l.f14bd, l.f14be, ) = (t3d2, ((l.f1488 - (l.f14cd * l.f14ac)) - l.f14db), );let t3d3: f64 = (l.f14cd * l.f14a7);let t3d4: f64 = (l.f147d - t3d3);let t3d5: f64 = (t3d4 - l.f14da);let t3d6: f64 = (l.f14c9 * t3d5);(l.f14b9, l.f14ba, l.f14bb, l.f14bc, ) = (t3d6, ((l.f14ca * t3d5) + (l.f14c9 * ((l.f147e - (l.f14cd * l.f14a8)) - l.f14db))), (l.f14c9 * (l.f147f - (l.f14cd * l.f14a9))), (l.f14c9 * (l.f1480 - (l.f14cd * l.f14aa))), );}
        let t3d7: f64 = if l.f14a3 > 0.0 { 1.0 } else { 0.0 };l.f1fee = t3d7;
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee != 0.0)) {let t3d8: f64 = (l.f14a3 * l.f14d1);l.f14d2 = t3d8;let t3d9: f64 = (l.f14d2 / l.f14d4);let t3da: f64 = (t3d9 * l.f14e3);let t3db: f64 = (t3da + l.f149d);(l.f1491, l.f1492, ) = (t3db, (((-((l.f14d2 * l.f14d5) / (l.f14d4 * l.f14d4))) * l.f14e3) + l.f149e), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee != 0.0)) {
            let t3dc: f64 = (-50.0);
            let (t3e9, t3ea,) = {
    if ((!(l.f1491 > 50.0)) && (!(l.f1491 < t3dc))) {
        let t3dd: f64 = (l.f1491).exp();
        (t3dd, (t3dd * l.f1492),)
    } else {
        let t3de: f64 = (-50.0);
        let (t3e7, t3e8,) = {
            if ((!(l.f1491 > 50.0)) && (l.f1491 < t3de)) {
                let t3df: f64 = (-50.0);let t3e0: f64 = (t3df).exp();
                (t3e0, 0.0,)
            } else {
                let (t3e5, t3e6,) = {
                    if (l.f1491 > 50.0) {
                        let t3e1: f64 = (50.0_f64).exp();let t3e2: f64 = (l.f1491 - 50.0);let t3e3: f64 = (1.0 + t3e2);let t3e4: f64 = (t3e1 * t3e3);
                        (t3e4, (t3e1 * l.f1492),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3e5, t3e6,)
            }
        };
        (t3e7, t3e8,)
    }
};
            (l.f1485, l.f1486, ) = (t3e9, t3ea, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee != 0.0)) {let t3eb: f64 = (l.f14cd * l.f14ab);let t3ec: f64 = (l.f1485 - t3eb);let t3ed: f64 = (t3ec - l.f14da);(l.f14b7, l.f14b8, ) = (t3ed, ((l.f1486 - (l.f14cd * l.f14ac)) - l.f14db), );let t3ee: f64 = (l.f14d2 / l.f14d4);let t3ef: f64 = (t3ee * l.f14e0);let t3f0: f64 = (t3ef + l.f149d);(l.f148d, l.f148e, l.f148f, l.f1490, ) = (t3f0, (((-((l.f14d2 * l.f14d5) / (l.f14d4 * l.f14d4))) * l.f14e0) + l.f149e), (t3ee * l.f14e1), (t3ee * l.f14e2), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee != 0.0)) {
            let t3f1: f64 = (-50.0);
            let (t402, t403, t404, t405,) = {
    if ((!(l.f148d > 50.0)) && (!(l.f148d < t3f1))) {
        let t3f2: f64 = (l.f148d).exp();
        (t3f2, (t3f2 * l.f148e), (t3f2 * l.f148f), (t3f2 * l.f1490),)
    } else {
        let t3f3: f64 = (-50.0);
        let (t3fe, t3ff, t400, t401,) = {
            if ((!(l.f148d > 50.0)) && (l.f148d < t3f3)) {
                let t3f4: f64 = (-50.0);let t3f5: f64 = (t3f4).exp();
                (t3f5, 0.0, 0.0, 0.0,)
            } else {
                let (t3fa, t3fb, t3fc, t3fd,) = {
                    if (l.f148d > 50.0) {
                        let t3f6: f64 = (50.0_f64).exp();let t3f7: f64 = (l.f148d - 50.0);let t3f8: f64 = (1.0 + t3f7);let t3f9: f64 = (t3f6 * t3f8);
                        (t3f9, (t3f6 * l.f148e), (t3f6 * l.f148f), (t3f6 * l.f1490),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t3fa, t3fb, t3fc, t3fd,)
            }
        };
        (t3fe, t3ff, t400, t401,)
    }
};
            (l.f1481, l.f1482, l.f1483, l.f1484, ) = (t402, t403, t404, t405, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee != 0.0)) {let t406: f64 = (l.f14c9 * l.f14bd);let t407: f64 = (t406 / l.f14b7);(l.f14b5, l.f14b6, ) = (t407, (((((l.f14ca * l.f14bd) + (l.f14c9 * l.f14be)) * l.f14b7) - (t406 * l.f14b8)) / (l.f14b7 * l.f14b7)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv8 = ctx.node_voltage(nodes[8]);
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee != 0.0)) {let t408: f64 = (l.f14cd * l.f14a7);let t409: f64 = (l.f1481 - t408);let t40a: f64 = (t409 - l.f14da);let t40b: f64 = (l.f14b5 * t40a);(l.f14b1, l.f14b2, l.f14b3, l.f14b4, ) = (t40b, ((l.f14b6 * t40a) + (l.f14b5 * ((l.f1482 - (l.f14cd * l.f14a8)) - l.f14db))), (l.f14b5 * (l.f1483 - (l.f14cd * l.f14a9))), (l.f14b5 * (l.f1484 - (l.f14cd * l.f14aa))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fee == 0.0)) {let t40c: f64 = (l.f14c9 * l.f14bd);(l.f14b1, l.f14b2, l.f14b3, l.f14b4, ) = (t40c, ((l.f14ca * l.f14bd) + (l.f14c9 * l.f14be)), 0.0, 0.0, );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {let t40d: f64 = (l.f1467 * l.f1467);let t40e: f64 = (t40d * l.f14d4);(l.f1465, l.f1466, ) = (t40e, (t40d * l.f14d5), );let t40f: f64 = (l.f1465 / 2.0);let t410: f64 = (l.f14e3 - t40f);let t411: f64 = (l.f14e0 - t410);let t412: f64 = (t411 / l.f1465);(l.f1479, l.f147a, l.f147b, l.f147c, ) = (t412, ((((-(-(l.f1466 / 2.0))) * l.f1465) - (t411 * l.f1466)) / (l.f1465 * l.f1465)), (l.f14e1 / l.f1465), (l.f14e2 / l.f1465), );}
        let t413: f64 = if l.f1479 > 50.0 { 1.0 } else { 0.0 };l.f1fef = t413;
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fef != 0.0)) {(l.f149f, l.f14a0, l.f14a1, l.f14a2, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t414: f64 = (-50.0);let t415: f64 = if l.f1479 < t414 { 1.0 } else { 0.0 };l.f1ff0 = t415;
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fef == 0.0)) && (l.f1ff0 != 0.0)) {(l.f149f, l.f14a0, l.f14a1, l.f14a2, ) = (1.0, 0.0, 0.0, 0.0, );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) && (l.f1fef == 0.0)) && (l.f1ff0 == 0.0)) {let t416: f64 = (l.f1479).exp();let t417: f64 = (1.0 + t416);let t418: f64 = (1.0 / t417);(l.f149f, l.f14a0, l.f14a1, l.f14a2, ) = (t418, (-((t416 * l.f147a) / (t417 * t417))), (-((t416 * l.f147b) / (t417 * t417))), (-((t416 * l.f147c) / (t417 * t417))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1fed == 0.0)) {let t419: f64 = (l.f149f * l.f14b9);let t41a: f64 = (1.0 - l.f149f);let t41b: f64 = (t41a * l.f14b1);let t41c: f64 = (t419 + t41b);(l.f14ad, l.f14ae, l.f14af, l.f14b0, ) = (t41c, (((l.f14a0 * l.f14b9) + (l.f149f * l.f14ba)) + (((-l.f14a0) * l.f14b1) + (t41a * l.f14b2))), (((l.f14a1 * l.f14b9) + (l.f149f * l.f14bb)) + (((-l.f14a1) * l.f14b1) + (t41a * l.f14b3))), (((l.f14a2 * l.f14b9) + (l.f149f * l.f14bc)) + (((-l.f14a2) * l.f14b1) + (t41a * l.f14b4))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t41d: f64 = (-l.f14e0);
            let (t42c, t42d, t42e,) = {
    if (p.p52 != 0.0) {
        let t41e: f64 = (l.f14e0 / l.f14e4);let t41f: f64 = (0.001 / p.p53);let t420: f64 = (l.f14e0 / l.f14e4);let t421: f64 = (t41f * t420);let t422: f64 = (t421).tanh();let t423: f64 = (t41e * t422);
        (t423, (((l.f14e1 / l.f14e4) * t422) + (t41e * ((t41f * (l.f14e1 / l.f14e4)) / ((t421).cosh() * (t421).cosh())))), (((l.f14e2 / l.f14e4) * t422) + (t41e * ((t41f * (l.f14e2 / l.f14e4)) / ((t421).cosh() * (t421).cosh())))),)
    } else {
        let (t429, t42a, t42b,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f14e4;let t424: f64 = (l.f14e0 * __rspice_inv_cse_0);let t425: f64 = (l.f14e0 * __rspice_inv_cse_0);let t426: f64 = (t424 * t425);let t427: f64 = (t426 + p.p53);let t428: f64 = (t427).sqrt();
                (t428, ((((l.f14e1 / l.f14e4) * t425) + (t424 * (l.f14e1 / l.f14e4))) / (2.0 * t428)), ((((l.f14e2 / l.f14e4) * t425) + (t424 * (l.f14e2 / l.f14e4))) / (2.0 * t428)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t429, t42a, t42b,)
    }
};
            let t42f: f64 = (t42c).powf(l.f1468);let t430: f64 = (1.0 + t42f);let t431: f64 = (1.0 / l.f1468);let t432: f64 = (t430).powf(t431);let t433: f64 = (t41d / t432);(l.f14a4, l.f14a5, l.f14a6, ) = (t433, ((((-l.f14e1) * t432) - (t41d * if 0.0 == 0.0 && ((t431) as f64).is_finite() && ((t431) as f64).fract() == 0.0 { if t431 == 0.0 { 0.0 } else { (t431 * ((t430).powf(t431 - 1.0) * if 0.0 == 0.0 && ((l.f1468) as f64).is_finite() && ((l.f1468) as f64).fract() == 0.0 { if l.f1468 == 0.0 { 0.0 } else { (l.f1468 * ((t42c).powf(l.f1468 - 1.0) * t42d)) } } else { (t42f * (l.f1468 * (t42d / t42c))) })) } } else { (t432 * (t431 * (if 0.0 == 0.0 && ((l.f1468) as f64).is_finite() && ((l.f1468) as f64).fract() == 0.0 { if l.f1468 == 0.0 { 0.0 } else { (l.f1468 * ((t42c).powf(l.f1468 - 1.0) * t42d)) } } else { (t42f * (l.f1468 * (t42d / t42c))) } / t430))) })) / (t432 * t432)), ((((-l.f14e2) * t432) - (t41d * if 0.0 == 0.0 && ((t431) as f64).is_finite() && ((t431) as f64).fract() == 0.0 { if t431 == 0.0 { 0.0 } else { (t431 * ((t430).powf(t431 - 1.0) * if 0.0 == 0.0 && ((l.f1468) as f64).is_finite() && ((l.f1468) as f64).fract() == 0.0 { if l.f1468 == 0.0 { 0.0 } else { (l.f1468 * ((t42c).powf(l.f1468 - 1.0) * t42e)) } } else { (t42f * (l.f1468 * (t42e / t42c))) })) } } else { (t432 * (t431 * (if 0.0 == 0.0 && ((l.f1468) as f64).is_finite() && ((l.f1468) as f64).fract() == 0.0 { if l.f1468 == 0.0 { 0.0 } else { (l.f1468 * ((t42c).powf(l.f1468 - 1.0) * t42e)) } } else { (t42f * (l.f1468 * (t42e / t42c))) } / t430))) })) / (t432 * t432)), );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t434: f64 = (-l.f14de);let t435: f64 = (t434 * l.f14e6);let t436: f64 = (t435 * l.f14ce);let t437: f64 = (t436 * l.f14c8);let t438: f64 = (t437 * l.f14dc);let t439: f64 = t438;(l.f14cb, l.f14cc, ) = (t439, (t437 * l.f14dd), );let t43a: f64 = (l.f14d3 / l.f14d4);let t43b: f64 = (t43a * l.f14a4);(l.f1499, l.f149a, l.f149b, l.f149c, ) = (t43b, ((-((l.f14d3 * l.f14d5) / (l.f14d4 * l.f14d4))) * l.f14a4), (t43a * l.f14a5), (t43a * l.f14a6), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t43c: f64 = (-50.0);
            let (t44d, t44e, t44f, t450,) = {
    if ((!(l.f1499 > 50.0)) && (!(l.f1499 < t43c))) {
        let t43d: f64 = (l.f1499).exp();
        (t43d, (t43d * l.f149a), (t43d * l.f149b), (t43d * l.f149c),)
    } else {
        let t43e: f64 = (-50.0);
        let (t449, t44a, t44b, t44c,) = {
            if ((!(l.f1499 > 50.0)) && (l.f1499 < t43e)) {
                let t43f: f64 = (-50.0);let t440: f64 = (t43f).exp();
                (t440, 0.0, 0.0, 0.0,)
            } else {
                let (t445, t446, t447, t448,) = {
                    if (l.f1499 > 50.0) {
                        let t441: f64 = (50.0_f64).exp();let t442: f64 = (l.f1499 - 50.0);let t443: f64 = (1.0 + t442);let t444: f64 = (t441 * t443);
                        (t444, (t441 * l.f149a), (t441 * l.f149b), (t441 * l.f149c),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t445, t446, t447, t448,)
            }
        };
        (t449, t44a, t44b, t44c,)
    }
};
            (l.f1495, l.f1496, l.f1497, l.f1498, ) = (t44d, t44e, t44f, t450, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t451: f64 = (l.f1495 - 1.0);let t452: f64 = (l.f14cb * t451);(l.f14bf, l.f14c0, l.f14c1, l.f14c2, ) = (t452, ((l.f14cc * t451) + (l.f14cb * l.f1496)), (l.f14cb * l.f1497), (l.f14cb * l.f1498), );let t453: f64 = (l.f14ad + l.f14bf);(l.f14c3, l.f14c4, l.f14c5, l.f14c6, ) = (t453, (l.f14ae + l.f14c0), (l.f14af + l.f14c1), (l.f14b0 + l.f14c2), );(l.f14d6, l.f14d7, l.f14d8, l.f14d9, ) = (l.f14c3, l.f14c4, l.f14c5, l.f14c6, );(l.f213f, l.f2140, l.f2141, l.f2142, ) = (l.f14d6, l.f14d7, l.f14d8, l.f14d9, );(l.f1558, l.f1559, l.f155a, l.f155b, ) = (0.0, 0.0, 0.0, 0.0, );(l.f154b, l.f154c, ) = (0.0, 0.0, );(l.f154d, l.f154e, ) = (0.0, 0.0, );let t454: f64 = (p.p6 * (nv8 - nv5));(l.f1562, l.f1563, l.f1564, ) = (t454, (-p.p6), p.p6, );(l.f1556, l.f1557, ) = (l.f215b, l.f215c, );l.f1565 = p.p265;l.f14e9 = p.p267;l.f1525 = p.p266;l.f1553 = p.p263;l.f1551 = p.p281;l.f1561 = p.p280;(l.f155e, l.f155f, ) = (l.f22f2, l.f22f3, );l.f1568 = p.p0;l.f1550 = p.p2;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t455: f64 = (p.p255 * p.p264);l.f1549 = t455;l.f154f = p.p279;l.f1566 = p.p274;l.f14ea = p.p275;let t456: f64 = (p.p255 * p.p273);l.f154a = t456;l.f1555 = p.p272;l.f1552 = p.p257;l.f1567 = p.p256;l.f1560 = p.p6;(l.f1545, l.f1546, l.f1547, l.f1548, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14e7, l.f14e8, ) = (0.0, 0.0, );(l.f155c, l.f155d, ) = (0.0, 0.0, );(l.f1521, l.f1522, l.f1523, l.f1524, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1529, l.f152a, l.f152b, l.f152c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f152f, l.f1530, l.f1531, l.f1532, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1526, l.f1527, l.f1528, ) = (0.0, 0.0, 0.0, );(l.f1541, l.f1542, l.f1543, l.f1544, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14f3, l.f14f4, l.f14f5, l.f14f6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14f9, l.f14fa, ) = (0.0, 0.0, );(l.f14eb, l.f14ec, l.f14ed, l.f14ee, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14f1, l.f14f2, ) = (0.0, 0.0, );(l.f151f, l.f1520, ) = (0.0, 0.0, );(l.f14fb, l.f14fc, l.f14fd, l.f14fe, ) = (0.0, 0.0, 0.0, 0.0, );(l.f150b, l.f150c, l.f150d, l.f150e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f14ff, l.f1500, l.f1501, l.f1502, ) = (0.0, 0.0, 0.0, 0.0, );(l.f151b, l.f151c, l.f151d, l.f151e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1517, l.f1518, l.f1519, l.f151a, ) = (0.0, 0.0, 0.0, 0.0, );l.f1554 = 0.0;(l.f14f7, l.f14f8, ) = (0.0, 0.0, );(l.f14ef, l.f14f0, ) = (0.0, 0.0, );(l.f152d, l.f152e, ) = (0.0, 0.0, );(l.f1515, l.f1516, ) = (0.0, 0.0, );(l.f1509, l.f150a, ) = (0.0, 0.0, );(l.f153f, l.f1540, ) = (0.0, 0.0, );(l.f153b, l.f153c, l.f153d, l.f153e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1513, l.f1514, ) = (0.0, 0.0, );(l.f1507, l.f1508, ) = (0.0, 0.0, );(l.f1539, l.f153a, ) = (0.0, 0.0, );(l.f150f, l.f1510, l.f1511, l.f1512, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1503, l.f1504, l.f1505, l.f1506, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1537, l.f1538, ) = (0.0, 0.0, );(l.f1533, l.f1534, l.f1535, l.f1536, ) = (0.0, 0.0, 0.0, 0.0, );let t457: f64 = (l.f1552 / l.f1556);let t458: f64 = (-l.f1567);let t459: f64 = (t457 * t458);(l.f151f, l.f1520, ) = (t459, ((-((l.f1552 * l.f1557) / (l.f1556 * l.f1556))) * t458), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t45a: f64 = (-50.0);
            let (t467, t468,) = {
    if ((!(l.f151f > 50.0)) && (!(l.f151f < t45a))) {
        let t45b: f64 = (l.f151f).exp();
        (t45b, (t45b * l.f1520),)
    } else {
        let t45c: f64 = (-50.0);
        let (t465, t466,) = {
            if ((!(l.f151f > 50.0)) && (l.f151f < t45c)) {
                let t45d: f64 = (-50.0);let t45e: f64 = (t45d).exp();
                (t45e, 0.0,)
            } else {
                let (t463, t464,) = {
                    if (l.f151f > 50.0) {
                        let t45f: f64 = (50.0_f64).exp();let t460: f64 = (l.f151f - 50.0);let t461: f64 = (1.0 + t460);let t462: f64 = (t45f * t461);
                        (t462, (t45f * l.f1520),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t463, t464,)
            }
        };
        (t465, t466,)
    }
};
            (l.f155c, l.f155d, ) = (t467, t468, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t469: f64 = (-l.f1562);let t46a: f64 = (t469 - l.f1561);let t46b: f64 = (l.f1551 * t46a);let t46c: f64 = (t46b + l.f151f);(l.f14f3, l.f14f4, l.f14f5, l.f14f6, ) = (t46c, l.f1520, (l.f1551 * (-l.f1563)), (l.f1551 * (-l.f1564)), );let t46d: f64 = (-l.f1551);let t46e: f64 = (t46d * l.f1561);let t46f: f64 = (t46e + l.f151f);(l.f14f9, l.f14fa, ) = (t46f, l.f1520, );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t470: f64 = (-50.0);
            let (t481, t482, t483, t484,) = {
    if ((!(l.f14f3 > 50.0)) && (!(l.f14f3 < t470))) {
        let t471: f64 = (l.f14f3).exp();
        (t471, (t471 * l.f14f4), (t471 * l.f14f5), (t471 * l.f14f6),)
    } else {
        let t472: f64 = (-50.0);
        let (t47d, t47e, t47f, t480,) = {
            if ((!(l.f14f3 > 50.0)) && (l.f14f3 < t472)) {
                let t473: f64 = (-50.0);let t474: f64 = (t473).exp();
                (t474, 0.0, 0.0, 0.0,)
            } else {
                let (t479, t47a, t47b, t47c,) = {
                    if (l.f14f3 > 50.0) {
                        let t475: f64 = (50.0_f64).exp();let t476: f64 = (l.f14f3 - 50.0);let t477: f64 = (1.0 + t476);let t478: f64 = (t475 * t477);
                        (t478, (t475 * l.f14f4), (t475 * l.f14f5), (t475 * l.f14f6),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t479, t47a, t47b, t47c,)
            }
        };
        (t47d, t47e, t47f, t480,)
    }
};
            (l.f14eb, l.f14ec, l.f14ed, l.f14ee, ) = (t481, t482, t483, t484, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t485: f64 = (-50.0);
            let (t492, t493,) = {
    if ((!(l.f14f9 > 50.0)) && (!(l.f14f9 < t485))) {
        let t486: f64 = (l.f14f9).exp();
        (t486, (t486 * l.f14fa),)
    } else {
        let t487: f64 = (-50.0);
        let (t490, t491,) = {
            if ((!(l.f14f9 > 50.0)) && (l.f14f9 < t487)) {
                let t488: f64 = (-50.0);let t489: f64 = (t488).exp();
                (t489, 0.0,)
            } else {
                let (t48e, t48f,) = {
                    if (l.f14f9 > 50.0) {
                        let t48a: f64 = (50.0_f64).exp();let t48b: f64 = (l.f14f9 - 50.0);let t48c: f64 = (1.0 + t48b);let t48d: f64 = (t48a * t48c);
                        (t48d, (t48a * l.f14fa),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t48e, t48f,)
            }
        };
        (t490, t491,)
    }
};
            (l.f14f1, l.f14f2, ) = (t492, t493, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        l: &mut StampLocals,
    ) {
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t494: f64 = (l.f14eb - l.f14f1);(l.f1529, l.f152a, l.f152b, l.f152c, ) = (t494, (l.f14ec - l.f14f2), l.f14ed, l.f14ee, );let t495: f64 = (l.f1560 * l.f1568);let t496: f64 = (t495 * l.f1550);let t497: f64 = (t496 * l.f1549);let t498: f64 = (t497 * l.f155e);(l.f154b, l.f154c, ) = (t498, (t497 * l.f155f), );let t499: f64 = (l.f1553 / l.f1556);let t49a: f64 = (t499 * l.f1562);let t49b: f64 = (t49a + l.f151f);(l.f150b, l.f150c, l.f150d, l.f150e, ) = (t49b, (((-((l.f1553 * l.f1557) / (l.f1556 * l.f1556))) * l.f1562) + l.f1520), (t499 * l.f1563), (t499 * l.f1564), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t49c: f64 = (-50.0);
            let (t4ad, t4ae, t4af, t4b0,) = {
    if ((!(l.f150b > 50.0)) && (!(l.f150b < t49c))) {
        let t49d: f64 = (l.f150b).exp();
        (t49d, (t49d * l.f150c), (t49d * l.f150d), (t49d * l.f150e),)
    } else {
        let t49e: f64 = (-50.0);
        let (t4a9, t4aa, t4ab, t4ac,) = {
            if ((!(l.f150b > 50.0)) && (l.f150b < t49e)) {
                let t49f: f64 = (-50.0);let t4a0: f64 = (t49f).exp();
                (t4a0, 0.0, 0.0, 0.0,)
            } else {
                let (t4a5, t4a6, t4a7, t4a8,) = {
                    if (l.f150b > 50.0) {
                        let t4a1: f64 = (50.0_f64).exp();let t4a2: f64 = (l.f150b - 50.0);let t4a3: f64 = (1.0 + t4a2);let t4a4: f64 = (t4a1 * t4a3);
                        (t4a4, (t4a1 * l.f150c), (t4a1 * l.f150d), (t4a1 * l.f150e),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t4a5, t4a6, t4a7, t4a8,)
            }
        };
        (t4a9, t4aa, t4ab, t4ac,)
    }
};
            (l.f14ff, l.f1500, l.f1501, l.f1502, ) = (t4ad, t4ae, t4af, t4b0, );
        }
        let t4b1: f64 = if l.f1525 == 1.0 { 1.0 } else { 0.0 };l.f1ff1 = t4b1;
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 != 0.0)) {let t4b2: f64 = (l.f154f * l.f1529);let t4b3: f64 = (l.f14ff - t4b2);let t4b4: f64 = (t4b3 - l.f155c);let t4b5: f64 = (l.f154b * t4b4);(l.f152f, l.f1530, l.f1531, l.f1532, ) = (t4b5, ((l.f154c * t4b4) + (l.f154b * ((l.f1500 - (l.f154f * l.f152a)) - l.f155d))), (l.f154b * (l.f1501 - (l.f154f * l.f152b))), (l.f154b * (l.f1502 - (l.f154f * l.f152c))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {let t4b6: f64 = (-l.f1565);let t4b7: f64 = (t4b6 - l.f1561);let t4b8: f64 = (l.f1551 * t4b7);let t4b9: f64 = (t4b8 + l.f151f);(l.f14f7, l.f14f8, ) = (t4b9, l.f1520, );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {
            let t4ba: f64 = (-50.0);
            let (t4c7, t4c8,) = {
    if ((!(l.f14f7 > 50.0)) && (!(l.f14f7 < t4ba))) {
        let t4bb: f64 = (l.f14f7).exp();
        (t4bb, (t4bb * l.f14f8),)
    } else {
        let t4bc: f64 = (-50.0);
        let (t4c5, t4c6,) = {
            if ((!(l.f14f7 > 50.0)) && (l.f14f7 < t4bc)) {
                let t4bd: f64 = (-50.0);let t4be: f64 = (t4bd).exp();
                (t4be, 0.0,)
            } else {
                let (t4c3, t4c4,) = {
                    if (l.f14f7 > 50.0) {
                        let t4bf: f64 = (50.0_f64).exp();let t4c0: f64 = (l.f14f7 - 50.0);let t4c1: f64 = (1.0 + t4c0);let t4c2: f64 = (t4bf * t4c1);
                        (t4c2, (t4bf * l.f14f8),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t4c3, t4c4,)
            }
        };
        (t4c5, t4c6,)
    }
};
            (l.f14ef, l.f14f0, ) = (t4c7, t4c8, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {let t4c9: f64 = (l.f14ef - l.f14f1);(l.f152d, l.f152e, ) = (t4c9, (l.f14f0 - l.f14f2), );let t4ca: f64 = (l.f1553 / l.f1556);let t4cb: f64 = (t4ca * l.f1565);let t4cc: f64 = (t4cb + l.f151f);(l.f1515, l.f1516, ) = (t4cc, (((-((l.f1553 * l.f1557) / (l.f1556 * l.f1556))) * l.f1565) + l.f1520), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {
            let t4cd: f64 = (-50.0);
            let (t4da, t4db,) = {
    if ((!(l.f1515 > 50.0)) && (!(l.f1515 < t4cd))) {
        let t4ce: f64 = (l.f1515).exp();
        (t4ce, (t4ce * l.f1516),)
    } else {
        let t4cf: f64 = (-50.0);
        let (t4d8, t4d9,) = {
            if ((!(l.f1515 > 50.0)) && (l.f1515 < t4cf)) {
                let t4d0: f64 = (-50.0);let t4d1: f64 = (t4d0).exp();
                (t4d1, 0.0,)
            } else {
                let (t4d6, t4d7,) = {
                    if (l.f1515 > 50.0) {
                        let t4d2: f64 = (50.0_f64).exp();let t4d3: f64 = (l.f1515 - 50.0);let t4d4: f64 = (1.0 + t4d3);let t4d5: f64 = (t4d2 * t4d4);
                        (t4d5, (t4d2 * l.f1516),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t4d6, t4d7,)
            }
        };
        (t4d8, t4d9,)
    }
};
            (l.f1509, l.f150a, ) = (t4da, t4db, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {let t4dc: f64 = (l.f154f * l.f152d);let t4dd: f64 = (l.f1509 - t4dc);let t4de: f64 = (t4dd - l.f155c);(l.f153f, l.f1540, ) = (t4de, ((l.f150a - (l.f154f * l.f152e)) - l.f155d), );let t4df: f64 = (l.f154f * l.f1529);let t4e0: f64 = (l.f14ff - t4df);let t4e1: f64 = (t4e0 - l.f155c);let t4e2: f64 = (l.f154b * t4e1);(l.f153b, l.f153c, l.f153d, l.f153e, ) = (t4e2, ((l.f154c * t4e1) + (l.f154b * ((l.f1500 - (l.f154f * l.f152a)) - l.f155d))), (l.f154b * (l.f1501 - (l.f154f * l.f152b))), (l.f154b * (l.f1502 - (l.f154f * l.f152c))), );}
        let t4e3: f64 = if l.f1525 > 0.0 { 1.0 } else { 0.0 };l.f1ff2 = t4e3;
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff2 != 0.0)) {let t4e4: f64 = (l.f1525 * l.f1553);l.f1554 = t4e4;let t4e5: f64 = (l.f1554 / l.f1556);let t4e6: f64 = (t4e5 * l.f1565);let t4e7: f64 = (t4e6 + l.f151f);(l.f1513, l.f1514, ) = (t4e7, (((-((l.f1554 * l.f1557) / (l.f1556 * l.f1556))) * l.f1565) + l.f1520), );}
    }
}
