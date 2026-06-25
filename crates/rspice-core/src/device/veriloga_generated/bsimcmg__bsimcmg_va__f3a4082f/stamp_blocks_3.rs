#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        if ((s.v[1445] != 0.0) && (s.v[1448] != 0.0)) {
            s.store_ad(341, &{
                if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp(A::neg(s.ad_value(177))), (-1.0))
                } else {
                    {
                        if ((((-s.v[177])) as f64).abs() < 1e-7) {
                            A::sub(A::mul(A::scale(A::neg(s.ad_value(177)), 0.5), A::neg(s.ad_value(177))), s.ad_value(177))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1445] != 0.0) && (s.v[1448] != 0.0)) {
            s.store_sqrt_ad(342, A::add(s.ad_value(341), s.ad_value(177)));
        }

        if ((s.v[1445] != 0.0) && (s.v[1448] != 0.0)) {
            s.store_mul_ad_lhs(401, A::mul(A::neg(s.ad_value(178)), s.ad_value(342)), 179);
        }

        s.v[1449] = if (s.v[340] < (-1e-15)) { 1.0 } else { 0.0 };

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (s.v[1449] != 0.0)) {
            s.store_sub_ad(344, A::neg(A::sub(s.ad_value(170), s.ad_value(340))), A::mul(s.ad_value(178), s.ad_value(176)));
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (s.v[1449] != 0.0)) {
            s.store_offset_ad(345, A::div(A::mul(A::scale(s.ad_value(178), 0.5), s.ad_value(175)), s.ad_value(176)), 1.0);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (s.v[1449] != 0.0)) {
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (s.v[1449] != 0.0)) {
            let assign23660_ad_e43068: A = A::add({
                if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp(A::neg(s.ad_value(177))), (-1.0))
                } else {
                    {
                        if ((((-s.v[177])) as f64).abs() < 1e-7) {
                            A::sub(A::mul(A::scale(A::neg(s.ad_value(177)), 0.5), A::neg(s.ad_value(177))), s.ad_value(177))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(177));
            s.store_ad(343, &assign23660_ad_e43068);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (s.v[1449] != 0.0)) {
            s.store_mul_ad_rhs(342, 178, A::sqrt(s.ad_value(343)));
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (!(s.v[1449] != 0.0))) {
            s.store_scalar(177, 0.0);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) && (!(s.v[1449] != 0.0))) {
            s.store_scalar(342, 0.0);
        }

        if ((s.v[1445] != 0.0) && (!(s.v[1448] != 0.0))) {
            s.store_mul(401, 342, 179);
        }

        if (s.v[1445] != 0.0) {
            s.store_mul_ad_lhs(904, A::mul(s.ad_value(178), A::limited_exp(A::scale(A::neg(s.ad_value(177)), 0.5))), 179);
        }

        if (s.v[1445] != 0.0) {
            s.store_scale_ad(921, A::add(A::offset(s.ad_value(177), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(177), (-1.0)), A::offset(s.ad_value(177), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1445] != 0.0) {
            s.store_sqrt(922, 921);
        }

        if (s.v[1445] != 0.0) {
            s.store_offset_ad(923, A::div(s.ad_value(178), s.ad_value(922)), 1.0);
        }

        s.store_scaled_add(399, 392, 393, 0.5);

        s.store_sub(402, 392, 393);

        s.store_scale_ad(168, A::square(s.ad_value(390)), 1600.0);

        s.v[1450] = if (p.p603 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1450] != 0.0) {
            s.store_add_ad(400, A::scale(A::add(s.ad_value(392), s.ad_value(393)), 0.5), A::mul(A::scale(A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(168)))), (p.p603 * 0.5)), s.ad_value(402)));
        }

        if (!(s.v[1450] != 0.0)) {
            s.store_scaled_add(400, 392, 393, 0.5);
        }

        s.v[1451] = if (s.v[655] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1451] != 0.0) {
            s.store_scale(172, 399, 1.0 / (p.p400));
        }

        if (s.v[1451] != 0.0) {
            s.store_offset_ad(174, A::pow(s.ad_value(172), s.ad_value(661)), 1.0);
        }

        if (s.v[1451] != 0.0) {
            s.store_div(374, 373, 174);
        }

        if (s.v[1451] != 0.0) {
            s.store_div_from_scalar_ad(372, 1.0, A::add(A::div_from_scalar(1.0, A::scale(s.ad_value(163), (p.p89 * 1.0 / (p.p90)))), A::scale(A::mul(s.ad_value(374), s.ad_value(655)), 1.0 / (s.v[143]))));
        }

        if (!(s.v[1451] != 0.0)) {
            s.copy_ad(372, 163);
        }

        s.v[1452] = if ((p.p61 != 0.0) && (s.v[656] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1452] != 0.0) {
            s.store_offset_ad(175, A::powf(A::scale(s.ad_value(904), 1.0 / (p.p401)), p.p402), 1.0);
        }

        if (s.v[1452] != 0.0) {
            s.store_div(374, 373, 175);
        }

        if (s.v[1452] != 0.0) {
            s.store_div_from_scalar_ad(494, 1.0, A::add(A::div_from_scalar(1.0, s.ad_value(494)), A::scale(A::mul(s.ad_value(374), s.ad_value(656)), 1.0 / (s.v[143]))));
        }

        s.store_div_ad_lhs(183, A::mul(A::mul(s.ad_value(416), s.ad_value(163)), s.ad_value(158)), 153);

        s.store_scale_ad(409, A::add(s.ad_value(396), A::mul(s.ad_value(407), s.ad_value(400))), s.v[420]);

        s.v[1453] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1453] != 0.0) {
            s.store_ad(170, &A::pow(A::scale(A::offset(A::div(s.ad_value(400), s.ad_value(406)), 1.0), 0.5), s.ad_value(317)));
        }

        if (!(s.v[1453] != 0.0)) {
            s.store_scale_ad(168, A::square(s.ad_value(390)), 1600.0);
        }

        if (!(s.v[1453] != 0.0)) {
            s.store_sub_from_scalar_ad(169, 1.0, A::limited_exp(A::neg(s.ad_value(168))));
        }

        if (!(s.v[1453] != 0.0)) {
            s.store_mul_ad_lhs(168, A::add(A::mul(s.ad_value(330), s.ad_value(392)), A::mul(s.ad_value(331), s.ad_value(393))), 169);
        }

        if (!(s.v[1453] != 0.0)) {
            s.store_ad(169, &{
                if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                    A::scale(A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-12) * 1e-12)))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-12)) {
                            A::div_from_scalar(((-1e-12) * 1e-12), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (!(s.v[1453] != 0.0)) {
            s.store_ad(170, &A::pow(A::scale(A::offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0), 0.5), s.ad_value(317)));
        }

        s.store_ad(168, &A::pow(s.ad_value(409), s.ad_value(822)));

        s.v[1454] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1454] != 0.0) {
            s.store_add_ad(171, A::mul(A::add(s.ad_value(819), A::mul(s.ad_value(821), s.ad_value(370))), s.ad_value(168)), A::div(s.ad_value(820), s.ad_value(170)));
        }

        if (!(s.v[1454] != 0.0)) {
            s.store_add_ad(171, A::mul(s.ad_value(819), s.ad_value(168)), A::div(s.ad_value(820), s.ad_value(170)));
        }

        s.store_offset(411, 171, 1.0);

        s.store_scale_ad(411, A::add(A::offset(s.ad_value(411), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(411), (-1.0)), A::offset(s.ad_value(411), (-1.0))), ((0.25 * p.p604) * p.p604)))), 0.5);

        s.store_scale_ad(215, A::sub_from_scalar(1.0, A::scale(A::limited_exp(A::scale(s.ad_value(390), (-p.p888))), p.p887)), p.p24);

        s.store_div(411, 411, 215);

        s.store_div(415, 416, 411);

        s.v[1455] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        s.v[1456] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1455] != 0.0) && (s.v[1456] != 0.0)) {
            s.store_ad(170, &A::pow(A::scale(A::offset(A::div(s.ad_value(404), s.ad_value(406)), 1.0), 0.5), s.ad_value(317)));
        }

        if ((s.v[1455] != 0.0) && (!(s.v[1456] != 0.0))) {
            s.store_add_ad(168, A::mul(s.ad_value(330), s.ad_value(394)), A::mul(s.ad_value(331), s.ad_value(395)));
        }

        if ((s.v[1455] != 0.0) && (!(s.v[1456] != 0.0))) {
            s.store_ad(169, &{
                if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                    A::scale(A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-12) * 1e-12)))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-12)) {
                            A::div_from_scalar(((-1e-12) * 1e-12), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1455] != 0.0) && (!(s.v[1456] != 0.0))) {
            s.store_ad(170, &A::pow(A::scale(A::offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0), 0.5), s.ad_value(317)));
        }

        if (s.v[1455] != 0.0) {
            s.store_scale_ad(410, A::add(s.ad_value(396), A::mul(s.ad_value(408), s.ad_value(404))), s.v[420]);
        }

        if (s.v[1455] != 0.0) {
            s.store_add_ad(171, A::mul(s.ad_value(304), A::pow(s.ad_value(410), s.ad_value(822))), A::div(s.ad_value(319), s.ad_value(170)));
        }

        if (!(s.v[1455] != 0.0)) {
            s.store_scale_ad(410, A::add(s.ad_value(396), A::mul(s.ad_value(408), s.ad_value(400))), s.v[420]);
        }

        if (!(s.v[1455] != 0.0)) {
            s.store_add_ad(171, A::mul(s.ad_value(819), A::pow(s.ad_value(410), s.ad_value(822))), A::div(s.ad_value(820), s.ad_value(170)));
        }

        s.store_offset(412, 171, 1.0);

        s.store_scale_ad(412, A::add(A::offset(s.ad_value(412), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(412), (-1.0)), A::offset(s.ad_value(412), (-1.0))), ((0.25 * p.p604) * p.p604)))), 0.5);

        s.store_div(412, 412, 215);

        s.store_offset_ad(360, A::div(A::mul(s.ad_value(719), s.ad_value(153)), s.ad_value(351)), 1e-6);

        s.v[1457] = if (s.v[360] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[1457] != 0.0) {
            s.store_add_ad_lhs(200, A::div(A::scale(s.ad_value(427), 0.5), A::offset(A::cosh(s.ad_value(360)), (-1.0))), 718);
        }

        if (!(s.v[1457] != 0.0)) {
            s.store_add_ad_lhs(200, A::mul(s.ad_value(427), A::limited_exp(A::neg(s.ad_value(360)))), 718);
        }

        s.v[1458] = if (s.v[720] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1458] != 0.0) {
            s.store_offset_ad(201, A::div(A::mul(s.ad_value(720), s.ad_value(399)), s.ad_value(217)), 1.0);
        }

        if (!(s.v[1458] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(720), s.ad_value(399)), s.ad_value(217))));
        }

        s.store_sub(202, 126, 390);

        s.v[1459] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1459] != 0.0) {
            s.store_add_ad_rhs(204, 399, A::scale(s.ad_value(179), 2.0));
        }

        if (!(s.v[1459] != 0.0)) {
            s.store_add_ad_rhs(204, 399, A::scale(s.ad_value(182), 2.0));
        }

        s.v[1460] = if (s.v[200] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1460] != 0.0) {
            s.copy_ad(169, 204);
        }

        if (s.v[1460] != 0.0) {
            s.store_div_ad_rhs(171, 169, A::add(s.ad_value(210), s.ad_value(169)));
        }

        if (s.v[1460] != 0.0) {
            s.store_mul_ad_lhs(203, A::mul(A::div(s.ad_value(169), s.ad_value(200)), s.ad_value(171)), 201);
        }

        if (s.v[1460] != 0.0) {
            s.store_offset_ad(205, A::div(s.ad_value(202), s.ad_value(203)), 1.0);
        }

        if (!(s.v[1460] != 0.0)) {
            s.store_scalar(205, 1.0);
        }

        s.v[1461] = if (s.v[795] > 0.0) { 1.0 } else { 0.0 };

        s.v[1462] = if (s.v[793] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1461] != 0.0) && (s.v[1462] != 0.0)) {
            s.store_div_from_scalar_ad(169, 1.0, A::sub(A::div_from_scalar(1.0, s.ad_value(795)), A::mul(s.ad_value(793), s.ad_value(399))));
        }

        if ((s.v[1461] != 0.0) && (!(s.v[1462] != 0.0))) {
            s.store_add_ad_rhs(169, 795, A::mul(s.ad_value(793), s.ad_value(399)));
        }

        if (s.v[1461] != 0.0) {
            let assign24430_ad_e43832: A = {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38) {
                            A::ln(A::offset(A::div(A::div(A::sub(s.ad_value(126), s.ad_value(390)), s.ad_value(169)), A::add(s.ad_value(210), s.ad_value(217))), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(206, A::mul(s.ad_value(169), assign24430_ad_e43832), 1.0);
        }

        if (!(s.v[1461] != 0.0)) {
            s.store_scalar(206, 1.0);
        }

        s.store_mul(205, 205, 206);

        s.store_div_ad_lhs(218, A::scale(s.ad_value(422), 2.0), 415);

        s.store_mul(219, 218, 153);

        s.store_limited_exp_ad(168, A::mul(s.ad_value(695), {
            if (!((s.v[402] / s.v[219]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((s.v[402] / s.v[219]) > 1e-38) {
                        A::ln(A::div(s.ad_value(402), s.ad_value(219)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }));

        s.store_div_from_scalar(169, 1.0, 695);

        s.store_offset_ad(225, A::limited_exp(A::mul(s.ad_value(169), {
            if (!(s.v[694] > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if (s.v[694] > 1e-38) {
                        A::ln(s.ad_value(694))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        })), 1.0);

        let assign24510_ad_e43924: A = A::div(A::offset(A::limited_exp(A::mul(s.ad_value(169), {
    if (!((s.v[694] + s.v[168]) > 1e-38)) {
        A::neg(A::constant(87.498233534))
    } else {
        {
            if ((s.v[694] + s.v[168]) > 1e-38) {
                A::ln(A::add(s.ad_value(694), s.ad_value(168)))
            } else {
                A::constant(0.0)
            }
        }
    }
})), 1.0), s.ad_value(225));
        s.store_ad(209, &assign24510_ad_e43924);

        s.store_add_ad_rhs(209, 209, A::mul(A::mul(A::mul(A::scale(s.ad_value(424), 0.5), s.ad_value(399)), s.ad_value(402)), s.ad_value(402)));

        s.store_add_ad_rhs(168, 241, A::div(s.ad_value(242), A::add(s.ad_value(399), A::scale(s.ad_value(181), 2.0))));

        s.store_mul_ad_lhs(169, A::mul(s.ad_value(168), s.ad_value(402)), 402);

        s.store_offset(170, 169, ((1.0) + ((-0.001))));

        s.store_offset_ad(171, A::scale(A::add(s.ad_value(170), A::sqrt(A::offset(A::square(s.ad_value(170)), 0.004))), 0.5), (-1.0));

        s.store_scale_ad(214, A::offset(A::sqrt(A::offset(s.ad_value(171), 1.0)), 1.0), 0.5);

        s.store_mul(209, 209, 214);

        s.store_scale_ad(209, A::add(A::offset(s.ad_value(209), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(209), (-1.0)), A::offset(s.ad_value(209), (-1.0))), ((0.25 * p.p453) * p.p453)))), 0.5);

        s.store_div_ad_rhs(169, 236, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(237), A::mul(A::mul(s.ad_value(294), s.ad_value(402)), s.ad_value(402)))), s.ad_value(399)), A::scale(s.ad_value(181), 2.0)));

        s.store_limited_exp_ad(366, A::neg(s.ad_value(169)));

        s.v[1463] = if (p.p61 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1463] != 0.0) {
            let assign24630_ad_e44082: A = {
                if (!((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::add(s.ad_value(293), A::mul(s.ad_value(240), s.ad_value(127))), A::sqrt(A::offset(A::mul(A::add(s.ad_value(293), A::mul(s.ad_value(240), s.ad_value(127))), A::add(s.ad_value(293), A::mul(s.ad_value(240), s.ad_value(127)))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add(s.ad_value(293), A::mul(s.ad_value(240), s.ad_value(127))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(168, &assign24630_ad_e44082);
        }

        if (s.v[1463] != 0.0) {
            s.store_div_ad_rhs(169, 168, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(238), A::mul(A::mul(s.ad_value(295), s.ad_value(402)), s.ad_value(402)))), s.ad_value(399)), A::scale(s.ad_value(181), 2.0)));
        }

        if (s.v[1463] != 0.0) {
            s.store_sub_ad(171, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
        }

        if (s.v[1463] != 0.0) {
            s.store_limited_exp_ad(371, A::mul(A::neg(s.ad_value(169)), s.ad_value(171)));
        }

        if (!(s.v[1463] != 0.0)) {
            s.store_scalar(371, 1.0);
        }

        s.v[1464] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1464] != 0.0) {
            s.store_div_ad_lhs(220, A::mul(A::scale(s.ad_value(336), 2.0), s.ad_value(412)), 414);
        }

        if (!(s.v[1464] != 0.0)) {
            s.store_div_ad_lhs(220, A::mul(A::scale(s.ad_value(336), 2.0), s.ad_value(412)), 416);
        }

        s.store_mul(221, 220, 156);

        s.v[1465] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1465] != 0.0) {
            s.store_ad(168, &A::pow(A::div(s.ad_value(405), s.ad_value(221)), s.ad_value(697)));
        }

        if (!(s.v[1465] != 0.0)) {
            s.store_ad(168, &A::pow(A::div(s.ad_value(402), s.ad_value(221)), s.ad_value(697)));
        }

        s.store_div_from_scalar(169, 1.0, 697);

        s.store_offset_ad(225, A::pow(s.ad_value(696), s.ad_value(169)), 1.0);

        s.store_div_ad_lhs(213, A::offset(A::pow(A::add(s.ad_value(696), s.ad_value(168)), s.ad_value(169)), 1.0), 225);

        s.store_scale_ad(881, A::add(A::offset(s.ad_value(881), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(881), (-0.1)), A::offset(s.ad_value(881), (-0.1))), ((0.25 * 0.001) * 0.001)))), 0.5);

        s.store_mul(213, 213, 881);

        s.v[1466] = if (s.v[794] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1466] != 0.0) {
            let assign24810_ad_e44264: A = {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38) {
                            A::ln(A::offset(A::div(A::div(A::sub(s.ad_value(126), s.ad_value(390)), s.ad_value(794)), A::add(s.ad_value(210), s.ad_value(221))), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(207, A::mul(s.ad_value(794), assign24810_ad_e44264), 1.0);
        }

        if (!(s.v[1466] != 0.0)) {
            s.store_scalar(207, 1.0);
        }

        s.store_mul_ad_lhs(140, A::mul(A::scale(s.ad_value(640), (-1.60219e-19)), s.ad_value(894)), 156);

        s.store_div_ad_rhs(131, 339, A::add(s.ad_value(339), s.ad_value(399)));

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_add_ad_rhs(123, 399, A::mul(A::sub_from_scalar(2.0, s.ad_value(131)), s.ad_value(181)));

        s.store_mul(122, 123, 402);

        s.v[1467] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        s.v[1468] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        s.v[1469] = if (p.p64 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1467] != 0.0) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(399)), 1.0);
        }

        if (s.v[1467] != 0.0) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if (s.v[1467] != 0.0) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if (s.v[1467] != 0.0) {
            s.store_mul_ad_lhs(197, A::mul(s.ad_value(194), A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908)), 189);
        }

        if (s.v[1467] != 0.0) {
            s.store_offset_ad(188, A::mul(A::div(A::mul(A::scale(s.ad_value(183), s.v[115]), s.ad_value(123)), A::mul(s.ad_value(411), s.ad_value(209))), s.ad_value(197)), 1.0);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_scalar(197, 0.0);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_scalar(188, 1.0);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_sub_ad_lhs(170, A::mul(s.ad_value(114), A::voltage(ctx, &nodes, Some(11), Some(8))), 479);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_sqrt_ad(171, A::offset(A::square(s.ad_value(170)), 0.1));
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_scaled_add(482, 170, 171, 0.5);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(482)), 1.0);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_mul_ad_rhs(174, 853, A::offset(A::mul(s.ad_value(425), A::powf(A::offset(A::square(A::voltage(ctx, &nodes, Some(2), Some(8))), 1e-6), (0.5 * p.p921))), 1.0));
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_sub_ad_lhs(170, A::mul(s.ad_value(114), A::voltage(ctx, &nodes, Some(11), Some(9))), 479);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_sqrt_ad(171, A::offset(A::square(s.ad_value(170)), 0.1));
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_scaled_add(483, 170, 171, 0.5);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_offset_ad(172, A::mul(s.ad_value(712), s.ad_value(483)), 1.0);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_mul_ad_rhs(174, 852, A::offset(A::mul(s.ad_value(426), A::powf(A::offset(A::square(A::voltage(ctx, &nodes, Some(0), Some(9))), 1e-6), (0.5 * p.p922))), 1.0));
        }

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(399)), 1.0);
        }

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_mul_ad_rhs(197, 194, A::add(A::add(A::mul(A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189)), s.ad_value(190)), s.ad_value(191)));
        }

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_offset_ad(188, A::mul(A::div(A::mul(A::scale(s.ad_value(183), s.v[115]), s.ad_value(123)), A::mul(s.ad_value(411), s.ad_value(209))), s.ad_value(197)), 1.0);
        }

        s.store_div_ad(124, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(183), s.v[115]), s.ad_value(122)), s.ad_value(205)), s.ad_value(366)), s.ad_value(371)), A::mul(A::mul(s.ad_value(411), s.ad_value(209)), s.ad_value(188)));

        s.store_scale(124, 124, p.p25);

        s.v[1470] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1470] != 0.0) {
            s.store_div_ad_lhs(341, A::add(A::scale(s.ad_value(403), 2.0), s.ad_value(181)), 213);
        }

        if (s.v[1470] != 0.0) {
            s.store_add_ad_rhs(138, 403, A::div(A::square(s.ad_value(405)), A::scale(s.ad_value(341), 6.0)));
        }

        if (s.v[1470] != 0.0) {
            s.store_scale_ad(137, A::sub(s.ad_value(403), A::mul(A::scale(s.ad_value(405), 0.16666666666666666), A::sub_from_scalar(1.0, A::mul(A::div(s.ad_value(405), s.ad_value(341)), A::offset(A::div(s.ad_value(405), A::scale(s.ad_value(341), 5.0)), 1.0))))), (-0.5));
        }

        if (!(s.v[1470] != 0.0)) {
            s.store_div_ad_lhs(341, A::add(A::scale(s.ad_value(399), 2.0), s.ad_value(181)), 213);
        }

        if (!(s.v[1470] != 0.0)) {
            s.store_add_ad_rhs(138, 399, A::div(A::square(s.ad_value(402)), A::scale(s.ad_value(341), 6.0)));
        }

        if (!(s.v[1470] != 0.0)) {
            s.store_scale_ad(137, A::sub(s.ad_value(399), A::mul(A::scale(s.ad_value(402), 0.16666666666666666), A::sub_from_scalar(1.0, A::mul(A::div(s.ad_value(402), s.ad_value(341)), A::offset(A::div(s.ad_value(402), A::scale(s.ad_value(341), 5.0)), 1.0))))), (-0.5));
        }

        s.store_div_from_scalar(208, 1.0, 207);

        s.store_add_ad(138, A::mul(s.ad_value(208), s.ad_value(138)), A::mul(A::offset(s.ad_value(207), (-1.0)), s.ad_value(393)));

        s.store_add_ad(137, A::mul(A::square(s.ad_value(208)), s.ad_value(137)), A::mul(A::scale(A::sub(s.ad_value(207), s.ad_value(208)), 0.5), s.ad_value(393)));

        s.store_sub_ad_lhs(139, A::neg(s.ad_value(138)), 137);

        s.store_mul_ad_lhs(175, A::mul(A::scale(s.ad_value(159), s.v[115]), s.ad_value(156)), 372);

        s.store_mul(138, 175, 138);

        s.store_mul(137, 175, 137);

        s.store_mul(139, 175, 139);

        s.copy_ad(592, 138);

        s.v[1472] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1473] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if ((s.v[1472] != 0.0) && (s.v[1473] != 0.0)) {
            s.store_mul_ad_lhs(169, A::mul(A::scale(s.ad_value(160), s.v[115]), s.ad_value(157)), 494);
        }

        if ((s.v[1472] != 0.0) && (!(s.v[1473] != 0.0))) {
            s.store_mul_ad_lhs(169, A::mul(A::scale(s.ad_value(159), s.v[115]), s.ad_value(157)), 494);
        }

        if (s.v[1472] != 0.0) {
            s.copy_ad(176, 904);
        }

        if (s.v[1472] != 0.0) {
            s.store_mul(340, 176, 169);
        }

        if (s.v[1472] != 0.0) {
            s.store_neg(495, 340);
        }

        if (s.v[1472] != 0.0) {
            s.copy_ad(496, 340);
        }

        if (s.v[1472] != 0.0) {
            s.store_mul_ad_lhs(169, A::mul(A::scale(s.ad_value(159), s.v[115]), s.ad_value(156)), 163);
        }

        if (s.v[1472] != 0.0) {
            s.store_sub(170, 401, 904);
        }

        if (s.v[1472] != 0.0) {
            s.store_mul(340, 169, 170);
        }

        if (s.v[1472] != 0.0) {
            s.store_sub(495, 495, 340);
        }

        if (s.v[1472] != 0.0) {
            s.store_add(496, 496, 340);
        }

        if (s.v[1472] != 0.0) {
            s.store_mul_ad_lhs(169, A::mul(A::scale(s.ad_value(159), s.v[115]), s.ad_value(156)), 163);
        }

        if (s.v[1472] != 0.0) {
            s.store_mul_ad(170, A::scale(A::offset(s.ad_value(923), (-1.0)), 0.5), A::add(s.ad_value(399), A::div(A::square(s.ad_value(402)), A::scale(s.ad_value(341), 6.0))));
        }

        if (s.v[1472] != 0.0) {
            s.store_mul(340, 169, 170);
        }

        if (s.v[1472] != 0.0) {
            s.store_sub(495, 495, 340);
        }

        if (s.v[1472] != 0.0) {
            s.store_add(496, 496, 340);
        }

        s.v[1474] = if (s.v[128] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1474] != 0.0) {
            s.copy_ad(169, 137);
        }

        if (s.v[1474] != 0.0) {
            s.copy_ad(137, 139);
        }

        if (s.v[1474] != 0.0) {
            s.copy_ad(139, 169);
        }

        s.v[1475] = if (p.p78 != 1.0) { 1.0 } else { 0.0 };

        s.v[1476] = if (p.p76 != 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_mul_ad_lhs(169, A::scale(s.ad_value(159), s.v[115]), 114);
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_mul_ad_rhs(170, 114, A::voltage(ctx, &nodes, Some(10), Some(6)));
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_offset_ad(168, A::sub(s.ad_value(170), s.ad_value(518)), 0.02);
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_scale_ad(510, A::sub(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02)))), 0.5);
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_mul_ad_rhs(498, 169, A::add(A::mul(s.ad_value(648), A::sub(A::sub(A::sub(s.ad_value(170), s.ad_value(518)), s.ad_value(510)), A::mul(A::scale(s.ad_value(651), 0.5), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(510), 4.0), s.ad_value(651)))), (-1.0))))), A::mul(s.ad_value(646), s.ad_value(170))));
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_mul_ad_rhs(170, 114, A::voltage(ctx, &nodes, Some(10), Some(5)));
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_offset_ad(168, A::sub(s.ad_value(170), s.ad_value(518)), 0.02);
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_scale_ad(511, A::sub(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02)))), 0.5);
        }

        if ((s.v[1475] != 0.0) && (s.v[1476] != 0.0)) {
            s.store_mul_ad_rhs(499, 169, A::add(A::mul(s.ad_value(649), A::sub(A::sub(A::sub(s.ad_value(170), s.ad_value(518)), s.ad_value(511)), A::mul(A::scale(s.ad_value(652), 0.5), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(511), 4.0), s.ad_value(652)))), (-1.0))))), A::mul(s.ad_value(647), s.ad_value(170))));
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_mul_ad_lhs(169, A::scale(s.ad_value(159), s.v[115]), 114);
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_mul_ad_rhs(170, 114, A::voltage(ctx, &nodes, Some(13), Some(6)));
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_offset_ad(168, A::sub(s.ad_value(170), s.ad_value(518)), 0.02);
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_scale_ad(510, A::sub(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02)))), 0.5);
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_mul_ad_rhs(498, 169, A::add(A::mul(s.ad_value(648), A::sub(A::sub(A::sub(s.ad_value(170), s.ad_value(518)), s.ad_value(510)), A::mul(A::scale(s.ad_value(651), 0.5), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(510), 4.0), s.ad_value(651)))), (-1.0))))), A::mul(s.ad_value(646), s.ad_value(170))));
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_mul_ad_rhs(170, 114, A::voltage(ctx, &nodes, Some(14), Some(5)));
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_offset_ad(168, A::sub(s.ad_value(170), s.ad_value(518)), 0.02);
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_scale_ad(511, A::sub(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02)))), 0.5);
        }

        if ((s.v[1475] != 0.0) && (!(s.v[1476] != 0.0))) {
            s.store_mul_ad_rhs(499, 169, A::add(A::mul(s.ad_value(649), A::sub(A::sub(A::sub(s.ad_value(170), s.ad_value(518)), s.ad_value(511)), A::mul(A::scale(s.ad_value(652), 0.5), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(511), 4.0), s.ad_value(652)))), (-1.0))))), A::mul(s.ad_value(647), s.ad_value(170))));
        }

        s.v[1477] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        s.v[1478] = if (p.p76 != 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_scale(169, 159, s.v[115]);
        }

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_ad(500, &A::mul(A::mul(s.ad_value(169), s.ad_value(643)), A::voltage(ctx, &nodes, Some(10), Some(6))));
        }

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_ad(501, &A::mul(A::mul(s.ad_value(169), s.ad_value(642)), A::voltage(ctx, &nodes, Some(10), Some(5))));
        }

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_add(505, 498, 500);
        }

        if ((s.v[1477] != 0.0) && (s.v[1478] != 0.0)) {
            s.store_add(506, 499, 501);
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_scale(169, 159, s.v[115]);
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_ad(500, &A::mul(A::mul(s.ad_value(169), s.ad_value(643)), A::voltage(ctx, &nodes, Some(13), Some(6))));
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_ad(501, &A::mul(A::mul(s.ad_value(169), s.ad_value(642)), A::voltage(ctx, &nodes, Some(14), Some(5))));
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_add(505, 498, 500);
        }

        if ((s.v[1477] != 0.0) && (!(s.v[1478] != 0.0))) {
            s.store_add(506, 499, 501);
        }

        s.v[1479] = if (p.p78 == 1.0) { 1.0 } else { 0.0 };

        s.v[1480] = if (p.p76 != 2.0) { 1.0 } else { 0.0 };

        s.v[1481] = if (p.p63 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_scale(168, 159, s.v[115]);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_mul(644, 168, 644);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_mul(645, 168, 645);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_scale(513, 168, p.p15);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (s.v[1481] != 0.0)) {
            s.store_scale(514, 168, p.p16);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scalar(513, p.p15);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) && (!(s.v[1481] != 0.0))) {
            s.store_scalar(514, p.p16);
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_mul_ad_rhs(498, 644, A::voltage(ctx, &nodes, Some(10), Some(6)));
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_mul_ad_rhs(499, 645, A::voltage(ctx, &nodes, Some(10), Some(5)));
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.copy_ad(505, 498);
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.copy_ad(506, 499);
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_mul_ad_rhs(500, 513, A::voltage(ctx, &nodes, Some(10), Some(2)));
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (s.v[1480] != 0.0)) {
            s.store_mul_ad_rhs(501, 514, A::voltage(ctx, &nodes, Some(10), Some(0)));
        }

        s.v[1482] = if (p.p63 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_scale(168, 159, s.v[115]);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_mul(644, 168, 644);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_mul(645, 168, 645);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_scale(513, 168, p.p15);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_scale(514, 168, p.p16);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (!(s.v[1482] != 0.0))) {
            s.store_scalar(513, p.p15);
        }

        if ((((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) && (!(s.v[1482] != 0.0))) {
            s.store_scalar(514, p.p16);
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.store_mul_ad_rhs(498, 644, A::voltage(ctx, &nodes, Some(13), Some(6)));
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.store_mul_ad_rhs(499, 645, A::voltage(ctx, &nodes, Some(14), Some(5)));
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.copy_ad(505, 498);
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.copy_ad(506, 499);
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.store_mul_ad_rhs(500, 513, A::voltage(ctx, &nodes, Some(13), Some(2)));
        }

        if (((!(s.v[1477] != 0.0)) && (s.v[1479] != 0.0)) && (!(s.v[1480] != 0.0))) {
            s.store_mul_ad_rhs(501, 514, A::voltage(ctx, &nodes, Some(14), Some(0)));
        }

        s.v[1483] = if (p.p76 != 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_mul_ad_rhs(500, 453, A::voltage(ctx, &nodes, Some(10), Some(6)));
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_mul_ad_rhs(501, 453, A::voltage(ctx, &nodes, Some(10), Some(5)));
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_add(505, 498, 500);
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (s.v[1483] != 0.0)) {
            s.store_add(506, 499, 501);
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (!(s.v[1483] != 0.0))) {
            s.store_mul_ad_rhs(500, 453, A::voltage(ctx, &nodes, Some(13), Some(6)));
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (!(s.v[1483] != 0.0))) {
            s.store_mul_ad_rhs(501, 453, A::voltage(ctx, &nodes, Some(14), Some(5)));
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (!(s.v[1483] != 0.0))) {
            s.store_add(505, 498, 500);
        }

        if (((!(s.v[1477] != 0.0)) && (!(s.v[1479] != 0.0))) && (!(s.v[1483] != 0.0))) {
            s.store_add(506, 499, 501);
        }

        s.v[1484] = if (p.p65 == 1.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1484] != 0.0) {
            s.store_scalar(239, 1e-6);
        }

        if (s.v[1484] != 0.0) {
            s.store_mul_ad(178, A::div(s.ad_value(239), A::scale(s.ad_value(181), 2.0)), A::sqrt(s.ad_value(179)));
        }

        if (s.v[1484] != 0.0) {
            s.store_scale(168, 178, 0.5);
        }

        if (s.v[1484] != 0.0) {
            s.store_div_ad_lhs(170, A::neg(A::offset(s.ad_value(132), (-p.p144))), 179);
        }

        s.v[1485] = if ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt()))) { 1.0 } else { 0.0 };

        if ((s.v[1484] != 0.0) && (s.v[1485] != 0.0)) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
        }

        if ((s.v[1484] != 0.0) && (s.v[1485] != 0.0)) {
            s.store_offset_ad(340, A::square(s.ad_value(169)), 1.0);
        }

        if ((s.v[1484] != 0.0) && (s.v[1485] != 0.0)) {
            s.store_offset_ad(175, A::limited_exp(A::neg(s.ad_value(340))), (-1.0));
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) {
            s.store_sub_ad(171, A::scale(s.ad_value(170), 0.5), A::scale(A::offset(A::scale(s.ad_value(178), 1.0 / (((2.0) as f64).sqrt())), 1.0), 3.0));
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) {
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add(A::square(s.ad_value(171)), A::scale(s.ad_value(170), 6.0))));
        }

        s.v[1486] = if (s.v[170] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (s.v[1486] != 0.0)) {
            s.store_div_ad_lhs(172, A::sub(s.ad_value(170), s.ad_value(340)), 178);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (s.v[1486] != 0.0)) {
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (s.v[1486] != 0.0)) {
            let assign26500_ad_e46045: A = A::neg({
                if (!(((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38) {
                            A::ln(A::add(A::sub_from_scalar(1.0, s.ad_value(340)), A::square(s.ad_value(172))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(340, &assign26500_ad_e46045);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (!(s.v[1486] != 0.0))) {
            s.store_limited_exp_ad(341, A::scale(s.ad_value(340), (-1.2)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (!(s.v[1486] != 0.0))) {
            s.store_sub_ad_lhs(172, A::sqrt(A::add(A::add(A::offset(s.ad_value(170), (-1.0)), s.ad_value(341)), A::square(s.ad_value(168)))), 168);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (!(s.v[1486] != 0.0))) {
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1485] != 0.0))) && (!(s.v[1486] != 0.0))) {
            s.store_offset_ad(175, A::limited_exp(A::neg(s.ad_value(340))), (-1.0));
        }

        if (s.v[1484] != 0.0) {
            s.store_sqrt_ad(176, A::add(s.ad_value(175), s.ad_value(340)));
        }

        s.v[1487] = if (s.v[340] > 1e-15) { 1.0 } else { 0.0 };

        if ((s.v[1484] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_add_ad(344, A::neg(A::sub(s.ad_value(170), s.ad_value(340))), A::mul(s.ad_value(178), s.ad_value(176)));
        }

        if ((s.v[1484] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_sub_from_scalar_ad(345, 1.0, A::div(A::mul(A::scale(s.ad_value(178), 0.5), s.ad_value(175)), s.ad_value(176)));
        }

        if ((s.v[1484] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        s.v[1488] = if (s.v[340] < (-1e-15)) { 1.0 } else { 0.0 };

        if (((s.v[1484] != 0.0) && (!(s.v[1487] != 0.0))) && (s.v[1488] != 0.0)) {
            s.store_sub_ad(344, A::neg(A::sub(s.ad_value(170), s.ad_value(340))), A::mul(s.ad_value(178), s.ad_value(176)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1487] != 0.0))) && (s.v[1488] != 0.0)) {
            s.store_offset_ad(345, A::div(A::mul(A::scale(s.ad_value(178), 0.5), s.ad_value(175)), s.ad_value(176)), 1.0);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1487] != 0.0))) && (s.v[1488] != 0.0)) {
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1487] != 0.0))) && (!(s.v[1488] != 0.0))) {
            s.store_scalar(177, 0.0);
        }

        if (s.v[1484] != 0.0) {
            s.store_mul_ad_lhs(906, A::mul(s.ad_value(178), A::limited_exp(A::scale(A::neg(s.ad_value(177)), 0.5))), 179);
        }

        if (s.v[1484] != 0.0) {
            s.store_ad(915, &A::abs(A::voltage(ctx, &nodes, Some(7), Some(6))));
        }

        if (s.v[1484] != 0.0) {
            s.store_mul_ad_lhs(916, A::div_from_scalar((2.0 * p.p454), s.ad_value(416)), 397);
        }

        if (s.v[1484] != 0.0) {
            s.store_scale(917, 916, p.p1);
        }

        if (s.v[1484] != 0.0) {
            s.store_scalar(920, (1.0 / p.p530));
        }

        if (s.v[1484] != 0.0) {
            s.store_scale_ad(175, A::add(s.ad_value(906), A::scale(s.ad_value(182), 2.0)), p.p491);
        }

        if (s.v[1484] != 0.0) {
            s.store_div_ad(918, A::mul(s.ad_value(917), s.ad_value(175)), A::add(s.ad_value(917), s.ad_value(175)));
        }

        if (s.v[1484] != 0.0) {
            let assign26720_ad_e46327: A = {
                if (!((s.v[918] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::scale(A::add(A::offset(s.ad_value(918), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(918), (-0.001)), A::offset(s.ad_value(918), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5)
                } else {
                    {
                        if ((s.v[918] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_from_scalar(((-1e-5) * 1e-5), A::offset(s.ad_value(918), (-0.001)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(918, assign26720_ad_e46327, 0.001);
        }

        if (s.v[1484] != 0.0) {
            s.store_powf_ad(176, A::offset(A::div(s.ad_value(915), s.ad_value(918)), 1e-6), p.p530);
        }

        if (s.v[1484] != 0.0) {
            s.store_ad(177, &A::pow(A::offset(s.ad_value(176), 1.0), s.ad_value(920)));
        }

        if (s.v[1484] != 0.0) {
            s.store_ad(919, &A::min(A::div(s.ad_value(915), s.ad_value(177)), s.ad_value(915)));
        }

        if (s.v[1484] != 0.0) {
            s.store_scalar(239, 1e-6);
        }

        if (s.v[1484] != 0.0) {
            s.store_mul_ad(178, A::div(s.ad_value(239), A::scale(s.ad_value(181), 2.0)), A::sqrt(s.ad_value(179)));
        }

        if (s.v[1484] != 0.0) {
            s.store_scale(168, 178, 0.5);
        }

        if (s.v[1484] != 0.0) {
            s.store_div_ad_lhs(170, A::neg(A::offset(A::add(s.ad_value(133), s.ad_value(919)), (-p.p143))), 179);
        }

        s.v[1489] = if ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt()))) { 1.0 } else { 0.0 };

        if ((s.v[1484] != 0.0) && (s.v[1489] != 0.0)) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
        }

        if ((s.v[1484] != 0.0) && (s.v[1489] != 0.0)) {
            s.store_offset_ad(340, A::square(s.ad_value(169)), 1.0);
        }

        if ((s.v[1484] != 0.0) && (s.v[1489] != 0.0)) {
            s.store_offset_ad(175, A::limited_exp(A::neg(s.ad_value(340))), (-1.0));
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) {
            s.store_sub_ad(171, A::scale(s.ad_value(170), 0.5), A::scale(A::offset(A::scale(s.ad_value(178), 1.0 / (((2.0) as f64).sqrt())), 1.0), 3.0));
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) {
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add(A::square(s.ad_value(171)), A::scale(s.ad_value(170), 6.0))));
        }

        s.v[1490] = if (s.v[170] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (s.v[1490] != 0.0)) {
            s.store_div_ad_lhs(172, A::sub(s.ad_value(170), s.ad_value(340)), 178);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (s.v[1490] != 0.0)) {
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (s.v[1490] != 0.0)) {
            let assign26890_ad_e46539: A = A::neg({
                if (!(((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38) {
                            A::ln(A::add(A::sub_from_scalar(1.0, s.ad_value(340)), A::square(s.ad_value(172))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_ad(340, &assign26890_ad_e46539);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (!(s.v[1490] != 0.0))) {
            s.store_limited_exp_ad(341, A::scale(s.ad_value(340), (-1.2)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (!(s.v[1490] != 0.0))) {
            s.store_sub_ad_lhs(172, A::sqrt(A::add(A::add(A::offset(s.ad_value(170), (-1.0)), s.ad_value(341)), A::square(s.ad_value(168)))), 168);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (!(s.v[1490] != 0.0))) {
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1489] != 0.0))) && (!(s.v[1490] != 0.0))) {
            s.store_offset_ad(175, A::limited_exp(A::neg(s.ad_value(340))), (-1.0));
        }

        if (s.v[1484] != 0.0) {
            s.store_sqrt_ad(176, A::add(s.ad_value(175), s.ad_value(340)));
        }

        s.v[1491] = if (s.v[340] > 1e-15) { 1.0 } else { 0.0 };

        if ((s.v[1484] != 0.0) && (s.v[1491] != 0.0)) {
            s.store_add_ad(344, A::neg(A::sub(s.ad_value(170), s.ad_value(340))), A::mul(s.ad_value(178), s.ad_value(176)));
        }

        if ((s.v[1484] != 0.0) && (s.v[1491] != 0.0)) {
            s.store_sub_from_scalar_ad(345, 1.0, A::div(A::mul(A::scale(s.ad_value(178), 0.5), s.ad_value(175)), s.ad_value(176)));
        }

        if ((s.v[1484] != 0.0) && (s.v[1491] != 0.0)) {
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        s.v[1492] = if (s.v[340] < (-1e-15)) { 1.0 } else { 0.0 };

        if (((s.v[1484] != 0.0) && (!(s.v[1491] != 0.0))) && (s.v[1492] != 0.0)) {
            s.store_sub_ad(344, A::neg(A::sub(s.ad_value(170), s.ad_value(340))), A::mul(s.ad_value(178), s.ad_value(176)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1491] != 0.0))) && (s.v[1492] != 0.0)) {
            s.store_offset_ad(345, A::div(A::mul(A::scale(s.ad_value(178), 0.5), s.ad_value(175)), s.ad_value(176)), 1.0);
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1491] != 0.0))) && (s.v[1492] != 0.0)) {
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        if (((s.v[1484] != 0.0) && (!(s.v[1491] != 0.0))) && (!(s.v[1492] != 0.0))) {
            s.store_scalar(177, 0.0);
        }

        if (s.v[1484] != 0.0) {
            s.store_mul_ad_lhs(907, A::mul(s.ad_value(178), A::limited_exp(A::scale(A::neg(s.ad_value(177)), 0.5))), 179);
        }

        if (s.v[1484] != 0.0) {
            s.store_sub(911, 906, 907);
        }

        if (s.v[1484] != 0.0) {
            s.store_scaled_add(910, 906, 907, 0.5);
        }

        if (s.v[1484] != 0.0) {
            s.store_div_ad_lhs(341, A::add(A::scale(s.ad_value(910), 2.0), s.ad_value(181)), 209);
        }

        if (s.v[1484] != 0.0) {
            s.store_add_ad_rhs(905, 910, A::div(A::square(s.ad_value(911)), A::scale(s.ad_value(341), 6.0)));
        }

        if (s.v[1484] != 0.0) {
            s.store_scale_ad(909, A::sub(s.ad_value(910), A::mul(A::scale(s.ad_value(911), 0.16666666666666666), A::sub_from_scalar(1.0, A::mul(A::div(s.ad_value(911), s.ad_value(341)), A::offset(A::div(s.ad_value(911), A::scale(s.ad_value(341), 5.0)), 1.0))))), 0.5);
        }

        if (s.v[1484] != 0.0) {
            s.store_sub(908, 905, 909);
        }

        s.v[1493] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if ((s.v[1484] != 0.0) && (s.v[1493] != 0.0)) {
            s.store_mul_ad_lhs(169, A::scale(s.ad_value(160), (s.v[115] * p.p1)), 494);
        }

        if ((s.v[1484] != 0.0) && (!(s.v[1493] != 0.0))) {
            s.store_mul_ad_lhs(169, A::scale(s.ad_value(159), (s.v[115] * p.p1)), 494);
        }

        if (s.v[1484] != 0.0) {
            s.copy_ad(176, 908);
        }

        if (s.v[1484] != 0.0) {
            s.copy_ad(177, 909);
        }

        if (s.v[1484] != 0.0) {
            s.store_mul(340, 176, 169);
        }

        if (s.v[1484] != 0.0) {
            s.store_mul(341, 177, 169);
        }

        if (s.v[1484] != 0.0) {
            s.copy_ad(908, 340);
        }

        if (s.v[1484] != 0.0) {
            s.copy_ad(909, 341);
        }

        if (s.v[1484] != 0.0) {
            s.copy_ad(504, 908);
        }

        if (s.v[1484] != 0.0) {
            s.copy_ad(503, 909);
        }

        s.store_ad(502, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(2)), p.p17));

        s.v[1494] = if (p.p71 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1494] != 0.0) {
            s.store_div_ad_lhs(168, A::add(s.ad_value(259), A::mul(s.ad_value(260), s.ad_value(153))), 153);
        }

        s.v[1495] = if ((s.v[168] <= 0.0) || (s.v[248] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1494] != 0.0) && (!(s.v[1495] != 0.0))) {
            s.store_div_ad(169, A::neg(s.ad_value(248)), A::offset(s.ad_value(202), 1e-30));
        }

        s.v[1496] = if (p.p71 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_div_ad_lhs(493, A::add(s.ad_value(261), A::mul(s.ad_value(262), s.ad_value(153))), 153);
        }

        s.v[1497] = if (s.v[493] <= 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul(168, 783, 153);
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_div_ad(169, A::mul(s.ad_value(249), s.ad_value(168)), A::offset(s.ad_value(168), 1.0));
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            let assign27350_ad_e47023: A = {
                if (!((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441))) {
                    A::scale(A::add(A::mul(s.ad_value(786), s.ad_value(348)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(786), s.ad_value(348)), A::mul(s.ad_value(786), s.ad_value(348))), ((4.0 * p.p1441) * p.p1441)))), 0.5)
                } else {
                    {
                        if ((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441)) {
                            A::div_from_scalar(((-p.p1441) * p.p1441), A::mul(s.ad_value(786), s.ad_value(348)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(168, 1.0, A::offset(assign27350_ad_e47023, 1.0));
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add(171, 168, 787);
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            let assign27370_ad_e47094: A = {
                if (!((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442))) {
                    A::scale(A::add(A::mul(s.ad_value(348), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(348), s.ad_value(171)), A::mul(s.ad_value(348), s.ad_value(171))), ((4.0 * p.p1442) * p.p1442)))), 0.5)
                } else {
                    {
                        if ((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442)) {
                            A::div_from_scalar(((-p.p1442) * p.p1442), A::mul(s.ad_value(348), s.ad_value(171)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(170, &assign27370_ad_e47094);
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_div_from_scalar_ad(171, 1.0, A::offset(A::mul(s.ad_value(788), s.ad_value(126)), 1.0));
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul_ad_lhs(491, A::mul(s.ad_value(169), s.ad_value(170)), 171);
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul_ad_rhs(490, 491, A::sub_from_scalar(1.0, A::div(s.ad_value(784), s.ad_value(153))));
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sub(489, 126, 490);
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_add_ad(168, A::add(s.ad_value(782), A::mul(s.ad_value(781), s.ad_value(489))), A::mul(A::mul(s.ad_value(780), s.ad_value(489)), s.ad_value(489)));
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1e-10));
        }

        s.v[1498] = if (p.p69 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1498] != 0.0) {
            s.store_div_ad_lhs(169, A::div(A::sub(s.ad_value(399), s.ad_value(725)), s.ad_value(726)), 179);
        }

        if (s.v[1498] != 0.0) {
            let assign27490_ad_e47360: A = A::add(A::offset(A::offset(A::sub(s.ad_value(243), A::mul(s.ad_value(723), s.ad_value(399))), (-(-p.p1110))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::sub(s.ad_value(243), A::mul(s.ad_value(723), s.ad_value(399))), (-(-p.p1110))), (-1e-6)), A::offset(A::offset(A::sub(s.ad_value(243), A::mul(s.ad_value(723), s.ad_value(399))), (-(-p.p1110))), (-1e-6))), (-((4.0 * (-p.p1110)) * 1e-6)))));
            s.store_offset_ad(170, A::scale(assign27490_ad_e47360, 0.5), (-p.p1110));
        }

        if (s.v[1498] != 0.0) {
            s.store_offset_ad(171, A::mul(s.ad_value(724), s.ad_value(399)), 1.0);
        }

        if (s.v[1498] != 0.0) {
            s.store_mul_ad_lhs(172, A::scale(s.ad_value(170), ((-982222000000.0) * p.p1109)), 171);
        }

        if (s.v[1498] != 0.0) {
            s.store_ad(174, &A::limited_exp(s.ad_value(172)));
        }

        if (s.v[1498] != 0.0) {
            s.store_scalar(175, 3.75956e-7);
        }

        if (s.v[1498] != 0.0) {
            s.store_sub_ad_lhs(468, A::sub(s.ad_value(167), A::scale(s.ad_value(146), 0.5)), 166);
        }

        if (s.v[1498] != 0.0) {
            s.store_sub(168, 468, 497);
        }

        if (s.v[1498] != 0.0) {
            s.store_div_ad_lhs(169, A::div(s.ad_value(168), s.ad_value(731)), 179);
        }

        s.v[1499] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1498] != 0.0) && (s.v[1499] != 0.0)) {
            s.copy_ad(466, 904);
        }

        s.v[1500] = if (s.v[468] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1498] != 0.0) && (!(s.v[1499] != 0.0))) && (s.v[1500] != 0.0)) {
            s.store_scale_ad(466, A::add(A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::sub(A::mul(A::offset(s.ad_value(168), (-0.02)), A::offset(s.ad_value(168), (-0.02))), A::scale(s.ad_value(468), 0.08)))), 0.5);
        }

        if (((s.v[1498] != 0.0) && (!(s.v[1499] != 0.0))) && (!(s.v[1500] != 0.0))) {
            s.store_scale_ad(466, A::add(A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::add(A::mul(A::offset(s.ad_value(168), (-0.02)), A::offset(s.ad_value(168), (-0.02))), A::scale(s.ad_value(468), 0.08)))), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1498] != 0.0) {
            let assign27650_ad_e47590: A = A::add(A::offset(A::offset(A::sub(s.ad_value(244), A::mul(s.ad_value(729), s.ad_value(466))), (-(-p.p1111))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::sub(s.ad_value(244), A::mul(s.ad_value(729), s.ad_value(466))), (-(-p.p1111))), (-1e-6)), A::offset(A::offset(A::sub(s.ad_value(244), A::mul(s.ad_value(729), s.ad_value(466))), (-(-p.p1111))), (-1e-6))), (-((4.0 * (-p.p1111)) * 1e-6)))));
            s.store_offset_ad(170, A::scale(assign27650_ad_e47590, 0.5), (-p.p1111));
        }

        if (s.v[1498] != 0.0) {
            s.store_offset_ad(171, A::mul(s.ad_value(730), s.ad_value(466)), 1.0);
        }

        if (s.v[1498] != 0.0) {
            s.store_mul_ad_lhs(172, A::scale(s.ad_value(170), ((-745669000000.0) * p.p1109)), 171);
        }

        if (s.v[1498] != 0.0) {
            s.store_ad(174, &A::limited_exp(s.ad_value(172)));
        }

        if (s.v[1498] != 0.0) {
            s.store_scalar(175, 4.97232e-7);
        }

        s.v[1501] = if (p.p68 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1501] != 0.0) {
            let assign27730_ad_e47693: A = A::add(A::offset(A::offset(A::sub(s.ad_value(245), A::mul(s.ad_value(734), s.ad_value(399))), (-(-p.p1112))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::sub(s.ad_value(245), A::mul(s.ad_value(734), s.ad_value(399))), (-(-p.p1112))), (-1e-6)), A::offset(A::offset(A::sub(s.ad_value(245), A::mul(s.ad_value(734), s.ad_value(399))), (-(-p.p1112))), (-1e-6))), (-((4.0 * (-p.p1112)) * 1e-6)))));
            s.store_offset_ad(169, A::scale(assign27730_ad_e47693, 0.5), (-p.p1112));
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(170, A::mul(s.ad_value(735), s.ad_value(399)), 1.0);
        }

        if (s.v[1501] != 0.0) {
            s.store_mul_ad_lhs(171, A::mul(A::scale(A::neg(s.ad_value(485)), p.p1109), s.ad_value(169)), 170);
        }

        if (s.v[1501] != 0.0) {
            s.store_mul_ad_rhs(172, 399, A::limited_exp(s.ad_value(171)));
        }

        if (s.v[1501] != 0.0) {
            s.store_add_ad(174, A::add(s.ad_value(497), A::scale(s.ad_value(127), 0.5)), A::scale(A::add(s.ad_value(521), s.ad_value(522)), 0.5));
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(473, A::sqrt(A::offset(A::square(s.ad_value(390)), 0.01)), (-0.1));
        }

        if (s.v[1501] != 0.0) {
            s.store_mul(169, 736, 473);
        }

        if (s.v[1501] != 0.0) {
            s.store_limited_exp_ad(474, A::neg(s.ad_value(169)));
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(171, A::offset(A::add(s.ad_value(169), s.ad_value(474)), (-1.0)), 0.0001);
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(172, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(169), 1.0), s.ad_value(474))), 0.0001);
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(174, A::square(s.ad_value(169)), 0.0002);
        }

        if (s.v[1501] != 0.0) {
            s.store_sub(168, 134, 479);
        }

        if (s.v[1501] != 0.0) {
            s.store_sqrt_ad(482, A::offset(A::square(s.ad_value(168)), 0.0001));
        }

        s.v[1502] = if (p.p82 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1501] != 0.0) && (s.v[1502] != 0.0)) {
            let assign27900_ad_e47905: A = {
                if (!((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::sub(s.ad_value(246), A::mul(s.ad_value(739), s.ad_value(482))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(246), A::mul(s.ad_value(739), s.ad_value(482))), A::sub(s.ad_value(246), A::mul(s.ad_value(739), s.ad_value(482)))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::sub(s.ad_value(246), A::mul(s.ad_value(739), s.ad_value(482))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(169, &assign27900_ad_e47905);
        }

        s.v[1503] = if (s.v[740] < 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1501] != 0.0) && (s.v[1502] != 0.0)) && (s.v[1503] != 0.0)) {
            s.store_scalar(740, 0.01);
        }

        if ((s.v[1501] != 0.0) && (!(s.v[1502] != 0.0))) {
            s.store_sub_ad_rhs(169, 246, A::mul(s.ad_value(739), s.ad_value(482)));
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(170, A::mul(s.ad_value(740), s.ad_value(482)), 1.0);
        }

        if (s.v[1501] != 0.0) {
            s.store_mul_ad_lhs(171, A::mul(A::mul(A::scale(A::neg(s.ad_value(485)), p.p1109), s.ad_value(742)), s.ad_value(169)), 170);
        }

        if (s.v[1501] != 0.0) {
            s.store_ad(172, &A::limited_exp(s.ad_value(171)));
        }

        if (s.v[1501] != 0.0) {
            s.store_sub(168, 136, 479);
        }

        if (s.v[1501] != 0.0) {
            s.store_sqrt_ad(483, A::offset(A::square(s.ad_value(168)), 0.0001));
        }

        s.v[1505] = if (p.p82 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1501] != 0.0) && (s.v[1505] != 0.0)) {
            let assign28030_ad_e48068: A = {
                if (!((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::sub(s.ad_value(247), A::mul(s.ad_value(745), s.ad_value(483))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(247), A::mul(s.ad_value(745), s.ad_value(483))), A::sub(s.ad_value(247), A::mul(s.ad_value(745), s.ad_value(483)))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::sub(s.ad_value(247), A::mul(s.ad_value(745), s.ad_value(483))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(169, &assign28030_ad_e48068);
        }

        s.v[1506] = if (s.v[746] < 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1501] != 0.0) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
            s.store_scalar(746, 0.01);
        }

        if ((s.v[1501] != 0.0) && (!(s.v[1505] != 0.0))) {
            s.store_sub_ad_rhs(169, 247, A::mul(s.ad_value(745), s.ad_value(483)));
        }

        if (s.v[1501] != 0.0) {
            s.store_offset_ad(170, A::mul(s.ad_value(746), s.ad_value(483)), 1.0);
        }

        if (s.v[1501] != 0.0) {
            s.store_mul_ad_lhs(171, A::mul(A::mul(A::scale(A::neg(s.ad_value(485)), p.p1109), s.ad_value(742)), s.ad_value(169)), 170);
        }

        if (s.v[1501] != 0.0) {
            s.store_ad(172, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1508] = if (p.p70 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1508] != 0.0) {
            s.store_scalar(168, (s.v[145] * p.p89));
        }

        s.v[1509] = if ((s.v[747] <= 0.0) || (s.v[252] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1508] != 0.0) && (s.v[1509] != 0.0)) {
            s.store_scalar(175, 0.0);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::neg(s.ad_value(136)), s.ad_value(750)), s.ad_value(479)), 168);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) {
            s.store_ad(169, &{
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01)))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) {
            s.store_div_ad_rhs(170, 252, A::offset(s.ad_value(169), 0.001));
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) {
            s.store_ad(171, &A::pow(s.ad_value(169), s.ad_value(751)));
        }

        s.v[1510] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) && (s.v[1510] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::neg(s.ad_value(522)), s.ad_value(522)), 522);
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) && (s.v[1510] != 0.0)) {
            s.store_offset_ad(173, A::add(s.ad_value(749), A::abs(s.ad_value(172))), 1e-5);
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) && (s.v[1510] != 0.0)) {
            let assign28240_ad_e48333: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(172), s.ad_value(173)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28240_ad_e48333, (-1e-6));
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) && (s.v[1510] != 0.0)) {
            s.store_mul_ad_lhs(175, A::mul(A::mul(A::mul(s.ad_value(747), s.ad_value(158)), s.ad_value(171)), A::limited_exp(A::neg(s.ad_value(170)))), 174);
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1509] != 0.0))) && (!(s.v[1510] != 0.0))) {
            s.store_mul_ad_lhs(175, A::mul(A::mul(A::mul(s.ad_value(747), s.ad_value(158)), s.ad_value(171)), A::limited_exp(A::neg(s.ad_value(170)))), 135);
        }

        s.v[1511] = if ((p.p70 == 3.0) && (s.v[752] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1512] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            let assign28290_ad_e48464: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(254, 754, assign28290_ad_e48464);
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::sub(A::mul(A::mul(s.ad_value(753), s.ad_value(136)), s.ad_value(136)), A::mul(s.ad_value(254), s.ad_value(136))), s.ad_value(755)), s.ad_value(479)), 179);
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_mul_ad(170, A::mul(A::mul(s.ad_value(752), s.ad_value(158)), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::neg(s.ad_value(522)), s.ad_value(522)), 522);
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_offset_ad(173, A::add(s.ad_value(749), A::abs(s.ad_value(172))), 1e-5);
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            let assign28340_ad_e48583: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(172), s.ad_value(173)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28340_ad_e48583, (-1e-6));
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_add_ad_rhs(175, 175, A::mul(s.ad_value(170), s.ad_value(174)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (!(s.v[1512] != 0.0))) {
            let assign28360_ad_e48678: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(254, 754, assign28360_ad_e48678);
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (!(s.v[1512] != 0.0))) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::sub(A::mul(A::mul(s.ad_value(753), s.ad_value(136)), s.ad_value(136)), A::mul(s.ad_value(254), s.ad_value(136))), s.ad_value(755)), s.ad_value(479)), 179);
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (!(s.v[1512] != 0.0))) {
            s.store_mul_ad(170, A::mul(A::mul(s.ad_value(752), s.ad_value(158)), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1511] != 0.0)) && (!(s.v[1512] != 0.0))) {
            s.store_add_ad_rhs(175, 175, A::mul(s.ad_value(170), s.ad_value(135)));
        }

        s.v[1513] = if (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };

        if ((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) {
            let assign28410_ad_e48832: A = {
                if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(255, 757, assign28410_ad_e48832);
        }

        s.v[1514] = if ((s.v[756] <= 0.0) || (s.v[255] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (s.v[1514] != 0.0)) {
            s.store_scalar(176, 0.0);
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::neg(s.ad_value(136)), s.ad_value(759)), s.ad_value(479)), 168);
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_ad(169, &{
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01)))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_div_ad_rhs(170, 255, A::offset(s.ad_value(169), 0.001));
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_ad(171, &A::pow(s.ad_value(169), s.ad_value(760)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_mul_ad_lhs(172, A::mul(A::neg(s.ad_value(522)), s.ad_value(522)), 522);
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_offset_ad(173, A::add(s.ad_value(758), A::abs(s.ad_value(172))), 1e-5);
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            let assign28500_ad_e49016: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(172), s.ad_value(173)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28500_ad_e49016, (-1e-6));
        }

        if (((s.v[1508] != 0.0) && (s.v[1513] != 0.0)) && (!(s.v[1514] != 0.0))) {
            s.store_mul_ad_lhs(176, A::mul(A::mul(A::mul(s.ad_value(756), s.ad_value(896)), s.ad_value(171)), A::limited_exp(A::neg(s.ad_value(170)))), 174);
        }

        s.v[1516] = if ((s.v[761] <= 0.0) || (s.v[250] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1508] != 0.0) && (s.v[1516] != 0.0)) {
            s.store_scalar(175, 0.0);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::neg(s.ad_value(134)), s.ad_value(764)), s.ad_value(479)), 168);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) {
            s.store_ad(169, &{
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01)))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) {
            s.store_div_ad_rhs(170, 250, A::offset(s.ad_value(169), 0.001));
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) {
            s.store_ad(171, &A::pow(s.ad_value(169), s.ad_value(765)));
        }

        s.v[1517] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) && (s.v[1517] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::neg(s.ad_value(521)), s.ad_value(521)), 521);
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) && (s.v[1517] != 0.0)) {
            s.store_offset_ad(173, A::add(s.ad_value(763), A::abs(s.ad_value(172))), 1e-5);
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) && (s.v[1517] != 0.0)) {
            let assign28660_ad_e49242: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(172), s.ad_value(173)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28660_ad_e49242, (-1e-6));
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) && (s.v[1517] != 0.0)) {
            s.store_mul_ad_lhs(175, A::mul(A::mul(A::mul(s.ad_value(761), s.ad_value(158)), s.ad_value(171)), A::limited_exp(A::neg(s.ad_value(170)))), 174);
        }

        if (((s.v[1508] != 0.0) && (!(s.v[1516] != 0.0))) && (!(s.v[1517] != 0.0))) {
            s.store_mul_ad(175, A::mul(A::mul(A::mul(s.ad_value(761), s.ad_value(158)), s.ad_value(171)), A::limited_exp(A::neg(s.ad_value(170)))), A::neg(s.ad_value(135)));
        }

        s.v[1518] = if ((p.p70 == 3.0) && (s.v[766] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1519] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            let assign28710_ad_e49374: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(253, 768, assign28710_ad_e49374);
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::sub(A::mul(A::mul(s.ad_value(767), s.ad_value(134)), s.ad_value(134)), A::mul(s.ad_value(253), s.ad_value(134))), s.ad_value(769)), s.ad_value(479)), 179);
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_mul_ad(170, A::mul(A::mul(s.ad_value(766), s.ad_value(158)), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::neg(s.ad_value(521)), s.ad_value(521)), 521);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_offset_ad(173, A::add(s.ad_value(763), A::abs(s.ad_value(172))), 1e-5);
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            let assign28760_ad_e49493: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(172), s.ad_value(173)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28760_ad_e49493, (-1e-6));
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_add_ad_rhs(175, 175, A::mul(s.ad_value(170), s.ad_value(174)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (!(s.v[1519] != 0.0))) {
            let assign28780_ad_e49588: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(253, 768, assign28780_ad_e49588);
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (!(s.v[1519] != 0.0))) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::sub(A::mul(A::mul(s.ad_value(767), s.ad_value(134)), s.ad_value(134)), A::mul(s.ad_value(253), s.ad_value(134))), s.ad_value(769)), s.ad_value(479)), 179);
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (!(s.v[1519] != 0.0))) {
            s.store_mul_ad(170, A::mul(A::mul(s.ad_value(766), s.ad_value(158)), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1518] != 0.0)) && (!(s.v[1519] != 0.0))) {
            s.store_add_ad_rhs(175, 175, A::mul(s.ad_value(170), A::neg(s.ad_value(135))));
        }

        s.v[1520] = if (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };

        if ((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) {
            let assign28830_ad_e49743: A = {
                if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(251, 771, assign28830_ad_e49743);
        }

        s.v[1521] = if ((s.v[770] <= 0.0) || (s.v[251] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (s.v[1521] != 0.0)) {
            s.store_scalar(176, 0.0);
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_div_ad_lhs(169, A::add(A::sub(A::neg(s.ad_value(134)), s.ad_value(773)), s.ad_value(479)), 168);
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_ad(169, &{
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01)))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_div_ad_rhs(170, 251, A::offset(s.ad_value(169), 0.001));
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_ad(171, &A::pow(s.ad_value(169), s.ad_value(774)));
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_mul_ad_lhs(172, A::mul(A::neg(s.ad_value(521)), s.ad_value(521)), 521);
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_offset_ad(173, A::add(s.ad_value(772), A::abs(s.ad_value(172))), 1e-5);
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            let assign28920_ad_e49927: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(A::div(s.ad_value(172), s.ad_value(173)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28920_ad_e49927, (-1e-6));
        }

        if (((s.v[1508] != 0.0) && (s.v[1520] != 0.0)) && (!(s.v[1521] != 0.0))) {
            s.store_mul_ad_lhs(176, A::mul(A::mul(A::mul(s.ad_value(770), s.ad_value(896)), s.ad_value(171)), A::limited_exp(A::neg(s.ad_value(170)))), 174);
        }

        s.v[1523] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1524] = if (s.v[537] > 0.0) { 1.0 } else { 0.0 };

        s.v[1525] = if (s.v[521] < s.v[543]) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (s.v[1525] != 0.0)) {
            s.store_div(168, 521, 539);
        }

        if (((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (s.v[1525] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(s.ad_value(168)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (s.v[1525] != 0.0)) {
            s.store_add_ad_rhs(170, 542, A::mul(s.ad_value(541), A::sub(s.ad_value(521), s.ad_value(543))));
        }

        s.v[1526] = if (s.v[521] <= s.v[546]) { 1.0 } else { 0.0 };

        if ((((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (!(s.v[1525] != 0.0))) && (s.v[1526] != 0.0)) {
            s.store_div(168, 521, 539);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (!(s.v[1525] != 0.0))) && (s.v[1526] != 0.0)) {
            s.store_div_ad_lhs(169, A::offset(s.ad_value(521), p.p1626), 539);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (!(s.v[1525] != 0.0))) && (s.v[1526] != 0.0)) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        s.v[1527] = if (s.v[281] > 0.0) { 1.0 } else { 0.0 };

        s.v[1528] = if ((p.p1643 - s.v[521]) < (p.p1643 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (s.v[1528] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 287);
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (s.v[1528] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (!(s.v[1528] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 287);
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (!(s.v[1528] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1643), A::sub_from_scalar(p.p1643, s.ad_value(521)))), (-1.0));
        }

        s.v[1529] = if (s.v[283] > 0.0) { 1.0 } else { 0.0 };

        s.v[1530] = if ((p.p1645 - s.v[521]) < (p.p1645 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (s.v[1530] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 289);
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (s.v[1530] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (!(s.v[1530] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 289);
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (!(s.v[1530] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1645), A::sub_from_scalar(p.p1645, s.ad_value(521)))), (-1.0));
        }

        s.v[1531] = if (s.v[285] > 0.0) { 1.0 } else { 0.0 };

        s.v[1532] = if ((p.p1647 - s.v[521]) < (p.p1647 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 291);
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 291);
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1647), A::sub_from_scalar(p.p1647, s.ad_value(521)))), (-1.0));
        }

        s.v[1533] = if (s.v[538] > 0.0) { 1.0 } else { 0.0 };

        s.v[1534] = if (s.v[522] < s.v[550]) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (s.v[1534] != 0.0)) {
            s.store_div(168, 522, 540);
        }

        if (((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (s.v[1534] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(s.ad_value(168)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (s.v[1534] != 0.0)) {
            s.store_add_ad_rhs(170, 549, A::mul(s.ad_value(548), A::sub(s.ad_value(522), s.ad_value(550))));
        }

        s.v[1535] = if (s.v[522] <= s.v[553]) { 1.0 } else { 0.0 };

        if ((((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (!(s.v[1534] != 0.0))) && (s.v[1535] != 0.0)) {
            s.store_div(168, 522, 540);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (!(s.v[1534] != 0.0))) && (s.v[1535] != 0.0)) {
            s.store_div_ad_lhs(169, A::offset(s.ad_value(522), p.p1627), 540);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (!(s.v[1534] != 0.0))) && (s.v[1535] != 0.0)) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        s.v[1536] = if (s.v[282] > 0.0) { 1.0 } else { 0.0 };

        s.v[1537] = if ((p.p1644 - s.v[522]) < (p.p1644 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 288);
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 288);
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1644), A::sub_from_scalar(p.p1644, s.ad_value(522)))), (-1.0));
        }

        s.v[1538] = if (s.v[284] > 0.0) { 1.0 } else { 0.0 };

        s.v[1539] = if ((p.p1646 - s.v[522]) < (p.p1646 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (s.v[1539] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 290);
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (s.v[1539] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (!(s.v[1539] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 290);
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (!(s.v[1539] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1646), A::sub_from_scalar(p.p1646, s.ad_value(522)))), (-1.0));
        }

        s.v[1540] = if (s.v[286] > 0.0) { 1.0 } else { 0.0 };

        s.v[1541] = if ((p.p1648 - s.v[522]) < (p.p1648 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (s.v[1541] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 292);
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (s.v[1541] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (!(s.v[1541] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 292);
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (!(s.v[1541] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1648), A::sub_from_scalar(p.p1648, s.ad_value(522)))), (-1.0));
        }

        s.v[1550] = if (s.v[523] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) {
            s.store_div(1542, 521, 269);
        }

        s.v[1551] = if (s.v[1542] < 0.9) { 1.0 } else { 0.0 };

        s.v[1552] = if (p.p1602 > 0.0) { 1.0 } else { 0.0 };

        s.v[1553] = if (s.v[521] > s.v[557]) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_sub_from_scalar(1547, 1.0, 1542);
        }

        s.v[1554] = if (p.p1596 != 1.0) { 1.0 } else { 0.0 };

        s.v[1555] = if (p.p1596 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) && (s.v[1554] != 0.0)) && (s.v[1555] != 0.0)) {
            s.store_div_from_scalar_ad(1548, 1.0, A::sqrt(s.ad_value(1547)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) && (s.v[1554] != 0.0)) && (!(s.v[1555] != 0.0))) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) && (s.v[1554] != 0.0)) {
            s.store_scale_ad(530, A::mul(A::mul(s.ad_value(269), s.ad_value(523)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548)))), 1.0 / ((1.0 - p.p1596)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) && (!(s.v[1554] != 0.0))) {
            s.store_ad(530, &A::mul(A::mul(A::neg(s.ad_value(269)), s.ad_value(523)), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) {
            s.store_sub_from_scalar_ad(1547, 1.0, A::div(s.ad_value(557), s.ad_value(269)));
        }

        s.v[1556] = if (p.p1596 != 1.0) { 1.0 } else { 0.0 };

        s.v[1557] = if (p.p1596 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (s.v[1556] != 0.0)) && (s.v[1557] != 0.0)) {
            s.store_div_from_scalar_ad(1548, 1.0, A::sqrt(s.ad_value(1547)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (s.v[1556] != 0.0)) && (!(s.v[1557] != 0.0))) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (s.v[1556] != 0.0)) {
            s.store_scale_ad(1549, A::mul(A::mul(s.ad_value(269), s.ad_value(523)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548)))), 1.0 / ((1.0 - p.p1596)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (!(s.v[1556] != 0.0))) {
            s.store_ad(1549, &A::mul(A::mul(A::neg(s.ad_value(269)), s.ad_value(523)), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) {
            s.store_sub_from_scalar_ad(1547, 1.0, A::div(A::sub(s.ad_value(521), s.ad_value(557)), s.ad_value(558)));
        }

        s.v[1558] = if (p.p1608 != 1.0) { 1.0 } else { 0.0 };

        s.v[1559] = if (p.p1608 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (s.v[1558] != 0.0)) && (s.v[1559] != 0.0)) {
            s.store_div_from_scalar_ad(1548, 1.0, A::sqrt(s.ad_value(1547)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (s.v[1558] != 0.0)) && (!(s.v[1559] != 0.0))) {
            s.store_powf(1548, 1547, (-p.p1608));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (s.v[1558] != 0.0)) {
            s.store_add_ad_rhs(530, 1549, A::scale(A::mul(A::mul(A::scale(s.ad_value(558), p.p1602), s.ad_value(523)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548)))), 1.0 / ((1.0 - p.p1608))));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) && (!(s.v[1558] != 0.0))) {
            s.store_sub_ad_rhs(530, 1549, A::mul(A::mul(A::scale(s.ad_value(558), p.p1602), s.ad_value(523)), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (!(s.v[1552] != 0.0))) {
            s.store_sub_from_scalar(1547, 1.0, 1542);
        }

        s.v[1560] = if (p.p1596 != 1.0) { 1.0 } else { 0.0 };

        s.v[1561] = if (p.p1596 == 0.5) { 1.0 } else { 0.0 };

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (!(s.v[1552] != 0.0))) && (s.v[1560] != 0.0)) && (s.v[1561] != 0.0)) {
            s.store_div_from_scalar_ad(1548, 1.0, A::sqrt(s.ad_value(1547)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (!(s.v[1552] != 0.0))) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (!(s.v[1552] != 0.0))) && (s.v[1560] != 0.0)) {
            s.store_scale_ad(530, A::mul(A::mul(s.ad_value(269), s.ad_value(523)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548)))), 1.0 / ((1.0 - p.p1596)));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (s.v[1551] != 0.0)) && (!(s.v[1552] != 0.0))) && (!(s.v[1560] != 0.0))) {
            s.store_ad(530, &A::mul(A::mul(A::neg(s.ad_value(269)), s.ad_value(523)), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        s.v[1562] = if (p.p1596 != 1.0) { 1.0 } else { 0.0 };

        s.v[1563] = if (p.p1596 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) && (s.v[1562] != 0.0)) && (s.v[1563] != 0.0)) {
            s.store_scalar(1543, (1.0 / ((0.1) as f64).sqrt()));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) && (s.v[1562] != 0.0)) && (!(s.v[1563] != 0.0))) {
            s.store_scalar(1543, ((0.1) as f64).powf((-p.p1596)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) && (s.v[1562] != 0.0)) {
            s.store_scalar(1544, (1.0 / (1.0 - p.p1596)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) && (s.v[1562] != 0.0)) {
            s.store_mul_ad_rhs(1546, 1544, A::sub_from_scalar(1.0, A::scale(s.ad_value(1543), ((0.05 * p.p1596) * (1.0 + p.p1596)))));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) && (!(s.v[1562] != 0.0))) {
            s.store_scalar(1543, 10.0);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) && (!(s.v[1562] != 0.0))) {
            s.store_scalar(1546, (1.5 - ((0.1) as f64).ln()));
        }

        if (((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) {
            s.store_mul_ad(1545, A::mul(s.ad_value(1543), A::offset(s.ad_value(1542), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(1542), (-1.0)), (5.0 * p.p1596)), (1.0 + p.p1596)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1550] != 0.0)) && (!(s.v[1551] != 0.0))) {
            s.store_mul_ad(530, A::mul(s.ad_value(269), s.ad_value(523)), A::add(s.ad_value(1545), s.ad_value(1546)));
        }

    }

    pub(super) fn stamp_reactive_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1523] != 0.0) && (!(s.v[1550] != 0.0))) {
            s.store_scalar(530, 0.0);
        }

        s.v[1572] = if (s.v[524] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) {
            s.store_div(1564, 521, 270);
        }

        s.v[1573] = if (s.v[1564] < 0.9) { 1.0 } else { 0.0 };

        s.v[1574] = if (p.p1604 > 0.0) { 1.0 } else { 0.0 };

        s.v[1575] = if (s.v[521] > s.v[559]) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) {
            s.store_sub_from_scalar(1569, 1.0, 1564);
        }

        s.v[1576] = if (p.p1598 != 1.0) { 1.0 } else { 0.0 };

        s.v[1577] = if (p.p1598 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) && (s.v[1576] != 0.0)) && (s.v[1577] != 0.0)) {
            s.store_div_from_scalar_ad(1570, 1.0, A::sqrt(s.ad_value(1569)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) && (s.v[1576] != 0.0)) && (!(s.v[1577] != 0.0))) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) && (s.v[1576] != 0.0)) {
            s.store_scale_ad(531, A::mul(A::mul(s.ad_value(270), s.ad_value(524)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570)))), 1.0 / ((1.0 - p.p1598)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) && (!(s.v[1576] != 0.0))) {
            s.store_ad(531, &A::mul(A::mul(A::neg(s.ad_value(270)), s.ad_value(524)), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) {
            s.store_sub_from_scalar_ad(1569, 1.0, A::div(s.ad_value(559), s.ad_value(270)));
        }

        s.v[1578] = if (p.p1598 != 1.0) { 1.0 } else { 0.0 };

        s.v[1579] = if (p.p1598 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1578] != 0.0)) && (s.v[1579] != 0.0)) {
            s.store_div_from_scalar_ad(1570, 1.0, A::sqrt(s.ad_value(1569)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1578] != 0.0)) && (!(s.v[1579] != 0.0))) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1578] != 0.0)) {
            s.store_scale_ad(1571, A::mul(A::mul(s.ad_value(270), s.ad_value(524)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570)))), 1.0 / ((1.0 - p.p1598)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1578] != 0.0))) {
            s.store_ad(1571, &A::mul(A::mul(A::neg(s.ad_value(270)), s.ad_value(524)), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) {
            s.store_sub_from_scalar_ad(1569, 1.0, A::div(A::sub(s.ad_value(521), s.ad_value(559)), s.ad_value(560)));
        }

        s.v[1580] = if (p.p1610 != 1.0) { 1.0 } else { 0.0 };

        s.v[1581] = if (p.p1610 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1580] != 0.0)) && (s.v[1581] != 0.0)) {
            s.store_div_from_scalar_ad(1570, 1.0, A::sqrt(s.ad_value(1569)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1580] != 0.0)) && (!(s.v[1581] != 0.0))) {
            s.store_powf(1570, 1569, (-p.p1610));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1580] != 0.0)) {
            s.store_add_ad_rhs(531, 1571, A::scale(A::mul(A::mul(A::scale(s.ad_value(560), p.p1604), s.ad_value(524)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570)))), 1.0 / ((1.0 - p.p1610))));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1580] != 0.0))) {
            s.store_sub_ad_rhs(531, 1571, A::mul(A::mul(A::scale(s.ad_value(560), p.p1604), s.ad_value(524)), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (!(s.v[1574] != 0.0))) {
            s.store_sub_from_scalar(1569, 1.0, 1564);
        }

        s.v[1582] = if (p.p1598 != 1.0) { 1.0 } else { 0.0 };

        s.v[1583] = if (p.p1598 == 0.5) { 1.0 } else { 0.0 };

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (!(s.v[1574] != 0.0))) && (s.v[1582] != 0.0)) && (s.v[1583] != 0.0)) {
            s.store_div_from_scalar_ad(1570, 1.0, A::sqrt(s.ad_value(1569)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (!(s.v[1574] != 0.0))) && (s.v[1582] != 0.0)) && (!(s.v[1583] != 0.0))) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (!(s.v[1574] != 0.0))) && (s.v[1582] != 0.0)) {
            s.store_scale_ad(531, A::mul(A::mul(s.ad_value(270), s.ad_value(524)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570)))), 1.0 / ((1.0 - p.p1598)));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (s.v[1573] != 0.0)) && (!(s.v[1574] != 0.0))) && (!(s.v[1582] != 0.0))) {
            s.store_ad(531, &A::mul(A::mul(A::neg(s.ad_value(270)), s.ad_value(524)), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        s.v[1584] = if (p.p1598 != 1.0) { 1.0 } else { 0.0 };

        s.v[1585] = if (p.p1598 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) && (s.v[1584] != 0.0)) && (s.v[1585] != 0.0)) {
            s.store_scalar(1565, (1.0 / ((0.1) as f64).sqrt()));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) && (s.v[1584] != 0.0)) && (!(s.v[1585] != 0.0))) {
            s.store_scalar(1565, ((0.1) as f64).powf((-p.p1598)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) && (s.v[1584] != 0.0)) {
            s.store_scalar(1566, (1.0 / (1.0 - p.p1598)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) && (s.v[1584] != 0.0)) {
            s.store_mul_ad_rhs(1568, 1566, A::sub_from_scalar(1.0, A::scale(s.ad_value(1565), ((0.05 * p.p1598) * (1.0 + p.p1598)))));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) && (!(s.v[1584] != 0.0))) {
            s.store_scalar(1565, 10.0);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) && (!(s.v[1584] != 0.0))) {
            s.store_scalar(1568, (1.5 - ((0.1) as f64).ln()));
        }

        if (((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) {
            s.store_mul_ad(1567, A::mul(s.ad_value(1565), A::offset(s.ad_value(1564), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(1564), (-1.0)), (5.0 * p.p1598)), (1.0 + p.p1598)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1572] != 0.0)) && (!(s.v[1573] != 0.0))) {
            s.store_mul_ad(531, A::mul(s.ad_value(270), s.ad_value(524)), A::add(s.ad_value(1567), s.ad_value(1568)));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1572] != 0.0))) {
            s.store_scalar(531, 0.0);
        }

        s.v[1594] = if (s.v[525] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_div(1586, 521, 271);
        }

        s.v[1595] = if (s.v[1586] < 0.9) { 1.0 } else { 0.0 };

        s.v[1596] = if (p.p1606 > 0.0) { 1.0 } else { 0.0 };

        s.v[1597] = if (s.v[521] > s.v[561]) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (s.v[1597] != 0.0)) {
            s.store_sub_from_scalar(1591, 1.0, 1586);
        }

        s.v[1598] = if (p.p1600 != 1.0) { 1.0 } else { 0.0 };

        s.v[1599] = if (p.p1600 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (s.v[1597] != 0.0)) && (s.v[1598] != 0.0)) && (s.v[1599] != 0.0)) {
            s.store_div_from_scalar_ad(1592, 1.0, A::sqrt(s.ad_value(1591)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (s.v[1597] != 0.0)) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (s.v[1597] != 0.0)) && (s.v[1598] != 0.0)) {
            s.store_scale_ad(532, A::mul(A::mul(s.ad_value(271), s.ad_value(525)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592)))), 1.0 / ((1.0 - p.p1600)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (s.v[1597] != 0.0)) && (!(s.v[1598] != 0.0))) {
            s.store_ad(532, &A::mul(A::mul(A::neg(s.ad_value(271)), s.ad_value(525)), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) {
            s.store_sub_from_scalar_ad(1591, 1.0, A::div(s.ad_value(561), s.ad_value(271)));
        }

        s.v[1600] = if (p.p1600 != 1.0) { 1.0 } else { 0.0 };

        s.v[1601] = if (p.p1600 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (s.v[1600] != 0.0)) && (s.v[1601] != 0.0)) {
            s.store_div_from_scalar_ad(1592, 1.0, A::sqrt(s.ad_value(1591)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (s.v[1600] != 0.0)) && (!(s.v[1601] != 0.0))) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (s.v[1600] != 0.0)) {
            s.store_scale_ad(1593, A::mul(A::mul(s.ad_value(271), s.ad_value(525)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592)))), 1.0 / ((1.0 - p.p1600)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_ad(1593, &A::mul(A::mul(A::neg(s.ad_value(271)), s.ad_value(525)), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) {
            s.store_sub_from_scalar_ad(1591, 1.0, A::div(A::sub(s.ad_value(521), s.ad_value(561)), s.ad_value(562)));
        }

        s.v[1602] = if (p.p1612 != 1.0) { 1.0 } else { 0.0 };

        s.v[1603] = if (p.p1612 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (s.v[1602] != 0.0)) && (s.v[1603] != 0.0)) {
            s.store_div_from_scalar_ad(1592, 1.0, A::sqrt(s.ad_value(1591)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (s.v[1602] != 0.0)) && (!(s.v[1603] != 0.0))) {
            s.store_powf(1592, 1591, (-p.p1612));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (s.v[1602] != 0.0)) {
            s.store_add_ad_rhs(532, 1593, A::scale(A::mul(A::mul(A::scale(s.ad_value(562), p.p1606), s.ad_value(525)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592)))), 1.0 / ((1.0 - p.p1612))));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (s.v[1596] != 0.0)) && (!(s.v[1597] != 0.0))) && (!(s.v[1602] != 0.0))) {
            s.store_sub_ad_rhs(532, 1593, A::mul(A::mul(A::scale(s.ad_value(562), p.p1606), s.ad_value(525)), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (!(s.v[1596] != 0.0))) {
            s.store_sub_from_scalar(1591, 1.0, 1586);
        }

        s.v[1604] = if (p.p1600 != 1.0) { 1.0 } else { 0.0 };

        s.v[1605] = if (p.p1600 == 0.5) { 1.0 } else { 0.0 };

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (!(s.v[1596] != 0.0))) && (s.v[1604] != 0.0)) && (s.v[1605] != 0.0)) {
            s.store_div_from_scalar_ad(1592, 1.0, A::sqrt(s.ad_value(1591)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (!(s.v[1596] != 0.0))) && (s.v[1604] != 0.0)) && (!(s.v[1605] != 0.0))) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (!(s.v[1596] != 0.0))) && (s.v[1604] != 0.0)) {
            s.store_scale_ad(532, A::mul(A::mul(s.ad_value(271), s.ad_value(525)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592)))), 1.0 / ((1.0 - p.p1600)));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) && (!(s.v[1596] != 0.0))) && (!(s.v[1604] != 0.0))) {
            s.store_ad(532, &A::mul(A::mul(A::neg(s.ad_value(271)), s.ad_value(525)), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        s.v[1606] = if (p.p1600 != 1.0) { 1.0 } else { 0.0 };

        s.v[1607] = if (p.p1600 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (s.v[1606] != 0.0)) && (s.v[1607] != 0.0)) {
            s.store_scalar(1587, (1.0 / ((0.1) as f64).sqrt()));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (s.v[1606] != 0.0)) && (!(s.v[1607] != 0.0))) {
            s.store_scalar(1587, ((0.1) as f64).powf((-p.p1600)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_scalar(1588, (1.0 / (1.0 - p.p1600)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (s.v[1606] != 0.0)) {
            s.store_mul_ad_rhs(1590, 1588, A::sub_from_scalar(1.0, A::scale(s.ad_value(1587), ((0.05 * p.p1600) * (1.0 + p.p1600)))));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_scalar(1587, 10.0);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1606] != 0.0))) {
            s.store_scalar(1590, (1.5 - ((0.1) as f64).ln()));
        }

        if (((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) {
            s.store_mul_ad(1589, A::mul(s.ad_value(1587), A::offset(s.ad_value(1586), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(1586), (-1.0)), (5.0 * p.p1600)), (1.0 + p.p1600)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) {
            s.store_mul_ad(532, A::mul(s.ad_value(271), s.ad_value(525)), A::add(s.ad_value(1589), s.ad_value(1590)));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_scalar(532, 0.0);
        }

        if (s.v[1523] != 0.0) {
            s.store_add_ad_lhs(529, A::add(s.ad_value(530), s.ad_value(531)), 532);
        }

        s.v[1616] = if (s.v[526] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) {
            s.store_div(1608, 522, 272);
        }

        s.v[1617] = if (s.v[1608] < 0.9) { 1.0 } else { 0.0 };

        s.v[1618] = if (p.p1603 > 0.0) { 1.0 } else { 0.0 };

        s.v[1619] = if (s.v[522] > s.v[563]) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (s.v[1619] != 0.0)) {
            s.store_sub_from_scalar(1613, 1.0, 1608);
        }

        s.v[1620] = if (p.p1597 != 1.0) { 1.0 } else { 0.0 };

        s.v[1621] = if (p.p1597 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (s.v[1619] != 0.0)) && (s.v[1620] != 0.0)) && (s.v[1621] != 0.0)) {
            s.store_div_from_scalar_ad(1614, 1.0, A::sqrt(s.ad_value(1613)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (s.v[1619] != 0.0)) && (s.v[1620] != 0.0)) && (!(s.v[1621] != 0.0))) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (s.v[1619] != 0.0)) && (s.v[1620] != 0.0)) {
            s.store_scale_ad(534, A::mul(A::mul(s.ad_value(272), s.ad_value(526)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614)))), 1.0 / ((1.0 - p.p1597)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (s.v[1619] != 0.0)) && (!(s.v[1620] != 0.0))) {
            s.store_ad(534, &A::mul(A::mul(A::neg(s.ad_value(272)), s.ad_value(526)), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) {
            s.store_sub_from_scalar_ad(1613, 1.0, A::div(s.ad_value(563), s.ad_value(272)));
        }

        s.v[1622] = if (p.p1597 != 1.0) { 1.0 } else { 0.0 };

        s.v[1623] = if (p.p1597 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (s.v[1622] != 0.0)) && (s.v[1623] != 0.0)) {
            s.store_div_from_scalar_ad(1614, 1.0, A::sqrt(s.ad_value(1613)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (s.v[1622] != 0.0)) && (!(s.v[1623] != 0.0))) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (s.v[1622] != 0.0)) {
            s.store_scale_ad(1615, A::mul(A::mul(s.ad_value(272), s.ad_value(526)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614)))), 1.0 / ((1.0 - p.p1597)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (!(s.v[1622] != 0.0))) {
            s.store_ad(1615, &A::mul(A::mul(A::neg(s.ad_value(272)), s.ad_value(526)), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) {
            s.store_sub_from_scalar_ad(1613, 1.0, A::div(A::sub(s.ad_value(522), s.ad_value(563)), s.ad_value(564)));
        }

        s.v[1624] = if (p.p1609 != 1.0) { 1.0 } else { 0.0 };

        s.v[1625] = if (p.p1609 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (s.v[1624] != 0.0)) && (s.v[1625] != 0.0)) {
            s.store_div_from_scalar_ad(1614, 1.0, A::sqrt(s.ad_value(1613)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (s.v[1624] != 0.0)) && (!(s.v[1625] != 0.0))) {
            s.store_powf(1614, 1613, (-p.p1609));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (s.v[1624] != 0.0)) {
            s.store_add_ad_rhs(534, 1615, A::scale(A::mul(A::mul(A::scale(s.ad_value(564), p.p1603), s.ad_value(526)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614)))), 1.0 / ((1.0 - p.p1609))));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (s.v[1618] != 0.0)) && (!(s.v[1619] != 0.0))) && (!(s.v[1624] != 0.0))) {
            s.store_sub_ad_rhs(534, 1615, A::mul(A::mul(A::scale(s.ad_value(564), p.p1603), s.ad_value(526)), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) {
            s.store_sub_from_scalar(1613, 1.0, 1608);
        }

        s.v[1626] = if (p.p1597 != 1.0) { 1.0 } else { 0.0 };

        s.v[1627] = if (p.p1597 == 0.5) { 1.0 } else { 0.0 };

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1626] != 0.0)) && (s.v[1627] != 0.0)) {
            s.store_div_from_scalar_ad(1614, 1.0, A::sqrt(s.ad_value(1613)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1626] != 0.0)) && (!(s.v[1627] != 0.0))) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (s.v[1626] != 0.0)) {
            s.store_scale_ad(534, A::mul(A::mul(s.ad_value(272), s.ad_value(526)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614)))), 1.0 / ((1.0 - p.p1597)));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (s.v[1617] != 0.0)) && (!(s.v[1618] != 0.0))) && (!(s.v[1626] != 0.0))) {
            s.store_ad(534, &A::mul(A::mul(A::neg(s.ad_value(272)), s.ad_value(526)), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        s.v[1628] = if (p.p1597 != 1.0) { 1.0 } else { 0.0 };

        s.v[1629] = if (p.p1597 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) && (s.v[1628] != 0.0)) && (s.v[1629] != 0.0)) {
            s.store_scalar(1609, (1.0 / ((0.1) as f64).sqrt()));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) && (s.v[1628] != 0.0)) && (!(s.v[1629] != 0.0))) {
            s.store_scalar(1609, ((0.1) as f64).powf((-p.p1597)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_scalar(1610, (1.0 / (1.0 - p.p1597)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) && (s.v[1628] != 0.0)) {
            s.store_mul_ad_rhs(1612, 1610, A::sub_from_scalar(1.0, A::scale(s.ad_value(1609), ((0.05 * p.p1597) * (1.0 + p.p1597)))));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scalar(1609, 10.0);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) && (!(s.v[1628] != 0.0))) {
            s.store_scalar(1612, (1.5 - ((0.1) as f64).ln()));
        }

        if (((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) {
            s.store_mul_ad(1611, A::mul(s.ad_value(1609), A::offset(s.ad_value(1608), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(1608), (-1.0)), (5.0 * p.p1597)), (1.0 + p.p1597)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1616] != 0.0)) && (!(s.v[1617] != 0.0))) {
            s.store_mul_ad(534, A::mul(s.ad_value(272), s.ad_value(526)), A::add(s.ad_value(1611), s.ad_value(1612)));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1616] != 0.0))) {
            s.store_scalar(534, 0.0);
        }

        s.v[1638] = if (s.v[527] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) {
            s.store_div(1630, 522, 273);
        }

        s.v[1639] = if (s.v[1630] < 0.9) { 1.0 } else { 0.0 };

        s.v[1640] = if (p.p1605 > 0.0) { 1.0 } else { 0.0 };

        s.v[1641] = if (s.v[522] > s.v[565]) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (s.v[1641] != 0.0)) {
            s.store_sub_from_scalar(1635, 1.0, 1630);
        }

        s.v[1642] = if (p.p1599 != 1.0) { 1.0 } else { 0.0 };

        s.v[1643] = if (p.p1599 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (s.v[1641] != 0.0)) && (s.v[1642] != 0.0)) && (s.v[1643] != 0.0)) {
            s.store_div_from_scalar_ad(1636, 1.0, A::sqrt(s.ad_value(1635)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (s.v[1641] != 0.0)) && (s.v[1642] != 0.0)) && (!(s.v[1643] != 0.0))) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (s.v[1641] != 0.0)) && (s.v[1642] != 0.0)) {
            s.store_scale_ad(535, A::mul(A::mul(s.ad_value(273), s.ad_value(527)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636)))), 1.0 / ((1.0 - p.p1599)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (s.v[1641] != 0.0)) && (!(s.v[1642] != 0.0))) {
            s.store_ad(535, &A::mul(A::mul(A::neg(s.ad_value(273)), s.ad_value(527)), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) {
            s.store_sub_from_scalar_ad(1635, 1.0, A::div(s.ad_value(565), s.ad_value(273)));
        }

        s.v[1644] = if (p.p1599 != 1.0) { 1.0 } else { 0.0 };

        s.v[1645] = if (p.p1599 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (s.v[1644] != 0.0)) && (s.v[1645] != 0.0)) {
            s.store_div_from_scalar_ad(1636, 1.0, A::sqrt(s.ad_value(1635)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (s.v[1644] != 0.0)) && (!(s.v[1645] != 0.0))) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (s.v[1644] != 0.0)) {
            s.store_scale_ad(1637, A::mul(A::mul(s.ad_value(273), s.ad_value(527)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636)))), 1.0 / ((1.0 - p.p1599)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (!(s.v[1644] != 0.0))) {
            s.store_ad(1637, &A::mul(A::mul(A::neg(s.ad_value(273)), s.ad_value(527)), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) {
            s.store_sub_from_scalar_ad(1635, 1.0, A::div(A::sub(s.ad_value(522), s.ad_value(565)), s.ad_value(566)));
        }

        s.v[1646] = if (p.p1611 != 1.0) { 1.0 } else { 0.0 };

        s.v[1647] = if (p.p1611 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (s.v[1646] != 0.0)) && (s.v[1647] != 0.0)) {
            s.store_div_from_scalar_ad(1636, 1.0, A::sqrt(s.ad_value(1635)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (s.v[1646] != 0.0)) && (!(s.v[1647] != 0.0))) {
            s.store_powf(1636, 1635, (-p.p1611));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (s.v[1646] != 0.0)) {
            s.store_add_ad_rhs(535, 1637, A::scale(A::mul(A::mul(A::scale(s.ad_value(566), p.p1605), s.ad_value(527)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636)))), 1.0 / ((1.0 - p.p1611))));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) && (!(s.v[1641] != 0.0))) && (!(s.v[1646] != 0.0))) {
            s.store_sub_ad_rhs(535, 1637, A::mul(A::mul(A::scale(s.ad_value(566), p.p1605), s.ad_value(527)), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) {
            s.store_sub_from_scalar(1635, 1.0, 1630);
        }

        s.v[1648] = if (p.p1599 != 1.0) { 1.0 } else { 0.0 };

        s.v[1649] = if (p.p1599 == 0.5) { 1.0 } else { 0.0 };

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) && (s.v[1648] != 0.0)) && (s.v[1649] != 0.0)) {
            s.store_div_from_scalar_ad(1636, 1.0, A::sqrt(s.ad_value(1635)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) && (s.v[1648] != 0.0)) && (!(s.v[1649] != 0.0))) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) && (s.v[1648] != 0.0)) {
            s.store_scale_ad(535, A::mul(A::mul(s.ad_value(273), s.ad_value(527)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636)))), 1.0 / ((1.0 - p.p1599)));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) && (!(s.v[1648] != 0.0))) {
            s.store_ad(535, &A::mul(A::mul(A::neg(s.ad_value(273)), s.ad_value(527)), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        s.v[1650] = if (p.p1599 != 1.0) { 1.0 } else { 0.0 };

        s.v[1651] = if (p.p1599 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) && (s.v[1650] != 0.0)) && (s.v[1651] != 0.0)) {
            s.store_scalar(1631, (1.0 / ((0.1) as f64).sqrt()));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) && (s.v[1650] != 0.0)) && (!(s.v[1651] != 0.0))) {
            s.store_scalar(1631, ((0.1) as f64).powf((-p.p1599)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_scalar(1632, (1.0 / (1.0 - p.p1599)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) && (s.v[1650] != 0.0)) {
            s.store_mul_ad_rhs(1634, 1632, A::sub_from_scalar(1.0, A::scale(s.ad_value(1631), ((0.05 * p.p1599) * (1.0 + p.p1599)))));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) && (!(s.v[1650] != 0.0))) {
            s.store_scalar(1631, 10.0);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) && (!(s.v[1650] != 0.0))) {
            s.store_scalar(1634, (1.5 - ((0.1) as f64).ln()));
        }

        if (((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) {
            s.store_mul_ad(1633, A::mul(s.ad_value(1631), A::offset(s.ad_value(1630), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(1630), (-1.0)), (5.0 * p.p1599)), (1.0 + p.p1599)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1638] != 0.0)) && (!(s.v[1639] != 0.0))) {
            s.store_mul_ad(535, A::mul(s.ad_value(273), s.ad_value(527)), A::add(s.ad_value(1633), s.ad_value(1634)));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1638] != 0.0))) {
            s.store_scalar(535, 0.0);
        }

        s.v[1660] = if (s.v[528] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) {
            s.store_div(1652, 522, 274);
        }

        s.v[1661] = if (s.v[1652] < 0.9) { 1.0 } else { 0.0 };

        s.v[1662] = if (p.p1607 > 0.0) { 1.0 } else { 0.0 };

        s.v[1663] = if (s.v[522] > s.v[567]) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (s.v[1663] != 0.0)) {
            s.store_sub_from_scalar(1657, 1.0, 1652);
        }

        s.v[1664] = if (p.p1601 != 1.0) { 1.0 } else { 0.0 };

        s.v[1665] = if (p.p1601 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (s.v[1663] != 0.0)) && (s.v[1664] != 0.0)) && (s.v[1665] != 0.0)) {
            s.store_div_from_scalar_ad(1658, 1.0, A::sqrt(s.ad_value(1657)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (s.v[1663] != 0.0)) && (s.v[1664] != 0.0)) && (!(s.v[1665] != 0.0))) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (s.v[1663] != 0.0)) && (s.v[1664] != 0.0)) {
            s.store_scale_ad(536, A::mul(A::mul(s.ad_value(274), s.ad_value(528)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658)))), 1.0 / ((1.0 - p.p1601)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (s.v[1663] != 0.0)) && (!(s.v[1664] != 0.0))) {
            s.store_ad(536, &A::mul(A::mul(A::neg(s.ad_value(274)), s.ad_value(528)), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) {
            s.store_sub_from_scalar_ad(1657, 1.0, A::div(s.ad_value(567), s.ad_value(274)));
        }

        s.v[1666] = if (p.p1601 != 1.0) { 1.0 } else { 0.0 };

        s.v[1667] = if (p.p1601 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (s.v[1667] != 0.0)) {
            s.store_div_from_scalar_ad(1658, 1.0, A::sqrt(s.ad_value(1657)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) && (!(s.v[1667] != 0.0))) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (s.v[1666] != 0.0)) {
            s.store_scale_ad(1659, A::mul(A::mul(s.ad_value(274), s.ad_value(528)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658)))), 1.0 / ((1.0 - p.p1601)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (!(s.v[1666] != 0.0))) {
            s.store_ad(1659, &A::mul(A::mul(A::neg(s.ad_value(274)), s.ad_value(528)), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) {
            s.store_sub_from_scalar_ad(1657, 1.0, A::div(A::sub(s.ad_value(522), s.ad_value(567)), s.ad_value(568)));
        }

        s.v[1668] = if (p.p1613 != 1.0) { 1.0 } else { 0.0 };

        s.v[1669] = if (p.p1613 == 0.5) { 1.0 } else { 0.0 };

        if (((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (s.v[1668] != 0.0)) && (s.v[1669] != 0.0)) {
            s.store_div_from_scalar_ad(1658, 1.0, A::sqrt(s.ad_value(1657)));
        }

        if (((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (s.v[1668] != 0.0)) && (!(s.v[1669] != 0.0))) {
            s.store_powf(1658, 1657, (-p.p1613));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (s.v[1668] != 0.0)) {
            s.store_add_ad_rhs(536, 1659, A::scale(A::mul(A::mul(A::scale(s.ad_value(568), p.p1607), s.ad_value(528)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658)))), 1.0 / ((1.0 - p.p1613))));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (s.v[1662] != 0.0)) && (!(s.v[1663] != 0.0))) && (!(s.v[1668] != 0.0))) {
            s.store_sub_ad_rhs(536, 1659, A::mul(A::mul(A::scale(s.ad_value(568), p.p1607), s.ad_value(528)), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (!(s.v[1662] != 0.0))) {
            s.store_sub_from_scalar(1657, 1.0, 1652);
        }

        s.v[1670] = if (p.p1601 != 1.0) { 1.0 } else { 0.0 };

        s.v[1671] = if (p.p1601 == 0.5) { 1.0 } else { 0.0 };

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (!(s.v[1662] != 0.0))) && (s.v[1670] != 0.0)) && (s.v[1671] != 0.0)) {
            s.store_div_from_scalar_ad(1658, 1.0, A::sqrt(s.ad_value(1657)));
        }

        if ((((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (!(s.v[1662] != 0.0))) && (s.v[1670] != 0.0)) && (!(s.v[1671] != 0.0))) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (!(s.v[1662] != 0.0))) && (s.v[1670] != 0.0)) {
            s.store_scale_ad(536, A::mul(A::mul(s.ad_value(274), s.ad_value(528)), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658)))), 1.0 / ((1.0 - p.p1601)));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (s.v[1661] != 0.0)) && (!(s.v[1662] != 0.0))) && (!(s.v[1670] != 0.0))) {
            s.store_ad(536, &A::mul(A::mul(A::neg(s.ad_value(274)), s.ad_value(528)), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        s.v[1672] = if (p.p1601 != 1.0) { 1.0 } else { 0.0 };

        s.v[1673] = if (p.p1601 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) && (s.v[1672] != 0.0)) && (s.v[1673] != 0.0)) {
            s.store_scalar(1653, (1.0 / ((0.1) as f64).sqrt()));
        }

        if (((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) && (s.v[1672] != 0.0)) && (!(s.v[1673] != 0.0))) {
            s.store_scalar(1653, ((0.1) as f64).powf((-p.p1601)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) && (s.v[1672] != 0.0)) {
            s.store_scalar(1654, (1.0 / (1.0 - p.p1601)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) && (s.v[1672] != 0.0)) {
            s.store_mul_ad_rhs(1656, 1654, A::sub_from_scalar(1.0, A::scale(s.ad_value(1653), ((0.05 * p.p1601) * (1.0 + p.p1601)))));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) && (!(s.v[1672] != 0.0))) {
            s.store_scalar(1653, 10.0);
        }

        if ((((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) && (!(s.v[1672] != 0.0))) {
            s.store_scalar(1656, (1.5 - ((0.1) as f64).ln()));
        }

        if (((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) {
            s.store_mul_ad(1655, A::mul(s.ad_value(1653), A::offset(s.ad_value(1652), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(1652), (-1.0)), (5.0 * p.p1601)), (1.0 + p.p1601)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1660] != 0.0)) && (!(s.v[1661] != 0.0))) {
            s.store_mul_ad(536, A::mul(s.ad_value(274), s.ad_value(528)), A::add(s.ad_value(1655), s.ad_value(1656)));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1660] != 0.0))) {
            s.store_scalar(536, 0.0);
        }

        if (s.v[1523] != 0.0) {
            s.store_add_ad_lhs(533, A::add(s.ad_value(534), s.ad_value(535)), 536);
        }

        s.store_add_ad_rhs(507, 529, A::scale(s.ad_value(521), s.v[515]));

        s.store_add_ad_rhs(508, 533, A::scale(s.ad_value(522), s.v[516]));

        s.store_ad(509, &A::mul(A::mul(s.ad_value(517), s.ad_value(114)), A::voltage(ctx, &nodes, Some(3), Some(10))));

        s.v[1674] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1674] != 0.0) {
            s.store_mul_ad_rhs(170, 114, A::voltage(ctx, &nodes, Some(10), Some(3)));
        }

        if (s.v[1674] != 0.0) {
            s.store_offset_ad(171, A::add(A::add(A::sub(s.ad_value(170), s.ad_value(167)), A::scale(s.ad_value(146), 0.5)), s.ad_value(166)), (-p.p1529));
        }

        if (s.v[1674] != 0.0) {
            s.store_offset(168, 171, 0.02);
        }

        if (s.v[1674] != 0.0) {
            s.store_scale_ad(512, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02)))), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1674] != 0.0) {
            s.store_sub_ad_rhs(509, 509, A::mul(A::scale(s.ad_value(156), s.v[115]), A::mul(s.ad_value(650), A::add(A::sub(s.ad_value(171), s.ad_value(512)), A::mul(A::scale(s.ad_value(653), 0.5), A::offset(A::sqrt(A::offset(A::div(A::scale(s.ad_value(512), 4.0), s.ad_value(653)), 1.0)), (-1.0)))))));
        }

        s.store_mul_ad_rhs(169, 126, A::add(s.ad_value(865), A::mul(A::mul(s.ad_value(866), s.ad_value(126)), s.ad_value(126))));

        s.store_div_ad_lhs(168, A::mul(A::mul(s.ad_value(415), s.ad_value(372)), s.ad_value(158)), 153);

        s.store_div_ad_lhs(579, A::scale(s.ad_value(428), 2.0), 415);

        s.v[1678] = if (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1678] != 0.0) {
            s.store_offset(580, 153, (-(2.0 * p.p1687)));
        }

        s.v[1679] = if (s.v[580] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1678] != 0.0) && (s.v[1679] != 0.0)) {
            s.copy_ad(580, 153);
        }

        s.v[1680] = if ((p.p79 == 1.0) || (p.p79 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_square(581, 580);
        }

        s.v[1681] = if (p.p1681 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1681] != 0.0)) {
            s.store_div_ad_lhs(168, A::offset(A::scale(s.ad_value(202), 1.0 / (s.v[578])), p.p1681), 579);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1681] != 0.0)) {
            s.store_ad(582, &A::scale({
                if (!(s.v[168] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[168] > 1e-38) {
                            A::ln(s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.v[578]));
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1681] != 0.0))) {
            s.store_scalar(582, 0.0);
        }

        s.v[1682] = if (p.p79 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1682] != 0.0)) {
            s.store_div(169, 400, 576);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1682] != 0.0)) {
            s.store_offset_ad(170, A::pow(s.ad_value(169), s.ad_value(575)), 1.0);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1682] != 0.0)) {
            s.store_div(171, 574, 170);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1682] != 0.0)) {
            s.store_scale(172, 171, 1.0 / (p.p1682));
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1682] != 0.0)) {
            s.store_scale_ad(174, A::add(A::offset(s.ad_value(172), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(172), (-1.0)), A::offset(s.ad_value(172), (-1.0))), ((0.25 * p.p1688) * p.p1688)))), 0.5);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1682] != 0.0)) {
            s.store_scale(573, 174, p.p1682);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1682] != 0.0))) {
            s.store_scalar(573, p.p1682);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad_lhs(169, A::mul(A::scale(s.ad_value(179), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(124))), 415);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(372), 10000000000.0), 581);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_scaled_mul(583, 372, 392, 6.241457005723417e18);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_scaled_mul(584, 372, 393, 6.241457005723417e18);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad(585, A::scale(s.ad_value(179), 6.241457005723417e18), A::add(s.ad_value(372), s.ad_value(669)));
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            let assign32970_ad_e55346: A = {
                if (!(((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38) {
                            A::ln(A::div(A::add(s.ad_value(583), s.ad_value(585)), A::add(s.ad_value(584), s.ad_value(585))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(171, 573, assign32970_ad_e55346);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_scaled_sub(172, 583, 584, p.p1683);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_scale_ad(174, A::sub(A::square(s.ad_value(583)), A::square(s.ad_value(584))), (0.5 * p.p1684));
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad_lhs(175, A::mul(A::scale(s.ad_value(179), 1.60219e-19), s.ad_value(124)), 124);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_scale_ad(176, A::mul(A::scale(s.ad_value(581), 10000000000.0), s.ad_value(158)), s.v[115]);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_add_ad(177, A::add(s.ad_value(573), A::scale(s.ad_value(584), p.p1683)), A::mul(A::scale(s.ad_value(584), p.p1684), s.ad_value(584)));
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad(178, A::add(s.ad_value(584), s.ad_value(585)), A::add(s.ad_value(584), s.ad_value(585)));
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_add_ad(586, A::mul(A::div(s.ad_value(169), s.ad_value(170)), A::add(A::add(s.ad_value(171), s.ad_value(172)), s.ad_value(174))), A::div(A::mul(A::mul(A::div(s.ad_value(175), s.ad_value(176)), s.ad_value(582)), s.ad_value(177)), s.ad_value(178)));
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad_lhs(340, A::scale(s.ad_value(573), 1.60219e-19), 179);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad_lhs(341, A::mul(A::scale(A::mul(A::scale(s.ad_value(158), s.v[115]), s.ad_value(580)), 10000000000.0), s.ad_value(585)), 585);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_mul_ad_lhs(587, A::mul(A::div(s.ad_value(340), s.ad_value(341)), s.ad_value(124)), 124);
        }

        if ((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) {
            s.store_add(169, 587, 586);
        }

        s.v[1684] = if (p.p79 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_div(169, 400, 576);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_offset_ad(170, A::pow(s.ad_value(169), s.ad_value(575)), 1.0);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_div(171, 574, 170);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_scale(172, 171, 1.0 / (p.p1682));
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_scale_ad(174, A::add(A::offset(s.ad_value(172), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(172), (-1.0)), A::offset(s.ad_value(172), (-1.0))), ((0.25 * p.p1688) * p.p1688)))), 0.5);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_scale(573, 174, p.p1682);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_div_ad_lhs(589, A::scale(s.ad_value(179), 2.0), 217);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(589), s.ad_value(402)), 1.0);
        }

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_offset_scaled(170, 402, p.p1685, 1.0);
        }

        s.v[1685] = if ((s.v[169] > 0.0) && (s.v[170] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            let assign33250_ad_e55734: A = A::mul({
                if (!(((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38) {
                            A::ln(A::div(A::offset(s.ad_value(392), 0.5), A::offset(s.ad_value(393), 0.5)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::offset(A::add(s.ad_value(392), s.ad_value(393)), 1.0));
            s.store_ad(171, &assign33250_ad_e55734);
        }

        if ((((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_scaled_sub(172, 392, 393, 2.0);
        }

        s.v[1686] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        s.v[1687] = if (p.p72 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1686] != 0.0) {
            s.store_mul(168, 415, 592);
        }

        if (s.v[1686] != 0.0) {
            s.store_add_ad(169, A::mul(s.ad_value(168), s.ad_value(197)), A::square(s.ad_value(153)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div(168, 399, 217);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_square(168, 168);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale_ad(597, A::offset(A::mul(A::scale(s.ad_value(168), p.p1709), s.ad_value(153)), 1.0), p.p1708);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale_ad(598, A::offset(A::mul(A::scale(s.ad_value(168), p.p1711), s.ad_value(153)), 1.0), p.p1710);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale_ad(599, A::offset(A::mul(A::scale(s.ad_value(168), p.p1713), s.ad_value(153)), 1.0), p.p1712);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale_ad(600, A::offset(A::mul(A::scale(s.ad_value(168), p.p1715), s.ad_value(153)), 1.0), p.p1714);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad_lhs(169, A::scale(s.ad_value(597), 3.0), 597);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(598), 7.5), 598);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale(171, 599, 2.5298);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(601, A::div(s.ad_value(393), s.ad_value(392)), A::sub_from_scalar(1.0, A::div(s.ad_value(390), s.ad_value(210))));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad_lhs(604, A::square(s.ad_value(209)), 209);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_rhs(602, 339, A::add(s.ad_value(339), s.ad_value(399)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_rhs(172, 236, A::add(A::mul(A::max_from_scalar(0.0, s.ad_value(237)), s.ad_value(392)), A::scale(s.ad_value(181), 2.0)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_limited_exp_ad(616, A::neg(s.ad_value(172)));
        }

        s.v[1688] = if (p.p61 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1688] != 0.0)) {
            s.store_ad(172, &{
                if (!(s.v[293] < ((-10000.0) * 1e-6))) {
                    A::scale(A::add(s.ad_value(293), A::sqrt(A::offset(A::square(s.ad_value(293)), ((4.0 * 1e-6) * 1e-6)))), 0.5)
                } else {
                    {
                        if (s.v[293] < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), s.ad_value(293))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1688] != 0.0)) {
            s.store_div_ad_rhs(174, 172, A::add(A::mul(A::max_from_scalar(0.0, s.ad_value(238)), s.ad_value(392)), A::scale(s.ad_value(181), 2.0)));
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1688] != 0.0)) {
            s.store_sub_ad(175, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1688] != 0.0)) {
            s.store_limited_exp_ad(617, A::mul(A::neg(s.ad_value(174)), s.ad_value(175)));
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (!(s.v[1688] != 0.0))) {
            s.store_scalar(617, 1.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale_ad(615, A::add(s.ad_value(401), A::mul(s.ad_value(407), s.ad_value(392))), s.v[420]);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_ad(172, &A::pow(A::scale(A::offset(A::abs(A::div(s.ad_value(392), s.ad_value(406))), 1.0), 0.5), s.ad_value(317)));
        }

        s.v[1689] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1689] != 0.0)) {
            s.store_add_ad(174, A::mul(A::add(s.ad_value(819), A::mul(s.ad_value(821), s.ad_value(370))), A::pow(A::abs(s.ad_value(615)), s.ad_value(822))), A::div(s.ad_value(820), s.ad_value(172)));
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (!(s.v[1689] != 0.0))) {
            s.store_add_ad(174, A::mul(s.ad_value(819), A::pow(A::abs(s.ad_value(615)), s.ad_value(822))), A::div(s.ad_value(820), s.ad_value(172)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_offset(618, 174, 1.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale_ad(618, A::add(A::offset(s.ad_value(618), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(618), (-1.0)), A::offset(s.ad_value(618), (-1.0))), ((0.25 * p.p604) * p.p604)))), 0.5);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale(618, 618, 1.0 / (p.p24));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scalar(619, (1.0 + (0.25 * p.p453)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_rhs(612, 339, A::add(s.ad_value(339), s.ad_value(392)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad_lhs(172, A::sub_from_scalar(2.0, s.ad_value(612)), 181);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_add(613, 392, 172);
        }

        s.v[1690] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        s.v[1691] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        s.v[1692] = if (p.p64 == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1690] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(392)), 1.0);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1690] != 0.0)) {
            s.store_div_from_scalar(174, 1.0, 172);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1690] != 0.0)) {
            s.store_scale_ad(175, A::add(s.ad_value(174), A::sqrt(A::offset(A::square(s.ad_value(174)), 0.01))), 0.5);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1690] != 0.0)) {
            s.store_mul_ad_lhs(614, A::mul(s.ad_value(194), A::offset(A::mul(s.ad_value(709), s.ad_value(175)), p.p908)), 189);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1690] != 0.0)) {
            s.store_offset_ad(620, A::mul(A::div(A::mul(A::scale(s.ad_value(183), s.v[115]), s.ad_value(613)), A::mul(s.ad_value(618), s.ad_value(619))), s.ad_value(614)), 1.0);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1691] != 0.0) && (!(s.v[1690] != 0.0)))) {
            s.store_scalar(620, 1.0);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1692] != 0.0) && (!((s.v[1690] != 0.0) || (s.v[1691] != 0.0))))) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(392)), 1.0);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1692] != 0.0) && (!((s.v[1690] != 0.0) || (s.v[1691] != 0.0))))) {
            s.store_div_from_scalar(174, 1.0, 172);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1692] != 0.0) && (!((s.v[1690] != 0.0) || (s.v[1691] != 0.0))))) {
            s.store_scale_ad(175, A::add(s.ad_value(174), A::sqrt(A::offset(A::square(s.ad_value(174)), 0.01))), 0.5);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1692] != 0.0) && (!((s.v[1690] != 0.0) || (s.v[1691] != 0.0))))) {
            s.store_mul_ad_lhs(614, A::offset(A::mul(s.ad_value(709), s.ad_value(175)), p.p908), 189);
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1692] != 0.0) && (!((s.v[1690] != 0.0) || (s.v[1691] != 0.0))))) {
            s.store_mul_ad_rhs(614, 194, A::add(A::add(s.ad_value(190), s.ad_value(191)), s.ad_value(614)));
        }

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && ((s.v[1692] != 0.0) && (!((s.v[1690] != 0.0) || (s.v[1691] != 0.0))))) {
            s.store_offset_ad(620, A::mul(A::div(A::mul(A::scale(s.ad_value(183), s.v[115]), s.ad_value(613)), A::mul(s.ad_value(618), s.ad_value(619))), s.ad_value(614)), 1.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad(603, A::mul(A::mul(A::mul(A::scale(s.ad_value(183), s.v[115]), s.ad_value(392)), s.ad_value(616)), s.ad_value(617)), A::mul(A::mul(s.ad_value(618), s.ad_value(619)), s.ad_value(620)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_offset(172, 601, 1.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_sub_from_scalar(174, 1.0, 601);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad_lhs(175, A::div(A::scale(s.ad_value(602), 2.0), s.ad_value(392)), 181);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_add(176, 172, 175);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_square(605, 174);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul(606, 605, 174);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul(607, 606, 174);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_square(608, 176);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul(609, 608, 176);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul(610, 609, 176);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul(611, 610, 176);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_scale(621, 172, 0.5);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_rhs(622, 605, A::scale(s.ad_value(176), 6.0));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(623, A::div(s.ad_value(205), s.ad_value(209)), A::add(s.ad_value(621), s.ad_value(622)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div(624, 172, 608);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad(625, A::mul(A::add(A::scale(s.ad_value(172), 6.0), s.ad_value(175)), s.ad_value(605)), A::scale(s.ad_value(610), 15.0));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_rhs(626, 607, A::scale(s.ad_value(611), 9.0));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(627, A::mul(A::scale(s.ad_value(205), 0.16666666666666666), s.ad_value(604)), A::add(A::sub(s.ad_value(624), s.ad_value(625)), s.ad_value(626)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_offset_ad(177, A::mul(A::div(A::square(s.ad_value(600)), A::offset(s.ad_value(399), p.p1716)), A::div(s.ad_value(390), s.ad_value(210))), 1.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(623, A::div(s.ad_value(205), s.ad_value(209)), A::add(A::mul(s.ad_value(177), s.ad_value(621)), A::mul(s.ad_value(169), s.ad_value(622))));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(627, A::mul(A::mul(A::scale(s.ad_value(205), 0.16666666666666666), s.ad_value(604)), s.ad_value(170)), A::add(A::sub(s.ad_value(624), s.ad_value(625)), s.ad_value(626)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_lhs(632, A::mul(A::mul(A::mul(A::scale(A::sqrt(A::div(s.ad_value(627), s.ad_value(623))), s.v[115]), s.ad_value(372)), s.ad_value(159)), s.ad_value(156)), 603);
        }

        s.v[1696] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        s.v[1705] = if (p.p76 != 2.0) { 1.0 } else { 0.0 };

        s.v[1706] = if (p.p65 == 1.0) { 1.0 } else { 0.0 };

        s.v[1707] = if (p.p78 == 1.0) { 1.0 } else { 0.0 };

        s.v[1708] = if (p.p65 == 1.0) { 1.0 } else { 0.0 };

        s.v[1709] = if (p.p78 == 1.0) { 1.0 } else { 0.0 };

        s.v[1710] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1711] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        s.v[1712] = if (p.p1910 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            let assign34330_ad_e57040: A = {
                if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(1039, &assign34330_ad_e57040);
        }

        s.v[1713] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) && (s.v[1713] != 0.0)) {
            let assign34350_ad_e57094: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), (-(-p.p1904))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), (-(-p.p1904))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), (-(-p.p1904))), (-1e-6))), (-((4.0 * (-p.p1904)) * 1e-6))))), 0.5), (-p.p1904)), p.p1904);
            s.store_ad(1044, &assign34350_ad_e57094);
        }

        if (((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) && (!(s.v[1713] != 0.0))) {
            let assign34360_ad_e57181: A = {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(1044, assign34360_ad_e57181, p.p1904);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_offset(168, 392, (-p.p1906));
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_scale_ad(168, A::add(A::offset(s.ad_value(168), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.1)), A::offset(s.ad_value(168), (-0.1))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_div_ad(169, A::scale(s.ad_value(168), (10.0 * p.p1907)), A::offset(s.ad_value(168), (10.0 * p.p1907)));
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_rhs(1045, 1044, A::offset(A::scale(s.ad_value(169), p.p1905), 1.0));
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_ad(1045, &{
                if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                    A::scale(A::add(s.ad_value(1045), A::sqrt(A::offset(A::square(s.ad_value(1045)), ((4.0 * 10.0) * 10.0)))), 0.5)
                } else {
                    {
                        if (s.v[1045] < ((-10000.0) * 10.0)) {
                            A::div_from_scalar(((-10.0) * 10.0), s.ad_value(1045))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(158), (s.v[115] * 1.60219e-19)), 1045);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_ad(174, &A::abs(A::voltage(ctx, &nodes, Some(9), Some(7))));
        }

        s.v[1714] = if (p.p1917 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) && (s.v[1714] != 0.0)) {
            s.store_scalar(171, 1.0);
        }

        if (((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) && (!(s.v[1714] != 0.0))) {
            s.store_scale_ad(171, A::add(A::offset(s.ad_value(174), (-p.p1916)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(174), (-p.p1916)), A::offset(s.ad_value(174), (-p.p1916))), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        if (((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) && (!(s.v[1714] != 0.0))) {
            s.store_offset_scaled(171, 171, p.p1917, 1.0);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_lhs(1047, A::scale(s.ad_value(170), p.p1903), 171);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_lhs(172, A::scale(s.ad_value(1039), p.p1910), 189);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul(1048, 1047, 172);
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_div_ad(1050, A::powf(s.ad_value(174), (4.0 - p.p1908)), A::add(A::powf(s.ad_value(174), (4.0 - p.p1908)), A::scale(A::powf(s.ad_value(1048), (4.0 - p.p1908)), p.p1914)));
        }

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_div_ad_lhs(175, A::mul(A::powf(s.ad_value(1050), (1.0 / p.p1908)), s.ad_value(174)), 1048);
        }

        s.v[1715] = if (p.p1911 > 0.0) { 1.0 } else { 0.0 };

        s.v[1716] = if (p.p1910 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            let assign34560_ad_e57528: A = {
                if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), p.p1912), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(1039, &assign34560_ad_e57528);
        }

        s.v[1717] = if (p.p75 != 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) && (s.v[1717] != 0.0)) {
            let assign34580_ad_e57584: A = A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), (-(-p.p1904))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), (-(-p.p1904))), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), (-(-p.p1904))), (-1e-6))), (-((4.0 * (-p.p1904)) * 1e-6))))), 0.5), (-p.p1904)), p.p1904);
            s.store_ad(1044, &assign34580_ad_e57584);
        }

        if ((((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) && (!(s.v[1717] != 0.0))) {
            let assign34590_ad_e57673: A = {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(232), (-p.p1913)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(1044, assign34590_ad_e57673, p.p1904);
        }

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            s.store_offset(168, 392, (-p.p1906));
        }

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            s.store_scale_ad(168, A::add(A::offset(s.ad_value(168), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.1)), A::offset(s.ad_value(168), (-0.1))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            s.store_div_ad(169, A::scale(s.ad_value(168), (10.0 * p.p1907)), A::offset(s.ad_value(168), (10.0 * p.p1907)));
        }

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            s.store_mul_ad_rhs(1045, 1044, A::offset(A::scale(s.ad_value(169), p.p1905), 1.0));
        }

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            s.store_ad(1045, &{
                if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                    A::scale(A::add(s.ad_value(1045), A::sqrt(A::offset(A::square(s.ad_value(1045)), ((4.0 * 10.0) * 10.0)))), 0.5)
                } else {
                    {
                        if (s.v[1045] < ((-10000.0) * 10.0)) {
                            A::div_from_scalar(((-10.0) * 10.0), s.ad_value(1045))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) && (s.v[1716] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(158), (s.v[115] * 1.60219e-19)), 1045);
        }

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_scale(1046, 170, p.p1909);
        }

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_mul_ad_lhs(172, A::scale(s.ad_value(1039), p.p1911), 189);
        }

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_mul(1049, 1046, 172);
        }

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_ad(174, &A::abs(A::voltage(ctx, &nodes, Some(6), Some(8))));
        }

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_div_ad(1051, A::powf(s.ad_value(174), (4.0 - p.p1908)), A::add(A::powf(s.ad_value(174), (4.0 - p.p1908)), A::scale(A::powf(s.ad_value(1049), (4.0 - p.p1908)), p.p1915)));
        }

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_div_ad_lhs(175, A::mul(A::powf(s.ad_value(1051), (1.0 / p.p1908)), s.ad_value(174)), 1049);
        }

        s.v[1723] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        s.v[1731] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        s.v[1736] = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };

        s.store_add_ad(339, A::div(A::scale(s.ad_value(179), 10.0), s.ad_value(898)), A::scale(s.ad_value(396), 2.0));

        s.store_mul_ad_rhs(169, 179, A::add(s.ad_value(179), s.ad_value(339)));

        s.store_mul_ad_lhs(170, A::square(s.ad_value(163)), 169);

        s.store_mul_ad_lhs(171, A::scale(s.ad_value(141), ((2.0 * 1.60219e-19) * s.v[143])), 179);

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
        let (eq0_e1945, eq0_e1945_d_n0, eq0_e1945_d_n1, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n12, eq0_e1945_d_n13, eq0_e1945_d_n14, eq0_e1945_d_n15, eq0_e1945_d_n16,) = {
    if (s.v[1695] != 0.0) {
        let eq0_e1943: f64 = (s.v[114] * s.v[124]);
        let eq0_e1943_d_n0: f64 = ((s.dn[114][0] * s.v[124]) + (s.v[114] * s.dn[124][0]));
        let eq0_e1943_d_n1: f64 = ((s.dn[114][1] * s.v[124]) + (s.v[114] * s.dn[124][1]));
        let eq0_e1943_d_n2: f64 = ((s.dn[114][2] * s.v[124]) + (s.v[114] * s.dn[124][2]));
        let eq0_e1943_d_n3: f64 = ((s.dn[114][3] * s.v[124]) + (s.v[114] * s.dn[124][3]));
        let eq0_e1943_d_n4: f64 = ((s.dn[114][4] * s.v[124]) + (s.v[114] * s.dn[124][4]));
        let eq0_e1943_d_n5: f64 = ((s.dn[114][5] * s.v[124]) + (s.v[114] * s.dn[124][5]));
        let eq0_e1943_d_n6: f64 = ((s.dn[114][6] * s.v[124]) + (s.v[114] * s.dn[124][6]));
        let eq0_e1943_d_n7: f64 = ((s.dn[114][7] * s.v[124]) + (s.v[114] * s.dn[124][7]));
        let eq0_e1943_d_n8: f64 = ((s.dn[114][8] * s.v[124]) + (s.v[114] * s.dn[124][8]));
        let eq0_e1943_d_n9: f64 = ((s.dn[114][9] * s.v[124]) + (s.v[114] * s.dn[124][9]));
        let eq0_e1943_d_n10: f64 = ((s.dn[114][10] * s.v[124]) + (s.v[114] * s.dn[124][10]));
        let eq0_e1943_d_n11: f64 = ((s.dn[114][11] * s.v[124]) + (s.v[114] * s.dn[124][11]));
        let eq0_e1943_d_n12: f64 = ((s.dn[114][12] * s.v[124]) + (s.v[114] * s.dn[124][12]));
        let eq0_e1943_d_n13: f64 = ((s.dn[114][13] * s.v[124]) + (s.v[114] * s.dn[124][13]));
        let eq0_e1943_d_n14: f64 = ((s.dn[114][14] * s.v[124]) + (s.v[114] * s.dn[124][14]));
        let eq0_e1943_d_n15: f64 = ((s.dn[114][15] * s.v[124]) + (s.v[114] * s.dn[124][15]));
        let eq0_e1943_d_n16: f64 = ((s.dn[114][16] * s.v[124]) + (s.v[114] * s.dn[124][16]));
        (eq0_e1943, eq0_e1943_d_n0, eq0_e1943_d_n1, eq0_e1943_d_n2, eq0_e1943_d_n3, eq0_e1943_d_n4, eq0_e1943_d_n5, eq0_e1943_d_n6, eq0_e1943_d_n7, eq0_e1943_d_n8, eq0_e1943_d_n9, eq0_e1943_d_n10, eq0_e1943_d_n11, eq0_e1943_d_n12, eq0_e1943_d_n13, eq0_e1943_d_n14, eq0_e1943_d_n15, eq0_e1943_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1945;
        let eq0_node_derivatives: [f64; 17] = [eq0_e1945_d_n0, eq0_e1945_d_n1, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n12, eq0_e1945_d_n13, eq0_e1945_d_n14, eq0_e1945_d_n15, eq0_e1945_d_n16];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
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
        let (eq1_e1952, eq1_e1952_d_n0, eq1_e1952_d_n1, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n12, eq1_e1952_d_n13, eq1_e1952_d_n14, eq1_e1952_d_n15, eq1_e1952_d_n16,) = {
    if (!(s.v[1695] != 0.0)) {
        let eq1_e1950: f64 = (s.v[114] * s.v[124]);
        let eq1_e1950_d_n0: f64 = ((s.dn[114][0] * s.v[124]) + (s.v[114] * s.dn[124][0]));
        let eq1_e1950_d_n1: f64 = ((s.dn[114][1] * s.v[124]) + (s.v[114] * s.dn[124][1]));
        let eq1_e1950_d_n2: f64 = ((s.dn[114][2] * s.v[124]) + (s.v[114] * s.dn[124][2]));
        let eq1_e1950_d_n3: f64 = ((s.dn[114][3] * s.v[124]) + (s.v[114] * s.dn[124][3]));
        let eq1_e1950_d_n4: f64 = ((s.dn[114][4] * s.v[124]) + (s.v[114] * s.dn[124][4]));
        let eq1_e1950_d_n5: f64 = ((s.dn[114][5] * s.v[124]) + (s.v[114] * s.dn[124][5]));
        let eq1_e1950_d_n6: f64 = ((s.dn[114][6] * s.v[124]) + (s.v[114] * s.dn[124][6]));
        let eq1_e1950_d_n7: f64 = ((s.dn[114][7] * s.v[124]) + (s.v[114] * s.dn[124][7]));
        let eq1_e1950_d_n8: f64 = ((s.dn[114][8] * s.v[124]) + (s.v[114] * s.dn[124][8]));
        let eq1_e1950_d_n9: f64 = ((s.dn[114][9] * s.v[124]) + (s.v[114] * s.dn[124][9]));
        let eq1_e1950_d_n10: f64 = ((s.dn[114][10] * s.v[124]) + (s.v[114] * s.dn[124][10]));
        let eq1_e1950_d_n11: f64 = ((s.dn[114][11] * s.v[124]) + (s.v[114] * s.dn[124][11]));
        let eq1_e1950_d_n12: f64 = ((s.dn[114][12] * s.v[124]) + (s.v[114] * s.dn[124][12]));
        let eq1_e1950_d_n13: f64 = ((s.dn[114][13] * s.v[124]) + (s.v[114] * s.dn[124][13]));
        let eq1_e1950_d_n14: f64 = ((s.dn[114][14] * s.v[124]) + (s.v[114] * s.dn[124][14]));
        let eq1_e1950_d_n15: f64 = ((s.dn[114][15] * s.v[124]) + (s.v[114] * s.dn[124][15]));
        let eq1_e1950_d_n16: f64 = ((s.dn[114][16] * s.v[124]) + (s.v[114] * s.dn[124][16]));
        (eq1_e1950, eq1_e1950_d_n0, eq1_e1950_d_n1, eq1_e1950_d_n2, eq1_e1950_d_n3, eq1_e1950_d_n4, eq1_e1950_d_n5, eq1_e1950_d_n6, eq1_e1950_d_n7, eq1_e1950_d_n8, eq1_e1950_d_n9, eq1_e1950_d_n10, eq1_e1950_d_n11, eq1_e1950_d_n12, eq1_e1950_d_n13, eq1_e1950_d_n14, eq1_e1950_d_n15, eq1_e1950_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1952;
        let eq1_node_derivatives: [f64; 17] = [eq1_e1952_d_n0, eq1_e1952_d_n1, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n12, eq1_e1952_d_n13, eq1_e1952_d_n14, eq1_e1952_d_n15, eq1_e1952_d_n16];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq2_e1961, eq2_e1961_d_n0, eq2_e1961_d_n1, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n12, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15, eq2_e1961_d_n16,) = {
    if (s.v[1696] != 0.0) {
        let eq2_e1956: f64 = (s.v[114] * s.v[570]);
        let eq2_e1956_d_n0: f64 = ((s.dn[114][0] * s.v[570]) + (s.v[114] * s.dn[570][0]));
        let eq2_e1956_d_n1: f64 = ((s.dn[114][1] * s.v[570]) + (s.v[114] * s.dn[570][1]));
        let eq2_e1956_d_n2: f64 = ((s.dn[114][2] * s.v[570]) + (s.v[114] * s.dn[570][2]));
        let eq2_e1956_d_n3: f64 = ((s.dn[114][3] * s.v[570]) + (s.v[114] * s.dn[570][3]));
        let eq2_e1956_d_n4: f64 = ((s.dn[114][4] * s.v[570]) + (s.v[114] * s.dn[570][4]));
        let eq2_e1956_d_n5: f64 = ((s.dn[114][5] * s.v[570]) + (s.v[114] * s.dn[570][5]));
        let eq2_e1956_d_n6: f64 = ((s.dn[114][6] * s.v[570]) + (s.v[114] * s.dn[570][6]));
        let eq2_e1956_d_n7: f64 = ((s.dn[114][7] * s.v[570]) + (s.v[114] * s.dn[570][7]));
        let eq2_e1956_d_n8: f64 = ((s.dn[114][8] * s.v[570]) + (s.v[114] * s.dn[570][8]));
        let eq2_e1956_d_n9: f64 = ((s.dn[114][9] * s.v[570]) + (s.v[114] * s.dn[570][9]));
        let eq2_e1956_d_n10: f64 = ((s.dn[114][10] * s.v[570]) + (s.v[114] * s.dn[570][10]));
        let eq2_e1956_d_n11: f64 = ((s.dn[114][11] * s.v[570]) + (s.v[114] * s.dn[570][11]));
        let eq2_e1956_d_n12: f64 = ((s.dn[114][12] * s.v[570]) + (s.v[114] * s.dn[570][12]));
        let eq2_e1956_d_n13: f64 = ((s.dn[114][13] * s.v[570]) + (s.v[114] * s.dn[570][13]));
        let eq2_e1956_d_n14: f64 = ((s.dn[114][14] * s.v[570]) + (s.v[114] * s.dn[570][14]));
        let eq2_e1956_d_n15: f64 = ((s.dn[114][15] * s.v[570]) + (s.v[114] * s.dn[570][15]));
        let eq2_e1956_d_n16: f64 = ((s.dn[114][16] * s.v[570]) + (s.v[114] * s.dn[570][16]));
        let eq2_e1958: f64 = (-(nv15 - 0.0));
        let eq2_e1958_d_n15: f64 = (-1.0);
        let eq2_e1959: f64 = (eq2_e1956 * eq2_e1958);
        let eq2_e1959_d_n0: f64 = (eq2_e1956_d_n0 * eq2_e1958);
        let eq2_e1959_d_n1: f64 = (eq2_e1956_d_n1 * eq2_e1958);
        let eq2_e1959_d_n2: f64 = (eq2_e1956_d_n2 * eq2_e1958);
        let eq2_e1959_d_n3: f64 = (eq2_e1956_d_n3 * eq2_e1958);
        let eq2_e1959_d_n4: f64 = (eq2_e1956_d_n4 * eq2_e1958);
        let eq2_e1959_d_n5: f64 = (eq2_e1956_d_n5 * eq2_e1958);
        let eq2_e1959_d_n6: f64 = (eq2_e1956_d_n6 * eq2_e1958);
        let eq2_e1959_d_n7: f64 = (eq2_e1956_d_n7 * eq2_e1958);
        let eq2_e1959_d_n8: f64 = (eq2_e1956_d_n8 * eq2_e1958);
        let eq2_e1959_d_n9: f64 = (eq2_e1956_d_n9 * eq2_e1958);
        let eq2_e1959_d_n10: f64 = (eq2_e1956_d_n10 * eq2_e1958);
        let eq2_e1959_d_n11: f64 = (eq2_e1956_d_n11 * eq2_e1958);
        let eq2_e1959_d_n12: f64 = (eq2_e1956_d_n12 * eq2_e1958);
        let eq2_e1959_d_n13: f64 = (eq2_e1956_d_n13 * eq2_e1958);
        let eq2_e1959_d_n14: f64 = (eq2_e1956_d_n14 * eq2_e1958);
        let eq2_e1959_d_n15: f64 = ((eq2_e1956_d_n15 * eq2_e1958) + (eq2_e1956 * eq2_e1958_d_n15));
        let eq2_e1959_d_n16: f64 = (eq2_e1956_d_n16 * eq2_e1958);
        (eq2_e1959, eq2_e1959_d_n0, eq2_e1959_d_n1, eq2_e1959_d_n2, eq2_e1959_d_n3, eq2_e1959_d_n4, eq2_e1959_d_n5, eq2_e1959_d_n6, eq2_e1959_d_n7, eq2_e1959_d_n8, eq2_e1959_d_n9, eq2_e1959_d_n10, eq2_e1959_d_n11, eq2_e1959_d_n12, eq2_e1959_d_n13, eq2_e1959_d_n14, eq2_e1959_d_n15, eq2_e1959_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1961;
        let eq2_node_derivatives: [f64; 17] = [eq2_e1961_d_n0, eq2_e1961_d_n1, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n12, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15, eq2_e1961_d_n16];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq3_e1971, eq3_e1971_d_n0, eq3_e1971_d_n1, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n12, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15, eq3_e1971_d_n16,) = {
    if (s.v[1696] != 0.0) {
        let eq3_e1965: f64 = (s.v[114] * s.v[571]);
        let eq3_e1965_d_n0: f64 = ((s.dn[114][0] * s.v[571]) + (s.v[114] * s.dn[571][0]));
        let eq3_e1965_d_n1: f64 = ((s.dn[114][1] * s.v[571]) + (s.v[114] * s.dn[571][1]));
        let eq3_e1965_d_n2: f64 = ((s.dn[114][2] * s.v[571]) + (s.v[114] * s.dn[571][2]));
        let eq3_e1965_d_n3: f64 = ((s.dn[114][3] * s.v[571]) + (s.v[114] * s.dn[571][3]));
        let eq3_e1965_d_n4: f64 = ((s.dn[114][4] * s.v[571]) + (s.v[114] * s.dn[571][4]));
        let eq3_e1965_d_n5: f64 = ((s.dn[114][5] * s.v[571]) + (s.v[114] * s.dn[571][5]));
        let eq3_e1965_d_n6: f64 = ((s.dn[114][6] * s.v[571]) + (s.v[114] * s.dn[571][6]));
        let eq3_e1965_d_n7: f64 = ((s.dn[114][7] * s.v[571]) + (s.v[114] * s.dn[571][7]));
        let eq3_e1965_d_n8: f64 = ((s.dn[114][8] * s.v[571]) + (s.v[114] * s.dn[571][8]));
        let eq3_e1965_d_n9: f64 = ((s.dn[114][9] * s.v[571]) + (s.v[114] * s.dn[571][9]));
        let eq3_e1965_d_n10: f64 = ((s.dn[114][10] * s.v[571]) + (s.v[114] * s.dn[571][10]));
        let eq3_e1965_d_n11: f64 = ((s.dn[114][11] * s.v[571]) + (s.v[114] * s.dn[571][11]));
        let eq3_e1965_d_n12: f64 = ((s.dn[114][12] * s.v[571]) + (s.v[114] * s.dn[571][12]));
        let eq3_e1965_d_n13: f64 = ((s.dn[114][13] * s.v[571]) + (s.v[114] * s.dn[571][13]));
        let eq3_e1965_d_n14: f64 = ((s.dn[114][14] * s.v[571]) + (s.v[114] * s.dn[571][14]));
        let eq3_e1965_d_n15: f64 = ((s.dn[114][15] * s.v[571]) + (s.v[114] * s.dn[571][15]));
        let eq3_e1965_d_n16: f64 = ((s.dn[114][16] * s.v[571]) + (s.v[114] * s.dn[571][16]));
        let eq3_e1967: f64 = (eq3_e1965 * s.v[570]);
        let eq3_e1967_d_n0: f64 = ((eq3_e1965_d_n0 * s.v[570]) + (eq3_e1965 * s.dn[570][0]));
        let eq3_e1967_d_n1: f64 = ((eq3_e1965_d_n1 * s.v[570]) + (eq3_e1965 * s.dn[570][1]));
        let eq3_e1967_d_n2: f64 = ((eq3_e1965_d_n2 * s.v[570]) + (eq3_e1965 * s.dn[570][2]));
        let eq3_e1967_d_n3: f64 = ((eq3_e1965_d_n3 * s.v[570]) + (eq3_e1965 * s.dn[570][3]));
        let eq3_e1967_d_n4: f64 = ((eq3_e1965_d_n4 * s.v[570]) + (eq3_e1965 * s.dn[570][4]));
        let eq3_e1967_d_n5: f64 = ((eq3_e1965_d_n5 * s.v[570]) + (eq3_e1965 * s.dn[570][5]));
        let eq3_e1967_d_n6: f64 = ((eq3_e1965_d_n6 * s.v[570]) + (eq3_e1965 * s.dn[570][6]));
        let eq3_e1967_d_n7: f64 = ((eq3_e1965_d_n7 * s.v[570]) + (eq3_e1965 * s.dn[570][7]));
        let eq3_e1967_d_n8: f64 = ((eq3_e1965_d_n8 * s.v[570]) + (eq3_e1965 * s.dn[570][8]));
        let eq3_e1967_d_n9: f64 = ((eq3_e1965_d_n9 * s.v[570]) + (eq3_e1965 * s.dn[570][9]));
        let eq3_e1967_d_n10: f64 = ((eq3_e1965_d_n10 * s.v[570]) + (eq3_e1965 * s.dn[570][10]));
        let eq3_e1967_d_n11: f64 = ((eq3_e1965_d_n11 * s.v[570]) + (eq3_e1965 * s.dn[570][11]));
        let eq3_e1967_d_n12: f64 = ((eq3_e1965_d_n12 * s.v[570]) + (eq3_e1965 * s.dn[570][12]));
        let eq3_e1967_d_n13: f64 = ((eq3_e1965_d_n13 * s.v[570]) + (eq3_e1965 * s.dn[570][13]));
        let eq3_e1967_d_n14: f64 = ((eq3_e1965_d_n14 * s.v[570]) + (eq3_e1965 * s.dn[570][14]));
        let eq3_e1967_d_n15: f64 = ((eq3_e1965_d_n15 * s.v[570]) + (eq3_e1965 * s.dn[570][15]));
        let eq3_e1967_d_n16: f64 = ((eq3_e1965_d_n16 * s.v[570]) + (eq3_e1965 * s.dn[570][16]));
        let eq3_e1969: f64 = (eq3_e1967 * (nv15 - 0.0));
        let eq3_e1969_d_n0: f64 = (eq3_e1967_d_n0 * (nv15 - 0.0));
        let eq3_e1969_d_n1: f64 = (eq3_e1967_d_n1 * (nv15 - 0.0));
        let eq3_e1969_d_n2: f64 = (eq3_e1967_d_n2 * (nv15 - 0.0));
        let eq3_e1969_d_n3: f64 = (eq3_e1967_d_n3 * (nv15 - 0.0));
        let eq3_e1969_d_n4: f64 = (eq3_e1967_d_n4 * (nv15 - 0.0));
        let eq3_e1969_d_n5: f64 = (eq3_e1967_d_n5 * (nv15 - 0.0));
        let eq3_e1969_d_n6: f64 = (eq3_e1967_d_n6 * (nv15 - 0.0));
        let eq3_e1969_d_n7: f64 = (eq3_e1967_d_n7 * (nv15 - 0.0));
        let eq3_e1969_d_n8: f64 = (eq3_e1967_d_n8 * (nv15 - 0.0));
        let eq3_e1969_d_n9: f64 = (eq3_e1967_d_n9 * (nv15 - 0.0));
        let eq3_e1969_d_n10: f64 = (eq3_e1967_d_n10 * (nv15 - 0.0));
        let eq3_e1969_d_n11: f64 = (eq3_e1967_d_n11 * (nv15 - 0.0));
        let eq3_e1969_d_n12: f64 = (eq3_e1967_d_n12 * (nv15 - 0.0));
        let eq3_e1969_d_n13: f64 = (eq3_e1967_d_n13 * (nv15 - 0.0));
        let eq3_e1969_d_n14: f64 = (eq3_e1967_d_n14 * (nv15 - 0.0));
        let eq3_e1969_d_n15: f64 = ((eq3_e1967_d_n15 * (nv15 - 0.0)) + eq3_e1967);
        let eq3_e1969_d_n16: f64 = (eq3_e1967_d_n16 * (nv15 - 0.0));
        (eq3_e1969, eq3_e1969_d_n0, eq3_e1969_d_n1, eq3_e1969_d_n2, eq3_e1969_d_n3, eq3_e1969_d_n4, eq3_e1969_d_n5, eq3_e1969_d_n6, eq3_e1969_d_n7, eq3_e1969_d_n8, eq3_e1969_d_n9, eq3_e1969_d_n10, eq3_e1969_d_n11, eq3_e1969_d_n12, eq3_e1969_d_n13, eq3_e1969_d_n14, eq3_e1969_d_n15, eq3_e1969_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1971;
        let eq3_node_derivatives: [f64; 17] = [eq3_e1971_d_n0, eq3_e1971_d_n1, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n12, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15, eq3_e1971_d_n16];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
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
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16,) = {
    if (!(s.v[1696] != 0.0)) {
        let eq4_e1976: f64 = self.eval_ddt(0, s.v[137]);
        let eq4_e1976_d_n0: f64 = self.ddt_jacobian(s.dn[137][0]);
        let eq4_e1976_d_n1: f64 = self.ddt_jacobian(s.dn[137][1]);
        let eq4_e1976_d_n2: f64 = self.ddt_jacobian(s.dn[137][2]);
        let eq4_e1976_d_n3: f64 = self.ddt_jacobian(s.dn[137][3]);
        let eq4_e1976_d_n4: f64 = self.ddt_jacobian(s.dn[137][4]);
        let eq4_e1976_d_n5: f64 = self.ddt_jacobian(s.dn[137][5]);
        let eq4_e1976_d_n6: f64 = self.ddt_jacobian(s.dn[137][6]);
        let eq4_e1976_d_n7: f64 = self.ddt_jacobian(s.dn[137][7]);
        let eq4_e1976_d_n8: f64 = self.ddt_jacobian(s.dn[137][8]);
        let eq4_e1976_d_n9: f64 = self.ddt_jacobian(s.dn[137][9]);
        let eq4_e1976_d_n10: f64 = self.ddt_jacobian(s.dn[137][10]);
        let eq4_e1976_d_n11: f64 = self.ddt_jacobian(s.dn[137][11]);
        let eq4_e1976_d_n12: f64 = self.ddt_jacobian(s.dn[137][12]);
        let eq4_e1976_d_n13: f64 = self.ddt_jacobian(s.dn[137][13]);
        let eq4_e1976_d_n14: f64 = self.ddt_jacobian(s.dn[137][14]);
        let eq4_e1976_d_n15: f64 = self.ddt_jacobian(s.dn[137][15]);
        let eq4_e1976_d_n16: f64 = self.ddt_jacobian(s.dn[137][16]);
        let eq4_e1977: f64 = (s.v[114] * eq4_e1976);
        let eq4_e1977_d_n0: f64 = ((s.dn[114][0] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n0));
        let eq4_e1977_d_n1: f64 = ((s.dn[114][1] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n1));
        let eq4_e1977_d_n2: f64 = ((s.dn[114][2] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n2));
        let eq4_e1977_d_n3: f64 = ((s.dn[114][3] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n3));
        let eq4_e1977_d_n4: f64 = ((s.dn[114][4] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n4));
        let eq4_e1977_d_n5: f64 = ((s.dn[114][5] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n5));
        let eq4_e1977_d_n6: f64 = ((s.dn[114][6] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n6));
        let eq4_e1977_d_n7: f64 = ((s.dn[114][7] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n7));
        let eq4_e1977_d_n8: f64 = ((s.dn[114][8] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n8));
        let eq4_e1977_d_n9: f64 = ((s.dn[114][9] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n9));
        let eq4_e1977_d_n10: f64 = ((s.dn[114][10] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n10));
        let eq4_e1977_d_n11: f64 = ((s.dn[114][11] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n11));
        let eq4_e1977_d_n12: f64 = ((s.dn[114][12] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n12));
        let eq4_e1977_d_n13: f64 = ((s.dn[114][13] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n13));
        let eq4_e1977_d_n14: f64 = ((s.dn[114][14] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n14));
        let eq4_e1977_d_n15: f64 = ((s.dn[114][15] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n15));
        let eq4_e1977_d_n16: f64 = ((s.dn[114][16] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n16));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n1, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n12, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_d_n15, eq4_e1977_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1979;
        let eq4_node_derivatives: [f64; 17] = [eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
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
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16,) = {
    if (!(s.v[1696] != 0.0)) {
        let eq5_e1984: f64 = self.eval_ddt(1, s.v[138]);
        let eq5_e1984_d_n0: f64 = self.ddt_jacobian(s.dn[138][0]);
        let eq5_e1984_d_n1: f64 = self.ddt_jacobian(s.dn[138][1]);
        let eq5_e1984_d_n2: f64 = self.ddt_jacobian(s.dn[138][2]);
        let eq5_e1984_d_n3: f64 = self.ddt_jacobian(s.dn[138][3]);
        let eq5_e1984_d_n4: f64 = self.ddt_jacobian(s.dn[138][4]);
        let eq5_e1984_d_n5: f64 = self.ddt_jacobian(s.dn[138][5]);
        let eq5_e1984_d_n6: f64 = self.ddt_jacobian(s.dn[138][6]);
        let eq5_e1984_d_n7: f64 = self.ddt_jacobian(s.dn[138][7]);
        let eq5_e1984_d_n8: f64 = self.ddt_jacobian(s.dn[138][8]);
        let eq5_e1984_d_n9: f64 = self.ddt_jacobian(s.dn[138][9]);
        let eq5_e1984_d_n10: f64 = self.ddt_jacobian(s.dn[138][10]);
        let eq5_e1984_d_n11: f64 = self.ddt_jacobian(s.dn[138][11]);
        let eq5_e1984_d_n12: f64 = self.ddt_jacobian(s.dn[138][12]);
        let eq5_e1984_d_n13: f64 = self.ddt_jacobian(s.dn[138][13]);
        let eq5_e1984_d_n14: f64 = self.ddt_jacobian(s.dn[138][14]);
        let eq5_e1984_d_n15: f64 = self.ddt_jacobian(s.dn[138][15]);
        let eq5_e1984_d_n16: f64 = self.ddt_jacobian(s.dn[138][16]);
        let eq5_e1985: f64 = (s.v[114] * eq5_e1984);
        let eq5_e1985_d_n0: f64 = ((s.dn[114][0] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n0));
        let eq5_e1985_d_n1: f64 = ((s.dn[114][1] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n1));
        let eq5_e1985_d_n2: f64 = ((s.dn[114][2] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n2));
        let eq5_e1985_d_n3: f64 = ((s.dn[114][3] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n3));
        let eq5_e1985_d_n4: f64 = ((s.dn[114][4] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n4));
        let eq5_e1985_d_n5: f64 = ((s.dn[114][5] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n5));
        let eq5_e1985_d_n6: f64 = ((s.dn[114][6] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n6));
        let eq5_e1985_d_n7: f64 = ((s.dn[114][7] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n7));
        let eq5_e1985_d_n8: f64 = ((s.dn[114][8] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n8));
        let eq5_e1985_d_n9: f64 = ((s.dn[114][9] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n9));
        let eq5_e1985_d_n10: f64 = ((s.dn[114][10] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n10));
        let eq5_e1985_d_n11: f64 = ((s.dn[114][11] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n11));
        let eq5_e1985_d_n12: f64 = ((s.dn[114][12] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n12));
        let eq5_e1985_d_n13: f64 = ((s.dn[114][13] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n13));
        let eq5_e1985_d_n14: f64 = ((s.dn[114][14] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n14));
        let eq5_e1985_d_n15: f64 = ((s.dn[114][15] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n15));
        let eq5_e1985_d_n16: f64 = ((s.dn[114][16] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n16));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n1, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n12, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_d_n15, eq5_e1985_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1987;
        let eq5_node_derivatives: [f64; 17] = [eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq6_e1993, eq6_e1993_d_n5, eq6_e1993_d_n7,) = {
    if (s.v[1697] != 0.0) {
        let eq6_e1991: f64 = ((nv7 - nv5) * 1000.0);
        let eq6_e1991_d_n5: f64 = (-1000.0);
        let eq6_e1991_d_n7: f64 = 1000.0;
        (eq6_e1991, eq6_e1991_d_n5, eq6_e1991_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1993;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq6_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq6_e1993_d_n5),
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq6_e1993_d_n7),
            ],
        );
    }
}
