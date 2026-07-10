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
        if (l.fa1e != 0.0) {l.f5a0 = l.f59c;}
        let t27: f64 = if param_given[345] { 1.0 } else { 0.0 };let t28: f64 = if t27 == 1.0 { 1.0 } else { 0.0 };l.fa68 = t28;
        if ((l.fa1e != 0.0) && (l.fa68 != 0.0)) {l.f5a0 = p.p345;}
        if (l.fa1e != 0.0) {l.f1b3 = p.p346;let t29: f64 = (p.p347 * p.p237);let t2a: f64 = (1e-6 * l.fe91);let t2b: f64 = (t29 / t2a);l.f2b = t2b;let t2c: f64 = (p.p348 * p.p238);let t2d: f64 = (1e-6 * l.fe91);let t2e: f64 = (t2c / t2d);l.f2f = t2e;l.f131 = p.p349;l.f137 = p.p350;l.f1690 = p.p351;l.f1694 = p.p352;l.f19f = p.p353;l.f1a3 = p.p354;let t2f: f64 = (8.8541878176e-12 * p.p210);let t30: f64 = (t2f * l.f1b01);let t31: f64 = (t30 * l.fef3);let t32: f64 = (t31 / p.p209);l.f1dd = t32;let t33: f64 = (8.8541878176e-12 * p.p210);let t34: f64 = (t33 * l.f1b01);let t35: f64 = (t34 * p.p237);let t36: f64 = (t35 / p.p235);l.f1a7 = t36;let t37: f64 = (8.8541878176e-12 * p.p210);let t38: f64 = (t37 * l.f1b01);let t39: f64 = (t38 * p.p238);let t3a: f64 = (t39 / p.p236);l.f1af = t3a;let t3b: f64 = (l.fdf1).powf(p.p357);let t3c: f64 = (p.p356 * t3b);let t3d: f64 = (p.p355 + t3c);let t3e: f64 = (p.p358 * l.fe91);let t3f: f64 = (t3d + t3e);let t40: f64 = (p.p359 * l.fd25);let t41: f64 = (t3f + t40);l.f2e2 = t41;let t42: f64 = (p.p361 * l.fdf1);let t43: f64 = (p.p360 + t42);let t44: f64 = (p.p362 * l.fe91);let t45: f64 = (t43 + t44);let t46: f64 = (p.p363 * l.fd25);let t47: f64 = (t45 + t46);l.f494 = t47;l.f176f = p.p297;}
        let t48: f64 = if param_given[364] { 1.0 } else { 0.0 };let t49: f64 = if t48 == 1.0 { 1.0 } else { 0.0 };l.fa74 = t49;
        if ((l.fa1e != 0.0) && (l.fa74 != 0.0)) {l.f176f = p.p364;}
        if (l.fa1e != 0.0) {l.f1769 = p.p298;}
        let t4a: f64 = if param_given[365] { 1.0 } else { 0.0 };let t4b: f64 = if t4a == 1.0 { 1.0 } else { 0.0 };l.fa80 = t4b;
        if ((l.fa1e != 0.0) && (l.fa80 != 0.0)) {l.f1769 = p.p365;}
        if (l.fa1e != 0.0) {l.f176b = p.p299;}
        let t4c: f64 = if param_given[366] { 1.0 } else { 0.0 };let t4d: f64 = if t4c == 1.0 { 1.0 } else { 0.0 };l.fa8c = t4d;
        if ((l.fa1e != 0.0) && (l.fa8c != 0.0)) {l.f176b = p.p366;}
        if (l.fa1e != 0.0) {l.f1771 = p.p300;}
        let t4e: f64 = if param_given[367] { 1.0 } else { 0.0 };let t4f: f64 = if t4e == 1.0 { 1.0 } else { 0.0 };l.fa98 = t4f;
        if ((l.fa1e != 0.0) && (l.fa98 != 0.0)) {l.f1771 = p.p367;}
        if (l.fa1e != 0.0) {l.f176d = p.p301;}
        let t50: f64 = if param_given[368] { 1.0 } else { 0.0 };let t51: f64 = if t50 == 1.0 { 1.0 } else { 0.0 };l.faa4 = t51;
        if ((l.fa1e != 0.0) && (l.faa4 != 0.0)) {l.f176d = p.p368;}
        if (l.fa1e != 0.0) {let t52: f64 = (l.f1769 * l.fc94);let t53: f64 = (t52 / l.f663);let t54: f64 = (l.fdf1).powf(l.f176b);let t55: f64 = (t53 * t54);let t56: f64 = (l.f176f + t55);let t57: f64 = (l.f1771 * l.fe91);let t58: f64 = (1.0 + t57);let t59: f64 = (t56 * t58);let t5a: f64 = (l.f176d * l.fd25);let t5b: f64 = (1.0 + t5a);let t5c: f64 = (t59 * t5b);l.f1764 = t5c;l.f104 = p.p309;}
        let t5d: f64 = if param_given[369] { 1.0 } else { 0.0 };let t5e: f64 = if t5d == 1.0 { 1.0 } else { 0.0 };l.fab0 = t5e;
        if ((l.fa1e != 0.0) && (l.fab0 != 0.0)) {l.f104 = p.p369;}
        if (l.fa1e != 0.0) {l.f102 = p.p310;}
        let t5f: f64 = if param_given[370] { 1.0 } else { 0.0 };let t60: f64 = if t5f == 1.0 { 1.0 } else { 0.0 };l.fabc = t60;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.fabc != 0.0)) {l.f102 = p.p370;}
        if (l.fa1e != 0.0) {let t61: f64 = (l.f102 * l.fdf1);let t62: f64 = (1.0 + t61);let t63: f64 = (l.f104 / t62);l.f100 = t63;let t64: f64 = (l.fdf1).powf(p.p372);let t65: f64 = (p.p371 * t64);let t66: f64 = (p.p373 * l.fe91);let t67: f64 = (1.0 + t66);let t68: f64 = (t65 * t67);l.f47 = t68;let t69: f64 = (l.fdf1).powf(p.p375);l.f17dd = t69;let t6a: f64 = (p.p374 * l.f17dd);let t6b: f64 = (p.p377 * l.fe91);let t6c: f64 = (1.0 + t6b);let t6d: f64 = (t6a * t6c);let t6e: f64 = (p.p376 * l.fdf1);let t6f: f64 = (t6e * l.f17dd);let t70: f64 = (1.0 + t6f);let t71: f64 = (t6d / t70);l.f3b = t71;l.f4d9 = p.p378;l.f4dd = p.p379;l.f1ab = p.p380;let t72: f64 = (p.p381 * l.fd84);l.f194 = t72;let t73: f64 = (p.p382 * l.fd91);l.f1b9 = t73;let t74: f64 = (p.p383 * l.fd91);l.f1bd = t74;l.f3b0 = p.p384;l.f4e5 = p.p385;l.f4e1 = p.p386;l.f108 = p.p387;let t75: f64 = (p.p388 * l.fd8d);l.f18c = t75;let t76: f64 = (p.p389 * l.fd8d);l.f190 = t76;let t77: f64 = (2.0 * p.p396);let t78: f64 = (t77 / l.fef1);let t79: f64 = (1.0 - t78);l.f16dd = t79;l.f521 = p.p390;let t7a: f64 = (p.p391 * l.f11d);let t7b: f64 = (t7a * l.f11d);let t7c: f64 = (t7b * l.fe91);let t7d: f64 = (t7c * l.fe91);l.f524 = t7d;let t7e: f64 = (2.0 * p.p398);let t7f: f64 = (p.p399 * l.f1afd);let t80: f64 = (t7e + t7f);l.f1afe = t80;l.f19ab = p.p400;let t81: f64 = (p.p402 * l.fdf1);let t82: f64 = (p.p401 + t81);let t83: f64 = (p.p403 * l.fe91);let t84: f64 = (t82 + t83);let t85: f64 = (p.p404 * l.fd25);let t86: f64 = (t84 + t85);l.f16c4 = t86;let t87: f64 = (l.fdf1).powf(p.p407);let t88: f64 = (p.p406 * t87);let t89: f64 = (p.p405 + t88);let t8a: f64 = (p.p408 * l.fe91);let t8b: f64 = (t89 + t8a);let t8c: f64 = (p.p409 * l.fd25);let t8d: f64 = (t8b + t8c);l.f32c = t8d;let t8e: f64 = (l.fdf1).powf(p.p412);let t8f: f64 = (p.p411 * t8e);let t90: f64 = (1.0 + t8f);let t91: f64 = (p.p410 * t90);let t92: f64 = (p.p413 * l.fe91);let t93: f64 = (1.0 + t92);let t94: f64 = (t91 * t93);let t95: f64 = (p.p414 * l.fd25);let t96: f64 = (1.0 + t95);let t97: f64 = (t94 * t96);l.ffbd = t97;let t98: f64 = (l.fdf1).powf(p.p417);let t99: f64 = (p.p416 * t98);let t9a: f64 = (p.p415 + t99);l.f21a = t9a;}
        if (l.fa1e != 0.0) {let t9b: f64 = (p.p418 * p.p419);let t9c: f64 = (t9b / l.fef1);let t9d: f64 = (-l.fef1);let t9e: f64 = (t9d / p.p419);let t9f: f64 = (t9e).exp();let ta0: f64 = (1.0 - t9f);let ta1: f64 = (t9c * ta0);let ta2: f64 = (1.0 + ta1);l.f664 = ta2;}
        if (l.fa1e != 0.0) {
            let (ta3,) = {
    if (l.f664 > 1e-15) {
        (l.f664,)
    } else {
        (1e-15,)
    }
};
            l.f664 = ta3;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.fa1e != 0.0) {let ta4: f64 = (p.p259 * l.f1afe);let ta5: f64 = (l.f664 * l.fef1);let ta6: f64 = (ta4 / ta5);let ta7: f64 = (p.p420 * l.fe91);let ta8: f64 = (1.0 + ta7);let ta9: f64 = (ta6 * ta8);l.f124 = ta9;let taa: f64 = (p.p422 * l.fdf1);let tab: f64 = (p.p421 + taa);let tac: f64 = (p.p423 * l.fe91);let tad: f64 = (tab + tac);let tae: f64 = (p.p424 * l.fd25);let taf: f64 = (tad + tae);l.f168c = taf;let tb0: f64 = (l.fdf1).powf(p.p426);let tb1: f64 = (p.p425 * tb0);let tb2: f64 = (p.p427 * l.fe91);let tb3: f64 = (1.0 + tb2);let tb4: f64 = (tb1 * tb3);l.f113b = tb4;l.f112f = p.p428;l.f1137 = p.p429;let tb5: f64 = (l.fdf1).powf(p.p431);let tb6: f64 = (p.p430 * tb5);let tb7: f64 = (p.p432 * l.fe91);let tb8: f64 = (1.0 + tb7);let tb9: f64 = (tb6 * tb8);l.f188 = tb9;l.f184 = p.p434;l.f17c = p.p433;let tba: f64 = (p.p832 * l.fdf1);let tbb: f64 = (p.p831 + tba);let tbc: f64 = (p.p833 * l.fe91);let tbd: f64 = (tbb + tbc);let tbe: f64 = (p.p834 * l.fd25);let tbf: f64 = (tbd + tbe);l.fedd = tbf;let tc0: f64 = (p.p836 * l.fdf1);let tc1: f64 = (p.p835 + tc0);let tc2: f64 = (p.p837 * l.fe91);let tc3: f64 = (tc1 + tc2);let tc4: f64 = (p.p838 * l.fd25);let tc5: f64 = (tc3 + tc4);l.fed9 = tc5;let tc6: f64 = (0.3333333333333333 * l.f1ad3);let tc7: f64 = (tc6 / l.ffc1);let tc8: f64 = (tc7 + l.f1cb0);let tc9: f64 = (p.p443 * tc8);let tca: f64 = (l.ffc1 * l.fee2);let tcb: f64 = (tc9 / tca);let tcc: f64 = (p.p441 + p.p442);let tcd: f64 = (l.f1ad3 * l.fedf);let tce: f64 = (tcc / tcd);let tcf: f64 = (tcb + tce);let td0: f64 = (l.ffbf * p.p440);let td1: f64 = (tcf + td0);l.f1402 = td1;}
        if (l.fa1e != 0.0) {
            let (td2,) = {
    if (p.p445 > 0.0) {
        (p.p445,)
    } else {
        (0.0,)
    }
};
            l.f1446 = td2;
        }
        if (l.fa1e != 0.0) {
            let (td3,) = {
    if (p.p446 > 0.0) {
        (p.p446,)
    } else {
        (0.0,)
    }
};
            l.f1447 = td3;
        }
        let td4: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.faca = td4;
        if ((l.fa1e != 0.0) && (l.faca != 0.0)) {l.f1447 = l.f1446;}
        if (l.fa1e != 0.0) {let td5: f64 = (l.ffbf * p.p12);let td6: f64 = (td5 * l.f1446);l.f1441 = td6;let td7: f64 = (l.ffbf * p.p13);let td8: f64 = (td7 * l.f1447);l.f1400 = td8;let td9: f64 = (l.ffbf * p.p448);l.f1451 = td9;let tda: f64 = (l.ffbf * p.p447);l.f13fe = tda;let tdb: f64 = (l.ffbf * p.p449);l.f1434 = tdb;let tdc: f64 = (l.ffbf * p.p450);l.f1432 = tdc;let tdd: f64 = (p.p454 / l.fdf1);let tde: f64 = (1.0 + tdd);let tdf: f64 = (tde / l.fe91);let te0: f64 = (p.p453 + tdf);l.f2d1 = te0;}
        if (l.fa1e != 0.0) {
            let (te1,) = {
    if (l.f2d1 > 1e-6) {
        (l.f2d1,)
    } else {
        (1e-6,)
    }
};
            l.f2d1 = te1;
        }
        if (l.fa1e != 0.0) {let te2: f64 = (p.p452 / l.f2d1);let te3: f64 = (p.p451 + te2);l.f144b = te3;let te4: f64 = (p.p458 / l.fdf1);let te5: f64 = (1.0 + te4);let te6: f64 = (p.p457 + te5);let te7: f64 = (p.p456 * te6);let te8: f64 = (te7 / l.fe91);let te9: f64 = (p.p455 + te8);l.f225 = te9;l.f16b1 = p.p459;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let tea: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };l.fad5 = tea;
        if ((l.fa1e != 0.0) && (l.fad5 != 0.0)) {let teb: f64 = (p.p461 * l.fdf1);let tec: f64 = (p.p460 + teb);let ted: f64 = (p.p462 * l.fe91);let tee: f64 = (tec + ted);let tef: f64 = (p.p463 * l.fd25);let tf0: f64 = (tee + tef);l.f19a4 = tf0;}
        let tf1: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };l.fae1 = tf1;
        if ((l.fa1e != 0.0) && (l.fae1 != 0.0)) {let tf2: f64 = (p.p465 * l.fdf1);let tf3: f64 = (p.p464 + tf2);let tf4: f64 = (p.p466 * l.fe91);let tf5: f64 = (tf3 + tf4);let tf6: f64 = (p.p467 * l.fd25);let tf7: f64 = (tf5 + tf6);l.f16c0 = tf7;}
        let tf8: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };l.faf0 = tf8;
        if ((l.fa1e != 0.0) && (l.faf0 != 0.0)) {let tf9: f64 = (p.p469 * l.fdf1);let tfa: f64 = (p.p468 + tf9);let tfb: f64 = (p.p470 * l.fe91);let tfc: f64 = (tfa + tfb);let tfd: f64 = (p.p471 * l.fd25);let tfe: f64 = (tfc + tfd);l.ffb7 = tfe;}
        let tff: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };l.fafc = tff;
        if ((l.fa1e != 0.0) && (l.fafc != 0.0)) {let t100: f64 = (p.p473 * l.fdf1);let t101: f64 = (p.p472 + t100);let t102: f64 = (p.p474 * l.fe91);let t103: f64 = (t101 + t102);let t104: f64 = (p.p475 * l.fd25);let t105: f64 = (t103 + t104);l.f5f4 = t105;}
        let t106: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };l.fb08 = t106;
        if ((l.fa1e != 0.0) && (l.fb08 != 0.0)) {let t107: f64 = (p.p477 * l.fdf1);let t108: f64 = (p.p476 + t107);let t109: f64 = (p.p478 * l.fe91);let t10a: f64 = (t108 + t109);let t10b: f64 = (p.p479 * l.fd25);let t10c: f64 = (t10a + t10b);l.f1a72 = t10c;}
        let t10d: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };l.fb14 = t10d;
        if ((l.fa1e != 0.0) && (l.fb14 != 0.0)) {let t10e: f64 = (p.p481 * l.fdf1);let t10f: f64 = (p.p480 + t10e);let t110: f64 = (p.p482 * l.fe91);let t111: f64 = (t10f + t110);let t112: f64 = (p.p483 * l.fd25);let t113: f64 = (t111 + t112);l.f328 = t113;}
        let t114: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };l.fb20 = t114;
        if ((l.fa1e != 0.0) && (l.fb20 != 0.0)) {let t115: f64 = (p.p485 * l.fdf1);let t116: f64 = (p.p484 + t115);let t117: f64 = (p.p486 * l.fe91);let t118: f64 = (t116 + t117);let t119: f64 = (p.p487 * l.fd25);let t11a: f64 = (t118 + t119);l.ffcd = t11a;}
        let t11b: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };l.fb2c = t11b;
        if ((l.fa1e != 0.0) && (l.fb2c != 0.0)) {let t11c: f64 = (p.p489 * l.fdf1);let t11d: f64 = (p.p488 + t11c);let t11e: f64 = (p.p490 * l.fe91);let t11f: f64 = (t11d + t11e);let t120: f64 = (p.p491 * l.fd25);let t121: f64 = (t11f + t120);l.ffc4 = t121;}
        let t122: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };l.fb38 = t122;
        if ((l.fa1e != 0.0) && (l.fb38 != 0.0)) {let t123: f64 = (p.p493 * l.fdf1);let t124: f64 = (p.p492 + t123);let t125: f64 = (p.p494 * l.fe91);let t126: f64 = (t124 + t125);let t127: f64 = (p.p495 * l.fd25);let t128: f64 = (t126 + t127);l.ffc8 = t128;}
        let t129: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };l.fb46 = t129;
        if ((l.fa1e != 0.0) && (l.fb46 != 0.0)) {let t12a: f64 = (p.p497 * l.fdf1);let t12b: f64 = (p.p496 + t12a);let t12c: f64 = (p.p498 * l.fe91);let t12d: f64 = (t12b + t12c);let t12e: f64 = (p.p499 * l.fd25);let t12f: f64 = (t12d + t12e);l.f209 = t12f;}
        let t130: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };l.fb52 = t130;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.fb52 != 0.0)) {let t131: f64 = (p.p505 * l.fdf1);let t132: f64 = (p.p504 + t131);let t133: f64 = (p.p506 * l.fe91);let t134: f64 = (t132 + t133);let t135: f64 = (p.p507 * l.fd25);let t136: f64 = (t134 + t135);l.f21e = t136;}
        let t137: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };l.fb5e = t137;
        if ((l.fa1e != 0.0) && (l.fb5e != 0.0)) {let t138: f64 = (p.p501 * l.fdf1);let t139: f64 = (p.p500 + t138);let t13a: f64 = (p.p502 * l.fe91);let t13b: f64 = (t139 + t13a);let t13c: f64 = (p.p503 * l.fd25);let t13d: f64 = (t13b + t13c);l.f216 = t13d;}
        let t13e: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };l.fb6a = t13e;
        if ((l.fa1e != 0.0) && (l.fb6a != 0.0)) {let t13f: f64 = (p.p509 * l.fdf1);let t140: f64 = (p.p508 + t13f);let t141: f64 = (p.p510 * l.fe91);let t142: f64 = (t140 + t141);let t143: f64 = (p.p511 * l.fd25);let t144: f64 = (t142 + t143);l.f169c = t144;}
        let t145: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };l.fb76 = t145;
        if ((l.fa1e != 0.0) && (l.fb76 != 0.0)) {let t146: f64 = (p.p513 * l.fdf1);let t147: f64 = (p.p512 + t146);let t148: f64 = (p.p514 * l.fe91);let t149: f64 = (t147 + t148);let t14a: f64 = (p.p515 * l.fd25);let t14b: f64 = (t149 + t14a);let t14c: f64 = (l.fdf2 * t14b);l.f174 = t14c;}
        let t14d: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };l.fb82 = t14d;
        if ((l.fa1e != 0.0) && (l.fb82 != 0.0)) {let t14e: f64 = (p.p521 * l.fdf1);let t14f: f64 = (p.p520 + t14e);let t150: f64 = (p.p522 * l.fe91);let t151: f64 = (t14f + t150);let t152: f64 = (p.p523 * l.fd25);let t153: f64 = (t151 + t152);l.f180 = t153;}
        let t154: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };l.fb8e = t154;
        if ((l.fa1e != 0.0) && (l.fb8e != 0.0)) {let t155: f64 = (p.p517 * l.fdf1);let t156: f64 = (p.p516 + t155);let t157: f64 = (p.p518 * l.fe91);let t158: f64 = (t156 + t157);let t159: f64 = (p.p519 * l.fd25);let t15a: f64 = (t158 + t159);l.f178 = t15a;}
        let t15b: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };l.fb9a = t15b;
        if ((l.fa1e != 0.0) && (l.fb9a != 0.0)) {let t15c: f64 = (p.p525 * l.fdf1);let t15d: f64 = (p.p524 + t15c);let t15e: f64 = (p.p526 * l.fe91);let t15f: f64 = (t15d + t15e);let t160: f64 = (p.p527 * l.fd25);let t161: f64 = (t15f + t160);let t162: f64 = (l.fdf2 * t161);l.f1127 = t162;}
        let t163: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };l.fba6 = t163;
        if ((l.fa1e != 0.0) && (l.fba6 != 0.0)) {let t164: f64 = (p.p533 * l.fdf1);let t165: f64 = (p.p532 + t164);let t166: f64 = (p.p534 * l.fe91);let t167: f64 = (t165 + t166);let t168: f64 = (p.p535 * l.fd25);let t169: f64 = (t167 + t168);l.f1133 = t169;}
        let t16a: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };l.fbb2 = t16a;
        if ((l.fa1e != 0.0) && (l.fbb2 != 0.0)) {let t16b: f64 = (p.p529 * l.fdf1);let t16c: f64 = (p.p528 + t16b);let t16d: f64 = (p.p530 * l.fe91);let t16e: f64 = (t16c + t16d);let t16f: f64 = (p.p531 * l.fd25);let t170: f64 = (t16e + t16f);l.f112b = t170;}
        let t171: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };l.fbc0 = t171;
        if ((l.fa1e != 0.0) && (l.fbc0 != 0.0)) {let t172: f64 = (l.f1afd / l.fef1);let t173: f64 = (p.p537 * l.fdf1);let t174: f64 = (p.p536 + t173);let t175: f64 = (p.p538 * l.fe91);let t176: f64 = (t174 + t175);let t177: f64 = (p.p539 * l.fd25);let t178: f64 = (t176 + t177);let t179: f64 = (t172 * t178);l.f11d = t179;}
        let t17a: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };l.fbcc = t17a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.fbcc != 0.0)) {let t17b: f64 = (p.p541 * l.fdf1);let t17c: f64 = (p.p540 + t17b);let t17d: f64 = (p.p542 * l.fe91);let t17e: f64 = (t17c + t17d);let t17f: f64 = (p.p543 * l.fd25);let t180: f64 = (t17e + t17f);l.f1688 = t180;}
        let t181: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };l.fbd8 = t181;
        if ((l.fa1e != 0.0) && (l.fbd8 != 0.0)) {let t182: f64 = (p.p545 * l.fdf1);let t183: f64 = (p.p544 + t182);let t184: f64 = (p.p546 * l.fe91);let t185: f64 = (t183 + t184);let t186: f64 = (p.p547 * l.fd25);let t187: f64 = (t185 + t186);l.ff99 = t187;}
        let t188: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };l.fbe4 = t188;
        if ((l.fa1e != 0.0) && (l.fbe4 != 0.0)) {let t189: f64 = (p.p549 * l.fdf1);let t18a: f64 = (p.p548 + t189);let t18b: f64 = (p.p550 * l.fe91);let t18c: f64 = (t18a + t18b);let t18d: f64 = (p.p551 * l.fd25);let t18e: f64 = (t18c + t18d);l.f1731 = t18e;}
        let t18f: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };l.fbf0 = t18f;
        if ((l.fa1e != 0.0) && (l.fbf0 != 0.0)) {let t190: f64 = (p.p553 * l.fdf1);let t191: f64 = (p.p552 + t190);let t192: f64 = (p.p554 * l.fe91);let t193: f64 = (t191 + t192);let t194: f64 = (p.p555 * l.fd25);let t195: f64 = (t193 + t194);l.f1ee = t195;}
        let t196: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };l.fbfc = t196;
        if ((l.fa1e != 0.0) && (l.fbfc != 0.0)) {let t197: f64 = (p.p557 * l.fdf1);let t198: f64 = (p.p556 + t197);let t199: f64 = (p.p558 * l.fe91);let t19a: f64 = (t198 + t199);let t19b: f64 = (p.p559 * l.fd25);let t19c: f64 = (t19a + t19b);l.f172a = t19c;}
        let t19d: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };l.fc08 = t19d;
        if ((l.fa1e != 0.0) && (l.fc08 != 0.0)) {let t19e: f64 = (p.p561 * l.fdf1);let t19f: f64 = (p.p560 + t19e);let t1a0: f64 = (p.p562 * l.fe91);let t1a1: f64 = (t19f + t1a0);let t1a2: f64 = (p.p563 * l.fd25);let t1a3: f64 = (t1a1 + t1a2);l.f1bee = t1a3;}
        let t1a4: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };l.fc14 = t1a4;
        if ((l.fa1e != 0.0) && (l.fc14 != 0.0)) {let t1a5: f64 = (p.p565 * l.fdf1);let t1a6: f64 = (p.p564 + t1a5);let t1a7: f64 = (p.p566 * l.fe91);let t1a8: f64 = (t1a6 + t1a7);let t1a9: f64 = (p.p567 * l.fd25);let t1aa: f64 = (t1a8 + t1a9);let t1ab: f64 = (l.fe91 * t1aa);l.f1437 = t1ab;}
        let t1ac: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };l.fc20 = t1ac;
        if ((l.fa1e != 0.0) && (l.fc20 != 0.0)) {let t1ad: f64 = (p.p569 * l.fdf1);let t1ae: f64 = (p.p568 + t1ad);let t1af: f64 = (p.p570 * l.fe91);let t1b0: f64 = (t1ae + t1af);let t1b1: f64 = (p.p571 * l.fd25);let t1b2: f64 = (t1b0 + t1b1);l.f16ae = t1b2;}
        let t1b3: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };l.fc2c = t1b3;
        if ((l.fa1e != 0.0) && (l.fc2c != 0.0)) {let t1b4: f64 = (p.p573 * l.fdf1);let t1b5: f64 = (p.p572 + t1b4);let t1b6: f64 = (p.p574 * l.fe91);let t1b7: f64 = (t1b5 + t1b6);let t1b8: f64 = (p.p575 * l.fd25);let t1b9: f64 = (t1b7 + t1b8);l.f143e = t1b9;}
        let t1ba: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };l.fc39 = t1ba;
        if ((l.fa1e != 0.0) && (l.fc39 != 0.0)) {let t1bb: f64 = (p.p577 * l.fdf1);let t1bc: f64 = (p.p576 + t1bb);let t1bd: f64 = (p.p578 * l.fe91);let t1be: f64 = (t1bc + t1bd);let t1bf: f64 = (p.p579 * l.fd25);let t1c0: f64 = (t1be + t1bf);l.f1444 = t1c0;}
        let t1c1: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };l.fc45 = t1c1;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.fc45 != 0.0)) {let t1c2: f64 = (p.p581 * l.fdf1);let t1c3: f64 = (p.p580 + t1c2);let t1c4: f64 = (p.p582 * l.fe91);let t1c5: f64 = (t1c3 + t1c4);let t1c6: f64 = (p.p583 * l.fd25);let t1c7: f64 = (t1c5 + t1c6);let t1c8: f64 = (l.fdf1 * t1c7);l.f175d = t1c8;}
        let t1c9: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };l.fc54 = t1c9;
        if ((l.fa1e != 0.0) && (l.fc54 != 0.0)) {let t1ca: f64 = (p.p585 * l.fdf1);let t1cb: f64 = (p.p584 + t1ca);let t1cc: f64 = (p.p586 * l.fe91);let t1cd: f64 = (t1cb + t1cc);let t1ce: f64 = (p.p587 * l.fd25);let t1cf: f64 = (t1cd + t1ce);l.f16bc = t1cf;}
        let t1d0: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };l.fc57 = t1d0;
        if ((l.fa1e != 0.0) && (l.fc57 != 0.0)) {let t1d1: f64 = (p.p589 * l.fdf1);let t1d2: f64 = (p.p588 + t1d1);let t1d3: f64 = (p.p590 * l.fe91);let t1d4: f64 = (t1d2 + t1d3);let t1d5: f64 = (p.p591 * l.fd25);let t1d6: f64 = (t1d4 + t1d5);l.f1775 = t1d6;}
        let t1d7: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };l.fc59 = t1d7;
        if ((l.fa1e != 0.0) && (l.fc59 != 0.0)) {let t1d8: f64 = (p.p593 * l.fdf1);let t1d9: f64 = (p.p592 + t1d8);let t1da: f64 = (p.p594 * l.fe91);let t1db: f64 = (t1d9 + t1da);let t1dc: f64 = (p.p595 * l.fd25);let t1dd: f64 = (t1db + t1dc);l.f1795 = t1dd;}
        let t1de: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };l.fc5b = t1de;
        if ((l.fa1e != 0.0) && (l.fc5b != 0.0)) {let t1df: f64 = (p.p597 * l.fdf1);let t1e0: f64 = (p.p596 + t1df);let t1e1: f64 = (p.p598 * l.fe91);let t1e2: f64 = (t1e0 + t1e1);let t1e3: f64 = (p.p599 * l.fd25);let t1e4: f64 = (t1e2 + t1e3);l.ffc = t1e4;}
        let t1e5: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };l.fc5d = t1e5;
        if ((l.fa1e != 0.0) && (l.fc5d != 0.0)) {let t1e6: f64 = (p.p601 * l.fdf1);let t1e7: f64 = (p.p600 + t1e6);let t1e8: f64 = (p.p602 * l.fe91);let t1e9: f64 = (t1e7 + t1e8);let t1ea: f64 = (p.p603 * l.fd25);let t1eb: f64 = (t1e9 + t1ea);let t1ec: f64 = (l.fdf1 * t1eb);l.f43 = t1ec;}
        let t1ed: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };l.fc5f = t1ed;
        if ((l.fa1e != 0.0) && (l.fc5f != 0.0)) {let t1ee: f64 = (p.p605 * l.fdf1);let t1ef: f64 = (p.p604 + t1ee);let t1f0: f64 = (p.p606 * l.fe91);let t1f1: f64 = (t1ef + t1f0);let t1f2: f64 = (p.p607 * l.fd25);let t1f3: f64 = (t1f1 + t1f2);l.f37 = t1f3;}
        let t1f4: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };l.fc61 = t1f4;
        if ((l.fa1e != 0.0) && (l.fc61 != 0.0)) {let t1f5: f64 = (p.p609 * l.fdf1);let t1f6: f64 = (p.p608 + t1f5);let t1f7: f64 = (p.p610 * l.fe91);let t1f8: f64 = (t1f6 + t1f7);let t1f9: f64 = (p.p611 * l.fd25);let t1fa: f64 = (t1f8 + t1f9);l.f3f = t1fa;}
        let t1fb: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };l.fc63 = t1fb;
        if ((l.fa1e != 0.0) && (l.fc63 != 0.0)) {let t1fc: f64 = (p.p613 * l.fdf1);let t1fd: f64 = (p.p612 + t1fc);let t1fe: f64 = (p.p614 * l.fe91);let t1ff: f64 = (t1fd + t1fe);let t200: f64 = (p.p615 * l.fd25);let t201: f64 = (t1ff + t200);l.f2 = t201;}
        let t202: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };l.fc65 = t202;
        if ((l.fa1e != 0.0) && (l.fc65 != 0.0)) {let t203: f64 = (p.p617 * l.fdf1);let t204: f64 = (p.p616 + t203);let t205: f64 = (p.p618 * l.fe91);let t206: f64 = (t204 + t205);let t207: f64 = (p.p619 * l.fd25);let t208: f64 = (t206 + t207);l.f1684 = t208;}
        let t209: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };l.fc67 = t209;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.fc67 != 0.0)) {let t20a: f64 = (p.p621 * l.fdf1);let t20b: f64 = (p.p620 + t20a);let t20c: f64 = (p.p622 * l.fe91);let t20d: f64 = (t20b + t20c);let t20e: f64 = (p.p623 * l.fd25);let t20f: f64 = (t20d + t20e);l.fd = t20f;}
        let t210: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };l.fc69 = t210;
        if ((l.fa1e != 0.0) && (l.fc69 != 0.0)) {let t211: f64 = (p.p625 * l.fdf1);let t212: f64 = (p.p624 + t211);let t213: f64 = (p.p626 * l.fe91);let t214: f64 = (t212 + t213);let t215: f64 = (p.p627 * l.fd25);let t216: f64 = (t214 + t215);l.f11 = t216;}
        let t217: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };l.fc6b = t217;
        if ((l.fa1e != 0.0) && (l.fc6b != 0.0)) {let t218: f64 = (p.p629 * l.fdf1);let t219: f64 = (p.p628 + t218);let t21a: f64 = (p.p630 * l.fe91);let t21b: f64 = (t219 + t21a);let t21c: f64 = (p.p631 * l.fd25);let t21d: f64 = (t21b + t21c);let t21e: f64 = (l.fd82 * t21d);l.fd72 = t21e;}
        let t21f: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };l.fc6d = t21f;
        if ((l.fa1e != 0.0) && (l.fc6d != 0.0)) {let t220: f64 = (p.p633 * l.fdf1);let t221: f64 = (p.p632 + t220);let t222: f64 = (p.p634 * l.fe91);let t223: f64 = (t221 + t222);let t224: f64 = (p.p635 * l.fd25);let t225: f64 = (t223 + t224);let t226: f64 = (l.fd8f * t225);l.fd76 = t226;}
        let t227: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };l.fc6f = t227;
        if ((l.fa1e != 0.0) && (l.fc6f != 0.0)) {let t228: f64 = (p.p637 * l.fdf1);let t229: f64 = (p.p636 + t228);let t22a: f64 = (p.p638 * l.fe91);let t22b: f64 = (t229 + t22a);let t22c: f64 = (p.p639 * l.fd25);let t22d: f64 = (t22b + t22c);let t22e: f64 = (l.fd8f * t22d);l.fd7a = t22e;}
        let t22f: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };l.fc71 = t22f;
        if ((l.fa1e != 0.0) && (l.fc71 != 0.0)) {let t230: f64 = (p.p641 * l.fdf1);let t231: f64 = (p.p640 + t230);let t232: f64 = (p.p642 * l.fe91);let t233: f64 = (t231 + t232);let t234: f64 = (p.p643 * l.fd25);let t235: f64 = (t233 + t234);l.f16a6 = t235;}
        let t236: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };l.fc73 = t236;
        if ((l.fa1e != 0.0) && (l.fc73 != 0.0)) {let t237: f64 = (p.p645 * l.fdf1);let t238: f64 = (p.p644 + t237);let t239: f64 = (p.p646 * l.fe91);let t23a: f64 = (t238 + t239);let t23b: f64 = (p.p647 * l.fd25);let t23c: f64 = (t23a + t23b);let t23d: f64 = (l.fd8f * t23c);l.f2b = t23d;}
        let t23e: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };l.fc75 = t23e;
        if ((l.fa1e != 0.0) && (l.fc75 != 0.0)) {let t23f: f64 = (p.p649 * l.fdf1);let t240: f64 = (p.p648 + t23f);let t241: f64 = (p.p650 * l.fe91);let t242: f64 = (t240 + t241);let t243: f64 = (p.p651 * l.fd25);let t244: f64 = (t242 + t243);let t245: f64 = (l.fd8f * t244);l.f2f = t245;}
        let t246: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };l.fc77 = t246;
        if ((l.fa1e != 0.0) && (l.fc77 != 0.0)) {let t247: f64 = (p.p653 * l.fdf1);let t248: f64 = (p.p652 + t247);let t249: f64 = (p.p654 * l.fe91);let t24a: f64 = (t248 + t249);let t24b: f64 = (p.p655 * l.fd25);let t24c: f64 = (t24a + t24b);l.f1690 = t24c;}
        let t24d: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };l.f685 = t24d;
        if ((l.fa1e != 0.0) && (l.f685 != 0.0)) {let t24e: f64 = (p.p657 * l.fdf1);let t24f: f64 = (p.p656 + t24e);let t250: f64 = (p.p658 * l.fe91);let t251: f64 = (t24f + t250);let t252: f64 = (p.p659 * l.fd25);let t253: f64 = (t251 + t252);l.f1694 = t253;}
        let t254: f64 = if (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) { 1.0 } else { 0.0 };l.f687 = t254;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f687 != 0.0)) {let t255: f64 = (l.fd91 * l.fef3);let t256: f64 = (t255 / 1e-6);let t257: f64 = (p.p661 * l.fdf1);let t258: f64 = (p.p660 + t257);let t259: f64 = (p.p662 * l.fe91);let t25a: f64 = (t258 + t259);let t25b: f64 = (p.p663 * l.fd25);let t25c: f64 = (t25a + t25b);let t25d: f64 = (t256 * t25c);l.f1dd = t25d;}
        let t25e: f64 = if (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) { 1.0 } else { 0.0 };l.f689 = t25e;
        if ((l.fa1e != 0.0) && (l.f689 != 0.0)) {let t25f: f64 = (p.p665 * l.fdf1);let t260: f64 = (p.p664 + t25f);let t261: f64 = (p.p666 * l.fe91);let t262: f64 = (t260 + t261);let t263: f64 = (p.p667 * l.fd25);let t264: f64 = (t262 + t263);l.f2e2 = t264;}
        let t265: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };l.f697 = t265;
        if ((l.fa1e != 0.0) && (l.f697 != 0.0)) {let t266: f64 = (p.p669 * l.fdf1);let t267: f64 = (p.p668 + t266);let t268: f64 = (p.p670 * l.fe91);let t269: f64 = (t267 + t268);let t26a: f64 = (p.p671 * l.fd25);let t26b: f64 = (t269 + t26a);l.f494 = t26b;}
        let t26c: f64 = if (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };l.f699 = t26c;
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f110c = p.p580;}
        let t26d: f64 = if param_given[672] { 1.0 } else { 0.0 };let t26e: f64 = if t26d == 1.0 { 1.0 } else { 0.0 };l.f69b = t26e;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f69b != 0.0)) {l.f110c = p.p672;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f10f8 = p.p581;}
        let t26f: f64 = if param_given[673] { 1.0 } else { 0.0 };let t270: f64 = if t26f == 1.0 { 1.0 } else { 0.0 };l.f69d = t270;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f69d != 0.0)) {l.f10f8 = p.p673;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f1148 = p.p582;}
        let t271: f64 = if param_given[674] { 1.0 } else { 0.0 };let t272: f64 = if t271 == 1.0 { 1.0 } else { 0.0 };l.f69f = t272;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f69f != 0.0)) {l.f1148 = p.p674;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f10fa = p.p583;}
        let t273: f64 = if param_given[675] { 1.0 } else { 0.0 };let t274: f64 = if t273 == 1.0 { 1.0 } else { 0.0 };l.f6a1 = t274;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f6a1 != 0.0)) {l.f10fa = p.p675;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {let t275: f64 = (l.f10f8 * l.fdf1);let t276: f64 = (l.f110c + t275);let t277: f64 = (l.f1148 * l.fe91);let t278: f64 = (t276 + t277);let t279: f64 = (l.f10fa * l.fd25);let t27a: f64 = (t278 + t279);let t27b: f64 = (l.fdf1 * t27a);l.f1764 = t27b;}
        let t27c: f64 = if (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };l.f6a3 = t27c;
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f110c = p.p596;}
        let t27d: f64 = if param_given[676] { 1.0 } else { 0.0 };let t27e: f64 = if t27d == 1.0 { 1.0 } else { 0.0 };l.f6a5 = t27e;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6a5 != 0.0)) {l.f110c = p.p676;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f10f8 = p.p597;}
        let t27f: f64 = if param_given[677] { 1.0 } else { 0.0 };let t280: f64 = if t27f == 1.0 { 1.0 } else { 0.0 };l.f6a7 = t280;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6a7 != 0.0)) {l.f10f8 = p.p677;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f1148 = p.p598;}
        let t281: f64 = if param_given[678] { 1.0 } else { 0.0 };let t282: f64 = if t281 == 1.0 { 1.0 } else { 0.0 };l.f6a9 = t282;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6a9 != 0.0)) {l.f1148 = p.p678;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f10fa = p.p599;}
        let t283: f64 = if param_given[679] { 1.0 } else { 0.0 };let t284: f64 = if t283 == 1.0 { 1.0 } else { 0.0 };l.f6ab = t284;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6ab != 0.0)) {l.f10fa = p.p679;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {let t285: f64 = (l.f10f8 * l.fdf1);let t286: f64 = (l.f110c + t285);let t287: f64 = (l.f1148 * l.fe91);let t288: f64 = (t286 + t287);let t289: f64 = (l.f10fa * l.fd25);let t28a: f64 = (t288 + t289);let t28b: f64 = t28a;l.f100 = t28b;}
        let t28c: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };l.f6ad = t28c;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f6ad != 0.0)) {let t28d: f64 = (p.p681 * l.fdf1);let t28e: f64 = (p.p680 + t28d);let t28f: f64 = (p.p682 * l.fe91);let t290: f64 = (t28e + t28f);let t291: f64 = (p.p683 * l.fd25);let t292: f64 = (t290 + t291);let t293: f64 = (l.fdf1 * t292);l.f47 = t293;}
        let t294: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };l.f6af = t294;
        if ((l.fa1e != 0.0) && (l.f6af != 0.0)) {let t295: f64 = (p.p685 * l.fdf1);let t296: f64 = (p.p684 + t295);let t297: f64 = (p.p686 * l.fe91);let t298: f64 = (t296 + t297);let t299: f64 = (p.p687 * l.fd25);let t29a: f64 = (t298 + t299);let t29b: f64 = (l.fdf1 * t29a);l.f3b = t29b;}
        let t29c: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };l.f6b1 = t29c;
        if ((l.fa1e != 0.0) && (l.f6b1 != 0.0)) {let t29d: f64 = (p.p689 * l.fdf1);let t29e: f64 = (p.p688 + t29d);let t29f: f64 = (p.p690 * l.fe91);let t2a0: f64 = (t29e + t29f);let t2a1: f64 = (p.p691 * l.fd25);let t2a2: f64 = (t2a0 + t2a1);let t2a3: f64 = (l.fd91 * t2a2);l.f1a7 = t2a3;}
        let t2a4: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };l.f6b3 = t2a4;
        if ((l.fa1e != 0.0) && (l.f6b3 != 0.0)) {let t2a5: f64 = (p.p693 * l.fdf1);let t2a6: f64 = (p.p692 + t2a5);let t2a7: f64 = (p.p694 * l.fe91);let t2a8: f64 = (t2a6 + t2a7);let t2a9: f64 = (p.p695 * l.fd25);let t2aa: f64 = (t2a8 + t2a9);let t2ab: f64 = (l.fd91 * t2aa);l.f1af = t2ab;}
        let t2ac: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };l.f6b5 = t2ac;
        if ((l.fa1e != 0.0) && (l.f6b5 != 0.0)) {let t2ad: f64 = (p.p697 * l.fdf1);let t2ae: f64 = (p.p696 + t2ad);let t2af: f64 = (p.p698 * l.fe91);let t2b0: f64 = (t2ae + t2af);let t2b1: f64 = (p.p699 * l.fd25);let t2b2: f64 = (t2b0 + t2b1);let t2b3: f64 = (l.fd84 * t2b2);l.f194 = t2b3;}
        let t2b4: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };l.f6b9 = t2b4;
        if ((l.fa1e != 0.0) && (l.f6b9 != 0.0)) {let t2b5: f64 = (p.p701 * l.fdf1);let t2b6: f64 = (p.p700 + t2b5);let t2b7: f64 = (p.p702 * l.fe91);let t2b8: f64 = (t2b6 + t2b7);let t2b9: f64 = (p.p703 * l.fd25);let t2ba: f64 = (t2b8 + t2b9);let t2bb: f64 = (l.fd91 * t2ba);l.f1b9 = t2bb;}
        let t2bc: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };l.f6cf = t2bc;
        if ((l.fa1e != 0.0) && (l.f6cf != 0.0)) {let t2bd: f64 = (p.p705 * l.fdf1);let t2be: f64 = (p.p704 + t2bd);let t2bf: f64 = (p.p706 * l.fe91);let t2c0: f64 = (t2be + t2bf);let t2c1: f64 = (p.p707 * l.fd25);let t2c2: f64 = (t2c0 + t2c1);let t2c3: f64 = (l.fd91 * t2c2);l.f1bd = t2c3;}
        let t2c4: f64 = if (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]) { 1.0 } else { 0.0 };l.f6e5 = t2c4;
        if ((l.fa1e != 0.0) && (l.f6e5 != 0.0)) {let t2c5: f64 = (p.p709 * l.fdf1);let t2c6: f64 = (p.p708 + t2c5);let t2c7: f64 = (p.p710 * l.fe91);let t2c8: f64 = (t2c6 + t2c7);let t2c9: f64 = (p.p711 * l.fd25);let t2ca: f64 = (t2c8 + t2c9);let t2cb: f64 = (l.fd8d * t2ca);l.f18c = t2cb;}
        let t2cc: f64 = if (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]) { 1.0 } else { 0.0 };l.f6fb = t2cc;
        if ((l.fa1e != 0.0) && (l.f6fb != 0.0)) {let t2cd: f64 = (p.p713 * l.fdf1);let t2ce: f64 = (p.p712 + t2cd);let t2cf: f64 = (p.p714 * l.fe91);let t2d0: f64 = (t2ce + t2cf);let t2d1: f64 = (p.p715 * l.fd25);let t2d2: f64 = (t2d0 + t2d1);let t2d3: f64 = (l.fd8d * t2d2);l.f190 = t2d3;}
        let t2d4: f64 = if (((param_given[716] || param_given[717]) || param_given[718]) || param_given[719]) { 1.0 } else { 0.0 };l.f711 = t2d4;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f711 != 0.0)) {let t2d5: f64 = (p.p717 * l.fdf1);let t2d6: f64 = (p.p716 + t2d5);let t2d7: f64 = (p.p718 * l.fe91);let t2d8: f64 = (t2d6 + t2d7);let t2d9: f64 = (p.p719 * l.fd25);let t2da: f64 = (t2d8 + t2d9);let t2db: f64 = (l.fdf2 * t2da);l.f524 = t2db;}
        let t2dc: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };l.f75a = t2dc;
        if ((l.fa1e != 0.0) && (l.f75a != 0.0)) {let t2dd: f64 = (p.p733 * l.fdf1);let t2de: f64 = (p.p732 + t2dd);let t2df: f64 = (p.p734 * l.fe91);let t2e0: f64 = (t2de + t2df);let t2e1: f64 = (p.p735 * l.fd25);let t2e2: f64 = (t2e0 + t2e1);l.f19ab = t2e2;}
        let t2e3: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };l.f770 = t2e3;
        if ((l.fa1e != 0.0) && (l.f770 != 0.0)) {let t2e4: f64 = (p.p737 * l.fdf1);let t2e5: f64 = (p.p736 + t2e4);let t2e6: f64 = (p.p738 * l.fe91);let t2e7: f64 = (t2e5 + t2e6);let t2e8: f64 = (p.p739 * l.fd25);let t2e9: f64 = (t2e7 + t2e8);l.f16c4 = t2e9;}
        let t2ea: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };l.f772 = t2ea;
        if ((l.fa1e != 0.0) && (l.f772 != 0.0)) {let t2eb: f64 = (p.p741 * l.fdf1);let t2ec: f64 = (p.p740 + t2eb);let t2ed: f64 = (p.p742 * l.fe91);let t2ee: f64 = (t2ec + t2ed);let t2ef: f64 = (p.p743 * l.fd25);let t2f0: f64 = (t2ee + t2ef);l.f32c = t2f0;}
        let t2f1: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };l.f774 = t2f1;
        if ((l.fa1e != 0.0) && (l.f774 != 0.0)) {let t2f2: f64 = (p.p745 * l.fdf1);let t2f3: f64 = (p.p744 + t2f2);let t2f4: f64 = (p.p746 * l.fe91);let t2f5: f64 = (t2f3 + t2f4);let t2f6: f64 = (p.p747 * l.fd25);let t2f7: f64 = (t2f5 + t2f6);l.ffbd = t2f7;}
        let t2f8: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };l.f776 = t2f8;
        if ((l.fa1e != 0.0) && (l.f776 != 0.0)) {let t2f9: f64 = (p.p749 * l.fdf1);let t2fa: f64 = (p.p748 + t2f9);let t2fb: f64 = (p.p750 * l.fe91);let t2fc: f64 = (t2fa + t2fb);let t2fd: f64 = (p.p751 * l.fd25);let t2fe: f64 = (t2fc + t2fd);l.f21a = t2fe;}
        let t2ff: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };l.f778 = t2ff;
        if ((l.fa1e != 0.0) && (l.f778 != 0.0)) {let t300: f64 = (l.f1afe / l.fef1);let t301: f64 = (p.p753 * l.fdf1);let t302: f64 = (p.p752 + t301);let t303: f64 = (p.p754 * l.fe91);let t304: f64 = (t302 + t303);let t305: f64 = (p.p755 * l.fd25);let t306: f64 = (t304 + t305);let t307: f64 = (t300 * t306);l.f124 = t307;}
        let t308: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };l.f77a = t308;
        if ((l.fa1e != 0.0) && (l.f77a != 0.0)) {let t309: f64 = (p.p757 * l.fdf1);let t30a: f64 = (p.p756 + t309);let t30b: f64 = (p.p758 * l.fe91);let t30c: f64 = (t30a + t30b);let t30d: f64 = (p.p759 * l.fd25);let t30e: f64 = (t30c + t30d);l.f168c = t30e;}
        let t30f: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };l.f77c = t30f;
        if ((l.fa1e != 0.0) && (l.f77c != 0.0)) {let t310: f64 = (p.p761 * l.fdf1);let t311: f64 = (p.p760 + t310);let t312: f64 = (p.p762 * l.fe91);let t313: f64 = (t311 + t312);let t314: f64 = (p.p763 * l.fd25);let t315: f64 = (t313 + t314);let t316: f64 = (l.fdf2 * t315);l.f113b = t316;}
        let t317: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };l.f77e = t317;
        if ((l.fa1e != 0.0) && (l.f77e != 0.0)) {let t318: f64 = (p.p765 * l.fdf1);let t319: f64 = (p.p764 + t318);let t31a: f64 = (p.p766 * l.fe91);let t31b: f64 = (t319 + t31a);let t31c: f64 = (p.p767 * l.fd25);let t31d: f64 = (t31b + t31c);l.f112f = t31d;}
        let t31e: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };l.f780 = t31e;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f780 != 0.0)) {let t31f: f64 = (p.p769 * l.fdf1);let t320: f64 = (p.p768 + t31f);let t321: f64 = (p.p770 * l.fe91);let t322: f64 = (t320 + t321);let t323: f64 = (p.p771 * l.fd25);let t324: f64 = (t322 + t323);l.f1137 = t324;}
        let t325: f64 = if (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]) { 1.0 } else { 0.0 };l.f782 = t325;
        if ((l.fa1e != 0.0) && (l.f782 != 0.0)) {let t326: f64 = (p.p773 * l.fdf1);let t327: f64 = (p.p772 + t326);let t328: f64 = (p.p774 * l.fe91);let t329: f64 = (t327 + t328);let t32a: f64 = (p.p775 * l.fd25);let t32b: f64 = (t329 + t32a);let t32c: f64 = (l.fdf2 * t32b);l.f188 = t32c;}
        let t32d: f64 = if (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]) { 1.0 } else { 0.0 };l.f784 = t32d;
        if ((l.fa1e != 0.0) && (l.f784 != 0.0)) {let t32e: f64 = (p.p781 * l.fdf1);let t32f: f64 = (p.p780 + t32e);let t330: f64 = (p.p782 * l.fe91);let t331: f64 = (t32f + t330);let t332: f64 = (p.p783 * l.fd25);let t333: f64 = (t331 + t332);l.f184 = t333;}
        let t334: f64 = if (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]) { 1.0 } else { 0.0 };l.f786 = t334;
        if ((l.fa1e != 0.0) && (l.f786 != 0.0)) {let t335: f64 = (p.p777 * l.fdf1);let t336: f64 = (p.p776 + t335);let t337: f64 = (p.p778 * l.fe91);let t338: f64 = (t336 + t337);let t339: f64 = (p.p779 * l.fd25);let t33a: f64 = (t338 + t339);l.f17c = t33a;}
        let t33b: f64 = if (((param_given[796] || param_given[797]) || param_given[798]) || param_given[799]) { 1.0 } else { 0.0 };l.f788 = t33b;
        if ((l.fa1e != 0.0) && (l.f788 != 0.0)) {let t33c: f64 = (p.p797 * l.fdf1);let t33d: f64 = (p.p796 + t33c);let t33e: f64 = (p.p798 * l.fe91);let t33f: f64 = (t33d + t33e);let t340: f64 = (p.p799 * l.fd25);let t341: f64 = (t33f + t340);let t342: f64 = (l.fd25 * t341);l.f144b = t342;}
        let t343: f64 = if (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]) { 1.0 } else { 0.0 };l.f789 = t343;
        if ((l.fa1e != 0.0) && (l.f789 != 0.0)) {let t344: f64 = (p.p801 * l.fdf1);let t345: f64 = (p.p800 + t344);let t346: f64 = (p.p802 * l.fe91);let t347: f64 = (t345 + t346);let t348: f64 = (p.p803 * l.fd25);let t349: f64 = (t347 + t348);let t34a: f64 = (l.fd82 * t349);l.f225 = t34a;}
        let t34b: f64 = if (((param_given[804] || param_given[805]) || param_given[806]) || param_given[807]) { 1.0 } else { 0.0 };l.f78b = t34b;
        if ((l.fa1e != 0.0) && (l.f78b != 0.0)) {let t34c: f64 = (p.p805 * l.fdf1);let t34d: f64 = (p.p804 + t34c);let t34e: f64 = (p.p806 * l.fe91);let t34f: f64 = (t34d + t34e);let t350: f64 = (p.p807 * l.fd25);let t351: f64 = (t34f + t350);l.f16b1 = t351;}
        if (l.fa1e != 0.0) {l.f17d3 = 0.0;l.f17d5 = 0.0;l.ff03 = 0.0;l.fedb = p.p812;}
        let t352: f64 = if param_given[813] { 1.0 } else { 0.0 };let t353: f64 = if t352 == 1.0 { 1.0 } else { 0.0 };l.f78c = t353;
        if ((l.fa1e != 0.0) && (l.f78c != 0.0)) {l.fedb = p.p813;}
        let t354: f64 = if (((l.f1487 > 0.0) && (l.f1489 > 0.0)) && ((l.ffbf == 1.0) || ((l.ffbf > 1.0) && (l.f1493 > 0.0)))) { 1.0 } else { 0.0 };l.f78e = t354;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let mut t366: usize = 0;
        while {
            let t364: f64 = (l.ffbf - 0.5);let t365: f64 = if (((l.fa1e != 0.0) && (l.f78e != 0.0)) && (l.ff03 < t364)) { 1.0 } else { 0.0 };
            t365 != 0.0
        } {
            t366 += 1;assert!(t366 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t355: f64 = (0.5 * l.fee0);let t356: f64 = (l.f1487 + t355);let t357: f64 = (l.f1493 + l.fee0);let t358: f64 = (l.ff03 * t357);let t359: f64 = (t356 + t358);let t35a: f64 = (1.0 / t359);let t35b: f64 = (l.f17d3 + t35a);l.f17d3 = t35b;let t35c: f64 = (0.5 * l.fee0);let t35d: f64 = (l.f1489 + t35c);let t35e: f64 = (l.f1493 + l.fee0);let t35f: f64 = (l.ff03 * t35e);let t360: f64 = (t35d + t35f);let t361: f64 = (1.0 / t360);let t362: f64 = (l.f17d5 + t361);l.f17d5 = t362;let t363: f64 = (l.ff03 + 1.0);l.ff03 = t363;}
        }
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t367: f64 = (l.f17d3 * l.fe57);l.fe59 = t367;let t368: f64 = (l.f17d5 * l.fe57);l.fe5d = t368;let t369: f64 = (0.5 * l.fee0);let t36a: f64 = (p.p808 + t369);let t36b: f64 = (1.0 / t36a);l.fe5b = t36b;let t36c: f64 = (0.5 * l.fee0);let t36d: f64 = (p.p809 + t36c);let t36e: f64 = (1.0 / t36d);l.fe5f = t36e;}
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {
            let t36f: f64 = (l.fee0 + l.f266);
            let (t371,) = {
    if (t36f > 1e-9) {
        let t370: f64 = (l.fee0 + l.f266);
        (t370,)
    } else {
        (1e-9,)
    }
};
            l.ff1e = t371;
        }
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {
            let t372: f64 = (l.f1ad4 + l.f2e8);let t373: f64 = (t372 + p.p810);
            let (t376,) = {
    if (t373 > 1e-9) {
        let t374: f64 = (l.f1ad4 + l.f2e8);let t375: f64 = (t374 + p.p810);
        (t375,)
    } else {
        (1e-9,)
    }
};
            l.f1b35 = t376;
        }
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t377: f64 = (l.ff1e).powf(p.p818);let t378: f64 = (1.0 / t377);l.f16fe = t378;let t379: f64 = (l.f1b35).powf(p.p819);let t37a: f64 = (1.0 / t379);l.f1700 = t37a;let t37b: f64 = (p.p815 * l.f16fe);let t37c: f64 = (1.0 + t37b);let t37d: f64 = (p.p816 * l.f1700);let t37e: f64 = (t37c + t37d);let t37f: f64 = (p.p817 * l.f16fe);let t380: f64 = (t37f * l.f1700);let t381: f64 = (t37e + t380);let t382: f64 = (l.f1448 - 1.0);let t383: f64 = (p.p814 * t382);let t384: f64 = (1.0 + t383);let t385: f64 = (t381 * t384);l.fec8 = t385;let t386: f64 = (l.fe59 + l.fe5d);let t387: f64 = (p.p811 * t386);let t388: f64 = (t387 / l.fec8);l.f1418 = t388;let t389: f64 = (l.fe5b + l.fe5f);let t38a: f64 = (p.p811 * t389);let t38b: f64 = (t38a / l.fec8);l.f141a = t38b;let t38c: f64 = (l.ff1e).powf(p.p824);let t38d: f64 = (1.0 / t38c);l.f16fe = t38d;let t38e: f64 = (l.f1b35).powf(p.p825);let t38f: f64 = (1.0 / t38e);l.f1700 = t38f;let t390: f64 = (p.p821 * l.f16fe);let t391: f64 = (1.0 + t390);let t392: f64 = (p.p822 * l.f1700);let t393: f64 = (t391 + t392);let t394: f64 = (p.p823 * l.f16fe);let t395: f64 = (t394 * l.f1700);let t396: f64 = (t393 + t395);l.feca = t396;let t397: f64 = (l.fe59 + l.fe5d);let t398: f64 = (t397 - l.fe5b);let t399: f64 = (t398 - l.fe5f);l.f16dd = t399;let t39a: f64 = (1.0 + l.f1418);let t39b: f64 = (1.0 + l.f141a);let t39c: f64 = (t39a / t39b);l.f16de = t39c;let t39d: f64 = (l.f11d * l.f16de);l.f11d = t39d;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t39e: f64 = (l.f175d * l.f16de);let t39f: f64 = (p.p812 * l.f141a);let t3a0: f64 = (1.0 + t39f);let t3a1: f64 = (t39e * t3a0);let t3a2: f64 = (p.p812 * l.f1418);let t3a3: f64 = (1.0 + t3a2);let t3a4: f64 = (t3a1 / t3a3);l.f175d = t3a4;let t3a5: f64 = (l.f1764 * l.f16de);let t3a6: f64 = (l.fedb * l.f141a);let t3a7: f64 = (1.0 + t3a6);let t3a8: f64 = (t3a5 * t3a7);let t3a9: f64 = (l.fedb * l.f1418);let t3aa: f64 = (1.0 + t3a9);let t3ab: f64 = (t3a8 / t3aa);l.f1764 = t3ab;let t3ac: f64 = (l.f124 * l.f16de);l.f124 = t3ac;let t3ad: f64 = (p.p820 * l.f16dd);let t3ae: f64 = (t3ad / l.feca);l.f16de = t3ae;let t3af: f64 = (l.f19a4 + l.f16de);l.f19a4 = t3af;let t3b0: f64 = (l.f19ab + l.f16de);l.f19ab = t3b0;let t3b1: f64 = (p.p826 * l.f16dd);let t3b2: f64 = (l.feca).powf(p.p827);let t3b3: f64 = (t3b1 / t3b2);l.f16de = t3b3;let t3b4: f64 = (l.f174 + l.f16de);l.f174 = t3b4;let t3b5: f64 = (l.f188 + l.f16de);l.f188 = t3b5;}
        let t3b6: f64 = if ((((l.f148d > 0.0) || (l.f148f > 0.0)) || (l.f1491 > 0.0)) || (l.f148b > 0.0)) { 1.0 } else { 0.0 };l.f79e = t3b6;let t3b7: f64 = if (((l.f148d == 0.0) && (l.f148f == 0.0)) && (l.f1491 == 0.0)) { 1.0 } else { 0.0 };l.f7b4 = t3b7;
        if (((l.fa1e != 0.0) && (l.f79e != 0.0)) && (l.f7b4 != 0.0)) {let t3b8: f64 = (l.f148b + l.f1ad4);l.f16dd = t3b8;let t3b9: f64 = (1.0 / p.p828);l.f16de = t3b9;let t3ba: f64 = (p.p828 * p.p828);let t3bb: f64 = (l.f148b * l.f16dd);let t3bc: f64 = (t3ba / t3bb);l.f148d = t3bc;let t3bd: f64 = (0.1 * l.f148b);let t3be: f64 = (0.01 * p.p828);let t3bf: f64 = (t3bd + t3be);let t3c0: f64 = (-10.0);let t3c1: f64 = (t3c0 * l.f148b);let t3c2: f64 = (t3c1 * l.f16de);let t3c3: f64 = (t3c2).exp();let t3c4: f64 = (t3bf * t3c3);let t3c5: f64 = (0.1 * l.f16dd);let t3c6: f64 = (0.01 * p.p828);let t3c7: f64 = (t3c5 + t3c6);let t3c8: f64 = (-10.0);let t3c9: f64 = (t3c8 * l.f16dd);let t3ca: f64 = (t3c9 * l.f16de);let t3cb: f64 = (t3ca).exp();let t3cc: f64 = (t3c7 * t3cb);let t3cd: f64 = (t3c4 - t3cc);let t3ce: f64 = (t3cd / l.f1ad4);l.f148f = t3ce;let t3cf: f64 = (0.05 * l.f148b);let t3d0: f64 = (0.0025 * p.p828);let t3d1: f64 = (t3cf + t3d0);let t3d2: f64 = (-20.0);let t3d3: f64 = (t3d2 * l.f148b);let t3d4: f64 = (t3d3 * l.f16de);let t3d5: f64 = (t3d4).exp();let t3d6: f64 = (t3d1 * t3d5);let t3d7: f64 = (0.05 * l.f16dd);let t3d8: f64 = (0.0025 * p.p828);let t3d9: f64 = (t3d7 + t3d8);let t3da: f64 = (-20.0);let t3db: f64 = (t3da * l.f16dd);let t3dc: f64 = (t3db * l.f16de);let t3dd: f64 = (t3dc).exp();let t3de: f64 = (t3d9 * t3dd);let t3df: f64 = (t3d6 - t3de);let t3e0: f64 = (t3df / l.f1ad4);l.f1491 = t3e0;}
        if ((l.fa1e != 0.0) && (l.f79e != 0.0)) {let t3e1: f64 = (p.p829 * l.f148f);let t3e2: f64 = (l.f148d + t3e1);let t3e3: f64 = (p.p830 * l.f1491);let t3e4: f64 = (t3e2 + t3e3);l.f16dd = t3e4;let t3e5: f64 = (l.fedd * l.f16dd);let t3e6: f64 = (l.f19a4 + t3e5);l.f19a4 = t3e6;let t3e7: f64 = (l.fed9 * l.f16dd);let t3e8: f64 = (1.0 + t3e7);let t3e9: f64 = (l.f11d * t3e8);l.f11d = t3e9;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f79e != 0.0)) {let t3ea: f64 = (l.fedd * l.f16dd);let t3eb: f64 = (l.f19ab + t3ea);l.f19ab = t3eb;let t3ec: f64 = (l.fed9 * l.f16dd);let t3ed: f64 = (1.0 + t3ec);let t3ee: f64 = (l.f124 * t3ed);l.f124 = t3ee;}
        l.f19a2 = l.f19a4;l.f16be = l.f16c0;l.f167e = l.f1680;l.f17df = l.f17e1;l.f41d = l.f41f;
        let (t3f0,) = {
    if (l.ffb7 > 1e20) {
        let (t3ef,) = {
            if (l.ffb7 < 1e26) {
                (l.ffb7,)
            } else {
                (1e26,)
            }
        };
        (t3ef,)
    } else {
        (1e20,)
    }
};
        l.ffb5 = t3f0;
        let (t3f1,) = {
    if (l.f5f4 > 0.01) {
        (l.f5f4,)
    } else {
        (0.01,)
    }
};
        l.f5f2 = t3f1;
        let (t3f2,) = {
    if (l.f1a72 > 0.0) {
        (l.f1a72,)
    } else {
        (0.0,)
    }
};
        l.f1a70 = t3f2;l.f3c7 = l.f3c9;l.f326 = l.f328;
        let (t3f3,) = {
    if (l.ffcd > 0.0) {
        (l.ffcd,)
    } else {
        (0.0,)
    }
};
        l.ffcb = t3f3;l.f17e5 = l.f17e7;l.f17e9 = l.f17eb;
        let (t3f5,) = {
    if (l.ffc4 > 1e23) {
        let (t3f4,) = {
            if (l.ffc4 < 1e27) {
                (l.ffc4,)
            } else {
                (1e27,)
            }
        };
        (t3f4,)
    } else {
        (1e23,)
    }
};
        l.ffc2 = t3f5;
        let (t3f7,) = {
    if (l.ffc8 > 1e23) {
        let (t3f6,) = {
            if (l.ffc8 < 1e27) {
                (l.ffc8,)
            } else {
                (1e27,)
            }
        };
        (t3f6,)
    } else {
        (1e23,)
    }
};
        l.ffc6 = t3f7;
        let (t3f8,) = {
    if (l.f209 > 0.0) {
        (l.f209,)
    } else {
        (0.0,)
    }
};
        l.f207 = t3f8;
        let (t3fa,) = {
    if (l.f216 > 0.0) {
        let (t3f9,) = {
            if (l.f216 < 0.5) {
                (l.f216,)
            } else {
                (0.5,)
            }
        };
        (t3f9,)
    } else {
        (0.0,)
    }
};
        l.f214 = t3fa;
        let (t3fc,) = {
    if (l.f21e > 0.0) {
        let (t3fb,) = {
            if (l.f21e < 1.0) {
                (l.f21e,)
            } else {
                (1.0,)
            }
        };
        (t3fb,)
    } else {
        (0.0,)
    }
};
        l.f21c = t3fc;l.f169a = l.f169c;
        let (t3fd,) = {
    if (l.f174 > 0.0) {
        (l.f174,)
    } else {
        (0.0,)
    }
};
        l.f172 = t3fd;
        let (t3ff,) = {
    if (l.f178 > 0.0) {
        let (t3fe,) = {
            if (l.f178 < 1.0) {
                (l.f178,)
            } else {
                (1.0,)
            }
        };
        (t3fe,)
    } else {
        (0.0,)
    }
};
        l.f176 = t3ff;
        let (t400,) = {
    if (l.f180 > 0.0) {
        (l.f180,)
    } else {
        (0.0,)
    }
};
        l.f17e = t400;
        let (t401,) = {
    if (l.f1127 > 0.0) {
        (l.f1127,)
    } else {
        (0.0,)
    }
};
        l.f1125 = t401;
        let (t403,) = {
    if (l.f112b > 0.0) {
        let (t402,) = {
            if (l.f112b < 1.0) {
                (l.f112b,)
            } else {
                (1.0,)
            }
        };
        (t402,)
    } else {
        (0.0,)
    }
};
        l.f1129 = t403;
        let (t404,) = {
    if (l.f1133 > 0.0) {
        (l.f1133,)
    } else {
        (0.0,)
    }
};
        l.f1131 = t404;
        let (t405,) = {
    if (l.f11d > 0.0) {
        (l.f11d,)
    } else {
        (0.0,)
    }
};
        l.f11b = t405;l.f1686 = l.f1688;
        let (t406,) = {
    if (l.ff99 > 0.0) {
        (l.ff99,)
    } else {
        (0.0,)
    }
};
        l.ff97 = t406;l.f16a8 = l.f16aa;
        let (t407,) = {
    if (l.f1731 > 0.0) {
        (l.f1731,)
    } else {
        (0.0,)
    }
};
        l.f172f = t407;l.f16b6 = l.f16b8;
        let (t408,) = {
    if (l.f1ee > 0.0) {
        (l.f1ee,)
    } else {
        (0.0,)
    }
};
        l.f1ec = t408;l.f1696 = l.f1698;
        let (t409,) = {
    if (l.f172a > 0.0) {
        (l.f172a,)
    } else {
        (0.0,)
    }
};
        l.f1728 = t409;l.f16b2 = l.f16b4;
        let (t40a,) = {
    if (l.f1bee > 0.0) {
        (l.f1bee,)
    } else {
        (0.0,)
    }
};
        l.f1bec = t40a;l.f16c6 = l.f16c8;l.f4e9 = l.f4eb;
        let (t40b,) = {
    if (l.f1437 > 0.0) {
        (l.f1437,)
    } else {
        (0.0,)
    }
};
        l.f1435 = t40b;l.f16ac = l.f16ae;let t40c: f64 = (-0.5);
        let (t40f,) = {
    if (l.f143e > t40c) {
        let (t40d,) = {
            if (l.f143e < 1.0) {
                (l.f143e,)
            } else {
                (1.0,)
            }
        };
        (t40d,)
    } else {
        let t40e: f64 = (-0.5);
        (t40e,)
    }
};
        l.f143c = t40f;let t410: f64 = (-0.5);
        let (t412,) = {
    if (l.f1444 > t410) {
        (l.f1444,)
    } else {
        let t411: f64 = (-0.5);
        (t411,)
    }
};
        l.f1442 = t412;
        let (t0,) = {
    if (l.f175d > 0.0) {
        (l.f175d,)
    } else {
        (0.0,)
    }
};
        l.f175b = t0;l.f16ba = l.f16bc;let t1: f64 = (-0.5);
        let (t4,) = {
    if (l.f1775 > t1) {
        let (t2,) = {
            if (l.f1775 < 1.0) {
                (l.f1775,)
            } else {
                (1.0,)
            }
        };
        (t2,)
    } else {
        let t3: f64 = (-0.5);
        (t3,)
    }
};
        l.f1773 = t4;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        l: &mut StampLocals,
    ) {
        let t5: f64 = (-0.5);
        let (t7,) = {
    if (l.f1795 > t5) {
        (l.f1795,)
    } else {
        let t6: f64 = (-0.5);
        (t6,)
    }
};
        l.f1793 = t7;
        let (t8,) = {
    if (l.f179f > 0.01) {
        (l.f179f,)
    } else {
        (0.01,)
    }
};
        l.f179d = t8;
        let (t9,) = {
    if (l.ffc > 2.0) {
        (l.ffc,)
    } else {
        (2.0,)
    }
};
        l.ffa = t9;
        let (ta,) = {
    if (l.f43 > 0.0) {
        (l.f43,)
    } else {
        (0.0,)
    }
};
        l.f41 = ta;
        let (tb,) = {
    if (l.f37 > 0.0) {
        (l.f37,)
    } else {
        (0.0,)
    }
};
        l.f35 = tb;
        let (tc,) = {
    if (l.f3f > 0.0) {
        (l.f3f,)
    } else {
        (0.0,)
    }
};
        l.f3d = tc;l.f1a6c = l.f1a6e;
        let (td,) = {
    if (l.f2 > 0.0) {
        (l.f2,)
    } else {
        (0.0,)
    }
};
        l.f0 = td;l.f4 = l.f6;l.f1682 = l.f1684;
        let (te,) = {
    if (l.fd > 0.0) {
        (l.fd,)
    } else {
        (0.0,)
    }
};
        l.fb = te;
        let (tf,) = {
    if (l.f11 > 0.0) {
        (l.f11,)
    } else {
        (0.0,)
    }
};
        l.ff = tf;
        let (t10,) = {
    if (l.fdf7 > 1e-12) {
        (l.fdf7,)
    } else {
        (1e-12,)
    }
};
        l.fdf5 = t10;l.f5a2 = l.f5a4;
        let (t11,) = {
    if (l.fd72 > 0.0) {
        (l.fd72,)
    } else {
        (0.0,)
    }
};
        l.fd70 = t11;
        let (t12,) = {
    if (l.fd76 > 0.0) {
        (l.fd76,)
    } else {
        (0.0,)
    }
};
        l.fd74 = t12;
        let (t13,) = {
    if (l.fd7a > 0.0) {
        (l.fd7a,)
    } else {
        (0.0,)
    }
};
        l.fd78 = t13;l.f16a4 = l.f16a6;l.f58a = l.f58c;l.f596 = l.f598;l.f58e = l.f590;l.f59a = l.f59c;l.f592 = l.f594;l.f59e = l.f5a0;l.f1b1 = l.f1b3;
        let (t14,) = {
    if (l.f2b > 0.0) {
        (l.f2b,)
    } else {
        (0.0,)
    }
};
        l.f29 = t14;
        let (t15,) = {
    if (l.f2f > 0.0) {
        (l.f2f,)
    } else {
        (0.0,)
    }
};
        l.f2d = t15;l.f12f = l.f131;l.f135 = l.f137;l.f168e = l.f1690;l.f1692 = l.f1694;l.f19d = l.f19f;l.f1a1 = l.f1a3;
        let (t16,) = {
    if (l.f1dd > 0.0) {
        (l.f1dd,)
    } else {
        (0.0,)
    }
};
        l.f1d9 = t16;l.f2e0 = l.f2e2;
        let (t17,) = {
    if (l.f494 > 0.0) {
        (l.f494,)
    } else {
        (0.0,)
    }
};
        l.f492 = t17;
        let (t18,) = {
    if (l.f1764 > 0.0) {
        (l.f1764,)
    } else {
        (0.0,)
    }
};
        l.f1762 = t18;
        let (t19,) = {
    if (l.f100 > 2.0) {
        (l.f100,)
    } else {
        (2.0,)
    }
};
        l.ffe = t19;l.f45 = l.f47;
        let (t1a,) = {
    if (l.f3b > 0.0) {
        (l.f3b,)
    } else {
        (0.0,)
    }
};
        l.f39 = t1a;
        let (t1b,) = {
    if (l.f1a7 > 0.0) {
        (l.f1a7,)
    } else {
        (0.0,)
    }
};
        l.f1a5 = t1b;
        let (t1c,) = {
    if (l.f1af > 0.0) {
        (l.f1af,)
    } else {
        (0.0,)
    }
};
        l.f1ad = t1c;l.f4d7 = l.f4d9;l.f4db = l.f4dd;l.f1a9 = l.f1ab;
        let (t1d,) = {
    if (l.f194 > 0.0) {
        (l.f194,)
    } else {
        (0.0,)
    }
};
        l.f192 = t1d;
        let (t1e,) = {
    if (l.f1b9 > 0.0) {
        (l.f1b9,)
    } else {
        (0.0,)
    }
};
        l.f1b7 = t1e;
        let (t1f,) = {
    if (l.f1bd > 0.0) {
        (l.f1bd,)
    } else {
        (0.0,)
    }
};
        l.f1bb = t1f;l.f3ae = l.f3b0;l.f4e3 = l.f4e5;l.f4df = l.f4e1;l.f106 = l.f108;
        let (t20,) = {
    if (l.f18c > 0.0) {
        (l.f18c,)
    } else {
        (0.0,)
    }
};
        l.f18a = t20;
        let (t21,) = {
    if (l.f190 > 0.0) {
        (l.f190,)
    } else {
        (0.0,)
    }
};
        l.f18e = t21;l.f51f = l.f521;
        let (t22,) = {
    if (l.f524 > 0.0) {
        (l.f524,)
    } else {
        (0.0,)
    }
};
        l.f523 = t22;l.f19a9 = l.f19ab;l.f16c2 = l.f16c4;l.f32a = l.f32c;
        let (t24,) = {
    if (l.ffbd > 1e20) {
        let (t23,) = {
            if (l.ffbd < 1e26) {
                (l.ffbd,)
            } else {
                (1e26,)
            }
        };
        (t23,)
    } else {
        (1e20,)
    }
};
        l.ffbb = t24;
        let (t25,) = {
    if (l.f21a > 0.0) {
        (l.f21a,)
    } else {
        (0.0,)
    }
};
        l.f218 = t25;
        let (t26,) = {
    if (l.f124 > 0.0) {
        (l.f124,)
    } else {
        (0.0,)
    }
};
        l.f122 = t26;
    }
}
