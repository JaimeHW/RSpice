#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.fbbe != 0.0)) {let t10e: f64 = (p.p605 * l.fd35);let t10f: f64 = (p.p604 + t10e);let t110: f64 = (p.p606 * l.fdc8);let t111: f64 = (t10f + t110);let t112: f64 = (p.p607 * l.fc70);let t113: f64 = (t111 + t112);l.f14c8 = t113;l.f14c9 = 0.0;}
        let t114: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };l.fbc0 = t114;l.fbc1 = 0.0;
        if ((l.f976 != 0.0) && (l.fbc0 != 0.0)) {let t115: f64 = (p.p609 * l.fd35);let t116: f64 = (p.p608 + t115);let t117: f64 = (p.p610 * l.fdc8);let t118: f64 = (t116 + t117);let t119: f64 = (p.p611 * l.fc70);let t11a: f64 = (t118 + t119);l.fc = t11a;l.fd = 0.0;}
        let t11b: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };l.fbc2 = t11b;l.fbc3 = 0.0;
        if ((l.f976 != 0.0) && (l.fbc2 != 0.0)) {let t11c: f64 = (p.p613 * l.fd35);let t11d: f64 = (p.p612 + t11c);let t11e: f64 = (p.p614 * l.fdc8);let t11f: f64 = (t11d + t11e);let t120: f64 = (p.p615 * l.fc70);let t121: f64 = (t11f + t120);l.f10 = t121;l.f11 = 0.0;}
        let t122: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };l.fbc4 = t122;l.fbc5 = 0.0;
        if ((l.f976 != 0.0) && (l.fbc4 != 0.0)) {let t123: f64 = (p.p617 * l.fd35);let t124: f64 = (p.p616 + t123);let t125: f64 = (p.p618 * l.fdc8);let t126: f64 = (t124 + t125);let t127: f64 = (p.p619 * l.fc70);let t128: f64 = (t126 + t127);let t129: f64 = (l.fcc7 * t128);l.fcb8 = t129;l.fcb9 = 0.0;}
        let t12a: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };l.fbc6 = t12a;l.fbc7 = 0.0;
        if ((l.f976 != 0.0) && (l.fbc6 != 0.0)) {let t12b: f64 = (p.p621 * l.fd35);let t12c: f64 = (p.p620 + t12b);let t12d: f64 = (p.p622 * l.fdc8);let t12e: f64 = (t12c + t12d);let t12f: f64 = (p.p623 * l.fc70);let t130: f64 = (t12e + t12f);let t131: f64 = (l.fcd3 * t130);l.fcbc = t131;l.fcbd = 0.0;}
        let t132: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };l.fbc8 = t132;l.fbc9 = 0.0;
        if ((l.f976 != 0.0) && (l.fbc8 != 0.0)) {let t133: f64 = (p.p625 * l.fd35);let t134: f64 = (p.p624 + t133);let t135: f64 = (p.p626 * l.fdc8);let t136: f64 = (t134 + t135);let t137: f64 = (p.p627 * l.fc70);let t138: f64 = (t136 + t137);let t139: f64 = (l.fcd3 * t138);l.fcc0 = t139;l.fcc1 = 0.0;}
        let t13a: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };l.fbca = t13a;l.fbcb = 0.0;
        if ((l.f976 != 0.0) && (l.fbca != 0.0)) {let t13b: f64 = (p.p629 * l.fd35);let t13c: f64 = (p.p628 + t13b);let t13d: f64 = (p.p630 * l.fdc8);let t13e: f64 = (t13c + t13d);let t13f: f64 = (p.p631 * l.fc70);let t140: f64 = (t13e + t13f);l.f14ea = t140;l.f14eb = 0.0;}
        let t141: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };l.fbcc = t141;l.fbcd = 0.0;
        if ((l.f976 != 0.0) && (l.fbcc != 0.0)) {let t142: f64 = (p.p633 * l.fd35);let t143: f64 = (p.p632 + t142);let t144: f64 = (p.p634 * l.fdc8);let t145: f64 = (t143 + t144);let t146: f64 = (p.p635 * l.fc70);let t147: f64 = (t145 + t146);let t148: f64 = (l.fcd3 * t147);l.f29 = t148;l.f2a = 0.0;}
        let t149: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };l.fbce = t149;l.fbcf = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.fbce != 0.0)) {let t14a: f64 = (p.p637 * l.fd35);let t14b: f64 = (p.p636 + t14a);let t14c: f64 = (p.p638 * l.fdc8);let t14d: f64 = (t14b + t14c);let t14e: f64 = (p.p639 * l.fc70);let t14f: f64 = (t14d + t14e);let t150: f64 = (l.fcd3 * t14f);l.f2d = t150;l.f2e = 0.0;}
        let t151: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };l.fbd0 = t151;l.fbd1 = 0.0;
        if ((l.f976 != 0.0) && (l.fbd0 != 0.0)) {let t152: f64 = (p.p641 * l.fd35);let t153: f64 = (p.p640 + t152);let t154: f64 = (p.p642 * l.fdc8);let t155: f64 = (t153 + t154);let t156: f64 = (p.p643 * l.fc70);let t157: f64 = (t155 + t156);l.f14d4 = t157;l.f14d5 = 0.0;}
        let t158: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };l.f5e5 = t158;l.f5e6 = 0.0;
        if ((l.f976 != 0.0) && (l.f5e5 != 0.0)) {let t159: f64 = (p.p645 * l.fd35);let t15a: f64 = (p.p644 + t159);let t15b: f64 = (p.p646 * l.fdc8);let t15c: f64 = (t15a + t15b);let t15d: f64 = (p.p647 * l.fc70);let t15e: f64 = (t15c + t15d);l.f14d8 = t15e;l.f14d9 = 0.0;}
        let t15f: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };l.f5e7 = t15f;l.f5ec = 0.0;
        if ((l.f976 != 0.0) && (l.f5e7 != 0.0)) {let t160: f64 = (l.fcd5 * l.fe21);let t161: f64 = (t160 / 1e-6);let t162: f64 = (p.p649 * l.fd35);let t163: f64 = (p.p648 + t162);let t164: f64 = (p.p650 * l.fdc8);let t165: f64 = (t163 + t164);let t166: f64 = (p.p651 * l.fc70);let t167: f64 = (t165 + t166);let t168: f64 = (t161 * t167);l.f1bb = t168;l.f1bc = 0.0;}
        let t169: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };l.f5ed = t169;l.f5ee = 0.0;
        if ((l.f976 != 0.0) && (l.f5ed != 0.0)) {let t16a: f64 = (p.p653 * l.fd35);let t16b: f64 = (p.p652 + t16a);let t16c: f64 = (p.p654 * l.fdc8);let t16d: f64 = (t16b + t16c);let t16e: f64 = (p.p655 * l.fc70);let t16f: f64 = (t16d + t16e);l.f29d = t16f;l.f29e = 0.0;}
        let t170: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };l.f5ef = t170;l.f5f0 = 0.0;
        if ((l.f976 != 0.0) && (l.f5ef != 0.0)) {let t171: f64 = (p.p657 * l.fd35);let t172: f64 = (p.p656 + t171);let t173: f64 = (p.p658 * l.fdc8);let t174: f64 = (t172 + t173);let t175: f64 = (p.p659 * l.fc70);let t176: f64 = (t174 + t175);l.f422 = t176;l.f423 = 0.0;}
        let t177: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };l.f5f1 = t177;l.f5f2 = 0.0;
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.fff8 = p.p568;l.fff9 = 0.0;}
        let t178: f64 = if param_given[660] { 1.0 } else { 0.0 };let t179: f64 = if t178 == 1.0 { 1.0 } else { 0.0 };l.f5f3 = t179;l.f5f4 = 0.0;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f3 != 0.0)) {l.fff8 = p.p660;l.fff9 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.ffe6 = p.p569;l.ffe7 = 0.0;}
        let t17a: f64 = if param_given[661] { 1.0 } else { 0.0 };let t17b: f64 = if t17a == 1.0 { 1.0 } else { 0.0 };l.f5f5 = t17b;l.f5f6 = 0.0;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f5 != 0.0)) {l.ffe6 = p.p661;l.ffe7 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.f1030 = p.p570;l.f1031 = 0.0;}
        let t17c: f64 = if param_given[662] { 1.0 } else { 0.0 };let t17d: f64 = if t17c == 1.0 { 1.0 } else { 0.0 };l.f5f7 = t17d;l.f5f8 = 0.0;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f7 != 0.0)) {l.f1030 = p.p662;l.f1031 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {l.ffe8 = p.p571;l.ffe9 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let t17e: f64 = if param_given[663] { 1.0 } else { 0.0 };let t17f: f64 = if t17e == 1.0 { 1.0 } else { 0.0 };l.f5f9 = t17f;l.f5fa = 0.0;
        if (((l.f976 != 0.0) && (l.f5f1 != 0.0)) && (l.f5f9 != 0.0)) {l.ffe8 = p.p663;l.ffe9 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5f1 != 0.0)) {let t180: f64 = (l.ffe6 * l.fd35);let t181: f64 = (l.fff8 + t180);let t182: f64 = (l.f1030 * l.fdc8);let t183: f64 = (t181 + t182);let t184: f64 = (l.ffe8 * l.fc70);let t185: f64 = (t183 + t184);let t186: f64 = (l.fd35 * t185);l.f158f = t186;l.f1590 = 0.0;}
        let t187: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };l.f5fb = t187;l.f5fc = 0.0;
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.fff8 = p.p584;l.fff9 = 0.0;}
        let t188: f64 = if param_given[664] { 1.0 } else { 0.0 };let t189: f64 = if t188 == 1.0 { 1.0 } else { 0.0 };l.f5fd = t189;l.f5fe = 0.0;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f5fd != 0.0)) {l.fff8 = p.p664;l.fff9 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.ffe6 = p.p585;l.ffe7 = 0.0;}
        let t18a: f64 = if param_given[665] { 1.0 } else { 0.0 };let t18b: f64 = if t18a == 1.0 { 1.0 } else { 0.0 };l.f5ff = t18b;l.f600 = 0.0;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f5ff != 0.0)) {l.ffe6 = p.p665;l.ffe7 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.f1030 = p.p586;l.f1031 = 0.0;}
        let t18c: f64 = if param_given[666] { 1.0 } else { 0.0 };let t18d: f64 = if t18c == 1.0 { 1.0 } else { 0.0 };l.f601 = t18d;l.f602 = 0.0;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f601 != 0.0)) {l.f1030 = p.p666;l.f1031 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {l.ffe8 = p.p587;l.ffe9 = 0.0;}
        let t18e: f64 = if param_given[667] { 1.0 } else { 0.0 };let t18f: f64 = if t18e == 1.0 { 1.0 } else { 0.0 };l.f603 = t18f;l.f604 = 0.0;
        if (((l.f976 != 0.0) && (l.f5fb != 0.0)) && (l.f603 != 0.0)) {l.ffe8 = p.p667;l.ffe9 = 0.0;}
        if ((l.f976 != 0.0) && (l.f5fb != 0.0)) {let t190: f64 = (l.ffe6 * l.fd35);let t191: f64 = (l.fff8 + t190);let t192: f64 = (l.f1030 * l.fdc8);let t193: f64 = (t191 + t192);let t194: f64 = (l.ffe8 * l.fc70);let t195: f64 = (t193 + t194);let t196: f64 = t195;l.fe8 = t196;l.fe9 = 0.0;}
        let t197: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };l.f605 = t197;l.f606 = 0.0;
        if ((l.f976 != 0.0) && (l.f605 != 0.0)) {let t198: f64 = (p.p669 * l.fd35);let t199: f64 = (p.p668 + t198);let t19a: f64 = (p.p670 * l.fdc8);let t19b: f64 = (t199 + t19a);let t19c: f64 = (p.p671 * l.fc70);let t19d: f64 = (t19b + t19c);let t19e: f64 = (l.fd35 * t19d);l.f45 = t19e;l.f46 = 0.0;}
        let t19f: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };l.f607 = t19f;l.f608 = 0.0;
        if ((l.f976 != 0.0) && (l.f607 != 0.0)) {let t1a0: f64 = (p.p673 * l.fd35);let t1a1: f64 = (p.p672 + t1a0);let t1a2: f64 = (p.p674 * l.fdc8);let t1a3: f64 = (t1a1 + t1a2);let t1a4: f64 = (p.p675 * l.fc70);let t1a5: f64 = (t1a3 + t1a4);let t1a6: f64 = (l.fd35 * t1a5);l.f39 = t1a6;l.f3a = 0.0;}
        let t1a7: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };l.f609 = t1a7;l.f60a = 0.0;
        if ((l.f976 != 0.0) && (l.f609 != 0.0)) {let t1a8: f64 = (p.p677 * l.fd35);let t1a9: f64 = (p.p676 + t1a8);let t1aa: f64 = (p.p678 * l.fdc8);let t1ab: f64 = (t1a9 + t1aa);let t1ac: f64 = (p.p679 * l.fc70);let t1ad: f64 = (t1ab + t1ac);let t1ae: f64 = (l.fcd5 * t1ad);l.f185 = t1ae;l.f186 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let t1af: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };l.f60b = t1af;l.f61c = 0.0;
        if ((l.f976 != 0.0) && (l.f60b != 0.0)) {let t1b0: f64 = (p.p681 * l.fd35);let t1b1: f64 = (p.p680 + t1b0);let t1b2: f64 = (p.p682 * l.fdc8);let t1b3: f64 = (t1b1 + t1b2);let t1b4: f64 = (p.p683 * l.fc70);let t1b5: f64 = (t1b3 + t1b4);let t1b6: f64 = (l.fcd5 * t1b5);l.f18d = t1b6;l.f18e = 0.0;}
        let t1b7: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };l.f61d = t1b7;l.f632 = 0.0;
        if ((l.f976 != 0.0) && (l.f61d != 0.0)) {let t1b8: f64 = (p.p685 * l.fd35);let t1b9: f64 = (p.p684 + t1b8);let t1ba: f64 = (p.p686 * l.fdc8);let t1bb: f64 = (t1b9 + t1ba);let t1bc: f64 = (p.p687 * l.fc70);let t1bd: f64 = (t1bb + t1bc);let t1be: f64 = (l.fcc9 * t1bd);l.f173 = t1be;l.f174 = 0.0;}
        let t1bf: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };l.f633 = t1bf;l.f648 = 0.0;
        if ((l.f976 != 0.0) && (l.f633 != 0.0)) {let t1c0: f64 = (p.p689 * l.fd35);let t1c1: f64 = (p.p688 + t1c0);let t1c2: f64 = (p.p690 * l.fdc8);let t1c3: f64 = (t1c1 + t1c2);let t1c4: f64 = (p.p691 * l.fc70);let t1c5: f64 = (t1c3 + t1c4);let t1c6: f64 = (l.fcd5 * t1c5);l.f197 = t1c6;l.f198 = 0.0;}
        let t1c7: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };l.f649 = t1c7;l.f65e = 0.0;
        if ((l.f976 != 0.0) && (l.f649 != 0.0)) {let t1c8: f64 = (p.p693 * l.fd35);let t1c9: f64 = (p.p692 + t1c8);let t1ca: f64 = (p.p694 * l.fdc8);let t1cb: f64 = (t1c9 + t1ca);let t1cc: f64 = (p.p695 * l.fc70);let t1cd: f64 = (t1cb + t1cc);let t1ce: f64 = (l.fcd5 * t1cd);l.f19b = t1ce;l.f19c = 0.0;}
        let t1cf: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };l.f65f = t1cf;l.f674 = 0.0;
        if ((l.f976 != 0.0) && (l.f65f != 0.0)) {let t1d0: f64 = (p.p697 * l.fd35);let t1d1: f64 = (p.p696 + t1d0);let t1d2: f64 = (p.p698 * l.fdc8);let t1d3: f64 = (t1d1 + t1d2);let t1d4: f64 = (p.p699 * l.fc70);let t1d5: f64 = (t1d3 + t1d4);let t1d6: f64 = (l.fcd1 * t1d5);l.f16b = t1d6;l.f16c = 0.0;}
        let t1d7: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };l.f675 = t1d7;l.f686 = 0.0;
        if ((l.f976 != 0.0) && (l.f675 != 0.0)) {let t1d8: f64 = (p.p701 * l.fd35);let t1d9: f64 = (p.p700 + t1d8);let t1da: f64 = (p.p702 * l.fdc8);let t1db: f64 = (t1d9 + t1da);let t1dc: f64 = (p.p703 * l.fc70);let t1dd: f64 = (t1db + t1dc);let t1de: f64 = (l.fcd1 * t1dd);l.f16f = t1de;l.f170 = 0.0;}
        let t1df: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };l.f6c6 = t1df;l.f6c7 = 0.0;
        if ((l.f976 != 0.0) && (l.f6c6 != 0.0)) {let t1e0: f64 = (p.p721 * l.fd35);let t1e1: f64 = (p.p720 + t1e0);let t1e2: f64 = (p.p722 * l.fdc8);let t1e3: f64 = (t1e1 + t1e2);let t1e4: f64 = (p.p723 * l.fc70);let t1e5: f64 = (t1e3 + t1e4);l.f17b1 = t1e5;l.f17b2 = 0.0;}
        let t1e6: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };l.f6c8 = t1e6;l.f6c9 = 0.0;
        if ((l.f976 != 0.0) && (l.f6c8 != 0.0)) {let t1e7: f64 = (p.p725 * l.fd35);let t1e8: f64 = (p.p724 + t1e7);let t1e9: f64 = (p.p726 * l.fdc8);let t1ea: f64 = (t1e8 + t1e9);let t1eb: f64 = (p.p727 * l.fc70);let t1ec: f64 = (t1ea + t1eb);l.f1506 = t1ec;l.f1507 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        let t1ed: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };l.f6ca = t1ed;l.f6cb = 0.0;
        if ((l.f976 != 0.0) && (l.f6ca != 0.0)) {let t1ee: f64 = (p.p729 * l.fd35);let t1ef: f64 = (p.p728 + t1ee);let t1f0: f64 = (p.p730 * l.fdc8);let t1f1: f64 = (t1ef + t1f0);let t1f2: f64 = (p.p731 * l.fc70);let t1f3: f64 = (t1f1 + t1f2);l.f2e0 = t1f3;l.f2e1 = 0.0;}
        let t1f4: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };l.f6cc = t1f4;l.f6cd = 0.0;
        if ((l.f976 != 0.0) && (l.f6cc != 0.0)) {let t1f5: f64 = (p.p733 * l.fd35);let t1f6: f64 = (p.p732 + t1f5);let t1f7: f64 = (p.p734 * l.fdc8);let t1f8: f64 = (t1f6 + t1f7);let t1f9: f64 = (p.p735 * l.fc70);let t1fa: f64 = (t1f8 + t1f9);l.fedb = t1fa;l.fedc = 0.0;}
        let t1fb: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };l.f6ce = t1fb;l.f6cf = 0.0;
        if ((l.f976 != 0.0) && (l.f6ce != 0.0)) {let t1fc: f64 = (p.p737 * l.fd35);let t1fd: f64 = (p.p736 + t1fc);let t1fe: f64 = (p.p738 * l.fdc8);let t1ff: f64 = (t1fd + t1fe);let t200: f64 = (p.p739 * l.fc70);let t201: f64 = (t1ff + t200);l.f1f3 = t201;l.f1f4 = 0.0;}
        let t202: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };l.f6d0 = t202;l.f6d1 = 0.0;
        if ((l.f976 != 0.0) && (l.f6d0 != 0.0)) {let t203: f64 = (l.f18e9 / l.fe1f);let t204: f64 = (p.p741 * l.fd35);let t205: f64 = (p.p740 + t204);let t206: f64 = (p.p742 * l.fdc8);let t207: f64 = (t205 + t206);let t208: f64 = (p.p743 * l.fc70);let t209: f64 = (t207 + t208);let t20a: f64 = (t203 * t209);l.f109 = t20a;l.f10a = 0.0;}
        let t20b: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };l.f6d2 = t20b;l.f6d3 = 0.0;
        if ((l.f976 != 0.0) && (l.f6d2 != 0.0)) {let t20c: f64 = (p.p745 * l.fd35);let t20d: f64 = (p.p744 + t20c);let t20e: f64 = (p.p746 * l.fdc8);let t20f: f64 = (t20d + t20e);let t210: f64 = (p.p747 * l.fc70);let t211: f64 = (t20f + t210);l.f14d0 = t211;l.f14d1 = 0.0;}
        let t212: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };l.f6d4 = t212;l.f6d5 = 0.0;
        if ((l.f976 != 0.0) && (l.f6d4 != 0.0)) {let t213: f64 = (p.p749 * l.fd35);let t214: f64 = (p.p748 + t213);let t215: f64 = (p.p750 * l.fdc8);let t216: f64 = (t214 + t215);let t217: f64 = (p.p751 * l.fc70);let t218: f64 = (t216 + t217);let t219: f64 = (l.fd36 * t218);l.f1024 = t219;l.f1025 = 0.0;}
        let t21a: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };l.f6d6 = t21a;l.f6d7 = 0.0;
        if ((l.f976 != 0.0) && (l.f6d6 != 0.0)) {let t21b: f64 = (p.p753 * l.fd35);let t21c: f64 = (p.p752 + t21b);let t21d: f64 = (p.p754 * l.fdc8);let t21e: f64 = (t21c + t21d);let t21f: f64 = (p.p755 * l.fc70);let t220: f64 = (t21e + t21f);l.f1018 = t220;l.f1019 = 0.0;}
        let t221: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };l.f6d8 = t221;l.f6d9 = 0.0;
        if ((l.f976 != 0.0) && (l.f6d8 != 0.0)) {let t222: f64 = (p.p757 * l.fd35);let t223: f64 = (p.p756 + t222);let t224: f64 = (p.p758 * l.fdc8);let t225: f64 = (t223 + t224);let t226: f64 = (p.p759 * l.fc70);let t227: f64 = (t225 + t226);l.f1020 = t227;l.f1021 = 0.0;}
        let t228: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };l.f6da = t228;l.f6db = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f6da != 0.0)) {let t229: f64 = (p.p761 * l.fd35);let t22a: f64 = (p.p760 + t229);let t22b: f64 = (p.p762 * l.fdc8);let t22c: f64 = (t22a + t22b);let t22d: f64 = (p.p763 * l.fc70);let t22e: f64 = (t22c + t22d);let t22f: f64 = (l.fd36 * t22e);l.f167 = t22f;l.f168 = 0.0;}
        let t230: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };l.f6dc = t230;l.f6dd = 0.0;
        if ((l.f976 != 0.0) && (l.f6dc != 0.0)) {let t231: f64 = (p.p769 * l.fd35);let t232: f64 = (p.p768 + t231);let t233: f64 = (p.p770 * l.fdc8);let t234: f64 = (t232 + t233);let t235: f64 = (p.p771 * l.fc70);let t236: f64 = (t234 + t235);l.f163 = t236;l.f164 = 0.0;}
        let t237: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };l.f6de = t237;l.f6df = 0.0;
        if ((l.f976 != 0.0) && (l.f6de != 0.0)) {let t238: f64 = (p.p765 * l.fd35);let t239: f64 = (p.p764 + t238);let t23a: f64 = (p.p766 * l.fdc8);let t23b: f64 = (t239 + t23a);let t23c: f64 = (p.p767 * l.fc70);let t23d: f64 = (t23b + t23c);l.f15b = t23d;l.f15c = 0.0;}
        if (l.f976 != 0.0) {l.f15f4 = 0.0;l.f15f5 = 0.0;l.f15f6 = 0.0;l.f15f7 = 0.0;l.fe2f = 0.0;l.fe30 = 0.0;l.fe0b = p.p788;l.fe0c = 0.0;}
        let t23e: f64 = if param_given[789] { 1.0 } else { 0.0 };let t23f: f64 = if t23e == 1.0 { 1.0 } else { 0.0 };l.f6e0 = t23f;l.f6e1 = 0.0;
        if ((l.f976 != 0.0) && (l.f6e0 != 0.0)) {l.fe0b = p.p789;l.fe0c = 0.0;}
        let t240: f64 = if (((l.f130b > 0.0) && (l.f130d > 0.0)) && ((l.fedd == 1.0) || ((l.fedd > 1.0) && (l.f1317 > 0.0)))) { 1.0 } else { 0.0 };l.f6e2 = t240;l.f6e3 = 0.0;let mut t252: usize = 0;
        while {
            let t250: f64 = (l.fedd - 0.5);let t251: f64 = if (((l.f976 != 0.0) && (l.f6e2 != 0.0)) && (l.fe2f < t250)) { 1.0 } else { 0.0 };
            t251 != 0.0
        } {
            t252 += 1;assert!(t252 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t241: f64 = (0.5 * l.fe10);let t242: f64 = (l.f130b + t241);let t243: f64 = (l.f1317 + l.fe10);let t244: f64 = (l.fe2f * t243);let t245: f64 = (t242 + t244);let t246: f64 = (1.0 / t245);let t247: f64 = (l.f15f4 + t246);l.f15f4 = t247;l.f15f5 = 0.0;let t248: f64 = (0.5 * l.fe10);let t249: f64 = (l.f130d + t248);let t24a: f64 = (l.f1317 + l.fe10);let t24b: f64 = (l.fe2f * t24a);let t24c: f64 = (t249 + t24b);let t24d: f64 = (1.0 / t24c);let t24e: f64 = (l.f15f6 + t24d);l.f15f6 = t24e;l.f15f7 = 0.0;let t24f: f64 = (l.fe2f + 1.0);l.fe2f = t24f;l.fe30 = 0.0;}
        }
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t253: f64 = (l.f15f4 * l.fd8e);l.fd90 = t253;l.fd91 = 0.0;let t254: f64 = (l.f15f6 * l.fd8e);l.fd94 = t254;l.fd95 = 0.0;let t255: f64 = (0.5 * l.fe10);let t256: f64 = (p.p784 + t255);let t257: f64 = (1.0 / t256);l.fd92 = t257;l.fd93 = 0.0;let t258: f64 = (0.5 * l.fe10);let t259: f64 = (p.p785 + t258);let t25a: f64 = (1.0 / t259);l.fd96 = t25a;l.fd97 = 0.0;}
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {
            let t25b: f64 = (l.fe10 + l.f231);
            let (t25d,) = {
    if (t25b > 1e-9) {
        let t25c: f64 = (l.fe10 + l.f231);
        (t25c,)
    } else {
        (1e-9,)
    }
};
            l.fe4a = t25d;l.fe4b = 0.0;
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {
            let t25e: f64 = (l.f18bf + l.f2a3);let t25f: f64 = (t25e + p.p786);
            let (t262,) = {
    if (t25f > 1e-9) {
        let t260: f64 = (l.f18bf + l.f2a3);let t261: f64 = (t260 + p.p786);
        (t261,)
    } else {
        (1e-9,)
    }
};
            l.f191e = t262;l.f191f = 0.0;
        }
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t263: f64 = (l.fe4a).powf(p.p794);let t264: f64 = (1.0 / t263);l.f153a = t264;l.f153b = 0.0;let t265: f64 = (l.f191e).powf(p.p795);let t266: f64 = (1.0 / t265);l.f153c = t266;l.f153d = 0.0;let t267: f64 = (p.p791 * l.f153a);let t268: f64 = (1.0 + t267);let t269: f64 = (p.p792 * l.f153c);let t26a: f64 = (t268 + t269);let t26b: f64 = (p.p793 * l.f153a);let t26c: f64 = (t26b * l.f153c);let t26d: f64 = (t26a + t26c);let t26e: f64 = (l.f12d7 - 1.0);let t26f: f64 = (p.p790 * t26e);let t270: f64 = (1.0 + t26f);let t271: f64 = (t26d * t270);l.fdf8 = t271;l.fdf9 = 0.0;let t272: f64 = (l.fd90 + l.fd94);let t273: f64 = (p.p787 * t272);let t274: f64 = (t273 / l.fdf8);l.f12ab = t274;l.f12ac = 0.0;let t275: f64 = (l.fd92 + l.fd96);let t276: f64 = (p.p787 * t275);let t277: f64 = (t276 / l.fdf8);l.f12ad = t277;l.f12ae = 0.0;let t278: f64 = (l.fe4a).powf(p.p800);let t279: f64 = (1.0 / t278);l.f153a = t279;l.f153b = 0.0;let t27a: f64 = (l.f191e).powf(p.p801);let t27b: f64 = (1.0 / t27a);l.f153c = t27b;l.f153d = 0.0;let t27c: f64 = (p.p797 * l.f153a);let t27d: f64 = (1.0 + t27c);let t27e: f64 = (p.p798 * l.f153c);let t27f: f64 = (t27d + t27e);let t280: f64 = (p.p799 * l.f153a);let t281: f64 = (t280 * l.f153c);let t282: f64 = (t27f + t281);l.fdfa = t282;l.fdfb = 0.0;let t283: f64 = (l.fd90 + l.fd94);let t284: f64 = (t283 - l.fd92);let t285: f64 = (t284 - l.fd96);l.f151d = t285;l.f1520 = 0.0;let t286: f64 = (1.0 + l.f12ab);let t287: f64 = (1.0 + l.f12ad);let t288: f64 = (t286 / t287);l.f151e = t288;l.f151f = 0.0;let t289: f64 = (l.f103 * l.f151e);l.f103 = t289;l.f104 = 0.0;let t28a: f64 = (l.f1589 * l.f151e);let t28b: f64 = (p.p788 * l.f12ad);let t28c: f64 = (1.0 + t28b);let t28d: f64 = (t28a * t28c);let t28e: f64 = (p.p788 * l.f12ab);let t28f: f64 = (1.0 + t28e);let t290: f64 = (t28d / t28f);l.f1589 = t290;l.f158a = 0.0;let t291: f64 = (l.f158f * l.f151e);let t292: f64 = (l.fe0b * l.f12ad);let t293: f64 = (1.0 + t292);let t294: f64 = (t291 * t293);let t295: f64 = (l.fe0b * l.f12ab);let t296: f64 = (1.0 + t295);let t297: f64 = (t294 / t296);l.f158f = t297;l.f1590 = 0.0;let t298: f64 = (l.f109 * l.f151e);l.f109 = t298;l.f10a = 0.0;let t299: f64 = (p.p796 * l.f151d);let t29a: f64 = (t299 / l.fdfa);l.f151e = t29a;l.f151f = 0.0;let t29b: f64 = (l.f17ab + l.f151e);l.f17ab = t29b;l.f17ac = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f976 != 0.0) && (l.f6e2 != 0.0)) {let t29c: f64 = (l.f17b1 + l.f151e);l.f17b1 = t29c;l.f17b2 = 0.0;let t29d: f64 = (p.p802 * l.f151d);let t29e: f64 = (l.fdfa).powf(p.p803);let t29f: f64 = (t29d / t29e);l.f151e = t29f;l.f151f = 0.0;let t2a0: f64 = (l.f153 + l.f151e);l.f153 = t2a0;l.f154 = 0.0;let t2a1: f64 = (l.f167 + l.f151e);l.f167 = t2a1;l.f168 = 0.0;}
        let t2a2: f64 = if ((((l.f1311 > 0.0) || (l.f1313 > 0.0)) || (l.f1315 > 0.0)) || (l.f130f > 0.0)) { 1.0 } else { 0.0 };l.f6e4 = t2a2;l.f6ed = 0.0;let t2a3: f64 = if (((l.f1311 == 0.0) && (l.f1313 == 0.0)) && (l.f1315 == 0.0)) { 1.0 } else { 0.0 };l.f6ee = t2a3;l.f703 = 0.0;
        if (((l.f976 != 0.0) && (l.f6e4 != 0.0)) && (l.f6ee != 0.0)) {let t2a4: f64 = (l.f130f + l.f18bf);l.f151d = t2a4;l.f1520 = 0.0;let t2a5: f64 = (1.0 / p.p804);l.f151e = t2a5;l.f151f = 0.0;let t2a6: f64 = (p.p804 * p.p804);let t2a7: f64 = (l.f130f * l.f151d);let t2a8: f64 = (t2a6 / t2a7);l.f1311 = t2a8;l.f1312 = 0.0;let t2a9: f64 = (0.1 * l.f130f);let t2aa: f64 = (0.01 * p.p804);let t2ab: f64 = (t2a9 + t2aa);let t2ac: f64 = (-10.0);let t2ad: f64 = (t2ac * l.f130f);let t2ae: f64 = (t2ad * l.f151e);let t2af: f64 = (t2ae).exp();let t2b0: f64 = (t2ab * t2af);let t2b1: f64 = (0.1 * l.f151d);let t2b2: f64 = (0.01 * p.p804);let t2b3: f64 = (t2b1 + t2b2);let t2b4: f64 = (-10.0);let t2b5: f64 = (t2b4 * l.f151d);let t2b6: f64 = (t2b5 * l.f151e);let t2b7: f64 = (t2b6).exp();let t2b8: f64 = (t2b3 * t2b7);let t2b9: f64 = (t2b0 - t2b8);let t2ba: f64 = (t2b9 / l.f18bf);l.f1313 = t2ba;l.f1314 = 0.0;let t2bb: f64 = (0.05 * l.f130f);let t2bc: f64 = (0.0025 * p.p804);let t2bd: f64 = (t2bb + t2bc);let t2be: f64 = (-20.0);let t2bf: f64 = (t2be * l.f130f);let t2c0: f64 = (t2bf * l.f151e);let t2c1: f64 = (t2c0).exp();let t2c2: f64 = (t2bd * t2c1);let t2c3: f64 = (0.05 * l.f151d);let t2c4: f64 = (0.0025 * p.p804);let t2c5: f64 = (t2c3 + t2c4);let t2c6: f64 = (-20.0);let t2c7: f64 = (t2c6 * l.f151d);let t2c8: f64 = (t2c7 * l.f151e);let t2c9: f64 = (t2c8).exp();let t2ca: f64 = (t2c5 * t2c9);let t2cb: f64 = (t2c2 - t2ca);let t2cc: f64 = (t2cb / l.f18bf);l.f1315 = t2cc;l.f1316 = 0.0;}
        if ((l.f976 != 0.0) && (l.f6e4 != 0.0)) {let t2cd: f64 = (p.p805 * l.f1313);let t2ce: f64 = (l.f1311 + t2cd);let t2cf: f64 = (p.p806 * l.f1315);let t2d0: f64 = (t2ce + t2cf);l.f151d = t2d0;l.f1520 = 0.0;let t2d1: f64 = (l.fe0d * l.f151d);let t2d2: f64 = (l.f17ab + t2d1);l.f17ab = t2d2;l.f17ac = 0.0;let t2d3: f64 = (l.fe09 * l.f151d);let t2d4: f64 = (1.0 + t2d3);let t2d5: f64 = (l.f103 * t2d4);l.f103 = t2d5;l.f104 = 0.0;let t2d6: f64 = (l.fe0d * l.f151d);let t2d7: f64 = (l.f17b1 + t2d6);l.f17b1 = t2d7;l.f17b2 = 0.0;let t2d8: f64 = (l.fe09 * l.f151d);let t2d9: f64 = (1.0 + t2d8);let t2da: f64 = (l.f109 * t2d9);l.f109 = t2da;l.f10a = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        l: &mut StampLocals,
    ) {
        l.f17a9 = l.f17ab;l.f17aa = 0.0;l.f1500 = l.f1502;l.f1501 = 0.0;l.f14c2 = l.f14c4;l.f14c3 = 0.0;l.f1600 = l.f1602;l.f1601 = 0.0;l.f3b3 = l.f3b5;l.f3b4 = 0.0;
        let (t2dc,) = {
    if (l.fed5 > 1e20) {
        let (t2db,) = {
            if (l.fed5 < 1e26) {
                (l.fed5,)
            } else {
                (1e26,)
            }
        };
        (t2db,)
    } else {
        (1e20,)
    }
};
        l.fed3 = t2dc;l.fed4 = 0.0;
        let (t2dd,) = {
    if (l.f567 > 0.01) {
        (l.f567,)
    } else {
        (0.01,)
    }
};
        l.f565 = t2dd;l.f566 = 0.0;
        let (t2de,) = {
    if (l.f1868 > 0.0) {
        (l.f1868,)
    } else {
        (0.0,)
    }
};
        l.f1866 = t2de;l.f1867 = 0.0;l.f367 = l.f369;l.f368 = 0.0;l.f2da = l.f2dc;l.f2db = 0.0;
        let (t2df,) = {
    if (l.feeb > 0.0) {
        (l.feeb,)
    } else {
        (0.0,)
    }
};
        l.fee9 = t2df;l.feea = 0.0;l.f1606 = l.f1608;l.f1607 = 0.0;l.f160a = l.f160c;l.f160b = 0.0;
        let (t2e1,) = {
    if (l.fee2 > 1e23) {
        let (t2e0,) = {
            if (l.fee2 < 1e27) {
                (l.fee2,)
            } else {
                (1e27,)
            }
        };
        (t2e0,)
    } else {
        (1e23,)
    }
};
        l.fee0 = t2e1;l.fee1 = 0.0;
        let (t2e3,) = {
    if (l.fee6 > 1e23) {
        let (t2e2,) = {
            if (l.fee6 < 1e27) {
                (l.fee6,)
            } else {
                (1e27,)
            }
        };
        (t2e2,)
    } else {
        (1e23,)
    }
};
        l.fee4 = t2e3;l.fee5 = 0.0;
        let (t2e4,) = {
    if (l.f1e3 > 0.0) {
        (l.f1e3,)
    } else {
        (0.0,)
    }
};
        l.f1e1 = t2e4;l.f1e2 = 0.0;
        let (t2e6,) = {
    if (l.f1ef > 0.0) {
        let (t2e5,) = {
            if (l.f1ef < 0.5) {
                (l.f1ef,)
            } else {
                (0.5,)
            }
        };
        (t2e5,)
    } else {
        (0.0,)
    }
};
        l.f1ed = t2e6;l.f1ee = 0.0;
        let (t2e8,) = {
    if (l.f1f7 > 0.0) {
        let (t2e7,) = {
            if (l.f1f7 < 1.0) {
                (l.f1f7,)
            } else {
                (1.0,)
            }
        };
        (t2e7,)
    } else {
        (0.0,)
    }
};
        l.f1f5 = t2e8;l.f1f6 = 0.0;l.f14de = l.f14e0;l.f14df = 0.0;
        let (t2e9,) = {
    if (l.f153 > 0.0) {
        (l.f153,)
    } else {
        (0.0,)
    }
};
        l.f151 = t2e9;l.f152 = 0.0;
        let (t2eb,) = {
    if (l.f157 > 0.0) {
        let (t2ea,) = {
            if (l.f157 < 1.0) {
                (l.f157,)
            } else {
                (1.0,)
            }
        };
        (t2ea,)
    } else {
        (0.0,)
    }
};
        l.f155 = t2eb;l.f156 = 0.0;
        let (t2ec,) = {
    if (l.f15f > 0.0) {
        (l.f15f,)
    } else {
        (0.0,)
    }
};
        l.f15d = t2ec;l.f15e = 0.0;
        let (t2ed,) = {
    if (l.f1010 > 0.0) {
        (l.f1010,)
    } else {
        (0.0,)
    }
};
        l.f100e = t2ed;l.f100f = 0.0;
        let (t2ef,) = {
    if (l.f1014 > 0.0) {
        let (t2ee,) = {
            if (l.f1014 < 1.0) {
                (l.f1014,)
            } else {
                (1.0,)
            }
        };
        (t2ee,)
    } else {
        (0.0,)
    }
};
        l.f1012 = t2ef;l.f1013 = 0.0;
        let (t2f0,) = {
    if (l.f101c > 0.0) {
        (l.f101c,)
    } else {
        (0.0,)
    }
};
        l.f101a = t2f0;l.f101b = 0.0;
        let (t2f1,) = {
    if (l.f103 > 0.0) {
        (l.f103,)
    } else {
        (0.0,)
    }
};
        l.f101 = t2f1;l.f102 = 0.0;l.f14ca = l.f14cc;l.f14cb = 0.0;
        let (t2f2,) = {
    if (l.febb > 0.0) {
        (l.febb,)
    } else {
        (0.0,)
    }
};
        l.feb9 = t2f2;l.feba = 0.0;l.f14ec = l.f14ee;l.f14ed = 0.0;
        let (t2f3,) = {
    if (l.f1564 > 0.0) {
        (l.f1564,)
    } else {
        (0.0,)
    }
};
        l.f1562 = t2f3;l.f1563 = 0.0;l.f14f8 = l.f14fa;l.f14f9 = 0.0;
        let (t2f4,) = {
    if (l.f1cb > 0.0) {
        (l.f1cb,)
    } else {
        (0.0,)
    }
};
        l.f1c9 = t2f4;l.f1ca = 0.0;l.f14da = l.f14dc;l.f14db = 0.0;
        let (t2f5,) = {
    if (l.f155e > 0.0) {
        (l.f155e,)
    } else {
        (0.0,)
    }
};
        l.f155c = t2f5;l.f155d = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        l: &mut StampLocals,
    ) {
        l.f14f4 = l.f14f6;l.f14f5 = 0.0;
        let (t2f6,) = {
    if (l.f19bc > 0.0) {
        (l.f19bc,)
    } else {
        (0.0,)
    }
};
        l.f19ba = t2f6;l.f19bb = 0.0;l.f1508 = l.f150a;l.f1509 = 0.0;l.f474 = l.f476;l.f475 = 0.0;
        let (t2f7,) = {
    if (l.f12c7 > 0.0) {
        (l.f12c7,)
    } else {
        (0.0,)
    }
};
        l.f12c5 = t2f7;l.f12c6 = 0.0;l.f14f0 = l.f14f2;l.f14f1 = 0.0;let t2f8: f64 = (-0.5);
        let (t2fb,) = {
    if (l.f12cd > t2f8) {
        let (t2f9,) = {
            if (l.f12cd < 1.0) {
                (l.f12cd,)
            } else {
                (1.0,)
            }
        };
        (t2f9,)
    } else {
        let t2fa: f64 = (-0.5);
        (t2fa,)
    }
};
        l.f12cb = t2fb;l.f12cc = 0.0;let t2fc: f64 = (-0.5);
        let (t2fe,) = {
    if (l.f12d3 > t2fc) {
        (l.f12d3,)
    } else {
        let t2fd: f64 = (-0.5);
        (t2fd,)
    }
};
        l.f12d1 = t2fe;l.f12d2 = 0.0;
        let (t2ff,) = {
    if (l.f1589 > 0.0) {
        (l.f1589,)
    } else {
        (0.0,)
    }
};
        l.f1587 = t2ff;l.f1588 = 0.0;l.f14fc = l.f14fe;l.f14fd = 0.0;let t300: f64 = (-0.5);
        let (t303,) = {
    if (l.f159f > t300) {
        let (t301,) = {
            if (l.f159f < 1.0) {
                (l.f159f,)
            } else {
                (1.0,)
            }
        };
        (t301,)
    } else {
        let t302: f64 = (-0.5);
        (t302,)
    }
};
        l.f159d = t303;l.f159e = 0.0;let t0: f64 = (-0.5);
        let (t2,) = {
    if (l.f15bb > t0) {
        (l.f15bb,)
    } else {
        let t1: f64 = (-0.5);
        (t1,)
    }
};
        l.f15b9 = t2;l.f15ba = 0.0;
        let (t3,) = {
    if (l.f15c3 > 0.01) {
        (l.f15c3,)
    } else {
        (0.01,)
    }
};
        l.f15c1 = t3;l.f15c2 = 0.0;
        let (t4,) = {
    if (l.fe4 > 2.0) {
        (l.fe4,)
    } else {
        (2.0,)
    }
};
        l.fe2 = t4;l.fe3 = 0.0;
        let (t5,) = {
    if (l.f41 > 0.0) {
        (l.f41,)
    } else {
        (0.0,)
    }
};
        l.f3f = t5;l.f40 = 0.0;
        let (t6,) = {
    if (l.f35 > 0.0) {
        (l.f35,)
    } else {
        (0.0,)
    }
};
        l.f33 = t6;l.f34 = 0.0;
        let (t7,) = {
    if (l.f3d > 0.0) {
        (l.f3d,)
    } else {
        (0.0,)
    }
};
        l.f3b = t7;l.f3c = 0.0;l.f1862 = l.f1864;l.f1863 = 0.0;
        let (t8,) = {
    if (l.f2 > 0.0) {
        (l.f2,)
    } else {
        (0.0,)
    }
};
        l.f0 = t8;l.f1 = 0.0;l.f4 = l.f6;l.f5 = 0.0;l.f14c6 = l.f14c8;l.f14c7 = 0.0;
        let (t9,) = {
    if (l.fc > 0.0) {
        (l.fc,)
    } else {
        (0.0,)
    }
};
        l.fa = t9;l.fb = 0.0;
        let (ta,) = {
    if (l.f10 > 0.0) {
        (l.f10,)
    } else {
        (0.0,)
    }
};
        l.fe = ta;l.ff = 0.0;
        let (tb,) = {
    if (l.fd3b > 1e-12) {
        (l.fd3b,)
    } else {
        (1e-12,)
    }
};
        l.fd39 = tb;l.fd3a = 0.0;l.f51f = l.f521;l.f520 = 0.0;
        let (tc,) = {
    if (l.fcb8 > 0.0) {
        (l.fcb8,)
    } else {
        (0.0,)
    }
};
        l.fcb6 = tc;l.fcb7 = 0.0;
        let (td,) = {
    if (l.fcbc > 0.0) {
        (l.fcbc,)
    } else {
        (0.0,)
    }
};
        l.fcba = td;l.fcbb = 0.0;
        let (te,) = {
    if (l.fcc0 > 0.0) {
        (l.fcc0,)
    } else {
        (0.0,)
    }
};
        l.fcbe = te;l.fcbf = 0.0;l.f14e8 = l.f14ea;l.f14e9 = 0.0;l.f507 = l.f509;l.f508 = 0.0;l.f513 = l.f515;l.f514 = 0.0;l.f50b = l.f50d;l.f50c = 0.0;l.f517 = l.f519;l.f518 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        l: &mut StampLocals,
    ) {
        l.f50f = l.f511;l.f510 = 0.0;l.f51b = l.f51d;l.f51c = 0.0;l.f18f = l.f191;l.f190 = 0.0;
        let (tf,) = {
    if (l.f29 > 0.0) {
        (l.f29,)
    } else {
        (0.0,)
    }
};
        l.f27 = tf;l.f28 = 0.0;
        let (t10,) = {
    if (l.f2d > 0.0) {
        (l.f2d,)
    } else {
        (0.0,)
    }
};
        l.f2b = t10;l.f2c = 0.0;l.f112 = l.f114;l.f113 = 0.0;l.f118 = l.f11a;l.f119 = 0.0;l.f14d2 = l.f14d4;l.f14d3 = 0.0;l.f14d6 = l.f14d8;l.f14d7 = 0.0;l.f17b = l.f17d;l.f17c = 0.0;l.f17f = l.f181;l.f180 = 0.0;
        let (t11,) = {
    if (l.f1bb > 0.0) {
        (l.f1bb,)
    } else {
        (0.0,)
    }
};
        l.f1b7 = t11;l.f1b8 = 0.0;l.f29b = l.f29d;l.f29c = 0.0;
        let (t12,) = {
    if (l.f422 > 0.0) {
        (l.f422,)
    } else {
        (0.0,)
    }
};
        l.f420 = t12;l.f421 = 0.0;
        let (t13,) = {
    if (l.f158f > 0.0) {
        (l.f158f,)
    } else {
        (0.0,)
    }
};
        l.f158d = t13;l.f158e = 0.0;
        let (t14,) = {
    if (l.fe8 > 2.0) {
        (l.fe8,)
    } else {
        (2.0,)
    }
};
        l.fe6 = t14;l.fe7 = 0.0;l.f43 = l.f45;l.f44 = 0.0;
        let (t15,) = {
    if (l.f39 > 0.0) {
        (l.f39,)
    } else {
        (0.0,)
    }
};
        l.f37 = t15;l.f38 = 0.0;
        let (t16,) = {
    if (l.f185 > 0.0) {
        (l.f185,)
    } else {
        (0.0,)
    }
};
        l.f183 = t16;l.f184 = 0.0;
        let (t17,) = {
    if (l.f18d > 0.0) {
        (l.f18d,)
    } else {
        (0.0,)
    }
};
        l.f18b = t17;l.f18c = 0.0;l.f462 = l.f464;l.f463 = 0.0;l.f466 = l.f468;l.f467 = 0.0;l.f187 = l.f189;l.f188 = 0.0;
        let (t18,) = {
    if (l.f173 > 0.0) {
        (l.f173,)
    } else {
        (0.0,)
    }
};
        l.f171 = t18;l.f172 = 0.0;
        let (t19,) = {
    if (l.f197 > 0.0) {
        (l.f197,)
    } else {
        (0.0,)
    }
};
        l.f195 = t19;l.f196 = 0.0;
        let (t1a,) = {
    if (l.f19b > 0.0) {
        (l.f19b,)
    } else {
        (0.0,)
    }
};
        l.f199 = t1a;l.f19a = 0.0;l.f351 = l.f353;l.f352 = 0.0;l.f46e = l.f470;l.f46f = 0.0;l.f46a = l.f46c;l.f46b = 0.0;l.fee = l.ff0;l.fef = 0.0;
        let (t1b,) = {
    if (l.f16b > 0.0) {
        (l.f16b,)
    } else {
        (0.0,)
    }
};
        l.f169 = t1b;l.f16a = 0.0;
        let (t1c,) = {
    if (l.f16f > 0.0) {
        (l.f16f,)
    } else {
        (0.0,)
    }
};
        l.f16d = t1c;l.f16e = 0.0;l.f4a5 = l.f4a7;l.f4a6 = 0.0;l.f17af = l.f17b1;l.f17b0 = 0.0;l.f1504 = l.f1506;l.f1505 = 0.0;l.f2de = l.f2e0;l.f2df = 0.0;
        let (t1e,) = {
    if (l.fedb > 1e20) {
        let (t1d,) = {
            if (l.fedb < 1e26) {
                (l.fedb,)
            } else {
                (1e26,)
            }
        };
        (t1d,)
    } else {
        (1e20,)
    }
};
        l.fed9 = t1e;l.feda = 0.0;
        let (t1f,) = {
    if (l.f1f3 > 0.0) {
        (l.f1f3,)
    } else {
        (0.0,)
    }
};
        l.f1f1 = t1f;l.f1f2 = 0.0;
        let (t20,) = {
    if (l.f109 > 0.0) {
        (l.f109,)
    } else {
        (0.0,)
    }
};
        l.f107 = t20;l.f108 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        l.f14ce = l.f14d0;l.f14cf = 0.0;
        let (t21,) = {
    if (l.f1024 > 0.0) {
        (l.f1024,)
    } else {
        (0.0,)
    }
};
        l.f1022 = t21;l.f1023 = 0.0;
        let (t23,) = {
    if (l.f1018 > 0.0) {
        let (t22,) = {
            if (l.f1018 < 1.0) {
                (l.f1018,)
            } else {
                (1.0,)
            }
        };
        (t22,)
    } else {
        (0.0,)
    }
};
        l.f1016 = t23;l.f1017 = 0.0;
        let (t24,) = {
    if (l.f1020 > 0.0) {
        (l.f1020,)
    } else {
        (0.0,)
    }
};
        l.f101e = t24;l.f101f = 0.0;
        let (t25,) = {
    if (l.f167 > 0.0) {
        (l.f167,)
    } else {
        (0.0,)
    }
};
        l.f165 = t25;l.f166 = 0.0;
        let (t27,) = {
    if (l.f15b > 0.0) {
        let (t26,) = {
            if (l.f15b < 1.0) {
                (l.f15b,)
            } else {
                (1.0,)
            }
        };
        (t26,)
    } else {
        (0.0,)
    }
};
        l.f159 = t27;l.f15a = 0.0;
        let (t28,) = {
    if (l.f163 > 0.0) {
        (l.f163,)
    } else {
        (0.0,)
    }
};
        l.f161 = t28;l.f162 = 0.0;let t29: f64 = (p.p31 * l.fedd);
        let (t2b,) = {
    if (t29 > 0.0) {
        let t2a: f64 = (p.p31 * l.fedd);
        (t2a,)
    } else {
        (0.0,)
    }
};
        l.febf = t2b;l.fec0 = 0.0;l.f436 = p.p16;l.f437 = 0.0;l.f29f = p.p15;l.f2a0 = 0.0;l.f438 = p.p18;l.f439 = 0.0;l.f2a1 = p.p17;l.f2a2 = 0.0;let t2c: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.f704 = t2c;l.f719 = 0.0;
        if (l.f704 != 0.0) {l.f160a = l.f1606;l.f160b = 0.0;l.fee4 = l.fee0;l.fee5 = 0.0;l.f2b = l.f27;l.f2c = 0.0;l.f118 = l.f112;l.f119 = 0.0;l.f14d6 = l.f14d2;l.f14d7 = 0.0;l.f17f = l.f17b;l.f180 = 0.0;l.fcbe = l.fcba;l.fcbf = 0.0;l.f50f = l.f50b;l.f510 = 0.0;l.f51b = l.f517;l.f51c = 0.0;l.f18b = l.f183;l.f18c = 0.0;l.f466 = l.f462;l.f467 = 0.0;l.f199 = l.f195;l.f19a = 0.0;l.f16d = l.f169;l.f16e = 0.0;}
        let t2d: f64 = (8.8541878176e-12 * l.f3b3);l.f3b1 = t2d;l.f3b2 = 0.0;let t2e: f64 = (l.f3b1 / l.f1600);l.f1c7 = t2e;l.f1c8 = 0.0;let t2f: f64 = (l.f1600 * l.f1600);l.f1604 = t2f;l.f1605 = 0.0;let t30: f64 = (l.f1c7 / 1.6021918e-19);l.f1b9 = t30;l.f1ba = 0.0;let t31: f64 = (l.f420 * l.fed3);l.fed7 = t31;l.fed8 = 0.0;
        let (t33,) = {
    if (l.fed7 > 1e20) {
        let (t32,) = {
            if (l.fed7 < 1e26) {
                (l.fed7,)
            } else {
                (1e26,)
            }
        };
        (t32,)
    } else {
        (1e20,)
    }
};
        l.fed7 = t33;l.fed8 = 0.0;l.f127a = 0.0;l.f127b = 0.0;let t34: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f71a = t34;l.f72f = 0.0;
        if (l.f71a != 0.0) {let t35: f64 = (0.4 * 5.951993);let t36: f64 = (t35 * p.p51);let t37: f64 = (l.f1c7).powf(0.6666666666666666);let t38: f64 = (t36 * t37);l.f127a = t38;l.f127b = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        l: &mut StampLocals,
    ) {
        let t39: f64 = (-1.0);let t3a: f64 = if l.f193 == t39 { 1.0 } else { 0.0 };l.f730 = t3a;l.f745 = 0.0;
        if ((l.f71a != 0.0) && (l.f730 != 0.0)) {let t3b: f64 = (7.448711 / 5.951993);let t3c: f64 = (t3b * l.f127a);l.f127a = t3c;l.f127b = 0.0;}
        let t3d: f64 = (1e-8 * l.f1c7);let t3e: f64 = (t3d / l.f3b7);l.f37d = t3e;l.f37e = 0.0;let t3f: f64 = (0.5 * l.f474);l.f3e3 = t3f;l.f3e6 = 0.0;l.f3e4 = 0.5;l.f3e5 = 0.0;let t40: f64 = (-1.0);let t41: f64 = if l.f193 == t40 { 1.0 } else { 0.0 };l.f746 = t41;l.f75b = 0.0;
        if (l.f746 != 0.0) {let t42: f64 = (0.3333333333333333 * l.f474);l.f3e3 = t42;l.f3e6 = 0.0;l.f3e4 = 0.3333333333333333;l.f3e5 = 0.0;}
        let t43: f64 = (-2.0);let t44: f64 = (t43 / l.fe2);let t45: f64 = (t44 + 1.0);let t46: f64 = (2.0_f64).powf(t45);let t47: f64 = (t46 - 1.0);l.f151c = t47;l.f1539 = 0.0;let t48: f64 = (l.f151c - 1.0);let t49: f64 = (l.f151c - 1.0);let t4a: f64 = (t48 * t49);let t4b: f64 = (4.0 * l.f151c);
        let (t4d,) = {
    if (t4b > 0.0001) {
        let t4c: f64 = (4.0 * l.f151c);
        (t4c,)
    } else {
        (0.0001,)
    }
};
        let t4e: f64 = (t4a / t4d);l.fa4 = t4e;l.fa5 = 0.0;let t4f: f64 = (-2.0);let t50: f64 = (t4f / l.fe6);let t51: f64 = (t50 + 1.0);let t52: f64 = (2.0_f64).powf(t51);let t53: f64 = (t52 - 1.0);l.f151c = t53;l.f1539 = 0.0;let t54: f64 = (l.f151c - 1.0);let t55: f64 = (l.f151c - 1.0);let t56: f64 = (t54 * t55);let t57: f64 = (4.0 * l.f151c);
        let (t59,) = {
    if (t57 > 0.0001) {
        let t58: f64 = (4.0 * l.f151c);
        (t58,)
    } else {
        (0.0001,)
    }
};
        let t5a: f64 = (t56 / t59);l.fa6 = t5a;l.fa7 = 0.0;let t5b: f64 = (1.0 / l.f1862);l.fd75 = t5b;l.fd76 = 0.0;let t5c: f64 = (l.f3b1 / l.f1606);l.f1c3 = t5c;l.f1c6 = 0.0;let t5d: f64 = (l.f3b1 / l.f160a);l.f1c4 = t5d;l.f1c5 = 0.0;let t5e: f64 = (2.0 * 1.6021918e-19);let t5f: f64 = (t5e * l.fee0);let t60: f64 = (t5f * l.f3b7);let t61: f64 = (t60 * l.fd73);let t62: f64 = (t61).sqrt();let t63: f64 = (t62 / l.f1c3);l.f5c5 = t63;l.f5c6 = 0.0;let t64: f64 = (2.0 * 1.6021918e-19);let t65: f64 = (t64 * l.fee4);let t66: f64 = (t65 * l.f3b7);let t67: f64 = (t66 * l.fd73);let t68: f64 = (t67).sqrt();let t69: f64 = (t68 / l.f1c4);l.f5c3 = t69;l.f5c4 = 0.0;let t6a: f64 = (l.f5c5 * l.f5c5);l.f5c1 = t6a;l.f5c2 = 0.0;let t6b: f64 = (l.f5c3 * l.f5c3);l.f5bf = t6b;l.f5c0 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        l: &mut StampLocals,
    ) {
        let t6c: f64 = (l.f187 * 0.005);let t6d: f64 = (t6c * l.fd73);let t6e: f64 = (t6d).exp();let t6f: f64 = (t6e - 1.0);let t70: f64 = (t6f).ln();let t71: f64 = (t70 / l.f187);let t72: f64 = (0.005 * l.fd73);let t73: f64 = (t72).exp();let t74: f64 = (t73 - 1.0);let t75: f64 = (t74).ln();let t76: f64 = (t71 - t75);l.f375 = t76;l.f376 = 0.0;let t77: f64 = (0.5 * l.f5c5);let t78: f64 = (t77).ln();let t79: f64 = (t78 + l.f375);l.f373 = t79;l.f374 = 0.0;let t7a: f64 = (0.5 * l.f5c3);let t7b: f64 = (t7a).ln();let t7c: f64 = (t7b + l.f375);l.f371 = t7c;l.f372 = 0.0;let t7d: f64 = (1.0 / l.f5c5);l.fd57 = t7d;l.fd58 = 0.0;let t7e: f64 = (3.1 * l.f5c5);let t7f: f64 = (t7e + 8.5);l.f133d = t7f;l.f1342 = 0.0;let t80: f64 = (l.f133d * l.f133d);l.f1340 = t80;l.f1341 = 0.0;let t81: f64 = (0.5 * l.f133d);l.f1337 = t81;l.f133c = 0.0;let t82: f64 = if l.fd57 < 0.06 { 1.0 } else { 0.0 };l.f75c = t82;l.f771 = 0.0;
        if (l.f75c != 0.0) {let t83: f64 = (64.0 * l.fd57);l.f1335 = t83;l.f1336 = 0.0;}
        let t84: f64 = if l.fd57 <= 0.45 { 1.0 } else { 0.0 };l.f772 = t84;l.f77d = 0.0;
        if ((l.f75c == 0.0) && (l.f772 != 0.0)) {let t85: f64 = (22.0 * l.fd57);let t86: f64 = (t85 + 3.0);l.f1335 = t86;l.f1336 = 0.0;}
        let t87: f64 = if l.fd57 <= 1.6 { 1.0 } else { 0.0 };l.f77e = t87;l.f77f = 0.0;
        if (((l.f75c == 0.0) && (l.f772 == 0.0)) && (l.f77e != 0.0)) {let t88: f64 = (-7.2);let t89: f64 = (t88 * l.fd57);let t8a: f64 = (t89 + 15.5);l.f1335 = t8a;l.f1336 = 0.0;}
        if (((l.f75c == 0.0) && (l.f772 == 0.0)) && (l.f77e == 0.0)) {l.f1335 = l.f5c5;l.f1336 = 0.0;}
        let t8b: f64 = (l.f5c1 * 0.5);let t8c: f64 = (l.f1337 + t8b);let t8d: f64 = (l.f5c1 * 0.25);let t8e: f64 = (l.f1337 + t8d);let t8f: f64 = (t8e + l.f1335);let t90: f64 = (t8f).sqrt();let t91: f64 = (l.f5c5 * t90);let t92: f64 = (t8c - t91);l.f133a = t92;l.f133b = 0.0;let t93: f64 = (1.0 / l.f5c3);l.fd57 = t93;l.fd58 = 0.0;let t94: f64 = (3.1 * l.f5c3);let t95: f64 = (t94 + 8.5);l.f133d = t95;l.f1342 = 0.0;let t96: f64 = (l.f133d * l.f133d);l.f133e = t96;l.f133f = 0.0;let t97: f64 = (0.5 * l.f133d);l.f1337 = t97;l.f133c = 0.0;let t98: f64 = if l.fd57 < 0.06 { 1.0 } else { 0.0 };l.f780 = t98;l.f781 = 0.0;
        if (l.f780 != 0.0) {let t99: f64 = (64.0 * l.fd57);l.f1333 = t99;l.f1334 = 0.0;}
        let t9a: f64 = if l.fd57 <= 0.45 { 1.0 } else { 0.0 };l.f782 = t9a;l.f783 = 0.0;
        if ((l.f780 == 0.0) && (l.f782 != 0.0)) {let t9b: f64 = (22.0 * l.fd57);let t9c: f64 = (t9b + 3.0);l.f1333 = t9c;l.f1334 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t9d: f64 = if l.fd57 <= 1.6 { 1.0 } else { 0.0 };l.f784 = t9d;l.f785 = 0.0;
        if (((l.f780 == 0.0) && (l.f782 == 0.0)) && (l.f784 != 0.0)) {let t9e: f64 = (-7.2);let t9f: f64 = (t9e * l.fd57);let ta0: f64 = (t9f + 15.5);l.f1333 = ta0;l.f1334 = 0.0;}
        if (((l.f780 == 0.0) && (l.f782 == 0.0)) && (l.f784 == 0.0)) {l.f1333 = l.f5c3;l.f1334 = 0.0;}
        let ta1: f64 = (l.f5bf * 0.5);let ta2: f64 = (l.f1337 + ta1);let ta3: f64 = (l.f5bf * 0.25);let ta4: f64 = (l.f1337 + ta3);let ta5: f64 = (ta4 + l.f1333);let ta6: f64 = (ta5).sqrt();let ta7: f64 = (l.f5c3 * ta6);let ta8: f64 = (ta2 - ta7);l.f1338 = ta8;l.f1339 = 0.0;let ta9: f64 = (l.f3a3 + l.f2da);let taa: f64 = (2.0 * l.ff9e);let tab: f64 = (-0.75);let tac: f64 = (l.ff72).powf(tab);let tad: f64 = (l.fed3 * tac);let tae: f64 = (tad * 4e-26);let taf: f64 = (tae).ln();let tb0: f64 = (taa * taf);let tb1: f64 = (ta9 + tb0);l.ff6d = tb1;l.ff6e = 0.0;
        if (!(l.ff6d > 0.05)) {l.ff6d = 0.05;l.ff6e = 0.0;}
        let tb2: f64 = (2.0 * 1.6021918e-19);let tb3: f64 = (tb2 * l.fed3);let tb4: f64 = (tb3 * l.f3b7);let tb5: f64 = (tb4 * l.fd59);let tb6: f64 = (tb5).sqrt();let tb7: f64 = (tb6 / l.f1c7);l.f4f2 = tb7;l.f4f3 = 0.0;l.fdf6 = 0.0;l.fdf7 = 0.0;l.fee8 = 0.0;l.feed = 0.0;let tb8: f64 = if l.fee9 > 0.0 { 1.0 } else { 0.0 };l.f786 = tb8;l.f791 = 0.0;
        if (l.f786 != 0.0) {let tb9: f64 = (80000000.0 / l.f1604);l.fae = tb9;l.faf = 0.0;}
        if (l.f786 != 0.0) {
            let (tba,) = {
    if (l.fee9 > l.fae) {
        (l.fee9,)
    } else {
        (l.fae,)
    }
};
            l.fee8 = tba;l.feed = 0.0;
        }
        if (l.f786 != 0.0) {
            let (tbb,) = {
    if (5e24 > l.fee8) {
        (5e24,)
    } else {
        (l.fee8,)
    }
};
            l.fee8 = tbb;l.feed = 0.0;
        }
        if (l.f786 != 0.0) {let tbc: f64 = (2.0 * l.f1c7);let tbd: f64 = (tbc * l.f1c7);let tbe: f64 = (tbd * l.ff9e);let tbf: f64 = (1.6021918e-19 * l.fee8);let tc0: f64 = (tbf * l.f3b7);let tc1: f64 = (tbe / tc0);l.fdf6 = tc1;l.fdf7 = 0.0;}
        let tc2: f64 = (100.0 * l.ff9e);let tc3: f64 = (tc2 * l.ff9e);l.f1252 = tc3;l.f1253 = 0.0;let tc4: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f792 = tc4;l.f7a7 = 0.0;
        if (l.f792 != 0.0) {let tc5: f64 = (l.ff9e * l.f4f2);let tc6: f64 = (tc5 * l.f4f2);let tc7: f64 = (tc6 * l.ff6d);let tc8: f64 = (tc7).sqrt();l.f1099 = tc8;l.f109a = 0.0;let tc9: f64 = (0.75 * l.f127a);let tca: f64 = (l.f1099).powf(0.6666666666666666);let tcb: f64 = (tc9 * tca);l.f2e2 = tcb;l.f2e3 = 0.0;let tcc: f64 = (l.ff6d + l.f2e2);l.ff6d = tcc;l.ff6e = 0.0;let tcd: f64 = (2.0 * 0.6666666666666666);let tce: f64 = (tcd * l.f2e2);let tcf: f64 = (tce / l.f1099);let td0: f64 = (1.0 + tcf);let td1: f64 = (l.f4f2 * td0);l.f4f2 = td1;l.f4f3 = 0.0;}
        let td2: f64 = (l.ff6d).sqrt();l.f149c = td2;l.f149d = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let td3: f64 = (0.95 * l.ff6d);l.ffe2 = td3;l.ffe3 = 0.0;let td4: f64 = (0.0025 * l.ff6d);let td5: f64 = (td4 * l.ff6d);l.f9f = td5;l.fa0 = 0.0;l.f128 = l.f9f;l.f129 = 0.0;let td6: f64 = (l.f128).sqrt();let td7: f64 = (0.5 * td6);l.ffdc = td7;l.ffdd = 0.0;let td8: f64 = (l.ffe2 - l.ffdc);let td9: f64 = td8;let tda: f64 = (l.ffe2 - l.ffdc);let tdb: f64 = tda;let tdc: f64 = (l.ffe2 - l.ffdc);let tdd: f64 = tdc;let tde: f64 = (tdb * tdd);let tdf: f64 = (tde + l.f9f);let te0: f64 = (tdf).sqrt();let te1: f64 = (td9 - te0);let te2: f64 = (0.5 * te1);l.ffd8 = te2;l.ffd9 = 0.0;let te3: f64 = (l.ff6d + l.f3a3);let te4: f64 = (0.5 * te3);l.f60 = te4;l.f61 = 0.0;let te5: f64 = (l.f1866 + l.ff6d);let te6: f64 = (te5).sqrt();let te7: f64 = (te6 - l.f149c);l.f1693 = te7;l.f1694 = 0.0;let te8: f64 = (l.f1866 + l.f367);let te9: f64 = (te8 + l.ff6d);let tea: f64 = (te9).sqrt();let teb: f64 = (tea - l.f149c);let tec: f64 = (teb - l.f1693);l.f1695 = tec;l.f1696 = 0.0;let ted: f64 = (l.f3a3 + l.f2da);let tee: f64 = (ted + l.f29b);let tef: f64 = (2.0 * l.ff9e);let tf0: f64 = (-0.75);let tf1: f64 = (l.ff72).powf(tf0);let tf2: f64 = (l.fed7 * tf1);let tf3: f64 = (tf2 * 4e-26);let tf4: f64 = (tf3).ln();let tf5: f64 = (tef * tf4);let tf6: f64 = (tee + tf5);l.ff6b = tf6;l.ff6c = 0.0;
        if (!(l.ff6b > 0.05)) {l.ff6b = 0.05;l.ff6c = 0.0;}
        let tf7: f64 = (2.0 * 1.6021918e-19);let tf8: f64 = (tf7 * l.fed7);let tf9: f64 = (tf8 * l.f3b7);let tfa: f64 = (tf9 * l.fd59);let tfb: f64 = (tfa).sqrt();let tfc: f64 = (tfb / l.f1c7);l.f4f0 = tfc;l.f4f1 = 0.0;let tfd: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f7a8 = tfd;l.f7bc = 0.0;
        if (l.f7a8 != 0.0) {let tfe: f64 = (l.ff9e * l.f4f0);let tff: f64 = (tfe * l.f4f0);let t100: f64 = (tff * l.ff6b);let t101: f64 = (t100).sqrt();l.f1099 = t101;l.f109a = 0.0;let t102: f64 = (0.75 * l.f127a);let t103: f64 = (l.f1099).powf(0.6666666666666666);let t104: f64 = (t102 * t103);l.f2e2 = t104;l.f2e3 = 0.0;let t105: f64 = (l.ff6b + l.f2e2);l.ff6b = t105;l.ff6c = 0.0;let t106: f64 = (2.0 * 0.6666666666666666);let t107: f64 = (t106 * l.f2e2);let t108: f64 = (t107 / l.f1099);let t109: f64 = (1.0 + t108);let t10a: f64 = (l.f4f0 * t109);l.f4f0 = t10a;l.f4f1 = 0.0;}
        let t10b: f64 = (0.95 * l.ff6b);l.ffe0 = t10b;l.ffe1 = 0.0;let t10c: f64 = (0.0025 * l.ff6b);let t10d: f64 = (t10c * l.ff6b);l.f9d = t10d;l.f9e = 0.0;
    }
}
