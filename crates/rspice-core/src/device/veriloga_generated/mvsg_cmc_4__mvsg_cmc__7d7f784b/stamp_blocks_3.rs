#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_scalar(786, p.p168);
            s.store_scalar(787, p.p182);
            s.store_scalar(788, p.p181);
            s.store_scalar(789, 0.0);
            s.store_scalar(790, p.p183);
            s.store_scalar(791, p.p187);
            s.store_scalar(792, p.p178);
            s.store_scalar(793, p.p179);
            s.store_scalar(794, p.p180);
            s.store_scalar(795, p.p186);
            s.store_scalar(796, p.p185);
            s.store_scalar(797, p.p184);
            s.store_scalar(798, p.p39);
            s.store_scalar(799, p.p47);
            s.store_scalar(800, p.p45);
            s.store_scalar(801, p.p42);
            s.store_scalar(802, p.p2);
            s.store_scalar(803, p.p6);
            s.store_scalar(804, 1.0);
            s.store_scalar(805, 0.0);
            s.store_scalar(806, 0.0);
            s.store_scalar(807, 0.0);
            s.store_scalar(808, 0.0);
            s.store_scalar(809, 0.0);
            s.store_scalar(810, 0.0);
            s.store_scalar(811, 0.0);
            s.store_scalar(812, 0.0);
            s.store_scalar(813, 0.0);
            s.store_scalar(814, 0.0);
            s.store_scalar(815, 0.0);
            s.store_scalar(816, 0.0);
            s.store_scalar(818, 0.0);
            s.store_scalar(819, 0.0);
            s.store_scalar(820, 0.0);
            s.store_scalar(821, 0.0);
            s.store_scalar(822, 0.0);
            s.store_scalar(823, 0.0);
            s.store_scalar(824, 0.0);
            s.store_scalar(825, 0.0);
            s.store_scalar(826, 0.0);
            s.store_scalar(827, 0.0);
            s.store_scalar(828, 0.0);
            s.store_scalar(829, 0.0);
            s.store_scalar(830, 0.0);
            s.store_scalar(831, 0.0);
            s.store_scalar(832, 0.0);
            s.store_scalar(833, 0.0);
            s.store_scalar(834, 0.0);
            s.store_scalar(835, 0.0);
            s.store_scalar(836, 0.0);
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
            s.store_scalar(839, 0.0);
            s.store_scalar(840, 0.0);
            s.store_scalar(841, 0.0);
            s.store_scalar(842, 0.0);
            s.store_scalar(843, 0.0);
            s.store_scalar(846, 0.0);
            s.store_scalar(847, 0.0);
            s.store_scalar(848, 0.0);
            s.store_scalar(849, 0.0);
            s.store_scalar(850, 0.0);
            s.store_scalar(851, 0.0);
            s.store_scalar(852, 0.0);
            s.store_scalar(853, 0.0);
            s.store_scalar(854, 0.0);
            s.store_scalar(855, 0.0);
            s.store_scalar(856, 0.0);
            s.store_scalar(857, 0.0);
            s.store_scalar(858, 0.0);
            s.store_scalar(859, 0.0);
            s.store_scalar(860, 0.0);
            s.store_scalar(861, 0.0);
            s.store_scalar(862, 0.0);
            s.store_scalar(863, 0.0);
            s.store_scalar(864, 0.0);
            s.store_scalar(865, 0.0);
            s.store_scalar(866, 0.0);
            s.store_scalar(867, 0.0);
            s.store_scalar(868, 0.0);
            s.store_scalar(869, 0.0);
            s.store_scalar(870, 0.0);
            s.store_scalar(871, 0.0);
            s.store_scalar(872, 0.0);
            s.store_scalar(873, 0.0);
        }

        if s.b[761] {
            s.store_ad_value(870, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(772), A::tanh_scaled_input(s.ad_value(772), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(772)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[761] {
            s.store_sub(871, 771, 772);
            s.store_mul(805, 791, 779);
            s.store_add_scaled_product(807, A::div_scaled_inputs(s.ad_value(787), 1.0, s.ad_value(779), 2.302585092994046), 1.0, s.ad_value(790), s.ad_value(870), 1.0);
            s.store_add_scaled_product(808, s.ad_value(786), 1.0, s.ad_value(797), A::sub(s.ad_value(777), s.ad_value(778)), 1.0);
            s.store_pow_ad(826, A::div(s.ad_value(777), s.ad_value(778)), s.ad_value(799));
        }

        s.b[874] = (s.v[798] != 0.0);
        s.v[874] = if s.b[874] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[874]) {
            s.store_div_ad_rhs(809, 870, A::pow(A::offset(A::pow(A::div(s.ad_value(870), s.ad_value(798)), s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.b[761] && (!s.b[874])) {
            s.store_scalar(809, 0.0);
        }

        if s.b[761] {
            s.store_mul_ad_lhs(806, A::add_scaled_product(s.ad_value(788), 1.0, s.ad_value(809), s.ad_value(789), (-1.0)), 870);
            s.store_sub(769, 808, 806);
            s.store_scaled_mul(811, 807, 779, 2.0);
            s.store_mul(812, 782, 811);
            s.store_sub_scaled_inputs(869, 769, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            let assign11540_ad_e11787: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(868, assign11540_ad_e11787, 1.0, s.ad_value(869), (-1.0), s.ad_value(805), 1.0);
        }

        s.b[875] = (s.v[868] > 50.0);
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[875]) {
            s.store_scalar(827, 0.0);
        }

        s.b[876] = (s.v[868] < (-50.0));
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[875])) && s.b[876]) {
            s.store_scalar(827, 1.0);
        }

        if ((s.b[761] && (!s.b[875])) && (!s.b[876])) {
            s.store_div_from_scalar_offset_ad(827, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            let assign11600_ad_e11875: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(828, assign11600_ad_e11875, 1.0, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(827), (-(p.p51 * 0.1))), (-1.0), s.ad_value(811), 1.0);
        }

        s.b[877] = (s.v[828] > 50.0);
        s.v[877] = if s.b[877] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[877]) {
            s.store_mul(829, 812, 828);
        }

        s.b[878] = (s.v[828] < (-50.0));
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[877])) && s.b[878]) {
            s.store_mul_exp_rhs(829, 812, 828);
        }

        if ((s.b[761] && (!s.b[877])) && (!s.b[878])) {
            s.store_mul_ad_rhs(829, 812, A::ln_one_plus_exp(s.ad_value(828)));
        }

        if s.b[761] {
            s.store_div_ad_rhs(815, 793, A::mul_offset_rhs(s.ad_value(826), A::div_scaled_product(s.ad_value(795), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0));
        }

        if s.b[761] {
            let assign11670_ad_e11976: A = A::div_scaled_product3(s.ad_value(792), A::div_scaled_offset_numerator(A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(801), s.ad_value(870), 1.0, s.ad_value(781), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(796), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0), 1.0);
            s.store_ad_value(816, assign11670_ad_e11976);
        }

        if s.b[761] {
            s.store_div_scaled_product_indices(833, 816, 781, 1.0, 815, 1.0);
            s.store_add_scaled_product(834, s.ad_value(833), (-1.0), s.ad_value(833), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(829), 2.0, s.ad_value(782), s.ad_value(833), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(835, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(827)), 1.0, s.ad_value(811), s.ad_value(827), 1.0);
            s.store_add_scaled_product(770, A::mul_sub_from_scalar_rhs(s.ad_value(834), 1.0, s.ad_value(827)), 1.0, s.ad_value(811), s.ad_value(827), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            let assign11730_ad_e12104: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::div(s.ad_value(772), s.ad_value(770)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(770))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(772), s.ad_value(770)), A::div(s.ad_value(772), s.ad_value(770)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(836, 1.0, A::offset(A::pow(assign11730_ad_e12104, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(837, 772, 836);
        }

        if s.b[761] {
            let assign11750_ad_e12185: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(838, 1.0, A::offset(A::pow(assign11750_ad_e12185, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(839, 772, 838);
            s.store_div_scaled_inputs2(868, s.ad_value(771), 1.0, s.ad_value(869), (-1.0), s.ad_value(805), 1.0);
        }

        s.b[879] = (s.v[868] > 50.0);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[879]) {
            s.store_scalar(810, 0.0);
        }

        s.b[880] = (s.v[868] < (-50.0));
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[879])) && s.b[880]) {
            s.store_scalar(810, 1.0);
        }

        if ((s.b[761] && (!s.b[879])) && (!s.b[880])) {
            s.store_div_from_scalar_offset_ad(810, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3(813, s.ad_value(871), 1.0, s.ad_value(839), (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(810), (-(p.p51 * 0.1))), -1.0, s.ad_value(811), 1.0);
        }

        s.b[881] = (s.v[813] > 50.0);
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[881]) {
            s.store_mul(814, 812, 813);
        }

        s.b[882] = (s.v[813] < (-50.0));
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[881])) && s.b[882]) {
            s.store_mul_exp_rhs(814, 812, 813);
        }

        if ((s.b[761] && (!s.b[881])) && (!s.b[882])) {
            s.store_mul_ad_rhs(814, 812, A::ln_one_plus_exp(s.ad_value(813)));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2(868, s.ad_value(871), 1.0, s.ad_value(869), (-1.0), s.ad_value(805), 1.0);
        }

        s.b[883] = (s.v[868] > 50.0);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[883]) {
            s.store_scalar(840, 0.0);
        }

        s.b[884] = (s.v[868] < (-50.0));
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[883])) && s.b[884]) {
            s.store_scalar(840, 1.0);
        }

        if ((s.b[761] && (!s.b[883])) && (!s.b[884])) {
            s.store_div_from_scalar_offset_ad(840, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3(841, s.ad_value(771), 1.0, s.ad_value(837), (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(840), (-(p.p51 * 0.1))), -1.0, s.ad_value(811), 1.0);
        }

        s.b[885] = (s.v[841] > 50.0);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[885]) {
            s.store_mul(842, 812, 841);
        }

        s.b[886] = (s.v[841] < (-50.0));
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[885])) && s.b[886]) {
            s.store_mul_exp_rhs(842, 812, 841);
        }

        if ((s.b[761] && (!s.b[885])) && (!s.b[886])) {
            s.store_mul_ad_rhs(842, 812, A::ln_one_plus_exp(s.ad_value(841)));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2(843, s.ad_value(814), 1.0, s.ad_value(842), (-1.0), s.ad_value(782), 1.0);
            s.store_div(869, 843, 835);
            s.store_div_scaled_inputs(818, s.ad_value(787), 1.0, s.ad_value(779), 2.302585092994046);
            s.store_scaled_mul(820, 818, 779, 2.0);
            s.store_mul(821, 782, 820);
            s.store_sub_scaled_inputs(873, 808, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            let assign12100_ad_e12566: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(872, assign12100_ad_e12566, 1.0, s.ad_value(873), (-1.0), s.ad_value(805), 1.0);
        }

        s.b[887] = (s.v[872] > 50.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[887]) {
            s.store_scalar(830, 0.0);
        }

        s.b[888] = (s.v[872] < (-50.0));
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[887])) && s.b[888]) {
            s.store_scalar(830, 1.0);
        }

        if ((s.b[761] && (!s.b[887])) && (!s.b[888])) {
            s.store_div_from_scalar_offset_ad(830, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            let assign12160_ad_e12654: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(831, assign12160_ad_e12654, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(830), (-(p.p51 * 0.1))), (-1.0), s.ad_value(820), 1.0);
        }

        s.b[889] = (s.v[831] > 50.0);
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[889]) {
            s.store_mul(832, 821, 831);
        }

        s.b[890] = (s.v[831] < (-50.0));
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[889])) && s.b[890]) {
            s.store_mul_exp_rhs(832, 821, 831);
        }

        if ((s.b[761] && (!s.b[889])) && (!s.b[890])) {
            s.store_mul_ad_rhs(832, 821, A::ln_one_plus_exp(s.ad_value(831)));
        }

        if s.b[761] {
            s.store_div(824, 793, 826);
            s.store_mul_ad_rhs(825, 792, A::div_scaled_offset_numerator(A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0));
            s.store_div_scaled_product_indices(846, 825, 781, 1.0, 824, 1.0);
            s.store_add_scaled_product(847, s.ad_value(846), (-1.0), s.ad_value(846), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(832), 2.0, s.ad_value(782), s.ad_value(846), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(848, A::mul_sub_from_scalar_rhs(s.ad_value(847), 1.0, s.ad_value(830)), 1.0, s.ad_value(820), s.ad_value(830), 1.0);
        }

        if s.b[761] {
            let assign12270_ad_e12829: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::div(s.ad_value(772), s.ad_value(848)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(772), s.ad_value(848)), A::div(s.ad_value(772), s.ad_value(848)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(849, 1.0, A::offset(A::pow(assign12270_ad_e12829, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(850, 772, 849);
        }

        if s.b[761] {
            let assign12290_ad_e12910: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(851, 1.0, A::offset(A::pow(assign12290_ad_e12910, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(852, 772, 851);
            s.store_div_scaled_inputs2(872, s.ad_value(771), 1.0, s.ad_value(873), (-1.0), s.ad_value(805), 1.0);
        }

        s.b[891] = (s.v[872] > 50.0);
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[891]) {
            s.store_scalar(819, 0.0);
        }

        s.b[892] = (s.v[872] < (-50.0));
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[891])) && s.b[892]) {
            s.store_scalar(819, 1.0);
        }

        if ((s.b[761] && (!s.b[891])) && (!s.b[892])) {
            s.store_div_from_scalar_offset_ad(819, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3(822, s.ad_value(871), 1.0, s.ad_value(852), (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(819), (-(p.p51 * 0.1))), -1.0, s.ad_value(820), 1.0);
        }

        s.b[893] = (s.v[822] > 50.0);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[893]) {
            s.store_mul(823, 821, 822);
        }

        s.b[894] = (s.v[822] < (-50.0));
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[893])) && s.b[894]) {
            s.store_mul_exp_rhs(823, 821, 822);
        }

        if ((s.b[761] && (!s.b[893])) && (!s.b[894])) {
            s.store_mul_ad_rhs(823, 821, A::ln_one_plus_exp(s.ad_value(822)));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2(872, s.ad_value(871), 1.0, s.ad_value(873), (-1.0), s.ad_value(805), 1.0);
        }

        s.b[895] = (s.v[872] > 50.0);
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[895]) {
            s.store_scalar(853, 0.0);
        }

        s.b[896] = (s.v[872] < (-50.0));
        s.v[896] = if s.b[896] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[895])) && s.b[896]) {
            s.store_scalar(853, 1.0);
        }

        if ((s.b[761] && (!s.b[895])) && (!s.b[896])) {
            s.store_div_from_scalar_offset_ad(853, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3(854, s.ad_value(771), 1.0, s.ad_value(850), (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(853), (-(p.p51 * 0.1))), -1.0, s.ad_value(820), 1.0);
        }

        s.b[897] = (s.v[854] > 50.0);
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[897]) {
            s.store_mul(855, 821, 854);
        }

        s.b[898] = (s.v[854] < (-50.0));
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[897])) && s.b[898]) {
            s.store_mul_exp_rhs(855, 821, 854);
        }

        if ((s.b[761] && (!s.b[897])) && (!s.b[898])) {
            s.store_mul_ad_rhs(855, 821, A::ln_one_plus_exp(s.ad_value(854)));
        }

        if s.b[761] {
            s.store_offset_square(856, 823, 1e-38);
            s.store_offset_mul(857, 856, 823, 1e-57);
            s.store_offset_square(858, 855, 1e-38);
            s.store_offset_mul(859, 858, 855, 1e-57);
            s.store_offset_mul(860, 823, 855, 1e-38);
            s.store_div_scaled_inputs3(861, s.ad_value(856), (2.0 / 3.0), s.ad_value(858), (2.0 / 3.0), s.ad_value(860), (2.0 / 3.0), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19), 1.0);
            s.store_div_ad(862, A::add_scaled_inputs_products(s.ad_value(857), (2.0 * 2.0), s.ad_value(859), (3.0 * 2.0), s.ad_value(856), s.ad_value(855), (4.0 * 2.0), s.ad_value(858), s.ad_value(823), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(856), 15.0, s.ad_value(858), 15.0, s.ad_value(860), (2.0 * 15.0)));
            s.store_sub(863, 861, 862);
            s.copy_ad(864, 862);
            s.store_mul_ad_lhs(764, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), s.ad_value(803), s.ad_value(863)), 804);
            s.store_mul_ad_lhs(765, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), s.ad_value(803), s.ad_value(864)), 804);
        }

        s.b[899] = (s.v[773] == 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[899]) {
            s.store_div_scaled_inputs3(865, s.ad_value(774), 1.0, s.ad_value(808), -1.0, s.ad_value(805), (-(-(p.p51 * 0.5))), s.ad_value(820), 1.0);
        }

        s.b[900] = (s.v[865] > 50.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if ((s.b[761] && s.b[899]) && s.b[900]) {
            s.copy_ad(868, 865);
        }

        s.b[901] = (s.v[865] < (-50.0));
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && s.b[901]) {
            s.store_exp(868, 865);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && (!s.b[901])) {
            s.store_ln_one_plus_exp(868, 865);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs(766, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(784), s.ad_value(820)), s.ad_value(868), 804);
            s.store_div_scaled_inputs3(866, s.ad_value(775), 1.0, s.ad_value(808), -1.0, s.ad_value(805), (-(-(p.p51 * 0.5))), s.ad_value(820), 1.0);
        }

        s.b[902] = (s.v[866] > 50.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if ((s.b[761] && s.b[899]) && s.b[902]) {
            s.copy_ad(868, 866);
        }

        s.b[903] = (s.v[866] < (-50.0));
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && s.b[903]) {
            s.store_exp(868, 866);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && (!s.b[903])) {
            s.store_ln_one_plus_exp(868, 866);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs(767, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(785), s.ad_value(820)), s.ad_value(868), 804);
        }

        if (s.b[761] && (!s.b[899])) {
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
        }

        s.b[904] = (s.v[776] == 1.0);
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[904]) {
            s.store_div_scaled_inputs3(867, s.ad_value(771), 1.0, s.ad_value(808), -1.0, s.ad_value(805), (-(-(p.p51 * 0.5))), s.ad_value(820), 1.0);
        }

        s.b[905] = (s.v[867] > 50.0);
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        if ((s.b[761] && s.b[904]) && s.b[905]) {
            s.copy_ad(868, 867);
        }

        s.b[906] = (s.v[867] < (-50.0));
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && s.b[906]) {
            s.store_exp(868, 867);
        }

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && (!s.b[906])) {
            s.store_ln_one_plus_exp(868, 867);
        }

        if (s.b[761] && s.b[904]) {
            s.store_mul_ad_product_lhs(768, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(783), s.ad_value(820)), s.ad_value(868), 804);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[761] && (!s.b[904])) {
            s.store_scalar(768, 0.0);
        }

        if s.b[761] {
            s.copy_ad(191, 764);
            s.copy_ad(192, 765);
            s.copy_ad(193, 766);
            s.copy_ad(194, 767);
            s.copy_ad(195, 768);
        }

        s.b[907] = (p.p166 == 1.0);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        s.v[167] = 0.0;

        s.v[168] = 0.0;

        s.v[169] = 0.0;

        s.v[170] = 0.0;

        s.v[171] = 0.0;

        s.b[908] = (p.p79 > p.p354);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if s.b[908] {
            s.store_scalar(911, 0.0);
            s.store_scalar(912, 0.0);
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
            s.store_scalar(915, 0.0);
            s.store_scalar(916, 0.0);
            s.store_scalar(917, 0.0);
            s.copy_ad(918, 60);
            s.copy_ad(919, 61);
            s.store_scalar(920, p.p85);
            s.copy_ad(921, 62);
            s.copy_ad(922, 63);
            s.store_scalar(923, p.p83);
            s.copy_ad(924, 111);
            s.store_scalar(925, s.v[109]);
            s.copy_ad(926, 113);
            s.store_scalar(927, p.p0);
            s.store_scalar(928, p.p79);
            s.copy_ad(929, 20);
            s.store_scalar(930, p.p84);
            s.copy_ad(931, 21);
            s.copy_ad(932, 22);
            s.store_scalar(933, p.p80);
            s.store_scalar(934, p.p94);
            s.store_scalar(935, p.p93);
            s.store_scalar(936, 0.0);
            s.store_scalar(937, p.p95);
            s.store_scalar(938, p.p99);
            s.store_scalar(939, p.p90);
            s.store_scalar(940, p.p91);
            s.store_scalar(941, p.p92);
            s.store_scalar(942, p.p98);
            s.store_scalar(943, p.p97);
            s.store_scalar(944, p.p96);
            s.store_scalar(945, p.p39);
            s.store_scalar(946, p.p47);
            s.store_scalar(947, p.p45);
            s.store_scalar(948, p.p42);
            s.store_scalar(949, p.p2);
            s.store_scalar(950, p.p6);
            s.store_scalar(951, 1.0);
            s.store_scalar(952, 0.0);
            s.store_scalar(953, 0.0);
            s.store_scalar(954, 0.0);
            s.store_scalar(955, 0.0);
            s.store_scalar(956, 0.0);
            s.store_scalar(957, 0.0);
            s.store_scalar(958, 0.0);
            s.store_scalar(959, 0.0);
            s.store_scalar(960, 0.0);
            s.store_scalar(961, 0.0);
            s.store_scalar(962, 0.0);
            s.store_scalar(963, 0.0);
            s.store_scalar(965, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(970, 0.0);
            s.store_scalar(971, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
            s.store_scalar(976, 0.0);
            s.store_scalar(977, 0.0);
            s.store_scalar(978, 0.0);
            s.store_scalar(979, 0.0);
            s.store_scalar(980, 0.0);
            s.store_scalar(981, 0.0);
            s.store_scalar(982, 0.0);
            s.store_scalar(983, 0.0);
            s.store_scalar(984, 0.0);
            s.store_scalar(985, 0.0);
            s.store_scalar(986, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(989, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(993, 0.0);
            s.store_scalar(994, 0.0);
            s.store_scalar(995, 0.0);
            s.store_scalar(996, 0.0);
            s.store_scalar(997, 0.0);
            s.store_scalar(998, 0.0);
            s.store_scalar(999, 0.0);
            s.store_scalar(1000, 0.0);
            s.store_scalar(1001, 0.0);
            s.store_scalar(1002, 0.0);
            s.store_scalar(1003, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1010, 0.0);
            s.store_scalar(1011, 0.0);
            s.store_scalar(1012, 0.0);
            s.store_scalar(1013, 0.0);
            s.store_scalar(1014, 0.0);
            s.store_scalar(1015, 0.0);
            s.store_scalar(1016, 0.0);
            s.store_scalar(1017, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
        }

        if s.b[908] {
            s.store_ad_value(1017, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(919), A::tanh_scaled_input(s.ad_value(919), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(919)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[908] {
            s.store_sub(1018, 918, 919);
            s.store_mul(952, 938, 926);
            s.store_add_scaled_product(954, A::div_scaled_inputs(s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046), 1.0, s.ad_value(937), s.ad_value(1017), 1.0);
            s.store_add_scaled_product(955, s.ad_value(933), 1.0, s.ad_value(944), A::sub(s.ad_value(924), s.ad_value(925)), 1.0);
            s.store_pow_ad(973, A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946));
        }

        s.b[1021] = (s.v[945] != 0.0);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1021]) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[908] && (!s.b[1021])) {
            s.store_scalar(956, 0.0);
        }

        if s.b[908] {
            s.store_mul_ad_lhs(953, A::add_scaled_product(s.ad_value(935), 1.0, s.ad_value(956), s.ad_value(936), (-1.0)), 1017);
            s.store_sub(916, 955, 953);
            s.store_scaled_mul(958, 954, 926, 2.0);
            s.store_mul(959, 929, 958);
            s.store_sub_scaled_inputs(1016, 916, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            let assign14380_ad_e14211: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1015, assign14380_ad_e14211, 1.0, s.ad_value(1016), (-1.0), s.ad_value(952), 1.0);
        }

        s.b[1022] = (s.v[1015] > 50.0);
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1022]) {
            s.store_scalar(974, 0.0);
        }

        s.b[1023] = (s.v[1015] < (-50.0));
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1022])) && s.b[1023]) {
            s.store_scalar(974, 1.0);
        }

        if ((s.b[908] && (!s.b[1022])) && (!s.b[1023])) {
            s.store_div_from_scalar_offset_ad(974, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            let assign14440_ad_e14299: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(975, assign14440_ad_e14299, 1.0, A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(974), (-(p.p51 * 0.1))), (-1.0), s.ad_value(958), 1.0);
        }

        s.b[1024] = (s.v[975] > 50.0);
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1024]) {
            s.store_mul(976, 959, 975);
        }

        s.b[1025] = (s.v[975] < (-50.0));
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1024])) && s.b[1025]) {
            s.store_mul_exp_rhs(976, 959, 975);
        }

        if ((s.b[908] && (!s.b[1024])) && (!s.b[1025])) {
            s.store_mul_ad_rhs(976, 959, A::ln_one_plus_exp(s.ad_value(975)));
        }

        if s.b[908] {
            s.store_div_ad_rhs(962, 940, A::mul_offset_rhs(s.ad_value(973), A::div_scaled_product(s.ad_value(942), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0));
        }

        if s.b[908] {
            let assign14510_ad_e14400: A = A::div_scaled_product3(s.ad_value(939), A::div_scaled_offset_numerator(A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(948), s.ad_value(1017), 1.0, s.ad_value(928), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(943), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0), 1.0);
            s.store_ad_value(963, assign14510_ad_e14400);
        }

        if s.b[908] {
            s.store_div_scaled_product_indices(980, 963, 928, 1.0, 962, 1.0);
            s.store_add_scaled_product(981, s.ad_value(980), (-1.0), s.ad_value(980), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(976), 2.0, s.ad_value(929), s.ad_value(980), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(982, A::mul_sub_from_scalar_rhs(s.ad_value(980), 1.0, s.ad_value(974)), 1.0, s.ad_value(958), s.ad_value(974), 1.0);
            s.store_add_scaled_product(917, A::mul_sub_from_scalar_rhs(s.ad_value(981), 1.0, s.ad_value(974)), 1.0, s.ad_value(958), s.ad_value(974), 1.0);
        }

        if s.b[908] {
            let assign14570_ad_e14528: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::div(s.ad_value(919), s.ad_value(917)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(919), s.ad_value(917)), A::div(s.ad_value(919), s.ad_value(917)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(983, 1.0, A::offset(A::pow(assign14570_ad_e14528, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(984, 919, 983);
        }

        if s.b[908] {
            let assign14590_ad_e14609: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(985, 1.0, A::offset(A::pow(assign14590_ad_e14609, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(986, 919, 985);
            s.store_div_scaled_inputs2(1015, s.ad_value(918), 1.0, s.ad_value(1016), (-1.0), s.ad_value(952), 1.0);
        }

        s.b[1026] = (s.v[1015] > 50.0);
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1026]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1027] = (s.v[1015] < (-50.0));
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1026])) && s.b[1027]) {
            s.store_scalar(957, 1.0);
        }

        if ((s.b[908] && (!s.b[1026])) && (!s.b[1027])) {
            s.store_div_from_scalar_offset_ad(957, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3(960, s.ad_value(1018), 1.0, s.ad_value(986), (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(957), (-(p.p51 * 0.1))), -1.0, s.ad_value(958), 1.0);
        }

        s.b[1028] = (s.v[960] > 50.0);
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1028]) {
            s.store_mul(961, 959, 960);
        }

        s.b[1029] = (s.v[960] < (-50.0));
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1028])) && s.b[1029]) {
            s.store_mul_exp_rhs(961, 959, 960);
        }

        if ((s.b[908] && (!s.b[1028])) && (!s.b[1029])) {
            s.store_mul_ad_rhs(961, 959, A::ln_one_plus_exp(s.ad_value(960)));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2(1015, s.ad_value(1018), 1.0, s.ad_value(1016), (-1.0), s.ad_value(952), 1.0);
        }

        s.b[1030] = (s.v[1015] > 50.0);
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1030]) {
            s.store_scalar(987, 0.0);
        }

        s.b[1031] = (s.v[1015] < (-50.0));
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1030])) && s.b[1031]) {
            s.store_scalar(987, 1.0);
        }

        if ((s.b[908] && (!s.b[1030])) && (!s.b[1031])) {
            s.store_div_from_scalar_offset_ad(987, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3(988, s.ad_value(918), 1.0, s.ad_value(984), (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(987), (-(p.p51 * 0.1))), -1.0, s.ad_value(958), 1.0);
        }

        s.b[1032] = (s.v[988] > 50.0);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1032]) {
            s.store_mul(989, 959, 988);
        }

        s.b[1033] = (s.v[988] < (-50.0));
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1032])) && s.b[1033]) {
            s.store_mul_exp_rhs(989, 959, 988);
        }

        if ((s.b[908] && (!s.b[1032])) && (!s.b[1033])) {
            s.store_mul_ad_rhs(989, 959, A::ln_one_plus_exp(s.ad_value(988)));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2(990, s.ad_value(961), 1.0, s.ad_value(989), (-1.0), s.ad_value(929), 1.0);
            s.store_div(1016, 990, 982);
            s.store_div_scaled_inputs(965, s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046);
            s.store_scaled_mul(967, 965, 926, 2.0);
            s.store_mul(968, 929, 967);
            s.store_sub_scaled_inputs(1020, 955, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            let assign14940_ad_e14990: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1019, assign14940_ad_e14990, 1.0, s.ad_value(1020), (-1.0), s.ad_value(952), 1.0);
        }

        s.b[1034] = (s.v[1019] > 50.0);
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1034]) {
            s.store_scalar(977, 0.0);
        }

        s.b[1035] = (s.v[1019] < (-50.0));
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1034])) && s.b[1035]) {
            s.store_scalar(977, 1.0);
        }

        if ((s.b[908] && (!s.b[1034])) && (!s.b[1035])) {
            s.store_div_from_scalar_offset_ad(977, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            let assign15000_ad_e15078: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(978, assign15000_ad_e15078, 1.0, A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(977), (-(p.p51 * 0.1))), (-1.0), s.ad_value(967), 1.0);
        }

        s.b[1036] = (s.v[978] > 50.0);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1036]) {
            s.store_mul(979, 968, 978);
        }

        s.b[1037] = (s.v[978] < (-50.0));
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1036])) && s.b[1037]) {
            s.store_mul_exp_rhs(979, 968, 978);
        }

        if ((s.b[908] && (!s.b[1036])) && (!s.b[1037])) {
            s.store_mul_ad_rhs(979, 968, A::ln_one_plus_exp(s.ad_value(978)));
        }

        if s.b[908] {
            s.store_div(971, 940, 973);
            s.store_mul_ad_rhs(972, 939, A::div_scaled_offset_numerator(A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0));
            s.store_div_scaled_product_indices(993, 972, 928, 1.0, 971, 1.0);
            s.store_add_scaled_product(994, s.ad_value(993), (-1.0), s.ad_value(993), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(979), 2.0, s.ad_value(929), s.ad_value(993), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(995, A::mul_sub_from_scalar_rhs(s.ad_value(994), 1.0, s.ad_value(977)), 1.0, s.ad_value(967), s.ad_value(977), 1.0);
        }

        if s.b[908] {
            let assign15110_ad_e15253: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::div(s.ad_value(919), s.ad_value(995)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(919), s.ad_value(995)), A::div(s.ad_value(919), s.ad_value(995)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(996, 1.0, A::offset(A::pow(assign15110_ad_e15253, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(997, 919, 996);
        }

        if s.b[908] {
            let assign15130_ad_e15334: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(998, 1.0, A::offset(A::pow(assign15130_ad_e15334, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(999, 919, 998);
            s.store_div_scaled_inputs2(1019, s.ad_value(918), 1.0, s.ad_value(1020), (-1.0), s.ad_value(952), 1.0);
        }

        s.b[1038] = (s.v[1019] > 50.0);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1038]) {
            s.store_scalar(966, 0.0);
        }

        s.b[1039] = (s.v[1019] < (-50.0));
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1038])) && s.b[1039]) {
            s.store_scalar(966, 1.0);
        }

        if ((s.b[908] && (!s.b[1038])) && (!s.b[1039])) {
            s.store_div_from_scalar_offset_ad(966, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3(969, s.ad_value(1018), 1.0, s.ad_value(999), (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(966), (-(p.p51 * 0.1))), -1.0, s.ad_value(967), 1.0);
        }

        s.b[1040] = (s.v[969] > 50.0);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1040]) {
            s.store_mul(970, 968, 969);
        }

        s.b[1041] = (s.v[969] < (-50.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1040])) && s.b[1041]) {
            s.store_mul_exp_rhs(970, 968, 969);
        }

        if ((s.b[908] && (!s.b[1040])) && (!s.b[1041])) {
            s.store_mul_ad_rhs(970, 968, A::ln_one_plus_exp(s.ad_value(969)));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2(1019, s.ad_value(1018), 1.0, s.ad_value(1020), (-1.0), s.ad_value(952), 1.0);
        }

        s.b[1042] = (s.v[1019] > 50.0);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1042]) {
            s.store_scalar(1000, 0.0);
        }

        s.b[1043] = (s.v[1019] < (-50.0));
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(1000, 1.0);
        }

        if ((s.b[908] && (!s.b[1042])) && (!s.b[1043])) {
            s.store_div_from_scalar_offset_ad(1000, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3(1001, s.ad_value(918), 1.0, s.ad_value(997), (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(1000), (-(p.p51 * 0.1))), -1.0, s.ad_value(967), 1.0);
        }

        s.b[1044] = (s.v[1001] > 50.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1044]) {
            s.store_mul(1002, 968, 1001);
        }

        s.b[1045] = (s.v[1001] < (-50.0));
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1044])) && s.b[1045]) {
            s.store_mul_exp_rhs(1002, 968, 1001);
        }

        if ((s.b[908] && (!s.b[1044])) && (!s.b[1045])) {
            s.store_mul_ad_rhs(1002, 968, A::ln_one_plus_exp(s.ad_value(1001)));
        }

        if s.b[908] {
            s.store_offset_square(1003, 970, 1e-38);
            s.store_offset_mul(1004, 1003, 970, 1e-57);
            s.store_offset_square(1005, 1002, 1e-38);
            s.store_offset_mul(1006, 1005, 1002, 1e-57);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[908] {
            s.store_offset_mul(1007, 970, 1002, 1e-38);
            s.store_div_scaled_inputs3(1008, s.ad_value(1003), (2.0 / 3.0), s.ad_value(1005), (2.0 / 3.0), s.ad_value(1007), (2.0 / 3.0), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19), 1.0);
            s.store_div_ad(1009, A::add_scaled_inputs_products(s.ad_value(1004), (2.0 * 2.0), s.ad_value(1006), (3.0 * 2.0), s.ad_value(1003), s.ad_value(1002), (4.0 * 2.0), s.ad_value(1005), s.ad_value(970), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1003), 15.0, s.ad_value(1005), 15.0, s.ad_value(1007), (2.0 * 15.0)));
            s.store_sub(1010, 1008, 1009);
            s.copy_ad(1011, 1009);
            s.store_mul_ad_lhs(911, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), s.ad_value(950), s.ad_value(1010)), 951);
            s.store_mul_ad_lhs(912, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), s.ad_value(950), s.ad_value(1011)), 951);
        }

        s.b[1046] = (s.v[920] == 1.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1046]) {
            s.store_div_scaled_inputs3(1012, s.ad_value(921), 1.0, s.ad_value(955), -1.0, s.ad_value(952), (-(-(p.p51 * 0.5))), s.ad_value(967), 1.0);
        }

        s.b[1047] = (s.v[1012] > 50.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if ((s.b[908] && s.b[1046]) && s.b[1047]) {
            s.copy_ad(1015, 1012);
        }

        s.b[1048] = (s.v[1012] < (-50.0));
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_exp(1015, 1012);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) {
            s.store_ln_one_plus_exp(1015, 1012);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs(913, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(931), s.ad_value(967)), s.ad_value(1015), 951);
            s.store_div_scaled_inputs3(1013, s.ad_value(922), 1.0, s.ad_value(955), -1.0, s.ad_value(952), (-(-(p.p51 * 0.5))), s.ad_value(967), 1.0);
        }

        s.b[1049] = (s.v[1013] > 50.0);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((s.b[908] && s.b[1046]) && s.b[1049]) {
            s.copy_ad(1015, 1013);
        }

        s.b[1050] = (s.v[1013] < (-50.0));
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && s.b[1050]) {
            s.store_exp(1015, 1013);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && (!s.b[1050])) {
            s.store_ln_one_plus_exp(1015, 1013);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs(914, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(932), s.ad_value(967)), s.ad_value(1015), 951);
        }

        if (s.b[908] && (!s.b[1046])) {
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
        }

        s.b[1051] = (s.v[923] == 1.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1051]) {
            s.store_div_scaled_inputs3(1014, s.ad_value(918), 1.0, s.ad_value(955), -1.0, s.ad_value(952), (-(-(p.p51 * 0.5))), s.ad_value(967), 1.0);
        }

        s.b[1052] = (s.v[1014] > 50.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if ((s.b[908] && s.b[1051]) && s.b[1052]) {
            s.copy_ad(1015, 1014);
        }

        s.b[1053] = (s.v[1014] < (-50.0));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && s.b[1053]) {
            s.store_exp(1015, 1014);
        }

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && (!s.b[1053])) {
            s.store_ln_one_plus_exp(1015, 1014);
        }

        if (s.b[908] && s.b[1051]) {
            s.store_mul_ad_product_lhs(915, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(930), s.ad_value(967)), s.ad_value(1015), 951);
        }

        if (s.b[908] && (!s.b[1051])) {
            s.store_scalar(915, 0.0);
        }

        if s.b[908] {
            s.copy_ad(167, 911);
            s.copy_ad(168, 912);
            s.copy_ad(169, 913);
            s.copy_ad(170, 914);
            s.copy_ad(171, 915);
        }

        s.b[1054] = (p.p78 == 1.0);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        s.v[173] = 0.0;

        s.v[174] = 0.0;

        s.v[175] = 0.0;

        s.v[176] = 0.0;

        s.v[177] = 0.0;

        s.b[1055] = (p.p101 > p.p354);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if s.b[1055] {
            s.store_scalar(1058, 0.0);
            s.store_scalar(1059, 0.0);
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
            s.store_scalar(1062, 0.0);
            s.store_scalar(1063, 0.0);
            s.store_scalar(1064, 0.0);
            s.copy_ad(1065, 66);
            s.copy_ad(1066, 67);
            s.store_scalar(1067, p.p107);
            s.copy_ad(1068, 68);
            s.copy_ad(1069, 69);
            s.store_scalar(1070, p.p105);
            s.copy_ad(1071, 111);
            s.store_scalar(1072, s.v[109]);
            s.copy_ad(1073, 113);
            s.store_scalar(1074, p.p0);
            s.store_scalar(1075, p.p101);
            s.copy_ad(1076, 23);
            s.store_scalar(1077, p.p106);
            s.copy_ad(1078, 24);
            s.copy_ad(1079, 25);
            s.store_scalar(1080, p.p102);
            s.store_scalar(1081, p.p116);
            s.store_scalar(1082, p.p115);
            s.store_scalar(1083, 0.0);
            s.store_scalar(1084, p.p117);
            s.store_scalar(1085, p.p121);
            s.store_scalar(1086, p.p112);
            s.store_scalar(1087, p.p113);
            s.store_scalar(1088, p.p114);
            s.store_scalar(1089, p.p120);
            s.store_scalar(1090, p.p119);
            s.store_scalar(1091, p.p118);
            s.store_scalar(1092, p.p39);
            s.store_scalar(1093, p.p47);
            s.store_scalar(1094, p.p45);
            s.store_scalar(1095, p.p42);
            s.store_scalar(1096, p.p2);
            s.store_scalar(1097, p.p6);
            s.store_scalar(1098, 1.0);
            s.store_scalar(1099, 0.0);
            s.store_scalar(1100, 0.0);
            s.store_scalar(1101, 0.0);
            s.store_scalar(1102, 0.0);
            s.store_scalar(1103, 0.0);
            s.store_scalar(1104, 0.0);
            s.store_scalar(1105, 0.0);
            s.store_scalar(1106, 0.0);
            s.store_scalar(1107, 0.0);
            s.store_scalar(1108, 0.0);
            s.store_scalar(1109, 0.0);
            s.store_scalar(1110, 0.0);
            s.store_scalar(1112, 0.0);
            s.store_scalar(1113, 0.0);
            s.store_scalar(1114, 0.0);
            s.store_scalar(1115, 0.0);
            s.store_scalar(1116, 0.0);
            s.store_scalar(1117, 0.0);
            s.store_scalar(1118, 0.0);
            s.store_scalar(1119, 0.0);
            s.store_scalar(1120, 0.0);
            s.store_scalar(1121, 0.0);
            s.store_scalar(1122, 0.0);
            s.store_scalar(1123, 0.0);
            s.store_scalar(1124, 0.0);
            s.store_scalar(1125, 0.0);
            s.store_scalar(1126, 0.0);
            s.store_scalar(1127, 0.0);
            s.store_scalar(1128, 0.0);
            s.store_scalar(1129, 0.0);
            s.store_scalar(1130, 0.0);
            s.store_scalar(1131, 0.0);
            s.store_scalar(1132, 0.0);
            s.store_scalar(1133, 0.0);
            s.store_scalar(1134, 0.0);
            s.store_scalar(1135, 0.0);
            s.store_scalar(1136, 0.0);
            s.store_scalar(1137, 0.0);
            s.store_scalar(1140, 0.0);
            s.store_scalar(1141, 0.0);
            s.store_scalar(1142, 0.0);
            s.store_scalar(1143, 0.0);
            s.store_scalar(1144, 0.0);
            s.store_scalar(1145, 0.0);
            s.store_scalar(1146, 0.0);
            s.store_scalar(1147, 0.0);
            s.store_scalar(1148, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_scalar(1149, 0.0);
            s.store_scalar(1150, 0.0);
            s.store_scalar(1151, 0.0);
            s.store_scalar(1152, 0.0);
            s.store_scalar(1153, 0.0);
            s.store_scalar(1154, 0.0);
            s.store_scalar(1155, 0.0);
            s.store_scalar(1156, 0.0);
            s.store_scalar(1157, 0.0);
            s.store_scalar(1158, 0.0);
            s.store_scalar(1159, 0.0);
            s.store_scalar(1160, 0.0);
            s.store_scalar(1161, 0.0);
            s.store_scalar(1162, 0.0);
            s.store_scalar(1163, 0.0);
            s.store_scalar(1164, 0.0);
            s.store_scalar(1165, 0.0);
            s.store_scalar(1166, 0.0);
            s.store_scalar(1167, 0.0);
        }

        if s.b[1055] {
            s.store_ad_value(1164, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1066), A::tanh_scaled_input(s.ad_value(1066), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1066)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1055] {
            s.store_sub(1165, 1065, 1066);
            s.store_mul(1099, 1085, 1073);
            s.store_add_scaled_product(1101, A::div_scaled_inputs(s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046), 1.0, s.ad_value(1084), s.ad_value(1164), 1.0);
            s.store_add_scaled_product(1102, s.ad_value(1080), 1.0, s.ad_value(1091), A::sub(s.ad_value(1071), s.ad_value(1072)), 1.0);
            s.store_pow_ad(1120, A::div(s.ad_value(1071), s.ad_value(1072)), s.ad_value(1093));
        }

        s.b[1168] = (s.v[1092] != 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1168]) {
            s.store_div_ad_rhs(1103, 1164, A::pow(A::offset(A::pow(A::div(s.ad_value(1164), s.ad_value(1092)), s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.b[1055] && (!s.b[1168])) {
            s.store_scalar(1103, 0.0);
        }

        if s.b[1055] {
            s.store_mul_ad_lhs(1100, A::add_scaled_product(s.ad_value(1082), 1.0, s.ad_value(1103), s.ad_value(1083), (-1.0)), 1164);
            s.store_sub(1063, 1102, 1100);
            s.store_scaled_mul(1105, 1101, 1073, 2.0);
            s.store_mul(1106, 1076, 1105);
            s.store_sub_scaled_inputs(1163, 1063, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            let assign17220_ad_e16635: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1162, assign17220_ad_e16635, 1.0, s.ad_value(1163), (-1.0), s.ad_value(1099), 1.0);
        }

        s.b[1169] = (s.v[1162] > 50.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1169]) {
            s.store_scalar(1121, 0.0);
        }

        s.b[1170] = (s.v[1162] < (-50.0));
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1169])) && s.b[1170]) {
            s.store_scalar(1121, 1.0);
        }

        if ((s.b[1055] && (!s.b[1169])) && (!s.b[1170])) {
            s.store_div_from_scalar_offset_ad(1121, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            let assign17280_ad_e16723: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1122, assign17280_ad_e16723, 1.0, A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1121), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1105), 1.0);
        }

        s.b[1171] = (s.v[1122] > 50.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1171]) {
            s.store_mul(1123, 1106, 1122);
        }

        s.b[1172] = (s.v[1122] < (-50.0));
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1171])) && s.b[1172]) {
            s.store_mul_exp_rhs(1123, 1106, 1122);
        }

        if ((s.b[1055] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_mul_ad_rhs(1123, 1106, A::ln_one_plus_exp(s.ad_value(1122)));
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1109, 1087, A::mul_offset_rhs(s.ad_value(1120), A::div_scaled_product(s.ad_value(1089), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0));
        }

        if s.b[1055] {
            let assign17350_ad_e16824: A = A::div_scaled_product3(s.ad_value(1086), A::div_scaled_offset_numerator(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1095), s.ad_value(1164), 1.0, s.ad_value(1075), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1090), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0), 1.0);
            s.store_ad_value(1110, assign17350_ad_e16824);
        }

        if s.b[1055] {
            s.store_div_scaled_product_indices(1127, 1110, 1075, 1.0, 1109, 1.0);
            s.store_add_scaled_product(1128, s.ad_value(1127), (-1.0), s.ad_value(1127), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1123), 2.0, s.ad_value(1076), s.ad_value(1127), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(1129, A::mul_sub_from_scalar_rhs(s.ad_value(1127), 1.0, s.ad_value(1121)), 1.0, s.ad_value(1105), s.ad_value(1121), 1.0);
            s.store_add_scaled_product(1064, A::mul_sub_from_scalar_rhs(s.ad_value(1128), 1.0, s.ad_value(1121)), 1.0, s.ad_value(1105), s.ad_value(1121), 1.0);
        }

        if s.b[1055] {
            let assign17410_ad_e16952: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::div(s.ad_value(1066), s.ad_value(1064)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1066), s.ad_value(1064)), A::div(s.ad_value(1066), s.ad_value(1064)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1130, 1.0, A::offset(A::pow(assign17410_ad_e16952, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1131, 1066, 1130);
        }

        if s.b[1055] {
            let assign17430_ad_e17033: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1132, 1.0, A::offset(A::pow(assign17430_ad_e17033, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1133, 1066, 1132);
            s.store_div_scaled_inputs2(1162, s.ad_value(1065), 1.0, s.ad_value(1163), (-1.0), s.ad_value(1099), 1.0);
        }

        s.b[1173] = (s.v[1162] > 50.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1173]) {
            s.store_scalar(1104, 0.0);
        }

        s.b[1174] = (s.v[1162] < (-50.0));
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1173])) && s.b[1174]) {
            s.store_scalar(1104, 1.0);
        }

        if ((s.b[1055] && (!s.b[1173])) && (!s.b[1174])) {
            s.store_div_from_scalar_offset_ad(1104, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3(1107, s.ad_value(1165), 1.0, s.ad_value(1133), (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1104), (-(p.p51 * 0.1))), -1.0, s.ad_value(1105), 1.0);
        }

        s.b[1175] = (s.v[1107] > 50.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1175]) {
            s.store_mul(1108, 1106, 1107);
        }

        s.b[1176] = (s.v[1107] < (-50.0));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1175])) && s.b[1176]) {
            s.store_mul_exp_rhs(1108, 1106, 1107);
        }

        if ((s.b[1055] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_mul_ad_rhs(1108, 1106, A::ln_one_plus_exp(s.ad_value(1107)));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2(1162, s.ad_value(1165), 1.0, s.ad_value(1163), (-1.0), s.ad_value(1099), 1.0);
        }

        s.b[1177] = (s.v[1162] > 50.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1177]) {
            s.store_scalar(1134, 0.0);
        }

        s.b[1178] = (s.v[1162] < (-50.0));
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(1134, 1.0);
        }

        if ((s.b[1055] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_div_from_scalar_offset_ad(1134, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3(1135, s.ad_value(1065), 1.0, s.ad_value(1131), (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1134), (-(p.p51 * 0.1))), -1.0, s.ad_value(1105), 1.0);
        }

        s.b[1179] = (s.v[1135] > 50.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1179]) {
            s.store_mul(1136, 1106, 1135);
        }

        s.b[1180] = (s.v[1135] < (-50.0));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1179])) && s.b[1180]) {
            s.store_mul_exp_rhs(1136, 1106, 1135);
        }

        if ((s.b[1055] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_mul_ad_rhs(1136, 1106, A::ln_one_plus_exp(s.ad_value(1135)));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2(1137, s.ad_value(1108), 1.0, s.ad_value(1136), (-1.0), s.ad_value(1076), 1.0);
            s.store_div(1163, 1137, 1129);
            s.store_div_scaled_inputs(1112, s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046);
            s.store_scaled_mul(1114, 1112, 1073, 2.0);
            s.store_mul(1115, 1076, 1114);
            s.store_sub_scaled_inputs(1167, 1102, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            let assign17780_ad_e17414: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1166, assign17780_ad_e17414, 1.0, s.ad_value(1167), (-1.0), s.ad_value(1099), 1.0);
        }

        s.b[1181] = (s.v[1166] > 50.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1181]) {
            s.store_scalar(1124, 0.0);
        }

        s.b[1182] = (s.v[1166] < (-50.0));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1181])) && s.b[1182]) {
            s.store_scalar(1124, 1.0);
        }

        if ((s.b[1055] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_div_from_scalar_offset_ad(1124, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            let assign17840_ad_e17502: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1125, assign17840_ad_e17502, 1.0, A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1124), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1114), 1.0);
        }

        s.b[1183] = (s.v[1125] > 50.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1183]) {
            s.store_mul(1126, 1115, 1125);
        }

        s.b[1184] = (s.v[1125] < (-50.0));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1183])) && s.b[1184]) {
            s.store_mul_exp_rhs(1126, 1115, 1125);
        }

        if ((s.b[1055] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_mul_ad_rhs(1126, 1115, A::ln_one_plus_exp(s.ad_value(1125)));
        }

        if s.b[1055] {
            s.store_div(1118, 1087, 1120);
            s.store_mul_ad_rhs(1119, 1086, A::div_scaled_offset_numerator(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0));
            s.store_div_scaled_product_indices(1140, 1119, 1075, 1.0, 1118, 1.0);
            s.store_add_scaled_product(1141, s.ad_value(1140), (-1.0), s.ad_value(1140), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1126), 2.0, s.ad_value(1076), s.ad_value(1140), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(1142, A::mul_sub_from_scalar_rhs(s.ad_value(1141), 1.0, s.ad_value(1124)), 1.0, s.ad_value(1114), s.ad_value(1124), 1.0);
        }

        if s.b[1055] {
            let assign17950_ad_e17677: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::div(s.ad_value(1066), s.ad_value(1142)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1066), s.ad_value(1142)), A::div(s.ad_value(1066), s.ad_value(1142)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1143, 1.0, A::offset(A::pow(assign17950_ad_e17677, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1144, 1066, 1143);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            let assign17970_ad_e17758: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1145, 1.0, A::offset(A::pow(assign17970_ad_e17758, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1146, 1066, 1145);
            s.store_div_scaled_inputs2(1166, s.ad_value(1065), 1.0, s.ad_value(1167), (-1.0), s.ad_value(1099), 1.0);
        }

        s.b[1185] = (s.v[1166] > 50.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1185]) {
            s.store_scalar(1113, 0.0);
        }

        s.b[1186] = (s.v[1166] < (-50.0));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1185])) && s.b[1186]) {
            s.store_scalar(1113, 1.0);
        }

        if ((s.b[1055] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_div_from_scalar_offset_ad(1113, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3(1116, s.ad_value(1165), 1.0, s.ad_value(1146), (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1113), (-(p.p51 * 0.1))), -1.0, s.ad_value(1114), 1.0);
        }

        s.b[1187] = (s.v[1116] > 50.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1187]) {
            s.store_mul(1117, 1115, 1116);
        }

        s.b[1188] = (s.v[1116] < (-50.0));
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1187])) && s.b[1188]) {
            s.store_mul_exp_rhs(1117, 1115, 1116);
        }

        if ((s.b[1055] && (!s.b[1187])) && (!s.b[1188])) {
            s.store_mul_ad_rhs(1117, 1115, A::ln_one_plus_exp(s.ad_value(1116)));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2(1166, s.ad_value(1165), 1.0, s.ad_value(1167), (-1.0), s.ad_value(1099), 1.0);
        }

        s.b[1189] = (s.v[1166] > 50.0);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1189]) {
            s.store_scalar(1147, 0.0);
        }

        s.b[1190] = (s.v[1166] < (-50.0));
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1189])) && s.b[1190]) {
            s.store_scalar(1147, 1.0);
        }

        if ((s.b[1055] && (!s.b[1189])) && (!s.b[1190])) {
            s.store_div_from_scalar_offset_ad(1147, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3(1148, s.ad_value(1065), 1.0, s.ad_value(1144), (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1147), (-(p.p51 * 0.1))), -1.0, s.ad_value(1114), 1.0);
        }

        s.b[1191] = (s.v[1148] > 50.0);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1191]) {
            s.store_mul(1149, 1115, 1148);
        }

        s.b[1192] = (s.v[1148] < (-50.0));
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1191])) && s.b[1192]) {
            s.store_mul_exp_rhs(1149, 1115, 1148);
        }

        if ((s.b[1055] && (!s.b[1191])) && (!s.b[1192])) {
            s.store_mul_ad_rhs(1149, 1115, A::ln_one_plus_exp(s.ad_value(1148)));
        }

        if s.b[1055] {
            s.store_offset_square(1150, 1117, 1e-38);
            s.store_offset_mul(1151, 1150, 1117, 1e-57);
            s.store_offset_square(1152, 1149, 1e-38);
            s.store_offset_mul(1153, 1152, 1149, 1e-57);
            s.store_offset_mul(1154, 1117, 1149, 1e-38);
            s.store_div_scaled_inputs3(1155, s.ad_value(1150), (2.0 / 3.0), s.ad_value(1152), (2.0 / 3.0), s.ad_value(1154), (2.0 / 3.0), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19), 1.0);
            s.store_div_ad(1156, A::add_scaled_inputs_products(s.ad_value(1151), (2.0 * 2.0), s.ad_value(1153), (3.0 * 2.0), s.ad_value(1150), s.ad_value(1149), (4.0 * 2.0), s.ad_value(1152), s.ad_value(1117), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1150), 15.0, s.ad_value(1152), 15.0, s.ad_value(1154), (2.0 * 15.0)));
            s.store_sub(1157, 1155, 1156);
            s.copy_ad(1158, 1156);
            s.store_mul_ad_lhs(1058, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), s.ad_value(1097), s.ad_value(1157)), 1098);
            s.store_mul_ad_lhs(1059, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), s.ad_value(1097), s.ad_value(1158)), 1098);
        }

        s.b[1193] = (s.v[1067] == 1.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1193]) {
            s.store_div_scaled_inputs3(1159, s.ad_value(1068), 1.0, s.ad_value(1102), -1.0, s.ad_value(1099), (-(-(p.p51 * 0.5))), s.ad_value(1114), 1.0);
        }

        s.b[1194] = (s.v[1159] > 50.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if ((s.b[1055] && s.b[1193]) && s.b[1194]) {
            s.copy_ad(1162, 1159);
        }

        s.b[1195] = (s.v[1159] < (-50.0));
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && s.b[1195]) {
            s.store_exp(1162, 1159);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && (!s.b[1195])) {
            s.store_ln_one_plus_exp(1162, 1159);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs(1060, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1078), s.ad_value(1114)), s.ad_value(1162), 1098);
            s.store_div_scaled_inputs3(1160, s.ad_value(1069), 1.0, s.ad_value(1102), -1.0, s.ad_value(1099), (-(-(p.p51 * 0.5))), s.ad_value(1114), 1.0);
        }

        s.b[1196] = (s.v[1160] > 50.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if ((s.b[1055] && s.b[1193]) && s.b[1196]) {
            s.copy_ad(1162, 1160);
        }

        s.b[1197] = (s.v[1160] < (-50.0));
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && s.b[1197]) {
            s.store_exp(1162, 1160);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && (!s.b[1197])) {
            s.store_ln_one_plus_exp(1162, 1160);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs(1061, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1079), s.ad_value(1114)), s.ad_value(1162), 1098);
        }

        if (s.b[1055] && (!s.b[1193])) {
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
        }

        s.b[1198] = (s.v[1070] == 1.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1198]) {
            s.store_div_scaled_inputs3(1161, s.ad_value(1065), 1.0, s.ad_value(1102), -1.0, s.ad_value(1099), (-(-(p.p51 * 0.5))), s.ad_value(1114), 1.0);
        }

        s.b[1199] = (s.v[1161] > 50.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if ((s.b[1055] && s.b[1198]) && s.b[1199]) {
            s.copy_ad(1162, 1161);
        }

        s.b[1200] = (s.v[1161] < (-50.0));
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && s.b[1200]) {
            s.store_exp(1162, 1161);
        }

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && (!s.b[1200])) {
            s.store_ln_one_plus_exp(1162, 1161);
        }

        if (s.b[1055] && s.b[1198]) {
            s.store_mul_ad_product_lhs(1062, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1077), s.ad_value(1114)), s.ad_value(1162), 1098);
        }

        if (s.b[1055] && (!s.b[1198])) {
            s.store_scalar(1062, 0.0);
        }

        if s.b[1055] {
            s.copy_ad(173, 1058);
            s.copy_ad(174, 1059);
            s.copy_ad(175, 1060);
            s.copy_ad(176, 1061);
            s.copy_ad(177, 1062);
        }

        s.b[1201] = (p.p100 == 1.0);
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        s.v[179] = 0.0;

        s.v[180] = 0.0;

        s.v[181] = 0.0;

        s.v[182] = 0.0;

        s.v[183] = 0.0;

        s.b[1202] = (p.p123 > p.p354);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if s.b[1202] {
            s.store_scalar(1205, 0.0);
            s.store_scalar(1206, 0.0);
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
            s.store_scalar(1209, 0.0);
            s.store_scalar(1210, 0.0);
            s.store_scalar(1211, 0.0);
            s.copy_ad(1212, 72);
            s.copy_ad(1213, 73);
            s.store_scalar(1214, p.p129);
            s.copy_ad(1215, 74);
            s.copy_ad(1216, 75);
            s.store_scalar(1217, p.p127);
            s.copy_ad(1218, 111);
            s.store_scalar(1219, s.v[109]);
            s.copy_ad(1220, 113);
            s.store_scalar(1221, p.p0);
            s.store_scalar(1222, p.p123);
            s.copy_ad(1223, 26);
            s.store_scalar(1224, p.p128);
            s.copy_ad(1225, 27);
            s.copy_ad(1226, 28);
            s.store_scalar(1227, p.p124);
            s.store_scalar(1228, p.p138);
            s.store_scalar(1229, p.p137);
            s.store_scalar(1230, 0.0);
            s.store_scalar(1231, p.p139);
            s.store_scalar(1232, p.p143);
            s.store_scalar(1233, p.p134);
            s.store_scalar(1234, p.p135);
            s.store_scalar(1235, p.p136);
            s.store_scalar(1236, p.p142);
            s.store_scalar(1237, p.p141);
            s.store_scalar(1238, p.p140);
            s.store_scalar(1239, p.p39);
            s.store_scalar(1240, p.p47);
            s.store_scalar(1241, p.p45);
            s.store_scalar(1242, p.p42);
            s.store_scalar(1243, p.p2);
            s.store_scalar(1244, p.p6);
            s.store_scalar(1245, 1.0);
            s.store_scalar(1246, 0.0);
            s.store_scalar(1247, 0.0);
            s.store_scalar(1248, 0.0);
            s.store_scalar(1249, 0.0);
            s.store_scalar(1250, 0.0);
            s.store_scalar(1251, 0.0);
            s.store_scalar(1252, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1254, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1256, 0.0);
            s.store_scalar(1257, 0.0);
            s.store_scalar(1259, 0.0);
            s.store_scalar(1260, 0.0);
            s.store_scalar(1261, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_scalar(1262, 0.0);
            s.store_scalar(1263, 0.0);
            s.store_scalar(1264, 0.0);
            s.store_scalar(1265, 0.0);
            s.store_scalar(1266, 0.0);
            s.store_scalar(1267, 0.0);
            s.store_scalar(1268, 0.0);
            s.store_scalar(1269, 0.0);
            s.store_scalar(1270, 0.0);
            s.store_scalar(1271, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1273, 0.0);
            s.store_scalar(1274, 0.0);
            s.store_scalar(1275, 0.0);
            s.store_scalar(1276, 0.0);
            s.store_scalar(1277, 0.0);
            s.store_scalar(1278, 0.0);
            s.store_scalar(1279, 0.0);
            s.store_scalar(1280, 0.0);
            s.store_scalar(1281, 0.0);
            s.store_scalar(1282, 0.0);
            s.store_scalar(1283, 0.0);
            s.store_scalar(1284, 0.0);
            s.store_scalar(1287, 0.0);
            s.store_scalar(1288, 0.0);
            s.store_scalar(1289, 0.0);
            s.store_scalar(1290, 0.0);
            s.store_scalar(1291, 0.0);
            s.store_scalar(1292, 0.0);
            s.store_scalar(1293, 0.0);
            s.store_scalar(1294, 0.0);
            s.store_scalar(1295, 0.0);
            s.store_scalar(1296, 0.0);
            s.store_scalar(1297, 0.0);
            s.store_scalar(1298, 0.0);
            s.store_scalar(1299, 0.0);
            s.store_scalar(1300, 0.0);
            s.store_scalar(1301, 0.0);
            s.store_scalar(1302, 0.0);
            s.store_scalar(1303, 0.0);
            s.store_scalar(1304, 0.0);
            s.store_scalar(1305, 0.0);
            s.store_scalar(1306, 0.0);
            s.store_scalar(1307, 0.0);
            s.store_scalar(1308, 0.0);
            s.store_scalar(1309, 0.0);
            s.store_scalar(1310, 0.0);
            s.store_scalar(1311, 0.0);
            s.store_scalar(1312, 0.0);
            s.store_scalar(1313, 0.0);
            s.store_scalar(1314, 0.0);
        }

        if s.b[1202] {
            s.store_ad_value(1311, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1213), A::tanh_scaled_input(s.ad_value(1213), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1213)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1202] {
            s.store_sub(1312, 1212, 1213);
            s.store_mul(1246, 1232, 1220);
            s.store_add_scaled_product(1248, A::div_scaled_inputs(s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046), 1.0, s.ad_value(1231), s.ad_value(1311), 1.0);
            s.store_add_scaled_product(1249, s.ad_value(1227), 1.0, s.ad_value(1238), A::sub(s.ad_value(1218), s.ad_value(1219)), 1.0);
            s.store_pow_ad(1267, A::div(s.ad_value(1218), s.ad_value(1219)), s.ad_value(1240));
        }

        s.b[1315] = (s.v[1239] != 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1315]) {
            s.store_div_ad_rhs(1250, 1311, A::pow(A::offset(A::pow(A::div(s.ad_value(1311), s.ad_value(1239)), s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.b[1202] && (!s.b[1315])) {
            s.store_scalar(1250, 0.0);
        }

        if s.b[1202] {
            s.store_mul_ad_lhs(1247, A::add_scaled_product(s.ad_value(1229), 1.0, s.ad_value(1250), s.ad_value(1230), (-1.0)), 1311);
            s.store_sub(1210, 1249, 1247);
            s.store_scaled_mul(1252, 1248, 1220, 2.0);
            s.store_mul(1253, 1223, 1252);
            s.store_sub_scaled_inputs(1310, 1210, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            let assign20060_ad_e19059: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1309, assign20060_ad_e19059, 1.0, s.ad_value(1310), (-1.0), s.ad_value(1246), 1.0);
        }

        s.b[1316] = (s.v[1309] > 50.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1316]) {
            s.store_scalar(1268, 0.0);
        }

        s.b[1317] = (s.v[1309] < (-50.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1268, 1.0);
        }

        if ((s.b[1202] && (!s.b[1316])) && (!s.b[1317])) {
            s.store_div_from_scalar_offset_ad(1268, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            let assign20120_ad_e19147: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1269, assign20120_ad_e19147, 1.0, A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1268), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1252), 1.0);
        }

        s.b[1318] = (s.v[1269] > 50.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1318]) {
            s.store_mul(1270, 1253, 1269);
        }

        s.b[1319] = (s.v[1269] < (-50.0));
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1318])) && s.b[1319]) {
            s.store_mul_exp_rhs(1270, 1253, 1269);
        }

        if ((s.b[1202] && (!s.b[1318])) && (!s.b[1319])) {
            s.store_mul_ad_rhs(1270, 1253, A::ln_one_plus_exp(s.ad_value(1269)));
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1256, 1234, A::mul_offset_rhs(s.ad_value(1267), A::div_scaled_product(s.ad_value(1236), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0));
        }

        if s.b[1202] {
            let assign20190_ad_e19248: A = A::div_scaled_product3(s.ad_value(1233), A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1242), s.ad_value(1311), 1.0, s.ad_value(1222), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1237), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0), 1.0);
            s.store_ad_value(1257, assign20190_ad_e19248);
        }

        if s.b[1202] {
            s.store_div_scaled_product_indices(1274, 1257, 1222, 1.0, 1256, 1.0);
            s.store_add_scaled_product(1275, s.ad_value(1274), (-1.0), s.ad_value(1274), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1270), 2.0, s.ad_value(1223), s.ad_value(1274), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(1276, A::mul_sub_from_scalar_rhs(s.ad_value(1274), 1.0, s.ad_value(1268)), 1.0, s.ad_value(1252), s.ad_value(1268), 1.0);
            s.store_add_scaled_product(1211, A::mul_sub_from_scalar_rhs(s.ad_value(1275), 1.0, s.ad_value(1268)), 1.0, s.ad_value(1252), s.ad_value(1268), 1.0);
        }

        if s.b[1202] {
            let assign20250_ad_e19376: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::div(s.ad_value(1213), s.ad_value(1211)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1213), s.ad_value(1211)), A::div(s.ad_value(1213), s.ad_value(1211)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1277, 1.0, A::offset(A::pow(assign20250_ad_e19376, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1278, 1213, 1277);
        }

        if s.b[1202] {
            let assign20270_ad_e19457: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1279, 1.0, A::offset(A::pow(assign20270_ad_e19457, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1280, 1213, 1279);
            s.store_div_scaled_inputs2(1309, s.ad_value(1212), 1.0, s.ad_value(1310), (-1.0), s.ad_value(1246), 1.0);
        }

        s.b[1320] = (s.v[1309] > 50.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1320]) {
            s.store_scalar(1251, 0.0);
        }

        s.b[1321] = (s.v[1309] < (-50.0));
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1320])) && s.b[1321]) {
            s.store_scalar(1251, 1.0);
        }

        if ((s.b[1202] && (!s.b[1320])) && (!s.b[1321])) {
            s.store_div_from_scalar_offset_ad(1251, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3(1254, s.ad_value(1312), 1.0, s.ad_value(1280), (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1251), (-(p.p51 * 0.1))), -1.0, s.ad_value(1252), 1.0);
        }

        s.b[1322] = (s.v[1254] > 50.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1322]) {
            s.store_mul(1255, 1253, 1254);
        }

        s.b[1323] = (s.v[1254] < (-50.0));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1322])) && s.b[1323]) {
            s.store_mul_exp_rhs(1255, 1253, 1254);
        }

        if ((s.b[1202] && (!s.b[1322])) && (!s.b[1323])) {
            s.store_mul_ad_rhs(1255, 1253, A::ln_one_plus_exp(s.ad_value(1254)));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2(1309, s.ad_value(1312), 1.0, s.ad_value(1310), (-1.0), s.ad_value(1246), 1.0);
        }

        s.b[1324] = (s.v[1309] > 50.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1324]) {
            s.store_scalar(1281, 0.0);
        }

        s.b[1325] = (s.v[1309] < (-50.0));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(1281, 1.0);
        }

        if ((s.b[1202] && (!s.b[1324])) && (!s.b[1325])) {
            s.store_div_from_scalar_offset_ad(1281, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3(1282, s.ad_value(1212), 1.0, s.ad_value(1278), (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1281), (-(p.p51 * 0.1))), -1.0, s.ad_value(1252), 1.0);
        }

        s.b[1326] = (s.v[1282] > 50.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1326]) {
            s.store_mul(1283, 1253, 1282);
        }

        s.b[1327] = (s.v[1282] < (-50.0));
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1326])) && s.b[1327]) {
            s.store_mul_exp_rhs(1283, 1253, 1282);
        }

        if ((s.b[1202] && (!s.b[1326])) && (!s.b[1327])) {
            s.store_mul_ad_rhs(1283, 1253, A::ln_one_plus_exp(s.ad_value(1282)));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2(1284, s.ad_value(1255), 1.0, s.ad_value(1283), (-1.0), s.ad_value(1223), 1.0);
            s.store_div(1310, 1284, 1276);
            s.store_div_scaled_inputs(1259, s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046);
            s.store_scaled_mul(1261, 1259, 1220, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_mul(1262, 1223, 1261);
            s.store_sub_scaled_inputs(1314, 1249, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            let assign20620_ad_e19838: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1313, assign20620_ad_e19838, 1.0, s.ad_value(1314), (-1.0), s.ad_value(1246), 1.0);
        }

        s.b[1328] = (s.v[1313] > 50.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1328]) {
            s.store_scalar(1271, 0.0);
        }

        s.b[1329] = (s.v[1313] < (-50.0));
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1328])) && s.b[1329]) {
            s.store_scalar(1271, 1.0);
        }

        if ((s.b[1202] && (!s.b[1328])) && (!s.b[1329])) {
            s.store_div_from_scalar_offset_ad(1271, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            let assign20680_ad_e19926: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1272, assign20680_ad_e19926, 1.0, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1271), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1261), 1.0);
        }

        s.b[1330] = (s.v[1272] > 50.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1330]) {
            s.store_mul(1273, 1262, 1272);
        }

        s.b[1331] = (s.v[1272] < (-50.0));
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1330])) && s.b[1331]) {
            s.store_mul_exp_rhs(1273, 1262, 1272);
        }

        if ((s.b[1202] && (!s.b[1330])) && (!s.b[1331])) {
            s.store_mul_ad_rhs(1273, 1262, A::ln_one_plus_exp(s.ad_value(1272)));
        }

        if s.b[1202] {
            s.store_div(1265, 1234, 1267);
            s.store_mul_ad_rhs(1266, 1233, A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0));
            s.store_div_scaled_product_indices(1287, 1266, 1222, 1.0, 1265, 1.0);
            s.store_add_scaled_product(1288, s.ad_value(1287), (-1.0), s.ad_value(1287), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1273), 2.0, s.ad_value(1223), s.ad_value(1287), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(1289, A::mul_sub_from_scalar_rhs(s.ad_value(1288), 1.0, s.ad_value(1271)), 1.0, s.ad_value(1261), s.ad_value(1271), 1.0);
        }

        if s.b[1202] {
            let assign20790_ad_e20101: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::div(s.ad_value(1213), s.ad_value(1289)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1213), s.ad_value(1289)), A::div(s.ad_value(1213), s.ad_value(1289)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1290, 1.0, A::offset(A::pow(assign20790_ad_e20101, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1291, 1213, 1290);
        }

        if s.b[1202] {
            let assign20810_ad_e20182: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1292, 1.0, A::offset(A::pow(assign20810_ad_e20182, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1293, 1213, 1292);
            s.store_div_scaled_inputs2(1313, s.ad_value(1212), 1.0, s.ad_value(1314), (-1.0), s.ad_value(1246), 1.0);
        }

        s.b[1332] = (s.v[1313] > 50.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1332]) {
            s.store_scalar(1260, 0.0);
        }

        s.b[1333] = (s.v[1313] < (-50.0));
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1332])) && s.b[1333]) {
            s.store_scalar(1260, 1.0);
        }

        if ((s.b[1202] && (!s.b[1332])) && (!s.b[1333])) {
            s.store_div_from_scalar_offset_ad(1260, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3(1263, s.ad_value(1312), 1.0, s.ad_value(1293), (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1260), (-(p.p51 * 0.1))), -1.0, s.ad_value(1261), 1.0);
        }

        s.b[1334] = (s.v[1263] > 50.0);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1334]) {
            s.store_mul(1264, 1262, 1263);
        }

        s.b[1335] = (s.v[1263] < (-50.0));
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1334])) && s.b[1335]) {
            s.store_mul_exp_rhs(1264, 1262, 1263);
        }

        if ((s.b[1202] && (!s.b[1334])) && (!s.b[1335])) {
            s.store_mul_ad_rhs(1264, 1262, A::ln_one_plus_exp(s.ad_value(1263)));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2(1313, s.ad_value(1312), 1.0, s.ad_value(1314), (-1.0), s.ad_value(1246), 1.0);
        }

        s.b[1336] = (s.v[1313] > 50.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1336]) {
            s.store_scalar(1294, 0.0);
        }

        s.b[1337] = (s.v[1313] < (-50.0));
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1336])) && s.b[1337]) {
            s.store_scalar(1294, 1.0);
        }

        if ((s.b[1202] && (!s.b[1336])) && (!s.b[1337])) {
            s.store_div_from_scalar_offset_ad(1294, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3(1295, s.ad_value(1212), 1.0, s.ad_value(1291), (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1294), (-(p.p51 * 0.1))), -1.0, s.ad_value(1261), 1.0);
        }

        s.b[1338] = (s.v[1295] > 50.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1338]) {
            s.store_mul(1296, 1262, 1295);
        }

        s.b[1339] = (s.v[1295] < (-50.0));
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if ((s.b[1202] && (!s.b[1338])) && s.b[1339]) {
            s.store_mul_exp_rhs(1296, 1262, 1295);
        }

        if ((s.b[1202] && (!s.b[1338])) && (!s.b[1339])) {
            s.store_mul_ad_rhs(1296, 1262, A::ln_one_plus_exp(s.ad_value(1295)));
        }

        if s.b[1202] {
            s.store_offset_square(1297, 1264, 1e-38);
            s.store_offset_mul(1298, 1297, 1264, 1e-57);
            s.store_offset_square(1299, 1296, 1e-38);
            s.store_offset_mul(1300, 1299, 1296, 1e-57);
            s.store_offset_mul(1301, 1264, 1296, 1e-38);
            s.store_div_scaled_inputs3(1302, s.ad_value(1297), (2.0 / 3.0), s.ad_value(1299), (2.0 / 3.0), s.ad_value(1301), (2.0 / 3.0), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19), 1.0);
            s.store_div_ad(1303, A::add_scaled_inputs_products(s.ad_value(1298), (2.0 * 2.0), s.ad_value(1300), (3.0 * 2.0), s.ad_value(1297), s.ad_value(1296), (4.0 * 2.0), s.ad_value(1299), s.ad_value(1264), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1297), 15.0, s.ad_value(1299), 15.0, s.ad_value(1301), (2.0 * 15.0)));
            s.store_sub(1304, 1302, 1303);
            s.copy_ad(1305, 1303);
            s.store_mul_ad_lhs(1205, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), s.ad_value(1244), s.ad_value(1304)), 1245);
            s.store_mul_ad_lhs(1206, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), s.ad_value(1244), s.ad_value(1305)), 1245);
        }

        s.b[1340] = (s.v[1214] == 1.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1340]) {
            s.store_div_scaled_inputs3(1306, s.ad_value(1215), 1.0, s.ad_value(1249), -1.0, s.ad_value(1246), (-(-(p.p51 * 0.5))), s.ad_value(1261), 1.0);
        }

        s.b[1341] = (s.v[1306] > 50.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1340]) && s.b[1341]) {
            s.copy_ad(1309, 1306);
        }

        s.b[1342] = (s.v[1306] < (-50.0));
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && s.b[1342]) {
            s.store_exp(1309, 1306);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && (!s.b[1342])) {
            s.store_ln_one_plus_exp(1309, 1306);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1225), s.ad_value(1261)), s.ad_value(1309), 1245);
            s.store_div_scaled_inputs3(1307, s.ad_value(1216), 1.0, s.ad_value(1249), -1.0, s.ad_value(1246), (-(-(p.p51 * 0.5))), s.ad_value(1261), 1.0);
        }

        s.b[1343] = (s.v[1307] > 50.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1340]) && s.b[1343]) {
            s.copy_ad(1309, 1307);
        }

        s.b[1344] = (s.v[1307] < (-50.0));
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && s.b[1344]) {
            s.store_exp(1309, 1307);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && (!s.b[1344])) {
            s.store_ln_one_plus_exp(1309, 1307);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs(1208, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1226), s.ad_value(1261)), s.ad_value(1309), 1245);
        }

        if (s.b[1202] && (!s.b[1340])) {
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
        }

        s.b[1345] = (s.v[1217] == 1.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1345]) {
            s.store_div_scaled_inputs3(1308, s.ad_value(1212), 1.0, s.ad_value(1249), -1.0, s.ad_value(1246), (-(-(p.p51 * 0.5))), s.ad_value(1261), 1.0);
        }

        s.b[1346] = (s.v[1308] > 50.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1345]) && s.b[1346]) {
            s.copy_ad(1309, 1308);
        }

        s.b[1347] = (s.v[1308] < (-50.0));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && s.b[1347]) {
            s.store_exp(1309, 1308);
        }

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_ln_one_plus_exp(1309, 1308);
        }

        if (s.b[1202] && s.b[1345]) {
            s.store_mul_ad_product_lhs(1209, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1224), s.ad_value(1261)), s.ad_value(1309), 1245);
        }

        if (s.b[1202] && (!s.b[1345])) {
            s.store_scalar(1209, 0.0);
        }

        if s.b[1202] {
            s.copy_ad(179, 1205);
            s.copy_ad(180, 1206);
            s.copy_ad(181, 1207);
            s.copy_ad(182, 1208);
            s.copy_ad(183, 1209);
        }

        s.b[1348] = (p.p122 == 1.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[189] = 0.0;

        s.b[1349] = (p.p145 > p.p354);
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if s.b[1349] {
            s.store_scalar(1352, 0.0);
            s.store_scalar(1353, 0.0);
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1357, 0.0);
            s.store_scalar(1358, 0.0);
            s.copy_ad(1359, 78);
            s.copy_ad(1360, 79);
            s.store_scalar(1361, p.p151);
            s.copy_ad(1362, 80);
            s.copy_ad(1363, 81);
            s.store_scalar(1364, p.p149);
            s.copy_ad(1365, 111);
            s.store_scalar(1366, s.v[109]);
            s.copy_ad(1367, 113);
            s.store_scalar(1368, p.p0);
            s.store_scalar(1369, p.p145);
            s.copy_ad(1370, 29);
            s.store_scalar(1371, p.p150);
            s.copy_ad(1372, 30);
            s.copy_ad(1373, 31);
            s.store_scalar(1374, p.p146);
            s.store_scalar(1375, p.p160);
            s.store_scalar(1376, p.p159);
            s.store_scalar(1377, 0.0);
            s.store_scalar(1378, p.p161);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
            s.store_scalar(1379, p.p165);
            s.store_scalar(1380, p.p156);
            s.store_scalar(1381, p.p157);
            s.store_scalar(1382, p.p158);
            s.store_scalar(1383, p.p164);
            s.store_scalar(1384, p.p163);
            s.store_scalar(1385, p.p162);
            s.store_scalar(1386, p.p39);
            s.store_scalar(1387, p.p47);
            s.store_scalar(1388, p.p45);
            s.store_scalar(1389, p.p42);
            s.store_scalar(1390, p.p2);
            s.store_scalar(1391, p.p6);
            s.store_scalar(1392, 1.0);
            s.store_scalar(1393, 0.0);
            s.store_scalar(1394, 0.0);
            s.store_scalar(1395, 0.0);
            s.store_scalar(1396, 0.0);
            s.store_scalar(1397, 0.0);
            s.store_scalar(1398, 0.0);
            s.store_scalar(1399, 0.0);
            s.store_scalar(1400, 0.0);
            s.store_scalar(1401, 0.0);
            s.store_scalar(1402, 0.0);
            s.store_scalar(1403, 0.0);
            s.store_scalar(1404, 0.0);
            s.store_scalar(1406, 0.0);
            s.store_scalar(1407, 0.0);
            s.store_scalar(1408, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1410, 0.0);
            s.store_scalar(1411, 0.0);
            s.store_scalar(1412, 0.0);
            s.store_scalar(1413, 0.0);
            s.store_scalar(1414, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
            s.store_scalar(1417, 0.0);
            s.store_scalar(1418, 0.0);
            s.store_scalar(1419, 0.0);
            s.store_scalar(1420, 0.0);
            s.store_scalar(1421, 0.0);
            s.store_scalar(1422, 0.0);
            s.store_scalar(1423, 0.0);
            s.store_scalar(1424, 0.0);
            s.store_scalar(1425, 0.0);
            s.store_scalar(1426, 0.0);
            s.store_scalar(1427, 0.0);
            s.store_scalar(1428, 0.0);
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
            s.store_scalar(1431, 0.0);
            s.store_scalar(1434, 0.0);
            s.store_scalar(1435, 0.0);
            s.store_scalar(1436, 0.0);
            s.store_scalar(1437, 0.0);
            s.store_scalar(1438, 0.0);
            s.store_scalar(1439, 0.0);
            s.store_scalar(1440, 0.0);
            s.store_scalar(1441, 0.0);
            s.store_scalar(1442, 0.0);
            s.store_scalar(1443, 0.0);
            s.store_scalar(1444, 0.0);
            s.store_scalar(1445, 0.0);
            s.store_scalar(1446, 0.0);
            s.store_scalar(1447, 0.0);
            s.store_scalar(1448, 0.0);
            s.store_scalar(1449, 0.0);
            s.store_scalar(1450, 0.0);
            s.store_scalar(1451, 0.0);
            s.store_scalar(1452, 0.0);
            s.store_scalar(1453, 0.0);
            s.store_scalar(1454, 0.0);
            s.store_scalar(1455, 0.0);
            s.store_scalar(1456, 0.0);
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1459, 0.0);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1461, 0.0);
        }

        if s.b[1349] {
            s.store_ad_value(1458, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1360), A::tanh_scaled_input(s.ad_value(1360), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1360)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1349] {
            s.store_sub(1459, 1359, 1360);
            s.store_mul(1393, 1379, 1367);
            s.store_add_scaled_product(1395, A::div_scaled_inputs(s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046), 1.0, s.ad_value(1378), s.ad_value(1458), 1.0);
            s.store_add_scaled_product(1396, s.ad_value(1374), 1.0, s.ad_value(1385), A::sub(s.ad_value(1365), s.ad_value(1366)), 1.0);
            s.store_pow_ad(1414, A::div(s.ad_value(1365), s.ad_value(1366)), s.ad_value(1387));
        }

        s.b[1462] = (s.v[1386] != 0.0);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1462]) {
            s.store_div_ad_rhs(1397, 1458, A::pow(A::offset(A::pow(A::div(s.ad_value(1458), s.ad_value(1386)), s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.b[1349] && (!s.b[1462])) {
            s.store_scalar(1397, 0.0);
        }

        if s.b[1349] {
            s.store_mul_ad_lhs(1394, A::add_scaled_product(s.ad_value(1376), 1.0, s.ad_value(1397), s.ad_value(1377), (-1.0)), 1458);
            s.store_sub(1357, 1396, 1394);
            s.store_scaled_mul(1399, 1395, 1367, 2.0);
            s.store_mul(1400, 1370, 1399);
            s.store_sub_scaled_inputs(1457, 1357, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            let assign22900_ad_e21483: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1456, assign22900_ad_e21483, 1.0, s.ad_value(1457), (-1.0), s.ad_value(1393), 1.0);
        }

        s.b[1463] = (s.v[1456] > 50.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1463]) {
            s.store_scalar(1415, 0.0);
        }

        s.b[1464] = (s.v[1456] < (-50.0));
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1463])) && s.b[1464]) {
            s.store_scalar(1415, 1.0);
        }

        if ((s.b[1349] && (!s.b[1463])) && (!s.b[1464])) {
            s.store_div_from_scalar_offset_ad(1415, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            let assign22960_ad_e21571: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1416, assign22960_ad_e21571, 1.0, A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1415), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1399), 1.0);
        }

        s.b[1465] = (s.v[1416] > 50.0);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1465]) {
            s.store_mul(1417, 1400, 1416);
        }

        s.b[1466] = (s.v[1416] < (-50.0));
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1465])) && s.b[1466]) {
            s.store_mul_exp_rhs(1417, 1400, 1416);
        }

        if ((s.b[1349] && (!s.b[1465])) && (!s.b[1466])) {
            s.store_mul_ad_rhs(1417, 1400, A::ln_one_plus_exp(s.ad_value(1416)));
        }

        if s.b[1349] {
            s.store_div_ad_rhs(1403, 1381, A::mul_offset_rhs(s.ad_value(1414), A::div_scaled_product(s.ad_value(1383), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0));
        }

        if s.b[1349] {
            let assign23030_ad_e21672: A = A::div_scaled_product3(s.ad_value(1380), A::div_scaled_offset_numerator(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1389), s.ad_value(1458), 1.0, s.ad_value(1369), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1384), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0), 1.0);
            s.store_ad_value(1404, assign23030_ad_e21672);
        }

        if s.b[1349] {
            s.store_div_scaled_product_indices(1421, 1404, 1369, 1.0, 1403, 1.0);
            s.store_add_scaled_product(1422, s.ad_value(1421), (-1.0), s.ad_value(1421), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1417), 2.0, s.ad_value(1370), s.ad_value(1421), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(1423, A::mul_sub_from_scalar_rhs(s.ad_value(1421), 1.0, s.ad_value(1415)), 1.0, s.ad_value(1399), s.ad_value(1415), 1.0);
            s.store_add_scaled_product(1358, A::mul_sub_from_scalar_rhs(s.ad_value(1422), 1.0, s.ad_value(1415)), 1.0, s.ad_value(1399), s.ad_value(1415), 1.0);
        }

        if s.b[1349] {
            let assign23090_ad_e21800: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::div(s.ad_value(1360), s.ad_value(1358)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1360), s.ad_value(1358)), A::div(s.ad_value(1360), s.ad_value(1358)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1424, 1.0, A::offset(A::pow(assign23090_ad_e21800, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1425, 1360, 1424);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
            let assign23110_ad_e21881: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1426, 1.0, A::offset(A::pow(assign23110_ad_e21881, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1427, 1360, 1426);
            s.store_div_scaled_inputs2(1456, s.ad_value(1359), 1.0, s.ad_value(1457), (-1.0), s.ad_value(1393), 1.0);
        }

        s.b[1467] = (s.v[1456] > 50.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1467]) {
            s.store_scalar(1398, 0.0);
        }

        s.b[1468] = (s.v[1456] < (-50.0));
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1467])) && s.b[1468]) {
            s.store_scalar(1398, 1.0);
        }

        if ((s.b[1349] && (!s.b[1467])) && (!s.b[1468])) {
            s.store_div_from_scalar_offset_ad(1398, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3(1401, s.ad_value(1459), 1.0, s.ad_value(1427), (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1398), (-(p.p51 * 0.1))), -1.0, s.ad_value(1399), 1.0);
        }

        s.b[1469] = (s.v[1401] > 50.0);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1469]) {
            s.store_mul(1402, 1400, 1401);
        }

        s.b[1470] = (s.v[1401] < (-50.0));
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1469])) && s.b[1470]) {
            s.store_mul_exp_rhs(1402, 1400, 1401);
        }

        if ((s.b[1349] && (!s.b[1469])) && (!s.b[1470])) {
            s.store_mul_ad_rhs(1402, 1400, A::ln_one_plus_exp(s.ad_value(1401)));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2(1456, s.ad_value(1459), 1.0, s.ad_value(1457), (-1.0), s.ad_value(1393), 1.0);
        }

        s.b[1471] = (s.v[1456] > 50.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1471]) {
            s.store_scalar(1428, 0.0);
        }

        s.b[1472] = (s.v[1456] < (-50.0));
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1471])) && s.b[1472]) {
            s.store_scalar(1428, 1.0);
        }

        if ((s.b[1349] && (!s.b[1471])) && (!s.b[1472])) {
            s.store_div_from_scalar_offset_ad(1428, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3(1429, s.ad_value(1359), 1.0, s.ad_value(1425), (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1428), (-(p.p51 * 0.1))), -1.0, s.ad_value(1399), 1.0);
        }

        s.b[1473] = (s.v[1429] > 50.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1473]) {
            s.store_mul(1430, 1400, 1429);
        }

        s.b[1474] = (s.v[1429] < (-50.0));
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1473])) && s.b[1474]) {
            s.store_mul_exp_rhs(1430, 1400, 1429);
        }

        if ((s.b[1349] && (!s.b[1473])) && (!s.b[1474])) {
            s.store_mul_ad_rhs(1430, 1400, A::ln_one_plus_exp(s.ad_value(1429)));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2(1431, s.ad_value(1402), 1.0, s.ad_value(1430), (-1.0), s.ad_value(1370), 1.0);
            s.store_div(1457, 1431, 1423);
            s.store_div_scaled_inputs(1406, s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046);
            s.store_scaled_mul(1408, 1406, 1367, 2.0);
            s.store_mul(1409, 1370, 1408);
            s.store_sub_scaled_inputs(1461, 1396, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            let assign23460_ad_e22262: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1460, assign23460_ad_e22262, 1.0, s.ad_value(1461), (-1.0), s.ad_value(1393), 1.0);
        }

        s.b[1475] = (s.v[1460] > 50.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1475]) {
            s.store_scalar(1418, 0.0);
        }

        s.b[1476] = (s.v[1460] < (-50.0));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1475])) && s.b[1476]) {
            s.store_scalar(1418, 1.0);
        }

        if ((s.b[1349] && (!s.b[1475])) && (!s.b[1476])) {
            s.store_div_from_scalar_offset_ad(1418, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            let assign23520_ad_e22350: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(1419, assign23520_ad_e22350, 1.0, A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1418), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1408), 1.0);
        }

        s.b[1477] = (s.v[1419] > 50.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1477]) {
            s.store_mul(1420, 1409, 1419);
        }

        s.b[1478] = (s.v[1419] < (-50.0));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1477])) && s.b[1478]) {
            s.store_mul_exp_rhs(1420, 1409, 1419);
        }

        if ((s.b[1349] && (!s.b[1477])) && (!s.b[1478])) {
            s.store_mul_ad_rhs(1420, 1409, A::ln_one_plus_exp(s.ad_value(1419)));
        }

        if s.b[1349] {
            s.store_div(1412, 1381, 1414);
            s.store_mul_ad_rhs(1413, 1380, A::div_scaled_offset_numerator(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0));
            s.store_div_scaled_product_indices(1434, 1413, 1369, 1.0, 1412, 1.0);
            s.store_add_scaled_product(1435, s.ad_value(1434), (-1.0), s.ad_value(1434), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1420), 2.0, s.ad_value(1370), s.ad_value(1434), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product(1436, A::mul_sub_from_scalar_rhs(s.ad_value(1435), 1.0, s.ad_value(1418)), 1.0, s.ad_value(1408), s.ad_value(1418), 1.0);
        }

        if s.b[1349] {
            let assign23630_ad_e22525: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::div(s.ad_value(1360), s.ad_value(1436)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1360), s.ad_value(1436)), A::div(s.ad_value(1360), s.ad_value(1436)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1437, 1.0, A::offset(A::pow(assign23630_ad_e22525, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1438, 1360, 1437);
        }

        if s.b[1349] {
            let assign23650_ad_e22606: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1439, 1.0, A::offset(A::pow(assign23650_ad_e22606, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1440, 1360, 1439);
            s.store_div_scaled_inputs2(1460, s.ad_value(1359), 1.0, s.ad_value(1461), (-1.0), s.ad_value(1393), 1.0);
        }

        s.b[1479] = (s.v[1460] > 50.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1479]) {
            s.store_scalar(1407, 0.0);
        }

        s.b[1480] = (s.v[1460] < (-50.0));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1479])) && s.b[1480]) {
            s.store_scalar(1407, 1.0);
        }

        if ((s.b[1349] && (!s.b[1479])) && (!s.b[1480])) {
            s.store_div_from_scalar_offset_ad(1407, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3(1410, s.ad_value(1459), 1.0, s.ad_value(1440), (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1407), (-(p.p51 * 0.1))), -1.0, s.ad_value(1408), 1.0);
        }

        s.b[1481] = (s.v[1410] > 50.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1481]) {
            s.store_mul(1411, 1409, 1410);
        }

        s.b[1482] = (s.v[1410] < (-50.0));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1481])) && s.b[1482]) {
            s.store_mul_exp_rhs(1411, 1409, 1410);
        }

        if ((s.b[1349] && (!s.b[1481])) && (!s.b[1482])) {
            s.store_mul_ad_rhs(1411, 1409, A::ln_one_plus_exp(s.ad_value(1410)));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2(1460, s.ad_value(1459), 1.0, s.ad_value(1461), (-1.0), s.ad_value(1393), 1.0);
        }

        s.b[1483] = (s.v[1460] > 50.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1483]) {
            s.store_scalar(1441, 0.0);
        }

        s.b[1484] = (s.v[1460] < (-50.0));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1483])) && s.b[1484]) {
            s.store_scalar(1441, 1.0);
        }

        if ((s.b[1349] && (!s.b[1483])) && (!s.b[1484])) {
            s.store_div_from_scalar_offset_ad(1441, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3(1442, s.ad_value(1359), 1.0, s.ad_value(1438), (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1441), (-(p.p51 * 0.1))), -1.0, s.ad_value(1408), 1.0);
        }

        s.b[1485] = (s.v[1442] > 50.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1485]) {
            s.store_mul(1443, 1409, 1442);
        }

        s.b[1486] = (s.v[1442] < (-50.0));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1485])) && s.b[1486]) {
            s.store_mul_exp_rhs(1443, 1409, 1442);
        }

        if ((s.b[1349] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_mul_ad_rhs(1443, 1409, A::ln_one_plus_exp(s.ad_value(1442)));
        }

        if s.b[1349] {
            s.store_offset_square(1444, 1411, 1e-38);
            s.store_offset_mul(1445, 1444, 1411, 1e-57);
            s.store_offset_square(1446, 1443, 1e-38);
            s.store_offset_mul(1447, 1446, 1443, 1e-57);
            s.store_offset_mul(1448, 1411, 1443, 1e-38);
            s.store_div_scaled_inputs3(1449, s.ad_value(1444), (2.0 / 3.0), s.ad_value(1446), (2.0 / 3.0), s.ad_value(1448), (2.0 / 3.0), A::offset(A::add(s.ad_value(1411), s.ad_value(1443)), 2e-19), 1.0);
            s.store_div_ad(1450, A::add_scaled_inputs_products(s.ad_value(1445), (2.0 * 2.0), s.ad_value(1447), (3.0 * 2.0), s.ad_value(1444), s.ad_value(1443), (4.0 * 2.0), s.ad_value(1446), s.ad_value(1411), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1444), 15.0, s.ad_value(1446), 15.0, s.ad_value(1448), (2.0 * 15.0)));
            s.store_sub(1451, 1449, 1450);
            s.copy_ad(1452, 1450);
            s.store_mul_ad_lhs(1352, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), s.ad_value(1391), s.ad_value(1451)), 1392);
            s.store_mul_ad_lhs(1353, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), s.ad_value(1391), s.ad_value(1452)), 1392);
        }

        s.b[1487] = (s.v[1361] == 1.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1487]) {
            s.store_div_scaled_inputs3(1453, s.ad_value(1362), 1.0, s.ad_value(1396), -1.0, s.ad_value(1393), (-(-(p.p51 * 0.5))), s.ad_value(1408), 1.0);
        }

        s.b[1488] = (s.v[1453] > 50.0);
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if ((s.b[1349] && s.b[1487]) && s.b[1488]) {
            s.copy_ad(1456, 1453);
        }

        s.b[1489] = (s.v[1453] < (-50.0));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {
            s.store_exp(1456, 1453);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_ln_one_plus_exp(1456, 1453);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs(1354, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1372), s.ad_value(1408)), s.ad_value(1456), 1392);
            s.store_div_scaled_inputs3(1454, s.ad_value(1363), 1.0, s.ad_value(1396), -1.0, s.ad_value(1393), (-(-(p.p51 * 0.5))), s.ad_value(1408), 1.0);
        }

        s.b[1490] = (s.v[1454] > 50.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if ((s.b[1349] && s.b[1487]) && s.b[1490]) {
            s.copy_ad(1456, 1454);
        }

        s.b[1491] = (s.v[1454] < (-50.0));
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {
            s.store_exp(1456, 1454);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && (!s.b[1491])) {
            s.store_ln_one_plus_exp(1456, 1454);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs(1355, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1373), s.ad_value(1408)), s.ad_value(1456), 1392);
        }

        if (s.b[1349] && (!s.b[1487])) {
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
        }

        s.b[1492] = (s.v[1364] == 1.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1492]) {
            s.store_div_scaled_inputs3(1455, s.ad_value(1359), 1.0, s.ad_value(1396), -1.0, s.ad_value(1393), (-(-(p.p51 * 0.5))), s.ad_value(1408), 1.0);
        }

        s.b[1493] = (s.v[1455] > 50.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if ((s.b[1349] && s.b[1492]) && s.b[1493]) {
            s.copy_ad(1456, 1455);
        }

        s.b[1494] = (s.v[1455] < (-50.0));
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && s.b[1494]) {
            s.store_exp(1456, 1455);
        }

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && (!s.b[1494])) {
            s.store_ln_one_plus_exp(1456, 1455);
        }

        if (s.b[1349] && s.b[1492]) {
            s.store_mul_ad_product_lhs(1356, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1371), s.ad_value(1408)), s.ad_value(1456), 1392);
        }

        if (s.b[1349] && (!s.b[1492])) {
            s.store_scalar(1356, 0.0);
        }

        if s.b[1349] {
            s.copy_ad(185, 1352);
            s.copy_ad(186, 1353);
            s.copy_ad(187, 1354);
            s.copy_ad(188, 1355);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
            s.copy_ad(189, 1356);
        }

        s.b[1495] = (p.p144 == 1.0);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        s.v[1788] = 0.0;

        s.v[1789] = 0.0;

        s.v[1790] = 0.0;

        s.v[1791] = 0.0;

        s.v[1795] = 0.0;

        s.v[1796] = 0.0;

        s.copy_ad(1797, 45);

        s.copy_ad(1798, 44);

        s.v[1799] = 0.0;

        s.v[1800] = 0.0;

        s.v[1801] = 0.0;

        s.v[1802] = 0.0;

        s.copy_ad(1803, 111);

        s.v[1804] = s.v[109];

        s.copy_ad(1805, 113);

        s.v[1806] = p.p0;

        s.v[1807] = p.p1;

        s.copy_ad(1808, 19);

        s.v[1812] = p.p35;

        s.v[1813] = p.p36;

        s.v[1814] = p.p37;

        s.v[1815] = p.p38;

        s.v[1816] = p.p40;

        s.v[1817] = p.p41;

        s.v[1818] = p.p32;

        s.v[1819] = p.p33;

        s.v[1820] = p.p34;

        s.v[1821] = p.p44;

        s.v[1822] = p.p43;

        s.v[1823] = p.p46;

        s.v[1824] = p.p39;

        s.v[1825] = p.p47;

        s.v[1826] = p.p45;

        s.v[1827] = p.p42;

        s.v[1828] = p.p2;

        s.v[1829] = p.p6;

        s.copy_ad(1830, 230);

        s.v[1831] = 0.0;

        s.v[1832] = 0.0;

        s.v[1833] = 0.0;

        s.v[1834] = 0.0;

        s.v[1835] = 0.0;

        s.v[1836] = 0.0;

        s.v[1837] = 0.0;

        s.v[1838] = 0.0;

        s.v[1839] = 0.0;

        s.v[1840] = 0.0;

        s.v[1841] = 0.0;

        s.v[1842] = 0.0;

        s.v[1843] = 0.0;

        s.v[1844] = 0.0;

        s.v[1845] = 0.0;

        s.v[1846] = 0.0;

        s.v[1847] = 0.0;

        s.v[1848] = 0.0;

        s.v[1849] = 0.0;

        s.v[1850] = 0.0;

        s.v[1851] = 0.0;

        s.v[1852] = 0.0;

        s.v[1853] = 0.0;

        s.v[1854] = 0.0;

        s.v[1855] = 0.0;

        s.v[1856] = 0.0;

        s.v[1857] = 0.0;

        s.v[1858] = 0.0;

        s.v[1859] = 0.0;

        s.v[1860] = 0.0;

        s.v[1861] = 0.0;

        s.v[1862] = 0.0;

        s.v[1863] = 0.0;

        s.v[1864] = 0.0;

        s.v[1865] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 0.0;

        s.v[1868] = 0.0;

        s.v[1869] = 0.0;

        s.v[1870] = 0.0;

        s.v[1871] = 0.0;

        s.v[1872] = 0.0;

        s.v[1873] = 0.0;

        s.v[1874] = 0.0;

        s.v[1875] = 0.0;

        s.v[1876] = 0.0;

        s.v[1877] = 0.0;

        s.v[1878] = 0.0;

        s.v[1879] = 0.0;

        s.v[1880] = 0.0;

        s.v[1881] = 0.0;

        s.v[1882] = 0.0;

        s.v[1883] = 0.0;

        s.v[1884] = 0.0;

        s.v[1885] = 0.0;

        s.v[1886] = 0.0;

        s.v[1887] = 0.0;

        s.v[1888] = 0.0;

        s.v[1889] = 0.0;

        s.v[1890] = 0.0;

        s.v[1891] = 0.0;

        s.v[1892] = 0.0;

        s.v[1893] = 0.0;

        s.v[1894] = 0.0;

        s.v[1895] = 0.0;

        s.v[1896] = 0.0;

        s.v[1897] = 0.0;

        s.v[1898] = 0.0;

        s.v[1899] = 0.0;

        if (p.p52 != 0.0) {
            s.store_mul_ad_rhs(1896, 1798, A::tanh_scaled_input(s.ad_value(1798), (0.001 / p.p53)));
        } else {
            if (p.p52 == 0.0) {
                s.store_sqrt_square_offset(1896, 1798, p.p53);
            } else {
                s.store_scalar(1896, 0.0);
            }
        }

        s.store_sub(1897, 1797, 1798);

        s.store_scale(1831, 1805, s.v[1817]);

        s.store_add_scaled_ad_lhs(1833, A::div_from_scalar(s.v[1813], A::scale(s.ad_value(1805), 2.302585092994046)), 1896, s.v[1816]);

        s.store_offset_scaled(1834, 1803, s.v[1823], (((((-s.v[1804])) * (s.v[1823]))) + (s.v[1812])));

        s.store_powf_ad(1852, A::scale(s.ad_value(1803), 1.0 / (s.v[1804])), s.v[1825]);

        s.b[1900] = (s.v[1824] != 0.0);
        s.v[1900] = if s.b[1900] { 1.0 } else { 0.0 };

        if s.b[1900] {
            s.store_div_ad_rhs(1835, 1896, A::powf(A::offset(A::powf(A::scale(s.ad_value(1896), 1.0 / (s.v[1824])), s.v[1820]), 1.0), (1.0 / s.v[1820])));
        }

        if (!s.b[1900]) {
            s.store_scalar(1835, 0.0);
        }

        s.store_mul_sub_from_scalar_ad_lhs(1832, s.v[1814], A::scale(s.ad_value(1835), s.v[1815]), 1896);

        s.store_sub(1795, 1834, 1832);

        s.store_scaled_mul(1837, 1833, 1805, 2.0);

        s.store_mul(1838, 1808, 1837);

        s.store_sub_scaled_inputs(1895, 1795, 1.0, 1831, (p.p51 * 0.5));

        let assign31310_ad_e28372: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2(1894, assign31310_ad_e28372, 1.0, s.ad_value(1895), (-1.0), s.ad_value(1831), 1.0);

        s.b[1901] = (s.v[1894] > 50.0);
        s.v[1901] = if s.b[1901] { 1.0 } else { 0.0 };

        if s.b[1901] {
            s.store_scalar(1853, 0.0);
        }

        s.b[1902] = (s.v[1894] < (-50.0));
        s.v[1902] = if s.b[1902] { 1.0 } else { 0.0 };

        if ((!s.b[1901]) && s.b[1902]) {
            s.store_scalar(1853, 1.0);
        }

        if ((!s.b[1901]) && (!s.b[1902])) {
            s.store_div_from_scalar_offset_ad(1853, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        let assign31370_ad_e28451: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2(1854, assign31370_ad_e28451, 1.0, A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1853), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1837), 1.0);

        s.b[1903] = (s.v[1854] > 50.0);
        s.v[1903] = if s.b[1903] { 1.0 } else { 0.0 };

        if s.b[1903] {
            s.store_mul(1855, 1838, 1854);
        }

        s.b[1904] = (s.v[1854] < (-50.0));
        s.v[1904] = if s.b[1904] { 1.0 } else { 0.0 };

        if ((!s.b[1903]) && s.b[1904]) {
            s.store_mul_exp_rhs(1855, 1838, 1854);
        }

        if ((!s.b[1903]) && (!s.b[1904])) {
            s.store_mul_ad_rhs(1855, 1838, A::ln_one_plus_exp(s.ad_value(1854)));
        }

        s.store_div_from_scalar_ad(1841, s.v[1819], A::mul_offset_rhs(s.ad_value(1852), A::div_scaled_inputs(s.ad_value(1855), s.v[1821], s.ad_value(1808), 1.0), 1.0));

        s.store_div_scaled_value_by_product(1842, A::scale_offset(s.ad_value(1896), (s.v[1827] * 1.0 / (s.v[1807])), 1.0), (s.v[1818] * (1.0 + (s.v[1826] * s.v[1804]))), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), A::offset(A::div_scaled_inputs(s.ad_value(1855), s.v[1822], s.ad_value(1808), 1.0), 1.0), 1.0);

        s.store_add_ad(1843, A::mul3_scaled_output(s.ad_value(1853), s.ad_value(1805), s.ad_value(1841), (2.0 * 1.0 / (s.v[1807]))), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1853), s.ad_value(1842)));

        s.store_div_scaled_inputs(1859, s.ad_value(1842), s.v[1807], s.ad_value(1841), 1.0);

        s.store_add_scaled_product(1860, s.ad_value(1859), (-1.0), s.ad_value(1859), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1855), 2.0, s.ad_value(1808), s.ad_value(1859), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product(1861, A::mul_sub_from_scalar_rhs(s.ad_value(1859), 1.0, s.ad_value(1853)), 1.0, s.ad_value(1837), s.ad_value(1853), 1.0);

        s.store_add_scaled_product(1796, A::mul_sub_from_scalar_rhs(s.ad_value(1860), 1.0, s.ad_value(1853)), 1.0, s.ad_value(1837), s.ad_value(1853), 1.0);

        let assign31500_ad_e28650: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::div(s.ad_value(1798), s.ad_value(1796)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1798), s.ad_value(1796)), A::div(s.ad_value(1798), s.ad_value(1796)), 1.0), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1862, 1.0, A::offset(A::powf(assign31500_ad_e28650, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1863, 1798, 1862);

        let assign31520_ad_e28725: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 1.0), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1864, 1.0, A::offset(A::powf(assign31520_ad_e28725, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1865, 1798, 1864);

        s.store_div_scaled_inputs2(1894, s.ad_value(1797), 1.0, s.ad_value(1895), (-1.0), s.ad_value(1831), 1.0);

        s.b[1905] = (s.v[1894] > 50.0);
        s.v[1905] = if s.b[1905] { 1.0 } else { 0.0 };

        if s.b[1905] {
            s.store_scalar(1836, 0.0);
        }

        s.b[1906] = (s.v[1894] < (-50.0));
        s.v[1906] = if s.b[1906] { 1.0 } else { 0.0 };

        if ((!s.b[1905]) && s.b[1906]) {
            s.store_scalar(1836, 1.0);
        }

        if ((!s.b[1905]) && (!s.b[1906])) {
            s.store_div_from_scalar_offset_ad(1836, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3(1839, s.ad_value(1897), 1.0, s.ad_value(1865), (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1836), (-(p.p51 * 0.1))), -1.0, s.ad_value(1837), 1.0);

        s.b[1907] = (s.v[1839] > 50.0);
        s.v[1907] = if s.b[1907] { 1.0 } else { 0.0 };

        if s.b[1907] {
            s.store_mul(1840, 1838, 1839);
        }

        s.b[1908] = (s.v[1839] < (-50.0));
        s.v[1908] = if s.b[1908] { 1.0 } else { 0.0 };

        if ((!s.b[1907]) && s.b[1908]) {
            s.store_mul_exp_rhs(1840, 1838, 1839);
        }

        if ((!s.b[1907]) && (!s.b[1908])) {
            s.store_mul_ad_rhs(1840, 1838, A::ln_one_plus_exp(s.ad_value(1839)));
        }

        s.store_div_scaled_inputs2(1894, s.ad_value(1897), 1.0, s.ad_value(1895), (-1.0), s.ad_value(1831), 1.0);

        s.b[1909] = (s.v[1894] > 50.0);
        s.v[1909] = if s.b[1909] { 1.0 } else { 0.0 };

        if s.b[1909] {
            s.store_scalar(1866, 0.0);
        }

        s.b[1910] = (s.v[1894] < (-50.0));
        s.v[1910] = if s.b[1910] { 1.0 } else { 0.0 };

        if ((!s.b[1909]) && s.b[1910]) {
            s.store_scalar(1866, 1.0);
        }

        if ((!s.b[1909]) && (!s.b[1910])) {
            s.store_div_from_scalar_offset_ad(1866, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3(1867, s.ad_value(1797), 1.0, s.ad_value(1863), (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1866), (-(p.p51 * 0.1))), -1.0, s.ad_value(1837), 1.0);

        s.b[1911] = (s.v[1867] > 50.0);
        s.v[1911] = if s.b[1911] { 1.0 } else { 0.0 };

        if s.b[1911] {
            s.store_mul(1868, 1838, 1867);
        }

        s.b[1912] = (s.v[1867] < (-50.0));
        s.v[1912] = if s.b[1912] { 1.0 } else { 0.0 };

        if ((!s.b[1911]) && s.b[1912]) {
            s.store_mul_exp_rhs(1868, 1838, 1867);
        }

        if ((!s.b[1911]) && (!s.b[1912])) {
            s.store_mul_ad_rhs(1868, 1838, A::ln_one_plus_exp(s.ad_value(1867)));
        }

        s.store_div_scaled_inputs2(1869, s.ad_value(1840), 1.0, s.ad_value(1868), (-1.0), s.ad_value(1808), 1.0);

        s.store_div(1895, 1869, 1861);

        let assign31800_ad_e28955: A = A::div(s.ad_value(1895), A::powf(A::offset(A::powf({
    if (p.p52 != 0.0) {
        A::mul(s.ad_value(1895), A::tanh_scaled_input(s.ad_value(1895), (0.001 / p.p53)))
    } else {
        {
            if (p.p52 == 0.0) {
                A::sqrt(A::offset(A::square(s.ad_value(1895)), p.p53))
            } else {
                A::constant(0.0)
            }
        }
    }
}, s.v[1820]), 1.0), (1.0 / s.v[1820])));
        s.store_ad_value(1870, assign31800_ad_e28955);

        s.store_mul(1871, 1843, 1870);

        s.store_mul_ad_affine_product_lhs(1789, A::add(s.ad_value(1840), s.ad_value(1868)), s.ad_value(1871), (((s.v[1829] * s.v[1806]) * s.v[1828]) * 0.5), 0.0, 1830);

        s.store_div_from_scalar_scaled_input(1844, s.v[1813], 1805, 2.302585092994046);

        s.store_scaled_mul(1846, 1844, 1805, 2.0);

        s.store_mul(1847, 1808, 1846);

        s.store_sub_scaled_inputs(1899, 1834, 1.0, 1831, (p.p51 * 0.5));

        let assign31870_ad_e29037: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2(1898, assign31870_ad_e29037, 1.0, s.ad_value(1899), (-1.0), s.ad_value(1831), 1.0);

        s.b[1913] = (s.v[1898] > 50.0);
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if s.b[1913] {
            s.store_scalar(1856, 0.0);
        }

        s.b[1914] = (s.v[1898] < (-50.0));
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        if ((!s.b[1913]) && s.b[1914]) {
            s.store_scalar(1856, 1.0);
        }

        if ((!s.b[1913]) && (!s.b[1914])) {
            s.store_div_from_scalar_offset_ad(1856, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        let assign31930_ad_e29116: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_scaled_inputs2(1857, assign31930_ad_e29116, 1.0, A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1856), (-(p.p51 * 0.1))), (-1.0), s.ad_value(1846), 1.0);

        s.b[1915] = (s.v[1857] > 50.0);
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        if s.b[1915] {
            s.store_mul(1858, 1847, 1857);
        }

        s.b[1916] = (s.v[1857] < (-50.0));
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        if ((!s.b[1915]) && s.b[1916]) {
            s.store_mul_exp_rhs(1858, 1847, 1857);
        }

        if ((!s.b[1915]) && (!s.b[1916])) {
            s.store_mul_ad_rhs(1858, 1847, A::ln_one_plus_exp(s.ad_value(1857)));
        }

        s.store_div_from_scalar(1850, s.v[1819], 1852);

        s.store_scaled_div_from_scalar_ad(1851, (1.0 + (s.v[1826] * s.v[1804])), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), s.v[1818]);

        s.store_div_scaled_inputs(1872, s.ad_value(1851), s.v[1807], s.ad_value(1850), 1.0);

        s.store_add_scaled_product(1873, s.ad_value(1872), (-1.0), s.ad_value(1872), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1858), 2.0, s.ad_value(1808), s.ad_value(1872), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product(1874, A::mul_sub_from_scalar_rhs(s.ad_value(1873), 1.0, s.ad_value(1856)), 1.0, s.ad_value(1846), s.ad_value(1856), 1.0);

        let assign32040_ad_e29267: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::div(s.ad_value(1798), s.ad_value(1874)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1798), s.ad_value(1874)), A::div(s.ad_value(1798), s.ad_value(1874)), 1.0), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1875, 1.0, A::offset(A::powf(assign32040_ad_e29267, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1876, 1798, 1875);

        let assign32060_ad_e29342: A = {
    if (p.p52 != 0.0) {
        A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), (0.001 / p.p53)), (-0.5))
    } else {
        {
            if (p.p52 == 0.0) {
                A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 1.0), p.p53)), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_powf_ad(1877, 1.0, A::offset(A::powf(assign32060_ad_e29342, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1878, 1798, 1877);

        s.store_div_scaled_inputs2(1898, s.ad_value(1797), 1.0, s.ad_value(1899), (-1.0), s.ad_value(1831), 1.0);

        s.b[1917] = (s.v[1898] > 50.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        if s.b[1917] {
            s.store_scalar(1845, 0.0);
        }

        s.b[1918] = (s.v[1898] < (-50.0));
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        if ((!s.b[1917]) && s.b[1918]) {
            s.store_scalar(1845, 1.0);
        }

        if ((!s.b[1917]) && (!s.b[1918])) {
            s.store_div_from_scalar_offset_ad(1845, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3(1848, s.ad_value(1897), 1.0, s.ad_value(1878), (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1845), (-(p.p51 * 0.1))), -1.0, s.ad_value(1846), 1.0);

        s.b[1919] = (s.v[1848] > 50.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if s.b[1919] {
            s.store_mul(1849, 1847, 1848);
        }

        s.b[1920] = (s.v[1848] < (-50.0));
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((!s.b[1919]) && s.b[1920]) {
            s.store_mul_exp_rhs(1849, 1847, 1848);
        }

        if ((!s.b[1919]) && (!s.b[1920])) {
            s.store_mul_ad_rhs(1849, 1847, A::ln_one_plus_exp(s.ad_value(1848)));
        }

        s.store_div_scaled_inputs2(1898, s.ad_value(1897), 1.0, s.ad_value(1899), (-1.0), s.ad_value(1831), 1.0);

        s.b[1921] = (s.v[1898] > 50.0);
        s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };

        if s.b[1921] {
            s.store_scalar(1879, 0.0);
        }

        s.b[1922] = (s.v[1898] < (-50.0));
        s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };

        if ((!s.b[1921]) && s.b[1922]) {
            s.store_scalar(1879, 1.0);
        }

        if ((!s.b[1921]) && (!s.b[1922])) {
            s.store_div_from_scalar_offset_ad(1879, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3(1880, s.ad_value(1797), 1.0, s.ad_value(1876), (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1879), (-(p.p51 * 0.1))), -1.0, s.ad_value(1846), 1.0);

        s.b[1923] = (s.v[1880] > 50.0);
        s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };

        if s.b[1923] {
            s.store_mul(1881, 1847, 1880);
        }

        s.b[1924] = (s.v[1880] < (-50.0));
        s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };

        if ((!s.b[1923]) && s.b[1924]) {
            s.store_mul_exp_rhs(1881, 1847, 1880);
        }

        if ((!s.b[1923]) && (!s.b[1924])) {
            s.store_mul_ad_rhs(1881, 1847, A::ln_one_plus_exp(s.ad_value(1880)));
        }

        s.store_offset_square(1882, 1849, 1e-38);

        s.store_offset_mul(1883, 1882, 1849, 1e-57);

        s.store_offset_square(1884, 1881, 1e-38);

        s.store_offset_mul(1885, 1884, 1881, 1e-57);

        s.store_offset_mul(1886, 1849, 1881, 1e-38);

        s.store_div_scaled_inputs3(1887, s.ad_value(1882), (2.0 / 3.0), s.ad_value(1884), (2.0 / 3.0), s.ad_value(1886), (2.0 / 3.0), A::offset(A::add(s.ad_value(1849), s.ad_value(1881)), 2e-19), 1.0);

        s.store_div_ad(1888, A::add_scaled_inputs_products(s.ad_value(1883), (2.0 * 2.0), s.ad_value(1885), (3.0 * 2.0), s.ad_value(1882), s.ad_value(1881), (4.0 * 2.0), s.ad_value(1884), s.ad_value(1849), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1882), 15.0, s.ad_value(1884), 15.0, s.ad_value(1886), (2.0 * 15.0)));

        s.store_sub(1889, 1887, 1888);

        s.copy_ad(1890, 1888);

        s.store_scaled_mul(1790, 1889, 1830, (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829]));

        s.store_scaled_mul(1791, 1890, 1830, (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829]));

        s.b[1925] = (s.v[1799] == 1.0);
        s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };

        if s.b[1925] {
            s.store_div_ad_lhs(1891, A::sub_from_scalar(s.v[1800], A::sub_scaled_inputs(s.ad_value(1834), 1.0, s.ad_value(1831), (p.p51 * 0.5))), 1846);
        }

        s.b[1926] = (s.v[1891] > 50.0);
        s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };

        if (s.b[1925] && s.b[1926]) {
            s.copy_ad(1894, 1891);
        }

        s.b[1927] = (s.v[1891] < (-50.0));
        s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };

        if ((s.b[1925] && (!s.b[1926])) && s.b[1927]) {
            s.store_exp(1894, 1891);
        }

        if ((s.b[1925] && (!s.b[1926])) && (!s.b[1927])) {
            s.store_ln_one_plus_exp(1894, 1891);
        }

        if s.b[1925] {
            s.store_div_ad_lhs(1892, A::sub_from_scalar(s.v[1801], A::sub_scaled_inputs(s.ad_value(1834), 1.0, s.ad_value(1831), (p.p51 * 0.5))), 1846);
        }

        s.b[1928] = (s.v[1892] > 50.0);
        s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };

        if (s.b[1925] && s.b[1928]) {
            s.copy_ad(1894, 1892);
        }

        s.b[1929] = (s.v[1892] < (-50.0));
        s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };

        if ((s.b[1925] && (!s.b[1928])) && s.b[1929]) {
            s.store_exp(1894, 1892);
        }

        if ((s.b[1925] && (!s.b[1928])) && (!s.b[1929])) {
            s.store_ln_one_plus_exp(1894, 1892);
        }

        s.b[1930] = (s.v[1802] == 1.0);
        s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };

        if s.b[1930] {
            s.store_div_scaled_inputs3(1893, s.ad_value(1797), 1.0, s.ad_value(1834), -1.0, s.ad_value(1831), (-(-(p.p51 * 0.5))), s.ad_value(1846), 1.0);
        }

        s.b[1931] = (s.v[1893] > 50.0);
        s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };

        if (s.b[1930] && s.b[1931]) {
            s.copy_ad(1894, 1893);
        }

        s.b[1932] = (s.v[1893] < (-50.0));
        s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };

        if ((s.b[1930] && (!s.b[1931])) && s.b[1932]) {
            s.store_exp(1894, 1893);
        }

        if ((s.b[1930] && (!s.b[1931])) && (!s.b[1932])) {
            s.store_ln_one_plus_exp(1894, 1893);
        }

        s.copy_ad(1788, 1789);

        s.copy_ad(115, 1789);

        s.copy_ad(117, 1790);

        s.copy_ad(118, 1791);

        s.copy_ad(115, 1788);

        s.b[1933] = (p.p322 == 0.0);
        s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };

        s.v[234] = 0.0;

        s.v[242] = 0.0;

        s.v[243] = 0.0;

        s.v[244] = 0.0;

        s.v[245] = 0.0;

        s.v[246] = 0.0;

        s.v[247] = 0.0;

        s.v[248] = 0.0;

        s.v[254] = 0.0;

        s.v[255] = 0.0;

        s.v[256] = 0.0;

        s.v[257] = 0.0;

        s.v[258] = 0.0;

        s.b[2418] = (p.p291 == 1.0);
        s.v[2418] = if s.b[2418] { 1.0 } else { 0.0 };

        if s.b[2418] {
            s.store_scaled_voltage(234, ctx, nodes, Some(8), Some(7), p.p6);
        }

        s.b[2540] = (s.v[234] <= (p.p308 * p.p306));
        s.v[2540] = if s.b[2540] { 1.0 } else { 0.0 };

        if (s.b[2418] && s.b[2540]) {
            s.store_offset_scaled_ad(242, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(234), 1.0 / (p.p306)))), (-((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306)), ((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306));
        }

        if (s.b[2418] && (!s.b[2540])) {
            s.store_scalar(243, (1.0 - (((1.0 - p.p308)) as f64).sqrt()));
        }

        s.b[2541] = (p.p309 >= 1.0);
        s.v[2541] = if s.b[2541] { 1.0 } else { 0.0 };

        if ((s.b[2418] && (!s.b[2540])) && s.b[2541]) {
            s.store_scalar(249, (1.0 / ((2.0 * p.p306) * (((1.0 - p.p308)) as f64).sqrt())));
            s.store_offset(254, 234, (-(p.p308 * p.p306)));
            s.store_mul(244, 249, 254);
        }

        s.b[2542] = (p.p309 >= 2.0);
        s.v[2542] = if s.b[2542] { 1.0 } else { 0.0 };

        if (((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) {
            s.store_scale(250, 249, 1.0 / (((4.0 * p.p306) * (1.0 - p.p308))));
            s.store_square(255, 254);
            s.store_mul(245, 250, 255);
        }

        s.b[2543] = (p.p309 >= 3.0);
        s.v[2543] = if s.b[2543] { 1.0 } else { 0.0 };

        if ((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) {
            s.store_scale(251, 250, 1.0 / (((2.0 * p.p306) * (1.0 - p.p308))));
            s.store_mul(256, 255, 254);
            s.store_mul(246, 251, 256);
        }

        s.b[2544] = (p.p309 >= 4.0);
        s.v[2544] = if s.b[2544] { 1.0 } else { 0.0 };

        if (((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) {
            s.store_scale(252, 251, (5.0 * 1.0 / (((8.0 * p.p306) * (1.0 - p.p308)))));
            s.store_mul(257, 256, 254);
            s.store_mul(247, 252, 257);
        }

        s.b[2545] = (p.p309 >= 5.0);
        s.v[2545] = if s.b[2545] { 1.0 } else { 0.0 };

        if ((((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) && s.b[2545]) {
            s.store_scale(253, 252, (7.0 * 1.0 / (((10.0 * p.p306) * (1.0 - p.p308)))));
            s.store_mul(258, 257, 254);
            s.store_mul(248, 253, 258);
        }

        if ((((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) && (!s.b[2545])) {
            s.store_scalar(253, 0.0);
        }

        if (((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && (!s.b[2544])) {
            s.store_scalar(252, 0.0);
        }

        if ((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && (!s.b[2543])) {
            s.store_scalar(251, 0.0);
        }

        if (((s.b[2418] && (!s.b[2540])) && s.b[2541]) && (!s.b[2542])) {
            s.store_scalar(250, 0.0);
        }

        if ((s.b[2418] && (!s.b[2540])) && (!s.b[2541])) {
            s.store_scalar(249, 0.0);
        }

        if (s.b[2418] && (!s.b[2540])) {
            s.store_scaled_add_ad_lhs(242, A::add(A::add_scaled_inputs4(s.ad_value(243), 1.0, s.ad_value(244), 1.0, s.ad_value(245), 1.0, s.ad_value(246), 1.0), s.ad_value(247)), 248, ((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306));
        }

        s.b[2674] = ((((nv6 - nv2) - p.p27) / p.p28) > 50.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if s.b[2674] {
            s.store_add_scaled_products(214, s.ad_value(13), A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), s.ad_value(7), A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2675] = ((((nv6 - nv2) - p.p27) / p.p28) < (-50.0));
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if ((!s.b[2674]) && s.b[2675]) {
            s.store_add_scaled_products(214, s.ad_value(13), A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), s.ad_value(7), A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2674]) && (!s.b[2675])) {
            s.store_add_scaled_products(214, s.ad_value(13), A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), s.ad_value(7), A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2676] = ((((nv6 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if s.b[2676] {
            s.store_add_scaled_products(215, s.ad_value(14), A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), s.ad_value(8), A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2677] = ((((nv6 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if ((!s.b[2676]) && s.b[2677]) {
            s.store_add_scaled_products(215, s.ad_value(14), A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), s.ad_value(8), A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2676]) && (!s.b[2677])) {
            s.store_add_scaled_products(215, s.ad_value(14), A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), s.ad_value(8), A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2678] = ((((nv2 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if s.b[2678] {
            s.store_add_scaled_products(216, s.ad_value(15), A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), s.ad_value(9), A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2679] = ((((nv2 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((!s.b[2678]) && s.b[2679]) {
            s.store_add_scaled_products(216, s.ad_value(15), A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), s.ad_value(9), A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2678]) && (!s.b[2679])) {
            s.store_add_scaled_products(216, s.ad_value(15), A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), s.ad_value(9), A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2680] = ((((nv3 - nv2) - p.p27) / p.p28) > 50.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if s.b[2680] {
            s.store_add_scaled_products(218, s.ad_value(16), A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), s.ad_value(10), A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2681] = ((((nv3 - nv2) - p.p27) / p.p28) < (-50.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        if ((!s.b[2680]) && s.b[2681]) {
            s.store_add_scaled_products(218, s.ad_value(16), A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), s.ad_value(10), A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2680]) && (!s.b[2681])) {
            s.store_add_scaled_products(218, s.ad_value(16), A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), s.ad_value(10), A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2682] = ((((nv3 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if s.b[2682] {
            s.store_add_scaled_products(217, s.ad_value(17), A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), s.ad_value(11), A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2683] = ((((nv3 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((!s.b[2682]) && s.b[2683]) {
            s.store_add_scaled_products(217, s.ad_value(17), A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), s.ad_value(11), A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2682]) && (!s.b[2683])) {
            s.store_add_scaled_products(217, s.ad_value(17), A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), s.ad_value(11), A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2684] = ((((nv6 - nv3) - p.p27) / p.p28) > 50.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if s.b[2684] {
            s.store_add_scaled_products(219, s.ad_value(18), A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), s.ad_value(12), A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), (p.p0 * p.p2));
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        s.b[2685] = ((((nv6 - nv3) - p.p27) / p.p28) < (-50.0));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if ((!s.b[2684]) && s.b[2685]) {
            s.store_add_scaled_products(219, s.ad_value(18), A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), s.ad_value(12), A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2684]) && (!s.b[2685])) {
            s.store_add_scaled_products(219, s.ad_value(18), A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), s.ad_value(12), A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2700] = (p.p320 > 0.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let (eq0_e383,) = {
    if s.b[308] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e383;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e387,) = {
    if s.b[308] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e387;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let (eq2_e391,) = {
    if s.b[308] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e391;
        stamper.stamp_potential_const_local(
            2,
            eq2_value,
        );
        let (eq3_e395,) = {
    if s.b[308] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e395;
        stamper.stamp_potential_const_local(
            3,
            eq3_value,
        );
        let (eq4_e399,) = {
    if s.b[308] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e399;
        stamper.stamp_potential_const_local(
            4,
            eq4_value,
        );
        let (eq5_e403,) = {
    if s.b[308] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e403;
        stamper.stamp_potential_const_local(
            5,
            eq5_value,
        );
        let (eq6_e408, eq6_e408_d_n0, eq6_e408_d_n1, eq6_e408_d_n2, eq6_e408_d_n3, eq6_e408_d_n4, eq6_e408_d_n5, eq6_e408_d_n6, eq6_e408_d_n7, eq6_e408_d_n8, eq6_e408_d_n9, eq6_e408_d_n10, eq6_e408_d_n11, eq6_e408_d_n12, eq6_e408_d_n13, eq6_e408_d_n14, eq6_e408_d_n15, eq6_e408_d_n16, eq6_e408_d_n17, eq6_e408_d_n18, eq6_e408_d_n19, eq6_e408_d_n20, eq6_e408_d_n21, eq6_e408_d_n22, eq6_e408_d_n23, eq6_e408_d_n24, eq6_e408_d_n25, eq6_e408_d_n26, eq6_e408_d_n27, eq6_e408_d_n28, eq6_e408_d_n29, eq6_e408_d_b0, eq6_e408_d_b1, eq6_e408_d_b2, eq6_e408_d_b3, eq6_e408_d_b4, eq6_e408_d_b5, eq6_e408_d_b6, eq6_e408_d_b7, eq6_e408_d_b8, eq6_e408_d_b9, eq6_e408_d_b10, eq6_e408_d_b11, eq6_e408_d_b12, eq6_e408_d_b13, eq6_e408_d_b14, eq6_e408_d_b15, eq6_e408_d_b16, eq6_e408_d_b17, eq6_e408_d_b18, eq6_e408_d_b19, eq6_e408_d_b20, eq6_e408_d_b21, eq6_e408_d_b22, eq6_e408_d_b23, eq6_e408_d_b24, eq6_e408_d_b25, eq6_e408_d_b26, eq6_e408_d_b27, eq6_e408_d_b28, eq6_e408_d_b29, eq6_e408_d_b30, eq6_e408_d_b31, eq6_e408_d_b32, eq6_e408_d_b33, eq6_e408_d_b34, eq6_e408_d_b35,) = {
    if s.b[308] {
        let eq6_e406: f64 = (-s.v[222]);
        let eq6_e406_d_n0: f64 = (-s.dn[222][0]);
        let eq6_e406_d_n1: f64 = (-s.dn[222][1]);
        let eq6_e406_d_n2: f64 = (-s.dn[222][2]);
        let eq6_e406_d_n3: f64 = (-s.dn[222][3]);
        let eq6_e406_d_n4: f64 = (-s.dn[222][4]);
        let eq6_e406_d_n5: f64 = (-s.dn[222][5]);
        let eq6_e406_d_n6: f64 = (-s.dn[222][6]);
        let eq6_e406_d_n7: f64 = (-s.dn[222][7]);
        let eq6_e406_d_n8: f64 = (-s.dn[222][8]);
        let eq6_e406_d_n9: f64 = (-s.dn[222][9]);
        let eq6_e406_d_n10: f64 = (-s.dn[222][10]);
        let eq6_e406_d_n11: f64 = (-s.dn[222][11]);
        let eq6_e406_d_n12: f64 = (-s.dn[222][12]);
        let eq6_e406_d_n13: f64 = (-s.dn[222][13]);
        let eq6_e406_d_n14: f64 = (-s.dn[222][14]);
        let eq6_e406_d_n15: f64 = (-s.dn[222][15]);
        let eq6_e406_d_n16: f64 = (-s.dn[222][16]);
        let eq6_e406_d_n17: f64 = (-s.dn[222][17]);
        let eq6_e406_d_n18: f64 = (-s.dn[222][18]);
        let eq6_e406_d_n19: f64 = (-s.dn[222][19]);
        let eq6_e406_d_n20: f64 = (-s.dn[222][20]);
        let eq6_e406_d_n21: f64 = (-s.dn[222][21]);
        let eq6_e406_d_n22: f64 = (-s.dn[222][22]);
        let eq6_e406_d_n23: f64 = (-s.dn[222][23]);
        let eq6_e406_d_n24: f64 = (-s.dn[222][24]);
        let eq6_e406_d_n25: f64 = (-s.dn[222][25]);
        let eq6_e406_d_n26: f64 = (-s.dn[222][26]);
        let eq6_e406_d_n27: f64 = (-s.dn[222][27]);
        let eq6_e406_d_n28: f64 = (-s.dn[222][28]);
        let eq6_e406_d_n29: f64 = (-s.dn[222][29]);
        let eq6_e406_d_b0: f64 = (-s.db[222][0]);
        let eq6_e406_d_b1: f64 = (-s.db[222][1]);
        let eq6_e406_d_b2: f64 = (-s.db[222][2]);
        let eq6_e406_d_b3: f64 = (-s.db[222][3]);
        let eq6_e406_d_b4: f64 = (-s.db[222][4]);
        let eq6_e406_d_b5: f64 = (-s.db[222][5]);
        let eq6_e406_d_b6: f64 = (-s.db[222][6]);
        let eq6_e406_d_b7: f64 = (-s.db[222][7]);
        let eq6_e406_d_b8: f64 = (-s.db[222][8]);
        let eq6_e406_d_b9: f64 = (-s.db[222][9]);
        let eq6_e406_d_b10: f64 = (-s.db[222][10]);
        let eq6_e406_d_b11: f64 = (-s.db[222][11]);
        let eq6_e406_d_b12: f64 = (-s.db[222][12]);
        let eq6_e406_d_b13: f64 = (-s.db[222][13]);
        let eq6_e406_d_b14: f64 = (-s.db[222][14]);
        let eq6_e406_d_b15: f64 = (-s.db[222][15]);
        let eq6_e406_d_b16: f64 = (-s.db[222][16]);
        let eq6_e406_d_b17: f64 = (-s.db[222][17]);
        let eq6_e406_d_b18: f64 = (-s.db[222][18]);
        let eq6_e406_d_b19: f64 = (-s.db[222][19]);
        let eq6_e406_d_b20: f64 = (-s.db[222][20]);
        let eq6_e406_d_b21: f64 = (-s.db[222][21]);
        let eq6_e406_d_b22: f64 = (-s.db[222][22]);
        let eq6_e406_d_b23: f64 = (-s.db[222][23]);
        let eq6_e406_d_b24: f64 = (-s.db[222][24]);
        let eq6_e406_d_b25: f64 = (-s.db[222][25]);
        let eq6_e406_d_b26: f64 = (-s.db[222][26]);
        let eq6_e406_d_b27: f64 = (-s.db[222][27]);
        let eq6_e406_d_b28: f64 = (-s.db[222][28]);
        let eq6_e406_d_b29: f64 = (-s.db[222][29]);
        let eq6_e406_d_b30: f64 = (-s.db[222][30]);
        let eq6_e406_d_b31: f64 = (-s.db[222][31]);
        let eq6_e406_d_b32: f64 = (-s.db[222][32]);
        let eq6_e406_d_b33: f64 = (-s.db[222][33]);
        let eq6_e406_d_b34: f64 = (-s.db[222][34]);
        let eq6_e406_d_b35: f64 = (-s.db[222][35]);
        (eq6_e406, eq6_e406_d_n0, eq6_e406_d_n1, eq6_e406_d_n2, eq6_e406_d_n3, eq6_e406_d_n4, eq6_e406_d_n5, eq6_e406_d_n6, eq6_e406_d_n7, eq6_e406_d_n8, eq6_e406_d_n9, eq6_e406_d_n10, eq6_e406_d_n11, eq6_e406_d_n12, eq6_e406_d_n13, eq6_e406_d_n14, eq6_e406_d_n15, eq6_e406_d_n16, eq6_e406_d_n17, eq6_e406_d_n18, eq6_e406_d_n19, eq6_e406_d_n20, eq6_e406_d_n21, eq6_e406_d_n22, eq6_e406_d_n23, eq6_e406_d_n24, eq6_e406_d_n25, eq6_e406_d_n26, eq6_e406_d_n27, eq6_e406_d_n28, eq6_e406_d_n29, eq6_e406_d_b0, eq6_e406_d_b1, eq6_e406_d_b2, eq6_e406_d_b3, eq6_e406_d_b4, eq6_e406_d_b5, eq6_e406_d_b6, eq6_e406_d_b7, eq6_e406_d_b8, eq6_e406_d_b9, eq6_e406_d_b10, eq6_e406_d_b11, eq6_e406_d_b12, eq6_e406_d_b13, eq6_e406_d_b14, eq6_e406_d_b15, eq6_e406_d_b16, eq6_e406_d_b17, eq6_e406_d_b18, eq6_e406_d_b19, eq6_e406_d_b20, eq6_e406_d_b21, eq6_e406_d_b22, eq6_e406_d_b23, eq6_e406_d_b24, eq6_e406_d_b25, eq6_e406_d_b26, eq6_e406_d_b27, eq6_e406_d_b28, eq6_e406_d_b29, eq6_e406_d_b30, eq6_e406_d_b31, eq6_e406_d_b32, eq6_e406_d_b33, eq6_e406_d_b34, eq6_e406_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e408;
        let eq6_node_derivatives: [f64; 30] = [eq6_e408_d_n0, eq6_e408_d_n1, eq6_e408_d_n2, eq6_e408_d_n3, eq6_e408_d_n4, eq6_e408_d_n5, eq6_e408_d_n6, eq6_e408_d_n7, eq6_e408_d_n8, eq6_e408_d_n9, eq6_e408_d_n10, eq6_e408_d_n11, eq6_e408_d_n12, eq6_e408_d_n13, eq6_e408_d_n14, eq6_e408_d_n15, eq6_e408_d_n16, eq6_e408_d_n17, eq6_e408_d_n18, eq6_e408_d_n19, eq6_e408_d_n20, eq6_e408_d_n21, eq6_e408_d_n22, eq6_e408_d_n23, eq6_e408_d_n24, eq6_e408_d_n25, eq6_e408_d_n26, eq6_e408_d_n27, eq6_e408_d_n28, eq6_e408_d_n29];
        let eq6_branch_derivatives: [f64; 36] = [eq6_e408_d_b0, eq6_e408_d_b1, eq6_e408_d_b2, eq6_e408_d_b3, eq6_e408_d_b4, eq6_e408_d_b5, eq6_e408_d_b6, eq6_e408_d_b7, eq6_e408_d_b8, eq6_e408_d_b9, eq6_e408_d_b10, eq6_e408_d_b11, eq6_e408_d_b12, eq6_e408_d_b13, eq6_e408_d_b14, eq6_e408_d_b15, eq6_e408_d_b16, eq6_e408_d_b17, eq6_e408_d_b18, eq6_e408_d_b19, eq6_e408_d_b20, eq6_e408_d_b21, eq6_e408_d_b22, eq6_e408_d_b23, eq6_e408_d_b24, eq6_e408_d_b25, eq6_e408_d_b26, eq6_e408_d_b27, eq6_e408_d_b28, eq6_e408_d_b29, eq6_e408_d_b30, eq6_e408_d_b31, eq6_e408_d_b32, eq6_e408_d_b33, eq6_e408_d_b34, eq6_e408_d_b35];
        stamper.stamp_current_dense_local(
            Some(21),
            None,
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e414, eq7_e414_d_n21,) = {
    if s.b[308] {
        let eq7_e412: f64 = ((nv21 - 0.0) / p.p329);
        let eq7_e412_d_n21: f64 = (1.0 / p.p329);
        (eq7_e412, eq7_e412_d_n21,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e414;
        stamper.stamp_current_node1_local(
            Some(21),
            None,
            multiplicity * (eq7_value),
            21,
            multiplicity * (eq7_e414_d_n21),
        );
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21,) = {
    if s.b[308] {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e418_d_n20: f64 = (-p.p330);
        let eq8_e418_d_n21: f64 = p.p330;
        let eq8_e419: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq8_e418);
        let eq8_e419_d_n20: f64 = (eq8_e418_d_n20 * ddt_scale);
        let eq8_e419_d_n21: f64 = (eq8_e418_d_n21 * ddt_scale);
        (eq8_e419, eq8_e419_d_n20, eq8_e419_d_n21,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e421;
        stamper.stamp_current_node2_local(
            Some(21),
            Some(20),
            multiplicity * (eq8_value),
            20,
            multiplicity * (eq8_e421_d_n20),
            21,
            multiplicity * (eq8_e421_d_n21),
        );
        let (eq9_e428, eq9_e428_d_n20,) = {
    if s.b[308] {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));
        let eq9_e425_d_n20: f64 = p.p332;
        let eq9_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq9_e425);
        let eq9_e426_d_n20: f64 = (eq9_e425_d_n20 * ddt_scale);
        (eq9_e426, eq9_e426_d_n20,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e428;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (eq9_value),
            20,
            multiplicity * (eq9_e428_d_n20),
        );
        let (eq10_e432, eq10_e432_d_n20,) = {
    if s.b[308] {
        ((nv20 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e432;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (eq10_value),
            20,
            multiplicity * (eq10_e432_d_n20),
        );
        let (eq11_e439,) = {
    if ((!s.b[308]) && s.b[309]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e439;
        stamper.stamp_potential_const_local(
            6,
            eq11_value,
        );
        let (eq12_e446,) = {
    if ((!s.b[308]) && s.b[309]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e446;
        stamper.stamp_potential_const_local(
            7,
            eq12_value,
        );
        let (eq13_e455, eq13_e455_d_n0, eq13_e455_d_n2,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq13_e453: f64 = (p.p6 * (nv0 - nv2));
        let eq13_e453_d_n0: f64 = p.p6;
        let eq13_e453_d_n2: f64 = (-p.p6);
        (eq13_e453, eq13_e453_d_n0, eq13_e453_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e455;
        stamper.stamp_potential_node2_local(
            8,
            eq13_value,
            0,
            eq13_e455_d_n0,
            2,
            eq13_e455_d_n2,
        );
        let eq14_ad_e518: A = {
    if ((!s.b[308]) && s.b[309]) {
        let eq14_ad_e513: A = {
            if ((!(((nv24 - nv23) / s.v[113]) > 50.0)) && (!(((nv24 - nv23) / s.v[113]) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, nodes, Some(24), Some(23)), s.ad_value(113)))
            } else {
                let eq14_ad_e512: A = {
                    if ((!(((nv24 - nv23) / s.v[113]) > 50.0)) && (((nv24 - nv23) / s.v[113]) < (-50.0))) {
                        A::exp_scaled_input(A::constant(50.0), -1.0)
                    } else {
                        {
                            if (((nv24 - nv23) / s.v[113]) > 50.0) {
                                A::scaled_offset(A::div(A::voltage(ctx, nodes, Some(24), Some(23)), s.ad_value(113)), (((-50.0)) + (1.0)), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                };
                eq14_ad_e512
            }
        };
        A::scaled_offset(eq14_ad_e513, (-1.0), p.p346)
    } else {
        A::constant(0.0)
    }
};
        let eq14_ad: A = eq14_ad_e518;
        stamper.stamp_current_dense_local(
            Some(24),
            Some(23),
            multiplicity * eq14_ad.value,
            &eq14_ad.dn,
            &eq14_ad.db,
            multiplicity,
        );
        let (eq15_e527, eq15_e527_d_n22, eq15_e527_d_n24,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq15_e525: f64 = ((nv22 - nv24) / p.p340);
        let eq15_e525_d_n22: f64 = (1.0 / p.p340);
        let eq15_e525_d_n24: f64 = (-1.0 / p.p340);
        (eq15_e525, eq15_e525_d_n22, eq15_e525_d_n24,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e527;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(24),
            multiplicity * (eq15_value),
            22,
            multiplicity * (eq15_e527_d_n22),
            24,
            multiplicity * (eq15_e527_d_n24),
        );
        let (eq16_e536, eq16_e536_d_n22, eq16_e536_d_n23,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq16_e534: f64 = ((nv22 - nv23) / p.p339);
        let eq16_e534_d_n22: f64 = (1.0 / p.p339);
        let eq16_e534_d_n23: f64 = (-1.0 / p.p339);
        (eq16_e534, eq16_e534_d_n22, eq16_e534_d_n23,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e536;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(23),
            multiplicity * (eq16_value),
            22,
            multiplicity * (eq16_e536_d_n22),
            23,
            multiplicity * (eq16_e536_d_n23),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq17_e564, eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29, eq17_e564_d_b0, eq17_e564_d_b1, eq17_e564_d_b2, eq17_e564_d_b3, eq17_e564_d_b4, eq17_e564_d_b5, eq17_e564_d_b6, eq17_e564_d_b7, eq17_e564_d_b8, eq17_e564_d_b9, eq17_e564_d_b10, eq17_e564_d_b11, eq17_e564_d_b12, eq17_e564_d_b13, eq17_e564_d_b14, eq17_e564_d_b15, eq17_e564_d_b16, eq17_e564_d_b17, eq17_e564_d_b18, eq17_e564_d_b19, eq17_e564_d_b20, eq17_e564_d_b21, eq17_e564_d_b22, eq17_e564_d_b23, eq17_e564_d_b24, eq17_e564_d_b25, eq17_e564_d_b26, eq17_e564_d_b27, eq17_e564_d_b28, eq17_e564_d_b29, eq17_e564_d_b30, eq17_e564_d_b31, eq17_e564_d_b32, eq17_e564_d_b33, eq17_e564_d_b34, eq17_e564_d_b35,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq17_e543: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[225]);
        let eq17_e543_d_n0: f64 = (s.dn[225][0] * ddt_scale);
        let eq17_e543_d_n1: f64 = (s.dn[225][1] * ddt_scale);
        let eq17_e543_d_n2: f64 = (s.dn[225][2] * ddt_scale);
        let eq17_e543_d_n3: f64 = (s.dn[225][3] * ddt_scale);
        let eq17_e543_d_n4: f64 = (s.dn[225][4] * ddt_scale);
        let eq17_e543_d_n5: f64 = (s.dn[225][5] * ddt_scale);
        let eq17_e543_d_n6: f64 = (s.dn[225][6] * ddt_scale);
        let eq17_e543_d_n7: f64 = (s.dn[225][7] * ddt_scale);
        let eq17_e543_d_n8: f64 = (s.dn[225][8] * ddt_scale);
        let eq17_e543_d_n9: f64 = (s.dn[225][9] * ddt_scale);
        let eq17_e543_d_n10: f64 = (s.dn[225][10] * ddt_scale);
        let eq17_e543_d_n11: f64 = (s.dn[225][11] * ddt_scale);
        let eq17_e543_d_n12: f64 = (s.dn[225][12] * ddt_scale);
        let eq17_e543_d_n13: f64 = (s.dn[225][13] * ddt_scale);
        let eq17_e543_d_n14: f64 = (s.dn[225][14] * ddt_scale);
        let eq17_e543_d_n15: f64 = (s.dn[225][15] * ddt_scale);
        let eq17_e543_d_n16: f64 = (s.dn[225][16] * ddt_scale);
        let eq17_e543_d_n17: f64 = (s.dn[225][17] * ddt_scale);
        let eq17_e543_d_n18: f64 = (s.dn[225][18] * ddt_scale);
        let eq17_e543_d_n19: f64 = (s.dn[225][19] * ddt_scale);
        let eq17_e543_d_n20: f64 = (s.dn[225][20] * ddt_scale);
        let eq17_e543_d_n21: f64 = (s.dn[225][21] * ddt_scale);
        let eq17_e543_d_n22: f64 = (s.dn[225][22] * ddt_scale);
        let eq17_e543_d_n23: f64 = (s.dn[225][23] * ddt_scale);
        let eq17_e543_d_n24: f64 = (s.dn[225][24] * ddt_scale);
        let eq17_e543_d_n25: f64 = (s.dn[225][25] * ddt_scale);
        let eq17_e543_d_n26: f64 = (s.dn[225][26] * ddt_scale);
        let eq17_e543_d_n27: f64 = (s.dn[225][27] * ddt_scale);
        let eq17_e543_d_n28: f64 = (s.dn[225][28] * ddt_scale);
        let eq17_e543_d_n29: f64 = (s.dn[225][29] * ddt_scale);
        let eq17_e543_d_b0: f64 = (s.db[225][0] * ddt_scale);
        let eq17_e543_d_b1: f64 = (s.db[225][1] * ddt_scale);
        let eq17_e543_d_b2: f64 = (s.db[225][2] * ddt_scale);
        let eq17_e543_d_b3: f64 = (s.db[225][3] * ddt_scale);
        let eq17_e543_d_b4: f64 = (s.db[225][4] * ddt_scale);
        let eq17_e543_d_b5: f64 = (s.db[225][5] * ddt_scale);
        let eq17_e543_d_b6: f64 = (s.db[225][6] * ddt_scale);
        let eq17_e543_d_b7: f64 = (s.db[225][7] * ddt_scale);
        let eq17_e543_d_b8: f64 = (s.db[225][8] * ddt_scale);
        let eq17_e543_d_b9: f64 = (s.db[225][9] * ddt_scale);
        let eq17_e543_d_b10: f64 = (s.db[225][10] * ddt_scale);
        let eq17_e543_d_b11: f64 = (s.db[225][11] * ddt_scale);
        let eq17_e543_d_b12: f64 = (s.db[225][12] * ddt_scale);
        let eq17_e543_d_b13: f64 = (s.db[225][13] * ddt_scale);
        let eq17_e543_d_b14: f64 = (s.db[225][14] * ddt_scale);
        let eq17_e543_d_b15: f64 = (s.db[225][15] * ddt_scale);
        let eq17_e543_d_b16: f64 = (s.db[225][16] * ddt_scale);
        let eq17_e543_d_b17: f64 = (s.db[225][17] * ddt_scale);
        let eq17_e543_d_b18: f64 = (s.db[225][18] * ddt_scale);
        let eq17_e543_d_b19: f64 = (s.db[225][19] * ddt_scale);
        let eq17_e543_d_b20: f64 = (s.db[225][20] * ddt_scale);
        let eq17_e543_d_b21: f64 = (s.db[225][21] * ddt_scale);
        let eq17_e543_d_b22: f64 = (s.db[225][22] * ddt_scale);
        let eq17_e543_d_b23: f64 = (s.db[225][23] * ddt_scale);
        let eq17_e543_d_b24: f64 = (s.db[225][24] * ddt_scale);
        let eq17_e543_d_b25: f64 = (s.db[225][25] * ddt_scale);
        let eq17_e543_d_b26: f64 = (s.db[225][26] * ddt_scale);
        let eq17_e543_d_b27: f64 = (s.db[225][27] * ddt_scale);
        let eq17_e543_d_b28: f64 = (s.db[225][28] * ddt_scale);
        let eq17_e543_d_b29: f64 = (s.db[225][29] * ddt_scale);
        let eq17_e543_d_b30: f64 = (s.db[225][30] * ddt_scale);
        let eq17_e543_d_b31: f64 = (s.db[225][31] * ddt_scale);
        let eq17_e543_d_b32: f64 = (s.db[225][32] * ddt_scale);
        let eq17_e543_d_b33: f64 = (s.db[225][33] * ddt_scale);
        let eq17_e543_d_b34: f64 = (s.db[225][34] * ddt_scale);
        let eq17_e543_d_b35: f64 = (s.db[225][35] * ddt_scale);
        let eq17_e544: f64 = (p.p341 * eq17_e543);
        let eq17_e544_d_n0: f64 = (p.p341 * eq17_e543_d_n0);
        let eq17_e544_d_n1: f64 = (p.p341 * eq17_e543_d_n1);
        let eq17_e544_d_n2: f64 = (p.p341 * eq17_e543_d_n2);
        let eq17_e544_d_n3: f64 = (p.p341 * eq17_e543_d_n3);
        let eq17_e544_d_n4: f64 = (p.p341 * eq17_e543_d_n4);
        let eq17_e544_d_n5: f64 = (p.p341 * eq17_e543_d_n5);
        let eq17_e544_d_n6: f64 = (p.p341 * eq17_e543_d_n6);
        let eq17_e544_d_n7: f64 = (p.p341 * eq17_e543_d_n7);
        let eq17_e544_d_n8: f64 = (p.p341 * eq17_e543_d_n8);
        let eq17_e544_d_n9: f64 = (p.p341 * eq17_e543_d_n9);
        let eq17_e544_d_n10: f64 = (p.p341 * eq17_e543_d_n10);
        let eq17_e544_d_n11: f64 = (p.p341 * eq17_e543_d_n11);
        let eq17_e544_d_n12: f64 = (p.p341 * eq17_e543_d_n12);
        let eq17_e544_d_n13: f64 = (p.p341 * eq17_e543_d_n13);
        let eq17_e544_d_n14: f64 = (p.p341 * eq17_e543_d_n14);
        let eq17_e544_d_n15: f64 = (p.p341 * eq17_e543_d_n15);
        let eq17_e544_d_n16: f64 = (p.p341 * eq17_e543_d_n16);
        let eq17_e544_d_n17: f64 = (p.p341 * eq17_e543_d_n17);
        let eq17_e544_d_n18: f64 = (p.p341 * eq17_e543_d_n18);
        let eq17_e544_d_n19: f64 = (p.p341 * eq17_e543_d_n19);
        let eq17_e544_d_n20: f64 = (p.p341 * eq17_e543_d_n20);
        let eq17_e544_d_n21: f64 = (p.p341 * eq17_e543_d_n21);
        let eq17_e544_d_n22: f64 = (p.p341 * eq17_e543_d_n22);
        let eq17_e544_d_n23: f64 = (p.p341 * eq17_e543_d_n23);
        let eq17_e544_d_n24: f64 = (p.p341 * eq17_e543_d_n24);
        let eq17_e544_d_n25: f64 = (p.p341 * eq17_e543_d_n25);
        let eq17_e544_d_n26: f64 = (p.p341 * eq17_e543_d_n26);
        let eq17_e544_d_n27: f64 = (p.p341 * eq17_e543_d_n27);
        let eq17_e544_d_n28: f64 = (p.p341 * eq17_e543_d_n28);
        let eq17_e544_d_n29: f64 = (p.p341 * eq17_e543_d_n29);
        let eq17_e544_d_b0: f64 = (p.p341 * eq17_e543_d_b0);
        let eq17_e544_d_b1: f64 = (p.p341 * eq17_e543_d_b1);
        let eq17_e544_d_b2: f64 = (p.p341 * eq17_e543_d_b2);
        let eq17_e544_d_b3: f64 = (p.p341 * eq17_e543_d_b3);
        let eq17_e544_d_b4: f64 = (p.p341 * eq17_e543_d_b4);
        let eq17_e544_d_b5: f64 = (p.p341 * eq17_e543_d_b5);
        let eq17_e544_d_b6: f64 = (p.p341 * eq17_e543_d_b6);
        let eq17_e544_d_b7: f64 = (p.p341 * eq17_e543_d_b7);
        let eq17_e544_d_b8: f64 = (p.p341 * eq17_e543_d_b8);
        let eq17_e544_d_b9: f64 = (p.p341 * eq17_e543_d_b9);
        let eq17_e544_d_b10: f64 = (p.p341 * eq17_e543_d_b10);
        let eq17_e544_d_b11: f64 = (p.p341 * eq17_e543_d_b11);
        let eq17_e544_d_b12: f64 = (p.p341 * eq17_e543_d_b12);
        let eq17_e544_d_b13: f64 = (p.p341 * eq17_e543_d_b13);
        let eq17_e544_d_b14: f64 = (p.p341 * eq17_e543_d_b14);
        let eq17_e544_d_b15: f64 = (p.p341 * eq17_e543_d_b15);
        let eq17_e544_d_b16: f64 = (p.p341 * eq17_e543_d_b16);
        let eq17_e544_d_b17: f64 = (p.p341 * eq17_e543_d_b17);
        let eq17_e544_d_b18: f64 = (p.p341 * eq17_e543_d_b18);
        let eq17_e544_d_b19: f64 = (p.p341 * eq17_e543_d_b19);
        let eq17_e544_d_b20: f64 = (p.p341 * eq17_e543_d_b20);
        let eq17_e544_d_b21: f64 = (p.p341 * eq17_e543_d_b21);
        let eq17_e544_d_b22: f64 = (p.p341 * eq17_e543_d_b22);
        let eq17_e544_d_b23: f64 = (p.p341 * eq17_e543_d_b23);
        let eq17_e544_d_b24: f64 = (p.p341 * eq17_e543_d_b24);
        let eq17_e544_d_b25: f64 = (p.p341 * eq17_e543_d_b25);
        let eq17_e544_d_b26: f64 = (p.p341 * eq17_e543_d_b26);
        let eq17_e544_d_b27: f64 = (p.p341 * eq17_e543_d_b27);
        let eq17_e544_d_b28: f64 = (p.p341 * eq17_e543_d_b28);
        let eq17_e544_d_b29: f64 = (p.p341 * eq17_e543_d_b29);
        let eq17_e544_d_b30: f64 = (p.p341 * eq17_e543_d_b30);
        let eq17_e544_d_b31: f64 = (p.p341 * eq17_e543_d_b31);
        let eq17_e544_d_b32: f64 = (p.p341 * eq17_e543_d_b32);
        let eq17_e544_d_b33: f64 = (p.p341 * eq17_e543_d_b33);
        let eq17_e544_d_b34: f64 = (p.p341 * eq17_e543_d_b34);
        let eq17_e544_d_b35: f64 = (p.p341 * eq17_e543_d_b35);
        let eq17_e549: f64 = (s.v[111] - s.v[109]);
        let eq17_e550: f64 = (p.p342 * eq17_e549);
        let eq17_e550_d_n0: f64 = (p.p342 * s.dn[111][0]);
        let eq17_e550_d_n1: f64 = (p.p342 * s.dn[111][1]);
        let eq17_e550_d_n2: f64 = (p.p342 * s.dn[111][2]);
        let eq17_e550_d_n3: f64 = (p.p342 * s.dn[111][3]);
        let eq17_e550_d_n4: f64 = (p.p342 * s.dn[111][4]);
        let eq17_e550_d_n5: f64 = (p.p342 * s.dn[111][5]);
        let eq17_e550_d_n6: f64 = (p.p342 * s.dn[111][6]);
        let eq17_e550_d_n7: f64 = (p.p342 * s.dn[111][7]);
        let eq17_e550_d_n8: f64 = (p.p342 * s.dn[111][8]);
        let eq17_e550_d_n9: f64 = (p.p342 * s.dn[111][9]);
        let eq17_e550_d_n10: f64 = (p.p342 * s.dn[111][10]);
        let eq17_e550_d_n11: f64 = (p.p342 * s.dn[111][11]);
        let eq17_e550_d_n12: f64 = (p.p342 * s.dn[111][12]);
        let eq17_e550_d_n13: f64 = (p.p342 * s.dn[111][13]);
        let eq17_e550_d_n14: f64 = (p.p342 * s.dn[111][14]);
        let eq17_e550_d_n15: f64 = (p.p342 * s.dn[111][15]);
        let eq17_e550_d_n16: f64 = (p.p342 * s.dn[111][16]);
        let eq17_e550_d_n17: f64 = (p.p342 * s.dn[111][17]);
        let eq17_e550_d_n18: f64 = (p.p342 * s.dn[111][18]);
        let eq17_e550_d_n19: f64 = (p.p342 * s.dn[111][19]);
        let eq17_e550_d_n20: f64 = (p.p342 * s.dn[111][20]);
        let eq17_e550_d_n21: f64 = (p.p342 * s.dn[111][21]);
        let eq17_e550_d_n22: f64 = (p.p342 * s.dn[111][22]);
        let eq17_e550_d_n23: f64 = (p.p342 * s.dn[111][23]);
        let eq17_e550_d_n24: f64 = (p.p342 * s.dn[111][24]);
        let eq17_e550_d_n25: f64 = (p.p342 * s.dn[111][25]);
        let eq17_e550_d_n26: f64 = (p.p342 * s.dn[111][26]);
        let eq17_e550_d_n27: f64 = (p.p342 * s.dn[111][27]);
        let eq17_e550_d_n28: f64 = (p.p342 * s.dn[111][28]);
        let eq17_e550_d_n29: f64 = (p.p342 * s.dn[111][29]);
        let eq17_e550_d_b0: f64 = (p.p342 * s.db[111][0]);
        let eq17_e550_d_b1: f64 = (p.p342 * s.db[111][1]);
        let eq17_e550_d_b2: f64 = (p.p342 * s.db[111][2]);
        let eq17_e550_d_b3: f64 = (p.p342 * s.db[111][3]);
        let eq17_e550_d_b4: f64 = (p.p342 * s.db[111][4]);
        let eq17_e550_d_b5: f64 = (p.p342 * s.db[111][5]);
        let eq17_e550_d_b6: f64 = (p.p342 * s.db[111][6]);
        let eq17_e550_d_b7: f64 = (p.p342 * s.db[111][7]);
        let eq17_e550_d_b8: f64 = (p.p342 * s.db[111][8]);
        let eq17_e550_d_b9: f64 = (p.p342 * s.db[111][9]);
        let eq17_e550_d_b10: f64 = (p.p342 * s.db[111][10]);
        let eq17_e550_d_b11: f64 = (p.p342 * s.db[111][11]);
        let eq17_e550_d_b12: f64 = (p.p342 * s.db[111][12]);
        let eq17_e550_d_b13: f64 = (p.p342 * s.db[111][13]);
        let eq17_e550_d_b14: f64 = (p.p342 * s.db[111][14]);
        let eq17_e550_d_b15: f64 = (p.p342 * s.db[111][15]);
        let eq17_e550_d_b16: f64 = (p.p342 * s.db[111][16]);
        let eq17_e550_d_b17: f64 = (p.p342 * s.db[111][17]);
        let eq17_e550_d_b18: f64 = (p.p342 * s.db[111][18]);
        let eq17_e550_d_b19: f64 = (p.p342 * s.db[111][19]);
        let eq17_e550_d_b20: f64 = (p.p342 * s.db[111][20]);
        let eq17_e550_d_b21: f64 = (p.p342 * s.db[111][21]);
        let eq17_e550_d_b22: f64 = (p.p342 * s.db[111][22]);
        let eq17_e550_d_b23: f64 = (p.p342 * s.db[111][23]);
        let eq17_e550_d_b24: f64 = (p.p342 * s.db[111][24]);
        let eq17_e550_d_b25: f64 = (p.p342 * s.db[111][25]);
        let eq17_e550_d_b26: f64 = (p.p342 * s.db[111][26]);
        let eq17_e550_d_b27: f64 = (p.p342 * s.db[111][27]);
        let eq17_e550_d_b28: f64 = (p.p342 * s.db[111][28]);
        let eq17_e550_d_b29: f64 = (p.p342 * s.db[111][29]);
        let eq17_e550_d_b30: f64 = (p.p342 * s.db[111][30]);
        let eq17_e550_d_b31: f64 = (p.p342 * s.db[111][31]);
        let eq17_e550_d_b32: f64 = (p.p342 * s.db[111][32]);
        let eq17_e550_d_b33: f64 = (p.p342 * s.db[111][33]);
        let eq17_e550_d_b34: f64 = (p.p342 * s.db[111][34]);
        let eq17_e550_d_b35: f64 = (p.p342 * s.db[111][35]);
        let eq17_e551: f64 = (1.0 + eq17_e550);
        let eq17_e555: f64 = (s.v[111] - s.v[109]);
        let eq17_e556: f64 = (p.p344 * eq17_e555);
        let eq17_e556_d_n0: f64 = (p.p344 * s.dn[111][0]);
        let eq17_e556_d_n1: f64 = (p.p344 * s.dn[111][1]);
        let eq17_e556_d_n2: f64 = (p.p344 * s.dn[111][2]);
        let eq17_e556_d_n3: f64 = (p.p344 * s.dn[111][3]);
        let eq17_e556_d_n4: f64 = (p.p344 * s.dn[111][4]);
        let eq17_e556_d_n5: f64 = (p.p344 * s.dn[111][5]);
        let eq17_e556_d_n6: f64 = (p.p344 * s.dn[111][6]);
        let eq17_e556_d_n7: f64 = (p.p344 * s.dn[111][7]);
        let eq17_e556_d_n8: f64 = (p.p344 * s.dn[111][8]);
        let eq17_e556_d_n9: f64 = (p.p344 * s.dn[111][9]);
        let eq17_e556_d_n10: f64 = (p.p344 * s.dn[111][10]);
        let eq17_e556_d_n11: f64 = (p.p344 * s.dn[111][11]);
        let eq17_e556_d_n12: f64 = (p.p344 * s.dn[111][12]);
        let eq17_e556_d_n13: f64 = (p.p344 * s.dn[111][13]);
        let eq17_e556_d_n14: f64 = (p.p344 * s.dn[111][14]);
        let eq17_e556_d_n15: f64 = (p.p344 * s.dn[111][15]);
        let eq17_e556_d_n16: f64 = (p.p344 * s.dn[111][16]);
        let eq17_e556_d_n17: f64 = (p.p344 * s.dn[111][17]);
        let eq17_e556_d_n18: f64 = (p.p344 * s.dn[111][18]);
        let eq17_e556_d_n19: f64 = (p.p344 * s.dn[111][19]);
        let eq17_e556_d_n20: f64 = (p.p344 * s.dn[111][20]);
        let eq17_e556_d_n21: f64 = (p.p344 * s.dn[111][21]);
        let eq17_e556_d_n22: f64 = (p.p344 * s.dn[111][22]);
        let eq17_e556_d_n23: f64 = (p.p344 * s.dn[111][23]);
        let eq17_e556_d_n24: f64 = (p.p344 * s.dn[111][24]);
        let eq17_e556_d_n25: f64 = (p.p344 * s.dn[111][25]);
        let eq17_e556_d_n26: f64 = (p.p344 * s.dn[111][26]);
        let eq17_e556_d_n27: f64 = (p.p344 * s.dn[111][27]);
        let eq17_e556_d_n28: f64 = (p.p344 * s.dn[111][28]);
        let eq17_e556_d_n29: f64 = (p.p344 * s.dn[111][29]);
        let eq17_e556_d_b0: f64 = (p.p344 * s.db[111][0]);
        let eq17_e556_d_b1: f64 = (p.p344 * s.db[111][1]);
        let eq17_e556_d_b2: f64 = (p.p344 * s.db[111][2]);
        let eq17_e556_d_b3: f64 = (p.p344 * s.db[111][3]);
        let eq17_e556_d_b4: f64 = (p.p344 * s.db[111][4]);
        let eq17_e556_d_b5: f64 = (p.p344 * s.db[111][5]);
        let eq17_e556_d_b6: f64 = (p.p344 * s.db[111][6]);
        let eq17_e556_d_b7: f64 = (p.p344 * s.db[111][7]);
        let eq17_e556_d_b8: f64 = (p.p344 * s.db[111][8]);
        let eq17_e556_d_b9: f64 = (p.p344 * s.db[111][9]);
        let eq17_e556_d_b10: f64 = (p.p344 * s.db[111][10]);
        let eq17_e556_d_b11: f64 = (p.p344 * s.db[111][11]);
        let eq17_e556_d_b12: f64 = (p.p344 * s.db[111][12]);
        let eq17_e556_d_b13: f64 = (p.p344 * s.db[111][13]);
        let eq17_e556_d_b14: f64 = (p.p344 * s.db[111][14]);
        let eq17_e556_d_b15: f64 = (p.p344 * s.db[111][15]);
        let eq17_e556_d_b16: f64 = (p.p344 * s.db[111][16]);
        let eq17_e556_d_b17: f64 = (p.p344 * s.db[111][17]);
        let eq17_e556_d_b18: f64 = (p.p344 * s.db[111][18]);
        let eq17_e556_d_b19: f64 = (p.p344 * s.db[111][19]);
        let eq17_e556_d_b20: f64 = (p.p344 * s.db[111][20]);
        let eq17_e556_d_b21: f64 = (p.p344 * s.db[111][21]);
        let eq17_e556_d_b22: f64 = (p.p344 * s.db[111][22]);
        let eq17_e556_d_b23: f64 = (p.p344 * s.db[111][23]);
        let eq17_e556_d_b24: f64 = (p.p344 * s.db[111][24]);
        let eq17_e556_d_b25: f64 = (p.p344 * s.db[111][25]);
        let eq17_e556_d_b26: f64 = (p.p344 * s.db[111][26]);
        let eq17_e556_d_b27: f64 = (p.p344 * s.db[111][27]);
        let eq17_e556_d_b28: f64 = (p.p344 * s.db[111][28]);
        let eq17_e556_d_b29: f64 = (p.p344 * s.db[111][29]);
        let eq17_e556_d_b30: f64 = (p.p344 * s.db[111][30]);
        let eq17_e556_d_b31: f64 = (p.p344 * s.db[111][31]);
        let eq17_e556_d_b32: f64 = (p.p344 * s.db[111][32]);
        let eq17_e556_d_b33: f64 = (p.p344 * s.db[111][33]);
        let eq17_e556_d_b34: f64 = (p.p344 * s.db[111][34]);
        let eq17_e556_d_b35: f64 = (p.p344 * s.db[111][35]);
        let eq17_e559: f64 = (s.v[111] - s.v[109]);
        let eq17_e560: f64 = (eq17_e556 * eq17_e559);
        let eq17_e560_d_n0: f64 = ((eq17_e556_d_n0 * eq17_e559) + (eq17_e556 * s.dn[111][0]));
        let eq17_e560_d_n1: f64 = ((eq17_e556_d_n1 * eq17_e559) + (eq17_e556 * s.dn[111][1]));
        let eq17_e560_d_n2: f64 = ((eq17_e556_d_n2 * eq17_e559) + (eq17_e556 * s.dn[111][2]));
        let eq17_e560_d_n3: f64 = ((eq17_e556_d_n3 * eq17_e559) + (eq17_e556 * s.dn[111][3]));
        let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * s.dn[111][4]));
        let eq17_e560_d_n5: f64 = ((eq17_e556_d_n5 * eq17_e559) + (eq17_e556 * s.dn[111][5]));
        let eq17_e560_d_n6: f64 = ((eq17_e556_d_n6 * eq17_e559) + (eq17_e556 * s.dn[111][6]));
        let eq17_e560_d_n7: f64 = ((eq17_e556_d_n7 * eq17_e559) + (eq17_e556 * s.dn[111][7]));
        let eq17_e560_d_n8: f64 = ((eq17_e556_d_n8 * eq17_e559) + (eq17_e556 * s.dn[111][8]));
        let eq17_e560_d_n9: f64 = ((eq17_e556_d_n9 * eq17_e559) + (eq17_e556 * s.dn[111][9]));
        let eq17_e560_d_n10: f64 = ((eq17_e556_d_n10 * eq17_e559) + (eq17_e556 * s.dn[111][10]));
        let eq17_e560_d_n11: f64 = ((eq17_e556_d_n11 * eq17_e559) + (eq17_e556 * s.dn[111][11]));
        let eq17_e560_d_n12: f64 = ((eq17_e556_d_n12 * eq17_e559) + (eq17_e556 * s.dn[111][12]));
        let eq17_e560_d_n13: f64 = ((eq17_e556_d_n13 * eq17_e559) + (eq17_e556 * s.dn[111][13]));
        let eq17_e560_d_n14: f64 = ((eq17_e556_d_n14 * eq17_e559) + (eq17_e556 * s.dn[111][14]));
        let eq17_e560_d_n15: f64 = ((eq17_e556_d_n15 * eq17_e559) + (eq17_e556 * s.dn[111][15]));
        let eq17_e560_d_n16: f64 = ((eq17_e556_d_n16 * eq17_e559) + (eq17_e556 * s.dn[111][16]));
        let eq17_e560_d_n17: f64 = ((eq17_e556_d_n17 * eq17_e559) + (eq17_e556 * s.dn[111][17]));
        let eq17_e560_d_n18: f64 = ((eq17_e556_d_n18 * eq17_e559) + (eq17_e556 * s.dn[111][18]));
        let eq17_e560_d_n19: f64 = ((eq17_e556_d_n19 * eq17_e559) + (eq17_e556 * s.dn[111][19]));
        let eq17_e560_d_n20: f64 = ((eq17_e556_d_n20 * eq17_e559) + (eq17_e556 * s.dn[111][20]));
        let eq17_e560_d_n21: f64 = ((eq17_e556_d_n21 * eq17_e559) + (eq17_e556 * s.dn[111][21]));
        let eq17_e560_d_n22: f64 = ((eq17_e556_d_n22 * eq17_e559) + (eq17_e556 * s.dn[111][22]));
        let eq17_e560_d_n23: f64 = ((eq17_e556_d_n23 * eq17_e559) + (eq17_e556 * s.dn[111][23]));
        let eq17_e560_d_n24: f64 = ((eq17_e556_d_n24 * eq17_e559) + (eq17_e556 * s.dn[111][24]));
        let eq17_e560_d_n25: f64 = ((eq17_e556_d_n25 * eq17_e559) + (eq17_e556 * s.dn[111][25]));
        let eq17_e560_d_n26: f64 = ((eq17_e556_d_n26 * eq17_e559) + (eq17_e556 * s.dn[111][26]));
        let eq17_e560_d_n27: f64 = ((eq17_e556_d_n27 * eq17_e559) + (eq17_e556 * s.dn[111][27]));
        let eq17_e560_d_n28: f64 = ((eq17_e556_d_n28 * eq17_e559) + (eq17_e556 * s.dn[111][28]));
        let eq17_e560_d_n29: f64 = ((eq17_e556_d_n29 * eq17_e559) + (eq17_e556 * s.dn[111][29]));
        let eq17_e560_d_b0: f64 = ((eq17_e556_d_b0 * eq17_e559) + (eq17_e556 * s.db[111][0]));
        let eq17_e560_d_b1: f64 = ((eq17_e556_d_b1 * eq17_e559) + (eq17_e556 * s.db[111][1]));
        let eq17_e560_d_b2: f64 = ((eq17_e556_d_b2 * eq17_e559) + (eq17_e556 * s.db[111][2]));
        let eq17_e560_d_b3: f64 = ((eq17_e556_d_b3 * eq17_e559) + (eq17_e556 * s.db[111][3]));
        let eq17_e560_d_b4: f64 = ((eq17_e556_d_b4 * eq17_e559) + (eq17_e556 * s.db[111][4]));
        let eq17_e560_d_b5: f64 = ((eq17_e556_d_b5 * eq17_e559) + (eq17_e556 * s.db[111][5]));
        let eq17_e560_d_b6: f64 = ((eq17_e556_d_b6 * eq17_e559) + (eq17_e556 * s.db[111][6]));
        let eq17_e560_d_b7: f64 = ((eq17_e556_d_b7 * eq17_e559) + (eq17_e556 * s.db[111][7]));
        let eq17_e560_d_b8: f64 = ((eq17_e556_d_b8 * eq17_e559) + (eq17_e556 * s.db[111][8]));
        let eq17_e560_d_b9: f64 = ((eq17_e556_d_b9 * eq17_e559) + (eq17_e556 * s.db[111][9]));
        let eq17_e560_d_b10: f64 = ((eq17_e556_d_b10 * eq17_e559) + (eq17_e556 * s.db[111][10]));
        let eq17_e560_d_b11: f64 = ((eq17_e556_d_b11 * eq17_e559) + (eq17_e556 * s.db[111][11]));
        let eq17_e560_d_b12: f64 = ((eq17_e556_d_b12 * eq17_e559) + (eq17_e556 * s.db[111][12]));
        let eq17_e560_d_b13: f64 = ((eq17_e556_d_b13 * eq17_e559) + (eq17_e556 * s.db[111][13]));
        let eq17_e560_d_b14: f64 = ((eq17_e556_d_b14 * eq17_e559) + (eq17_e556 * s.db[111][14]));
        let eq17_e560_d_b15: f64 = ((eq17_e556_d_b15 * eq17_e559) + (eq17_e556 * s.db[111][15]));
        let eq17_e560_d_b16: f64 = ((eq17_e556_d_b16 * eq17_e559) + (eq17_e556 * s.db[111][16]));
        let eq17_e560_d_b17: f64 = ((eq17_e556_d_b17 * eq17_e559) + (eq17_e556 * s.db[111][17]));
        let eq17_e560_d_b18: f64 = ((eq17_e556_d_b18 * eq17_e559) + (eq17_e556 * s.db[111][18]));
        let eq17_e560_d_b19: f64 = ((eq17_e556_d_b19 * eq17_e559) + (eq17_e556 * s.db[111][19]));
        let eq17_e560_d_b20: f64 = ((eq17_e556_d_b20 * eq17_e559) + (eq17_e556 * s.db[111][20]));
        let eq17_e560_d_b21: f64 = ((eq17_e556_d_b21 * eq17_e559) + (eq17_e556 * s.db[111][21]));
        let eq17_e560_d_b22: f64 = ((eq17_e556_d_b22 * eq17_e559) + (eq17_e556 * s.db[111][22]));
        let eq17_e560_d_b23: f64 = ((eq17_e556_d_b23 * eq17_e559) + (eq17_e556 * s.db[111][23]));
        let eq17_e560_d_b24: f64 = ((eq17_e556_d_b24 * eq17_e559) + (eq17_e556 * s.db[111][24]));
        let eq17_e560_d_b25: f64 = ((eq17_e556_d_b25 * eq17_e559) + (eq17_e556 * s.db[111][25]));
        let eq17_e560_d_b26: f64 = ((eq17_e556_d_b26 * eq17_e559) + (eq17_e556 * s.db[111][26]));
        let eq17_e560_d_b27: f64 = ((eq17_e556_d_b27 * eq17_e559) + (eq17_e556 * s.db[111][27]));
        let eq17_e560_d_b28: f64 = ((eq17_e556_d_b28 * eq17_e559) + (eq17_e556 * s.db[111][28]));
        let eq17_e560_d_b29: f64 = ((eq17_e556_d_b29 * eq17_e559) + (eq17_e556 * s.db[111][29]));
        let eq17_e560_d_b30: f64 = ((eq17_e556_d_b30 * eq17_e559) + (eq17_e556 * s.db[111][30]));
        let eq17_e560_d_b31: f64 = ((eq17_e556_d_b31 * eq17_e559) + (eq17_e556 * s.db[111][31]));
        let eq17_e560_d_b32: f64 = ((eq17_e556_d_b32 * eq17_e559) + (eq17_e556 * s.db[111][32]));
        let eq17_e560_d_b33: f64 = ((eq17_e556_d_b33 * eq17_e559) + (eq17_e556 * s.db[111][33]));
        let eq17_e560_d_b34: f64 = ((eq17_e556_d_b34 * eq17_e559) + (eq17_e556 * s.db[111][34]));
        let eq17_e560_d_b35: f64 = ((eq17_e556_d_b35 * eq17_e559) + (eq17_e556 * s.db[111][35]));
        let eq17_e561: f64 = (eq17_e551 + eq17_e560);
        let eq17_e561_d_n0: f64 = (eq17_e550_d_n0 + eq17_e560_d_n0);
        let eq17_e561_d_n1: f64 = (eq17_e550_d_n1 + eq17_e560_d_n1);
        let eq17_e561_d_n2: f64 = (eq17_e550_d_n2 + eq17_e560_d_n2);
        let eq17_e561_d_n3: f64 = (eq17_e550_d_n3 + eq17_e560_d_n3);
        let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);
        let eq17_e561_d_n5: f64 = (eq17_e550_d_n5 + eq17_e560_d_n5);
        let eq17_e561_d_n6: f64 = (eq17_e550_d_n6 + eq17_e560_d_n6);
        let eq17_e561_d_n7: f64 = (eq17_e550_d_n7 + eq17_e560_d_n7);
        let eq17_e561_d_n8: f64 = (eq17_e550_d_n8 + eq17_e560_d_n8);
        let eq17_e561_d_n9: f64 = (eq17_e550_d_n9 + eq17_e560_d_n9);
        let eq17_e561_d_n10: f64 = (eq17_e550_d_n10 + eq17_e560_d_n10);
        let eq17_e561_d_n11: f64 = (eq17_e550_d_n11 + eq17_e560_d_n11);
        let eq17_e561_d_n12: f64 = (eq17_e550_d_n12 + eq17_e560_d_n12);
        let eq17_e561_d_n13: f64 = (eq17_e550_d_n13 + eq17_e560_d_n13);
        let eq17_e561_d_n14: f64 = (eq17_e550_d_n14 + eq17_e560_d_n14);
        let eq17_e561_d_n15: f64 = (eq17_e550_d_n15 + eq17_e560_d_n15);
        let eq17_e561_d_n16: f64 = (eq17_e550_d_n16 + eq17_e560_d_n16);
        let eq17_e561_d_n17: f64 = (eq17_e550_d_n17 + eq17_e560_d_n17);
        let eq17_e561_d_n18: f64 = (eq17_e550_d_n18 + eq17_e560_d_n18);
        let eq17_e561_d_n19: f64 = (eq17_e550_d_n19 + eq17_e560_d_n19);
        let eq17_e561_d_n20: f64 = (eq17_e550_d_n20 + eq17_e560_d_n20);
        let eq17_e561_d_n21: f64 = (eq17_e550_d_n21 + eq17_e560_d_n21);
        let eq17_e561_d_n22: f64 = (eq17_e550_d_n22 + eq17_e560_d_n22);
        let eq17_e561_d_n23: f64 = (eq17_e550_d_n23 + eq17_e560_d_n23);
        let eq17_e561_d_n24: f64 = (eq17_e550_d_n24 + eq17_e560_d_n24);
        let eq17_e561_d_n25: f64 = (eq17_e550_d_n25 + eq17_e560_d_n25);
        let eq17_e561_d_n26: f64 = (eq17_e550_d_n26 + eq17_e560_d_n26);
        let eq17_e561_d_n27: f64 = (eq17_e550_d_n27 + eq17_e560_d_n27);
        let eq17_e561_d_n28: f64 = (eq17_e550_d_n28 + eq17_e560_d_n28);
        let eq17_e561_d_n29: f64 = (eq17_e550_d_n29 + eq17_e560_d_n29);
        let eq17_e561_d_b0: f64 = (eq17_e550_d_b0 + eq17_e560_d_b0);
        let eq17_e561_d_b1: f64 = (eq17_e550_d_b1 + eq17_e560_d_b1);
        let eq17_e561_d_b2: f64 = (eq17_e550_d_b2 + eq17_e560_d_b2);
        let eq17_e561_d_b3: f64 = (eq17_e550_d_b3 + eq17_e560_d_b3);
        let eq17_e561_d_b4: f64 = (eq17_e550_d_b4 + eq17_e560_d_b4);
        let eq17_e561_d_b5: f64 = (eq17_e550_d_b5 + eq17_e560_d_b5);
        let eq17_e561_d_b6: f64 = (eq17_e550_d_b6 + eq17_e560_d_b6);
        let eq17_e561_d_b7: f64 = (eq17_e550_d_b7 + eq17_e560_d_b7);
        let eq17_e561_d_b8: f64 = (eq17_e550_d_b8 + eq17_e560_d_b8);
        let eq17_e561_d_b9: f64 = (eq17_e550_d_b9 + eq17_e560_d_b9);
        let eq17_e561_d_b10: f64 = (eq17_e550_d_b10 + eq17_e560_d_b10);
        let eq17_e561_d_b11: f64 = (eq17_e550_d_b11 + eq17_e560_d_b11);
        let eq17_e561_d_b12: f64 = (eq17_e550_d_b12 + eq17_e560_d_b12);
        let eq17_e561_d_b13: f64 = (eq17_e550_d_b13 + eq17_e560_d_b13);
        let eq17_e561_d_b14: f64 = (eq17_e550_d_b14 + eq17_e560_d_b14);
        let eq17_e561_d_b15: f64 = (eq17_e550_d_b15 + eq17_e560_d_b15);
        let eq17_e561_d_b16: f64 = (eq17_e550_d_b16 + eq17_e560_d_b16);
        let eq17_e561_d_b17: f64 = (eq17_e550_d_b17 + eq17_e560_d_b17);
        let eq17_e561_d_b18: f64 = (eq17_e550_d_b18 + eq17_e560_d_b18);
        let eq17_e561_d_b19: f64 = (eq17_e550_d_b19 + eq17_e560_d_b19);
        let eq17_e561_d_b20: f64 = (eq17_e550_d_b20 + eq17_e560_d_b20);
        let eq17_e561_d_b21: f64 = (eq17_e550_d_b21 + eq17_e560_d_b21);
        let eq17_e561_d_b22: f64 = (eq17_e550_d_b22 + eq17_e560_d_b22);
        let eq17_e561_d_b23: f64 = (eq17_e550_d_b23 + eq17_e560_d_b23);
        let eq17_e561_d_b24: f64 = (eq17_e550_d_b24 + eq17_e560_d_b24);
        let eq17_e561_d_b25: f64 = (eq17_e550_d_b25 + eq17_e560_d_b25);
        let eq17_e561_d_b26: f64 = (eq17_e550_d_b26 + eq17_e560_d_b26);
        let eq17_e561_d_b27: f64 = (eq17_e550_d_b27 + eq17_e560_d_b27);
        let eq17_e561_d_b28: f64 = (eq17_e550_d_b28 + eq17_e560_d_b28);
        let eq17_e561_d_b29: f64 = (eq17_e550_d_b29 + eq17_e560_d_b29);
        let eq17_e561_d_b30: f64 = (eq17_e550_d_b30 + eq17_e560_d_b30);
        let eq17_e561_d_b31: f64 = (eq17_e550_d_b31 + eq17_e560_d_b31);
        let eq17_e561_d_b32: f64 = (eq17_e550_d_b32 + eq17_e560_d_b32);
        let eq17_e561_d_b33: f64 = (eq17_e550_d_b33 + eq17_e560_d_b33);
        let eq17_e561_d_b34: f64 = (eq17_e550_d_b34 + eq17_e560_d_b34);
        let eq17_e561_d_b35: f64 = (eq17_e550_d_b35 + eq17_e560_d_b35);
        let eq17_e562: f64 = (eq17_e544 * eq17_e561);
        let eq17_e562_d_n0: f64 = ((eq17_e544_d_n0 * eq17_e561) + (eq17_e544 * eq17_e561_d_n0));
        let eq17_e562_d_n1: f64 = ((eq17_e544_d_n1 * eq17_e561) + (eq17_e544 * eq17_e561_d_n1));
        let eq17_e562_d_n2: f64 = ((eq17_e544_d_n2 * eq17_e561) + (eq17_e544 * eq17_e561_d_n2));
        let eq17_e562_d_n3: f64 = ((eq17_e544_d_n3 * eq17_e561) + (eq17_e544 * eq17_e561_d_n3));
        let eq17_e562_d_n4: f64 = ((eq17_e544_d_n4 * eq17_e561) + (eq17_e544 * eq17_e561_d_n4));
        let eq17_e562_d_n5: f64 = ((eq17_e544_d_n5 * eq17_e561) + (eq17_e544 * eq17_e561_d_n5));
        let eq17_e562_d_n6: f64 = ((eq17_e544_d_n6 * eq17_e561) + (eq17_e544 * eq17_e561_d_n6));
        let eq17_e562_d_n7: f64 = ((eq17_e544_d_n7 * eq17_e561) + (eq17_e544 * eq17_e561_d_n7));
        let eq17_e562_d_n8: f64 = ((eq17_e544_d_n8 * eq17_e561) + (eq17_e544 * eq17_e561_d_n8));
        let eq17_e562_d_n9: f64 = ((eq17_e544_d_n9 * eq17_e561) + (eq17_e544 * eq17_e561_d_n9));
        let eq17_e562_d_n10: f64 = ((eq17_e544_d_n10 * eq17_e561) + (eq17_e544 * eq17_e561_d_n10));
        let eq17_e562_d_n11: f64 = ((eq17_e544_d_n11 * eq17_e561) + (eq17_e544 * eq17_e561_d_n11));
        let eq17_e562_d_n12: f64 = ((eq17_e544_d_n12 * eq17_e561) + (eq17_e544 * eq17_e561_d_n12));
        let eq17_e562_d_n13: f64 = ((eq17_e544_d_n13 * eq17_e561) + (eq17_e544 * eq17_e561_d_n13));
        let eq17_e562_d_n14: f64 = ((eq17_e544_d_n14 * eq17_e561) + (eq17_e544 * eq17_e561_d_n14));
        let eq17_e562_d_n15: f64 = ((eq17_e544_d_n15 * eq17_e561) + (eq17_e544 * eq17_e561_d_n15));
        let eq17_e562_d_n16: f64 = ((eq17_e544_d_n16 * eq17_e561) + (eq17_e544 * eq17_e561_d_n16));
        let eq17_e562_d_n17: f64 = ((eq17_e544_d_n17 * eq17_e561) + (eq17_e544 * eq17_e561_d_n17));
        let eq17_e562_d_n18: f64 = ((eq17_e544_d_n18 * eq17_e561) + (eq17_e544 * eq17_e561_d_n18));
        let eq17_e562_d_n19: f64 = ((eq17_e544_d_n19 * eq17_e561) + (eq17_e544 * eq17_e561_d_n19));
        let eq17_e562_d_n20: f64 = ((eq17_e544_d_n20 * eq17_e561) + (eq17_e544 * eq17_e561_d_n20));
        let eq17_e562_d_n21: f64 = ((eq17_e544_d_n21 * eq17_e561) + (eq17_e544 * eq17_e561_d_n21));
        let eq17_e562_d_n22: f64 = ((eq17_e544_d_n22 * eq17_e561) + (eq17_e544 * eq17_e561_d_n22));
        let eq17_e562_d_n23: f64 = ((eq17_e544_d_n23 * eq17_e561) + (eq17_e544 * eq17_e561_d_n23));
        let eq17_e562_d_n24: f64 = ((eq17_e544_d_n24 * eq17_e561) + (eq17_e544 * eq17_e561_d_n24));
        let eq17_e562_d_n25: f64 = ((eq17_e544_d_n25 * eq17_e561) + (eq17_e544 * eq17_e561_d_n25));
        let eq17_e562_d_n26: f64 = ((eq17_e544_d_n26 * eq17_e561) + (eq17_e544 * eq17_e561_d_n26));
        let eq17_e562_d_n27: f64 = ((eq17_e544_d_n27 * eq17_e561) + (eq17_e544 * eq17_e561_d_n27));
        let eq17_e562_d_n28: f64 = ((eq17_e544_d_n28 * eq17_e561) + (eq17_e544 * eq17_e561_d_n28));
        let eq17_e562_d_n29: f64 = ((eq17_e544_d_n29 * eq17_e561) + (eq17_e544 * eq17_e561_d_n29));
        let eq17_e562_d_b0: f64 = ((eq17_e544_d_b0 * eq17_e561) + (eq17_e544 * eq17_e561_d_b0));
        let eq17_e562_d_b1: f64 = ((eq17_e544_d_b1 * eq17_e561) + (eq17_e544 * eq17_e561_d_b1));
        let eq17_e562_d_b2: f64 = ((eq17_e544_d_b2 * eq17_e561) + (eq17_e544 * eq17_e561_d_b2));
        let eq17_e562_d_b3: f64 = ((eq17_e544_d_b3 * eq17_e561) + (eq17_e544 * eq17_e561_d_b3));
        let eq17_e562_d_b4: f64 = ((eq17_e544_d_b4 * eq17_e561) + (eq17_e544 * eq17_e561_d_b4));
        let eq17_e562_d_b5: f64 = ((eq17_e544_d_b5 * eq17_e561) + (eq17_e544 * eq17_e561_d_b5));
        let eq17_e562_d_b6: f64 = ((eq17_e544_d_b6 * eq17_e561) + (eq17_e544 * eq17_e561_d_b6));
        let eq17_e562_d_b7: f64 = ((eq17_e544_d_b7 * eq17_e561) + (eq17_e544 * eq17_e561_d_b7));
        let eq17_e562_d_b8: f64 = ((eq17_e544_d_b8 * eq17_e561) + (eq17_e544 * eq17_e561_d_b8));
        let eq17_e562_d_b9: f64 = ((eq17_e544_d_b9 * eq17_e561) + (eq17_e544 * eq17_e561_d_b9));
        let eq17_e562_d_b10: f64 = ((eq17_e544_d_b10 * eq17_e561) + (eq17_e544 * eq17_e561_d_b10));
        let eq17_e562_d_b11: f64 = ((eq17_e544_d_b11 * eq17_e561) + (eq17_e544 * eq17_e561_d_b11));
        let eq17_e562_d_b12: f64 = ((eq17_e544_d_b12 * eq17_e561) + (eq17_e544 * eq17_e561_d_b12));
        let eq17_e562_d_b13: f64 = ((eq17_e544_d_b13 * eq17_e561) + (eq17_e544 * eq17_e561_d_b13));
        let eq17_e562_d_b14: f64 = ((eq17_e544_d_b14 * eq17_e561) + (eq17_e544 * eq17_e561_d_b14));
        let eq17_e562_d_b15: f64 = ((eq17_e544_d_b15 * eq17_e561) + (eq17_e544 * eq17_e561_d_b15));
        let eq17_e562_d_b16: f64 = ((eq17_e544_d_b16 * eq17_e561) + (eq17_e544 * eq17_e561_d_b16));
        let eq17_e562_d_b17: f64 = ((eq17_e544_d_b17 * eq17_e561) + (eq17_e544 * eq17_e561_d_b17));
        let eq17_e562_d_b18: f64 = ((eq17_e544_d_b18 * eq17_e561) + (eq17_e544 * eq17_e561_d_b18));
        let eq17_e562_d_b19: f64 = ((eq17_e544_d_b19 * eq17_e561) + (eq17_e544 * eq17_e561_d_b19));
        let eq17_e562_d_b20: f64 = ((eq17_e544_d_b20 * eq17_e561) + (eq17_e544 * eq17_e561_d_b20));
        let eq17_e562_d_b21: f64 = ((eq17_e544_d_b21 * eq17_e561) + (eq17_e544 * eq17_e561_d_b21));
        let eq17_e562_d_b22: f64 = ((eq17_e544_d_b22 * eq17_e561) + (eq17_e544 * eq17_e561_d_b22));
        let eq17_e562_d_b23: f64 = ((eq17_e544_d_b23 * eq17_e561) + (eq17_e544 * eq17_e561_d_b23));
        let eq17_e562_d_b24: f64 = ((eq17_e544_d_b24 * eq17_e561) + (eq17_e544 * eq17_e561_d_b24));
        let eq17_e562_d_b25: f64 = ((eq17_e544_d_b25 * eq17_e561) + (eq17_e544 * eq17_e561_d_b25));
        let eq17_e562_d_b26: f64 = ((eq17_e544_d_b26 * eq17_e561) + (eq17_e544 * eq17_e561_d_b26));
        let eq17_e562_d_b27: f64 = ((eq17_e544_d_b27 * eq17_e561) + (eq17_e544 * eq17_e561_d_b27));
        let eq17_e562_d_b28: f64 = ((eq17_e544_d_b28 * eq17_e561) + (eq17_e544 * eq17_e561_d_b28));
        let eq17_e562_d_b29: f64 = ((eq17_e544_d_b29 * eq17_e561) + (eq17_e544 * eq17_e561_d_b29));
        let eq17_e562_d_b30: f64 = ((eq17_e544_d_b30 * eq17_e561) + (eq17_e544 * eq17_e561_d_b30));
        let eq17_e562_d_b31: f64 = ((eq17_e544_d_b31 * eq17_e561) + (eq17_e544 * eq17_e561_d_b31));
        let eq17_e562_d_b32: f64 = ((eq17_e544_d_b32 * eq17_e561) + (eq17_e544 * eq17_e561_d_b32));
        let eq17_e562_d_b33: f64 = ((eq17_e544_d_b33 * eq17_e561) + (eq17_e544 * eq17_e561_d_b33));
        let eq17_e562_d_b34: f64 = ((eq17_e544_d_b34 * eq17_e561) + (eq17_e544 * eq17_e561_d_b34));
        let eq17_e562_d_b35: f64 = ((eq17_e544_d_b35 * eq17_e561) + (eq17_e544 * eq17_e561_d_b35));
        (eq17_e562, eq17_e562_d_n0, eq17_e562_d_n1, eq17_e562_d_n2, eq17_e562_d_n3, eq17_e562_d_n4, eq17_e562_d_n5, eq17_e562_d_n6, eq17_e562_d_n7, eq17_e562_d_n8, eq17_e562_d_n9, eq17_e562_d_n10, eq17_e562_d_n11, eq17_e562_d_n12, eq17_e562_d_n13, eq17_e562_d_n14, eq17_e562_d_n15, eq17_e562_d_n16, eq17_e562_d_n17, eq17_e562_d_n18, eq17_e562_d_n19, eq17_e562_d_n20, eq17_e562_d_n21, eq17_e562_d_n22, eq17_e562_d_n23, eq17_e562_d_n24, eq17_e562_d_n25, eq17_e562_d_n26, eq17_e562_d_n27, eq17_e562_d_n28, eq17_e562_d_n29, eq17_e562_d_b0, eq17_e562_d_b1, eq17_e562_d_b2, eq17_e562_d_b3, eq17_e562_d_b4, eq17_e562_d_b5, eq17_e562_d_b6, eq17_e562_d_b7, eq17_e562_d_b8, eq17_e562_d_b9, eq17_e562_d_b10, eq17_e562_d_b11, eq17_e562_d_b12, eq17_e562_d_b13, eq17_e562_d_b14, eq17_e562_d_b15, eq17_e562_d_b16, eq17_e562_d_b17, eq17_e562_d_b18, eq17_e562_d_b19, eq17_e562_d_b20, eq17_e562_d_b21, eq17_e562_d_b22, eq17_e562_d_b23, eq17_e562_d_b24, eq17_e562_d_b25, eq17_e562_d_b26, eq17_e562_d_b27, eq17_e562_d_b28, eq17_e562_d_b29, eq17_e562_d_b30, eq17_e562_d_b31, eq17_e562_d_b32, eq17_e562_d_b33, eq17_e562_d_b34, eq17_e562_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e564;
        let eq17_node_derivatives: [f64; 30] = [eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29];
        let eq17_branch_derivatives: [f64; 36] = [eq17_e564_d_b0, eq17_e564_d_b1, eq17_e564_d_b2, eq17_e564_d_b3, eq17_e564_d_b4, eq17_e564_d_b5, eq17_e564_d_b6, eq17_e564_d_b7, eq17_e564_d_b8, eq17_e564_d_b9, eq17_e564_d_b10, eq17_e564_d_b11, eq17_e564_d_b12, eq17_e564_d_b13, eq17_e564_d_b14, eq17_e564_d_b15, eq17_e564_d_b16, eq17_e564_d_b17, eq17_e564_d_b18, eq17_e564_d_b19, eq17_e564_d_b20, eq17_e564_d_b21, eq17_e564_d_b22, eq17_e564_d_b23, eq17_e564_d_b24, eq17_e564_d_b25, eq17_e564_d_b26, eq17_e564_d_b27, eq17_e564_d_b28, eq17_e564_d_b29, eq17_e564_d_b30, eq17_e564_d_b31, eq17_e564_d_b32, eq17_e564_d_b33, eq17_e564_d_b34, eq17_e564_d_b35];
        stamper.stamp_current_dense_local(
            Some(23),
            None,
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e573, eq18_e573_d_n1, eq18_e573_d_n2,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq18_e571: f64 = (p.p6 * (nv1 - nv2));
        let eq18_e571_d_n1: f64 = p.p6;
        let eq18_e571_d_n2: f64 = (-p.p6);
        (eq18_e571, eq18_e571_d_n1, eq18_e571_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e573;
        stamper.stamp_potential_node2_local(
            9,
            eq18_value,
            1,
            eq18_e573_d_n1,
            2,
            eq18_e573_d_n2,
        );
    }
}
