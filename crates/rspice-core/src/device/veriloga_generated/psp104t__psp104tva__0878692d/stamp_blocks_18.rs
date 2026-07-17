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
        if ((l.fa1e != 0.0) && (l.fc6b != 0.0)) {let tf6: f64 = (p.p629 * l.fdf1);let tf7: f64 = (p.p628 + tf6);let tf8: f64 = (p.p630 * l.fe91);let tf9: f64 = (tf7 + tf8);let tfa: f64 = (p.p631 * l.fd25);let tfb: f64 = (tf9 + tfa);let tfc: f64 = (l.fd82 * tfb);l.fd72 = tfc;l.fd73 = 0.0;}
        let tfd: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };l.fc6d = tfd;l.fc6e = 0.0;
        if ((l.fa1e != 0.0) && (l.fc6d != 0.0)) {let tfe: f64 = (p.p633 * l.fdf1);let tff: f64 = (p.p632 + tfe);let t100: f64 = (p.p634 * l.fe91);let t101: f64 = (tff + t100);let t102: f64 = (p.p635 * l.fd25);let t103: f64 = (t101 + t102);let t104: f64 = (l.fd8f * t103);l.fd76 = t104;l.fd77 = 0.0;}
        let t105: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };l.fc6f = t105;l.fc70 = 0.0;
        if ((l.fa1e != 0.0) && (l.fc6f != 0.0)) {let t106: f64 = (p.p637 * l.fdf1);let t107: f64 = (p.p636 + t106);let t108: f64 = (p.p638 * l.fe91);let t109: f64 = (t107 + t108);let t10a: f64 = (p.p639 * l.fd25);let t10b: f64 = (t109 + t10a);let t10c: f64 = (l.fd8f * t10b);l.fd7a = t10c;l.fd7b = 0.0;}
        let t10d: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };l.fc71 = t10d;l.fc72 = 0.0;
        if ((l.fa1e != 0.0) && (l.fc71 != 0.0)) {let t10e: f64 = (p.p641 * l.fdf1);let t10f: f64 = (p.p640 + t10e);let t110: f64 = (p.p642 * l.fe91);let t111: f64 = (t10f + t110);let t112: f64 = (p.p643 * l.fd25);let t113: f64 = (t111 + t112);l.f16a6 = t113;l.f16a7 = 0.0;}
        let t114: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };l.fc73 = t114;l.fc74 = 0.0;
        if ((l.fa1e != 0.0) && (l.fc73 != 0.0)) {let t115: f64 = (p.p645 * l.fdf1);let t116: f64 = (p.p644 + t115);let t117: f64 = (p.p646 * l.fe91);let t118: f64 = (t116 + t117);let t119: f64 = (p.p647 * l.fd25);let t11a: f64 = (t118 + t119);let t11b: f64 = (l.fd8f * t11a);l.f2b = t11b;l.f2c = 0.0;}
        let t11c: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };l.fc75 = t11c;l.fc76 = 0.0;
        if ((l.fa1e != 0.0) && (l.fc75 != 0.0)) {let t11d: f64 = (p.p649 * l.fdf1);let t11e: f64 = (p.p648 + t11d);let t11f: f64 = (p.p650 * l.fe91);let t120: f64 = (t11e + t11f);let t121: f64 = (p.p651 * l.fd25);let t122: f64 = (t120 + t121);let t123: f64 = (l.fd8f * t122);l.f2f = t123;l.f30 = 0.0;}
        let t124: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };l.fc77 = t124;l.fc78 = 0.0;
        if ((l.fa1e != 0.0) && (l.fc77 != 0.0)) {let t125: f64 = (p.p653 * l.fdf1);let t126: f64 = (p.p652 + t125);let t127: f64 = (p.p654 * l.fe91);let t128: f64 = (t126 + t127);let t129: f64 = (p.p655 * l.fd25);let t12a: f64 = (t128 + t129);l.f1690 = t12a;l.f1691 = 0.0;}
        let t12b: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };l.f685 = t12b;l.f686 = 0.0;
        if ((l.fa1e != 0.0) && (l.f685 != 0.0)) {let t12c: f64 = (p.p657 * l.fdf1);let t12d: f64 = (p.p656 + t12c);let t12e: f64 = (p.p658 * l.fe91);let t12f: f64 = (t12d + t12e);let t130: f64 = (p.p659 * l.fd25);let t131: f64 = (t12f + t130);l.f1694 = t131;l.f1695 = 0.0;}
        let t132: f64 = if (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) { 1.0 } else { 0.0 };l.f687 = t132;l.f688 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f687 != 0.0)) {let t133: f64 = (l.fd91 * l.fef3);let t134: f64 = (t133 / 1e-6);let t135: f64 = (p.p661 * l.fdf1);let t136: f64 = (p.p660 + t135);let t137: f64 = (p.p662 * l.fe91);let t138: f64 = (t136 + t137);let t139: f64 = (p.p663 * l.fd25);let t13a: f64 = (t138 + t139);let t13b: f64 = (t134 * t13a);l.f1dd = t13b;l.f1de = 0.0;}
        let t13c: f64 = if (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) { 1.0 } else { 0.0 };l.f689 = t13c;l.f696 = 0.0;
        if ((l.fa1e != 0.0) && (l.f689 != 0.0)) {let t13d: f64 = (p.p665 * l.fdf1);let t13e: f64 = (p.p664 + t13d);let t13f: f64 = (p.p666 * l.fe91);let t140: f64 = (t13e + t13f);let t141: f64 = (p.p667 * l.fd25);let t142: f64 = (t140 + t141);l.f2e2 = t142;l.f2e3 = 0.0;}
        let t143: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };l.f697 = t143;l.f698 = 0.0;
        if ((l.fa1e != 0.0) && (l.f697 != 0.0)) {let t144: f64 = (p.p669 * l.fdf1);let t145: f64 = (p.p668 + t144);let t146: f64 = (p.p670 * l.fe91);let t147: f64 = (t145 + t146);let t148: f64 = (p.p671 * l.fd25);let t149: f64 = (t147 + t148);l.f494 = t149;l.f495 = 0.0;}
        let t14a: f64 = if (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };l.f699 = t14a;l.f69a = 0.0;
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f110c = p.p580;l.f110d = 0.0;}
        let t14b: f64 = if param_given[672] { 1.0 } else { 0.0 };let t14c: f64 = if t14b == 1.0 { 1.0 } else { 0.0 };l.f69b = t14c;l.f69c = 0.0;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f69b != 0.0)) {l.f110c = p.p672;l.f110d = 0.0;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f10f8 = p.p581;l.f10f9 = 0.0;}
        let t14d: f64 = if param_given[673] { 1.0 } else { 0.0 };let t14e: f64 = if t14d == 1.0 { 1.0 } else { 0.0 };l.f69d = t14e;l.f69e = 0.0;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f69d != 0.0)) {l.f10f8 = p.p673;l.f10f9 = 0.0;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f1148 = p.p582;l.f1149 = 0.0;}
        let t14f: f64 = if param_given[674] { 1.0 } else { 0.0 };let t150: f64 = if t14f == 1.0 { 1.0 } else { 0.0 };l.f69f = t150;l.f6a0 = 0.0;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f69f != 0.0)) {l.f1148 = p.p674;l.f1149 = 0.0;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {l.f10fa = p.p583;l.f10fb = 0.0;}
        let t151: f64 = if param_given[675] { 1.0 } else { 0.0 };let t152: f64 = if t151 == 1.0 { 1.0 } else { 0.0 };l.f6a1 = t152;l.f6a2 = 0.0;
        if (((l.fa1e != 0.0) && (l.f699 != 0.0)) && (l.f6a1 != 0.0)) {l.f10fa = p.p675;l.f10fb = 0.0;}
        if ((l.fa1e != 0.0) && (l.f699 != 0.0)) {let t153: f64 = (l.f10f8 * l.fdf1);let t154: f64 = (l.f110c + t153);let t155: f64 = (l.f1148 * l.fe91);let t156: f64 = (t154 + t155);let t157: f64 = (l.f10fa * l.fd25);let t158: f64 = (t156 + t157);let t159: f64 = (l.fdf1 * t158);l.f1764 = t159;l.f1765 = 0.0;}
        let t15a: f64 = if (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };l.f6a3 = t15a;l.f6a4 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f110c = p.p596;l.f110d = 0.0;}
        let t15b: f64 = if param_given[676] { 1.0 } else { 0.0 };let t15c: f64 = if t15b == 1.0 { 1.0 } else { 0.0 };l.f6a5 = t15c;l.f6a6 = 0.0;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6a5 != 0.0)) {l.f110c = p.p676;l.f110d = 0.0;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f10f8 = p.p597;l.f10f9 = 0.0;}
        let t15d: f64 = if param_given[677] { 1.0 } else { 0.0 };let t15e: f64 = if t15d == 1.0 { 1.0 } else { 0.0 };l.f6a7 = t15e;l.f6a8 = 0.0;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6a7 != 0.0)) {l.f10f8 = p.p677;l.f10f9 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f1148 = p.p598;l.f1149 = 0.0;}
        let t15f: f64 = if param_given[678] { 1.0 } else { 0.0 };let t160: f64 = if t15f == 1.0 { 1.0 } else { 0.0 };l.f6a9 = t160;l.f6aa = 0.0;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6a9 != 0.0)) {l.f1148 = p.p678;l.f1149 = 0.0;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {l.f10fa = p.p599;l.f10fb = 0.0;}
        let t161: f64 = if param_given[679] { 1.0 } else { 0.0 };let t162: f64 = if t161 == 1.0 { 1.0 } else { 0.0 };l.f6ab = t162;l.f6ac = 0.0;
        if (((l.fa1e != 0.0) && (l.f6a3 != 0.0)) && (l.f6ab != 0.0)) {l.f10fa = p.p679;l.f10fb = 0.0;}
        if ((l.fa1e != 0.0) && (l.f6a3 != 0.0)) {let t163: f64 = (l.f10f8 * l.fdf1);let t164: f64 = (l.f110c + t163);let t165: f64 = (l.f1148 * l.fe91);let t166: f64 = (t164 + t165);let t167: f64 = (l.f10fa * l.fd25);let t168: f64 = (t166 + t167);let t169: f64 = t168;l.f100 = t169;l.f101 = 0.0;}
        let t16a: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };l.f6ad = t16a;l.f6ae = 0.0;
        if ((l.fa1e != 0.0) && (l.f6ad != 0.0)) {let t16b: f64 = (p.p681 * l.fdf1);let t16c: f64 = (p.p680 + t16b);let t16d: f64 = (p.p682 * l.fe91);let t16e: f64 = (t16c + t16d);let t16f: f64 = (p.p683 * l.fd25);let t170: f64 = (t16e + t16f);let t171: f64 = (l.fdf1 * t170);l.f47 = t171;l.f48 = 0.0;}
        let t172: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };l.f6af = t172;l.f6b0 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6af != 0.0)) {let t173: f64 = (p.p685 * l.fdf1);let t174: f64 = (p.p684 + t173);let t175: f64 = (p.p686 * l.fe91);let t176: f64 = (t174 + t175);let t177: f64 = (p.p687 * l.fd25);let t178: f64 = (t176 + t177);let t179: f64 = (l.fdf1 * t178);l.f3b = t179;l.f3c = 0.0;}
        let t17a: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };l.f6b1 = t17a;l.f6b2 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6b1 != 0.0)) {let t17b: f64 = (p.p689 * l.fdf1);let t17c: f64 = (p.p688 + t17b);let t17d: f64 = (p.p690 * l.fe91);let t17e: f64 = (t17c + t17d);let t17f: f64 = (p.p691 * l.fd25);let t180: f64 = (t17e + t17f);let t181: f64 = (l.fd91 * t180);l.f1a7 = t181;l.f1a8 = 0.0;}
        let t182: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };l.f6b3 = t182;l.f6b4 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6b3 != 0.0)) {let t183: f64 = (p.p693 * l.fdf1);let t184: f64 = (p.p692 + t183);let t185: f64 = (p.p694 * l.fe91);let t186: f64 = (t184 + t185);let t187: f64 = (p.p695 * l.fd25);let t188: f64 = (t186 + t187);let t189: f64 = (l.fd91 * t188);l.f1af = t189;l.f1b0 = 0.0;}
        let t18a: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };l.f6b5 = t18a;l.f6b8 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6b5 != 0.0)) {let t18b: f64 = (p.p697 * l.fdf1);let t18c: f64 = (p.p696 + t18b);let t18d: f64 = (p.p698 * l.fe91);let t18e: f64 = (t18c + t18d);let t18f: f64 = (p.p699 * l.fd25);let t190: f64 = (t18e + t18f);let t191: f64 = (l.fd84 * t190);l.f194 = t191;l.f195 = 0.0;}
        let t192: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };l.f6b9 = t192;l.f6ce = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f6b9 != 0.0)) {let t193: f64 = (p.p701 * l.fdf1);let t194: f64 = (p.p700 + t193);let t195: f64 = (p.p702 * l.fe91);let t196: f64 = (t194 + t195);let t197: f64 = (p.p703 * l.fd25);let t198: f64 = (t196 + t197);let t199: f64 = (l.fd91 * t198);l.f1b9 = t199;l.f1ba = 0.0;}
        let t19a: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };l.f6cf = t19a;l.f6e4 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6cf != 0.0)) {let t19b: f64 = (p.p705 * l.fdf1);let t19c: f64 = (p.p704 + t19b);let t19d: f64 = (p.p706 * l.fe91);let t19e: f64 = (t19c + t19d);let t19f: f64 = (p.p707 * l.fd25);let t1a0: f64 = (t19e + t19f);let t1a1: f64 = (l.fd91 * t1a0);l.f1bd = t1a1;l.f1be = 0.0;}
        let t1a2: f64 = if (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]) { 1.0 } else { 0.0 };l.f6e5 = t1a2;l.f6fa = 0.0;
        if ((l.fa1e != 0.0) && (l.f6e5 != 0.0)) {let t1a3: f64 = (p.p709 * l.fdf1);let t1a4: f64 = (p.p708 + t1a3);let t1a5: f64 = (p.p710 * l.fe91);let t1a6: f64 = (t1a4 + t1a5);let t1a7: f64 = (p.p711 * l.fd25);let t1a8: f64 = (t1a6 + t1a7);let t1a9: f64 = (l.fd8d * t1a8);l.f18c = t1a9;l.f18d = 0.0;}
        let t1aa: f64 = if (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]) { 1.0 } else { 0.0 };l.f6fb = t1aa;l.f710 = 0.0;
        if ((l.fa1e != 0.0) && (l.f6fb != 0.0)) {let t1ab: f64 = (p.p713 * l.fdf1);let t1ac: f64 = (p.p712 + t1ab);let t1ad: f64 = (p.p714 * l.fe91);let t1ae: f64 = (t1ac + t1ad);let t1af: f64 = (p.p715 * l.fd25);let t1b0: f64 = (t1ae + t1af);let t1b1: f64 = (l.fd8d * t1b0);l.f190 = t1b1;l.f191 = 0.0;}
        let t1b2: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };l.f75a = t1b2;l.f76f = 0.0;
        if ((l.fa1e != 0.0) && (l.f75a != 0.0)) {let t1b3: f64 = (p.p733 * l.fdf1);let t1b4: f64 = (p.p732 + t1b3);let t1b5: f64 = (p.p734 * l.fe91);let t1b6: f64 = (t1b4 + t1b5);let t1b7: f64 = (p.p735 * l.fd25);let t1b8: f64 = (t1b6 + t1b7);l.f19ab = t1b8;l.f19ac = 0.0;}
        let t1b9: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };l.f770 = t1b9;l.f771 = 0.0;
        if ((l.fa1e != 0.0) && (l.f770 != 0.0)) {let t1ba: f64 = (p.p737 * l.fdf1);let t1bb: f64 = (p.p736 + t1ba);let t1bc: f64 = (p.p738 * l.fe91);let t1bd: f64 = (t1bb + t1bc);let t1be: f64 = (p.p739 * l.fd25);let t1bf: f64 = (t1bd + t1be);l.f16c4 = t1bf;l.f16c5 = 0.0;}
        let t1c0: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };l.f772 = t1c0;l.f773 = 0.0;
        if ((l.fa1e != 0.0) && (l.f772 != 0.0)) {let t1c1: f64 = (p.p741 * l.fdf1);let t1c2: f64 = (p.p740 + t1c1);let t1c3: f64 = (p.p742 * l.fe91);let t1c4: f64 = (t1c2 + t1c3);let t1c5: f64 = (p.p743 * l.fd25);let t1c6: f64 = (t1c4 + t1c5);l.f32c = t1c6;l.f32d = 0.0;}
        let t1c7: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };l.f774 = t1c7;l.f775 = 0.0;
        if ((l.fa1e != 0.0) && (l.f774 != 0.0)) {let t1c8: f64 = (p.p745 * l.fdf1);let t1c9: f64 = (p.p744 + t1c8);let t1ca: f64 = (p.p746 * l.fe91);let t1cb: f64 = (t1c9 + t1ca);let t1cc: f64 = (p.p747 * l.fd25);let t1cd: f64 = (t1cb + t1cc);l.ffbd = t1cd;l.ffbe = 0.0;}
        let t1ce: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };l.f776 = t1ce;l.f777 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f776 != 0.0)) {let t1cf: f64 = (p.p749 * l.fdf1);let t1d0: f64 = (p.p748 + t1cf);let t1d1: f64 = (p.p750 * l.fe91);let t1d2: f64 = (t1d0 + t1d1);let t1d3: f64 = (p.p751 * l.fd25);let t1d4: f64 = (t1d2 + t1d3);l.f21a = t1d4;l.f21b = 0.0;}
        let t1d5: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };l.f778 = t1d5;l.f779 = 0.0;
        if ((l.fa1e != 0.0) && (l.f778 != 0.0)) {let t1d6: f64 = (l.f1afe / l.fef1);let t1d7: f64 = (p.p753 * l.fdf1);let t1d8: f64 = (p.p752 + t1d7);let t1d9: f64 = (p.p754 * l.fe91);let t1da: f64 = (t1d8 + t1d9);let t1db: f64 = (p.p755 * l.fd25);let t1dc: f64 = (t1da + t1db);let t1dd: f64 = (t1d6 * t1dc);l.f124 = t1dd;l.f125 = 0.0;}
        let t1de: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };l.f77a = t1de;l.f77b = 0.0;
        if ((l.fa1e != 0.0) && (l.f77a != 0.0)) {let t1df: f64 = (p.p757 * l.fdf1);let t1e0: f64 = (p.p756 + t1df);let t1e1: f64 = (p.p758 * l.fe91);let t1e2: f64 = (t1e0 + t1e1);let t1e3: f64 = (p.p759 * l.fd25);let t1e4: f64 = (t1e2 + t1e3);l.f168c = t1e4;l.f168d = 0.0;}
        let t1e5: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };l.f77c = t1e5;l.f77d = 0.0;
        if ((l.fa1e != 0.0) && (l.f77c != 0.0)) {let t1e6: f64 = (p.p761 * l.fdf1);let t1e7: f64 = (p.p760 + t1e6);let t1e8: f64 = (p.p762 * l.fe91);let t1e9: f64 = (t1e7 + t1e8);let t1ea: f64 = (p.p763 * l.fd25);let t1eb: f64 = (t1e9 + t1ea);let t1ec: f64 = (l.fdf2 * t1eb);l.f113b = t1ec;l.f113c = 0.0;}
        let t1ed: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };l.f77e = t1ed;l.f77f = 0.0;
        if ((l.fa1e != 0.0) && (l.f77e != 0.0)) {let t1ee: f64 = (p.p765 * l.fdf1);let t1ef: f64 = (p.p764 + t1ee);let t1f0: f64 = (p.p766 * l.fe91);let t1f1: f64 = (t1ef + t1f0);let t1f2: f64 = (p.p767 * l.fd25);let t1f3: f64 = (t1f1 + t1f2);l.f112f = t1f3;l.f1130 = 0.0;}
        let t1f4: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };l.f780 = t1f4;l.f781 = 0.0;
        if ((l.fa1e != 0.0) && (l.f780 != 0.0)) {let t1f5: f64 = (p.p769 * l.fdf1);let t1f6: f64 = (p.p768 + t1f5);let t1f7: f64 = (p.p770 * l.fe91);let t1f8: f64 = (t1f6 + t1f7);let t1f9: f64 = (p.p771 * l.fd25);let t1fa: f64 = (t1f8 + t1f9);l.f1137 = t1fa;l.f1138 = 0.0;}
        let t1fb: f64 = if (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]) { 1.0 } else { 0.0 };l.f782 = t1fb;l.f783 = 0.0;
        if ((l.fa1e != 0.0) && (l.f782 != 0.0)) {let t1fc: f64 = (p.p773 * l.fdf1);let t1fd: f64 = (p.p772 + t1fc);let t1fe: f64 = (p.p774 * l.fe91);let t1ff: f64 = (t1fd + t1fe);let t200: f64 = (p.p775 * l.fd25);let t201: f64 = (t1ff + t200);let t202: f64 = (l.fdf2 * t201);l.f188 = t202;l.f189 = 0.0;}
        let t203: f64 = if (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]) { 1.0 } else { 0.0 };l.f784 = t203;l.f785 = 0.0;
        if ((l.fa1e != 0.0) && (l.f784 != 0.0)) {let t204: f64 = (p.p781 * l.fdf1);let t205: f64 = (p.p780 + t204);let t206: f64 = (p.p782 * l.fe91);let t207: f64 = (t205 + t206);let t208: f64 = (p.p783 * l.fd25);let t209: f64 = (t207 + t208);l.f184 = t209;l.f185 = 0.0;}
        let t20a: f64 = if (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]) { 1.0 } else { 0.0 };l.f786 = t20a;l.f787 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f786 != 0.0)) {let t20b: f64 = (p.p777 * l.fdf1);let t20c: f64 = (p.p776 + t20b);let t20d: f64 = (p.p778 * l.fe91);let t20e: f64 = (t20c + t20d);let t20f: f64 = (p.p779 * l.fd25);let t210: f64 = (t20e + t20f);l.f17c = t210;l.f17d = 0.0;}
        let t211: f64 = if (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]) { 1.0 } else { 0.0 };l.f789 = t211;l.f78a = 0.0;
        if ((l.fa1e != 0.0) && (l.f789 != 0.0)) {let t212: f64 = (p.p801 * l.fdf1);let t213: f64 = (p.p800 + t212);let t214: f64 = (p.p802 * l.fe91);let t215: f64 = (t213 + t214);let t216: f64 = (p.p803 * l.fd25);let t217: f64 = (t215 + t216);let t218: f64 = (l.fd82 * t217);l.f225 = t218;l.f226 = 0.0;}
        if (l.fa1e != 0.0) {l.f17d3 = 0.0;l.f17d4 = 0.0;l.f17d5 = 0.0;l.f17d6 = 0.0;l.ff03 = 0.0;l.ff04 = 0.0;l.fedb = p.p812;l.fedc = 0.0;}
        let t219: f64 = if param_given[813] { 1.0 } else { 0.0 };let t21a: f64 = if t219 == 1.0 { 1.0 } else { 0.0 };l.f78c = t21a;l.f78d = 0.0;
        if ((l.fa1e != 0.0) && (l.f78c != 0.0)) {l.fedb = p.p813;l.fedc = 0.0;}
        let t21b: f64 = if (((l.f1487 > 0.0) && (l.f1489 > 0.0)) && ((l.ffbf == 1.0) || ((l.ffbf > 1.0) && (l.f1493 > 0.0)))) { 1.0 } else { 0.0 };l.f78e = t21b;l.f79d = 0.0;let mut t22d: usize = 0;
        while {
            let t22b: f64 = (l.ffbf - 0.5);let t22c: f64 = if (((l.fa1e != 0.0) && (l.f78e != 0.0)) && (l.ff03 < t22b)) { 1.0 } else { 0.0 };
            t22c != 0.0
        } {
            t22d += 1;assert!(t22d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t21c: f64 = (0.5 * l.fee0);let t21d: f64 = (l.f1487 + t21c);let t21e: f64 = (l.f1493 + l.fee0);let t21f: f64 = (l.ff03 * t21e);let t220: f64 = (t21d + t21f);let t221: f64 = (1.0 / t220);let t222: f64 = (l.f17d3 + t221);l.f17d3 = t222;l.f17d4 = 0.0;let t223: f64 = (0.5 * l.fee0);let t224: f64 = (l.f1489 + t223);let t225: f64 = (l.f1493 + l.fee0);let t226: f64 = (l.ff03 * t225);let t227: f64 = (t224 + t226);let t228: f64 = (1.0 / t227);let t229: f64 = (l.f17d5 + t228);l.f17d5 = t229;l.f17d6 = 0.0;let t22a: f64 = (l.ff03 + 1.0);l.ff03 = t22a;l.ff04 = 0.0;}
        }
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t22e: f64 = (l.f17d3 * l.fe57);l.fe59 = t22e;l.fe5a = 0.0;let t22f: f64 = (l.f17d5 * l.fe57);l.fe5d = t22f;l.fe5e = 0.0;let t230: f64 = (0.5 * l.fee0);let t231: f64 = (p.p808 + t230);let t232: f64 = (1.0 / t231);l.fe5b = t232;l.fe5c = 0.0;let t233: f64 = (0.5 * l.fee0);let t234: f64 = (p.p809 + t233);let t235: f64 = (1.0 / t234);l.fe5f = t235;l.fe60 = 0.0;}
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {
            let t236: f64 = (l.fee0 + l.f266);
            let (t238,) = {
    if (t236 > 1e-9) {
        let t237: f64 = (l.fee0 + l.f266);
        (t237,)
    } else {
        (1e-9,)
    }
};
            l.ff1e = t238;l.ff1f = 0.0;
        }
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {
            let t239: f64 = (l.f1ad4 + l.f2e8);let t23a: f64 = (t239 + p.p810);
            let (t23d,) = {
    if (t23a > 1e-9) {
        let t23b: f64 = (l.f1ad4 + l.f2e8);let t23c: f64 = (t23b + p.p810);
        (t23c,)
    } else {
        (1e-9,)
    }
};
            l.f1b35 = t23d;l.f1b36 = 0.0;
        }
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t23e: f64 = (l.ff1e).powf(p.p818);let t23f: f64 = (1.0 / t23e);l.f16fe = t23f;l.f16ff = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t240: f64 = (l.f1b35).powf(p.p819);let t241: f64 = (1.0 / t240);l.f1700 = t241;l.f1701 = 0.0;let t242: f64 = (p.p815 * l.f16fe);let t243: f64 = (1.0 + t242);let t244: f64 = (p.p816 * l.f1700);let t245: f64 = (t243 + t244);let t246: f64 = (p.p817 * l.f16fe);let t247: f64 = (t246 * l.f1700);let t248: f64 = (t245 + t247);let t249: f64 = (l.f1448 - 1.0);let t24a: f64 = (p.p814 * t249);let t24b: f64 = (1.0 + t24a);let t24c: f64 = (t248 * t24b);l.fec8 = t24c;l.fec9 = 0.0;let t24d: f64 = (l.fe59 + l.fe5d);let t24e: f64 = (p.p811 * t24d);let t24f: f64 = (t24e / l.fec8);l.f1418 = t24f;l.f1419 = 0.0;let t250: f64 = (l.fe5b + l.fe5f);let t251: f64 = (p.p811 * t250);let t252: f64 = (t251 / l.fec8);l.f141a = t252;l.f141b = 0.0;let t253: f64 = (l.ff1e).powf(p.p824);let t254: f64 = (1.0 / t253);l.f16fe = t254;l.f16ff = 0.0;let t255: f64 = (l.f1b35).powf(p.p825);let t256: f64 = (1.0 / t255);l.f1700 = t256;l.f1701 = 0.0;let t257: f64 = (p.p821 * l.f16fe);let t258: f64 = (1.0 + t257);let t259: f64 = (p.p822 * l.f1700);let t25a: f64 = (t258 + t259);let t25b: f64 = (p.p823 * l.f16fe);let t25c: f64 = (t25b * l.f1700);let t25d: f64 = (t25a + t25c);l.feca = t25d;l.fecb = 0.0;let t25e: f64 = (l.fe59 + l.fe5d);let t25f: f64 = (t25e - l.fe5b);let t260: f64 = (t25f - l.fe5f);l.f16dd = t260;l.f16e0 = 0.0;let t261: f64 = (1.0 + l.f1418);let t262: f64 = (1.0 + l.f141a);let t263: f64 = (t261 / t262);l.f16de = t263;l.f16df = 0.0;let t264: f64 = (l.f11d * l.f16de);l.f11d = t264;l.f11e = 0.0;let t265: f64 = (l.f175d * l.f16de);let t266: f64 = (p.p812 * l.f141a);let t267: f64 = (1.0 + t266);let t268: f64 = (t265 * t267);let t269: f64 = (p.p812 * l.f1418);let t26a: f64 = (1.0 + t269);let t26b: f64 = (t268 / t26a);l.f175d = t26b;l.f175e = 0.0;let t26c: f64 = (l.f1764 * l.f16de);let t26d: f64 = (l.fedb * l.f141a);let t26e: f64 = (1.0 + t26d);let t26f: f64 = (t26c * t26e);let t270: f64 = (l.fedb * l.f1418);let t271: f64 = (1.0 + t270);let t272: f64 = (t26f / t271);l.f1764 = t272;l.f1765 = 0.0;let t273: f64 = (l.f124 * l.f16de);l.f124 = t273;l.f125 = 0.0;let t274: f64 = (p.p820 * l.f16dd);let t275: f64 = (t274 / l.feca);l.f16de = t275;l.f16df = 0.0;let t276: f64 = (l.f19a4 + l.f16de);l.f19a4 = t276;l.f19a5 = 0.0;let t277: f64 = (l.f19ab + l.f16de);l.f19ab = t277;l.f19ac = 0.0;let t278: f64 = (p.p826 * l.f16dd);let t279: f64 = (l.feca).powf(p.p827);let t27a: f64 = (t278 / t279);l.f16de = t27a;l.f16df = 0.0;let t27b: f64 = (l.f174 + l.f16de);l.f174 = t27b;l.f175 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.fa1e != 0.0) && (l.f78e != 0.0)) {let t27c: f64 = (l.f188 + l.f16de);l.f188 = t27c;l.f189 = 0.0;}
        let t27d: f64 = if ((((l.f148d > 0.0) || (l.f148f > 0.0)) || (l.f1491 > 0.0)) || (l.f148b > 0.0)) { 1.0 } else { 0.0 };l.f79e = t27d;l.f7b3 = 0.0;let t27e: f64 = if (((l.f148d == 0.0) && (l.f148f == 0.0)) && (l.f1491 == 0.0)) { 1.0 } else { 0.0 };l.f7b4 = t27e;l.f7c9 = 0.0;
        if (((l.fa1e != 0.0) && (l.f79e != 0.0)) && (l.f7b4 != 0.0)) {let t27f: f64 = (l.f148b + l.f1ad4);l.f16dd = t27f;l.f16e0 = 0.0;let t280: f64 = (1.0 / p.p828);l.f16de = t280;l.f16df = 0.0;let t281: f64 = (p.p828 * p.p828);let t282: f64 = (l.f148b * l.f16dd);let t283: f64 = (t281 / t282);l.f148d = t283;l.f148e = 0.0;let t284: f64 = (0.1 * l.f148b);let t285: f64 = (0.01 * p.p828);let t286: f64 = (t284 + t285);let t287: f64 = (-10.0);let t288: f64 = (t287 * l.f148b);let t289: f64 = (t288 * l.f16de);let t28a: f64 = (t289).exp();let t28b: f64 = (t286 * t28a);let t28c: f64 = (0.1 * l.f16dd);let t28d: f64 = (0.01 * p.p828);let t28e: f64 = (t28c + t28d);let t28f: f64 = (-10.0);let t290: f64 = (t28f * l.f16dd);let t291: f64 = (t290 * l.f16de);let t292: f64 = (t291).exp();let t293: f64 = (t28e * t292);let t294: f64 = (t28b - t293);let t295: f64 = (t294 / l.f1ad4);l.f148f = t295;l.f1490 = 0.0;let t296: f64 = (0.05 * l.f148b);let t297: f64 = (0.0025 * p.p828);let t298: f64 = (t296 + t297);let t299: f64 = (-20.0);let t29a: f64 = (t299 * l.f148b);let t29b: f64 = (t29a * l.f16de);let t29c: f64 = (t29b).exp();let t29d: f64 = (t298 * t29c);let t29e: f64 = (0.05 * l.f16dd);let t29f: f64 = (0.0025 * p.p828);let t2a0: f64 = (t29e + t29f);let t2a1: f64 = (-20.0);let t2a2: f64 = (t2a1 * l.f16dd);let t2a3: f64 = (t2a2 * l.f16de);let t2a4: f64 = (t2a3).exp();let t2a5: f64 = (t2a0 * t2a4);let t2a6: f64 = (t29d - t2a5);let t2a7: f64 = (t2a6 / l.f1ad4);l.f1491 = t2a7;l.f1492 = 0.0;}
        if ((l.fa1e != 0.0) && (l.f79e != 0.0)) {let t2a8: f64 = (p.p829 * l.f148f);let t2a9: f64 = (l.f148d + t2a8);let t2aa: f64 = (p.p830 * l.f1491);let t2ab: f64 = (t2a9 + t2aa);l.f16dd = t2ab;l.f16e0 = 0.0;let t2ac: f64 = (l.fedd * l.f16dd);let t2ad: f64 = (l.f19a4 + t2ac);l.f19a4 = t2ad;l.f19a5 = 0.0;let t2ae: f64 = (l.fed9 * l.f16dd);let t2af: f64 = (1.0 + t2ae);let t2b0: f64 = (l.f11d * t2af);l.f11d = t2b0;l.f11e = 0.0;let t2b1: f64 = (l.fedd * l.f16dd);let t2b2: f64 = (l.f19ab + t2b1);l.f19ab = t2b2;l.f19ac = 0.0;let t2b3: f64 = (l.fed9 * l.f16dd);let t2b4: f64 = (1.0 + t2b3);let t2b5: f64 = (l.f124 * t2b4);l.f124 = t2b5;l.f125 = 0.0;}
        l.f19a2 = l.f19a4;l.f19a3 = 0.0;l.f16be = l.f16c0;l.f16bf = 0.0;l.f167e = l.f1680;l.f167f = 0.0;l.f17df = l.f17e1;l.f17e0 = 0.0;l.f41d = l.f41f;l.f41e = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        l: &mut StampLocals,
    ) {
        let (t2b7,) = {
    if (l.ffb7 > 1e20) {
        let (t2b6,) = {
            if (l.ffb7 < 1e26) {
                (l.ffb7,)
            } else {
                (1e26,)
            }
        };
        (t2b6,)
    } else {
        (1e20,)
    }
};
        l.ffb5 = t2b7;l.ffb6 = 0.0;
        let (t2b8,) = {
    if (l.f5f4 > 0.01) {
        (l.f5f4,)
    } else {
        (0.01,)
    }
};
        l.f5f2 = t2b8;l.f5f3 = 0.0;
        let (t2b9,) = {
    if (l.f1a72 > 0.0) {
        (l.f1a72,)
    } else {
        (0.0,)
    }
};
        l.f1a70 = t2b9;l.f1a71 = 0.0;l.f3c7 = l.f3c9;l.f3c8 = 0.0;l.f326 = l.f328;l.f327 = 0.0;
        let (t2ba,) = {
    if (l.ffcd > 0.0) {
        (l.ffcd,)
    } else {
        (0.0,)
    }
};
        l.ffcb = t2ba;l.ffcc = 0.0;l.f17e5 = l.f17e7;l.f17e6 = 0.0;l.f17e9 = l.f17eb;l.f17ea = 0.0;
        let (t2bc,) = {
    if (l.ffc4 > 1e23) {
        let (t2bb,) = {
            if (l.ffc4 < 1e27) {
                (l.ffc4,)
            } else {
                (1e27,)
            }
        };
        (t2bb,)
    } else {
        (1e23,)
    }
};
        l.ffc2 = t2bc;l.ffc3 = 0.0;
        let (t2be,) = {
    if (l.ffc8 > 1e23) {
        let (t2bd,) = {
            if (l.ffc8 < 1e27) {
                (l.ffc8,)
            } else {
                (1e27,)
            }
        };
        (t2bd,)
    } else {
        (1e23,)
    }
};
        l.ffc6 = t2be;l.ffc7 = 0.0;
        let (t2bf,) = {
    if (l.f209 > 0.0) {
        (l.f209,)
    } else {
        (0.0,)
    }
};
        l.f207 = t2bf;l.f208 = 0.0;
        let (t2c1,) = {
    if (l.f216 > 0.0) {
        let (t2c0,) = {
            if (l.f216 < 0.5) {
                (l.f216,)
            } else {
                (0.5,)
            }
        };
        (t2c0,)
    } else {
        (0.0,)
    }
};
        l.f214 = t2c1;l.f215 = 0.0;
        let (t2c3,) = {
    if (l.f21e > 0.0) {
        let (t2c2,) = {
            if (l.f21e < 1.0) {
                (l.f21e,)
            } else {
                (1.0,)
            }
        };
        (t2c2,)
    } else {
        (0.0,)
    }
};
        l.f21c = t2c3;l.f21d = 0.0;l.f169a = l.f169c;l.f169b = 0.0;
        let (t2c4,) = {
    if (l.f174 > 0.0) {
        (l.f174,)
    } else {
        (0.0,)
    }
};
        l.f172 = t2c4;l.f173 = 0.0;
        let (t2c6,) = {
    if (l.f178 > 0.0) {
        let (t2c5,) = {
            if (l.f178 < 1.0) {
                (l.f178,)
            } else {
                (1.0,)
            }
        };
        (t2c5,)
    } else {
        (0.0,)
    }
};
        l.f176 = t2c6;l.f177 = 0.0;
        let (t2c7,) = {
    if (l.f180 > 0.0) {
        (l.f180,)
    } else {
        (0.0,)
    }
};
        l.f17e = t2c7;l.f17f = 0.0;
        let (t2c8,) = {
    if (l.f1127 > 0.0) {
        (l.f1127,)
    } else {
        (0.0,)
    }
};
        l.f1125 = t2c8;l.f1126 = 0.0;
        let (t2ca,) = {
    if (l.f112b > 0.0) {
        let (t2c9,) = {
            if (l.f112b < 1.0) {
                (l.f112b,)
            } else {
                (1.0,)
            }
        };
        (t2c9,)
    } else {
        (0.0,)
    }
};
        l.f1129 = t2ca;l.f112a = 0.0;
        let (t2cb,) = {
    if (l.f1133 > 0.0) {
        (l.f1133,)
    } else {
        (0.0,)
    }
};
        l.f1131 = t2cb;l.f1132 = 0.0;
        let (t2cc,) = {
    if (l.f11d > 0.0) {
        (l.f11d,)
    } else {
        (0.0,)
    }
};
        l.f11b = t2cc;l.f11c = 0.0;l.f1686 = l.f1688;l.f1687 = 0.0;
        let (t2cd,) = {
    if (l.ff99 > 0.0) {
        (l.ff99,)
    } else {
        (0.0,)
    }
};
        l.ff97 = t2cd;l.ff98 = 0.0;l.f16a8 = l.f16aa;l.f16a9 = 0.0;
        let (t2ce,) = {
    if (l.f1731 > 0.0) {
        (l.f1731,)
    } else {
        (0.0,)
    }
};
        l.f172f = t2ce;l.f1730 = 0.0;l.f16b6 = l.f16b8;l.f16b7 = 0.0;
        let (t2cf,) = {
    if (l.f1ee > 0.0) {
        (l.f1ee,)
    } else {
        (0.0,)
    }
};
        l.f1ec = t2cf;l.f1ed = 0.0;l.f1696 = l.f1698;l.f1697 = 0.0;
        let (t2d0,) = {
    if (l.f172a > 0.0) {
        (l.f172a,)
    } else {
        (0.0,)
    }
};
        l.f1728 = t2d0;l.f1729 = 0.0;l.f16b2 = l.f16b4;l.f16b3 = 0.0;
        let (t2d1,) = {
    if (l.f1bee > 0.0) {
        (l.f1bee,)
    } else {
        (0.0,)
    }
};
        l.f1bec = t2d1;l.f1bed = 0.0;l.f16c6 = l.f16c8;l.f16c7 = 0.0;l.f4e9 = l.f4eb;l.f4ea = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        l: &mut StampLocals,
    ) {
        let (t2d2,) = {
    if (l.f1437 > 0.0) {
        (l.f1437,)
    } else {
        (0.0,)
    }
};
        l.f1435 = t2d2;l.f1436 = 0.0;l.f16ac = l.f16ae;l.f16ad = 0.0;let t2d3: f64 = (-0.5);
        let (t2d6,) = {
    if (l.f143e > t2d3) {
        let (t2d4,) = {
            if (l.f143e < 1.0) {
                (l.f143e,)
            } else {
                (1.0,)
            }
        };
        (t2d4,)
    } else {
        let t2d5: f64 = (-0.5);
        (t2d5,)
    }
};
        l.f143c = t2d6;l.f143d = 0.0;let t2d7: f64 = (-0.5);
        let (t2d9,) = {
    if (l.f1444 > t2d7) {
        (l.f1444,)
    } else {
        let t2d8: f64 = (-0.5);
        (t2d8,)
    }
};
        l.f1442 = t2d9;l.f1443 = 0.0;
        let (t0,) = {
    if (l.f175d > 0.0) {
        (l.f175d,)
    } else {
        (0.0,)
    }
};
        l.f175b = t0;l.f175c = 0.0;l.f16ba = l.f16bc;l.f16bb = 0.0;let t1: f64 = (-0.5);
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
        l.f1773 = t4;l.f1774 = 0.0;let t5: f64 = (-0.5);
        let (t7,) = {
    if (l.f1795 > t5) {
        (l.f1795,)
    } else {
        let t6: f64 = (-0.5);
        (t6,)
    }
};
        l.f1793 = t7;l.f1794 = 0.0;
        let (t8,) = {
    if (l.f179f > 0.01) {
        (l.f179f,)
    } else {
        (0.01,)
    }
};
        l.f179d = t8;l.f179e = 0.0;
        let (t9,) = {
    if (l.ffc > 2.0) {
        (l.ffc,)
    } else {
        (2.0,)
    }
};
        l.ffa = t9;l.ffb = 0.0;
        let (ta,) = {
    if (l.f43 > 0.0) {
        (l.f43,)
    } else {
        (0.0,)
    }
};
        l.f41 = ta;l.f42 = 0.0;
        let (tb,) = {
    if (l.f37 > 0.0) {
        (l.f37,)
    } else {
        (0.0,)
    }
};
        l.f35 = tb;l.f36 = 0.0;
        let (tc,) = {
    if (l.f3f > 0.0) {
        (l.f3f,)
    } else {
        (0.0,)
    }
};
        l.f3d = tc;l.f3e = 0.0;l.f1a6c = l.f1a6e;l.f1a6d = 0.0;
        let (td,) = {
    if (l.f2 > 0.0) {
        (l.f2,)
    } else {
        (0.0,)
    }
};
        l.f0 = td;l.f1 = 0.0;l.f4 = l.f6;l.f5 = 0.0;l.f1682 = l.f1684;l.f1683 = 0.0;
        let (te,) = {
    if (l.fd > 0.0) {
        (l.fd,)
    } else {
        (0.0,)
    }
};
        l.fb = te;l.fc = 0.0;
        let (tf,) = {
    if (l.f11 > 0.0) {
        (l.f11,)
    } else {
        (0.0,)
    }
};
        l.ff = tf;l.f10 = 0.0;
        let (t10,) = {
    if (l.fdf7 > 1e-12) {
        (l.fdf7,)
    } else {
        (1e-12,)
    }
};
        l.fdf5 = t10;l.fdf6 = 0.0;l.f5a2 = l.f5a4;l.f5a3 = 0.0;
        let (t11,) = {
    if (l.fd72 > 0.0) {
        (l.fd72,)
    } else {
        (0.0,)
    }
};
        l.fd70 = t11;l.fd71 = 0.0;
        let (t12,) = {
    if (l.fd76 > 0.0) {
        (l.fd76,)
    } else {
        (0.0,)
    }
};
        l.fd74 = t12;l.fd75 = 0.0;
        let (t13,) = {
    if (l.fd7a > 0.0) {
        (l.fd7a,)
    } else {
        (0.0,)
    }
};
        l.fd78 = t13;l.fd79 = 0.0;l.f16a4 = l.f16a6;l.f16a5 = 0.0;l.f58a = l.f58c;l.f58b = 0.0;l.f596 = l.f598;l.f597 = 0.0;l.f58e = l.f590;l.f58f = 0.0;l.f59a = l.f59c;l.f59b = 0.0;l.f592 = l.f594;l.f593 = 0.0;l.f59e = l.f5a0;l.f59f = 0.0;l.f1b1 = l.f1b3;l.f1b2 = 0.0;
        let (t14,) = {
    if (l.f2b > 0.0) {
        (l.f2b,)
    } else {
        (0.0,)
    }
};
        l.f29 = t14;l.f2a = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        l: &mut StampLocals,
    ) {
        let (t15,) = {
    if (l.f2f > 0.0) {
        (l.f2f,)
    } else {
        (0.0,)
    }
};
        l.f2d = t15;l.f2e = 0.0;l.f12f = l.f131;l.f130 = 0.0;l.f135 = l.f137;l.f136 = 0.0;l.f168e = l.f1690;l.f168f = 0.0;l.f1692 = l.f1694;l.f1693 = 0.0;l.f19d = l.f19f;l.f19e = 0.0;l.f1a1 = l.f1a3;l.f1a2 = 0.0;
        let (t16,) = {
    if (l.f1dd > 0.0) {
        (l.f1dd,)
    } else {
        (0.0,)
    }
};
        l.f1d9 = t16;l.f1da = 0.0;l.f2e0 = l.f2e2;l.f2e1 = 0.0;
        let (t17,) = {
    if (l.f494 > 0.0) {
        (l.f494,)
    } else {
        (0.0,)
    }
};
        l.f492 = t17;l.f493 = 0.0;
        let (t18,) = {
    if (l.f1764 > 0.0) {
        (l.f1764,)
    } else {
        (0.0,)
    }
};
        l.f1762 = t18;l.f1763 = 0.0;
        let (t19,) = {
    if (l.f100 > 2.0) {
        (l.f100,)
    } else {
        (2.0,)
    }
};
        l.ffe = t19;l.fff = 0.0;l.f45 = l.f47;l.f46 = 0.0;
        let (t1a,) = {
    if (l.f3b > 0.0) {
        (l.f3b,)
    } else {
        (0.0,)
    }
};
        l.f39 = t1a;l.f3a = 0.0;
        let (t1b,) = {
    if (l.f1a7 > 0.0) {
        (l.f1a7,)
    } else {
        (0.0,)
    }
};
        l.f1a5 = t1b;l.f1a6 = 0.0;
        let (t1c,) = {
    if (l.f1af > 0.0) {
        (l.f1af,)
    } else {
        (0.0,)
    }
};
        l.f1ad = t1c;l.f1ae = 0.0;l.f4d7 = l.f4d9;l.f4d8 = 0.0;l.f4db = l.f4dd;l.f4dc = 0.0;l.f1a9 = l.f1ab;l.f1aa = 0.0;
        let (t1d,) = {
    if (l.f194 > 0.0) {
        (l.f194,)
    } else {
        (0.0,)
    }
};
        l.f192 = t1d;l.f193 = 0.0;
        let (t1e,) = {
    if (l.f1b9 > 0.0) {
        (l.f1b9,)
    } else {
        (0.0,)
    }
};
        l.f1b7 = t1e;l.f1b8 = 0.0;
        let (t1f,) = {
    if (l.f1bd > 0.0) {
        (l.f1bd,)
    } else {
        (0.0,)
    }
};
        l.f1bb = t1f;l.f1bc = 0.0;l.f3ae = l.f3b0;l.f3af = 0.0;l.f4e3 = l.f4e5;l.f4e4 = 0.0;l.f4df = l.f4e1;l.f4e0 = 0.0;l.f106 = l.f108;l.f107 = 0.0;
        let (t20,) = {
    if (l.f18c > 0.0) {
        (l.f18c,)
    } else {
        (0.0,)
    }
};
        l.f18a = t20;l.f18b = 0.0;
        let (t21,) = {
    if (l.f190 > 0.0) {
        (l.f190,)
    } else {
        (0.0,)
    }
};
        l.f18e = t21;l.f18f = 0.0;l.f51f = l.f521;l.f520 = 0.0;l.f19a9 = l.f19ab;l.f19aa = 0.0;l.f16c2 = l.f16c4;l.f16c3 = 0.0;l.f32a = l.f32c;l.f32b = 0.0;
        let (t23,) = {
    if (l.ffbd > 1e20) {
        let (t22,) = {
            if (l.ffbd < 1e26) {
                (l.ffbd,)
            } else {
                (1e26,)
            }
        };
        (t22,)
    } else {
        (1e20,)
    }
};
        l.ffbb = t23;l.ffbc = 0.0;
        let (t24,) = {
    if (l.f21a > 0.0) {
        (l.f21a,)
    } else {
        (0.0,)
    }
};
        l.f218 = t24;l.f219 = 0.0;
        let (t25,) = {
    if (l.f124 > 0.0) {
        (l.f124,)
    } else {
        (0.0,)
    }
};
        l.f122 = t25;l.f123 = 0.0;l.f168a = l.f168c;l.f168b = 0.0;
        let (t26,) = {
    if (l.f113b > 0.0) {
        (l.f113b,)
    } else {
        (0.0,)
    }
};
        l.f1139 = t26;l.f113a = 0.0;
        let (t28,) = {
    if (l.f112f > 0.0) {
        let (t27,) = {
            if (l.f112f < 1.0) {
                (l.f112f,)
            } else {
                (1.0,)
            }
        };
        (t27,)
    } else {
        (0.0,)
    }
};
        l.f112d = t28;l.f112e = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let (t29,) = {
    if (l.f1137 > 0.0) {
        (l.f1137,)
    } else {
        (0.0,)
    }
};
        l.f1135 = t29;l.f1136 = 0.0;
        let (t2a,) = {
    if (l.f188 > 0.0) {
        (l.f188,)
    } else {
        (0.0,)
    }
};
        l.f186 = t2a;l.f187 = 0.0;
        let (t2c,) = {
    if (l.f17c > 0.0) {
        let (t2b,) = {
            if (l.f17c < 1.0) {
                (l.f17c,)
            } else {
                (1.0,)
            }
        };
        (t2b,)
    } else {
        (0.0,)
    }
};
        l.f17a = t2c;l.f17b = 0.0;
        let (t2d,) = {
    if (l.f184 > 0.0) {
        (l.f184,)
    } else {
        (0.0,)
    }
};
        l.f182 = t2d;l.f183 = 0.0;
        let (t2e,) = {
    if (l.f225 > 0.0) {
        (l.f225,)
    } else {
        (0.0,)
    }
};
        l.f223 = t2e;l.f224 = 0.0;let t2f: f64 = (p.p31 * l.ffbf);
        let (t31,) = {
    if (t2f > 0.0) {
        let t30: f64 = (p.p31 * l.ffbf);
        (t30,)
    } else {
        (0.0,)
    }
};
        l.ff9e = t31;l.ff9f = 0.0;l.f4ab = p.p16;l.f4ac = 0.0;l.f2e4 = p.p15;l.f2e5 = 0.0;l.f4ad = p.p18;l.f4ae = 0.0;l.f2e6 = p.p17;l.f2e7 = 0.0;let t32: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };l.f7ca = t32;l.f7df = 0.0;
        if (l.f7ca != 0.0) {l.f17e9 = l.f17e5;l.f17ea = 0.0;l.ffc6 = l.ffc2;l.ffc7 = 0.0;l.f2d = l.f29;l.f2e = 0.0;l.f135 = l.f12f;l.f136 = 0.0;l.f1692 = l.f168e;l.f1693 = 0.0;l.f1a1 = l.f19d;l.f1a2 = 0.0;l.fd78 = l.fd74;l.fd79 = 0.0;l.f592 = l.f58e;l.f593 = 0.0;l.f59e = l.f59a;l.f59f = 0.0;l.f1ad = l.f1a5;l.f1ae = 0.0;l.f4db = l.f4d7;l.f4dc = 0.0;l.f1bb = l.f1b7;l.f1bc = 0.0;l.f18e = l.f18a;l.f18f = 0.0;}
        let t33: f64 = (8.8541878176e-12 * l.f41d);l.f41b = t33;l.f41c = 0.0;let t34: f64 = (l.f41b / l.f17df);l.f1ea = t34;l.f1eb = 0.0;let t35: f64 = (l.f17df * l.f17df);l.f17e3 = t35;l.f17e4 = 0.0;let t36: f64 = (l.f1ea / 1.6021918e-19);l.f1db = t36;l.f1dc = 0.0;let t37: f64 = (l.f492 * l.ffb5);l.ffb9 = t37;l.ffba = 0.0;
        let (t39,) = {
    if (l.ffb9 > 1e20) {
        let (t38,) = {
            if (l.ffb9 < 1e26) {
                (l.ffb9,)
            } else {
                (1e26,)
            }
        };
        (t38,)
    } else {
        (1e20,)
    }
};
        l.ffb9 = t39;l.ffba = 0.0;l.f13e0 = 0.0;l.f13e1 = 0.0;let t3a: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };l.f7e0 = t3a;l.f7f5 = 0.0;
        if (l.f7e0 != 0.0) {let t3b: f64 = (0.4 * 5.951993);let t3c: f64 = (t3b * p.p51);let t3d: f64 = (l.f1ea).powf(0.6666666666666666);let t3e: f64 = (t3c * t3d);l.f13e0 = t3e;l.f13e1 = 0.0;}
        let t3f: f64 = (-1.0);let t40: f64 = if l.f1b5 == t3f { 1.0 } else { 0.0 };l.f7f6 = t40;l.f80b = 0.0;
        if ((l.f7e0 != 0.0) && (l.f7f6 != 0.0)) {let t41: f64 = (7.448711 / 5.951993);let t42: f64 = (t41 * l.f13e0);l.f13e0 = t42;l.f13e1 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        l: &mut StampLocals,
    ) {
        let t43: f64 = (1e-8 * l.f1ea);let t44: f64 = (t43 / l.f421);l.f3de = t44;l.f3df = 0.0;let t45: f64 = (0.5 * l.f4e9);l.f450 = t45;l.f453 = 0.0;l.f451 = 0.5;l.f452 = 0.0;let t46: f64 = (-1.0);let t47: f64 = if l.f1b5 == t46 { 1.0 } else { 0.0 };l.f80c = t47;l.f821 = 0.0;
        if (l.f80c != 0.0) {let t48: f64 = (0.3333333333333333 * l.f4e9);l.f450 = t48;l.f453 = 0.0;l.f451 = 0.3333333333333333;l.f452 = 0.0;}
        let t49: f64 = (-2.0);let t4a: f64 = (t49 / l.ffa);let t4b: f64 = (t4a + 1.0);let t4c: f64 = (2.0_f64).powf(t4b);let t4d: f64 = (t4c - 1.0);l.f16dc = t4d;l.f16fd = 0.0;let t4e: f64 = (l.f16dc - 1.0);let t4f: f64 = (l.f16dc - 1.0);let t50: f64 = (t4e * t4f);let t51: f64 = (4.0 * l.f16dc);
        let (t53,) = {
    if (t51 > 0.0001) {
        let t52: f64 = (4.0 * l.f16dc);
        (t52,)
    } else {
        (0.0001,)
    }
};
        let t54: f64 = (t50 / t53);l.fb8 = t54;l.fb9 = 0.0;let t55: f64 = (-2.0);let t56: f64 = (t55 / l.ffe);let t57: f64 = (t56 + 1.0);let t58: f64 = (2.0_f64).powf(t57);let t59: f64 = (t58 - 1.0);l.f16dc = t59;l.f16fd = 0.0;let t5a: f64 = (l.f16dc - 1.0);let t5b: f64 = (l.f16dc - 1.0);let t5c: f64 = (t5a * t5b);let t5d: f64 = (4.0 * l.f16dc);
        let (t5f,) = {
    if (t5d > 0.0001) {
        let t5e: f64 = (4.0 * l.f16dc);
        (t5e,)
    } else {
        (0.0001,)
    }
};
        let t60: f64 = (t5c / t5f);l.fba = t60;l.fbb = 0.0;let t61: f64 = (1.0 / l.f1a6c);l.fe3a = t61;l.fe3b = 0.0;let t62: f64 = (l.f41b / l.f17e5);l.f1e6 = t62;l.f1e9 = 0.0;let t63: f64 = (l.f41b / l.f17e9);l.f1e7 = t63;l.f1e8 = 0.0;let t64: f64 = (2.0 * 1.6021918e-19);let t65: f64 = (t64 * l.ffc2);let t66: f64 = (t65 * l.f421);let t67: f64 = (t66 * l.fe38);let t68: f64 = (t67).sqrt();let t69: f64 = (t68 / l.f1e6);l.f661 = t69;l.f662 = 0.0;let t6a: f64 = (2.0 * 1.6021918e-19);let t6b: f64 = (t6a * l.ffc6);let t6c: f64 = (t6b * l.f421);let t6d: f64 = (t6c * l.fe38);let t6e: f64 = (t6d).sqrt();let t6f: f64 = (t6e / l.f1e7);l.f65f = t6f;l.f660 = 0.0;let t70: f64 = (l.f661 * l.f661);l.f65d = t70;l.f65e = 0.0;let t71: f64 = (l.f65f * l.f65f);l.f65b = t71;l.f65c = 0.0;let t72: f64 = (l.f1a9 * 0.005);let t73: f64 = (t72 * l.fe38);let t74: f64 = (t73).exp();let t75: f64 = (t74 - 1.0);let t76: f64 = (t75).ln();let t77: f64 = (t76 / l.f1a9);let t78: f64 = (0.005 * l.fe38);let t79: f64 = (t78).exp();let t7a: f64 = (t79 - 1.0);let t7b: f64 = (t7a).ln();let t7c: f64 = (t77 - t7b);l.f3d5 = t7c;l.f3d6 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        l: &mut StampLocals,
    ) {
        let t7d: f64 = (0.5 * l.f661);let t7e: f64 = (t7d).ln();let t7f: f64 = (t7e + l.f3d5);l.f3d3 = t7f;l.f3d4 = 0.0;let t80: f64 = (0.5 * l.f65f);let t81: f64 = (t80).ln();let t82: f64 = (t81 + l.f3d5);l.f3d1 = t82;l.f3d2 = 0.0;let t83: f64 = (1.0 / l.f661);l.fe17 = t83;l.fe18 = 0.0;let t84: f64 = (3.1 * l.f661);let t85: f64 = (t84 + 8.5);l.f14bb = t85;l.f14c0 = 0.0;let t86: f64 = (l.f14bb * l.f14bb);l.f14be = t86;l.f14bf = 0.0;let t87: f64 = (0.5 * l.f14bb);l.f14b5 = t87;l.f14ba = 0.0;let t88: f64 = if l.fe17 < 0.06 { 1.0 } else { 0.0 };l.f822 = t88;l.f827 = 0.0;
        if (l.f822 != 0.0) {let t89: f64 = (64.0 * l.fe17);l.f14b3 = t89;l.f14b4 = 0.0;}
        let t8a: f64 = if l.fe17 <= 0.45 { 1.0 } else { 0.0 };l.f828 = t8a;l.f829 = 0.0;
        if ((l.f822 == 0.0) && (l.f828 != 0.0)) {let t8b: f64 = (22.0 * l.fe17);let t8c: f64 = (t8b + 3.0);l.f14b3 = t8c;l.f14b4 = 0.0;}
        let t8d: f64 = if l.fe17 <= 1.6 { 1.0 } else { 0.0 };l.f82a = t8d;l.f82b = 0.0;
        if (((l.f822 == 0.0) && (l.f828 == 0.0)) && (l.f82a != 0.0)) {let t8e: f64 = (-7.2);let t8f: f64 = (t8e * l.fe17);let t90: f64 = (t8f + 15.5);l.f14b3 = t90;l.f14b4 = 0.0;}
        if (((l.f822 == 0.0) && (l.f828 == 0.0)) && (l.f82a == 0.0)) {l.f14b3 = l.f661;l.f14b4 = 0.0;}
        let t91: f64 = (l.f65d * 0.5);let t92: f64 = (l.f14b5 + t91);let t93: f64 = (l.f65d * 0.25);let t94: f64 = (l.f14b5 + t93);let t95: f64 = (t94 + l.f14b3);let t96: f64 = (t95).sqrt();let t97: f64 = (l.f661 * t96);let t98: f64 = (t92 - t97);l.f14b8 = t98;l.f14b9 = 0.0;let t99: f64 = (1.0 / l.f65f);l.fe17 = t99;l.fe18 = 0.0;let t9a: f64 = (3.1 * l.f65f);let t9b: f64 = (t9a + 8.5);l.f14bb = t9b;l.f14c0 = 0.0;let t9c: f64 = (l.f14bb * l.f14bb);l.f14bc = t9c;l.f14bd = 0.0;let t9d: f64 = (0.5 * l.f14bb);l.f14b5 = t9d;l.f14ba = 0.0;let t9e: f64 = if l.fe17 < 0.06 { 1.0 } else { 0.0 };l.f82c = t9e;l.f82d = 0.0;
        if (l.f82c != 0.0) {let t9f: f64 = (64.0 * l.fe17);l.f14b1 = t9f;l.f14b2 = 0.0;}
        let ta0: f64 = if l.fe17 <= 0.45 { 1.0 } else { 0.0 };l.f82e = ta0;l.f831 = 0.0;
        if ((l.f82c == 0.0) && (l.f82e != 0.0)) {let ta1: f64 = (22.0 * l.fe17);let ta2: f64 = (ta1 + 3.0);l.f14b1 = ta2;l.f14b2 = 0.0;}
        let ta3: f64 = if l.fe17 <= 1.6 { 1.0 } else { 0.0 };l.f832 = ta3;l.f841 = 0.0;
        if (((l.f82c == 0.0) && (l.f82e == 0.0)) && (l.f832 != 0.0)) {let ta4: f64 = (-7.2);let ta5: f64 = (ta4 * l.fe17);let ta6: f64 = (ta5 + 15.5);l.f14b1 = ta6;l.f14b2 = 0.0;}
        if (((l.f82c == 0.0) && (l.f82e == 0.0)) && (l.f832 == 0.0)) {l.f14b1 = l.f65f;l.f14b2 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        l: &mut StampLocals,
    ) {
        let ta7: f64 = (l.f65b * 0.5);let ta8: f64 = (l.f14b5 + ta7);let ta9: f64 = (l.f65b * 0.25);let taa: f64 = (l.f14b5 + ta9);let tab: f64 = (taa + l.f14b1);let tac: f64 = (tab).sqrt();let tad: f64 = (l.f65f * tac);let tae: f64 = (ta8 - tad);l.f14b6 = tae;l.f14b7 = 0.0;let taf: f64 = (1.0 / l.f1b1);l.fdf9 = taf;l.fdfa = 0.0;let tb0: f64 = (4.0 * 0.3333333333333333);let tb1: f64 = (2.0 * 1.6021918e-19);let tb2: f64 = (tb1 * 9.1093826e-31);let tb3: f64 = (tb2 * l.f1b1);let tb4: f64 = (tb3).sqrt();let tb5: f64 = (tb0 * tb4);let tb6: f64 = (tb5 / 1.05457168e-34);l.f10a = tb6;l.f10b = 0.0;let tb7: f64 = (l.f10a * l.f17df);l.f10e = tb7;l.f10f = 0.0;let tb8: f64 = (l.f10a * l.f17e5);l.f13f = tb8;l.f142 = 0.0;let tb9: f64 = (l.f10a * l.f17e9);l.f140 = tb9;l.f141 = 0.0;l.f5a6 = 0.0;l.f5a7 = 0.0;let tba: f64 = if l.f596 < 0.0 { 1.0 } else { 0.0 };l.f842 = tba;l.f857 = 0.0;
        if (l.f842 != 0.0) {let tbb: f64 = (-0.495);let tbc: f64 = (tbb * l.f58a);let tbd: f64 = (tbc / l.f596);l.f5a6 = tbd;l.f5a7 = 0.0;}
        l.f5a8 = 0.0;l.f5a9 = 0.0;let tbe: f64 = if l.f59a < 0.0 { 1.0 } else { 0.0 };l.f858 = tbe;l.f86b = 0.0;
        if (l.f858 != 0.0) {let tbf: f64 = (-0.495);let tc0: f64 = (tbf * l.f58e);let tc1: f64 = (tc0 / l.f59a);l.f5a8 = tc1;l.f5a9 = 0.0;}
        let tc2: f64 = if l.f59e < 0.0 { 1.0 } else { 0.0 };l.f86c = tc2;l.f880 = 0.0;
        if (l.f86c != 0.0) {let tc3: f64 = (-0.495);let tc4: f64 = (tc3 * l.f592);let tc5: f64 = (tc4 / l.f59e);l.f5aa = tc5;l.f5ab = 0.0;}
        let tc6: f64 = (l.f1448).powf(l.f16a4);l.f171a = tc6;l.f171b = 0.0;let tc7: f64 = (l.fd70 * l.f171a);l.fd70 = tc7;l.fd71 = 0.0;let tc8: f64 = (l.fd74 * l.f171a);l.fd74 = tc8;l.fd75 = 0.0;let tc9: f64 = (l.fd78 * l.f171a);l.fd78 = tc9;l.fd79 = 0.0;let tca: f64 = (l.f168e * l.f279);let tcb: f64 = (1.0 + tca);
        let (tce,) = {
    if (tcb > 0.0) {
        let tcc: f64 = (l.f168e * l.f279);let tcd: f64 = (1.0 + tcc);
        (tcd,)
    } else {
        (0.0,)
    }
};
        l.f10a = tce;l.f10b = 0.0;let tcf: f64 = (l.f12f * l.f10a);l.f133 = tcf;l.f134 = 0.0;let td0: f64 = (l.f133 * l.f17e5);let td1: f64 = (td0 * 500000000.0);l.f13d = td1;l.f13e = 0.0;let td2: f64 = (l.f1692 * l.f279);let td3: f64 = (1.0 + td2);
        let (td6,) = {
    if (td3 > 0.0) {
        let td4: f64 = (l.f1692 * l.f279);let td5: f64 = (1.0 + td4);
        (td5,)
    } else {
        (0.0,)
    }
};
        l.f10a = td6;l.f10b = 0.0;let td7: f64 = (l.f135 * l.f10a);l.f139 = td7;l.f13a = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let td8: f64 = (l.f139 * l.f17e9);let td9: f64 = (td8 * 500000000.0);l.f13b = td9;l.f13c = 0.0;l.f19fc = 0.0;l.f19fd = 0.0;let tda: f64 = if l.f4df > 1e-10 { 1.0 } else { 0.0 };l.f881 = tda;l.f893 = 0.0;
        if (l.f881 != 0.0) {let tdb: f64 = (0.75 / l.f4df);l.f19fc = tdb;l.f19fd = 0.0;}
        let tdc: f64 = (l.f106 * l.f106);l.f33 = tdc;l.f34 = 0.0;let tdd: f64 = (l.f1b * l.fe57);l.f19 = tdd;l.f1a = 0.0;let tde: f64 = (l.ff0f * l.fe57);l.ff0d = tde;l.ff0e = 0.0;let tdf: f64 = (l.fefb * l.fe57);l.fef9 = tdf;l.fefa = 0.0;let te0: f64 = (l.f17 * l.fe57);l.f15 = te0;l.f16 = 0.0;let te1: f64 = (l.ff0b * l.fe57);l.ff09 = te1;l.ff0a = 0.0;let te2: f64 = (l.fef7 * l.fe57);l.fef5 = te2;l.fef6 = 0.0;l.fe95 = 0.0;l.fe96 = 0.0;let te3: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };l.f91e = te3;l.f931 = 0.0;
        if (l.f91e != 0.0) {l.fe95 = 1.0;l.fe96 = 0.0;}
        l.fe97 = l.f1afd;l.fe98 = 0.0;let te4: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };l.f932 = te4;l.f946 = 0.0;
        if (l.f932 != 0.0) {
            let (te5,) = {
    if (l.fe93 > 0.0) {
        (l.fe93,)
    } else {
        (0.0,)
    }
};
            l.fe97 = te5;l.fe98 = 0.0;
        }
        let te6: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };l.f947 = te6;l.f957 = 0.0;
        if (l.f947 != 0.0) {let te7: f64 = (l.fd0 * l.fe57);l.f19 = te7;l.f1a = 0.0;let te8: f64 = (l.f1122 * l.fe57);let te9: f64 = (l.fe95 * l.fe97);let tea: f64 = (te8 - te9);l.ff0d = tea;l.ff0e = 0.0;l.fef9 = l.fe97;l.fefa = 0.0;let teb: f64 = (l.f1d * l.fe57);l.f15 = teb;l.f16 = 0.0;let tec: f64 = (l.f1049 * l.fe57);let ted: f64 = (l.fe95 * l.fe97);let tee: f64 = (tec - ted);l.ff09 = tee;l.ff0a = 0.0;l.fef5 = l.fe97;l.fef6 = 0.0;}
        let tef: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };l.f958 = tef;l.f961 = 0.0;
        if (l.f958 != 0.0) {
            let (tf0,) = {
    if (l.f19 > 0.0) {
        (l.f19,)
    } else {
        (0.0,)
    }
};
            l.f1b = tf0;l.f1c = 0.0;
        }
        if (l.f958 != 0.0) {
            let (tf1,) = {
    if (l.ff0d > 0.0) {
        (l.ff0d,)
    } else {
        (0.0,)
    }
};
            l.ff0f = tf1;l.ff10 = 0.0;
        }
        if (l.f958 != 0.0) {
            let (tf2,) = {
    if (l.fef9 > 0.0) {
        (l.fef9,)
    } else {
        (0.0,)
    }
};
            l.fefb = tf2;l.fefc = 0.0;
        }
        if (l.f958 != 0.0) {
            let (tf3,) = {
    if (l.f15 > 0.0) {
        (l.f15,)
    } else {
        (0.0,)
    }
};
            l.f17 = tf3;l.f18 = 0.0;
        }
        if (l.f958 != 0.0) {
            let (tf4,) = {
    if (l.ff09 > 0.0) {
        (l.ff09,)
    } else {
        (0.0,)
    }
};
            l.ff0b = tf4;l.ff0c = 0.0;
        }
        if (l.f958 != 0.0) {
            let (tf5,) = {
    if (l.fef5 > 0.0) {
        (l.fef5,)
    } else {
        (0.0,)
    }
};
            l.fef7 = tf5;l.fef8 = 0.0;
        }
        if (l.f958 == 0.0) {l.f1b = 0.0;l.f1c = 0.0;l.ff0f = 0.0;l.ff10 = 0.0;}
    }
}
