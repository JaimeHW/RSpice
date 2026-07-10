#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_256(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        if ((((l.f82f != 0.0) && (l.f833 == 0.0)) && (l.f927 == 0.0)) && (l.f94a != 0.0)) {let t20a: f64 = (1.0 - l.f17c7);let t20b: f64 = (l.f13d6 * t20a);let t20c: f64 = (l.ffea - l.f1a12);let t20d: f64 = (l.f13c6 * t20c);let t20e: f64 = (t20b + t20d);let t20f: f64 = (p.p30 * t20e);(l.f138e, l.f1391, l.f1392, l.f1393, l.f1394, l.f138f, l.f1390, ) = (t20f, (p.p30 * (l.f13d6 * (-l.f17ca))), (p.p30 * ((l.f13d6 * (-l.f17cb)) + (l.f13c6 * (l.ffed - l.f1a15)))), (p.p30 * ((l.f13d6 * (-l.f17cc)) + (l.f13c6 * (l.ffee - l.f1a16)))), (p.p30 * (l.f13d6 * (-l.f17cd))), (p.p30 * ((l.f13d6 * (-l.f17c8)) + (l.f13c6 * (l.ffeb - l.f1a13)))), (p.p30 * ((l.f13d6 * (-l.f17c9)) + (l.f13c6 * (l.ffec - l.f1a14)))), );let t210: f64 = (l.f1396 + l.f138e);(l.f1396, l.f1399, l.f139a, l.f139b, l.f139c, l.f1397, l.f1398, ) = (t210, (l.f1399 + l.f1391), (l.f139a + l.f1392), (l.f139b + l.f1393), (l.f139c + l.f1394), (l.f1397 + l.f138f), (l.f1398 + l.f1390), );}
        let t211: f64 = if l.ffff == 0.5 { 1.0 } else { 0.0 };l.f950 = t211;
        if (((((l.f82f != 0.0) && (l.f833 == 0.0)) && (l.f927 == 0.0)) && (l.f94a == 0.0)) && (l.f950 != 0.0)) {let t212: f64 = (l.f19fe * l.f1909);let t213: f64 = (1.0 - t212);let t214: f64 = (t213).sqrt();(l.f17c7, l.f17ca, l.f17cb, l.f17cc, l.f17cd, l.f17c8, l.f17c9, ) = (t214, 0.0, ((-(l.f1a01 * l.f1909)) / (2.0 * t214)), ((-(l.f1a02 * l.f1909)) / (2.0 * t214)), 0.0, ((-(l.f19ff * l.f1909)) / (2.0 * t214)), ((-(l.f1a00 * l.f1909)) / (2.0 * t214)), );}
        if (((((l.f82f != 0.0) && (l.f833 == 0.0)) && (l.f927 == 0.0)) && (l.f94a == 0.0)) && (l.f950 == 0.0)) {let t215: f64 = (l.f19fe * l.f1909);let t216: f64 = (1.0 - t215);let t217: f64 = (t216).powf(l.ffff);(l.f17c7, l.f17ca, l.f17cb, l.f17cc, l.f17cd, l.f17c8, l.f17c9, ) = (t217, 0.0, if 0.0 == 0.0 && ((l.ffff) as f64).is_finite() && ((l.ffff) as f64).fract() == 0.0 { if l.ffff == 0.0 { 0.0 } else { (l.ffff * ((t216).powf(l.ffff - 1.0) * (-(l.f1a01 * l.f1909)))) } } else { (t217 * (l.ffff * ((-(l.f1a01 * l.f1909)) / t216))) }, if 0.0 == 0.0 && ((l.ffff) as f64).is_finite() && ((l.ffff) as f64).fract() == 0.0 { if l.ffff == 0.0 { 0.0 } else { (l.ffff * ((t216).powf(l.ffff - 1.0) * (-(l.f1a02 * l.f1909)))) } } else { (t217 * (l.ffff * ((-(l.f1a02 * l.f1909)) / t216))) }, 0.0, if 0.0 == 0.0 && ((l.ffff) as f64).is_finite() && ((l.ffff) as f64).fract() == 0.0 { if l.ffff == 0.0 { 0.0 } else { (l.ffff * ((t216).powf(l.ffff - 1.0) * (-(l.f19ff * l.f1909)))) } } else { (t217 * (l.ffff * ((-(l.f19ff * l.f1909)) / t216))) }, if 0.0 == 0.0 && ((l.ffff) as f64).is_finite() && ((l.ffff) as f64).fract() == 0.0 { if l.ffff == 0.0 { 0.0 } else { (l.ffff * ((t216).powf(l.ffff - 1.0) * (-(l.f1a00 * l.f1909)))) } } else { (t217 * (l.ffff * ((-(l.f1a00 * l.f1909)) / t216))) }, );}
        if ((((l.f82f != 0.0) && (l.f833 == 0.0)) && (l.f927 == 0.0)) && (l.f94a == 0.0)) {let t218: f64 = (1.0 - l.f17c7);let t219: f64 = (l.f13d9 * t218);let t21a: f64 = (l.f1a18 - l.f19fe);let t21b: f64 = (l.f13c9 * t21a);let t21c: f64 = (t219 + t21b);let t21d: f64 = (p.p30 * t21c);(l.f1396, l.f1399, l.f139a, l.f139b, l.f139c, l.f1397, l.f1398, ) = (t21d, (p.p30 * (l.f13d9 * (-l.f17ca))), (p.p30 * ((l.f13d9 * (-l.f17cb)) + (l.f13c9 * (-l.f1a01)))), (p.p30 * ((l.f13d9 * (-l.f17cc)) + (l.f13c9 * (l.f1a1a - l.f1a02)))), (p.p30 * (l.f13d9 * (-l.f17cd))), (p.p30 * ((l.f13d9 * (-l.f17c8)) + (l.f13c9 * (-l.f19ff)))), (p.p30 * ((l.f13d9 * (-l.f17c9)) + (l.f13c9 * (l.f1a19 - l.f1a00)))), );}
        if ((l.f82f != 0.0) && (l.f833 == 0.0)) {let t21e: f64 = (l.f17 * l.fda2);let t21f: f64 = (l.ff0b * l.fddd);let t220: f64 = (t21e + t21f);let t221: f64 = (l.fef7 * l.fdc3);let t222: f64 = (t220 + t221);(l.fd93, l.fd96, l.fd97, l.fd98, l.fd99, l.fd94, l.fd95, ) = (t222, (((l.f17 * l.fda5) + (l.ff0b * l.fde0)) + (l.fef7 * l.fdc6)), (((l.f17 * l.fda6) + (l.ff0b * l.fde1)) + (l.fef7 * l.fdc7)), (((l.f17 * l.fda7) + (l.ff0b * l.fde2)) + (l.fef7 * l.fdc8)), (((l.f17 * l.fda8) + (l.ff0b * l.fde3)) + (l.fef7 * l.fdc9)), (((l.f17 * l.fda3) + (l.ff0b * l.fdde)) + (l.fef7 * l.fdc4)), (((l.f17 * l.fda4) + (l.ff0b * l.fddf)) + (l.fef7 * l.fdc5)), );}
        let t223: f64 = if l.f14a1 > 0.0 { 1.0 } else { 0.0 };l.f952 = t223;let t224: f64 = if l.f1401 > 0.0 { 1.0 } else { 0.0 };l.f953 = t224;let t225: f64 = if l.f1440 > 0.0 { 1.0 } else { 0.0 };l.f954 = t225;let t226: f64 = if l.f13ff > 0.0 { 1.0 } else { 0.0 };l.f955 = t226;let t227: f64 = if l.f13fd > 0.0 { 1.0 } else { 0.0 };l.f956 = t227;let t228: f64 = if l.f1433 > 0.0 { 1.0 } else { 0.0 };l.f959 = t228;let t229: f64 = if l.f1431 > 0.0 { 1.0 } else { 0.0 };l.f95a = t229;let t22a: f64 = if l.f1450 > 0.0 { 1.0 } else { 0.0 };l.f95b = t22a;(l.f104c, l.f104d, l.f104e, l.f104f, l.f1050, l.f1051, l.f1052, l.f1053, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1057, l.f1058, l.f1059, ) = (0.0, 0.0, 0.0, );(l.f1054, l.f1055, l.f1056, ) = (0.0, 0.0, 0.0, );let t22b: f64 = if l.f1440 > 0.0 { 1.0 } else { 0.0 };l.f95c = t22b;
        if (l.f95c != 0.0) {let t22c: f64 = (l.f683 * (nv2 - nv7));let t22d: f64 = (t22c * (nv2 - nv7));(l.f1057, l.f1058, l.f1059, ) = (t22d, ((l.f683 * (nv2 - nv7)) + t22c), (((-l.f683) * (nv2 - nv7)) + (-t22c)), );}
        let t22e: f64 = if l.f13ff > 0.0 { 1.0 } else { 0.0 };l.f95d = t22e;
        if (l.f95d != 0.0) {let t22f: f64 = (l.f5ba * (nv0 - nv8));let t230: f64 = (t22f * (nv0 - nv8));(l.f1054, l.f1055, l.f1056, ) = (t230, ((l.f5ba * (nv0 - nv8)) + t22f), (((-l.f5ba) * (nv0 - nv8)) + (-t22f)), );}
        let t231: f64 = if l.f144b > 0.001 { 1.0 } else { 0.0 };l.f95e = t231;
        if (l.f95e != 0.0) {let t232: f64 = (l.fcf9 + l.fd00);let t233: f64 = (t232 * l.f18a5);let t234: f64 = (l.f18a5 + l.f18bc);let t235: f64 = (l.fd86 * t234);let t236: f64 = (t233 + t235);let t237: f64 = (t236 + l.f1057);let t238: f64 = (t237 + l.f1054);(l.f104c, l.f104d, l.f104e, l.f104f, l.f1050, l.f1051, l.f1052, l.f1053, ) = (t238, l.f1055, l.f1058, (((l.fcfa + l.fd01) * l.f18a5) + (l.fd87 * t234)), (((l.fcfb + l.fd02) * l.f18a5) + (l.fd88 * t234)), (((((l.fcfc + l.fd03) * l.f18a5) + (t232 * l.f18a6)) + ((l.fd89 * t234) + (l.fd86 * (l.f18a6 + l.f18bd)))) + l.f1059), (((((l.fcfd + l.fd04) * l.f18a5) + (t232 * l.f18a7)) + ((l.fd8a * t234) + (l.fd86 * (l.f18a7 + l.f18be)))) + l.f1056), (((l.fcfe + l.fd05) * l.f18a5) + ((l.fd8b * t234) + (l.fd86 * l.f18bf))), );}
        let t239: f64 = (l.f12a7 + l.f11c1);let t23a: f64 = (t239 + l.f1250);let t23b: f64 = (-t23a);(l.f13e2, l.f13e3, l.f13e4, l.f13e5, l.f13e6, l.f13e7, ) = (t23b, (-((l.f12af + l.f11cc) + l.f1258)), (-((l.f12b0 + l.f11cd) + l.f1259)), (-((l.f12b1 + l.f11ce) + l.f125a)), (-((l.f12b2 + l.f11cf) + l.f125b)), (-((l.f12b3 + l.f11d0) + l.f125c)), );let t23c: f64 = (l.f12a2 + l.f12dd);(l.f12a2, l.f12a3, l.f12a4, l.f12a5, ) = (t23c, (l.f12a3 + l.f12de), (l.f12a4 + l.f12df), (l.f12a5 + l.f12e0), );let t23d: f64 = (l.f129d + l.f12d1);(l.f129d, l.f129e, l.f129f, l.f12a0, ) = (t23d, (l.f129e + l.f12d2), (l.f129f + l.f12d3), (l.f12a0 + l.f12d4), );let t23e: f64 = (l.f1b * l.f1386);let t23f: f64 = (l.ff0f * l.f13ae);let t240: f64 = (t23e + t23f);let t241: f64 = (l.fefb * l.f139e);let t242: f64 = (t240 + t241);(l.f1376, l.f1379, l.f137a, l.f137b, l.f137c, l.f1377, l.f1378, ) = (t242, (((l.f1b * l.f1389) + (l.ff0f * l.f13b1)) + (l.fefb * l.f13a1)), (((l.f1b * l.f138a) + (l.ff0f * l.f13b2)) + (l.fefb * l.f13a2)), (((l.f1b * l.f138b) + (l.ff0f * l.f13b3)) + (l.fefb * l.f13a3)), (((l.f1b * l.f138c) + (l.ff0f * l.f13b4)) + (l.fefb * l.f13a4)), (((l.f1b * l.f1387) + (l.ff0f * l.f13af)) + (l.fefb * l.f139f)), (((l.f1b * l.f1388) + (l.ff0f * l.f13b0)) + (l.fefb * l.f13a0)), );let t243: f64 = (l.f17 * l.f137e);let t244: f64 = (l.ff0b * l.f13a6);let t245: f64 = (t243 + t244);let t246: f64 = (l.fef7 * l.f1396);let t247: f64 = (t245 + t246);(l.f136e, l.f1371, l.f1372, l.f1373, l.f1374, l.f136f, l.f1370, ) = (t247, (((l.f17 * l.f1381) + (l.ff0b * l.f13a9)) + (l.fef7 * l.f1399)), (((l.f17 * l.f1382) + (l.ff0b * l.f13aa)) + (l.fef7 * l.f139a)), (((l.f17 * l.f1383) + (l.ff0b * l.f13ab)) + (l.fef7 * l.f139b)), (((l.f17 * l.f1384) + (l.ff0b * l.f13ac)) + (l.fef7 * l.f139c)), (((l.f17 * l.f137f) + (l.ff0b * l.f13a7)) + (l.fef7 * l.f1397)), (((l.f17 * l.f1380) + (l.ff0b * l.f13a8)) + (l.fef7 * l.f1398)), );let t248: f64 = if l.f14a1 < 0.0 { 1.0 } else { 0.0 };l.f95f = t248;
        if (l.f95f != 0.0) {(l.f16ef, l.f16f0, l.f16f1, l.f16f2, l.f16f3, l.f16f4, ) = (l.f1250, l.f1258, l.f1259, l.f125a, l.f125b, l.f125c, );(l.f1250, l.f1258, l.f1259, l.f125a, l.f125b, l.f125c, ) = (l.f13e2, l.f13e3, l.f13e4, l.f13e5, l.f13e6, l.f13e7, );(l.f13e2, l.f13e3, l.f13e4, l.f13e5, l.f13e6, l.f13e7, ) = (l.f16ef, l.f16f0, l.f16f1, l.f16f2, l.f16f3, l.f16f4, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_257(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        (l.f149b, l.f149c, l.f149d, l.f149e, l.f149f, l.f14a0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff5a, l.ff5b, l.ff5c, l.ff5d, l.ff5e, l.ff5f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff6e, l.ff6f, l.ff70, l.ff71, l.ff72, l.ff73, ) = (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.ff74, l.ff7b, l.ff7c, l.ff7d, l.ff7e, l.ff7f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f165, l.f166, l.f167, l.f168, l.f169, l.f16a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );let t249: f64 = (l.f1df * l.f45c);(l.f196, l.f197, l.f198, l.f199, l.f19a, l.f19b, ) = (t249, ((l.f1e0 * l.f45c) + (l.f1df * l.f45d)), ((l.f1e1 * l.f45c) + (l.f1df * l.f45e)), ((l.f1e2 * l.f45c) + (l.f1df * l.f45f)), ((l.f1e3 * l.f45c) + (l.f1df * l.f460)), ((l.f1e4 * l.f45c) + (l.f1df * l.f461)), );(l.f1639, l.f163a, l.f163b, l.f163c, l.f163d, l.f163e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f163f, l.f1640, l.f1641, l.f1642, l.f1643, l.f1644, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );let t24a: f64 = if ((l.f1c28 > 0.0) && (l.f115 > 0.0)) { 1.0 } else { 0.0 };l.f969 = t24a;let t24b: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };l.f96b = t24b;
        if ((l.f969 != 0.0) && (l.f96b != 0.0)) {let t24c: f64 = (l.f12f9 / l.f69);(l.fc97, l.fc98, l.fc99, l.fc9a, l.fc9b, l.fc9c, ) = (t24c, (((l.f12fa * l.f69) - (l.f12f9 * l.f6a)) / (l.f69 * l.f69)), (((l.f12fb * l.f69) - (l.f12f9 * l.f6b)) / (l.f69 * l.f69)), (((l.f12fc * l.f69) - (l.f12f9 * l.f6c)) / (l.f69 * l.f69)), (((l.f12fd * l.f69) - (l.f12f9 * l.f6d)) / (l.f69 * l.f69)), (((l.f12fe * l.f69) - (l.f12f9 * l.f6e)) / (l.f69 * l.f69)), );let t24d: f64 = (l.f1314 / l.f12f9);(l.f16d0, l.f16d1, l.f16d2, l.f16d3, l.f16d4, l.f16d5, ) = (t24d, (((l.f1315 * l.f12f9) - (l.f1314 * l.f12fa)) / (l.f12f9 * l.f12f9)), (((l.f1316 * l.f12f9) - (l.f1314 * l.f12fb)) / (l.f12f9 * l.f12f9)), (((l.f1317 * l.f12f9) - (l.f1314 * l.f12fc)) / (l.f12f9 * l.f12f9)), (((l.f1318 * l.f12f9) - (l.f1314 * l.f12fd)) / (l.f12f9 * l.f12f9)), (((l.f1319 * l.f12f9) - (l.f1314 * l.f12fe)) / (l.f12f9 * l.f12f9)), );let t24e: f64 = (0.5 * 0.16666666666666666);let t24f: f64 = (l.f355 / l.fc97);let t250: f64 = (t24e * t24f);(l.f1678, l.f1679, l.f167a, l.f167b, l.f167c, l.f167d, ) = (t250, (t24e * (((l.f356 * l.fc97) - (l.f355 * l.fc98)) / (l.fc97 * l.fc97))), (t24e * (((l.f357 * l.fc97) - (l.f355 * l.fc99)) / (l.fc97 * l.fc97))), (t24e * (((l.f358 * l.fc97) - (l.f355 * l.fc9a)) / (l.fc97 * l.fc97))), (t24e * (((l.f359 * l.fc97) - (l.f355 * l.fc9b)) / (l.fc97 * l.fc97))), (t24e * (((l.f35a * l.fc97) - (l.f355 * l.fc9c)) / (l.fc97 * l.fc97))), );let t251: f64 = (l.f1678 * l.f1678);(l.f16d6, l.f16d7, l.f16d8, l.f16d9, l.f16da, l.f16db, ) = (t251, ((l.f1679 * l.f1678) + (l.f1678 * l.f1679)), ((l.f167a * l.f1678) + (l.f1678 * l.f167a)), ((l.f167b * l.f1678) + (l.f1678 * l.f167b)), ((l.f167c * l.f1678) + (l.f1678 * l.f167c)), ((l.f167d * l.f1678) + (l.f1678 * l.f167d)), );let t252: f64 = (l.fc97 / l.fcc0);let t253: f64 = (t252 - 1.0);(l.f13f7, l.f13f8, l.f13f9, l.f13fa, l.f13fb, l.f13fc, ) = (t253, (((l.fc98 * l.fcc0) - (l.fc97 * l.fcc1)) / (l.fcc0 * l.fcc0)), (((l.fc99 * l.fcc0) - (l.fc97 * l.fcc2)) / (l.fcc0 * l.fcc0)), (((l.fc9a * l.fcc0) - (l.fc97 * l.fcc3)) / (l.fcc0 * l.fcc0)), (((l.fc9b * l.fcc0) - (l.fc97 * l.fcc4)) / (l.fcc0 * l.fcc0)), (((l.fc9c * l.fcc0) - (l.fc97 * l.fcc5)) / (l.fcc0 * l.fcc0)), );}
        if ((l.f969 != 0.0) && (l.f96b != 0.0)) {
            let t254: f64 = (l.f13f7 * l.f16d6);let t255: f64 = (12.0 * t254);let t256: f64 = (1.0 - t255);
            let (t25a, t25b, t25c, t25d, t25e, t25f,) = {
    if (t256 > 1e-20) {
        let t257: f64 = (l.f13f7 * l.f16d6);let t258: f64 = (12.0 * t257);let t259: f64 = (1.0 - t258);
        (t259, (-(12.0 * ((l.f13f8 * l.f16d6) + (l.f13f7 * l.f16d7)))), (-(12.0 * ((l.f13f9 * l.f16d6) + (l.f13f7 * l.f16d8)))), (-(12.0 * ((l.f13fa * l.f16d6) + (l.f13f7 * l.f16d9)))), (-(12.0 * ((l.f13fb * l.f16d6) + (l.f13f7 * l.f16da)))), (-(12.0 * ((l.f13fc * l.f16d6) + (l.f13f7 * l.f16db)))),)
    } else {
        (1e-20, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.fee3, l.fee4, l.fee5, l.fee6, l.fee7, l.fee8, ) = (t25a, t25b, t25c, t25d, t25e, t25f, );
        }
        if ((l.f969 != 0.0) && (l.f96b != 0.0)) {let t260: f64 = (l.fee3 * l.fee3);let t261: f64 = (1.0 / t260);(l.fee9, l.feea, l.feeb, l.feec, l.feed, l.feee, ) = (t261, (-(((l.fee4 * l.fee3) + (l.fee3 * l.fee4)) / (t260 * t260))), (-(((l.fee5 * l.fee3) + (l.fee3 * l.fee5)) / (t260 * t260))), (-(((l.fee6 * l.fee3) + (l.fee3 * l.fee6)) / (t260 * t260))), (-(((l.fee7 * l.fee3) + (l.fee3 * l.fee7)) / (t260 * t260))), (-(((l.fee8 * l.fee3) + (l.fee3 * l.fee8)) / (t260 * t260))), );let t262: f64 = (l.f115 * l.f12f9);let t263: f64 = (t262 * l.fc8d);(l.f577, l.f578, l.f579, l.f57a, l.f57b, l.f57c, ) = (t263, ((((l.f116 * l.f12f9) + (l.f115 * l.f12fa)) * l.fc8d) + (t262 * l.fc8e)), (((l.f115 * l.f12fb) * l.fc8d) + (t262 * l.fc8f)), (((l.f115 * l.f12fc) * l.fc8d) + (t262 * l.fc90)), (((l.f115 * l.f12fd) * l.fc8d) + (t262 * l.fc91)), (((l.f115 * l.f12fe) * l.fc8d) + (t262 * l.fc92)), );let t264: f64 = (12.0 * l.f16d6);let t265: f64 = (l.f16d0 + t264);let t266: f64 = (1.0 + l.f16d0);let t267: f64 = (t266 * l.f16d6);let t268: f64 = (t267 * l.f13f7);let t269: f64 = (24.0 * t268);let t26a: f64 = (t265 - t269);(l.ff5a, l.ff5b, l.ff5c, l.ff5d, l.ff5e, l.ff5f, ) = (t26a, ((l.f16d1 + (12.0 * l.f16d7)) - (24.0 * ((((l.f16d1 * l.f16d6) + (t266 * l.f16d7)) * l.f13f7) + (t267 * l.f13f8)))), ((l.f16d2 + (12.0 * l.f16d8)) - (24.0 * ((((l.f16d2 * l.f16d6) + (t266 * l.f16d8)) * l.f13f7) + (t267 * l.f13f9)))), ((l.f16d3 + (12.0 * l.f16d9)) - (24.0 * ((((l.f16d3 * l.f16d6) + (t266 * l.f16d9)) * l.f13f7) + (t267 * l.f13fa)))), ((l.f16d4 + (12.0 * l.f16da)) - (24.0 * ((((l.f16d4 * l.f16d6) + (t266 * l.f16da)) * l.f13f7) + (t267 * l.f13fb)))), ((l.f16d5 + (12.0 * l.f16db)) - (24.0 * ((((l.f16d5 * l.f16d6) + (t266 * l.f16db)) * l.f13f7) + (t267 * l.f13fc)))), );}
        if ((l.f969 != 0.0) && (l.f96b != 0.0)) {
            let (t26b, t26c, t26d, t26e, t26f, t270,) = {
    if (l.ff5a > 1e-40) {
        (l.ff5a, l.ff5b, l.ff5c, l.ff5d, l.ff5e, l.ff5f,)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.ff5a, l.ff5b, l.ff5c, l.ff5d, l.ff5e, l.ff5f, ) = (t26b, t26c, t26d, t26e, t26f, t270, );
        }
        if ((l.f969 != 0.0) && (l.f96b != 0.0)) {let t271: f64 = (l.f577 * l.fee9);let t272: f64 = (t271 * l.ff5a);(l.ff5a, l.ff5b, l.ff5c, l.ff5d, l.ff5e, l.ff5f, ) = (t272, ((((l.f578 * l.fee9) + (l.f577 * l.feea)) * l.ff5a) + (t271 * l.ff5b)), ((((l.f579 * l.fee9) + (l.f577 * l.feeb)) * l.ff5a) + (t271 * l.ff5c)), ((((l.f57a * l.fee9) + (l.f577 * l.feec)) * l.ff5a) + (t271 * l.ff5d)), ((((l.f57b * l.fee9) + (l.f577 * l.feed)) * l.ff5a) + (t271 * l.ff5e)), ((((l.f57c * l.fee9) + (l.f577 * l.feee)) * l.ff5a) + (t271 * l.ff5f)), );}
        let t273: f64 = if l.f523 > 0.0 { 1.0 } else { 0.0 };l.f96c = t273;
        if (((l.f969 != 0.0) && (l.f96b != 0.0)) && (l.f96c != 0.0)) {let t274: f64 = (l.f1786 / l.f60f);(l.f1754, l.f1755, l.f1756, l.f1757, l.f1758, l.f1759, ) = (t274, (((l.f1787 * l.f60f) - (l.f1786 * l.f610)) / (l.f60f * l.f60f)), (((l.f1788 * l.f60f) - (l.f1786 * l.f611)) / (l.f60f * l.f60f)), (((l.f1789 * l.f60f) - (l.f1786 * l.f612)) / (l.f60f * l.f60f)), (((l.f178a * l.f60f) - (l.f1786 * l.f613)) / (l.f60f * l.f60f)), (((l.f178b * l.f60f) - (l.f1786 * l.f614)) / (l.f60f * l.f60f)), );let t275: f64 = (l.f1754 * l.f1754);let t276: f64 = (t275 * l.f355);let t277: f64 = (t276 * l.f355);(l.f1e5c, l.f1e5d, l.f1e5e, l.f1e5f, l.f1e60, l.f1e61, ) = (t277, ((((((l.f1755 * l.f1754) + (l.f1754 * l.f1755)) * l.f355) + (t275 * l.f356)) * l.f355) + (t276 * l.f356)), ((((((l.f1756 * l.f1754) + (l.f1754 * l.f1756)) * l.f355) + (t275 * l.f357)) * l.f355) + (t276 * l.f357)), ((((((l.f1757 * l.f1754) + (l.f1754 * l.f1757)) * l.f355) + (t275 * l.f358)) * l.f355) + (t276 * l.f358)), ((((((l.f1758 * l.f1754) + (l.f1754 * l.f1758)) * l.f355) + (t275 * l.f359)) * l.f355) + (t276 * l.f359)), ((((((l.f1759 * l.f1754) + (l.f1754 * l.f1759)) * l.f355) + (t275 * l.f35a)) * l.f355) + (t276 * l.f35a)), );}
        let t278: f64 = (-1.0);let t279: f64 = if l.f1b5 == t278 { 1.0 } else { 0.0 };l.f96d = t279;
        if ((((l.f969 != 0.0) && (l.f96b != 0.0)) && (l.f96c != 0.0)) && (l.f96d != 0.0)) {let t27a: f64 = (l.f1754 * l.f355);let t27b: f64 = (1.0 + t27a);let t27c: f64 = (l.f1e5c / t27b);(l.f1e5c, l.f1e5d, l.f1e5e, l.f1e5f, l.f1e60, l.f1e61, ) = (t27c, (((l.f1e5d * t27b) - (l.f1e5c * ((l.f1755 * l.f355) + (l.f1754 * l.f356)))) / (t27b * t27b)), (((l.f1e5e * t27b) - (l.f1e5c * ((l.f1756 * l.f355) + (l.f1754 * l.f357)))) / (t27b * t27b)), (((l.f1e5f * t27b) - (l.f1e5c * ((l.f1757 * l.f355) + (l.f1754 * l.f358)))) / (t27b * t27b)), (((l.f1e60 * t27b) - (l.f1e5c * ((l.f1758 * l.f355) + (l.f1754 * l.f359)))) / (t27b * t27b)), (((l.f1e61 * t27b) - (l.f1e5c * ((l.f1759 * l.f355) + (l.f1754 * l.f35a)))) / (t27b * t27b)), );}
        if (((l.f969 != 0.0) && (l.f96b != 0.0)) && (l.f96c != 0.0)) {let t27d: f64 = (2.0 * l.f1e5c);let t27e: f64 = (1.0 + t27d);let t27f: f64 = (t27e).sqrt();let t280: f64 = (1.0 + t27f);let t281: f64 = (l.f60f * t280);let t282: f64 = (0.5 * t281);(l.fc86, l.fc87, l.fc88, l.fc89, l.fc8a, l.fc8b, ) = (t282, (0.5 * ((l.f610 * t280) + (l.f60f * ((2.0 * l.f1e5d) / (2.0 * t27f))))), (0.5 * ((l.f611 * t280) + (l.f60f * ((2.0 * l.f1e5e) / (2.0 * t27f))))), (0.5 * ((l.f612 * t280) + (l.f60f * ((2.0 * l.f1e5f) / (2.0 * t27f))))), (0.5 * ((l.f613 * t280) + (l.f60f * ((2.0 * l.f1e60) / (2.0 * t27f))))), (0.5 * ((l.f614 * t280) + (l.f60f * ((2.0 * l.f1e61) / (2.0 * t27f))))), );let t283: f64 = (l.fc86 * l.fee3);let t284: f64 = (l.f60f / t283);(l.f5ec, l.f5ed, l.f5ee, l.f5ef, l.f5f0, l.f5f1, ) = (t284, (((l.f610 * t283) - (l.f60f * ((l.fc87 * l.fee3) + (l.fc86 * l.fee4)))) / (t283 * t283)), (((l.f611 * t283) - (l.f60f * ((l.fc88 * l.fee3) + (l.fc86 * l.fee5)))) / (t283 * t283)), (((l.f612 * t283) - (l.f60f * ((l.fc89 * l.fee3) + (l.fc86 * l.fee6)))) / (t283 * t283)), (((l.f613 * t283) - (l.f60f * ((l.fc8a * l.fee3) + (l.fc86 * l.fee7)))) / (t283 * t283)), (((l.f614 * t283) - (l.f60f * ((l.fc8b * l.fee3) + (l.fc86 * l.fee8)))) / (t283 * t283)), );let t285: f64 = (l.f491 * l.fcf9);let t286: f64 = (t285 * l.f1985);let t287: f64 = (t286 * l.f5ec);let t288: f64 = (t287 * l.f5ec);(l.f149b, l.f149c, l.f149d, l.f149e, l.f149f, l.f14a0, ) = (t288, (((((((l.f491 * l.fcfa) * l.f1985) + (t285 * l.f1986)) * l.f5ec) + (t286 * l.f5ed)) * l.f5ec) + (t287 * l.f5ed)), (((((((l.f491 * l.fcfb) * l.f1985) + (t285 * l.f1987)) * l.f5ec) + (t286 * l.f5ee)) * l.f5ec) + (t287 * l.f5ee)), (((((((l.f491 * l.fcfc) * l.f1985) + (t285 * l.f1988)) * l.f5ec) + (t286 * l.f5ef)) * l.f5ec) + (t287 * l.f5ef)), (((((((l.f491 * l.fcfd) * l.f1985) + (t285 * l.f1989)) * l.f5ec) + (t286 * l.f5f0)) * l.f5ec) + (t287 * l.f5f0)), (((((((l.f491 * l.fcfe) * l.f1985) + (t285 * l.f198a)) * l.f5ec) + (t286 * l.f5f1)) * l.f5ec) + (t287 * l.f5f1)), );let t289: f64 = (l.f149b / l.ffe5);let t28a: f64 = (l.ff5a + t289);(l.ff5a, l.ff5b, l.ff5c, l.ff5d, l.ff5e, l.ff5f, ) = (t28a, (l.ff5b + (((l.f149c * l.ffe5) - (l.f149b * l.ffe6)) / (l.ffe5 * l.ffe5))), (l.ff5c + (l.f149d / l.ffe5)), (l.ff5d + (l.f149e / l.ffe5)), (l.ff5e + (l.f149f / l.ffe5)), (l.ff5f + (l.f14a0 / l.ffe5)), );}
        if ((l.f969 != 0.0) && (l.f96b != 0.0)) {let t28b: f64 = (l.ffe4 * l.ff5a);let t28c: f64 = (t28b).sqrt();(l.f1639, l.f163a, l.f163b, l.f163c, l.f163d, l.f163e, ) = (t28c, (((l.ffe7 * l.ff5a) + (l.ffe4 * l.ff5b)) / (2.0 * t28c)), ((l.ffe4 * l.ff5c) / (2.0 * t28c)), ((l.ffe4 * l.ff5d) / (2.0 * t28c)), ((l.ffe4 * l.ff5e) / (2.0 * t28c)), ((l.ffe4 * l.ff5f) / (2.0 * t28c)), );}
        let t28d: f64 = if ((((p.p50 == 1.0) && (l.ffe4 > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };l.f96e = t28d;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_258(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {let t28e: f64 = (l.f16d0 / 12.0);let t28f: f64 = (l.f16d0 + 0.2);let t290: f64 = (12.0 * l.f16d6);let t291: f64 = (t28f - t290);let t292: f64 = (l.f16d6 * t291);let t293: f64 = (t28e - t292);let t294: f64 = (l.f16d0 + 1.0);let t295: f64 = (12.0 * l.f16d6);let t296: f64 = (t294 - t295);let t297: f64 = (l.f16d6 * t296);let t298: f64 = (t297 * l.f13f7);let t299: f64 = (1.6 * t298);let t29a: f64 = (t293 - t299);(l.ff6e, l.ff6f, l.ff70, l.ff71, l.ff72, l.ff73, ) = (t29a, (((l.f16d1 / 12.0) - ((l.f16d7 * t291) + (l.f16d6 * (l.f16d1 - (12.0 * l.f16d7))))) - (1.6 * ((((l.f16d7 * t296) + (l.f16d6 * (l.f16d1 - (12.0 * l.f16d7)))) * l.f13f7) + (t297 * l.f13f8)))), (((l.f16d2 / 12.0) - ((l.f16d8 * t291) + (l.f16d6 * (l.f16d2 - (12.0 * l.f16d8))))) - (1.6 * ((((l.f16d8 * t296) + (l.f16d6 * (l.f16d2 - (12.0 * l.f16d8)))) * l.f13f7) + (t297 * l.f13f9)))), (((l.f16d3 / 12.0) - ((l.f16d9 * t291) + (l.f16d6 * (l.f16d3 - (12.0 * l.f16d9))))) - (1.6 * ((((l.f16d9 * t296) + (l.f16d6 * (l.f16d3 - (12.0 * l.f16d9)))) * l.f13f7) + (t297 * l.f13fa)))), (((l.f16d4 / 12.0) - ((l.f16da * t291) + (l.f16d6 * (l.f16d4 - (12.0 * l.f16da))))) - (1.6 * ((((l.f16da * t296) + (l.f16d6 * (l.f16d4 - (12.0 * l.f16da)))) * l.f13f7) + (t297 * l.f13fb)))), (((l.f16d5 / 12.0) - ((l.f16db * t291) + (l.f16d6 * (l.f16d5 - (12.0 * l.f16db))))) - (1.6 * ((((l.f16db * t296) + (l.f16d6 * (l.f16d5 - (12.0 * l.f16db)))) * l.f13f7) + (t297 * l.f13fc)))), );}
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {
            let (t29b, t29c, t29d, t29e, t29f, t2a0,) = {
    if (l.ff6e > 1e-40) {
        (l.ff6e, l.ff6f, l.ff70, l.ff71, l.ff72, l.ff73,)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.ff6e, l.ff6f, l.ff70, l.ff71, l.ff72, l.ff73, ) = (t29b, t29c, t29d, t29e, t29f, t2a0, );
        }
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {let t2a1: f64 = (l.fee9 / l.f577);let t2a2: f64 = (t2a1 * l.ff6e);(l.ff6e, l.ff6f, l.ff70, l.ff71, l.ff72, l.ff73, ) = (t2a2, (((((l.feea * l.f577) - (l.fee9 * l.f578)) / (l.f577 * l.f577)) * l.ff6e) + (t2a1 * l.ff6f)), (((((l.feeb * l.f577) - (l.fee9 * l.f579)) / (l.f577 * l.f577)) * l.ff6e) + (t2a1 * l.ff70)), (((((l.feec * l.f577) - (l.fee9 * l.f57a)) / (l.f577 * l.f577)) * l.ff6e) + (t2a1 * l.ff71)), (((((l.feed * l.f577) - (l.fee9 * l.f57b)) / (l.f577 * l.f577)) * l.ff6e) + (t2a1 * l.ff72)), (((((l.feee * l.f577) - (l.fee9 * l.f57c)) / (l.f577 * l.f577)) * l.ff6e) + (t2a1 * l.ff73)), );let t2a3: f64 = (l.fee9 * l.f1678);let t2a4: f64 = (12.0 * l.f16d6);let t2a5: f64 = (1.0 - t2a4);let t2a6: f64 = (19.2 * l.f16d6);let t2a7: f64 = (l.f16d0 + t2a6);let t2a8: f64 = (l.f16d0 * l.f16d6);let t2a9: f64 = (12.0 * t2a8);let t2aa: f64 = (t2a7 - t2a9);let t2ab: f64 = (t2aa * l.f13f7);let t2ac: f64 = (t2a5 - t2ab);let t2ad: f64 = (t2a3 * t2ac);(l.ff75, l.ff76, l.ff77, l.ff78, l.ff79, l.ff7a, ) = (t2ad, ((((l.feea * l.f1678) + (l.fee9 * l.f1679)) * t2ac) + (t2a3 * ((-(12.0 * l.f16d7)) - ((((l.f16d1 + (19.2 * l.f16d7)) - (12.0 * ((l.f16d1 * l.f16d6) + (l.f16d0 * l.f16d7)))) * l.f13f7) + (t2aa * l.f13f8))))), ((((l.feeb * l.f1678) + (l.fee9 * l.f167a)) * t2ac) + (t2a3 * ((-(12.0 * l.f16d8)) - ((((l.f16d2 + (19.2 * l.f16d8)) - (12.0 * ((l.f16d2 * l.f16d6) + (l.f16d0 * l.f16d8)))) * l.f13f7) + (t2aa * l.f13f9))))), ((((l.feec * l.f1678) + (l.fee9 * l.f167b)) * t2ac) + (t2a3 * ((-(12.0 * l.f16d9)) - ((((l.f16d3 + (19.2 * l.f16d9)) - (12.0 * ((l.f16d3 * l.f16d6) + (l.f16d0 * l.f16d9)))) * l.f13f7) + (t2aa * l.f13fa))))), ((((l.feed * l.f1678) + (l.fee9 * l.f167c)) * t2ac) + (t2a3 * ((-(12.0 * l.f16da)) - ((((l.f16d4 + (19.2 * l.f16da)) - (12.0 * ((l.f16d4 * l.f16d6) + (l.f16d0 * l.f16da)))) * l.f13f7) + (t2aa * l.f13fb))))), ((((l.feee * l.f1678) + (l.fee9 * l.f167d)) * t2ac) + (t2a3 * ((-(12.0 * l.f16db)) - ((((l.f16d5 + (19.2 * l.f16db)) - (12.0 * ((l.f16d5 * l.f16d6) + (l.f16d0 * l.f16db)))) * l.f13f7) + (t2aa * l.f13fc))))), );}
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {let t2ae: f64 = (l.fc7a * l.fc7a);let t2af: f64 = (t2ae * l.f1df);let t2b0: f64 = (t2af * l.f45c);let t2b1: f64 = (l.f616 * l.f616);let t2b2: f64 = (t2b0 / t2b1);(l.f196, l.f197, l.f198, l.f199, l.f19a, l.f19b, ) = (t2b2, (((((((((l.fc7b * l.fc7a) + (l.fc7a * l.fc7b)) * l.f1df) + (t2ae * l.f1e0)) * l.f45c) + (t2af * l.f45d)) * t2b1) - (t2b0 * ((l.f617 * l.f616) + (l.f616 * l.f617)))) / (t2b1 * t2b1)), (((((((((l.fc7c * l.fc7a) + (l.fc7a * l.fc7c)) * l.f1df) + (t2ae * l.f1e1)) * l.f45c) + (t2af * l.f45e)) * t2b1) - (t2b0 * ((l.f618 * l.f616) + (l.f616 * l.f618)))) / (t2b1 * t2b1)), (((((((((l.fc7d * l.fc7a) + (l.fc7a * l.fc7d)) * l.f1df) + (t2ae * l.f1e2)) * l.f45c) + (t2af * l.f45f)) * t2b1) - (t2b0 * ((l.f619 * l.f616) + (l.f616 * l.f619)))) / (t2b1 * t2b1)), (((((((((l.fc7e * l.fc7a) + (l.fc7a * l.fc7e)) * l.f1df) + (t2ae * l.f1e3)) * l.f45c) + (t2af * l.f460)) * t2b1) - (t2b0 * ((l.f61a * l.f616) + (l.f616 * l.f61a)))) / (t2b1 * t2b1)), (((((((((l.fc7f * l.fc7a) + (l.fc7a * l.fc7f)) * l.f1df) + (t2ae * l.f1e4)) * l.f45c) + (t2af * l.f461)) * t2b1) - (t2b0 * ((l.f61b * l.f616) + (l.f616 * l.f61b)))) / (t2b1 * t2b1)), );}
        let t2b3: f64 = if l.f523 > 0.0 { 1.0 } else { 0.0 };l.f970 = t2b3;
        if (((l.f969 != 0.0) && (l.f96e != 0.0)) && (l.f970 != 0.0)) {let t2b4: f64 = (12.0 * l.f16d6);let t2b5: f64 = (1.0 + t2b4);let t2b6: f64 = (l.f149b * t2b5);let t2b7: f64 = (12.0 * l.f577);let t2b8: f64 = (t2b7 * l.f577);let t2b9: f64 = (t2b8 * l.ffe5);let t2ba: f64 = (t2b6 / t2b9);let t2bb: f64 = (l.ff6e + t2ba);(l.ff6e, l.ff6f, l.ff70, l.ff71, l.ff72, l.ff73, ) = (t2bb, (l.ff6f + (((((l.f149c * t2b5) + (l.f149b * (12.0 * l.f16d7))) * t2b9) - (t2b6 * (((((12.0 * l.f578) * l.f577) + (t2b7 * l.f578)) * l.ffe5) + (t2b8 * l.ffe6)))) / (t2b9 * t2b9))), (l.ff70 + (((((l.f149d * t2b5) + (l.f149b * (12.0 * l.f16d8))) * t2b9) - (t2b6 * ((((12.0 * l.f579) * l.f577) + (t2b7 * l.f579)) * l.ffe5))) / (t2b9 * t2b9))), (l.ff71 + (((((l.f149e * t2b5) + (l.f149b * (12.0 * l.f16d9))) * t2b9) - (t2b6 * ((((12.0 * l.f57a) * l.f577) + (t2b7 * l.f57a)) * l.ffe5))) / (t2b9 * t2b9))), (l.ff72 + (((((l.f149f * t2b5) + (l.f149b * (12.0 * l.f16da))) * t2b9) - (t2b6 * ((((12.0 * l.f57b) * l.f577) + (t2b7 * l.f57b)) * l.ffe5))) / (t2b9 * t2b9))), (l.ff73 + (((((l.f14a0 * t2b5) + (l.f149b * (12.0 * l.f16db))) * t2b9) - (t2b6 * ((((12.0 * l.f57c) * l.f577) + (t2b7 * l.f57c)) * l.ffe5))) / (t2b9 * t2b9))), );let t2bc: f64 = (l.f149b * l.f1678);let t2bd: f64 = (1.0 + l.f13f7);let t2be: f64 = (t2bc * t2bd);let t2bf: f64 = (l.f577 * l.ffe5);let t2c0: f64 = (t2be / t2bf);let t2c1: f64 = (l.ff75 - t2c0);(l.ff75, l.ff76, l.ff77, l.ff78, l.ff79, l.ff7a, ) = (t2c1, (l.ff76 - (((((((l.f149c * l.f1678) + (l.f149b * l.f1679)) * t2bd) + (t2bc * l.f13f8)) * t2bf) - (t2be * ((l.f578 * l.ffe5) + (l.f577 * l.ffe6)))) / (t2bf * t2bf))), (l.ff77 - (((((((l.f149d * l.f1678) + (l.f149b * l.f167a)) * t2bd) + (t2bc * l.f13f9)) * t2bf) - (t2be * (l.f579 * l.ffe5))) / (t2bf * t2bf))), (l.ff78 - (((((((l.f149e * l.f1678) + (l.f149b * l.f167b)) * t2bd) + (t2bc * l.f13fa)) * t2bf) - (t2be * (l.f57a * l.ffe5))) / (t2bf * t2bf))), (l.ff79 - (((((((l.f149f * l.f1678) + (l.f149b * l.f167c)) * t2bd) + (t2bc * l.f13fb)) * t2bf) - (t2be * (l.f57b * l.ffe5))) / (t2bf * t2bf))), (l.ff7a - (((((((l.f14a0 * l.f1678) + (l.f149b * l.f167d)) * t2bd) + (t2bc * l.f13fc)) * t2bf) - (t2be * (l.f57c * l.ffe5))) / (t2bf * t2bf))), );}
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {let t2c2: f64 = (l.ffe4 / l.ff6e);let t2c3: f64 = (t2c2).sqrt();(l.f163f, l.f1640, l.f1641, l.f1642, l.f1643, l.f1644, ) = (t2c3, ((((l.ffe7 * l.ff6e) - (l.ffe4 * l.ff6f)) / (l.ff6e * l.ff6e)) / (2.0 * t2c3)), ((-((l.ffe4 * l.ff70) / (l.ff6e * l.ff6e))) / (2.0 * t2c3)), ((-((l.ffe4 * l.ff71) / (l.ff6e * l.ff6e))) / (2.0 * t2c3)), ((-((l.ffe4 * l.ff72) / (l.ff6e * l.ff6e))) / (2.0 * t2c3)), ((-((l.ffe4 * l.ff73) / (l.ff6e * l.ff6e))) / (2.0 * t2c3)), );}
        let t2c4: f64 = if l.f1639 <= 0.0 { 1.0 } else { 0.0 };l.f971 = t2c4;
        if (((l.f969 != 0.0) && (l.f96e != 0.0)) && (l.f971 != 0.0)) {(l.f165, l.f166, l.f167, l.f168, l.f169, l.f16a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f969 != 0.0) && (l.f96e != 0.0)) && (l.f971 == 0.0)) {let t2c5: f64 = (l.ff75 * l.f163f);let t2c6: f64 = (t2c5 / l.f1639);(l.f165, l.f166, l.f167, l.f168, l.f169, l.f16a, ) = (t2c6, (((((l.ff76 * l.f163f) + (l.ff75 * l.f1640)) * l.f1639) - (t2c5 * l.f163a)) / (l.f1639 * l.f1639)), (((((l.ff77 * l.f163f) + (l.ff75 * l.f1641)) * l.f1639) - (t2c5 * l.f163b)) / (l.f1639 * l.f1639)), (((((l.ff78 * l.f163f) + (l.ff75 * l.f1642)) * l.f1639) - (t2c5 * l.f163c)) / (l.f1639 * l.f1639)), (((((l.ff79 * l.f163f) + (l.ff75 * l.f1643)) * l.f1639) - (t2c5 * l.f163d)) / (l.f1639 * l.f1639)), (((((l.ff7a * l.f163f) + (l.ff75 * l.f1644)) * l.f1639) - (t2c5 * l.f163e)) / (l.f1639 * l.f1639)), );}
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {
            let (t2cd, t2ce, t2cf, t2d0, t2d1, t2d2,) = {
    if (l.f165 > 0.0) {
        let (t2c7, t2c8, t2c9, t2ca, t2cb, t2cc,) = {
            if (l.f165 < 1.0) {
                (l.f165, l.f166, l.f167, l.f168, l.f169, l.f16a,)
            } else {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2c7, t2c8, t2c9, t2ca, t2cb, t2cc,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.f165, l.f166, l.f167, l.f168, l.f169, l.f16a, ) = (t2cd, t2ce, t2cf, t2d0, t2d1, t2d2, );
        }
        if ((l.f969 != 0.0) && (l.f96e != 0.0)) {let t2d3: f64 = (l.f165 * l.f1639);let t2d4: f64 = (t2d3 / l.f163f);(l.ff74, l.ff7b, l.ff7c, l.ff7d, l.ff7e, l.ff7f, ) = (t2d4, (((((l.f166 * l.f1639) + (l.f165 * l.f163a)) * l.f163f) - (t2d3 * l.f1640)) / (l.f163f * l.f163f)), (((((l.f167 * l.f1639) + (l.f165 * l.f163b)) * l.f163f) - (t2d3 * l.f1641)) / (l.f163f * l.f163f)), (((((l.f168 * l.f1639) + (l.f165 * l.f163c)) * l.f163f) - (t2d3 * l.f1642)) / (l.f163f * l.f163f)), (((((l.f169 * l.f1639) + (l.f165 * l.f163d)) * l.f163f) - (t2d3 * l.f1643)) / (l.f163f * l.f163f)), (((((l.f16a * l.f1639) + (l.f165 * l.f163e)) * l.f163f) - (t2d3 * l.f1644)) / (l.f163f * l.f163f)), );}
        let t2d5: f64 = if (((p.p46 != 0.0) && (l.f122 > 0.0)) && (l.f1c5d > 0.0)) { 1.0 } else { 0.0 };l.f974 = t2d5;
        if (l.f974 != 0.0) {let t2d6: f64 = (4.0 * l.f38b);let t2d7: f64 = (t2d6 / l.f5f7);(l.f16e1, l.f16e2, l.f16e3, l.f16e4, l.f16e5, l.f16e6, ) = (t2d7, ((((4.0 * l.f38c) * l.f5f7) - (t2d6 * l.f5f8)) / (l.f5f7 * l.f5f7)), ((4.0 * l.f38d) / l.f5f7), ((4.0 * l.f38e) / l.f5f7), ((4.0 * l.f38f) / l.f5f7), ((4.0 * l.f390) / l.f5f7), );let t2d8: f64 = (l.f1db * l.f109f);(l.f16e1, l.f16e2, l.f16e3, l.f16e4, l.f16e5, l.f16e6, ) = (t2d8, (l.f1db * l.f10c6), 0.0, 0.0, 0.0, 0.0, );let t2d9: f64 = (l.f69 * l.fcc0);(l.f16e1, l.f16e2, l.f16e3, l.f16e4, l.f16e5, l.f16e6, ) = (t2d9, ((l.f6a * l.fcc0) + (l.f69 * l.fcc1)), ((l.f6b * l.fcc0) + (l.f69 * l.fcc2)), ((l.f6c * l.fcc0) + (l.f69 * l.fcc3)), ((l.f6d * l.fcc0) + (l.f69 * l.fcc4)), ((l.f6e * l.fcc0) + (l.f69 * l.fcc5)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t0: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };l.f684 = t0;l.f991 = 0.0;
        if (l.f684 != 0.0) {let t2: f64 = 1.0;l.f1b5 = t2;l.f1b6 = 0.0;}
        if (l.f684 == 0.0) {let t43: f64 = (-1.0);l.f1b5 = t43;l.f1b6 = 0.0;}
        let t18a: f64 = (8.8541878176e-12 * 11.8);l.f421 = t18a;l.f422 = 0.0;let t1f3: f64 = (273.15 + p.p38);l.f17ab = t1f3;l.f17ae = 0.0;l.f16ce = 0.0;l.f16cf = 0.0;let t209: f64 = if p.p944 > 0.5 { 1.0 } else { 0.0 };l.f992 = t209;l.f9d4 = 0.0;
        if (l.f992 != 0.0) {l.f16ce = 1.0;l.f16cf = 0.0;}
        if (l.f992 == 0.0) {l.f16ce = 0.0;l.f16cf = 0.0;}
        let t2da: f64 = (273.15 + p.p840);l.f17ac = t2da;l.f17ad = 0.0;let t1: f64 = (1.3806505e-23 / 1.6021918e-19);l.fea7 = t1;l.fea8 = 0.0;let t3: f64 = (l.fea7 * l.f17ac);l.f10dc = t3;l.f10dd = 0.0;let t4: f64 = (1.0 / l.f10dc);l.f10de = t4;l.f10df = 0.0;let t5: f64 = (0.000702 * l.f17ac);let t6: f64 = (t5 * l.f17ac);let t7: f64 = (-t6);let t8: f64 = (1108.0 + l.f17ac);let t9: f64 = (t7 / t8);l.f2cf = t9;l.f2d0 = 0.0;let ta: f64 = (p.p851 + l.f2cf);l.f108d = ta;l.f1090 = 0.0;let tb: f64 = (p.p852 + l.f2cf);l.f1099 = tb;l.f109c = 0.0;let tc: f64 = (p.p853 + l.f2cf);l.f1091 = tc;l.f1098 = 0.0;let t15: f64 = (1.0 - p.p848);l.fff6 = t15;l.fff9 = 0.0;let t22: f64 = (1.0 - p.p849);l.f1002 = t22;l.f1005 = 0.0;let t27: f64 = (1.0 - p.p850);l.fffa = t27;l.f1001 = 0.0;let t3a: f64 = (1.0 / l.fff6);l.f1006 = t3a;l.f1009 = 0.0;let t4a: f64 = (1.0 / l.f1002);l.f1012 = t4a;l.f1015 = 0.0;let t6b: f64 = (1.0 / l.fffa);l.f100a = t6b;l.f1011 = 0.0;let t97: f64 = (l.f421 / p.p842);l.f1ae5 = t97;l.f1ae8 = 0.0;let tab: f64 = (p.p860 * l.f421);let tac: f64 = (tab / p.p843);l.f1af9 = tac;l.f1afc = 0.0;let td2: f64 = (p.p861 * l.f421);let td3: f64 = (td2 / p.p844);l.f1ae9 = td3;l.f1aec = 0.0;let tee: f64 = (1.0 / l.f1ae5);l.f1aed = tee;l.f1af0 = 0.0;let t10e: f64 = (1.0 / l.f1af9);l.f1af5 = t10e;l.f1af8 = 0.0;let t140: f64 = (1.0 / l.f1ae9);l.f1af1 = t140;l.f1af4 = 0.0;let t154: f64 = (1.0 / p.p845);l.f1916 = t154;l.f1919 = 0.0;let t172: f64 = (1.0 / p.p846);l.f1926 = t172;l.f1929 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t192: f64 = (1.0 / p.p847);l.f1920 = t192;l.f1923 = 0.0;let t1b6: f64 = (1.0 / p.p841);let t1b7: f64 = (1.0 - t1b6);l.f76 = t1b7;l.f77 = 0.0;let t1f1: f64 = (1.0 / p.p877);l.f1942 = t1f1;l.f1945 = 0.0;let t1f2: f64 = (1.0 / p.p878);l.f1952 = t1f2;l.f1955 = 0.0;let t1f4: f64 = (1.0 / p.p879);(l.f1946, l.f194d, l.f194e, l.f194f, l.f1950, ) = (t1f4, 0.0, 0.0, 0.0, 0.0, );l.f1951 = 0.0;let t1f5: f64 = if ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0)) { 1.0 } else { 0.0 };l.f9d5 = t1f5;l.fa4e = 0.0;
        if (l.f9d5 != 0.0) {l.f16ca = 1.0;l.f16cd = 0.0;}
        if (l.f9d5 == 0.0) {l.f16ca = 0.0;l.f16cd = 0.0;}
        let t1f6: f64 = if l.f16ca == 1.0 { 1.0 } else { 0.0 };l.fa4f = t1f6;l.fac8 = 0.0;
        if (l.fa4f != 0.0) {
            let t1f7: f64 = (p.p844 * p.p883);
            let (t1f9,) = {
    if (t1f7 > 1e-18) {
        let t1f8: f64 = (p.p844 * p.p883);
        (t1f8,)
    } else {
        (1e-18,)
    }
};
            l.f1cd = t1f9;l.f1d0 = 0.0;
        }
        if (l.fa4f != 0.0) {
            let t1fa: f64 = (p.p847 * p.p884);
            let (t1fc,) = {
    if (t1fa > 0.05) {
        let t1fb: f64 = (p.p847 * p.p884);
        (t1fb,)
    } else {
        (0.05,)
    }
};
            l.f191a = t1fc;l.f191d = 0.0;
        }
        if (l.fa4f != 0.0) {
            let t1fd: f64 = (p.p850 * p.p885);
            let (t1ff,) = {
    if (t1fd > 0.05) {
        let t1fe: f64 = (p.p850 * p.p885);
        (t1fe,)
    } else {
        (0.05,)
    }
};
            let (t203,) = {
    if (t1ff < 0.95) {
        let t200: f64 = (p.p850 * p.p885);
        let (t202,) = {
            if (t200 > 0.05) {
                let t201: f64 = (p.p850 * p.p885);
                (t201,)
            } else {
                (0.05,)
            }
        };
        (t202,)
    } else {
        (0.95,)
    }
};
            l.f105d = t203;l.f1060 = 0.0;
        }
        if (l.fa4f != 0.0) {let t204: f64 = (p.p853 * p.p886);l.f1087 = t204;l.f108a = 0.0;let t205: f64 = (l.f1087 + l.f2cf);l.f1092 = t205;l.f1095 = 0.0;let t206: f64 = (1.0 - l.f105d);l.fffb = t206;l.fffe = 0.0;let t207: f64 = (1.0 / l.fffb);l.f100b = t207;l.f100e = 0.0;}
        let t208: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.fac9 = t208;l.fb44 = 0.0;
        if (l.fac9 != 0.0) {l.f1cb = p.p842;l.f1cc = 0.0;l.f1d3 = p.p843;l.f1d4 = 0.0;l.f1d1 = p.p844;l.f1d2 = 0.0;l.f1914 = p.p845;l.f1915 = 0.0;l.f1924 = p.p846;l.f1925 = 0.0;l.f191e = p.p847;l.f191f = 0.0;l.f1026 = p.p848;l.f1027 = 0.0;l.f1146 = p.p849;l.f1147 = 0.0;l.f1061 = p.p850;l.f1062 = 0.0;l.f1075 = p.p851;l.f1076 = 0.0;l.f109d = p.p852;l.f109e = 0.0;l.f108b = p.p853;l.f108c = 0.0;l.fd48 = p.p854;l.fd49 = 0.0;l.fd4c = p.p855;l.fd4d = 0.0;l.fd4a = p.p856;l.fd4b = 0.0;l.f1f3 = p.p857;l.f1f4 = 0.0;l.f1f7 = p.p858;l.f1f8 = 0.0;l.f1f5 = p.p859;l.f1f6 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fac9 != 0.0) {l.f1d47 = p.p860;l.f1d48 = 0.0;l.f1d45 = p.p861;l.f1d46 = 0.0;l.f20e = p.p862;l.f20f = 0.0;l.f212 = p.p863;l.f213 = 0.0;l.f210 = p.p864;l.f211 = 0.0;l.ff48 = p.p865;l.ff49 = 0.0;l.ff4c = p.p866;l.ff4d = 0.0;l.ff4a = p.p867;l.ff4b = 0.0;l.f16b = p.p868;l.f16c = 0.0;l.f16f = p.p869;l.f170 = 0.0;l.f16d = p.p870;l.f16e = 0.0;l.f4bf = p.p871;l.f4c0 = 0.0;l.f4c3 = p.p872;l.f4c4 = 0.0;l.f4c1 = p.p873;l.f4c2 = 0.0;l.f169e = p.p874;l.f169f = 0.0;l.f16a2 = p.p875;l.f16a3 = 0.0;l.f16a0 = p.p876;l.f16a1 = 0.0;l.f1932 = p.p877;l.f1933 = 0.0;l.f1956 = p.p878;l.f1957 = 0.0;l.f1940 = p.p879;l.f1941 = 0.0;l.f1028 = p.p880;l.f1029 = 0.0;l.f102c = p.p881;l.f102d = 0.0;l.f102a = p.p882;l.f102b = 0.0;l.f510 = p.p946;l.f511 = 0.0;l.f21 = p.p889;l.f22 = 0.0;l.f112 = p.p890;l.f113 = 0.0;l.f1f = p.p891;l.f20 = 0.0;l.f110 = p.p892;l.f111 = 0.0;l.f4e7 = p.p883;l.f4e8 = 0.0;l.f569 = p.p884;l.f56a = 0.0;l.f525 = p.p885;l.f526 = 0.0;l.f527 = p.p886;l.f528 = 0.0;l.f1ad1 = p.p887;l.f1ad2 = 0.0;l.fa7 = p.p888;l.fa8 = 0.0;}
        if (l.fac9 == 0.0) {l.f1cb = p.p893;l.f1cc = 0.0;l.f1d3 = p.p894;l.f1d4 = 0.0;l.f1d1 = p.p895;l.f1d2 = 0.0;l.f1914 = p.p896;l.f1915 = 0.0;l.f1924 = p.p897;l.f1925 = 0.0;l.f191e = p.p898;l.f191f = 0.0;l.f1026 = p.p899;l.f1027 = 0.0;l.f1146 = p.p900;l.f1147 = 0.0;l.f1061 = p.p901;l.f1062 = 0.0;l.f1075 = p.p902;l.f1076 = 0.0;l.f109d = p.p903;l.f109e = 0.0;l.f108b = p.p904;l.f108c = 0.0;l.fd48 = p.p905;l.fd49 = 0.0;l.fd4c = p.p906;l.fd4d = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fac9 == 0.0) {l.fd4a = p.p907;l.fd4b = 0.0;l.f1f3 = p.p908;l.f1f4 = 0.0;l.f1f7 = p.p909;l.f1f8 = 0.0;l.f1f5 = p.p910;l.f1f6 = 0.0;l.f1d47 = p.p911;l.f1d48 = 0.0;l.f1d45 = p.p912;l.f1d46 = 0.0;l.f20e = p.p913;l.f20f = 0.0;l.f212 = p.p914;l.f213 = 0.0;l.f210 = p.p915;l.f211 = 0.0;l.ff48 = p.p916;l.ff49 = 0.0;l.ff4c = p.p917;l.ff4d = 0.0;l.ff4a = p.p918;l.ff4b = 0.0;l.f16b = p.p919;l.f16c = 0.0;l.f16f = p.p920;l.f170 = 0.0;l.f16d = p.p921;l.f16e = 0.0;l.f4bf = p.p922;l.f4c0 = 0.0;l.f4c3 = p.p923;l.f4c4 = 0.0;l.f4c1 = p.p924;l.f4c2 = 0.0;l.f169e = p.p925;l.f169f = 0.0;l.f16a2 = p.p926;l.f16a3 = 0.0;l.f16a0 = p.p927;l.f16a1 = 0.0;l.f1932 = p.p928;l.f1933 = 0.0;l.f1956 = p.p929;l.f1957 = 0.0;l.f1940 = p.p930;l.f1941 = 0.0;l.f1028 = p.p931;l.f1029 = 0.0;l.f102c = p.p932;l.f102d = 0.0;l.f102a = p.p933;l.f102b = 0.0;l.f510 = p.p948;l.f511 = 0.0;l.f21 = p.p940;l.f22 = 0.0;l.f112 = p.p941;l.f113 = 0.0;l.f1f = p.p942;l.f20 = 0.0;l.f110 = p.p943;l.f111 = 0.0;l.f4e7 = p.p934;l.f4e8 = 0.0;l.f569 = p.p935;l.f56a = 0.0;l.f525 = p.p936;l.f526 = 0.0;l.f527 = p.p937;l.f528 = 0.0;l.f1ad1 = p.p938;l.f1ad2 = 0.0;l.fa7 = p.p939;l.fa8 = 0.0;}
        let td: f64 = (l.f1075 + l.f2cf);l.f108e = td;l.f108f = 0.0;let te: f64 = (l.f109d + l.f2cf);l.f109a = te;l.f109b = 0.0;let tf: f64 = (l.f108b + l.f2cf);l.f1096 = tf;l.f1097 = 0.0;let t10: f64 = (1.0 - l.f1026);l.fff7 = t10;l.fff8 = 0.0;let t11: f64 = (1.0 - l.f1146);l.f1003 = t11;l.f1004 = 0.0;let t12: f64 = (1.0 - l.f1061);l.ffff = t12;l.f1000 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t13: f64 = (1.0 / l.fff7);l.f1007 = t13;l.f1008 = 0.0;let t14: f64 = (1.0 / l.f1003);l.f1013 = t14;l.f1014 = 0.0;let t16: f64 = (1.0 / l.ffff);l.f100f = t16;l.f1010 = 0.0;let t17: f64 = (l.f421 / l.f1cb);l.f1ae6 = t17;l.f1ae7 = 0.0;let t18: f64 = (l.f1d47 * l.f421);let t19: f64 = (t18 / l.f1d3);l.f1afa = t19;l.f1afb = 0.0;let t1a: f64 = (l.f1d45 * l.f421);let t1b: f64 = (t1a / l.f1d1);l.f1aea = t1b;l.f1aeb = 0.0;let t1c: f64 = (1.0 / l.f1ae6);l.f1aee = t1c;l.f1aef = 0.0;let t1d: f64 = (1.0 / l.f1afa);l.f1af6 = t1d;l.f1af7 = 0.0;let t1e: f64 = (1.0 / l.f1aea);l.f1af2 = t1e;l.f1af3 = 0.0;let t1f: f64 = (1.0 / l.f1914);l.f1917 = t1f;l.f1918 = 0.0;let t20: f64 = (1.0 / l.f1924);l.f1927 = t20;l.f1928 = 0.0;let t21: f64 = (1.0 / l.f191e);l.f1921 = t21;l.f1922 = 0.0;let t23: f64 = (1.0 / l.f1932);l.f1943 = t23;l.f1944 = 0.0;let t24: f64 = (1.0 / l.f1956);l.f1953 = t24;l.f1954 = 0.0;let t25: f64 = (1.0 / l.f1940);(l.f1947, l.f1948, l.f1949, l.f194a, l.f194b, ) = (t25, 0.0, 0.0, 0.0, 0.0, );l.f194c = 0.0;let t26: f64 = if ((((l.f4e7 != 1.0) || (l.f569 != 1.0)) || (l.f525 != 1.0)) || (l.f527 != 1.0)) { 1.0 } else { 0.0 };l.fb45 = t26;l.fbbe = 0.0;
        if (l.fb45 != 0.0) {l.f16cb = 1.0;l.f16cc = 0.0;}
        if (l.fb45 == 0.0) {l.f16cb = 0.0;l.f16cc = 0.0;}
        let t28: f64 = if l.f16cb == 1.0 { 1.0 } else { 0.0 };l.fbbf = t28;l.fc38 = 0.0;
        if (l.fbbf != 0.0) {
            let t29: f64 = (l.f1d1 * l.f4e7);
            let (t2b,) = {
    if (t29 > 1e-18) {
        let t2a: f64 = (l.f1d1 * l.f4e7);
        (t2a,)
    } else {
        (1e-18,)
    }
};
            l.f1ce = t2b;l.f1cf = 0.0;
        }
        if (l.fbbf != 0.0) {
            let t2c: f64 = (l.f191e * l.f569);
            let (t2e,) = {
    if (t2c > 0.05) {
        let t2d: f64 = (l.f191e * l.f569);
        (t2d,)
    } else {
        (0.05,)
    }
};
            l.f191b = t2e;l.f191c = 0.0;
        }
        if (l.fbbf != 0.0) {
            let t2f: f64 = (l.f1061 * l.f525);
            let (t31,) = {
    if (t2f > 0.05) {
        let t30: f64 = (l.f1061 * l.f525);
        (t30,)
    } else {
        (0.05,)
    }
};
            let (t35,) = {
    if (t31 < 0.95) {
        let t32: f64 = (l.f1061 * l.f525);
        let (t34,) = {
            if (t32 > 0.05) {
                let t33: f64 = (l.f1061 * l.f525);
                (t33,)
            } else {
                (0.05,)
            }
        };
        (t34,)
    } else {
        (0.95,)
    }
};
            l.f105e = t35;l.f105f = 0.0;
        }
        if (l.fbbf != 0.0) {let t36: f64 = (l.f108b * l.f527);l.f1088 = t36;l.f1089 = 0.0;let t37: f64 = (l.f1088 + l.f2cf);l.f1093 = t37;l.f1094 = 0.0;let t38: f64 = (1.0 - l.f105e);l.fffc = t38;l.fffd = 0.0;let t39: f64 = (1.0 / l.fffc);l.f100c = t39;l.f100d = 0.0;}
        let t3b: f64 = ctx_temp;let t3c: f64 = (t3b + p.p55);let t3d: f64 = (t3c + p.p35);l.f17a1 = t3d;l.f17a2 = 0.0;let t3e: f64 = (l.f17a1 / l.f17ab);l.f1448 = t3e;l.f1449 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t3f: f64 = (l.f17a1 - l.f17ab);l.f279 = t3f;l.f2c0 = 0.0;let t40: f64 = (l.f17a1 * 1.3806505e-23);let t41: f64 = (t40 / 1.6021918e-19);l.f10c8 = t41;l.f10c9 = 0.0;let t42: f64 = (1.0 / l.f10c8);l.fe38 = t42;l.fe39 = 0.0;let t44: f64 = ctx_temp;let t45: f64 = (t44 + p.p55);let t46: f64 = (t45 + p.p35);let t47: f64 = (-250.0);let t48: f64 = (273.15 + t47);let t49: f64 = (t46).max(t48);l.f17a4 = t49;l.f17a5 = 0.0;let t4b: f64 = (l.f17a4 / l.f17ac);l.ff8 = t4b;l.ff9 = 0.0;let t4c: f64 = (l.fea7 * l.f17a4);l.f10d8 = t4c;l.f10d9 = 0.0;let t4d: f64 = (1.0 / l.f10d8);l.f10da = t4d;l.f10db = 0.0;let t4e: f64 = (0.000702 * l.f17a4);let t4f: f64 = (t4e * l.f17a4);let t50: f64 = (-t4f);let t51: f64 = (1108.0 + l.f17a4);let t52: f64 = (t50 / t51);l.f2cd = t52;l.f2ce = 0.0;let t53: f64 = (p.p851 + l.f2cd);l.f1077 = t53;l.f107a = 0.0;let t54: f64 = (p.p852 + l.f2cd);l.f1083 = t54;l.f1086 = 0.0;let t55: f64 = (p.p853 + l.f2cd);l.f107b = t55;l.f1082 = 0.0;let t56: f64 = (l.ff8).powf(1.5);let t57: f64 = (l.f108d * l.f10de);let t58: f64 = (l.f1077 * l.f10da);let t59: f64 = (t57 - t58);let t5a: f64 = (0.5 * t59);let t5b: f64 = (t5a).exp();let t5c: f64 = (t56 * t5b);l.f559 = t5c;l.f55c = 0.0;let t5d: f64 = (l.ff8).powf(1.5);let t5e: f64 = (l.f1099 * l.f10de);let t5f: f64 = (l.f1083 * l.f10da);let t60: f64 = (t5e - t5f);let t61: f64 = (0.5 * t60);let t62: f64 = (t61).exp();let t63: f64 = (t5d * t62);l.f565 = t63;l.f568 = 0.0;let t64: f64 = (l.ff8).powf(1.5);let t65: f64 = (l.f1091 * l.f10de);let t66: f64 = (l.f107b * l.f10da);let t67: f64 = (t65 - t66);let t68: f64 = (0.5 * t67);let t69: f64 = (t68).exp();let t6a: f64 = (t64 * t69);l.f55d = t6a;l.f564 = 0.0;let t6c: f64 = (p.p854 * l.f559);let t6d: f64 = (t6c * l.f559);l.fd40 = t6d;l.fd43 = 0.0;let t6e: f64 = (p.p855 * l.f565);let t6f: f64 = (t6e * l.f565);l.fd4e = t6f;l.fd51 = 0.0;let t70: f64 = (p.p856 * l.f55d);let t71: f64 = (t70 * l.f55d);l.fd44 = t71;l.fd47 = 0.0;let t72: f64 = (p.p845 * l.ff8);let t73: f64 = (2.0 * l.f10d8);let t74: f64 = (l.f559).ln();let t75: f64 = (t73 * t74);let t76: f64 = (t72 - t75);l.f182d = t76;l.f1830 = 0.0;let t77: f64 = (p.p846 * l.ff8);let t78: f64 = (2.0 * l.f10d8);let t79: f64 = (l.f565).ln();let t7a: f64 = (t78 * t79);let t7b: f64 = (t77 - t7a);l.f1839 = t7b;l.f183c = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t7c: f64 = (p.p847 * l.ff8);let t7d: f64 = (2.0 * l.f10d8);let t7e: f64 = (l.f55d).ln();let t7f: f64 = (t7d * t7e);let t80: f64 = (t7c - t7f);l.f1831 = t80;l.f1838 = 0.0;let t81: f64 = (0.05 - l.f182d);let t82: f64 = (t81 * l.f10da);let t83: f64 = (t82).exp();let t84: f64 = (1.0 + t83);let t85: f64 = (t84).ln();let t86: f64 = (l.f10d8 * t85);let t87: f64 = (l.f182d + t86);l.f18ec = t87;l.f18f3 = 0.0;let t88: f64 = (0.05 - l.f1839);let t89: f64 = (t88 * l.f10da);let t8a: f64 = (t89).exp();let t8b: f64 = (1.0 + t8a);let t8c: f64 = (t8b).ln();let t8d: f64 = (l.f10d8 * t8c);let t8e: f64 = (l.f1839 + t8d);l.f192a = t8e;l.f1931 = 0.0;let t8f: f64 = (0.05 - l.f1831);let t90: f64 = (t8f * l.f10da);let t91: f64 = (t90).exp();let t92: f64 = (1.0 + t91);let t93: f64 = (t92).ln();let t94: f64 = (l.f10d8 * t93);let t95: f64 = (l.f1831 + t94);l.f18f4 = t95;l.f18ff = 0.0;let t96: f64 = (1.0 / l.f18ec);l.f1900 = t96;l.f1903 = 0.0;let t98: f64 = (1.0 / l.f192a);l.f190c = t98;l.f190f = 0.0;let t99: f64 = (1.0 / l.f18f4);l.f1904 = t99;l.f190b = 0.0;let t9a: f64 = (p.p845 * l.f1900);let t9b: f64 = (t9a).powf(p.p848);let t9c: f64 = (p.p842 * t9b);l.f1bf = t9c;l.f1c2 = 0.0;let t9d: f64 = (p.p846 * l.f190c);let t9e: f64 = (t9d).powf(p.p849);let t9f: f64 = (p.p843 * t9e);l.f1d5 = t9f;l.f1d8 = 0.0;let ta0: f64 = (p.p847 * l.f1904);let ta1: f64 = (ta0).powf(p.p850);let ta2: f64 = (p.p844 * ta1);l.f1c3 = ta2;l.f1ca = 0.0;let ta3: f64 = (l.f1bf * l.f18ec);let ta4: f64 = (ta3 * l.f1006);l.f13d0 = ta4;l.f13d3 = 0.0;let ta5: f64 = (l.f1d5 * l.f192a);let ta6: f64 = (ta5 * l.f1012);l.f13dc = ta6;l.f13df = 0.0;let ta7: f64 = (l.f1c3 * l.f18f4);let ta8: f64 = (ta7 * l.f100a);l.f13d4 = ta8;l.f13db = 0.0;let ta9: f64 = (2.0 * l.f1bf);l.f13c0 = ta9;l.f13c3 = 0.0;let taa: f64 = (2.0 * l.f1d5);l.f13cc = taa;l.f13cf = 0.0;let tad: f64 = (2.0 * l.f1c3);l.f13c4 = tad;l.f13cb = 0.0;let tae: f64 = (0.5 * l.f1077);let taf: f64 = (tae).max(l.f10d8);l.f2c1 = taf;l.f2c4 = 0.0;let tb0: f64 = (0.5 * l.f1083);let tb1: f64 = (tb0).max(l.f10d8);l.f2c9 = tb1;l.f2cc = 0.0;let tb2: f64 = (0.5 * l.f107b);let tb3: f64 = (tb2).max(l.f10d8);l.f2c5 = tb3;l.f2c8 = 0.0;let tb4: f64 = (l.f2c1 * l.f10da);l.fec = tb4;l.fef = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tb5: f64 = (l.f2c9 * l.f10da);l.ff4 = tb5;l.ff7 = 0.0;let tb6: f64 = (l.f2c5 * l.f10da);l.ff0 = tb6;l.ff3 = 0.0;let tb7: f64 = (32.0 * p.p865);let tb8: f64 = (tb7 * 9.1093826e-31);let tb9: f64 = (tb8 * 1.6021918e-19);let tba: f64 = (l.f2c1 * l.f2c1);let tbb: f64 = (tba * l.f2c1);let tbc: f64 = (tb9 * tbb);let tbd: f64 = (tbc).sqrt();let tbe: f64 = (3.0 * 1.05457168e-34);let tbf: f64 = (tbd / tbe);l.f159 = tbf;l.f15c = 0.0;let tc0: f64 = (32.0 * p.p866);let tc1: f64 = (tc0 * 9.1093826e-31);let tc2: f64 = (tc1 * 1.6021918e-19);let tc3: f64 = (l.f2c9 * l.f2c9);let tc4: f64 = (tc3 * l.f2c9);let tc5: f64 = (tc2 * tc4);let tc6: f64 = (tc5).sqrt();let tc7: f64 = (3.0 * 1.05457168e-34);let tc8: f64 = (tc6 / tc7);l.f161 = tc8;l.f164 = 0.0;let tc9: f64 = (32.0 * p.p867);let tca: f64 = (tc9 * 9.1093826e-31);let tcb: f64 = (tca * 1.6021918e-19);let tcc: f64 = (l.f2c5 * l.f2c5);let tcd: f64 = (tcc * l.f2c5);let tce: f64 = (tcb * tcd);let tcf: f64 = (tce).sqrt();let td0: f64 = (3.0 * 1.05457168e-34);let td1: f64 = (tcf / td0);l.f15d = td1;l.f160 = 0.0;let td4: f64 = (l.f17a4 - l.f17ac);let td5: f64 = (p.p874 * td4);let td6: f64 = (1.0 + td5);let td7: f64 = (p.p871 * td6);l.f4af = td7;l.f4b2 = 0.0;let td8: f64 = (l.f17a4 - l.f17ac);let td9: f64 = (p.p875 * td8);let tda: f64 = (1.0 + td9);let tdb: f64 = (p.p872 * tda);l.f4c5 = tdb;l.f4c8 = 0.0;let tdc: f64 = (l.f17a4 - l.f17ac);let tdd: f64 = (p.p876 * tdc);let tde: f64 = (1.0 + tdd);let tdf: f64 = (p.p873 * tde);(l.f4b3, l.f4ba, l.f4bb, l.f4bc, l.f4bd, ) = (tdf, 0.0, 0.0, 0.0, 0.0, );l.f4be = 0.0;
        if (!(l.f4af > 0.0)) {l.f4af = 0.0;l.f4b2 = 0.0;}
        if (!(l.f4c5 > 0.0)) {l.f4c5 = 0.0;l.f4c8 = 0.0;}
        if (!(l.f4b3 > 0.0)) {(l.f4b3, l.f4ba, l.f4bb, l.f4bc, l.f4bd, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f4be = 0.0;}
        let te0: f64 = if l.f16ca == 1.0 { 1.0 } else { 0.0 };l.f9b0 = te0;l.f9bb = 0.0;
        if (l.f9b0 != 0.0) {let te1: f64 = (l.f1087 + l.f2cd);l.f107c = te1;l.f107f = 0.0;let te2: f64 = (l.ff8).powf(1.5);let te3: f64 = (l.f1092 * l.f10de);let te4: f64 = (l.f107c * l.f10da);let te5: f64 = (te3 - te4);let te6: f64 = (0.5 * te5);let te7: f64 = (te6).exp();let te8: f64 = (te2 * te7);l.f55e = te8;l.f561 = 0.0;let te9: f64 = (l.f191a * l.ff8);let tea: f64 = (2.0 * l.f10d8);let teb: f64 = (l.f55e).ln();let tec: f64 = (tea * teb);let ted: f64 = (te9 - tec);l.f1832 = ted;l.f1835 = 0.0;let tef: f64 = (0.05 - l.f1832);let tf0: f64 = (tef * l.f10da);let tf1: f64 = (tf0).exp();let tf2: f64 = (1.0 + tf1);let tf3: f64 = (tf2).ln();let tf4: f64 = (l.f10d8 * tf3);let tf5: f64 = (l.f1832 + tf4);l.f18f7 = tf5;l.f18fa = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        l: &mut StampLocals,
    ) {
        if (l.f9b0 != 0.0) {let tf6: f64 = (1.0 / l.f18f7);l.f1905 = tf6;l.f1908 = 0.0;let tf7: f64 = (l.f191a * l.f1905);let tf8: f64 = (tf7).powf(l.f105d);let tf9: f64 = (l.f1cd * tf8);l.f1c4 = tf9;l.f1c7 = 0.0;let tfa: f64 = (l.f1c4 * l.f18f7);let tfb: f64 = (tfa * l.f100b);l.f13d5 = tfb;l.f13d8 = 0.0;let tfc: f64 = (2.0 * l.f1c4);l.f13c5 = tfc;l.f13c8 = 0.0;}
        let tfd: f64 = (l.f1075 + l.f2cd);l.f1078 = tfd;l.f1079 = 0.0;let tfe: f64 = (l.f109d + l.f2cd);l.f1084 = tfe;l.f1085 = 0.0;let tff: f64 = (l.f108b + l.f2cd);l.f1080 = tff;l.f1081 = 0.0;let t100: f64 = (l.ff8).powf(1.5);let t101: f64 = (l.f108e * l.f10de);let t102: f64 = (l.f1078 * l.f10da);let t103: f64 = (t101 - t102);let t104: f64 = (0.5 * t103);let t105: f64 = (t104).exp();let t106: f64 = (t100 * t105);l.f55a = t106;l.f55b = 0.0;let t107: f64 = (l.ff8).powf(1.5);let t108: f64 = (l.f109a * l.f10de);let t109: f64 = (l.f1084 * l.f10da);let t10a: f64 = (t108 - t109);let t10b: f64 = (0.5 * t10a);let t10c: f64 = (t10b).exp();let t10d: f64 = (t107 * t10c);l.f566 = t10d;l.f567 = 0.0;let t10f: f64 = (l.ff8).powf(1.5);let t110: f64 = (l.f1096 * l.f10de);let t111: f64 = (l.f1080 * l.f10da);let t112: f64 = (t110 - t111);let t113: f64 = (0.5 * t112);let t114: f64 = (t113).exp();let t115: f64 = (t10f * t114);l.f562 = t115;l.f563 = 0.0;let t116: f64 = (l.fd48 * l.f55a);let t117: f64 = (t116 * l.f55a);l.fd41 = t117;l.fd42 = 0.0;let t118: f64 = (l.fd4c * l.f566);let t119: f64 = (t118 * l.f566);l.fd4f = t119;l.fd50 = 0.0;let t11a: f64 = (l.fd4a * l.f562);let t11b: f64 = (t11a * l.f562);l.fd45 = t11b;l.fd46 = 0.0;let t11c: f64 = (l.f1914 * l.ff8);let t11d: f64 = (2.0 * l.f10d8);let t11e: f64 = (l.f55a).ln();let t11f: f64 = (t11d * t11e);let t120: f64 = (t11c - t11f);l.f182e = t120;l.f182f = 0.0;let t121: f64 = (l.f1924 * l.ff8);let t122: f64 = (2.0 * l.f10d8);let t123: f64 = (l.f566).ln();let t124: f64 = (t122 * t123);let t125: f64 = (t121 - t124);l.f183a = t125;l.f183b = 0.0;let t126: f64 = (l.f191e * l.ff8);let t127: f64 = (2.0 * l.f10d8);let t128: f64 = (l.f562).ln();let t129: f64 = (t127 * t128);let t12a: f64 = (t126 - t129);l.f1836 = t12a;l.f1837 = 0.0;let t12b: f64 = (0.05 - l.f182e);let t12c: f64 = (t12b * l.f10da);let t12d: f64 = (t12c).exp();let t12e: f64 = (1.0 + t12d);let t12f: f64 = (t12e).ln();let t130: f64 = (l.f10d8 * t12f);let t131: f64 = (l.f182e + t130);l.f18f1 = t131;l.f18f2 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        l: &mut StampLocals,
    ) {
        let t132: f64 = (0.05 - l.f183a);let t133: f64 = (t132 * l.f10da);let t134: f64 = (t133).exp();let t135: f64 = (1.0 + t134);let t136: f64 = (t135).ln();let t137: f64 = (l.f10d8 * t136);let t138: f64 = (l.f183a + t137);l.f192f = t138;l.f1930 = 0.0;let t139: f64 = (0.05 - l.f1836);let t13a: f64 = (t139 * l.f10da);let t13b: f64 = (t13a).exp();let t13c: f64 = (1.0 + t13b);let t13d: f64 = (t13c).ln();let t13e: f64 = (l.f10d8 * t13d);let t13f: f64 = (l.f1836 + t13e);l.f18fd = t13f;l.f18fe = 0.0;let t141: f64 = (1.0 / l.f18f1);l.f1901 = t141;l.f1902 = 0.0;let t142: f64 = (1.0 / l.f192f);l.f190d = t142;l.f190e = 0.0;let t143: f64 = (1.0 / l.f18fd);l.f1909 = t143;l.f190a = 0.0;let t144: f64 = (l.f1914 * l.f1901);let t145: f64 = (t144).powf(l.f1026);let t146: f64 = (l.f1cb * t145);l.f1c0 = t146;l.f1c1 = 0.0;let t147: f64 = (l.f1924 * l.f190d);let t148: f64 = (t147).powf(l.f1146);let t149: f64 = (l.f1d3 * t148);l.f1d6 = t149;l.f1d7 = 0.0;let t14a: f64 = (l.f191e * l.f1909);let t14b: f64 = (t14a).powf(l.f1061);let t14c: f64 = (l.f1d1 * t14b);l.f1c8 = t14c;l.f1c9 = 0.0;let t14d: f64 = (l.f1c0 * l.f18f1);let t14e: f64 = (t14d * l.f1007);l.f13d1 = t14e;l.f13d2 = 0.0;let t14f: f64 = (l.f1d6 * l.f192f);let t150: f64 = (t14f * l.f1013);l.f13dd = t150;l.f13de = 0.0;let t151: f64 = (l.f1c8 * l.f18fd);let t152: f64 = (t151 * l.f100f);l.f13d9 = t152;l.f13da = 0.0;let t153: f64 = (2.0 * l.f1c0);l.f13c1 = t153;l.f13c2 = 0.0;let t155: f64 = (2.0 * l.f1d6);l.f13cd = t155;l.f13ce = 0.0;let t156: f64 = (2.0 * l.f1c8);l.f13c9 = t156;l.f13ca = 0.0;let t157: f64 = (0.5 * l.f1078);let t158: f64 = (t157).max(l.f10d8);l.f2c2 = t158;l.f2c3 = 0.0;let t159: f64 = (0.5 * l.f1084);let t15a: f64 = (t159).max(l.f10d8);l.f2ca = t15a;l.f2cb = 0.0;let t15b: f64 = (0.5 * l.f1080);let t15c: f64 = (t15b).max(l.f10d8);l.f2c6 = t15c;l.f2c7 = 0.0;let t15d: f64 = (l.f2c2 * l.f10da);l.fed = t15d;l.fee = 0.0;let t15e: f64 = (l.f2ca * l.f10da);l.ff5 = t15e;l.ff6 = 0.0;let t15f: f64 = (l.f2c6 * l.f10da);l.ff1 = t15f;l.ff2 = 0.0;let t160: f64 = (32.0 * l.ff48);let t161: f64 = (t160 * 9.1093826e-31);let t162: f64 = (t161 * 1.6021918e-19);let t163: f64 = (l.f2c2 * l.f2c2);let t164: f64 = (t163 * l.f2c2);let t165: f64 = (t162 * t164);let t166: f64 = (t165).sqrt();let t167: f64 = (3.0 * 1.05457168e-34);let t168: f64 = (t166 / t167);l.f15a = t168;l.f15b = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        l: &mut StampLocals,
    ) {
        let t169: f64 = (32.0 * l.ff4c);let t16a: f64 = (t169 * 9.1093826e-31);let t16b: f64 = (t16a * 1.6021918e-19);let t16c: f64 = (l.f2ca * l.f2ca);let t16d: f64 = (t16c * l.f2ca);let t16e: f64 = (t16b * t16d);let t16f: f64 = (t16e).sqrt();let t170: f64 = (3.0 * 1.05457168e-34);let t171: f64 = (t16f / t170);l.f162 = t171;l.f163 = 0.0;let t173: f64 = (32.0 * l.ff4a);let t174: f64 = (t173 * 9.1093826e-31);let t175: f64 = (t174 * 1.6021918e-19);let t176: f64 = (l.f2c6 * l.f2c6);let t177: f64 = (t176 * l.f2c6);let t178: f64 = (t175 * t177);let t179: f64 = (t178).sqrt();let t17a: f64 = (3.0 * 1.05457168e-34);let t17b: f64 = (t179 / t17a);l.f15e = t17b;l.f15f = 0.0;let t17c: f64 = (l.f17a4 - l.f17ac);let t17d: f64 = (l.f169e * t17c);let t17e: f64 = (1.0 + t17d);let t17f: f64 = (l.f4bf * t17e);l.f4b0 = t17f;l.f4b1 = 0.0;let t180: f64 = (l.f17a4 - l.f17ac);let t181: f64 = (l.f16a2 * t180);let t182: f64 = (1.0 + t181);let t183: f64 = (l.f4c3 * t182);l.f4c6 = t183;l.f4c7 = 0.0;let t184: f64 = (l.f17a4 - l.f17ac);let t185: f64 = (l.f16a0 * t184);let t186: f64 = (1.0 + t185);let t187: f64 = (l.f4c1 * t186);(l.f4b4, l.f4b5, l.f4b6, l.f4b7, l.f4b8, ) = (t187, 0.0, 0.0, 0.0, 0.0, );l.f4b9 = 0.0;
        if (!(l.f4b0 > 0.0)) {l.f4b0 = 0.0;l.f4b1 = 0.0;}
        if (!(l.f4c6 > 0.0)) {l.f4c6 = 0.0;l.f4c7 = 0.0;}
        if (!(l.f4b4 > 0.0)) {(l.f4b4, l.f4b5, l.f4b6, l.f4b7, l.f4b8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f4b9 = 0.0;}
        let t188: f64 = if l.f16cb == 1.0 { 1.0 } else { 0.0 };l.f9bc = t188;l.f9c7 = 0.0;
        if (l.f9bc != 0.0) {let t189: f64 = (l.f1088 + l.f2cd);l.f107d = t189;l.f107e = 0.0;let t18b: f64 = (l.ff8).powf(1.5);let t18c: f64 = (l.f1093 * l.f10de);let t18d: f64 = (l.f107d * l.f10da);let t18e: f64 = (t18c - t18d);let t18f: f64 = (0.5 * t18e);let t190: f64 = (t18f).exp();let t191: f64 = (t18b * t190);l.f55f = t191;l.f560 = 0.0;let t193: f64 = (l.f191b * l.ff8);let t194: f64 = (2.0 * l.f10d8);let t195: f64 = (l.f55f).ln();let t196: f64 = (t194 * t195);let t197: f64 = (t193 - t196);l.f1833 = t197;l.f1834 = 0.0;let t198: f64 = (0.05 - l.f1833);let t199: f64 = (t198 * l.f10da);let t19a: f64 = (t199).exp();let t19b: f64 = (1.0 + t19a);let t19c: f64 = (t19b).ln();let t19d: f64 = (l.f10d8 * t19c);let t19e: f64 = (l.f1833 + t19d);l.f18f8 = t19e;l.f18f9 = 0.0;let t19f: f64 = (1.0 / l.f18f8);l.f1906 = t19f;l.f1907 = 0.0;let t1a0: f64 = (l.f191b * l.f1906);let t1a1: f64 = (t1a0).powf(l.f105e);let t1a2: f64 = (l.f1ce * t1a1);l.f1c5 = t1a2;l.f1c6 = 0.0;let t1a3: f64 = (l.f1c5 * l.f18f8);let t1a4: f64 = (t1a3 * l.f100c);l.f13d6 = t1a4;l.f13d7 = 0.0;let t1a5: f64 = (2.0 * l.f1c5);l.f13c6 = t1a5;l.f13c7 = 0.0;}
        l.ffbf = 1.0;l.ffc0 = 0.0;l.fe57 = 1.0;l.fe58 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        l.fef1 = 0.0;l.fef2 = 0.0;l.f1afd = 0.0;l.f1b00 = 0.0;l.fee0 = p.p0;l.fee1 = 0.0;l.f1ad4 = p.p1;l.f1ad5 = 0.0;l.f1487 = p.p2;l.f1488 = 0.0;l.f1489 = p.p3;l.f148a = 0.0;l.f1493 = p.p4;l.f1494 = 0.0;l.f148b = p.p8;l.f148c = 0.0;l.f1b = p.p19;l.f1c = 0.0;l.ff0f = p.p20;l.ff10 = 0.0;l.fefb = p.p21;l.fefc = 0.0;l.f17 = p.p22;l.f18 = 0.0;l.ff0b = p.p23;l.ff0c = 0.0;l.fef7 = p.p24;l.fef8 = 0.0;l.fd0 = p.p25;l.fd1 = 0.0;l.f1122 = p.p26;l.f1123 = 0.0;l.f1d = p.p27;l.f1e = 0.0;l.f1049 = p.p28;l.f104a = 0.0;l.fe93 = p.p14;l.fe94 = 0.0;let t1a6: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };l.f9c8 = t1a6;l.f9d3 = 0.0;
        if (l.f9c8 != 0.0) {
            let (t1a7,) = {
    if (p.p9 > 1.0) {
        (p.p9,)
    } else {
        (1.0,)
    }
};
            l.ffbf = t1a7;l.ffc0 = 0.0;
        }
        if (l.f9c8 != 0.0) {let t1a8: f64 = (l.ffbf + 0.5);let t1a9: f64 = (t1a8).floor();l.ffbf = t1a9;l.ffc0 = 0.0;let t1aa: f64 = (1.0 / l.ffbf);l.fe57 = t1aa;l.fe58 = 0.0;}
        let t1ab: f64 = (l.f1ad4 * l.fe57);
        let (t1ad,) = {
    if (t1ab > 1e-9) {
        let t1ac: f64 = (l.f1ad4 * l.fe57);
        (t1ac,)
    } else {
        (1e-9,)
    }
};
        l.f1ad4 = t1ad;l.f1ad5 = 0.0;l.f148d = p.p5;l.f148e = 0.0;l.f148f = p.p6;l.f1490 = 0.0;l.f1491 = p.p7;l.f1492 = 0.0;let t1ae: f64 = (1e-6 / l.fee0);l.fdef = t1ae;l.fdf0 = 0.0;let t1af: f64 = (1e-6 / l.f1ad4);l.fe8f = t1af;l.fe90 = 0.0;let t1b0: f64 = (p.p190 * l.fdef);let t1b1: f64 = (1.0 + t1b0);let t1b2: f64 = (p.p189 * t1b1);let t1b3: f64 = (p.p191 * l.fe8f);let t1b4: f64 = (1.0 + t1b3);let t1b5: f64 = (t1b2 * t1b4);l.f266 = t1b5;l.f267 = 0.0;let t1b8: f64 = (p.p194 * l.fdef);let t1b9: f64 = (1.0 + t1b8);let t1ba: f64 = (p.p193 * t1b9);let t1bb: f64 = (p.p195 * l.fe8f);let t1bc: f64 = (1.0 + t1bb);let t1bd: f64 = (t1ba * t1bc);l.f2e8 = t1bd;l.f2e9 = 0.0;let t1be: f64 = (l.fee0 + l.f266);let t1bf: f64 = (2.0 * p.p192);let t1c0: f64 = (t1be - t1bf);
        let (t1c4,) = {
    if (t1c0 > 1e-9) {
        let t1c1: f64 = (l.fee0 + l.f266);let t1c2: f64 = (2.0 * p.p192);let t1c3: f64 = (t1c1 - t1c2);
        (t1c3,)
    } else {
        (1e-9,)
    }
};
        l.fef1 = t1c4;l.fef2 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t1c5: f64 = (l.f1ad4 + l.f2e8);let t1c6: f64 = (2.0 * p.p196);let t1c7: f64 = (t1c5 - t1c6);
        let (t1cb,) = {
    if (t1c7 > 1e-9) {
        let t1c8: f64 = (l.f1ad4 + l.f2e8);let t1c9: f64 = (2.0 * p.p196);let t1ca: f64 = (t1c8 - t1c9);
        (t1ca,)
    } else {
        (1e-9,)
    }
};
        l.f1afd = t1cb;l.f1b00 = 0.0;let t1cc: f64 = (1e-6 / l.fef1);l.fdf1 = t1cc;l.fdf4 = 0.0;let t1cd: f64 = (l.fdf1 * l.fdf1);l.fdf2 = t1cd;l.fdf3 = 0.0;let t1ce: f64 = (1e-6 / l.f1afd);l.fe91 = t1ce;l.fe92 = 0.0;let t1cf: f64 = (1.0 / l.fe91);l.fd8f = t1cf;l.fd90 = 0.0;let t1d0: f64 = (l.fdf1 * l.fe91);l.fd25 = t1d0;l.fd26 = 0.0;let t1d1: f64 = (1.0 / l.fd25);l.fd82 = t1d1;l.fd83 = 0.0;let t1d2: f64 = (l.fee0 + l.f266);let t1d3: f64 = (2.0 * p.p192);let t1d4: f64 = (t1d2 - t1d3);let t1d5: f64 = (t1d4 + p.p197);
        let (t1da,) = {
    if (t1d5 > 1e-9) {
        let t1d6: f64 = (l.fee0 + l.f266);let t1d7: f64 = (2.0 * p.p192);let t1d8: f64 = (t1d6 - t1d7);let t1d9: f64 = (t1d8 + p.p197);
        (t1d9,)
    } else {
        (1e-9,)
    }
};
        l.fef3 = t1da;l.fef4 = 0.0;let t1db: f64 = (l.f1ad4 + l.f2e8);let t1dc: f64 = (2.0 * p.p196);let t1dd: f64 = (t1db - t1dc);let t1de: f64 = (t1dd + p.p198);
        let (t1e3,) = {
    if (t1de > 1e-9) {
        let t1df: f64 = (l.f1ad4 + l.f2e8);let t1e0: f64 = (2.0 * p.p196);let t1e1: f64 = (t1df - t1e0);let t1e2: f64 = (t1e1 + p.p198);
        (t1e2,)
    } else {
        (1e-9,)
    }
};
        l.f1b01 = t1e3;l.f1b02 = 0.0;let t1e4: f64 = (l.f1b01 / 1e-6);l.fd91 = t1e4;l.fd92 = 0.0;let t1e5: f64 = (l.fee0 + l.f266);let t1e6: f64 = (t1e5 + p.p197);
        let (t1e9,) = {
    if (t1e6 > 1e-9) {
        let t1e7: f64 = (l.fee0 + l.f266);let t1e8: f64 = (t1e7 + p.p197);
        (t1e8,)
    } else {
        (1e-9,)
    }
};
        l.feef = t1e9;l.fef0 = 0.0;let t1ea: f64 = (l.f1ad4 + l.f2e8);let t1eb: f64 = (t1ea + p.p198);
        let (t1ee,) = {
    if (t1eb > 1e-9) {
        let t1ec: f64 = (l.f1ad4 + l.f2e8);let t1ed: f64 = (t1ec + p.p198);
        (t1ed,)
    } else {
        (1e-9,)
    }
};
        l.f1ad6 = t1ee;l.f1ad7 = 0.0;let t1ef: f64 = (l.feef / 1e-6);l.fd84 = t1ef;l.fd85 = 0.0;let t1f0: f64 = (l.f1ad6 / 1e-6);l.fd8d = t1f0;l.fd8e = 0.0;l.f19a4 = p.p56;l.f19a5 = 0.0;l.f16c0 = p.p57;l.f16c1 = 0.0;l.f1680 = p.p58;l.f1681 = 0.0;l.f17e1 = p.p59;l.f17e2 = 0.0;l.f41f = p.p60;l.f420 = 0.0;l.ffb7 = p.p61;l.ffb8 = 0.0;l.f5f4 = p.p62;l.f5f5 = 0.0;l.f1a72 = p.p63;l.f1a73 = 0.0;l.f3c9 = p.p64;l.f3ca = 0.0;l.f328 = p.p65;l.f329 = 0.0;l.ffcd = p.p66;l.ffce = 0.0;l.f17e7 = p.p67;l.f17e8 = 0.0;
    }
}
