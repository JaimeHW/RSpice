#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let t2f: f64 = if param_given[339] { 1.0 } else { 0.0 };let t30: f64 = if t2f == 1.0 { 1.0 } else { 0.0 };l.f99a = t30;
        if ((l.f976 != 0.0) && (l.f99a != 0.0)) {l.f50d = p.p339;}
        if (l.f976 != 0.0) {l.f519 = p.p338;}
        let t31: f64 = if param_given[340] { 1.0 } else { 0.0 };let t32: f64 = if t31 == 1.0 { 1.0 } else { 0.0 };l.f9a8 = t32;
        if ((l.f976 != 0.0) && (l.f9a8 != 0.0)) {l.f519 = p.p340;}
        if (l.f976 != 0.0) {l.f511 = l.f50d;}
        let t33: f64 = if param_given[341] { 1.0 } else { 0.0 };let t34: f64 = if t33 == 1.0 { 1.0 } else { 0.0 };l.f9b4 = t34;
        if ((l.f976 != 0.0) && (l.f9b4 != 0.0)) {l.f511 = p.p341;}
        if (l.f976 != 0.0) {l.f51d = l.f519;}
        let t35: f64 = if param_given[342] { 1.0 } else { 0.0 };let t36: f64 = if t35 == 1.0 { 1.0 } else { 0.0 };l.f9c0 = t36;
        if ((l.f976 != 0.0) && (l.f9c0 != 0.0)) {l.f51d = p.p342;}
        if (l.f976 != 0.0) {l.f191 = p.p343;let t37: f64 = (p.p344 * p.p234);let t38: f64 = (1e-6 * l.fdc8);let t39: f64 = (t37 / t38);l.f29 = t39;let t3a: f64 = (p.p345 * p.p235);let t3b: f64 = (1e-6 * l.fdc8);let t3c: f64 = (t3a / t3b);l.f2d = t3c;l.f114 = p.p346;l.f11a = p.p347;l.f14d4 = p.p348;l.f14d8 = p.p349;l.f17d = p.p350;l.f181 = p.p351;let t3d: f64 = (8.8541878176e-12 * p.p207);let t3e: f64 = (t3d * l.f18ec);let t3f: f64 = (t3e * l.fe21);let t40: f64 = (t3f / p.p206);l.f1bb = t40;let t41: f64 = (8.8541878176e-12 * p.p207);let t42: f64 = (t41 * l.f18ec);let t43: f64 = (t42 * p.p234);let t44: f64 = (t43 / p.p232);l.f185 = t44;let t45: f64 = (8.8541878176e-12 * p.p207);let t46: f64 = (t45 * l.f18ec);let t47: f64 = (t46 * p.p235);let t48: f64 = (t47 / p.p233);l.f18d = t48;let t49: f64 = (l.fd35).powf(p.p354);let t4a: f64 = (p.p353 * t49);let t4b: f64 = (p.p352 + t4a);let t4c: f64 = (p.p355 * l.fdc8);let t4d: f64 = (t4b + t4c);let t4e: f64 = (p.p356 * l.fc70);let t4f: f64 = (t4d + t4e);l.f29d = t4f;let t50: f64 = (p.p358 * l.fd35);let t51: f64 = (p.p357 + t50);let t52: f64 = (p.p359 * l.fdc8);let t53: f64 = (t51 + t52);let t54: f64 = (p.p360 * l.fc70);let t55: f64 = (t53 + t54);l.f422 = t55;l.f1599 = p.p294;}
        let t56: f64 = if param_given[361] { 1.0 } else { 0.0 };let t57: f64 = if t56 == 1.0 { 1.0 } else { 0.0 };l.f9cc = t57;
        if ((l.f976 != 0.0) && (l.f9cc != 0.0)) {l.f1599 = p.p361;}
        if (l.f976 != 0.0) {l.f1593 = p.p295;}
        let t58: f64 = if param_given[362] { 1.0 } else { 0.0 };let t59: f64 = if t58 == 1.0 { 1.0 } else { 0.0 };l.f9d8 = t59;
        if ((l.f976 != 0.0) && (l.f9d8 != 0.0)) {l.f1593 = p.p362;}
        if (l.f976 != 0.0) {l.f1595 = p.p296;}
        let t5a: f64 = if param_given[363] { 1.0 } else { 0.0 };let t5b: f64 = if t5a == 1.0 { 1.0 } else { 0.0 };l.f9e4 = t5b;
        if ((l.f976 != 0.0) && (l.f9e4 != 0.0)) {l.f1595 = p.p363;}
        if (l.f976 != 0.0) {l.f159b = p.p297;}
        let t5c: f64 = if param_given[364] { 1.0 } else { 0.0 };let t5d: f64 = if t5c == 1.0 { 1.0 } else { 0.0 };l.f9f0 = t5d;
        if ((l.f976 != 0.0) && (l.f9f0 != 0.0)) {l.f159b = p.p364;}
        if (l.f976 != 0.0) {l.f1597 = p.p298;}
        let t5e: f64 = if param_given[365] { 1.0 } else { 0.0 };let t5f: f64 = if t5e == 1.0 { 1.0 } else { 0.0 };l.f9fc = t5f;
        if ((l.f976 != 0.0) && (l.f9fc != 0.0)) {l.f1597 = p.p365;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if (l.f976 != 0.0) {let t60: f64 = (l.f1593 * l.fbe9);let t61: f64 = (t60 / l.f5c7);let t62: f64 = (l.fd35).powf(l.f1595);let t63: f64 = (t61 * t62);let t64: f64 = (l.f1599 + t63);let t65: f64 = (l.f159b * l.fdc8);let t66: f64 = (1.0 + t65);let t67: f64 = (t64 * t66);let t68: f64 = (l.f1597 * l.fc70);let t69: f64 = (1.0 + t68);let t6a: f64 = (t67 * t69);l.f158f = t6a;l.fec = p.p306;}
        let t6b: f64 = if param_given[366] { 1.0 } else { 0.0 };let t6c: f64 = if t6b == 1.0 { 1.0 } else { 0.0 };l.fa08 = t6c;
        if ((l.f976 != 0.0) && (l.fa08 != 0.0)) {l.fec = p.p366;}
        if (l.f976 != 0.0) {l.fea = p.p307;}
        let t6d: f64 = if param_given[367] { 1.0 } else { 0.0 };let t6e: f64 = if t6d == 1.0 { 1.0 } else { 0.0 };l.fa14 = t6e;
        if ((l.f976 != 0.0) && (l.fa14 != 0.0)) {l.fea = p.p367;}
        if (l.f976 != 0.0) {let t6f: f64 = (l.fea * l.fd35);let t70: f64 = (1.0 + t6f);let t71: f64 = (l.fec / t70);l.fe8 = t71;let t72: f64 = (l.fd35).powf(p.p369);let t73: f64 = (p.p368 * t72);let t74: f64 = (p.p370 * l.fdc8);let t75: f64 = (1.0 + t74);let t76: f64 = (t73 * t75);l.f45 = t76;let t77: f64 = (l.fd35).powf(p.p372);l.f15fe = t77;let t78: f64 = (p.p371 * l.f15fe);let t79: f64 = (p.p374 * l.fdc8);let t7a: f64 = (1.0 + t79);let t7b: f64 = (t78 * t7a);let t7c: f64 = (p.p373 * l.fd35);let t7d: f64 = (t7c * l.f15fe);let t7e: f64 = (1.0 + t7d);let t7f: f64 = (t7b / t7e);l.f39 = t7f;l.f464 = p.p375;l.f468 = p.p376;l.f189 = p.p377;let t80: f64 = (p.p378 * l.fcc9);l.f173 = t80;let t81: f64 = (p.p379 * l.fcd5);l.f197 = t81;let t82: f64 = (p.p380 * l.fcd5);l.f19b = t82;l.f353 = p.p381;l.f470 = p.p382;l.f46c = p.p383;l.ff0 = p.p384;let t83: f64 = (p.p385 * l.fcd1);l.f16b = t83;let t84: f64 = (p.p386 * l.fcd1);l.f16f = t84;let t85: f64 = (2.0 * p.p393);let t86: f64 = (t85 / l.fe1f);let t87: f64 = (1.0 - t86);l.f151d = t87;l.f4a7 = p.p387;let t88: f64 = (p.p388 * l.f103);let t89: f64 = (t88 * l.f103);let t8a: f64 = (t89 * l.fdc8);let t8b: f64 = (t8a * l.fdc8);l.f4aa = t8b;let t8c: f64 = (2.0 * p.p395);let t8d: f64 = (p.p396 * l.f18e8);let t8e: f64 = (t8c + t8d);l.f18e9 = t8e;l.f17b1 = p.p397;let t8f: f64 = (p.p399 * l.fd35);let t90: f64 = (p.p398 + t8f);let t91: f64 = (p.p400 * l.fdc8);let t92: f64 = (t90 + t91);let t93: f64 = (p.p401 * l.fc70);let t94: f64 = (t92 + t93);l.f1506 = t94;let t95: f64 = (l.fd35).powf(p.p404);let t96: f64 = (p.p403 * t95);let t97: f64 = (p.p402 + t96);let t98: f64 = (p.p405 * l.fdc8);let t99: f64 = (t97 + t98);let t9a: f64 = (p.p406 * l.fc70);let t9b: f64 = (t99 + t9a);l.f2e0 = t9b;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f976 != 0.0) {let t9c: f64 = (l.fd35).powf(p.p409);let t9d: f64 = (p.p408 * t9c);let t9e: f64 = (1.0 + t9d);let t9f: f64 = (p.p407 * t9e);let ta0: f64 = (p.p410 * l.fdc8);let ta1: f64 = (1.0 + ta0);let ta2: f64 = (t9f * ta1);let ta3: f64 = (p.p411 * l.fc70);let ta4: f64 = (1.0 + ta3);let ta5: f64 = (ta2 * ta4);l.fedb = ta5;let ta6: f64 = (l.fd35).powf(p.p414);let ta7: f64 = (p.p413 * ta6);let ta8: f64 = (p.p412 + ta7);l.f1f3 = ta8;let ta9: f64 = (p.p415 * p.p416);let taa: f64 = (ta9 / l.fe1f);let tab: f64 = (-l.fe1f);let tac: f64 = (tab / p.p416);let tad: f64 = (tac).exp();let tae: f64 = (1.0 - tad);let taf: f64 = (taa * tae);let tb0: f64 = (1.0 + taf);l.f5c8 = tb0;}
        if (l.f976 != 0.0) {
            let (tb1,) = {
    if (l.f5c8 > 1e-15) {
        (l.f5c8,)
    } else {
        (1e-15,)
    }
};
            l.f5c8 = tb1;
        }
        if (l.f976 != 0.0) {let tb2: f64 = (p.p256 * l.f18e9);let tb3: f64 = (l.f5c8 * l.fe1f);let tb4: f64 = (tb2 / tb3);let tb5: f64 = (p.p417 * l.fdc8);let tb6: f64 = (1.0 + tb5);let tb7: f64 = (tb4 * tb6);l.f109 = tb7;let tb8: f64 = (p.p419 * l.fd35);let tb9: f64 = (p.p418 + tb8);let tba: f64 = (p.p420 * l.fdc8);let tbb: f64 = (tb9 + tba);let tbc: f64 = (p.p421 * l.fc70);let tbd: f64 = (tbb + tbc);l.f14d0 = tbd;let tbe: f64 = (l.fd35).powf(p.p423);let tbf: f64 = (p.p422 * tbe);let tc0: f64 = (p.p424 * l.fdc8);let tc1: f64 = (1.0 + tc0);let tc2: f64 = (tbf * tc1);l.f1024 = tc2;l.f1018 = p.p425;l.f1020 = p.p426;let tc3: f64 = (l.fd35).powf(p.p428);let tc4: f64 = (p.p427 * tc3);let tc5: f64 = (p.p429 * l.fdc8);let tc6: f64 = (1.0 + tc5);let tc7: f64 = (tc4 * tc6);l.f167 = tc7;l.f163 = p.p431;l.f15b = p.p430;let tc8: f64 = (p.p808 * l.fd35);let tc9: f64 = (p.p807 + tc8);let tca: f64 = (p.p809 * l.fdc8);let tcb: f64 = (tc9 + tca);let tcc: f64 = (p.p810 * l.fc70);let tcd: f64 = (tcb + tcc);l.fe0d = tcd;let tce: f64 = (p.p812 * l.fd35);let tcf: f64 = (p.p811 + tce);let td0: f64 = (p.p813 * l.fdc8);let td1: f64 = (tcf + td0);let td2: f64 = (p.p814 * l.fc70);let td3: f64 = (td1 + td2);l.fe09 = td3;let td4: f64 = (0.3333333333333333 * l.f18be);let td5: f64 = (td4 / l.fedf);let td6: f64 = (td5 + l.f1a65);let td7: f64 = (p.p440 * td6);let td8: f64 = (l.fedf * l.fe12);let td9: f64 = (td7 / td8);let tda: f64 = (p.p438 + p.p439);let tdb: f64 = (l.f18be * l.fe0f);let tdc: f64 = (tda / tdb);let tdd: f64 = (td9 + tdc);let tde: f64 = (l.fedd * p.p437);let tdf: f64 = (tdd + tde);l.f1298 = tdf;}
        if (l.f976 != 0.0) {
            let (te0,) = {
    if (p.p442 > 0.0) {
        (p.p442,)
    } else {
        (0.0,)
    }
};
            l.f12d5 = te0;
        }
        if (l.f976 != 0.0) {
            let (te1,) = {
    if (p.p443 > 0.0) {
        (p.p443,)
    } else {
        (0.0,)
    }
};
            l.f12d6 = te1;
        }
        let te2: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.fa22 = te2;
        if ((l.f976 != 0.0) && (l.fa22 != 0.0)) {l.f12d6 = l.f12d5;}
        if (l.f976 != 0.0) {let te3: f64 = (l.fedd * p.p12);let te4: f64 = (te3 * l.f12d5);l.f12d0 = te4;let te5: f64 = (l.fedd * p.p13);let te6: f64 = (te5 * l.f12d6);l.f1296 = te6;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if (l.f976 != 0.0) {let te7: f64 = (l.fedd * p.p445);l.f12dc = te7;let te8: f64 = (l.fedd * p.p444);l.f1294 = te8;let te9: f64 = (l.fedd * p.p446);l.f12c4 = te9;let tea: f64 = (l.fedd * p.p447);l.f12c2 = tea;}
        let teb: f64 = if (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]) { 1.0 } else { 0.0 };l.fa2d = teb;
        if ((l.f976 != 0.0) && (l.fa2d != 0.0)) {let tec: f64 = (p.p449 * l.fd35);let ted: f64 = (p.p448 + tec);let tee: f64 = (p.p450 * l.fdc8);let tef: f64 = (ted + tee);let tf0: f64 = (p.p451 * l.fc70);let tf1: f64 = (tef + tf0);l.f17ab = tf1;}
        let tf2: f64 = if (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]) { 1.0 } else { 0.0 };l.fa39 = tf2;
        if ((l.f976 != 0.0) && (l.fa39 != 0.0)) {let tf3: f64 = (p.p453 * l.fd35);let tf4: f64 = (p.p452 + tf3);let tf5: f64 = (p.p454 * l.fdc8);let tf6: f64 = (tf4 + tf5);let tf7: f64 = (p.p455 * l.fc70);let tf8: f64 = (tf6 + tf7);l.f1502 = tf8;}
        let tf9: f64 = if (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]) { 1.0 } else { 0.0 };l.fa47 = tf9;
        if ((l.f976 != 0.0) && (l.fa47 != 0.0)) {let tfa: f64 = (p.p457 * l.fd35);let tfb: f64 = (p.p456 + tfa);let tfc: f64 = (p.p458 * l.fdc8);let tfd: f64 = (tfb + tfc);let tfe: f64 = (p.p459 * l.fc70);let tff: f64 = (tfd + tfe);l.fed5 = tff;}
        let t100: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };l.fa54 = t100;
        if ((l.f976 != 0.0) && (l.fa54 != 0.0)) {let t101: f64 = (p.p461 * l.fd35);let t102: f64 = (p.p460 + t101);let t103: f64 = (p.p462 * l.fdc8);let t104: f64 = (t102 + t103);let t105: f64 = (p.p463 * l.fc70);let t106: f64 = (t104 + t105);l.f567 = t106;}
        let t107: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };l.fa60 = t107;
        if ((l.f976 != 0.0) && (l.fa60 != 0.0)) {let t108: f64 = (p.p465 * l.fd35);let t109: f64 = (p.p464 + t108);let t10a: f64 = (p.p466 * l.fdc8);let t10b: f64 = (t109 + t10a);let t10c: f64 = (p.p467 * l.fc70);let t10d: f64 = (t10b + t10c);l.f1868 = t10d;}
        let t10e: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };l.fa6c = t10e;
        if ((l.f976 != 0.0) && (l.fa6c != 0.0)) {let t10f: f64 = (p.p469 * l.fd35);let t110: f64 = (p.p468 + t10f);let t111: f64 = (p.p470 * l.fdc8);let t112: f64 = (t110 + t111);let t113: f64 = (p.p471 * l.fc70);let t114: f64 = (t112 + t113);l.f2dc = t114;}
        let t115: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };l.fa78 = t115;
        if ((l.f976 != 0.0) && (l.fa78 != 0.0)) {let t116: f64 = (p.p473 * l.fd35);let t117: f64 = (p.p472 + t116);let t118: f64 = (p.p474 * l.fdc8);let t119: f64 = (t117 + t118);let t11a: f64 = (p.p475 * l.fc70);let t11b: f64 = (t119 + t11a);l.feeb = t11b;}
        let t11c: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };l.fa84 = t11c;
        if ((l.f976 != 0.0) && (l.fa84 != 0.0)) {let t11d: f64 = (p.p477 * l.fd35);let t11e: f64 = (p.p476 + t11d);let t11f: f64 = (p.p478 * l.fdc8);let t120: f64 = (t11e + t11f);let t121: f64 = (p.p479 * l.fc70);let t122: f64 = (t120 + t121);l.fee2 = t122;}
        let t123: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };l.fa90 = t123;
        if ((l.f976 != 0.0) && (l.fa90 != 0.0)) {let t124: f64 = (p.p481 * l.fd35);let t125: f64 = (p.p480 + t124);let t126: f64 = (p.p482 * l.fdc8);let t127: f64 = (t125 + t126);let t128: f64 = (p.p483 * l.fc70);let t129: f64 = (t127 + t128);l.fee6 = t129;}
        let t12a: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };l.fa9e = t12a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.fa9e != 0.0)) {let t12b: f64 = (p.p485 * l.fd35);let t12c: f64 = (p.p484 + t12b);let t12d: f64 = (p.p486 * l.fdc8);let t12e: f64 = (t12c + t12d);let t12f: f64 = (p.p487 * l.fc70);let t130: f64 = (t12e + t12f);l.f1e3 = t130;}
        let t131: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };l.faaa = t131;
        if ((l.f976 != 0.0) && (l.faaa != 0.0)) {let t132: f64 = (p.p493 * l.fd35);let t133: f64 = (p.p492 + t132);let t134: f64 = (p.p494 * l.fdc8);let t135: f64 = (t133 + t134);let t136: f64 = (p.p495 * l.fc70);let t137: f64 = (t135 + t136);l.f1f7 = t137;}
        let t138: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };l.fab6 = t138;
        if ((l.f976 != 0.0) && (l.fab6 != 0.0)) {let t139: f64 = (p.p489 * l.fd35);let t13a: f64 = (p.p488 + t139);let t13b: f64 = (p.p490 * l.fdc8);let t13c: f64 = (t13a + t13b);let t13d: f64 = (p.p491 * l.fc70);let t13e: f64 = (t13c + t13d);l.f1ef = t13e;}
        let t13f: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };l.fac2 = t13f;
        if ((l.f976 != 0.0) && (l.fac2 != 0.0)) {let t140: f64 = (p.p497 * l.fd35);let t141: f64 = (p.p496 + t140);let t142: f64 = (p.p498 * l.fdc8);let t143: f64 = (t141 + t142);let t144: f64 = (p.p499 * l.fc70);let t145: f64 = (t143 + t144);l.f14e0 = t145;}
        let t146: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };l.face = t146;
        if ((l.f976 != 0.0) && (l.face != 0.0)) {let t147: f64 = (p.p501 * l.fd35);let t148: f64 = (p.p500 + t147);let t149: f64 = (p.p502 * l.fdc8);let t14a: f64 = (t148 + t149);let t14b: f64 = (p.p503 * l.fc70);let t14c: f64 = (t14a + t14b);let t14d: f64 = (l.fd36 * t14c);l.f153 = t14d;}
        let t14e: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };l.fada = t14e;
        if ((l.f976 != 0.0) && (l.fada != 0.0)) {let t14f: f64 = (p.p509 * l.fd35);let t150: f64 = (p.p508 + t14f);let t151: f64 = (p.p510 * l.fdc8);let t152: f64 = (t150 + t151);let t153: f64 = (p.p511 * l.fc70);let t154: f64 = (t152 + t153);l.f15f = t154;}
        let t155: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };l.fae6 = t155;
        if ((l.f976 != 0.0) && (l.fae6 != 0.0)) {let t156: f64 = (p.p505 * l.fd35);let t157: f64 = (p.p504 + t156);let t158: f64 = (p.p506 * l.fdc8);let t159: f64 = (t157 + t158);let t15a: f64 = (p.p507 * l.fc70);let t15b: f64 = (t159 + t15a);l.f157 = t15b;}
        let t15c: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };l.faf2 = t15c;
        if ((l.f976 != 0.0) && (l.faf2 != 0.0)) {let t15d: f64 = (p.p513 * l.fd35);let t15e: f64 = (p.p512 + t15d);let t15f: f64 = (p.p514 * l.fdc8);let t160: f64 = (t15e + t15f);let t161: f64 = (p.p515 * l.fc70);let t162: f64 = (t160 + t161);let t163: f64 = (l.fd36 * t162);l.f1010 = t163;}
        let t164: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };l.fafe = t164;
        if ((l.f976 != 0.0) && (l.fafe != 0.0)) {let t165: f64 = (p.p521 * l.fd35);let t166: f64 = (p.p520 + t165);let t167: f64 = (p.p522 * l.fdc8);let t168: f64 = (t166 + t167);let t169: f64 = (p.p523 * l.fc70);let t16a: f64 = (t168 + t169);l.f101c = t16a;}
        let t16b: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };l.fb0a = t16b;
        if ((l.f976 != 0.0) && (l.fb0a != 0.0)) {let t16c: f64 = (p.p517 * l.fd35);let t16d: f64 = (p.p516 + t16c);let t16e: f64 = (p.p518 * l.fdc8);let t16f: f64 = (t16d + t16e);let t170: f64 = (p.p519 * l.fc70);let t171: f64 = (t16f + t170);l.f1014 = t171;}
        let t172: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };l.fb18 = t172;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.fb18 != 0.0)) {let t173: f64 = (l.f18e8 / l.fe1f);let t174: f64 = (p.p525 * l.fd35);let t175: f64 = (p.p524 + t174);let t176: f64 = (p.p526 * l.fdc8);let t177: f64 = (t175 + t176);let t178: f64 = (p.p527 * l.fc70);let t179: f64 = (t177 + t178);let t17a: f64 = (t173 * t179);l.f103 = t17a;}
        let t17b: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };l.fb24 = t17b;
        if ((l.f976 != 0.0) && (l.fb24 != 0.0)) {let t17c: f64 = (p.p529 * l.fd35);let t17d: f64 = (p.p528 + t17c);let t17e: f64 = (p.p530 * l.fdc8);let t17f: f64 = (t17d + t17e);let t180: f64 = (p.p531 * l.fc70);let t181: f64 = (t17f + t180);l.f14cc = t181;}
        let t182: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };l.fb30 = t182;
        if ((l.f976 != 0.0) && (l.fb30 != 0.0)) {let t183: f64 = (p.p533 * l.fd35);let t184: f64 = (p.p532 + t183);let t185: f64 = (p.p534 * l.fdc8);let t186: f64 = (t184 + t185);let t187: f64 = (p.p535 * l.fc70);let t188: f64 = (t186 + t187);l.febb = t188;}
        let t189: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };l.fb3c = t189;
        if ((l.f976 != 0.0) && (l.fb3c != 0.0)) {let t18a: f64 = (p.p537 * l.fd35);let t18b: f64 = (p.p536 + t18a);let t18c: f64 = (p.p538 * l.fdc8);let t18d: f64 = (t18b + t18c);let t18e: f64 = (p.p539 * l.fc70);let t18f: f64 = (t18d + t18e);l.f1564 = t18f;}
        let t190: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };l.fb48 = t190;
        if ((l.f976 != 0.0) && (l.fb48 != 0.0)) {let t191: f64 = (p.p541 * l.fd35);let t192: f64 = (p.p540 + t191);let t193: f64 = (p.p542 * l.fdc8);let t194: f64 = (t192 + t193);let t195: f64 = (p.p543 * l.fc70);let t196: f64 = (t194 + t195);l.f1cb = t196;}
        let t197: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };l.fb54 = t197;
        if ((l.f976 != 0.0) && (l.fb54 != 0.0)) {let t198: f64 = (p.p545 * l.fd35);let t199: f64 = (p.p544 + t198);let t19a: f64 = (p.p546 * l.fdc8);let t19b: f64 = (t199 + t19a);let t19c: f64 = (p.p547 * l.fc70);let t19d: f64 = (t19b + t19c);l.f155e = t19d;}
        let t19e: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };l.fb60 = t19e;
        if ((l.f976 != 0.0) && (l.fb60 != 0.0)) {let t19f: f64 = (p.p549 * l.fd35);let t1a0: f64 = (p.p548 + t19f);let t1a1: f64 = (p.p550 * l.fdc8);let t1a2: f64 = (t1a0 + t1a1);let t1a3: f64 = (p.p551 * l.fc70);let t1a4: f64 = (t1a2 + t1a3);l.f19bc = t1a4;}
        let t1a5: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };l.fb6c = t1a5;
        if ((l.f976 != 0.0) && (l.fb6c != 0.0)) {let t1a6: f64 = (p.p553 * l.fd35);let t1a7: f64 = (p.p552 + t1a6);let t1a8: f64 = (p.p554 * l.fdc8);let t1a9: f64 = (t1a7 + t1a8);let t1aa: f64 = (p.p555 * l.fc70);let t1ab: f64 = (t1a9 + t1aa);let t1ac: f64 = (l.fdc8 * t1ab);l.f12c7 = t1ac;}
        let t1ad: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };l.fb78 = t1ad;
        if ((l.f976 != 0.0) && (l.fb78 != 0.0)) {let t1ae: f64 = (p.p557 * l.fd35);let t1af: f64 = (p.p556 + t1ae);let t1b0: f64 = (p.p558 * l.fdc8);let t1b1: f64 = (t1af + t1b0);let t1b2: f64 = (p.p559 * l.fc70);let t1b3: f64 = (t1b1 + t1b2);l.f14f2 = t1b3;}
        let t1b4: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };l.fb84 = t1b4;
        if ((l.f976 != 0.0) && (l.fb84 != 0.0)) {let t1b5: f64 = (p.p561 * l.fd35);let t1b6: f64 = (p.p560 + t1b5);let t1b7: f64 = (p.p562 * l.fdc8);let t1b8: f64 = (t1b6 + t1b7);let t1b9: f64 = (p.p563 * l.fc70);let t1ba: f64 = (t1b8 + t1b9);l.f12cd = t1ba;}
        let t1bb: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };l.fb91 = t1bb;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.fb91 != 0.0)) {let t1bc: f64 = (p.p565 * l.fd35);let t1bd: f64 = (p.p564 + t1bc);let t1be: f64 = (p.p566 * l.fdc8);let t1bf: f64 = (t1bd + t1be);let t1c0: f64 = (p.p567 * l.fc70);let t1c1: f64 = (t1bf + t1c0);l.f12d3 = t1c1;}
        let t1c2: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };l.fb9d = t1c2;
        if ((l.f976 != 0.0) && (l.fb9d != 0.0)) {let t1c3: f64 = (p.p569 * l.fd35);let t1c4: f64 = (p.p568 + t1c3);let t1c5: f64 = (p.p570 * l.fdc8);let t1c6: f64 = (t1c4 + t1c5);let t1c7: f64 = (p.p571 * l.fc70);let t1c8: f64 = (t1c6 + t1c7);let t1c9: f64 = (l.fd35 * t1c8);l.f1589 = t1c9;}
        let t1ca: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };l.fbab = t1ca;
        if ((l.f976 != 0.0) && (l.fbab != 0.0)) {let t1cb: f64 = (p.p573 * l.fd35);let t1cc: f64 = (p.p572 + t1cb);let t1cd: f64 = (p.p574 * l.fdc8);let t1ce: f64 = (t1cc + t1cd);let t1cf: f64 = (p.p575 * l.fc70);let t1d0: f64 = (t1ce + t1cf);l.f14fe = t1d0;}
        let t1d1: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };l.fbb0 = t1d1;
        if ((l.f976 != 0.0) && (l.fbb0 != 0.0)) {let t1d2: f64 = (p.p577 * l.fd35);let t1d3: f64 = (p.p576 + t1d2);let t1d4: f64 = (p.p578 * l.fdc8);let t1d5: f64 = (t1d3 + t1d4);let t1d6: f64 = (p.p579 * l.fc70);let t1d7: f64 = (t1d5 + t1d6);l.f159f = t1d7;}
        let t1d8: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };l.fbb2 = t1d8;
        if ((l.f976 != 0.0) && (l.fbb2 != 0.0)) {let t1d9: f64 = (p.p581 * l.fd35);let t1da: f64 = (p.p580 + t1d9);let t1db: f64 = (p.p582 * l.fdc8);let t1dc: f64 = (t1da + t1db);let t1dd: f64 = (p.p583 * l.fc70);let t1de: f64 = (t1dc + t1dd);l.f15bb = t1de;}
        let t1df: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };l.fbb4 = t1df;
        if ((l.f976 != 0.0) && (l.fbb4 != 0.0)) {let t1e0: f64 = (p.p585 * l.fd35);let t1e1: f64 = (p.p584 + t1e0);let t1e2: f64 = (p.p586 * l.fdc8);let t1e3: f64 = (t1e1 + t1e2);let t1e4: f64 = (p.p587 * l.fc70);let t1e5: f64 = (t1e3 + t1e4);l.fe4 = t1e5;}
        let t1e6: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };l.fbb6 = t1e6;
        if ((l.f976 != 0.0) && (l.fbb6 != 0.0)) {let t1e7: f64 = (p.p589 * l.fd35);let t1e8: f64 = (p.p588 + t1e7);let t1e9: f64 = (p.p590 * l.fdc8);let t1ea: f64 = (t1e8 + t1e9);let t1eb: f64 = (p.p591 * l.fc70);let t1ec: f64 = (t1ea + t1eb);let t1ed: f64 = (l.fd35 * t1ec);l.f41 = t1ed;}
        let t1ee: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };l.fbb8 = t1ee;
        if ((l.f976 != 0.0) && (l.fbb8 != 0.0)) {let t1ef: f64 = (p.p593 * l.fd35);let t1f0: f64 = (p.p592 + t1ef);let t1f1: f64 = (p.p594 * l.fdc8);let t1f2: f64 = (t1f0 + t1f1);let t1f3: f64 = (p.p595 * l.fc70);let t1f4: f64 = (t1f2 + t1f3);l.f35 = t1f4;}
        let t1f5: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };l.fbba = t1f5;
        if ((l.f976 != 0.0) && (l.fbba != 0.0)) {let t1f6: f64 = (p.p597 * l.fd35);let t1f7: f64 = (p.p596 + t1f6);let t1f8: f64 = (p.p598 * l.fdc8);let t1f9: f64 = (t1f7 + t1f8);let t1fa: f64 = (p.p599 * l.fc70);let t1fb: f64 = (t1f9 + t1fa);l.f3d = t1fb;}
        let t1fc: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };l.fbbc = t1fc;
        if ((l.f976 != 0.0) && (l.fbbc != 0.0)) {let t1fd: f64 = (p.p601 * l.fd35);let t1fe: f64 = (p.p600 + t1fd);let t1ff: f64 = (p.p602 * l.fdc8);let t200: f64 = (t1fe + t1ff);let t201: f64 = (p.p603 * l.fc70);let t202: f64 = (t200 + t201);l.f2 = t202;}
        let t203: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };l.fbbe = t203;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.fbbe != 0.0)) {let t204: f64 = (p.p605 * l.fd35);let t205: f64 = (p.p604 + t204);let t206: f64 = (p.p606 * l.fdc8);let t207: f64 = (t205 + t206);let t208: f64 = (p.p607 * l.fc70);let t209: f64 = (t207 + t208);l.f14c8 = t209;}
        let t20a: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };l.fbc0 = t20a;
        if ((l.f976 != 0.0) && (l.fbc0 != 0.0)) {let t20b: f64 = (p.p609 * l.fd35);let t20c: f64 = (p.p608 + t20b);let t20d: f64 = (p.p610 * l.fdc8);let t20e: f64 = (t20c + t20d);let t20f: f64 = (p.p611 * l.fc70);let t210: f64 = (t20e + t20f);l.fc = t210;}
        let t211: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };l.fbc2 = t211;
        if ((l.f976 != 0.0) && (l.fbc2 != 0.0)) {let t212: f64 = (p.p613 * l.fd35);let t213: f64 = (p.p612 + t212);let t214: f64 = (p.p614 * l.fdc8);let t215: f64 = (t213 + t214);let t216: f64 = (p.p615 * l.fc70);let t217: f64 = (t215 + t216);l.f10 = t217;}
        let t218: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };l.fbc4 = t218;
        if ((l.f976 != 0.0) && (l.fbc4 != 0.0)) {let t219: f64 = (p.p617 * l.fd35);let t21a: f64 = (p.p616 + t219);let t21b: f64 = (p.p618 * l.fdc8);let t21c: f64 = (t21a + t21b);let t21d: f64 = (p.p619 * l.fc70);let t21e: f64 = (t21c + t21d);let t21f: f64 = (l.fcc7 * t21e);l.fcb8 = t21f;}
        let t220: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };l.fbc6 = t220;
        if ((l.f976 != 0.0) && (l.fbc6 != 0.0)) {let t221: f64 = (p.p621 * l.fd35);let t222: f64 = (p.p620 + t221);let t223: f64 = (p.p622 * l.fdc8);let t224: f64 = (t222 + t223);let t225: f64 = (p.p623 * l.fc70);let t226: f64 = (t224 + t225);let t227: f64 = (l.fcd3 * t226);l.fcbc = t227;}
        let t228: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };l.fbc8 = t228;
        if ((l.f976 != 0.0) && (l.fbc8 != 0.0)) {let t229: f64 = (p.p625 * l.fd35);let t22a: f64 = (p.p624 + t229);let t22b: f64 = (p.p626 * l.fdc8);let t22c: f64 = (t22a + t22b);let t22d: f64 = (p.p627 * l.fc70);let t22e: f64 = (t22c + t22d);let t22f: f64 = (l.fcd3 * t22e);l.fcc0 = t22f;}
        let t230: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };l.fbca = t230;
        if ((l.f976 != 0.0) && (l.fbca != 0.0)) {let t231: f64 = (p.p629 * l.fd35);let t232: f64 = (p.p628 + t231);let t233: f64 = (p.p630 * l.fdc8);let t234: f64 = (t232 + t233);let t235: f64 = (p.p631 * l.fc70);let t236: f64 = (t234 + t235);l.f14ea = t236;}
        let t237: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };l.fbcc = t237;
        if ((l.f976 != 0.0) && (l.fbcc != 0.0)) {let t238: f64 = (p.p633 * l.fd35);let t239: f64 = (p.p632 + t238);let t23a: f64 = (p.p634 * l.fdc8);let t23b: f64 = (t239 + t23a);let t23c: f64 = (p.p635 * l.fc70);let t23d: f64 = (t23b + t23c);let t23e: f64 = (l.fcd3 * t23d);l.f29 = t23e;}
        let t23f: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };l.fbce = t23f;
        if ((l.f976 != 0.0) && (l.fbce != 0.0)) {let t240: f64 = (p.p637 * l.fd35);let t241: f64 = (p.p636 + t240);let t242: f64 = (p.p638 * l.fdc8);let t243: f64 = (t241 + t242);let t244: f64 = (p.p639 * l.fc70);let t245: f64 = (t243 + t244);let t246: f64 = (l.fcd3 * t245);l.f2d = t246;}
        let t247: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };l.fbd0 = t247;
        if ((l.f976 != 0.0) && (l.fbd0 != 0.0)) {let t248: f64 = (p.p641 * l.fd35);let t249: f64 = (p.p640 + t248);let t24a: f64 = (p.p642 * l.fdc8);let t24b: f64 = (t249 + t24a);let t24c: f64 = (p.p643 * l.fc70);let t24d: f64 = (t24b + t24c);l.f14d4 = t24d;}
        let t24e: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };l.f5e5 = t24e;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f5e5 != 0.0)) {let t24f: f64 = (p.p645 * l.fd35);let t250: f64 = (p.p644 + t24f);let t251: f64 = (p.p646 * l.fdc8);let t252: f64 = (t250 + t251);let t253: f64 = (p.p647 * l.fc70);let t254: f64 = (t252 + t253);l.f14d8 = t254;}
        let t255: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };l.f5e7 = t255;
        if ((l.f976 != 0.0) && (l.f5e7 != 0.0)) {let t256: f64 = (l.fcd5 * l.fe21);let t257: f64 = (t256 / 1e-6);let t258: f64 = (p.p649 * l.fd35);let t259: f64 = (p.p648 + t258);let t25a: f64 = (p.p650 * l.fdc8);let t25b: f64 = (t259 + t25a);let t25c: f64 = (p.p651 * l.fc70);let t25d: f64 = (t25b + t25c);let t25e: f64 = (t257 * t25d);l.f1bb = t25e;}
        let t25f: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };l.f5ed = t25f;
        if ((l.f976 != 0.0) && (l.f5ed != 0.0)) {let t260: f64 = (p.p653 * l.fd35);let t261: f64 = (p.p652 + t260);let t262: f64 = (p.p654 * l.fdc8);let t263: f64 = (t261 + t262);let t264: f64 = (p.p655 * l.fc70);let t265: f64 = (t263 + t264);l.f29d = t265;}
        let t266: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };l.f5ef = t266;
        if ((l.f976 != 0.0) && (l.f5ef != 0.0)) {let t267: f64 = (p.p657 * l.fd35);let t268: f64 = (p.p656 + t267);let t269: f64 = (p.p658 * l.fdc8);let t26a: f64 = (t268 + t269);let t26b: f64 = (p.p659 * l.fc70);let t26c: f64 = (t26a + t26b);l.f422 = t26c;}
        let t26d: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };l.f5f1 = t26d;
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.fff8 = p.p568;}
        let t26e: f64 = if param_given[660] { 1.0 } else { 0.0 };let t26f: f64 = if t26e == 1.0 { 1.0 } else { 0.0 };l.f5f3 = t26f;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f3 != 0.0)) {l.fff8 = p.p660;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.ffe6 = p.p569;}
        let t270: f64 = if param_given[661] { 1.0 } else { 0.0 };let t271: f64 = if t270 == 1.0 { 1.0 } else { 0.0 };l.f5f5 = t271;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f5 != 0.0)) {l.ffe6 = p.p661;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.f1030 = p.p570;}
        let t272: f64 = if param_given[662] { 1.0 } else { 0.0 };let t273: f64 = if t272 == 1.0 { 1.0 } else { 0.0 };l.f5f7 = t273;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f7 != 0.0)) {l.f1030 = p.p662;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.ffe8 = p.p571;}
        let t274: f64 = if param_given[663] { 1.0 } else { 0.0 };let t275: f64 = if t274 == 1.0 { 1.0 } else { 0.0 };l.f5f9 = t275;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f9 != 0.0)) {l.ffe8 = p.p663;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {let t276: f64 = (l.ffe6 * l.fd35);let t277: f64 = (l.fff8 + t276);let t278: f64 = (l.f1030 * l.fdc8);let t279: f64 = (t277 + t278);let t27a: f64 = (l.ffe8 * l.fc70);let t27b: f64 = (t279 + t27a);let t27c: f64 = (l.fd35 * t27b);l.f158f = t27c;}
        let t27d: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };l.f5fb = t27d;
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.fff8 = p.p584;}
        let t27e: f64 = if param_given[664] { 1.0 } else { 0.0 };let t27f: f64 = if t27e == 1.0 { 1.0 } else { 0.0 };l.f5fd = t27f;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f5fd != 0.0)) {l.fff8 = p.p664;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.ffe6 = p.p585;}
        let t280: f64 = if param_given[665] { 1.0 } else { 0.0 };let t281: f64 = if t280 == 1.0 { 1.0 } else { 0.0 };l.f5ff = t281;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f5ff != 0.0)) {l.ffe6 = p.p665;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.f1030 = p.p586;}
        let t282: f64 = if param_given[666] { 1.0 } else { 0.0 };let t283: f64 = if t282 == 1.0 { 1.0 } else { 0.0 };l.f601 = t283;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f601 != 0.0)) {l.f1030 = p.p666;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.ffe8 = p.p587;}
        let t284: f64 = if param_given[667] { 1.0 } else { 0.0 };let t285: f64 = if t284 == 1.0 { 1.0 } else { 0.0 };l.f603 = t285;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f603 != 0.0)) {l.ffe8 = p.p667;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {let t286: f64 = (l.ffe6 * l.fd35);let t287: f64 = (l.fff8 + t286);let t288: f64 = (l.f1030 * l.fdc8);let t289: f64 = (t287 + t288);let t28a: f64 = (l.ffe8 * l.fc70);let t28b: f64 = (t289 + t28a);let t28c: f64 = t28b;l.fe8 = t28c;}
        let t28d: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };l.f605 = t28d;
        if ((l.f976 != 0.0) && (l.f605 != 0.0)) {let t28e: f64 = (p.p669 * l.fd35);let t28f: f64 = (p.p668 + t28e);let t290: f64 = (p.p670 * l.fdc8);let t291: f64 = (t28f + t290);let t292: f64 = (p.p671 * l.fc70);let t293: f64 = (t291 + t292);let t294: f64 = (l.fd35 * t293);l.f45 = t294;}
        let t295: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };l.f607 = t295;
        if ((l.f976 != 0.0) && (l.f607 != 0.0)) {let t296: f64 = (p.p673 * l.fd35);let t297: f64 = (p.p672 + t296);let t298: f64 = (p.p674 * l.fdc8);let t299: f64 = (t297 + t298);let t29a: f64 = (p.p675 * l.fc70);let t29b: f64 = (t299 + t29a);let t29c: f64 = (l.fd35 * t29b);l.f39 = t29c;}
        let t29d: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };l.f609 = t29d;
        if ((l.f976 != 0.0) && (l.f609 != 0.0)) {let t29e: f64 = (p.p677 * l.fd35);let t29f: f64 = (p.p676 + t29e);let t2a0: f64 = (p.p678 * l.fdc8);let t2a1: f64 = (t29f + t2a0);let t2a2: f64 = (p.p679 * l.fc70);let t2a3: f64 = (t2a1 + t2a2);let t2a4: f64 = (l.fcd5 * t2a3);l.f185 = t2a4;}
        let t2a5: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };l.f60b = t2a5;
        if ((l.f976 != 0.0) && (l.f60b != 0.0)) {let t2a6: f64 = (p.p681 * l.fd35);let t2a7: f64 = (p.p680 + t2a6);let t2a8: f64 = (p.p682 * l.fdc8);let t2a9: f64 = (t2a7 + t2a8);let t2aa: f64 = (p.p683 * l.fc70);let t2ab: f64 = (t2a9 + t2aa);let t2ac: f64 = (l.fcd5 * t2ab);l.f18d = t2ac;}
        let t2ad: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };l.f61d = t2ad;
        if ((l.f976 != 0.0) && (l.f61d != 0.0)) {let t2ae: f64 = (p.p685 * l.fd35);let t2af: f64 = (p.p684 + t2ae);let t2b0: f64 = (p.p686 * l.fdc8);let t2b1: f64 = (t2af + t2b0);let t2b2: f64 = (p.p687 * l.fc70);let t2b3: f64 = (t2b1 + t2b2);let t2b4: f64 = (l.fcc9 * t2b3);l.f173 = t2b4;}
        let t2b5: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };l.f633 = t2b5;
        if ((l.f976 != 0.0) && (l.f633 != 0.0)) {let t2b6: f64 = (p.p689 * l.fd35);let t2b7: f64 = (p.p688 + t2b6);let t2b8: f64 = (p.p690 * l.fdc8);let t2b9: f64 = (t2b7 + t2b8);let t2ba: f64 = (p.p691 * l.fc70);let t2bb: f64 = (t2b9 + t2ba);let t2bc: f64 = (l.fcd5 * t2bb);l.f197 = t2bc;}
        let t2bd: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };l.f649 = t2bd;
        if ((l.f976 != 0.0) && (l.f649 != 0.0)) {let t2be: f64 = (p.p693 * l.fd35);let t2bf: f64 = (p.p692 + t2be);let t2c0: f64 = (p.p694 * l.fdc8);let t2c1: f64 = (t2bf + t2c0);let t2c2: f64 = (p.p695 * l.fc70);let t2c3: f64 = (t2c1 + t2c2);let t2c4: f64 = (l.fcd5 * t2c3);l.f19b = t2c4;}
        let t2c5: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };l.f65f = t2c5;
        if ((l.f976 != 0.0) && (l.f65f != 0.0)) {let t2c6: f64 = (p.p697 * l.fd35);let t2c7: f64 = (p.p696 + t2c6);let t2c8: f64 = (p.p698 * l.fdc8);let t2c9: f64 = (t2c7 + t2c8);let t2ca: f64 = (p.p699 * l.fc70);let t2cb: f64 = (t2c9 + t2ca);let t2cc: f64 = (l.fcd1 * t2cb);l.f16b = t2cc;}
        let t2cd: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };l.f675 = t2cd;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f675 != 0.0)) {let t2ce: f64 = (p.p701 * l.fd35);let t2cf: f64 = (p.p700 + t2ce);let t2d0: f64 = (p.p702 * l.fdc8);let t2d1: f64 = (t2cf + t2d0);let t2d2: f64 = (p.p703 * l.fc70);let t2d3: f64 = (t2d1 + t2d2);let t2d4: f64 = (l.fcd1 * t2d3);l.f16f = t2d4;}
        let t2d5: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };l.f687 = t2d5;
        if ((l.f976 != 0.0) && (l.f687 != 0.0)) {let t2d6: f64 = (p.p705 * l.fd35);let t2d7: f64 = (p.p704 + t2d6);let t2d8: f64 = (p.p706 * l.fdc8);let t2d9: f64 = (t2d7 + t2d8);let t2da: f64 = (p.p707 * l.fc70);let t2db: f64 = (t2d9 + t2da);let t2dc: f64 = (l.fd36 * t2db);l.f4aa = t2dc;}
        let t2dd: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };l.f6c6 = t2dd;
        if ((l.f976 != 0.0) && (l.f6c6 != 0.0)) {let t2de: f64 = (p.p721 * l.fd35);let t2df: f64 = (p.p720 + t2de);let t2e0: f64 = (p.p722 * l.fdc8);let t2e1: f64 = (t2df + t2e0);let t2e2: f64 = (p.p723 * l.fc70);let t2e3: f64 = (t2e1 + t2e2);l.f17b1 = t2e3;}
        let t2e4: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };l.f6c8 = t2e4;
        if ((l.f976 != 0.0) && (l.f6c8 != 0.0)) {let t2e5: f64 = (p.p725 * l.fd35);let t2e6: f64 = (p.p724 + t2e5);let t2e7: f64 = (p.p726 * l.fdc8);let t2e8: f64 = (t2e6 + t2e7);let t2e9: f64 = (p.p727 * l.fc70);let t2ea: f64 = (t2e8 + t2e9);l.f1506 = t2ea;}
        let t2eb: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };l.f6ca = t2eb;
        if ((l.f976 != 0.0) && (l.f6ca != 0.0)) {let t2ec: f64 = (p.p729 * l.fd35);let t2ed: f64 = (p.p728 + t2ec);let t2ee: f64 = (p.p730 * l.fdc8);let t2ef: f64 = (t2ed + t2ee);let t2f0: f64 = (p.p731 * l.fc70);let t2f1: f64 = (t2ef + t2f0);l.f2e0 = t2f1;}
        let t2f2: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };l.f6cc = t2f2;
        if ((l.f976 != 0.0) && (l.f6cc != 0.0)) {let t2f3: f64 = (p.p733 * l.fd35);let t2f4: f64 = (p.p732 + t2f3);let t2f5: f64 = (p.p734 * l.fdc8);let t2f6: f64 = (t2f4 + t2f5);let t2f7: f64 = (p.p735 * l.fc70);let t2f8: f64 = (t2f6 + t2f7);l.fedb = t2f8;}
        let t2f9: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };l.f6ce = t2f9;
        if ((l.f976 != 0.0) && (l.f6ce != 0.0)) {let t2fa: f64 = (p.p737 * l.fd35);let t2fb: f64 = (p.p736 + t2fa);let t2fc: f64 = (p.p738 * l.fdc8);let t2fd: f64 = (t2fb + t2fc);let t2fe: f64 = (p.p739 * l.fc70);let t2ff: f64 = (t2fd + t2fe);l.f1f3 = t2ff;}
        let t300: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };l.f6d0 = t300;
        if ((l.f976 != 0.0) && (l.f6d0 != 0.0)) {let t301: f64 = (l.f18e9 / l.fe1f);let t302: f64 = (p.p741 * l.fd35);let t303: f64 = (p.p740 + t302);let t304: f64 = (p.p742 * l.fdc8);let t305: f64 = (t303 + t304);let t306: f64 = (p.p743 * l.fc70);let t307: f64 = (t305 + t306);let t308: f64 = (t301 * t307);l.f109 = t308;}
        let t309: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };l.f6d2 = t309;
        if ((l.f976 != 0.0) && (l.f6d2 != 0.0)) {let t30a: f64 = (p.p745 * l.fd35);let t30b: f64 = (p.p744 + t30a);let t30c: f64 = (p.p746 * l.fdc8);let t30d: f64 = (t30b + t30c);let t30e: f64 = (p.p747 * l.fc70);let t30f: f64 = (t30d + t30e);l.f14d0 = t30f;}
        let t310: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };l.f6d4 = t310;
        if ((l.f976 != 0.0) && (l.f6d4 != 0.0)) {let t311: f64 = (p.p749 * l.fd35);let t312: f64 = (p.p748 + t311);let t313: f64 = (p.p750 * l.fdc8);let t314: f64 = (t312 + t313);let t315: f64 = (p.p751 * l.fc70);let t316: f64 = (t314 + t315);let t317: f64 = (l.fd36 * t316);l.f1024 = t317;}
        let t318: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };l.f6d6 = t318;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f6d6 != 0.0)) {let t319: f64 = (p.p753 * l.fd35);let t31a: f64 = (p.p752 + t319);let t31b: f64 = (p.p754 * l.fdc8);let t31c: f64 = (t31a + t31b);let t31d: f64 = (p.p755 * l.fc70);let t31e: f64 = (t31c + t31d);l.f1018 = t31e;}
        let t31f: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };l.f6d8 = t31f;
        if ((l.f976 != 0.0) && (l.f6d8 != 0.0)) {let t320: f64 = (p.p757 * l.fd35);let t321: f64 = (p.p756 + t320);let t322: f64 = (p.p758 * l.fdc8);let t323: f64 = (t321 + t322);let t324: f64 = (p.p759 * l.fc70);let t325: f64 = (t323 + t324);l.f1020 = t325;}
        let t326: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };l.f6da = t326;
        if ((l.f976 != 0.0) && (l.f6da != 0.0)) {let t327: f64 = (p.p761 * l.fd35);let t328: f64 = (p.p760 + t327);let t329: f64 = (p.p762 * l.fdc8);let t32a: f64 = (t328 + t329);let t32b: f64 = (p.p763 * l.fc70);let t32c: f64 = (t32a + t32b);let t32d: f64 = (l.fd36 * t32c);l.f167 = t32d;}
        let t32e: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };l.f6dc = t32e;
        if ((l.f976 != 0.0) && (l.f6dc != 0.0)) {let t32f: f64 = (p.p769 * l.fd35);let t330: f64 = (p.p768 + t32f);let t331: f64 = (p.p770 * l.fdc8);let t332: f64 = (t330 + t331);let t333: f64 = (p.p771 * l.fc70);let t334: f64 = (t332 + t333);l.f163 = t334;}
        let t335: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };l.f6de = t335;
        if ((l.f976 != 0.0) && (l.f6de != 0.0)) {let t336: f64 = (p.p765 * l.fd35);let t337: f64 = (p.p764 + t336);let t338: f64 = (p.p766 * l.fdc8);let t339: f64 = (t337 + t338);let t33a: f64 = (p.p767 * l.fc70);let t33b: f64 = (t339 + t33a);l.f15b = t33b;}
        if (l.f976 != 0.0) {l.f15f4 = 0.0;l.f15f6 = 0.0;l.fe2f = 0.0;l.fe0b = p.p788;}
        let t33c: f64 = if param_given[789] { 1.0 } else { 0.0 };let t33d: f64 = if t33c == 1.0 { 1.0 } else { 0.0 };l.f6e0 = t33d;
        if ((l.f976 != 0.0) && (l.f6e0 != 0.0)) {l.fe0b = p.p789;}
        let t33e: f64 = if (((l.f130b > 0.0) && (l.f130d > 0.0)) && ((l.fedd == 1.0) || ((l.fedd > 1.0) && (l.f1317 > 0.0)))) { 1.0 } else { 0.0 };l.f6e2 = t33e;let mut t350: usize = 0;
        while {
            let t34e: f64 = (l.fedd - 0.5);let t34f: f64 = if (((l.f976 != 0.0) && (l.f6e2 != 0.0)) && (l.fe2f < t34e)) { 1.0 } else { 0.0 };
            t34f != 0.0
        } {
            t350 += 1;assert!(t350 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t33f: f64 = (0.5 * l.fe10);let t340: f64 = (l.f130b + t33f);let t341: f64 = (l.f1317 + l.fe10);let t342: f64 = (l.fe2f * t341);let t343: f64 = (t340 + t342);let t344: f64 = (1.0 / t343);let t345: f64 = (l.f15f4 + t344);l.f15f4 = t345;let t346: f64 = (0.5 * l.fe10);let t347: f64 = (l.f130d + t346);let t348: f64 = (l.f1317 + l.fe10);let t349: f64 = (l.fe2f * t348);let t34a: f64 = (t347 + t349);let t34b: f64 = (1.0 / t34a);let t34c: f64 = (l.f15f6 + t34b);l.f15f6 = t34c;let t34d: f64 = (l.fe2f + 1.0);l.fe2f = t34d;}
        }
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t351: f64 = (l.f15f4 * l.fd8e);l.fd90 = t351;let t352: f64 = (l.f15f6 * l.fd8e);l.fd94 = t352;let t353: f64 = (0.5 * l.fe10);let t354: f64 = (p.p784 + t353);let t355: f64 = (1.0 / t354);l.fd92 = t355;let t356: f64 = (0.5 * l.fe10);let t357: f64 = (p.p785 + t356);let t358: f64 = (1.0 / t357);l.fd96 = t358;}
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {
            let t359: f64 = (l.fe10 + l.f231);
            let (t35b,) = {
    if (t359 > 1e-9) {
        let t35a: f64 = (l.fe10 + l.f231);
        (t35a,)
    } else {
        (1e-9,)
    }
};
            l.fe4a = t35b;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {
            let t35c: f64 = (l.f18bf + l.f2a3);let t35d: f64 = (t35c + p.p786);
            let (t360,) = {
    if (t35d > 1e-9) {
        let t35e: f64 = (l.f18bf + l.f2a3);let t35f: f64 = (t35e + p.p786);
        (t35f,)
    } else {
        (1e-9,)
    }
};
            l.f191e = t360;
        }
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t361: f64 = (l.fe4a).powf(p.p794);let t362: f64 = (1.0 / t361);l.f153a = t362;let t363: f64 = (l.f191e).powf(p.p795);let t364: f64 = (1.0 / t363);l.f153c = t364;let t365: f64 = (p.p791 * l.f153a);let t366: f64 = (1.0 + t365);let t367: f64 = (p.p792 * l.f153c);let t368: f64 = (t366 + t367);let t369: f64 = (p.p793 * l.f153a);let t36a: f64 = (t369 * l.f153c);let t36b: f64 = (t368 + t36a);let t36c: f64 = (l.f12d7 - 1.0);let t36d: f64 = (p.p790 * t36c);let t36e: f64 = (1.0 + t36d);let t36f: f64 = (t36b * t36e);l.fdf8 = t36f;let t370: f64 = (l.fd90 + l.fd94);let t371: f64 = (p.p787 * t370);let t372: f64 = (t371 / l.fdf8);l.f12ab = t372;let t373: f64 = (l.fd92 + l.fd96);let t374: f64 = (p.p787 * t373);let t375: f64 = (t374 / l.fdf8);l.f12ad = t375;let t376: f64 = (l.fe4a).powf(p.p800);let t377: f64 = (1.0 / t376);l.f153a = t377;let t378: f64 = (l.f191e).powf(p.p801);let t379: f64 = (1.0 / t378);l.f153c = t379;let t37a: f64 = (p.p797 * l.f153a);let t37b: f64 = (1.0 + t37a);let t37c: f64 = (p.p798 * l.f153c);let t37d: f64 = (t37b + t37c);let t37e: f64 = (p.p799 * l.f153a);let t37f: f64 = (t37e * l.f153c);let t380: f64 = (t37d + t37f);l.fdfa = t380;let t381: f64 = (l.fd90 + l.fd94);let t382: f64 = (t381 - l.fd92);let t383: f64 = (t382 - l.fd96);l.f151d = t383;let t384: f64 = (1.0 + l.f12ab);let t385: f64 = (1.0 + l.f12ad);let t386: f64 = (t384 / t385);l.f151e = t386;let t387: f64 = (l.f103 * l.f151e);l.f103 = t387;let t388: f64 = (l.f1589 * l.f151e);let t389: f64 = (p.p788 * l.f12ad);let t38a: f64 = (1.0 + t389);let t38b: f64 = (t388 * t38a);let t38c: f64 = (p.p788 * l.f12ab);let t38d: f64 = (1.0 + t38c);let t38e: f64 = (t38b / t38d);l.f1589 = t38e;let t38f: f64 = (l.f158f * l.f151e);let t390: f64 = (l.fe0b * l.f12ad);let t391: f64 = (1.0 + t390);let t392: f64 = (t38f * t391);let t393: f64 = (l.fe0b * l.f12ab);let t394: f64 = (1.0 + t393);let t395: f64 = (t392 / t394);l.f158f = t395;let t396: f64 = (l.f109 * l.f151e);l.f109 = t396;let t397: f64 = (p.p796 * l.f151d);let t398: f64 = (t397 / l.fdfa);l.f151e = t398;let t399: f64 = (l.f17ab + l.f151e);l.f17ab = t399;let t39a: f64 = (l.f17b1 + l.f151e);l.f17b1 = t39a;let t39b: f64 = (p.p802 * l.f151d);let t39c: f64 = (l.fdfa).powf(p.p803);let t39d: f64 = (t39b / t39c);l.f151e = t39d;let t39e: f64 = (l.f153 + l.f151e);l.f153 = t39e;let t39f: f64 = (l.f167 + l.f151e);l.f167 = t39f;}
        let t3a0: f64 = if ((((l.f1311 > 0.0) || (l.f1313 > 0.0)) || (l.f1315 > 0.0)) || (l.f130f > 0.0)) { 1.0 } else { 0.0 };l.f6e4 = t3a0;let t3a1: f64 = if (((l.f1311 == 0.0) && (l.f1313 == 0.0)) && (l.f1315 == 0.0)) { 1.0 } else { 0.0 };l.f6ee = t3a1;
        if (((l.f976 != 0.0) && (l.f6e4 != 0.0)) && (l.f6ee != 0.0)) {let t3a2: f64 = (l.f130f + l.f18bf);l.f151d = t3a2;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f976 != 0.0) && (l.f6e4 != 0.0)) && (l.f6ee != 0.0)) {let t3a3: f64 = (1.0 / p.p804);l.f151e = t3a3;let t3a4: f64 = (p.p804 * p.p804);let t3a5: f64 = (l.f130f * l.f151d);let t3a6: f64 = (t3a4 / t3a5);l.f1311 = t3a6;let t3a7: f64 = (0.1 * l.f130f);let t3a8: f64 = (0.01 * p.p804);let t3a9: f64 = (t3a7 + t3a8);let t3aa: f64 = (-10.0);let t3ab: f64 = (t3aa * l.f130f);let t3ac: f64 = (t3ab * l.f151e);let t3ad: f64 = (t3ac).exp();let t3ae: f64 = (t3a9 * t3ad);let t3af: f64 = (0.1 * l.f151d);let t3b0: f64 = (0.01 * p.p804);let t3b1: f64 = (t3af + t3b0);let t3b2: f64 = (-10.0);let t3b3: f64 = (t3b2 * l.f151d);let t3b4: f64 = (t3b3 * l.f151e);let t3b5: f64 = (t3b4).exp();let t3b6: f64 = (t3b1 * t3b5);let t3b7: f64 = (t3ae - t3b6);let t3b8: f64 = (t3b7 / l.f18bf);l.f1313 = t3b8;let t3b9: f64 = (0.05 * l.f130f);let t3ba: f64 = (0.0025 * p.p804);let t3bb: f64 = (t3b9 + t3ba);let t3bc: f64 = (-20.0);let t3bd: f64 = (t3bc * l.f130f);let t3be: f64 = (t3bd * l.f151e);let t3bf: f64 = (t3be).exp();let t3c0: f64 = (t3bb * t3bf);let t3c1: f64 = (0.05 * l.f151d);let t3c2: f64 = (0.0025 * p.p804);let t3c3: f64 = (t3c1 + t3c2);let t3c4: f64 = (-20.0);let t3c5: f64 = (t3c4 * l.f151d);let t3c6: f64 = (t3c5 * l.f151e);let t3c7: f64 = (t3c6).exp();let t3c8: f64 = (t3c3 * t3c7);let t3c9: f64 = (t3c0 - t3c8);let t3ca: f64 = (t3c9 / l.f18bf);l.f1315 = t3ca;}
        if ((l.f976 != 0.0) && (l.f6e4 != 0.0)) {let t3cb: f64 = (p.p805 * l.f1313);let t3cc: f64 = (l.f1311 + t3cb);let t3cd: f64 = (p.p806 * l.f1315);let t3ce: f64 = (t3cc + t3cd);l.f151d = t3ce;let t3cf: f64 = (l.fe0d * l.f151d);let t3d0: f64 = (l.f17ab + t3cf);l.f17ab = t3d0;let t3d1: f64 = (l.fe09 * l.f151d);let t3d2: f64 = (1.0 + t3d1);let t3d3: f64 = (l.f103 * t3d2);l.f103 = t3d3;let t3d4: f64 = (l.fe0d * l.f151d);let t3d5: f64 = (l.f17b1 + t3d4);l.f17b1 = t3d5;let t3d6: f64 = (l.fe09 * l.f151d);let t3d7: f64 = (1.0 + t3d6);let t3d8: f64 = (l.f109 * t3d7);l.f109 = t3d8;}
        l.f17a9 = l.f17ab;l.f1500 = l.f1502;l.f14c2 = l.f14c4;l.f1600 = l.f1602;l.f3b3 = l.f3b5;
        let (t3da,) = {
    if (l.fed5 > 1e20) {
        let (t3d9,) = {
            if (l.fed5 < 1e26) {
                (l.fed5,)
            } else {
                (1e26,)
            }
        };
        (t3d9,)
    } else {
        (1e20,)
    }
};
        l.fed3 = t3da;
        let (t3db,) = {
    if (l.f567 > 0.01) {
        (l.f567,)
    } else {
        (0.01,)
    }
};
        l.f565 = t3db;
        let (t3dc,) = {
    if (l.f1868 > 0.0) {
        (l.f1868,)
    } else {
        (0.0,)
    }
};
        l.f1866 = t3dc;l.f367 = l.f369;l.f2da = l.f2dc;
        let (t3dd,) = {
    if (l.feeb > 0.0) {
        (l.feeb,)
    } else {
        (0.0,)
    }
};
        l.fee9 = t3dd;l.f1606 = l.f1608;l.f160a = l.f160c;
        let (t3df,) = {
    if (l.fee2 > 1e23) {
        let (t3de,) = {
            if (l.fee2 < 1e27) {
                (l.fee2,)
            } else {
                (1e27,)
            }
        };
        (t3de,)
    } else {
        (1e23,)
    }
};
        l.fee0 = t3df;
        let (t3e1,) = {
    if (l.fee6 > 1e23) {
        let (t3e0,) = {
            if (l.fee6 < 1e27) {
                (l.fee6,)
            } else {
                (1e27,)
            }
        };
        (t3e0,)
    } else {
        (1e23,)
    }
};
        l.fee4 = t3e1;
        let (t3e2,) = {
    if (l.f1e3 > 0.0) {
        (l.f1e3,)
    } else {
        (0.0,)
    }
};
        l.f1e1 = t3e2;
        let (t3e4,) = {
    if (l.f1ef > 0.0) {
        let (t3e3,) = {
            if (l.f1ef < 0.5) {
                (l.f1ef,)
            } else {
                (0.5,)
            }
        };
        (t3e3,)
    } else {
        (0.0,)
    }
};
        l.f1ed = t3e4;
        let (t3e6,) = {
    if (l.f1f7 > 0.0) {
        let (t3e5,) = {
            if (l.f1f7 < 1.0) {
                (l.f1f7,)
            } else {
                (1.0,)
            }
        };
        (t3e5,)
    } else {
        (0.0,)
    }
};
        l.f1f5 = t3e6;l.f14de = l.f14e0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        l: &mut StampLocals,
    ) {
        let (t3e7,) = {
    if (l.f153 > 0.0) {
        (l.f153,)
    } else {
        (0.0,)
    }
};
        l.f151 = t3e7;
        let (t3e9,) = {
    if (l.f157 > 0.0) {
        let (t3e8,) = {
            if (l.f157 < 1.0) {
                (l.f157,)
            } else {
                (1.0,)
            }
        };
        (t3e8,)
    } else {
        (0.0,)
    }
};
        l.f155 = t3e9;
        let (t3ea,) = {
    if (l.f15f > 0.0) {
        (l.f15f,)
    } else {
        (0.0,)
    }
};
        l.f15d = t3ea;
        let (t3eb,) = {
    if (l.f1010 > 0.0) {
        (l.f1010,)
    } else {
        (0.0,)
    }
};
        l.f100e = t3eb;
        let (t3ed,) = {
    if (l.f1014 > 0.0) {
        let (t3ec,) = {
            if (l.f1014 < 1.0) {
                (l.f1014,)
            } else {
                (1.0,)
            }
        };
        (t3ec,)
    } else {
        (0.0,)
    }
};
        l.f1012 = t3ed;
        let (t3ee,) = {
    if (l.f101c > 0.0) {
        (l.f101c,)
    } else {
        (0.0,)
    }
};
        l.f101a = t3ee;
        let (t3ef,) = {
    if (l.f103 > 0.0) {
        (l.f103,)
    } else {
        (0.0,)
    }
};
        l.f101 = t3ef;l.f14ca = l.f14cc;
        let (t3f0,) = {
    if (l.febb > 0.0) {
        (l.febb,)
    } else {
        (0.0,)
    }
};
        l.feb9 = t3f0;l.f14ec = l.f14ee;
        let (t3f1,) = {
    if (l.f1564 > 0.0) {
        (l.f1564,)
    } else {
        (0.0,)
    }
};
        l.f1562 = t3f1;l.f14f8 = l.f14fa;
        let (t3f2,) = {
    if (l.f1cb > 0.0) {
        (l.f1cb,)
    } else {
        (0.0,)
    }
};
        l.f1c9 = t3f2;l.f14da = l.f14dc;
        let (t3f3,) = {
    if (l.f155e > 0.0) {
        (l.f155e,)
    } else {
        (0.0,)
    }
};
        l.f155c = t3f3;l.f14f4 = l.f14f6;
        let (t3f4,) = {
    if (l.f19bc > 0.0) {
        (l.f19bc,)
    } else {
        (0.0,)
    }
};
        l.f19ba = t3f4;l.f1508 = l.f150a;l.f474 = l.f476;
        let (t3f5,) = {
    if (l.f12c7 > 0.0) {
        (l.f12c7,)
    } else {
        (0.0,)
    }
};
        l.f12c5 = t3f5;l.f14f0 = l.f14f2;let t3f6: f64 = (-0.5);
        let (t3f9,) = {
    if (l.f12cd > t3f6) {
        let (t3f7,) = {
            if (l.f12cd < 1.0) {
                (l.f12cd,)
            } else {
                (1.0,)
            }
        };
        (t3f7,)
    } else {
        let t3f8: f64 = (-0.5);
        (t3f8,)
    }
};
        l.f12cb = t3f9;let t3fa: f64 = (-0.5);
        let (t3fc,) = {
    if (l.f12d3 > t3fa) {
        (l.f12d3,)
    } else {
        let t3fb: f64 = (-0.5);
        (t3fb,)
    }
};
        l.f12d1 = t3fc;
        let (t3fd,) = {
    if (l.f1589 > 0.0) {
        (l.f1589,)
    } else {
        (0.0,)
    }
};
        l.f1587 = t3fd;l.f14fc = l.f14fe;let t3fe: f64 = (-0.5);
        let (t401,) = {
    if (l.f159f > t3fe) {
        let (t3ff,) = {
            if (l.f159f < 1.0) {
                (l.f159f,)
            } else {
                (1.0,)
            }
        };
        (t3ff,)
    } else {
        let t400: f64 = (-0.5);
        (t400,)
    }
};
        l.f159d = t401;let t0: f64 = (-0.5);
        let (t2,) = {
    if (l.f15bb > t0) {
        (l.f15bb,)
    } else {
        let t1: f64 = (-0.5);
        (t1,)
    }
};
        l.f15b9 = t2;
        let (t3,) = {
    if (l.f15c3 > 0.01) {
        (l.f15c3,)
    } else {
        (0.01,)
    }
};
        l.f15c1 = t3;
        let (t4,) = {
    if (l.fe4 > 2.0) {
        (l.fe4,)
    } else {
        (2.0,)
    }
};
        l.fe2 = t4;
        let (t5,) = {
    if (l.f41 > 0.0) {
        (l.f41,)
    } else {
        (0.0,)
    }
};
        l.f3f = t5;
        let (t6,) = {
    if (l.f35 > 0.0) {
        (l.f35,)
    } else {
        (0.0,)
    }
};
        l.f33 = t6;
        let (t7,) = {
    if (l.f3d > 0.0) {
        (l.f3d,)
    } else {
        (0.0,)
    }
};
        l.f3b = t7;l.f1862 = l.f1864;
        let (t8,) = {
    if (l.f2 > 0.0) {
        (l.f2,)
    } else {
        (0.0,)
    }
};
        l.f0 = t8;l.f4 = l.f6;l.f14c6 = l.f14c8;
        let (t9,) = {
    if (l.fc > 0.0) {
        (l.fc,)
    } else {
        (0.0,)
    }
};
        l.fa = t9;
        let (ta,) = {
    if (l.f10 > 0.0) {
        (l.f10,)
    } else {
        (0.0,)
    }
};
        l.fe = ta;
        let (tb,) = {
    if (l.fd3b > 1e-12) {
        (l.fd3b,)
    } else {
        (1e-12,)
    }
};
        l.fd39 = tb;l.f51f = l.f521;
        let (tc,) = {
    if (l.fcb8 > 0.0) {
        (l.fcb8,)
    } else {
        (0.0,)
    }
};
        l.fcb6 = tc;
        let (td,) = {
    if (l.fcbc > 0.0) {
        (l.fcbc,)
    } else {
        (0.0,)
    }
};
        l.fcba = td;
        let (te,) = {
    if (l.fcc0 > 0.0) {
        (l.fcc0,)
    } else {
        (0.0,)
    }
};
        l.fcbe = te;l.f14e8 = l.f14ea;l.f507 = l.f509;l.f513 = l.f515;l.f50b = l.f50d;l.f517 = l.f519;l.f50f = l.f511;l.f51b = l.f51d;l.f18f = l.f191;
        let (tf,) = {
    if (l.f29 > 0.0) {
        (l.f29,)
    } else {
        (0.0,)
    }
};
        l.f27 = tf;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t10,) = {
    if (l.f2d > 0.0) {
        (l.f2d,)
    } else {
        (0.0,)
    }
};
        l.f2b = t10;l.f112 = l.f114;l.f118 = l.f11a;l.f14d2 = l.f14d4;l.f14d6 = l.f14d8;l.f17b = l.f17d;l.f17f = l.f181;
        let (t11,) = {
    if (l.f1bb > 0.0) {
        (l.f1bb,)
    } else {
        (0.0,)
    }
};
        l.f1b7 = t11;l.f29b = l.f29d;
        let (t12,) = {
    if (l.f422 > 0.0) {
        (l.f422,)
    } else {
        (0.0,)
    }
};
        l.f420 = t12;
        let (t13,) = {
    if (l.f158f > 0.0) {
        (l.f158f,)
    } else {
        (0.0,)
    }
};
        l.f158d = t13;
        let (t14,) = {
    if (l.fe8 > 2.0) {
        (l.fe8,)
    } else {
        (2.0,)
    }
};
        l.fe6 = t14;l.f43 = l.f45;
        let (t15,) = {
    if (l.f39 > 0.0) {
        (l.f39,)
    } else {
        (0.0,)
    }
};
        l.f37 = t15;
        let (t16,) = {
    if (l.f185 > 0.0) {
        (l.f185,)
    } else {
        (0.0,)
    }
};
        l.f183 = t16;
        let (t17,) = {
    if (l.f18d > 0.0) {
        (l.f18d,)
    } else {
        (0.0,)
    }
};
        l.f18b = t17;l.f462 = l.f464;l.f466 = l.f468;l.f187 = l.f189;
        let (t18,) = {
    if (l.f173 > 0.0) {
        (l.f173,)
    } else {
        (0.0,)
    }
};
        l.f171 = t18;
        let (t19,) = {
    if (l.f197 > 0.0) {
        (l.f197,)
    } else {
        (0.0,)
    }
};
        l.f195 = t19;
        let (t1a,) = {
    if (l.f19b > 0.0) {
        (l.f19b,)
    } else {
        (0.0,)
    }
};
        l.f199 = t1a;l.f351 = l.f353;l.f46e = l.f470;l.f46a = l.f46c;l.fee = l.ff0;
        let (t1b,) = {
    if (l.f16b > 0.0) {
        (l.f16b,)
    } else {
        (0.0,)
    }
};
        l.f169 = t1b;
        let (t1c,) = {
    if (l.f16f > 0.0) {
        (l.f16f,)
    } else {
        (0.0,)
    }
};
        l.f16d = t1c;l.f4a5 = l.f4a7;
        let (t1d,) = {
    if (l.f4aa > 0.0) {
        (l.f4aa,)
    } else {
        (0.0,)
    }
};
        l.f4a9 = t1d;l.f17af = l.f17b1;l.f1504 = l.f1506;l.f2de = l.f2e0;
        let (t1f,) = {
    if (l.fedb > 1e20) {
        let (t1e,) = {
            if (l.fedb < 1e26) {
                (l.fedb,)
            } else {
                (1e26,)
            }
        };
        (t1e,)
    } else {
        (1e20,)
    }
};
        l.fed9 = t1f;
        let (t20,) = {
    if (l.f1f3 > 0.0) {
        (l.f1f3,)
    } else {
        (0.0,)
    }
};
        l.f1f1 = t20;
        let (t21,) = {
    if (l.f109 > 0.0) {
        (l.f109,)
    } else {
        (0.0,)
    }
};
        l.f107 = t21;l.f14ce = l.f14d0;
        let (t22,) = {
    if (l.f1024 > 0.0) {
        (l.f1024,)
    } else {
        (0.0,)
    }
};
        l.f1022 = t22;
        let (t24,) = {
    if (l.f1018 > 0.0) {
        let (t23,) = {
            if (l.f1018 < 1.0) {
                (l.f1018,)
            } else {
                (1.0,)
            }
        };
        (t23,)
    } else {
        (0.0,)
    }
};
        l.f1016 = t24;
        let (t25,) = {
    if (l.f1020 > 0.0) {
        (l.f1020,)
    } else {
        (0.0,)
    }
};
        l.f101e = t25;
        let (t26,) = {
    if (l.f167 > 0.0) {
        (l.f167,)
    } else {
        (0.0,)
    }
};
        l.f165 = t26;
        let (t28,) = {
    if (l.f15b > 0.0) {
        let (t27,) = {
            if (l.f15b < 1.0) {
                (l.f15b,)
            } else {
                (1.0,)
            }
        };
        (t27,)
    } else {
        (0.0,)
    }
};
        l.f159 = t28;
        let (t29,) = {
    if (l.f163 > 0.0) {
        (l.f163,)
    } else {
        (0.0,)
    }
};
        l.f161 = t29;
        let (t2a,) = {
    if (l.f1298 > 0.0) {
        (l.f1298,)
    } else {
        (0.0,)
    }
};
        l.f1297 = t2a;l.f12cf = l.f12d0;l.f1295 = l.f1296;l.f1293 = l.f1294;l.f12c3 = l.f12c4;l.f12c1 = l.f12c2;l.f12db = l.f12dc;let t2b: f64 = (p.p31 * l.fedd);
        let (t2d,) = {
    if (t2b > 0.0) {
        let t2c: f64 = (p.p31 * l.fedd);
        (t2c,)
    } else {
        (0.0,)
    }
};
        l.febf = t2d;l.f436 = p.p16;l.f29f = p.p15;l.f438 = p.p18;l.f2a1 = p.p17;let t2e: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.f704 = t2e;
        if (l.f704 != 0.0) {l.f160a = l.f1606;l.fee4 = l.fee0;l.f2b = l.f27;l.f118 = l.f112;l.f14d6 = l.f14d2;l.f17f = l.f17b;l.fcbe = l.fcba;l.f50f = l.f50b;l.f51b = l.f517;}
    }
}
