#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t0: f64 = (4.0 * 1.3806226e-23);let t1: f64 = (t0 * l.f173f);let t2: f64 = t1;(l.f1a20, l.f1a21, ) = (t2, (t0 * l.f1740), );l.f1a22 = 0.0;let t3: f64 = (l.fb19 * l.fc73);(l.fc2f, l.fc30, l.fc34, l.fc35, l.fc36, l.fc37, l.fc38, l.fc31, l.fc32, l.fc33, ) = (t3, (l.fb19 * l.fc74), (l.fb19 * l.fc78), (l.fb19 * l.fc79), (l.fb19 * l.fc7a), (l.fb19 * l.fc7b), (l.fb19 * l.fc7c), (l.fb19 * l.fc75), (l.fb19 * l.fc76), (l.fb19 * l.fc77), );l.fc39 = 0.0;let t4: f64 = l.f12c7;(l.ff0, l.ff1, l.ff5, l.ff6, l.ff7, l.ff8, l.ff9, l.ff2, l.ff3, l.ff4, ) = (t4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.ffa = 0.0;let t5: f64 = (p.p33 * l.ff0);(l.ff0, l.ff1, l.ff5, l.ff6, l.ff7, l.ff8, l.ff9, l.ff2, l.ff3, l.ff4, ) = (t5, (p.p33 * l.ff1), (p.p33 * l.ff5), (p.p33 * l.ff6), (p.p33 * l.ff7), (p.p33 * l.ff8), (p.p33 * l.ff9), (p.p33 * l.ff2), (p.p33 * l.ff3), (p.p33 * l.ff4), );l.ffa = 0.0;let t6: f64 = l.f12c8;(l.f1f4, l.f1f5, l.f1f9, l.f1fa, l.f1fb, l.f1fc, l.f1fd, l.f1f6, l.f1f7, l.f1f8, ) = (t6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1fe = 0.0;let t7: f64 = (p.p33 * l.f1f4);(l.f1f4, l.f1f5, l.f1f9, l.f1fa, l.f1fb, l.f1fc, l.f1fd, l.f1f6, l.f1f7, l.f1f8, ) = (t7, (p.p33 * l.f1f5), (p.p33 * l.f1f9), (p.p33 * l.f1fa), (p.p33 * l.f1fb), (p.p33 * l.f1fc), (p.p33 * l.f1fd), (p.p33 * l.f1f6), (p.p33 * l.f1f7), (p.p33 * l.f1f8), );l.f1fe = 0.0;
        let (t8, t9, td, te, tf, t10, t11, ta, tb, tc,) = {
    if (l.fb55 > 0.0) {
        (l.f1f4, l.f1f5, l.f1f9, l.f1fa, l.f1fb, l.f1fc, l.f1fd, l.f1f6, l.f1f7, l.f1f8,)
    } else {
        (l.ff0, l.ff1, l.ff5, l.ff6, l.ff7, l.ff8, l.ff9, l.ff2, l.ff3, l.ff4,)
    }
};
        (l.f1e9, l.f1ea, l.f1ee, l.f1ef, l.f1f0, l.f1f1, l.f1f2, l.f1eb, l.f1ec, l.f1ed, ) = (t8, t9, td, te, tf, t10, t11, ta, tb, tc, );l.f1f3 = 0.0;let t12: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (l.f589 == 1.0)) && (l.f58b == 0.0)) { 1.0 } else { 0.0 };l.f833 = t12;l.f834 = 0.0;
        if (l.f833 != 0.0) {let t13: f64 = (1e-6 * l.fa2);let t14: f64 = (t13 * l.f1a06);let t15: f64 = (t14 * l.fad2);(l.f14d0, l.f14e7, l.f14eb, l.f14ec, l.f14ed, l.f14ee, l.f14ef, l.f14e8, l.f14e9, l.f14ea, ) = (t15, (((((1e-6 * l.fa7) * l.f1a06) + (t13 * l.f1a07)) * l.fad2) + (t14 * l.fad3)), (((((1e-6 * l.fab) * l.f1a06) + (t13 * l.f1a0b)) * l.fad2) + (t14 * l.fad7)), (((((1e-6 * l.fac) * l.f1a06) + (t13 * l.f1a0c)) * l.fad2) + (t14 * l.fad8)), (((((1e-6 * l.fad) * l.f1a06) + (t13 * l.f1a0d)) * l.fad2) + (t14 * l.fad9)), (((((1e-6 * l.fae) * l.f1a06) + (t13 * l.f1a0e)) * l.fad2) + (t14 * l.fada)), (((((1e-6 * l.faf) * l.f1a06) + (t13 * l.f1a0f)) * l.fad2) + (t14 * l.fadb)), (((((1e-6 * l.fa8) * l.f1a06) + (t13 * l.f1a08)) * l.fad2) + (t14 * l.fad4)), (((((1e-6 * l.fa9) * l.f1a06) + (t13 * l.f1a09)) * l.fad2) + (t14 * l.fad5)), (((((1e-6 * l.faa) * l.f1a06) + (t13 * l.f1a0a)) * l.fad2) + (t14 * l.fad6)), );l.f14f0 = 0.0;let t16: f64 = (l.f1e9 / l.fb19);(l.f14f2, l.f14fd, l.f1501, l.f1502, l.f1503, l.f1504, l.f1505, l.f14fe, l.f14ff, l.f1500, ) = (t16, (l.f1ea / l.fb19), (l.f1ee / l.fb19), (l.f1ef / l.fb19), (l.f1f0 / l.fb19), (l.f1f1 / l.fb19), (l.f1f2 / l.fb19), (l.f1eb / l.fb19), (l.f1ec / l.fb19), (l.f1ed / l.fb19), );l.f1506 = 0.0;let t17: f64 = (0.1185185185185185 * 1.6021918e-19);let t18: f64 = (t17 * l.f80);let t19: f64 = (t18 * l.f14f2);let t1a: f64 = (t19 * l.f14f2);let t1b: f64 = (t1a / l.f621);(l.fbf7, l.fbf8, l.fbfc, l.fbfd, l.fbfe, l.fbff, l.fc00, l.fbf9, l.fbfa, l.fbfb, ) = (t1b, ((((((t18 * l.f14fd) * l.f14f2) + (t19 * l.f14fd)) * l.f621) - (t1a * l.f622)) / (l.f621 * l.f621)), ((((((t18 * l.f1501) * l.f14f2) + (t19 * l.f1501)) * l.f621) - (t1a * l.f626)) / (l.f621 * l.f621)), ((((((((t17 * l.f81) * l.f14f2) + (t18 * l.f1502)) * l.f14f2) + (t19 * l.f1502)) * l.f621) - (t1a * l.f627)) / (l.f621 * l.f621)), ((((((t18 * l.f1503) * l.f14f2) + (t19 * l.f1503)) * l.f621) - (t1a * l.f628)) / (l.f621 * l.f621)), ((((((t18 * l.f1504) * l.f14f2) + (t19 * l.f1504)) * l.f621) - (t1a * l.f629)) / (l.f621 * l.f621)), ((((((t18 * l.f1505) * l.f14f2) + (t19 * l.f1505)) * l.f621) - (t1a * l.f62a)) / (l.f621 * l.f621)), ((((((t18 * l.f14fe) * l.f14f2) + (t19 * l.f14fe)) * l.f621) - (t1a * l.f623)) / (l.f621 * l.f621)), ((((((t18 * l.f14ff) * l.f14f2) + (t19 * l.f14ff)) * l.f621) - (t1a * l.f624)) / (l.f621 * l.f621)), ((((((t18 * l.f1500) * l.f14f2) + (t19 * l.f1500)) * l.f621) - (t1a * l.f625)) / (l.f621 * l.f621)), );l.fc01 = 0.0;}
        let t1c: f64 = (10.0 * 2.220446049250313e-16);let t1d: f64 = (10.0 * 2.220446049250313e-16);let t1e: f64 = if ((l.fa8c > t1c) && (l.f183d > t1d)) { 1.0 } else { 0.0 };l.f835 = t1e;l.f836 = 0.0;
        if ((l.f833 != 0.0) && (l.f835 != 0.0)) {let t1f: f64 = (l.fbd2 / l.fb5e);(l.fbbc, l.fbbd, l.fbc1, l.fbc2, l.fbc3, l.fbc4, l.fbc5, l.fbbe, l.fbbf, l.fbc0, ) = (t1f, (((l.fbd3 * l.fb5e) - (l.fbd2 * l.fb94)) / (l.fb5e * l.fb5e)), (((l.fbd7 * l.fb5e) - (l.fbd2 * l.fb98)) / (l.fb5e * l.fb5e)), (((l.fbd8 * l.fb5e) - (l.fbd2 * l.fb99)) / (l.fb5e * l.fb5e)), (((l.fbd9 * l.fb5e) - (l.fbd2 * l.fb9a)) / (l.fb5e * l.fb5e)), (((l.fbda * l.fb5e) - (l.fbd2 * l.fb9b)) / (l.fb5e * l.fb5e)), (((l.fbdb * l.fb5e) - (l.fbd2 * l.fb9c)) / (l.fb5e * l.fb5e)), (((l.fbd4 * l.fb5e) - (l.fbd2 * l.fb95)) / (l.fb5e * l.fb5e)), (((l.fbd5 * l.fb5e) - (l.fbd2 * l.fb96)) / (l.fb5e * l.fb5e)), (((l.fbd6 * l.fb5e) - (l.fbd2 * l.fb97)) / (l.fb5e * l.fb5e)), );l.fbc6 = 0.0;let t20: f64 = (l.fbd2 / l.fba9);let t21: f64 = (t20 - l.fbbc);let t22: f64 = (t21 / l.f183d);(l.fbc7, l.fbc8, l.fbcc, l.fbcd, l.fbce, l.fbcf, l.fbd0, l.fbc9, l.fbca, l.fbcb, ) = (t22, (((((((l.fbd3 * l.fba9) - (l.fbd2 * l.fbaa)) / (l.fba9 * l.fba9)) - l.fbbd) * l.f183d) - (t21 * l.f183e)) / (l.f183d * l.f183d)), (((((((l.fbd7 * l.fba9) - (l.fbd2 * l.fbae)) / (l.fba9 * l.fba9)) - l.fbc1) * l.f183d) - (t21 * l.f1842)) / (l.f183d * l.f183d)), (((((((l.fbd8 * l.fba9) - (l.fbd2 * l.fbaf)) / (l.fba9 * l.fba9)) - l.fbc2) * l.f183d) - (t21 * l.f1843)) / (l.f183d * l.f183d)), (((((((l.fbd9 * l.fba9) - (l.fbd2 * l.fbb0)) / (l.fba9 * l.fba9)) - l.fbc3) * l.f183d) - (t21 * l.f1844)) / (l.f183d * l.f183d)), (((((((l.fbda * l.fba9) - (l.fbd2 * l.fbb1)) / (l.fba9 * l.fba9)) - l.fbc4) * l.f183d) - (t21 * l.f1845)) / (l.f183d * l.f183d)), (((((((l.fbdb * l.fba9) - (l.fbd2 * l.fbb2)) / (l.fba9 * l.fba9)) - l.fbc5) * l.f183d) - (t21 * l.f1846)) / (l.f183d * l.f183d)), (((((((l.fbd4 * l.fba9) - (l.fbd2 * l.fbab)) / (l.fba9 * l.fba9)) - l.fbbe) * l.f183d) - (t21 * l.f183f)) / (l.f183d * l.f183d)), (((((((l.fbd5 * l.fba9) - (l.fbd2 * l.fbac)) / (l.fba9 * l.fba9)) - l.fbbf) * l.f183d) - (t21 * l.f1840)) / (l.f183d * l.f183d)), (((((((l.fbd6 * l.fba9) - (l.fbd2 * l.fbad)) / (l.fba9 * l.fba9)) - l.fbc0) * l.f183d) - (t21 * l.f1841)) / (l.f183d * l.f183d)), );l.fbd1 = 0.0;}
        if ((l.f833 != 0.0) && (l.f835 != 0.0)) {let t23: f64 = (0.6666666666666667 * l.fbc7);let t24: f64 = (l.f192b * l.f14c3);let t25: f64 = (l.fa81 + t24);let t26: f64 = (t25 + l.faad);let t27: f64 = (t23 * t26);let t28: f64 = (l.f192b + l.f14c3);let t29: f64 = (t27 / t28);let t2a: f64 = (l.fbbc + t29);(l.f295, l.f296, l.f29a, l.f29b, l.f29c, l.f29d, l.f29e, l.f297, l.f298, l.f299, ) = (t2a, (l.fbbd + ((((((0.6666666666666667 * l.fbc8) * t26) + (t23 * ((l.fa82 + ((l.f192c * l.f14c3) + (l.f192b * l.f14c4))) + l.faae))) * t28) - (t27 * (l.f192c + l.f14c4))) / (t28 * t28))), (l.fbc1 + ((((((0.6666666666666667 * l.fbcc) * t26) + (t23 * ((l.fa86 + ((l.f1930 * l.f14c3) + (l.f192b * l.f14c8))) + l.fab2))) * t28) - (t27 * (l.f1930 + l.f14c8))) / (t28 * t28))), (l.fbc2 + ((((((0.6666666666666667 * l.fbcd) * t26) + (t23 * ((l.fa87 + ((l.f1931 * l.f14c3) + (l.f192b * l.f14c9))) + l.fab3))) * t28) - (t27 * (l.f1931 + l.f14c9))) / (t28 * t28))), (l.fbc3 + ((((((0.6666666666666667 * l.fbce) * t26) + (t23 * ((l.fa88 + ((l.f1932 * l.f14c3) + (l.f192b * l.f14ca))) + l.fab4))) * t28) - (t27 * (l.f1932 + l.f14ca))) / (t28 * t28))), (l.fbc4 + ((((((0.6666666666666667 * l.fbcf) * t26) + (t23 * ((l.fa89 + ((l.f1933 * l.f14c3) + (l.f192b * l.f14cb))) + l.fab5))) * t28) - (t27 * (l.f1933 + l.f14cb))) / (t28 * t28))), (l.fbc5 + ((((((0.6666666666666667 * l.fbd0) * t26) + (t23 * ((l.fa8a + ((l.f1934 * l.f14c3) + (l.f192b * l.f14cc))) + l.fab6))) * t28) - (t27 * (l.f1934 + l.f14cc))) / (t28 * t28))), (l.fbbe + ((((((0.6666666666666667 * l.fbc9) * t26) + (t23 * ((l.fa83 + ((l.f192d * l.f14c3) + (l.f192b * l.f14c5))) + l.faaf))) * t28) - (t27 * (l.f192d + l.f14c5))) / (t28 * t28))), (l.fbbf + ((((((0.6666666666666667 * l.fbca) * t26) + (t23 * ((l.fa84 + ((l.f192e * l.f14c3) + (l.f192b * l.f14c6))) + l.fab0))) * t28) - (t27 * (l.f192e + l.f14c6))) / (t28 * t28))), (l.fbc0 + ((((((0.6666666666666667 * l.fbcb) * t26) + (t23 * ((l.fa85 + ((l.f192f * l.f14c3) + (l.f192b * l.f14c7))) + l.fab1))) * t28) - (t27 * (l.f192f + l.f14c7))) / (t28 * t28))), );l.f29f = 0.0;}
        if ((l.f833 != 0.0) && (l.f835 == 0.0)) {let t2b: f64 = (l.fbd2 / l.fba9);(l.f295, l.f296, l.f29a, l.f29b, l.f29c, l.f29d, l.f29e, l.f297, l.f298, l.f299, ) = (t2b, (((l.fbd3 * l.fba9) - (l.fbd2 * l.fbaa)) / (l.fba9 * l.fba9)), (((l.fbd7 * l.fba9) - (l.fbd2 * l.fbae)) / (l.fba9 * l.fba9)), (((l.fbd8 * l.fba9) - (l.fbd2 * l.fbaf)) / (l.fba9 * l.fba9)), (((l.fbd9 * l.fba9) - (l.fbd2 * l.fbb0)) / (l.fba9 * l.fba9)), (((l.fbda * l.fba9) - (l.fbd2 * l.fbb1)) / (l.fba9 * l.fba9)), (((l.fbdb * l.fba9) - (l.fbd2 * l.fbb2)) / (l.fba9 * l.fba9)), (((l.fbd4 * l.fba9) - (l.fbd2 * l.fbab)) / (l.fba9 * l.fba9)), (((l.fbd5 * l.fba9) - (l.fbd2 * l.fbac)) / (l.fba9 * l.fba9)), (((l.fbd6 * l.fba9) - (l.fbd2 * l.fbad)) / (l.fba9 * l.fba9)), );l.f29f = 0.0;}
        if (l.f833 != 0.0) {let t2c: f64 = (l.fb19 * l.fbf7);let t2d: f64 = (t2c * l.fa97);let t2e: f64 = (t2d * l.f295);(l.fc24, l.fc25, l.fc29, l.fc2a, l.fc2b, l.fc2c, l.fc2d, l.fc26, l.fc27, l.fc28, ) = (t2e, (((((l.fb19 * l.fbf8) * l.fa97) + (t2c * l.fa98)) * l.f295) + (t2d * l.f296)), (((((l.fb19 * l.fbfc) * l.fa97) + (t2c * l.fa9c)) * l.f295) + (t2d * l.f29a)), (((((l.fb19 * l.fbfd) * l.fa97) + (t2c * l.fa9d)) * l.f295) + (t2d * l.f29b)), (((((l.fb19 * l.fbfe) * l.fa97) + (t2c * l.fa9e)) * l.f295) + (t2d * l.f29c)), (((((l.fb19 * l.fbff) * l.fa97) + (t2c * l.fa9f)) * l.f295) + (t2d * l.f29d)), (((((l.fb19 * l.fc00) * l.fa97) + (t2c * l.faa0)) * l.f295) + (t2d * l.f29e)), (((((l.fb19 * l.fbf9) * l.fa97) + (t2c * l.fa99)) * l.f295) + (t2d * l.f297)), (((((l.fb19 * l.fbfa) * l.fa97) + (t2c * l.fa9a)) * l.f295) + (t2d * l.f298)), (((((l.fb19 * l.fbfb) * l.fa97) + (t2c * l.fa9b)) * l.f295) + (t2d * l.f299)), );l.fc2e = 0.0;}
        if (l.f833 != 0.0) {
            let (t2f, t30, t34, t35, t36, t37, t38, t31, t32, t33,) = {
    if (l.fc24 < 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (l.fc24, l.fc25, l.fc29, l.fc2a, l.fc2b, l.fc2c, l.fc2d, l.fc26, l.fc27, l.fc28,)
    }
};
            (l.fc24, l.fc25, l.fc29, l.fc2a, l.fc2b, l.fc2c, l.fc2d, l.fc26, l.fc27, l.fc28, ) = (t2f, t30, t34, t35, t36, t37, t38, t31, t32, t33, );l.fc2e = 0.0;
        }
        if (l.f833 != 0.0) {
            let t39: f64 = (-l.f14f2);
            let (t3a, t3b, t3f, t40, t41, t42, t43, t3c, t3d, t3e,) = {
    if (t39 > l.f14d0) {
        (l.fc24, l.fc25, l.fc29, l.fc2a, l.fc2b, l.fc2c, l.fc2d, l.fc26, l.fc27, l.fc28,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (l.fc24, l.fc25, l.fc29, l.fc2a, l.fc2b, l.fc2c, l.fc2d, l.fc26, l.fc27, l.fc28, ) = (t3a, t3b, t3f, t40, t41, t42, t43, t3c, t3d, t3e, );l.fc2e = 0.0;
        }
        if (l.f833 == 0.0) {(l.fc24, l.fc25, l.fc29, l.fc2a, l.fc2b, l.fc2c, l.fc2d, l.fc26, l.fc27, l.fc28, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.fc2e = 0.0;}
        let t44: f64 = (l.f1a20 * l.fc2f);(l.f1476, l.f1477, l.f147b, l.f147c, l.f147d, l.f147e, l.f147f, l.f1478, l.f1479, l.f147a, ) = (t44, (l.f1a20 * l.fc30), (l.f1a20 * l.fc34), ((l.f1a21 * l.fc2f) + (l.f1a20 * l.fc35)), (l.f1a20 * l.fc36), (l.f1a20 * l.fc37), (l.f1a20 * l.fc38), (l.f1a20 * l.fc31), (l.f1a20 * l.fc32), (l.f1a20 * l.fc33), );l.f1480 = 0.0;
        let (t47, t48, t4c, t4d, t4e, t4f, t50, t49, t4a, t4b,) = {
    if ((l.f1476 > 0.0) && (l.fc24 > 0.0)) {
        let t45: f64 = (l.fc24 / l.f1476);let t46: f64 = (t45).sqrt();
        (t46, ((((l.fc25 * l.f1476) - (l.fc24 * l.f1477)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc29 * l.f1476) - (l.fc24 * l.f147b)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc2a * l.f1476) - (l.fc24 * l.f147c)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc2b * l.f1476) - (l.fc24 * l.f147d)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc2c * l.f1476) - (l.fc24 * l.f147e)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc2d * l.f1476) - (l.fc24 * l.f147f)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc26 * l.f1476) - (l.fc24 * l.f1478)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc27 * l.f1476) - (l.fc24 * l.f1479)) / (l.f1476 * l.f1476)) / (2.0 * t46)), ((((l.fc28 * l.f1476) - (l.fc24 * l.f147a)) / (l.f1476 * l.f1476)) / (2.0 * t46)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        (l.f1481, l.f148d, l.f1491, l.f1492, l.f1493, l.f1494, l.f1495, l.f148e, l.f148f, l.f1490, ) = (t47, t48, t4c, t4d, t4e, t4f, t50, t49, t4a, t4b, );l.f1496 = 0.0;
        let (t54, t55, t59, t5a, t5b, t5c, t5d, t56, t57, t58,) = {
    if (l.fb55 > 0.0) {
        let t51: f64 = (1.0 - l.f1287);let t52: f64 = (l.f1481 * t51);
        (t52, (l.f148d * t51), (l.f1491 * t51), (l.f1492 * t51), (l.f1493 * t51), (l.f1494 * t51), (l.f1495 * t51), (l.f148e * t51), (l.f148f * t51), (l.f1490 * t51),)
    } else {
        let t53: f64 = (l.f1481 * l.f1287);
        (t53, (l.f148d * l.f1287), (l.f1491 * l.f1287), (l.f1492 * l.f1287), (l.f1493 * l.f1287), (l.f1494 * l.f1287), (l.f1495 * l.f1287), (l.f148e * l.f1287), (l.f148f * l.f1287), (l.f1490 * l.f1287),)
    }
};
        (l.f1497, l.f1498, l.f149c, l.f149d, l.f149e, l.f149f, l.f14a0, l.f1499, l.f149a, l.f149b, ) = (t54, t55, t59, t5a, t5b, t5c, t5d, t56, t57, t58, );l.f14a1 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv12 = ctx.node_voltage(nodes[12]);
        let (t61, t62, t66, t67, t68, t69, t6a, t63, t64, t65,) = {
    if (l.fb55 > 0.0) {
        let t5e: f64 = (l.f1481 * l.f1287);
        (t5e, (l.f148d * l.f1287), (l.f1491 * l.f1287), (l.f1492 * l.f1287), (l.f1493 * l.f1287), (l.f1494 * l.f1287), (l.f1495 * l.f1287), (l.f148e * l.f1287), (l.f148f * l.f1287), (l.f1490 * l.f1287),)
    } else {
        let t5f: f64 = (1.0 - l.f1287);let t60: f64 = (l.f1481 * t5f);
        (t60, (l.f148d * t5f), (l.f1491 * t5f), (l.f1492 * t5f), (l.f1493 * t5f), (l.f1494 * t5f), (l.f1495 * t5f), (l.f148e * t5f), (l.f148f * t5f), (l.f1490 * t5f),)
    }
};
        (l.f1482, l.f1483, l.f1487, l.f1488, l.f1489, l.f148a, l.f148b, l.f1484, l.f1485, l.f1486, ) = (t61, t62, t66, t67, t68, t69, t6a, t63, t64, t65, );l.f148c = 0.0;let t6b: f64 = if p.p312 == 1.0 { 1.0 } else { 0.0 };l.f839 = t6b;l.f83a = 0.0;
        if (l.f839 != 0.0) {l.fb45 = p.p317;l.fb48 = 0.0;l.fb49 = p.p319;l.fb4c = 0.0;(l.f1416, l.f141a, ) = (p.p324, 0.0, );l.f141b = 0.0;l.face = p.p311;l.fad1 = 0.0;let t6c: f64 = (p.p33 * (nv12 - nv2));(l.f196d, l.f1973, l.f1972, ) = (t6c, (-p.p33), p.p33, );l.f1974 = 0.0;let t6d: f64 = (l.fb45 / 10000.0);l.fb45 = t6d;l.fb48 = 0.0;let t6e: f64 = (l.fb49 / 100.0);l.fb49 = t6e;l.fb4c = 0.0;let t6f: f64 = (l.f173f / l.f17aa);(l.f1739, l.f173d, ) = (t6f, (l.f1740 / l.f17aa), );l.f173e = 0.0;}
        if (l.f839 != 0.0) {let t70: f64 = (l.f1739).powf(p.p320);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (t70, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((l.f1739).powf(p.p320 - 1.0) * l.f173d)) } } else { (t70 * (p.p320 * (l.f173d / l.f1739))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1576 = 0.0;}
        if (l.f839 != 0.0) {let t71: f64 = (l.fb45 / l.f14f1);(l.fb5f, l.fb6b, l.fb6f, l.fb70, l.fb71, l.fb72, l.fb73, l.fb6c, l.fb6d, l.fb6e, ) = (t71, (-((l.fb45 * l.f1557) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f155b) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f155c) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f155d) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f155e) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f155f) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f1558) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f1559) / (l.f14f1 * l.f14f1))), (-((l.fb45 * l.f155a) / (l.f14f1 * l.f14f1))), );l.fb74 = 0.0;let t72: f64 = (0.4 * l.f1739);let t73: f64 = (1.8 + t72);let t74: f64 = (0.1 * l.f1739);let t75: f64 = (t74 * l.f1739);let t76: f64 = (t73 + t75);let t77: f64 = (1.0 - l.f1739);let t78: f64 = (p.p321 * t77);let t79: f64 = (t76 - t78);(l.f14d0, l.f14e7, l.f14eb, l.f14ec, l.f14ed, l.f14ee, l.f14ef, l.f14e8, l.f14e9, l.f14ea, ) = (t79, 0.0, 0.0, (((0.4 * l.f173d) + (((0.1 * l.f173d) * l.f1739) + (t74 * l.f173d))) - (p.p321 * (-l.f173d))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f14f0 = 0.0;let t7a: f64 = (l.fb49 / l.f14d0);(l.f1942, l.f1943, l.f1947, l.f1948, l.f1949, l.f194a, l.f194b, l.f1944, l.f1945, l.f1946, ) = (t7a, (-((l.fb49 * l.f14e7) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14eb) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14ec) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14ed) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14ee) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14ef) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14e8) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14e9) / (l.f14d0 * l.f14d0))), (-((l.fb49 * l.f14ea) / (l.f14d0 * l.f14d0))), );l.f194c = 0.0;let t7b: f64 = (l.f173f - l.f17aa);let t7c: f64 = (p.p325 * t7b);let t7d: f64 = (l.f1416 + t7c);(l.f1416, l.f141a, ) = (t7d, (l.f141a + (p.p325 * l.f1740)), );l.f141b = 0.0;let t7e: f64 = (l.fadd).powf(p.p331);let t7f: f64 = (p.p330 / t7e);let t80: f64 = (1.0 + t7f);l.f13f0 = t80;l.f13f3 = 0.0;let t81: f64 = (l.fadd).powf(p.p329);let t82: f64 = (p.p328 / t81);let t83: f64 = (1.0 + t82);l.f13f4 = t83;l.f13f7 = 0.0;let t84: f64 = (l.f1a11).powf(p.p327);let t85: f64 = (p.p326 / t84);let t86: f64 = (1.0 + t85);l.f13f8 = t86;l.f13fb = 0.0;let t87: f64 = (l.fb5f * l.f13f0);(l.fb5f, l.fb6b, l.fb6f, l.fb70, l.fb71, l.fb72, l.fb73, l.fb6c, l.fb6d, l.fb6e, ) = (t87, (l.fb6b * l.f13f0), (l.fb6f * l.f13f0), (l.fb70 * l.f13f0), (l.fb71 * l.f13f0), (l.fb72 * l.f13f0), (l.fb73 * l.f13f0), (l.fb6c * l.f13f0), (l.fb6d * l.f13f0), (l.fb6e * l.f13f0), );l.fb74 = 0.0;let t88: f64 = (l.f1942 * l.f13f8);let t89: f64 = (t88 * l.f13f4);let t8a: f64 = (t89 + 1e-50);(l.f1942, l.f1943, l.f1947, l.f1948, l.f1949, l.f194a, l.f194b, l.f1944, l.f1945, l.f1946, ) = (t8a, ((l.f1943 * l.f13f8) * l.f13f4), ((l.f1947 * l.f13f8) * l.f13f4), ((l.f1948 * l.f13f8) * l.f13f4), ((l.f1949 * l.f13f8) * l.f13f4), ((l.f194a * l.f13f8) * l.f13f4), ((l.f194b * l.f13f8) * l.f13f4), ((l.f1944 * l.f13f8) * l.f13f4), ((l.f1945 * l.f13f8) * l.f13f4), ((l.f1946 * l.f13f8) * l.f13f4), );l.f194c = 0.0;let t8b: f64 = (l.f196d / l.face);(l.f44e, l.f454, l.f453, ) = (t8b, (l.f1973 / l.face), (l.f1972 / l.face), );l.f455 = 0.0;}
        if (l.f839 != 0.0) {let t8c: f64 = (l.fb5f * l.f44e);(l.f1827, l.f1833, l.f1837, l.f1838, l.f1839, l.f183a, l.f183b, l.f1834, l.f1835, l.f1836, ) = (t8c, (l.fb6b * l.f44e), ((l.fb6f * l.f44e) + (l.fb5f * l.f454)), (l.fb70 * l.f44e), (l.fb71 * l.f44e), (l.fb72 * l.f44e), (l.fb73 * l.f44e), (l.fb6c * l.f44e), (l.fb6d * l.f44e), ((l.fb6e * l.f44e) + (l.fb5f * l.f453)), );l.f183c = 0.0;}
        let t8d: f64 = if l.f196d >= 0.0 { 1.0 } else { 0.0 };l.f83f = t8d;l.f840 = 0.0;
        if ((l.f839 != 0.0) && (l.f83f != 0.0)) {let t8e: f64 = (l.f1827 / l.f1942);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (t8e, (((l.f1833 * l.f1942) - (l.f1827 * l.f1943)) / (l.f1942 * l.f1942)), (((l.f1837 * l.f1942) - (l.f1827 * l.f1947)) / (l.f1942 * l.f1942)), (((l.f1838 * l.f1942) - (l.f1827 * l.f1948)) / (l.f1942 * l.f1942)), (((l.f1839 * l.f1942) - (l.f1827 * l.f1949)) / (l.f1942 * l.f1942)), (((l.f183a * l.f1942) - (l.f1827 * l.f194a)) / (l.f1942 * l.f1942)), (((l.f183b * l.f1942) - (l.f1827 * l.f194b)) / (l.f1942 * l.f1942)), (((l.f1834 * l.f1942) - (l.f1827 * l.f1944)) / (l.f1942 * l.f1942)), (((l.f1835 * l.f1942) - (l.f1827 * l.f1945)) / (l.f1942 * l.f1942)), (((l.f1836 * l.f1942) - (l.f1827 * l.f1946)) / (l.f1942 * l.f1942)), );l.f1576 = 0.0;}
        if ((l.f839 != 0.0) && (l.f83f == 0.0)) {let t8f: f64 = (-l.f1827);let t90: f64 = (t8f / l.f1942);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (t90, ((((-l.f1833) * l.f1942) - (t8f * l.f1943)) / (l.f1942 * l.f1942)), ((((-l.f1837) * l.f1942) - (t8f * l.f1947)) / (l.f1942 * l.f1942)), ((((-l.f1838) * l.f1942) - (t8f * l.f1948)) / (l.f1942 * l.f1942)), ((((-l.f1839) * l.f1942) - (t8f * l.f1949)) / (l.f1942 * l.f1942)), ((((-l.f183a) * l.f1942) - (t8f * l.f194a)) / (l.f1942 * l.f1942)), ((((-l.f183b) * l.f1942) - (t8f * l.f194b)) / (l.f1942 * l.f1942)), ((((-l.f1834) * l.f1942) - (t8f * l.f1944)) / (l.f1942 * l.f1942)), ((((-l.f1835) * l.f1942) - (t8f * l.f1945)) / (l.f1942 * l.f1942)), ((((-l.f1836) * l.f1942) - (t8f * l.f1946)) / (l.f1942 * l.f1942)), );l.f1576 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv11 = ctx.node_voltage(nodes[11]);let t91: f64 = (10.0 * 2.220446049250313e-16);let t92: f64 = (1.0 - t91);let t93: f64 = (10.0 * 2.220446049250313e-16);let t94: f64 = (1.0 + t93);let t95: f64 = if ((t92 <= l.f1416) && (l.f1416 <= t94)) { 1.0 } else { 0.0 };l.f841 = t95;l.f842 = 0.0;
        if ((l.f839 != 0.0) && (l.f841 != 0.0)) {(l.f15b0, l.f15d2, l.f15d6, l.f15d7, l.f15d8, l.f15d9, l.f15da, l.f15d3, l.f15d4, l.f15d5, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f15db = 0.0;}
        let t96: f64 = (10.0 * 2.220446049250313e-16);let t97: f64 = (2.0 - t96);let t98: f64 = (10.0 * 2.220446049250313e-16);let t99: f64 = (2.0 + t98);let t9a: f64 = if ((t97 <= l.f1416) && (l.f1416 <= t99)) { 1.0 } else { 0.0 };l.f843 = t9a;l.f844 = 0.0;
        if (((l.f839 != 0.0) && (l.f841 == 0.0)) && (l.f843 != 0.0)) {(l.f15b0, l.f15d2, l.f15d6, l.f15d7, l.f15d8, l.f15d9, l.f15da, l.f15d3, l.f15d4, l.f15d5, ) = (l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, );l.f15db = 0.0;}
        if (((l.f839 != 0.0) && (l.f841 == 0.0)) && (l.f843 == 0.0)) {let t9b: f64 = (l.f1416 - 1.0);let t9c: f64 = (l.f14f1).powf(t9b);(l.f15b0, l.f15d2, l.f15d6, l.f15d7, l.f15d8, l.f15d9, l.f15da, l.f15d3, l.f15d4, l.f15d5, ) = (t9c, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f1557)) } } else { (t9c * (t9b * (l.f1557 / l.f14f1))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f155b)) } } else { (t9c * (t9b * (l.f155b / l.f14f1))) }, if l.f141a == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f155c)) } } else { (t9c * ((l.f141a * (l.f14f1).ln()) + (t9b * (l.f155c / l.f14f1)))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f155d)) } } else { (t9c * (t9b * (l.f155d / l.f14f1))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f155e)) } } else { (t9c * (t9b * (l.f155e / l.f14f1))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f155f)) } } else { (t9c * (t9b * (l.f155f / l.f14f1))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f1558)) } } else { (t9c * (t9b * (l.f1558 / l.f14f1))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f1559)) } } else { (t9c * (t9b * (l.f1559 / l.f14f1))) }, if 0.0 == 0.0 && ((t9b) as f64).is_finite() && ((t9b) as f64).fract() == 0.0 { if t9b == 0.0 { 0.0 } else { (t9b * ((l.f14f1).powf(t9b - 1.0) * l.f155a)) } } else { (t9c * (t9b * (l.f155a / l.f14f1))) }, );l.f15db = 0.0;}
        if (l.f839 != 0.0) {let t9d: f64 = (l.f14f1 * l.f15b0);(l.f1577, l.f1599, l.f159d, l.f159e, l.f159f, l.f15a0, l.f15a1, l.f159a, l.f159b, l.f159c, ) = (t9d, ((l.f1557 * l.f15b0) + (l.f14f1 * l.f15d2)), ((l.f155b * l.f15b0) + (l.f14f1 * l.f15d6)), ((l.f155c * l.f15b0) + (l.f14f1 * l.f15d7)), ((l.f155d * l.f15b0) + (l.f14f1 * l.f15d8)), ((l.f155e * l.f15b0) + (l.f14f1 * l.f15d9)), ((l.f155f * l.f15b0) + (l.f14f1 * l.f15da)), ((l.f1558 * l.f15b0) + (l.f14f1 * l.f15d3)), ((l.f1559 * l.f15b0) + (l.f14f1 * l.f15d4)), ((l.f155a * l.f15b0) + (l.f14f1 * l.f15d5)), );l.f15af = 0.0;let t9e: f64 = (1.0 + l.f1577);(l.f15dc, l.f15e8, l.f15ec, l.f15ed, l.f15ee, l.f15ef, l.f15f0, l.f15e9, l.f15ea, l.f15eb, ) = (t9e, l.f1599, l.f159d, l.f159e, l.f159f, l.f15a0, l.f15a1, l.f159a, l.f159b, l.f159c, );l.f1612 = 0.0;}
        let t9f: f64 = (10.0 * 2.220446049250313e-16);let ta0: f64 = (1.0 - t9f);let ta1: f64 = (10.0 * 2.220446049250313e-16);let ta2: f64 = (1.0 + ta1);let ta3: f64 = if ((ta0 <= l.f1416) && (l.f1416 <= ta2)) { 1.0 } else { 0.0 };l.f845 = ta3;l.f846 = 0.0;
        if ((l.f839 != 0.0) && (l.f845 != 0.0)) {let ta4: f64 = (1.0 / l.f15dc);(l.f1613, l.f162a, l.f162e, l.f162f, l.f1630, l.f1631, l.f1632, l.f162b, l.f162c, l.f162d, ) = (ta4, (-(l.f15e8 / (l.f15dc * l.f15dc))), (-(l.f15ec / (l.f15dc * l.f15dc))), (-(l.f15ed / (l.f15dc * l.f15dc))), (-(l.f15ee / (l.f15dc * l.f15dc))), (-(l.f15ef / (l.f15dc * l.f15dc))), (-(l.f15f0 / (l.f15dc * l.f15dc))), (-(l.f15e9 / (l.f15dc * l.f15dc))), (-(l.f15ea / (l.f15dc * l.f15dc))), (-(l.f15eb / (l.f15dc * l.f15dc))), );l.f1633 = 0.0;}
        let ta5: f64 = (10.0 * 2.220446049250313e-16);let ta6: f64 = (2.0 - ta5);let ta7: f64 = (10.0 * 2.220446049250313e-16);let ta8: f64 = (2.0 + ta7);let ta9: f64 = if ((ta6 <= l.f1416) && (l.f1416 <= ta8)) { 1.0 } else { 0.0 };l.f847 = ta9;l.f848 = 0.0;
        if (((l.f839 != 0.0) && (l.f845 == 0.0)) && (l.f847 != 0.0)) {let taa: f64 = (l.f15dc).sqrt();let tab: f64 = (1.0 / taa);(l.f1613, l.f162a, l.f162e, l.f162f, l.f1630, l.f1631, l.f1632, l.f162b, l.f162c, l.f162d, ) = (tab, (-((l.f15e8 / (2.0 * taa)) / (taa * taa))), (-((l.f15ec / (2.0 * taa)) / (taa * taa))), (-((l.f15ed / (2.0 * taa)) / (taa * taa))), (-((l.f15ee / (2.0 * taa)) / (taa * taa))), (-((l.f15ef / (2.0 * taa)) / (taa * taa))), (-((l.f15f0 / (2.0 * taa)) / (taa * taa))), (-((l.f15e9 / (2.0 * taa)) / (taa * taa))), (-((l.f15ea / (2.0 * taa)) / (taa * taa))), (-((l.f15eb / (2.0 * taa)) / (taa * taa))), );l.f1633 = 0.0;}
        if (((l.f839 != 0.0) && (l.f845 == 0.0)) && (l.f847 == 0.0)) {let tac: f64 = (-1.0);let tad: f64 = (tac / l.f1416);let tae: f64 = (tad - 1.0);let taf: f64 = (l.f15dc).powf(tae);(l.f1634, l.f164a, l.f164e, l.f164f, l.f1650, l.f1651, l.f1652, l.f164b, l.f164c, l.f164d, ) = (taf, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15e8)) } } else { (taf * (tae * (l.f15e8 / l.f15dc))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15ec)) } } else { (taf * (tae * (l.f15ec / l.f15dc))) }, if (-((tac * l.f141a) / (l.f1416 * l.f1416))) == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15ed)) } } else { (taf * (((-((tac * l.f141a) / (l.f1416 * l.f1416))) * (l.f15dc).ln()) + (tae * (l.f15ed / l.f15dc)))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15ee)) } } else { (taf * (tae * (l.f15ee / l.f15dc))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15ef)) } } else { (taf * (tae * (l.f15ef / l.f15dc))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15f0)) } } else { (taf * (tae * (l.f15f0 / l.f15dc))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15e9)) } } else { (taf * (tae * (l.f15e9 / l.f15dc))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15ea)) } } else { (taf * (tae * (l.f15ea / l.f15dc))) }, if 0.0 == 0.0 && ((tae) as f64).is_finite() && ((tae) as f64).fract() == 0.0 { if tae == 0.0 { 0.0 } else { (tae * ((l.f15dc).powf(tae - 1.0) * l.f15eb)) } } else { (taf * (tae * (l.f15eb / l.f15dc))) }, );l.f1653 = 0.0;}
        if (((l.f839 != 0.0) && (l.f845 == 0.0)) && (l.f847 == 0.0)) {let tb0: f64 = (l.f15dc * l.f1634);(l.f1613, l.f162a, l.f162e, l.f162f, l.f1630, l.f1631, l.f1632, l.f162b, l.f162c, l.f162d, ) = (tb0, ((l.f15e8 * l.f1634) + (l.f15dc * l.f164a)), ((l.f15ec * l.f1634) + (l.f15dc * l.f164e)), ((l.f15ed * l.f1634) + (l.f15dc * l.f164f)), ((l.f15ee * l.f1634) + (l.f15dc * l.f1650)), ((l.f15ef * l.f1634) + (l.f15dc * l.f1651)), ((l.f15f0 * l.f1634) + (l.f15dc * l.f1652)), ((l.f15e9 * l.f1634) + (l.f15dc * l.f164b)), ((l.f15ea * l.f1634) + (l.f15dc * l.f164c)), ((l.f15eb * l.f1634) + (l.f15dc * l.f164d)), );l.f1633 = 0.0;}
        if (l.f839 != 0.0) {let tb1: f64 = (1.6021918e-19 / l.face);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (tb1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1576 = 0.0;}
        let tb2: f64 = if p.p313 == 1.0 { 1.0 } else { 0.0 };l.f84b = tb2;l.f84c = 0.0;
        if (l.f84b != 0.0) {l.fb46 = p.p316;l.fb47 = 0.0;l.fb4a = p.p318;l.fb4b = 0.0;(l.f1417, l.f1418, ) = (p.p323, 0.0, );l.f1419 = 0.0;l.facf = p.p310;l.fad0 = 0.0;let tb3: f64 = (p.p33 * (nv0 - nv11));(l.f196e, l.f196f, l.f1970, ) = (tb3, p.p33, (-p.p33), );l.f1971 = 0.0;let tb4: f64 = (l.fb46 / 10000.0);l.fb46 = tb4;l.fb47 = 0.0;let tb5: f64 = (l.fb4a / 100.0);l.fb4a = tb5;l.fb4b = 0.0;let tb6: f64 = (l.f173f / l.f17aa);(l.f173a, l.f173b, ) = (tb6, (l.f1740 / l.f17aa), );l.f173c = 0.0;}
        if (l.f84b != 0.0) {let tb7: f64 = (l.f173a).powf(p.p320);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (tb7, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((l.f173a).powf(p.p320 - 1.0) * l.f173b)) } } else { (tb7 * (p.p320 * (l.f173b / l.f173a))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1576 = 0.0;}
        if (l.f84b != 0.0) {let tb8: f64 = (l.fb46 / l.f14f1);(l.fb60, l.fb61, l.fb65, l.fb66, l.fb67, l.fb68, l.fb69, l.fb62, l.fb63, l.fb64, ) = (tb8, (-((l.fb46 * l.f1557) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f155b) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f155c) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f155d) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f155e) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f155f) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f1558) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f1559) / (l.f14f1 * l.f14f1))), (-((l.fb46 * l.f155a) / (l.f14f1 * l.f14f1))), );l.fb6a = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f84b != 0.0) {let tb9: f64 = (0.4 * l.f173a);let tba: f64 = (1.8 + tb9);let tbb: f64 = (0.1 * l.f173a);let tbc: f64 = (tbb * l.f173a);let tbd: f64 = (tba + tbc);let tbe: f64 = (1.0 - l.f173a);let tbf: f64 = (p.p321 * tbe);let tc0: f64 = (tbd - tbf);(l.f14d0, l.f14e7, l.f14eb, l.f14ec, l.f14ed, l.f14ee, l.f14ef, l.f14e8, l.f14e9, l.f14ea, ) = (tc0, 0.0, 0.0, (((0.4 * l.f173b) + (((0.1 * l.f173b) * l.f173a) + (tbb * l.f173b))) - (p.p321 * (-l.f173b))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f14f0 = 0.0;let tc1: f64 = (l.fb4a / l.f14d0);(l.f194d, l.f194e, l.f1952, l.f1953, l.f1954, l.f1955, l.f1956, l.f194f, l.f1950, l.f1951, ) = (tc1, (-((l.fb4a * l.f14e7) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14eb) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14ec) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14ed) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14ee) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14ef) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14e8) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14e9) / (l.f14d0 * l.f14d0))), (-((l.fb4a * l.f14ea) / (l.f14d0 * l.f14d0))), );l.f1957 = 0.0;let tc2: f64 = (l.f173f - l.f17aa);let tc3: f64 = (p.p325 * tc2);let tc4: f64 = (l.f1417 + tc3);(l.f1417, l.f1418, ) = (tc4, (l.f1418 + (p.p325 * l.f1740)), );l.f1419 = 0.0;let tc5: f64 = (l.fadd).powf(p.p331);let tc6: f64 = (p.p330 / tc5);let tc7: f64 = (1.0 + tc6);l.f13f1 = tc7;l.f13f2 = 0.0;let tc8: f64 = (l.fadd).powf(p.p329);let tc9: f64 = (p.p328 / tc8);let tca: f64 = (1.0 + tc9);l.f13f5 = tca;l.f13f6 = 0.0;let tcb: f64 = (l.f1a11).powf(p.p327);let tcc: f64 = (p.p326 / tcb);let tcd: f64 = (1.0 + tcc);l.f13f9 = tcd;l.f13fa = 0.0;let tce: f64 = (l.fb60 * l.f13f1);(l.fb60, l.fb61, l.fb65, l.fb66, l.fb67, l.fb68, l.fb69, l.fb62, l.fb63, l.fb64, ) = (tce, (l.fb61 * l.f13f1), (l.fb65 * l.f13f1), (l.fb66 * l.f13f1), (l.fb67 * l.f13f1), (l.fb68 * l.f13f1), (l.fb69 * l.f13f1), (l.fb62 * l.f13f1), (l.fb63 * l.f13f1), (l.fb64 * l.f13f1), );l.fb6a = 0.0;let tcf: f64 = (l.f194d * l.f13f9);let td0: f64 = (tcf * l.f13f5);let td1: f64 = (td0 + 1e-50);(l.f194d, l.f194e, l.f1952, l.f1953, l.f1954, l.f1955, l.f1956, l.f194f, l.f1950, l.f1951, ) = (td1, ((l.f194e * l.f13f9) * l.f13f5), ((l.f1952 * l.f13f9) * l.f13f5), ((l.f1953 * l.f13f9) * l.f13f5), ((l.f1954 * l.f13f9) * l.f13f5), ((l.f1955 * l.f13f9) * l.f13f5), ((l.f1956 * l.f13f9) * l.f13f5), ((l.f194f * l.f13f9) * l.f13f5), ((l.f1950 * l.f13f9) * l.f13f5), ((l.f1951 * l.f13f9) * l.f13f5), );l.f1957 = 0.0;let td2: f64 = (l.f196e / l.facf);(l.f44f, l.f450, l.f451, ) = (td2, (l.f196f / l.facf), (l.f1970 / l.facf), );l.f452 = 0.0;let td3: f64 = (l.fb60 * l.f44f);(l.f1828, l.f1829, l.f182d, l.f182e, l.f182f, l.f1830, l.f1831, l.f182a, l.f182b, l.f182c, ) = (td3, ((l.fb61 * l.f44f) + (l.fb60 * l.f450)), (l.fb65 * l.f44f), (l.fb66 * l.f44f), (l.fb67 * l.f44f), (l.fb68 * l.f44f), (l.fb69 * l.f44f), (l.fb62 * l.f44f), ((l.fb63 * l.f44f) + (l.fb60 * l.f451)), (l.fb64 * l.f44f), );l.f1832 = 0.0;}
        let td4: f64 = if l.f196e >= 0.0 { 1.0 } else { 0.0 };l.f851 = td4;l.f852 = 0.0;
        if ((l.f84b != 0.0) && (l.f851 != 0.0)) {let td5: f64 = (l.f1828 / l.f194d);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (td5, (((l.f1829 * l.f194d) - (l.f1828 * l.f194e)) / (l.f194d * l.f194d)), (((l.f182d * l.f194d) - (l.f1828 * l.f1952)) / (l.f194d * l.f194d)), (((l.f182e * l.f194d) - (l.f1828 * l.f1953)) / (l.f194d * l.f194d)), (((l.f182f * l.f194d) - (l.f1828 * l.f1954)) / (l.f194d * l.f194d)), (((l.f1830 * l.f194d) - (l.f1828 * l.f1955)) / (l.f194d * l.f194d)), (((l.f1831 * l.f194d) - (l.f1828 * l.f1956)) / (l.f194d * l.f194d)), (((l.f182a * l.f194d) - (l.f1828 * l.f194f)) / (l.f194d * l.f194d)), (((l.f182b * l.f194d) - (l.f1828 * l.f1950)) / (l.f194d * l.f194d)), (((l.f182c * l.f194d) - (l.f1828 * l.f1951)) / (l.f194d * l.f194d)), );l.f1576 = 0.0;}
        if ((l.f84b != 0.0) && (l.f851 == 0.0)) {let td6: f64 = (-l.f1828);let td7: f64 = (td6 / l.f194d);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (td7, ((((-l.f1829) * l.f194d) - (td6 * l.f194e)) / (l.f194d * l.f194d)), ((((-l.f182d) * l.f194d) - (td6 * l.f1952)) / (l.f194d * l.f194d)), ((((-l.f182e) * l.f194d) - (td6 * l.f1953)) / (l.f194d * l.f194d)), ((((-l.f182f) * l.f194d) - (td6 * l.f1954)) / (l.f194d * l.f194d)), ((((-l.f1830) * l.f194d) - (td6 * l.f1955)) / (l.f194d * l.f194d)), ((((-l.f1831) * l.f194d) - (td6 * l.f1956)) / (l.f194d * l.f194d)), ((((-l.f182a) * l.f194d) - (td6 * l.f194f)) / (l.f194d * l.f194d)), ((((-l.f182b) * l.f194d) - (td6 * l.f1950)) / (l.f194d * l.f194d)), ((((-l.f182c) * l.f194d) - (td6 * l.f1951)) / (l.f194d * l.f194d)), );l.f1576 = 0.0;}
        let td8: f64 = (10.0 * 2.220446049250313e-16);let td9: f64 = (1.0 - td8);let tda: f64 = (10.0 * 2.220446049250313e-16);let tdb: f64 = (1.0 + tda);let tdc: f64 = if ((td9 <= l.f1417) && (l.f1417 <= tdb)) { 1.0 } else { 0.0 };l.f853 = tdc;l.f854 = 0.0;
        if ((l.f84b != 0.0) && (l.f853 != 0.0)) {(l.f15b0, l.f15d2, l.f15d6, l.f15d7, l.f15d8, l.f15d9, l.f15da, l.f15d3, l.f15d4, l.f15d5, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f15db = 0.0;}
        let tdd: f64 = (10.0 * 2.220446049250313e-16);let tde: f64 = (2.0 - tdd);let tdf: f64 = (10.0 * 2.220446049250313e-16);let te0: f64 = (2.0 + tdf);let te1: f64 = if ((tde <= l.f1417) && (l.f1417 <= te0)) { 1.0 } else { 0.0 };l.f855 = te1;l.f856 = 0.0;
        if (((l.f84b != 0.0) && (l.f853 == 0.0)) && (l.f855 != 0.0)) {(l.f15b0, l.f15d2, l.f15d6, l.f15d7, l.f15d8, l.f15d9, l.f15da, l.f15d3, l.f15d4, l.f15d5, ) = (l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, );l.f15db = 0.0;}
        if (((l.f84b != 0.0) && (l.f853 == 0.0)) && (l.f855 == 0.0)) {let te2: f64 = (l.f1417 - 1.0);let te3: f64 = (l.f14f1).powf(te2);(l.f15b0, l.f15d2, l.f15d6, l.f15d7, l.f15d8, l.f15d9, l.f15da, l.f15d3, l.f15d4, l.f15d5, ) = (te3, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f1557)) } } else { (te3 * (te2 * (l.f1557 / l.f14f1))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f155b)) } } else { (te3 * (te2 * (l.f155b / l.f14f1))) }, if l.f1418 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f155c)) } } else { (te3 * ((l.f1418 * (l.f14f1).ln()) + (te2 * (l.f155c / l.f14f1)))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f155d)) } } else { (te3 * (te2 * (l.f155d / l.f14f1))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f155e)) } } else { (te3 * (te2 * (l.f155e / l.f14f1))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f155f)) } } else { (te3 * (te2 * (l.f155f / l.f14f1))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f1558)) } } else { (te3 * (te2 * (l.f1558 / l.f14f1))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f1559)) } } else { (te3 * (te2 * (l.f1559 / l.f14f1))) }, if 0.0 == 0.0 && ((te2) as f64).is_finite() && ((te2) as f64).fract() == 0.0 { if te2 == 0.0 { 0.0 } else { (te2 * ((l.f14f1).powf(te2 - 1.0) * l.f155a)) } } else { (te3 * (te2 * (l.f155a / l.f14f1))) }, );l.f15db = 0.0;}
        if (l.f84b != 0.0) {let te4: f64 = (l.f14f1 * l.f15b0);(l.f1577, l.f1599, l.f159d, l.f159e, l.f159f, l.f15a0, l.f15a1, l.f159a, l.f159b, l.f159c, ) = (te4, ((l.f1557 * l.f15b0) + (l.f14f1 * l.f15d2)), ((l.f155b * l.f15b0) + (l.f14f1 * l.f15d6)), ((l.f155c * l.f15b0) + (l.f14f1 * l.f15d7)), ((l.f155d * l.f15b0) + (l.f14f1 * l.f15d8)), ((l.f155e * l.f15b0) + (l.f14f1 * l.f15d9)), ((l.f155f * l.f15b0) + (l.f14f1 * l.f15da)), ((l.f1558 * l.f15b0) + (l.f14f1 * l.f15d3)), ((l.f1559 * l.f15b0) + (l.f14f1 * l.f15d4)), ((l.f155a * l.f15b0) + (l.f14f1 * l.f15d5)), );l.f15af = 0.0;let te5: f64 = (1.0 + l.f1577);(l.f15dc, l.f15e8, l.f15ec, l.f15ed, l.f15ee, l.f15ef, l.f15f0, l.f15e9, l.f15ea, l.f15eb, ) = (te5, l.f1599, l.f159d, l.f159e, l.f159f, l.f15a0, l.f15a1, l.f159a, l.f159b, l.f159c, );l.f1612 = 0.0;}
        let te6: f64 = (10.0 * 2.220446049250313e-16);let te7: f64 = (1.0 - te6);let te8: f64 = (10.0 * 2.220446049250313e-16);let te9: f64 = (1.0 + te8);let tea: f64 = if ((te7 <= l.f1417) && (l.f1417 <= te9)) { 1.0 } else { 0.0 };l.f859 = tea;l.f85a = 0.0;
        if ((l.f84b != 0.0) && (l.f859 != 0.0)) {let teb: f64 = (1.0 / l.f15dc);(l.f1613, l.f162a, l.f162e, l.f162f, l.f1630, l.f1631, l.f1632, l.f162b, l.f162c, l.f162d, ) = (teb, (-(l.f15e8 / (l.f15dc * l.f15dc))), (-(l.f15ec / (l.f15dc * l.f15dc))), (-(l.f15ed / (l.f15dc * l.f15dc))), (-(l.f15ee / (l.f15dc * l.f15dc))), (-(l.f15ef / (l.f15dc * l.f15dc))), (-(l.f15f0 / (l.f15dc * l.f15dc))), (-(l.f15e9 / (l.f15dc * l.f15dc))), (-(l.f15ea / (l.f15dc * l.f15dc))), (-(l.f15eb / (l.f15dc * l.f15dc))), );l.f1633 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_118(
        l: &mut StampLocals,
    ) {
        let tec: f64 = (10.0 * 2.220446049250313e-16);let ted: f64 = (2.0 - tec);let tee: f64 = (10.0 * 2.220446049250313e-16);let tef: f64 = (2.0 + tee);let tf0: f64 = if ((ted <= l.f1417) && (l.f1417 <= tef)) { 1.0 } else { 0.0 };l.f85b = tf0;l.f85c = 0.0;
        if (((l.f84b != 0.0) && (l.f859 == 0.0)) && (l.f85b != 0.0)) {let tf1: f64 = (l.f15dc).sqrt();let tf2: f64 = (1.0 / tf1);(l.f1613, l.f162a, l.f162e, l.f162f, l.f1630, l.f1631, l.f1632, l.f162b, l.f162c, l.f162d, ) = (tf2, (-((l.f15e8 / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15ec / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15ed / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15ee / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15ef / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15f0 / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15e9 / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15ea / (2.0 * tf1)) / (tf1 * tf1))), (-((l.f15eb / (2.0 * tf1)) / (tf1 * tf1))), );l.f1633 = 0.0;}
        if (((l.f84b != 0.0) && (l.f859 == 0.0)) && (l.f85b == 0.0)) {let tf3: f64 = (-1.0);let tf4: f64 = (tf3 / l.f1417);let tf5: f64 = (tf4 - 1.0);let tf6: f64 = (l.f15dc).powf(tf5);(l.f1634, l.f164a, l.f164e, l.f164f, l.f1650, l.f1651, l.f1652, l.f164b, l.f164c, l.f164d, ) = (tf6, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15e8)) } } else { (tf6 * (tf5 * (l.f15e8 / l.f15dc))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15ec)) } } else { (tf6 * (tf5 * (l.f15ec / l.f15dc))) }, if (-((tf3 * l.f1418) / (l.f1417 * l.f1417))) == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15ed)) } } else { (tf6 * (((-((tf3 * l.f1418) / (l.f1417 * l.f1417))) * (l.f15dc).ln()) + (tf5 * (l.f15ed / l.f15dc)))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15ee)) } } else { (tf6 * (tf5 * (l.f15ee / l.f15dc))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15ef)) } } else { (tf6 * (tf5 * (l.f15ef / l.f15dc))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15f0)) } } else { (tf6 * (tf5 * (l.f15f0 / l.f15dc))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15e9)) } } else { (tf6 * (tf5 * (l.f15e9 / l.f15dc))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15ea)) } } else { (tf6 * (tf5 * (l.f15ea / l.f15dc))) }, if 0.0 == 0.0 && ((tf5) as f64).is_finite() && ((tf5) as f64).fract() == 0.0 { if tf5 == 0.0 { 0.0 } else { (tf5 * ((l.f15dc).powf(tf5 - 1.0) * l.f15eb)) } } else { (tf6 * (tf5 * (l.f15eb / l.f15dc))) }, );l.f1653 = 0.0;}
        if (((l.f84b != 0.0) && (l.f859 == 0.0)) && (l.f85b == 0.0)) {let tf7: f64 = (l.f15dc * l.f1634);(l.f1613, l.f162a, l.f162e, l.f162f, l.f1630, l.f1631, l.f1632, l.f162b, l.f162c, l.f162d, ) = (tf7, ((l.f15e8 * l.f1634) + (l.f15dc * l.f164a)), ((l.f15ec * l.f1634) + (l.f15dc * l.f164e)), ((l.f15ed * l.f1634) + (l.f15dc * l.f164f)), ((l.f15ee * l.f1634) + (l.f15dc * l.f1650)), ((l.f15ef * l.f1634) + (l.f15dc * l.f1651)), ((l.f15f0 * l.f1634) + (l.f15dc * l.f1652)), ((l.f15e9 * l.f1634) + (l.f15dc * l.f164b)), ((l.f15ea * l.f1634) + (l.f15dc * l.f164c)), ((l.f15eb * l.f1634) + (l.f15dc * l.f164d)), );l.f1633 = 0.0;}
        if (l.f84b != 0.0) {let tf8: f64 = (1.6021918e-19 / l.facf);(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (tf8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1576 = 0.0;}
        let tf9: f64 = if l.f16a2 < 1e-18 { 1.0 } else { 0.0 };l.f85f = tf9;l.f860 = 0.0;
        if ((l.f58d != 0.0) && (l.f85f != 0.0)) {(l.f16a2, l.f16a3, l.f16a7, l.f16a8, l.f16a9, l.f16aa, l.f16ab, l.f16a4, l.f16a5, l.f16a6, ) = (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f16ac = 0.0;}
        let tfa: f64 = if l.f16ad < 1e-18 { 1.0 } else { 0.0 };l.f861 = tfa;l.f862 = 0.0;
        if ((l.f58d != 0.0) && (l.f861 != 0.0)) {(l.f16ad, l.f16ae, l.f16b2, l.f16b3, l.f16b4, l.f16b5, l.f16b6, l.f16af, l.f16b0, l.f16b1, ) = (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f16b7 = 0.0;}
        if (l.f58d != 0.0) {let tfb: f64 = (l.f130e - l.f1311);let tfc: f64 = (tfb / l.f16a2);(l.fa56, l.fa57, l.fa5b, l.fa5c, l.fa5d, l.fa5e, l.fa5f, l.fa58, l.fa59, l.fa5a, ) = (tfc, ((((-l.f1312) * l.f16a2) - (tfb * l.f16a3)) / (l.f16a2 * l.f16a2)), ((((-l.f1316) * l.f16a2) - (tfb * l.f16a7)) / (l.f16a2 * l.f16a2)), ((((-l.f1317) * l.f16a2) - (tfb * l.f16a8)) / (l.f16a2 * l.f16a2)), ((((-l.f1318) * l.f16a2) - (tfb * l.f16a9)) / (l.f16a2 * l.f16a2)), ((((-l.f1319) * l.f16a2) - (tfb * l.f16aa)) / (l.f16a2 * l.f16a2)), ((((l.f130f - l.f131a) * l.f16a2) - (tfb * l.f16ab)) / (l.f16a2 * l.f16a2)), ((((-l.f1313) * l.f16a2) - (tfb * l.f16a4)) / (l.f16a2 * l.f16a2)), ((((-l.f1314) * l.f16a2) - (tfb * l.f16a5)) / (l.f16a2 * l.f16a2)), ((((-l.f1315) * l.f16a2) - (tfb * l.f16a6)) / (l.f16a2 * l.f16a2)), );l.fa60 = 0.0;let tfd: f64 = (l.f1220 - l.f1223);let tfe: f64 = (tfd / l.f16ad);(l.fa3f, l.fa40, l.fa44, l.fa45, l.fa46, l.fa47, l.fa48, l.fa49, l.fa41, l.fa42, l.fa43, ) = (tfe, ((((-l.f1224) * l.f16ad) - (tfd * l.f16ae)) / (l.f16ad * l.f16ad)), ((((-l.f1228) * l.f16ad) - (tfd * l.f16b2)) / (l.f16ad * l.f16ad)), ((((-l.f1229) * l.f16ad) - (tfd * l.f16b3)) / (l.f16ad * l.f16ad)), ((((-l.f122a) * l.f16ad) - (tfd * l.f16b4)) / (l.f16ad * l.f16ad)), ((((-l.f122b) * l.f16ad) - (tfd * l.f16b5)) / (l.f16ad * l.f16ad)), ((((-l.f122c) * l.f16ad) - (tfd * l.f16b6)) / (l.f16ad * l.f16ad)), (l.f1221 / l.f16ad), ((((-l.f1225) * l.f16ad) - (tfd * l.f16af)) / (l.f16ad * l.f16ad)), ((((-l.f1226) * l.f16ad) - (tfd * l.f16b0)) / (l.f16ad * l.f16ad)), ((((-l.f1227) * l.f16ad) - (tfd * l.f16b1)) / (l.f16ad * l.f16ad)), );l.fa4a = 0.0;let tff: f64 = (-l.f130e);let t100: f64 = (tff - l.f1220);(l.f12bf, l.f12c0, l.f12c1, ) = (t100, (-l.f130f), (-l.f1221), );l.f12c2 = 0.0;let t101: f64 = (l.f130e * l.f1287);(l.f1265, l.f1266, l.f126a, l.f126b, l.f126c, l.f126d, l.f126e, l.f1267, l.f1268, l.f1269, ) = (t101, 0.0, 0.0, 0.0, 0.0, 0.0, (l.f130f * l.f1287), 0.0, 0.0, 0.0, );l.f126f = 0.0;let t102: f64 = (1.0 - l.f1287);let t103: f64 = (l.f130e * t102);(l.f13a3, l.f13a4, l.f13a8, l.f13a9, l.f13aa, l.f13ab, l.f13ac, l.f13a5, l.f13a6, l.f13a7, ) = (t103, 0.0, 0.0, 0.0, 0.0, 0.0, (l.f130f * t102), 0.0, 0.0, 0.0, );l.f13ad = 0.0;}
        if (l.f58d == 0.0) {(l.fa56, l.fa57, l.fa5b, l.fa5c, l.fa5d, l.fa5e, l.fa5f, l.fa58, l.fa59, l.fa5a, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.fa60 = 0.0;(l.fa3f, l.fa40, l.fa44, l.fa45, l.fa46, l.fa47, l.fa48, l.fa49, l.fa41, l.fa42, l.fa43, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.fa4a = 0.0;(l.f1265, l.f1266, l.f126a, l.f126b, l.f126c, l.f126d, l.f126e, l.f1267, l.f1268, l.f1269, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f126f = 0.0;(l.f13a3, l.f13a4, l.f13a8, l.f13a9, l.f13aa, l.f13ab, l.f13ac, l.f13a5, l.f13a6, l.f13a7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f13ad = 0.0;(l.f12bf, l.f12c0, l.f12c1, ) = (0.0, 0.0, 0.0, );l.f12c2 = 0.0;(l.f1220, l.f1221, ) = (0.0, 0.0, );l.f1222 = 0.0;}
        let t104: f64 = if l.fb55 == 1.0 { 1.0 } else { 0.0 };l.f863 = t104;l.f864 = 0.0;
        if (l.f863 != 0.0) {(l.f900, l.f94e, l.f952, l.f953, l.f954, l.f955, l.f956, l.f94f, l.f950, l.f951, ) = (l.f963, l.f964, l.f968, l.f969, l.f96a, l.f96b, l.f96c, l.f965, l.f966, l.f967, );l.f962 = 0.0;(l.fa61, l.fa62, l.fa66, l.fa67, l.fa68, l.fa69, l.fa6a, l.fa63, l.fa64, l.fa65, ) = (l.fa6c, l.fa6d, l.fa71, l.fa72, l.fa73, l.fa74, l.fa75, l.fa6e, l.fa6f, l.fa70, );l.fa6b = 0.0;(l.f12b5, l.f12b6, l.f12ba, l.f12bb, l.f12bc, l.f12bd, l.f12be, l.f12b7, l.f12b8, l.f12b9, ) = (l.f12c4, l.f12c5, l.f12c9, l.f12ca, l.f12cb, l.f12cc, l.f12cd, l.f12c6, l.f12c7, l.f12c8, );l.f12c3 = 0.0;(l.f125b, l.f125c, l.f1260, l.f1261, l.f1262, l.f1263, l.f1264, l.f125d, l.f125e, l.f125f, ) = (l.f1271, l.f1272, l.f1276, l.f1277, l.f1278, l.f1279, l.f127a, l.f1273, l.f1274, l.f1275, );l.f1270 = 0.0;let t105: f64 = (l.f12c4 + l.f1271);let t106: f64 = (t105 + l.f13ae);let t107: f64 = (-t106);(l.f123a, l.f123b, l.f123f, l.f1240, l.f1241, l.f1242, l.f1243, l.f123c, l.f123d, l.f123e, ) = (t107, (-((l.f12c5 + l.f1272) + l.f13af)), (-((l.f12c9 + l.f1276) + l.f13b3)), (-((l.f12ca + l.f1277) + l.f13b4)), (-((l.f12cb + l.f1278) + l.f13b5)), (-((l.f12cc + l.f1279) + l.f13b6)), (-((l.f12cd + l.f127a) + l.f13b7)), (-((l.f12c6 + l.f1273) + l.f13b0)), (-((l.f12c7 + l.f1274) + l.f13b1)), (-((l.f12c8 + l.f1275) + l.f13b2)), );l.f1244 = 0.0;(l.f11fe, l.f1217, l.f121b, l.f121c, l.f121d, l.f121e, l.f121f, l.f1218, l.f1219, l.f121a, ) = (l.f123a, l.f123b, l.f123f, l.f1240, l.f1241, l.f1242, l.f1243, l.f123c, l.f123d, l.f123e, );l.f122e = 0.0;}
        if (l.f863 == 0.0) {let t108: f64 = (-l.f963);(l.f900, l.f94e, l.f952, l.f953, l.f954, l.f955, l.f956, l.f94f, l.f950, l.f951, ) = (t108, (-l.f964), (-l.f968), (-l.f969), (-l.f96a), (-l.f96b), (-l.f96c), (-l.f965), (-l.f966), (-l.f967), );l.f962 = 0.0;(l.fa61, l.fa62, l.fa66, l.fa67, l.fa68, l.fa69, l.fa6a, l.fa63, l.fa64, l.fa65, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.fa6b = 0.0;(l.f12b5, l.f12b6, l.f12ba, l.f12bb, l.f12bc, l.f12bd, l.f12be, l.f12b7, l.f12b8, l.f12b9, ) = (l.f12c4, l.f12c5, l.f12c9, l.f12ca, l.f12cb, l.f12cc, l.f12cd, l.f12c6, l.f12c7, l.f12c8, );l.f12c3 = 0.0;(l.f125b, l.f125c, l.f1260, l.f1261, l.f1262, l.f1263, l.f1264, l.f125d, l.f125e, l.f125f, ) = (l.f13ae, l.f13af, l.f13b3, l.f13b4, l.f13b5, l.f13b6, l.f13b7, l.f13b0, l.f13b1, l.f13b2, );l.f1270 = 0.0;let t109: f64 = (l.f12c4 + l.f1271);let t10a: f64 = (t109 + l.f13ae);let t10b: f64 = (-t10a);(l.f123a, l.f123b, l.f123f, l.f1240, l.f1241, l.f1242, l.f1243, l.f123c, l.f123d, l.f123e, ) = (t10b, (-((l.f12c5 + l.f1272) + l.f13af)), (-((l.f12c9 + l.f1276) + l.f13b3)), (-((l.f12ca + l.f1277) + l.f13b4)), (-((l.f12cb + l.f1278) + l.f13b5)), (-((l.f12cc + l.f1279) + l.f13b6)), (-((l.f12cd + l.f127a) + l.f13b7)), (-((l.f12c6 + l.f1273) + l.f13b0)), (-((l.f12c7 + l.f1274) + l.f13b1)), (-((l.f12c8 + l.f1275) + l.f13b2)), );l.f1244 = 0.0;}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_119(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (l.f863 == 0.0) {(l.f11fe, l.f1217, l.f121b, l.f121c, l.f121d, l.f121e, l.f121f, l.f1218, l.f1219, l.f121a, ) = (l.f123a, l.f123b, l.f123f, l.f1240, l.f1241, l.f1242, l.f1243, l.f123c, l.f123d, l.f123e, );l.f122e = 0.0;(l.f13ae, l.f13af, l.f13b3, l.f13b4, l.f13b5, l.f13b6, l.f13b7, l.f13b0, l.f13b1, l.f13b2, ) = (l.f1271, l.f1272, l.f1276, l.f1277, l.f1278, l.f1279, l.f127a, l.f1273, l.f1274, l.f1275, );l.f13b8 = 0.0;(l.f1271, l.f1272, l.f1276, l.f1277, l.f1278, l.f1279, l.f127a, l.f1273, l.f1274, l.f1275, ) = (l.f125b, l.f125c, l.f1260, l.f1261, l.f1262, l.f1263, l.f1264, l.f125d, l.f125e, l.f125f, );l.f127b = 0.0;}
        if ((l.f863 == 0.0) && (l.f58d != 0.0)) {(l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, ) = (l.f1265, l.f1266, l.f126a, l.f126b, l.f126c, l.f126d, l.f126e, l.f1267, l.f1268, l.f1269, );l.f1576 = 0.0;(l.f1265, l.f1266, l.f126a, l.f126b, l.f126c, l.f126d, l.f126e, l.f1267, l.f1268, l.f1269, ) = (l.f13a3, l.f13a4, l.f13a8, l.f13a9, l.f13aa, l.f13ab, l.f13ac, l.f13a5, l.f13a6, l.f13a7, );l.f126f = 0.0;(l.f13a3, l.f13a4, l.f13a8, l.f13a9, l.f13aa, l.f13ab, l.f13ac, l.f13a5, l.f13a6, l.f13a7, ) = (l.f14f1, l.f1557, l.f155b, l.f155c, l.f155d, l.f155e, l.f155f, l.f1558, l.f1559, l.f155a, );l.f13ad = 0.0;}
        let t10c: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };l.f865 = t10c;l.f866 = 0.0;
        if (l.f865 != 0.0) {let t10d: f64 = (l.f963 * l.f183d);(l.f140b, l.f140c, l.f1410, l.f1411, l.f1412, l.f1413, l.f1414, l.f140d, l.f140e, l.f140f, ) = (t10d, ((l.f964 * l.f183d) + (l.f963 * l.f183e)), ((l.f968 * l.f183d) + (l.f963 * l.f1842)), ((l.f969 * l.f183d) + (l.f963 * l.f1843)), ((l.f96a * l.f183d) + (l.f963 * l.f1844)), ((l.f96b * l.f183d) + (l.f963 * l.f1845)), ((l.f96c * l.f183d) + (l.f963 * l.f1846)), ((l.f965 * l.f183d) + (l.f963 * l.f183f)), ((l.f966 * l.f183d) + (l.f963 * l.f1840)), ((l.f967 * l.f183d) + (l.f963 * l.f1841)), );l.f1415 = 0.0;(l.f327, l.f328, l.f32c, l.f32d, l.f32e, l.f32f, l.f330, l.f329, l.f32a, l.f32b, ) = (l.f31c, l.f31d, l.f321, l.f322, l.f323, l.f324, l.f325, l.f31e, l.f31f, l.f320, );l.f331 = 0.0;let t10e: f64 = (1.0 / l.f145e);(l.f638, l.f639, l.f63d, l.f63e, l.f63f, l.f640, l.f641, l.f63a, l.f63b, l.f63c, ) = (t10e, (-(l.f145f / (l.f145e * l.f145e))), (-(l.f1463 / (l.f145e * l.f145e))), (-(l.f1464 / (l.f145e * l.f145e))), (-(l.f1465 / (l.f145e * l.f145e))), (-(l.f1466 / (l.f145e * l.f145e))), (-(l.f1467 / (l.f145e * l.f145e))), (-(l.f1460 / (l.f145e * l.f145e))), (-(l.f1461 / (l.f145e * l.f145e))), (-(l.f1462 / (l.f145e * l.f145e))), );l.f642 = 0.0;}
        if (l.f865 == 0.0) {(l.f140b, l.f140c, l.f1410, l.f1411, l.f1412, l.f1413, l.f1414, l.f140d, l.f140e, l.f140f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1415 = 0.0;(l.f327, l.f328, l.f32c, l.f32d, l.f32e, l.f32f, l.f330, l.f329, l.f32a, l.f32b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f331 = 0.0;(l.f638, l.f639, l.f63d, l.f63e, l.f63f, l.f640, l.f641, l.f63a, l.f63b, l.f63c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f642 = 0.0;}
        (l.f963, l.f964, l.f968, l.f969, l.f96a, l.f96b, l.f96c, l.f965, l.f966, l.f967, ) = (l.f900, l.f94e, l.f952, l.f953, l.f954, l.f955, l.f956, l.f94f, l.f950, l.f951, );l.f96d = 0.0;let t10f: f64 = l.f12c7;(l.ff0, l.ff1, l.ff5, l.ff6, l.ff7, l.ff8, l.ff9, l.ff2, l.ff3, l.ff4, ) = (t10f, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.ffa = 0.0;let t110: f64 = (p.p33 * l.ff0);(l.ff0, l.ff1, l.ff5, l.ff6, l.ff7, l.ff8, l.ff9, l.ff2, l.ff3, l.ff4, ) = (t110, (p.p33 * l.ff1), (p.p33 * l.ff5), (p.p33 * l.ff6), (p.p33 * l.ff7), (p.p33 * l.ff8), (p.p33 * l.ff9), (p.p33 * l.ff2), (p.p33 * l.ff3), (p.p33 * l.ff4), );l.ffa = 0.0;let t111: f64 = l.f12c8;(l.f1f4, l.f1f5, l.f1f9, l.f1fa, l.f1fb, l.f1fc, l.f1fd, l.f1f6, l.f1f7, l.f1f8, ) = (t111, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1fe = 0.0;let t112: f64 = (p.p33 * l.f1f4);(l.f1f4, l.f1f5, l.f1f9, l.f1fa, l.f1fb, l.f1fc, l.f1fd, l.f1f6, l.f1f7, l.f1f8, ) = (t112, (p.p33 * l.f1f5), (p.p33 * l.f1f9), (p.p33 * l.f1fa), (p.p33 * l.f1fb), (p.p33 * l.f1fc), (p.p33 * l.f1fd), (p.p33 * l.f1f6), (p.p33 * l.f1f7), (p.p33 * l.f1f8), );l.f1fe = 0.0;let t113: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };l.f868 = t113;l.f869 = 0.0;let t114: f64 = if (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };l.f86a = t114;l.f86b = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let eq0_e348: f64 = (p.p33 * l.f900);let eq0_e348_d_n0: f64 = (p.p33 * l.f94e);let eq0_e348_d_n2: f64 = (p.p33 * l.f952);let eq0_e348_d_n4: f64 = (p.p33 * l.f953);let eq0_e348_d_n5: f64 = (p.p33 * l.f954);let eq0_e348_d_n6: f64 = (p.p33 * l.f955);let eq0_e348_d_n8: f64 = (p.p33 * l.f956);let eq0_e348_d_n10: f64 = (p.p33 * l.f94f);let eq0_e348_d_n11: f64 = (p.p33 * l.f950);let eq0_e348_d_n12: f64 = (p.p33 * l.f951);let eq0_value: f64 = eq0_e348;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq0_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq0_e348_d_n0), multiplicity * (eq0_e348_d_n2), multiplicity * (eq0_e348_d_n4), multiplicity * (eq0_e348_d_n5), multiplicity * (eq0_e348_d_n6), multiplicity * (eq0_e348_d_n8), multiplicity * (eq0_e348_d_n10), multiplicity * (eq0_e348_d_n11), multiplicity * (eq0_e348_d_n12)],
            [],
            [],
            1.0,
        );let eq1_e352: f64 = (l.fa03 + l.fa61);let eq1_e352_d_n0: f64 = (l.fa04 + l.fa62);let eq1_e352_d_n2: f64 = (l.fa08 + l.fa66);let eq1_e352_d_n4: f64 = (l.fa09 + l.fa67);let eq1_e352_d_n5: f64 = (l.fa0a + l.fa68);let eq1_e352_d_n6: f64 = (l.fa0b + l.fa69);let eq1_e352_d_n8: f64 = (l.fa0c + l.fa6a);let eq1_e352_d_n10: f64 = (l.fa05 + l.fa63);let eq1_e352_d_n11: f64 = (l.fa06 + l.fa64);let eq1_e352_d_n12: f64 = (l.fa07 + l.fa65);let eq1_e353: f64 = (p.p33 * eq1_e352);let eq1_e353_d_n0: f64 = (p.p33 * eq1_e352_d_n0);let eq1_e353_d_n2: f64 = (p.p33 * eq1_e352_d_n2);let eq1_e353_d_n4: f64 = (p.p33 * eq1_e352_d_n4);let eq1_e353_d_n5: f64 = (p.p33 * eq1_e352_d_n5);let eq1_e353_d_n6: f64 = (p.p33 * eq1_e352_d_n6);let eq1_e353_d_n8: f64 = (p.p33 * eq1_e352_d_n8);let eq1_e353_d_n10: f64 = (p.p33 * eq1_e352_d_n10);let eq1_e353_d_n11: f64 = (p.p33 * eq1_e352_d_n11);let eq1_e353_d_n12: f64 = (p.p33 * eq1_e352_d_n12);let eq1_value: f64 = eq1_e353;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq1_e353_d_n0), multiplicity * (eq1_e353_d_n2), multiplicity * (eq1_e353_d_n4), multiplicity * (eq1_e353_d_n5), multiplicity * (eq1_e353_d_n6), multiplicity * (eq1_e353_d_n8), multiplicity * (eq1_e353_d_n10), multiplicity * (eq1_e353_d_n11), multiplicity * (eq1_e353_d_n12)],
            [],
            [],
            1.0,
        );let eq2_e357: f64 = (l.fa17 + l.fa77);let eq2_e357_d_n0: f64 = (l.fa18 + l.fa78);let eq2_e357_d_n2: f64 = (l.fa1c + l.fa7c);let eq2_e357_d_n4: f64 = (l.fa1d + l.fa7d);let eq2_e357_d_n5: f64 = (l.fa1e + l.fa7e);let eq2_e357_d_n6: f64 = (l.fa1f + l.fa7f);let eq2_e357_d_n8: f64 = (l.fa20 + l.fa80);let eq2_e357_d_n10: f64 = (l.fa19 + l.fa79);let eq2_e357_d_n11: f64 = (l.fa1a + l.fa7a);let eq2_e357_d_n12: f64 = (l.fa1b + l.fa7b);let eq2_e358: f64 = (p.p33 * eq2_e357);let eq2_e358_d_n0: f64 = (p.p33 * eq2_e357_d_n0);let eq2_e358_d_n2: f64 = (p.p33 * eq2_e357_d_n2);let eq2_e358_d_n4: f64 = (p.p33 * eq2_e357_d_n4);let eq2_e358_d_n5: f64 = (p.p33 * eq2_e357_d_n5);let eq2_e358_d_n6: f64 = (p.p33 * eq2_e357_d_n6);let eq2_e358_d_n8: f64 = (p.p33 * eq2_e357_d_n8);let eq2_e358_d_n10: f64 = (p.p33 * eq2_e357_d_n10);let eq2_e358_d_n11: f64 = (p.p33 * eq2_e357_d_n11);let eq2_e358_d_n12: f64 = (p.p33 * eq2_e357_d_n12);let eq2_value: f64 = eq2_e358;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq2_e358_d_n0), multiplicity * (eq2_e358_d_n2), multiplicity * (eq2_e358_d_n4), multiplicity * (eq2_e358_d_n5), multiplicity * (eq2_e358_d_n6), multiplicity * (eq2_e358_d_n8), multiplicity * (eq2_e358_d_n10), multiplicity * (eq2_e358_d_n11), multiplicity * (eq2_e358_d_n12)],
            [],
            [],
            1.0,
        );let eq3_e361: f64 = (p.p33 * l.fa2b);let eq3_e361_d_n0: f64 = (p.p33 * l.fa2c);let eq3_e361_d_n2: f64 = (p.p33 * l.fa30);let eq3_e361_d_n4: f64 = (p.p33 * l.fa31);let eq3_e361_d_n5: f64 = (p.p33 * l.fa32);let eq3_e361_d_n6: f64 = (p.p33 * l.fa33);let eq3_e361_d_n8: f64 = (p.p33 * l.fa34);let eq3_e361_d_n10: f64 = (p.p33 * l.fa2d);let eq3_e361_d_n11: f64 = (p.p33 * l.fa2e);let eq3_e361_d_n12: f64 = (p.p33 * l.fa2f);let eq3_value: f64 = eq3_e361;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq3_e361_d_n0), multiplicity * (eq3_e361_d_n2), multiplicity * (eq3_e361_d_n4), multiplicity * (eq3_e361_d_n5), multiplicity * (eq3_e361_d_n6), multiplicity * (eq3_e361_d_n8), multiplicity * (eq3_e361_d_n10), multiplicity * (eq3_e361_d_n11), multiplicity * (eq3_e361_d_n12)],
            [],
            [],
            1.0,
        );let eq4_e364: f64 = (p.p33 * l.f9ef);let eq4_e364_d_n0: f64 = (p.p33 * l.f9f0);let eq4_e364_d_n2: f64 = (p.p33 * l.f9f4);let eq4_e364_d_n4: f64 = (p.p33 * l.f9f5);let eq4_e364_d_n5: f64 = (p.p33 * l.f9f6);let eq4_e364_d_n6: f64 = (p.p33 * l.f9f7);let eq4_e364_d_n8: f64 = (p.p33 * l.f9f8);let eq4_e364_d_n10: f64 = (p.p33 * l.f9f1);let eq4_e364_d_n11: f64 = (p.p33 * l.f9f2);let eq4_e364_d_n12: f64 = (p.p33 * l.f9f3);let eq4_value: f64 = eq4_e364;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq4_e364_d_n0), multiplicity * (eq4_e364_d_n2), multiplicity * (eq4_e364_d_n4), multiplicity * (eq4_e364_d_n5), multiplicity * (eq4_e364_d_n6), multiplicity * (eq4_e364_d_n8), multiplicity * (eq4_e364_d_n10), multiplicity * (eq4_e364_d_n11), multiplicity * (eq4_e364_d_n12)],
            [],
            [],
            1.0,
        );let eq5_e367: f64 = (p.p33 * l.f9c5);let eq5_e367_d_n0: f64 = (p.p33 * l.f9dc);let eq5_e367_d_n2: f64 = (p.p33 * l.f9e0);let eq5_e367_d_n4: f64 = (p.p33 * l.f9e1);let eq5_e367_d_n5: f64 = (p.p33 * l.f9e2);let eq5_e367_d_n6: f64 = (p.p33 * l.f9e3);let eq5_e367_d_n8: f64 = (p.p33 * l.f9e4);let eq5_e367_d_n10: f64 = (p.p33 * l.f9dd);let eq5_e367_d_n11: f64 = (p.p33 * l.f9de);let eq5_e367_d_n12: f64 = (p.p33 * l.f9df);let eq5_value: f64 = eq5_e367;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq5_e367_d_n0), multiplicity * (eq5_e367_d_n2), multiplicity * (eq5_e367_d_n4), multiplicity * (eq5_e367_d_n5), multiplicity * (eq5_e367_d_n6), multiplicity * (eq5_e367_d_n8), multiplicity * (eq5_e367_d_n10), multiplicity * (eq5_e367_d_n11), multiplicity * (eq5_e367_d_n12)],
            [],
            [],
            1.0,
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq6_e373, eq6_e373_d_n0, eq6_e373_d_n2, eq6_e373_d_n4, eq6_e373_d_n5, eq6_e373_d_n6, eq6_e373_d_n8, eq6_e373_d_n10, eq6_e373_d_n11, eq6_e373_d_n12,) = {
    if (p.p312 != 0.0) {
        let eq6_e371: f64 = ((nv12 - nv2) / l.f1448);let eq6_e371_d_n0: f64 = (-(((nv12 - nv2) * l.f144b) / (l.f1448 * l.f1448)));let eq6_e371_d_n2: f64 = (((-l.f1448) - ((nv12 - nv2) * l.f144f)) / (l.f1448 * l.f1448));let eq6_e371_d_n4: f64 = (-(((nv12 - nv2) * l.f1450) / (l.f1448 * l.f1448)));let eq6_e371_d_n5: f64 = (-(((nv12 - nv2) * l.f1451) / (l.f1448 * l.f1448)));let eq6_e371_d_n6: f64 = (-(((nv12 - nv2) * l.f1452) / (l.f1448 * l.f1448)));let eq6_e371_d_n8: f64 = (-(((nv12 - nv2) * l.f1453) / (l.f1448 * l.f1448)));let eq6_e371_d_n10: f64 = (-(((nv12 - nv2) * l.f144c) / (l.f1448 * l.f1448)));let eq6_e371_d_n11: f64 = (-(((nv12 - nv2) * l.f144d) / (l.f1448 * l.f1448)));let eq6_e371_d_n12: f64 = ((l.f1448 - ((nv12 - nv2) * l.f144e)) / (l.f1448 * l.f1448));
        (eq6_e371, eq6_e371_d_n0, eq6_e371_d_n2, eq6_e371_d_n4, eq6_e371_d_n5, eq6_e371_d_n6, eq6_e371_d_n8, eq6_e371_d_n10, eq6_e371_d_n11, eq6_e371_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e373;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq6_e373_d_n0), multiplicity * (eq6_e373_d_n2), multiplicity * (eq6_e373_d_n4), multiplicity * (eq6_e373_d_n5), multiplicity * (eq6_e373_d_n6), multiplicity * (eq6_e373_d_n8), multiplicity * (eq6_e373_d_n10), multiplicity * (eq6_e373_d_n11), multiplicity * (eq6_e373_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq7_e378,) = {
    if (p.p312 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e378;
        stamper.stamp_potential_const_local(
            0,
            eq7_value,
        );
        let (eq8_e384, eq8_e384_d_n0, eq8_e384_d_n2, eq8_e384_d_n4, eq8_e384_d_n5, eq8_e384_d_n6, eq8_e384_d_n8, eq8_e384_d_n10, eq8_e384_d_n11, eq8_e384_d_n12,) = {
    if (p.p313 != 0.0) {
        let eq8_e382: f64 = ((nv0 - nv11) / l.f13dc);let eq8_e382_d_n0: f64 = ((l.f13dc - ((nv0 - nv11) * l.f13dd)) / (l.f13dc * l.f13dc));let eq8_e382_d_n2: f64 = (-(((nv0 - nv11) * l.f13e1) / (l.f13dc * l.f13dc)));let eq8_e382_d_n4: f64 = (-(((nv0 - nv11) * l.f13e2) / (l.f13dc * l.f13dc)));let eq8_e382_d_n5: f64 = (-(((nv0 - nv11) * l.f13e3) / (l.f13dc * l.f13dc)));let eq8_e382_d_n6: f64 = (-(((nv0 - nv11) * l.f13e4) / (l.f13dc * l.f13dc)));let eq8_e382_d_n8: f64 = (-(((nv0 - nv11) * l.f13e5) / (l.f13dc * l.f13dc)));let eq8_e382_d_n10: f64 = (-(((nv0 - nv11) * l.f13de) / (l.f13dc * l.f13dc)));let eq8_e382_d_n11: f64 = (((-l.f13dc) - ((nv0 - nv11) * l.f13df)) / (l.f13dc * l.f13dc));let eq8_e382_d_n12: f64 = (-(((nv0 - nv11) * l.f13e0) / (l.f13dc * l.f13dc)));
        (eq8_e382, eq8_e382_d_n0, eq8_e382_d_n2, eq8_e382_d_n4, eq8_e382_d_n5, eq8_e382_d_n6, eq8_e382_d_n8, eq8_e382_d_n10, eq8_e382_d_n11, eq8_e382_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e384;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(11),
            multiplicity * (eq8_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq8_e384_d_n0), multiplicity * (eq8_e384_d_n2), multiplicity * (eq8_e384_d_n4), multiplicity * (eq8_e384_d_n5), multiplicity * (eq8_e384_d_n6), multiplicity * (eq8_e384_d_n8), multiplicity * (eq8_e384_d_n10), multiplicity * (eq8_e384_d_n11), multiplicity * (eq8_e384_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq9_e389,) = {
    if (p.p313 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e389;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );let eq10_e393: f64 = (l.f12b5 + l.f12bf);let eq10_e393_d_n8: f64 = (l.f12be + l.f12c0);let eq10_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e393);let eq10_e395: f64 = (p.p33 * eq10_e394);let eq10_e395_d_n0: f64 = (p.p33 * (l.f12b6 * ddt_scale));let eq10_e395_d_n2: f64 = (p.p33 * (l.f12ba * ddt_scale));let eq10_e395_d_n4: f64 = (p.p33 * (l.f12bb * ddt_scale));let eq10_e395_d_n5: f64 = (p.p33 * (l.f12bc * ddt_scale));let eq10_e395_d_n6: f64 = (p.p33 * (l.f12bd * ddt_scale));let eq10_e395_d_n8: f64 = (p.p33 * (eq10_e393_d_n8 * ddt_scale));let eq10_e395_d_n9: f64 = (p.p33 * (l.f12c1 * ddt_scale));let eq10_e395_d_n10: f64 = (p.p33 * (l.f12b7 * ddt_scale));let eq10_e395_d_n11: f64 = (p.p33 * (l.f12b8 * ddt_scale));let eq10_e395_d_n12: f64 = (p.p33 * (l.f12b9 * ddt_scale));let eq10_value: f64 = eq10_e395;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e395_d_n0), multiplicity * (eq10_e395_d_n2), multiplicity * (eq10_e395_d_n4), multiplicity * (eq10_e395_d_n5), multiplicity * (eq10_e395_d_n6), multiplicity * (eq10_e395_d_n8), multiplicity * (eq10_e395_d_n9), multiplicity * (eq10_e395_d_n10), multiplicity * (eq10_e395_d_n11), multiplicity * (eq10_e395_d_n12)],
            [],
            [],
            1.0,
        );let eq11_e399: f64 = (l.f125b + l.f1265);let eq11_e399_d_n0: f64 = (l.f125c + l.f1266);let eq11_e399_d_n2: f64 = (l.f1260 + l.f126a);let eq11_e399_d_n4: f64 = (l.f1261 + l.f126b);let eq11_e399_d_n5: f64 = (l.f1262 + l.f126c);let eq11_e399_d_n6: f64 = (l.f1263 + l.f126d);let eq11_e399_d_n8: f64 = (l.f1264 + l.f126e);let eq11_e399_d_n10: f64 = (l.f125d + l.f1267);let eq11_e399_d_n11: f64 = (l.f125e + l.f1268);let eq11_e399_d_n12: f64 = (l.f125f + l.f1269);let eq11_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e399);let eq11_e401: f64 = (p.p33 * eq11_e400);let eq11_e401_d_n0: f64 = (p.p33 * (eq11_e399_d_n0 * ddt_scale));let eq11_e401_d_n2: f64 = (p.p33 * (eq11_e399_d_n2 * ddt_scale));let eq11_e401_d_n4: f64 = (p.p33 * (eq11_e399_d_n4 * ddt_scale));let eq11_e401_d_n5: f64 = (p.p33 * (eq11_e399_d_n5 * ddt_scale));let eq11_e401_d_n6: f64 = (p.p33 * (eq11_e399_d_n6 * ddt_scale));let eq11_e401_d_n8: f64 = (p.p33 * (eq11_e399_d_n8 * ddt_scale));let eq11_e401_d_n10: f64 = (p.p33 * (eq11_e399_d_n10 * ddt_scale));let eq11_e401_d_n11: f64 = (p.p33 * (eq11_e399_d_n11 * ddt_scale));let eq11_e401_d_n12: f64 = (p.p33 * (eq11_e399_d_n12 * ddt_scale));let eq11_value: f64 = eq11_e401;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq11_e401_d_n0), multiplicity * (eq11_e401_d_n2), multiplicity * (eq11_e401_d_n4), multiplicity * (eq11_e401_d_n5), multiplicity * (eq11_e401_d_n6), multiplicity * (eq11_e401_d_n8), multiplicity * (eq11_e401_d_n10), multiplicity * (eq11_e401_d_n11), multiplicity * (eq11_e401_d_n12)],
            [],
            [],
            1.0,
        );let eq12_e405: f64 = (l.f11fe + l.f1220);let eq12_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e405);let eq12_e407: f64 = (p.p33 * eq12_e406);let eq12_e407_d_n0: f64 = (p.p33 * (l.f1217 * ddt_scale));let eq12_e407_d_n2: f64 = (p.p33 * (l.f121b * ddt_scale));let eq12_e407_d_n4: f64 = (p.p33 * (l.f121c * ddt_scale));let eq12_e407_d_n5: f64 = (p.p33 * (l.f121d * ddt_scale));let eq12_e407_d_n6: f64 = (p.p33 * (l.f121e * ddt_scale));let eq12_e407_d_n8: f64 = (p.p33 * (l.f121f * ddt_scale));let eq12_e407_d_n9: f64 = (p.p33 * (l.f1221 * ddt_scale));let eq12_e407_d_n10: f64 = (p.p33 * (l.f1218 * ddt_scale));let eq12_e407_d_n11: f64 = (p.p33 * (l.f1219 * ddt_scale));let eq12_e407_d_n12: f64 = (p.p33 * (l.f121a * ddt_scale));let eq12_value: f64 = eq12_e407;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq12_e407_d_n0), multiplicity * (eq12_e407_d_n2), multiplicity * (eq12_e407_d_n4), multiplicity * (eq12_e407_d_n5), multiplicity * (eq12_e407_d_n6), multiplicity * (eq12_e407_d_n8), multiplicity * (eq12_e407_d_n9), multiplicity * (eq12_e407_d_n10), multiplicity * (eq12_e407_d_n11), multiplicity * (eq12_e407_d_n12)],
            [],
            [],
            1.0,
        );let eq14_e418: f64 = (nv7 - 0.0);let eq14_value: f64 = eq14_e418;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (1.0),
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
        let nv1 = ctx.node_voltage(nodes[1]);let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let eq17_e433: f64 = (l.f238 * (nv7 - 0.0));let eq17_e433_d_n0: f64 = (l.f239 * (nv7 - 0.0));let eq17_e433_d_n2: f64 = (l.f23d * (nv7 - 0.0));let eq17_e433_d_n4: f64 = (l.f23e * (nv7 - 0.0));let eq17_e433_d_n5: f64 = (l.f23f * (nv7 - 0.0));let eq17_e433_d_n6: f64 = (l.f240 * (nv7 - 0.0));let eq17_e433_d_n8: f64 = (l.f241 * (nv7 - 0.0));let eq17_e433_d_n10: f64 = (l.f23a * (nv7 - 0.0));let eq17_e433_d_n11: f64 = (l.f23b * (nv7 - 0.0));let eq17_e433_d_n12: f64 = (l.f23c * (nv7 - 0.0));let eq17_value: f64 = eq17_e433;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * (eq17_e433_d_n0), multiplicity * (eq17_e433_d_n2), multiplicity * (eq17_e433_d_n4), multiplicity * (eq17_e433_d_n5), multiplicity * (eq17_e433_d_n6), multiplicity * (l.f238), multiplicity * (eq17_e433_d_n8), multiplicity * (eq17_e433_d_n10), multiplicity * (eq17_e433_d_n11), multiplicity * (eq17_e433_d_n12)],
            [],
            [],
            1.0,
        );let eq18_e436: f64 = ((nv7 - 0.0) * l.f1497);let eq18_e436_d_n0: f64 = ((nv7 - 0.0) * l.f1498);let eq18_e436_d_n2: f64 = ((nv7 - 0.0) * l.f149c);let eq18_e436_d_n4: f64 = ((nv7 - 0.0) * l.f149d);let eq18_e436_d_n5: f64 = ((nv7 - 0.0) * l.f149e);let eq18_e436_d_n6: f64 = ((nv7 - 0.0) * l.f149f);let eq18_e436_d_n8: f64 = ((nv7 - 0.0) * l.f14a0);let eq18_e436_d_n10: f64 = ((nv7 - 0.0) * l.f1499);let eq18_e436_d_n11: f64 = ((nv7 - 0.0) * l.f149a);let eq18_e436_d_n12: f64 = ((nv7 - 0.0) * l.f149b);let eq18_e437: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e436);let eq18_value: f64 = eq18_e437;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq18_e436_d_n0 * ddt_scale)), multiplicity * ((eq18_e436_d_n2 * ddt_scale)), multiplicity * ((eq18_e436_d_n4 * ddt_scale)), multiplicity * ((eq18_e436_d_n5 * ddt_scale)), multiplicity * ((eq18_e436_d_n6 * ddt_scale)), multiplicity * ((l.f1497 * ddt_scale)), multiplicity * ((eq18_e436_d_n8 * ddt_scale)), multiplicity * ((eq18_e436_d_n10 * ddt_scale)), multiplicity * ((eq18_e436_d_n11 * ddt_scale)), multiplicity * ((eq18_e436_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );let eq19_e440: f64 = ((nv7 - 0.0) * l.f1482);let eq19_e440_d_n0: f64 = ((nv7 - 0.0) * l.f1483);let eq19_e440_d_n2: f64 = ((nv7 - 0.0) * l.f1487);let eq19_e440_d_n4: f64 = ((nv7 - 0.0) * l.f1488);let eq19_e440_d_n5: f64 = ((nv7 - 0.0) * l.f1489);let eq19_e440_d_n6: f64 = ((nv7 - 0.0) * l.f148a);let eq19_e440_d_n8: f64 = ((nv7 - 0.0) * l.f148b);let eq19_e440_d_n10: f64 = ((nv7 - 0.0) * l.f1484);let eq19_e440_d_n11: f64 = ((nv7 - 0.0) * l.f1485);let eq19_e440_d_n12: f64 = ((nv7 - 0.0) * l.f1486);let eq19_e441: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e440);let eq19_value: f64 = eq19_e441;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq19_e440_d_n0 * ddt_scale)), multiplicity * ((eq19_e440_d_n2 * ddt_scale)), multiplicity * ((eq19_e440_d_n4 * ddt_scale)), multiplicity * ((eq19_e440_d_n5 * ddt_scale)), multiplicity * ((eq19_e440_d_n6 * ddt_scale)), multiplicity * ((l.f1482 * ddt_scale)), multiplicity * ((eq19_e440_d_n8 * ddt_scale)), multiplicity * ((eq19_e440_d_n10 * ddt_scale)), multiplicity * ((eq19_e440_d_n11 * ddt_scale)), multiplicity * ((eq19_e440_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq25_e484, eq25_e484_d_n0, eq25_e484_d_n1, eq25_e484_d_n2, eq25_e484_d_n4, eq25_e484_d_n5, eq25_e484_d_n6, eq25_e484_d_n8, eq25_e484_d_n10, eq25_e484_d_n11, eq25_e484_d_n12,) = {
    if (p.p25 != 0.0) {
        let eq25_e482: f64 = (l.f62e * (nv1 - nv5));let eq25_e482_d_n0: f64 = (l.f62f * (nv1 - nv5));let eq25_e482_d_n2: f64 = (l.f633 * (nv1 - nv5));let eq25_e482_d_n4: f64 = (l.f634 * (nv1 - nv5));let eq25_e482_d_n5: f64 = ((l.f635 * (nv1 - nv5)) + (-l.f62e));let eq25_e482_d_n6: f64 = (l.f636 * (nv1 - nv5));let eq25_e482_d_n8: f64 = (l.f637 * (nv1 - nv5));let eq25_e482_d_n10: f64 = (l.f630 * (nv1 - nv5));let eq25_e482_d_n11: f64 = (l.f631 * (nv1 - nv5));let eq25_e482_d_n12: f64 = (l.f632 * (nv1 - nv5));
        (eq25_e482, eq25_e482_d_n0, l.f62e, eq25_e482_d_n2, eq25_e482_d_n4, eq25_e482_d_n5, eq25_e482_d_n6, eq25_e482_d_n8, eq25_e482_d_n10, eq25_e482_d_n11, eq25_e482_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e484;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq25_value),
            [0, 1, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq25_e484_d_n0), multiplicity * (eq25_e484_d_n1), multiplicity * (eq25_e484_d_n2), multiplicity * (eq25_e484_d_n4), multiplicity * (eq25_e484_d_n5), multiplicity * (eq25_e484_d_n6), multiplicity * (eq25_e484_d_n8), multiplicity * (eq25_e484_d_n10), multiplicity * (eq25_e484_d_n11), multiplicity * (eq25_e484_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq26_e489,) = {
    if (p.p25 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e489;
        stamper.stamp_potential_const_local(
            2,
            eq26_value,
        );let eq27_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            3,
            eq27_value,
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
        let nv4 = ctx.node_voltage(nodes[4]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq28_e504, eq28_e504_d_n0, eq28_e504_d_n2, eq28_e504_d_n4, eq28_e504_d_n5, eq28_e504_d_n6, eq28_e504_d_n8, eq28_e504_d_n10, eq28_e504_d_n11, eq28_e504_d_n12,) = {
    if (l.f868 != 0.0) {
        let eq28_e493: f64 = (-l.f140b);let eq28_e496: f64 = (l.f327 * (nv4 - 0.0));let eq28_e496_d_n0: f64 = (l.f328 * (nv4 - 0.0));let eq28_e496_d_n2: f64 = (l.f32c * (nv4 - 0.0));let eq28_e496_d_n4: f64 = ((l.f32d * (nv4 - 0.0)) + l.f327);let eq28_e496_d_n5: f64 = (l.f32e * (nv4 - 0.0));let eq28_e496_d_n6: f64 = (l.f32f * (nv4 - 0.0));let eq28_e496_d_n8: f64 = (l.f330 * (nv4 - 0.0));let eq28_e496_d_n10: f64 = (l.f329 * (nv4 - 0.0));let eq28_e496_d_n11: f64 = (l.f32a * (nv4 - 0.0));let eq28_e496_d_n12: f64 = (l.f32b * (nv4 - 0.0));let eq28_e497: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e496);let eq28_e498: f64 = (eq28_e493 + eq28_e497);let eq28_e498_d_n0: f64 = ((-l.f140c) + (eq28_e496_d_n0 * ddt_scale));let eq28_e498_d_n2: f64 = ((-l.f1410) + (eq28_e496_d_n2 * ddt_scale));let eq28_e498_d_n4: f64 = ((-l.f1411) + (eq28_e496_d_n4 * ddt_scale));let eq28_e498_d_n5: f64 = ((-l.f1412) + (eq28_e496_d_n5 * ddt_scale));let eq28_e498_d_n6: f64 = ((-l.f1413) + (eq28_e496_d_n6 * ddt_scale));let eq28_e498_d_n8: f64 = ((-l.f1414) + (eq28_e496_d_n8 * ddt_scale));let eq28_e498_d_n10: f64 = ((-l.f140d) + (eq28_e496_d_n10 * ddt_scale));let eq28_e498_d_n11: f64 = ((-l.f140e) + (eq28_e496_d_n11 * ddt_scale));let eq28_e498_d_n12: f64 = ((-l.f140f) + (eq28_e496_d_n12 * ddt_scale));let eq28_e501: f64 = ((nv4 - 0.0) * l.f638);let eq28_e501_d_n0: f64 = ((nv4 - 0.0) * l.f639);let eq28_e501_d_n2: f64 = ((nv4 - 0.0) * l.f63d);let eq28_e501_d_n4: f64 = (l.f638 + ((nv4 - 0.0) * l.f63e));let eq28_e501_d_n5: f64 = ((nv4 - 0.0) * l.f63f);let eq28_e501_d_n6: f64 = ((nv4 - 0.0) * l.f640);let eq28_e501_d_n8: f64 = ((nv4 - 0.0) * l.f641);let eq28_e501_d_n10: f64 = ((nv4 - 0.0) * l.f63a);let eq28_e501_d_n11: f64 = ((nv4 - 0.0) * l.f63b);let eq28_e501_d_n12: f64 = ((nv4 - 0.0) * l.f63c);let eq28_e502: f64 = (eq28_e498 + eq28_e501);let eq28_e502_d_n0: f64 = (eq28_e498_d_n0 + eq28_e501_d_n0);let eq28_e502_d_n2: f64 = (eq28_e498_d_n2 + eq28_e501_d_n2);let eq28_e502_d_n4: f64 = (eq28_e498_d_n4 + eq28_e501_d_n4);let eq28_e502_d_n5: f64 = (eq28_e498_d_n5 + eq28_e501_d_n5);let eq28_e502_d_n6: f64 = (eq28_e498_d_n6 + eq28_e501_d_n6);let eq28_e502_d_n8: f64 = (eq28_e498_d_n8 + eq28_e501_d_n8);let eq28_e502_d_n10: f64 = (eq28_e498_d_n10 + eq28_e501_d_n10);let eq28_e502_d_n11: f64 = (eq28_e498_d_n11 + eq28_e501_d_n11);let eq28_e502_d_n12: f64 = (eq28_e498_d_n12 + eq28_e501_d_n12);
        (eq28_e502, eq28_e502_d_n0, eq28_e502_d_n2, eq28_e502_d_n4, eq28_e502_d_n5, eq28_e502_d_n6, eq28_e502_d_n8, eq28_e502_d_n10, eq28_e502_d_n11, eq28_e502_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e504;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            None,
            multiplicity * (eq28_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq28_e504_d_n0), multiplicity * (eq28_e504_d_n2), multiplicity * (eq28_e504_d_n4), multiplicity * (eq28_e504_d_n5), multiplicity * (eq28_e504_d_n6), multiplicity * (eq28_e504_d_n8), multiplicity * (eq28_e504_d_n10), multiplicity * (eq28_e504_d_n11), multiplicity * (eq28_e504_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq29_e509,) = {
    if (l.f868 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e509;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e518, eq30_e518_d_n0, eq30_e518_d_n2, eq30_e518_d_n4, eq30_e518_d_n5, eq30_e518_d_n6, eq30_e518_d_n8, eq30_e518_d_n10, eq30_e518_d_n11, eq30_e518_d_n12,) = {
    if (l.f86a != 0.0) {
        let eq30_e514: f64 = (1e-9 * (nv10 - 0.0));let eq30_e515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq30_e514);let eq30_e516: f64 = (l.fa4b + eq30_e515);let eq30_e516_d_n10: f64 = (l.fa4d + (1e-9 * ddt_scale));
        (eq30_e516, l.fa4c, l.fa50, l.fa51, l.fa52, l.fa53, l.fa54, eq30_e516_d_n10, l.fa4e, l.fa4f,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e518;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            None,
            multiplicity * (eq30_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq30_e518_d_n0), multiplicity * (eq30_e518_d_n2), multiplicity * (eq30_e518_d_n4), multiplicity * (eq30_e518_d_n5), multiplicity * (eq30_e518_d_n6), multiplicity * (eq30_e518_d_n8), multiplicity * (eq30_e518_d_n10), multiplicity * (eq30_e518_d_n11), multiplicity * (eq30_e518_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq31_e523,) = {
    if (l.f86a == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e523;
        stamper.stamp_potential_const_local(
            5,
            eq31_value,
        );
        let (eq32_e532, eq32_e532_d_n0, eq32_e532_d_n2, eq32_e532_d_n4, eq32_e532_d_n5, eq32_e532_d_n6, eq32_e532_d_n8, eq32_e532_d_n10, eq32_e532_d_n11, eq32_e532_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq32_e528: f64 = (1e-9 * (nv8 - 0.0));let eq32_e529: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq32_e528);let eq32_e530: f64 = (l.fa56 + eq32_e529);let eq32_e530_d_n8: f64 = (l.fa5f + (1e-9 * ddt_scale));
        (eq32_e530, l.fa57, l.fa5b, l.fa5c, l.fa5d, l.fa5e, eq32_e530_d_n8, l.fa58, l.fa59, l.fa5a,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e532;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            None,
            multiplicity * (eq32_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq32_e532_d_n0), multiplicity * (eq32_e532_d_n2), multiplicity * (eq32_e532_d_n4), multiplicity * (eq32_e532_d_n5), multiplicity * (eq32_e532_d_n6), multiplicity * (eq32_e532_d_n8), multiplicity * (eq32_e532_d_n10), multiplicity * (eq32_e532_d_n11), multiplicity * (eq32_e532_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq33_e541, eq33_e541_d_n0, eq33_e541_d_n2, eq33_e541_d_n4, eq33_e541_d_n5, eq33_e541_d_n6, eq33_e541_d_n8, eq33_e541_d_n9, eq33_e541_d_n10, eq33_e541_d_n11, eq33_e541_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq33_e537: f64 = (1e-9 * (nv9 - 0.0));let eq33_e538: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq33_e537);let eq33_e539: f64 = (l.fa3f + eq33_e538);let eq33_e539_d_n9: f64 = (l.fa49 + (1e-9 * ddt_scale));
        (eq33_e539, l.fa40, l.fa44, l.fa45, l.fa46, l.fa47, l.fa48, eq33_e539_d_n9, l.fa41, l.fa42, l.fa43,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e541;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            None,
            multiplicity * (eq33_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e541_d_n0), multiplicity * (eq33_e541_d_n2), multiplicity * (eq33_e541_d_n4), multiplicity * (eq33_e541_d_n5), multiplicity * (eq33_e541_d_n6), multiplicity * (eq33_e541_d_n8), multiplicity * (eq33_e541_d_n9), multiplicity * (eq33_e541_d_n10), multiplicity * (eq33_e541_d_n11), multiplicity * (eq33_e541_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq34_e546,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e546;
        stamper.stamp_potential_const_local(
            6,
            eq34_value,
        );
        let (eq35_e551,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e551;
        stamper.stamp_potential_const_local(
            7,
            eq35_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let eq10_e393: f64 = (l.f12b5 + l.f12bf);let eq10_e393_d_n8: f64 = (l.f12be + l.f12c0);let eq10_e394_q: f64 = eq10_e393;let eq10_e395: f64 = (p.p33 * eq10_e393);let eq10_e395_d_n0: f64 = (p.p33 * l.f12b6);let eq10_e395_d_n2: f64 = (p.p33 * l.f12ba);let eq10_e395_d_n4: f64 = (p.p33 * l.f12bb);let eq10_e395_d_n5: f64 = (p.p33 * l.f12bc);let eq10_e395_d_n6: f64 = (p.p33 * l.f12bd);let eq10_e395_d_n8: f64 = (p.p33 * eq10_e393_d_n8);let eq10_e395_d_n9: f64 = (p.p33 * l.f12c1);let eq10_e395_d_n10: f64 = (p.p33 * l.f12b7);let eq10_e395_d_n11: f64 = (p.p33 * l.f12b8);let eq10_e395_d_n12: f64 = (p.p33 * l.f12b9);let eq10_e395_q: f64 = (p.p33 * eq10_e394_q);let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e395_d_n0, 0.0, eq10_e395_d_n2, 0.0, eq10_e395_d_n4, eq10_e395_d_n5, eq10_e395_d_n6, 0.0, eq10_e395_d_n8, eq10_e395_d_n9, eq10_e395_d_n10, eq10_e395_d_n11, eq10_e395_d_n12];let eq10_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(12),
            &eq10_reactive_node_derivatives,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );let eq11_e399: f64 = (l.f125b + l.f1265);let eq11_e399_d_n0: f64 = (l.f125c + l.f1266);let eq11_e399_d_n2: f64 = (l.f1260 + l.f126a);let eq11_e399_d_n4: f64 = (l.f1261 + l.f126b);let eq11_e399_d_n5: f64 = (l.f1262 + l.f126c);let eq11_e399_d_n6: f64 = (l.f1263 + l.f126d);let eq11_e399_d_n8: f64 = (l.f1264 + l.f126e);let eq11_e399_d_n10: f64 = (l.f125d + l.f1267);let eq11_e399_d_n11: f64 = (l.f125e + l.f1268);let eq11_e399_d_n12: f64 = (l.f125f + l.f1269);let eq11_e400_q: f64 = eq11_e399;let eq11_e401: f64 = (p.p33 * eq11_e399);let eq11_e401_d_n0: f64 = (p.p33 * eq11_e399_d_n0);let eq11_e401_d_n2: f64 = (p.p33 * eq11_e399_d_n2);let eq11_e401_d_n4: f64 = (p.p33 * eq11_e399_d_n4);let eq11_e401_d_n5: f64 = (p.p33 * eq11_e399_d_n5);let eq11_e401_d_n6: f64 = (p.p33 * eq11_e399_d_n6);let eq11_e401_d_n8: f64 = (p.p33 * eq11_e399_d_n8);let eq11_e401_d_n10: f64 = (p.p33 * eq11_e399_d_n10);let eq11_e401_d_n11: f64 = (p.p33 * eq11_e399_d_n11);let eq11_e401_d_n12: f64 = (p.p33 * eq11_e399_d_n12);let eq11_e401_q: f64 = (p.p33 * eq11_e400_q);let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e401_d_n0, 0.0, eq11_e401_d_n2, 0.0, eq11_e401_d_n4, eq11_e401_d_n5, eq11_e401_d_n6, 0.0, eq11_e401_d_n8, 0.0, eq11_e401_d_n10, eq11_e401_d_n11, eq11_e401_d_n12];let eq11_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            Some(12),
            &eq11_reactive_node_derivatives,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );let eq12_e405: f64 = (l.f11fe + l.f1220);let eq12_e406_q: f64 = eq12_e405;let eq12_e407: f64 = (p.p33 * eq12_e405);let eq12_e407_d_n0: f64 = (p.p33 * l.f1217);let eq12_e407_d_n2: f64 = (p.p33 * l.f121b);let eq12_e407_d_n4: f64 = (p.p33 * l.f121c);let eq12_e407_d_n5: f64 = (p.p33 * l.f121d);let eq12_e407_d_n6: f64 = (p.p33 * l.f121e);let eq12_e407_d_n8: f64 = (p.p33 * l.f121f);let eq12_e407_d_n9: f64 = (p.p33 * l.f1221);let eq12_e407_d_n10: f64 = (p.p33 * l.f1218);let eq12_e407_d_n11: f64 = (p.p33 * l.f1219);let eq12_e407_d_n12: f64 = (p.p33 * l.f121a);let eq12_e407_q: f64 = (p.p33 * eq12_e406_q);let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e407_d_n0, 0.0, eq12_e407_d_n2, 0.0, eq12_e407_d_n4, eq12_e407_d_n5, eq12_e407_d_n6, 0.0, eq12_e407_d_n8, eq12_e407_d_n9, eq12_e407_d_n10, eq12_e407_d_n11, eq12_e407_d_n12];let eq12_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(6),
            Some(12),
            &eq12_reactive_node_derivatives,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );let eq18_e436: f64 = ((nv7 - 0.0) * l.f1497);let eq18_e436_d_n0: f64 = ((nv7 - 0.0) * l.f1498);let eq18_e436_d_n2: f64 = ((nv7 - 0.0) * l.f149c);let eq18_e436_d_n4: f64 = ((nv7 - 0.0) * l.f149d);let eq18_e436_d_n5: f64 = ((nv7 - 0.0) * l.f149e);let eq18_e436_d_n6: f64 = ((nv7 - 0.0) * l.f149f);let eq18_e436_d_n8: f64 = ((nv7 - 0.0) * l.f14a0);let eq18_e436_d_n10: f64 = ((nv7 - 0.0) * l.f1499);let eq18_e436_d_n11: f64 = ((nv7 - 0.0) * l.f149a);let eq18_e436_d_n12: f64 = ((nv7 - 0.0) * l.f149b);let eq18_e437_q: f64 = eq18_e436;let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e436_d_n0, 0.0, eq18_e436_d_n2, 0.0, eq18_e436_d_n4, eq18_e436_d_n5, eq18_e436_d_n6, l.f1497, eq18_e436_d_n8, 0.0, eq18_e436_d_n10, eq18_e436_d_n11, eq18_e436_d_n12];let eq18_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(12),
            &eq18_reactive_node_derivatives,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        l: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let eq19_e440: f64 = ((nv7 - 0.0) * l.f1482);let eq19_e440_d_n0: f64 = ((nv7 - 0.0) * l.f1483);let eq19_e440_d_n2: f64 = ((nv7 - 0.0) * l.f1487);let eq19_e440_d_n4: f64 = ((nv7 - 0.0) * l.f1488);let eq19_e440_d_n5: f64 = ((nv7 - 0.0) * l.f1489);let eq19_e440_d_n6: f64 = ((nv7 - 0.0) * l.f148a);let eq19_e440_d_n8: f64 = ((nv7 - 0.0) * l.f148b);let eq19_e440_d_n10: f64 = ((nv7 - 0.0) * l.f1484);let eq19_e440_d_n11: f64 = ((nv7 - 0.0) * l.f1485);let eq19_e440_d_n12: f64 = ((nv7 - 0.0) * l.f1486);let eq19_e441_q: f64 = eq19_e440;let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e440_d_n0, 0.0, eq19_e440_d_n2, 0.0, eq19_e440_d_n4, eq19_e440_d_n5, eq19_e440_d_n6, l.f1482, eq19_e440_d_n8, 0.0, eq19_e440_d_n10, eq19_e440_d_n11, eq19_e440_d_n12];let eq19_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(5),
            Some(11),
            &eq19_reactive_node_derivatives,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e504, eq28_e504_d_n0, eq28_e504_d_n2, eq28_e504_d_n4, eq28_e504_d_n5, eq28_e504_d_n6, eq28_e504_d_n8, eq28_e504_d_n10, eq28_e504_d_n11, eq28_e504_d_n12, eq28_e504_q, eq28_e504_q_d_n0, eq28_e504_q_d_n2, eq28_e504_q_d_n4, eq28_e504_q_d_n5, eq28_e504_q_d_n6, eq28_e504_q_d_n8, eq28_e504_q_d_n10, eq28_e504_q_d_n11, eq28_e504_q_d_n12,) = {
    if (l.f868 != 0.0) {
        let eq28_e493: f64 = (-l.f140b);let eq28_e496: f64 = (l.f327 * (nv4 - 0.0));let eq28_e496_d_n0: f64 = (l.f328 * (nv4 - 0.0));let eq28_e496_d_n2: f64 = (l.f32c * (nv4 - 0.0));let eq28_e496_d_n4: f64 = ((l.f32d * (nv4 - 0.0)) + l.f327);let eq28_e496_d_n5: f64 = (l.f32e * (nv4 - 0.0));let eq28_e496_d_n6: f64 = (l.f32f * (nv4 - 0.0));let eq28_e496_d_n8: f64 = (l.f330 * (nv4 - 0.0));let eq28_e496_d_n10: f64 = (l.f329 * (nv4 - 0.0));let eq28_e496_d_n11: f64 = (l.f32a * (nv4 - 0.0));let eq28_e496_d_n12: f64 = (l.f32b * (nv4 - 0.0));let eq28_e497_q: f64 = eq28_e496;let eq28_e498: f64 = (eq28_e493 + eq28_e496);let eq28_e498_d_n0: f64 = ((-l.f140c) + eq28_e496_d_n0);let eq28_e498_d_n2: f64 = ((-l.f1410) + eq28_e496_d_n2);let eq28_e498_d_n4: f64 = ((-l.f1411) + eq28_e496_d_n4);let eq28_e498_d_n5: f64 = ((-l.f1412) + eq28_e496_d_n5);let eq28_e498_d_n6: f64 = ((-l.f1413) + eq28_e496_d_n6);let eq28_e498_d_n8: f64 = ((-l.f1414) + eq28_e496_d_n8);let eq28_e498_d_n10: f64 = ((-l.f140d) + eq28_e496_d_n10);let eq28_e498_d_n11: f64 = ((-l.f140e) + eq28_e496_d_n11);let eq28_e498_d_n12: f64 = ((-l.f140f) + eq28_e496_d_n12);let eq28_e498_q: f64 = eq28_e497_q;let eq28_e501: f64 = ((nv4 - 0.0) * l.f638);let eq28_e501_d_n0: f64 = ((nv4 - 0.0) * l.f639);let eq28_e501_d_n2: f64 = ((nv4 - 0.0) * l.f63d);let eq28_e501_d_n4: f64 = (l.f638 + ((nv4 - 0.0) * l.f63e));let eq28_e501_d_n5: f64 = ((nv4 - 0.0) * l.f63f);let eq28_e501_d_n6: f64 = ((nv4 - 0.0) * l.f640);let eq28_e501_d_n8: f64 = ((nv4 - 0.0) * l.f641);let eq28_e501_d_n10: f64 = ((nv4 - 0.0) * l.f63a);let eq28_e501_d_n11: f64 = ((nv4 - 0.0) * l.f63b);let eq28_e501_d_n12: f64 = ((nv4 - 0.0) * l.f63c);let eq28_e502: f64 = (eq28_e498 + eq28_e501);let eq28_e502_d_n0: f64 = (eq28_e498_d_n0 + eq28_e501_d_n0);let eq28_e502_d_n2: f64 = (eq28_e498_d_n2 + eq28_e501_d_n2);let eq28_e502_d_n4: f64 = (eq28_e498_d_n4 + eq28_e501_d_n4);let eq28_e502_d_n5: f64 = (eq28_e498_d_n5 + eq28_e501_d_n5);let eq28_e502_d_n6: f64 = (eq28_e498_d_n6 + eq28_e501_d_n6);let eq28_e502_d_n8: f64 = (eq28_e498_d_n8 + eq28_e501_d_n8);let eq28_e502_d_n10: f64 = (eq28_e498_d_n10 + eq28_e501_d_n10);let eq28_e502_d_n11: f64 = (eq28_e498_d_n11 + eq28_e501_d_n11);let eq28_e502_d_n12: f64 = (eq28_e498_d_n12 + eq28_e501_d_n12);let eq28_e502_q: f64 = eq28_e498_q;
        (eq28_e502, eq28_e502_d_n0, eq28_e502_d_n2, eq28_e502_d_n4, eq28_e502_d_n5, eq28_e502_d_n6, eq28_e502_d_n8, eq28_e502_d_n10, eq28_e502_d_n11, eq28_e502_d_n12, eq28_e502_q, eq28_e496_d_n0, eq28_e496_d_n2, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n8, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e504_q_d_n0, 0.0, eq28_e504_q_d_n2, 0.0, eq28_e504_q_d_n4, eq28_e504_q_d_n5, eq28_e504_q_d_n6, 0.0, eq28_e504_q_d_n8, 0.0, eq28_e504_q_d_n10, eq28_e504_q_d_n11, eq28_e504_q_d_n12];let eq28_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &eq28_reactive_node_derivatives,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e518, eq30_e518_d_n0, eq30_e518_d_n2, eq30_e518_d_n4, eq30_e518_d_n5, eq30_e518_d_n6, eq30_e518_d_n8, eq30_e518_d_n10, eq30_e518_d_n11, eq30_e518_d_n12, eq30_e518_q, eq30_e518_q_d_n10,) = {
    if (l.f86a != 0.0) {
        let eq30_e514: f64 = (1e-9 * (nv10 - 0.0));let eq30_e515_q: f64 = eq30_e514;let eq30_e516: f64 = (l.fa4b + eq30_e514);let eq30_e516_d_n10: f64 = (l.fa4d + 1e-9);let eq30_e516_q: f64 = eq30_e515_q;
        (eq30_e516, l.fa4c, l.fa50, l.fa51, l.fa52, l.fa53, l.fa54, eq30_e516_d_n10, l.fa4e, l.fa4f, eq30_e516_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(10),
            None,
            10,
            multiplicity * (eq30_e518_q_d_n10),
        );
        let (eq32_e532, eq32_e532_d_n0, eq32_e532_d_n2, eq32_e532_d_n4, eq32_e532_d_n5, eq32_e532_d_n6, eq32_e532_d_n8, eq32_e532_d_n10, eq32_e532_d_n11, eq32_e532_d_n12, eq32_e532_q, eq32_e532_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e528: f64 = (1e-9 * (nv8 - 0.0));let eq32_e529_q: f64 = eq32_e528;let eq32_e530: f64 = (l.fa56 + eq32_e528);let eq32_e530_d_n8: f64 = (l.fa5f + 1e-9);let eq32_e530_q: f64 = eq32_e529_q;
        (eq32_e530, l.fa57, l.fa5b, l.fa5c, l.fa5d, l.fa5e, eq32_e530_d_n8, l.fa58, l.fa59, l.fa5a, eq32_e530_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(8),
            None,
            8,
            multiplicity * (eq32_e532_q_d_n8),
        );
        let (eq33_e541, eq33_e541_d_n0, eq33_e541_d_n2, eq33_e541_d_n4, eq33_e541_d_n5, eq33_e541_d_n6, eq33_e541_d_n8, eq33_e541_d_n9, eq33_e541_d_n10, eq33_e541_d_n11, eq33_e541_d_n12, eq33_e541_q, eq33_e541_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e537: f64 = (1e-9 * (nv9 - 0.0));let eq33_e538_q: f64 = eq33_e537;let eq33_e539: f64 = (l.fa3f + eq33_e537);let eq33_e539_d_n9: f64 = (l.fa49 + 1e-9);let eq33_e539_q: f64 = eq33_e538_q;
        (eq33_e539, l.fa40, l.fa44, l.fa45, l.fa46, l.fa47, l.fa48, eq33_e539_d_n9, l.fa41, l.fa42, l.fa43, eq33_e539_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(9),
            None,
            9,
            multiplicity * (eq33_e541_q_d_n9),
        );
    }
}
