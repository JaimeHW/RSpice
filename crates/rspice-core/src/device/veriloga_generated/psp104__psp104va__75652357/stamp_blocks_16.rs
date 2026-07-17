#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_256(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f787 != 0.0) && (l.f789 == 0.0)) && (l.f880 == 0.0)) && (l.f8a1 != 0.0)) {let t208: f64 = (1.0 - l.f15e8);let t209: f64 = (l.f1270 * t208);let t20a: f64 = (l.ff04 - l.f180f);let t20b: f64 = (l.f1260 * t20a);let t20c: f64 = (t209 + t20b);let t20d: f64 = (p.p30 * t20c);(l.f122a, l.f122d, l.f122e, l.f122f, l.f1230, l.f122b, l.f122c, ) = (t20d, (p.p30 * (l.f1270 * (-l.f15eb))), (p.p30 * ((l.f1270 * (-l.f15ec)) + (l.f1260 * (l.ff07 - l.f1812)))), (p.p30 * ((l.f1270 * (-l.f15ed)) + (l.f1260 * (l.ff08 - l.f1813)))), (p.p30 * (l.f1270 * (-l.f15ee))), (p.p30 * ((l.f1270 * (-l.f15e9)) + (l.f1260 * (l.ff05 - l.f1810)))), (p.p30 * ((l.f1270 * (-l.f15ea)) + (l.f1260 * (l.ff06 - l.f1811)))), );let t20e: f64 = (l.f1232 + l.f122a);(l.f1232, l.f1235, l.f1236, l.f1237, l.f1238, l.f1233, l.f1234, ) = (t20e, (l.f1235 + l.f122d), (l.f1236 + l.f122e), (l.f1237 + l.f122f), (l.f1238 + l.f1230), (l.f1233 + l.f122b), (l.f1234 + l.f122c), );}
        let t20f: f64 = if l.ff18 == 0.5 { 1.0 } else { 0.0 };l.f8a7 = t20f;
        if (((((l.f787 != 0.0) && (l.f789 == 0.0)) && (l.f880 == 0.0)) && (l.f8a1 == 0.0)) && (l.f8a7 != 0.0)) {let t210: f64 = (l.f17fb * l.f1717);let t211: f64 = (1.0 - t210);let t212: f64 = (t211).sqrt();(l.f15e8, l.f15eb, l.f15ec, l.f15ed, l.f15ee, l.f15e9, l.f15ea, ) = (t212, 0.0, ((-(l.f17fe * l.f1717)) / (2.0 * t212)), ((-(l.f17ff * l.f1717)) / (2.0 * t212)), 0.0, ((-(l.f17fc * l.f1717)) / (2.0 * t212)), ((-(l.f17fd * l.f1717)) / (2.0 * t212)), );}
        if (((((l.f787 != 0.0) && (l.f789 == 0.0)) && (l.f880 == 0.0)) && (l.f8a1 == 0.0)) && (l.f8a7 == 0.0)) {let t213: f64 = (l.f17fb * l.f1717);let t214: f64 = (1.0 - t213);let t215: f64 = (t214).powf(l.ff18);(l.f15e8, l.f15eb, l.f15ec, l.f15ed, l.f15ee, l.f15e9, l.f15ea, ) = (t215, 0.0, if 0.0 == 0.0 && ((l.ff18) as f64).is_finite() && ((l.ff18) as f64).fract() == 0.0 { if l.ff18 == 0.0 { 0.0 } else { (l.ff18 * ((t214).powf(l.ff18 - 1.0) * (-(l.f17fe * l.f1717)))) } } else { (t215 * (l.ff18 * ((-(l.f17fe * l.f1717)) / t214))) }, if 0.0 == 0.0 && ((l.ff18) as f64).is_finite() && ((l.ff18) as f64).fract() == 0.0 { if l.ff18 == 0.0 { 0.0 } else { (l.ff18 * ((t214).powf(l.ff18 - 1.0) * (-(l.f17ff * l.f1717)))) } } else { (t215 * (l.ff18 * ((-(l.f17ff * l.f1717)) / t214))) }, 0.0, if 0.0 == 0.0 && ((l.ff18) as f64).is_finite() && ((l.ff18) as f64).fract() == 0.0 { if l.ff18 == 0.0 { 0.0 } else { (l.ff18 * ((t214).powf(l.ff18 - 1.0) * (-(l.f17fc * l.f1717)))) } } else { (t215 * (l.ff18 * ((-(l.f17fc * l.f1717)) / t214))) }, if 0.0 == 0.0 && ((l.ff18) as f64).is_finite() && ((l.ff18) as f64).fract() == 0.0 { if l.ff18 == 0.0 { 0.0 } else { (l.ff18 * ((t214).powf(l.ff18 - 1.0) * (-(l.f17fd * l.f1717)))) } } else { (t215 * (l.ff18 * ((-(l.f17fd * l.f1717)) / t214))) }, );}
        if ((((l.f787 != 0.0) && (l.f789 == 0.0)) && (l.f880 == 0.0)) && (l.f8a1 == 0.0)) {let t216: f64 = (1.0 - l.f15e8);let t217: f64 = (l.f1273 * t216);let t218: f64 = (l.f1815 - l.f17fb);let t219: f64 = (l.f1263 * t218);let t21a: f64 = (t217 + t219);let t21b: f64 = (p.p30 * t21a);(l.f1232, l.f1235, l.f1236, l.f1237, l.f1238, l.f1233, l.f1234, ) = (t21b, (p.p30 * (l.f1273 * (-l.f15eb))), (p.p30 * ((l.f1273 * (-l.f15ec)) + (l.f1263 * (-l.f17fe)))), (p.p30 * ((l.f1273 * (-l.f15ed)) + (l.f1263 * (l.f1817 - l.f17ff)))), (p.p30 * (l.f1273 * (-l.f15ee))), (p.p30 * ((l.f1273 * (-l.f15e9)) + (l.f1263 * (-l.f17fc)))), (p.p30 * ((l.f1273 * (-l.f15ea)) + (l.f1263 * (l.f1816 - l.f17fd)))), );}
        if ((l.f787 != 0.0) && (l.f789 == 0.0)) {let t21c: f64 = (l.f16 * l.fce6);let t21d: f64 = (l.fe37 * l.fd21);let t21e: f64 = (t21c + t21d);let t21f: f64 = (l.fe25 * l.fd07);let t220: f64 = (t21e + t21f);(l.fcd7, l.fcda, l.fcdb, l.fcdc, l.fcdd, l.fcd8, l.fcd9, ) = (t220, (((l.f16 * l.fce9) + (l.fe37 * l.fd24)) + (l.fe25 * l.fd0a)), (((l.f16 * l.fcea) + (l.fe37 * l.fd25)) + (l.fe25 * l.fd0b)), (((l.f16 * l.fceb) + (l.fe37 * l.fd26)) + (l.fe25 * l.fd0c)), (((l.f16 * l.fcec) + (l.fe37 * l.fd27)) + (l.fe25 * l.fd0d)), (((l.f16 * l.fce7) + (l.fe37 * l.fd22)) + (l.fe25 * l.fd08)), (((l.f16 * l.fce8) + (l.fe37 * l.fd23)) + (l.fe25 * l.fd09)), );}
        let t221: f64 = if l.f1323 > 0.0 { 1.0 } else { 0.0 };l.f8a9 = t221;let t222: f64 = if l.f1297 > 0.0 { 1.0 } else { 0.0 };l.f8aa = t222;let t223: f64 = if l.f12cf > 0.0 { 1.0 } else { 0.0 };l.f8ac = t223;let t224: f64 = if l.f1295 > 0.0 { 1.0 } else { 0.0 };l.f8ad = t224;let t225: f64 = if l.f1293 > 0.0 { 1.0 } else { 0.0 };l.f8ae = t225;let t226: f64 = if l.f12c3 > 0.0 { 1.0 } else { 0.0 };l.f8af = t226;let t227: f64 = if l.f12c1 > 0.0 { 1.0 } else { 0.0 };l.f8b0 = t227;let t228: f64 = if l.f12db > 0.0 { 1.0 } else { 0.0 };l.f8b1 = t228;let t229: f64 = (l.f115e + l.f1098);let t22a: f64 = (t229 + l.f1112);let t22b: f64 = (-t22a);(l.f127c, l.f127d, l.f127e, l.f127f, l.f1280, ) = (t22b, (-((l.f1165 + l.f10a1) + l.f1119)), (-((l.f1166 + l.f10a2) + l.f111a)), (-((l.f1167 + l.f10a3) + l.f111b)), (-((l.f1168 + l.f10a4) + l.f111c)), );let t22c: f64 = (l.f1159 + l.f118d);(l.f1159, l.f115a, l.f115b, l.f115c, ) = (t22c, (l.f115a + l.f118e), (l.f115b + l.f118f), (l.f115c + l.f1190), );let t22d: f64 = (l.f1154 + l.f1182);(l.f1154, l.f1155, l.f1156, l.f1157, ) = (t22d, (l.f1155 + l.f1183), (l.f1156 + l.f1184), (l.f1157 + l.f1185), );let t22e: f64 = (l.f1a * l.f1222);let t22f: f64 = (l.fe3b * l.f124a);let t230: f64 = (t22e + t22f);let t231: f64 = (l.fe29 * l.f123a);let t232: f64 = (t230 + t231);(l.f1212, l.f1215, l.f1216, l.f1217, l.f1218, l.f1213, l.f1214, ) = (t232, (((l.f1a * l.f1225) + (l.fe3b * l.f124d)) + (l.fe29 * l.f123d)), (((l.f1a * l.f1226) + (l.fe3b * l.f124e)) + (l.fe29 * l.f123e)), (((l.f1a * l.f1227) + (l.fe3b * l.f124f)) + (l.fe29 * l.f123f)), (((l.f1a * l.f1228) + (l.fe3b * l.f1250)) + (l.fe29 * l.f1240)), (((l.f1a * l.f1223) + (l.fe3b * l.f124b)) + (l.fe29 * l.f123b)), (((l.f1a * l.f1224) + (l.fe3b * l.f124c)) + (l.fe29 * l.f123c)), );let t233: f64 = (l.f16 * l.f121a);let t234: f64 = (l.fe37 * l.f1242);let t235: f64 = (t233 + t234);let t236: f64 = (l.fe25 * l.f1232);let t237: f64 = (t235 + t236);(l.f120a, l.f120d, l.f120e, l.f120f, l.f1210, l.f120b, l.f120c, ) = (t237, (((l.f16 * l.f121d) + (l.fe37 * l.f1245)) + (l.fe25 * l.f1235)), (((l.f16 * l.f121e) + (l.fe37 * l.f1246)) + (l.fe25 * l.f1236)), (((l.f16 * l.f121f) + (l.fe37 * l.f1247)) + (l.fe25 * l.f1237)), (((l.f16 * l.f1220) + (l.fe37 * l.f1248)) + (l.fe25 * l.f1238)), (((l.f16 * l.f121b) + (l.fe37 * l.f1243)) + (l.fe25 * l.f1233)), (((l.f16 * l.f121c) + (l.fe37 * l.f1244)) + (l.fe25 * l.f1234)), );let t238: f64 = if l.f1323 < 0.0 { 1.0 } else { 0.0 };l.f8b2 = t238;
        if (l.f8b2 != 0.0) {(l.f152d, l.f152e, l.f152f, l.f1530, l.f1531, ) = (l.f1112, l.f1119, l.f111a, l.f111b, l.f111c, );(l.f1112, l.f1119, l.f111a, l.f111b, l.f111c, ) = (l.f127c, l.f127d, l.f127e, l.f127f, l.f1280, );(l.f127c, l.f127d, l.f127e, l.f127f, l.f1280, ) = (l.f152d, l.f152e, l.f152f, l.f1530, l.f1531, );}
        (l.f131e, l.f131f, l.f1320, l.f1321, l.f1322, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe82, l.fe83, l.fe84, l.fe85, l.fe86, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.fe93, l.fe94, l.fe95, l.fe96, l.fe97, ) = (1e-40, 0.0, 0.0, 0.0, 0.0, );(l.fe98, l.fe9e, l.fe9f, l.fea0, l.fea1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f145, l.f146, l.f147, l.f148, l.f149, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );let t239: f64 = (l.f1bd * l.f3ee);(l.f175, l.f176, l.f177, l.f178, l.f179, ) = (t239, ((l.f1be * l.f3ee) + (l.f1bd * l.f3ef)), ((l.f1bf * l.f3ee) + (l.f1bd * l.f3f0)), ((l.f1c0 * l.f3ee) + (l.f1bd * l.f3f1)), ((l.f1c1 * l.f3ee) + (l.f1bd * l.f3f2)), );(l.f1486, l.f1487, l.f1488, l.f1489, l.f148a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );(l.f148b, l.f148c, l.f148d, l.f148e, l.f148f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );let t23a: f64 = if ((l.f19ef > 0.0) && (l.ffd > 0.0)) { 1.0 } else { 0.0 };l.f8bc = t23a;let t23b: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };l.f8be = t23b;
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {let t23c: f64 = (l.f11a6 / l.f62);(l.fbec, l.fbed, l.fbee, l.fbef, l.fbf0, ) = (t23c, (((l.f11a7 * l.f62) - (l.f11a6 * l.f63)) / (l.f62 * l.f62)), (((l.f11a8 * l.f62) - (l.f11a6 * l.f64)) / (l.f62 * l.f62)), (((l.f11a9 * l.f62) - (l.f11a6 * l.f65)) / (l.f62 * l.f62)), (((l.f11aa * l.f62) - (l.f11a6 * l.f66)) / (l.f62 * l.f62)), );let t23d: f64 = (l.f11bd / l.f11a6);(l.f1512, l.f1513, l.f1514, l.f1515, l.f1516, ) = (t23d, (((l.f11be * l.f11a6) - (l.f11bd * l.f11a7)) / (l.f11a6 * l.f11a6)), (((l.f11bf * l.f11a6) - (l.f11bd * l.f11a8)) / (l.f11a6 * l.f11a6)), (((l.f11c0 * l.f11a6) - (l.f11bd * l.f11a9)) / (l.f11a6 * l.f11a6)), (((l.f11c1 * l.f11a6) - (l.f11bd * l.f11aa)) / (l.f11a6 * l.f11a6)), );let t23e: f64 = (0.5 * 0.16666666666666666);let t23f: f64 = (l.f303 / l.fbec);let t240: f64 = (t23e * t23f);(l.f14bd, l.f14be, l.f14bf, l.f14c0, l.f14c1, ) = (t240, (t23e * (((l.f304 * l.fbec) - (l.f303 * l.fbed)) / (l.fbec * l.fbec))), (t23e * (((l.f305 * l.fbec) - (l.f303 * l.fbee)) / (l.fbec * l.fbec))), (t23e * (((l.f306 * l.fbec) - (l.f303 * l.fbef)) / (l.fbec * l.fbec))), (t23e * (((l.f307 * l.fbec) - (l.f303 * l.fbf0)) / (l.fbec * l.fbec))), );let t241: f64 = (l.f14bd * l.f14bd);(l.f1517, l.f1518, l.f1519, l.f151a, l.f151b, ) = (t241, ((l.f14be * l.f14bd) + (l.f14bd * l.f14be)), ((l.f14bf * l.f14bd) + (l.f14bd * l.f14bf)), ((l.f14c0 * l.f14bd) + (l.f14bd * l.f14c0)), ((l.f14c1 * l.f14bd) + (l.f14bd * l.f14c1)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_257(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {let t242: f64 = (l.fbec / l.fc13);let t243: f64 = (t242 - 1.0);(l.f128e, l.f128f, l.f1290, l.f1291, l.f1292, ) = (t243, (((l.fbed * l.fc13) - (l.fbec * l.fc14)) / (l.fc13 * l.fc13)), (((l.fbee * l.fc13) - (l.fbec * l.fc15)) / (l.fc13 * l.fc13)), (((l.fbef * l.fc13) - (l.fbec * l.fc16)) / (l.fc13 * l.fc13)), (((l.fbf0 * l.fc13) - (l.fbec * l.fc17)) / (l.fc13 * l.fc13)), );}
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {
            let t244: f64 = (l.f128e * l.f1517);let t245: f64 = (12.0 * t244);let t246: f64 = (1.0 - t245);
            let (t24a, t24b, t24c, t24d, t24e,) = {
    if (t246 > 1e-20) {
        let t247: f64 = (l.f128e * l.f1517);let t248: f64 = (12.0 * t247);let t249: f64 = (1.0 - t248);
        (t249, (-(12.0 * ((l.f128f * l.f1517) + (l.f128e * l.f1518)))), (-(12.0 * ((l.f1290 * l.f1517) + (l.f128e * l.f1519)))), (-(12.0 * ((l.f1291 * l.f1517) + (l.f128e * l.f151a)))), (-(12.0 * ((l.f1292 * l.f1517) + (l.f128e * l.f151b)))),)
    } else {
        (1e-20, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.fe13, l.fe14, l.fe15, l.fe16, l.fe17, ) = (t24a, t24b, t24c, t24d, t24e, );
        }
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {let t24f: f64 = (l.fe13 * l.fe13);let t250: f64 = (1.0 / t24f);(l.fe18, l.fe19, l.fe1a, l.fe1b, l.fe1c, ) = (t250, (-(((l.fe14 * l.fe13) + (l.fe13 * l.fe14)) / (t24f * t24f))), (-(((l.fe15 * l.fe13) + (l.fe13 * l.fe15)) / (t24f * t24f))), (-(((l.fe16 * l.fe13) + (l.fe13 * l.fe16)) / (t24f * t24f))), (-(((l.fe17 * l.fe13) + (l.fe13 * l.fe17)) / (t24f * t24f))), );let t251: f64 = (l.ffd * l.f11a6);let t252: f64 = (t251 * l.fbe3);(l.f4f5, l.f4f6, l.f4f7, l.f4f8, l.f4f9, ) = (t252, (((l.ffd * l.f11a7) * l.fbe3) + (t251 * l.fbe4)), (((l.ffd * l.f11a8) * l.fbe3) + (t251 * l.fbe5)), (((l.ffd * l.f11a9) * l.fbe3) + (t251 * l.fbe6)), (((l.ffd * l.f11aa) * l.fbe3) + (t251 * l.fbe7)), );let t253: f64 = (12.0 * l.f1517);let t254: f64 = (l.f1512 + t253);let t255: f64 = (1.0 + l.f1512);let t256: f64 = (t255 * l.f1517);let t257: f64 = (t256 * l.f128e);let t258: f64 = (24.0 * t257);let t259: f64 = (t254 - t258);(l.fe82, l.fe83, l.fe84, l.fe85, l.fe86, ) = (t259, ((l.f1513 + (12.0 * l.f1518)) - (24.0 * ((((l.f1513 * l.f1517) + (t255 * l.f1518)) * l.f128e) + (t256 * l.f128f)))), ((l.f1514 + (12.0 * l.f1519)) - (24.0 * ((((l.f1514 * l.f1517) + (t255 * l.f1519)) * l.f128e) + (t256 * l.f1290)))), ((l.f1515 + (12.0 * l.f151a)) - (24.0 * ((((l.f1515 * l.f1517) + (t255 * l.f151a)) * l.f128e) + (t256 * l.f1291)))), ((l.f1516 + (12.0 * l.f151b)) - (24.0 * ((((l.f1516 * l.f1517) + (t255 * l.f151b)) * l.f128e) + (t256 * l.f1292)))), );}
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {
            let (t25a, t25b, t25c, t25d, t25e,) = {
    if (l.fe82 > 1e-40) {
        (l.fe82, l.fe83, l.fe84, l.fe85, l.fe86,)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.fe82, l.fe83, l.fe84, l.fe85, l.fe86, ) = (t25a, t25b, t25c, t25d, t25e, );
        }
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {let t25f: f64 = (l.f4f5 * l.fe18);let t260: f64 = (t25f * l.fe82);(l.fe82, l.fe83, l.fe84, l.fe85, l.fe86, ) = (t260, ((((l.f4f6 * l.fe18) + (l.f4f5 * l.fe19)) * l.fe82) + (t25f * l.fe83)), ((((l.f4f7 * l.fe18) + (l.f4f5 * l.fe1a)) * l.fe82) + (t25f * l.fe84)), ((((l.f4f8 * l.fe18) + (l.f4f5 * l.fe1b)) * l.fe82) + (t25f * l.fe85)), ((((l.f4f9 * l.fe18) + (l.f4f5 * l.fe1c)) * l.fe82) + (t25f * l.fe86)), );}
        let t261: f64 = if l.f4a9 > 0.0 { 1.0 } else { 0.0 };l.f8bf = t261;
        if (((l.f8bc != 0.0) && (l.f8be != 0.0)) && (l.f8bf != 0.0)) {let t262: f64 = (l.f15ae / l.f57e);(l.f1581, l.f1582, l.f1583, l.f1584, l.f1585, ) = (t262, (((l.f15af * l.f57e) - (l.f15ae * l.f57f)) / (l.f57e * l.f57e)), (((l.f15b0 * l.f57e) - (l.f15ae * l.f580)) / (l.f57e * l.f57e)), (((l.f15b1 * l.f57e) - (l.f15ae * l.f581)) / (l.f57e * l.f57e)), (((l.f15b2 * l.f57e) - (l.f15ae * l.f582)) / (l.f57e * l.f57e)), );let t263: f64 = (l.f1581 * l.f1581);let t264: f64 = (t263 * l.f303);let t265: f64 = (t264 * l.f303);(l.f1bde, l.f1bdf, l.f1be0, l.f1be1, l.f1be2, ) = (t265, ((((((l.f1582 * l.f1581) + (l.f1581 * l.f1582)) * l.f303) + (t263 * l.f304)) * l.f303) + (t264 * l.f304)), ((((((l.f1583 * l.f1581) + (l.f1581 * l.f1583)) * l.f303) + (t263 * l.f305)) * l.f303) + (t264 * l.f305)), ((((((l.f1584 * l.f1581) + (l.f1581 * l.f1584)) * l.f303) + (t263 * l.f306)) * l.f303) + (t264 * l.f306)), ((((((l.f1585 * l.f1581) + (l.f1581 * l.f1585)) * l.f303) + (t263 * l.f307)) * l.f303) + (t264 * l.f307)), );}
        let t266: f64 = (-1.0);let t267: f64 = if l.f193 == t266 { 1.0 } else { 0.0 };l.f8c0 = t267;
        if ((((l.f8bc != 0.0) && (l.f8be != 0.0)) && (l.f8bf != 0.0)) && (l.f8c0 != 0.0)) {let t268: f64 = (l.f1581 * l.f303);let t269: f64 = (1.0 + t268);let t26a: f64 = (l.f1bde / t269);(l.f1bde, l.f1bdf, l.f1be0, l.f1be1, l.f1be2, ) = (t26a, (((l.f1bdf * t269) - (l.f1bde * ((l.f1582 * l.f303) + (l.f1581 * l.f304)))) / (t269 * t269)), (((l.f1be0 * t269) - (l.f1bde * ((l.f1583 * l.f303) + (l.f1581 * l.f305)))) / (t269 * t269)), (((l.f1be1 * t269) - (l.f1bde * ((l.f1584 * l.f303) + (l.f1581 * l.f306)))) / (t269 * t269)), (((l.f1be2 * t269) - (l.f1bde * ((l.f1585 * l.f303) + (l.f1581 * l.f307)))) / (t269 * t269)), );}
        if (((l.f8bc != 0.0) && (l.f8be != 0.0)) && (l.f8bf != 0.0)) {let t26b: f64 = (2.0 * l.f1bde);let t26c: f64 = (1.0 + t26b);let t26d: f64 = (t26c).sqrt();let t26e: f64 = (1.0 + t26d);let t26f: f64 = (l.f57e * t26e);let t270: f64 = (0.5 * t26f);(l.fbdd, l.fbde, l.fbdf, l.fbe0, l.fbe1, ) = (t270, (0.5 * ((l.f57f * t26e) + (l.f57e * ((2.0 * l.f1bdf) / (2.0 * t26d))))), (0.5 * ((l.f580 * t26e) + (l.f57e * ((2.0 * l.f1be0) / (2.0 * t26d))))), (0.5 * ((l.f581 * t26e) + (l.f57e * ((2.0 * l.f1be1) / (2.0 * t26d))))), (0.5 * ((l.f582 * t26e) + (l.f57e * ((2.0 * l.f1be2) / (2.0 * t26d))))), );let t271: f64 = (l.fbdd * l.fe13);let t272: f64 = (l.f57e / t271);(l.f560, l.f561, l.f562, l.f563, l.f564, ) = (t272, (((l.f57f * t271) - (l.f57e * ((l.fbde * l.fe13) + (l.fbdd * l.fe14)))) / (t271 * t271)), (((l.f580 * t271) - (l.f57e * ((l.fbdf * l.fe13) + (l.fbdd * l.fe15)))) / (t271 * t271)), (((l.f581 * t271) - (l.f57e * ((l.fbe0 * l.fe13) + (l.fbdd * l.fe16)))) / (t271 * t271)), (((l.f582 * t271) - (l.f57e * ((l.fbe1 * l.fe13) + (l.fbdd * l.fe17)))) / (t271 * t271)), );let t273: f64 = (l.f41f * l.fc4b);let t274: f64 = (t273 * l.f178e);let t275: f64 = (t274 * l.f560);let t276: f64 = (t275 * l.f560);(l.f131e, l.f131f, l.f1320, l.f1321, l.f1322, ) = (t276, (((((((l.f41f * l.fc4c) * l.f178e) + (t273 * l.f178f)) * l.f560) + (t274 * l.f561)) * l.f560) + (t275 * l.f561)), (((((((l.f41f * l.fc4d) * l.f178e) + (t273 * l.f1790)) * l.f560) + (t274 * l.f562)) * l.f560) + (t275 * l.f562)), (((((((l.f41f * l.fc4e) * l.f178e) + (t273 * l.f1791)) * l.f560) + (t274 * l.f563)) * l.f560) + (t275 * l.f563)), (((((((l.f41f * l.fc4f) * l.f178e) + (t273 * l.f1792)) * l.f560) + (t274 * l.f564)) * l.f560) + (t275 * l.f564)), );let t277: f64 = (l.f131e / l.ff01);let t278: f64 = (l.fe82 + t277);(l.fe82, l.fe83, l.fe84, l.fe85, l.fe86, ) = (t278, (l.fe83 + (l.f131f / l.ff01)), (l.fe84 + (l.f1320 / l.ff01)), (l.fe85 + (l.f1321 / l.ff01)), (l.fe86 + (l.f1322 / l.ff01)), );}
        if ((l.f8bc != 0.0) && (l.f8be != 0.0)) {let t279: f64 = (l.ff00 * l.fe82);let t27a: f64 = (t279).sqrt();(l.f1486, l.f1487, l.f1488, l.f1489, l.f148a, ) = (t27a, ((l.ff00 * l.fe83) / (2.0 * t27a)), ((l.ff00 * l.fe84) / (2.0 * t27a)), ((l.ff00 * l.fe85) / (2.0 * t27a)), ((l.ff00 * l.fe86) / (2.0 * t27a)), );}
        let t27b: f64 = if ((((p.p50 == 1.0) && (l.ff00 > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };l.f8c1 = t27b;
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {let t27c: f64 = (l.f1512 / 12.0);let t27d: f64 = (l.f1512 + 0.2);let t27e: f64 = (12.0 * l.f1517);let t27f: f64 = (t27d - t27e);let t280: f64 = (l.f1517 * t27f);let t281: f64 = (t27c - t280);let t282: f64 = (l.f1512 + 1.0);let t283: f64 = (12.0 * l.f1517);let t284: f64 = (t282 - t283);let t285: f64 = (l.f1517 * t284);let t286: f64 = (t285 * l.f128e);let t287: f64 = (1.6 * t286);let t288: f64 = (t281 - t287);(l.fe93, l.fe94, l.fe95, l.fe96, l.fe97, ) = (t288, (((l.f1513 / 12.0) - ((l.f1518 * t27f) + (l.f1517 * (l.f1513 - (12.0 * l.f1518))))) - (1.6 * ((((l.f1518 * t284) + (l.f1517 * (l.f1513 - (12.0 * l.f1518)))) * l.f128e) + (t285 * l.f128f)))), (((l.f1514 / 12.0) - ((l.f1519 * t27f) + (l.f1517 * (l.f1514 - (12.0 * l.f1519))))) - (1.6 * ((((l.f1519 * t284) + (l.f1517 * (l.f1514 - (12.0 * l.f1519)))) * l.f128e) + (t285 * l.f1290)))), (((l.f1515 / 12.0) - ((l.f151a * t27f) + (l.f1517 * (l.f1515 - (12.0 * l.f151a))))) - (1.6 * ((((l.f151a * t284) + (l.f1517 * (l.f1515 - (12.0 * l.f151a)))) * l.f128e) + (t285 * l.f1291)))), (((l.f1516 / 12.0) - ((l.f151b * t27f) + (l.f1517 * (l.f1516 - (12.0 * l.f151b))))) - (1.6 * ((((l.f151b * t284) + (l.f1517 * (l.f1516 - (12.0 * l.f151b)))) * l.f128e) + (t285 * l.f1292)))), );}
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {
            let (t289, t28a, t28b, t28c, t28d,) = {
    if (l.fe93 > 1e-40) {
        (l.fe93, l.fe94, l.fe95, l.fe96, l.fe97,)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.fe93, l.fe94, l.fe95, l.fe96, l.fe97, ) = (t289, t28a, t28b, t28c, t28d, );
        }
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {let t28e: f64 = (l.fe18 / l.f4f5);let t28f: f64 = (t28e * l.fe93);(l.fe93, l.fe94, l.fe95, l.fe96, l.fe97, ) = (t28f, (((((l.fe19 * l.f4f5) - (l.fe18 * l.f4f6)) / (l.f4f5 * l.f4f5)) * l.fe93) + (t28e * l.fe94)), (((((l.fe1a * l.f4f5) - (l.fe18 * l.f4f7)) / (l.f4f5 * l.f4f5)) * l.fe93) + (t28e * l.fe95)), (((((l.fe1b * l.f4f5) - (l.fe18 * l.f4f8)) / (l.f4f5 * l.f4f5)) * l.fe93) + (t28e * l.fe96)), (((((l.fe1c * l.f4f5) - (l.fe18 * l.f4f9)) / (l.f4f5 * l.f4f5)) * l.fe93) + (t28e * l.fe97)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_258(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {let t290: f64 = (l.fe18 * l.f14bd);let t291: f64 = (12.0 * l.f1517);let t292: f64 = (1.0 - t291);let t293: f64 = (19.2 * l.f1517);let t294: f64 = (l.f1512 + t293);let t295: f64 = (l.f1512 * l.f1517);let t296: f64 = (12.0 * t295);let t297: f64 = (t294 - t296);let t298: f64 = (t297 * l.f128e);let t299: f64 = (t292 - t298);let t29a: f64 = (t290 * t299);(l.fe99, l.fe9a, l.fe9b, l.fe9c, l.fe9d, ) = (t29a, ((((l.fe19 * l.f14bd) + (l.fe18 * l.f14be)) * t299) + (t290 * ((-(12.0 * l.f1518)) - ((((l.f1513 + (19.2 * l.f1518)) - (12.0 * ((l.f1513 * l.f1517) + (l.f1512 * l.f1518)))) * l.f128e) + (t297 * l.f128f))))), ((((l.fe1a * l.f14bd) + (l.fe18 * l.f14bf)) * t299) + (t290 * ((-(12.0 * l.f1519)) - ((((l.f1514 + (19.2 * l.f1519)) - (12.0 * ((l.f1514 * l.f1517) + (l.f1512 * l.f1519)))) * l.f128e) + (t297 * l.f1290))))), ((((l.fe1b * l.f14bd) + (l.fe18 * l.f14c0)) * t299) + (t290 * ((-(12.0 * l.f151a)) - ((((l.f1515 + (19.2 * l.f151a)) - (12.0 * ((l.f1515 * l.f1517) + (l.f1512 * l.f151a)))) * l.f128e) + (t297 * l.f1291))))), ((((l.fe1c * l.f14bd) + (l.fe18 * l.f14c1)) * t299) + (t290 * ((-(12.0 * l.f151b)) - ((((l.f1516 + (19.2 * l.f151b)) - (12.0 * ((l.f1516 * l.f1517) + (l.f1512 * l.f151b)))) * l.f128e) + (t297 * l.f1292))))), );let t29b: f64 = (l.fbd3 * l.fbd3);let t29c: f64 = (t29b * l.f1bd);let t29d: f64 = (t29c * l.f3ee);let t29e: f64 = (l.f584 * l.f584);let t29f: f64 = (t29d / t29e);(l.f175, l.f176, l.f177, l.f178, l.f179, ) = (t29f, (((((((((l.fbd4 * l.fbd3) + (l.fbd3 * l.fbd4)) * l.f1bd) + (t29b * l.f1be)) * l.f3ee) + (t29c * l.f3ef)) * t29e) - (t29d * ((l.f585 * l.f584) + (l.f584 * l.f585)))) / (t29e * t29e)), (((((((((l.fbd5 * l.fbd3) + (l.fbd3 * l.fbd5)) * l.f1bd) + (t29b * l.f1bf)) * l.f3ee) + (t29c * l.f3f0)) * t29e) - (t29d * ((l.f586 * l.f584) + (l.f584 * l.f586)))) / (t29e * t29e)), (((((((((l.fbd6 * l.fbd3) + (l.fbd3 * l.fbd6)) * l.f1bd) + (t29b * l.f1c0)) * l.f3ee) + (t29c * l.f3f1)) * t29e) - (t29d * ((l.f587 * l.f584) + (l.f584 * l.f587)))) / (t29e * t29e)), (((((((((l.fbd7 * l.fbd3) + (l.fbd3 * l.fbd7)) * l.f1bd) + (t29b * l.f1c1)) * l.f3ee) + (t29c * l.f3f2)) * t29e) - (t29d * ((l.f588 * l.f584) + (l.f584 * l.f588)))) / (t29e * t29e)), );}
        let t2a0: f64 = if l.f4a9 > 0.0 { 1.0 } else { 0.0 };l.f8c3 = t2a0;
        if (((l.f8bc != 0.0) && (l.f8c1 != 0.0)) && (l.f8c3 != 0.0)) {let t2a1: f64 = (12.0 * l.f1517);let t2a2: f64 = (1.0 + t2a1);let t2a3: f64 = (l.f131e * t2a2);let t2a4: f64 = (12.0 * l.f4f5);let t2a5: f64 = (t2a4 * l.f4f5);let t2a6: f64 = (t2a5 * l.ff01);let t2a7: f64 = (t2a3 / t2a6);let t2a8: f64 = (l.fe93 + t2a7);(l.fe93, l.fe94, l.fe95, l.fe96, l.fe97, ) = (t2a8, (l.fe94 + (((((l.f131f * t2a2) + (l.f131e * (12.0 * l.f1518))) * t2a6) - (t2a3 * ((((12.0 * l.f4f6) * l.f4f5) + (t2a4 * l.f4f6)) * l.ff01))) / (t2a6 * t2a6))), (l.fe95 + (((((l.f1320 * t2a2) + (l.f131e * (12.0 * l.f1519))) * t2a6) - (t2a3 * ((((12.0 * l.f4f7) * l.f4f5) + (t2a4 * l.f4f7)) * l.ff01))) / (t2a6 * t2a6))), (l.fe96 + (((((l.f1321 * t2a2) + (l.f131e * (12.0 * l.f151a))) * t2a6) - (t2a3 * ((((12.0 * l.f4f8) * l.f4f5) + (t2a4 * l.f4f8)) * l.ff01))) / (t2a6 * t2a6))), (l.fe97 + (((((l.f1322 * t2a2) + (l.f131e * (12.0 * l.f151b))) * t2a6) - (t2a3 * ((((12.0 * l.f4f9) * l.f4f5) + (t2a4 * l.f4f9)) * l.ff01))) / (t2a6 * t2a6))), );let t2a9: f64 = (l.f131e * l.f14bd);let t2aa: f64 = (1.0 + l.f128e);let t2ab: f64 = (t2a9 * t2aa);let t2ac: f64 = (l.f4f5 * l.ff01);let t2ad: f64 = (t2ab / t2ac);let t2ae: f64 = (l.fe99 - t2ad);(l.fe99, l.fe9a, l.fe9b, l.fe9c, l.fe9d, ) = (t2ae, (l.fe9a - (((((((l.f131f * l.f14bd) + (l.f131e * l.f14be)) * t2aa) + (t2a9 * l.f128f)) * t2ac) - (t2ab * (l.f4f6 * l.ff01))) / (t2ac * t2ac))), (l.fe9b - (((((((l.f1320 * l.f14bd) + (l.f131e * l.f14bf)) * t2aa) + (t2a9 * l.f1290)) * t2ac) - (t2ab * (l.f4f7 * l.ff01))) / (t2ac * t2ac))), (l.fe9c - (((((((l.f1321 * l.f14bd) + (l.f131e * l.f14c0)) * t2aa) + (t2a9 * l.f1291)) * t2ac) - (t2ab * (l.f4f8 * l.ff01))) / (t2ac * t2ac))), (l.fe9d - (((((((l.f1322 * l.f14bd) + (l.f131e * l.f14c1)) * t2aa) + (t2a9 * l.f1292)) * t2ac) - (t2ab * (l.f4f9 * l.ff01))) / (t2ac * t2ac))), );}
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {let t2af: f64 = (l.ff00 / l.fe93);let t2b0: f64 = (t2af).sqrt();(l.f148b, l.f148c, l.f148d, l.f148e, l.f148f, ) = (t2b0, ((-((l.ff00 * l.fe94) / (l.fe93 * l.fe93))) / (2.0 * t2b0)), ((-((l.ff00 * l.fe95) / (l.fe93 * l.fe93))) / (2.0 * t2b0)), ((-((l.ff00 * l.fe96) / (l.fe93 * l.fe93))) / (2.0 * t2b0)), ((-((l.ff00 * l.fe97) / (l.fe93 * l.fe93))) / (2.0 * t2b0)), );}
        let t2b1: f64 = if l.f1486 <= 0.0 { 1.0 } else { 0.0 };l.f8c4 = t2b1;
        if (((l.f8bc != 0.0) && (l.f8c1 != 0.0)) && (l.f8c4 != 0.0)) {(l.f145, l.f146, l.f147, l.f148, l.f149, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );}
        if (((l.f8bc != 0.0) && (l.f8c1 != 0.0)) && (l.f8c4 == 0.0)) {let t2b2: f64 = (l.fe99 * l.f148b);let t2b3: f64 = (t2b2 / l.f1486);(l.f145, l.f146, l.f147, l.f148, l.f149, ) = (t2b3, (((((l.fe9a * l.f148b) + (l.fe99 * l.f148c)) * l.f1486) - (t2b2 * l.f1487)) / (l.f1486 * l.f1486)), (((((l.fe9b * l.f148b) + (l.fe99 * l.f148d)) * l.f1486) - (t2b2 * l.f1488)) / (l.f1486 * l.f1486)), (((((l.fe9c * l.f148b) + (l.fe99 * l.f148e)) * l.f1486) - (t2b2 * l.f1489)) / (l.f1486 * l.f1486)), (((((l.fe9d * l.f148b) + (l.fe99 * l.f148f)) * l.f1486) - (t2b2 * l.f148a)) / (l.f1486 * l.f1486)), );}
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {
            let (t2b9, t2ba, t2bb, t2bc, t2bd,) = {
    if (l.f145 > 0.0) {
        let (t2b4, t2b5, t2b6, t2b7, t2b8,) = {
            if (l.f145 < 1.0) {
                (l.f145, l.f146, l.f147, l.f148, l.f149,)
            } else {
                (1.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2b4, t2b5, t2b6, t2b7, t2b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.f145, l.f146, l.f147, l.f148, l.f149, ) = (t2b9, t2ba, t2bb, t2bc, t2bd, );
        }
        if ((l.f8bc != 0.0) && (l.f8c1 != 0.0)) {let t2be: f64 = (l.f145 * l.f1486);let t2bf: f64 = (t2be / l.f148b);(l.fe98, l.fe9e, l.fe9f, l.fea0, l.fea1, ) = (t2bf, (((((l.f146 * l.f1486) + (l.f145 * l.f1487)) * l.f148b) - (t2be * l.f148c)) / (l.f148b * l.f148b)), (((((l.f147 * l.f1486) + (l.f145 * l.f1488)) * l.f148b) - (t2be * l.f148d)) / (l.f148b * l.f148b)), (((((l.f148 * l.f1486) + (l.f145 * l.f1489)) * l.f148b) - (t2be * l.f148e)) / (l.f148b * l.f148b)), (((((l.f149 * l.f1486) + (l.f145 * l.f148a)) * l.f148b) - (t2be * l.f148f)) / (l.f148b * l.f148b)), );}
        let t2c0: f64 = if (((p.p46 != 0.0) && (l.f107 > 0.0)) && (l.f1a1d > 0.0)) { 1.0 } else { 0.0 };l.f8c5 = t2c0;
        if (l.f8c5 != 0.0) {let t2c1: f64 = (4.0 * l.f331);let t2c2: f64 = (t2c1 / l.f56a);(l.f1521, l.f1522, l.f1523, l.f1524, l.f1525, ) = (t2c2, ((4.0 * l.f332) / l.f56a), ((4.0 * l.f333) / l.f56a), ((4.0 * l.f334) / l.f56a), ((4.0 * l.f335) / l.f56a), );let t2c3: f64 = (l.f1b9 * l.ff9e);(l.f1521, l.f1522, l.f1523, l.f1524, l.f1525, ) = (t2c3, 0.0, 0.0, 0.0, 0.0, );let t2c4: f64 = (l.f62 * l.fc13);(l.f1521, l.f1522, l.f1523, l.f1524, l.f1525, ) = (t2c4, ((l.f63 * l.fc13) + (l.f62 * l.fc14)), ((l.f64 * l.fc13) + (l.f62 * l.fc15)), ((l.f65 * l.fc13) + (l.f62 * l.fc16)), ((l.f66 * l.fc13) + (l.f62 * l.fc17)), );}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t0: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };l.f5e4 = t0;l.f8ea = 0.0;
        if (l.f5e4 != 0.0) {let t2: f64 = 1.0;l.f193 = t2;l.f194 = 0.0;}
        if (l.f5e4 == 0.0) {let t43: f64 = (-1.0);l.f193 = t43;l.f194 = 0.0;}
        let t175: f64 = (8.8541878176e-12 * 11.8);l.f3b7 = t175;l.f3b8 = 0.0;let t1f1: f64 = (273.15 + p.p38);l.f15cd = t1f1;l.f15d0 = 0.0;l.f1510 = 0.0;l.f1511 = 0.0;let t207: f64 = if p.p920 > 0.5 { 1.0 } else { 0.0 };l.f8eb = t207;l.f92c = 0.0;
        if (l.f8eb != 0.0) {l.f1510 = 1.0;l.f1511 = 0.0;}
        if (l.f8eb == 0.0) {l.f1510 = 0.0;l.f1511 = 0.0;}
        let t2c5: f64 = (273.15 + p.p816);l.f15ce = t2c5;l.f15cf = 0.0;let t1: f64 = (1.3806505e-23 / 1.6021918e-19);l.fddc = t1;l.fddd = 0.0;let t3: f64 = (l.fddc * l.f15ce);l.ffd2 = t3;l.ffd3 = 0.0;let t4: f64 = (1.0 / l.ffd2);l.ffd4 = t4;l.ffd5 = 0.0;let t5: f64 = (0.000702 * l.f15ce);let t6: f64 = (t5 * l.f15ce);let t7: f64 = (-t6);let t8: f64 = (1108.0 + l.f15ce);let t9: f64 = (t7 / t8);l.f28d = t9;l.f28e = 0.0;let ta: f64 = (p.p827 + l.f28d);l.ff8c = ta;l.ff8f = 0.0;let tb: f64 = (p.p828 + l.f28d);l.ff98 = tb;l.ff9b = 0.0;let tc: f64 = (p.p829 + l.f28d);l.ff90 = tc;l.ff97 = 0.0;let t15: f64 = (1.0 - p.p824);l.ff0f = t15;l.ff12 = 0.0;let t22: f64 = (1.0 - p.p825);l.ff1b = t22;l.ff1e = 0.0;let t27: f64 = (1.0 - p.p826);l.ff13 = t27;l.ff1a = 0.0;let t3a: f64 = (1.0 / l.ff0f);l.ff1f = t3a;l.ff22 = 0.0;let t44: f64 = (1.0 / l.ff1b);l.ff2b = t44;l.ff2e = 0.0;let t59: f64 = (1.0 / l.ff13);l.ff23 = t59;l.ff2a = 0.0;let t79: f64 = (l.f3b7 / p.p818);l.f18d0 = t79;l.f18d3 = 0.0;let tab: f64 = (p.p836 * l.f3b7);let tac: f64 = (tab / p.p819);l.f18e4 = tac;l.f18e7 = 0.0;let tc0: f64 = (p.p837 * l.f3b7);let tc1: f64 = (tc0 / p.p820);l.f18d4 = tc1;l.f18d7 = 0.0;let tdf: f64 = (1.0 / l.f18d0);l.f18d8 = tdf;l.f18db = 0.0;let tfe: f64 = (1.0 / l.f18e4);l.f18e0 = tfe;l.f18e3 = 0.0;let t11c: f64 = (1.0 / l.f18d4);l.f18dc = t11c;l.f18df = 0.0;let t14e: f64 = (1.0 / p.p821);l.f1724 = t14e;l.f1727 = 0.0;let t168: f64 = (1.0 / p.p822);l.f1734 = t168;l.f1737 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t17f: f64 = (1.0 / p.p823);l.f172e = t17f;l.f1731 = 0.0;let t1bb: f64 = (1.0 / p.p817);let t1bc: f64 = (1.0 - t1bb);l.f6d = t1bc;l.f6e = 0.0;let t1ef: f64 = (1.0 / p.p853);l.f1750 = t1ef;l.f1753 = 0.0;let t1f0: f64 = (1.0 / p.p854);l.f1760 = t1f0;l.f1763 = 0.0;let t1f2: f64 = (1.0 / p.p855);(l.f1754, l.f175b, l.f175c, l.f175d, l.f175e, ) = (t1f2, 0.0, 0.0, 0.0, 0.0, );l.f175f = 0.0;let t1f3: f64 = if ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0)) { 1.0 } else { 0.0 };l.f92d = t1f3;l.f9a6 = 0.0;
        if (l.f92d != 0.0) {l.f150c = 1.0;l.f150f = 0.0;}
        if (l.f92d == 0.0) {l.f150c = 0.0;l.f150f = 0.0;}
        let t1f4: f64 = if l.f150c == 1.0 { 1.0 } else { 0.0 };l.f9a7 = t1f4;l.fa20 = 0.0;
        if (l.f9a7 != 0.0) {
            let t1f5: f64 = (p.p820 * p.p859);
            let (t1f7,) = {
    if (t1f5 > 1e-18) {
        let t1f6: f64 = (p.p820 * p.p859);
        (t1f6,)
    } else {
        (1e-18,)
    }
};
            l.f1ab = t1f7;l.f1ae = 0.0;
        }
        if (l.f9a7 != 0.0) {
            let t1f8: f64 = (p.p823 * p.p860);
            let (t1fa,) = {
    if (t1f8 > 0.05) {
        let t1f9: f64 = (p.p823 * p.p860);
        (t1f9,)
    } else {
        (0.05,)
    }
};
            l.f1728 = t1fa;l.f172b = 0.0;
        }
        if (l.f9a7 != 0.0) {
            let t1fb: f64 = (p.p826 * p.p861);
            let (t1fd,) = {
    if (t1fb > 0.05) {
        let t1fc: f64 = (p.p826 * p.p861);
        (t1fc,)
    } else {
        (0.05,)
    }
};
            let (t201,) = {
    if (t1fd < 0.95) {
        let t1fe: f64 = (p.p826 * p.p861);
        let (t200,) = {
            if (t1fe > 0.05) {
                let t1ff: f64 = (p.p826 * p.p861);
                (t1ff,)
            } else {
                (0.05,)
            }
        };
        (t200,)
    } else {
        (0.95,)
    }
};
            l.ff62 = t201;l.ff65 = 0.0;
        }
        if (l.f9a7 != 0.0) {let t202: f64 = (p.p829 * p.p862);l.ff86 = t202;l.ff89 = 0.0;let t203: f64 = (l.ff86 + l.f28d);l.ff91 = t203;l.ff94 = 0.0;let t204: f64 = (1.0 - l.ff62);l.ff14 = t204;l.ff17 = 0.0;let t205: f64 = (1.0 / l.ff14);l.ff24 = t205;l.ff27 = 0.0;}
        let t206: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.fa21 = t206;l.fa9c = 0.0;
        if (l.fa21 != 0.0) {l.f1a9 = p.p818;l.f1aa = 0.0;l.f1b1 = p.p819;l.f1b2 = 0.0;l.f1af = p.p820;l.f1b0 = 0.0;l.f1722 = p.p821;l.f1723 = 0.0;l.f1732 = p.p822;l.f1733 = 0.0;l.f172c = p.p823;l.f172d = 0.0;l.ff3d = p.p824;l.ff3e = 0.0;l.f102e = p.p825;l.f102f = 0.0;l.ff66 = p.p826;l.ff67 = 0.0;l.ff74 = p.p827;l.ff75 = 0.0;l.ff9c = p.p828;l.ff9d = 0.0;l.ff8a = p.p829;l.ff8b = 0.0;l.fc93 = p.p830;l.fc94 = 0.0;l.fc97 = p.p831;l.fc98 = 0.0;l.fc95 = p.p832;l.fc96 = 0.0;l.f1cf = p.p833;l.f1d0 = 0.0;l.f1d3 = p.p834;l.f1d4 = 0.0;l.f1d1 = p.p835;l.f1d2 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fa21 != 0.0) {l.f1aea = p.p836;l.f1aeb = 0.0;l.f1ae8 = p.p837;l.f1ae9 = 0.0;l.f1e7 = p.p838;l.f1e8 = 0.0;l.f1eb = p.p839;l.f1ec = 0.0;l.f1e9 = p.p840;l.f1ea = 0.0;l.fe70 = p.p841;l.fe71 = 0.0;l.fe74 = p.p842;l.fe75 = 0.0;l.fe72 = p.p843;l.fe73 = 0.0;l.f14a = p.p844;l.f14b = 0.0;l.f14e = p.p845;l.f14f = 0.0;l.f14c = p.p846;l.f14d = 0.0;l.f44a = p.p847;l.f44b = 0.0;l.f44e = p.p848;l.f44f = 0.0;l.f44c = p.p849;l.f44d = 0.0;l.f14e2 = p.p850;l.f14e3 = 0.0;l.f14e6 = p.p851;l.f14e7 = 0.0;l.f14e4 = p.p852;l.f14e5 = 0.0;l.f1740 = p.p853;l.f1741 = 0.0;l.f1764 = p.p854;l.f1765 = 0.0;l.f174e = p.p855;l.f174f = 0.0;l.ff3f = p.p856;l.ff40 = 0.0;l.ff43 = p.p857;l.ff44 = 0.0;l.ff41 = p.p858;l.ff42 = 0.0;l.f496 = p.p922;l.f497 = 0.0;l.f20 = p.p865;l.f21 = 0.0;l.ffa = p.p866;l.ffb = 0.0;l.f1e = p.p867;l.f1f = 0.0;l.ff8 = p.p868;l.ff9 = 0.0;l.f472 = p.p859;l.f473 = 0.0;l.f4eb = p.p860;l.f4ec = 0.0;l.f4ab = p.p861;l.f4ac = 0.0;l.f4ad = p.p862;l.f4ae = 0.0;l.f18bc = p.p863;l.f18bd = 0.0;l.f98 = p.p864;l.f99 = 0.0;}
        if (l.fa21 == 0.0) {l.f1a9 = p.p869;l.f1aa = 0.0;l.f1b1 = p.p870;l.f1b2 = 0.0;l.f1af = p.p871;l.f1b0 = 0.0;l.f1722 = p.p872;l.f1723 = 0.0;l.f1732 = p.p873;l.f1733 = 0.0;l.f172c = p.p874;l.f172d = 0.0;l.ff3d = p.p875;l.ff3e = 0.0;l.f102e = p.p876;l.f102f = 0.0;l.ff66 = p.p877;l.ff67 = 0.0;l.ff74 = p.p878;l.ff75 = 0.0;l.ff9c = p.p879;l.ff9d = 0.0;l.ff8a = p.p880;l.ff8b = 0.0;l.fc93 = p.p881;l.fc94 = 0.0;l.fc97 = p.p882;l.fc98 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fa21 == 0.0) {l.fc95 = p.p883;l.fc96 = 0.0;l.f1cf = p.p884;l.f1d0 = 0.0;l.f1d3 = p.p885;l.f1d4 = 0.0;l.f1d1 = p.p886;l.f1d2 = 0.0;l.f1aea = p.p887;l.f1aeb = 0.0;l.f1ae8 = p.p888;l.f1ae9 = 0.0;l.f1e7 = p.p889;l.f1e8 = 0.0;l.f1eb = p.p890;l.f1ec = 0.0;l.f1e9 = p.p891;l.f1ea = 0.0;l.fe70 = p.p892;l.fe71 = 0.0;l.fe74 = p.p893;l.fe75 = 0.0;l.fe72 = p.p894;l.fe73 = 0.0;l.f14a = p.p895;l.f14b = 0.0;l.f14e = p.p896;l.f14f = 0.0;l.f14c = p.p897;l.f14d = 0.0;l.f44a = p.p898;l.f44b = 0.0;l.f44e = p.p899;l.f44f = 0.0;l.f44c = p.p900;l.f44d = 0.0;l.f14e2 = p.p901;l.f14e3 = 0.0;l.f14e6 = p.p902;l.f14e7 = 0.0;l.f14e4 = p.p903;l.f14e5 = 0.0;l.f1740 = p.p904;l.f1741 = 0.0;l.f1764 = p.p905;l.f1765 = 0.0;l.f174e = p.p906;l.f174f = 0.0;l.ff3f = p.p907;l.ff40 = 0.0;l.ff43 = p.p908;l.ff44 = 0.0;l.ff41 = p.p909;l.ff42 = 0.0;l.f496 = p.p924;l.f497 = 0.0;l.f20 = p.p916;l.f21 = 0.0;l.ffa = p.p917;l.ffb = 0.0;l.f1e = p.p918;l.f1f = 0.0;l.ff8 = p.p919;l.ff9 = 0.0;l.f472 = p.p910;l.f473 = 0.0;l.f4eb = p.p911;l.f4ec = 0.0;l.f4ab = p.p912;l.f4ac = 0.0;l.f4ad = p.p913;l.f4ae = 0.0;l.f18bc = p.p914;l.f18bd = 0.0;l.f98 = p.p915;l.f99 = 0.0;}
        let td: f64 = (l.ff74 + l.f28d);l.ff8d = td;l.ff8e = 0.0;let te: f64 = (l.ff9c + l.f28d);l.ff99 = te;l.ff9a = 0.0;let tf: f64 = (l.ff8a + l.f28d);l.ff95 = tf;l.ff96 = 0.0;let t10: f64 = (1.0 - l.ff3d);l.ff10 = t10;l.ff11 = 0.0;let t11: f64 = (1.0 - l.f102e);l.ff1c = t11;l.ff1d = 0.0;let t12: f64 = (1.0 - l.ff66);l.ff18 = t12;l.ff19 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t13: f64 = (1.0 / l.ff10);l.ff20 = t13;l.ff21 = 0.0;let t14: f64 = (1.0 / l.ff1c);l.ff2c = t14;l.ff2d = 0.0;let t16: f64 = (1.0 / l.ff18);l.ff28 = t16;l.ff29 = 0.0;let t17: f64 = (l.f3b7 / l.f1a9);l.f18d1 = t17;l.f18d2 = 0.0;let t18: f64 = (l.f1aea * l.f3b7);let t19: f64 = (t18 / l.f1b1);l.f18e5 = t19;l.f18e6 = 0.0;let t1a: f64 = (l.f1ae8 * l.f3b7);let t1b: f64 = (t1a / l.f1af);l.f18d5 = t1b;l.f18d6 = 0.0;let t1c: f64 = (1.0 / l.f18d1);l.f18d9 = t1c;l.f18da = 0.0;let t1d: f64 = (1.0 / l.f18e5);l.f18e1 = t1d;l.f18e2 = 0.0;let t1e: f64 = (1.0 / l.f18d5);l.f18dd = t1e;l.f18de = 0.0;let t1f: f64 = (1.0 / l.f1722);l.f1725 = t1f;l.f1726 = 0.0;let t20: f64 = (1.0 / l.f1732);l.f1735 = t20;l.f1736 = 0.0;let t21: f64 = (1.0 / l.f172c);l.f172f = t21;l.f1730 = 0.0;let t23: f64 = (1.0 / l.f1740);l.f1751 = t23;l.f1752 = 0.0;let t24: f64 = (1.0 / l.f1764);l.f1761 = t24;l.f1762 = 0.0;let t25: f64 = (1.0 / l.f174e);(l.f1755, l.f1756, l.f1757, l.f1758, l.f1759, ) = (t25, 0.0, 0.0, 0.0, 0.0, );l.f175a = 0.0;let t26: f64 = if ((((l.f472 != 1.0) || (l.f4eb != 1.0)) || (l.f4ab != 1.0)) || (l.f4ad != 1.0)) { 1.0 } else { 0.0 };l.fa9d = t26;l.fb16 = 0.0;
        if (l.fa9d != 0.0) {l.f150d = 1.0;l.f150e = 0.0;}
        if (l.fa9d == 0.0) {l.f150d = 0.0;l.f150e = 0.0;}
        let t28: f64 = if l.f150d == 1.0 { 1.0 } else { 0.0 };l.fb17 = t28;l.fb90 = 0.0;
        if (l.fb17 != 0.0) {
            let t29: f64 = (l.f1af * l.f472);
            let (t2b,) = {
    if (t29 > 1e-18) {
        let t2a: f64 = (l.f1af * l.f472);
        (t2a,)
    } else {
        (1e-18,)
    }
};
            l.f1ac = t2b;l.f1ad = 0.0;
        }
        if (l.fb17 != 0.0) {
            let t2c: f64 = (l.f172c * l.f4eb);
            let (t2e,) = {
    if (t2c > 0.05) {
        let t2d: f64 = (l.f172c * l.f4eb);
        (t2d,)
    } else {
        (0.05,)
    }
};
            l.f1729 = t2e;l.f172a = 0.0;
        }
        if (l.fb17 != 0.0) {
            let t2f: f64 = (l.ff66 * l.f4ab);
            let (t31,) = {
    if (t2f > 0.05) {
        let t30: f64 = (l.ff66 * l.f4ab);
        (t30,)
    } else {
        (0.05,)
    }
};
            let (t35,) = {
    if (t31 < 0.95) {
        let t32: f64 = (l.ff66 * l.f4ab);
        let (t34,) = {
            if (t32 > 0.05) {
                let t33: f64 = (l.ff66 * l.f4ab);
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
            l.ff63 = t35;l.ff64 = 0.0;
        }
        if (l.fb17 != 0.0) {let t36: f64 = (l.ff8a * l.f4ad);l.ff87 = t36;l.ff88 = 0.0;let t37: f64 = (l.ff87 + l.f28d);l.ff92 = t37;l.ff93 = 0.0;let t38: f64 = (1.0 - l.ff63);l.ff15 = t38;l.ff16 = 0.0;let t39: f64 = (1.0 / l.ff15);l.ff25 = t39;l.ff26 = 0.0;}
        let t3b: f64 = ctx_temp;let t3c: f64 = (t3b + p.p55);let t3d: f64 = (t3c + p.p35);l.f15c5 = t3d;l.f15c6 = 0.0;let t3e: f64 = (l.f15c5 / l.f15cd);l.f12d7 = t3e;l.f12d8 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let t3f: f64 = (l.f15c5 - l.f15cd);l.f241 = t3f;l.f27e = 0.0;let t40: f64 = (l.f15c5 * 1.3806505e-23);let t41: f64 = (t40 / 1.6021918e-19);l.ffc0 = t41;l.ffc1 = 0.0;let t42: f64 = (1.0 / l.ffc0);l.fd73 = t42;l.fd74 = 0.0;l.f15c7 = l.f15c5;l.f15ca = 0.0;let t45: f64 = (l.f15c7 * l.f15c7);l.f15cb = t45;l.f15cc = 0.0;let t46: f64 = (l.f15c7 - l.f15cd);l.f23f = t46;l.f240 = 0.0;let t47: f64 = (l.f15cd / l.f15c7);l.f12d9 = t47;l.f12da = 0.0;let t48: f64 = (l.f12d9).ln();l.fe2b = t48;l.fe2c = 0.0;let t49: f64 = (l.f15c7 * 1.3806505e-23);let t4a: f64 = (t49 / 1.6021918e-19);l.ff9e = t4a;l.ffbf = 0.0;let t4b: f64 = (1.0 / l.ff9e);l.fd59 = t4b;l.fd72 = 0.0;let t4c: f64 = (9.025e-5 * l.f15c7);let t4d: f64 = (1.179 - t4c);let t4e: f64 = (3.05e-7 * l.f15cb);let t4f: f64 = (t4d - t4e);l.f3a3 = t4f;l.f3a4 = 0.0;let t50: f64 = (0.00045 * l.f15c7);let t51: f64 = (1.045 + t50);let t52: f64 = (0.0014 * l.f15c7);let t53: f64 = (0.523 + t52);let t54: f64 = (1.48e-6 * l.f15cb);let t55: f64 = (t53 - t54);let t56: f64 = (t51 * t55);let t57: f64 = (t56 * l.f15cb);let t58: f64 = (t57 / 90000.0);l.ff72 = t58;l.ff73 = 0.0;
        if (!(l.ff72 > 0.001)) {l.ff72 = 0.001;l.ff73 = 0.0;}
        let t5a: f64 = ctx_temp;let t5b: f64 = (t5a + p.p55);let t5c: f64 = (t5b + p.p35);let t5d: f64 = (-250.0);let t5e: f64 = (273.15 + t5d);let t5f: f64 = (t5c).max(t5e);l.f15c8 = t5f;l.f15c9 = 0.0;let t60: f64 = (l.f15c8 / l.f15ce);l.fe0 = t60;l.fe1 = 0.0;let t61: f64 = (l.fddc * l.f15c8);l.ffce = t61;l.ffcf = 0.0;let t62: f64 = (1.0 / l.ffce);l.ffd0 = t62;l.ffd1 = 0.0;let t63: f64 = (0.000702 * l.f15c8);let t64: f64 = (t63 * l.f15c8);let t65: f64 = (-t64);let t66: f64 = (1108.0 + l.f15c8);let t67: f64 = (t65 / t66);l.f28b = t67;l.f28c = 0.0;let t68: f64 = (p.p827 + l.f28b);l.ff76 = t68;l.ff79 = 0.0;let t69: f64 = (p.p828 + l.f28b);l.ff82 = t69;l.ff85 = 0.0;let t6a: f64 = (p.p829 + l.f28b);l.ff7a = t6a;l.ff81 = 0.0;let t6b: f64 = (l.fe0).powf(1.5);let t6c: f64 = (l.ff8c * l.ffd4);let t6d: f64 = (l.ff76 * l.ffd0);let t6e: f64 = (t6c - t6d);let t6f: f64 = (0.5 * t6e);let t70: f64 = (t6f).exp();let t71: f64 = (t6b * t70);l.f4db = t71;l.f4de = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t72: f64 = (l.fe0).powf(1.5);let t73: f64 = (l.ff98 * l.ffd4);let t74: f64 = (l.ff82 * l.ffd0);let t75: f64 = (t73 - t74);let t76: f64 = (0.5 * t75);let t77: f64 = (t76).exp();let t78: f64 = (t72 * t77);l.f4e7 = t78;l.f4ea = 0.0;let t7a: f64 = (l.fe0).powf(1.5);let t7b: f64 = (l.ff90 * l.ffd4);let t7c: f64 = (l.ff7a * l.ffd0);let t7d: f64 = (t7b - t7c);let t7e: f64 = (0.5 * t7d);let t7f: f64 = (t7e).exp();let t80: f64 = (t7a * t7f);l.f4df = t80;l.f4e6 = 0.0;let t81: f64 = (p.p830 * l.f4db);let t82: f64 = (t81 * l.f4db);l.fc8b = t82;l.fc8e = 0.0;let t83: f64 = (p.p831 * l.f4e7);let t84: f64 = (t83 * l.f4e7);l.fc99 = t84;l.fc9c = 0.0;let t85: f64 = (p.p832 * l.f4df);let t86: f64 = (t85 * l.f4df);l.fc8f = t86;l.fc92 = 0.0;let t87: f64 = (p.p821 * l.fe0);let t88: f64 = (2.0 * l.ffce);let t89: f64 = (l.f4db).ln();let t8a: f64 = (t88 * t89);let t8b: f64 = (t87 - t8a);l.f1649 = t8b;l.f164c = 0.0;let t8c: f64 = (p.p822 * l.fe0);let t8d: f64 = (2.0 * l.ffce);let t8e: f64 = (l.f4e7).ln();let t8f: f64 = (t8d * t8e);let t90: f64 = (t8c - t8f);l.f1655 = t90;l.f1658 = 0.0;let t91: f64 = (p.p823 * l.fe0);let t92: f64 = (2.0 * l.ffce);let t93: f64 = (l.f4df).ln();let t94: f64 = (t92 * t93);let t95: f64 = (t91 - t94);l.f164d = t95;l.f1654 = 0.0;let t96: f64 = (0.05 - l.f1649);let t97: f64 = (t96 * l.ffd0);let t98: f64 = (t97).exp();let t99: f64 = (1.0 + t98);let t9a: f64 = (t99).ln();let t9b: f64 = (l.ffce * t9a);let t9c: f64 = (l.f1649 + t9b);l.f16fa = t9c;l.f1701 = 0.0;let t9d: f64 = (0.05 - l.f1655);let t9e: f64 = (t9d * l.ffd0);let t9f: f64 = (t9e).exp();let ta0: f64 = (1.0 + t9f);let ta1: f64 = (ta0).ln();let ta2: f64 = (l.ffce * ta1);let ta3: f64 = (l.f1655 + ta2);l.f1738 = ta3;l.f173f = 0.0;let ta4: f64 = (0.05 - l.f164d);let ta5: f64 = (ta4 * l.ffd0);let ta6: f64 = (ta5).exp();let ta7: f64 = (1.0 + ta6);let ta8: f64 = (ta7).ln();let ta9: f64 = (l.ffce * ta8);let taa: f64 = (l.f164d + ta9);l.f1702 = taa;l.f170d = 0.0;let tad: f64 = (1.0 / l.f16fa);l.f170e = tad;l.f1711 = 0.0;let tae: f64 = (1.0 / l.f1738);l.f171a = tae;l.f171d = 0.0;let taf: f64 = (1.0 / l.f1702);l.f1712 = taf;l.f1719 = 0.0;let tb0: f64 = (p.p821 * l.f170e);let tb1: f64 = (tb0).powf(p.p824);let tb2: f64 = (p.p818 * tb1);l.f19d = tb2;l.f1a0 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let tb3: f64 = (p.p822 * l.f171a);let tb4: f64 = (tb3).powf(p.p825);let tb5: f64 = (p.p819 * tb4);l.f1b3 = tb5;l.f1b6 = 0.0;let tb6: f64 = (p.p823 * l.f1712);let tb7: f64 = (tb6).powf(p.p826);let tb8: f64 = (p.p820 * tb7);l.f1a1 = tb8;l.f1a8 = 0.0;let tb9: f64 = (l.f19d * l.f16fa);let tba: f64 = (tb9 * l.ff1f);l.f126a = tba;l.f126d = 0.0;let tbb: f64 = (l.f1b3 * l.f1738);let tbc: f64 = (tbb * l.ff2b);l.f1276 = tbc;l.f1279 = 0.0;let tbd: f64 = (l.f1a1 * l.f1702);let tbe: f64 = (tbd * l.ff23);l.f126e = tbe;l.f1275 = 0.0;let tbf: f64 = (2.0 * l.f19d);l.f125a = tbf;l.f125d = 0.0;let tc2: f64 = (2.0 * l.f1b3);l.f1266 = tc2;l.f1269 = 0.0;let tc3: f64 = (2.0 * l.f1a1);l.f125e = tc3;l.f1265 = 0.0;let tc4: f64 = (0.5 * l.ff76);let tc5: f64 = (tc4).max(l.ffce);l.f27f = tc5;l.f282 = 0.0;let tc6: f64 = (0.5 * l.ff82);let tc7: f64 = (tc6).max(l.ffce);l.f287 = tc7;l.f28a = 0.0;let tc8: f64 = (0.5 * l.ff7a);let tc9: f64 = (tc8).max(l.ffce);l.f283 = tc9;l.f286 = 0.0;let tca: f64 = (l.f27f * l.ffd0);l.fd4 = tca;l.fd7 = 0.0;let tcb: f64 = (l.f287 * l.ffd0);l.fdc = tcb;l.fdf = 0.0;let tcc: f64 = (l.f283 * l.ffd0);l.fd8 = tcc;l.fdb = 0.0;let tcd: f64 = (32.0 * p.p841);let tce: f64 = (tcd * 9.1093826e-31);let tcf: f64 = (tce * 1.6021918e-19);let td0: f64 = (l.f27f * l.f27f);let td1: f64 = (td0 * l.f27f);let td2: f64 = (tcf * td1);let td3: f64 = (td2).sqrt();let td4: f64 = (3.0 * 1.05457168e-34);let td5: f64 = (td3 / td4);l.f139 = td5;l.f13c = 0.0;let td6: f64 = (32.0 * p.p842);let td7: f64 = (td6 * 9.1093826e-31);let td8: f64 = (td7 * 1.6021918e-19);let td9: f64 = (l.f287 * l.f287);let tda: f64 = (td9 * l.f287);let tdb: f64 = (td8 * tda);let tdc: f64 = (tdb).sqrt();let tdd: f64 = (3.0 * 1.05457168e-34);let tde: f64 = (tdc / tdd);l.f141 = tde;l.f144 = 0.0;let te0: f64 = (32.0 * p.p843);let te1: f64 = (te0 * 9.1093826e-31);let te2: f64 = (te1 * 1.6021918e-19);let te3: f64 = (l.f283 * l.f283);let te4: f64 = (te3 * l.f283);let te5: f64 = (te2 * te4);let te6: f64 = (te5).sqrt();let te7: f64 = (3.0 * 1.05457168e-34);let te8: f64 = (te6 / te7);l.f13d = te8;l.f140 = 0.0;let te9: f64 = (l.f15c8 - l.f15ce);let tea: f64 = (p.p850 * te9);let teb: f64 = (1.0 + tea);let tec: f64 = (p.p847 * teb);l.f43a = tec;l.f43d = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let ted: f64 = (l.f15c8 - l.f15ce);let tee: f64 = (p.p851 * ted);let tef: f64 = (1.0 + tee);let tf0: f64 = (p.p848 * tef);l.f450 = tf0;l.f453 = 0.0;let tf1: f64 = (l.f15c8 - l.f15ce);let tf2: f64 = (p.p852 * tf1);let tf3: f64 = (1.0 + tf2);let tf4: f64 = (p.p849 * tf3);(l.f43e, l.f445, l.f446, l.f447, l.f448, ) = (tf4, 0.0, 0.0, 0.0, 0.0, );l.f449 = 0.0;
        if (!(l.f43a > 0.0)) {l.f43a = 0.0;l.f43d = 0.0;}
        if (!(l.f450 > 0.0)) {l.f450 = 0.0;l.f453 = 0.0;}
        if (!(l.f43e > 0.0)) {(l.f43e, l.f445, l.f446, l.f447, l.f448, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f449 = 0.0;}
        let tf5: f64 = if l.f150c == 1.0 { 1.0 } else { 0.0 };l.f908 = tf5;l.f913 = 0.0;
        if (l.f908 != 0.0) {let tf6: f64 = (l.ff86 + l.f28b);l.ff7b = tf6;l.ff7e = 0.0;let tf7: f64 = (l.fe0).powf(1.5);let tf8: f64 = (l.ff91 * l.ffd4);let tf9: f64 = (l.ff7b * l.ffd0);let tfa: f64 = (tf8 - tf9);let tfb: f64 = (0.5 * tfa);let tfc: f64 = (tfb).exp();let tfd: f64 = (tf7 * tfc);l.f4e0 = tfd;l.f4e3 = 0.0;let tff: f64 = (l.f1728 * l.fe0);let t100: f64 = (2.0 * l.ffce);let t101: f64 = (l.f4e0).ln();let t102: f64 = (t100 * t101);let t103: f64 = (tff - t102);l.f164e = t103;l.f1651 = 0.0;let t104: f64 = (0.05 - l.f164e);let t105: f64 = (t104 * l.ffd0);let t106: f64 = (t105).exp();let t107: f64 = (1.0 + t106);let t108: f64 = (t107).ln();let t109: f64 = (l.ffce * t108);let t10a: f64 = (l.f164e + t109);l.f1705 = t10a;l.f1708 = 0.0;let t10b: f64 = (1.0 / l.f1705);l.f1713 = t10b;l.f1716 = 0.0;let t10c: f64 = (l.f1728 * l.f1713);let t10d: f64 = (t10c).powf(l.ff62);let t10e: f64 = (l.f1ab * t10d);l.f1a2 = t10e;l.f1a5 = 0.0;let t10f: f64 = (l.f1a2 * l.f1705);let t110: f64 = (t10f * l.ff24);l.f126f = t110;l.f1272 = 0.0;let t111: f64 = (2.0 * l.f1a2);l.f125f = t111;l.f1262 = 0.0;}
        let t112: f64 = (l.ff74 + l.f28b);l.ff77 = t112;l.ff78 = 0.0;let t113: f64 = (l.ff9c + l.f28b);l.ff83 = t113;l.ff84 = 0.0;let t114: f64 = (l.ff8a + l.f28b);l.ff7f = t114;l.ff80 = 0.0;let t115: f64 = (l.fe0).powf(1.5);let t116: f64 = (l.ff8d * l.ffd4);let t117: f64 = (l.ff77 * l.ffd0);let t118: f64 = (t116 - t117);let t119: f64 = (0.5 * t118);let t11a: f64 = (t119).exp();let t11b: f64 = (t115 * t11a);l.f4dc = t11b;l.f4dd = 0.0;let t11d: f64 = (l.fe0).powf(1.5);let t11e: f64 = (l.ff99 * l.ffd4);let t11f: f64 = (l.ff83 * l.ffd0);let t120: f64 = (t11e - t11f);let t121: f64 = (0.5 * t120);let t122: f64 = (t121).exp();let t123: f64 = (t11d * t122);l.f4e8 = t123;l.f4e9 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        l: &mut StampLocals,
    ) {
        let t124: f64 = (l.fe0).powf(1.5);let t125: f64 = (l.ff95 * l.ffd4);let t126: f64 = (l.ff7f * l.ffd0);let t127: f64 = (t125 - t126);let t128: f64 = (0.5 * t127);let t129: f64 = (t128).exp();let t12a: f64 = (t124 * t129);l.f4e4 = t12a;l.f4e5 = 0.0;let t12b: f64 = (l.fc93 * l.f4dc);let t12c: f64 = (t12b * l.f4dc);l.fc8c = t12c;l.fc8d = 0.0;let t12d: f64 = (l.fc97 * l.f4e8);let t12e: f64 = (t12d * l.f4e8);l.fc9a = t12e;l.fc9b = 0.0;let t12f: f64 = (l.fc95 * l.f4e4);let t130: f64 = (t12f * l.f4e4);l.fc90 = t130;l.fc91 = 0.0;let t131: f64 = (l.f1722 * l.fe0);let t132: f64 = (2.0 * l.ffce);let t133: f64 = (l.f4dc).ln();let t134: f64 = (t132 * t133);let t135: f64 = (t131 - t134);l.f164a = t135;l.f164b = 0.0;let t136: f64 = (l.f1732 * l.fe0);let t137: f64 = (2.0 * l.ffce);let t138: f64 = (l.f4e8).ln();let t139: f64 = (t137 * t138);let t13a: f64 = (t136 - t139);l.f1656 = t13a;l.f1657 = 0.0;let t13b: f64 = (l.f172c * l.fe0);let t13c: f64 = (2.0 * l.ffce);let t13d: f64 = (l.f4e4).ln();let t13e: f64 = (t13c * t13d);let t13f: f64 = (t13b - t13e);l.f1652 = t13f;l.f1653 = 0.0;let t140: f64 = (0.05 - l.f164a);let t141: f64 = (t140 * l.ffd0);let t142: f64 = (t141).exp();let t143: f64 = (1.0 + t142);let t144: f64 = (t143).ln();let t145: f64 = (l.ffce * t144);let t146: f64 = (l.f164a + t145);l.f16ff = t146;l.f1700 = 0.0;let t147: f64 = (0.05 - l.f1656);let t148: f64 = (t147 * l.ffd0);let t149: f64 = (t148).exp();let t14a: f64 = (1.0 + t149);let t14b: f64 = (t14a).ln();let t14c: f64 = (l.ffce * t14b);let t14d: f64 = (l.f1656 + t14c);l.f173d = t14d;l.f173e = 0.0;let t14f: f64 = (0.05 - l.f1652);let t150: f64 = (t14f * l.ffd0);let t151: f64 = (t150).exp();let t152: f64 = (1.0 + t151);let t153: f64 = (t152).ln();let t154: f64 = (l.ffce * t153);let t155: f64 = (l.f1652 + t154);l.f170b = t155;l.f170c = 0.0;let t156: f64 = (1.0 / l.f16ff);l.f170f = t156;l.f1710 = 0.0;let t157: f64 = (1.0 / l.f173d);l.f171b = t157;l.f171c = 0.0;let t158: f64 = (1.0 / l.f170b);l.f1717 = t158;l.f1718 = 0.0;let t159: f64 = (l.f1722 * l.f170f);let t15a: f64 = (t159).powf(l.ff3d);let t15b: f64 = (l.f1a9 * t15a);l.f19e = t15b;l.f19f = 0.0;let t15c: f64 = (l.f1732 * l.f171b);let t15d: f64 = (t15c).powf(l.f102e);let t15e: f64 = (l.f1b1 * t15d);l.f1b4 = t15e;l.f1b5 = 0.0;let t15f: f64 = (l.f172c * l.f1717);let t160: f64 = (t15f).powf(l.ff66);let t161: f64 = (l.f1af * t160);l.f1a6 = t161;l.f1a7 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        l: &mut StampLocals,
    ) {
        let t162: f64 = (l.f19e * l.f16ff);let t163: f64 = (t162 * l.ff20);l.f126b = t163;l.f126c = 0.0;let t164: f64 = (l.f1b4 * l.f173d);let t165: f64 = (t164 * l.ff2c);l.f1277 = t165;l.f1278 = 0.0;let t166: f64 = (l.f1a6 * l.f170b);let t167: f64 = (t166 * l.ff28);l.f1273 = t167;l.f1274 = 0.0;let t169: f64 = (2.0 * l.f19e);l.f125b = t169;l.f125c = 0.0;let t16a: f64 = (2.0 * l.f1b4);l.f1267 = t16a;l.f1268 = 0.0;let t16b: f64 = (2.0 * l.f1a6);l.f1263 = t16b;l.f1264 = 0.0;let t16c: f64 = (0.5 * l.ff77);let t16d: f64 = (t16c).max(l.ffce);l.f280 = t16d;l.f281 = 0.0;let t16e: f64 = (0.5 * l.ff83);let t16f: f64 = (t16e).max(l.ffce);l.f288 = t16f;l.f289 = 0.0;let t170: f64 = (0.5 * l.ff7f);let t171: f64 = (t170).max(l.ffce);l.f284 = t171;l.f285 = 0.0;let t172: f64 = (l.f280 * l.ffd0);l.fd5 = t172;l.fd6 = 0.0;let t173: f64 = (l.f288 * l.ffd0);l.fdd = t173;l.fde = 0.0;let t174: f64 = (l.f284 * l.ffd0);l.fd9 = t174;l.fda = 0.0;let t176: f64 = (32.0 * l.fe70);let t177: f64 = (t176 * 9.1093826e-31);let t178: f64 = (t177 * 1.6021918e-19);let t179: f64 = (l.f280 * l.f280);let t17a: f64 = (t179 * l.f280);let t17b: f64 = (t178 * t17a);let t17c: f64 = (t17b).sqrt();let t17d: f64 = (3.0 * 1.05457168e-34);let t17e: f64 = (t17c / t17d);l.f13a = t17e;l.f13b = 0.0;let t180: f64 = (32.0 * l.fe74);let t181: f64 = (t180 * 9.1093826e-31);let t182: f64 = (t181 * 1.6021918e-19);let t183: f64 = (l.f288 * l.f288);let t184: f64 = (t183 * l.f288);let t185: f64 = (t182 * t184);let t186: f64 = (t185).sqrt();let t187: f64 = (3.0 * 1.05457168e-34);let t188: f64 = (t186 / t187);l.f142 = t188;l.f143 = 0.0;let t189: f64 = (32.0 * l.fe72);let t18a: f64 = (t189 * 9.1093826e-31);let t18b: f64 = (t18a * 1.6021918e-19);let t18c: f64 = (l.f284 * l.f284);let t18d: f64 = (t18c * l.f284);let t18e: f64 = (t18b * t18d);let t18f: f64 = (t18e).sqrt();let t190: f64 = (3.0 * 1.05457168e-34);let t191: f64 = (t18f / t190);l.f13e = t191;l.f13f = 0.0;let t192: f64 = (l.f15c8 - l.f15ce);let t193: f64 = (l.f14e2 * t192);let t194: f64 = (1.0 + t193);let t195: f64 = (l.f44a * t194);l.f43b = t195;l.f43c = 0.0;let t196: f64 = (l.f15c8 - l.f15ce);let t197: f64 = (l.f14e6 * t196);let t198: f64 = (1.0 + t197);let t199: f64 = (l.f44e * t198);l.f451 = t199;l.f452 = 0.0;let t19a: f64 = (l.f15c8 - l.f15ce);let t19b: f64 = (l.f14e4 * t19a);let t19c: f64 = (1.0 + t19b);let t19d: f64 = (l.f44c * t19c);(l.f43f, l.f440, l.f441, l.f442, l.f443, ) = (t19d, 0.0, 0.0, 0.0, 0.0, );l.f444 = 0.0;
        if (!(l.f43b > 0.0)) {l.f43b = 0.0;l.f43c = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (!(l.f451 > 0.0)) {l.f451 = 0.0;l.f452 = 0.0;}
        if (!(l.f43f > 0.0)) {(l.f43f, l.f440, l.f441, l.f442, l.f443, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );l.f444 = 0.0;}
        let t19e: f64 = if l.f150d == 1.0 { 1.0 } else { 0.0 };l.f914 = t19e;l.f91f = 0.0;
        if (l.f914 != 0.0) {let t19f: f64 = (l.ff87 + l.f28b);l.ff7c = t19f;l.ff7d = 0.0;let t1a0: f64 = (l.fe0).powf(1.5);let t1a1: f64 = (l.ff92 * l.ffd4);let t1a2: f64 = (l.ff7c * l.ffd0);let t1a3: f64 = (t1a1 - t1a2);let t1a4: f64 = (0.5 * t1a3);let t1a5: f64 = (t1a4).exp();let t1a6: f64 = (t1a0 * t1a5);l.f4e1 = t1a6;l.f4e2 = 0.0;let t1a7: f64 = (l.f1729 * l.fe0);let t1a8: f64 = (2.0 * l.ffce);let t1a9: f64 = (l.f4e1).ln();let t1aa: f64 = (t1a8 * t1a9);let t1ab: f64 = (t1a7 - t1aa);l.f164f = t1ab;l.f1650 = 0.0;let t1ac: f64 = (0.05 - l.f164f);let t1ad: f64 = (t1ac * l.ffd0);let t1ae: f64 = (t1ad).exp();let t1af: f64 = (1.0 + t1ae);let t1b0: f64 = (t1af).ln();let t1b1: f64 = (l.ffce * t1b0);let t1b2: f64 = (l.f164f + t1b1);l.f1706 = t1b2;l.f1707 = 0.0;let t1b3: f64 = (1.0 / l.f1706);l.f1714 = t1b3;l.f1715 = 0.0;let t1b4: f64 = (l.f1729 * l.f1714);let t1b5: f64 = (t1b4).powf(l.ff63);let t1b6: f64 = (l.f1ac * t1b5);l.f1a3 = t1b6;l.f1a4 = 0.0;let t1b7: f64 = (l.f1a3 * l.f1706);let t1b8: f64 = (t1b7 * l.ff25);l.f1270 = t1b8;l.f1271 = 0.0;let t1b9: f64 = (2.0 * l.f1a3);l.f1260 = t1b9;l.f1261 = 0.0;}
        l.fedd = 1.0;l.fede = 0.0;l.fd8e = 1.0;l.fd8f = 0.0;l.fe1f = 0.0;l.fe20 = 0.0;l.f18e8 = 0.0;l.f18eb = 0.0;l.fe10 = p.p0;l.fe11 = 0.0;l.f18bf = p.p1;l.f18c0 = 0.0;l.f130b = p.p2;l.f130c = 0.0;l.f130d = p.p3;l.f130e = 0.0;l.f1317 = p.p4;l.f1318 = 0.0;l.f130f = p.p8;l.f1310 = 0.0;l.f1a = p.p19;l.f1b = 0.0;l.fe3b = p.p20;l.fe3c = 0.0;l.fe29 = p.p21;l.fe2a = 0.0;l.f16 = p.p22;l.f17 = 0.0;l.fe37 = p.p23;l.fe38 = 0.0;l.fe25 = p.p24;l.fe26 = 0.0;l.fba = p.p25;l.fbb = 0.0;l.f100b = p.p26;l.f100c = 0.0;l.f1c = p.p27;l.f1d = 0.0;l.ff5c = p.p28;l.ff5d = 0.0;l.fdca = p.p14;l.fdcb = 0.0;let t1ba: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };l.f920 = t1ba;l.f92b = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f920 != 0.0) {
            let (t1bd,) = {
    if (p.p9 > 1.0) {
        (p.p9,)
    } else {
        (1.0,)
    }
};
            l.fedd = t1bd;l.fede = 0.0;
        }
        if (l.f920 != 0.0) {let t1be: f64 = (l.fedd + 0.5);let t1bf: f64 = (t1be).floor();l.fedd = t1bf;l.fede = 0.0;let t1c0: f64 = (1.0 / l.fedd);l.fd8e = t1c0;l.fd8f = 0.0;}
        let t1c1: f64 = (l.f18bf * l.fd8e);
        let (t1c3,) = {
    if (t1c1 > 1e-9) {
        let t1c2: f64 = (l.f18bf * l.fd8e);
        (t1c2,)
    } else {
        (1e-9,)
    }
};
        l.f18bf = t1c3;l.f18c0 = 0.0;l.f1311 = p.p5;l.f1312 = 0.0;l.f1313 = p.p6;l.f1314 = 0.0;l.f1315 = p.p7;l.f1316 = 0.0;let t1c4: f64 = (1e-6 / l.fe10);l.fd33 = t1c4;l.fd34 = 0.0;let t1c5: f64 = (1e-6 / l.f18bf);l.fdc6 = t1c5;l.fdc7 = 0.0;let t1c6: f64 = (p.p187 * l.fd33);let t1c7: f64 = (1.0 + t1c6);let t1c8: f64 = (p.p186 * t1c7);let t1c9: f64 = (p.p188 * l.fdc6);let t1ca: f64 = (1.0 + t1c9);let t1cb: f64 = (t1c8 * t1ca);l.f231 = t1cb;l.f232 = 0.0;let t1cc: f64 = (p.p191 * l.fd33);let t1cd: f64 = (1.0 + t1cc);let t1ce: f64 = (p.p190 * t1cd);let t1cf: f64 = (p.p192 * l.fdc6);let t1d0: f64 = (1.0 + t1cf);let t1d1: f64 = (t1ce * t1d0);l.f2a3 = t1d1;l.f2a4 = 0.0;let t1d2: f64 = (l.fe10 + l.f231);let t1d3: f64 = (2.0 * p.p189);let t1d4: f64 = (t1d2 - t1d3);
        let (t1d8,) = {
    if (t1d4 > 1e-9) {
        let t1d5: f64 = (l.fe10 + l.f231);let t1d6: f64 = (2.0 * p.p189);let t1d7: f64 = (t1d5 - t1d6);
        (t1d7,)
    } else {
        (1e-9,)
    }
};
        l.fe1f = t1d8;l.fe20 = 0.0;let t1d9: f64 = (l.f18bf + l.f2a3);let t1da: f64 = (2.0 * p.p193);let t1db: f64 = (t1d9 - t1da);
        let (t1df,) = {
    if (t1db > 1e-9) {
        let t1dc: f64 = (l.f18bf + l.f2a3);let t1dd: f64 = (2.0 * p.p193);let t1de: f64 = (t1dc - t1dd);
        (t1de,)
    } else {
        (1e-9,)
    }
};
        l.f18e8 = t1df;l.f18eb = 0.0;let t1e0: f64 = (1e-6 / l.fe1f);l.fd35 = t1e0;l.fd38 = 0.0;let t1e1: f64 = (l.fd35 * l.fd35);l.fd36 = t1e1;l.fd37 = 0.0;let t1e2: f64 = (1e-6 / l.f18e8);l.fdc8 = t1e2;l.fdc9 = 0.0;let t1e3: f64 = (1.0 / l.fdc8);l.fcd3 = t1e3;l.fcd4 = 0.0;let t1e4: f64 = (l.fd35 * l.fdc8);l.fc70 = t1e4;l.fc71 = 0.0;let t1e5: f64 = (1.0 / l.fc70);l.fcc7 = t1e5;l.fcc8 = 0.0;let t1e6: f64 = (l.fe10 + l.f231);let t1e7: f64 = (2.0 * p.p189);let t1e8: f64 = (t1e6 - t1e7);let t1e9: f64 = (t1e8 + p.p194);
        let (t1ee,) = {
    if (t1e9 > 1e-9) {
        let t1ea: f64 = (l.fe10 + l.f231);let t1eb: f64 = (2.0 * p.p189);let t1ec: f64 = (t1ea - t1eb);let t1ed: f64 = (t1ec + p.p194);
        (t1ed,)
    } else {
        (1e-9,)
    }
};
        l.fe21 = t1ee;l.fe22 = 0.0;
    }
}
