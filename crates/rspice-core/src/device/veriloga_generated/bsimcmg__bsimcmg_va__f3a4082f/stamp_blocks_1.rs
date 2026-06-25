#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(424, 257);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(422, 334);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(425, 299);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(426, 301);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(795, 797);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(428, 332);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(659, 660);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(805, 804);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(669, 666);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(416, 413);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(819, 303);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(820, 318);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(821, 314);
        }

        if (!(s.v[1416] != 0.0)) {
            s.copy_ad(822, 323);
        }

        s.store_div_from_scalar(212, 1.0, 423);

        s.store_add_ad_lhs(353, A::offset(s.ad_value(166), 0.4), 672);

        s.store_div_ad(169, A::scale(A::div(s.ad_value(893), s.ad_value(895)), 2.0), A::offset(s.ad_value(898), 2.0));

        s.store_mul_ad_rhs(164, 362, A::add(s.ad_value(662), A::mul(s.ad_value(664), s.ad_value(127))));

        s.v[1417] = if (p.p175 == 0.0) { 1.0 } else { 0.0 };

        s.v[1418] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1417] != 0.0) && (s.v[1418] != 0.0)) {
            s.store_mul_ad(181, A::mul(s.ad_value(179), s.ad_value(235)), A::offset(A::div(A::add(s.ad_value(669), s.ad_value(164)), s.ad_value(169)), 1.0));
        }

        if ((s.v[1417] != 0.0) && (!(s.v[1418] != 0.0))) {
            s.store_mul_ad(181, A::mul(s.ad_value(182), s.ad_value(235)), A::offset(A::div(A::add(s.ad_value(669), s.ad_value(164)), s.ad_value(169)), 1.0));
        }

        if (!(s.v[1417] != 0.0)) {
            s.store_scalar(181, p.p175);
        }

        s.store_div(897, 903, 181);

        if (!(((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38)) {
            s.store_scalar(900, (-87.498233534));
        } else {
            s.store_ad(900, &{
                if (((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38) {
                    A::ln(A::div(A::mul(s.ad_value(893), s.ad_value(181)), A::mul(A::scale(s.ad_value(148), (1.60219e-19 * 2.0)), s.ad_value(894))))
                } else {
                    A::constant(0.0)
                }
            });
        }

        let assign20300_ad_e38153: A = {
    if (!(A::div(A::mul(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898))), A::offset(A::sub(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), A::mul(s.ad_value(897), s.ad_value(898))), (-1.0))).value > 1e-38)) {
        A::neg(A::constant(87.498233534))
    } else {
        let assign20300_ad_e38152: A = {
            if (A::div(A::mul(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898))), A::offset(A::sub(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), A::mul(s.ad_value(897), s.ad_value(898))), (-1.0))).value > 1e-38) {
                A::ln(A::div(A::mul(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898))), A::offset(A::sub(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), A::mul(s.ad_value(897), s.ad_value(898))), (-1.0))))
            } else {
                A::constant(0.0)
            }
        };
        assign20300_ad_e38152
    }
};
        s.store_add_ad_lhs(899, assign20300_ad_e38153, 900);

        s.store_add_ad(339, A::div(A::scale(s.ad_value(181), 10.0), s.ad_value(898)), A::scale(s.ad_value(396), 2.0));

        s.store_div_ad(912, A::mul(s.ad_value(179), s.ad_value(893)), A::scale(s.ad_value(895), s.v[143]));

        s.v[913] = ((((((4.5 * 1.05457e-34) * 3.141592653589793) * 1.60219e-19) / (4.0 * (((2.0 * s.v[381])) as f64).sqrt()))) as f64).powf(0.666666667);

        s.store_div_ad(914, A::scale(A::powf(s.ad_value(912), 0.666666667), (p.p1804 * s.v[913])), A::scale(s.ad_value(179), 1.60219e-19));

        s.store_mul_ad(354, A::mul(A::neg(s.ad_value(667)), s.ad_value(361)), A::sub(s.ad_value(352), s.ad_value(353)));

        s.store_add_ad(355, A::mul(A::mul(A::neg(s.ad_value(676)), s.ad_value(363)), A::add(s.ad_value(127), A::mul(s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01))))), A::mul(A::mul(s.ad_value(681), s.ad_value(365)), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));

        s.store_mul_ad(357, A::mul(s.ad_value(802), s.ad_value(364)), A::sqrt(s.ad_value(353)));

        s.store_add_ad_lhs(358, A::add(A::add(A::add(s.ad_value(354), s.ad_value(355)), s.ad_value(357)), s.ad_value(231)), 805);

        s.store_sub(347, 347, 358);

        s.store_div_ad_lhs(184, A::mul(A::mul(s.ad_value(416), s.ad_value(163)), s.ad_value(158)), 153);

        s.v[1419] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1419] != 0.0) {
            s.store_ad(171, &A::pow(A::div(A::scale(s.ad_value(163), (2.0 * p.p108)), A::scale(A::mul(A::scale(A::mul(s.ad_value(184), s.ad_value(181)), 1.60219e-19), s.ad_value(148)), p.p3)), s.ad_value(181)));
        }

        if (s.v[1419] != 0.0) {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (s.v[1419] != 0.0) {
            s.store_offset_ad(169, A::add(s.ad_value(347), s.ad_value(168)), p.p23);
        }

        if (s.v[1419] != 0.0) {
            let assign20450_ad_e38344: A = A::sub({
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(168));
            s.store_ad(348, &assign20450_ad_e38344);
        }

        if (!(s.v[1419] != 0.0)) {
            let assign20460_ad_e38406: A = {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div(A::scale(s.ad_value(163), (2.0 * p.p108)), A::scale(A::mul(A::scale(A::mul(s.ad_value(184), s.ad_value(181)), 1.60219e-19), s.ad_value(148)), p.p3)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad(168, A::neg(s.ad_value(181)), assign20460_ad_e38406);
        }

        if (!(s.v[1419] != 0.0)) {
            s.store_sub_ad_lhs(169, A::scale(A::add(A::offset(s.ad_value(168), 0.01), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.01)), A::offset(s.ad_value(168), (-0.01))), ((0.25 * 0.0001) * 0.0001)))), 0.5), 375);
        }

        if (!(s.v[1419] != 0.0)) {
            s.store_offset_ad(170, A::add(s.ad_value(347), s.ad_value(169)), p.p23);
        }

        if (!(s.v[1419] != 0.0)) {
            let assign20490_ad_e38485: A = A::sub({
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::scale(A::add(s.ad_value(170), A::sqrt(A::offset(A::square(s.ad_value(170)), ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(169));
            s.store_ad(348, &assign20490_ad_e38485);
        }

        s.copy_ad(129, 375);

        s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);

        s.v[1420] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1420] != 0.0) {
            let assign20530_ad_e38568: A = {
                if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::scale(A::add(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367)), A::sqrt(A::offset(A::mul(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367)), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367))), ((4.0 * 0.1) * 0.1)))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(169, &assign20530_ad_e38568);
        }

        if (s.v[1420] != 0.0) {
            s.store_mul_ad(171, A::div(A::neg(s.ad_value(239)), A::scale(s.ad_value(181), 2.0)), A::sub(A::sqrt(s.ad_value(169)), A::sqrt(A::scale(s.ad_value(166), 2.0))));
        }

        if (s.v[1420] != 0.0) {
            s.store_add_ad(168, A::add(A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), s.ad_value(899)), A::mul(s.ad_value(914), s.ad_value(172)));
        }

        if (s.v[1420] != 0.0) {
            s.store_add_ad_lhs(169, A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), 900);
        }

        if (!(s.v[1420] != 0.0)) {
            s.store_add_ad(168, A::sub(s.ad_value(899), s.ad_value(897)), A::mul(s.ad_value(914), s.ad_value(172)));
        }

        if (!(s.v[1420] != 0.0)) {
            s.store_sub(169, 900, 897);
        }

        s.store_div_ad_lhs(170, A::sub(s.ad_value(348), s.ad_value(129)), 181);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_ad(901, &A::limited_exp(s.ad_value(171)));

        s.v[1421] = if (s.v[901] > 1e-7) { 1.0 } else { 0.0 };

        if (s.v[1421] != 0.0) {
            s.store_ln_ad(176, A::offset(s.ad_value(901), 1.0));
        }

        if (s.v[1421] != 0.0) {
            s.store_scale_ad(901, A::sub_from_scalar(1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0))), 2.0);
        }

        if (s.v[1421] != 0.0) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if (s.v[1421] != 0.0) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if (s.v[1421] != 0.0) {
            s.store_mul(174, 177, 172);
        }

        if (s.v[1421] != 0.0) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if (s.v[1421] != 0.0) {
            let assign20700_ad_e38740: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign20700_ad_e38740, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if (s.v[1421] != 0.0) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if (s.v[1421] != 0.0) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if (s.v[1421] != 0.0) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if (s.v[1421] != 0.0) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if (s.v[1421] != 0.0) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if (s.v[1421] != 0.0) {
            s.store_mul(174, 177, 172);
        }

        if (s.v[1421] != 0.0) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if (s.v[1421] != 0.0) {
            let assign20780_ad_e38891: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign20780_ad_e38891, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if (s.v[1421] != 0.0) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if (s.v[1421] != 0.0) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if (s.v[1421] != 0.0) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if (!(s.v[1421] != 0.0)) {
            s.store_mul_ad_lhs(901, A::neg(s.ad_value(901)), 901);
        }

        s.store_mul_ad_lhs(392, A::neg(s.ad_value(901)), 181);

        s.v[1422] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1422] != 0.0) {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(347), s.ad_value(129)), 181);
        }

        if (s.v[1422] != 0.0) {
            s.store_scale_ad(1016, A::add(s.ad_value(1015), A::sqrt(A::add(A::mul(s.ad_value(1015), s.ad_value(1015)), A::mul(A::scale(s.ad_value(963), 0.25), s.ad_value(963))))), 0.5);
        }

        if (s.v[1422] != 0.0) {
            s.store_ad(1017, &A::pow(s.ad_value(1016), A::scale(s.ad_value(960), 0.5)));
        }

        if (s.v[1422] != 0.0) {
            s.store_mul_ad(1004, A::mul(s.ad_value(1010), s.ad_value(1017)), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
        }

        if (s.v[1422] != 0.0) {
            s.store_div_ad_lhs(1018, A::sub(A::sub(s.ad_value(347), s.ad_value(129)), s.ad_value(985)), 181);
        }

        if (s.v[1422] != 0.0) {
            s.store_scale_ad(1019, A::add(s.ad_value(1018), A::sqrt(A::add(A::mul(s.ad_value(1018), s.ad_value(1018)), A::mul(A::scale(s.ad_value(964), 0.25), s.ad_value(964))))), 0.5);
        }

        if (s.v[1422] != 0.0) {
            s.store_ad(1020, &A::pow(s.ad_value(1019), A::scale(s.ad_value(961), 0.5)));
        }

        if (s.v[1422] != 0.0) {
            s.store_mul_ad(1005, A::mul(s.ad_value(1011), s.ad_value(1020)), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
        }

        if (s.v[1422] != 0.0) {
            s.store_div_ad_lhs(1021, A::sub(A::sub(s.ad_value(347), s.ad_value(129)), s.ad_value(986)), 181);
        }

        if (s.v[1422] != 0.0) {
            s.store_scale_ad(1022, A::add(s.ad_value(1021), A::sqrt(A::add(A::mul(s.ad_value(1021), s.ad_value(1021)), A::mul(A::scale(s.ad_value(965), 0.25), s.ad_value(965))))), 0.5);
        }

        if (s.v[1422] != 0.0) {
            s.store_ad(1023, &A::pow(s.ad_value(1022), A::scale(s.ad_value(962), 0.5)));
        }

        if (s.v[1422] != 0.0) {
            s.store_mul_ad(1006, A::mul(s.ad_value(1012), s.ad_value(1023)), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
        }

        if (s.v[1422] != 0.0) {
            s.store_add_ad(392, A::mul(s.ad_value(983), s.ad_value(392)), A::mul(s.ad_value(984), A::add(A::add(s.ad_value(1004), s.ad_value(1005)), s.ad_value(1006))));
        }

        s.store_div_from_scalar(406, 0.01, 163);

        s.store_scale_ad(419, A::add(s.ad_value(396), A::mul(s.ad_value(407), s.ad_value(392))), s.v[420]);

        s.store_ad(170, &A::pow(A::scale(A::offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0), 0.5), s.ad_value(317)));

        s.store_ad(171, &A::pow(s.ad_value(419), s.ad_value(822)));

        s.v[1423] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1423] != 0.0) {
            s.store_add_ad(171, A::mul(A::add(s.ad_value(819), A::mul(s.ad_value(821), s.ad_value(370))), s.ad_value(171)), A::div(s.ad_value(820), s.ad_value(170)));
        }

        if (!(s.v[1423] != 0.0)) {
            s.store_add_ad(171, A::mul(s.ad_value(819), s.ad_value(171)), A::div(s.ad_value(820), s.ad_value(170)));
        }

        s.store_offset(397, 171, 1.0);

        s.store_scale_ad(397, A::add(A::offset(s.ad_value(397), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(397), (-1.0)), A::offset(s.ad_value(397), (-1.0))), ((0.25 * p.p604) * p.p604)))), 0.5);

        s.store_scale(397, 397, 1.0 / (p.p24));

        s.v[1424] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1424] != 0.0) {
            s.store_scalar(198, 0.0);
        }

        s.v[1425] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1424] != 0.0)) && (s.v[1425] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(392)), 1.0);
        }

        if ((!(s.v[1424] != 0.0)) && (s.v[1425] != 0.0)) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if ((!(s.v[1424] != 0.0)) && (s.v[1425] != 0.0)) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if ((!(s.v[1424] != 0.0)) && (s.v[1425] != 0.0)) {
            s.store_mul_ad_lhs(198, A::scale(A::mul(A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189)), s.v[115]), 194);
        }

        if ((!(s.v[1424] != 0.0)) && (!(s.v[1425] != 0.0))) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(392)), 1.0);
        }

        if ((!(s.v[1424] != 0.0)) && (!(s.v[1425] != 0.0))) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if ((!(s.v[1424] != 0.0)) && (!(s.v[1425] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1424] != 0.0)) && (!(s.v[1425] != 0.0))) {
            s.store_mul_ad_lhs(198, A::add(A::add(s.ad_value(190), s.ad_value(191)), A::scale(A::mul(A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189)), s.v[115])), 194);
        }

        s.store_mul_ad_lhs(216, A::div(A::scale(s.ad_value(428), 2.0), s.ad_value(416)), 397);

        s.store_mul(217, 216, 153);

        s.v[1426] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1426] != 0.0) {
            s.store_mul_ad_rhs(175, 659, A::add(s.ad_value(392), A::scale(s.ad_value(179), 2.0)));
        }

        if (!(s.v[1426] != 0.0)) {
            s.store_mul_ad_rhs(175, 659, A::add(s.ad_value(392), A::scale(s.ad_value(182), 2.0)));
        }

        s.v[1427] = if (s.v[198] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1427] != 0.0) {
            s.store_mul_ad_lhs(224, A::mul(s.ad_value(158), s.ad_value(428)), 163);
        }

        if (s.v[1427] != 0.0) {
            s.store_mul(168, 224, 198);
        }

        if (s.v[1427] != 0.0) {
            s.store_scale(225, 168, 2.0);
        }

        if (s.v[1427] != 0.0) {
            s.store_add_ad(226, A::add(s.ad_value(175), s.ad_value(217)), A::mul(A::scale(s.ad_value(175), 3.0), s.ad_value(168)));
        }

        if (s.v[1427] != 0.0) {
            s.store_mul_ad_rhs(227, 175, A::add(s.ad_value(217), A::mul(A::scale(s.ad_value(175), 2.0), s.ad_value(168))));
        }

        if (s.v[1427] != 0.0) {
            s.store_div_ad(210, A::sub(A::square(s.ad_value(226)), A::sub(A::square(s.ad_value(226)), A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(227)))), A::mul(A::add(s.ad_value(226), A::sqrt(A::sub(A::square(s.ad_value(226)), A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(227))))), s.ad_value(225)));
        }

        if (!(s.v[1427] != 0.0)) {
            s.store_div_ad(210, A::mul(s.ad_value(217), s.ad_value(175)), A::add(s.ad_value(217), s.ad_value(175)));
        }

        let assign21320_ad_e39524: A = {
    if (!((s.v[210] - 0.001) < ((-10000.0) * 1e-5))) {
        A::scale(A::add(A::offset(s.ad_value(210), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(210), (-0.001)), A::offset(s.ad_value(210), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5)
    } else {
        {
            if ((s.v[210] - 0.001) < ((-10000.0) * 1e-5)) {
                A::div_from_scalar(((-1e-5) * 1e-5), A::offset(s.ad_value(210), (-0.001)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(210, assign21320_ad_e39524, 0.001);

        s.store_ad(176, &A::pow(A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423)));

        s.store_ad(177, &A::pow(A::offset(s.ad_value(176), 1.0), s.ad_value(212)));

        s.store_ad(390, &A::min(A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126)));

        s.store_add(129, 390, 375);

        s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);

        s.v[1428] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1428] != 0.0) {
            let assign21390_ad_e39626: A = {
                if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::scale(A::add(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367)), A::sqrt(A::offset(A::mul(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367)), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367))), ((4.0 * 0.1) * 0.1)))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(129)), s.ad_value(367)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(169, &assign21390_ad_e39626);
        }

        if (s.v[1428] != 0.0) {
            s.store_mul_ad(171, A::div(A::neg(s.ad_value(239)), A::scale(s.ad_value(181), 2.0)), A::sub(A::sqrt(s.ad_value(169)), A::sqrt(A::scale(s.ad_value(166), 2.0))));
        }

        if (s.v[1428] != 0.0) {
            s.store_add_ad(168, A::add(A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), s.ad_value(899)), A::mul(s.ad_value(914), s.ad_value(170)));
        }

        if (s.v[1428] != 0.0) {
            s.store_add_ad_lhs(169, A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), 900);
        }

        if (!(s.v[1428] != 0.0)) {
            s.store_add_ad(168, A::sub(s.ad_value(899), s.ad_value(897)), A::mul(s.ad_value(914), s.ad_value(170)));
        }

        if (!(s.v[1428] != 0.0)) {
            s.store_sub(169, 900, 897);
        }

        s.store_div_ad_lhs(170, A::sub(s.ad_value(348), s.ad_value(129)), 181);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_ad(901, &A::limited_exp(s.ad_value(171)));

        s.v[1429] = if (s.v[901] > 1e-7) { 1.0 } else { 0.0 };

        if (s.v[1429] != 0.0) {
            s.store_ln_ad(176, A::offset(s.ad_value(901), 1.0));
        }

        if (s.v[1429] != 0.0) {
            s.store_scale_ad(901, A::sub_from_scalar(1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0))), 2.0);
        }

        if (s.v[1429] != 0.0) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if (s.v[1429] != 0.0) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if (s.v[1429] != 0.0) {
            s.store_mul(174, 177, 172);
        }

        if (s.v[1429] != 0.0) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if (s.v[1429] != 0.0) {
            let assign21560_ad_e39798: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign21560_ad_e39798, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if (s.v[1429] != 0.0) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if (s.v[1429] != 0.0) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if (s.v[1429] != 0.0) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if (s.v[1429] != 0.0) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if (s.v[1429] != 0.0) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if (s.v[1429] != 0.0) {
            s.store_mul(174, 177, 172);
        }

        if (s.v[1429] != 0.0) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if (s.v[1429] != 0.0) {
            let assign21640_ad_e39949: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign21640_ad_e39949, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if (s.v[1429] != 0.0) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if (s.v[1429] != 0.0) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if (s.v[1429] != 0.0) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if (!(s.v[1429] != 0.0)) {
            s.store_mul_ad_lhs(901, A::neg(s.ad_value(901)), 901);
        }

        s.store_mul_ad_lhs(393, A::neg(s.ad_value(901)), 181);

        s.v[1430] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1430] != 0.0) {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(347), s.ad_value(129)), 181);
        }

        if (s.v[1430] != 0.0) {
            s.store_scale_ad(1016, A::add(s.ad_value(1015), A::sqrt(A::add(A::mul(s.ad_value(1015), s.ad_value(1015)), A::mul(A::scale(s.ad_value(963), 0.25), s.ad_value(963))))), 0.5);
        }

        if (s.v[1430] != 0.0) {
            s.store_ad(1017, &A::pow(s.ad_value(1016), A::scale(s.ad_value(960), 0.5)));
        }

        if (s.v[1430] != 0.0) {
            s.store_mul_ad(1007, A::mul(s.ad_value(1010), s.ad_value(1017)), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
        }

        if (s.v[1430] != 0.0) {
            s.store_div_ad_lhs(1018, A::sub(A::sub(s.ad_value(347), s.ad_value(129)), s.ad_value(985)), 181);
        }

        if (s.v[1430] != 0.0) {
            s.store_scale_ad(1019, A::add(s.ad_value(1018), A::sqrt(A::add(A::mul(s.ad_value(1018), s.ad_value(1018)), A::mul(A::scale(s.ad_value(964), 0.25), s.ad_value(964))))), 0.5);
        }

        if (s.v[1430] != 0.0) {
            s.store_ad(1020, &A::pow(s.ad_value(1019), A::scale(s.ad_value(961), 0.5)));
        }

        if (s.v[1430] != 0.0) {
            s.store_mul_ad(1008, A::mul(s.ad_value(1011), s.ad_value(1020)), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
        }

        if (s.v[1430] != 0.0) {
            s.store_div_ad_lhs(1021, A::sub(A::sub(s.ad_value(347), s.ad_value(129)), s.ad_value(986)), 181);
        }

        if (s.v[1430] != 0.0) {
            s.store_scale_ad(1022, A::add(s.ad_value(1021), A::sqrt(A::add(A::mul(s.ad_value(1021), s.ad_value(1021)), A::mul(A::scale(s.ad_value(965), 0.25), s.ad_value(965))))), 0.5);
        }

        if (s.v[1430] != 0.0) {
            s.store_ad(1023, &A::pow(s.ad_value(1022), A::scale(s.ad_value(962), 0.5)));
        }

        if (s.v[1430] != 0.0) {
            s.store_mul_ad(1009, A::mul(s.ad_value(1012), s.ad_value(1023)), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
        }

        if (s.v[1430] != 0.0) {
            s.store_add_ad(393, A::mul(s.ad_value(983), s.ad_value(393)), A::mul(s.ad_value(984), A::add(A::add(s.ad_value(1007), s.ad_value(1008)), s.ad_value(1009))));
        }

        s.v[1431] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1431] != 0.0) {
            s.store_add_ad(356, A::mul(A::mul(A::neg(s.ad_value(297)), s.ad_value(363)), A::add(s.ad_value(127), A::mul(s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01))))), A::mul(A::mul(s.ad_value(681), s.ad_value(365)), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));
        }

        if (s.v[1431] != 0.0) {
            s.store_add_ad_lhs(359, A::add(A::add(A::add(s.ad_value(354), s.ad_value(356)), s.ad_value(357)), s.ad_value(231)), 805);
        }

        if (s.v[1431] != 0.0) {
            s.store_sub_ad_lhs(349, A::sub(s.ad_value(125), s.ad_value(167)), 359);
        }

        if (s.v[1431] != 0.0) {
            s.store_div_ad_lhs(185, A::mul(A::mul(s.ad_value(414), s.ad_value(163)), s.ad_value(158)), 153);
        }

        s.v[1432] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_ad(171, &A::pow(A::div(A::scale(s.ad_value(163), (2.0 * p.p108)), A::scale(A::mul(A::scale(A::mul(s.ad_value(185), s.ad_value(181)), 1.60219e-19), s.ad_value(148)), p.p3)), s.ad_value(181)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if ((s.v[1431] != 0.0) && (s.v[1432] != 0.0)) {
            s.store_offset_ad(169, A::add(s.ad_value(349), s.ad_value(168)), p.p23);
        }

        if ((s.v[1431] != 0.0) && (s.v[1432] != 0.0)) {
            let assign21930_ad_e40368: A = A::sub({
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(168));
            s.store_ad(350, &assign21930_ad_e40368);
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1432] != 0.0))) {
            let assign21940_ad_e40432: A = {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div(A::scale(s.ad_value(163), (2.0 * p.p108)), A::scale(A::mul(A::scale(A::mul(s.ad_value(185), s.ad_value(181)), 1.60219e-19), s.ad_value(148)), p.p3)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad(168, A::neg(s.ad_value(181)), assign21940_ad_e40432);
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1432] != 0.0))) {
            s.store_sub_ad_lhs(169, A::scale(A::add(A::offset(s.ad_value(168), 0.01), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.01)), A::offset(s.ad_value(168), (-0.01))), ((0.25 * 0.0001) * 0.0001)))), 0.5), 375);
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1432] != 0.0))) {
            s.store_offset_ad(170, A::add(s.ad_value(349), s.ad_value(169)), p.p23);
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1432] != 0.0))) {
            let assign21970_ad_e40517: A = A::sub({
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::scale(A::add(s.ad_value(170), A::sqrt(A::offset(A::square(s.ad_value(170)), ((4.0 * 0.0001) * 0.0001)))), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(169));
            s.store_ad(350, &assign21970_ad_e40517);
        }

        if (s.v[1431] != 0.0) {
            s.copy_ad(130, 375);
        }

        if (s.v[1431] != 0.0) {
            s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.v[1433] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1433] != 0.0)) {
            let assign22010_ad_e40608: A = {
                if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::scale(A::add(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367)), A::sqrt(A::offset(A::mul(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367)), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367))), ((4.0 * 0.1) * 0.1)))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(169, &assign22010_ad_e40608);
        }

        if ((s.v[1431] != 0.0) && (s.v[1433] != 0.0)) {
            s.store_mul_ad(171, A::div(A::neg(s.ad_value(239)), A::scale(s.ad_value(181), 2.0)), A::sub(A::sqrt(s.ad_value(169)), A::sqrt(A::scale(s.ad_value(166), 2.0))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1433] != 0.0)) {
            s.store_add_ad(168, A::add(A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), s.ad_value(899)), A::mul(s.ad_value(914), s.ad_value(172)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1433] != 0.0)) {
            s.store_add_ad_lhs(169, A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), 900);
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1433] != 0.0))) {
            s.store_add_ad(168, A::sub(s.ad_value(899), s.ad_value(897)), A::mul(s.ad_value(914), s.ad_value(172)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1433] != 0.0))) {
            s.store_sub(169, 900, 897);
        }

        if (s.v[1431] != 0.0) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(350), s.ad_value(130)), 181);
        }

        if (s.v[1431] != 0.0) {
            s.store_sub(924, 169, 170);
        }

        if (s.v[1431] != 0.0) {
            s.store_scaled_sub(171, 170, 168, 0.5);
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(901, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1434] = if (s.v[901] > 1e-7) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_ln_ad(176, A::offset(s.ad_value(901), 1.0));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_scale_ad(901, A::sub_from_scalar(1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0))), 2.0);
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_mul(174, 177, 172);
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            let assign22180_ad_e40816: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign22180_ad_e40816, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_mul(174, 177, 172);
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            let assign22260_ad_e40983: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign22260_ad_e40983, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1434] != 0.0)) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1434] != 0.0))) {
            s.store_mul_ad_lhs(901, A::neg(s.ad_value(901)), 901);
        }

        if (s.v[1431] != 0.0) {
            s.store_mul_ad_lhs(394, A::neg(s.ad_value(901)), 181);
        }

        s.v[1435] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(349), s.ad_value(130)), 181);
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_scale_ad(1016, A::add(s.ad_value(1015), A::sqrt(A::add(A::mul(s.ad_value(1015), s.ad_value(1015)), A::mul(A::scale(s.ad_value(963), 0.25), s.ad_value(963))))), 0.5);
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_ad(1017, &A::pow(s.ad_value(1016), A::scale(s.ad_value(960), 0.5)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_mul_ad(1004, A::mul(s.ad_value(1010), s.ad_value(1017)), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_div_ad_lhs(1018, A::sub(A::sub(s.ad_value(349), s.ad_value(130)), s.ad_value(985)), 181);
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_scale_ad(1019, A::add(s.ad_value(1018), A::sqrt(A::add(A::mul(s.ad_value(1018), s.ad_value(1018)), A::mul(A::scale(s.ad_value(964), 0.25), s.ad_value(964))))), 0.5);
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_ad(1020, &A::pow(s.ad_value(1019), A::scale(s.ad_value(961), 0.5)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_mul_ad(1005, A::mul(s.ad_value(1011), s.ad_value(1020)), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_div_ad_lhs(1021, A::sub(A::sub(s.ad_value(349), s.ad_value(130)), s.ad_value(986)), 181);
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_scale_ad(1022, A::add(s.ad_value(1021), A::sqrt(A::add(A::mul(s.ad_value(1021), s.ad_value(1021)), A::mul(A::scale(s.ad_value(965), 0.25), s.ad_value(965))))), 0.5);
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_ad(1023, &A::pow(s.ad_value(1022), A::scale(s.ad_value(962), 0.5)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_mul_ad(1006, A::mul(s.ad_value(1012), s.ad_value(1023)), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1435] != 0.0)) {
            s.store_add_ad(394, A::mul(s.ad_value(983), s.ad_value(394)), A::mul(s.ad_value(984), A::add(A::add(s.ad_value(1004), s.ad_value(1005)), s.ad_value(1006))));
        }

        if (s.v[1431] != 0.0) {
            s.store_scale_ad(421, A::add(s.ad_value(396), A::mul(s.ad_value(407), s.ad_value(394))), s.v[420]);
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(170, &A::pow(A::scale(A::offset(A::div(s.ad_value(394), s.ad_value(406)), 1.0), 0.5), s.ad_value(317)));
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(171, &A::pow(s.ad_value(421), s.ad_value(822)));
        }

        s.v[1436] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1436] != 0.0)) {
            s.store_add_ad(171, A::mul(A::add(s.ad_value(304), A::mul(s.ad_value(315), s.ad_value(370))), s.ad_value(171)), A::div(s.ad_value(319), s.ad_value(170)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1436] != 0.0))) {
            s.store_add_ad(171, A::mul(s.ad_value(304), s.ad_value(171)), A::div(s.ad_value(319), s.ad_value(170)));
        }

        if (s.v[1431] != 0.0) {
            s.store_offset(398, 171, 1.0);
        }

        if (s.v[1431] != 0.0) {
            s.store_scale_ad(398, A::add(A::offset(s.ad_value(398), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(398), (-1.0)), A::offset(s.ad_value(398), (-1.0))), ((0.25 * p.p604) * p.p604)))), 0.5);
        }

        if (s.v[1431] != 0.0) {
            s.store_scale(398, 398, 1.0 / (p.p24));
        }

        s.v[1437] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1437] != 0.0)) {
            s.store_scalar(199, 0.0);
        }

        s.v[1438] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(394)), 1.0);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_mul_ad_lhs(199, A::scale(A::mul(A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189)), s.v[115]), 194);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_offset_ad(172, A::mul(s.ad_value(711), s.ad_value(394)), 1.0);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_div_from_scalar(169, 1.0, 172);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if (((s.v[1431] != 0.0) && (!(s.v[1437] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_mul_ad_lhs(199, A::add(A::add(s.ad_value(190), s.ad_value(191)), A::scale(A::mul(A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189)), s.v[115])), 194);
        }

        if (s.v[1431] != 0.0) {
            s.store_mul_ad_lhs(222, A::div(A::scale(s.ad_value(336), 2.0), s.ad_value(414)), 398);
        }

        if (s.v[1431] != 0.0) {
            s.store_mul(223, 222, 153);
        }

        s.v[1439] = if (p.p80 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1439] != 0.0)) {
            s.store_mul_ad_rhs(175, 659, A::add(s.ad_value(394), A::scale(s.ad_value(179), 2.0)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1439] != 0.0))) {
            s.store_mul_ad_rhs(175, 659, A::add(s.ad_value(394), A::scale(s.ad_value(182), 2.0)));
        }

        s.v[1440] = if (s.v[199] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1440] != 0.0)) {
            s.store_mul_ad_lhs(168, A::mul(A::mul(s.ad_value(158), s.ad_value(336)), s.ad_value(163)), 199);
        }

        if ((s.v[1431] != 0.0) && (s.v[1440] != 0.0)) {
            s.store_scale(225, 168, 2.0);
        }

        if ((s.v[1431] != 0.0) && (s.v[1440] != 0.0)) {
            s.store_add_ad(226, A::add(s.ad_value(175), s.ad_value(223)), A::mul(A::scale(s.ad_value(175), 3.0), s.ad_value(168)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1440] != 0.0)) {
            s.store_mul_ad_rhs(227, 175, A::add(s.ad_value(223), A::mul(A::scale(s.ad_value(175), 2.0), s.ad_value(168))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1440] != 0.0)) {
            s.store_div_ad(211, A::sub(A::square(s.ad_value(226)), A::sub(A::square(s.ad_value(226)), A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(227)))), A::mul(A::add(s.ad_value(226), A::sqrt(A::sub(A::square(s.ad_value(226)), A::mul(A::scale(s.ad_value(225), 2.0), s.ad_value(227))))), s.ad_value(225)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1440] != 0.0))) {
            s.store_div_ad(211, A::mul(s.ad_value(223), s.ad_value(175)), A::add(s.ad_value(223), s.ad_value(175)));
        }

        if (s.v[1431] != 0.0) {
            let assign22780_ad_e41709: A = {
                if (!((s.v[211] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::scale(A::add(A::offset(s.ad_value(211), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(211), (-0.001)), A::offset(s.ad_value(211), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5)
                } else {
                    {
                        if ((s.v[211] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_from_scalar(((-1e-5) * 1e-5), A::offset(s.ad_value(211), (-0.001)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(211, assign22780_ad_e41709, 0.001);
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(176, &A::pow(A::offset(A::div(s.ad_value(126), s.ad_value(211)), 1e-6), s.ad_value(423)));
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(177, &A::pow(A::offset(s.ad_value(176), 1.0), s.ad_value(212)));
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(391, &A::min(A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126)));
        }

        if (s.v[1431] != 0.0) {
            s.store_add(130, 391, 375);
        }

        if (s.v[1431] != 0.0) {
            s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.v[1441] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1441] != 0.0)) {
            let assign22850_ad_e41830: A = {
                if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::scale(A::add(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367)), A::sqrt(A::offset(A::mul(A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367)), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367))), ((4.0 * 0.1) * 0.1)))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::sub(A::add(A::scale(s.ad_value(166), 2.0), s.ad_value(130)), s.ad_value(367)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad(169, &assign22850_ad_e41830);
        }

        if ((s.v[1431] != 0.0) && (s.v[1441] != 0.0)) {
            s.store_mul_ad(171, A::div(A::neg(s.ad_value(239)), A::scale(s.ad_value(181), 2.0)), A::sub(A::sqrt(s.ad_value(169)), A::sqrt(A::scale(s.ad_value(166), 2.0))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1441] != 0.0)) {
            s.store_add_ad(168, A::add(A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), s.ad_value(899)), A::mul(s.ad_value(914), s.ad_value(170)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1441] != 0.0)) {
            s.store_add_ad_lhs(169, A::sub(A::neg(s.ad_value(897)), s.ad_value(171)), 900);
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1441] != 0.0))) {
            s.store_add_ad(168, A::sub(s.ad_value(899), s.ad_value(897)), A::mul(s.ad_value(914), s.ad_value(170)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1441] != 0.0))) {
            s.store_sub(169, 900, 897);
        }

        if (s.v[1431] != 0.0) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(350), s.ad_value(130)), 181);
        }

        if (s.v[1431] != 0.0) {
            s.store_sub(924, 169, 170);
        }

        if (s.v[1431] != 0.0) {
            s.store_scaled_sub(171, 170, 168, 0.5);
        }

        if (s.v[1431] != 0.0) {
            s.store_ad(901, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1442] = if (s.v[901] > 1e-7) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_ln_ad(176, A::offset(s.ad_value(901), 1.0));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_scale_ad(901, A::sub_from_scalar(1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0))), 2.0);
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_mul(174, 177, 172);
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            let assign23020_ad_e42038: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign23020_ad_e42038, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_mul_ad_lhs(177, A::add(A::scale(s.ad_value(901), p.p1805), s.ad_value(897)), 898);
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_mul(174, 177, 172);
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_ln_ad(902, A::neg(A::add(s.ad_value(901), s.ad_value(897))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            let assign23100_ad_e42205: A = A::add(A::add(A::sub(s.ad_value(924), s.ad_value(901)), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln(A::neg(s.ad_value(901)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
            s.store_add_ad(344, assign23100_ad_e42205, A::mul(s.ad_value(914), A::exp(A::scale(s.ad_value(902), 0.666666667))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_sub_ad(345, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), A::mul(A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898))), A::mul(A::scale(s.ad_value(914), 0.666666667), A::exp(A::scale(s.ad_value(902), (-0.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_sub_ad(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), A::mul(A::scale(s.ad_value(914), (2.0 / 9.0)), A::exp(A::scale(s.ad_value(902), (-1.333333333)))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1442] != 0.0)) {
            s.store_sub_ad_rhs(901, 901, A::mul(A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div(A::mul(s.ad_value(344), s.ad_value(346)), A::mul(A::scale(s.ad_value(345), 2.0), s.ad_value(345))), 1.0)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1442] != 0.0))) {
            s.store_mul_ad_lhs(901, A::neg(s.ad_value(901)), 901);
        }

        if (s.v[1431] != 0.0) {
            s.store_mul_ad_lhs(395, A::neg(s.ad_value(901)), 181);
        }

        s.v[1443] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(349), s.ad_value(130)), 181);
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_scale_ad(1016, A::add(s.ad_value(1015), A::sqrt(A::add(A::mul(s.ad_value(1015), s.ad_value(1015)), A::mul(A::scale(s.ad_value(963), 0.25), s.ad_value(963))))), 0.5);
        }

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_ad(1017, &A::pow(s.ad_value(1016), A::scale(s.ad_value(960), 0.5)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_mul_ad(1007, A::mul(s.ad_value(1010), s.ad_value(1017)), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_div_ad_lhs(1018, A::sub(A::sub(s.ad_value(349), s.ad_value(130)), s.ad_value(985)), 181);
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_scale_ad(1019, A::add(s.ad_value(1018), A::sqrt(A::add(A::mul(s.ad_value(1018), s.ad_value(1018)), A::mul(A::scale(s.ad_value(964), 0.25), s.ad_value(964))))), 0.5);
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_ad(1020, &A::pow(s.ad_value(1019), A::scale(s.ad_value(961), 0.5)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_mul_ad(1008, A::mul(s.ad_value(1011), s.ad_value(1020)), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_div_ad_lhs(1021, A::sub(A::sub(s.ad_value(349), s.ad_value(130)), s.ad_value(986)), 181);
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_scale_ad(1022, A::add(s.ad_value(1021), A::sqrt(A::add(A::mul(s.ad_value(1021), s.ad_value(1021)), A::mul(A::scale(s.ad_value(965), 0.25), s.ad_value(965))))), 0.5);
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_ad(1023, &A::pow(s.ad_value(1022), A::scale(s.ad_value(962), 0.5)));
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_mul_ad(1009, A::mul(s.ad_value(1012), s.ad_value(1023)), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
        }

        if ((s.v[1431] != 0.0) && (s.v[1443] != 0.0)) {
            s.store_add_ad(395, A::mul(s.ad_value(983), s.ad_value(395)), A::mul(s.ad_value(984), A::add(A::add(s.ad_value(1007), s.ad_value(1008)), s.ad_value(1009))));
        }

        if (s.v[1431] != 0.0) {
            s.store_scaled_add(403, 394, 395, 0.5);
        }

        if (s.v[1431] != 0.0) {
            s.store_sub(405, 394, 395);
        }

        if (s.v[1431] != 0.0) {
            s.store_scale_ad(168, A::square(s.ad_value(391)), 1600.0);
        }

        s.v[1444] = if (p.p603 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1431] != 0.0) && (s.v[1444] != 0.0)) {
            s.store_add_ad(404, A::scale(A::add(s.ad_value(394), s.ad_value(395)), 0.5), A::mul(A::scale(A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(168)))), (p.p603 * 0.5)), s.ad_value(405)));
        }

        if ((s.v[1431] != 0.0) && (!(s.v[1444] != 0.0))) {
            s.store_scaled_add(404, 394, 395, 0.5);
        }

        s.v[1445] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1445] != 0.0) {
            s.store_mul_ad(178, A::div(s.ad_value(239), A::scale(s.ad_value(181), 2.0)), A::sqrt(s.ad_value(179)));
        }

        if (s.v[1445] != 0.0) {
            s.store_scale(168, 178, 0.5);
        }

        if (s.v[1445] != 0.0) {
            let assign23390_ad_e42612: A = A::offset(A::sub(A::sub(s.ad_value(167), s.ad_value(146)), A::mul(s.ad_value(179), {
                if (!((s.v[640] / s.v[148]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] / s.v[148]) > 1e-38) {
                            A::ln(A::div(s.ad_value(640), s.ad_value(148)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            })), p.p1529);
            s.store_div_ad_lhs(170, A::sub(s.ad_value(497), assign23390_ad_e42612), 179);
        }

        s.v[1446] = if ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt()))) { 1.0 } else { 0.0 };

        if ((s.v[1445] != 0.0) && (s.v[1446] != 0.0)) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
        }

        if ((s.v[1445] != 0.0) && (s.v[1446] != 0.0)) {
            s.store_offset_ad(340, A::square(s.ad_value(169)), 1.0);
        }

        if ((s.v[1445] != 0.0) && (s.v[1446] != 0.0)) {
            s.store_ad(175, &{
                if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp(A::neg(s.ad_value(340))), (-1.0))
                } else {
                    {
                        if ((((-s.v[340])) as f64).abs() < 1e-7) {
                            A::sub(A::mul(A::scale(A::neg(s.ad_value(340)), 0.5), A::neg(s.ad_value(340))), s.ad_value(340))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) {
            s.store_sub_ad(171, A::scale(s.ad_value(170), 0.5), A::scale(A::offset(A::scale(s.ad_value(178), 1.0 / (((2.0) as f64).sqrt())), 1.0), 3.0));
        }

        if ((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) {
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add(A::square(s.ad_value(171)), A::scale(s.ad_value(170), 6.0))));
        }

        s.v[1447] = if (s.v[170] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (s.v[1447] != 0.0)) {
            s.store_div_ad_lhs(172, A::sub(s.ad_value(170), s.ad_value(340)), 178);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (s.v[1447] != 0.0)) {
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (s.v[1447] != 0.0)) {
            let assign23490_ad_e42791: A = A::neg({
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
            s.store_ad(340, &assign23490_ad_e42791);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (!(s.v[1447] != 0.0))) {
            s.store_limited_exp_ad(341, A::neg(s.ad_value(340)));
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (!(s.v[1447] != 0.0))) {
            s.store_sub_ad_lhs(172, A::sqrt(A::add(A::add(A::offset(s.ad_value(170), (-1.0)), s.ad_value(341)), A::square(s.ad_value(168)))), 168);
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (!(s.v[1447] != 0.0))) {
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
        }

        if (((s.v[1445] != 0.0) && (!(s.v[1446] != 0.0))) && (!(s.v[1447] != 0.0))) {
            s.store_ad(175, &{
                if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp(A::neg(s.ad_value(340))), (-1.0))
                } else {
                    {
                        if ((((-s.v[340])) as f64).abs() < 1e-7) {
                            A::sub(A::mul(A::scale(A::neg(s.ad_value(340)), 0.5), A::neg(s.ad_value(340))), s.ad_value(340))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1445] != 0.0) {
            s.store_sqrt_ad(176, A::add(s.ad_value(175), s.ad_value(340)));
        }

        s.v[1448] = if (s.v[340] > 1e-15) { 1.0 } else { 0.0 };

        if ((s.v[1445] != 0.0) && (s.v[1448] != 0.0)) {
            s.store_add_ad(344, A::neg(A::sub(s.ad_value(170), s.ad_value(340))), A::mul(s.ad_value(178), s.ad_value(176)));
        }

        if ((s.v[1445] != 0.0) && (s.v[1448] != 0.0)) {
            s.store_sub_from_scalar_ad(345, 1.0, A::div(A::mul(A::scale(s.ad_value(178), 0.5), s.ad_value(175)), s.ad_value(176)));
        }

        if ((s.v[1445] != 0.0) && (s.v[1448] != 0.0)) {
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

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

    }

    pub(super) fn stamp_transient_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        s.store_add_ad_rhs(123, 399, A::mul(A::sub_from_scalar(2.0, s.ad_value(131)), s.ad_value(181)));

        s.store_mul(122, 123, 402);

        s.v[1467] = if (p.p64 == 0.0) { 1.0 } else { 0.0 };

        s.v[1468] = if (p.p64 == 1.0) { 1.0 } else { 0.0 };

        s.v[1469] = if (p.p64 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1467] != 0.0) {
            s.copy_ad(193, 190);
        }

        if (s.v[1467] != 0.0) {
            s.copy_ad(192, 191);
        }

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
            s.store_mul_ad_rhs(193, 194, A::add(s.ad_value(190), A::mul(A::offset(A::mul(s.ad_value(174), s.ad_value(168)), p.p911), s.ad_value(189))));
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

        if ((s.v[1468] != 0.0) && (!(s.v[1467] != 0.0))) {
            s.store_mul_ad_rhs(192, 194, A::add(s.ad_value(191), A::mul(A::offset(A::mul(s.ad_value(174), s.ad_value(168)), p.p914), s.ad_value(189))));
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

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_scalar(193, 0.0);
        }

        if ((s.v[1469] != 0.0) && (!((s.v[1467] != 0.0) || (s.v[1468] != 0.0)))) {
            s.store_scalar(192, 0.0);
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

        s.v[1471] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1471] != 0.0) {
            s.store_div_ad_lhs(571, A::neg(s.ad_value(137)), 138);
        }

        if (!(s.v[1471] != 0.0)) {
            s.store_scalar(571, 0.0);
        }

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

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        if ((s.v[1494] != 0.0) && (s.v[1495] != 0.0)) {
            s.store_scalar(488, 0.0);
        }

        if ((s.v[1494] != 0.0) && (!(s.v[1495] != 0.0))) {
            s.store_div_ad(169, A::neg(s.ad_value(248)), A::offset(s.ad_value(202), 1e-30));
        }

        if ((s.v[1494] != 0.0) && (!(s.v[1495] != 0.0))) {
            s.store_mul_ad(488, A::mul(A::mul(s.ad_value(168), s.ad_value(202)), s.ad_value(124)), A::limited_exp(s.ad_value(169)));
        }

        s.v[1496] = if (p.p71 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) {
            s.store_div_ad_lhs(493, A::add(s.ad_value(261), A::mul(s.ad_value(262), s.ad_value(153))), 153);
        }

        s.v[1497] = if (s.v[493] <= 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (s.v[1497] != 0.0)) {
            s.store_scalar(488, 0.0);
        }

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

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            let assign27440_ad_e47245: A = A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(493)), A::limited_exp(A::div(s.ad_value(489), s.ad_value(169)))), (-(-10.0))), (-p.p1443)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(493)), A::limited_exp(A::div(s.ad_value(489), s.ad_value(169)))), (-(-10.0))), (-p.p1443)), A::offset(A::offset(A::mul(A::neg(s.ad_value(493)), A::limited_exp(A::div(s.ad_value(489), s.ad_value(169)))), (-(-10.0))), (-p.p1443))), (-((4.0 * (-10.0)) * p.p1443)))));
            s.store_neg_ad(492, A::offset(A::scale(assign27440_ad_e47245, 0.5), (-10.0)));
        }

        if (((!(s.v[1494] != 0.0)) && (s.v[1496] != 0.0)) && (!(s.v[1497] != 0.0))) {
            s.store_mul(488, 492, 124);
        }

        s.v[1498] = if (p.p69 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1498] != 0.0) {
            s.store_div_ad_lhs(169, A::div(A::sub(s.ad_value(399), s.ad_value(725)), s.ad_value(726)), 179);
        }

        if (s.v[1498] != 0.0) {
            let assign27480_ad_e47313: A = {
                if ((!(s.v[169] > 37.0)) && (!(s.v[169] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(169)), 1.0))
                } else {
                    {
                        if ((!(s.v[169] > 37.0)) && (s.v[169] < (-37.0))) {
                            A::exp(s.ad_value(169))
                        } else {
                            {
                                if (s.v[169] > 37.0) {
                                    s.ad_value(169)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(460, A::mul(s.ad_value(726), s.ad_value(179)), assign27480_ad_e47313);
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
            s.store_mul_ad_lhs(461, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(158), s.ad_value(153)), s.ad_value(175)), s.ad_value(486)), s.ad_value(497)), s.ad_value(460)), 174);
        }

        if (s.v[1498] != 0.0) {
            s.store_mul_ad_lhs(461, A::scale(s.ad_value(461), p.p27), 256);
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

        if (s.v[1498] != 0.0) {
            let assign27590_ad_e47478: A = {
                if ((!(s.v[169] > 37.0)) && (!(s.v[169] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(169)), 1.0))
                } else {
                    {
                        if ((!(s.v[169] > 37.0)) && (s.v[169] < (-37.0))) {
                            A::exp(s.ad_value(169))
                        } else {
                            {
                                if (s.v[169] > 37.0) {
                                    s.ad_value(169)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_mul_ad(467, A::mul(s.ad_value(731), s.ad_value(179)), assign27590_ad_e47478);
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

        if (s.v[1498] != 0.0) {
            s.store_mul_ad_lhs(469, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(158), s.ad_value(153)), s.ad_value(175)), s.ad_value(486)), s.ad_value(497)), s.ad_value(467)), 174);
        }

        if (s.v[1498] != 0.0) {
            s.store_mul_ad_lhs(469, A::scale(s.ad_value(469), p.p27), 256);
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
            s.store_mul_ad_lhs(472, A::mul(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(158), p.p26), s.ad_value(153)), s.ad_value(484)), s.ad_value(486)), s.ad_value(172)), s.ad_value(174)), 256);
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
            s.store_div_ad_lhs(471, A::mul(s.ad_value(472), s.ad_value(172)), 174);
        }

        if (s.v[1501] != 0.0) {
            s.store_div_ad_lhs(470, A::mul(s.ad_value(472), s.ad_value(171)), 174);
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

        s.v[1504] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1501] != 0.0) && (s.v[1504] != 0.0)) {
            s.store_mul_ad_lhs(480, A::mul(A::mul(A::scale(s.ad_value(462), p.p1104), s.ad_value(134)), s.ad_value(482)), 172);
        }

        if ((s.v[1501] != 0.0) && (!(s.v[1504] != 0.0))) {
            s.store_mul_ad_lhs(481, A::mul(A::mul(A::scale(s.ad_value(462), p.p1104), s.ad_value(134)), s.ad_value(482)), 172);
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

        s.v[1507] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1501] != 0.0) && (s.v[1507] != 0.0)) {
            s.store_mul_ad_lhs(481, A::mul(A::mul(A::scale(s.ad_value(462), p.p1105), s.ad_value(136)), s.ad_value(483)), 172);
        }

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1501] != 0.0) && (!(s.v[1507] != 0.0))) {
            s.store_mul_ad_lhs(480, A::mul(A::mul(A::scale(s.ad_value(462), p.p1105), s.ad_value(136)), s.ad_value(483)), 172);
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

        s.v[1515] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1508] != 0.0) && (s.v[1515] != 0.0)) {
            s.copy_ad(476, 175);
        }

        if ((s.v[1508] != 0.0) && (s.v[1515] != 0.0)) {
            s.copy_ad(478, 176);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1515] != 0.0))) {
            s.copy_ad(475, 175);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1515] != 0.0))) {
            s.copy_ad(477, 176);
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

        s.v[1522] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1508] != 0.0) && (s.v[1522] != 0.0)) {
            s.copy_ad(475, 175);
        }

    }

    pub(super) fn stamp_transient_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1508] != 0.0) && (s.v[1522] != 0.0)) {
            s.copy_ad(477, 176);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1522] != 0.0))) {
            s.copy_ad(476, 175);
        }

        if ((s.v[1508] != 0.0) && (!(s.v[1522] != 0.0))) {
            s.copy_ad(478, 176);
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

        if (((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (s.v[1525] != 0.0)) {
            s.store_mul(519, 169, 170);
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

        if ((((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (!(s.v[1525] != 0.0))) && (s.v[1526] != 0.0)) {
            s.store_mul_ad_rhs(519, 537, A::sub(A::offset(A::add(A::limited_exp(s.ad_value(168)), s.ad_value(547)), (-1.0)), A::scale(s.ad_value(170), p.p1628)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1524] != 0.0)) && (!(s.v[1525] != 0.0))) && (!(s.v[1526] != 0.0))) {
            s.store_add_ad_rhs(519, 545, A::mul(s.ad_value(544), A::sub(s.ad_value(521), s.ad_value(546))));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1524] != 0.0))) {
            s.store_scalar(519, 0.0);
        }

        s.v[1527] = if (s.v[281] > 0.0) { 1.0 } else { 0.0 };

        s.v[1528] = if ((p.p1643 - s.v[521]) < (p.p1643 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (s.v[1528] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 287);
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (s.v[1528] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (s.v[1528] != 0.0)) {
            s.store_sub_ad_rhs(519, 519, A::mul(A::scale(s.ad_value(281), p.p11), s.ad_value(169)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (!(s.v[1528] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 287);
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (!(s.v[1528] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1643), A::sub_from_scalar(p.p1643, s.ad_value(521)))), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1527] != 0.0)) && (!(s.v[1528] != 0.0))) {
            s.store_sub_ad_rhs(519, 519, A::mul(A::scale(s.ad_value(281), p.p11), s.ad_value(169)));
        }

        s.v[1529] = if (s.v[283] > 0.0) { 1.0 } else { 0.0 };

        s.v[1530] = if ((p.p1645 - s.v[521]) < (p.p1645 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (s.v[1530] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 289);
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (s.v[1530] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (s.v[1530] != 0.0)) {
            s.store_sub_ad_rhs(519, 519, A::mul(A::scale(s.ad_value(283), p.p13), s.ad_value(169)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (!(s.v[1530] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 289);
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (!(s.v[1530] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1645), A::sub_from_scalar(p.p1645, s.ad_value(521)))), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1529] != 0.0)) && (!(s.v[1530] != 0.0))) {
            s.store_sub_ad_rhs(519, 519, A::mul(A::scale(s.ad_value(283), p.p13), s.ad_value(169)));
        }

        s.v[1531] = if (s.v[285] > 0.0) { 1.0 } else { 0.0 };

        s.v[1532] = if ((p.p1647 - s.v[521]) < (p.p1647 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 291);
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_sub_ad_rhs(519, 519, A::mul(A::scale(s.ad_value(285), (p.p3 * s.v[115])), s.ad_value(169)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(521)), s.ad_value(180)), 291);
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1647), A::sub_from_scalar(p.p1647, s.ad_value(521)))), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_sub_ad_rhs(519, 519, A::mul(A::scale(s.ad_value(285), (p.p3 * s.v[115])), s.ad_value(169)));
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

        if (((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (s.v[1534] != 0.0)) {
            s.store_mul(520, 169, 170);
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

        if ((((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (!(s.v[1534] != 0.0))) && (s.v[1535] != 0.0)) {
            s.store_mul_ad_rhs(520, 538, A::sub(A::offset(A::add(A::limited_exp(s.ad_value(168)), s.ad_value(554)), (-1.0)), A::scale(s.ad_value(170), p.p1629)));
        }

        if ((((s.v[1523] != 0.0) && (s.v[1533] != 0.0)) && (!(s.v[1534] != 0.0))) && (!(s.v[1535] != 0.0))) {
            s.store_add_ad_rhs(520, 552, A::mul(s.ad_value(551), A::sub(s.ad_value(522), s.ad_value(553))));
        }

        if ((s.v[1523] != 0.0) && (!(s.v[1533] != 0.0))) {
            s.store_scalar(520, 0.0);
        }

        s.v[1536] = if (s.v[282] > 0.0) { 1.0 } else { 0.0 };

        s.v[1537] = if ((p.p1644 - s.v[522]) < (p.p1644 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 288);
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_sub_ad_rhs(520, 520, A::mul(A::scale(s.ad_value(282), p.p12), s.ad_value(169)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 288);
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1644), A::sub_from_scalar(p.p1644, s.ad_value(522)))), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_sub_ad_rhs(520, 520, A::mul(A::scale(s.ad_value(282), p.p12), s.ad_value(169)));
        }

        s.v[1538] = if (s.v[284] > 0.0) { 1.0 } else { 0.0 };

        s.v[1539] = if ((p.p1646 - s.v[522]) < (p.p1646 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (s.v[1539] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 290);
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (s.v[1539] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (s.v[1539] != 0.0)) {
            s.store_sub_ad_rhs(520, 520, A::mul(A::scale(s.ad_value(284), p.p14), s.ad_value(169)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (!(s.v[1539] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 290);
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (!(s.v[1539] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1646), A::sub_from_scalar(p.p1646, s.ad_value(522)))), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1538] != 0.0)) && (!(s.v[1539] != 0.0))) {
            s.store_sub_ad_rhs(520, 520, A::mul(A::scale(s.ad_value(284), p.p14), s.ad_value(169)));
        }

        s.v[1540] = if (s.v[286] > 0.0) { 1.0 } else { 0.0 };

        s.v[1541] = if ((p.p1648 - s.v[522]) < (p.p1648 * 0.001)) { 1.0 } else { 0.0 };

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (s.v[1541] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 292);
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (s.v[1541] != 0.0)) {
            s.store_offset_ad(169, A::limited_exp(A::scale(s.ad_value(168), 1000.0)), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (s.v[1541] != 0.0)) {
            s.store_sub_ad_rhs(520, 520, A::mul(A::scale(s.ad_value(286), (p.p3 * s.v[115])), s.ad_value(169)));
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (!(s.v[1541] != 0.0))) {
            s.store_div_ad_lhs(168, A::div(A::neg(s.ad_value(522)), s.ad_value(180)), 292);
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (!(s.v[1541] != 0.0))) {
            s.store_offset_ad(169, A::limited_exp(A::div(A::scale(s.ad_value(168), p.p1648), A::sub_from_scalar(p.p1648, s.ad_value(522)))), (-1.0));
        }

        if (((s.v[1523] != 0.0) && (s.v[1540] != 0.0)) && (!(s.v[1541] != 0.0))) {
            s.store_sub_ad_rhs(520, 520, A::mul(A::scale(s.ad_value(286), (p.p3 * s.v[115])), s.ad_value(169)));
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

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        if (s.v[1674] != 0.0) {
            s.store_sub_ad_rhs(509, 509, A::mul(A::scale(s.ad_value(156), s.v[115]), A::mul(s.ad_value(650), A::add(A::sub(s.ad_value(171), s.ad_value(512)), A::mul(A::scale(s.ad_value(653), 0.5), A::offset(A::sqrt(A::offset(A::div(A::scale(s.ad_value(512), 4.0), s.ad_value(653)), 1.0)), (-1.0)))))));
        }

        s.store_mul_ad_rhs(169, 126, A::add(s.ad_value(865), A::mul(A::mul(s.ad_value(866), s.ad_value(126)), s.ad_value(126))));

        s.store_mul_ad_lhs(556, A::mul(A::scale(A::sub(s.ad_value(153), A::scale(s.ad_value(875), 2.0)), (p.p92 * p.p3)), s.ad_value(555)), 169);

        s.store_div_ad_lhs(168, A::mul(A::mul(s.ad_value(415), s.ad_value(372)), s.ad_value(158)), 153);

        s.v[1675] = if ((p.p73 != 0.0) && (s.v[873] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1675] != 0.0) {
            s.store_div_ad(572, A::mul(A::mul(s.ad_value(183), s.ad_value(123)), s.ad_value(205)), A::mul(A::mul(s.ad_value(411), s.ad_value(209)), s.ad_value(188)));
        }

        if (s.v[1675] != 0.0) {
            s.store_mul_ad(569, A::scale(s.ad_value(873), s.v[115]), A::add(s.ad_value(572), A::mul(A::mul(s.ad_value(874), s.ad_value(179)), s.ad_value(168))));
        }

        s.v[1676] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[1676] != 0.0) {
            s.store_div_ad(570, A::scale(s.ad_value(569), 1e-9), A::mul(A::mul(s.ad_value(163), s.ad_value(158)), s.ad_value(153)));
        }

        s.store_scale(476, 476, s.v[115]);

        s.store_scale(475, 475, s.v[115]);

        s.store_scale(478, 478, s.v[115]);

        s.store_scale(477, 477, s.v[115]);

        s.store_scale(471, 471, s.v[115]);

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_scale(470, 470, s.v[115]);

        s.store_scale(480, 480, s.v[115]);

        s.store_scale(481, 481, s.v[115]);

        s.store_scale(461, 461, s.v[115]);

        s.store_scale(469, 469, s.v[115]);

        s.store_scale(556, 556, s.v[115]);

        s.v[1677] = if (p.p61 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1677] != 0.0) {
            s.store_mul_ad_lhs(464, A::add(s.ad_value(461), s.ad_value(469)), 186);
        }

        if (s.v[1677] != 0.0) {
            s.store_mul_ad_lhs(465, A::add(s.ad_value(461), s.ad_value(469)), 187);
        }

        s.store_mul(4, 114, 124);

        s.store_scalar(20, A::ddx_projection(&s.ad_value(4), Some(11), None));

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

        s.v[1683] = if (s.v[169] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (s.v[1683] != 0.0)) {
            s.store_div_ad_lhs(588, A::mul(s.ad_value(586), s.ad_value(587)), 169);
        }

        if (((s.v[1678] != 0.0) && (s.v[1680] != 0.0)) && (!(s.v[1683] != 0.0))) {
            s.store_scalar(588, 0.0);
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

        if (((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) {
            s.store_add_ad(591, A::add(s.ad_value(573), A::scale(s.ad_value(399), p.p1683)), A::mul(A::scale(s.ad_value(399), p.p1684), s.ad_value(399)));
        }

        s.v[1685] = if ((s.v[169] > 0.0) && (s.v[170] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            s.store_powf(590, 170, (-p.p1686));
        }

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

        if ((((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) && (s.v[1685] != 0.0)) {
            let assign33270_ad_e55792: A = A::div(A::mul(A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::scale(s.ad_value(179), ((1.602176634e-19 * 1.602176634e-19) * 1.602176634e-19))), s.ad_value(20)), s.ad_value(20)), s.ad_value(591)), s.ad_value(590)), A::scale(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(163), 100000000000000.0), s.ad_value(163)), s.ad_value(158)), s.ad_value(158)), s.ad_value(580)), s.v[115]));
            s.store_ad(588, &assign33270_ad_e55792);
        }

        if ((((s.v[1678] != 0.0) && (!(s.v[1680] != 0.0))) && (s.v[1684] != 0.0)) && (!(s.v[1685] != 0.0))) {
            s.store_scalar(588, 0.0);
        }

        if (!(s.v[1678] != 0.0)) {
            s.store_scalar(588, 0.0);
        }

        s.v[1686] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        s.v[1687] = if (p.p72 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1686] != 0.0) {
            s.store_mul(168, 415, 592);
        }

        if (s.v[1686] != 0.0) {
            s.store_add_ad(169, A::mul(s.ad_value(168), s.ad_value(197)), A::square(s.ad_value(153)));
        }

        if (s.v[1686] != 0.0) {
            s.store_scaled_div(593, 168, 169, p.p1707);
        }

        if (s.v[1686] != 0.0) {
            s.store_mul_ad_lhs(594, A::scale(s.ad_value(179), (4.0 * 1.60219e-19)), 593);
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

    }

    pub(super) fn stamp_transient_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
            s.store_div(628, 174, 176);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_rhs(629, 606, A::scale(s.ad_value(609), 3.0));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(630, A::mul(A::scale(s.ad_value(205), 0.16666666666666666), s.ad_value(209)), A::sub(s.ad_value(628), s.ad_value(629)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad(631, A::mul(s.ad_value(171), s.ad_value(630)), A::sqrt(A::mul(s.ad_value(623), s.ad_value(627))));
        }

        s.v[1693] = if (s.v[631] > 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (s.v[1693] != 0.0)) {
            s.store_scalar(631, 1.0);
        }

        s.v[1694] = if (s.v[631] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) && (!(s.v[1693] != 0.0))) && (s.v[1694] != 0.0)) {
            s.store_scalar(631, 0.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_offset_ad(177, A::mul(A::div(A::square(s.ad_value(600)), A::offset(s.ad_value(399), p.p1716)), A::div(s.ad_value(390), s.ad_value(210))), 1.0);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(623, A::div(s.ad_value(205), s.ad_value(209)), A::add(A::mul(s.ad_value(177), s.ad_value(621)), A::mul(s.ad_value(169), s.ad_value(622))));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad_lhs(594, A::mul(A::scale(s.ad_value(179), (4.0 * 1.60219e-19)), s.ad_value(623)), 603);
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_mul_ad(627, A::mul(A::mul(A::scale(s.ad_value(205), 0.16666666666666666), s.ad_value(604)), s.ad_value(170)), A::add(A::sub(s.ad_value(624), s.ad_value(625)), s.ad_value(626)));
        }

        if ((s.v[1687] != 0.0) && (!(s.v[1686] != 0.0))) {
            s.store_div_ad_lhs(632, A::mul(A::mul(A::mul(A::scale(A::sqrt(A::div(s.ad_value(627), s.ad_value(623))), s.v[115]), s.ad_value(372)), s.ad_value(159)), s.ad_value(156)), 603);
        }

        s.v[1695] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        s.v[1696] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        s.v[1697] = if (p.p65 == 1.0) { 1.0 } else { 0.0 };

        s.v[1698] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        s.v[1699] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1700] = if ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };

        s.v[1701] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1702] = if ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };

        s.v[1703] = if (p.p61 == 0.0) { 1.0 } else { 0.0 };

        s.v[1704] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

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

        if ((s.v[1711] != 0.0) && (s.v[1712] != 0.0)) {
            s.store_mul_ad_rhs(1041, 172, A::powf(A::offset(A::powf(s.ad_value(175), p.p1908), 1.0), (1.0 / p.p1908)));
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

        if ((s.v[1711] != 0.0) && (s.v[1715] != 0.0)) {
            s.store_mul_ad_rhs(1040, 172, A::powf(A::offset(A::powf(s.ad_value(175), p.p1908), 1.0), (1.0 / p.p1908)));
        }

        s.v[1718] = if ((p.p64 != 2.0) && (s.v[191] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1718] != 0.0) {
            s.store_div_from_scalar(596, 1.0, 192);
        }

        s.v[1719] = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1718] != 0.0) && (s.v[1719] != 0.0)) {
            s.store_div_from_scalar(1042, 1.0, 1041);
        }

        s.v[1720] = if ((p.p64 != 2.0) && (s.v[190] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1720] != 0.0) {
            s.store_div_from_scalar(595, 1.0, 193);
        }

        s.v[1721] = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1720] != 0.0) && (s.v[1721] != 0.0)) {
            s.store_div_from_scalar(1043, 1.0, 1040);
        }

        s.v[1722] = if ((p.p73 == 1.0) && (s.v[873] != 0.0)) { 1.0 } else { 0.0 };

        s.v[1723] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        s.v[1724] = if (p.p76 != 0.0) { 1.0 } else { 0.0 };

        s.v[1725] = if (p.p76 == 2.0) { 1.0 } else { 0.0 };

        s.v[1726] = if ((p.p64 != 2.0) && (s.v[191] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1727] = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1728] = if ((p.p64 != 2.0) && (s.v[190] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1729] = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1730] = if (p.p76 != 0.0) { 1.0 } else { 0.0 };

        s.v[1731] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        s.v[1732] = if (p.p68 != 0.0) { 1.0 } else { 0.0 };

        s.v[1733] = if (s.v[128] > 0.0) { 1.0 } else { 0.0 };

        s.v[1734] = if (p.p69 != 0.0) { 1.0 } else { 0.0 };

        s.v[1735] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1736] = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1737] = if ((p.p64 != 2.0) && (s.v[191] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1738] = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1739] = if ((p.p64 != 2.0) && (s.v[190] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1740] = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };

        s.store_mul(4, 114, 124);

        s.store_add_ad(339, A::div(A::scale(s.ad_value(179), 10.0), s.ad_value(898)), A::scale(s.ad_value(396), 2.0));

        s.store_mul_ad_rhs(169, 179, A::add(s.ad_value(179), s.ad_value(339)));

        s.store_mul_ad_lhs(170, A::square(s.ad_value(163)), 169);

        s.store_mul_ad_lhs(171, A::scale(s.ad_value(141), ((2.0 * 1.60219e-19) * s.v[143])), 179);

        s.store_scalar(20, A::ddx_projection(&s.ad_value(4), Some(11), None));

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[188] = 0.0;

        s.v[197] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 0.0;

        s.v[265] = 0.0;

        s.v[266] = 0.0;

        s.v[267] = 0.0;

        s.v[268] = 0.0;

        s.v[269] = 0.0;

        s.v[270] = 0.0;

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.v[277] = 0.0;

        s.v[278] = 0.0;

        s.v[279] = 0.0;

        s.v[280] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[284] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[287] = 0.0;

        s.v[288] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.v[292] = 0.0;

        s.v[300] = 0.0;

        s.v[302] = 0.0;

        s.v[305] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[316] = 0.0;

        s.v[320] = 0.0;

        s.v[333] = 0.0;

        s.v[335] = 0.0;

        s.v[338] = 0.0;

        s.v[258] = 0.0;

        s.v[857] = 0.0;

        s.v[373] = 0.0;

        s.v[401] = 0.0;

        s.v[417] = 0.0;

        s.v[453] = 0.0;

        s.v[756] = 0.0;

        s.v[757] = 0.0;

        s.v[255] = 0.0;

        s.v[758] = 0.0;

        s.v[759] = 0.0;

        s.v[760] = 0.0;

        s.v[770] = 0.0;

        s.v[771] = 0.0;

        s.v[251] = 0.0;

        s.v[772] = 0.0;

        s.v[773] = 0.0;

        s.v[774] = 0.0;

        s.v[494] = 0.0;

        s.v[495] = 0.0;

        s.v[496] = 0.0;

        s.v[498] = 0.0;

        s.v[499] = 0.0;

        s.v[523] = 0.0;

        s.v[524] = 0.0;

        s.v[525] = 0.0;

        s.v[526] = 0.0;

        s.v[527] = 0.0;

        s.v[528] = 0.0;

        s.v[529] = 0.0;

        s.v[533] = 0.0;

        s.v[537] = 0.0;

        s.v[538] = 0.0;

        s.v[539] = 0.0;

        s.v[540] = 0.0;

        s.v[546] = 0.0;

        s.v[547] = 0.0;

        s.v[541] = 0.0;

        s.v[542] = 0.0;

        s.v[543] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[548] = 0.0;

        s.v[549] = 0.0;

        s.v[550] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[567] = 0.0;

        s.v[568] = 0.0;

        s.v[589] = 0.0;

        s.v[574] = 0.0;

        s.v[575] = 0.0;

        s.v[620] = 0.0;

        s.v[632] = 0.0;

        s.v[634] = 0.0;

        s.v[668] = 0.0;

        s.v[665] = 0.0;

        s.v[677] = 0.0;

        s.v[806] = 0.0;

        s.v[370] = 0.0;

        s.v[689] = 0.0;

        s.v[690] = 0.0;

        s.v[691] = 0.0;

        s.v[692] = 0.0;

        s.v[693] = 0.0;

        s.v[871] = 0.0;

        s.v[872] = 0.0;

        s.v[680] = 0.0;

        s.v[699] = 0.0;

        s.v[658] = 0.0;

        s.v[791] = 0.0;

        s.v[701] = 0.0;

        s.v[851] = 0.0;

        s.v[706] = 0.0;

        s.v[710] = 0.0;

        s.v[815] = 0.0;

        s.v[809] = 0.0;

        s.v[817] = 0.0;

        s.v[816] = 0.0;

        s.v[818] = 0.0;

        s.v[845] = 0.0;

        s.v[846] = 0.0;

        s.v[825] = 0.0;

        s.v[828] = 0.0;

        s.v[843] = 0.0;

        s.v[844] = 0.0;

        s.v[715] = 0.0;

        s.v[717] = 0.0;

        s.v[796] = 0.0;

        s.v[646] = 0.0;

        s.v[647] = 0.0;

        s.v[645] = 0.0;

        s.v[644] = 0.0;

        s.v[893] = 0.0;

        s.v[894] = 0.0;

        s.v[895] = 0.0;

        s.v[896] = 0.0;

        s.v[898] = 0.0;

        s.v[903] = 0.0;

        s.v[904] = 0.0;

        s.v[923] = 0.0;

        s.v[392] = 0.0;

        s.v[393] = 0.0;

        s.v[503] = 0.0;

        s.v[504] = 0.0;

        s.v[949] = 0.0;

        s.v[950] = 0.0;

        s.v[951] = 0.0;

        s.v[952] = 0.0;

        s.v[953] = 0.0;

        s.v[955] = 0.0;

        s.v[956] = 0.0;

        s.v[957] = 0.0;

        s.v[958] = 0.0;

        s.v[959] = 0.0;

        s.v[1004] = 0.0;

        s.v[1005] = 0.0;

        s.v[1006] = 0.0;

        s.v[1007] = 0.0;

        s.v[1008] = 0.0;

        s.v[1009] = 0.0;

        s.v[983] = 1.0;

        s.v[960] = 0.0;

        s.v[961] = 0.0;

        s.v[962] = 0.0;

        s.v[963] = 0.0;

        s.v[964] = 0.0;

        s.v[965] = 0.0;

        s.v[984] = 0.0;

        s.v[985] = 0.0;

        s.v[986] = 0.0;

        s.v[1010] = 0.0;

        s.v[1011] = 0.0;

        s.v[1012] = 0.0;

        s.v[882] = 0.0;

        s.v[883] = 0.0;

        s.v[884] = 0.0;

        s.v[885] = 0.0;

        s.v[886] = 0.0;

        s.v[887] = 0.0;

        s.v[888] = 0.0;

        s.v[889] = 0.0;

        s.v[890] = 0.0;

        s.v[891] = 0.0;

        s.v[892] = 0.0;

        s.v[119] = 0.0;

        s.v[120] = 0.0;

        s.v[118] = 0.0;

        s.v[117] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[182] = 0.0;

        s.v[142] = 0.0;

        s.v[324] = 0.0;

        s.v[327] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[310] = 0.0;

        s.v[311] = 0.0;

        s.v[313] = 0.0;

        s.v[312] = 0.0;

        s.v[331] = 0.0;

        s.v[330] = 0.0;

        s.v[1039] = 0.0;

        s.v[446] = 0.0;

        s.v[576] = 0.0;

        s.v[1057] = if (p.p60 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1057] != 0.0) {
            s.store_scalar(114, 1.0);
        }

        if (!(s.v[1057] != 0.0)) {
            s.store_scalar(114, (-1.0));
        }

        s.v[143] = (p.p103 * 8.8542e-12);

        s.v[144] = (p.p1088 * 8.8542e-12);

        s.v[165] = ((p.p102 * 8.8542e-12) / p.p91);

        s.v[145] = (p.p103 / p.p102);

        s.v[381] = (0.916 * 9.11e-31);

        s.v[382] = (0.19 * 9.11e-31);

        s.v[383] = (0.19 * 9.11e-31);

        s.v[384] = (0.417 * 9.11e-31);

        s.v[385] = 4.0;

        s.v[386] = 2.0;

        s.v[876] = (((p.p109 + ((1e-6 * p.p110) / p.p0)) + (p.p111 / p.p5)) + ((p.p112 * 1e-6) / (p.p0 * p.p5)));

        s.v[878] = (((p.p117 + ((1e-6 * p.p118) / p.p0)) + (p.p119 / p.p5)) + ((p.p120 * 1e-6) / (p.p0 * p.p5)));

        s.v[877] = (((p.p113 + ((1e-6 * p.p114) / p.p0)) + (p.p115 / p.p5)) + ((p.p116 * 1e-6) / (p.p0 * p.p5)));

        s.v[149] = (p.p0 + s.v[876]);

        s.v[1058] = if (s.v[149] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1058] != 0.0) {
            s.store_scalar(149, p.p0);
        }

        s.store_powf(168, 149, (-p.p84));

        s.store_offset_scaled(150, 168, p.p83, s.v[877]);

        s.store_offset_ad(151, A::scale(A::powf(A::offset(s.ad_value(149), s.v[878]), (-p.p84)), p.p83), s.v[877]);

        s.store_offset_scaled(152, 168, p.p88, p.p85);

        s.store_sub_ad_rhs(153, 149, A::scale(s.ad_value(150), 2.0));

        s.store_sub_ad(155, A::offset(s.ad_value(149), s.v[878]), A::scale(s.ad_value(151), 2.0));

        s.store_sub_ad_rhs(156, 149, A::scale(s.ad_value(152), 2.0));

        s.store_offset(157, 156, (-p.p86));

        s.v[1059] = if (s.v[153] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1059] != 0.0) {
            s.copy_ad(153, 149);
        }

        s.v[1061] = if (s.v[155] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1061] != 0.0) {
            s.copy_ad(155, 149);
        }

        s.v[1063] = if (s.v[156] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1063] != 0.0) {
            s.copy_ad(156, 149);
        }

        s.v[1065] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1066] = if (s.v[157] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1065] != 0.0) && (s.v[1066] != 0.0)) {
            s.copy_ad(157, 149);
        }

        s.v[1068] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if (s.v[1068] != 0.0) {
            s.store_scalar(879, (((((p.p121 + ((1e-6 * p.p122) / p.p0)) + (p.p123 / p.p5)) + ((p.p124 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p125) / p.p43)) + ((p.p126 * 1e-12) / (p.p0 * p.p43))));
        }

        if (s.v[1068] != 0.0) {
            s.store_scalar(880, (((((p.p127 + ((1e-6 * p.p128) / p.p0)) + (p.p129 / p.p5)) + ((p.p130 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p131) / p.p43)) + ((p.p132 * 1e-12) / (p.p0 * p.p43))));
        }

        if (!(s.v[1068] != 0.0)) {
            s.store_scalar(879, 0.0);
        }

        if (!(s.v[1068] != 0.0)) {
            s.store_scalar(880, 0.0);
        }

        s.store_offset(161, 879, p.p43);

        s.store_add(162, 161, 880);

        s.v[1069] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1070] = if (s.v[162] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1069] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_scalar(162, p.p43);
        }

        s.v[115] = (p.p5 * p.p59);

        s.store_div_from_scalar(635, 1e-6, 155);

        s.v[636] = (1.0 / p.p5);

        s.store_div_from_scalar_ad(637, 1e-6, A::scale(s.ad_value(155), p.p5));

        s.v[1072] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if (s.v[1072] != 0.0) {
            s.store_div_from_scalar(638, 1e-6, 162);
        }

        if (s.v[1072] != 0.0) {
            s.store_div_from_scalar_ad(639, 1e-12, A::mul(s.ad_value(162), s.ad_value(155)));
        }

        if (!(s.v[1072] != 0.0)) {
            s.store_scalar(638, 0.0);
        }

        if (!(s.v[1072] != 0.0)) {
            s.store_scalar(639, 0.0);
        }

        s.store_add_ad(640, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p134), p.p133), (s.v[636] * p.p135)), A::scale(s.ad_value(637), p.p136)), A::scale(s.ad_value(638), 0.0)), A::scale(s.ad_value(639), 0.0));

        s.v[1073] = if (p.p95 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1073] != 0.0) {
            s.store_scale(640, 640, (1.0 + ((p.p95 / p.p5) * (if (!((1.0 + (p.p5 / p.p96)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p96)) > 1e-38) { (((1.0 + (p.p5 / p.p96))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1074] = if (s.v[640] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1074] != 0.0) {
            s.store_scalar(640, 1e22);
        }

        s.v[1076] = if (p.p62 == 0.0) { 1.0 } else { 0.0 };

        s.v[1077] = if (p.p62 == 1.0) { 1.0 } else { 0.0 };

        s.v[1078] = if (p.p62 == 2.0) { 1.0 } else { 0.0 };

        s.v[1079] = if (p.p62 == 3.0) { 1.0 } else { 0.0 };

        s.v[1080] = if (p.p62 == 4.0) { 1.0 } else { 0.0 };

        s.v[1081] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1082] = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1076] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_scalar(895, (2.0 * p.p92));
        }

        if ((s.v[1076] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if ((s.v[1076] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.v[1076] != 0.0) && (!(s.v[1082] != 0.0))) {
            s.store_scalar(895, (2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()));
        }

        if ((s.v[1076] != 0.0) && (!(s.v[1082] != 0.0))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if ((s.v[1076] != 0.0) && (!(s.v[1082] != 0.0))) {
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.v[1083] = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (s.v[1083] != 0.0)) {
            s.store_scalar(895, ((2.0 * p.p92) + p.p3));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (s.v[1083] != 0.0)) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (s.v[1083] != 0.0)) {
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (!(s.v[1083] != 0.0))) {
            s.store_scalar(895, ((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (!(s.v[1083] != 0.0))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1077] != 0.0) && (!(s.v[1076] != 0.0))) && (!(s.v[1083] != 0.0))) {
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.v[1084] = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (s.v[1084] != 0.0)) {
            s.store_scalar(895, ((2.0 * p.p92) + (2.0 * p.p3)));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (s.v[1084] != 0.0)) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (s.v[1084] != 0.0)) {
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (!(s.v[1084] != 0.0))) {
            s.store_scalar(895, (((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802) + p.p1803));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (!(s.v[1084] != 0.0))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        if (((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) && (!(s.v[1084] != 0.0))) {
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        if ((s.v[1078] != 0.0) && (!((s.v[1076] != 0.0) || (s.v[1077] != 0.0)))) {
            s.store_scalar(896, p.p1803);
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(895, (3.141592653589793 * p.p2));
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(893, ((((2.0 * 3.141592653589793) * p.p102) * 8.8542e-12) / (if (!((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38) { (((1.0 + ((2.0 * p.p89) / p.p2))) as f64).ln() } else { 0.0 }) })));
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(894, (((3.141592653589793 * p.p2) * p.p2) / 4.0));
        }

        if ((s.v[1079] != 0.0) && (!(((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)))) {
            s.store_scalar(896, p.p2);
        }

        if ((s.v[1080] != 0.0) && (!((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)))) {
            s.store_scalar(895, p.p1801);
        }

        if ((s.v[1080] != 0.0) && (!((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)))) {
            s.store_scalar(893, p.p1800);
        }

        if ((s.v[1080] != 0.0) && (!((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)))) {
            s.store_scalar(894, p.p1799);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_offset_ad(954, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p44);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_offset_scaled(948, 161, p.p40, p.p45);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.copy_ad(895, 954);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.copy_ad(894, 948);
        }

        s.v[1085] = if (p.p56 > 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_offset_ad(955, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p46);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_offset_scaled(949, 161, p.p40, p.p47);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_add(895, 954, 955);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1085] != 0.0)) {
            s.store_add(894, 948, 949);
        }

        s.v[1086] = if (p.p56 > 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_offset_ad(956, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p48);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_offset_scaled(950, 161, p.p40, p.p49);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(s.ad_value(954), s.ad_value(955)), 956);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1086] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(s.ad_value(948), s.ad_value(949)), 950);
        }

        s.v[1087] = if (p.p56 > 3.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_offset_ad(957, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p50);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_offset_scaled(951, 161, p.p40, p.p51);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(A::add(s.ad_value(954), s.ad_value(955)), s.ad_value(956)), 957);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1087] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(A::add(s.ad_value(948), s.ad_value(949)), s.ad_value(950)), 951);
        }

        s.v[1088] = if (p.p56 > 4.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_offset_ad(958, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p52);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_offset_scaled(952, 161, p.p40, p.p53);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(A::add(A::add(s.ad_value(954), s.ad_value(955)), s.ad_value(956)), s.ad_value(957)), 958);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1088] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(A::add(A::add(s.ad_value(948), s.ad_value(949)), s.ad_value(950)), s.ad_value(951)), 952);
        }

        s.v[1089] = if (p.p56 > 5.0) { 1.0 } else { 0.0 };

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_offset_ad(959, A::scale(A::offset(s.ad_value(161), p.p40), 2.0), p.p54);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_offset_scaled(953, 161, p.p40, p.p55);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_add_ad_lhs(895, A::add(A::add(A::add(A::add(s.ad_value(954), s.ad_value(955)), s.ad_value(956)), s.ad_value(957)), s.ad_value(958)), 959);
        }

        if (((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) && (s.v[1089] != 0.0)) {
            s.store_add_ad_lhs(894, A::add(A::add(A::add(A::add(s.ad_value(948), s.ad_value(949)), s.ad_value(950)), s.ad_value(951)), s.ad_value(952)), 953);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_scalar(896, p.p43);
        }

        if ((s.v[1081] != 0.0) && (!(((((s.v[1076] != 0.0) || (s.v[1077] != 0.0)) || (s.v[1078] != 0.0)) || (s.v[1079] != 0.0)) || (s.v[1080] != 0.0)))) {
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        s.store_div_ad(898, A::scale(s.ad_value(893), 2.0), A::div(A::scale(A::square(s.ad_value(895)), s.v[143]), s.ad_value(894)));

        s.store_div_ad_lhs(903, A::mul(A::scale(s.ad_value(640), (-1.60219e-19)), s.ad_value(894)), 893);

        s.store_div(163, 893, 895);

        s.v[1090] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1090] != 0.0) {
            s.store_scale(494, 163, (p.p89 * 1.0 / (p.p1528)));
        }

        s.store_offset(158, 895, (-p.p93));

        s.store_offset(159, 895, (-p.p94));

        s.v[1091] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        if (s.v[1091] != 0.0) {
            s.store_offset(160, 158, (-((2.0 * p.p56) * p.p87)));
        }

        if (!(s.v[1091] != 0.0)) {
            s.copy_ad(160, 158);
        }

        s.v[1092] = if (p.p62 == 5.0) { 1.0 } else { 0.0 };

        s.v[1093] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        s.v[1094] = if (s.v[160] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1092] != 0.0) && (s.v[1093] != 0.0)) && (s.v[1094] != 0.0)) {
            s.copy_ad(160, 895);
        }

        s.v[446] = p.p1085;

        s.store_add_ad(641, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p138), p.p137), (s.v[636] * p.p139)), A::scale(s.ad_value(637), p.p140)), A::scale(s.ad_value(638), p.p141)), A::scale(s.ad_value(639), p.p142));

        s.store_add_ad(666, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p189), p.p188), (s.v[636] * p.p190)), A::scale(s.ad_value(637), p.p191)), A::scale(s.ad_value(638), p.p192)), A::scale(s.ad_value(639), p.p193));

        s.store_add_ad(662, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p201), p.p200), (s.v[636] * p.p202)), A::scale(s.ad_value(637), p.p203)), A::scale(s.ad_value(638), p.p204)), A::scale(s.ad_value(639), p.p205));

        s.store_add_ad(663, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p207), p.p206), (s.v[636] * p.p208)), A::scale(s.ad_value(637), p.p209)), A::scale(s.ad_value(638), p.p210)), A::scale(s.ad_value(639), p.p211));

        s.store_add_ad(667, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p219), p.p218), (s.v[636] * p.p220)), A::scale(s.ad_value(637), p.p221)), A::scale(s.ad_value(638), p.p222)), A::scale(s.ad_value(639), p.p223));

        s.store_add_ad(670, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p225), p.p224), (s.v[636] * p.p226)), A::scale(s.ad_value(637), p.p227)), A::scale(s.ad_value(638), p.p228)), A::scale(s.ad_value(639), p.p229));

        s.store_add_ad(671, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p231), p.p230), (s.v[636] * p.p232)), A::scale(s.ad_value(637), p.p233)), A::scale(s.ad_value(638), p.p234)), A::scale(s.ad_value(639), p.p235));

        s.store_add_ad(672, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p237), p.p236), (s.v[636] * p.p238)), A::scale(s.ad_value(637), p.p239)), A::scale(s.ad_value(638), p.p240)), A::scale(s.ad_value(639), p.p241));

        s.store_add_ad(673, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p243), p.p242), (s.v[636] * p.p244)), A::scale(s.ad_value(637), p.p245)), A::scale(s.ad_value(638), p.p246)), A::scale(s.ad_value(639), p.p247));

        s.store_add_ad(674, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p249), p.p248), (s.v[636] * p.p250)), A::scale(s.ad_value(637), p.p251)), A::scale(s.ad_value(638), p.p252)), A::scale(s.ad_value(639), p.p253));

        s.store_add_ad(678, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p267), p.p266), (s.v[636] * p.p268)), A::scale(s.ad_value(637), p.p269)), A::scale(s.ad_value(638), p.p270)), A::scale(s.ad_value(639), p.p271));

        s.store_add_ad(802, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p273), p.p272), (s.v[636] * p.p274)), A::scale(s.ad_value(637), p.p275)), A::scale(s.ad_value(638), p.p276)), A::scale(s.ad_value(639), p.p277));

        s.store_add_ad(803, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p279), p.p278), (s.v[636] * p.p280)), A::scale(s.ad_value(637), p.p281)), A::scale(s.ad_value(638), p.p282)), A::scale(s.ad_value(639), p.p283));

        s.store_add_ad(804, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p285), p.p284), (s.v[636] * p.p286)), A::scale(s.ad_value(637), p.p287)), A::scale(s.ad_value(638), p.p288)), A::scale(s.ad_value(639), p.p289));

        s.store_add_ad(683, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p297), p.p296), (s.v[636] * p.p298)), A::scale(s.ad_value(637), p.p299)), A::scale(s.ad_value(638), p.p300)), A::scale(s.ad_value(639), p.p301));

        s.store_add_ad(684, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p303), p.p302), (s.v[636] * p.p304)), A::scale(s.ad_value(637), p.p305)), A::scale(s.ad_value(638), p.p306)), A::scale(s.ad_value(639), p.p307));

        s.store_add_ad(685, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p309), p.p308), (s.v[636] * p.p310)), A::scale(s.ad_value(637), p.p311)), A::scale(s.ad_value(638), p.p312)), A::scale(s.ad_value(639), p.p313));

        s.store_add_ad(686, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p315), p.p314), (s.v[636] * p.p316)), A::scale(s.ad_value(637), p.p317)), A::scale(s.ad_value(638), p.p318)), A::scale(s.ad_value(639), p.p319));

        s.store_add_ad(687, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p321), p.p320), (s.v[636] * p.p322)), A::scale(s.ad_value(637), p.p323)), A::scale(s.ad_value(638), p.p324)), A::scale(s.ad_value(639), p.p325));

        s.store_add_ad(688, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p327), p.p326), (s.v[636] * p.p328)), A::scale(s.ad_value(637), p.p329)), A::scale(s.ad_value(638), p.p330)), A::scale(s.ad_value(639), p.p331));

        s.store_add_ad(867, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p333), p.p332), (s.v[636] * p.p334)), A::scale(s.ad_value(637), p.p335)), A::scale(s.ad_value(638), p.p336)), A::scale(s.ad_value(639), p.p337));

        s.store_add_ad(868, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p339), p.p338), (s.v[636] * p.p340)), A::scale(s.ad_value(637), p.p341)), A::scale(s.ad_value(638), p.p342)), A::scale(s.ad_value(639), p.p343));

        s.store_add_ad(869, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p345), p.p344), (s.v[636] * p.p346)), A::scale(s.ad_value(637), p.p347)), A::scale(s.ad_value(638), p.p348)), A::scale(s.ad_value(639), p.p349));

        s.store_add_ad(870, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p351), p.p350), (s.v[636] * p.p352)), A::scale(s.ad_value(637), p.p353)), A::scale(s.ad_value(638), p.p354)), A::scale(s.ad_value(639), p.p355));

        s.store_add_ad(654, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p404), p.p403), (s.v[636] * p.p405)), A::scale(s.ad_value(637), p.p406)), A::scale(s.ad_value(638), p.p407)), A::scale(s.ad_value(639), p.p408));

        s.store_add_ad(655, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p410), p.p409), (s.v[636] * p.p411)), A::scale(s.ad_value(637), p.p412)), A::scale(s.ad_value(638), p.p413)), A::scale(s.ad_value(639), p.p414));

        s.store_add_ad(656, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p416), p.p415), (s.v[636] * p.p417)), A::scale(s.ad_value(637), p.p418)), A::scale(s.ad_value(638), p.p419)), A::scale(s.ad_value(639), p.p420));

        s.store_add_ad(661, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p422), p.p421), (s.v[636] * p.p423)), A::scale(s.ad_value(637), p.p424)), A::scale(s.ad_value(638), p.p425)), A::scale(s.ad_value(639), p.p426));

        s.store_add_ad(679, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p456), p.p455), (s.v[636] * p.p457)), A::scale(s.ad_value(637), p.p458)), A::scale(s.ad_value(638), p.p459)), A::scale(s.ad_value(639), p.p460));

        s.store_add_ad(698, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p468), p.p467), (s.v[636] * p.p469)), A::scale(s.ad_value(637), p.p470)), A::scale(s.ad_value(638), p.p471)), A::scale(s.ad_value(639), p.p472));

        s.store_add_ad(702, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p507), p.p506), (s.v[636] * p.p508)), A::scale(s.ad_value(637), p.p509)), A::scale(s.ad_value(638), p.p510)), A::scale(s.ad_value(639), p.p511));

        s.store_add_ad(881, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p513), p.p512), (s.v[636] * p.p514)), A::scale(s.ad_value(637), p.p515)), A::scale(s.ad_value(638), p.p516)), A::scale(s.ad_value(639), p.p517));

        s.store_add_ad(694, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p480), p.p479), (s.v[636] * p.p481)), A::scale(s.ad_value(637), p.p482)), A::scale(s.ad_value(638), p.p483)), A::scale(s.ad_value(639), p.p484));

        s.store_add_ad(695, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p486), p.p485), (s.v[636] * p.p487)), A::scale(s.ad_value(637), p.p488)), A::scale(s.ad_value(638), p.p489)), A::scale(s.ad_value(639), p.p490));

        s.store_add_ad(696, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p519), p.p518), (s.v[636] * p.p520)), A::scale(s.ad_value(637), p.p521)), A::scale(s.ad_value(638), p.p522)), A::scale(s.ad_value(639), p.p523));

        s.store_add_ad(697, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p525), p.p524), (s.v[636] * p.p526)), A::scale(s.ad_value(637), p.p527)), A::scale(s.ad_value(638), p.p528)), A::scale(s.ad_value(639), p.p529));

        s.store_add_ad(657, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p493), p.p492), (s.v[636] * p.p494)), A::scale(s.ad_value(637), p.p495)), A::scale(s.ad_value(638), p.p496)), A::scale(s.ad_value(639), p.p497));

        s.store_add_ad(790, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p532), p.p531), (s.v[636] * p.p533)), A::scale(s.ad_value(637), p.p534)), A::scale(s.ad_value(638), p.p535)), A::scale(s.ad_value(639), p.p536));

        s.store_add_ad(700, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p544), p.p543), (s.v[636] * p.p545)), A::scale(s.ad_value(637), p.p546)), A::scale(s.ad_value(638), p.p547)), A::scale(s.ad_value(639), p.p548));

        s.store_add_ad(704, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p606), p.p605), (s.v[636] * p.p607)), A::scale(s.ad_value(637), p.p608)), A::scale(s.ad_value(638), p.p609)), A::scale(s.ad_value(639), p.p610));

        s.store_add_ad(707, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p624), p.p623), (s.v[636] * p.p625)), A::scale(s.ad_value(637), p.p626)), A::scale(s.ad_value(638), p.p627)), A::scale(s.ad_value(639), p.p628));

        s.store_add_ad(703, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p630), p.p629), (s.v[636] * p.p631)), A::scale(s.ad_value(637), p.p632)), A::scale(s.ad_value(638), p.p633)), A::scale(s.ad_value(639), p.p634));

        s.store_add_ad(807, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p642), p.p641), (s.v[636] * p.p643)), A::scale(s.ad_value(637), p.p644)), A::scale(s.ad_value(638), p.p645)), A::scale(s.ad_value(639), p.p646));

        s.store_add_ad(811, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p678), p.p677), (s.v[636] * p.p679)), A::scale(s.ad_value(637), p.p680)), A::scale(s.ad_value(638), p.p681)), A::scale(s.ad_value(639), p.p682));

        s.store_add_ad(812, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p690), p.p689), (s.v[636] * p.p691)), A::scale(s.ad_value(637), p.p692)), A::scale(s.ad_value(638), p.p693)), A::scale(s.ad_value(639), p.p694));

        s.store_add_ad(814, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p708), p.p707), (s.v[636] * p.p709)), A::scale(s.ad_value(637), p.p710)), A::scale(s.ad_value(638), p.p711)), A::scale(s.ad_value(639), p.p712));

        s.store_add_ad(325, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p714), p.p713), (s.v[636] * p.p715)), A::scale(s.ad_value(637), p.p716)), A::scale(s.ad_value(638), p.p717)), A::scale(s.ad_value(639), p.p718));

        s.store_add_ad(326, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p720), p.p719), (s.v[636] * p.p721)), A::scale(s.ad_value(637), p.p722)), A::scale(s.ad_value(638), p.p723)), A::scale(s.ad_value(639), p.p724));

        s.store_add_ad(328, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p726), p.p725), (s.v[636] * p.p727)), A::scale(s.ad_value(637), p.p728)), A::scale(s.ad_value(638), p.p729)), A::scale(s.ad_value(639), p.p730));

        s.store_add_ad(329, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p732), p.p731), (s.v[636] * p.p733)), A::scale(s.ad_value(637), p.p734)), A::scale(s.ad_value(638), p.p735)), A::scale(s.ad_value(639), p.p736));

        s.store_add_ad(792, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1027), p.p1025), (s.v[636] * p.p1028)), A::scale(s.ad_value(637), p.p1029)), A::scale(s.ad_value(638), p.p1030)), A::scale(s.ad_value(639), p.p1031));

        s.store_add_ad(793, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1039), p.p1038), (s.v[636] * p.p1040)), A::scale(s.ad_value(637), p.p1041)), A::scale(s.ad_value(638), p.p1042)), A::scale(s.ad_value(639), p.p1043));

        s.store_add_ad(794, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1045), p.p1044), (s.v[636] * p.p1046)), A::scale(s.ad_value(637), p.p1047)), A::scale(s.ad_value(638), p.p1048)), A::scale(s.ad_value(639), p.p1049));

        s.store_add_ad(798, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1051), p.p1050), (s.v[636] * p.p1052)), A::scale(s.ad_value(637), p.p1053)), A::scale(s.ad_value(638), p.p1054)), A::scale(s.ad_value(639), p.p1055));

        s.store_add_ad(800, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1057), p.p1056), (s.v[636] * p.p1058)), A::scale(s.ad_value(637), p.p1059)), A::scale(s.ad_value(638), p.p1060)), A::scale(s.ad_value(639), p.p1061));

        s.store_add_ad(799, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1063), p.p1062), (s.v[636] * p.p1064)), A::scale(s.ad_value(637), p.p1065)), A::scale(s.ad_value(638), p.p1066)), A::scale(s.ad_value(639), p.p1067));

        s.store_add_ad(801, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1069), p.p1068), (s.v[636] * p.p1070)), A::scale(s.ad_value(637), p.p1071)), A::scale(s.ad_value(638), p.p1072)), A::scale(s.ad_value(639), p.p1073));

        s.store_add_ad(709, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p926), p.p925), (s.v[636] * p.p927)), A::scale(s.ad_value(637), p.p928)), A::scale(s.ad_value(638), p.p929)), A::scale(s.ad_value(639), p.p930));

        s.store_add_ad(853, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p932), p.p931), (s.v[636] * p.p933)), A::scale(s.ad_value(637), p.p934)), A::scale(s.ad_value(638), p.p935)), A::scale(s.ad_value(639), p.p936));

        s.store_add_ad(852, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p938), p.p937), (s.v[636] * p.p939)), A::scale(s.ad_value(637), p.p940)), A::scale(s.ad_value(638), p.p941)), A::scale(s.ad_value(639), p.p942));

        s.store_add_ad(712, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p950), p.p949), (s.v[636] * p.p951)), A::scale(s.ad_value(637), p.p952)), A::scale(s.ad_value(638), p.p953)), A::scale(s.ad_value(639), p.p954));

        s.store_add_ad(711, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p944), p.p943), (s.v[636] * p.p945)), A::scale(s.ad_value(637), p.p946)), A::scale(s.ad_value(638), p.p947)), A::scale(s.ad_value(639), p.p948));

        s.store_add_ad(713, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p956), p.p955), (s.v[636] * p.p957)), A::scale(s.ad_value(637), p.p958)), A::scale(s.ad_value(638), p.p959)), A::scale(s.ad_value(639), p.p960));

        s.store_add_ad(714, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p986), p.p985), (s.v[636] * p.p987)), A::scale(s.ad_value(637), p.p988)), A::scale(s.ad_value(638), p.p989)), A::scale(s.ad_value(639), p.p990));

        s.store_add_ad(716, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p992), p.p991), (s.v[636] * p.p993)), A::scale(s.ad_value(637), p.p994)), A::scale(s.ad_value(638), p.p995)), A::scale(s.ad_value(639), p.p996));

        s.store_add_ad(719, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1010), p.p1009), (s.v[636] * p.p1011)), A::scale(s.ad_value(637), p.p1012)), A::scale(s.ad_value(638), p.p1013)), A::scale(s.ad_value(639), p.p1014));

        s.store_add_ad(720, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1016), p.p1015), (s.v[636] * p.p1017)), A::scale(s.ad_value(637), p.p1018)), A::scale(s.ad_value(638), p.p1019)), A::scale(s.ad_value(639), p.p1020));

        s.store_add_ad(721, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1120), p.p1119), (s.v[636] * p.p1121)), A::scale(s.ad_value(637), p.p1122)), A::scale(s.ad_value(638), p.p1123)), A::scale(s.ad_value(639), p.p1124));

        s.store_add_ad(722, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1126), p.p1125), (s.v[636] * p.p1127)), A::scale(s.ad_value(637), p.p1128)), A::scale(s.ad_value(638), p.p1129)), A::scale(s.ad_value(639), p.p1130));

        s.store_add_ad(723, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1132), p.p1131), (s.v[636] * p.p1133)), A::scale(s.ad_value(637), p.p1134)), A::scale(s.ad_value(638), p.p1135)), A::scale(s.ad_value(639), p.p1136));

        s.store_add_ad(724, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1138), p.p1137), (s.v[636] * p.p1139)), A::scale(s.ad_value(637), p.p1140)), A::scale(s.ad_value(638), p.p1141)), A::scale(s.ad_value(639), p.p1142));

        s.store_add_ad(725, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1144), p.p1143), (s.v[636] * p.p1145)), A::scale(s.ad_value(637), p.p1146)), A::scale(s.ad_value(638), p.p1147)), A::scale(s.ad_value(639), p.p1148));

        s.store_add_ad(726, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1150), p.p1149), (s.v[636] * p.p1151)), A::scale(s.ad_value(637), p.p1152)), A::scale(s.ad_value(638), p.p1153)), A::scale(s.ad_value(639), p.p1154));

        s.store_add_ad(727, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1156), p.p1155), (s.v[636] * p.p1157)), A::scale(s.ad_value(637), p.p1158)), A::scale(s.ad_value(638), p.p1159)), A::scale(s.ad_value(639), p.p1160));

        s.store_add_ad(728, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1162), p.p1161), (s.v[636] * p.p1163)), A::scale(s.ad_value(637), p.p1164)), A::scale(s.ad_value(638), p.p1165)), A::scale(s.ad_value(639), p.p1166));

        s.store_add_ad(729, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1168), p.p1167), (s.v[636] * p.p1169)), A::scale(s.ad_value(637), p.p1170)), A::scale(s.ad_value(638), p.p1171)), A::scale(s.ad_value(639), p.p1172));

        s.store_add_ad(730, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1174), p.p1173), (s.v[636] * p.p1175)), A::scale(s.ad_value(637), p.p1176)), A::scale(s.ad_value(638), p.p1177)), A::scale(s.ad_value(639), p.p1178));

        s.store_add_ad(731, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1180), p.p1179), (s.v[636] * p.p1181)), A::scale(s.ad_value(637), p.p1182)), A::scale(s.ad_value(638), p.p1183)), A::scale(s.ad_value(639), p.p1184));

        s.store_add_ad(732, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1186), p.p1185), (s.v[636] * p.p1187)), A::scale(s.ad_value(637), p.p1188)), A::scale(s.ad_value(638), p.p1189)), A::scale(s.ad_value(639), p.p1190));

        s.store_add_ad(733, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1192), p.p1191), (s.v[636] * p.p1193)), A::scale(s.ad_value(637), p.p1194)), A::scale(s.ad_value(638), p.p1195)), A::scale(s.ad_value(639), p.p1196));

        s.store_add_ad(734, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1198), p.p1197), (s.v[636] * p.p1199)), A::scale(s.ad_value(637), p.p1200)), A::scale(s.ad_value(638), p.p1201)), A::scale(s.ad_value(639), p.p1202));

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_add_ad(735, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1204), p.p1203), (s.v[636] * p.p1205)), A::scale(s.ad_value(637), p.p1206)), A::scale(s.ad_value(638), p.p1207)), A::scale(s.ad_value(639), p.p1208));

        s.store_add_ad(736, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1210), p.p1209), (s.v[636] * p.p1211)), A::scale(s.ad_value(637), p.p1212)), A::scale(s.ad_value(638), p.p1213)), A::scale(s.ad_value(639), p.p1214));

        s.store_add_ad(737, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1216), p.p1215), (s.v[636] * p.p1217)), A::scale(s.ad_value(637), p.p1218)), A::scale(s.ad_value(638), p.p1219)), A::scale(s.ad_value(639), p.p1220));

        s.store_add_ad(738, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1222), p.p1221), (s.v[636] * p.p1223)), A::scale(s.ad_value(637), p.p1224)), A::scale(s.ad_value(638), p.p1225)), A::scale(s.ad_value(639), p.p1226));

        s.store_add_ad(739, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1228), p.p1227), (s.v[636] * p.p1229)), A::scale(s.ad_value(637), p.p1230)), A::scale(s.ad_value(638), p.p1231)), A::scale(s.ad_value(639), p.p1232));

        s.store_add_ad(740, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1234), p.p1233), (s.v[636] * p.p1235)), A::scale(s.ad_value(637), p.p1236)), A::scale(s.ad_value(638), p.p1237)), A::scale(s.ad_value(639), p.p1238));

        s.store_add_ad(743, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1240), p.p1239), (s.v[636] * p.p1241)), A::scale(s.ad_value(637), p.p1242)), A::scale(s.ad_value(638), p.p1243)), A::scale(s.ad_value(639), p.p1244));

        s.store_add_ad(744, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1246), p.p1245), (s.v[636] * p.p1247)), A::scale(s.ad_value(637), p.p1248)), A::scale(s.ad_value(638), p.p1249)), A::scale(s.ad_value(639), p.p1250));

        s.store_add_ad(745, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1252), p.p1251), (s.v[636] * p.p1253)), A::scale(s.ad_value(637), p.p1254)), A::scale(s.ad_value(638), p.p1255)), A::scale(s.ad_value(639), p.p1256));

        s.store_add_ad(746, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1258), p.p1257), (s.v[636] * p.p1259)), A::scale(s.ad_value(637), p.p1260)), A::scale(s.ad_value(638), p.p1261)), A::scale(s.ad_value(639), p.p1262));

        s.store_add_ad(742, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1264), p.p1263), (s.v[636] * p.p1265)), A::scale(s.ad_value(637), p.p1266)), A::scale(s.ad_value(638), p.p1267)), A::scale(s.ad_value(639), p.p1268));

        s.store_add_ad(747, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1270), p.p1269), (s.v[636] * p.p1271)), A::scale(s.ad_value(637), p.p1272)), A::scale(s.ad_value(638), p.p1273)), A::scale(s.ad_value(639), p.p1274));

        s.store_add_ad(748, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1276), p.p1275), (s.v[636] * p.p1277)), A::scale(s.ad_value(637), p.p1278)), A::scale(s.ad_value(638), p.p1279)), A::scale(s.ad_value(639), p.p1280));

        s.store_add_ad(749, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1282), p.p1281), (s.v[636] * p.p1283)), A::scale(s.ad_value(637), p.p1284)), A::scale(s.ad_value(638), p.p1285)), A::scale(s.ad_value(639), p.p1286));

        s.store_add_ad(750, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1288), p.p1287), (s.v[636] * p.p1289)), A::scale(s.ad_value(637), p.p1290)), A::scale(s.ad_value(638), p.p1291)), A::scale(s.ad_value(639), p.p1292));

        s.store_add_ad(751, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1294), p.p1293), (s.v[636] * p.p1295)), A::scale(s.ad_value(637), p.p1296)), A::scale(s.ad_value(638), p.p1297)), A::scale(s.ad_value(639), p.p1298));

        s.store_add_ad(752, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1330), p.p1329), (s.v[636] * p.p1331)), A::scale(s.ad_value(637), p.p1332)), A::scale(s.ad_value(638), p.p1333)), A::scale(s.ad_value(639), p.p1334));

        s.store_add_ad(753, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1336), p.p1335), (s.v[636] * p.p1337)), A::scale(s.ad_value(637), p.p1338)), A::scale(s.ad_value(638), p.p1339)), A::scale(s.ad_value(639), p.p1340));

        s.store_add_ad(754, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1342), p.p1341), (s.v[636] * p.p1343)), A::scale(s.ad_value(637), p.p1344)), A::scale(s.ad_value(638), p.p1345)), A::scale(s.ad_value(639), p.p1346));

        s.store_add_ad(755, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1348), p.p1347), (s.v[636] * p.p1349)), A::scale(s.ad_value(637), p.p1350)), A::scale(s.ad_value(638), p.p1351)), A::scale(s.ad_value(639), p.p1352));

        s.store_add_ad(761, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1300), p.p1299), (s.v[636] * p.p1301)), A::scale(s.ad_value(637), p.p1302)), A::scale(s.ad_value(638), p.p1303)), A::scale(s.ad_value(639), p.p1304));

        s.store_add_ad(762, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1306), p.p1305), (s.v[636] * p.p1307)), A::scale(s.ad_value(637), p.p1308)), A::scale(s.ad_value(638), p.p1309)), A::scale(s.ad_value(639), p.p1310));

        s.store_add_ad(763, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1312), p.p1311), (s.v[636] * p.p1313)), A::scale(s.ad_value(637), p.p1314)), A::scale(s.ad_value(638), p.p1315)), A::scale(s.ad_value(639), p.p1316));

        s.store_add_ad(764, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1318), p.p1317), (s.v[636] * p.p1319)), A::scale(s.ad_value(637), p.p1320)), A::scale(s.ad_value(638), p.p1321)), A::scale(s.ad_value(639), p.p1322));

        s.store_add_ad(765, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1324), p.p1323), (s.v[636] * p.p1325)), A::scale(s.ad_value(637), p.p1326)), A::scale(s.ad_value(638), p.p1327)), A::scale(s.ad_value(639), p.p1328));

        s.store_add_ad(766, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1354), p.p1353), (s.v[636] * p.p1355)), A::scale(s.ad_value(637), p.p1356)), A::scale(s.ad_value(638), p.p1357)), A::scale(s.ad_value(639), p.p1358));

        s.store_add_ad(767, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1360), p.p1359), (s.v[636] * p.p1361)), A::scale(s.ad_value(637), p.p1362)), A::scale(s.ad_value(638), p.p1363)), A::scale(s.ad_value(639), p.p1364));

        s.store_add_ad(768, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1366), p.p1365), (s.v[636] * p.p1367)), A::scale(s.ad_value(637), p.p1368)), A::scale(s.ad_value(638), p.p1369)), A::scale(s.ad_value(639), p.p1370));

        s.store_add_ad(769, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1372), p.p1371), (s.v[636] * p.p1373)), A::scale(s.ad_value(637), p.p1374)), A::scale(s.ad_value(638), p.p1375)), A::scale(s.ad_value(639), p.p1376));

        s.store_add_ad(775, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1445), p.p1444), (s.v[636] * p.p1446)), A::scale(s.ad_value(637), p.p1447)), A::scale(s.ad_value(638), p.p1448)), A::scale(s.ad_value(639), p.p1449));

        s.store_add_ad(776, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1451), p.p1450), (s.v[636] * p.p1452)), A::scale(s.ad_value(637), p.p1453)), A::scale(s.ad_value(638), p.p1454)), A::scale(s.ad_value(639), p.p1455));

        s.store_add_ad(777, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1463), p.p1462), (s.v[636] * p.p1464)), A::scale(s.ad_value(637), p.p1465)), A::scale(s.ad_value(638), p.p1466)), A::scale(s.ad_value(639), p.p1467));

        s.store_add_ad(778, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1469), p.p1468), (s.v[636] * p.p1470)), A::scale(s.ad_value(637), p.p1471)), A::scale(s.ad_value(638), p.p1472)), A::scale(s.ad_value(639), p.p1473));

        s.store_add_ad(779, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1457), p.p1456), (s.v[636] * p.p1458)), A::scale(s.ad_value(637), p.p1459)), A::scale(s.ad_value(638), p.p1460)), A::scale(s.ad_value(639), p.p1461));

        s.store_add_ad(780, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1475), p.p1474), (s.v[636] * p.p1476)), A::scale(s.ad_value(637), p.p1477)), A::scale(s.ad_value(638), p.p1478)), A::scale(s.ad_value(639), p.p1479));

        s.store_add_ad(781, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1481), p.p1480), (s.v[636] * p.p1482)), A::scale(s.ad_value(637), p.p1483)), A::scale(s.ad_value(638), p.p1484)), A::scale(s.ad_value(639), p.p1485));

        s.store_add_ad(782, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1487), p.p1486), (s.v[636] * p.p1488)), A::scale(s.ad_value(637), p.p1489)), A::scale(s.ad_value(638), p.p1490)), A::scale(s.ad_value(639), p.p1491));

        s.store_add_ad(783, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1493), p.p1492), (s.v[636] * p.p1494)), A::scale(s.ad_value(637), p.p1495)), A::scale(s.ad_value(638), p.p1496)), A::scale(s.ad_value(639), p.p1497));

        s.store_add_ad(784, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1499), p.p1498), (s.v[636] * p.p1500)), A::scale(s.ad_value(637), p.p1501)), A::scale(s.ad_value(638), p.p1502)), A::scale(s.ad_value(639), p.p1503));

        s.store_add_ad(785, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1505), p.p1504), (s.v[636] * p.p1506)), A::scale(s.ad_value(637), p.p1507)), A::scale(s.ad_value(638), p.p1508)), A::scale(s.ad_value(639), p.p1509));

        s.store_add_ad(786, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1511), p.p1510), (s.v[636] * p.p1512)), A::scale(s.ad_value(637), p.p1513)), A::scale(s.ad_value(638), p.p1514)), A::scale(s.ad_value(639), p.p1515));

        s.store_add_ad(787, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1517), p.p1516), (s.v[636] * p.p1518)), A::scale(s.ad_value(637), p.p1519)), A::scale(s.ad_value(638), p.p1520)), A::scale(s.ad_value(639), p.p1521));

        s.store_add_ad(788, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1523), p.p1522), (s.v[636] * p.p1524)), A::scale(s.ad_value(637), p.p1525)), A::scale(s.ad_value(638), p.p1526)), A::scale(s.ad_value(639), p.p1527));

        s.store_add_ad(789, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1763), p.p1762), (s.v[636] * p.p1764)), A::scale(s.ad_value(637), p.p1765)), A::scale(s.ad_value(638), p.p1766)), A::scale(s.ad_value(639), p.p1767));

        s.store_add_ad(643, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1531), p.p1530), (s.v[636] * p.p1532)), A::scale(s.ad_value(637), p.p1533)), A::scale(s.ad_value(638), p.p1534)), A::scale(s.ad_value(639), p.p1535));

        s.store_add_ad(642, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1537), p.p1536), (s.v[636] * p.p1538)), A::scale(s.ad_value(637), p.p1539)), A::scale(s.ad_value(638), p.p1540)), A::scale(s.ad_value(639), p.p1541));

        s.store_add_ad(644, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p29), p.p28), (s.v[636] * p.p30)), A::scale(s.ad_value(637), p.p31)), A::scale(s.ad_value(638), p.p32)), A::scale(s.ad_value(639), p.p33));

        s.store_add_ad(645, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p35), p.p34), (s.v[636] * p.p36)), A::scale(s.ad_value(637), p.p37)), A::scale(s.ad_value(638), p.p38)), A::scale(s.ad_value(639), p.p39));

        s.store_add_ad(648, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1548), p.p1547), (s.v[636] * p.p1549)), A::scale(s.ad_value(637), p.p1550)), A::scale(s.ad_value(638), p.p1551)), A::scale(s.ad_value(639), p.p1552));

        s.store_add_ad(649, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1554), p.p1553), (s.v[636] * p.p1555)), A::scale(s.ad_value(637), p.p1556)), A::scale(s.ad_value(638), p.p1557)), A::scale(s.ad_value(639), p.p1558));

        s.store_add_ad(650, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1560), p.p1559), (s.v[636] * p.p1561)), A::scale(s.ad_value(637), p.p1562)), A::scale(s.ad_value(638), p.p1563)), A::scale(s.ad_value(639), p.p1564));

        s.store_add_ad(651, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1566), p.p1565), (s.v[636] * p.p1567)), A::scale(s.ad_value(637), p.p1568)), A::scale(s.ad_value(638), p.p1569)), A::scale(s.ad_value(639), p.p1570));

        s.store_add_ad(652, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1572), p.p1571), (s.v[636] * p.p1573)), A::scale(s.ad_value(637), p.p1574)), A::scale(s.ad_value(638), p.p1575)), A::scale(s.ad_value(639), p.p1576));

        s.store_add_ad(653, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1578), p.p1577), (s.v[636] * p.p1579)), A::scale(s.ad_value(637), p.p1580)), A::scale(s.ad_value(638), p.p1581)), A::scale(s.ad_value(639), p.p1582));

        s.store_add_ad(865, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1657), p.p1656), (s.v[636] * p.p1658)), A::scale(s.ad_value(637), p.p1659)), A::scale(s.ad_value(638), p.p1660)), A::scale(s.ad_value(639), p.p1661));

        s.store_add_ad(866, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1663), p.p1662), (s.v[636] * p.p1664)), A::scale(s.ad_value(637), p.p1665)), A::scale(s.ad_value(638), p.p1666)), A::scale(s.ad_value(639), p.p1667));

        s.store_add_ad(836, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p738), p.p737), (s.v[636] * p.p739)), A::scale(s.ad_value(637), p.p740)), A::scale(s.ad_value(638), p.p741)), A::scale(s.ad_value(639), p.p742));

        s.store_add_ad(837, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p756), p.p755), (s.v[636] * p.p757)), A::scale(s.ad_value(637), p.p758)), A::scale(s.ad_value(638), p.p759)), A::scale(s.ad_value(639), p.p760));

        s.store_add_ad(838, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p768), p.p767), (s.v[636] * p.p769)), A::scale(s.ad_value(637), p.p770)), A::scale(s.ad_value(638), p.p771)), A::scale(s.ad_value(639), p.p772));

        s.store_add_ad(842, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p786), p.p785), (s.v[636] * p.p787)), A::scale(s.ad_value(637), p.p788)), A::scale(s.ad_value(638), p.p789)), A::scale(s.ad_value(639), p.p790));

        s.store_add_ad(823, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p792), p.p791), (s.v[636] * p.p793)), A::scale(s.ad_value(637), p.p794)), A::scale(s.ad_value(638), p.p795)), A::scale(s.ad_value(639), p.p796));

        s.store_add_ad(824, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p810), p.p809), (s.v[636] * p.p811)), A::scale(s.ad_value(637), p.p812)), A::scale(s.ad_value(638), p.p813)), A::scale(s.ad_value(639), p.p814));

        s.store_add_ad(847, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p822), p.p821), (s.v[636] * p.p823)), A::scale(s.ad_value(637), p.p824)), A::scale(s.ad_value(638), p.p825)), A::scale(s.ad_value(639), p.p826));

        s.store_add_ad(830, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p846), p.p845), (s.v[636] * p.p847)), A::scale(s.ad_value(637), p.p848)), A::scale(s.ad_value(638), p.p849)), A::scale(s.ad_value(639), p.p850));

        s.store_add_ad(831, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p864), p.p863), (s.v[636] * p.p865)), A::scale(s.ad_value(637), p.p866)), A::scale(s.ad_value(638), p.p867)), A::scale(s.ad_value(639), p.p868));

        s.store_add_ad(834, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p876), p.p875), (s.v[636] * p.p877)), A::scale(s.ad_value(637), p.p878)), A::scale(s.ad_value(638), p.p879)), A::scale(s.ad_value(639), p.p880));

        s.store_add_ad(835, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p882), p.p881), (s.v[636] * p.p883)), A::scale(s.ad_value(637), p.p884)), A::scale(s.ad_value(638), p.p885)), A::scale(s.ad_value(639), p.p886));

        s.store_add_ad(848, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p576), p.p575), (s.v[636] * p.p577)), A::scale(s.ad_value(637), p.p578)), A::scale(s.ad_value(638), p.p579)), A::scale(s.ad_value(639), p.p580));

        s.store_add_ad(849, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p556), p.p555), (s.v[636] * p.p557)), A::scale(s.ad_value(637), p.p558)), A::scale(s.ad_value(638), p.p559)), A::scale(s.ad_value(639), p.p560));

        s.store_add_ad(850, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p569), p.p568), (s.v[636] * p.p570)), A::scale(s.ad_value(637), p.p571)), A::scale(s.ad_value(638), p.p572)), A::scale(s.ad_value(639), p.p573));

        s.store_add_ad(854, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p962), p.p961), (s.v[636] * p.p963)), A::scale(s.ad_value(637), p.p964)), A::scale(s.ad_value(638), p.p965)), A::scale(s.ad_value(639), p.p966));

        s.store_add_ad(855, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p968), p.p967), (s.v[636] * p.p969)), A::scale(s.ad_value(637), p.p970)), A::scale(s.ad_value(638), p.p971)), A::scale(s.ad_value(639), p.p972));

        s.store_add_ad(856, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p974), p.p973), (s.v[636] * p.p975)), A::scale(s.ad_value(637), p.p976)), A::scale(s.ad_value(638), p.p977)), A::scale(s.ad_value(639), p.p978));

        s.store_add_ad(857, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p980), p.p979), (s.v[636] * p.p981)), A::scale(s.ad_value(637), p.p982)), A::scale(s.ad_value(638), p.p983)), A::scale(s.ad_value(639), p.p984));

        s.store_add_ad(858, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1742), p.p1741), (s.v[636] * p.p1743)), A::scale(s.ad_value(637), p.p1744)), A::scale(s.ad_value(638), p.p1745)), A::scale(s.ad_value(639), p.p1746));

        s.store_add_ad(859, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1751), p.p1750), (s.v[636] * p.p1752)), A::scale(s.ad_value(637), p.p1753)), A::scale(s.ad_value(638), p.p1754)), A::scale(s.ad_value(639), p.p1755));

        s.store_add_ad(860, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1757), p.p1756), (s.v[636] * p.p1758)), A::scale(s.ad_value(637), p.p1759)), A::scale(s.ad_value(638), p.p1760)), A::scale(s.ad_value(639), p.p1761));

        s.store_add_ad(862, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1769), p.p1768), (s.v[636] * p.p1770)), A::scale(s.ad_value(637), p.p1771)), A::scale(s.ad_value(638), p.p1772)), A::scale(s.ad_value(639), p.p1773));

        s.store_add_ad(863, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1775), p.p1774), (s.v[636] * p.p1776)), A::scale(s.ad_value(637), p.p1777)), A::scale(s.ad_value(638), p.p1778)), A::scale(s.ad_value(639), p.p1779));

        s.store_add_ad(681, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p177), p.p176), (s.v[636] * p.p178)), A::scale(s.ad_value(637), p.p179)), A::scale(s.ad_value(638), p.p180)), A::scale(s.ad_value(639), p.p181));

        s.store_add_ad(682, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p183), p.p182), (s.v[636] * p.p184)), A::scale(s.ad_value(637), p.p185)), A::scale(s.ad_value(638), p.p186)), A::scale(s.ad_value(639), p.p187));

        s.store_add_ad(574, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1690), p.p1689), (s.v[636] * p.p1691)), A::scale(s.ad_value(637), p.p1692)), A::scale(s.ad_value(638), p.p1693)), A::scale(s.ad_value(639), p.p1694));

        s.store_add_ad(576, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1702), p.p1701), (s.v[636] * p.p1703)), A::scale(s.ad_value(637), p.p1704)), A::scale(s.ad_value(638), p.p1705)), A::scale(s.ad_value(639), p.p1706));

        s.store_add_ad(575, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1696), p.p1695), (s.v[636] * p.p1697)), A::scale(s.ad_value(637), p.p1698)), A::scale(s.ad_value(638), p.p1699)), A::scale(s.ad_value(639), p.p1700));

        s.v[1096] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1096] != 0.0) {
            s.store_add_ad(689, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p357), p.p356), (s.v[636] * p.p358)), A::scale(s.ad_value(637), p.p359)), A::scale(s.ad_value(638), p.p360)), A::scale(s.ad_value(639), p.p361));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(690, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p363), p.p362), (s.v[636] * p.p364)), A::scale(s.ad_value(637), p.p365)), A::scale(s.ad_value(638), p.p366)), A::scale(s.ad_value(639), p.p367));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(691, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p369), p.p368), (s.v[636] * p.p370)), A::scale(s.ad_value(637), p.p371)), A::scale(s.ad_value(638), p.p372)), A::scale(s.ad_value(639), p.p373));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(809, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p660), p.p659), (s.v[636] * p.p661)), A::scale(s.ad_value(637), p.p662)), A::scale(s.ad_value(638), p.p663)), A::scale(s.ad_value(639), p.p664));
        }

        if (s.v[1096] != 0.0) {
            s.store_add_ad(828, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p828), p.p827), (s.v[636] * p.p829)), A::scale(s.ad_value(637), p.p830)), A::scale(s.ad_value(638), p.p831)), A::scale(s.ad_value(639), p.p832));
        }

        s.v[1097] = if (p.p61 == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(871, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p387), p.p386), (s.v[636] * p.p388)), A::scale(s.ad_value(637), p.p389)), A::scale(s.ad_value(638), p.p390)), A::scale(s.ad_value(639), p.p391));
        }

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(872, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p393), p.p392), (s.v[636] * p.p394)), A::scale(s.ad_value(637), p.p395)), A::scale(s.ad_value(638), p.p396)), A::scale(s.ad_value(639), p.p397));
        }

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(692, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p375), p.p374), (s.v[636] * p.p376)), A::scale(s.ad_value(637), p.p377)), A::scale(s.ad_value(638), p.p378)), A::scale(s.ad_value(639), p.p379));
        }

        if ((s.v[1096] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(693, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p381), p.p380), (s.v[636] * p.p382)), A::scale(s.ad_value(637), p.p383)), A::scale(s.ad_value(638), p.p384)), A::scale(s.ad_value(639), p.p385));
        }

        s.v[1098] = if (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(756, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1378), p.p1377), (s.v[636] * p.p1379)), A::scale(s.ad_value(637), p.p1380)), A::scale(s.ad_value(638), p.p1381)), A::scale(s.ad_value(639), p.p1382));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(757, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1384), p.p1383), (s.v[636] * p.p1385)), A::scale(s.ad_value(637), p.p1386)), A::scale(s.ad_value(638), p.p1387)), A::scale(s.ad_value(639), p.p1388));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(758, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1390), p.p1389), (s.v[636] * p.p1391)), A::scale(s.ad_value(637), p.p1392)), A::scale(s.ad_value(638), p.p1393)), A::scale(s.ad_value(639), p.p1394));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(759, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1396), p.p1395), (s.v[636] * p.p1397)), A::scale(s.ad_value(637), p.p1398)), A::scale(s.ad_value(638), p.p1399)), A::scale(s.ad_value(639), p.p1400));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(760, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1402), p.p1401), (s.v[636] * p.p1403)), A::scale(s.ad_value(637), p.p1404)), A::scale(s.ad_value(638), p.p1405)), A::scale(s.ad_value(639), p.p1406));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(770, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1408), p.p1407), (s.v[636] * p.p1409)), A::scale(s.ad_value(637), p.p1410)), A::scale(s.ad_value(638), p.p1411)), A::scale(s.ad_value(639), p.p1412));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(771, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1414), p.p1413), (s.v[636] * p.p1415)), A::scale(s.ad_value(637), p.p1416)), A::scale(s.ad_value(638), p.p1417)), A::scale(s.ad_value(639), p.p1418));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(772, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1420), p.p1419), (s.v[636] * p.p1421)), A::scale(s.ad_value(637), p.p1422)), A::scale(s.ad_value(638), p.p1423)), A::scale(s.ad_value(639), p.p1424));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(773, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1426), p.p1425), (s.v[636] * p.p1427)), A::scale(s.ad_value(637), p.p1428)), A::scale(s.ad_value(638), p.p1429)), A::scale(s.ad_value(639), p.p1430));
        }

        if ((s.v[1096] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_add_ad(774, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1432), p.p1431), (s.v[636] * p.p1433)), A::scale(s.ad_value(637), p.p1434)), A::scale(s.ad_value(638), p.p1435)), A::scale(s.ad_value(639), p.p1436));
        }

        s.v[1099] = if (p.p66 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1099] != 0.0) {
            s.store_add_ad(665, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p213), p.p212), (s.v[636] * p.p214)), A::scale(s.ad_value(637), p.p215)), A::scale(s.ad_value(638), p.p216)), A::scale(s.ad_value(639), p.p217));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(668, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p195), p.p194), (s.v[636] * p.p196)), A::scale(s.ad_value(637), p.p197)), A::scale(s.ad_value(638), p.p198)), A::scale(s.ad_value(639), p.p199));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(677, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p255), p.p254), (s.v[636] * p.p256)), A::scale(s.ad_value(637), p.p257)), A::scale(s.ad_value(638), p.p258)), A::scale(s.ad_value(639), p.p259));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(699, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p474), p.p473), (s.v[636] * p.p475)), A::scale(s.ad_value(637), p.p476)), A::scale(s.ad_value(638), p.p477)), A::scale(s.ad_value(639), p.p478));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(791, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p538), p.p537), (s.v[636] * p.p539)), A::scale(s.ad_value(637), p.p540)), A::scale(s.ad_value(638), p.p541)), A::scale(s.ad_value(639), p.p542));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(701, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p550), p.p549), (s.v[636] * p.p551)), A::scale(s.ad_value(637), p.p552)), A::scale(s.ad_value(638), p.p553)), A::scale(s.ad_value(639), p.p554));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(715, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p998), p.p997), (s.v[636] * p.p999)), A::scale(s.ad_value(637), p.p1000)), A::scale(s.ad_value(638), p.p1001)), A::scale(s.ad_value(639), p.p1002));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(717, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1004), p.p1003), (s.v[636] * p.p1005)), A::scale(s.ad_value(637), p.p1006)), A::scale(s.ad_value(638), p.p1007)), A::scale(s.ad_value(639), p.p1008));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(796, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1033), p.p1032), (s.v[636] * p.p1034)), A::scale(s.ad_value(637), p.p1035)), A::scale(s.ad_value(638), p.p1036)), A::scale(s.ad_value(639), p.p1037));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(806, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p291), p.p290), (s.v[636] * p.p292)), A::scale(s.ad_value(637), p.p293)), A::scale(s.ad_value(638), p.p294)), A::scale(s.ad_value(639), p.p295));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(680, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p462), p.p461), (s.v[636] * p.p463)), A::scale(s.ad_value(637), p.p464)), A::scale(s.ad_value(638), p.p465)), A::scale(s.ad_value(639), p.p466));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(658, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p501), p.p500), (s.v[636] * p.p502)), A::scale(s.ad_value(637), p.p503)), A::scale(s.ad_value(638), p.p504)), A::scale(s.ad_value(639), p.p505));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(706, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p612), p.p611), (s.v[636] * p.p613)), A::scale(s.ad_value(637), p.p614)), A::scale(s.ad_value(638), p.p615)), A::scale(s.ad_value(639), p.p616));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(815, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p648), p.p647), (s.v[636] * p.p649)), A::scale(s.ad_value(637), p.p650)), A::scale(s.ad_value(638), p.p651)), A::scale(s.ad_value(639), p.p652));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(710, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p636), p.p635), (s.v[636] * p.p637)), A::scale(s.ad_value(637), p.p638)), A::scale(s.ad_value(638), p.p639)), A::scale(s.ad_value(639), p.p640));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(816, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p684), p.p683), (s.v[636] * p.p685)), A::scale(s.ad_value(637), p.p686)), A::scale(s.ad_value(638), p.p687)), A::scale(s.ad_value(639), p.p688));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(818, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p696), p.p695), (s.v[636] * p.p697)), A::scale(s.ad_value(637), p.p698)), A::scale(s.ad_value(638), p.p699)), A::scale(s.ad_value(639), p.p700));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(845, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p744), p.p743), (s.v[636] * p.p745)), A::scale(s.ad_value(637), p.p746)), A::scale(s.ad_value(638), p.p747)), A::scale(s.ad_value(639), p.p748));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(846, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p774), p.p773), (s.v[636] * p.p775)), A::scale(s.ad_value(637), p.p776)), A::scale(s.ad_value(638), p.p777)), A::scale(s.ad_value(639), p.p778));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(825, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p798), p.p797), (s.v[636] * p.p799)), A::scale(s.ad_value(637), p.p800)), A::scale(s.ad_value(638), p.p801)), A::scale(s.ad_value(639), p.p802));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(844, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p852), p.p851), (s.v[636] * p.p853)), A::scale(s.ad_value(637), p.p854)), A::scale(s.ad_value(638), p.p855)), A::scale(s.ad_value(639), p.p856));
        }

        if (s.v[1099] != 0.0) {
            s.store_add_ad(851, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p563), p.p562), (s.v[636] * p.p564)), A::scale(s.ad_value(637), p.p565)), A::scale(s.ad_value(638), p.p566)), A::scale(s.ad_value(639), p.p567));
        }

        s.v[1100] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1099] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_add_ad(817, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p666), p.p665), (s.v[636] * p.p667)), A::scale(s.ad_value(637), p.p668)), A::scale(s.ad_value(638), p.p669)), A::scale(s.ad_value(639), p.p670));
        }

        if ((s.v[1099] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_add_ad(843, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p834), p.p833), (s.v[636] * p.p835)), A::scale(s.ad_value(637), p.p836)), A::scale(s.ad_value(638), p.p837)), A::scale(s.ad_value(639), p.p838));
        }

        s.v[1101] = if (p.p67 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1101] != 0.0) {
            s.store_add_ad(705, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p618), p.p617), (s.v[636] * p.p619)), A::scale(s.ad_value(637), p.p620)), A::scale(s.ad_value(638), p.p621)), A::scale(s.ad_value(639), p.p622));
        }

        s.v[1102] = if (p.p582 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1102] != 0.0)) {
            s.store_scale(705, 705, (1.0 + ((p.p582 / p.p5) * (if (!((1.0 + (p.p5 / p.p585)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p585)) > 1e-38) { (((1.0 + (p.p5 / p.p585))) as f64).ln() } else { 0.0 }) }))));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(808, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p654), p.p653), (s.v[636] * p.p655)), A::scale(s.ad_value(637), p.p656)), A::scale(s.ad_value(638), p.p657)), A::scale(s.ad_value(639), p.p658));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(813, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p702), p.p701), (s.v[636] * p.p703)), A::scale(s.ad_value(637), p.p704)), A::scale(s.ad_value(638), p.p705)), A::scale(s.ad_value(639), p.p706));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(839, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p750), p.p749), (s.v[636] * p.p751)), A::scale(s.ad_value(637), p.p752)), A::scale(s.ad_value(638), p.p753)), A::scale(s.ad_value(639), p.p754));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(840, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p762), p.p761), (s.v[636] * p.p763)), A::scale(s.ad_value(637), p.p764)), A::scale(s.ad_value(638), p.p765)), A::scale(s.ad_value(639), p.p766));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(841, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p780), p.p779), (s.v[636] * p.p781)), A::scale(s.ad_value(637), p.p782)), A::scale(s.ad_value(638), p.p783)), A::scale(s.ad_value(639), p.p784));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(826, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p804), p.p803), (s.v[636] * p.p805)), A::scale(s.ad_value(637), p.p806)), A::scale(s.ad_value(638), p.p807)), A::scale(s.ad_value(639), p.p808));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(827, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p816), p.p815), (s.v[636] * p.p817)), A::scale(s.ad_value(637), p.p818)), A::scale(s.ad_value(638), p.p819)), A::scale(s.ad_value(639), p.p820));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(832, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p858), p.p857), (s.v[636] * p.p859)), A::scale(s.ad_value(637), p.p860)), A::scale(s.ad_value(638), p.p861)), A::scale(s.ad_value(639), p.p862));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(833, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p870), p.p869), (s.v[636] * p.p871)), A::scale(s.ad_value(637), p.p872)), A::scale(s.ad_value(638), p.p873)), A::scale(s.ad_value(639), p.p874));
        }

        s.v[1103] = if (p.p61 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_add_ad(810, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p672), p.p671), (s.v[636] * p.p673)), A::scale(s.ad_value(637), p.p674)), A::scale(s.ad_value(638), p.p675)), A::scale(s.ad_value(639), p.p676));
        }

        if ((s.v[1101] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_add_ad(829, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p840), p.p839), (s.v[636] * p.p841)), A::scale(s.ad_value(637), p.p842)), A::scale(s.ad_value(638), p.p843)), A::scale(s.ad_value(639), p.p844));
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(675, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p261), p.p260), (s.v[636] * p.p262)), A::scale(s.ad_value(637), p.p263)), A::scale(s.ad_value(638), p.p264)), A::scale(s.ad_value(639), p.p265));
        }

        s.v[1104] = if (p.p161 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1104] != 0.0)) {
            s.store_scale(675, 675, (1.0 + ((p.p161 / p.p5) * (if (!((1.0 + (p.p5 / p.p162)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p162)) > 1e-38) { (((1.0 + (p.p5 / p.p162))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1105] = if (p.p21 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1101] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(705, 705, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p588)), 1.0));
        }

        if ((s.v[1101] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(675, 675, A::offset(A::scale(s.ad_value(153), ((p.p5 - p.p21) * p.p163)), 1.0));
        }

        s.v[1107] = if (p.p57 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1107] != 0.0) {
            s.store_add_ad(882, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1808), p.p1807), (s.v[636] * p.p1809)), A::scale(s.ad_value(637), p.p1810)), A::scale(s.ad_value(638), p.p1811)), A::scale(s.ad_value(639), p.p1812));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(883, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1815), p.p1814), (s.v[636] * p.p1816)), A::scale(s.ad_value(637), p.p1817)), A::scale(s.ad_value(638), p.p1818)), A::scale(s.ad_value(639), p.p1819));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(884, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1822), p.p1821), (s.v[636] * p.p1823)), A::scale(s.ad_value(637), p.p1824)), A::scale(s.ad_value(638), p.p1825)), A::scale(s.ad_value(639), p.p1826));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(885, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1830), p.p1829), (s.v[636] * p.p1831)), A::scale(s.ad_value(637), p.p1832)), A::scale(s.ad_value(638), p.p1833)), A::scale(s.ad_value(639), p.p1834));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(886, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1836), p.p1835), (s.v[636] * p.p1837)), A::scale(s.ad_value(637), p.p1838)), A::scale(s.ad_value(638), p.p1839)), A::scale(s.ad_value(639), p.p1840));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(887, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1842), p.p1841), (s.v[636] * p.p1843)), A::scale(s.ad_value(637), p.p1844)), A::scale(s.ad_value(638), p.p1845)), A::scale(s.ad_value(639), p.p1846));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(888, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1854), p.p1853), (s.v[636] * p.p1855)), A::scale(s.ad_value(637), p.p1856)), A::scale(s.ad_value(638), p.p1857)), A::scale(s.ad_value(639), p.p1858));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(889, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1860), p.p1859), (s.v[636] * p.p1861)), A::scale(s.ad_value(637), p.p1862)), A::scale(s.ad_value(638), p.p1863)), A::scale(s.ad_value(639), p.p1864));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(890, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1870), p.p1869), (s.v[636] * p.p1871)), A::scale(s.ad_value(637), p.p1872)), A::scale(s.ad_value(638), p.p1873)), A::scale(s.ad_value(639), p.p1874));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(891, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1876), p.p1875), (s.v[636] * p.p1877)), A::scale(s.ad_value(637), p.p1878)), A::scale(s.ad_value(638), p.p1879)), A::scale(s.ad_value(639), p.p1880));
        }

        if (s.v[1107] != 0.0) {
            s.store_add_ad(892, A::add(A::add(A::offset(A::offset(A::scale(s.ad_value(635), p.p1882), p.p1881), (s.v[636] * p.p1883)), A::scale(s.ad_value(637), p.p1884)), A::scale(s.ad_value(638), p.p1885)), A::scale(s.ad_value(639), p.p1886));
        }

        s.v[1108] = if (p.p100 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1108] != 0.0) {
            s.store_scale(641, 641, (1.0 + ((p.p100 / p.p5) * (if (!((1.0 + (p.p5 / p.p101)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p101)) > 1e-38) { (((1.0 + (p.p5 / p.p101))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1109] = if (p.p158 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1109] != 0.0) {
            s.store_scale(673, 673, (1.0 + ((p.p158 / p.p5) * (if (!((1.0 + (p.p5 / p.p159)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p159)) > 1e-38) { (((1.0 + (p.p5 / p.p159))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1110] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1110] != 0.0) {
            s.store_scale(662, 662, (1.0 + ((p.p152 / p.p5) * (if (!((1.0 + (p.p5 / p.p153)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p153)) > 1e-38) { (((1.0 + (p.p5 / p.p153))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1111] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1111] != 0.0) {
            s.store_scale(663, 663, (1.0 + ((p.p154 / p.p5) * (if (!((1.0 + (p.p5 / p.p155)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p155)) > 1e-38) { (((1.0 + (p.p5 / p.p155))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1112] = if (p.p156 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1112] != 0.0) {
            s.store_scale(665, 665, (1.0 + ((p.p156 / p.p5) * (if (!((1.0 + (p.p5 / p.p157)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p157)) > 1e-38) { (((1.0 + (p.p5 / p.p157))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1113] = if (p.p428 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1113] != 0.0) {
            s.store_scale(679, 679, (1.0 + ((p.p428 / p.p5) * (if (!((1.0 + (p.p5 / p.p429)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p429)) > 1e-38) { (((1.0 + (p.p5 / p.p429))) as f64).ln() } else { 0.0 }) }))));
        }

        s.v[1114] = if (p.p432 != 0.0) { 1.0 } else { 0.0 };

    }
}
