#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_offset(16, 15, 1.0);

        s.v[1365] = if ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015))) { 1.0 } else { 0.0 };

        if (s.v[1365] != 0.0) {
            s.store_div_from_scalar_ad(133, ((-0.0015) * 0.0015), A::scale(s.ad_value(16), 16.0));
        }

        if (!(s.v[1365] != 0.0)) {
            s.store_scale_ad(133, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        s.store_div_from_scalar_ad(235, 1.0, A::scale(A::pow_from_scalar((s.v[29] * 1000000.0), s.ad_value(527)), p.p2));

        s.v[1366] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1366] != 0.0) {
            s.store_scalar(243, 0.0);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_offset_ad(12, A::mul(s.ad_value(526), s.ad_value(130)), 1.0);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_mul_ad_rhs(13, 543, A::sub(s.ad_value(111), s.ad_value(128)));
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_add_ad_lhs(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);
        }

        if (!(s.v[1366] != 0.0)) {
            s.store_add_ad_rhs(15, 14, A::sqrt(A::offset(A::square(s.ad_value(14)), 0.01)));
        }

        s.v[1367] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1366] != 0.0)) && (s.v[1367] != 0.0)) {
            s.store_mul_ad_lhs(243, A::scale(A::mul(A::add(s.ad_value(533), A::mul(s.ad_value(532), s.ad_value(15))), s.ad_value(235)), p.p2), 408);
        }

        if ((!(s.v[1366] != 0.0)) && (!(s.v[1367] != 0.0))) {
            s.store_mul_ad_lhs(243, A::add(A::add(s.ad_value(239), A::scale(A::mul(A::add(s.ad_value(533), A::mul(s.ad_value(532), s.ad_value(15))), s.ad_value(235)), p.p2)), s.ad_value(240)), 408);
        }

        s.store_ad(12, &A::pow(s.ad_value(133), A::div_from_scalar(1.0, s.ad_value(166))));

        s.store_mul(23, 453, 61);

        s.store_sqrt_ad(24, A::offset(A::square(s.ad_value(23)), 0.1));

        s.store_scale_ad(13, A::add(A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(23)), A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24)))), 0.5);

        s.store_div_ad(14, A::mul(A::scale(s.ad_value(200), (10.0 * p.p433)), s.ad_value(13)), A::offset(A::mul(s.ad_value(200), s.ad_value(13)), (10.0 * p.p433)));

        s.v[1368] = if (s.v[536] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1368] != 0.0) {
            s.store_mul_ad(138, A::scale(A::div(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), A::scale(s.ad_value(502), s.v[30])), 2.0), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))));
        }

        if (!(s.v[1368] != 0.0)) {
            s.store_mul_ad(138, A::scale(A::div(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), A::scale(s.ad_value(502), s.v[30])), 2.0), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0));
        }

        s.v[1369] = if (s.v[243] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1369] != 0.0) {
            s.store_mul_ad_lhs(23, A::mul(A::scale(s.ad_value(90), ((s.v[29] * 2.0) * s.v[46])), s.ad_value(106)), 502);
        }

        if (s.v[1369] != 0.0) {
            s.store_div_ad(24, A::mul(A::mul(s.ad_value(23), s.ad_value(138)), s.ad_value(243)), A::scale(s.ad_value(106), 2.0));
        }

        if (s.v[1369] != 0.0) {
            s.store_div_ad(12, A::mul(A::scale(s.ad_value(138), 0.5), A::add(A::square(s.ad_value(200)), s.ad_value(200))), A::offset(A::mul(A::scale(s.ad_value(138), 0.5), A::offset(s.ad_value(200), 1.0)), 1.0));
        }

        if (s.v[1369] != 0.0) {
            s.store_mul_ad(13, A::scale(s.ad_value(138), 2.0), A::sub(s.ad_value(200), s.ad_value(12)));
        }

        if (s.v[1369] != 0.0) {
            s.store_sqrt_ad(14, A::offset(A::square(s.ad_value(13)), 1.0));
        }

        s.v[1370] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1369] != 0.0) && (s.v[1370] != 0.0)) {
            s.store_asinh(147, 13);
        }

        if ((s.v[1369] != 0.0) && (s.v[1370] != 0.0)) {
            s.store_add_ad_rhs(15, 14, A::mul(A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147)));
        }

        if ((s.v[1369] != 0.0) && (!(s.v[1370] != 0.0))) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (s.v[1369] != 0.0) {
            s.store_sub_ad(16, A::add(A::mul(s.ad_value(12), s.ad_value(15)), A::mul(A::mul(s.ad_value(24), s.ad_value(12)), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0))), A::mul(s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12)))));
        }

        s.v[1371] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1369] != 0.0) && (s.v[1371] != 0.0)) {
            s.store_div_ad(17, A::mul(A::scale(s.ad_value(138), (-2.0)), A::sub(A::mul(s.ad_value(13), s.ad_value(14)), s.ad_value(147))), A::square(s.ad_value(13)));
        }

        if ((s.v[1369] != 0.0) && (!(s.v[1371] != 0.0))) {
            s.store_mul_ad(17, A::scale(s.ad_value(138), (-2.0)), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (s.v[1369] != 0.0) {
            s.store_add_ad(18, A::add(A::add(A::mul(s.ad_value(12), s.ad_value(17)), s.ad_value(15)), A::mul(s.ad_value(24), A::offset(A::add(s.ad_value(200), A::scale(s.ad_value(12), 2.0)), 1.0))), A::mul(s.ad_value(138), A::offset(A::scale(s.ad_value(12), 2.0), 1.0)));
        }

        if (s.v[1369] != 0.0) {
            s.store_sub_ad_rhs(12, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        if (s.v[1369] != 0.0) {
            s.store_mul_ad(13, A::scale(s.ad_value(138), 2.0), A::sub(s.ad_value(200), s.ad_value(12)));
        }

        if (s.v[1369] != 0.0) {
            s.store_sqrt_ad(14, A::offset(A::square(s.ad_value(13)), 1.0));
        }

        s.v[1372] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1369] != 0.0) && (s.v[1372] != 0.0)) {
            s.store_asinh(147, 13);
        }

        if ((s.v[1369] != 0.0) && (s.v[1372] != 0.0)) {
            s.store_add_ad_rhs(15, 14, A::mul(A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147)));
        }

        if ((s.v[1369] != 0.0) && (!(s.v[1372] != 0.0))) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (s.v[1369] != 0.0) {
            s.store_sub_ad(16, A::add(A::mul(s.ad_value(12), s.ad_value(15)), A::mul(A::mul(s.ad_value(24), s.ad_value(12)), A::offset(A::add(s.ad_value(200), s.ad_value(12)), 1.0))), A::mul(s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12)))));
        }

        s.v[1373] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1369] != 0.0) && (s.v[1373] != 0.0)) {
            s.store_div_ad(17, A::mul(A::scale(s.ad_value(138), (-2.0)), A::sub(A::mul(s.ad_value(13), s.ad_value(14)), s.ad_value(147))), A::square(s.ad_value(13)));
        }

        if ((s.v[1369] != 0.0) && (!(s.v[1373] != 0.0))) {
            s.store_mul_ad(17, A::scale(s.ad_value(138), (-2.0)), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (s.v[1369] != 0.0) {
            s.store_add_ad(18, A::add(A::add(A::mul(s.ad_value(12), s.ad_value(17)), s.ad_value(15)), A::mul(s.ad_value(24), A::offset(A::add(s.ad_value(200), A::scale(s.ad_value(12), 2.0)), 1.0))), A::mul(s.ad_value(138), A::offset(A::scale(s.ad_value(12), 2.0), 1.0)));
        }

        if (s.v[1369] != 0.0) {
            s.store_sub_ad_rhs(131, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_div_ad(12, A::mul(A::scale(s.ad_value(138), 0.5), A::add(A::square(s.ad_value(200)), s.ad_value(200))), A::offset(A::mul(A::scale(s.ad_value(138), 0.5), A::offset(s.ad_value(200), 1.0)), 1.0));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_mul_ad(13, A::scale(s.ad_value(138), 2.0), A::sub(s.ad_value(200), s.ad_value(12)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_sqrt_ad(14, A::offset(A::square(s.ad_value(13)), 1.0));
        }

        s.v[1374] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1369] != 0.0)) && (s.v[1374] != 0.0)) {
            s.store_asinh(147, 13);
        }

        if ((!(s.v[1369] != 0.0)) && (s.v[1374] != 0.0)) {
            s.store_add_ad_rhs(15, 14, A::mul(A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147)));
        }

        if ((!(s.v[1369] != 0.0)) && (!(s.v[1374] != 0.0))) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_sub_ad(16, A::mul(s.ad_value(12), s.ad_value(15)), A::mul(s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12)))));
        }

        s.v[1375] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1369] != 0.0)) && (s.v[1375] != 0.0)) {
            s.store_div_ad(17, A::mul(A::scale(s.ad_value(138), (-2.0)), A::sub(A::mul(s.ad_value(13), s.ad_value(14)), s.ad_value(147))), A::square(s.ad_value(13)));
        }

        if ((!(s.v[1369] != 0.0)) && (!(s.v[1375] != 0.0))) {
            s.store_mul_ad(17, A::scale(s.ad_value(138), (-2.0)), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_add_ad(18, A::add(A::mul(s.ad_value(12), s.ad_value(17)), s.ad_value(15)), A::mul(s.ad_value(138), A::offset(A::scale(s.ad_value(12), 2.0), 1.0)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_sub_ad_rhs(12, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_mul_ad(13, A::scale(s.ad_value(138), 2.0), A::sub(s.ad_value(200), s.ad_value(12)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_sqrt_ad(14, A::offset(A::square(s.ad_value(13)), 1.0));
        }

        s.v[1376] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1369] != 0.0)) && (s.v[1376] != 0.0)) {
            s.store_asinh(147, 13);
        }

        if ((!(s.v[1369] != 0.0)) && (s.v[1376] != 0.0)) {
            s.store_add_ad_rhs(15, 14, A::mul(A::div_from_scalar(1.0, s.ad_value(13)), s.ad_value(147)));
        }

        if ((!(s.v[1369] != 0.0)) && (!(s.v[1376] != 0.0))) {
            s.store_add_ad_rhs(15, 14, A::div_from_scalar(1.0, s.ad_value(14)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_sub_ad(16, A::mul(s.ad_value(12), s.ad_value(15)), A::mul(s.ad_value(138), A::sub(A::add(A::square(s.ad_value(200)), s.ad_value(200)), A::add(A::square(s.ad_value(12)), s.ad_value(12)))));
        }

        s.v[1377] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1369] != 0.0)) && (s.v[1377] != 0.0)) {
            s.store_div_ad(17, A::mul(A::scale(s.ad_value(138), (-2.0)), A::sub(A::mul(s.ad_value(13), s.ad_value(14)), s.ad_value(147))), A::square(s.ad_value(13)));
        }

        if ((!(s.v[1369] != 0.0)) && (!(s.v[1377] != 0.0))) {
            s.store_mul_ad(17, A::scale(s.ad_value(138), (-2.0)), A::div(s.ad_value(13), s.ad_value(14)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_add_ad(18, A::add(A::mul(s.ad_value(12), s.ad_value(17)), s.ad_value(15)), A::mul(s.ad_value(138), A::offset(A::scale(s.ad_value(12), 2.0), 1.0)));
        }

        if (!(s.v[1369] != 0.0)) {
            s.store_sub_ad_rhs(131, 12, A::div(s.ad_value(16), s.ad_value(18)));
        }

        let assign18670_ad_e26371: A = A::sub(A::sub(s.ad_value(91), A::scale(s.ad_value(89), 2.0)), A::add(A::scale(s.ad_value(131), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(131), 2.0), s.ad_value(90)), s.ad_value(126)), A::add(A::mul(A::mul(A::scale(s.ad_value(131), 2.0), s.ad_value(90)), s.ad_value(126)), A::div(s.ad_value(125), A::offset(s.ad_value(90), (-1.0))))), 1e-38))));
        s.store_ad(143, &assign18670_ad_e26371);

        s.store_mul(136, 143, 106);

        s.v[1378] = if ((p.p1130 == 0.0) && (p.p1131 == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1378] != 0.0) {
            s.store_scalar(782, 1.0);
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_div_from_scalar_ad(13, s.v[30], A::offset(A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]));
        }

        if (!(s.v[1378] != 0.0)) {
            s.store_offset_ad(782, A::div(A::sub(A::scale(s.ad_value(13), p.p1130), A::mul(A::mul(A::scale(s.ad_value(13), p.p1131), A::powf(s.ad_value(200), p.p1132)), s.ad_value(106))), A::offset(A::scale(s.ad_value(61), p.p1133), 1.0)), 1.0);
        }

        s.v[1379] = if ((0.1 == 0.0) && (s.v[782] < ((-2500.0) * 0.0005))) { 1.0 } else { 0.0 };

        if ((!(s.v[1378] != 0.0)) && (s.v[1379] != 0.0)) {
            s.store_div_from_scalar_ad(782, ((-0.0005) * 0.0005), A::scale(s.ad_value(782), 16.0));
        }

        if ((!(s.v[1378] != 0.0)) && (!(s.v[1379] != 0.0))) {
            s.store_scale_ad(782, A::add(A::offset(s.ad_value(782), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(782), (-0.1)), A::offset(s.ad_value(782), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        s.v[1380] = if ((0.0 == 0.0) && ((s.v[136] - s.v[70]) < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if (s.v[1380] != 0.0) {
            s.store_div_from_scalar_ad(140, ((-0.001) * 0.001), A::scale(A::sub(s.ad_value(136), s.ad_value(70)), 16.0));
        }

        if (!(s.v[1380] != 0.0)) {
            s.store_scale_ad(140, A::add(A::sub(s.ad_value(136), s.ad_value(70)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(136), s.ad_value(70)), A::sub(s.ad_value(136), s.ad_value(70))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        s.store_div(140, 140, 782);

        s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(140)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));

        s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));

        s.store_mul(139, 74, 20);

        s.store_mul_ad_lhs(142, A::add(s.ad_value(139), s.ad_value(70)), 107);

        s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);

        s.store_sqrt(96, 20);

        s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(125), A::scale(s.ad_value(96), 2.0)), 1.0), 125);

        s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(89), 2.0)), 142);

        s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));

        s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);

        s.copy_ad(94, 96);

        s.v[1381] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if (s.v[1381] != 0.0) {
            s.store_scalar(16, (-100.0));
        }

        if (s.v[1381] != 0.0) {
            s.store_scalar(17, 20.0);
        }

        s.v[1382] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((s.v[1381] != 0.0) && (s.v[1382] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1383] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1381] != 0.0) && (!(s.v[1382] != 0.0))) && (s.v[1383] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if (((s.v[1381] != 0.0) && (!(s.v[1382] != 0.0))) && (!(s.v[1383] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if (((s.v[1381] != 0.0) && (!(s.v[1382] != 0.0))) && (!(s.v[1383] != 0.0))) {
            s.store_square(18, 14);
        }

        if (((s.v[1381] != 0.0) && (!(s.v[1382] != 0.0))) && (!(s.v[1383] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if (s.v[1381] != 0.0) {
            s.store_mul_ad_rhs(144, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if (!(s.v[1381] != 0.0)) {
            s.store_sub_ad_rhs(144, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.store_offset_ad(92, A::sub(A::sub(s.ad_value(91), s.ad_value(200)), s.ad_value(144)), (-1.0));

        s.v[1384] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if (s.v[1384] != 0.0) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if (!(s.v[1384] != 0.0)) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        s.store_sqrt(14, 12);

        s.store_offset_ad(90, A::div(s.ad_value(125), A::add(s.ad_value(96), s.ad_value(14))), 1.0);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_div_from_scalar_ad(12, 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)));

        s.store_mul(13, 217, 12);

        s.store_sub_ad(189, A::sub(s.ad_value(59), s.ad_value(91)), A::mul(A::offset(s.ad_value(90), (-1.0)), A::add(A::add(s.ad_value(200), s.ad_value(144)), A::scale(s.ad_value(13), 0.3333333333333333))));

        s.store_scale(14, 90, 0.3333333333333333);

        s.store_mul(15, 13, 12);

        s.store_mul_ad_rhs(190, 14, A::add(A::add(A::scale(s.ad_value(200), 2.0), s.ad_value(144)), A::mul(A::scale(A::add(A::offset(A::scale(s.ad_value(200), 0.8), 1.0), A::scale(s.ad_value(144), 1.2)), 0.5), s.ad_value(15))));

        s.store_mul_ad_rhs(193, 14, A::add(A::add(s.ad_value(200), A::scale(s.ad_value(144), 2.0)), A::mul(A::scale(A::add(A::offset(A::scale(s.ad_value(200), 1.2), 1.0), A::scale(s.ad_value(144), 0.8)), 0.5), s.ad_value(15))));

        s.v[1385] = if ((0.0 == 0.0) && ((s.v[106] * s.v[189]) < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if (s.v[1385] != 0.0) {
            s.store_div_from_scalar_ad(81, ((-0.1) * 0.1), A::scale(A::mul(s.ad_value(106), s.ad_value(189)), 16.0));
        }

        if (!(s.v[1385] != 0.0)) {
            s.store_scale_ad(81, A::add(A::mul(s.ad_value(106), s.ad_value(189)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(106), s.ad_value(189)), A::mul(s.ad_value(106), s.ad_value(189))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        s.store_mul_ad_rhs(80, 106, A::add(s.ad_value(190), s.ad_value(193)));

        s.store_scale_ad(156, A::add(s.ad_value(81), A::scale(s.ad_value(80), s.v[158])), s.v[155]);

        s.store_ad(14, &A::pow(A::scale(A::offset(A::div(s.ad_value(80), s.ad_value(81)), 1.0), 0.5), s.ad_value(513)));

        s.store_add_ad(15, A::mul(A::add(s.ad_value(506), A::mul(s.ad_value(516), s.ad_value(61))), A::pow(s.ad_value(156), s.ad_value(407))), A::div(s.ad_value(510), s.ad_value(14)));

        s.store_offset(16, 15, 1.0);

        s.v[1386] = if ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015))) { 1.0 } else { 0.0 };

        if (s.v[1386] != 0.0) {
            s.store_div_from_scalar_ad(159, ((-0.0015) * 0.0015), A::scale(s.ad_value(16), 16.0));
        }

        if (!(s.v[1386] != 0.0)) {
            s.store_scale_ad(159, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        s.store_div_ad(134, A::scale(s.ad_value(502), 2.0), A::div(s.ad_value(499), s.ad_value(159)));

        s.store_scale(135, 134, s.v[30]);

        s.v[1387] = if (s.v[537] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1387] != 0.0) {
            s.store_offset_ad(172, A::div(A::mul(s.ad_value(537), s.ad_value(80)), s.ad_value(135)), 1.0);
        }

        if (!(s.v[1387] != 0.0)) {
            s.store_div_from_scalar_ad(172, 1.0, A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(537), s.ad_value(80)), s.ad_value(135))));
        }

        s.copy_ad(171, 519);

        s.store_sub(167, 74, 139);

        s.store_add_ad_rhs(174, 80, A::scale(s.ad_value(106), 2.0));

        s.v[1388] = if (s.v[171] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1388] != 0.0) {
            s.store_div_ad_rhs(15, 174, A::add(s.ad_value(140), s.ad_value(174)));
        }

        if (s.v[1388] != 0.0) {
            let assign19470_ad_e27354: A = {
                if (!((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0), A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[520] * s.v[61])) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(520), s.ad_value(61)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(16, &assign19470_ad_e27354);
        }

        if (s.v[1388] != 0.0) {
            s.store_div_from_scalar(17, 1.0, 16);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1388] != 0.0) {
            s.store_mul_ad_lhs(173, A::mul(A::mul(A::div(s.ad_value(174), s.ad_value(171)), s.ad_value(15)), s.ad_value(172)), 17);
        }

        if (s.v[1388] != 0.0) {
            s.store_offset_ad(175, A::div(s.ad_value(167), s.ad_value(173)), 1.0);
        }

        if (!(s.v[1388] != 0.0)) {
            s.store_scalar(175, 1.0);
        }

        s.v[1389] = if (s.v[525] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1389] != 0.0) {
            s.store_scalar(105, 1.0);
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_div_ad_lhs(21, A::scale(s.ad_value(525), ((s.v[30]) as f64).sqrt()), 174);
        }

        if (!(s.v[1389] != 0.0)) {
            s.store_div_from_scalar_ad(105, 1.0, A::offset(s.ad_value(21), 1.0));
        }

        s.store_add(170, 140, 135);

        s.v[1390] = if (s.v[541] > 0.0) { 1.0 } else { 0.0 };

        s.v[1391] = if (p.p350 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1390] != 0.0) && (s.v[1391] != 0.0)) {
            s.store_div_ad_lhs(13, A::div(s.ad_value(541), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(80), p.p350), s.ad_value(135)))), 105);
        }

        if ((s.v[1390] != 0.0) && (!(s.v[1391] != 0.0))) {
            s.store_div_ad_lhs(13, A::mul(s.ad_value(541), A::offset(A::div(A::scale(s.ad_value(80), p.p350), s.ad_value(135)), 1.0)), 105);
        }

        if (s.v[1390] != 0.0) {
            s.store_offset_ad(176, A::mul(s.ad_value(13), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(167), s.ad_value(13)), s.ad_value(170)), 1.0), 1e-38))), 1.0);
        }

        s.v[1392] = if (p.p350 < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1390] != 0.0)) && (s.v[1392] != 0.0)) {
            s.store_div_ad_lhs(13, A::div(s.ad_value(541), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(80), p.p350), s.ad_value(135)))), 105);
        }

        if ((!(s.v[1390] != 0.0)) && (!(s.v[1392] != 0.0))) {
            s.store_div_ad_lhs(13, A::mul(s.ad_value(541), A::offset(A::div(A::scale(s.ad_value(80), p.p350), s.ad_value(135)), 1.0)), 105);
        }

        if (!(s.v[1390] != 0.0)) {
            s.store_offset(176, 13, 1.0);
        }

        s.store_mul(175, 175, 176);

        s.store_limited_exp_ad(13, A::mul(s.ad_value(524), s.ad_value(74)));

        s.v[1393] = if (s.v[523] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1393] != 0.0) {
            s.store_scalar(14, (1.0 + (p.p369 * s.v[30])));
        }

        if (s.v[1393] != 0.0) {
            s.store_div_ad_lhs(168, A::offset(A::mul(s.ad_value(14), s.ad_value(13)), 1.0), 523);
        }

        if (s.v[1393] != 0.0) {
            s.store_mul(168, 168, 105);
        }

        if (!(s.v[1393] != 0.0)) {
            s.store_scalar(168, 5.540622384e34);
        }

        s.store_div(16, 167, 168);

        s.store_offset(12, 16, 1.0);

        s.store_mul(175, 175, 12);

        s.v[1394] = if (s.v[522] > 0.0) { 1.0 } else { 0.0 };

        s.v[1395] = if (s.v[167] > ((s.v[521] * s.v[129]) / 80.0)) { 1.0 } else { 0.0 };

        if ((s.v[1394] != 0.0) && (s.v[1395] != 0.0)) {
            s.store_div_ad_lhs(12, A::mul(s.ad_value(521), s.ad_value(129)), 167);
        }

        if ((s.v[1394] != 0.0) && (s.v[1395] != 0.0)) {
            s.store_div_ad_lhs(169, A::scale(A::limited_exp(s.ad_value(12)), s.v[30]), 522);
        }

        if ((s.v[1394] != 0.0) && (!(s.v[1395] != 0.0))) {
            s.store_div_from_scalar(169, (5.540622384e34 * s.v[30]), 522);
        }

        if (!(s.v[1394] != 0.0)) {
            s.store_scalar(169, 5.540622384e34);
        }

        s.store_offset_ad(177, A::div(s.ad_value(167), s.ad_value(169)), 1.0);

        s.store_mul(175, 175, 177);

        s.store_ad(12, &A::pow(s.ad_value(159), A::div_from_scalar(1.0, s.ad_value(166))));

        s.store_mul(23, 453, 61);

        s.store_sqrt_ad(24, A::offset(A::square(s.ad_value(23)), 0.1));

        s.store_scale_ad(13, A::add(A::sub_from_scalar(1.0, s.ad_value(23)), A::sqrt(A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(23)), A::sub_from_scalar(1.0, s.ad_value(23))), s.ad_value(24)))), 0.5);

        s.store_div_ad(14, A::mul(A::scale(s.ad_value(80), (10.0 * p.p433)), s.ad_value(13)), A::offset(A::mul(s.ad_value(80), s.ad_value(13)), (10.0 * p.p433)));

        s.v[1396] = if (s.v[536] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1396] != 0.0) {
            s.store_mul_ad(138, A::scale(A::div(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), A::scale(s.ad_value(502), s.v[30])), 2.0), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(536), s.ad_value(14)))));
        }

        if (!(s.v[1396] != 0.0)) {
            s.store_mul_ad(138, A::scale(A::div(A::mul(A::div(s.ad_value(499), s.ad_value(12)), s.ad_value(106)), A::scale(s.ad_value(502), s.v[30])), 2.0), A::offset(A::mul(s.ad_value(536), s.ad_value(14)), 1.0));
        }

        s.store_mul_ad(13, A::scale(s.ad_value(138), 2.0), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_sqrt_ad(14, A::offset(A::square(s.ad_value(13)), 1.0));

        s.v[1397] = if (s.v[13] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1397] != 0.0) {
            s.store_scale_ad(162, A::add(s.ad_value(14), A::mul(A::div_from_scalar(1.0, s.ad_value(13)), A::asinh(s.ad_value(13)))), 0.5);
        }

        if (!(s.v[1397] != 0.0)) {
            s.store_scale_ad(162, A::add(s.ad_value(14), A::div_from_scalar(1.0, s.ad_value(14))), 0.5);
        }

        s.copy_ad(163, 162);

        s.v[241] = 0.0;

        s.v[242] = 0.0;

        s.v[1398] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1398] != 0.0) {
            s.store_scalar(244, 0.0);
        }

        if (s.v[1398] != 0.0) {
            s.store_scalar(245, 1.0);
        }

        if (s.v[1398] != 0.0) {
            s.store_mul_ad_rhs(71, 187, A::voltage(ctx, &nodes, Some(8), Some(11)));
        }

        if (s.v[1398] != 0.0) {
            s.store_sub(53, 64, 71);
        }

        if (s.v[1398] != 0.0) {
            s.store_sub(14, 53, 63);
        }

        if (s.v[1398] != 0.0) {
            s.store_sqrt_ad(15, A::offset(A::square(s.ad_value(14)), 0.01));
        }

        if (s.v[1398] != 0.0) {
            s.store_scaled_add(77, 14, 15, 0.5);
        }

        if (s.v[1398] != 0.0) {
            s.store_offset_ad(17, A::mul(s.ad_value(526), s.ad_value(77)), 1.0);
        }

        if (s.v[1398] != 0.0) {
            s.copy_ad(51, 71);
        }

        if (s.v[1398] != 0.0) {
            s.store_add_ad(18, A::div_from_scalar(1.0, s.ad_value(17)), A::mul(s.ad_value(543), s.ad_value(51)));
        }

        if (s.v[1398] != 0.0) {
            s.store_scale_ad(16, A::add(s.ad_value(18), A::sqrt(A::offset(A::square(s.ad_value(18)), 0.01))), 0.5);
        }

        if (s.v[1398] != 0.0) {
            s.store_mul_ad_rhs(241, 408, A::add(s.ad_value(239), A::mul(A::add(s.ad_value(529), A::mul(s.ad_value(531), s.ad_value(16))), s.ad_value(235))));
        }

        if (s.v[1398] != 0.0) {
            s.store_mul_ad_rhs(67, 187, A::voltage(ctx, &nodes, Some(6), Some(11)));
        }

        if (s.v[1398] != 0.0) {
            s.store_sub(55, 64, 67);
        }

        if (s.v[1398] != 0.0) {
            s.store_sub(14, 55, 63);
        }

        if (s.v[1398] != 0.0) {
            s.store_sqrt_ad(15, A::offset(A::square(s.ad_value(14)), 0.01));
        }

        if (s.v[1398] != 0.0) {
            s.store_scaled_add(78, 14, 15, 0.5);
        }

        if (s.v[1398] != 0.0) {
            s.store_offset_ad(17, A::mul(s.ad_value(526), s.ad_value(78)), 1.0);
        }

        if (s.v[1398] != 0.0) {
            s.copy_ad(49, 67);
        }

        if (s.v[1398] != 0.0) {
            s.store_add_ad(18, A::div_from_scalar(1.0, s.ad_value(17)), A::mul(s.ad_value(543), s.ad_value(49)));
        }

        if (s.v[1398] != 0.0) {
            s.store_scale_ad(16, A::add(s.ad_value(18), A::sqrt(A::offset(A::square(s.ad_value(18)), 0.01))), 0.5);
        }

        if (s.v[1398] != 0.0) {
            s.store_mul_ad_rhs(242, 408, A::add(s.ad_value(240), A::mul(A::add(s.ad_value(528), A::mul(s.ad_value(530), s.ad_value(16))), s.ad_value(235))));
        }

        if (!(s.v[1398] != 0.0)) {
            s.store_offset_ad(12, A::mul(s.ad_value(526), s.ad_value(80)), 1.0);
        }

        if (!(s.v[1398] != 0.0)) {
            s.store_mul_ad_rhs(13, 543, A::sub(s.ad_value(111), s.ad_value(128)));
        }

        if (!(s.v[1398] != 0.0)) {
            s.store_add_ad_lhs(14, A::div_from_scalar(1.0, s.ad_value(12)), 13);
        }

        if (!(s.v[1398] != 0.0)) {
            s.store_scale_ad(15, A::add(s.ad_value(14), A::sqrt(A::offset(A::square(s.ad_value(14)), 0.01))), 0.5);
        }

        if (!(s.v[1398] != 0.0)) {
            s.store_scale_ad(244, A::mul(A::mul(s.ad_value(408), A::add(s.ad_value(533), A::mul(s.ad_value(532), s.ad_value(15)))), s.ad_value(235)), p.p2);
        }

        if (!(s.v[1398] != 0.0)) {
            s.copy_ad(242, 240);
        }

        if (!(s.v[1398] != 0.0)) {
            s.copy_ad(241, 239);
        }

        if (!(s.v[1398] != 0.0)) {
            s.store_offset_ad(245, A::mul(A::mul(A::scale(A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), (s.v[46] * (s.v[29] * 1.0 / (s.v[30])))), s.ad_value(80)), s.ad_value(244)), 1.0);
        }

        s.v[1399] = if (p.p42 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1398] != 0.0)) && (s.v[1399] != 0.0)) {
            s.store_mul_ad_rhs(244, 408, A::add(A::add(s.ad_value(239), A::scale(A::mul(A::add(s.ad_value(533), A::mul(s.ad_value(532), s.ad_value(15))), s.ad_value(235)), p.p2)), s.ad_value(240)));
        }

        if ((!(s.v[1398] != 0.0)) && (s.v[1399] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        if ((!(s.v[1398] != 0.0)) && (s.v[1399] != 0.0)) {
            s.store_scalar(241, 0.0);
        }

        if ((!(s.v[1398] != 0.0)) && (s.v[1399] != 0.0)) {
            s.store_offset_ad(245, A::mul(A::mul(A::scale(A::div(s.ad_value(499), A::mul(s.ad_value(162), s.ad_value(159))), (s.v[46] * (s.v[29] * 1.0 / (s.v[30])))), s.ad_value(80)), s.ad_value(244)), 1.0);
        }

        s.store_add_ad_rhs(12, 150, A::div(s.ad_value(153), A::add(s.ad_value(80), A::mul(A::scale(s.ad_value(104), 2.0), s.ad_value(393)))));

        s.store_sub(216, 200, 144);

        s.store_mul_ad_lhs(13, A::mul(s.ad_value(12), s.ad_value(216)), 216);

        s.store_offset(14, 13, ((1.0) + ((-0.001))));

        s.store_offset_ad(15, A::scale(A::add(s.ad_value(14), A::sqrt(A::offset(A::square(s.ad_value(14)), 0.004))), 0.5), (-1.0));

        s.store_scale_ad(154, A::offset(A::sqrt(A::offset(s.ad_value(15), 1.0)), 1.0), 0.5);

        s.store_offset_ad(154, A::scale(A::sub(A::offset(s.ad_value(154), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(154), (-1.0)), A::offset(s.ad_value(154), (-1.0))), ((0.25 * 0.01) * 0.01)))), 0.5), (0.25 * 0.01));

        s.store_add(12, 200, 144);

        s.store_sub(13, 200, 144);

        s.store_div_ad_rhs(14, 13, A::add(s.ad_value(12), s.ad_value(610)));

        s.store_mul_ad_lhs(15, A::mul(s.ad_value(609), s.ad_value(14)), 14);

        s.store_offset(611, 15, 1.0);

        s.store_div_ad_rhs(21, 633, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(636), A::mul(A::mul(s.ad_value(639), s.ad_value(13)), s.ad_value(13)))), s.ad_value(12)), A::mul(A::scale(s.ad_value(104), 2.0), s.ad_value(393))));

        s.store_limited_exp_ad(628, A::neg(s.ad_value(21)));

        s.store_mul_ad_lhs(160, A::mul(s.ad_value(159), s.ad_value(162)), 245);

        s.store_div(157, 499, 160);

        let assign20520_ad_e28230: A = A::div(A::mul(A::mul(A::mul(A::mul(A::scale(A::scale(A::mul(A::scale(s.ad_value(90), (2.0 * p.p2)), s.ad_value(157)), (s.v[29] * 1.0 / (s.v[30]))), s.v[46]), s.ad_value(106)), s.ad_value(106)), A::mul(A::sub(s.ad_value(200), s.ad_value(144)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)))), s.ad_value(175)), s.ad_value(154));
        s.store_mul_ad_lhs(188, A::mul(assign20520_ad_e28230, s.ad_value(611)), 628);

        s.store_scale(188, 188, p.p36);

        s.v[1400] = if ((p.p42 == 1.0) && (p.p1094 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1400] != 0.0) {
            s.store_mul_ad_rhs(753, 108, A::ln(A::div(A::scale(s.ad_value(481), p.p1117), A::powf(s.ad_value(28), 2.0))));
        }

        s.v[1401] = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1400] != 0.0) && (s.v[1401] != 0.0)) {
            s.store_mul_ad_rhs(753, 108, A::sqrt(A::offset(A::square(s.ad_value(753)), 1e-6)));
        }

        if (s.v[1400] != 0.0) {
            s.store_sub_from_scalar_ad(16, 1.0, A::scale(s.ad_value(50), p.p1113));
        }

        s.v[1402] = if ((0.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if ((s.v[1400] != 0.0) && (s.v[1402] != 0.0)) {
            s.store_div_from_scalar_ad(16, ((-0.001) * 0.001), A::scale(s.ad_value(16), 16.0));
        }

        if ((s.v[1400] != 0.0) && (!(s.v[1402] != 0.0))) {
            s.store_scale_ad(16, A::add(s.ad_value(16), A::sqrt(A::offset(A::mul(s.ad_value(16), s.ad_value(16)), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (s.v[1400] != 0.0) {
            s.store_offset(13, 200, (-p.p1102));
        }

        s.v[1403] = if ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1400] != 0.0) && (s.v[1403] != 0.0)) {
            s.store_div_from_scalar_ad(13, ((-2.0) * 2.0), A::scale(s.ad_value(13), 16.0));
        }

        if ((s.v[1400] != 0.0) && (!(s.v[1403] != 0.0))) {
            s.store_scale_ad(13, A::add(A::offset(s.ad_value(13), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-0.1)), A::offset(s.ad_value(13), (-0.1))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1400] != 0.0) {
            s.store_div_ad(14, A::scale(s.ad_value(13), (10.0 * p.p1103)), A::offset(s.ad_value(13), (10.0 * p.p1103)));
        }

        if (s.v[1400] != 0.0) {
            s.store_mul_ad_rhs(754, 763, A::offset(A::scale(s.ad_value(14), p.p1101), 1.0));
        }

        if (s.v[1400] != 0.0) {
            s.store_scale(23, 754, ((p.p2 * s.v[29]) * 1.60219e-19));
        }

        s.v[1404] = if (p.p1110 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_ad(757, &A::abs(A::voltage(ctx, &nodes, Some(6), Some(5))));
        }

        s.v[1405] = if (p.p1127 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_scalar(21, 1.0);
        }

        s.v[1406] = if ((0.0 == 0.0) && ((s.v[757] - p.p1126) < ((-2500.0) * 0.5))) { 1.0 } else { 0.0 };

        if ((((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1405] != 0.0))) && (s.v[1406] != 0.0)) {
            s.store_div_from_scalar_ad(22, ((-0.5) * 0.5), A::scale(A::offset(s.ad_value(757), (-p.p1126)), 16.0));
        }

        if ((((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1405] != 0.0))) && (!(s.v[1406] != 0.0))) {
            s.store_scale_ad(22, A::add(A::offset(s.ad_value(757), (-p.p1126)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(757), (-p.p1126)), A::offset(s.ad_value(757), (-p.p1126))), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1405] != 0.0))) {
            s.store_offset_scaled(21, 22, p.p1127, 1.0);
        }

        s.v[1408] = if ((p.p1098 != 0.0) && (p.p514 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (s.v[1408] != 0.0)) {
            s.store_sqrt_ad(760, A::offset(A::square(A::voltage(ctx, &nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p514) as f64).ln())) / p.p515))));
        }

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (s.v[1408] != 0.0)) {
            s.store_mul_ad(750, A::mul(A::scale(s.ad_value(23), p.p1099), s.ad_value(21)), A::offset(A::scale(A::powf(s.ad_value(760), p.p515), p.p514), 1.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1408] != 0.0))) {
            s.store_mul_ad_lhs(750, A::scale(s.ad_value(23), p.p1099), 21);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_offset_ad(14, A::div(s.ad_value(50), s.ad_value(753)), 1.0);
        }

        s.v[1409] = if ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (s.v[1409] != 0.0)) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::scale(s.ad_value(14), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1409] != 0.0))) {
            s.store_scale_ad(14, A::add(s.ad_value(14), A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_sub_ad(18, A::sub_from_scalar(1.0, A::scale(A::offset(A::sqrt(s.ad_value(14)), (-1.0)), p.p1124)), A::scale(s.ad_value(50), p.p1125));
        }

        s.v[1410] = if ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_div_from_scalar_ad(18, ((-0.05) * 0.05), A::scale(s.ad_value(18), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1410] != 0.0))) {
            s.store_scale_ad(18, A::add(s.ad_value(18), A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_mul(750, 18, 750);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_mul_ad_lhs(19, A::mul(A::scale(s.ad_value(762), p.p1110), s.ad_value(235)), 16);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_mul(755, 750, 19);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_div_ad(752, A::powf(s.ad_value(757), (4.0 - p.p1107)), A::add(A::powf(s.ad_value(757), (4.0 - p.p1107)), A::scale(A::powf(s.ad_value(755), (4.0 - p.p1107)), p.p1122)));
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_powf(17, 752, (1.0 / p.p1107));
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_div_ad_lhs(20, A::mul(s.ad_value(17), s.ad_value(757)), 755);
        }

        s.v[1411] = if ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (s.v[1411] != 0.0)) {
            s.store_div_from_scalar_ad(20, ((-0.001) * 0.001), A::scale(s.ad_value(20), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) && (!(s.v[1411] != 0.0))) {
            s.store_scale_ad(20, A::add(s.ad_value(20), A::sqrt(A::offset(A::mul(s.ad_value(20), s.ad_value(20)), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1404] != 0.0)) {
            s.store_mul_ad_rhs(759, 19, A::powf(A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107)));
        }

        s.v[1412] = if (p.p1112 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_ad(758, &A::abs(A::voltage(ctx, &nodes, Some(7), Some(8))));
        }

        s.v[1414] = if ((p.p1098 != 0.0) && (p.p516 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (s.v[1414] != 0.0)) {
            s.store_sqrt_ad(760, A::offset(A::square(A::voltage(ctx, &nodes, Some(11), Some(3))), ((10.0) as f64).powf(((2.0 * ((-3.0) - ((p.p516) as f64).ln())) / p.p517))));
        }

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (s.v[1414] != 0.0)) {
            s.store_mul_ad(751, A::scale(s.ad_value(23), p.p1109), A::offset(A::scale(A::powf(s.ad_value(760), p.p517), p.p516), 1.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (!(s.v[1414] != 0.0))) {
            s.store_scale(751, 23, p.p1109);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_offset_ad(14, A::div(s.ad_value(50), s.ad_value(753)), 1.0);
        }

        s.v[1415] = if ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (s.v[1415] != 0.0)) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::scale(s.ad_value(14), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (!(s.v[1415] != 0.0))) {
            s.store_scale_ad(14, A::add(s.ad_value(14), A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_sub_ad(18, A::sub_from_scalar(1.0, A::scale(A::offset(A::sqrt(s.ad_value(14)), (-1.0)), p.p1124)), A::scale(s.ad_value(50), p.p1125));
        }

        s.v[1416] = if ((0.0 == 0.0) && (s.v[18] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (s.v[1416] != 0.0)) {
            s.store_div_from_scalar_ad(18, ((-0.05) * 0.05), A::scale(s.ad_value(18), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (!(s.v[1416] != 0.0))) {
            s.store_scale_ad(18, A::add(s.ad_value(18), A::sqrt(A::offset(A::mul(s.ad_value(18), s.ad_value(18)), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_mul(751, 18, 751);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_mul_ad_lhs(19, A::mul(A::scale(s.ad_value(762), p.p1112), s.ad_value(235)), 16);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_mul(756, 751, 19);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_div_ad(752, A::powf(s.ad_value(758), (4.0 - p.p1107)), A::add(A::powf(s.ad_value(758), (4.0 - p.p1107)), A::scale(A::powf(s.ad_value(756), (4.0 - p.p1107)), p.p1122)));
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_powf(17, 752, (1.0 / p.p1107));
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_div_ad_lhs(20, A::mul(s.ad_value(17), s.ad_value(758)), 756);
        }

        s.v[1417] = if ((0.0 == 0.0) && (s.v[20] < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (s.v[1417] != 0.0)) {
            s.store_div_from_scalar_ad(20, ((-0.001) * 0.001), A::scale(s.ad_value(20), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) && (!(s.v[1417] != 0.0))) {
            s.store_scale_ad(20, A::add(s.ad_value(20), A::sqrt(A::offset(A::mul(s.ad_value(20), s.ad_value(20)), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1412] != 0.0)) {
            s.store_mul_ad_rhs(761, 19, A::powf(A::offset(A::powf(s.ad_value(20), p.p1107), 1.0), (1.0 / p.p1107)));
        }

        s.v[1418] = if ((p.p1110 != 0.0) && (p.p1112 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) {
            s.store_div_ad(17, A::mul(s.ad_value(57), s.ad_value(188)), A::min(s.ad_value(750), s.ad_value(751)));
        }

        if ((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) {
            s.store_offset_ad(17, A::scale(A::sub(A::offset(s.ad_value(17), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-1.0)), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108)))), 0.5), (0.25 * p.p1108));
        }

        if ((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) {
            s.store_offset_ad(17, A::offset(A::offset(s.ad_value(17), (0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())), (-0.5)), (-(0.25 * p.p1108)));
        }

        s.v[1419] = if (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108))) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) && (s.v[1419] != 0.0)) {
            s.store_div_from_scalar_ad(17, ((-p.p1108) * p.p1108), A::scale(s.ad_value(17), 16.0));
        }

        if (((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) && (!(s.v[1419] != 0.0))) {
            s.store_scale_ad(17, A::add(A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-(-1.0))), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108)))), 0.5);
        }

        if ((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
        }

        if ((s.v[1400] != 0.0) && (s.v[1418] != 0.0)) {
            s.store_mul_ad_lhs(188, A::mul(s.ad_value(57), A::min(s.ad_value(750), s.ad_value(751))), 17);
        }

        s.v[1420] = if (p.p1110 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_div_ad_lhs(17, A::mul(s.ad_value(57), s.ad_value(188)), 750);
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_offset_ad(17, A::scale(A::sub(A::offset(s.ad_value(17), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-1.0)), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108)))), 0.5), (0.25 * p.p1108));
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_offset_ad(17, A::offset(A::offset(s.ad_value(17), (0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())), (-0.5)), (-(0.25 * p.p1108)));
        }

        s.v[1421] = if (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108))) { 1.0 } else { 0.0 };

        if ((((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) && (s.v[1421] != 0.0)) {
            s.store_div_from_scalar_ad(17, ((-p.p1108) * p.p1108), A::scale(s.ad_value(17), 16.0));
        }

        if ((((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) && (!(s.v[1421] != 0.0))) {
            s.store_scale_ad(17, A::add(A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-(-1.0))), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108)))), 0.5);
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_mul_ad_lhs(188, A::mul(s.ad_value(57), s.ad_value(750)), 17);
        }

        s.v[1422] = if (p.p1112 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_div_ad_lhs(17, A::mul(s.ad_value(57), s.ad_value(188)), 751);
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_offset_ad(17, A::scale(A::sub(A::offset(s.ad_value(17), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-1.0)), A::offset(s.ad_value(17), (-1.0))), ((0.25 * p.p1108) * p.p1108)))), 0.5), (0.25 * p.p1108));
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_offset_ad(17, A::offset(A::offset(s.ad_value(17), (0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt())), (-0.5)), (-(0.25 * p.p1108)));
        }

        s.v[1423] = if (((-1.0) == 0.0) && (s.v[17] < ((-2500.0) * p.p1108))) { 1.0 } else { 0.0 };

        if ((((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) && (s.v[1423] != 0.0)) {
            s.store_div_from_scalar_ad(17, ((-p.p1108) * p.p1108), A::scale(s.ad_value(17), 16.0));
        }

        if ((((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) && (!(s.v[1423] != 0.0))) {
            s.store_scale_ad(17, A::add(A::offset(s.ad_value(17), (-1.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(17), (-(-1.0))), A::offset(s.ad_value(17), (-(-1.0)))), ((0.25 * p.p1108) * p.p1108)))), 0.5);
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_offset(17, 17, (((-(0.5 * (((1.0 + ((0.25 * p.p1108) * p.p1108))) as f64).sqrt()))) + (0.5)));
        }

        if (((s.v[1400] != 0.0) && (!(s.v[1418] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_mul_ad_lhs(188, A::mul(s.ad_value(57), s.ad_value(751)), 17);
        }

        s.v[774] = 0.0;

        s.v[775] = 0.0;

        s.v[776] = 0.0;

        s.v[777] = 0.0;

        s.v[1424] = if (((p.p42 == 1.0) && (p.p1095 == 1.0)) && (p.p1094 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1424] != 0.0) {
            s.store_offset_ad(764, A::neg(s.ad_value(232)), (-p.p1114));
        }

        if (s.v[1424] != 0.0) {
            s.store_div(764, 764, 108);
        }

        if (s.v[1424] != 0.0) {
            s.store_scale_ad(765, A::sqrt(A::scale(s.ad_value(109), (((2.0 * 1.60219e-19) * s.v[26]) * p.p1117))), 1.0 / (s.v[46]));
        }

        if (s.v[1424] != 0.0) {
            s.store_ln_ad(766, A::max_with_scalar(A::div_from_scalar(p.p1117, s.ad_value(28)), 1e-38));
        }

        if (s.v[1424] != 0.0) {
            s.store_scalar(13, 1.0);
        }

        if (s.v[1424] != 0.0) {
            s.store_div(204, 764, 13);
        }

        if (s.v[1424] != 0.0) {
            s.store_div(205, 765, 13);
        }

        if (s.v[1424] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1424] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1425] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1424] != 0.0) && (s.v[1425] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1424] != 0.0) && (s.v[1425] != 0.0)) {
            s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1425] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1425] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1425] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1425] != 0.0))) {
            s.store_sub_ad_lhs(767, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1424] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1424] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1424] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(765), A::scale(s.ad_value(96), 2.0)), 1.0), 765);
        }

        if (s.v[1424] != 0.0) {
            s.store_sub_ad(13, A::sub(s.ad_value(767), A::scale(s.ad_value(766), 2.0)), A::div(s.ad_value(69), s.ad_value(108)));
        }

        if (s.v[1424] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1424] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1424] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1426] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1427] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) && (s.v[1427] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1428] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) && (!(s.v[1427] != 0.0))) && (s.v[1428] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1424] != 0.0) && (s.v[1426] != 0.0)) {
            s.store_mul_ad_rhs(768, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1426] != 0.0))) {
            s.store_sub_ad_rhs(768, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1429] = if ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1424] != 0.0) && (s.v[1429] != 0.0)) {
            s.store_div_from_scalar_ad(769, ((-2.0) * 2.0), A::scale(s.ad_value(767), 16.0));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1429] != 0.0))) {
            s.store_scale_ad(769, A::add(A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1424] != 0.0) {
            s.store_sqrt(770, 769);
        }

        if (s.v[1424] != 0.0) {
            s.store_sub_ad_rhs(771, 767, A::scale(s.ad_value(768), 2.0));
        }

        s.v[1430] = if ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1424] != 0.0) && (s.v[1430] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(771), 16.0));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1430] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(771), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(771), (-1.0)), A::offset(s.ad_value(771), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1424] != 0.0) {
            s.store_offset_ad(772, A::div(s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12)))), 1.0);
        }

        if (s.v[1424] != 0.0) {
            s.store_sub_ad_rhs(773, 767, A::scale(s.ad_value(768), 2.0));
        }

        if (s.v[1424] != 0.0) {
            s.store_mul_ad(775, A::mul(A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), s.ad_value(108)), A::sub(A::sub(s.ad_value(764), s.ad_value(773)), A::mul(A::scale(s.ad_value(772), 2.0), s.ad_value(768))));
        }

        s.v[1431] = if (p.p1118 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1424] != 0.0) && (s.v[1431] != 0.0)) {
            s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);
        }

        if ((s.v[1424] != 0.0) && (s.v[1431] != 0.0)) {
            s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);
        }

        if ((s.v[1424] != 0.0) && (s.v[1431] != 0.0)) {
            s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add(A::scale(s.ad_value(229), (3.9 * 1.0 / (p.p111))), A::scale(s.ad_value(14), 1.0 / (s.v[47]))));
        }

        if ((s.v[1424] != 0.0) && (!(s.v[1431] != 0.0))) {
            s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);
        }

        if (s.v[1424] != 0.0) {
            s.store_mul_ad_lhs(774, A::mul(A::mul(A::scale(s.ad_value(772), (((p.p2 * s.v[33]) * p.p1116) * 2.0)), s.ad_value(108)), s.ad_value(12)), 768);
        }

        s.v[1432] = if (p.p1096 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_offset_ad(764, A::mul(A::neg(s.ad_value(187)), A::voltage(ctx, &nodes, Some(10), Some(7))), (-p.p1114));
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_div(764, 764, 108);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_scalar(13, 1.0);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_div(204, 764, 13);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_div(205, 765, 13);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1433] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1433] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1433] != 0.0)) {
            s.store_neg_ad(767, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1433] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1433] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1433] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1433] != 0.0))) {
            s.store_sub_ad_lhs(767, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sqrt(96, 20);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(765), A::scale(s.ad_value(96), 2.0)), 1.0), 765);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sub_ad(13, A::sub(s.ad_value(767), A::scale(s.ad_value(766), 2.0)), A::div(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(7), Some(11))), s.ad_value(108)));
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.copy_ad(94, 96);
        }

        s.v[1434] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1435] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) && (s.v[1435] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1436] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) && (!(s.v[1435] != 0.0))) && (s.v[1436] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if (((((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) && (!(s.v[1435] != 0.0))) && (!(s.v[1436] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if (((((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) && (!(s.v[1435] != 0.0))) && (!(s.v[1436] != 0.0))) {
            s.store_square(18, 14);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        if (((((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) && (!(s.v[1435] != 0.0))) && (!(s.v[1436] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1434] != 0.0)) {
            s.store_mul_ad_rhs(768, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1434] != 0.0))) {
            s.store_sub_ad_rhs(768, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1437] = if ((1.0 == 0.0) && (s.v[767] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1437] != 0.0)) {
            s.store_div_from_scalar_ad(769, ((-2.0) * 2.0), A::scale(s.ad_value(767), 16.0));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1437] != 0.0))) {
            s.store_scale_ad(769, A::add(A::offset(s.ad_value(767), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(767), (-1.0)), A::offset(s.ad_value(767), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sqrt(770, 769);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sub_ad_rhs(771, 767, A::scale(s.ad_value(768), 2.0));
        }

        s.v[1438] = if ((1.0 == 0.0) && (s.v[771] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1438] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(771), 16.0));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1438] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(771), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(771), (-1.0)), A::offset(s.ad_value(771), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_offset_ad(772, A::div(s.ad_value(765), A::add(s.ad_value(770), A::sqrt(s.ad_value(12)))), 1.0);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_sub_ad_rhs(773, 767, A::scale(s.ad_value(768), 2.0));
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_mul_ad(777, A::mul(A::div_from_scalar(((((p.p2 * s.v[33]) * p.p1115) * 8.85418e-12) * p.p111), s.ad_value(229)), s.ad_value(108)), A::sub(A::sub(s.ad_value(764), s.ad_value(773)), A::mul(A::scale(s.ad_value(772), 2.0), s.ad_value(768))));
        }

        s.v[1439] = if (p.p1118 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1439] != 0.0)) {
            s.store_offset_scaled(13, 768, 1.0 / (p.p1119), 1.0);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1439] != 0.0)) {
            s.store_div_from_scalar(14, (p.p1118 * 1.9e-9), 13);
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (s.v[1439] != 0.0)) {
            s.store_div_from_scalar_ad(12, (3.9 * 8.85418e-12), A::add(A::scale(s.ad_value(229), (3.9 * 1.0 / (p.p111))), A::scale(s.ad_value(14), 1.0 / (s.v[47]))));
        }

        if (((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) && (!(s.v[1439] != 0.0))) {
            s.store_div_from_scalar(12, (8.85418e-12 * p.p111), 229);
        }

        if ((s.v[1424] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_mul_ad_lhs(776, A::mul(A::mul(A::scale(s.ad_value(772), (((p.p2 * s.v[33]) * p.p1116) * 2.0)), s.ad_value(108)), s.ad_value(12)), 768);
        }

        s.v[254] = 0.0;

        s.v[1440] = if (p.p7 > 1.0) { 1.0 } else { 0.0 };

        if (s.v[1440] != 0.0) {
            s.store_mul_ad_lhs(255, A::scale(A::scale(s.ad_value(157), (s.v[29] * 1.0 / (s.v[30]))), s.v[46]), 80);
        }

        if (s.v[1440] != 0.0) {
            s.store_scale(21, 108, p.p755);
        }

        if (s.v[1440] != 0.0) {
            s.store_scale_ad(12, A::scale(A::mul(s.ad_value(21), s.ad_value(157)), (s.v[29] * 1.0 / (s.v[30]))), s.v[46]);
        }

        if (s.v[1440] != 0.0) {
            s.store_scaled_add(254, 12, 255, (p.p754 * p.p2));
        }

        s.v[1441] = if (p.p7 == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1440] != 0.0) && (s.v[1441] != 0.0)) {
            s.store_div_from_scalar(253, 1.0, 252);
        }

        s.v[1442] = if (s.v[253] < p.p1093) { 1.0 } else { 0.0 };

        if (((s.v[1440] != 0.0) && (s.v[1441] != 0.0)) && (s.v[1442] != 0.0)) {
            s.store_scalar(253, p.p1093);
        }

        if (((s.v[1440] != 0.0) && (s.v[1441] != 0.0)) && (s.v[1442] != 0.0)) {
            s.store_div_from_scalar(252, 1.0, 253);
        }

        if ((s.v[1440] != 0.0) && (s.v[1441] != 0.0)) {
            s.store_add(23, 252, 254);
        }

        if ((s.v[1440] != 0.0) && (s.v[1441] != 0.0)) {
            s.store_div_ad_lhs(254, A::mul(s.ad_value(252), s.ad_value(254)), 23);
        }

        s.v[1443] = if (p.p1094 == 0.0) { 1.0 } else { 0.0 };

        s.v[1444] = if ((s.v[553] <= 0.0) || (s.v[558] <= 0.0)) { 1.0 } else { 0.0 };

        s.v[1445] = if (s.v[167] > (s.v[558] / 80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1443] != 0.0) && (!(s.v[1444] != 0.0))) && (s.v[1445] != 0.0)) {
            s.store_div_ad_lhs(13, A::neg(s.ad_value(558)), 167);
        }

        s.v[1446] = if (p.p1094 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_mul_ad_lhs(184, A::offset(A::mul(s.ad_value(555), s.ad_value(74)), 1.0), 140);
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(184)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_mul(183, 74, 20);
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_sub(185, 74, 183);
        }

        s.v[1447] = if ((0.0 == 0.0) && (s.v[185] < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if (((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) && (s.v[1447] != 0.0)) {
            s.store_div_from_scalar_ad(185, ((-0.001) * 0.001), A::scale(s.ad_value(185), 16.0));
        }

        if (((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) && (!(s.v[1447] != 0.0))) {
            s.store_scale_ad(185, A::add(s.ad_value(185), A::sqrt(A::offset(A::mul(s.ad_value(185), s.ad_value(185)), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_mul_ad(181, A::scale(s.ad_value(558), 0.5), A::offset(A::powf(s.ad_value(183), s.v[556]), 1.0));
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_offset_ad(13, A::scale(A::limited_exp(A::scale(s.ad_value(76), p.p492)), p.p493), 1.0);
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_div(182, 553, 13);
        }

        if ((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_mul_ad_rhs(14, 182, A::add(A::offset(A::scale(s.ad_value(61), p.p505), 1.0), A::mul(A::scale(s.ad_value(61), p.p506), s.ad_value(61))));
        }

        s.v[1448] = if ((0.0 == 0.0) && (s.v[14] < ((-2500.0) * 1e-12))) { 1.0 } else { 0.0 };

        if (((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) && (s.v[1448] != 0.0)) {
            s.store_div_from_scalar_ad(182, ((-1e-12) * 1e-12), A::scale(s.ad_value(14), 16.0));
        }

        if (((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) && (!(s.v[1448] != 0.0))) {
            s.store_scale_ad(182, A::add(s.ad_value(14), A::sqrt(A::offset(A::mul(s.ad_value(14), s.ad_value(14)), ((0.25 * 1e-12) * 1e-12)))), 0.5);
        }

        s.v[1449] = if ((s.v[553] <= 0.0) || (s.v[558] <= 0.0)) { 1.0 } else { 0.0 };

        s.v[1450] = if (s.v[185] > (s.v[181] / 80.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1443] != 0.0)) && (s.v[1446] != 0.0)) && (!(s.v[1449] != 0.0))) && (s.v[1450] != 0.0)) {
            s.store_div_ad(13, A::neg(s.ad_value(181)), A::powf(s.ad_value(185), p.p524));
        }

        s.v[1451] = if ((p.p1094 == 1.0) && (p.p1098 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1451] != 0.0) {
            s.store_offset(13, 200, (-p.p1105));
        }

        s.v[1452] = if ((0.1 == 0.0) && (s.v[13] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1451] != 0.0) && (s.v[1452] != 0.0)) {
            s.store_div_from_scalar_ad(13, ((-2.0) * 2.0), A::scale(s.ad_value(13), 16.0));
        }

        if ((s.v[1451] != 0.0) && (!(s.v[1452] != 0.0))) {
            s.store_scale_ad(13, A::add(A::offset(s.ad_value(13), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-0.1)), A::offset(s.ad_value(13), (-0.1))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1451] != 0.0) {
            s.store_div_ad(14, A::scale(s.ad_value(13), (10.0 * p.p1106)), A::offset(s.ad_value(13), (10.0 * p.p1106)));
        }

        if (s.v[1451] != 0.0) {
            s.store_mul_ad_rhs(754, 763, A::offset(A::scale(s.ad_value(14), p.p1104), 1.0));
        }

        if (s.v[1451] != 0.0) {
            s.store_div_ad(778, A::scale(s.ad_value(188), p.p502), A::scale(s.ad_value(754), ((p.p2 * s.v[29]) * 1.60219e-19)));
        }

        if (s.v[1451] != 0.0) {
            s.store_offset_scaled(779, 778, 1.0 / (p.p1099), (-1.0));
        }

        s.v[1453] = if ((0.0 == 0.0) && (s.v[779] < ((-2500.0) * p.p504))) { 1.0 } else { 0.0 };

        if ((s.v[1451] != 0.0) && (s.v[1453] != 0.0)) {
            s.store_div_from_scalar_ad(779, ((-p.p504) * p.p504), A::scale(s.ad_value(779), 16.0));
        }

        if ((s.v[1451] != 0.0) && (!(s.v[1453] != 0.0))) {
            s.store_scale_ad(779, A::add(s.ad_value(779), A::sqrt(A::offset(A::mul(s.ad_value(779), s.ad_value(779)), ((0.25 * p.p504) * p.p504)))), 0.5);
        }

        if (s.v[1451] != 0.0) {
            s.store_scale(779, 779, p.p1099);
        }

        s.v[1454] = if (p.p514 > 0.0) { 1.0 } else { 0.0 };

        s.v[1455] = if ((0.0 == 0.0) && (((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) - (p.p514 * ((s.v[760]) as f64).powf(p.p513))) < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if (((s.v[1451] != 0.0) && (s.v[1454] != 0.0)) && (s.v[1455] != 0.0)) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::scale(A::sub(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), A::scale(A::powf(s.ad_value(760), p.p513), p.p514)), 16.0));
        }

        if (((s.v[1451] != 0.0) && (s.v[1454] != 0.0)) && (!(s.v[1455] != 0.0))) {
            let assign23370_ad_e32293: A = A::mul(A::sub(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), A::scale(A::powf(s.ad_value(760), p.p513), p.p514)), A::sub(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), A::scale(A::powf(s.ad_value(760), p.p513), p.p514)));
            s.store_scale_ad(14, A::add(A::sub(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), A::scale(A::powf(s.ad_value(760), p.p513), p.p514)), A::sqrt(A::offset(assign23370_ad_e32293, ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        s.v[1456] = if ((0.0 == 0.0) && ((((s.v[187] * (nv0 - nv2)) - (p.p512 * s.v[183])) - p.p503) < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if (((s.v[1451] != 0.0) && (!(s.v[1454] != 0.0))) && (s.v[1456] != 0.0)) {
            s.store_div_from_scalar_ad(14, ((-0.05) * 0.05), A::scale(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), 16.0));
        }

        if (((s.v[1451] != 0.0) && (!(s.v[1454] != 0.0))) && (!(s.v[1456] != 0.0))) {
            let assign23400_ad_e32396: A = A::add(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), A::sqrt(A::offset(A::mul(A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503)), A::offset(A::sub(A::mul(s.ad_value(187), A::voltage(ctx, &nodes, Some(0), Some(2))), A::scale(s.ad_value(183), p.p512)), (-p.p503))), ((0.25 * 0.05) * 0.05))));
            s.store_scale_ad(14, assign23400_ad_e32396, 0.5);
        }

        if (s.v[1451] != 0.0) {
            s.store_scale(15, 779, ((2.0 * 1.60219e-19) / (p.p110 * 8.85418e-12)));
        }

        if (s.v[1451] != 0.0) {
            s.store_powf_ad(15, A::mul(s.ad_value(15), s.ad_value(14)), 0.5);
        }

        if (s.v[1451] != 0.0) {
            s.store_add_ad(16, A::scale(s.ad_value(61), p.p507), A::mul(A::scale(s.ad_value(61), p.p508), s.ad_value(61)));
        }

        if (s.v[1451] != 0.0) {
            s.store_add_ad(17, A::scale(s.ad_value(14), p.p509), A::scale(A::powf(s.ad_value(14), p.p511), p.p510));
        }

        if (s.v[1451] != 0.0) {
            s.store_scale_ad(18, A::add(A::offset(s.ad_value(16), 1.0), s.ad_value(17)), p.p500);
        }

        s.v[1458] = if (s.v[15] > (p.p501 / 80.0)) { 1.0 } else { 0.0 };

        if ((s.v[1451] != 0.0) && (s.v[1458] != 0.0)) {
            s.store_div_from_scalar(13, (-p.p501), 15);
        }

        s.v[1459] = if ((p.p46 != 0.0) || (p.p47 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1459] != 0.0) {
            s.store_mul_ad_rhs(277, 106, A::add(A::add(A::sub(s.ad_value(59), s.ad_value(91)), s.ad_value(200)), s.ad_value(144)));
        }

        if (s.v[1459] != 0.0) {
            s.store_sqrt_ad(13, A::offset(A::square(s.ad_value(277)), 0.0001));
        }

        if (s.v[1459] != 0.0) {
            s.store_scaled_sub(279, 13, 277, 0.5);
        }

        if (s.v[1459] != 0.0) {
            s.store_scaled_add(278, 277, 13, 0.5);
        }

        s.v[1460] = if (p.p47 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_div_ad_lhs(13, A::div(s.ad_value(277), s.ad_value(589)), 108);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_sub_ad_rhs(14, 586, A::mul(s.ad_value(587), s.ad_value(279)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_offset_ad(15, A::mul(s.ad_value(588), s.ad_value(279)), 1.0);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_mul_ad_lhs(16, A::scale(s.ad_value(14), ((-745669000000.0) * p.p77)), 15);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_ad(17, &A::limited_exp(s.ad_value(16)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_scalar(18, 4.97232e-7);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_div_ad_lhs(13, A::div(A::sub(s.ad_value(277), s.ad_value(584)), s.ad_value(585)), 108);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_sub_ad_rhs(14, 581, A::mul(s.ad_value(582), s.ad_value(278)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_offset_ad(15, A::mul(s.ad_value(583), s.ad_value(278)), 1.0);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_mul_ad_lhs(16, A::scale(s.ad_value(14), ((-982222000000.0) * p.p77)), 15);
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_ad(17, &A::limited_exp(s.ad_value(16)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1460] != 0.0)) {
            s.store_scalar(18, 3.75956e-7);
        }

        s.v[1461] = if (p.p46 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_sub_ad_rhs(13, 590, A::mul(s.ad_value(591), s.ad_value(278)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(592), s.ad_value(278)), 1.0);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_mul_ad_lhs(15, A::scale(s.ad_value(13), s.v[295]), 14);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_mul_ad(16, A::mul(A::mul(s.ad_value(90), s.ad_value(106)), A::add(s.ad_value(200), s.ad_value(144))), A::limited_exp(s.ad_value(15)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(280, A::sqrt(A::offset(A::square(s.ad_value(139)), 0.01)), (-0.1));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_scale(13, 280, s.v[600]);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_limited_exp_ad(289, A::neg(s.ad_value(13)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(15, A::offset(A::add(s.ad_value(13), s.ad_value(289)), (-1.0)), 0.0001);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(16, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(13), 1.0), s.ad_value(289))), 0.0001);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(17, A::square(s.ad_value(13)), 0.0002);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_sub(14, 52, 63);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_sqrt_ad(77, A::offset(A::square(s.ad_value(14)), 0.0001));
        }

        s.v[1463] = if (p.p1041 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) && (s.v[1463] != 0.0)) {
            let assign24060_ad_e33151: A = {
                if (!((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::sub(s.ad_value(593), A::mul(s.ad_value(594), s.ad_value(77))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(593), A::mul(s.ad_value(594), s.ad_value(77))), A::sub(s.ad_value(593), A::mul(s.ad_value(594), s.ad_value(77)))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[593] - (s.v[594] * s.v[77])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::sub(s.ad_value(593), A::mul(s.ad_value(594), s.ad_value(77))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(13, &assign24060_ad_e33151);
        }

        s.v[1464] = if (s.v[595] < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) && (s.v[1463] != 0.0)) && (s.v[1464] != 0.0)) {
            s.store_scalar(595, 0.01);
        }

        if (((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) && (!(s.v[1463] != 0.0))) {
            s.store_sub_ad_rhs(13, 593, A::mul(s.ad_value(594), s.ad_value(77)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(595), s.ad_value(77)), 1.0);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_mul_ad_lhs(15, A::mul(s.ad_value(297), s.ad_value(13)), 14);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_ad(16, &A::limited_exp(s.ad_value(15)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_sub(14, 54, 63);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_sqrt_ad(78, A::offset(A::square(s.ad_value(14)), 0.0001));
        }

        s.v[1465] = if (p.p1041 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) && (s.v[1465] != 0.0)) {
            let assign24180_ad_e33317: A = {
                if (!((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::sub(s.ad_value(596), A::mul(s.ad_value(597), s.ad_value(78))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(596), A::mul(s.ad_value(597), s.ad_value(78))), A::sub(s.ad_value(596), A::mul(s.ad_value(597), s.ad_value(78)))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[596] - (s.v[597] * s.v[78])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::sub(s.ad_value(596), A::mul(s.ad_value(597), s.ad_value(78))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(13, &assign24180_ad_e33317);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1466] = if (s.v[598] < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) && (s.v[1465] != 0.0)) && (s.v[1466] != 0.0)) {
            s.store_scalar(598, 0.01);
        }

        if (((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) && (!(s.v[1465] != 0.0))) {
            s.store_sub_ad_rhs(13, 596, A::mul(s.ad_value(597), s.ad_value(78)));
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(598), s.ad_value(78)), 1.0);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_mul_ad_lhs(15, A::mul(s.ad_value(297), s.ad_value(13)), 14);
        }

        if ((s.v[1459] != 0.0) && (s.v[1461] != 0.0)) {
            s.store_ad(16, &A::limited_exp(s.ad_value(15)));
        }

        s.v[1467] = if (p.p45 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1467] != 0.0) {
            s.store_scalar(12, (s.v[47] * p.p77));
        }

        s.v[1468] = if (((s.v[559] <= 0.0) || (s.v[417] <= 0.0)) || (s.v[561] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1467] != 0.0) && (s.v[1468] != 0.0)) {
            s.store_scalar(18, 0.0);
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) {
            s.store_div_ad_lhs(13, A::add(A::sub(A::neg(s.ad_value(54)), s.ad_value(562)), s.ad_value(63)), 12);
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) {
            s.store_ad(13, &{
                if (!(s.v[13] < ((-10000.0) * 0.01))) {
                    A::scale(A::add(s.ad_value(13), A::sqrt(A::offset(A::square(s.ad_value(13)), ((4.0 * 0.01) * 0.01)))), 0.5)
                } else {
                    {
                        if (s.v[13] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(13))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) {
            s.store_div_ad_rhs(14, 417, A::offset(s.ad_value(13), 0.001));
        }

        s.v[1469] = if (s.v[561] != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) && (s.v[1469] != 0.0)) {
            s.store_mul_ad_lhs(15, A::square(s.ad_value(48)), 48);
        }

        if (((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) && (s.v[1469] != 0.0)) {
            s.store_offset_ad(16, A::add(s.ad_value(561), A::abs(s.ad_value(15))), 0.0001);
        }

        if (((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) && (s.v[1469] != 0.0)) {
            let assign24440_ad_e33600: A = {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(15), s.ad_value(16)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(15), s.ad_value(16)), A::div(s.ad_value(15), s.ad_value(16))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(17, assign24440_ad_e33600, (-1e-6));
        }

        if (((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) && (!(s.v[1469] != 0.0))) {
            s.store_scalar(17, 1.0);
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1468] != 0.0))) {
            s.store_mul_ad_lhs(18, A::mul(A::mul(A::scale(s.ad_value(559), s.v[29]), s.ad_value(13)), A::limited_exp(A::neg(s.ad_value(14)))), 17);
        }

        s.v[1470] = if (((s.v[563] <= 0.0) || (s.v[418] <= 0.0)) || (s.v[565] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1467] != 0.0) && (s.v[1470] != 0.0)) {
            s.store_scalar(18, 0.0);
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) {
            s.store_div_ad_lhs(13, A::add(A::sub(A::neg(s.ad_value(52)), s.ad_value(566)), s.ad_value(63)), 12);
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) {
            s.store_ad(13, &{
                if (!(s.v[13] < ((-10000.0) * 0.01))) {
                    A::scale(A::add(s.ad_value(13), A::sqrt(A::offset(A::square(s.ad_value(13)), ((4.0 * 0.01) * 0.01)))), 0.5)
                } else {
                    {
                        if (s.v[13] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(13))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) {
            s.store_div_ad_rhs(14, 418, A::offset(s.ad_value(13), 0.001));
        }

        s.v[1471] = if (s.v[565] != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) && (s.v[1471] != 0.0)) {
            s.store_mul_ad_lhs(15, A::square(s.ad_value(50)), 50);
        }

        if (((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) && (s.v[1471] != 0.0)) {
            s.store_offset_ad(16, A::add(s.ad_value(565), A::abs(s.ad_value(15))), 0.0001);
        }

        if (((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) && (s.v[1471] != 0.0)) {
            let assign24560_ad_e33803: A = {
                if (!((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(15), s.ad_value(16)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(15), s.ad_value(16)), A::div(s.ad_value(15), s.ad_value(16))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[15] / s.v[16]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(15), s.ad_value(16)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(17, assign24560_ad_e33803, (-1e-6));
        }

        if (((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) && (!(s.v[1471] != 0.0))) {
            s.store_scalar(17, 1.0);
        }

        if ((s.v[1467] != 0.0) && (!(s.v[1470] != 0.0))) {
            s.store_mul_ad_lhs(18, A::mul(A::mul(A::scale(s.ad_value(563), s.v[29]), s.ad_value(13)), A::limited_exp(A::neg(s.ad_value(14)))), 17);
        }

        s.store_div(12, 306, 343);

        s.store_offset_ad(13, A::limited_exp(s.ad_value(12)), (-1.0));

        s.store_add_ad_rhs(14, 346, A::mul(s.ad_value(345), A::sub(s.ad_value(306), s.ad_value(347))));

        s.store_mul(15, 13, 14);

        s.store_div_ad_lhs(13, A::offset(s.ad_value(306), p.p731), 343);

        s.store_limited_exp_ad(14, A::neg(s.ad_value(13)));

        s.store_mul_ad_rhs(16, 341, A::sub(A::offset(A::add(A::limited_exp(s.ad_value(12)), s.ad_value(351)), (-1.0)), A::scale(s.ad_value(14), p.p733)));

        s.store_add_ad_rhs(17, 349, A::mul(s.ad_value(348), A::sub(s.ad_value(306), s.ad_value(350))));

        s.v[1472] = if (s.v[341] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1472] != 0.0) {
            s.store_add_ad(18, A::mul(A::scale(s.ad_value(15), 0.5), A::sub_from_scalar(1.0, A::tanh(A::div(A::sub(s.ad_value(306), s.ad_value(347)), s.ad_value(343))))), A::mul(A::scale(s.ad_value(16), 0.5), A::offset(A::tanh(A::div(A::sub(s.ad_value(306), s.ad_value(347)), s.ad_value(343))), 1.0)));
        }

        s.v[1473] = if (s.v[441] > 0.0) { 1.0 } else { 0.0 };

        s.v[1474] = if ((p.p748 - s.v[306]) < (p.p748 * 0.001)) { 1.0 } else { 0.0 };

        if ((s.v[1473] != 0.0) && (s.v[1474] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(306)), s.ad_value(394)), 447);
        }

        if ((s.v[1473] != 0.0) && (s.v[1474] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if ((s.v[1473] != 0.0) && (!(s.v[1474] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(306)), s.ad_value(394)), 447);
        }

        if ((s.v[1473] != 0.0) && (!(s.v[1474] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p748), A::sub_from_scalar(p.p748, s.ad_value(306)))), (-1.0));
        }

        s.v[1475] = if (s.v[443] > 0.0) { 1.0 } else { 0.0 };

        s.v[1476] = if ((p.p750 - s.v[306]) < (p.p750 * 0.001)) { 1.0 } else { 0.0 };

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(306)), s.ad_value(394)), 449);
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(306)), s.ad_value(394)), 449);
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p750), A::sub_from_scalar(p.p750, s.ad_value(306)))), (-1.0));
        }

        s.v[1477] = if (s.v[445] > 0.0) { 1.0 } else { 0.0 };

        s.v[1478] = if ((p.p752 - s.v[306]) < (p.p752 * 0.001)) { 1.0 } else { 0.0 };

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(306)), s.ad_value(394)), 451);
        }

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(306)), s.ad_value(394)), 451);
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p752), A::sub_from_scalar(p.p752, s.ad_value(306)))), (-1.0));
        }

        s.store_div(12, 307, 344);

        s.store_offset_ad(13, A::limited_exp(s.ad_value(12)), (-1.0));

        s.store_add_ad_rhs(14, 353, A::mul(s.ad_value(352), A::sub(s.ad_value(307), s.ad_value(354))));

        s.store_mul_ad_lhs(15, A::mul(s.ad_value(302), s.ad_value(13)), 14);

        s.store_div_ad_lhs(13, A::offset(s.ad_value(307), p.p732), 344);

        s.store_limited_exp_ad(14, A::neg(s.ad_value(13)));

        s.store_mul_ad(16, A::mul(s.ad_value(302), s.ad_value(342)), A::sub(A::offset(A::add(A::limited_exp(s.ad_value(12)), s.ad_value(358)), (-1.0)), A::scale(s.ad_value(14), p.p734)));

        s.store_mul_ad_rhs(17, 302, A::add(s.ad_value(356), A::mul(s.ad_value(355), A::sub(s.ad_value(307), s.ad_value(357)))));

        s.v[1479] = if (s.v[342] > 0.0) { 1.0 } else { 0.0 };

        s.v[1480] = if (s.v[302] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1479] != 0.0) && (s.v[1480] != 0.0)) {
            s.store_add_ad(18, A::mul(A::scale(s.ad_value(15), 0.5), A::sub_from_scalar(1.0, A::tanh(A::div(A::sub(s.ad_value(307), s.ad_value(354)), s.ad_value(344))))), A::mul(A::scale(s.ad_value(16), 0.5), A::offset(A::tanh(A::div(A::sub(s.ad_value(307), s.ad_value(354)), s.ad_value(344))), 1.0)));
        }

        s.v[1481] = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_div(12, 309, 344);
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(s.ad_value(12)), (-1.0));
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_add_ad_rhs(14, 353, A::mul(s.ad_value(352), A::sub(s.ad_value(309), s.ad_value(354))));
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_mul_ad_lhs(15, A::scale(s.ad_value(13), p.p1128), 14);
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_div_ad_lhs(13, A::offset(s.ad_value(309), p.p732), 344);
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_limited_exp_ad(14, A::neg(s.ad_value(13)));
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_mul_ad(16, A::scale(s.ad_value(342), p.p1128), A::sub(A::offset(A::add(A::limited_exp(s.ad_value(12)), s.ad_value(358)), (-1.0)), A::scale(s.ad_value(14), p.p734)));
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_scale_ad(17, A::add(s.ad_value(356), A::mul(s.ad_value(355), A::sub(s.ad_value(309), s.ad_value(357)))), p.p1128);
        }

        if ((s.v[1479] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_add_ad(18, A::mul(A::scale(s.ad_value(15), 0.5), A::sub_from_scalar(1.0, A::tanh(A::div(A::sub(s.ad_value(309), s.ad_value(354)), s.ad_value(344))))), A::mul(A::scale(s.ad_value(16), 0.5), A::offset(A::tanh(A::div(A::sub(s.ad_value(309), s.ad_value(354)), s.ad_value(344))), 1.0)));
        }

        s.v[1482] = if (s.v[442] > 0.0) { 1.0 } else { 0.0 };

        s.v[1483] = if ((p.p749 - s.v[307]) < (p.p749 * 0.001)) { 1.0 } else { 0.0 };

        if ((s.v[1482] != 0.0) && (s.v[1483] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(307)), s.ad_value(394)), 448);
        }

        if ((s.v[1482] != 0.0) && (s.v[1483] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if ((s.v[1482] != 0.0) && (!(s.v[1483] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(307)), s.ad_value(394)), 448);
        }

        if ((s.v[1482] != 0.0) && (!(s.v[1483] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p749), A::sub_from_scalar(p.p749, s.ad_value(307)))), (-1.0));
        }

        s.v[1484] = if (s.v[444] > 0.0) { 1.0 } else { 0.0 };

        s.v[1485] = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        s.v[1486] = if (s.v[301] > (s.v[35] * p.p2)) { 1.0 } else { 0.0 };

        if (((s.v[1484] != 0.0) && (s.v[1485] != 0.0)) && (s.v[1486] != 0.0)) {
            s.store_mul_ad_lhs(14, A::mul(s.ad_value(302), A::offset(s.ad_value(301), (-(s.v[35] * p.p2)))), 444);
        }

        if (((s.v[1484] != 0.0) && (s.v[1485] != 0.0)) && (!(s.v[1486] != 0.0))) {
            s.store_mul_ad_lhs(14, A::mul(s.ad_value(302), s.ad_value(301)), 444);
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) {
            s.store_mul_ad_lhs(14, A::mul(s.ad_value(302), s.ad_value(301)), 444);
        }

        s.v[1487] = if ((p.p751 - s.v[307]) < (p.p751 * 0.001)) { 1.0 } else { 0.0 };

        if ((s.v[1484] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(307)), s.ad_value(394)), 450);
        }

        if ((s.v[1484] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1487] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(307)), s.ad_value(394)), 450);
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1487] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p751), A::sub_from_scalar(p.p751, s.ad_value(307)))), (-1.0));
        }

        s.v[1488] = if (s.v[446] > 0.0) { 1.0 } else { 0.0 };

        s.v[1489] = if ((p.p753 - s.v[307]) < (p.p753 * 0.001)) { 1.0 } else { 0.0 };

        if ((s.v[1488] != 0.0) && (s.v[1489] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(307)), s.ad_value(394)), 452);
        }

        if ((s.v[1488] != 0.0) && (s.v[1489] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if ((s.v[1488] != 0.0) && (!(s.v[1489] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(307)), s.ad_value(394)), 452);
        }

        if ((s.v[1488] != 0.0) && (!(s.v[1489] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p753), A::sub_from_scalar(p.p753, s.ad_value(307)))), (-1.0));
        }

        s.v[1490] = if (p.p1128 > 0.0) { 1.0 } else { 0.0 };

        s.v[1491] = if (s.v[442] > 0.0) { 1.0 } else { 0.0 };

        s.v[1492] = if ((p.p749 - s.v[309]) < (p.p749 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1490] != 0.0) && (s.v[1491] != 0.0)) && (s.v[1492] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(309)), s.ad_value(394)), 448);
        }

        if (((s.v[1490] != 0.0) && (s.v[1491] != 0.0)) && (s.v[1492] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if (((s.v[1490] != 0.0) && (s.v[1491] != 0.0)) && (!(s.v[1492] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(309)), s.ad_value(394)), 448);
        }

        if (((s.v[1490] != 0.0) && (s.v[1491] != 0.0)) && (!(s.v[1492] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p749), A::sub_from_scalar(p.p749, s.ad_value(309)))), (-1.0));
        }

        s.v[1493] = if (s.v[444] > 0.0) { 1.0 } else { 0.0 };

        s.v[1494] = if (s.v[301] > (s.v[35] * p.p2)) { 1.0 } else { 0.0 };

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (s.v[1494] != 0.0)) {
            s.store_mul_ad_lhs(14, A::offset(A::scale(A::offset(s.ad_value(301), (-(s.v[35] * p.p2))), p.p1128), (s.v[35] * p.p2)), 444);
        }

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (!(s.v[1494] != 0.0))) {
            s.store_mul_ad_lhs(14, A::scale(s.ad_value(301), p.p1128), 444);
        }

        s.v[1495] = if ((p.p751 - s.v[309]) < (p.p751 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(309)), s.ad_value(394)), 450);
        }

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (s.v[1495] != 0.0)) {
            s.store_offset_ad(13, A::limited_exp(A::scale(s.ad_value(12), 1000.0)), (-1.0));
        }

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_div_ad_lhs(12, A::div(A::neg(s.ad_value(309)), s.ad_value(394)), 450);
        }

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p751), A::sub_from_scalar(p.p751, s.ad_value(309)))), (-1.0));
        }

        s.store_mul(312, 423, 250);

        s.store_mul(315, 424, 300);

        s.store_scale(318, 428, (s.v[35] * p.p2));

        s.v[313] = ((0.1) as f64).powf((-p.p713));

        s.v[1496] = if (p.p713 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1496] != 0.0) {
            s.store_scalar(314, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1496] != 0.0)) {
            s.store_scalar(314, ((1.0 / (1.0 - p.p713)) * (1.0 - (((0.05 * p.p713) * (1.0 + p.p713)) * s.v[313]))));
        }

        s.v[316] = ((0.1) as f64).powf((-p.p715));

        s.v[1497] = if (p.p715 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1497] != 0.0) {
            s.store_scalar(317, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1497] != 0.0)) {
            s.store_scalar(317, ((1.0 / (1.0 - p.p715)) * (1.0 - (((0.05 * p.p715) * (1.0 + p.p715)) * s.v[316]))));
        }

        s.v[319] = ((0.1) as f64).powf((-p.p717));

        s.v[1498] = if (p.p717 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1498] != 0.0) {
            s.store_scalar(320, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1498] != 0.0)) {
            s.store_scalar(320, ((1.0 / (1.0 - p.p717)) * (1.0 - (((0.05 * p.p717) * (1.0 + p.p717)) * s.v[319]))));
        }

        s.v[1499] = if (s.v[312] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1499] != 0.0) {
            s.store_div(13, 306, 429);
        }

        s.v[1500] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1501] = if (p.p713 != 1.0) { 1.0 } else { 0.0 };

        s.v[1502] = if (p.p713 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p713)));
        }

        if (((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (s.v[1501] != 0.0)) {
            s.store_scale_ad(331, A::mul(A::mul(s.ad_value(429), s.ad_value(312)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p713)));
        }

        if (((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (!(s.v[1501] != 0.0))) {
            s.store_mul_ad(331, A::mul(s.ad_value(429), s.ad_value(312)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1499] != 0.0) && (!(s.v[1500] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[313]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p713)), (1.0 + p.p713)));
        }

        if ((s.v[1499] != 0.0) && (!(s.v[1500] != 0.0))) {
            s.store_mul_ad(331, A::mul(s.ad_value(429), s.ad_value(312)), A::add(s.ad_value(14), s.ad_value(314)));
        }

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1499] != 0.0)) {
            s.store_scalar(331, 0.0);
        }

        s.v[1503] = if (s.v[315] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1503] != 0.0) {
            s.store_div(13, 306, 430);
        }

        s.v[1504] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1505] = if (p.p715 != 1.0) { 1.0 } else { 0.0 };

        s.v[1506] = if (p.p715 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p715)));
        }

        if (((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (s.v[1505] != 0.0)) {
            s.store_scale_ad(332, A::mul(A::mul(s.ad_value(430), s.ad_value(315)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p715)));
        }

        if (((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (!(s.v[1505] != 0.0))) {
            s.store_mul_ad(332, A::mul(s.ad_value(430), s.ad_value(315)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1503] != 0.0) && (!(s.v[1504] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[316]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p715)), (1.0 + p.p715)));
        }

        if ((s.v[1503] != 0.0) && (!(s.v[1504] != 0.0))) {
            s.store_mul_ad(332, A::mul(s.ad_value(430), s.ad_value(315)), A::add(s.ad_value(14), s.ad_value(317)));
        }

        if (!(s.v[1503] != 0.0)) {
            s.store_scalar(332, 0.0);
        }

        s.v[1507] = if (s.v[318] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1507] != 0.0) {
            s.store_div(13, 306, 431);
        }

        s.v[1508] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1509] = if (p.p717 != 1.0) { 1.0 } else { 0.0 };

        s.v[1510] = if (p.p717 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (s.v[1509] != 0.0)) && (s.v[1510] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (s.v[1509] != 0.0)) && (!(s.v[1510] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p717)));
        }

        if (((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (s.v[1509] != 0.0)) {
            s.store_scale_ad(333, A::mul(A::mul(s.ad_value(431), s.ad_value(318)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p717)));
        }

        if (((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (!(s.v[1509] != 0.0))) {
            s.store_mul_ad(333, A::mul(s.ad_value(431), s.ad_value(318)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1507] != 0.0) && (!(s.v[1508] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[319]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p717)), (1.0 + p.p717)));
        }

        if ((s.v[1507] != 0.0) && (!(s.v[1508] != 0.0))) {
            s.store_mul_ad(333, A::mul(s.ad_value(431), s.ad_value(318)), A::add(s.ad_value(14), s.ad_value(320)));
        }

        if (!(s.v[1507] != 0.0)) {
            s.store_scalar(333, 0.0);
        }

        s.store_add_ad_lhs(330, A::add(s.ad_value(331), s.ad_value(332)), 333);

        s.store_mul_ad_lhs(321, A::mul(s.ad_value(302), s.ad_value(426)), 251);

        s.v[1511] = if (s.v[301] > (s.v[35] * p.p2)) { 1.0 } else { 0.0 };

        s.v[1512] = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1511] != 0.0) && (s.v[1512] != 0.0)) {
            s.store_mul_ad(324, A::mul(s.ad_value(302), s.ad_value(427)), A::offset(s.ad_value(301), (-(s.v[35] * p.p2))));
        }

        if ((s.v[1511] != 0.0) && (!(s.v[1512] != 0.0))) {
            s.store_mul_ad_lhs(324, A::mul(s.ad_value(302), s.ad_value(427)), 301);
        }

        if (!(s.v[1511] != 0.0)) {
            s.store_mul_ad_lhs(324, A::mul(s.ad_value(302), s.ad_value(427)), 301);
        }

        s.store_scale(327, 425, (s.v[35] * p.p2));

        s.v[322] = ((0.1) as f64).powf((-p.p714));

        s.v[1513] = if (p.p714 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1513] != 0.0) {
            s.store_scalar(323, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1513] != 0.0)) {
            s.store_scalar(323, ((1.0 / (1.0 - p.p714)) * (1.0 - (((0.05 * p.p714) * (1.0 + p.p714)) * s.v[322]))));
        }

        s.v[325] = ((0.1) as f64).powf((-p.p716));

        s.v[1514] = if (p.p716 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1514] != 0.0) {
            s.store_scalar(326, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1514] != 0.0)) {
            s.store_scalar(326, ((1.0 / (1.0 - p.p716)) * (1.0 - (((0.05 * p.p716) * (1.0 + p.p716)) * s.v[325]))));
        }

        s.v[328] = ((0.1) as f64).powf((-p.p718));

        s.v[1515] = if (p.p718 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1515] != 0.0) {
            s.store_scalar(329, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1515] != 0.0)) {
            s.store_scalar(329, ((1.0 / (1.0 - p.p718)) * (1.0 - (((0.05 * p.p718) * (1.0 + p.p718)) * s.v[328]))));
        }

        s.v[1516] = if (s.v[321] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1516] != 0.0) {
            s.store_div(13, 308, 432);
        }

        s.v[1517] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1518] = if (p.p714 != 1.0) { 1.0 } else { 0.0 };

        s.v[1519] = if (p.p714 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) && (!(s.v[1519] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p714)));
        }

        if (((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) {
            s.store_scale_ad(335, A::mul(A::mul(s.ad_value(432), s.ad_value(321)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p714)));
        }

        if (((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (!(s.v[1518] != 0.0))) {
            s.store_mul_ad(335, A::mul(s.ad_value(432), s.ad_value(321)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1516] != 0.0) && (!(s.v[1517] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[322]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p714)), (1.0 + p.p714)));
        }

        if ((s.v[1516] != 0.0) && (!(s.v[1517] != 0.0))) {
            s.store_mul_ad(335, A::mul(s.ad_value(432), s.ad_value(321)), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if (!(s.v[1516] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        s.v[1520] = if (s.v[324] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1520] != 0.0) {
            s.store_div(13, 308, 433);
        }

        s.v[1521] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1522] = if (p.p716 != 1.0) { 1.0 } else { 0.0 };

        s.v[1523] = if (p.p716 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) && (s.v[1523] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) && (!(s.v[1523] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p716)));
        }

        if (((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) {
            s.store_scale_ad(336, A::mul(A::mul(s.ad_value(433), s.ad_value(324)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p716)));
        }

        if (((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (!(s.v[1522] != 0.0))) {
            s.store_mul_ad(336, A::mul(s.ad_value(433), s.ad_value(324)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1520] != 0.0) && (!(s.v[1521] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[325]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p716)), (1.0 + p.p716)));
        }

        if ((s.v[1520] != 0.0) && (!(s.v[1521] != 0.0))) {
            s.store_mul_ad(336, A::mul(s.ad_value(433), s.ad_value(324)), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if (!(s.v[1520] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        s.v[1524] = if (s.v[327] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1524] != 0.0) {
            s.store_div(13, 308, 434);
        }

        s.v[1525] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1526] = if (p.p718 != 1.0) { 1.0 } else { 0.0 };

        s.v[1527] = if (p.p718 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (s.v[1526] != 0.0)) && (s.v[1527] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (s.v[1526] != 0.0)) && (!(s.v[1527] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p718)));
        }

        if (((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (s.v[1526] != 0.0)) {
            s.store_scale_ad(337, A::mul(A::mul(s.ad_value(434), s.ad_value(327)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p718)));
        }

        if (((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (!(s.v[1526] != 0.0))) {
            s.store_mul_ad(337, A::mul(s.ad_value(434), s.ad_value(327)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1524] != 0.0) && (!(s.v[1525] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[328]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p718)), (1.0 + p.p718)));
        }

        if ((s.v[1524] != 0.0) && (!(s.v[1525] != 0.0))) {
            s.store_mul_ad(337, A::mul(s.ad_value(434), s.ad_value(327)), A::add(s.ad_value(14), s.ad_value(329)));
        }

        if (!(s.v[1524] != 0.0)) {
            s.store_scalar(337, 0.0);
        }

        s.store_add_ad_lhs(334, A::add(s.ad_value(335), s.ad_value(336)), 337);

        s.v[1528] = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1528] != 0.0) {
            s.store_mul_ad_lhs(321, A::scale(s.ad_value(426), p.p1128), 251);
        }

        s.v[1529] = if (s.v[301] > (s.v[35] * p.p2)) { 1.0 } else { 0.0 };

        if ((s.v[1528] != 0.0) && (s.v[1529] != 0.0)) {
            s.store_mul_ad_rhs(324, 427, A::offset(A::scale(A::offset(s.ad_value(301), (-(s.v[35] * p.p2))), p.p1128), (s.v[35] * p.p2)));
        }

        if ((s.v[1528] != 0.0) && (!(s.v[1529] != 0.0))) {
            s.store_mul_ad_lhs(324, A::scale(s.ad_value(427), p.p1128), 301);
        }

        s.v[1530] = if (s.v[321] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) {
            s.store_div(13, 309, 432);
        }

        s.v[1531] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if (((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1532] = if (p.p714 != 1.0) { 1.0 } else { 0.0 };

        s.v[1533] = if (p.p714 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) && (s.v[1533] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if (((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) && (!(s.v[1533] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p714)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_scale_ad(339, A::mul(A::mul(s.ad_value(432), s.ad_value(321)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p714)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_mul_ad(339, A::mul(s.ad_value(432), s.ad_value(321)), A::neg(A::ln(s.ad_value(310))));
        }

        if (((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (!(s.v[1531] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[322]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p714)), (1.0 + p.p714)));
        }

        if (((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (!(s.v[1531] != 0.0))) {
            s.store_mul_ad(339, A::mul(s.ad_value(432), s.ad_value(321)), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if ((s.v[1528] != 0.0) && (!(s.v[1530] != 0.0))) {
            s.store_scalar(339, 0.0);
        }

        s.v[1534] = if (s.v[324] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) {
            s.store_div(13, 309, 433);
        }

        s.v[1535] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if (((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1536] = if (p.p716 != 1.0) { 1.0 } else { 0.0 };

        s.v[1537] = if (p.p716 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if (((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p716)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (s.v[1536] != 0.0)) {
            s.store_scale_ad(340, A::mul(A::mul(s.ad_value(433), s.ad_value(324)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p716)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (!(s.v[1536] != 0.0))) {
            s.store_mul_ad(340, A::mul(s.ad_value(433), s.ad_value(324)), A::neg(A::ln(s.ad_value(310))));
        }

        if (((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (!(s.v[1535] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[325]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p716)), (1.0 + p.p716)));
        }

        if (((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (!(s.v[1535] != 0.0))) {
            s.store_mul_ad(340, A::mul(s.ad_value(433), s.ad_value(324)), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if ((s.v[1528] != 0.0) && (!(s.v[1534] != 0.0))) {
            s.store_scalar(340, 0.0);
        }

        if (s.v[1528] != 0.0) {
            s.store_add(338, 339, 340);
        }

        if (!(s.v[1528] != 0.0)) {
            s.store_scalar(338, 0.0);
        }

        s.v[1538] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1538] != 0.0) {
            s.store_powf_ad(13, A::scale(s.ad_value(481), 1.0000000000000001e-23), p.p954);
        }

        if (s.v[1538] != 0.0) {
            s.store_powf_ad(14, A::div_from_scalar(300.0, s.ad_value(391)), p.p955);
        }

        if (s.v[1538] != 0.0) {
            s.store_div_ad_lhs(15, A::mul(A::scale(s.ad_value(187), p.p953), A::voltage(ctx, &nodes, Some(11), Some(7))), 108);
        }

        s.store_div_ad_lhs(360, A::scale(s.ad_value(502), 2.0), 157);

        s.v[1539] = if (p.p784 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1539] != 0.0) {
            s.store_scalar(363, 0.0);
        }

        if (!(s.v[1539] != 0.0)) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(167), s.ad_value(129)), p.p784), 360);
        }

        if (!(s.v[1539] != 0.0)) {
            s.store_mul_ad_rhs(363, 129, A::ln(A::max_with_scalar(s.ad_value(12), 1e-38)));
        }

        s.v[1540] = if (s.v[363] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1539] != 0.0)) && (s.v[1540] != 0.0)) {
            s.store_scalar(363, 0.0);
        }

        s.store_mul_ad(367, A::scale(s.ad_value(108), 6.241457005723417e18), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(483)));

        s.store_scale_ad(366, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(90), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(144)), s.ad_value(628)), s.ad_value(611)), 6.241457005723417e18);

        s.store_mul_ad_lhs(736, A::mul(A::scale(s.ad_value(108), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(188))), 157);

        s.store_mul_ad_lhs(737, A::mul(A::scale(s.ad_value(108), 1.60219e-19), s.ad_value(188)), 188);

        s.store_add_ad(738, A::offset(A::scale(s.ad_value(366), p.p799), p.p785), A::mul(A::scale(s.ad_value(366), p.p800), s.ad_value(366)));

        s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));

        s.store_scale(740, 108, (p.p785 * 1.60219e-19));

        s.v[1541] = if (p.p1065 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1541] != 0.0) {
            s.store_scalar(745, s.v[30]);
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad_lhs(712, A::sub(s.ad_value(64), s.ad_value(482)), 108);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale_ad(713, A::sqrt(A::div_from_scalar((((2.0 * 1.60219e-19) * s.v[26]) * p.p1068), s.ad_value(108))), 1.0 / (s.v[46]));
        }

        if (s.v[1541] != 0.0) {
            s.store_ln_ad(714, A::div_from_scalar(p.p1068, s.ad_value(28)));
        }

        if (s.v[1541] != 0.0) {
            s.store_scalar(13, 1.0);
        }

        if (s.v[1541] != 0.0) {
            s.store_div(204, 712, 13);
        }

        if (s.v[1541] != 0.0) {
            s.store_div(205, 713, 13);
        }

        if (s.v[1541] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1541] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1542] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1542] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1541] != 0.0) && (s.v[1542] != 0.0)) {
            s.store_neg_ad(715, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_sub_ad_lhs(715, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1541] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(715), (-1.0)), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1541] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(713), A::scale(s.ad_value(96), 2.0)), 1.0), 713);
        }

        if (s.v[1541] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(715), A::scale(s.ad_value(714), 2.0)), 73);
        }

        if (s.v[1541] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1541] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1541] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1543] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1544] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (s.v[1544] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1545] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (s.v[1545] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (!(s.v[1545] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (!(s.v[1545] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (!(s.v[1545] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) {
            s.store_mul_ad_rhs(717, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_rhs(717, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1546] = if ((1.0 == 0.0) && (s.v[715] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1546] != 0.0)) {
            s.store_div_from_scalar_ad(716, ((-2.0) * 2.0), A::scale(s.ad_value(715), 16.0));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1546] != 0.0))) {
            s.store_scale_ad(716, A::add(A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(715), (-1.0)), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1541] != 0.0) {
            s.store_offset_ad(718, A::div(s.ad_value(713), A::scale(A::sqrt(s.ad_value(716)), 2.0)), 1.0);
        }

        if (s.v[1541] != 0.0) {
            s.copy_ad(719, 157);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale(726, 719, (s.v[46] * s.v[29]));
        }

        if (s.v[1541] != 0.0) {
            s.store_scale(725, 157, (s.v[46] * s.v[29]));
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad(720, A::mul(s.ad_value(188), s.ad_value(746)), A::mul(A::mul(A::mul(A::scale(s.ad_value(718), 2.0), s.ad_value(726)), s.ad_value(108)), s.ad_value(108)));
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad(722, A::mul(s.ad_value(188), A::sub(s.ad_value(745), s.ad_value(746))), A::mul(A::mul(A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(725)), s.ad_value(106)), s.ad_value(106)));
        }

        if (s.v[1541] != 0.0) {
            s.store_offset_ad(12, A::scale(A::sub(A::add(A::square(s.ad_value(717)), s.ad_value(717)), s.ad_value(720)), 4.0), 1.0);
        }

        if (s.v[1541] != 0.0) {
            s.store_offset_ad(723, A::scale(A::sqrt(A::offset(A::scale(A::add(A::add(A::square(s.ad_value(144)), s.ad_value(144)), s.ad_value(722)), 4.0), 1.0)), 0.5), (-0.5));
        }

        s.v[1548] = if (s.v[30] != s.v[746]) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale_ad(724, A::mul(A::mul(A::scale(s.ad_value(90), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(723)), 6.241457005723417e18);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_sub_ad_lhs(361, A::sub(s.ad_value(745), A::scale(s.ad_value(359), 2.0)), 746);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_square(362, 361);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale(13, 362, (10000000000.0 * s.v[46]));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale_ad(14, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(724), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367))), 1e-38)), p.p785);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scaled_sub(15, 724, 366, p.p799);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale_ad(16, A::sub(A::square(s.ad_value(724)), A::square(s.ad_value(366))), (0.5 * p.p800));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale(17, 362, (10000000000.0 * (s.v[29] * p.p2)));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_add_ad(732, A::mul(A::div(s.ad_value(736), s.ad_value(13)), A::add(A::add(s.ad_value(14), s.ad_value(15)), s.ad_value(16))), A::div(A::mul(A::mul(A::div(s.ad_value(737), s.ad_value(17)), s.ad_value(363)), s.ad_value(738)), s.ad_value(739)));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_mul_ad_lhs(18, A::mul(A::scale(s.ad_value(361), ((s.v[29] * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_mul_ad_lhs(733, A::mul(A::div(s.ad_value(740), s.ad_value(18)), s.ad_value(188)), 188);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_add(19, 733, 732);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale(20, 108, (p.p1067 * 1.60219e-19));
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(21, A::mul(A::scale(s.ad_value(746), ((s.v[29] * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(741, A::mul(A::div(s.ad_value(20), s.ad_value(21)), s.ad_value(188)), 188);
        }

        if (s.v[1541] != 0.0) {
            s.copy_ad(22, 741);
        }

        s.v[1551] = if (p.p801 >= (s.v[30] / 2.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1541] != 0.0)) && (s.v[1551] != 0.0)) {
            s.store_scalar(359, 0.0);
        }

        if ((!(s.v[1541] != 0.0)) && (!(s.v[1551] != 0.0))) {
            s.store_scalar(359, p.p801);
        }

        s.v[1552] = if (((p.p785 > 0.0) || (p.p799 > 0.0)) || (p.p800 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1553] = if ((p.p786 != 0.0) && (p.p785 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_div(13, 80, 641);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_offset_ad(14, A::pow(s.ad_value(13), s.ad_value(642)), 1.0);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_div(15, 640, 14);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_scale(16, 15, 1.0 / (p.p785));
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_scale_ad(17, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * p.p798) * p.p798)))), 0.5);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_scale(364, 17, p.p785);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) {
            s.store_scalar(364, p.p785);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_sub_from_scalar_ad(361, s.v[30], A::scale(s.ad_value(359), 2.0));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_square(362, 361);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale(12, 362, (10000000000.0 * s.v[46]));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale_ad(365, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(90), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(200)), s.ad_value(628)), s.ad_value(611)), 6.241457005723417e18);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_rhs(13, 364, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(365), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367))), 1e-38)));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scaled_sub(14, 365, 366, p.p799);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale_ad(15, A::sub(A::square(s.ad_value(365)), A::square(s.ad_value(366))), (0.5 * p.p800));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale(16, 362, (10000000000.0 * (s.v[29] * p.p2)));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_add_ad(368, A::mul(A::div(s.ad_value(736), s.ad_value(12)), A::add(A::add(s.ad_value(13), s.ad_value(14)), s.ad_value(15))), A::div(A::mul(A::mul(A::div(s.ad_value(737), s.ad_value(16)), s.ad_value(363)), s.ad_value(738)), s.ad_value(739)));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_lhs(17, A::mul(A::scale(s.ad_value(361), ((s.v[29] * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_lhs(740, A::scale(s.ad_value(364), 1.60219e-19), 108);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_lhs(369, A::mul(A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(188)), 188);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_add(18, 369, 368);
        }

        s.store_scaled_div(12, 80, 360, 1.0 / (s.v[30]));

        s.store_square(13, 12);

        s.store_scale_ad(15, A::offset(A::scale(s.ad_value(13), (p.p814 * s.v[30])), 1.0), p.p811);

        s.store_scale_ad(16, A::offset(A::scale(s.ad_value(13), (p.p815 * s.v[30])), 1.0), p.p812);

        s.store_scale_ad(17, A::offset(A::scale(s.ad_value(13), (p.p1044 * s.v[30])), 1.0), p.p1043);

        s.store_square(389, 17);

        s.store_square(388, 16);

        s.v[1555] = if (p.p48 == 0.0) { 1.0 } else { 0.0 };

        s.v[1556] = if (p.p48 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1555] != 0.0) {
            s.store_mul_ad_lhs(196, A::scale(s.ad_value(108), ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46])), 190);
        }

        if (s.v[1555] != 0.0) {
            s.store_mul_ad_lhs(197, A::scale(s.ad_value(108), ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46])), 193);
        }

        if (s.v[1555] != 0.0) {
            s.store_mul_ad_rhs(12, 157, A::abs(A::add(s.ad_value(196), s.ad_value(197))));
        }

        if (s.v[1555] != 0.0) {
            s.store_offset_ad(13, A::mul(s.ad_value(12), s.ad_value(244)), (s.v[30] * s.v[30]));
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad_lhs(382, A::scale(s.ad_value(90), 2.0), 106);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad_lhs(12, A::scale(A::mul(A::mul(s.ad_value(157), s.ad_value(163)), s.ad_value(175)), s.v[46]), 382);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scaled_add(13, 200, 144, 0.5);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_offset(15, 13, 0.5);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_square(16, 15);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul(17, 16, 15);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_sub(18, 200, 144);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_square(19, 18);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul(20, 19, 18);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad_lhs(21, A::offset(A::scale(s.ad_value(13), 6.0), 0.5), 19);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scale(381, 163, s.v[30]);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scale(22, 381, 1.0 / (s.v[30]));
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_offset_ad(24, A::div(A::mul(s.ad_value(389), A::div(s.ad_value(139), s.ad_value(140))), A::offset(s.ad_value(80), p.p1045)), 1.0);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_offset_ad(24, A::scale(A::offset(s.ad_value(24), (-1.0)), ((((-s.v[30]) / p.p1042)) as f64).exp()), 1.0);
        }

        s.v[1557] = if ((0.0 == 0.0) && (s.v[24] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (s.v[1557] != 0.0)) {
            s.store_div_from_scalar_ad(24, ((-0.1) * 0.1), A::scale(s.ad_value(24), 16.0));
        }

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (!(s.v[1557] != 0.0))) {
            s.store_scale_ad(24, A::add(s.ad_value(24), A::sqrt(A::offset(A::mul(s.ad_value(24), s.ad_value(24)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            let assign29020_ad_e38614: A = A::div(A::mul(A::scale(A::mul(A::mul(A::mul(s.ad_value(381), s.ad_value(22)), s.ad_value(22)), A::add(A::sub(A::div(s.ad_value(13), s.ad_value(16)), A::div(s.ad_value(21), A::mul(A::scale(s.ad_value(16), 60.0), s.ad_value(16)))), A::div(A::square(s.ad_value(19)), A::mul(A::scale(s.ad_value(16), 144.0), s.ad_value(17))))), (15.0 * 0.25)), s.ad_value(388)), A::scale(s.ad_value(12), ((p.p2 * s.v[29]) * 12.0)));
            s.store_ad(378, &assign29020_ad_e38614);
        }

        s.copy_ad(60, 59);

        s.v[218] = 0.0;

        s.v[1562] = if (p.p40 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1562] != 0.0) {
            s.store_offset(549, 549, p.p35);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(65, 64, 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(73, 72, 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(58, 549, 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub(60, 65, 58);
        }

        if (s.v[1562] != 0.0) {
            s.store_ln_ad(233, A::max_with_scalar(A::div(s.ad_value(550), s.ad_value(28)), 1e-38));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(234, A::sqrt(A::mul(A::scale(s.ad_value(550), ((2.0 * 1.60219e-19) * s.v[26])), s.ad_value(109))), 1.0 / (s.v[46]));
        }

        if (s.v[1562] != 0.0) {
            s.store_div_from_scalar(126, 1.0, 234);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(206, A::scale(s.ad_value(479), ((2.0 * 1.60219e-19) * s.v[26])), A::scale(s.ad_value(108), (s.v[46] * s.v[46])));
        }

        if (s.v[1562] != 0.0) {
            s.store_ad(218, &{
                if (s.v[479] > 0.0) {
                    A::div_from_scalar(1.0, s.ad_value(206))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1562] != 0.0) {
            s.store_ad(203, &{
                if (s.v[479] > 0.0) {
                    A::div(s.ad_value(550), s.ad_value(479))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(13, 203, 1.0);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(204, 60, 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(205, 234, 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1562] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1563] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1563] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1562] != 0.0) && (s.v[1563] != 0.0)) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(234), A::scale(s.ad_value(96), 2.0)), 1.0), 234);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(233), 2.0)), 73);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(14, A::scale(s.ad_value(13), 1.0 / (p.p1137)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1564] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1565] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (s.v[1565] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1566] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (s.v[1566] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (!(s.v[1566] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (!(s.v[1566] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (!(s.v[1566] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) {
            s.store_mul_ad_rhs(200, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), A::scale(s.ad_value(20), p.p1137)), A::scale(A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_mul_ad(18, A::scale(A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), p.p1137), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-p.p1137)), A::div_from_scalar(p.p1137, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_rhs(200, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1567] = if ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1567] != 0.0)) {
            s.store_div_from_scalar_ad(93, ((-2.0) * 2.0), A::scale(s.ad_value(91), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1567] != 0.0))) {
            s.store_scale_ad(93, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(96, 93);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad_rhs(92, 91, A::scale(s.ad_value(200), 2.0));
        }

        s.v[1568] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1568] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1568] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_offset_ad(90, A::div(s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12)))), 1.0);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_rhs(12, 108, A::sub(A::sub(s.ad_value(60), s.ad_value(91)), A::mul(A::scale(s.ad_value(200), 2.0), A::offset(s.ad_value(90), (-1.0)))));
        }

        s.v[1569] = if ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1569] != 0.0)) {
            s.store_div_from_scalar_ad(84, ((-0.1) * 0.1), A::scale(s.ad_value(12), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1569] != 0.0))) {
            s.store_scale_ad(84, A::add(s.ad_value(12), A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_lhs(130, A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(108)), 200);
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(132, A::add(s.ad_value(84), A::scale(s.ad_value(130), s.v[158])), s.v[155]);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(15, A::add(s.ad_value(506), A::mul(s.ad_value(516), s.ad_value(62))), A::pow(s.ad_value(132), s.ad_value(407)));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(16, 15, 1.0);
        }

        s.v[1570] = if ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1570] != 0.0)) {
            s.store_div_from_scalar_ad(133, ((-0.0015) * 0.0015), A::scale(s.ad_value(16), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1570] != 0.0))) {
            s.store_scale_ad(133, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(137, A::mul(A::div(s.ad_value(499), s.ad_value(133)), s.ad_value(108)), A::scale(s.ad_value(411), s.v[34]));
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(131, A::mul(s.ad_value(137), A::add(A::square(s.ad_value(200)), s.ad_value(200))), A::offset(A::mul(s.ad_value(137), A::offset(s.ad_value(200), 1.0)), 1.0));
        }

        if (s.v[1562] != 0.0) {
            let assign29900_ad_e39747: A = A::sub(A::sub(s.ad_value(91), A::scale(s.ad_value(233), 2.0)), A::add(A::scale(s.ad_value(131), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(131), 2.0), s.ad_value(90)), s.ad_value(126)), A::add(A::mul(A::mul(A::scale(s.ad_value(131), 2.0), s.ad_value(90)), s.ad_value(126)), A::div(s.ad_value(234), A::offset(s.ad_value(90), (-1.0))))), 1e-38))));
            s.store_ad(145, &assign29900_ad_e39747);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(146, 145, 108);
        }

        s.v[1571] = if ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1571] != 0.0)) {
            s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::scale(A::sub(s.ad_value(146), s.ad_value(72)), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1571] != 0.0))) {
            s.store_scale_ad(141, A::add(A::sub(s.ad_value(146), s.ad_value(72)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(146), s.ad_value(72)), A::sub(s.ad_value(146), s.ad_value(72))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        s.v[1572] = if ((p.p1134 == 0.0) && (p.p1135 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1572] != 0.0)) {
            s.store_scalar(783, p.p1129);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) {
            s.store_div_from_scalar_ad(13, s.v[30], A::offset(A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) {
            s.store_offset_ad(783, A::div(A::sub(A::scale(s.ad_value(13), p.p1134), A::mul(A::mul(A::scale(s.ad_value(13), p.p1135), s.ad_value(200)), s.ad_value(106))), A::offset(A::scale(s.ad_value(61), p.p1136), 1.0)), 1.0);
        }

        s.v[1573] = if ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005))) { 1.0 } else { 0.0 };

        if (((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) && (s.v[1573] != 0.0)) {
            s.store_div_from_scalar_ad(783, ((-0.0005) * 0.0005), A::scale(s.ad_value(783), 16.0));
        }

        if (((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) && (!(s.v[1573] != 0.0))) {
            s.store_scale_ad(783, A::add(A::offset(s.ad_value(783), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(783), (-0.1)), A::offset(s.ad_value(783), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(141, 141, 783);
        }

        if (s.v[1562] != 0.0) {
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
        }

        if (s.v[1562] != 0.0) {
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(139, 75, 20);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_lhs(142, A::add(s.ad_value(139), s.ad_value(72)), 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(234), A::scale(s.ad_value(96), 2.0)), 1.0), 234);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(233), 2.0)), 142);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(14, A::scale(s.ad_value(13), 1.0 / (p.p1137)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1574] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1575] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1576] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1576] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1576] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1576] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1576] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) {
            s.store_mul_ad_rhs(144, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), A::scale(s.ad_value(20), p.p1137)), A::scale(A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_mul_ad(18, A::scale(A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), p.p1137), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-p.p1137)), A::div_from_scalar(p.p1137, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_rhs(144, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset_ad(92, A::sub(A::sub(s.ad_value(91), s.ad_value(200)), s.ad_value(144)), (-1.0));
        }

        s.v[1577] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1577] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1577] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(14, 12);
        }

        if (s.v[1562] != 0.0) {
            s.store_add_ad(15, A::offset(s.ad_value(203), 1.0), A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset_ad(16, A::mul(A::mul(s.ad_value(203), s.ad_value(14)), s.ad_value(126)), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt_ad(17, A::add(A::square(s.ad_value(16)), A::mul(A::mul(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144))), s.ad_value(218))));
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_rhs(90, 15, A::add(s.ad_value(16), s.ad_value(17)));
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_rhs(12, 108, A::sub(A::sub(s.ad_value(60), s.ad_value(91)), A::mul(A::scale(s.ad_value(200), 2.0), A::offset(s.ad_value(90), (-1.0)))));
        }

        s.v[1578] = if ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1578] != 0.0)) {
            s.store_div_from_scalar_ad(84, ((-0.1) * 0.1), A::scale(s.ad_value(12), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1578] != 0.0))) {
            s.store_scale_ad(84, A::add(s.ad_value(12), A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_rhs(13, 108, A::sub(A::sub(s.ad_value(60), s.ad_value(91)), A::mul(A::scale(s.ad_value(144), 2.0), A::offset(s.ad_value(90), (-1.0)))));
        }

        s.v[1579] = if ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1579] != 0.0)) {
            s.store_div_from_scalar_ad(85, ((-0.1) * 0.1), A::scale(s.ad_value(13), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1579] != 0.0))) {
            s.store_scale_ad(85, A::add(s.ad_value(13), A::sqrt(A::offset(A::mul(s.ad_value(13), s.ad_value(13)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_scaled_add(86, 84, 85, 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(80, A::mul(s.ad_value(90), s.ad_value(108)), A::add(s.ad_value(200), s.ad_value(144)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(156, A::add(s.ad_value(86), A::scale(s.ad_value(80), s.v[158])), s.v[155]);
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(13, 203, 1.0);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_lhs(204, A::add(s.ad_value(60), A::scale(s.ad_value(109), p.p136)), 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(205, 234, 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1562] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1580] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1580] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1562] != 0.0) && (s.v[1580] != 0.0)) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(15, A::add(s.ad_value(506), A::mul(s.ad_value(516), s.ad_value(62))), A::pow(s.ad_value(156), s.ad_value(407)));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(16, 15, 1.0);
        }

        s.v[1581] = if ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1581] != 0.0)) {
            s.store_div_from_scalar_ad(159, ((-0.0015) * 0.0015), A::scale(s.ad_value(16), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1581] != 0.0))) {
            s.store_scale_ad(159, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(138, A::mul(A::scale(A::div(s.ad_value(499), s.ad_value(159)), 2.0), s.ad_value(108)), A::scale(s.ad_value(411), s.v[34]));
        }

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1562] != 0.0) {
            s.store_sub(87, 200, 144);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(13, A::scale(A::mul(s.ad_value(138), s.ad_value(87)), 2.0), A::mul(s.ad_value(138), s.ad_value(87)));
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt_ad(161, A::offset(s.ad_value(13), 1.0));
        }

        if (s.v[1562] != 0.0) {
            s.store_scaled_offset(162, 161, 1.0, 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(134, A::scale(s.ad_value(411), 2.0), A::div(s.ad_value(499), s.ad_value(159)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale(135, 134, s.v[34]);
        }

        if (s.v[1562] != 0.0) {
            s.store_add(170, 141, 135);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub(167, 75, 139);
        }

        s.v[1582] = if (s.v[542] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1582] != 0.0) {
            s.store_offset_ad(176, A::mul(s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(167), s.ad_value(542)), s.ad_value(170)), 1.0), 1e-38))), 1.0);
        }

        if (!(s.v[1582] != 0.0)) {
            s.store_scalar(176, 1.0);
        }

        s.store_square(207, 176);

        s.store_div_from_scalar(208, 1.0, 176);

        s.store_div_from_scalar(209, 1.0, 207);

        s.store_offset(210, 176, (-1.0));

        s.store_sub(213, 60, 91);

        s.store_sub(216, 200, 144);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_add_ad_rhs(211, 213, A::scale(s.ad_value(200), 2.0));

        s.store_add_ad_rhs(212, 213, A::scale(s.ad_value(144), 2.0));

        s.v[1583] = if ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5))) { 1.0 } else { 0.0 };

        if (s.v[1583] != 0.0) {
            s.store_div_from_scalar_ad(13, ((-0.5) * 0.5), A::scale(s.ad_value(211), 16.0));
        }

        if (!(s.v[1583] != 0.0)) {
            s.store_scale_ad(13, A::add(s.ad_value(211), A::sqrt(A::offset(A::mul(s.ad_value(211), s.ad_value(211)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        s.v[1584] = if ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5))) { 1.0 } else { 0.0 };

        if (s.v[1584] != 0.0) {
            s.store_div_from_scalar_ad(14, ((-0.5) * 0.5), A::scale(s.ad_value(212), 16.0));
        }

        if (!(s.v[1584] != 0.0)) {
            s.store_scale_ad(14, A::add(s.ad_value(212), A::sqrt(A::offset(A::mul(s.ad_value(212), s.ad_value(212)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        s.store_sqrt_ad(214, A::offset(A::mul(s.ad_value(13), s.ad_value(218)), 0.25));

        s.store_sqrt_ad(215, A::offset(A::mul(s.ad_value(14), s.ad_value(218)), 0.25));

        s.store_div_ad_rhs(13, 211, A::offset(A::scale(s.ad_value(214), 2.0), 1.0));

        s.store_div_ad_rhs(14, 212, A::offset(A::scale(s.ad_value(215), 2.0), 1.0));

        s.store_add(15, 214, 215);

        s.store_scale_ad(16, A::div(s.ad_value(217), A::mul(A::square(s.ad_value(15)), s.ad_value(15))), 0.3333333333333333);

        s.store_div_ad(17, A::mul(A::mul(s.ad_value(783), s.ad_value(162)), s.ad_value(208)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)));

        s.store_mul_ad_lhs(18, A::scale(A::add(A::square(s.ad_value(15)), A::mul(s.ad_value(214), s.ad_value(215))), 0.8), 17);

        s.store_add_ad_rhs(19, 18, A::scale(s.ad_value(218), 2.0));

        s.store_mul_ad_lhs(20, A::scale(s.ad_value(217), 0.3333333333333333), 17);

        s.store_div_ad(202, A::mul(s.ad_value(212), A::offset(A::scale(s.ad_value(215), 2.0), (-1.0))), A::offset(A::scale(s.ad_value(215), 2.0), 1.0));

        s.store_add_ad_lhs(201, A::sub(s.ad_value(213), A::mul(A::scale(A::offset(s.ad_value(90), (-1.0)), 2.0), s.ad_value(144))), 202);

        s.store_add_ad(189, A::mul(s.ad_value(208), A::add(A::add(s.ad_value(13), s.ad_value(14)), A::sub(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(s.ad_value(90), A::add(A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(20)))))), A::mul(s.ad_value(210), s.ad_value(201)));

        s.store_add(21, 200, 144);

        s.store_mul_ad_lhs(22, A::mul(s.ad_value(217), s.ad_value(17)), 17);

        s.store_add_ad(194, A::mul(A::mul(s.ad_value(90), s.ad_value(208)), A::add(s.ad_value(21), A::mul(A::scale(s.ad_value(217), 0.3333333333333333), s.ad_value(17)))), A::mul(A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(210)), s.ad_value(144)));

        s.store_mul_ad(191, A::mul(s.ad_value(90), s.ad_value(209)), A::sub(A::scale(s.ad_value(21), 0.5), A::mul(A::scale(s.ad_value(216), 0.16666666666666666), A::sub(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), A::scale(s.ad_value(22), 0.2)))));

        s.store_mul_ad_lhs(192, A::mul(s.ad_value(90), A::sub(s.ad_value(176), s.ad_value(208))), 144);

        s.store_add(193, 191, 192);

        s.store_sub(190, 194, 193);

        s.v[1585] = if ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p.p694))) { 1.0 } else { 0.0 };

        if (s.v[1585] != 0.0) {
            s.store_div_from_scalar_ad(83, ((-p.p694) * p.p694), A::scale(A::mul(s.ad_value(108), s.ad_value(189)), 16.0));
        }

        if (!(s.v[1585] != 0.0)) {
            s.store_scale_ad(83, A::add(A::mul(s.ad_value(108), s.ad_value(189)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(108), s.ad_value(189)), A::mul(s.ad_value(108), s.ad_value(189))), ((0.25 * p.p694) * p.p694)))), 0.5);
        }

        s.store_mul_ad_rhs(82, 108, A::add(s.ad_value(190), s.ad_value(193)));

        s.store_scale_ad(12, A::add(s.ad_value(82), A::scale(s.ad_value(83), p.p208)), 1.0 / (p.p207));

        s.store_offset_ad(13, A::powf(s.ad_value(12), (0.7 * p.p206)), 1.0);

        s.store_div_from_scalar(227, (p.p205 * 1.9e-9), 13);

        s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add(A::scale(s.ad_value(229), (3.9 * 1.0 / (p.p111))), A::scale(s.ad_value(227), 1.0 / (s.v[47]))));

        s.store_mul_ad_lhs(195, A::mul(A::scale(A::div_from_scalar((8.85418e-12 * p.p111), s.ad_value(229)), (((-p.p2) * s.v[33]) * s.v[34])), s.ad_value(108)), 189);

        s.store_mul_ad_lhs(199, A::scale(s.ad_value(228), ((p.p2 * s.v[33]) * s.v[34])), 108);

        s.store_mul_ad_lhs(196, A::neg(s.ad_value(199)), 190);

        s.store_mul_ad_lhs(197, A::neg(s.ad_value(199)), 193);

        s.store_neg_ad(198, A::add(A::add(s.ad_value(195), s.ad_value(196)), s.ad_value(197)));

        s.v[1586] = if !(if self.param_given[666] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1586] != 0.0) {
            s.store_scalar(544, ((((2.0 * p.p111) * 8.85418e-12) / 3.141592653589793) * ((((p.p670 * (1.0 + (4e-7 / p.p77)))).max(1e-38)) as f64).ln()));
        }

        s.store_offset(225, 544, p.p671);

        s.store_offset(226, 544, p.p672);

        s.v[1587] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1587] != 0.0) {
            s.store_mul_ad_lhs(223, A::scale(s.ad_value(225), ((-s.v[33]) * p.p2)), 231);
        }

        if (s.v[1587] != 0.0) {
            s.store_mul_ad_lhs(224, A::scale(s.ad_value(226), ((-s.v[33]) * p.p2)), 232);
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(12, A::offset(A::mul(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02), A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(219, A::sub(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02), s.ad_value(12)), 0.5);
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_div_ad_rhs(18, 219, A::powf(A::offset(A::powf(A::scale(A::neg(s.ad_value(219)), 1.0 / (p.p692)), p.p693), 1.0), (1.0 / p.p693)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(13, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(18), 4.0), s.ad_value(547))));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(223, A::add(A::mul(s.ad_value(225), s.ad_value(231)), A::mul(s.ad_value(545), A::sub(A::sub(A::sub(s.ad_value(231), s.ad_value(63)), s.ad_value(219)), A::mul(A::scale(s.ad_value(547), 0.5), A::offset(s.ad_value(13), (-1.0)))))), ((-s.v[33]) * p.p2));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(12, A::offset(A::mul(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02), A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(220, A::sub(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02), s.ad_value(12)), 0.5);
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_div_ad_rhs(18, 220, A::powf(A::offset(A::powf(A::scale(A::neg(s.ad_value(220)), 1.0 / (p.p690)), p.p691), 1.0), (1.0 / p.p691)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(14, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(18), 4.0), s.ad_value(548))));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(224, A::add(A::mul(s.ad_value(226), s.ad_value(232)), A::mul(s.ad_value(546), A::sub(A::sub(A::sub(s.ad_value(232), s.ad_value(63)), s.ad_value(220)), A::mul(A::scale(s.ad_value(548), 0.5), A::offset(s.ad_value(14), (-1.0)))))), ((-s.v[33]) * p.p2));
        }

        s.store_ad(221, &A::mul(A::scale(A::neg(s.ad_value(187)), (p.p2 * (s.v[34] * p.p673))), A::voltage(ctx, &nodes, Some(10), Some(11))));

        s.v[1588] = if (p.p37 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1588] != 0.0) {
            s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));
        }

        if (s.v[1588] != 0.0) {
            s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt_ad(114, A::div_from_scalar((2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19)));
        }

        if (s.v[1588] != 0.0) {
            let assign31550_ad_e41781: A = {
                if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0), A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(674, 612, assign31550_ad_e41781);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(673, 616, A::offset(A::mul(s.ad_value(623), A::offset(s.ad_value(395), (-1.0))), 1.0));
        }

        s.v[1589] = if ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1589] != 0.0)) {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::scale(A::sub(s.ad_value(127), s.ad_value(61)), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1589] != 0.0))) {
            s.store_scale_ad(110, A::add(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(111, 110);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(112, 114, 111);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_from_scalar(97, s.v[26], 112);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad(113, A::add(A::add(s.ad_value(613), s.ad_value(674)), A::mul(s.ad_value(614), s.ad_value(76))), A::mul(s.ad_value(615), s.ad_value(61)));
        }

        if (s.v[1588] != 0.0) {
            s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);
        }

        s.v[1590] = if ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1590] != 0.0)) {
            s.store_div_from_scalar_ad(104, ((-0.05) * 0.05), A::scale(s.ad_value(13), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1590] != 0.0))) {
            s.store_scale_ad(104, A::add(A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-1.0)), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(106, 104, 108);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_from_scalar(107, 1.0, 106);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(65, 64, 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(73, 70, 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(58, 482, 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(677, A::neg(A::add(s.ad_value(673), A::mul(s.ad_value(617), s.ad_value(61)))), 76);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(124, A::add(A::add(s.ad_value(618), A::scale(s.ad_value(619), 1.0 / (s.v[30]))), A::mul(s.ad_value(620), s.ad_value(61))), A::offset(A::pow(s.ad_value(395), s.ad_value(621)), (-1.0)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(679, 129, A::offset(A::scale(s.ad_value(61), p.p1016), 1.0));
        }

        s.v[1591] = if (s.v[679] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1591] != 0.0)) {
            s.store_div_from_scalar(12, (p.p1015 * s.v[30]), 679);
        }

        s.v[1592] = if (s.v[12] < 40.0) { 1.0 } else { 0.0 };

        if (((s.v[1588] != 0.0) && (s.v[1591] != 0.0)) && (s.v[1592] != 0.0)) {
            s.store_div_from_scalar_ad(676, (0.5 * p.p1014), A::offset(A::cosh(s.ad_value(12)), (-1.0)));
        }

        if (((s.v[1588] != 0.0) && (s.v[1591] != 0.0)) && (!(s.v[1592] != 0.0))) {
            s.store_scale_ad(676, A::limited_exp(A::neg(s.ad_value(12))), p.p1014);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1591] != 0.0))) {
            s.store_scalar(676, 0.0);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(678, 676, A::sub(s.ad_value(675), s.ad_value(127)));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad_lhs(79, A::sub(A::add(A::offset(A::add(A::sub(s.ad_value(677), s.ad_value(124)), s.ad_value(678)), p.p961), s.ad_value(688)), A::mul(A::add(s.ad_value(624), s.ad_value(666)), s.ad_value(61))), 665);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad(59, A::sub(s.ad_value(65), s.ad_value(58)), A::mul(s.ad_value(79), s.ad_value(107)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(680, (p.p958 * (1.0 + (p.p959 * ((s.v[30]) as f64).powf((-p.p960))))));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(687, A::sqrt(A::mul(A::scale(s.ad_value(686), ((2.0 * 1.60219e-19) * s.v[26])), s.ad_value(107))), 1.0 / (s.v[46]));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(687, 687, A::offset(s.ad_value(680), 1.0));
        }

        if (s.v[1588] != 0.0) {
            s.store_div(685, 684, 104);
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(13, 1.0);
        }

        if (s.v[1588] != 0.0) {
            s.store_div(204, 59, 13);
        }

        if (s.v[1588] != 0.0) {
            s.store_div(205, 687, 13);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1593] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1593] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1588] != 0.0) && (s.v[1593] != 0.0)) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(687), A::scale(s.ad_value(96), 2.0)), 1.0), 687);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(685), 2.0)), 73);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1594] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1595] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1596] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (s.v[1596] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1596] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1596] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1596] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_mul_ad_rhs(693, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_rhs(693, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad(681, A::mul(A::scale(s.ad_value(106), 2.0), s.ad_value(693)), A::scale(s.ad_value(106), 2.0));
        }

        if (s.v[1588] != 0.0) {
            s.copy_ad(682, 681);
        }

        if (s.v[1588] != 0.0) {
            s.store_add(682, 682, 70);
        }

        s.v[1597] = if ((0.0 == 0.0) && ((s.v[682] - s.v[70]) < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1597] != 0.0)) {
            s.store_div_from_scalar_ad(683, ((-0.001) * 0.001), A::scale(A::sub(s.ad_value(682), s.ad_value(70)), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1597] != 0.0))) {
            s.store_scale_ad(683, A::add(A::sub(s.ad_value(682), s.ad_value(70)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(682), s.ad_value(70)), A::sub(s.ad_value(682), s.ad_value(70))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_pow_ad(19, A::div(s.ad_value(74), s.ad_value(683)), A::div_from_scalar(1.0, s.ad_value(412)));
        }

        if (s.v[1588] != 0.0) {
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(139, 74, 20);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(142, A::add(s.ad_value(139), s.ad_value(70)), 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(687), A::scale(s.ad_value(96), 2.0)), 1.0), 687);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(685), 2.0)), 142);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1598] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1599] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (s.v[1599] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1600] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (s.v[1600] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) {
            s.store_mul_ad_rhs(692, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_rhs(692, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1601] = if ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1601] != 0.0)) {
            s.store_div_from_scalar_ad(93, ((-2.0) * 2.0), A::scale(s.ad_value(91), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1601] != 0.0))) {
            s.store_scale_ad(93, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(96, 93);
        }

        if (s.v[1588] != 0.0) {
            s.store_offset_ad(92, A::sub(A::sub(s.ad_value(91), s.ad_value(693)), s.ad_value(692)), (-1.0));
        }

        s.v[1602] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1602] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1602] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(14, 12);
        }

        if (s.v[1588] != 0.0) {
            s.store_offset_ad(691, A::div(s.ad_value(687), A::add(s.ad_value(96), s.ad_value(14))), 1.0);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(672, A::mul(A::mul(A::mul(A::scale(A::scale(A::mul(A::scale(s.ad_value(691), (2.0 * p.p2)), s.ad_value(157)), (p.p957 * 1.0 / (s.v[30]))), s.v[46]), s.ad_value(106)), s.ad_value(106)), A::mul(A::sub(s.ad_value(693), s.ad_value(692)), A::add(A::offset(s.ad_value(693), 1.0), s.ad_value(692)))), 175);
        }

        if (s.v[1588] != 0.0) {
            s.store_add(188, 672, 188);
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(696, (p.p785 * p.p1062));
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(697, (p.p799 * p.p1062));
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(698, (p.p800 * p.p1062));
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_from_scalar_ad(694, s.v[30], A::scale(s.ad_value(359), 2.0));
        }

        if (s.v[1588] != 0.0) {
            s.store_square(695, 694);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(367, A::scale(s.ad_value(108), 6.241457005723417e18), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(613)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(366, A::mul(A::mul(A::scale(s.ad_value(691), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(692)), 6.241457005723417e18);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(736, A::mul(A::scale(s.ad_value(108), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(672))), 157);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(737, A::mul(A::scale(s.ad_value(108), 1.60219e-19), s.ad_value(672)), 672);
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad(738, A::add(s.ad_value(696), A::mul(s.ad_value(697), s.ad_value(366))), A::mul(A::mul(s.ad_value(698), s.ad_value(366)), s.ad_value(366)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(740, A::scale(s.ad_value(696), 1.60219e-19), 108);
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(365, A::mul(A::mul(A::scale(s.ad_value(691), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(693)), 6.241457005723417e18);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(13, 696, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(365), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367))), 1e-38)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(14, 697, A::sub(s.ad_value(365), s.ad_value(366)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(15, A::scale(s.ad_value(698), 0.5), A::sub(A::square(s.ad_value(365)), A::square(s.ad_value(366))));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale(16, 695, (10000000000.0 * (p.p957 * p.p2)));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad(368, A::mul(A::div(s.ad_value(736), s.ad_value(12)), A::add(A::add(s.ad_value(13), s.ad_value(14)), s.ad_value(15))), A::div(A::mul(A::mul(A::div(s.ad_value(737), s.ad_value(16)), s.ad_value(363)), s.ad_value(738)), s.ad_value(739)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(17, A::mul(A::scale(s.ad_value(694), ((p.p957 * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(369, A::mul(A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(672)), 672);
        }

        if (s.v[1588] != 0.0) {
            s.store_add(18, 369, 368);
        }

        s.v[1603] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1603] != 0.0)) {
            s.store_div_ad_lhs(19, A::mul(s.ad_value(368), s.ad_value(369)), 18);
        }

        if ((s.v[1588] != 0.0) && (s.v[1603] != 0.0)) {
            s.store_offset_ad(20, A::scale(A::powf(A::sub(s.ad_value(693), s.ad_value(692)), p.p1064), p.p1063), 1.0);
        }

        s.v[1604] = if (s.v[57] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(785, A::scale(s.ad_value(187), p.p29), 196);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(786, A::scale(s.ad_value(187), p.p29), 197);
        }

        if (!(s.v[1604] != 0.0)) {
            s.store_mul_ad_lhs(785, A::scale(s.ad_value(187), p.p29), 197);
        }

        if (!(s.v[1604] != 0.0)) {
            s.store_mul_ad_lhs(786, A::scale(s.ad_value(187), p.p29), 196);
        }

        s.v[1605] = if ((p.p1094 == 1.0) && (p.p1095 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1605] != 0.0) {
            s.store_add(221, 221, 774);
        }

        if (s.v[1605] != 0.0) {
            s.store_add(224, 224, 775);
        }

        s.v[1606] = if (p.p1096 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1605] != 0.0) && (s.v[1606] != 0.0)) {
            s.store_add(221, 221, 776);
        }

        if ((s.v[1605] != 0.0) && (s.v[1606] != 0.0)) {
            s.store_add(223, 223, 777);
        }

        s.store_mul_ad_lhs(787, A::scale(s.ad_value(187), p.p29), 198);

        s.v[1612] = if ((p.p42 != 2.0) && (s.v[240] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1612] != 0.0) {
            s.store_div_from_scalar(372, 1.0, 242);
        }

        s.v[1613] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1612] != 0.0) && (s.v[1613] != 0.0)) {
            s.store_div_from_scalar(374, 1.0, 759);
        }

        s.v[1614] = if ((p.p42 != 2.0) && (s.v[239] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1614] != 0.0) {
            s.store_div_from_scalar(371, 1.0, 241);
        }

        s.v[1615] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1614] != 0.0) && (s.v[1615] != 0.0)) {
            s.store_div_from_scalar(373, 1.0, 761);
        }

        s.v[1621] = if ((p.p49 != 0.0) && (p.p909 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1621] != 0.0) {
            s.store_ad(749, &A::mul(A::mul(A::mul(s.ad_value(187), s.ad_value(57)), s.ad_value(188)), A::voltage(ctx, &nodes, Some(5), Some(7))));
        }

        s.v[1622] = if ((p.p42 != 2.0) && (s.v[240] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1623] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1621] != 0.0) && (s.v[1622] != 0.0)) && (s.v[1623] != 0.0)) {
            s.store_add_ad(749, A::add(s.ad_value(749), A::mul(A::square(A::voltage(ctx, &nodes, Some(0), Some(6))), s.ad_value(372))), A::mul(A::square(A::voltage(ctx, &nodes, Some(6), Some(5))), s.ad_value(374)));
        }

        if (((s.v[1621] != 0.0) && (s.v[1622] != 0.0)) && (!(s.v[1623] != 0.0))) {
            s.store_add_ad_rhs(749, 749, A::mul(A::square(A::voltage(ctx, &nodes, Some(0), Some(6))), s.ad_value(372)));
        }

        s.v[1624] = if ((p.p42 != 2.0) && (s.v[239] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1625] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1621] != 0.0) && (s.v[1624] != 0.0)) && (s.v[1625] != 0.0)) {
            s.store_add_ad(749, A::add(s.ad_value(749), A::mul(A::square(A::voltage(ctx, &nodes, Some(2), Some(8))), s.ad_value(371))), A::mul(A::square(A::voltage(ctx, &nodes, Some(8), Some(7))), s.ad_value(373)));
        }

        if (((s.v[1621] != 0.0) && (s.v[1624] != 0.0)) && (!(s.v[1625] != 0.0))) {
            s.store_add_ad_rhs(749, 749, A::mul(A::square(A::voltage(ctx, &nodes, Some(2), Some(8))), s.ad_value(371)));
        }

        s.v[1627] = if (p.p8 != 0.0) { 1.0 } else { 0.0 };

        s.v[1628] = if (p.p1097 == 0.0) { 1.0 } else { 0.0 };

        s.v[1630] = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq0_e1199,) = {
    if ((s.v[896] != 0.0) && (s.v[897] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1199;
        stamper.stamp_potential(
            branches[0],
            eq0_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e1207, eq1_e1207_d_n0, eq1_e1207_d_n1, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14, eq1_e1207_d_n15, eq1_e1207_d_n16,) = {
    if (s.v[1538] != 0.0) {
        let eq1_e1203: f64 = (s.v[187] * p.p28);
        let eq1_e1203_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq1_e1203_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq1_e1203_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq1_e1203_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq1_e1203_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq1_e1203_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq1_e1203_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq1_e1203_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq1_e1203_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq1_e1203_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq1_e1203_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq1_e1203_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq1_e1203_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq1_e1203_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq1_e1203_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq1_e1203_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq1_e1203_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq1_e1205: f64 = (eq1_e1203 * s.v[706]);
        let eq1_e1205_d_n0: f64 = ((eq1_e1203_d_n0 * s.v[706]) + (eq1_e1203 * s.dn[706][0]));
        let eq1_e1205_d_n1: f64 = ((eq1_e1203_d_n1 * s.v[706]) + (eq1_e1203 * s.dn[706][1]));
        let eq1_e1205_d_n2: f64 = ((eq1_e1203_d_n2 * s.v[706]) + (eq1_e1203 * s.dn[706][2]));
        let eq1_e1205_d_n3: f64 = ((eq1_e1203_d_n3 * s.v[706]) + (eq1_e1203 * s.dn[706][3]));
        let eq1_e1205_d_n4: f64 = ((eq1_e1203_d_n4 * s.v[706]) + (eq1_e1203 * s.dn[706][4]));
        let eq1_e1205_d_n5: f64 = ((eq1_e1203_d_n5 * s.v[706]) + (eq1_e1203 * s.dn[706][5]));
        let eq1_e1205_d_n6: f64 = ((eq1_e1203_d_n6 * s.v[706]) + (eq1_e1203 * s.dn[706][6]));
        let eq1_e1205_d_n7: f64 = ((eq1_e1203_d_n7 * s.v[706]) + (eq1_e1203 * s.dn[706][7]));
        let eq1_e1205_d_n8: f64 = ((eq1_e1203_d_n8 * s.v[706]) + (eq1_e1203 * s.dn[706][8]));
        let eq1_e1205_d_n9: f64 = ((eq1_e1203_d_n9 * s.v[706]) + (eq1_e1203 * s.dn[706][9]));
        let eq1_e1205_d_n10: f64 = ((eq1_e1203_d_n10 * s.v[706]) + (eq1_e1203 * s.dn[706][10]));
        let eq1_e1205_d_n11: f64 = ((eq1_e1203_d_n11 * s.v[706]) + (eq1_e1203 * s.dn[706][11]));
        let eq1_e1205_d_n12: f64 = ((eq1_e1203_d_n12 * s.v[706]) + (eq1_e1203 * s.dn[706][12]));
        let eq1_e1205_d_n13: f64 = ((eq1_e1203_d_n13 * s.v[706]) + (eq1_e1203 * s.dn[706][13]));
        let eq1_e1205_d_n14: f64 = ((eq1_e1203_d_n14 * s.v[706]) + (eq1_e1203 * s.dn[706][14]));
        let eq1_e1205_d_n15: f64 = ((eq1_e1203_d_n15 * s.v[706]) + (eq1_e1203 * s.dn[706][15]));
        let eq1_e1205_d_n16: f64 = ((eq1_e1203_d_n16 * s.v[706]) + (eq1_e1203 * s.dn[706][16]));
        (eq1_e1205, eq1_e1205_d_n0, eq1_e1205_d_n1, eq1_e1205_d_n2, eq1_e1205_d_n3, eq1_e1205_d_n4, eq1_e1205_d_n5, eq1_e1205_d_n6, eq1_e1205_d_n7, eq1_e1205_d_n8, eq1_e1205_d_n9, eq1_e1205_d_n10, eq1_e1205_d_n11, eq1_e1205_d_n12, eq1_e1205_d_n13, eq1_e1205_d_n14, eq1_e1205_d_n15, eq1_e1205_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1207;
        let eq1_node_derivatives: [f64; 17] = [eq1_e1207_d_n0, eq1_e1207_d_n1, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14, eq1_e1207_d_n15, eq1_e1207_d_n16];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq1_value),
            &nodes,
            &eq1_node_derivatives,
            &branches,
            &eq1_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq2_e1218,) = {
    if (s.v[1541] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1218;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq2_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e1230,) = {
    if (!(s.v[1541] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1230;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq3_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e1238,) = {
    if (s.v[1555] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1238;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq4_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e1247,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1247;
        stamper.stamp_current(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq5_value),
            &[
            ],
        );
    }
}
