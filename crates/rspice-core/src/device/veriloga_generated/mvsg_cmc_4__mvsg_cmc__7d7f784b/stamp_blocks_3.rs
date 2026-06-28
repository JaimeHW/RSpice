#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
            s.store_div_scaled_inputs3_mixed_iiai(813, 871, 1.0, 839, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(810), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(814, 812, 813);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(868, 871, 1.0, 869, (-1.0), 805, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(841, 771, 1.0, 837, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(840), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(842, 812, 841);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(843, 814, 1.0, 842, (-1.0), 782, 1.0);
            s.store_div(869, 843, 835);
            s.store_div_scaled_inputs_indices(818, 787, 1.0, 779, 2.302585092994046);
            s.store_scaled_mul(820, 818, 779, 2.0);
            s.store_mul(821, 782, 820);
            s.store_sub_scaled_inputs(873, 808, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(872, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 873, (-1.0), 805, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(831, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(830), (-(p.p51 * 0.1))), (-1.0), 820, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(832, 821, 831);
        }

        if s.b[761] {
            s.store_div(824, 793, 826);
            s.store_mul_div_scaled_offset_numerator_rhs(825, 792, A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0);
            s.store_div_scaled_product_indices(846, 825, 781, 1.0, 824, 1.0);
            s.store_add_scaled_product_right_ad(847, 846, (-1.0), 846, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(832), 2.0, s.ad_value(782), s.ad_value(846), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(848, A::mul_sub_from_scalar_rhs(s.ad_value(847), 1.0, s.ad_value(830)), 1.0, 820, 830, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(849, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::div(s.ad_value(772), s.ad_value(848)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(772), s.ad_value(848)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(850, 772, 849);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(851, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(852, 772, 851);
            s.store_div_scaled_inputs2_indices(872, 771, 1.0, 873, (-1.0), 805, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(822, 871, 1.0, 852, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(819), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(823, 821, 822);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(872, 871, 1.0, 873, (-1.0), 805, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(854, 771, 1.0, 850, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(853), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(855, 821, 854);
        }

        if s.b[761] {
            s.store_offset_square(856, 823, 1e-38);
            s.store_offset_mul(857, 856, 823, 1e-57);
            s.store_offset_square(858, 855, 1e-38);
            s.store_offset_mul(859, 858, 855, 1e-57);
            s.store_offset_mul(860, 823, 855, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(861, 856, (2.0 / 3.0), 858, (2.0 / 3.0), 860, (2.0 / 3.0), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19), 1.0);
            s.store_div_ad(862, A::add_scaled_inputs_products(s.ad_value(857), (2.0 * 2.0), s.ad_value(859), (3.0 * 2.0), s.ad_value(856), s.ad_value(855), (4.0 * 2.0), s.ad_value(858), s.ad_value(823), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(856), 15.0, s.ad_value(858), 15.0, s.ad_value(860), (2.0 * 15.0)));
            s.store_sub(863, 861, 862);
            s.copy_ad(864, 862);
            s.store_mul_product3_rhs(764, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), s.ad_value(803), s.ad_value(863), 1.0);
            s.store_mul_product3_rhs(765, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), s.ad_value(803), s.ad_value(864), 1.0);
        }

        s.b[899] = (s.v[773] == 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[899]) {
            s.store_div_scaled_inputs3_indices(865, 774, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
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
            s.store_div_scaled_inputs3_indices(866, 775, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
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
            s.store_div_scaled_inputs3_indices(867, 771, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
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
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[908] {
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
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1017, 919, A::tanh_scaled_input(s.ad_value(919), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1017, 919, p.p53);
                } else {
                    s.store_scalar(1017, 0.0);
                }
            }
        }

        if s.b[908] {
            s.store_sub(1018, 918, 919);
            s.store_mul(952, 938, 926);
            s.store_add_scaled_product_value_ad(954, A::div_scaled_inputs(s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046), 1.0, 937, 1017, 1.0);
            s.store_add_scaled_product_right_sub(955, 933, 1.0, 944, 924, 925, 1.0);
            s.store_pow_ad(973, A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946));
        }

        s.b[1021] = (s.v[945] != 0.0);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1021]) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.b[908] && (!s.b[1021])) {
            s.store_scalar(956, 0.0);
        }

        if s.b[908] {
            s.store_mul_add_scaled_product_rhs(953, 1017, s.ad_value(935), 1.0, s.ad_value(956), s.ad_value(936), (-1.0));
            s.store_sub(916, 955, 953);
            s.store_scaled_mul(958, 954, 926, 2.0);
            s.store_mul(959, 929, 958);
            s.store_sub_scaled_inputs(1016, 916, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1015, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1022] = (s.v[1015] > 50.0);
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1022]) {
            s.store_scalar(974, 0.0);
        }

        s.b[1023] = (s.v[1015] < (-50.0));
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[908] && (!s.b[1022])) && s.b[1023]) {
            s.store_scalar(974, 1.0);
        }

        if ((s.b[908] && (!s.b[1022])) && (!s.b[1023])) {
            s.store_div_from_scalar_offset_ad(974, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(975, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(974), (-(p.p51 * 0.1))), (-1.0), 958, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(976, 959, 975);
        }

        if s.b[908] {
            s.store_div_ad_rhs(962, 940, A::mul_offset_rhs(s.ad_value(973), A::div_scaled_product(s.ad_value(942), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(963, 939, A::div_scaled_offset_numerator(A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(948), s.ad_value(1017), 1.0, s.ad_value(928), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(943), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(980, 963, 928, 1.0, 962, 1.0);
            s.store_add_scaled_product_right_ad(981, 980, (-1.0), 980, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(976), 2.0, s.ad_value(929), s.ad_value(980), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(982, A::mul_sub_from_scalar_rhs(s.ad_value(980), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
            s.store_add_scaled_product_value_ad(917, A::mul_sub_from_scalar_rhs(s.ad_value(981), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(983, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::div(s.ad_value(919), s.ad_value(917)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(919), s.ad_value(917)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(984, 919, 983);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(985, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(986, 919, 985);
            s.store_div_scaled_inputs2_indices(1015, 918, 1.0, 1016, (-1.0), 952, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(960, 1018, 1.0, 986, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(957), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(961, 959, 960);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1015, 1018, 1.0, 1016, (-1.0), 952, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(988, 918, 1.0, 984, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(987), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(989, 959, 988);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(990, 961, 1.0, 989, (-1.0), 929, 1.0);
            s.store_div(1016, 990, 982);
            s.store_div_scaled_inputs_indices(965, 934, 1.0, 926, 2.302585092994046);
            s.store_scaled_mul(967, 965, 926, 2.0);
            s.store_mul(968, 929, 967);
            s.store_sub_scaled_inputs(1020, 955, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1019, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1020, (-1.0), 952, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(978, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(977), (-(p.p51 * 0.1))), (-1.0), 967, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(979, 968, 978);
        }

        if s.b[908] {
            s.store_div(971, 940, 973);
            s.store_mul_div_scaled_offset_numerator_rhs(972, 939, A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0);
            s.store_div_scaled_product_indices(993, 972, 928, 1.0, 971, 1.0);
            s.store_add_scaled_product_right_ad(994, 993, (-1.0), 993, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(979), 2.0, s.ad_value(929), s.ad_value(993), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(995, A::mul_sub_from_scalar_rhs(s.ad_value(994), 1.0, s.ad_value(977)), 1.0, 967, 977, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(996, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::div(s.ad_value(919), s.ad_value(995)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(919), s.ad_value(995)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(997, 919, 996);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(998, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(999, 919, 998);
            s.store_div_scaled_inputs2_indices(1019, 918, 1.0, 1020, (-1.0), 952, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(969, 1018, 1.0, 999, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(966), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(970, 968, 969);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1019, 1018, 1.0, 1020, (-1.0), 952, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1001, 918, 1.0, 997, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(1000), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1002, 968, 1001);
        }

        if s.b[908] {
            s.store_offset_square(1003, 970, 1e-38);
            s.store_offset_mul(1004, 1003, 970, 1e-57);
            s.store_offset_square(1005, 1002, 1e-38);
            s.store_offset_mul(1006, 1005, 1002, 1e-57);
            s.store_offset_mul(1007, 970, 1002, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1008, 1003, (2.0 / 3.0), 1005, (2.0 / 3.0), 1007, (2.0 / 3.0), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19), 1.0);
            s.store_div_ad(1009, A::add_scaled_inputs_products(s.ad_value(1004), (2.0 * 2.0), s.ad_value(1006), (3.0 * 2.0), s.ad_value(1003), s.ad_value(1002), (4.0 * 2.0), s.ad_value(1005), s.ad_value(970), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1003), 15.0, s.ad_value(1005), 15.0, s.ad_value(1007), (2.0 * 15.0)));
            s.store_sub(1010, 1008, 1009);
            s.copy_ad(1011, 1009);
            s.store_mul_product3_rhs(911, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), s.ad_value(950), s.ad_value(1010), 1.0);
            s.store_mul_product3_rhs(912, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), s.ad_value(950), s.ad_value(1011), 1.0);
        }

        s.b[1046] = (s.v[920] == 1.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1046]) {
            s.store_div_scaled_inputs3_indices(1012, 921, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
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
            s.store_div_scaled_inputs3_indices(1013, 922, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
            s.store_div_scaled_inputs3_indices(1014, 918, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
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
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_scalar(1164, 0.0);
            s.store_scalar(1165, 0.0);
            s.store_scalar(1166, 0.0);
            s.store_scalar(1167, 0.0);
        }

        if s.b[1055] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1164, 1066, A::tanh_scaled_input(s.ad_value(1066), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1164, 1066, p.p53);
                } else {
                    s.store_scalar(1164, 0.0);
                }
            }
        }

        if s.b[1055] {
            s.store_sub(1165, 1065, 1066);
            s.store_mul(1099, 1085, 1073);
            s.store_add_scaled_product_value_ad(1101, A::div_scaled_inputs(s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046), 1.0, 1084, 1164, 1.0);
            s.store_add_scaled_product_right_sub(1102, 1080, 1.0, 1091, 1071, 1072, 1.0);
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
            s.store_mul_add_scaled_product_rhs(1100, 1164, s.ad_value(1082), 1.0, s.ad_value(1103), s.ad_value(1083), (-1.0));
            s.store_sub(1063, 1102, 1100);
            s.store_scaled_mul(1105, 1101, 1073, 2.0);
            s.store_mul(1106, 1076, 1105);
            s.store_sub_scaled_inputs(1163, 1063, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1162, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1163, (-1.0), 1099, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(1122, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1121), (-(p.p51 * 0.1))), (-1.0), 1105, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1123, 1106, 1122);
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1109, 1087, A::mul_offset_rhs(s.ad_value(1120), A::div_scaled_product(s.ad_value(1089), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1110, 1086, A::div_scaled_offset_numerator(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1095), s.ad_value(1164), 1.0, s.ad_value(1075), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1090), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1127, 1110, 1075, 1.0, 1109, 1.0);
            s.store_add_scaled_product_right_ad(1128, 1127, (-1.0), 1127, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1123), 2.0, s.ad_value(1076), s.ad_value(1127), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1129, A::mul_sub_from_scalar_rhs(s.ad_value(1127), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
            s.store_add_scaled_product_value_ad(1064, A::mul_sub_from_scalar_rhs(s.ad_value(1128), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1130, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::div(s.ad_value(1066), s.ad_value(1064)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1066), s.ad_value(1064)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1131, 1066, 1130);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1132, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1133, 1066, 1132);
            s.store_div_scaled_inputs2_indices(1162, 1065, 1.0, 1163, (-1.0), 1099, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1107, 1165, 1.0, 1133, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1104), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1108, 1106, 1107);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1162, 1165, 1.0, 1163, (-1.0), 1099, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1135, 1065, 1.0, 1131, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1134), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1136, 1106, 1135);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1137, 1108, 1.0, 1136, (-1.0), 1076, 1.0);
            s.store_div(1163, 1137, 1129);
            s.store_div_scaled_inputs_indices(1112, 1081, 1.0, 1073, 2.302585092994046);
            s.store_scaled_mul(1114, 1112, 1073, 2.0);
            s.store_mul(1115, 1076, 1114);
            s.store_sub_scaled_inputs(1167, 1102, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1166, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1167, (-1.0), 1099, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(1125, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1124), (-(p.p51 * 0.1))), (-1.0), 1114, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1126, 1115, 1125);
        }

        if s.b[1055] {
            s.store_div(1118, 1087, 1120);
            s.store_mul_div_scaled_offset_numerator_rhs(1119, 1086, A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1140, 1119, 1075, 1.0, 1118, 1.0);
            s.store_add_scaled_product_right_ad(1141, 1140, (-1.0), 1140, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1126), 2.0, s.ad_value(1076), s.ad_value(1140), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1142, A::mul_sub_from_scalar_rhs(s.ad_value(1141), 1.0, s.ad_value(1124)), 1.0, 1114, 1124, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1143, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::div(s.ad_value(1066), s.ad_value(1142)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1066), s.ad_value(1142)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1144, 1066, 1143);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1145, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1146, 1066, 1145);
            s.store_div_scaled_inputs2_indices(1166, 1065, 1.0, 1167, (-1.0), 1099, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1116, 1165, 1.0, 1146, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1113), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1117, 1115, 1116);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1166, 1165, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1189] = (s.v[1166] > 50.0);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1189]) {
            s.store_scalar(1147, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1190] = (s.v[1166] < (-50.0));
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1189])) && s.b[1190]) {
            s.store_scalar(1147, 1.0);
        }

        if ((s.b[1055] && (!s.b[1189])) && (!s.b[1190])) {
            s.store_div_from_scalar_offset_ad(1147, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1148, 1065, 1.0, 1144, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1147), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1149, 1115, 1148);
        }

        if s.b[1055] {
            s.store_offset_square(1150, 1117, 1e-38);
            s.store_offset_mul(1151, 1150, 1117, 1e-57);
            s.store_offset_square(1152, 1149, 1e-38);
            s.store_offset_mul(1153, 1152, 1149, 1e-57);
            s.store_offset_mul(1154, 1117, 1149, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1155, 1150, (2.0 / 3.0), 1152, (2.0 / 3.0), 1154, (2.0 / 3.0), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19), 1.0);
            s.store_div_ad(1156, A::add_scaled_inputs_products(s.ad_value(1151), (2.0 * 2.0), s.ad_value(1153), (3.0 * 2.0), s.ad_value(1150), s.ad_value(1149), (4.0 * 2.0), s.ad_value(1152), s.ad_value(1117), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1150), 15.0, s.ad_value(1152), 15.0, s.ad_value(1154), (2.0 * 15.0)));
            s.store_sub(1157, 1155, 1156);
            s.copy_ad(1158, 1156);
            s.store_mul_product3_rhs(1058, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), s.ad_value(1097), s.ad_value(1157), 1.0);
            s.store_mul_product3_rhs(1059, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), s.ad_value(1097), s.ad_value(1158), 1.0);
        }

        s.b[1193] = (s.v[1067] == 1.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1193]) {
            s.store_div_scaled_inputs3_indices(1159, 1068, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
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
            s.store_div_scaled_inputs3_indices(1160, 1069, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
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
            s.store_div_scaled_inputs3_indices(1161, 1065, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
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
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
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
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1311, 1213, A::tanh_scaled_input(s.ad_value(1213), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1311, 1213, p.p53);
                } else {
                    s.store_scalar(1311, 0.0);
                }
            }
        }

        if s.b[1202] {
            s.store_sub(1312, 1212, 1213);
            s.store_mul(1246, 1232, 1220);
            s.store_add_scaled_product_value_ad(1248, A::div_scaled_inputs(s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046), 1.0, 1231, 1311, 1.0);
            s.store_add_scaled_product_right_sub(1249, 1227, 1.0, 1238, 1218, 1219, 1.0);
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
            s.store_mul_add_scaled_product_rhs(1247, 1311, s.ad_value(1229), 1.0, s.ad_value(1250), s.ad_value(1230), (-1.0));
            s.store_sub(1210, 1249, 1247);
            s.store_scaled_mul(1252, 1248, 1220, 2.0);
            s.store_mul(1253, 1223, 1252);
            s.store_sub_scaled_inputs(1310, 1210, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1309, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1310, (-1.0), 1246, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(1269, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1268), (-(p.p51 * 0.1))), (-1.0), 1252, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1270, 1253, 1269);
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1256, 1234, A::mul_offset_rhs(s.ad_value(1267), A::div_scaled_product(s.ad_value(1236), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1257, 1233, A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1242), s.ad_value(1311), 1.0, s.ad_value(1222), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1237), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1274, 1257, 1222, 1.0, 1256, 1.0);
            s.store_add_scaled_product_right_ad(1275, 1274, (-1.0), 1274, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1270), 2.0, s.ad_value(1223), s.ad_value(1274), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1276, A::mul_sub_from_scalar_rhs(s.ad_value(1274), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
            s.store_add_scaled_product_value_ad(1211, A::mul_sub_from_scalar_rhs(s.ad_value(1275), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1277, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::div(s.ad_value(1213), s.ad_value(1211)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1213), s.ad_value(1211)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1278, 1213, 1277);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1279, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1280, 1213, 1279);
            s.store_div_scaled_inputs2_indices(1309, 1212, 1.0, 1310, (-1.0), 1246, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1254, 1312, 1.0, 1280, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1251), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1255, 1253, 1254);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1309, 1312, 1.0, 1310, (-1.0), 1246, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1282, 1212, 1.0, 1278, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1281), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1283, 1253, 1282);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1284, 1255, 1.0, 1283, (-1.0), 1223, 1.0);
            s.store_div(1310, 1284, 1276);
            s.store_div_scaled_inputs_indices(1259, 1228, 1.0, 1220, 2.302585092994046);
            s.store_scaled_mul(1261, 1259, 1220, 2.0);
            s.store_mul(1262, 1223, 1261);
            s.store_sub_scaled_inputs(1314, 1249, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1313, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1314, (-1.0), 1246, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(1272, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1271), (-(p.p51 * 0.1))), (-1.0), 1261, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1273, 1262, 1272);
        }

        if s.b[1202] {
            s.store_div(1265, 1234, 1267);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_mul_div_scaled_offset_numerator_rhs(1266, 1233, A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1287, 1266, 1222, 1.0, 1265, 1.0);
            s.store_add_scaled_product_right_ad(1288, 1287, (-1.0), 1287, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1273), 2.0, s.ad_value(1223), s.ad_value(1287), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1289, A::mul_sub_from_scalar_rhs(s.ad_value(1288), 1.0, s.ad_value(1271)), 1.0, 1261, 1271, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1290, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::div(s.ad_value(1213), s.ad_value(1289)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1213), s.ad_value(1289)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1291, 1213, 1290);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1292, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1293, 1213, 1292);
            s.store_div_scaled_inputs2_indices(1313, 1212, 1.0, 1314, (-1.0), 1246, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1263, 1312, 1.0, 1293, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1260), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1264, 1262, 1263);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1313, 1312, 1.0, 1314, (-1.0), 1246, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1295, 1212, 1.0, 1291, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1294), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1296, 1262, 1295);
        }

        if s.b[1202] {
            s.store_offset_square(1297, 1264, 1e-38);
            s.store_offset_mul(1298, 1297, 1264, 1e-57);
            s.store_offset_square(1299, 1296, 1e-38);
            s.store_offset_mul(1300, 1299, 1296, 1e-57);
            s.store_offset_mul(1301, 1264, 1296, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1302, 1297, (2.0 / 3.0), 1299, (2.0 / 3.0), 1301, (2.0 / 3.0), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19), 1.0);
            s.store_div_ad(1303, A::add_scaled_inputs_products(s.ad_value(1298), (2.0 * 2.0), s.ad_value(1300), (3.0 * 2.0), s.ad_value(1297), s.ad_value(1296), (4.0 * 2.0), s.ad_value(1299), s.ad_value(1264), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1297), 15.0, s.ad_value(1299), 15.0, s.ad_value(1301), (2.0 * 15.0)));
            s.store_sub(1304, 1302, 1303);
            s.copy_ad(1305, 1303);
            s.store_mul_product3_rhs(1205, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), s.ad_value(1244), s.ad_value(1304), 1.0);
            s.store_mul_product3_rhs(1206, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), s.ad_value(1244), s.ad_value(1305), 1.0);
        }

        s.b[1340] = (s.v[1214] == 1.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1340]) {
            s.store_div_scaled_inputs3_indices(1306, 1215, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
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
            s.store_div_scaled_inputs3_indices(1307, 1216, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
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
            s.store_div_scaled_inputs3_indices(1308, 1212, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
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
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
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
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1458, 1360, A::tanh_scaled_input(s.ad_value(1360), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1458, 1360, p.p53);
                } else {
                    s.store_scalar(1458, 0.0);
                }
            }
        }

        if s.b[1349] {
            s.store_sub(1459, 1359, 1360);
            s.store_mul(1393, 1379, 1367);
            s.store_add_scaled_product_value_ad(1395, A::div_scaled_inputs(s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046), 1.0, 1378, 1458, 1.0);
            s.store_add_scaled_product_right_sub(1396, 1374, 1.0, 1385, 1365, 1366, 1.0);
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
            s.store_mul_add_scaled_product_rhs(1394, 1458, s.ad_value(1376), 1.0, s.ad_value(1397), s.ad_value(1377), (-1.0));
            s.store_sub(1357, 1396, 1394);
            s.store_scaled_mul(1399, 1395, 1367, 2.0);
            s.store_mul(1400, 1370, 1399);
            s.store_sub_scaled_inputs(1457, 1357, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aii(1456, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1457, (-1.0), 1393, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(1416, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1415), (-(p.p51 * 0.1))), (-1.0), 1399, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1417, 1400, 1416);
        }

        if s.b[1349] {
            s.store_div_ad_rhs(1403, 1381, A::mul_offset_rhs(s.ad_value(1414), A::div_scaled_product(s.ad_value(1383), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1404, 1380, A::div_scaled_offset_numerator(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1389), s.ad_value(1458), 1.0, s.ad_value(1369), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1384), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1421, 1404, 1369, 1.0, 1403, 1.0);
            s.store_add_scaled_product_right_ad(1422, 1421, (-1.0), 1421, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1417), 2.0, s.ad_value(1370), s.ad_value(1421), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1423, A::mul_sub_from_scalar_rhs(s.ad_value(1421), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
            s.store_add_scaled_product_value_ad(1358, A::mul_sub_from_scalar_rhs(s.ad_value(1422), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1424, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::div(s.ad_value(1360), s.ad_value(1358)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1360), s.ad_value(1358)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1425, 1360, 1424);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1426, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1427, 1360, 1426);
            s.store_div_scaled_inputs2_indices(1456, 1359, 1.0, 1457, (-1.0), 1393, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1401, 1459, 1.0, 1427, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1398), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1402, 1400, 1401);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1456, 1459, 1.0, 1457, (-1.0), 1393, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1429, 1359, 1.0, 1425, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1428), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1473] = (s.v[1429] > 50.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1473]) {
            s.store_mul(1430, 1400, 1429);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1474] = (s.v[1429] < (-50.0));
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((s.b[1349] && (!s.b[1473])) && s.b[1474]) {
            s.store_mul_exp_rhs(1430, 1400, 1429);
        }

        if ((s.b[1349] && (!s.b[1473])) && (!s.b[1474])) {
            s.store_mul_ln_one_plus_exp_rhs(1430, 1400, 1429);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1431, 1402, 1.0, 1430, (-1.0), 1370, 1.0);
            s.store_div(1457, 1431, 1423);
            s.store_div_scaled_inputs_indices(1406, 1375, 1.0, 1367, 2.302585092994046);
            s.store_scaled_mul(1408, 1406, 1367, 2.0);
            s.store_mul(1409, 1370, 1408);
            s.store_sub_scaled_inputs(1461, 1396, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aii(1460, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1461, (-1.0), 1393, 1.0);
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
            s.store_div_scaled_inputs2_mixed_aai(1419, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1418), (-(p.p51 * 0.1))), (-1.0), 1408, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1420, 1409, 1419);
        }

        if s.b[1349] {
            s.store_div(1412, 1381, 1414);
            s.store_mul_div_scaled_offset_numerator_rhs(1413, 1380, A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1434, 1413, 1369, 1.0, 1412, 1.0);
            s.store_add_scaled_product_right_ad(1435, 1434, (-1.0), 1434, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1420), 2.0, s.ad_value(1370), s.ad_value(1434), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1436, A::mul_sub_from_scalar_rhs(s.ad_value(1435), 1.0, s.ad_value(1418)), 1.0, 1408, 1418, 1.0);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1437, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::div(s.ad_value(1360), s.ad_value(1436)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1360), s.ad_value(1436)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1438, 1360, 1437);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1439, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1440, 1360, 1439);
            s.store_div_scaled_inputs2_indices(1460, 1359, 1.0, 1461, (-1.0), 1393, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1410, 1459, 1.0, 1440, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1407), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1411, 1409, 1410);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1460, 1459, 1.0, 1461, (-1.0), 1393, 1.0);
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
            s.store_div_scaled_inputs3_mixed_iiai(1442, 1359, 1.0, 1438, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1441), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
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
            s.store_mul_ln_one_plus_exp_rhs(1443, 1409, 1442);
        }

        if s.b[1349] {
            s.store_offset_square(1444, 1411, 1e-38);
            s.store_offset_mul(1445, 1444, 1411, 1e-57);
            s.store_offset_square(1446, 1443, 1e-38);
            s.store_offset_mul(1447, 1446, 1443, 1e-57);
            s.store_offset_mul(1448, 1411, 1443, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1449, 1444, (2.0 / 3.0), 1446, (2.0 / 3.0), 1448, (2.0 / 3.0), A::offset(A::add(s.ad_value(1411), s.ad_value(1443)), 2e-19), 1.0);
            s.store_div_ad(1450, A::add_scaled_inputs_products(s.ad_value(1445), (2.0 * 2.0), s.ad_value(1447), (3.0 * 2.0), s.ad_value(1444), s.ad_value(1443), (4.0 * 2.0), s.ad_value(1446), s.ad_value(1411), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1444), 15.0, s.ad_value(1446), 15.0, s.ad_value(1448), (2.0 * 15.0)));
            s.store_sub(1451, 1449, 1450);
            s.copy_ad(1452, 1450);
            s.store_mul_product3_rhs(1352, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), s.ad_value(1391), s.ad_value(1451), 1.0);
            s.store_mul_product3_rhs(1353, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), s.ad_value(1391), s.ad_value(1452), 1.0);
        }

        s.b[1487] = (s.v[1361] == 1.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1349] && s.b[1487]) {
            s.store_div_scaled_inputs3_indices(1453, 1362, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
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
            s.store_div_scaled_inputs3_indices(1454, 1363, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
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
            s.store_div_scaled_inputs3_indices(1455, 1359, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
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

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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

        s.store_powf_scaled_input(1852, 1803, 1.0 / (s.v[1804]), s.v[1825]);

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

        s.store_div_scaled_inputs2_mixed_aii(1894, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 1895, (-1.0), 1831, 1.0);

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

        s.store_div_scaled_inputs2_mixed_aai(1854, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1853), (-(p.p51 * 0.1))), (-1.0), 1837, 1.0);

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
            s.store_mul_ln_one_plus_exp_rhs(1855, 1838, 1854);
        }

        s.store_div_from_scalar_ad(1841, s.v[1819], A::mul_offset_rhs(s.ad_value(1852), A::div_scaled_inputs(s.ad_value(1855), s.v[1821], s.ad_value(1808), 1.0), 1.0));

        s.store_div_scaled_value_by_product(1842, A::scale_offset(s.ad_value(1896), (s.v[1827] * 1.0 / (s.v[1807])), 1.0), (s.v[1818] * (1.0 + (s.v[1826] * s.v[1804]))), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), A::offset(A::div_scaled_inputs(s.ad_value(1855), s.v[1822], s.ad_value(1808), 1.0), 1.0), 1.0);

        s.store_add_ad(1843, A::mul3_scaled_output(s.ad_value(1853), s.ad_value(1805), s.ad_value(1841), (2.0 * 1.0 / (s.v[1807]))), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1853), s.ad_value(1842)));

        s.store_div_scaled_inputs_indices(1859, 1842, s.v[1807], 1841, 1.0);

        s.store_add_scaled_product_right_ad(1860, 1859, (-1.0), 1859, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1855), 2.0, s.ad_value(1808), s.ad_value(1859), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1861, A::mul_sub_from_scalar_rhs(s.ad_value(1859), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_add_scaled_product_value_ad(1796, A::mul_sub_from_scalar_rhs(s.ad_value(1860), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_div_from_scalar_powf_ad(1862, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::div(s.ad_value(1798), s.ad_value(1796)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1798), s.ad_value(1796)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1863, 1798, 1862);

        s.store_div_from_scalar_powf_ad(1864, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1865, 1798, 1864);

        s.store_div_scaled_inputs2_indices(1894, 1797, 1.0, 1895, (-1.0), 1831, 1.0);

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

        s.store_div_scaled_inputs3_mixed_iiai(1839, 1897, 1.0, 1865, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1836), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

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
            s.store_mul_ln_one_plus_exp_rhs(1840, 1838, 1839);
        }

        s.store_div_scaled_inputs2_indices(1894, 1897, 1.0, 1895, (-1.0), 1831, 1.0);

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

        s.store_div_scaled_inputs3_mixed_iiai(1867, 1797, 1.0, 1863, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1866), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

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
            s.store_mul_ln_one_plus_exp_rhs(1868, 1838, 1867);
        }

        s.store_div_scaled_inputs2_indices(1869, 1840, 1.0, 1868, (-1.0), 1808, 1.0);

        s.store_div(1895, 1869, 1861);

        s.store_div_ad_rhs(1870, 1895, A::powf(A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::mul(s.ad_value(1895), A::tanh_scaled_input(s.ad_value(1895), (0.001 / p.p53)))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::sqrt_square_offset(s.ad_value(1895), p.p53)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul(1871, 1843, 1870);

        s.store_mul_ad_affine_product_lhs(1789, A::add(s.ad_value(1840), s.ad_value(1868)), s.ad_value(1871), (((s.v[1829] * s.v[1806]) * s.v[1828]) * 0.5), 0.0, 1830);

        s.store_div_from_scalar_scaled_input(1844, s.v[1813], 1805, 2.302585092994046);

        s.store_scaled_mul(1846, 1844, 1805, 2.0);

        s.store_mul(1847, 1808, 1846);

        s.store_sub_scaled_inputs(1899, 1834, 1.0, 1831, (p.p51 * 0.5));

        s.store_div_scaled_inputs2_mixed_aii(1898, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1913] = (s.v[1898] > 50.0);
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if s.b[1913] {
            s.store_scalar(1856, 0.0);
        }

        s.b[1914] = (s.v[1898] < (-50.0));
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

        if ((!s.b[1913]) && s.b[1914]) {
            s.store_scalar(1856, 1.0);
        }

        if ((!s.b[1913]) && (!s.b[1914])) {
            s.store_div_from_scalar_offset_ad(1856, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs2_mixed_aai(1857, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1856), (-(p.p51 * 0.1))), (-1.0), 1846, 1.0);

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
            s.store_mul_ln_one_plus_exp_rhs(1858, 1847, 1857);
        }

        s.store_div_from_scalar(1850, s.v[1819], 1852);

        s.store_scaled_div_from_scalar_ad(1851, (1.0 + (s.v[1826] * s.v[1804])), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), s.v[1818]);

        s.store_div_scaled_inputs_indices(1872, 1851, s.v[1807], 1850, 1.0);

        s.store_add_scaled_product_right_ad(1873, 1872, (-1.0), 1872, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1858), 2.0, s.ad_value(1808), s.ad_value(1872), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1874, A::mul_sub_from_scalar_rhs(s.ad_value(1873), 1.0, s.ad_value(1856)), 1.0, 1846, 1856, 1.0);

        s.store_div_from_scalar_powf_ad(1875, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::div(s.ad_value(1798), s.ad_value(1874)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1798), s.ad_value(1874)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1876, 1798, 1875);

        s.store_div_from_scalar_powf_ad(1877, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::sqrt_square_offset(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1878, 1798, 1877);

        s.store_div_scaled_inputs2_indices(1898, 1797, 1.0, 1899, (-1.0), 1831, 1.0);

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

        s.store_div_scaled_inputs3_mixed_iiai(1848, 1897, 1.0, 1878, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1845), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

        s.b[1919] = (s.v[1848] > 50.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if s.b[1919] {
            s.store_mul(1849, 1847, 1848);
        }

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
        s.b[1920] = (s.v[1848] < (-50.0));
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((!s.b[1919]) && s.b[1920]) {
            s.store_mul_exp_rhs(1849, 1847, 1848);
        }

        if ((!s.b[1919]) && (!s.b[1920])) {
            s.store_mul_ln_one_plus_exp_rhs(1849, 1847, 1848);
        }

        s.store_div_scaled_inputs2_indices(1898, 1897, 1.0, 1899, (-1.0), 1831, 1.0);

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

        s.store_div_scaled_inputs3_mixed_iiai(1880, 1797, 1.0, 1876, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1879), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

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
            s.store_mul_ln_one_plus_exp_rhs(1881, 1847, 1880);
        }

        s.store_offset_square(1882, 1849, 1e-38);

        s.store_offset_mul(1883, 1882, 1849, 1e-57);

        s.store_offset_square(1884, 1881, 1e-38);

        s.store_offset_mul(1885, 1884, 1881, 1e-57);

        s.store_offset_mul(1886, 1849, 1881, 1e-38);

        s.store_div_scaled_inputs3_mixed_iiia(1887, 1882, (2.0 / 3.0), 1884, (2.0 / 3.0), 1886, (2.0 / 3.0), A::offset(A::add(s.ad_value(1849), s.ad_value(1881)), 2e-19), 1.0);

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
            s.store_div_scaled_inputs3_indices(1893, 1797, 1.0, 1834, -1.0, 1831, (-(-(p.p51 * 0.5))), 1846, 1.0);
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
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2675] = ((((nv6 - nv2) - p.p27) / p.p28) < (-50.0));
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if ((!s.b[2674]) && s.b[2675]) {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2674]) && (!s.b[2675])) {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2676] = ((((nv6 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if s.b[2676] {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2677] = ((((nv6 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if ((!s.b[2676]) && s.b[2677]) {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2676]) && (!s.b[2677])) {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2678] = ((((nv2 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if s.b[2678] {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2679] = ((((nv2 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((!s.b[2678]) && s.b[2679]) {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2678]) && (!s.b[2679])) {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2680] = ((((nv3 - nv2) - p.p27) / p.p28) > 50.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if s.b[2680] {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2681] = ((((nv3 - nv2) - p.p27) / p.p28) < (-50.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        if ((!s.b[2680]) && s.b[2681]) {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2680]) && (!s.b[2681])) {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2682] = ((((nv3 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if s.b[2682] {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2683] = ((((nv3 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((!s.b[2682]) && s.b[2683]) {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2682]) && (!s.b[2683])) {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2684] = ((((nv6 - nv3) - p.p27) / p.p28) > 50.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if s.b[2684] {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2685] = ((((nv6 - nv3) - p.p27) / p.p28) < (-50.0));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if ((!s.b[2684]) && s.b[2685]) {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2684]) && (!s.b[2685])) {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
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
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21,) = {
    if s.b[308] {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e418_d_n20: f64 = (-p.p330);
        let eq8_e418_d_n21: f64 = p.p330;
        let eq8_e419: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq8_e418);
        (eq8_e419, (eq8_e418_d_n20 * ddt_scale), (eq8_e418_d_n21 * ddt_scale),)
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
        (eq9_e426, (eq9_e425_d_n20 * ddt_scale),)
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
        let eq14_ad: A = {
    if ((!s.b[308]) && s.b[309]) {
        A::scaled_offset({
            if ((!(((nv24 - nv23) / s.v[113]) > 50.0)) && (!(((nv24 - nv23) / s.v[113]) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, nodes, Some(24), Some(23)), s.ad_value(113)))
            } else {
                {
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
                }
            }
        }, (-1.0), p.p346)
    } else {
        A::constant(0.0)
    }
};
        stamper.stamp_current_dense_local(
            Some(24),
            Some(23),
            multiplicity * eq14_ad.value,
            &eq14_ad.dn,
            &eq14_ad.db,
            multiplicity,
        );
        let (eq17_e564, eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29, eq17_e564_d_b0, eq17_e564_d_b1, eq17_e564_d_b2, eq17_e564_d_b3, eq17_e564_d_b4, eq17_e564_d_b5, eq17_e564_d_b6, eq17_e564_d_b7, eq17_e564_d_b8, eq17_e564_d_b9, eq17_e564_d_b10, eq17_e564_d_b11, eq17_e564_d_b12, eq17_e564_d_b13, eq17_e564_d_b14, eq17_e564_d_b15, eq17_e564_d_b16, eq17_e564_d_b17, eq17_e564_d_b18, eq17_e564_d_b19, eq17_e564_d_b20, eq17_e564_d_b21, eq17_e564_d_b22, eq17_e564_d_b23, eq17_e564_d_b24, eq17_e564_d_b25, eq17_e564_d_b26, eq17_e564_d_b27, eq17_e564_d_b28, eq17_e564_d_b29, eq17_e564_d_b30, eq17_e564_d_b31, eq17_e564_d_b32, eq17_e564_d_b33, eq17_e564_d_b34, eq17_e564_d_b35,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq17_e543: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[225]);
        let eq17_e544: f64 = (p.p341 * eq17_e543);
        let eq17_e544_d_n0: f64 = (p.p341 * (s.dn[225][0] * ddt_scale));
        let eq17_e544_d_n1: f64 = (p.p341 * (s.dn[225][1] * ddt_scale));
        let eq17_e544_d_n2: f64 = (p.p341 * (s.dn[225][2] * ddt_scale));
        let eq17_e544_d_n3: f64 = (p.p341 * (s.dn[225][3] * ddt_scale));
        let eq17_e544_d_n4: f64 = (p.p341 * (s.dn[225][4] * ddt_scale));
        let eq17_e544_d_n5: f64 = (p.p341 * (s.dn[225][5] * ddt_scale));
        let eq17_e544_d_n6: f64 = (p.p341 * (s.dn[225][6] * ddt_scale));
        let eq17_e544_d_n7: f64 = (p.p341 * (s.dn[225][7] * ddt_scale));
        let eq17_e544_d_n8: f64 = (p.p341 * (s.dn[225][8] * ddt_scale));
        let eq17_e544_d_n9: f64 = (p.p341 * (s.dn[225][9] * ddt_scale));
        let eq17_e544_d_n10: f64 = (p.p341 * (s.dn[225][10] * ddt_scale));
        let eq17_e544_d_n11: f64 = (p.p341 * (s.dn[225][11] * ddt_scale));
        let eq17_e544_d_n12: f64 = (p.p341 * (s.dn[225][12] * ddt_scale));
        let eq17_e544_d_n13: f64 = (p.p341 * (s.dn[225][13] * ddt_scale));
        let eq17_e544_d_n14: f64 = (p.p341 * (s.dn[225][14] * ddt_scale));
        let eq17_e544_d_n15: f64 = (p.p341 * (s.dn[225][15] * ddt_scale));
        let eq17_e544_d_n16: f64 = (p.p341 * (s.dn[225][16] * ddt_scale));
        let eq17_e544_d_n17: f64 = (p.p341 * (s.dn[225][17] * ddt_scale));
        let eq17_e544_d_n18: f64 = (p.p341 * (s.dn[225][18] * ddt_scale));
        let eq17_e544_d_n19: f64 = (p.p341 * (s.dn[225][19] * ddt_scale));
        let eq17_e544_d_n20: f64 = (p.p341 * (s.dn[225][20] * ddt_scale));
        let eq17_e544_d_n21: f64 = (p.p341 * (s.dn[225][21] * ddt_scale));
        let eq17_e544_d_n22: f64 = (p.p341 * (s.dn[225][22] * ddt_scale));
        let eq17_e544_d_n23: f64 = (p.p341 * (s.dn[225][23] * ddt_scale));
        let eq17_e544_d_n24: f64 = (p.p341 * (s.dn[225][24] * ddt_scale));
        let eq17_e544_d_n25: f64 = (p.p341 * (s.dn[225][25] * ddt_scale));
        let eq17_e544_d_n26: f64 = (p.p341 * (s.dn[225][26] * ddt_scale));
        let eq17_e544_d_n27: f64 = (p.p341 * (s.dn[225][27] * ddt_scale));
        let eq17_e544_d_n28: f64 = (p.p341 * (s.dn[225][28] * ddt_scale));
        let eq17_e544_d_n29: f64 = (p.p341 * (s.dn[225][29] * ddt_scale));
        let eq17_e544_d_b0: f64 = (p.p341 * (s.db[225][0] * ddt_scale));
        let eq17_e544_d_b1: f64 = (p.p341 * (s.db[225][1] * ddt_scale));
        let eq17_e544_d_b2: f64 = (p.p341 * (s.db[225][2] * ddt_scale));
        let eq17_e544_d_b3: f64 = (p.p341 * (s.db[225][3] * ddt_scale));
        let eq17_e544_d_b4: f64 = (p.p341 * (s.db[225][4] * ddt_scale));
        let eq17_e544_d_b5: f64 = (p.p341 * (s.db[225][5] * ddt_scale));
        let eq17_e544_d_b6: f64 = (p.p341 * (s.db[225][6] * ddt_scale));
        let eq17_e544_d_b7: f64 = (p.p341 * (s.db[225][7] * ddt_scale));
        let eq17_e544_d_b8: f64 = (p.p341 * (s.db[225][8] * ddt_scale));
        let eq17_e544_d_b9: f64 = (p.p341 * (s.db[225][9] * ddt_scale));
        let eq17_e544_d_b10: f64 = (p.p341 * (s.db[225][10] * ddt_scale));
        let eq17_e544_d_b11: f64 = (p.p341 * (s.db[225][11] * ddt_scale));
        let eq17_e544_d_b12: f64 = (p.p341 * (s.db[225][12] * ddt_scale));
        let eq17_e544_d_b13: f64 = (p.p341 * (s.db[225][13] * ddt_scale));
        let eq17_e544_d_b14: f64 = (p.p341 * (s.db[225][14] * ddt_scale));
        let eq17_e544_d_b15: f64 = (p.p341 * (s.db[225][15] * ddt_scale));
        let eq17_e544_d_b16: f64 = (p.p341 * (s.db[225][16] * ddt_scale));
        let eq17_e544_d_b17: f64 = (p.p341 * (s.db[225][17] * ddt_scale));
        let eq17_e544_d_b18: f64 = (p.p341 * (s.db[225][18] * ddt_scale));
        let eq17_e544_d_b19: f64 = (p.p341 * (s.db[225][19] * ddt_scale));
        let eq17_e544_d_b20: f64 = (p.p341 * (s.db[225][20] * ddt_scale));
        let eq17_e544_d_b21: f64 = (p.p341 * (s.db[225][21] * ddt_scale));
        let eq17_e544_d_b22: f64 = (p.p341 * (s.db[225][22] * ddt_scale));
        let eq17_e544_d_b23: f64 = (p.p341 * (s.db[225][23] * ddt_scale));
        let eq17_e544_d_b24: f64 = (p.p341 * (s.db[225][24] * ddt_scale));
        let eq17_e544_d_b25: f64 = (p.p341 * (s.db[225][25] * ddt_scale));
        let eq17_e544_d_b26: f64 = (p.p341 * (s.db[225][26] * ddt_scale));
        let eq17_e544_d_b27: f64 = (p.p341 * (s.db[225][27] * ddt_scale));
        let eq17_e544_d_b28: f64 = (p.p341 * (s.db[225][28] * ddt_scale));
        let eq17_e544_d_b29: f64 = (p.p341 * (s.db[225][29] * ddt_scale));
        let eq17_e544_d_b30: f64 = (p.p341 * (s.db[225][30] * ddt_scale));
        let eq17_e544_d_b31: f64 = (p.p341 * (s.db[225][31] * ddt_scale));
        let eq17_e544_d_b32: f64 = (p.p341 * (s.db[225][32] * ddt_scale));
        let eq17_e544_d_b33: f64 = (p.p341 * (s.db[225][33] * ddt_scale));
        let eq17_e544_d_b34: f64 = (p.p341 * (s.db[225][34] * ddt_scale));
        let eq17_e544_d_b35: f64 = (p.p341 * (s.db[225][35] * ddt_scale));
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let eq19_ad: A = {
    if ((!s.b[308]) && s.b[309]) {
        A::scaled_offset({
            if ((!(((nv26 - nv27) / s.v[113]) > 50.0)) && (!(((nv26 - nv27) / s.v[113]) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, nodes, Some(26), Some(27)), s.ad_value(113)))
            } else {
                {
                    if ((!(((nv26 - nv27) / s.v[113]) > 50.0)) && (((nv26 - nv27) / s.v[113]) < (-50.0))) {
                        A::exp_scaled_input(A::constant(50.0), -1.0)
                    } else {
                        {
                            if (((nv26 - nv27) / s.v[113]) > 50.0) {
                                A::scaled_offset(A::div(A::voltage(ctx, nodes, Some(26), Some(27)), s.ad_value(113)), (((-50.0)) + (1.0)), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                }
            }
        }, (-1.0), p.p346)
    } else {
        A::constant(0.0)
    }
};
        stamper.stamp_current_dense_local(
            Some(26),
            Some(27),
            multiplicity * eq19_ad.value,
            &eq19_ad.dn,
            &eq19_ad.db,
            multiplicity,
        );
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29, eq22_e682_d_b0, eq22_e682_d_b1, eq22_e682_d_b2, eq22_e682_d_b3, eq22_e682_d_b4, eq22_e682_d_b5, eq22_e682_d_b6, eq22_e682_d_b7, eq22_e682_d_b8, eq22_e682_d_b9, eq22_e682_d_b10, eq22_e682_d_b11, eq22_e682_d_b12, eq22_e682_d_b13, eq22_e682_d_b14, eq22_e682_d_b15, eq22_e682_d_b16, eq22_e682_d_b17, eq22_e682_d_b18, eq22_e682_d_b19, eq22_e682_d_b20, eq22_e682_d_b21, eq22_e682_d_b22, eq22_e682_d_b23, eq22_e682_d_b24, eq22_e682_d_b25, eq22_e682_d_b26, eq22_e682_d_b27, eq22_e682_d_b28, eq22_e682_d_b29, eq22_e682_d_b30, eq22_e682_d_b31, eq22_e682_d_b32, eq22_e682_d_b33, eq22_e682_d_b34, eq22_e682_d_b35,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq22_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[227]);
        let eq22_e662: f64 = (p.p341 * eq22_e661);
        let eq22_e662_d_n0: f64 = (p.p341 * (s.dn[227][0] * ddt_scale));
        let eq22_e662_d_n1: f64 = (p.p341 * (s.dn[227][1] * ddt_scale));
        let eq22_e662_d_n2: f64 = (p.p341 * (s.dn[227][2] * ddt_scale));
        let eq22_e662_d_n3: f64 = (p.p341 * (s.dn[227][3] * ddt_scale));
        let eq22_e662_d_n4: f64 = (p.p341 * (s.dn[227][4] * ddt_scale));
        let eq22_e662_d_n5: f64 = (p.p341 * (s.dn[227][5] * ddt_scale));
        let eq22_e662_d_n6: f64 = (p.p341 * (s.dn[227][6] * ddt_scale));
        let eq22_e662_d_n7: f64 = (p.p341 * (s.dn[227][7] * ddt_scale));
        let eq22_e662_d_n8: f64 = (p.p341 * (s.dn[227][8] * ddt_scale));
        let eq22_e662_d_n9: f64 = (p.p341 * (s.dn[227][9] * ddt_scale));
        let eq22_e662_d_n10: f64 = (p.p341 * (s.dn[227][10] * ddt_scale));
        let eq22_e662_d_n11: f64 = (p.p341 * (s.dn[227][11] * ddt_scale));
        let eq22_e662_d_n12: f64 = (p.p341 * (s.dn[227][12] * ddt_scale));
        let eq22_e662_d_n13: f64 = (p.p341 * (s.dn[227][13] * ddt_scale));
        let eq22_e662_d_n14: f64 = (p.p341 * (s.dn[227][14] * ddt_scale));
        let eq22_e662_d_n15: f64 = (p.p341 * (s.dn[227][15] * ddt_scale));
        let eq22_e662_d_n16: f64 = (p.p341 * (s.dn[227][16] * ddt_scale));
        let eq22_e662_d_n17: f64 = (p.p341 * (s.dn[227][17] * ddt_scale));
        let eq22_e662_d_n18: f64 = (p.p341 * (s.dn[227][18] * ddt_scale));
        let eq22_e662_d_n19: f64 = (p.p341 * (s.dn[227][19] * ddt_scale));
        let eq22_e662_d_n20: f64 = (p.p341 * (s.dn[227][20] * ddt_scale));
        let eq22_e662_d_n21: f64 = (p.p341 * (s.dn[227][21] * ddt_scale));
        let eq22_e662_d_n22: f64 = (p.p341 * (s.dn[227][22] * ddt_scale));
        let eq22_e662_d_n23: f64 = (p.p341 * (s.dn[227][23] * ddt_scale));
        let eq22_e662_d_n24: f64 = (p.p341 * (s.dn[227][24] * ddt_scale));
        let eq22_e662_d_n25: f64 = (p.p341 * (s.dn[227][25] * ddt_scale));
        let eq22_e662_d_n26: f64 = (p.p341 * (s.dn[227][26] * ddt_scale));
        let eq22_e662_d_n27: f64 = (p.p341 * (s.dn[227][27] * ddt_scale));
        let eq22_e662_d_n28: f64 = (p.p341 * (s.dn[227][28] * ddt_scale));
        let eq22_e662_d_n29: f64 = (p.p341 * (s.dn[227][29] * ddt_scale));
        let eq22_e662_d_b0: f64 = (p.p341 * (s.db[227][0] * ddt_scale));
        let eq22_e662_d_b1: f64 = (p.p341 * (s.db[227][1] * ddt_scale));
        let eq22_e662_d_b2: f64 = (p.p341 * (s.db[227][2] * ddt_scale));
        let eq22_e662_d_b3: f64 = (p.p341 * (s.db[227][3] * ddt_scale));
        let eq22_e662_d_b4: f64 = (p.p341 * (s.db[227][4] * ddt_scale));
        let eq22_e662_d_b5: f64 = (p.p341 * (s.db[227][5] * ddt_scale));
        let eq22_e662_d_b6: f64 = (p.p341 * (s.db[227][6] * ddt_scale));
        let eq22_e662_d_b7: f64 = (p.p341 * (s.db[227][7] * ddt_scale));
        let eq22_e662_d_b8: f64 = (p.p341 * (s.db[227][8] * ddt_scale));
        let eq22_e662_d_b9: f64 = (p.p341 * (s.db[227][9] * ddt_scale));
        let eq22_e662_d_b10: f64 = (p.p341 * (s.db[227][10] * ddt_scale));
        let eq22_e662_d_b11: f64 = (p.p341 * (s.db[227][11] * ddt_scale));
        let eq22_e662_d_b12: f64 = (p.p341 * (s.db[227][12] * ddt_scale));
        let eq22_e662_d_b13: f64 = (p.p341 * (s.db[227][13] * ddt_scale));
        let eq22_e662_d_b14: f64 = (p.p341 * (s.db[227][14] * ddt_scale));
        let eq22_e662_d_b15: f64 = (p.p341 * (s.db[227][15] * ddt_scale));
        let eq22_e662_d_b16: f64 = (p.p341 * (s.db[227][16] * ddt_scale));
        let eq22_e662_d_b17: f64 = (p.p341 * (s.db[227][17] * ddt_scale));
        let eq22_e662_d_b18: f64 = (p.p341 * (s.db[227][18] * ddt_scale));
        let eq22_e662_d_b19: f64 = (p.p341 * (s.db[227][19] * ddt_scale));
        let eq22_e662_d_b20: f64 = (p.p341 * (s.db[227][20] * ddt_scale));
        let eq22_e662_d_b21: f64 = (p.p341 * (s.db[227][21] * ddt_scale));
        let eq22_e662_d_b22: f64 = (p.p341 * (s.db[227][22] * ddt_scale));
        let eq22_e662_d_b23: f64 = (p.p341 * (s.db[227][23] * ddt_scale));
        let eq22_e662_d_b24: f64 = (p.p341 * (s.db[227][24] * ddt_scale));
        let eq22_e662_d_b25: f64 = (p.p341 * (s.db[227][25] * ddt_scale));
        let eq22_e662_d_b26: f64 = (p.p341 * (s.db[227][26] * ddt_scale));
        let eq22_e662_d_b27: f64 = (p.p341 * (s.db[227][27] * ddt_scale));
        let eq22_e662_d_b28: f64 = (p.p341 * (s.db[227][28] * ddt_scale));
        let eq22_e662_d_b29: f64 = (p.p341 * (s.db[227][29] * ddt_scale));
        let eq22_e662_d_b30: f64 = (p.p341 * (s.db[227][30] * ddt_scale));
        let eq22_e662_d_b31: f64 = (p.p341 * (s.db[227][31] * ddt_scale));
        let eq22_e662_d_b32: f64 = (p.p341 * (s.db[227][32] * ddt_scale));
        let eq22_e662_d_b33: f64 = (p.p341 * (s.db[227][33] * ddt_scale));
        let eq22_e662_d_b34: f64 = (p.p341 * (s.db[227][34] * ddt_scale));
        let eq22_e662_d_b35: f64 = (p.p341 * (s.db[227][35] * ddt_scale));
        let eq22_e667: f64 = (s.v[111] - s.v[109]);
        let eq22_e668: f64 = (p.p343 * eq22_e667);
        let eq22_e668_d_n0: f64 = (p.p343 * s.dn[111][0]);
        let eq22_e668_d_n1: f64 = (p.p343 * s.dn[111][1]);
        let eq22_e668_d_n2: f64 = (p.p343 * s.dn[111][2]);
        let eq22_e668_d_n3: f64 = (p.p343 * s.dn[111][3]);
        let eq22_e668_d_n4: f64 = (p.p343 * s.dn[111][4]);
        let eq22_e668_d_n5: f64 = (p.p343 * s.dn[111][5]);
        let eq22_e668_d_n6: f64 = (p.p343 * s.dn[111][6]);
        let eq22_e668_d_n7: f64 = (p.p343 * s.dn[111][7]);
        let eq22_e668_d_n8: f64 = (p.p343 * s.dn[111][8]);
        let eq22_e668_d_n9: f64 = (p.p343 * s.dn[111][9]);
        let eq22_e668_d_n10: f64 = (p.p343 * s.dn[111][10]);
        let eq22_e668_d_n11: f64 = (p.p343 * s.dn[111][11]);
        let eq22_e668_d_n12: f64 = (p.p343 * s.dn[111][12]);
        let eq22_e668_d_n13: f64 = (p.p343 * s.dn[111][13]);
        let eq22_e668_d_n14: f64 = (p.p343 * s.dn[111][14]);
        let eq22_e668_d_n15: f64 = (p.p343 * s.dn[111][15]);
        let eq22_e668_d_n16: f64 = (p.p343 * s.dn[111][16]);
        let eq22_e668_d_n17: f64 = (p.p343 * s.dn[111][17]);
        let eq22_e668_d_n18: f64 = (p.p343 * s.dn[111][18]);
        let eq22_e668_d_n19: f64 = (p.p343 * s.dn[111][19]);
        let eq22_e668_d_n20: f64 = (p.p343 * s.dn[111][20]);
        let eq22_e668_d_n21: f64 = (p.p343 * s.dn[111][21]);
        let eq22_e668_d_n22: f64 = (p.p343 * s.dn[111][22]);
        let eq22_e668_d_n23: f64 = (p.p343 * s.dn[111][23]);
        let eq22_e668_d_n24: f64 = (p.p343 * s.dn[111][24]);
        let eq22_e668_d_n25: f64 = (p.p343 * s.dn[111][25]);
        let eq22_e668_d_n26: f64 = (p.p343 * s.dn[111][26]);
        let eq22_e668_d_n27: f64 = (p.p343 * s.dn[111][27]);
        let eq22_e668_d_n28: f64 = (p.p343 * s.dn[111][28]);
        let eq22_e668_d_n29: f64 = (p.p343 * s.dn[111][29]);
        let eq22_e668_d_b0: f64 = (p.p343 * s.db[111][0]);
        let eq22_e668_d_b1: f64 = (p.p343 * s.db[111][1]);
        let eq22_e668_d_b2: f64 = (p.p343 * s.db[111][2]);
        let eq22_e668_d_b3: f64 = (p.p343 * s.db[111][3]);
        let eq22_e668_d_b4: f64 = (p.p343 * s.db[111][4]);
        let eq22_e668_d_b5: f64 = (p.p343 * s.db[111][5]);
        let eq22_e668_d_b6: f64 = (p.p343 * s.db[111][6]);
        let eq22_e668_d_b7: f64 = (p.p343 * s.db[111][7]);
        let eq22_e668_d_b8: f64 = (p.p343 * s.db[111][8]);
        let eq22_e668_d_b9: f64 = (p.p343 * s.db[111][9]);
        let eq22_e668_d_b10: f64 = (p.p343 * s.db[111][10]);
        let eq22_e668_d_b11: f64 = (p.p343 * s.db[111][11]);
        let eq22_e668_d_b12: f64 = (p.p343 * s.db[111][12]);
        let eq22_e668_d_b13: f64 = (p.p343 * s.db[111][13]);
        let eq22_e668_d_b14: f64 = (p.p343 * s.db[111][14]);
        let eq22_e668_d_b15: f64 = (p.p343 * s.db[111][15]);
        let eq22_e668_d_b16: f64 = (p.p343 * s.db[111][16]);
        let eq22_e668_d_b17: f64 = (p.p343 * s.db[111][17]);
        let eq22_e668_d_b18: f64 = (p.p343 * s.db[111][18]);
        let eq22_e668_d_b19: f64 = (p.p343 * s.db[111][19]);
        let eq22_e668_d_b20: f64 = (p.p343 * s.db[111][20]);
        let eq22_e668_d_b21: f64 = (p.p343 * s.db[111][21]);
        let eq22_e668_d_b22: f64 = (p.p343 * s.db[111][22]);
        let eq22_e668_d_b23: f64 = (p.p343 * s.db[111][23]);
        let eq22_e668_d_b24: f64 = (p.p343 * s.db[111][24]);
        let eq22_e668_d_b25: f64 = (p.p343 * s.db[111][25]);
        let eq22_e668_d_b26: f64 = (p.p343 * s.db[111][26]);
        let eq22_e668_d_b27: f64 = (p.p343 * s.db[111][27]);
        let eq22_e668_d_b28: f64 = (p.p343 * s.db[111][28]);
        let eq22_e668_d_b29: f64 = (p.p343 * s.db[111][29]);
        let eq22_e668_d_b30: f64 = (p.p343 * s.db[111][30]);
        let eq22_e668_d_b31: f64 = (p.p343 * s.db[111][31]);
        let eq22_e668_d_b32: f64 = (p.p343 * s.db[111][32]);
        let eq22_e668_d_b33: f64 = (p.p343 * s.db[111][33]);
        let eq22_e668_d_b34: f64 = (p.p343 * s.db[111][34]);
        let eq22_e668_d_b35: f64 = (p.p343 * s.db[111][35]);
        let eq22_e669: f64 = (1.0 + eq22_e668);
        let eq22_e673: f64 = (s.v[111] - s.v[109]);
        let eq22_e674: f64 = (p.p345 * eq22_e673);
        let eq22_e674_d_n0: f64 = (p.p345 * s.dn[111][0]);
        let eq22_e674_d_n1: f64 = (p.p345 * s.dn[111][1]);
        let eq22_e674_d_n2: f64 = (p.p345 * s.dn[111][2]);
        let eq22_e674_d_n3: f64 = (p.p345 * s.dn[111][3]);
        let eq22_e674_d_n4: f64 = (p.p345 * s.dn[111][4]);
        let eq22_e674_d_n5: f64 = (p.p345 * s.dn[111][5]);
        let eq22_e674_d_n6: f64 = (p.p345 * s.dn[111][6]);
        let eq22_e674_d_n7: f64 = (p.p345 * s.dn[111][7]);
        let eq22_e674_d_n8: f64 = (p.p345 * s.dn[111][8]);
        let eq22_e674_d_n9: f64 = (p.p345 * s.dn[111][9]);
        let eq22_e674_d_n10: f64 = (p.p345 * s.dn[111][10]);
        let eq22_e674_d_n11: f64 = (p.p345 * s.dn[111][11]);
        let eq22_e674_d_n12: f64 = (p.p345 * s.dn[111][12]);
        let eq22_e674_d_n13: f64 = (p.p345 * s.dn[111][13]);
        let eq22_e674_d_n14: f64 = (p.p345 * s.dn[111][14]);
        let eq22_e674_d_n15: f64 = (p.p345 * s.dn[111][15]);
        let eq22_e674_d_n16: f64 = (p.p345 * s.dn[111][16]);
        let eq22_e674_d_n17: f64 = (p.p345 * s.dn[111][17]);
        let eq22_e674_d_n18: f64 = (p.p345 * s.dn[111][18]);
        let eq22_e674_d_n19: f64 = (p.p345 * s.dn[111][19]);
        let eq22_e674_d_n20: f64 = (p.p345 * s.dn[111][20]);
        let eq22_e674_d_n21: f64 = (p.p345 * s.dn[111][21]);
        let eq22_e674_d_n22: f64 = (p.p345 * s.dn[111][22]);
        let eq22_e674_d_n23: f64 = (p.p345 * s.dn[111][23]);
        let eq22_e674_d_n24: f64 = (p.p345 * s.dn[111][24]);
        let eq22_e674_d_n25: f64 = (p.p345 * s.dn[111][25]);
        let eq22_e674_d_n26: f64 = (p.p345 * s.dn[111][26]);
        let eq22_e674_d_n27: f64 = (p.p345 * s.dn[111][27]);
        let eq22_e674_d_n28: f64 = (p.p345 * s.dn[111][28]);
        let eq22_e674_d_n29: f64 = (p.p345 * s.dn[111][29]);
        let eq22_e674_d_b0: f64 = (p.p345 * s.db[111][0]);
        let eq22_e674_d_b1: f64 = (p.p345 * s.db[111][1]);
        let eq22_e674_d_b2: f64 = (p.p345 * s.db[111][2]);
        let eq22_e674_d_b3: f64 = (p.p345 * s.db[111][3]);
        let eq22_e674_d_b4: f64 = (p.p345 * s.db[111][4]);
        let eq22_e674_d_b5: f64 = (p.p345 * s.db[111][5]);
        let eq22_e674_d_b6: f64 = (p.p345 * s.db[111][6]);
        let eq22_e674_d_b7: f64 = (p.p345 * s.db[111][7]);
        let eq22_e674_d_b8: f64 = (p.p345 * s.db[111][8]);
        let eq22_e674_d_b9: f64 = (p.p345 * s.db[111][9]);
        let eq22_e674_d_b10: f64 = (p.p345 * s.db[111][10]);
        let eq22_e674_d_b11: f64 = (p.p345 * s.db[111][11]);
        let eq22_e674_d_b12: f64 = (p.p345 * s.db[111][12]);
        let eq22_e674_d_b13: f64 = (p.p345 * s.db[111][13]);
        let eq22_e674_d_b14: f64 = (p.p345 * s.db[111][14]);
        let eq22_e674_d_b15: f64 = (p.p345 * s.db[111][15]);
        let eq22_e674_d_b16: f64 = (p.p345 * s.db[111][16]);
        let eq22_e674_d_b17: f64 = (p.p345 * s.db[111][17]);
        let eq22_e674_d_b18: f64 = (p.p345 * s.db[111][18]);
        let eq22_e674_d_b19: f64 = (p.p345 * s.db[111][19]);
        let eq22_e674_d_b20: f64 = (p.p345 * s.db[111][20]);
        let eq22_e674_d_b21: f64 = (p.p345 * s.db[111][21]);
        let eq22_e674_d_b22: f64 = (p.p345 * s.db[111][22]);
        let eq22_e674_d_b23: f64 = (p.p345 * s.db[111][23]);
        let eq22_e674_d_b24: f64 = (p.p345 * s.db[111][24]);
        let eq22_e674_d_b25: f64 = (p.p345 * s.db[111][25]);
        let eq22_e674_d_b26: f64 = (p.p345 * s.db[111][26]);
        let eq22_e674_d_b27: f64 = (p.p345 * s.db[111][27]);
        let eq22_e674_d_b28: f64 = (p.p345 * s.db[111][28]);
        let eq22_e674_d_b29: f64 = (p.p345 * s.db[111][29]);
        let eq22_e674_d_b30: f64 = (p.p345 * s.db[111][30]);
        let eq22_e674_d_b31: f64 = (p.p345 * s.db[111][31]);
        let eq22_e674_d_b32: f64 = (p.p345 * s.db[111][32]);
        let eq22_e674_d_b33: f64 = (p.p345 * s.db[111][33]);
        let eq22_e674_d_b34: f64 = (p.p345 * s.db[111][34]);
        let eq22_e674_d_b35: f64 = (p.p345 * s.db[111][35]);
        let eq22_e677: f64 = (s.v[111] - s.v[109]);
        let eq22_e678: f64 = (eq22_e674 * eq22_e677);
        let eq22_e678_d_n0: f64 = ((eq22_e674_d_n0 * eq22_e677) + (eq22_e674 * s.dn[111][0]));
        let eq22_e678_d_n1: f64 = ((eq22_e674_d_n1 * eq22_e677) + (eq22_e674 * s.dn[111][1]));
        let eq22_e678_d_n2: f64 = ((eq22_e674_d_n2 * eq22_e677) + (eq22_e674 * s.dn[111][2]));
        let eq22_e678_d_n3: f64 = ((eq22_e674_d_n3 * eq22_e677) + (eq22_e674 * s.dn[111][3]));
        let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * s.dn[111][4]));
        let eq22_e678_d_n5: f64 = ((eq22_e674_d_n5 * eq22_e677) + (eq22_e674 * s.dn[111][5]));
        let eq22_e678_d_n6: f64 = ((eq22_e674_d_n6 * eq22_e677) + (eq22_e674 * s.dn[111][6]));
        let eq22_e678_d_n7: f64 = ((eq22_e674_d_n7 * eq22_e677) + (eq22_e674 * s.dn[111][7]));
        let eq22_e678_d_n8: f64 = ((eq22_e674_d_n8 * eq22_e677) + (eq22_e674 * s.dn[111][8]));
        let eq22_e678_d_n9: f64 = ((eq22_e674_d_n9 * eq22_e677) + (eq22_e674 * s.dn[111][9]));
        let eq22_e678_d_n10: f64 = ((eq22_e674_d_n10 * eq22_e677) + (eq22_e674 * s.dn[111][10]));
        let eq22_e678_d_n11: f64 = ((eq22_e674_d_n11 * eq22_e677) + (eq22_e674 * s.dn[111][11]));
        let eq22_e678_d_n12: f64 = ((eq22_e674_d_n12 * eq22_e677) + (eq22_e674 * s.dn[111][12]));
        let eq22_e678_d_n13: f64 = ((eq22_e674_d_n13 * eq22_e677) + (eq22_e674 * s.dn[111][13]));
        let eq22_e678_d_n14: f64 = ((eq22_e674_d_n14 * eq22_e677) + (eq22_e674 * s.dn[111][14]));
        let eq22_e678_d_n15: f64 = ((eq22_e674_d_n15 * eq22_e677) + (eq22_e674 * s.dn[111][15]));
        let eq22_e678_d_n16: f64 = ((eq22_e674_d_n16 * eq22_e677) + (eq22_e674 * s.dn[111][16]));
        let eq22_e678_d_n17: f64 = ((eq22_e674_d_n17 * eq22_e677) + (eq22_e674 * s.dn[111][17]));
        let eq22_e678_d_n18: f64 = ((eq22_e674_d_n18 * eq22_e677) + (eq22_e674 * s.dn[111][18]));
        let eq22_e678_d_n19: f64 = ((eq22_e674_d_n19 * eq22_e677) + (eq22_e674 * s.dn[111][19]));
        let eq22_e678_d_n20: f64 = ((eq22_e674_d_n20 * eq22_e677) + (eq22_e674 * s.dn[111][20]));
        let eq22_e678_d_n21: f64 = ((eq22_e674_d_n21 * eq22_e677) + (eq22_e674 * s.dn[111][21]));
        let eq22_e678_d_n22: f64 = ((eq22_e674_d_n22 * eq22_e677) + (eq22_e674 * s.dn[111][22]));
        let eq22_e678_d_n23: f64 = ((eq22_e674_d_n23 * eq22_e677) + (eq22_e674 * s.dn[111][23]));
        let eq22_e678_d_n24: f64 = ((eq22_e674_d_n24 * eq22_e677) + (eq22_e674 * s.dn[111][24]));
        let eq22_e678_d_n25: f64 = ((eq22_e674_d_n25 * eq22_e677) + (eq22_e674 * s.dn[111][25]));
        let eq22_e678_d_n26: f64 = ((eq22_e674_d_n26 * eq22_e677) + (eq22_e674 * s.dn[111][26]));
        let eq22_e678_d_n27: f64 = ((eq22_e674_d_n27 * eq22_e677) + (eq22_e674 * s.dn[111][27]));
        let eq22_e678_d_n28: f64 = ((eq22_e674_d_n28 * eq22_e677) + (eq22_e674 * s.dn[111][28]));
        let eq22_e678_d_n29: f64 = ((eq22_e674_d_n29 * eq22_e677) + (eq22_e674 * s.dn[111][29]));
        let eq22_e678_d_b0: f64 = ((eq22_e674_d_b0 * eq22_e677) + (eq22_e674 * s.db[111][0]));
        let eq22_e678_d_b1: f64 = ((eq22_e674_d_b1 * eq22_e677) + (eq22_e674 * s.db[111][1]));
        let eq22_e678_d_b2: f64 = ((eq22_e674_d_b2 * eq22_e677) + (eq22_e674 * s.db[111][2]));
        let eq22_e678_d_b3: f64 = ((eq22_e674_d_b3 * eq22_e677) + (eq22_e674 * s.db[111][3]));
        let eq22_e678_d_b4: f64 = ((eq22_e674_d_b4 * eq22_e677) + (eq22_e674 * s.db[111][4]));
        let eq22_e678_d_b5: f64 = ((eq22_e674_d_b5 * eq22_e677) + (eq22_e674 * s.db[111][5]));
        let eq22_e678_d_b6: f64 = ((eq22_e674_d_b6 * eq22_e677) + (eq22_e674 * s.db[111][6]));
        let eq22_e678_d_b7: f64 = ((eq22_e674_d_b7 * eq22_e677) + (eq22_e674 * s.db[111][7]));
        let eq22_e678_d_b8: f64 = ((eq22_e674_d_b8 * eq22_e677) + (eq22_e674 * s.db[111][8]));
        let eq22_e678_d_b9: f64 = ((eq22_e674_d_b9 * eq22_e677) + (eq22_e674 * s.db[111][9]));
        let eq22_e678_d_b10: f64 = ((eq22_e674_d_b10 * eq22_e677) + (eq22_e674 * s.db[111][10]));
        let eq22_e678_d_b11: f64 = ((eq22_e674_d_b11 * eq22_e677) + (eq22_e674 * s.db[111][11]));
        let eq22_e678_d_b12: f64 = ((eq22_e674_d_b12 * eq22_e677) + (eq22_e674 * s.db[111][12]));
        let eq22_e678_d_b13: f64 = ((eq22_e674_d_b13 * eq22_e677) + (eq22_e674 * s.db[111][13]));
        let eq22_e678_d_b14: f64 = ((eq22_e674_d_b14 * eq22_e677) + (eq22_e674 * s.db[111][14]));
        let eq22_e678_d_b15: f64 = ((eq22_e674_d_b15 * eq22_e677) + (eq22_e674 * s.db[111][15]));
        let eq22_e678_d_b16: f64 = ((eq22_e674_d_b16 * eq22_e677) + (eq22_e674 * s.db[111][16]));
        let eq22_e678_d_b17: f64 = ((eq22_e674_d_b17 * eq22_e677) + (eq22_e674 * s.db[111][17]));
        let eq22_e678_d_b18: f64 = ((eq22_e674_d_b18 * eq22_e677) + (eq22_e674 * s.db[111][18]));
        let eq22_e678_d_b19: f64 = ((eq22_e674_d_b19 * eq22_e677) + (eq22_e674 * s.db[111][19]));
        let eq22_e678_d_b20: f64 = ((eq22_e674_d_b20 * eq22_e677) + (eq22_e674 * s.db[111][20]));
        let eq22_e678_d_b21: f64 = ((eq22_e674_d_b21 * eq22_e677) + (eq22_e674 * s.db[111][21]));
        let eq22_e678_d_b22: f64 = ((eq22_e674_d_b22 * eq22_e677) + (eq22_e674 * s.db[111][22]));
        let eq22_e678_d_b23: f64 = ((eq22_e674_d_b23 * eq22_e677) + (eq22_e674 * s.db[111][23]));
        let eq22_e678_d_b24: f64 = ((eq22_e674_d_b24 * eq22_e677) + (eq22_e674 * s.db[111][24]));
        let eq22_e678_d_b25: f64 = ((eq22_e674_d_b25 * eq22_e677) + (eq22_e674 * s.db[111][25]));
        let eq22_e678_d_b26: f64 = ((eq22_e674_d_b26 * eq22_e677) + (eq22_e674 * s.db[111][26]));
        let eq22_e678_d_b27: f64 = ((eq22_e674_d_b27 * eq22_e677) + (eq22_e674 * s.db[111][27]));
        let eq22_e678_d_b28: f64 = ((eq22_e674_d_b28 * eq22_e677) + (eq22_e674 * s.db[111][28]));
        let eq22_e678_d_b29: f64 = ((eq22_e674_d_b29 * eq22_e677) + (eq22_e674 * s.db[111][29]));
        let eq22_e678_d_b30: f64 = ((eq22_e674_d_b30 * eq22_e677) + (eq22_e674 * s.db[111][30]));
        let eq22_e678_d_b31: f64 = ((eq22_e674_d_b31 * eq22_e677) + (eq22_e674 * s.db[111][31]));
        let eq22_e678_d_b32: f64 = ((eq22_e674_d_b32 * eq22_e677) + (eq22_e674 * s.db[111][32]));
        let eq22_e678_d_b33: f64 = ((eq22_e674_d_b33 * eq22_e677) + (eq22_e674 * s.db[111][33]));
        let eq22_e678_d_b34: f64 = ((eq22_e674_d_b34 * eq22_e677) + (eq22_e674 * s.db[111][34]));
        let eq22_e678_d_b35: f64 = ((eq22_e674_d_b35 * eq22_e677) + (eq22_e674 * s.db[111][35]));
        let eq22_e679: f64 = (eq22_e669 + eq22_e678);
        let eq22_e679_d_n0: f64 = (eq22_e668_d_n0 + eq22_e678_d_n0);
        let eq22_e679_d_n1: f64 = (eq22_e668_d_n1 + eq22_e678_d_n1);
        let eq22_e679_d_n2: f64 = (eq22_e668_d_n2 + eq22_e678_d_n2);
        let eq22_e679_d_n3: f64 = (eq22_e668_d_n3 + eq22_e678_d_n3);
        let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);
        let eq22_e679_d_n5: f64 = (eq22_e668_d_n5 + eq22_e678_d_n5);
        let eq22_e679_d_n6: f64 = (eq22_e668_d_n6 + eq22_e678_d_n6);
        let eq22_e679_d_n7: f64 = (eq22_e668_d_n7 + eq22_e678_d_n7);
        let eq22_e679_d_n8: f64 = (eq22_e668_d_n8 + eq22_e678_d_n8);
        let eq22_e679_d_n9: f64 = (eq22_e668_d_n9 + eq22_e678_d_n9);
        let eq22_e679_d_n10: f64 = (eq22_e668_d_n10 + eq22_e678_d_n10);
        let eq22_e679_d_n11: f64 = (eq22_e668_d_n11 + eq22_e678_d_n11);
        let eq22_e679_d_n12: f64 = (eq22_e668_d_n12 + eq22_e678_d_n12);
        let eq22_e679_d_n13: f64 = (eq22_e668_d_n13 + eq22_e678_d_n13);
        let eq22_e679_d_n14: f64 = (eq22_e668_d_n14 + eq22_e678_d_n14);
        let eq22_e679_d_n15: f64 = (eq22_e668_d_n15 + eq22_e678_d_n15);
        let eq22_e679_d_n16: f64 = (eq22_e668_d_n16 + eq22_e678_d_n16);
        let eq22_e679_d_n17: f64 = (eq22_e668_d_n17 + eq22_e678_d_n17);
        let eq22_e679_d_n18: f64 = (eq22_e668_d_n18 + eq22_e678_d_n18);
        let eq22_e679_d_n19: f64 = (eq22_e668_d_n19 + eq22_e678_d_n19);
        let eq22_e679_d_n20: f64 = (eq22_e668_d_n20 + eq22_e678_d_n20);
        let eq22_e679_d_n21: f64 = (eq22_e668_d_n21 + eq22_e678_d_n21);
        let eq22_e679_d_n22: f64 = (eq22_e668_d_n22 + eq22_e678_d_n22);
        let eq22_e679_d_n23: f64 = (eq22_e668_d_n23 + eq22_e678_d_n23);
        let eq22_e679_d_n24: f64 = (eq22_e668_d_n24 + eq22_e678_d_n24);
        let eq22_e679_d_n25: f64 = (eq22_e668_d_n25 + eq22_e678_d_n25);
        let eq22_e679_d_n26: f64 = (eq22_e668_d_n26 + eq22_e678_d_n26);
        let eq22_e679_d_n27: f64 = (eq22_e668_d_n27 + eq22_e678_d_n27);
        let eq22_e679_d_n28: f64 = (eq22_e668_d_n28 + eq22_e678_d_n28);
        let eq22_e679_d_n29: f64 = (eq22_e668_d_n29 + eq22_e678_d_n29);
        let eq22_e679_d_b0: f64 = (eq22_e668_d_b0 + eq22_e678_d_b0);
        let eq22_e679_d_b1: f64 = (eq22_e668_d_b1 + eq22_e678_d_b1);
        let eq22_e679_d_b2: f64 = (eq22_e668_d_b2 + eq22_e678_d_b2);
        let eq22_e679_d_b3: f64 = (eq22_e668_d_b3 + eq22_e678_d_b3);
        let eq22_e679_d_b4: f64 = (eq22_e668_d_b4 + eq22_e678_d_b4);
        let eq22_e679_d_b5: f64 = (eq22_e668_d_b5 + eq22_e678_d_b5);
        let eq22_e679_d_b6: f64 = (eq22_e668_d_b6 + eq22_e678_d_b6);
        let eq22_e679_d_b7: f64 = (eq22_e668_d_b7 + eq22_e678_d_b7);
        let eq22_e679_d_b8: f64 = (eq22_e668_d_b8 + eq22_e678_d_b8);
        let eq22_e679_d_b9: f64 = (eq22_e668_d_b9 + eq22_e678_d_b9);
        let eq22_e679_d_b10: f64 = (eq22_e668_d_b10 + eq22_e678_d_b10);
        let eq22_e679_d_b11: f64 = (eq22_e668_d_b11 + eq22_e678_d_b11);
        let eq22_e679_d_b12: f64 = (eq22_e668_d_b12 + eq22_e678_d_b12);
        let eq22_e679_d_b13: f64 = (eq22_e668_d_b13 + eq22_e678_d_b13);
        let eq22_e679_d_b14: f64 = (eq22_e668_d_b14 + eq22_e678_d_b14);
        let eq22_e679_d_b15: f64 = (eq22_e668_d_b15 + eq22_e678_d_b15);
        let eq22_e679_d_b16: f64 = (eq22_e668_d_b16 + eq22_e678_d_b16);
        let eq22_e679_d_b17: f64 = (eq22_e668_d_b17 + eq22_e678_d_b17);
        let eq22_e679_d_b18: f64 = (eq22_e668_d_b18 + eq22_e678_d_b18);
        let eq22_e679_d_b19: f64 = (eq22_e668_d_b19 + eq22_e678_d_b19);
        let eq22_e679_d_b20: f64 = (eq22_e668_d_b20 + eq22_e678_d_b20);
        let eq22_e679_d_b21: f64 = (eq22_e668_d_b21 + eq22_e678_d_b21);
        let eq22_e679_d_b22: f64 = (eq22_e668_d_b22 + eq22_e678_d_b22);
        let eq22_e679_d_b23: f64 = (eq22_e668_d_b23 + eq22_e678_d_b23);
        let eq22_e679_d_b24: f64 = (eq22_e668_d_b24 + eq22_e678_d_b24);
        let eq22_e679_d_b25: f64 = (eq22_e668_d_b25 + eq22_e678_d_b25);
        let eq22_e679_d_b26: f64 = (eq22_e668_d_b26 + eq22_e678_d_b26);
        let eq22_e679_d_b27: f64 = (eq22_e668_d_b27 + eq22_e678_d_b27);
        let eq22_e679_d_b28: f64 = (eq22_e668_d_b28 + eq22_e678_d_b28);
        let eq22_e679_d_b29: f64 = (eq22_e668_d_b29 + eq22_e678_d_b29);
        let eq22_e679_d_b30: f64 = (eq22_e668_d_b30 + eq22_e678_d_b30);
        let eq22_e679_d_b31: f64 = (eq22_e668_d_b31 + eq22_e678_d_b31);
        let eq22_e679_d_b32: f64 = (eq22_e668_d_b32 + eq22_e678_d_b32);
        let eq22_e679_d_b33: f64 = (eq22_e668_d_b33 + eq22_e678_d_b33);
        let eq22_e679_d_b34: f64 = (eq22_e668_d_b34 + eq22_e678_d_b34);
        let eq22_e679_d_b35: f64 = (eq22_e668_d_b35 + eq22_e678_d_b35);
        let eq22_e680: f64 = (eq22_e662 * eq22_e679);
        let eq22_e680_d_n0: f64 = ((eq22_e662_d_n0 * eq22_e679) + (eq22_e662 * eq22_e679_d_n0));
        let eq22_e680_d_n1: f64 = ((eq22_e662_d_n1 * eq22_e679) + (eq22_e662 * eq22_e679_d_n1));
        let eq22_e680_d_n2: f64 = ((eq22_e662_d_n2 * eq22_e679) + (eq22_e662 * eq22_e679_d_n2));
        let eq22_e680_d_n3: f64 = ((eq22_e662_d_n3 * eq22_e679) + (eq22_e662 * eq22_e679_d_n3));
        let eq22_e680_d_n4: f64 = ((eq22_e662_d_n4 * eq22_e679) + (eq22_e662 * eq22_e679_d_n4));
        let eq22_e680_d_n5: f64 = ((eq22_e662_d_n5 * eq22_e679) + (eq22_e662 * eq22_e679_d_n5));
        let eq22_e680_d_n6: f64 = ((eq22_e662_d_n6 * eq22_e679) + (eq22_e662 * eq22_e679_d_n6));
        let eq22_e680_d_n7: f64 = ((eq22_e662_d_n7 * eq22_e679) + (eq22_e662 * eq22_e679_d_n7));
        let eq22_e680_d_n8: f64 = ((eq22_e662_d_n8 * eq22_e679) + (eq22_e662 * eq22_e679_d_n8));
        let eq22_e680_d_n9: f64 = ((eq22_e662_d_n9 * eq22_e679) + (eq22_e662 * eq22_e679_d_n9));
        let eq22_e680_d_n10: f64 = ((eq22_e662_d_n10 * eq22_e679) + (eq22_e662 * eq22_e679_d_n10));
        let eq22_e680_d_n11: f64 = ((eq22_e662_d_n11 * eq22_e679) + (eq22_e662 * eq22_e679_d_n11));
        let eq22_e680_d_n12: f64 = ((eq22_e662_d_n12 * eq22_e679) + (eq22_e662 * eq22_e679_d_n12));
        let eq22_e680_d_n13: f64 = ((eq22_e662_d_n13 * eq22_e679) + (eq22_e662 * eq22_e679_d_n13));
        let eq22_e680_d_n14: f64 = ((eq22_e662_d_n14 * eq22_e679) + (eq22_e662 * eq22_e679_d_n14));
        let eq22_e680_d_n15: f64 = ((eq22_e662_d_n15 * eq22_e679) + (eq22_e662 * eq22_e679_d_n15));
        let eq22_e680_d_n16: f64 = ((eq22_e662_d_n16 * eq22_e679) + (eq22_e662 * eq22_e679_d_n16));
        let eq22_e680_d_n17: f64 = ((eq22_e662_d_n17 * eq22_e679) + (eq22_e662 * eq22_e679_d_n17));
        let eq22_e680_d_n18: f64 = ((eq22_e662_d_n18 * eq22_e679) + (eq22_e662 * eq22_e679_d_n18));
        let eq22_e680_d_n19: f64 = ((eq22_e662_d_n19 * eq22_e679) + (eq22_e662 * eq22_e679_d_n19));
        let eq22_e680_d_n20: f64 = ((eq22_e662_d_n20 * eq22_e679) + (eq22_e662 * eq22_e679_d_n20));
        let eq22_e680_d_n21: f64 = ((eq22_e662_d_n21 * eq22_e679) + (eq22_e662 * eq22_e679_d_n21));
        let eq22_e680_d_n22: f64 = ((eq22_e662_d_n22 * eq22_e679) + (eq22_e662 * eq22_e679_d_n22));
        let eq22_e680_d_n23: f64 = ((eq22_e662_d_n23 * eq22_e679) + (eq22_e662 * eq22_e679_d_n23));
        let eq22_e680_d_n24: f64 = ((eq22_e662_d_n24 * eq22_e679) + (eq22_e662 * eq22_e679_d_n24));
        let eq22_e680_d_n25: f64 = ((eq22_e662_d_n25 * eq22_e679) + (eq22_e662 * eq22_e679_d_n25));
        let eq22_e680_d_n26: f64 = ((eq22_e662_d_n26 * eq22_e679) + (eq22_e662 * eq22_e679_d_n26));
        let eq22_e680_d_n27: f64 = ((eq22_e662_d_n27 * eq22_e679) + (eq22_e662 * eq22_e679_d_n27));
        let eq22_e680_d_n28: f64 = ((eq22_e662_d_n28 * eq22_e679) + (eq22_e662 * eq22_e679_d_n28));
        let eq22_e680_d_n29: f64 = ((eq22_e662_d_n29 * eq22_e679) + (eq22_e662 * eq22_e679_d_n29));
        let eq22_e680_d_b0: f64 = ((eq22_e662_d_b0 * eq22_e679) + (eq22_e662 * eq22_e679_d_b0));
        let eq22_e680_d_b1: f64 = ((eq22_e662_d_b1 * eq22_e679) + (eq22_e662 * eq22_e679_d_b1));
        let eq22_e680_d_b2: f64 = ((eq22_e662_d_b2 * eq22_e679) + (eq22_e662 * eq22_e679_d_b2));
        let eq22_e680_d_b3: f64 = ((eq22_e662_d_b3 * eq22_e679) + (eq22_e662 * eq22_e679_d_b3));
        let eq22_e680_d_b4: f64 = ((eq22_e662_d_b4 * eq22_e679) + (eq22_e662 * eq22_e679_d_b4));
        let eq22_e680_d_b5: f64 = ((eq22_e662_d_b5 * eq22_e679) + (eq22_e662 * eq22_e679_d_b5));
        let eq22_e680_d_b6: f64 = ((eq22_e662_d_b6 * eq22_e679) + (eq22_e662 * eq22_e679_d_b6));
        let eq22_e680_d_b7: f64 = ((eq22_e662_d_b7 * eq22_e679) + (eq22_e662 * eq22_e679_d_b7));
        let eq22_e680_d_b8: f64 = ((eq22_e662_d_b8 * eq22_e679) + (eq22_e662 * eq22_e679_d_b8));
        let eq22_e680_d_b9: f64 = ((eq22_e662_d_b9 * eq22_e679) + (eq22_e662 * eq22_e679_d_b9));
        let eq22_e680_d_b10: f64 = ((eq22_e662_d_b10 * eq22_e679) + (eq22_e662 * eq22_e679_d_b10));
        let eq22_e680_d_b11: f64 = ((eq22_e662_d_b11 * eq22_e679) + (eq22_e662 * eq22_e679_d_b11));
        let eq22_e680_d_b12: f64 = ((eq22_e662_d_b12 * eq22_e679) + (eq22_e662 * eq22_e679_d_b12));
        let eq22_e680_d_b13: f64 = ((eq22_e662_d_b13 * eq22_e679) + (eq22_e662 * eq22_e679_d_b13));
        let eq22_e680_d_b14: f64 = ((eq22_e662_d_b14 * eq22_e679) + (eq22_e662 * eq22_e679_d_b14));
        let eq22_e680_d_b15: f64 = ((eq22_e662_d_b15 * eq22_e679) + (eq22_e662 * eq22_e679_d_b15));
        let eq22_e680_d_b16: f64 = ((eq22_e662_d_b16 * eq22_e679) + (eq22_e662 * eq22_e679_d_b16));
        let eq22_e680_d_b17: f64 = ((eq22_e662_d_b17 * eq22_e679) + (eq22_e662 * eq22_e679_d_b17));
        let eq22_e680_d_b18: f64 = ((eq22_e662_d_b18 * eq22_e679) + (eq22_e662 * eq22_e679_d_b18));
        let eq22_e680_d_b19: f64 = ((eq22_e662_d_b19 * eq22_e679) + (eq22_e662 * eq22_e679_d_b19));
        let eq22_e680_d_b20: f64 = ((eq22_e662_d_b20 * eq22_e679) + (eq22_e662 * eq22_e679_d_b20));
        let eq22_e680_d_b21: f64 = ((eq22_e662_d_b21 * eq22_e679) + (eq22_e662 * eq22_e679_d_b21));
        let eq22_e680_d_b22: f64 = ((eq22_e662_d_b22 * eq22_e679) + (eq22_e662 * eq22_e679_d_b22));
        let eq22_e680_d_b23: f64 = ((eq22_e662_d_b23 * eq22_e679) + (eq22_e662 * eq22_e679_d_b23));
        let eq22_e680_d_b24: f64 = ((eq22_e662_d_b24 * eq22_e679) + (eq22_e662 * eq22_e679_d_b24));
        let eq22_e680_d_b25: f64 = ((eq22_e662_d_b25 * eq22_e679) + (eq22_e662 * eq22_e679_d_b25));
        let eq22_e680_d_b26: f64 = ((eq22_e662_d_b26 * eq22_e679) + (eq22_e662 * eq22_e679_d_b26));
        let eq22_e680_d_b27: f64 = ((eq22_e662_d_b27 * eq22_e679) + (eq22_e662 * eq22_e679_d_b27));
        let eq22_e680_d_b28: f64 = ((eq22_e662_d_b28 * eq22_e679) + (eq22_e662 * eq22_e679_d_b28));
        let eq22_e680_d_b29: f64 = ((eq22_e662_d_b29 * eq22_e679) + (eq22_e662 * eq22_e679_d_b29));
        let eq22_e680_d_b30: f64 = ((eq22_e662_d_b30 * eq22_e679) + (eq22_e662 * eq22_e679_d_b30));
        let eq22_e680_d_b31: f64 = ((eq22_e662_d_b31 * eq22_e679) + (eq22_e662 * eq22_e679_d_b31));
        let eq22_e680_d_b32: f64 = ((eq22_e662_d_b32 * eq22_e679) + (eq22_e662 * eq22_e679_d_b32));
        let eq22_e680_d_b33: f64 = ((eq22_e662_d_b33 * eq22_e679) + (eq22_e662 * eq22_e679_d_b33));
        let eq22_e680_d_b34: f64 = ((eq22_e662_d_b34 * eq22_e679) + (eq22_e662 * eq22_e679_d_b34));
        let eq22_e680_d_b35: f64 = ((eq22_e662_d_b35 * eq22_e679) + (eq22_e662 * eq22_e679_d_b35));
        (eq22_e680, eq22_e680_d_n0, eq22_e680_d_n1, eq22_e680_d_n2, eq22_e680_d_n3, eq22_e680_d_n4, eq22_e680_d_n5, eq22_e680_d_n6, eq22_e680_d_n7, eq22_e680_d_n8, eq22_e680_d_n9, eq22_e680_d_n10, eq22_e680_d_n11, eq22_e680_d_n12, eq22_e680_d_n13, eq22_e680_d_n14, eq22_e680_d_n15, eq22_e680_d_n16, eq22_e680_d_n17, eq22_e680_d_n18, eq22_e680_d_n19, eq22_e680_d_n20, eq22_e680_d_n21, eq22_e680_d_n22, eq22_e680_d_n23, eq22_e680_d_n24, eq22_e680_d_n25, eq22_e680_d_n26, eq22_e680_d_n27, eq22_e680_d_n28, eq22_e680_d_n29, eq22_e680_d_b0, eq22_e680_d_b1, eq22_e680_d_b2, eq22_e680_d_b3, eq22_e680_d_b4, eq22_e680_d_b5, eq22_e680_d_b6, eq22_e680_d_b7, eq22_e680_d_b8, eq22_e680_d_b9, eq22_e680_d_b10, eq22_e680_d_b11, eq22_e680_d_b12, eq22_e680_d_b13, eq22_e680_d_b14, eq22_e680_d_b15, eq22_e680_d_b16, eq22_e680_d_b17, eq22_e680_d_b18, eq22_e680_d_b19, eq22_e680_d_b20, eq22_e680_d_b21, eq22_e680_d_b22, eq22_e680_d_b23, eq22_e680_d_b24, eq22_e680_d_b25, eq22_e680_d_b26, eq22_e680_d_b27, eq22_e680_d_b28, eq22_e680_d_b29, eq22_e680_d_b30, eq22_e680_d_b31, eq22_e680_d_b32, eq22_e680_d_b33, eq22_e680_d_b34, eq22_e680_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e682;
        let eq22_node_derivatives: [f64; 30] = [eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29];
        let eq22_branch_derivatives: [f64; 36] = [eq22_e682_d_b0, eq22_e682_d_b1, eq22_e682_d_b2, eq22_e682_d_b3, eq22_e682_d_b4, eq22_e682_d_b5, eq22_e682_d_b6, eq22_e682_d_b7, eq22_e682_d_b8, eq22_e682_d_b9, eq22_e682_d_b10, eq22_e682_d_b11, eq22_e682_d_b12, eq22_e682_d_b13, eq22_e682_d_b14, eq22_e682_d_b15, eq22_e682_d_b16, eq22_e682_d_b17, eq22_e682_d_b18, eq22_e682_d_b19, eq22_e682_d_b20, eq22_e682_d_b21, eq22_e682_d_b22, eq22_e682_d_b23, eq22_e682_d_b24, eq22_e682_d_b25, eq22_e682_d_b26, eq22_e682_d_b27, eq22_e682_d_b28, eq22_e682_d_b29, eq22_e682_d_b30, eq22_e682_d_b31, eq22_e682_d_b32, eq22_e682_d_b33, eq22_e682_d_b34, eq22_e682_d_b35];
        stamper.stamp_current_dense_local(
            Some(26),
            None,
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq31_e754, eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29, eq31_e754_d_b0, eq31_e754_d_b1, eq31_e754_d_b2, eq31_e754_d_b3, eq31_e754_d_b4, eq31_e754_d_b5, eq31_e754_d_b6, eq31_e754_d_b7, eq31_e754_d_b8, eq31_e754_d_b9, eq31_e754_d_b10, eq31_e754_d_b11, eq31_e754_d_b12, eq31_e754_d_b13, eq31_e754_d_b14, eq31_e754_d_b15, eq31_e754_d_b16, eq31_e754_d_b17, eq31_e754_d_b18, eq31_e754_d_b19, eq31_e754_d_b20, eq31_e754_d_b21, eq31_e754_d_b22, eq31_e754_d_b23, eq31_e754_d_b24, eq31_e754_d_b25, eq31_e754_d_b26, eq31_e754_d_b27, eq31_e754_d_b28, eq31_e754_d_b29, eq31_e754_d_b30, eq31_e754_d_b31, eq31_e754_d_b32, eq31_e754_d_b33, eq31_e754_d_b34, eq31_e754_d_b35,) = {
    if s.b[320] {
        let eq31_e751: f64 = (s.v[0] * (nv17 - nv16));
        let eq31_e751_d_n16: f64 = (-s.v[0]);
        let eq31_e751_d_n17: f64 = s.v[0];
        let eq31_e752: f64 = (s.v[208] + eq31_e751);
        let eq31_e752_d_n16: f64 = (s.dn[208][16] + eq31_e751_d_n16);
        let eq31_e752_d_n17: f64 = (s.dn[208][17] + eq31_e751_d_n17);
        (eq31_e752, s.dn[208][0], s.dn[208][1], s.dn[208][2], s.dn[208][3], s.dn[208][4], s.dn[208][5], s.dn[208][6], s.dn[208][7], s.dn[208][8], s.dn[208][9], s.dn[208][10], s.dn[208][11], s.dn[208][12], s.dn[208][13], s.dn[208][14], s.dn[208][15], eq31_e752_d_n16, eq31_e752_d_n17, s.dn[208][18], s.dn[208][19], s.dn[208][20], s.dn[208][21], s.dn[208][22], s.dn[208][23], s.dn[208][24], s.dn[208][25], s.dn[208][26], s.dn[208][27], s.dn[208][28], s.dn[208][29], s.db[208][0], s.db[208][1], s.db[208][2], s.db[208][3], s.db[208][4], s.db[208][5], s.db[208][6], s.db[208][7], s.db[208][8], s.db[208][9], s.db[208][10], s.db[208][11], s.db[208][12], s.db[208][13], s.db[208][14], s.db[208][15], s.db[208][16], s.db[208][17], s.db[208][18], s.db[208][19], s.db[208][20], s.db[208][21], s.db[208][22], s.db[208][23], s.db[208][24], s.db[208][25], s.db[208][26], s.db[208][27], s.db[208][28], s.db[208][29], s.db[208][30], s.db[208][31], s.db[208][32], s.db[208][33], s.db[208][34], s.db[208][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e754;
        let eq31_node_derivatives: [f64; 30] = [eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29];
        let eq31_branch_derivatives: [f64; 36] = [eq31_e754_d_b0, eq31_e754_d_b1, eq31_e754_d_b2, eq31_e754_d_b3, eq31_e754_d_b4, eq31_e754_d_b5, eq31_e754_d_b6, eq31_e754_d_b7, eq31_e754_d_b8, eq31_e754_d_b9, eq31_e754_d_b10, eq31_e754_d_b11, eq31_e754_d_b12, eq31_e754_d_b13, eq31_e754_d_b14, eq31_e754_d_b15, eq31_e754_d_b16, eq31_e754_d_b17, eq31_e754_d_b18, eq31_e754_d_b19, eq31_e754_d_b20, eq31_e754_d_b21, eq31_e754_d_b22, eq31_e754_d_b23, eq31_e754_d_b24, eq31_e754_d_b25, eq31_e754_d_b26, eq31_e754_d_b27, eq31_e754_d_b28, eq31_e754_d_b29, eq31_e754_d_b30, eq31_e754_d_b31, eq31_e754_d_b32, eq31_e754_d_b33, eq31_e754_d_b34, eq31_e754_d_b35];
        stamper.stamp_current_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29, eq33_e769_d_b0, eq33_e769_d_b1, eq33_e769_d_b2, eq33_e769_d_b3, eq33_e769_d_b4, eq33_e769_d_b5, eq33_e769_d_b6, eq33_e769_d_b7, eq33_e769_d_b8, eq33_e769_d_b9, eq33_e769_d_b10, eq33_e769_d_b11, eq33_e769_d_b12, eq33_e769_d_b13, eq33_e769_d_b14, eq33_e769_d_b15, eq33_e769_d_b16, eq33_e769_d_b17, eq33_e769_d_b18, eq33_e769_d_b19, eq33_e769_d_b20, eq33_e769_d_b21, eq33_e769_d_b22, eq33_e769_d_b23, eq33_e769_d_b24, eq33_e769_d_b25, eq33_e769_d_b26, eq33_e769_d_b27, eq33_e769_d_b28, eq33_e769_d_b29, eq33_e769_d_b30, eq33_e769_d_b31, eq33_e769_d_b32, eq33_e769_d_b33, eq33_e769_d_b34, eq33_e769_d_b35,) = {
    if s.b[466] {
        let eq33_e762: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[209]);
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e765_d_n7: f64 = p.p355;
        let eq33_e765_d_n16: f64 = (-p.p355);
        let eq33_e766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq33_e765);
        let eq33_e767: f64 = (eq33_e762 + eq33_e766);
        let eq33_e767_d_n7: f64 = ((s.dn[209][7] * ddt_scale) + (eq33_e765_d_n7 * ddt_scale));
        let eq33_e767_d_n16: f64 = ((s.dn[209][16] * ddt_scale) + (eq33_e765_d_n16 * ddt_scale));
        (eq33_e767, (s.dn[209][0] * ddt_scale), (s.dn[209][1] * ddt_scale), (s.dn[209][2] * ddt_scale), (s.dn[209][3] * ddt_scale), (s.dn[209][4] * ddt_scale), (s.dn[209][5] * ddt_scale), (s.dn[209][6] * ddt_scale), eq33_e767_d_n7, (s.dn[209][8] * ddt_scale), (s.dn[209][9] * ddt_scale), (s.dn[209][10] * ddt_scale), (s.dn[209][11] * ddt_scale), (s.dn[209][12] * ddt_scale), (s.dn[209][13] * ddt_scale), (s.dn[209][14] * ddt_scale), (s.dn[209][15] * ddt_scale), eq33_e767_d_n16, (s.dn[209][17] * ddt_scale), (s.dn[209][18] * ddt_scale), (s.dn[209][19] * ddt_scale), (s.dn[209][20] * ddt_scale), (s.dn[209][21] * ddt_scale), (s.dn[209][22] * ddt_scale), (s.dn[209][23] * ddt_scale), (s.dn[209][24] * ddt_scale), (s.dn[209][25] * ddt_scale), (s.dn[209][26] * ddt_scale), (s.dn[209][27] * ddt_scale), (s.dn[209][28] * ddt_scale), (s.dn[209][29] * ddt_scale), (s.db[209][0] * ddt_scale), (s.db[209][1] * ddt_scale), (s.db[209][2] * ddt_scale), (s.db[209][3] * ddt_scale), (s.db[209][4] * ddt_scale), (s.db[209][5] * ddt_scale), (s.db[209][6] * ddt_scale), (s.db[209][7] * ddt_scale), (s.db[209][8] * ddt_scale), (s.db[209][9] * ddt_scale), (s.db[209][10] * ddt_scale), (s.db[209][11] * ddt_scale), (s.db[209][12] * ddt_scale), (s.db[209][13] * ddt_scale), (s.db[209][14] * ddt_scale), (s.db[209][15] * ddt_scale), (s.db[209][16] * ddt_scale), (s.db[209][17] * ddt_scale), (s.db[209][18] * ddt_scale), (s.db[209][19] * ddt_scale), (s.db[209][20] * ddt_scale), (s.db[209][21] * ddt_scale), (s.db[209][22] * ddt_scale), (s.db[209][23] * ddt_scale), (s.db[209][24] * ddt_scale), (s.db[209][25] * ddt_scale), (s.db[209][26] * ddt_scale), (s.db[209][27] * ddt_scale), (s.db[209][28] * ddt_scale), (s.db[209][29] * ddt_scale), (s.db[209][30] * ddt_scale), (s.db[209][31] * ddt_scale), (s.db[209][32] * ddt_scale), (s.db[209][33] * ddt_scale), (s.db[209][34] * ddt_scale), (s.db[209][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e769;
        let eq33_node_derivatives: [f64; 30] = [eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29];
        let eq33_branch_derivatives: [f64; 36] = [eq33_e769_d_b0, eq33_e769_d_b1, eq33_e769_d_b2, eq33_e769_d_b3, eq33_e769_d_b4, eq33_e769_d_b5, eq33_e769_d_b6, eq33_e769_d_b7, eq33_e769_d_b8, eq33_e769_d_b9, eq33_e769_d_b10, eq33_e769_d_b11, eq33_e769_d_b12, eq33_e769_d_b13, eq33_e769_d_b14, eq33_e769_d_b15, eq33_e769_d_b16, eq33_e769_d_b17, eq33_e769_d_b18, eq33_e769_d_b19, eq33_e769_d_b20, eq33_e769_d_b21, eq33_e769_d_b22, eq33_e769_d_b23, eq33_e769_d_b24, eq33_e769_d_b25, eq33_e769_d_b26, eq33_e769_d_b27, eq33_e769_d_b28, eq33_e769_d_b29, eq33_e769_d_b30, eq33_e769_d_b31, eq33_e769_d_b32, eq33_e769_d_b33, eq33_e769_d_b34, eq33_e769_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(16),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29, eq34_e779_d_b0, eq34_e779_d_b1, eq34_e779_d_b2, eq34_e779_d_b3, eq34_e779_d_b4, eq34_e779_d_b5, eq34_e779_d_b6, eq34_e779_d_b7, eq34_e779_d_b8, eq34_e779_d_b9, eq34_e779_d_b10, eq34_e779_d_b11, eq34_e779_d_b12, eq34_e779_d_b13, eq34_e779_d_b14, eq34_e779_d_b15, eq34_e779_d_b16, eq34_e779_d_b17, eq34_e779_d_b18, eq34_e779_d_b19, eq34_e779_d_b20, eq34_e779_d_b21, eq34_e779_d_b22, eq34_e779_d_b23, eq34_e779_d_b24, eq34_e779_d_b25, eq34_e779_d_b26, eq34_e779_d_b27, eq34_e779_d_b28, eq34_e779_d_b29, eq34_e779_d_b30, eq34_e779_d_b31, eq34_e779_d_b32, eq34_e779_d_b33, eq34_e779_d_b34, eq34_e779_d_b35,) = {
    if s.b[466] {
        let eq34_e772: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[210]);
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e775_d_n7: f64 = p.p355;
        let eq34_e775_d_n17: f64 = (-p.p355);
        let eq34_e776: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq34_e775);
        let eq34_e777: f64 = (eq34_e772 + eq34_e776);
        let eq34_e777_d_n7: f64 = ((s.dn[210][7] * ddt_scale) + (eq34_e775_d_n7 * ddt_scale));
        let eq34_e777_d_n17: f64 = ((s.dn[210][17] * ddt_scale) + (eq34_e775_d_n17 * ddt_scale));
        (eq34_e777, (s.dn[210][0] * ddt_scale), (s.dn[210][1] * ddt_scale), (s.dn[210][2] * ddt_scale), (s.dn[210][3] * ddt_scale), (s.dn[210][4] * ddt_scale), (s.dn[210][5] * ddt_scale), (s.dn[210][6] * ddt_scale), eq34_e777_d_n7, (s.dn[210][8] * ddt_scale), (s.dn[210][9] * ddt_scale), (s.dn[210][10] * ddt_scale), (s.dn[210][11] * ddt_scale), (s.dn[210][12] * ddt_scale), (s.dn[210][13] * ddt_scale), (s.dn[210][14] * ddt_scale), (s.dn[210][15] * ddt_scale), (s.dn[210][16] * ddt_scale), eq34_e777_d_n17, (s.dn[210][18] * ddt_scale), (s.dn[210][19] * ddt_scale), (s.dn[210][20] * ddt_scale), (s.dn[210][21] * ddt_scale), (s.dn[210][22] * ddt_scale), (s.dn[210][23] * ddt_scale), (s.dn[210][24] * ddt_scale), (s.dn[210][25] * ddt_scale), (s.dn[210][26] * ddt_scale), (s.dn[210][27] * ddt_scale), (s.dn[210][28] * ddt_scale), (s.dn[210][29] * ddt_scale), (s.db[210][0] * ddt_scale), (s.db[210][1] * ddt_scale), (s.db[210][2] * ddt_scale), (s.db[210][3] * ddt_scale), (s.db[210][4] * ddt_scale), (s.db[210][5] * ddt_scale), (s.db[210][6] * ddt_scale), (s.db[210][7] * ddt_scale), (s.db[210][8] * ddt_scale), (s.db[210][9] * ddt_scale), (s.db[210][10] * ddt_scale), (s.db[210][11] * ddt_scale), (s.db[210][12] * ddt_scale), (s.db[210][13] * ddt_scale), (s.db[210][14] * ddt_scale), (s.db[210][15] * ddt_scale), (s.db[210][16] * ddt_scale), (s.db[210][17] * ddt_scale), (s.db[210][18] * ddt_scale), (s.db[210][19] * ddt_scale), (s.db[210][20] * ddt_scale), (s.db[210][21] * ddt_scale), (s.db[210][22] * ddt_scale), (s.db[210][23] * ddt_scale), (s.db[210][24] * ddt_scale), (s.db[210][25] * ddt_scale), (s.db[210][26] * ddt_scale), (s.db[210][27] * ddt_scale), (s.db[210][28] * ddt_scale), (s.db[210][29] * ddt_scale), (s.db[210][30] * ddt_scale), (s.db[210][31] * ddt_scale), (s.db[210][32] * ddt_scale), (s.db[210][33] * ddt_scale), (s.db[210][34] * ddt_scale), (s.db[210][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e779;
        let eq34_node_derivatives: [f64; 30] = [eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29];
        let eq34_branch_derivatives: [f64; 36] = [eq34_e779_d_b0, eq34_e779_d_b1, eq34_e779_d_b2, eq34_e779_d_b3, eq34_e779_d_b4, eq34_e779_d_b5, eq34_e779_d_b6, eq34_e779_d_b7, eq34_e779_d_b8, eq34_e779_d_b9, eq34_e779_d_b10, eq34_e779_d_b11, eq34_e779_d_b12, eq34_e779_d_b13, eq34_e779_d_b14, eq34_e779_d_b15, eq34_e779_d_b16, eq34_e779_d_b17, eq34_e779_d_b18, eq34_e779_d_b19, eq34_e779_d_b20, eq34_e779_d_b21, eq34_e779_d_b22, eq34_e779_d_b23, eq34_e779_d_b24, eq34_e779_d_b25, eq34_e779_d_b26, eq34_e779_d_b27, eq34_e779_d_b28, eq34_e779_d_b29, eq34_e779_d_b30, eq34_e779_d_b31, eq34_e779_d_b32, eq34_e779_d_b33, eq34_e779_d_b34, eq34_e779_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(17),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29, eq35_e789_d_b0, eq35_e789_d_b1, eq35_e789_d_b2, eq35_e789_d_b3, eq35_e789_d_b4, eq35_e789_d_b5, eq35_e789_d_b6, eq35_e789_d_b7, eq35_e789_d_b8, eq35_e789_d_b9, eq35_e789_d_b10, eq35_e789_d_b11, eq35_e789_d_b12, eq35_e789_d_b13, eq35_e789_d_b14, eq35_e789_d_b15, eq35_e789_d_b16, eq35_e789_d_b17, eq35_e789_d_b18, eq35_e789_d_b19, eq35_e789_d_b20, eq35_e789_d_b21, eq35_e789_d_b22, eq35_e789_d_b23, eq35_e789_d_b24, eq35_e789_d_b25, eq35_e789_d_b26, eq35_e789_d_b27, eq35_e789_d_b28, eq35_e789_d_b29, eq35_e789_d_b30, eq35_e789_d_b31, eq35_e789_d_b32, eq35_e789_d_b33, eq35_e789_d_b34, eq35_e789_d_b35,) = {
    if s.b[466] {
        let eq35_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[211]);
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e785_d_n2: f64 = p.p355;
        let eq35_e785_d_n16: f64 = (-p.p355);
        let eq35_e786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq35_e785);
        let eq35_e787: f64 = (eq35_e782 + eq35_e786);
        let eq35_e787_d_n2: f64 = ((s.dn[211][2] * ddt_scale) + (eq35_e785_d_n2 * ddt_scale));
        let eq35_e787_d_n16: f64 = ((s.dn[211][16] * ddt_scale) + (eq35_e785_d_n16 * ddt_scale));
        (eq35_e787, (s.dn[211][0] * ddt_scale), (s.dn[211][1] * ddt_scale), eq35_e787_d_n2, (s.dn[211][3] * ddt_scale), (s.dn[211][4] * ddt_scale), (s.dn[211][5] * ddt_scale), (s.dn[211][6] * ddt_scale), (s.dn[211][7] * ddt_scale), (s.dn[211][8] * ddt_scale), (s.dn[211][9] * ddt_scale), (s.dn[211][10] * ddt_scale), (s.dn[211][11] * ddt_scale), (s.dn[211][12] * ddt_scale), (s.dn[211][13] * ddt_scale), (s.dn[211][14] * ddt_scale), (s.dn[211][15] * ddt_scale), eq35_e787_d_n16, (s.dn[211][17] * ddt_scale), (s.dn[211][18] * ddt_scale), (s.dn[211][19] * ddt_scale), (s.dn[211][20] * ddt_scale), (s.dn[211][21] * ddt_scale), (s.dn[211][22] * ddt_scale), (s.dn[211][23] * ddt_scale), (s.dn[211][24] * ddt_scale), (s.dn[211][25] * ddt_scale), (s.dn[211][26] * ddt_scale), (s.dn[211][27] * ddt_scale), (s.dn[211][28] * ddt_scale), (s.dn[211][29] * ddt_scale), (s.db[211][0] * ddt_scale), (s.db[211][1] * ddt_scale), (s.db[211][2] * ddt_scale), (s.db[211][3] * ddt_scale), (s.db[211][4] * ddt_scale), (s.db[211][5] * ddt_scale), (s.db[211][6] * ddt_scale), (s.db[211][7] * ddt_scale), (s.db[211][8] * ddt_scale), (s.db[211][9] * ddt_scale), (s.db[211][10] * ddt_scale), (s.db[211][11] * ddt_scale), (s.db[211][12] * ddt_scale), (s.db[211][13] * ddt_scale), (s.db[211][14] * ddt_scale), (s.db[211][15] * ddt_scale), (s.db[211][16] * ddt_scale), (s.db[211][17] * ddt_scale), (s.db[211][18] * ddt_scale), (s.db[211][19] * ddt_scale), (s.db[211][20] * ddt_scale), (s.db[211][21] * ddt_scale), (s.db[211][22] * ddt_scale), (s.db[211][23] * ddt_scale), (s.db[211][24] * ddt_scale), (s.db[211][25] * ddt_scale), (s.db[211][26] * ddt_scale), (s.db[211][27] * ddt_scale), (s.db[211][28] * ddt_scale), (s.db[211][29] * ddt_scale), (s.db[211][30] * ddt_scale), (s.db[211][31] * ddt_scale), (s.db[211][32] * ddt_scale), (s.db[211][33] * ddt_scale), (s.db[211][34] * ddt_scale), (s.db[211][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e789;
        let eq35_node_derivatives: [f64; 30] = [eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29];
        let eq35_branch_derivatives: [f64; 36] = [eq35_e789_d_b0, eq35_e789_d_b1, eq35_e789_d_b2, eq35_e789_d_b3, eq35_e789_d_b4, eq35_e789_d_b5, eq35_e789_d_b6, eq35_e789_d_b7, eq35_e789_d_b8, eq35_e789_d_b9, eq35_e789_d_b10, eq35_e789_d_b11, eq35_e789_d_b12, eq35_e789_d_b13, eq35_e789_d_b14, eq35_e789_d_b15, eq35_e789_d_b16, eq35_e789_d_b17, eq35_e789_d_b18, eq35_e789_d_b19, eq35_e789_d_b20, eq35_e789_d_b21, eq35_e789_d_b22, eq35_e789_d_b23, eq35_e789_d_b24, eq35_e789_d_b25, eq35_e789_d_b26, eq35_e789_d_b27, eq35_e789_d_b28, eq35_e789_d_b29, eq35_e789_d_b30, eq35_e789_d_b31, eq35_e789_d_b32, eq35_e789_d_b33, eq35_e789_d_b34, eq35_e789_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29, eq37_e803_d_b0, eq37_e803_d_b1, eq37_e803_d_b2, eq37_e803_d_b3, eq37_e803_d_b4, eq37_e803_d_b5, eq37_e803_d_b6, eq37_e803_d_b7, eq37_e803_d_b8, eq37_e803_d_b9, eq37_e803_d_b10, eq37_e803_d_b11, eq37_e803_d_b12, eq37_e803_d_b13, eq37_e803_d_b14, eq37_e803_d_b15, eq37_e803_d_b16, eq37_e803_d_b17, eq37_e803_d_b18, eq37_e803_d_b19, eq37_e803_d_b20, eq37_e803_d_b21, eq37_e803_d_b22, eq37_e803_d_b23, eq37_e803_d_b24, eq37_e803_d_b25, eq37_e803_d_b26, eq37_e803_d_b27, eq37_e803_d_b28, eq37_e803_d_b29, eq37_e803_d_b30, eq37_e803_d_b31, eq37_e803_d_b32, eq37_e803_d_b33, eq37_e803_d_b34, eq37_e803_d_b35,) = {
    if s.b[466] {
        let eq37_e796: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[213]);
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e799_d_n7: f64 = p.p355;
        let eq37_e799_d_n9: f64 = (-p.p355);
        let eq37_e800: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq37_e799);
        let eq37_e801: f64 = (eq37_e796 + eq37_e800);
        let eq37_e801_d_n7: f64 = ((s.dn[213][7] * ddt_scale) + (eq37_e799_d_n7 * ddt_scale));
        let eq37_e801_d_n9: f64 = ((s.dn[213][9] * ddt_scale) + (eq37_e799_d_n9 * ddt_scale));
        (eq37_e801, (s.dn[213][0] * ddt_scale), (s.dn[213][1] * ddt_scale), (s.dn[213][2] * ddt_scale), (s.dn[213][3] * ddt_scale), (s.dn[213][4] * ddt_scale), (s.dn[213][5] * ddt_scale), (s.dn[213][6] * ddt_scale), eq37_e801_d_n7, (s.dn[213][8] * ddt_scale), eq37_e801_d_n9, (s.dn[213][10] * ddt_scale), (s.dn[213][11] * ddt_scale), (s.dn[213][12] * ddt_scale), (s.dn[213][13] * ddt_scale), (s.dn[213][14] * ddt_scale), (s.dn[213][15] * ddt_scale), (s.dn[213][16] * ddt_scale), (s.dn[213][17] * ddt_scale), (s.dn[213][18] * ddt_scale), (s.dn[213][19] * ddt_scale), (s.dn[213][20] * ddt_scale), (s.dn[213][21] * ddt_scale), (s.dn[213][22] * ddt_scale), (s.dn[213][23] * ddt_scale), (s.dn[213][24] * ddt_scale), (s.dn[213][25] * ddt_scale), (s.dn[213][26] * ddt_scale), (s.dn[213][27] * ddt_scale), (s.dn[213][28] * ddt_scale), (s.dn[213][29] * ddt_scale), (s.db[213][0] * ddt_scale), (s.db[213][1] * ddt_scale), (s.db[213][2] * ddt_scale), (s.db[213][3] * ddt_scale), (s.db[213][4] * ddt_scale), (s.db[213][5] * ddt_scale), (s.db[213][6] * ddt_scale), (s.db[213][7] * ddt_scale), (s.db[213][8] * ddt_scale), (s.db[213][9] * ddt_scale), (s.db[213][10] * ddt_scale), (s.db[213][11] * ddt_scale), (s.db[213][12] * ddt_scale), (s.db[213][13] * ddt_scale), (s.db[213][14] * ddt_scale), (s.db[213][15] * ddt_scale), (s.db[213][16] * ddt_scale), (s.db[213][17] * ddt_scale), (s.db[213][18] * ddt_scale), (s.db[213][19] * ddt_scale), (s.db[213][20] * ddt_scale), (s.db[213][21] * ddt_scale), (s.db[213][22] * ddt_scale), (s.db[213][23] * ddt_scale), (s.db[213][24] * ddt_scale), (s.db[213][25] * ddt_scale), (s.db[213][26] * ddt_scale), (s.db[213][27] * ddt_scale), (s.db[213][28] * ddt_scale), (s.db[213][29] * ddt_scale), (s.db[213][30] * ddt_scale), (s.db[213][31] * ddt_scale), (s.db[213][32] * ddt_scale), (s.db[213][33] * ddt_scale), (s.db[213][34] * ddt_scale), (s.db[213][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e803;
        let eq37_node_derivatives: [f64; 30] = [eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29];
        let eq37_branch_derivatives: [f64; 36] = [eq37_e803_d_b0, eq37_e803_d_b1, eq37_e803_d_b2, eq37_e803_d_b3, eq37_e803_d_b4, eq37_e803_d_b5, eq37_e803_d_b6, eq37_e803_d_b7, eq37_e803_d_b8, eq37_e803_d_b9, eq37_e803_d_b10, eq37_e803_d_b11, eq37_e803_d_b12, eq37_e803_d_b13, eq37_e803_d_b14, eq37_e803_d_b15, eq37_e803_d_b16, eq37_e803_d_b17, eq37_e803_d_b18, eq37_e803_d_b19, eq37_e803_d_b20, eq37_e803_d_b21, eq37_e803_d_b22, eq37_e803_d_b23, eq37_e803_d_b24, eq37_e803_d_b25, eq37_e803_d_b26, eq37_e803_d_b27, eq37_e803_d_b28, eq37_e803_d_b29, eq37_e803_d_b30, eq37_e803_d_b31, eq37_e803_d_b32, eq37_e803_d_b33, eq37_e803_d_b34, eq37_e803_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29, eq38_e814_d_b0, eq38_e814_d_b1, eq38_e814_d_b2, eq38_e814_d_b3, eq38_e814_d_b4, eq38_e814_d_b5, eq38_e814_d_b6, eq38_e814_d_b7, eq38_e814_d_b8, eq38_e814_d_b9, eq38_e814_d_b10, eq38_e814_d_b11, eq38_e814_d_b12, eq38_e814_d_b13, eq38_e814_d_b14, eq38_e814_d_b15, eq38_e814_d_b16, eq38_e814_d_b17, eq38_e814_d_b18, eq38_e814_d_b19, eq38_e814_d_b20, eq38_e814_d_b21, eq38_e814_d_b22, eq38_e814_d_b23, eq38_e814_d_b24, eq38_e814_d_b25, eq38_e814_d_b26, eq38_e814_d_b27, eq38_e814_d_b28, eq38_e814_d_b29, eq38_e814_d_b30, eq38_e814_d_b31, eq38_e814_d_b32, eq38_e814_d_b33, eq38_e814_d_b34, eq38_e814_d_b35,) = {
    if (!s.b[466]) {
        let eq38_e807: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[209]);
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e810_d_n2: f64 = p.p355;
        let eq38_e810_d_n16: f64 = (-p.p355);
        let eq38_e811: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq38_e810);
        let eq38_e812: f64 = (eq38_e807 + eq38_e811);
        let eq38_e812_d_n2: f64 = ((s.dn[209][2] * ddt_scale) + (eq38_e810_d_n2 * ddt_scale));
        let eq38_e812_d_n16: f64 = ((s.dn[209][16] * ddt_scale) + (eq38_e810_d_n16 * ddt_scale));
        (eq38_e812, (s.dn[209][0] * ddt_scale), (s.dn[209][1] * ddt_scale), eq38_e812_d_n2, (s.dn[209][3] * ddt_scale), (s.dn[209][4] * ddt_scale), (s.dn[209][5] * ddt_scale), (s.dn[209][6] * ddt_scale), (s.dn[209][7] * ddt_scale), (s.dn[209][8] * ddt_scale), (s.dn[209][9] * ddt_scale), (s.dn[209][10] * ddt_scale), (s.dn[209][11] * ddt_scale), (s.dn[209][12] * ddt_scale), (s.dn[209][13] * ddt_scale), (s.dn[209][14] * ddt_scale), (s.dn[209][15] * ddt_scale), eq38_e812_d_n16, (s.dn[209][17] * ddt_scale), (s.dn[209][18] * ddt_scale), (s.dn[209][19] * ddt_scale), (s.dn[209][20] * ddt_scale), (s.dn[209][21] * ddt_scale), (s.dn[209][22] * ddt_scale), (s.dn[209][23] * ddt_scale), (s.dn[209][24] * ddt_scale), (s.dn[209][25] * ddt_scale), (s.dn[209][26] * ddt_scale), (s.dn[209][27] * ddt_scale), (s.dn[209][28] * ddt_scale), (s.dn[209][29] * ddt_scale), (s.db[209][0] * ddt_scale), (s.db[209][1] * ddt_scale), (s.db[209][2] * ddt_scale), (s.db[209][3] * ddt_scale), (s.db[209][4] * ddt_scale), (s.db[209][5] * ddt_scale), (s.db[209][6] * ddt_scale), (s.db[209][7] * ddt_scale), (s.db[209][8] * ddt_scale), (s.db[209][9] * ddt_scale), (s.db[209][10] * ddt_scale), (s.db[209][11] * ddt_scale), (s.db[209][12] * ddt_scale), (s.db[209][13] * ddt_scale), (s.db[209][14] * ddt_scale), (s.db[209][15] * ddt_scale), (s.db[209][16] * ddt_scale), (s.db[209][17] * ddt_scale), (s.db[209][18] * ddt_scale), (s.db[209][19] * ddt_scale), (s.db[209][20] * ddt_scale), (s.db[209][21] * ddt_scale), (s.db[209][22] * ddt_scale), (s.db[209][23] * ddt_scale), (s.db[209][24] * ddt_scale), (s.db[209][25] * ddt_scale), (s.db[209][26] * ddt_scale), (s.db[209][27] * ddt_scale), (s.db[209][28] * ddt_scale), (s.db[209][29] * ddt_scale), (s.db[209][30] * ddt_scale), (s.db[209][31] * ddt_scale), (s.db[209][32] * ddt_scale), (s.db[209][33] * ddt_scale), (s.db[209][34] * ddt_scale), (s.db[209][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e814;
        let eq38_node_derivatives: [f64; 30] = [eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29];
        let eq38_branch_derivatives: [f64; 36] = [eq38_e814_d_b0, eq38_e814_d_b1, eq38_e814_d_b2, eq38_e814_d_b3, eq38_e814_d_b4, eq38_e814_d_b5, eq38_e814_d_b6, eq38_e814_d_b7, eq38_e814_d_b8, eq38_e814_d_b9, eq38_e814_d_b10, eq38_e814_d_b11, eq38_e814_d_b12, eq38_e814_d_b13, eq38_e814_d_b14, eq38_e814_d_b15, eq38_e814_d_b16, eq38_e814_d_b17, eq38_e814_d_b18, eq38_e814_d_b19, eq38_e814_d_b20, eq38_e814_d_b21, eq38_e814_d_b22, eq38_e814_d_b23, eq38_e814_d_b24, eq38_e814_d_b25, eq38_e814_d_b26, eq38_e814_d_b27, eq38_e814_d_b28, eq38_e814_d_b29, eq38_e814_d_b30, eq38_e814_d_b31, eq38_e814_d_b32, eq38_e814_d_b33, eq38_e814_d_b34, eq38_e814_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29, eq39_e825_d_b0, eq39_e825_d_b1, eq39_e825_d_b2, eq39_e825_d_b3, eq39_e825_d_b4, eq39_e825_d_b5, eq39_e825_d_b6, eq39_e825_d_b7, eq39_e825_d_b8, eq39_e825_d_b9, eq39_e825_d_b10, eq39_e825_d_b11, eq39_e825_d_b12, eq39_e825_d_b13, eq39_e825_d_b14, eq39_e825_d_b15, eq39_e825_d_b16, eq39_e825_d_b17, eq39_e825_d_b18, eq39_e825_d_b19, eq39_e825_d_b20, eq39_e825_d_b21, eq39_e825_d_b22, eq39_e825_d_b23, eq39_e825_d_b24, eq39_e825_d_b25, eq39_e825_d_b26, eq39_e825_d_b27, eq39_e825_d_b28, eq39_e825_d_b29, eq39_e825_d_b30, eq39_e825_d_b31, eq39_e825_d_b32, eq39_e825_d_b33, eq39_e825_d_b34, eq39_e825_d_b35,) = {
    if (!s.b[466]) {
        let eq39_e818: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[210]);
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e821_d_n2: f64 = p.p355;
        let eq39_e821_d_n17: f64 = (-p.p355);
        let eq39_e822: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq39_e821);
        let eq39_e823: f64 = (eq39_e818 + eq39_e822);
        let eq39_e823_d_n2: f64 = ((s.dn[210][2] * ddt_scale) + (eq39_e821_d_n2 * ddt_scale));
        let eq39_e823_d_n17: f64 = ((s.dn[210][17] * ddt_scale) + (eq39_e821_d_n17 * ddt_scale));
        (eq39_e823, (s.dn[210][0] * ddt_scale), (s.dn[210][1] * ddt_scale), eq39_e823_d_n2, (s.dn[210][3] * ddt_scale), (s.dn[210][4] * ddt_scale), (s.dn[210][5] * ddt_scale), (s.dn[210][6] * ddt_scale), (s.dn[210][7] * ddt_scale), (s.dn[210][8] * ddt_scale), (s.dn[210][9] * ddt_scale), (s.dn[210][10] * ddt_scale), (s.dn[210][11] * ddt_scale), (s.dn[210][12] * ddt_scale), (s.dn[210][13] * ddt_scale), (s.dn[210][14] * ddt_scale), (s.dn[210][15] * ddt_scale), (s.dn[210][16] * ddt_scale), eq39_e823_d_n17, (s.dn[210][18] * ddt_scale), (s.dn[210][19] * ddt_scale), (s.dn[210][20] * ddt_scale), (s.dn[210][21] * ddt_scale), (s.dn[210][22] * ddt_scale), (s.dn[210][23] * ddt_scale), (s.dn[210][24] * ddt_scale), (s.dn[210][25] * ddt_scale), (s.dn[210][26] * ddt_scale), (s.dn[210][27] * ddt_scale), (s.dn[210][28] * ddt_scale), (s.dn[210][29] * ddt_scale), (s.db[210][0] * ddt_scale), (s.db[210][1] * ddt_scale), (s.db[210][2] * ddt_scale), (s.db[210][3] * ddt_scale), (s.db[210][4] * ddt_scale), (s.db[210][5] * ddt_scale), (s.db[210][6] * ddt_scale), (s.db[210][7] * ddt_scale), (s.db[210][8] * ddt_scale), (s.db[210][9] * ddt_scale), (s.db[210][10] * ddt_scale), (s.db[210][11] * ddt_scale), (s.db[210][12] * ddt_scale), (s.db[210][13] * ddt_scale), (s.db[210][14] * ddt_scale), (s.db[210][15] * ddt_scale), (s.db[210][16] * ddt_scale), (s.db[210][17] * ddt_scale), (s.db[210][18] * ddt_scale), (s.db[210][19] * ddt_scale), (s.db[210][20] * ddt_scale), (s.db[210][21] * ddt_scale), (s.db[210][22] * ddt_scale), (s.db[210][23] * ddt_scale), (s.db[210][24] * ddt_scale), (s.db[210][25] * ddt_scale), (s.db[210][26] * ddt_scale), (s.db[210][27] * ddt_scale), (s.db[210][28] * ddt_scale), (s.db[210][29] * ddt_scale), (s.db[210][30] * ddt_scale), (s.db[210][31] * ddt_scale), (s.db[210][32] * ddt_scale), (s.db[210][33] * ddt_scale), (s.db[210][34] * ddt_scale), (s.db[210][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e825;
        let eq39_node_derivatives: [f64; 30] = [eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29];
        let eq39_branch_derivatives: [f64; 36] = [eq39_e825_d_b0, eq39_e825_d_b1, eq39_e825_d_b2, eq39_e825_d_b3, eq39_e825_d_b4, eq39_e825_d_b5, eq39_e825_d_b6, eq39_e825_d_b7, eq39_e825_d_b8, eq39_e825_d_b9, eq39_e825_d_b10, eq39_e825_d_b11, eq39_e825_d_b12, eq39_e825_d_b13, eq39_e825_d_b14, eq39_e825_d_b15, eq39_e825_d_b16, eq39_e825_d_b17, eq39_e825_d_b18, eq39_e825_d_b19, eq39_e825_d_b20, eq39_e825_d_b21, eq39_e825_d_b22, eq39_e825_d_b23, eq39_e825_d_b24, eq39_e825_d_b25, eq39_e825_d_b26, eq39_e825_d_b27, eq39_e825_d_b28, eq39_e825_d_b29, eq39_e825_d_b30, eq39_e825_d_b31, eq39_e825_d_b32, eq39_e825_d_b33, eq39_e825_d_b34, eq39_e825_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(17),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29, eq40_e836_d_b0, eq40_e836_d_b1, eq40_e836_d_b2, eq40_e836_d_b3, eq40_e836_d_b4, eq40_e836_d_b5, eq40_e836_d_b6, eq40_e836_d_b7, eq40_e836_d_b8, eq40_e836_d_b9, eq40_e836_d_b10, eq40_e836_d_b11, eq40_e836_d_b12, eq40_e836_d_b13, eq40_e836_d_b14, eq40_e836_d_b15, eq40_e836_d_b16, eq40_e836_d_b17, eq40_e836_d_b18, eq40_e836_d_b19, eq40_e836_d_b20, eq40_e836_d_b21, eq40_e836_d_b22, eq40_e836_d_b23, eq40_e836_d_b24, eq40_e836_d_b25, eq40_e836_d_b26, eq40_e836_d_b27, eq40_e836_d_b28, eq40_e836_d_b29, eq40_e836_d_b30, eq40_e836_d_b31, eq40_e836_d_b32, eq40_e836_d_b33, eq40_e836_d_b34, eq40_e836_d_b35,) = {
    if (!s.b[466]) {
        let eq40_e829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[211]);
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e832_d_n7: f64 = p.p355;
        let eq40_e832_d_n16: f64 = (-p.p355);
        let eq40_e833: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, eq40_e832);
        let eq40_e834: f64 = (eq40_e829 + eq40_e833);
        let eq40_e834_d_n7: f64 = ((s.dn[211][7] * ddt_scale) + (eq40_e832_d_n7 * ddt_scale));
        let eq40_e834_d_n16: f64 = ((s.dn[211][16] * ddt_scale) + (eq40_e832_d_n16 * ddt_scale));
        (eq40_e834, (s.dn[211][0] * ddt_scale), (s.dn[211][1] * ddt_scale), (s.dn[211][2] * ddt_scale), (s.dn[211][3] * ddt_scale), (s.dn[211][4] * ddt_scale), (s.dn[211][5] * ddt_scale), (s.dn[211][6] * ddt_scale), eq40_e834_d_n7, (s.dn[211][8] * ddt_scale), (s.dn[211][9] * ddt_scale), (s.dn[211][10] * ddt_scale), (s.dn[211][11] * ddt_scale), (s.dn[211][12] * ddt_scale), (s.dn[211][13] * ddt_scale), (s.dn[211][14] * ddt_scale), (s.dn[211][15] * ddt_scale), eq40_e834_d_n16, (s.dn[211][17] * ddt_scale), (s.dn[211][18] * ddt_scale), (s.dn[211][19] * ddt_scale), (s.dn[211][20] * ddt_scale), (s.dn[211][21] * ddt_scale), (s.dn[211][22] * ddt_scale), (s.dn[211][23] * ddt_scale), (s.dn[211][24] * ddt_scale), (s.dn[211][25] * ddt_scale), (s.dn[211][26] * ddt_scale), (s.dn[211][27] * ddt_scale), (s.dn[211][28] * ddt_scale), (s.dn[211][29] * ddt_scale), (s.db[211][0] * ddt_scale), (s.db[211][1] * ddt_scale), (s.db[211][2] * ddt_scale), (s.db[211][3] * ddt_scale), (s.db[211][4] * ddt_scale), (s.db[211][5] * ddt_scale), (s.db[211][6] * ddt_scale), (s.db[211][7] * ddt_scale), (s.db[211][8] * ddt_scale), (s.db[211][9] * ddt_scale), (s.db[211][10] * ddt_scale), (s.db[211][11] * ddt_scale), (s.db[211][12] * ddt_scale), (s.db[211][13] * ddt_scale), (s.db[211][14] * ddt_scale), (s.db[211][15] * ddt_scale), (s.db[211][16] * ddt_scale), (s.db[211][17] * ddt_scale), (s.db[211][18] * ddt_scale), (s.db[211][19] * ddt_scale), (s.db[211][20] * ddt_scale), (s.db[211][21] * ddt_scale), (s.db[211][22] * ddt_scale), (s.db[211][23] * ddt_scale), (s.db[211][24] * ddt_scale), (s.db[211][25] * ddt_scale), (s.db[211][26] * ddt_scale), (s.db[211][27] * ddt_scale), (s.db[211][28] * ddt_scale), (s.db[211][29] * ddt_scale), (s.db[211][30] * ddt_scale), (s.db[211][31] * ddt_scale), (s.db[211][32] * ddt_scale), (s.db[211][33] * ddt_scale), (s.db[211][34] * ddt_scale), (s.db[211][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e836;
        let eq40_node_derivatives: [f64; 30] = [eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29];
        let eq40_branch_derivatives: [f64; 36] = [eq40_e836_d_b0, eq40_e836_d_b1, eq40_e836_d_b2, eq40_e836_d_b3, eq40_e836_d_b4, eq40_e836_d_b5, eq40_e836_d_b6, eq40_e836_d_b7, eq40_e836_d_b8, eq40_e836_d_b9, eq40_e836_d_b10, eq40_e836_d_b11, eq40_e836_d_b12, eq40_e836_d_b13, eq40_e836_d_b14, eq40_e836_d_b15, eq40_e836_d_b16, eq40_e836_d_b17, eq40_e836_d_b18, eq40_e836_d_b19, eq40_e836_d_b20, eq40_e836_d_b21, eq40_e836_d_b22, eq40_e836_d_b23, eq40_e836_d_b24, eq40_e836_d_b25, eq40_e836_d_b26, eq40_e836_d_b27, eq40_e836_d_b28, eq40_e836_d_b29, eq40_e836_d_b30, eq40_e836_d_b31, eq40_e836_d_b32, eq40_e836_d_b33, eq40_e836_d_b34, eq40_e836_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(16),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq43_e848: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, s.v[212]);
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e851_d_n3: f64 = p.p355;
        let eq43_e851_d_n16: f64 = (-p.p355);
        let eq43_e852: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq43_e851);
        let eq43_e853: f64 = (eq43_e848 + eq43_e852);
        let eq43_e853_d_n3: f64 = ((s.dn[212][3] * ddt_scale) + (eq43_e851_d_n3 * ddt_scale));
        let eq43_e853_d_n16: f64 = ((s.dn[212][16] * ddt_scale) + (eq43_e851_d_n16 * ddt_scale));
        let eq43_value: f64 = eq43_e853;
        let eq43_node_derivatives: [f64; 30] = [(s.dn[212][0] * ddt_scale), (s.dn[212][1] * ddt_scale), (s.dn[212][2] * ddt_scale), eq43_e853_d_n3, (s.dn[212][4] * ddt_scale), (s.dn[212][5] * ddt_scale), (s.dn[212][6] * ddt_scale), (s.dn[212][7] * ddt_scale), (s.dn[212][8] * ddt_scale), (s.dn[212][9] * ddt_scale), (s.dn[212][10] * ddt_scale), (s.dn[212][11] * ddt_scale), (s.dn[212][12] * ddt_scale), (s.dn[212][13] * ddt_scale), (s.dn[212][14] * ddt_scale), (s.dn[212][15] * ddt_scale), eq43_e853_d_n16, (s.dn[212][17] * ddt_scale), (s.dn[212][18] * ddt_scale), (s.dn[212][19] * ddt_scale), (s.dn[212][20] * ddt_scale), (s.dn[212][21] * ddt_scale), (s.dn[212][22] * ddt_scale), (s.dn[212][23] * ddt_scale), (s.dn[212][24] * ddt_scale), (s.dn[212][25] * ddt_scale), (s.dn[212][26] * ddt_scale), (s.dn[212][27] * ddt_scale), (s.dn[212][28] * ddt_scale), (s.dn[212][29] * ddt_scale)];
        let eq43_branch_derivatives: [f64; 36] = [(s.db[212][0] * ddt_scale), (s.db[212][1] * ddt_scale), (s.db[212][2] * ddt_scale), (s.db[212][3] * ddt_scale), (s.db[212][4] * ddt_scale), (s.db[212][5] * ddt_scale), (s.db[212][6] * ddt_scale), (s.db[212][7] * ddt_scale), (s.db[212][8] * ddt_scale), (s.db[212][9] * ddt_scale), (s.db[212][10] * ddt_scale), (s.db[212][11] * ddt_scale), (s.db[212][12] * ddt_scale), (s.db[212][13] * ddt_scale), (s.db[212][14] * ddt_scale), (s.db[212][15] * ddt_scale), (s.db[212][16] * ddt_scale), (s.db[212][17] * ddt_scale), (s.db[212][18] * ddt_scale), (s.db[212][19] * ddt_scale), (s.db[212][20] * ddt_scale), (s.db[212][21] * ddt_scale), (s.db[212][22] * ddt_scale), (s.db[212][23] * ddt_scale), (s.db[212][24] * ddt_scale), (s.db[212][25] * ddt_scale), (s.db[212][26] * ddt_scale), (s.db[212][27] * ddt_scale), (s.db[212][28] * ddt_scale), (s.db[212][29] * ddt_scale), (s.db[212][30] * ddt_scale), (s.db[212][31] * ddt_scale), (s.db[212][32] * ddt_scale), (s.db[212][33] * ddt_scale), (s.db[212][34] * ddt_scale), (s.db[212][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(16),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e861, eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29, eq44_e861_d_b0, eq44_e861_d_b1, eq44_e861_d_b2, eq44_e861_d_b3, eq44_e861_d_b4, eq44_e861_d_b5, eq44_e861_d_b6, eq44_e861_d_b7, eq44_e861_d_b8, eq44_e861_d_b9, eq44_e861_d_b10, eq44_e861_d_b11, eq44_e861_d_b12, eq44_e861_d_b13, eq44_e861_d_b14, eq44_e861_d_b15, eq44_e861_d_b16, eq44_e861_d_b17, eq44_e861_d_b18, eq44_e861_d_b19, eq44_e861_d_b20, eq44_e861_d_b21, eq44_e861_d_b22, eq44_e861_d_b23, eq44_e861_d_b24, eq44_e861_d_b25, eq44_e861_d_b26, eq44_e861_d_b27, eq44_e861_d_b28, eq44_e861_d_b29, eq44_e861_d_b30, eq44_e861_d_b31, eq44_e861_d_b32, eq44_e861_d_b33, eq44_e861_d_b34, eq44_e861_d_b35,) = {
    if s.b[467] {
        let eq44_e858: f64 = (s.v[0] * (nv16 - nv15));
        let eq44_e858_d_n15: f64 = (-s.v[0]);
        let eq44_e858_d_n16: f64 = s.v[0];
        let eq44_e859: f64 = (s.v[202] + eq44_e858);
        let eq44_e859_d_n15: f64 = (s.dn[202][15] + eq44_e858_d_n15);
        let eq44_e859_d_n16: f64 = (s.dn[202][16] + eq44_e858_d_n16);
        (eq44_e859, s.dn[202][0], s.dn[202][1], s.dn[202][2], s.dn[202][3], s.dn[202][4], s.dn[202][5], s.dn[202][6], s.dn[202][7], s.dn[202][8], s.dn[202][9], s.dn[202][10], s.dn[202][11], s.dn[202][12], s.dn[202][13], s.dn[202][14], eq44_e859_d_n15, eq44_e859_d_n16, s.dn[202][17], s.dn[202][18], s.dn[202][19], s.dn[202][20], s.dn[202][21], s.dn[202][22], s.dn[202][23], s.dn[202][24], s.dn[202][25], s.dn[202][26], s.dn[202][27], s.dn[202][28], s.dn[202][29], s.db[202][0], s.db[202][1], s.db[202][2], s.db[202][3], s.db[202][4], s.db[202][5], s.db[202][6], s.db[202][7], s.db[202][8], s.db[202][9], s.db[202][10], s.db[202][11], s.db[202][12], s.db[202][13], s.db[202][14], s.db[202][15], s.db[202][16], s.db[202][17], s.db[202][18], s.db[202][19], s.db[202][20], s.db[202][21], s.db[202][22], s.db[202][23], s.db[202][24], s.db[202][25], s.db[202][26], s.db[202][27], s.db[202][28], s.db[202][29], s.db[202][30], s.db[202][31], s.db[202][32], s.db[202][33], s.db[202][34], s.db[202][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e861;
        let eq44_node_derivatives: [f64; 30] = [eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29];
        let eq44_branch_derivatives: [f64; 36] = [eq44_e861_d_b0, eq44_e861_d_b1, eq44_e861_d_b2, eq44_e861_d_b3, eq44_e861_d_b4, eq44_e861_d_b5, eq44_e861_d_b6, eq44_e861_d_b7, eq44_e861_d_b8, eq44_e861_d_b9, eq44_e861_d_b10, eq44_e861_d_b11, eq44_e861_d_b12, eq44_e861_d_b13, eq44_e861_d_b14, eq44_e861_d_b15, eq44_e861_d_b16, eq44_e861_d_b17, eq44_e861_d_b18, eq44_e861_d_b19, eq44_e861_d_b20, eq44_e861_d_b21, eq44_e861_d_b22, eq44_e861_d_b23, eq44_e861_d_b24, eq44_e861_d_b25, eq44_e861_d_b26, eq44_e861_d_b27, eq44_e861_d_b28, eq44_e861_d_b29, eq44_e861_d_b30, eq44_e861_d_b31, eq44_e861_d_b32, eq44_e861_d_b33, eq44_e861_d_b34, eq44_e861_d_b35];
        stamper.stamp_current_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29, eq46_e876_d_b0, eq46_e876_d_b1, eq46_e876_d_b2, eq46_e876_d_b3, eq46_e876_d_b4, eq46_e876_d_b5, eq46_e876_d_b6, eq46_e876_d_b7, eq46_e876_d_b8, eq46_e876_d_b9, eq46_e876_d_b10, eq46_e876_d_b11, eq46_e876_d_b12, eq46_e876_d_b13, eq46_e876_d_b14, eq46_e876_d_b15, eq46_e876_d_b16, eq46_e876_d_b17, eq46_e876_d_b18, eq46_e876_d_b19, eq46_e876_d_b20, eq46_e876_d_b21, eq46_e876_d_b22, eq46_e876_d_b23, eq46_e876_d_b24, eq46_e876_d_b25, eq46_e876_d_b26, eq46_e876_d_b27, eq46_e876_d_b28, eq46_e876_d_b29, eq46_e876_d_b30, eq46_e876_d_b31, eq46_e876_d_b32, eq46_e876_d_b33, eq46_e876_d_b34, eq46_e876_d_b35,) = {
    if s.b[613] {
        let eq46_e869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, s.v[203]);
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e872_d_n7: f64 = p.p355;
        let eq46_e872_d_n15: f64 = (-p.p355);
        let eq46_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, eq46_e872);
        let eq46_e874: f64 = (eq46_e869 + eq46_e873);
        let eq46_e874_d_n7: f64 = ((s.dn[203][7] * ddt_scale) + (eq46_e872_d_n7 * ddt_scale));
        let eq46_e874_d_n15: f64 = ((s.dn[203][15] * ddt_scale) + (eq46_e872_d_n15 * ddt_scale));
        (eq46_e874, (s.dn[203][0] * ddt_scale), (s.dn[203][1] * ddt_scale), (s.dn[203][2] * ddt_scale), (s.dn[203][3] * ddt_scale), (s.dn[203][4] * ddt_scale), (s.dn[203][5] * ddt_scale), (s.dn[203][6] * ddt_scale), eq46_e874_d_n7, (s.dn[203][8] * ddt_scale), (s.dn[203][9] * ddt_scale), (s.dn[203][10] * ddt_scale), (s.dn[203][11] * ddt_scale), (s.dn[203][12] * ddt_scale), (s.dn[203][13] * ddt_scale), (s.dn[203][14] * ddt_scale), eq46_e874_d_n15, (s.dn[203][16] * ddt_scale), (s.dn[203][17] * ddt_scale), (s.dn[203][18] * ddt_scale), (s.dn[203][19] * ddt_scale), (s.dn[203][20] * ddt_scale), (s.dn[203][21] * ddt_scale), (s.dn[203][22] * ddt_scale), (s.dn[203][23] * ddt_scale), (s.dn[203][24] * ddt_scale), (s.dn[203][25] * ddt_scale), (s.dn[203][26] * ddt_scale), (s.dn[203][27] * ddt_scale), (s.dn[203][28] * ddt_scale), (s.dn[203][29] * ddt_scale), (s.db[203][0] * ddt_scale), (s.db[203][1] * ddt_scale), (s.db[203][2] * ddt_scale), (s.db[203][3] * ddt_scale), (s.db[203][4] * ddt_scale), (s.db[203][5] * ddt_scale), (s.db[203][6] * ddt_scale), (s.db[203][7] * ddt_scale), (s.db[203][8] * ddt_scale), (s.db[203][9] * ddt_scale), (s.db[203][10] * ddt_scale), (s.db[203][11] * ddt_scale), (s.db[203][12] * ddt_scale), (s.db[203][13] * ddt_scale), (s.db[203][14] * ddt_scale), (s.db[203][15] * ddt_scale), (s.db[203][16] * ddt_scale), (s.db[203][17] * ddt_scale), (s.db[203][18] * ddt_scale), (s.db[203][19] * ddt_scale), (s.db[203][20] * ddt_scale), (s.db[203][21] * ddt_scale), (s.db[203][22] * ddt_scale), (s.db[203][23] * ddt_scale), (s.db[203][24] * ddt_scale), (s.db[203][25] * ddt_scale), (s.db[203][26] * ddt_scale), (s.db[203][27] * ddt_scale), (s.db[203][28] * ddt_scale), (s.db[203][29] * ddt_scale), (s.db[203][30] * ddt_scale), (s.db[203][31] * ddt_scale), (s.db[203][32] * ddt_scale), (s.db[203][33] * ddt_scale), (s.db[203][34] * ddt_scale), (s.db[203][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e876;
        let eq46_node_derivatives: [f64; 30] = [eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29];
        let eq46_branch_derivatives: [f64; 36] = [eq46_e876_d_b0, eq46_e876_d_b1, eq46_e876_d_b2, eq46_e876_d_b3, eq46_e876_d_b4, eq46_e876_d_b5, eq46_e876_d_b6, eq46_e876_d_b7, eq46_e876_d_b8, eq46_e876_d_b9, eq46_e876_d_b10, eq46_e876_d_b11, eq46_e876_d_b12, eq46_e876_d_b13, eq46_e876_d_b14, eq46_e876_d_b15, eq46_e876_d_b16, eq46_e876_d_b17, eq46_e876_d_b18, eq46_e876_d_b19, eq46_e876_d_b20, eq46_e876_d_b21, eq46_e876_d_b22, eq46_e876_d_b23, eq46_e876_d_b24, eq46_e876_d_b25, eq46_e876_d_b26, eq46_e876_d_b27, eq46_e876_d_b28, eq46_e876_d_b29, eq46_e876_d_b30, eq46_e876_d_b31, eq46_e876_d_b32, eq46_e876_d_b33, eq46_e876_d_b34, eq46_e876_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(15),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29, eq47_e886_d_b0, eq47_e886_d_b1, eq47_e886_d_b2, eq47_e886_d_b3, eq47_e886_d_b4, eq47_e886_d_b5, eq47_e886_d_b6, eq47_e886_d_b7, eq47_e886_d_b8, eq47_e886_d_b9, eq47_e886_d_b10, eq47_e886_d_b11, eq47_e886_d_b12, eq47_e886_d_b13, eq47_e886_d_b14, eq47_e886_d_b15, eq47_e886_d_b16, eq47_e886_d_b17, eq47_e886_d_b18, eq47_e886_d_b19, eq47_e886_d_b20, eq47_e886_d_b21, eq47_e886_d_b22, eq47_e886_d_b23, eq47_e886_d_b24, eq47_e886_d_b25, eq47_e886_d_b26, eq47_e886_d_b27, eq47_e886_d_b28, eq47_e886_d_b29, eq47_e886_d_b30, eq47_e886_d_b31, eq47_e886_d_b32, eq47_e886_d_b33, eq47_e886_d_b34, eq47_e886_d_b35,) = {
    if s.b[613] {
        let eq47_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, s.v[204]);
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e882_d_n7: f64 = p.p355;
        let eq47_e882_d_n16: f64 = (-p.p355);
        let eq47_e883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 23, eq47_e882);
        let eq47_e884: f64 = (eq47_e879 + eq47_e883);
        let eq47_e884_d_n7: f64 = ((s.dn[204][7] * ddt_scale) + (eq47_e882_d_n7 * ddt_scale));
        let eq47_e884_d_n16: f64 = ((s.dn[204][16] * ddt_scale) + (eq47_e882_d_n16 * ddt_scale));
        (eq47_e884, (s.dn[204][0] * ddt_scale), (s.dn[204][1] * ddt_scale), (s.dn[204][2] * ddt_scale), (s.dn[204][3] * ddt_scale), (s.dn[204][4] * ddt_scale), (s.dn[204][5] * ddt_scale), (s.dn[204][6] * ddt_scale), eq47_e884_d_n7, (s.dn[204][8] * ddt_scale), (s.dn[204][9] * ddt_scale), (s.dn[204][10] * ddt_scale), (s.dn[204][11] * ddt_scale), (s.dn[204][12] * ddt_scale), (s.dn[204][13] * ddt_scale), (s.dn[204][14] * ddt_scale), (s.dn[204][15] * ddt_scale), eq47_e884_d_n16, (s.dn[204][17] * ddt_scale), (s.dn[204][18] * ddt_scale), (s.dn[204][19] * ddt_scale), (s.dn[204][20] * ddt_scale), (s.dn[204][21] * ddt_scale), (s.dn[204][22] * ddt_scale), (s.dn[204][23] * ddt_scale), (s.dn[204][24] * ddt_scale), (s.dn[204][25] * ddt_scale), (s.dn[204][26] * ddt_scale), (s.dn[204][27] * ddt_scale), (s.dn[204][28] * ddt_scale), (s.dn[204][29] * ddt_scale), (s.db[204][0] * ddt_scale), (s.db[204][1] * ddt_scale), (s.db[204][2] * ddt_scale), (s.db[204][3] * ddt_scale), (s.db[204][4] * ddt_scale), (s.db[204][5] * ddt_scale), (s.db[204][6] * ddt_scale), (s.db[204][7] * ddt_scale), (s.db[204][8] * ddt_scale), (s.db[204][9] * ddt_scale), (s.db[204][10] * ddt_scale), (s.db[204][11] * ddt_scale), (s.db[204][12] * ddt_scale), (s.db[204][13] * ddt_scale), (s.db[204][14] * ddt_scale), (s.db[204][15] * ddt_scale), (s.db[204][16] * ddt_scale), (s.db[204][17] * ddt_scale), (s.db[204][18] * ddt_scale), (s.db[204][19] * ddt_scale), (s.db[204][20] * ddt_scale), (s.db[204][21] * ddt_scale), (s.db[204][22] * ddt_scale), (s.db[204][23] * ddt_scale), (s.db[204][24] * ddt_scale), (s.db[204][25] * ddt_scale), (s.db[204][26] * ddt_scale), (s.db[204][27] * ddt_scale), (s.db[204][28] * ddt_scale), (s.db[204][29] * ddt_scale), (s.db[204][30] * ddt_scale), (s.db[204][31] * ddt_scale), (s.db[204][32] * ddt_scale), (s.db[204][33] * ddt_scale), (s.db[204][34] * ddt_scale), (s.db[204][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e886;
        let eq47_node_derivatives: [f64; 30] = [eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29];
        let eq47_branch_derivatives: [f64; 36] = [eq47_e886_d_b0, eq47_e886_d_b1, eq47_e886_d_b2, eq47_e886_d_b3, eq47_e886_d_b4, eq47_e886_d_b5, eq47_e886_d_b6, eq47_e886_d_b7, eq47_e886_d_b8, eq47_e886_d_b9, eq47_e886_d_b10, eq47_e886_d_b11, eq47_e886_d_b12, eq47_e886_d_b13, eq47_e886_d_b14, eq47_e886_d_b15, eq47_e886_d_b16, eq47_e886_d_b17, eq47_e886_d_b18, eq47_e886_d_b19, eq47_e886_d_b20, eq47_e886_d_b21, eq47_e886_d_b22, eq47_e886_d_b23, eq47_e886_d_b24, eq47_e886_d_b25, eq47_e886_d_b26, eq47_e886_d_b27, eq47_e886_d_b28, eq47_e886_d_b29, eq47_e886_d_b30, eq47_e886_d_b31, eq47_e886_d_b32, eq47_e886_d_b33, eq47_e886_d_b34, eq47_e886_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(16),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29, eq48_e896_d_b0, eq48_e896_d_b1, eq48_e896_d_b2, eq48_e896_d_b3, eq48_e896_d_b4, eq48_e896_d_b5, eq48_e896_d_b6, eq48_e896_d_b7, eq48_e896_d_b8, eq48_e896_d_b9, eq48_e896_d_b10, eq48_e896_d_b11, eq48_e896_d_b12, eq48_e896_d_b13, eq48_e896_d_b14, eq48_e896_d_b15, eq48_e896_d_b16, eq48_e896_d_b17, eq48_e896_d_b18, eq48_e896_d_b19, eq48_e896_d_b20, eq48_e896_d_b21, eq48_e896_d_b22, eq48_e896_d_b23, eq48_e896_d_b24, eq48_e896_d_b25, eq48_e896_d_b26, eq48_e896_d_b27, eq48_e896_d_b28, eq48_e896_d_b29, eq48_e896_d_b30, eq48_e896_d_b31, eq48_e896_d_b32, eq48_e896_d_b33, eq48_e896_d_b34, eq48_e896_d_b35,) = {
    if s.b[613] {
        let eq48_e889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 24, s.v[205]);
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e892_d_n2: f64 = p.p355;
        let eq48_e892_d_n15: f64 = (-p.p355);
        let eq48_e893: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 25, eq48_e892);
        let eq48_e894: f64 = (eq48_e889 + eq48_e893);
        let eq48_e894_d_n2: f64 = ((s.dn[205][2] * ddt_scale) + (eq48_e892_d_n2 * ddt_scale));
        let eq48_e894_d_n15: f64 = ((s.dn[205][15] * ddt_scale) + (eq48_e892_d_n15 * ddt_scale));
        (eq48_e894, (s.dn[205][0] * ddt_scale), (s.dn[205][1] * ddt_scale), eq48_e894_d_n2, (s.dn[205][3] * ddt_scale), (s.dn[205][4] * ddt_scale), (s.dn[205][5] * ddt_scale), (s.dn[205][6] * ddt_scale), (s.dn[205][7] * ddt_scale), (s.dn[205][8] * ddt_scale), (s.dn[205][9] * ddt_scale), (s.dn[205][10] * ddt_scale), (s.dn[205][11] * ddt_scale), (s.dn[205][12] * ddt_scale), (s.dn[205][13] * ddt_scale), (s.dn[205][14] * ddt_scale), eq48_e894_d_n15, (s.dn[205][16] * ddt_scale), (s.dn[205][17] * ddt_scale), (s.dn[205][18] * ddt_scale), (s.dn[205][19] * ddt_scale), (s.dn[205][20] * ddt_scale), (s.dn[205][21] * ddt_scale), (s.dn[205][22] * ddt_scale), (s.dn[205][23] * ddt_scale), (s.dn[205][24] * ddt_scale), (s.dn[205][25] * ddt_scale), (s.dn[205][26] * ddt_scale), (s.dn[205][27] * ddt_scale), (s.dn[205][28] * ddt_scale), (s.dn[205][29] * ddt_scale), (s.db[205][0] * ddt_scale), (s.db[205][1] * ddt_scale), (s.db[205][2] * ddt_scale), (s.db[205][3] * ddt_scale), (s.db[205][4] * ddt_scale), (s.db[205][5] * ddt_scale), (s.db[205][6] * ddt_scale), (s.db[205][7] * ddt_scale), (s.db[205][8] * ddt_scale), (s.db[205][9] * ddt_scale), (s.db[205][10] * ddt_scale), (s.db[205][11] * ddt_scale), (s.db[205][12] * ddt_scale), (s.db[205][13] * ddt_scale), (s.db[205][14] * ddt_scale), (s.db[205][15] * ddt_scale), (s.db[205][16] * ddt_scale), (s.db[205][17] * ddt_scale), (s.db[205][18] * ddt_scale), (s.db[205][19] * ddt_scale), (s.db[205][20] * ddt_scale), (s.db[205][21] * ddt_scale), (s.db[205][22] * ddt_scale), (s.db[205][23] * ddt_scale), (s.db[205][24] * ddt_scale), (s.db[205][25] * ddt_scale), (s.db[205][26] * ddt_scale), (s.db[205][27] * ddt_scale), (s.db[205][28] * ddt_scale), (s.db[205][29] * ddt_scale), (s.db[205][30] * ddt_scale), (s.db[205][31] * ddt_scale), (s.db[205][32] * ddt_scale), (s.db[205][33] * ddt_scale), (s.db[205][34] * ddt_scale), (s.db[205][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e896;
        let eq48_node_derivatives: [f64; 30] = [eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29];
        let eq48_branch_derivatives: [f64; 36] = [eq48_e896_d_b0, eq48_e896_d_b1, eq48_e896_d_b2, eq48_e896_d_b3, eq48_e896_d_b4, eq48_e896_d_b5, eq48_e896_d_b6, eq48_e896_d_b7, eq48_e896_d_b8, eq48_e896_d_b9, eq48_e896_d_b10, eq48_e896_d_b11, eq48_e896_d_b12, eq48_e896_d_b13, eq48_e896_d_b14, eq48_e896_d_b15, eq48_e896_d_b16, eq48_e896_d_b17, eq48_e896_d_b18, eq48_e896_d_b19, eq48_e896_d_b20, eq48_e896_d_b21, eq48_e896_d_b22, eq48_e896_d_b23, eq48_e896_d_b24, eq48_e896_d_b25, eq48_e896_d_b26, eq48_e896_d_b27, eq48_e896_d_b28, eq48_e896_d_b29, eq48_e896_d_b30, eq48_e896_d_b31, eq48_e896_d_b32, eq48_e896_d_b33, eq48_e896_d_b34, eq48_e896_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29, eq50_e910_d_b0, eq50_e910_d_b1, eq50_e910_d_b2, eq50_e910_d_b3, eq50_e910_d_b4, eq50_e910_d_b5, eq50_e910_d_b6, eq50_e910_d_b7, eq50_e910_d_b8, eq50_e910_d_b9, eq50_e910_d_b10, eq50_e910_d_b11, eq50_e910_d_b12, eq50_e910_d_b13, eq50_e910_d_b14, eq50_e910_d_b15, eq50_e910_d_b16, eq50_e910_d_b17, eq50_e910_d_b18, eq50_e910_d_b19, eq50_e910_d_b20, eq50_e910_d_b21, eq50_e910_d_b22, eq50_e910_d_b23, eq50_e910_d_b24, eq50_e910_d_b25, eq50_e910_d_b26, eq50_e910_d_b27, eq50_e910_d_b28, eq50_e910_d_b29, eq50_e910_d_b30, eq50_e910_d_b31, eq50_e910_d_b32, eq50_e910_d_b33, eq50_e910_d_b34, eq50_e910_d_b35,) = {
    if s.b[613] {
        let eq50_e903: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 26, s.v[207]);
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e906_d_n7: f64 = p.p355;
        let eq50_e906_d_n9: f64 = (-p.p355);
        let eq50_e907: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 27, eq50_e906);
        let eq50_e908: f64 = (eq50_e903 + eq50_e907);
        let eq50_e908_d_n7: f64 = ((s.dn[207][7] * ddt_scale) + (eq50_e906_d_n7 * ddt_scale));
        let eq50_e908_d_n9: f64 = ((s.dn[207][9] * ddt_scale) + (eq50_e906_d_n9 * ddt_scale));
        (eq50_e908, (s.dn[207][0] * ddt_scale), (s.dn[207][1] * ddt_scale), (s.dn[207][2] * ddt_scale), (s.dn[207][3] * ddt_scale), (s.dn[207][4] * ddt_scale), (s.dn[207][5] * ddt_scale), (s.dn[207][6] * ddt_scale), eq50_e908_d_n7, (s.dn[207][8] * ddt_scale), eq50_e908_d_n9, (s.dn[207][10] * ddt_scale), (s.dn[207][11] * ddt_scale), (s.dn[207][12] * ddt_scale), (s.dn[207][13] * ddt_scale), (s.dn[207][14] * ddt_scale), (s.dn[207][15] * ddt_scale), (s.dn[207][16] * ddt_scale), (s.dn[207][17] * ddt_scale), (s.dn[207][18] * ddt_scale), (s.dn[207][19] * ddt_scale), (s.dn[207][20] * ddt_scale), (s.dn[207][21] * ddt_scale), (s.dn[207][22] * ddt_scale), (s.dn[207][23] * ddt_scale), (s.dn[207][24] * ddt_scale), (s.dn[207][25] * ddt_scale), (s.dn[207][26] * ddt_scale), (s.dn[207][27] * ddt_scale), (s.dn[207][28] * ddt_scale), (s.dn[207][29] * ddt_scale), (s.db[207][0] * ddt_scale), (s.db[207][1] * ddt_scale), (s.db[207][2] * ddt_scale), (s.db[207][3] * ddt_scale), (s.db[207][4] * ddt_scale), (s.db[207][5] * ddt_scale), (s.db[207][6] * ddt_scale), (s.db[207][7] * ddt_scale), (s.db[207][8] * ddt_scale), (s.db[207][9] * ddt_scale), (s.db[207][10] * ddt_scale), (s.db[207][11] * ddt_scale), (s.db[207][12] * ddt_scale), (s.db[207][13] * ddt_scale), (s.db[207][14] * ddt_scale), (s.db[207][15] * ddt_scale), (s.db[207][16] * ddt_scale), (s.db[207][17] * ddt_scale), (s.db[207][18] * ddt_scale), (s.db[207][19] * ddt_scale), (s.db[207][20] * ddt_scale), (s.db[207][21] * ddt_scale), (s.db[207][22] * ddt_scale), (s.db[207][23] * ddt_scale), (s.db[207][24] * ddt_scale), (s.db[207][25] * ddt_scale), (s.db[207][26] * ddt_scale), (s.db[207][27] * ddt_scale), (s.db[207][28] * ddt_scale), (s.db[207][29] * ddt_scale), (s.db[207][30] * ddt_scale), (s.db[207][31] * ddt_scale), (s.db[207][32] * ddt_scale), (s.db[207][33] * ddt_scale), (s.db[207][34] * ddt_scale), (s.db[207][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e910;
        let eq50_node_derivatives: [f64; 30] = [eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29];
        let eq50_branch_derivatives: [f64; 36] = [eq50_e910_d_b0, eq50_e910_d_b1, eq50_e910_d_b2, eq50_e910_d_b3, eq50_e910_d_b4, eq50_e910_d_b5, eq50_e910_d_b6, eq50_e910_d_b7, eq50_e910_d_b8, eq50_e910_d_b9, eq50_e910_d_b10, eq50_e910_d_b11, eq50_e910_d_b12, eq50_e910_d_b13, eq50_e910_d_b14, eq50_e910_d_b15, eq50_e910_d_b16, eq50_e910_d_b17, eq50_e910_d_b18, eq50_e910_d_b19, eq50_e910_d_b20, eq50_e910_d_b21, eq50_e910_d_b22, eq50_e910_d_b23, eq50_e910_d_b24, eq50_e910_d_b25, eq50_e910_d_b26, eq50_e910_d_b27, eq50_e910_d_b28, eq50_e910_d_b29, eq50_e910_d_b30, eq50_e910_d_b31, eq50_e910_d_b32, eq50_e910_d_b33, eq50_e910_d_b34, eq50_e910_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29, eq51_e921_d_b0, eq51_e921_d_b1, eq51_e921_d_b2, eq51_e921_d_b3, eq51_e921_d_b4, eq51_e921_d_b5, eq51_e921_d_b6, eq51_e921_d_b7, eq51_e921_d_b8, eq51_e921_d_b9, eq51_e921_d_b10, eq51_e921_d_b11, eq51_e921_d_b12, eq51_e921_d_b13, eq51_e921_d_b14, eq51_e921_d_b15, eq51_e921_d_b16, eq51_e921_d_b17, eq51_e921_d_b18, eq51_e921_d_b19, eq51_e921_d_b20, eq51_e921_d_b21, eq51_e921_d_b22, eq51_e921_d_b23, eq51_e921_d_b24, eq51_e921_d_b25, eq51_e921_d_b26, eq51_e921_d_b27, eq51_e921_d_b28, eq51_e921_d_b29, eq51_e921_d_b30, eq51_e921_d_b31, eq51_e921_d_b32, eq51_e921_d_b33, eq51_e921_d_b34, eq51_e921_d_b35,) = {
    if (!s.b[613]) {
        let eq51_e914: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 28, s.v[203]);
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e917_d_n2: f64 = p.p355;
        let eq51_e917_d_n15: f64 = (-p.p355);
        let eq51_e918: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 29, eq51_e917);
        let eq51_e919: f64 = (eq51_e914 + eq51_e918);
        let eq51_e919_d_n2: f64 = ((s.dn[203][2] * ddt_scale) + (eq51_e917_d_n2 * ddt_scale));
        let eq51_e919_d_n15: f64 = ((s.dn[203][15] * ddt_scale) + (eq51_e917_d_n15 * ddt_scale));
        (eq51_e919, (s.dn[203][0] * ddt_scale), (s.dn[203][1] * ddt_scale), eq51_e919_d_n2, (s.dn[203][3] * ddt_scale), (s.dn[203][4] * ddt_scale), (s.dn[203][5] * ddt_scale), (s.dn[203][6] * ddt_scale), (s.dn[203][7] * ddt_scale), (s.dn[203][8] * ddt_scale), (s.dn[203][9] * ddt_scale), (s.dn[203][10] * ddt_scale), (s.dn[203][11] * ddt_scale), (s.dn[203][12] * ddt_scale), (s.dn[203][13] * ddt_scale), (s.dn[203][14] * ddt_scale), eq51_e919_d_n15, (s.dn[203][16] * ddt_scale), (s.dn[203][17] * ddt_scale), (s.dn[203][18] * ddt_scale), (s.dn[203][19] * ddt_scale), (s.dn[203][20] * ddt_scale), (s.dn[203][21] * ddt_scale), (s.dn[203][22] * ddt_scale), (s.dn[203][23] * ddt_scale), (s.dn[203][24] * ddt_scale), (s.dn[203][25] * ddt_scale), (s.dn[203][26] * ddt_scale), (s.dn[203][27] * ddt_scale), (s.dn[203][28] * ddt_scale), (s.dn[203][29] * ddt_scale), (s.db[203][0] * ddt_scale), (s.db[203][1] * ddt_scale), (s.db[203][2] * ddt_scale), (s.db[203][3] * ddt_scale), (s.db[203][4] * ddt_scale), (s.db[203][5] * ddt_scale), (s.db[203][6] * ddt_scale), (s.db[203][7] * ddt_scale), (s.db[203][8] * ddt_scale), (s.db[203][9] * ddt_scale), (s.db[203][10] * ddt_scale), (s.db[203][11] * ddt_scale), (s.db[203][12] * ddt_scale), (s.db[203][13] * ddt_scale), (s.db[203][14] * ddt_scale), (s.db[203][15] * ddt_scale), (s.db[203][16] * ddt_scale), (s.db[203][17] * ddt_scale), (s.db[203][18] * ddt_scale), (s.db[203][19] * ddt_scale), (s.db[203][20] * ddt_scale), (s.db[203][21] * ddt_scale), (s.db[203][22] * ddt_scale), (s.db[203][23] * ddt_scale), (s.db[203][24] * ddt_scale), (s.db[203][25] * ddt_scale), (s.db[203][26] * ddt_scale), (s.db[203][27] * ddt_scale), (s.db[203][28] * ddt_scale), (s.db[203][29] * ddt_scale), (s.db[203][30] * ddt_scale), (s.db[203][31] * ddt_scale), (s.db[203][32] * ddt_scale), (s.db[203][33] * ddt_scale), (s.db[203][34] * ddt_scale), (s.db[203][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e921;
        let eq51_node_derivatives: [f64; 30] = [eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29];
        let eq51_branch_derivatives: [f64; 36] = [eq51_e921_d_b0, eq51_e921_d_b1, eq51_e921_d_b2, eq51_e921_d_b3, eq51_e921_d_b4, eq51_e921_d_b5, eq51_e921_d_b6, eq51_e921_d_b7, eq51_e921_d_b8, eq51_e921_d_b9, eq51_e921_d_b10, eq51_e921_d_b11, eq51_e921_d_b12, eq51_e921_d_b13, eq51_e921_d_b14, eq51_e921_d_b15, eq51_e921_d_b16, eq51_e921_d_b17, eq51_e921_d_b18, eq51_e921_d_b19, eq51_e921_d_b20, eq51_e921_d_b21, eq51_e921_d_b22, eq51_e921_d_b23, eq51_e921_d_b24, eq51_e921_d_b25, eq51_e921_d_b26, eq51_e921_d_b27, eq51_e921_d_b28, eq51_e921_d_b29, eq51_e921_d_b30, eq51_e921_d_b31, eq51_e921_d_b32, eq51_e921_d_b33, eq51_e921_d_b34, eq51_e921_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29, eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35,) = {
    if (!s.b[613]) {
        let eq52_e925: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 30, s.v[204]);
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e928_d_n2: f64 = p.p355;
        let eq52_e928_d_n16: f64 = (-p.p355);
        let eq52_e929: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 31, eq52_e928);
        let eq52_e930: f64 = (eq52_e925 + eq52_e929);
        let eq52_e930_d_n2: f64 = ((s.dn[204][2] * ddt_scale) + (eq52_e928_d_n2 * ddt_scale));
        let eq52_e930_d_n16: f64 = ((s.dn[204][16] * ddt_scale) + (eq52_e928_d_n16 * ddt_scale));
        (eq52_e930, (s.dn[204][0] * ddt_scale), (s.dn[204][1] * ddt_scale), eq52_e930_d_n2, (s.dn[204][3] * ddt_scale), (s.dn[204][4] * ddt_scale), (s.dn[204][5] * ddt_scale), (s.dn[204][6] * ddt_scale), (s.dn[204][7] * ddt_scale), (s.dn[204][8] * ddt_scale), (s.dn[204][9] * ddt_scale), (s.dn[204][10] * ddt_scale), (s.dn[204][11] * ddt_scale), (s.dn[204][12] * ddt_scale), (s.dn[204][13] * ddt_scale), (s.dn[204][14] * ddt_scale), (s.dn[204][15] * ddt_scale), eq52_e930_d_n16, (s.dn[204][17] * ddt_scale), (s.dn[204][18] * ddt_scale), (s.dn[204][19] * ddt_scale), (s.dn[204][20] * ddt_scale), (s.dn[204][21] * ddt_scale), (s.dn[204][22] * ddt_scale), (s.dn[204][23] * ddt_scale), (s.dn[204][24] * ddt_scale), (s.dn[204][25] * ddt_scale), (s.dn[204][26] * ddt_scale), (s.dn[204][27] * ddt_scale), (s.dn[204][28] * ddt_scale), (s.dn[204][29] * ddt_scale), (s.db[204][0] * ddt_scale), (s.db[204][1] * ddt_scale), (s.db[204][2] * ddt_scale), (s.db[204][3] * ddt_scale), (s.db[204][4] * ddt_scale), (s.db[204][5] * ddt_scale), (s.db[204][6] * ddt_scale), (s.db[204][7] * ddt_scale), (s.db[204][8] * ddt_scale), (s.db[204][9] * ddt_scale), (s.db[204][10] * ddt_scale), (s.db[204][11] * ddt_scale), (s.db[204][12] * ddt_scale), (s.db[204][13] * ddt_scale), (s.db[204][14] * ddt_scale), (s.db[204][15] * ddt_scale), (s.db[204][16] * ddt_scale), (s.db[204][17] * ddt_scale), (s.db[204][18] * ddt_scale), (s.db[204][19] * ddt_scale), (s.db[204][20] * ddt_scale), (s.db[204][21] * ddt_scale), (s.db[204][22] * ddt_scale), (s.db[204][23] * ddt_scale), (s.db[204][24] * ddt_scale), (s.db[204][25] * ddt_scale), (s.db[204][26] * ddt_scale), (s.db[204][27] * ddt_scale), (s.db[204][28] * ddt_scale), (s.db[204][29] * ddt_scale), (s.db[204][30] * ddt_scale), (s.db[204][31] * ddt_scale), (s.db[204][32] * ddt_scale), (s.db[204][33] * ddt_scale), (s.db[204][34] * ddt_scale), (s.db[204][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 30] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29];
        let eq52_branch_derivatives: [f64; 36] = [eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29, eq53_e943_d_b0, eq53_e943_d_b1, eq53_e943_d_b2, eq53_e943_d_b3, eq53_e943_d_b4, eq53_e943_d_b5, eq53_e943_d_b6, eq53_e943_d_b7, eq53_e943_d_b8, eq53_e943_d_b9, eq53_e943_d_b10, eq53_e943_d_b11, eq53_e943_d_b12, eq53_e943_d_b13, eq53_e943_d_b14, eq53_e943_d_b15, eq53_e943_d_b16, eq53_e943_d_b17, eq53_e943_d_b18, eq53_e943_d_b19, eq53_e943_d_b20, eq53_e943_d_b21, eq53_e943_d_b22, eq53_e943_d_b23, eq53_e943_d_b24, eq53_e943_d_b25, eq53_e943_d_b26, eq53_e943_d_b27, eq53_e943_d_b28, eq53_e943_d_b29, eq53_e943_d_b30, eq53_e943_d_b31, eq53_e943_d_b32, eq53_e943_d_b33, eq53_e943_d_b34, eq53_e943_d_b35,) = {
    if (!s.b[613]) {
        let eq53_e936: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 32, s.v[205]);
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e939_d_n7: f64 = p.p355;
        let eq53_e939_d_n15: f64 = (-p.p355);
        let eq53_e940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 33, eq53_e939);
        let eq53_e941: f64 = (eq53_e936 + eq53_e940);
        let eq53_e941_d_n7: f64 = ((s.dn[205][7] * ddt_scale) + (eq53_e939_d_n7 * ddt_scale));
        let eq53_e941_d_n15: f64 = ((s.dn[205][15] * ddt_scale) + (eq53_e939_d_n15 * ddt_scale));
        (eq53_e941, (s.dn[205][0] * ddt_scale), (s.dn[205][1] * ddt_scale), (s.dn[205][2] * ddt_scale), (s.dn[205][3] * ddt_scale), (s.dn[205][4] * ddt_scale), (s.dn[205][5] * ddt_scale), (s.dn[205][6] * ddt_scale), eq53_e941_d_n7, (s.dn[205][8] * ddt_scale), (s.dn[205][9] * ddt_scale), (s.dn[205][10] * ddt_scale), (s.dn[205][11] * ddt_scale), (s.dn[205][12] * ddt_scale), (s.dn[205][13] * ddt_scale), (s.dn[205][14] * ddt_scale), eq53_e941_d_n15, (s.dn[205][16] * ddt_scale), (s.dn[205][17] * ddt_scale), (s.dn[205][18] * ddt_scale), (s.dn[205][19] * ddt_scale), (s.dn[205][20] * ddt_scale), (s.dn[205][21] * ddt_scale), (s.dn[205][22] * ddt_scale), (s.dn[205][23] * ddt_scale), (s.dn[205][24] * ddt_scale), (s.dn[205][25] * ddt_scale), (s.dn[205][26] * ddt_scale), (s.dn[205][27] * ddt_scale), (s.dn[205][28] * ddt_scale), (s.dn[205][29] * ddt_scale), (s.db[205][0] * ddt_scale), (s.db[205][1] * ddt_scale), (s.db[205][2] * ddt_scale), (s.db[205][3] * ddt_scale), (s.db[205][4] * ddt_scale), (s.db[205][5] * ddt_scale), (s.db[205][6] * ddt_scale), (s.db[205][7] * ddt_scale), (s.db[205][8] * ddt_scale), (s.db[205][9] * ddt_scale), (s.db[205][10] * ddt_scale), (s.db[205][11] * ddt_scale), (s.db[205][12] * ddt_scale), (s.db[205][13] * ddt_scale), (s.db[205][14] * ddt_scale), (s.db[205][15] * ddt_scale), (s.db[205][16] * ddt_scale), (s.db[205][17] * ddt_scale), (s.db[205][18] * ddt_scale), (s.db[205][19] * ddt_scale), (s.db[205][20] * ddt_scale), (s.db[205][21] * ddt_scale), (s.db[205][22] * ddt_scale), (s.db[205][23] * ddt_scale), (s.db[205][24] * ddt_scale), (s.db[205][25] * ddt_scale), (s.db[205][26] * ddt_scale), (s.db[205][27] * ddt_scale), (s.db[205][28] * ddt_scale), (s.db[205][29] * ddt_scale), (s.db[205][30] * ddt_scale), (s.db[205][31] * ddt_scale), (s.db[205][32] * ddt_scale), (s.db[205][33] * ddt_scale), (s.db[205][34] * ddt_scale), (s.db[205][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e943;
        let eq53_node_derivatives: [f64; 30] = [eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29];
        let eq53_branch_derivatives: [f64; 36] = [eq53_e943_d_b0, eq53_e943_d_b1, eq53_e943_d_b2, eq53_e943_d_b3, eq53_e943_d_b4, eq53_e943_d_b5, eq53_e943_d_b6, eq53_e943_d_b7, eq53_e943_d_b8, eq53_e943_d_b9, eq53_e943_d_b10, eq53_e943_d_b11, eq53_e943_d_b12, eq53_e943_d_b13, eq53_e943_d_b14, eq53_e943_d_b15, eq53_e943_d_b16, eq53_e943_d_b17, eq53_e943_d_b18, eq53_e943_d_b19, eq53_e943_d_b20, eq53_e943_d_b21, eq53_e943_d_b22, eq53_e943_d_b23, eq53_e943_d_b24, eq53_e943_d_b25, eq53_e943_d_b26, eq53_e943_d_b27, eq53_e943_d_b28, eq53_e943_d_b29, eq53_e943_d_b30, eq53_e943_d_b31, eq53_e943_d_b32, eq53_e943_d_b33, eq53_e943_d_b34, eq53_e943_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(15),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let eq56_e955: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 34, s.v[206]);
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e958_d_n3: f64 = p.p355;
        let eq56_e958_d_n15: f64 = (-p.p355);
        let eq56_e959: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 35, eq56_e958);
        let eq56_e960: f64 = (eq56_e955 + eq56_e959);
        let eq56_e960_d_n3: f64 = ((s.dn[206][3] * ddt_scale) + (eq56_e958_d_n3 * ddt_scale));
        let eq56_e960_d_n15: f64 = ((s.dn[206][15] * ddt_scale) + (eq56_e958_d_n15 * ddt_scale));
        let eq56_value: f64 = eq56_e960;
        let eq56_node_derivatives: [f64; 30] = [(s.dn[206][0] * ddt_scale), (s.dn[206][1] * ddt_scale), (s.dn[206][2] * ddt_scale), eq56_e960_d_n3, (s.dn[206][4] * ddt_scale), (s.dn[206][5] * ddt_scale), (s.dn[206][6] * ddt_scale), (s.dn[206][7] * ddt_scale), (s.dn[206][8] * ddt_scale), (s.dn[206][9] * ddt_scale), (s.dn[206][10] * ddt_scale), (s.dn[206][11] * ddt_scale), (s.dn[206][12] * ddt_scale), (s.dn[206][13] * ddt_scale), (s.dn[206][14] * ddt_scale), eq56_e960_d_n15, (s.dn[206][16] * ddt_scale), (s.dn[206][17] * ddt_scale), (s.dn[206][18] * ddt_scale), (s.dn[206][19] * ddt_scale), (s.dn[206][20] * ddt_scale), (s.dn[206][21] * ddt_scale), (s.dn[206][22] * ddt_scale), (s.dn[206][23] * ddt_scale), (s.dn[206][24] * ddt_scale), (s.dn[206][25] * ddt_scale), (s.dn[206][26] * ddt_scale), (s.dn[206][27] * ddt_scale), (s.dn[206][28] * ddt_scale), (s.dn[206][29] * ddt_scale)];
        let eq56_branch_derivatives: [f64; 36] = [(s.db[206][0] * ddt_scale), (s.db[206][1] * ddt_scale), (s.db[206][2] * ddt_scale), (s.db[206][3] * ddt_scale), (s.db[206][4] * ddt_scale), (s.db[206][5] * ddt_scale), (s.db[206][6] * ddt_scale), (s.db[206][7] * ddt_scale), (s.db[206][8] * ddt_scale), (s.db[206][9] * ddt_scale), (s.db[206][10] * ddt_scale), (s.db[206][11] * ddt_scale), (s.db[206][12] * ddt_scale), (s.db[206][13] * ddt_scale), (s.db[206][14] * ddt_scale), (s.db[206][15] * ddt_scale), (s.db[206][16] * ddt_scale), (s.db[206][17] * ddt_scale), (s.db[206][18] * ddt_scale), (s.db[206][19] * ddt_scale), (s.db[206][20] * ddt_scale), (s.db[206][21] * ddt_scale), (s.db[206][22] * ddt_scale), (s.db[206][23] * ddt_scale), (s.db[206][24] * ddt_scale), (s.db[206][25] * ddt_scale), (s.db[206][26] * ddt_scale), (s.db[206][27] * ddt_scale), (s.db[206][28] * ddt_scale), (s.db[206][29] * ddt_scale), (s.db[206][30] * ddt_scale), (s.db[206][31] * ddt_scale), (s.db[206][32] * ddt_scale), (s.db[206][33] * ddt_scale), (s.db[206][34] * ddt_scale), (s.db[206][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(15),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e968, eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29, eq57_e968_d_b0, eq57_e968_d_b1, eq57_e968_d_b2, eq57_e968_d_b3, eq57_e968_d_b4, eq57_e968_d_b5, eq57_e968_d_b6, eq57_e968_d_b7, eq57_e968_d_b8, eq57_e968_d_b9, eq57_e968_d_b10, eq57_e968_d_b11, eq57_e968_d_b12, eq57_e968_d_b13, eq57_e968_d_b14, eq57_e968_d_b15, eq57_e968_d_b16, eq57_e968_d_b17, eq57_e968_d_b18, eq57_e968_d_b19, eq57_e968_d_b20, eq57_e968_d_b21, eq57_e968_d_b22, eq57_e968_d_b23, eq57_e968_d_b24, eq57_e968_d_b25, eq57_e968_d_b26, eq57_e968_d_b27, eq57_e968_d_b28, eq57_e968_d_b29, eq57_e968_d_b30, eq57_e968_d_b31, eq57_e968_d_b32, eq57_e968_d_b33, eq57_e968_d_b34, eq57_e968_d_b35,) = {
    if s.b[614] {
        let eq57_e965: f64 = (s.v[0] * (nv15 - nv14));
        let eq57_e965_d_n14: f64 = (-s.v[0]);
        let eq57_e965_d_n15: f64 = s.v[0];
        let eq57_e966: f64 = (s.v[196] + eq57_e965);
        let eq57_e966_d_n14: f64 = (s.dn[196][14] + eq57_e965_d_n14);
        let eq57_e966_d_n15: f64 = (s.dn[196][15] + eq57_e965_d_n15);
        (eq57_e966, s.dn[196][0], s.dn[196][1], s.dn[196][2], s.dn[196][3], s.dn[196][4], s.dn[196][5], s.dn[196][6], s.dn[196][7], s.dn[196][8], s.dn[196][9], s.dn[196][10], s.dn[196][11], s.dn[196][12], s.dn[196][13], eq57_e966_d_n14, eq57_e966_d_n15, s.dn[196][16], s.dn[196][17], s.dn[196][18], s.dn[196][19], s.dn[196][20], s.dn[196][21], s.dn[196][22], s.dn[196][23], s.dn[196][24], s.dn[196][25], s.dn[196][26], s.dn[196][27], s.dn[196][28], s.dn[196][29], s.db[196][0], s.db[196][1], s.db[196][2], s.db[196][3], s.db[196][4], s.db[196][5], s.db[196][6], s.db[196][7], s.db[196][8], s.db[196][9], s.db[196][10], s.db[196][11], s.db[196][12], s.db[196][13], s.db[196][14], s.db[196][15], s.db[196][16], s.db[196][17], s.db[196][18], s.db[196][19], s.db[196][20], s.db[196][21], s.db[196][22], s.db[196][23], s.db[196][24], s.db[196][25], s.db[196][26], s.db[196][27], s.db[196][28], s.db[196][29], s.db[196][30], s.db[196][31], s.db[196][32], s.db[196][33], s.db[196][34], s.db[196][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e968;
        let eq57_node_derivatives: [f64; 30] = [eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29];
        let eq57_branch_derivatives: [f64; 36] = [eq57_e968_d_b0, eq57_e968_d_b1, eq57_e968_d_b2, eq57_e968_d_b3, eq57_e968_d_b4, eq57_e968_d_b5, eq57_e968_d_b6, eq57_e968_d_b7, eq57_e968_d_b8, eq57_e968_d_b9, eq57_e968_d_b10, eq57_e968_d_b11, eq57_e968_d_b12, eq57_e968_d_b13, eq57_e968_d_b14, eq57_e968_d_b15, eq57_e968_d_b16, eq57_e968_d_b17, eq57_e968_d_b18, eq57_e968_d_b19, eq57_e968_d_b20, eq57_e968_d_b21, eq57_e968_d_b22, eq57_e968_d_b23, eq57_e968_d_b24, eq57_e968_d_b25, eq57_e968_d_b26, eq57_e968_d_b27, eq57_e968_d_b28, eq57_e968_d_b29, eq57_e968_d_b30, eq57_e968_d_b31, eq57_e968_d_b32, eq57_e968_d_b33, eq57_e968_d_b34, eq57_e968_d_b35];
        stamper.stamp_current_dense_local(
            Some(15),
            Some(14),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29, eq59_e983_d_b0, eq59_e983_d_b1, eq59_e983_d_b2, eq59_e983_d_b3, eq59_e983_d_b4, eq59_e983_d_b5, eq59_e983_d_b6, eq59_e983_d_b7, eq59_e983_d_b8, eq59_e983_d_b9, eq59_e983_d_b10, eq59_e983_d_b11, eq59_e983_d_b12, eq59_e983_d_b13, eq59_e983_d_b14, eq59_e983_d_b15, eq59_e983_d_b16, eq59_e983_d_b17, eq59_e983_d_b18, eq59_e983_d_b19, eq59_e983_d_b20, eq59_e983_d_b21, eq59_e983_d_b22, eq59_e983_d_b23, eq59_e983_d_b24, eq59_e983_d_b25, eq59_e983_d_b26, eq59_e983_d_b27, eq59_e983_d_b28, eq59_e983_d_b29, eq59_e983_d_b30, eq59_e983_d_b31, eq59_e983_d_b32, eq59_e983_d_b33, eq59_e983_d_b34, eq59_e983_d_b35,) = {
    if s.b[760] {
        let eq59_e976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 36, s.v[197]);
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e979_d_n7: f64 = p.p355;
        let eq59_e979_d_n14: f64 = (-p.p355);
        let eq59_e980: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 37, eq59_e979);
        let eq59_e981: f64 = (eq59_e976 + eq59_e980);
        let eq59_e981_d_n7: f64 = ((s.dn[197][7] * ddt_scale) + (eq59_e979_d_n7 * ddt_scale));
        let eq59_e981_d_n14: f64 = ((s.dn[197][14] * ddt_scale) + (eq59_e979_d_n14 * ddt_scale));
        (eq59_e981, (s.dn[197][0] * ddt_scale), (s.dn[197][1] * ddt_scale), (s.dn[197][2] * ddt_scale), (s.dn[197][3] * ddt_scale), (s.dn[197][4] * ddt_scale), (s.dn[197][5] * ddt_scale), (s.dn[197][6] * ddt_scale), eq59_e981_d_n7, (s.dn[197][8] * ddt_scale), (s.dn[197][9] * ddt_scale), (s.dn[197][10] * ddt_scale), (s.dn[197][11] * ddt_scale), (s.dn[197][12] * ddt_scale), (s.dn[197][13] * ddt_scale), eq59_e981_d_n14, (s.dn[197][15] * ddt_scale), (s.dn[197][16] * ddt_scale), (s.dn[197][17] * ddt_scale), (s.dn[197][18] * ddt_scale), (s.dn[197][19] * ddt_scale), (s.dn[197][20] * ddt_scale), (s.dn[197][21] * ddt_scale), (s.dn[197][22] * ddt_scale), (s.dn[197][23] * ddt_scale), (s.dn[197][24] * ddt_scale), (s.dn[197][25] * ddt_scale), (s.dn[197][26] * ddt_scale), (s.dn[197][27] * ddt_scale), (s.dn[197][28] * ddt_scale), (s.dn[197][29] * ddt_scale), (s.db[197][0] * ddt_scale), (s.db[197][1] * ddt_scale), (s.db[197][2] * ddt_scale), (s.db[197][3] * ddt_scale), (s.db[197][4] * ddt_scale), (s.db[197][5] * ddt_scale), (s.db[197][6] * ddt_scale), (s.db[197][7] * ddt_scale), (s.db[197][8] * ddt_scale), (s.db[197][9] * ddt_scale), (s.db[197][10] * ddt_scale), (s.db[197][11] * ddt_scale), (s.db[197][12] * ddt_scale), (s.db[197][13] * ddt_scale), (s.db[197][14] * ddt_scale), (s.db[197][15] * ddt_scale), (s.db[197][16] * ddt_scale), (s.db[197][17] * ddt_scale), (s.db[197][18] * ddt_scale), (s.db[197][19] * ddt_scale), (s.db[197][20] * ddt_scale), (s.db[197][21] * ddt_scale), (s.db[197][22] * ddt_scale), (s.db[197][23] * ddt_scale), (s.db[197][24] * ddt_scale), (s.db[197][25] * ddt_scale), (s.db[197][26] * ddt_scale), (s.db[197][27] * ddt_scale), (s.db[197][28] * ddt_scale), (s.db[197][29] * ddt_scale), (s.db[197][30] * ddt_scale), (s.db[197][31] * ddt_scale), (s.db[197][32] * ddt_scale), (s.db[197][33] * ddt_scale), (s.db[197][34] * ddt_scale), (s.db[197][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e983;
        let eq59_node_derivatives: [f64; 30] = [eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29];
        let eq59_branch_derivatives: [f64; 36] = [eq59_e983_d_b0, eq59_e983_d_b1, eq59_e983_d_b2, eq59_e983_d_b3, eq59_e983_d_b4, eq59_e983_d_b5, eq59_e983_d_b6, eq59_e983_d_b7, eq59_e983_d_b8, eq59_e983_d_b9, eq59_e983_d_b10, eq59_e983_d_b11, eq59_e983_d_b12, eq59_e983_d_b13, eq59_e983_d_b14, eq59_e983_d_b15, eq59_e983_d_b16, eq59_e983_d_b17, eq59_e983_d_b18, eq59_e983_d_b19, eq59_e983_d_b20, eq59_e983_d_b21, eq59_e983_d_b22, eq59_e983_d_b23, eq59_e983_d_b24, eq59_e983_d_b25, eq59_e983_d_b26, eq59_e983_d_b27, eq59_e983_d_b28, eq59_e983_d_b29, eq59_e983_d_b30, eq59_e983_d_b31, eq59_e983_d_b32, eq59_e983_d_b33, eq59_e983_d_b34, eq59_e983_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(14),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29, eq60_e993_d_b0, eq60_e993_d_b1, eq60_e993_d_b2, eq60_e993_d_b3, eq60_e993_d_b4, eq60_e993_d_b5, eq60_e993_d_b6, eq60_e993_d_b7, eq60_e993_d_b8, eq60_e993_d_b9, eq60_e993_d_b10, eq60_e993_d_b11, eq60_e993_d_b12, eq60_e993_d_b13, eq60_e993_d_b14, eq60_e993_d_b15, eq60_e993_d_b16, eq60_e993_d_b17, eq60_e993_d_b18, eq60_e993_d_b19, eq60_e993_d_b20, eq60_e993_d_b21, eq60_e993_d_b22, eq60_e993_d_b23, eq60_e993_d_b24, eq60_e993_d_b25, eq60_e993_d_b26, eq60_e993_d_b27, eq60_e993_d_b28, eq60_e993_d_b29, eq60_e993_d_b30, eq60_e993_d_b31, eq60_e993_d_b32, eq60_e993_d_b33, eq60_e993_d_b34, eq60_e993_d_b35,) = {
    if s.b[760] {
        let eq60_e986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 38, s.v[198]);
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e989_d_n7: f64 = p.p355;
        let eq60_e989_d_n15: f64 = (-p.p355);
        let eq60_e990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 39, eq60_e989);
        let eq60_e991: f64 = (eq60_e986 + eq60_e990);
        let eq60_e991_d_n7: f64 = ((s.dn[198][7] * ddt_scale) + (eq60_e989_d_n7 * ddt_scale));
        let eq60_e991_d_n15: f64 = ((s.dn[198][15] * ddt_scale) + (eq60_e989_d_n15 * ddt_scale));
        (eq60_e991, (s.dn[198][0] * ddt_scale), (s.dn[198][1] * ddt_scale), (s.dn[198][2] * ddt_scale), (s.dn[198][3] * ddt_scale), (s.dn[198][4] * ddt_scale), (s.dn[198][5] * ddt_scale), (s.dn[198][6] * ddt_scale), eq60_e991_d_n7, (s.dn[198][8] * ddt_scale), (s.dn[198][9] * ddt_scale), (s.dn[198][10] * ddt_scale), (s.dn[198][11] * ddt_scale), (s.dn[198][12] * ddt_scale), (s.dn[198][13] * ddt_scale), (s.dn[198][14] * ddt_scale), eq60_e991_d_n15, (s.dn[198][16] * ddt_scale), (s.dn[198][17] * ddt_scale), (s.dn[198][18] * ddt_scale), (s.dn[198][19] * ddt_scale), (s.dn[198][20] * ddt_scale), (s.dn[198][21] * ddt_scale), (s.dn[198][22] * ddt_scale), (s.dn[198][23] * ddt_scale), (s.dn[198][24] * ddt_scale), (s.dn[198][25] * ddt_scale), (s.dn[198][26] * ddt_scale), (s.dn[198][27] * ddt_scale), (s.dn[198][28] * ddt_scale), (s.dn[198][29] * ddt_scale), (s.db[198][0] * ddt_scale), (s.db[198][1] * ddt_scale), (s.db[198][2] * ddt_scale), (s.db[198][3] * ddt_scale), (s.db[198][4] * ddt_scale), (s.db[198][5] * ddt_scale), (s.db[198][6] * ddt_scale), (s.db[198][7] * ddt_scale), (s.db[198][8] * ddt_scale), (s.db[198][9] * ddt_scale), (s.db[198][10] * ddt_scale), (s.db[198][11] * ddt_scale), (s.db[198][12] * ddt_scale), (s.db[198][13] * ddt_scale), (s.db[198][14] * ddt_scale), (s.db[198][15] * ddt_scale), (s.db[198][16] * ddt_scale), (s.db[198][17] * ddt_scale), (s.db[198][18] * ddt_scale), (s.db[198][19] * ddt_scale), (s.db[198][20] * ddt_scale), (s.db[198][21] * ddt_scale), (s.db[198][22] * ddt_scale), (s.db[198][23] * ddt_scale), (s.db[198][24] * ddt_scale), (s.db[198][25] * ddt_scale), (s.db[198][26] * ddt_scale), (s.db[198][27] * ddt_scale), (s.db[198][28] * ddt_scale), (s.db[198][29] * ddt_scale), (s.db[198][30] * ddt_scale), (s.db[198][31] * ddt_scale), (s.db[198][32] * ddt_scale), (s.db[198][33] * ddt_scale), (s.db[198][34] * ddt_scale), (s.db[198][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e993;
        let eq60_node_derivatives: [f64; 30] = [eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29];
        let eq60_branch_derivatives: [f64; 36] = [eq60_e993_d_b0, eq60_e993_d_b1, eq60_e993_d_b2, eq60_e993_d_b3, eq60_e993_d_b4, eq60_e993_d_b5, eq60_e993_d_b6, eq60_e993_d_b7, eq60_e993_d_b8, eq60_e993_d_b9, eq60_e993_d_b10, eq60_e993_d_b11, eq60_e993_d_b12, eq60_e993_d_b13, eq60_e993_d_b14, eq60_e993_d_b15, eq60_e993_d_b16, eq60_e993_d_b17, eq60_e993_d_b18, eq60_e993_d_b19, eq60_e993_d_b20, eq60_e993_d_b21, eq60_e993_d_b22, eq60_e993_d_b23, eq60_e993_d_b24, eq60_e993_d_b25, eq60_e993_d_b26, eq60_e993_d_b27, eq60_e993_d_b28, eq60_e993_d_b29, eq60_e993_d_b30, eq60_e993_d_b31, eq60_e993_d_b32, eq60_e993_d_b33, eq60_e993_d_b34, eq60_e993_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(15),
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29, eq61_e1003_d_b0, eq61_e1003_d_b1, eq61_e1003_d_b2, eq61_e1003_d_b3, eq61_e1003_d_b4, eq61_e1003_d_b5, eq61_e1003_d_b6, eq61_e1003_d_b7, eq61_e1003_d_b8, eq61_e1003_d_b9, eq61_e1003_d_b10, eq61_e1003_d_b11, eq61_e1003_d_b12, eq61_e1003_d_b13, eq61_e1003_d_b14, eq61_e1003_d_b15, eq61_e1003_d_b16, eq61_e1003_d_b17, eq61_e1003_d_b18, eq61_e1003_d_b19, eq61_e1003_d_b20, eq61_e1003_d_b21, eq61_e1003_d_b22, eq61_e1003_d_b23, eq61_e1003_d_b24, eq61_e1003_d_b25, eq61_e1003_d_b26, eq61_e1003_d_b27, eq61_e1003_d_b28, eq61_e1003_d_b29, eq61_e1003_d_b30, eq61_e1003_d_b31, eq61_e1003_d_b32, eq61_e1003_d_b33, eq61_e1003_d_b34, eq61_e1003_d_b35,) = {
    if s.b[760] {
        let eq61_e996: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 40, s.v[199]);
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e999_d_n2: f64 = p.p355;
        let eq61_e999_d_n14: f64 = (-p.p355);
        let eq61_e1000: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 41, eq61_e999);
        let eq61_e1001: f64 = (eq61_e996 + eq61_e1000);
        let eq61_e1001_d_n2: f64 = ((s.dn[199][2] * ddt_scale) + (eq61_e999_d_n2 * ddt_scale));
        let eq61_e1001_d_n14: f64 = ((s.dn[199][14] * ddt_scale) + (eq61_e999_d_n14 * ddt_scale));
        (eq61_e1001, (s.dn[199][0] * ddt_scale), (s.dn[199][1] * ddt_scale), eq61_e1001_d_n2, (s.dn[199][3] * ddt_scale), (s.dn[199][4] * ddt_scale), (s.dn[199][5] * ddt_scale), (s.dn[199][6] * ddt_scale), (s.dn[199][7] * ddt_scale), (s.dn[199][8] * ddt_scale), (s.dn[199][9] * ddt_scale), (s.dn[199][10] * ddt_scale), (s.dn[199][11] * ddt_scale), (s.dn[199][12] * ddt_scale), (s.dn[199][13] * ddt_scale), eq61_e1001_d_n14, (s.dn[199][15] * ddt_scale), (s.dn[199][16] * ddt_scale), (s.dn[199][17] * ddt_scale), (s.dn[199][18] * ddt_scale), (s.dn[199][19] * ddt_scale), (s.dn[199][20] * ddt_scale), (s.dn[199][21] * ddt_scale), (s.dn[199][22] * ddt_scale), (s.dn[199][23] * ddt_scale), (s.dn[199][24] * ddt_scale), (s.dn[199][25] * ddt_scale), (s.dn[199][26] * ddt_scale), (s.dn[199][27] * ddt_scale), (s.dn[199][28] * ddt_scale), (s.dn[199][29] * ddt_scale), (s.db[199][0] * ddt_scale), (s.db[199][1] * ddt_scale), (s.db[199][2] * ddt_scale), (s.db[199][3] * ddt_scale), (s.db[199][4] * ddt_scale), (s.db[199][5] * ddt_scale), (s.db[199][6] * ddt_scale), (s.db[199][7] * ddt_scale), (s.db[199][8] * ddt_scale), (s.db[199][9] * ddt_scale), (s.db[199][10] * ddt_scale), (s.db[199][11] * ddt_scale), (s.db[199][12] * ddt_scale), (s.db[199][13] * ddt_scale), (s.db[199][14] * ddt_scale), (s.db[199][15] * ddt_scale), (s.db[199][16] * ddt_scale), (s.db[199][17] * ddt_scale), (s.db[199][18] * ddt_scale), (s.db[199][19] * ddt_scale), (s.db[199][20] * ddt_scale), (s.db[199][21] * ddt_scale), (s.db[199][22] * ddt_scale), (s.db[199][23] * ddt_scale), (s.db[199][24] * ddt_scale), (s.db[199][25] * ddt_scale), (s.db[199][26] * ddt_scale), (s.db[199][27] * ddt_scale), (s.db[199][28] * ddt_scale), (s.db[199][29] * ddt_scale), (s.db[199][30] * ddt_scale), (s.db[199][31] * ddt_scale), (s.db[199][32] * ddt_scale), (s.db[199][33] * ddt_scale), (s.db[199][34] * ddt_scale), (s.db[199][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1003;
        let eq61_node_derivatives: [f64; 30] = [eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29];
        let eq61_branch_derivatives: [f64; 36] = [eq61_e1003_d_b0, eq61_e1003_d_b1, eq61_e1003_d_b2, eq61_e1003_d_b3, eq61_e1003_d_b4, eq61_e1003_d_b5, eq61_e1003_d_b6, eq61_e1003_d_b7, eq61_e1003_d_b8, eq61_e1003_d_b9, eq61_e1003_d_b10, eq61_e1003_d_b11, eq61_e1003_d_b12, eq61_e1003_d_b13, eq61_e1003_d_b14, eq61_e1003_d_b15, eq61_e1003_d_b16, eq61_e1003_d_b17, eq61_e1003_d_b18, eq61_e1003_d_b19, eq61_e1003_d_b20, eq61_e1003_d_b21, eq61_e1003_d_b22, eq61_e1003_d_b23, eq61_e1003_d_b24, eq61_e1003_d_b25, eq61_e1003_d_b26, eq61_e1003_d_b27, eq61_e1003_d_b28, eq61_e1003_d_b29, eq61_e1003_d_b30, eq61_e1003_d_b31, eq61_e1003_d_b32, eq61_e1003_d_b33, eq61_e1003_d_b34, eq61_e1003_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(14),
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29, eq63_e1017_d_b0, eq63_e1017_d_b1, eq63_e1017_d_b2, eq63_e1017_d_b3, eq63_e1017_d_b4, eq63_e1017_d_b5, eq63_e1017_d_b6, eq63_e1017_d_b7, eq63_e1017_d_b8, eq63_e1017_d_b9, eq63_e1017_d_b10, eq63_e1017_d_b11, eq63_e1017_d_b12, eq63_e1017_d_b13, eq63_e1017_d_b14, eq63_e1017_d_b15, eq63_e1017_d_b16, eq63_e1017_d_b17, eq63_e1017_d_b18, eq63_e1017_d_b19, eq63_e1017_d_b20, eq63_e1017_d_b21, eq63_e1017_d_b22, eq63_e1017_d_b23, eq63_e1017_d_b24, eq63_e1017_d_b25, eq63_e1017_d_b26, eq63_e1017_d_b27, eq63_e1017_d_b28, eq63_e1017_d_b29, eq63_e1017_d_b30, eq63_e1017_d_b31, eq63_e1017_d_b32, eq63_e1017_d_b33, eq63_e1017_d_b34, eq63_e1017_d_b35,) = {
    if s.b[760] {
        let eq63_e1010: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 42, s.v[201]);
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1013_d_n7: f64 = p.p355;
        let eq63_e1013_d_n9: f64 = (-p.p355);
        let eq63_e1014: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 43, eq63_e1013);
        let eq63_e1015: f64 = (eq63_e1010 + eq63_e1014);
        let eq63_e1015_d_n7: f64 = ((s.dn[201][7] * ddt_scale) + (eq63_e1013_d_n7 * ddt_scale));
        let eq63_e1015_d_n9: f64 = ((s.dn[201][9] * ddt_scale) + (eq63_e1013_d_n9 * ddt_scale));
        (eq63_e1015, (s.dn[201][0] * ddt_scale), (s.dn[201][1] * ddt_scale), (s.dn[201][2] * ddt_scale), (s.dn[201][3] * ddt_scale), (s.dn[201][4] * ddt_scale), (s.dn[201][5] * ddt_scale), (s.dn[201][6] * ddt_scale), eq63_e1015_d_n7, (s.dn[201][8] * ddt_scale), eq63_e1015_d_n9, (s.dn[201][10] * ddt_scale), (s.dn[201][11] * ddt_scale), (s.dn[201][12] * ddt_scale), (s.dn[201][13] * ddt_scale), (s.dn[201][14] * ddt_scale), (s.dn[201][15] * ddt_scale), (s.dn[201][16] * ddt_scale), (s.dn[201][17] * ddt_scale), (s.dn[201][18] * ddt_scale), (s.dn[201][19] * ddt_scale), (s.dn[201][20] * ddt_scale), (s.dn[201][21] * ddt_scale), (s.dn[201][22] * ddt_scale), (s.dn[201][23] * ddt_scale), (s.dn[201][24] * ddt_scale), (s.dn[201][25] * ddt_scale), (s.dn[201][26] * ddt_scale), (s.dn[201][27] * ddt_scale), (s.dn[201][28] * ddt_scale), (s.dn[201][29] * ddt_scale), (s.db[201][0] * ddt_scale), (s.db[201][1] * ddt_scale), (s.db[201][2] * ddt_scale), (s.db[201][3] * ddt_scale), (s.db[201][4] * ddt_scale), (s.db[201][5] * ddt_scale), (s.db[201][6] * ddt_scale), (s.db[201][7] * ddt_scale), (s.db[201][8] * ddt_scale), (s.db[201][9] * ddt_scale), (s.db[201][10] * ddt_scale), (s.db[201][11] * ddt_scale), (s.db[201][12] * ddt_scale), (s.db[201][13] * ddt_scale), (s.db[201][14] * ddt_scale), (s.db[201][15] * ddt_scale), (s.db[201][16] * ddt_scale), (s.db[201][17] * ddt_scale), (s.db[201][18] * ddt_scale), (s.db[201][19] * ddt_scale), (s.db[201][20] * ddt_scale), (s.db[201][21] * ddt_scale), (s.db[201][22] * ddt_scale), (s.db[201][23] * ddt_scale), (s.db[201][24] * ddt_scale), (s.db[201][25] * ddt_scale), (s.db[201][26] * ddt_scale), (s.db[201][27] * ddt_scale), (s.db[201][28] * ddt_scale), (s.db[201][29] * ddt_scale), (s.db[201][30] * ddt_scale), (s.db[201][31] * ddt_scale), (s.db[201][32] * ddt_scale), (s.db[201][33] * ddt_scale), (s.db[201][34] * ddt_scale), (s.db[201][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1017;
        let eq63_node_derivatives: [f64; 30] = [eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29];
        let eq63_branch_derivatives: [f64; 36] = [eq63_e1017_d_b0, eq63_e1017_d_b1, eq63_e1017_d_b2, eq63_e1017_d_b3, eq63_e1017_d_b4, eq63_e1017_d_b5, eq63_e1017_d_b6, eq63_e1017_d_b7, eq63_e1017_d_b8, eq63_e1017_d_b9, eq63_e1017_d_b10, eq63_e1017_d_b11, eq63_e1017_d_b12, eq63_e1017_d_b13, eq63_e1017_d_b14, eq63_e1017_d_b15, eq63_e1017_d_b16, eq63_e1017_d_b17, eq63_e1017_d_b18, eq63_e1017_d_b19, eq63_e1017_d_b20, eq63_e1017_d_b21, eq63_e1017_d_b22, eq63_e1017_d_b23, eq63_e1017_d_b24, eq63_e1017_d_b25, eq63_e1017_d_b26, eq63_e1017_d_b27, eq63_e1017_d_b28, eq63_e1017_d_b29, eq63_e1017_d_b30, eq63_e1017_d_b31, eq63_e1017_d_b32, eq63_e1017_d_b33, eq63_e1017_d_b34, eq63_e1017_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29, eq64_e1028_d_b0, eq64_e1028_d_b1, eq64_e1028_d_b2, eq64_e1028_d_b3, eq64_e1028_d_b4, eq64_e1028_d_b5, eq64_e1028_d_b6, eq64_e1028_d_b7, eq64_e1028_d_b8, eq64_e1028_d_b9, eq64_e1028_d_b10, eq64_e1028_d_b11, eq64_e1028_d_b12, eq64_e1028_d_b13, eq64_e1028_d_b14, eq64_e1028_d_b15, eq64_e1028_d_b16, eq64_e1028_d_b17, eq64_e1028_d_b18, eq64_e1028_d_b19, eq64_e1028_d_b20, eq64_e1028_d_b21, eq64_e1028_d_b22, eq64_e1028_d_b23, eq64_e1028_d_b24, eq64_e1028_d_b25, eq64_e1028_d_b26, eq64_e1028_d_b27, eq64_e1028_d_b28, eq64_e1028_d_b29, eq64_e1028_d_b30, eq64_e1028_d_b31, eq64_e1028_d_b32, eq64_e1028_d_b33, eq64_e1028_d_b34, eq64_e1028_d_b35,) = {
    if (!s.b[760]) {
        let eq64_e1021: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 44, s.v[197]);
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1024_d_n2: f64 = p.p355;
        let eq64_e1024_d_n14: f64 = (-p.p355);
        let eq64_e1025: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 45, eq64_e1024);
        let eq64_e1026: f64 = (eq64_e1021 + eq64_e1025);
        let eq64_e1026_d_n2: f64 = ((s.dn[197][2] * ddt_scale) + (eq64_e1024_d_n2 * ddt_scale));
        let eq64_e1026_d_n14: f64 = ((s.dn[197][14] * ddt_scale) + (eq64_e1024_d_n14 * ddt_scale));
        (eq64_e1026, (s.dn[197][0] * ddt_scale), (s.dn[197][1] * ddt_scale), eq64_e1026_d_n2, (s.dn[197][3] * ddt_scale), (s.dn[197][4] * ddt_scale), (s.dn[197][5] * ddt_scale), (s.dn[197][6] * ddt_scale), (s.dn[197][7] * ddt_scale), (s.dn[197][8] * ddt_scale), (s.dn[197][9] * ddt_scale), (s.dn[197][10] * ddt_scale), (s.dn[197][11] * ddt_scale), (s.dn[197][12] * ddt_scale), (s.dn[197][13] * ddt_scale), eq64_e1026_d_n14, (s.dn[197][15] * ddt_scale), (s.dn[197][16] * ddt_scale), (s.dn[197][17] * ddt_scale), (s.dn[197][18] * ddt_scale), (s.dn[197][19] * ddt_scale), (s.dn[197][20] * ddt_scale), (s.dn[197][21] * ddt_scale), (s.dn[197][22] * ddt_scale), (s.dn[197][23] * ddt_scale), (s.dn[197][24] * ddt_scale), (s.dn[197][25] * ddt_scale), (s.dn[197][26] * ddt_scale), (s.dn[197][27] * ddt_scale), (s.dn[197][28] * ddt_scale), (s.dn[197][29] * ddt_scale), (s.db[197][0] * ddt_scale), (s.db[197][1] * ddt_scale), (s.db[197][2] * ddt_scale), (s.db[197][3] * ddt_scale), (s.db[197][4] * ddt_scale), (s.db[197][5] * ddt_scale), (s.db[197][6] * ddt_scale), (s.db[197][7] * ddt_scale), (s.db[197][8] * ddt_scale), (s.db[197][9] * ddt_scale), (s.db[197][10] * ddt_scale), (s.db[197][11] * ddt_scale), (s.db[197][12] * ddt_scale), (s.db[197][13] * ddt_scale), (s.db[197][14] * ddt_scale), (s.db[197][15] * ddt_scale), (s.db[197][16] * ddt_scale), (s.db[197][17] * ddt_scale), (s.db[197][18] * ddt_scale), (s.db[197][19] * ddt_scale), (s.db[197][20] * ddt_scale), (s.db[197][21] * ddt_scale), (s.db[197][22] * ddt_scale), (s.db[197][23] * ddt_scale), (s.db[197][24] * ddt_scale), (s.db[197][25] * ddt_scale), (s.db[197][26] * ddt_scale), (s.db[197][27] * ddt_scale), (s.db[197][28] * ddt_scale), (s.db[197][29] * ddt_scale), (s.db[197][30] * ddt_scale), (s.db[197][31] * ddt_scale), (s.db[197][32] * ddt_scale), (s.db[197][33] * ddt_scale), (s.db[197][34] * ddt_scale), (s.db[197][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e1028;
        let eq64_node_derivatives: [f64; 30] = [eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29];
        let eq64_branch_derivatives: [f64; 36] = [eq64_e1028_d_b0, eq64_e1028_d_b1, eq64_e1028_d_b2, eq64_e1028_d_b3, eq64_e1028_d_b4, eq64_e1028_d_b5, eq64_e1028_d_b6, eq64_e1028_d_b7, eq64_e1028_d_b8, eq64_e1028_d_b9, eq64_e1028_d_b10, eq64_e1028_d_b11, eq64_e1028_d_b12, eq64_e1028_d_b13, eq64_e1028_d_b14, eq64_e1028_d_b15, eq64_e1028_d_b16, eq64_e1028_d_b17, eq64_e1028_d_b18, eq64_e1028_d_b19, eq64_e1028_d_b20, eq64_e1028_d_b21, eq64_e1028_d_b22, eq64_e1028_d_b23, eq64_e1028_d_b24, eq64_e1028_d_b25, eq64_e1028_d_b26, eq64_e1028_d_b27, eq64_e1028_d_b28, eq64_e1028_d_b29, eq64_e1028_d_b30, eq64_e1028_d_b31, eq64_e1028_d_b32, eq64_e1028_d_b33, eq64_e1028_d_b34, eq64_e1028_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(14),
            multiplicity * (eq64_value),
            &eq64_node_derivatives,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29, eq65_e1039_d_b0, eq65_e1039_d_b1, eq65_e1039_d_b2, eq65_e1039_d_b3, eq65_e1039_d_b4, eq65_e1039_d_b5, eq65_e1039_d_b6, eq65_e1039_d_b7, eq65_e1039_d_b8, eq65_e1039_d_b9, eq65_e1039_d_b10, eq65_e1039_d_b11, eq65_e1039_d_b12, eq65_e1039_d_b13, eq65_e1039_d_b14, eq65_e1039_d_b15, eq65_e1039_d_b16, eq65_e1039_d_b17, eq65_e1039_d_b18, eq65_e1039_d_b19, eq65_e1039_d_b20, eq65_e1039_d_b21, eq65_e1039_d_b22, eq65_e1039_d_b23, eq65_e1039_d_b24, eq65_e1039_d_b25, eq65_e1039_d_b26, eq65_e1039_d_b27, eq65_e1039_d_b28, eq65_e1039_d_b29, eq65_e1039_d_b30, eq65_e1039_d_b31, eq65_e1039_d_b32, eq65_e1039_d_b33, eq65_e1039_d_b34, eq65_e1039_d_b35,) = {
    if (!s.b[760]) {
        let eq65_e1032: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 46, s.v[198]);
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1035_d_n2: f64 = p.p355;
        let eq65_e1035_d_n15: f64 = (-p.p355);
        let eq65_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 47, eq65_e1035);
        let eq65_e1037: f64 = (eq65_e1032 + eq65_e1036);
        let eq65_e1037_d_n2: f64 = ((s.dn[198][2] * ddt_scale) + (eq65_e1035_d_n2 * ddt_scale));
        let eq65_e1037_d_n15: f64 = ((s.dn[198][15] * ddt_scale) + (eq65_e1035_d_n15 * ddt_scale));
        (eq65_e1037, (s.dn[198][0] * ddt_scale), (s.dn[198][1] * ddt_scale), eq65_e1037_d_n2, (s.dn[198][3] * ddt_scale), (s.dn[198][4] * ddt_scale), (s.dn[198][5] * ddt_scale), (s.dn[198][6] * ddt_scale), (s.dn[198][7] * ddt_scale), (s.dn[198][8] * ddt_scale), (s.dn[198][9] * ddt_scale), (s.dn[198][10] * ddt_scale), (s.dn[198][11] * ddt_scale), (s.dn[198][12] * ddt_scale), (s.dn[198][13] * ddt_scale), (s.dn[198][14] * ddt_scale), eq65_e1037_d_n15, (s.dn[198][16] * ddt_scale), (s.dn[198][17] * ddt_scale), (s.dn[198][18] * ddt_scale), (s.dn[198][19] * ddt_scale), (s.dn[198][20] * ddt_scale), (s.dn[198][21] * ddt_scale), (s.dn[198][22] * ddt_scale), (s.dn[198][23] * ddt_scale), (s.dn[198][24] * ddt_scale), (s.dn[198][25] * ddt_scale), (s.dn[198][26] * ddt_scale), (s.dn[198][27] * ddt_scale), (s.dn[198][28] * ddt_scale), (s.dn[198][29] * ddt_scale), (s.db[198][0] * ddt_scale), (s.db[198][1] * ddt_scale), (s.db[198][2] * ddt_scale), (s.db[198][3] * ddt_scale), (s.db[198][4] * ddt_scale), (s.db[198][5] * ddt_scale), (s.db[198][6] * ddt_scale), (s.db[198][7] * ddt_scale), (s.db[198][8] * ddt_scale), (s.db[198][9] * ddt_scale), (s.db[198][10] * ddt_scale), (s.db[198][11] * ddt_scale), (s.db[198][12] * ddt_scale), (s.db[198][13] * ddt_scale), (s.db[198][14] * ddt_scale), (s.db[198][15] * ddt_scale), (s.db[198][16] * ddt_scale), (s.db[198][17] * ddt_scale), (s.db[198][18] * ddt_scale), (s.db[198][19] * ddt_scale), (s.db[198][20] * ddt_scale), (s.db[198][21] * ddt_scale), (s.db[198][22] * ddt_scale), (s.db[198][23] * ddt_scale), (s.db[198][24] * ddt_scale), (s.db[198][25] * ddt_scale), (s.db[198][26] * ddt_scale), (s.db[198][27] * ddt_scale), (s.db[198][28] * ddt_scale), (s.db[198][29] * ddt_scale), (s.db[198][30] * ddt_scale), (s.db[198][31] * ddt_scale), (s.db[198][32] * ddt_scale), (s.db[198][33] * ddt_scale), (s.db[198][34] * ddt_scale), (s.db[198][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1039;
        let eq65_node_derivatives: [f64; 30] = [eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29];
        let eq65_branch_derivatives: [f64; 36] = [eq65_e1039_d_b0, eq65_e1039_d_b1, eq65_e1039_d_b2, eq65_e1039_d_b3, eq65_e1039_d_b4, eq65_e1039_d_b5, eq65_e1039_d_b6, eq65_e1039_d_b7, eq65_e1039_d_b8, eq65_e1039_d_b9, eq65_e1039_d_b10, eq65_e1039_d_b11, eq65_e1039_d_b12, eq65_e1039_d_b13, eq65_e1039_d_b14, eq65_e1039_d_b15, eq65_e1039_d_b16, eq65_e1039_d_b17, eq65_e1039_d_b18, eq65_e1039_d_b19, eq65_e1039_d_b20, eq65_e1039_d_b21, eq65_e1039_d_b22, eq65_e1039_d_b23, eq65_e1039_d_b24, eq65_e1039_d_b25, eq65_e1039_d_b26, eq65_e1039_d_b27, eq65_e1039_d_b28, eq65_e1039_d_b29, eq65_e1039_d_b30, eq65_e1039_d_b31, eq65_e1039_d_b32, eq65_e1039_d_b33, eq65_e1039_d_b34, eq65_e1039_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29, eq66_e1050_d_b0, eq66_e1050_d_b1, eq66_e1050_d_b2, eq66_e1050_d_b3, eq66_e1050_d_b4, eq66_e1050_d_b5, eq66_e1050_d_b6, eq66_e1050_d_b7, eq66_e1050_d_b8, eq66_e1050_d_b9, eq66_e1050_d_b10, eq66_e1050_d_b11, eq66_e1050_d_b12, eq66_e1050_d_b13, eq66_e1050_d_b14, eq66_e1050_d_b15, eq66_e1050_d_b16, eq66_e1050_d_b17, eq66_e1050_d_b18, eq66_e1050_d_b19, eq66_e1050_d_b20, eq66_e1050_d_b21, eq66_e1050_d_b22, eq66_e1050_d_b23, eq66_e1050_d_b24, eq66_e1050_d_b25, eq66_e1050_d_b26, eq66_e1050_d_b27, eq66_e1050_d_b28, eq66_e1050_d_b29, eq66_e1050_d_b30, eq66_e1050_d_b31, eq66_e1050_d_b32, eq66_e1050_d_b33, eq66_e1050_d_b34, eq66_e1050_d_b35,) = {
    if (!s.b[760]) {
        let eq66_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 48, s.v[199]);
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1046_d_n7: f64 = p.p355;
        let eq66_e1046_d_n14: f64 = (-p.p355);
        let eq66_e1047: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 49, eq66_e1046);
        let eq66_e1048: f64 = (eq66_e1043 + eq66_e1047);
        let eq66_e1048_d_n7: f64 = ((s.dn[199][7] * ddt_scale) + (eq66_e1046_d_n7 * ddt_scale));
        let eq66_e1048_d_n14: f64 = ((s.dn[199][14] * ddt_scale) + (eq66_e1046_d_n14 * ddt_scale));
        (eq66_e1048, (s.dn[199][0] * ddt_scale), (s.dn[199][1] * ddt_scale), (s.dn[199][2] * ddt_scale), (s.dn[199][3] * ddt_scale), (s.dn[199][4] * ddt_scale), (s.dn[199][5] * ddt_scale), (s.dn[199][6] * ddt_scale), eq66_e1048_d_n7, (s.dn[199][8] * ddt_scale), (s.dn[199][9] * ddt_scale), (s.dn[199][10] * ddt_scale), (s.dn[199][11] * ddt_scale), (s.dn[199][12] * ddt_scale), (s.dn[199][13] * ddt_scale), eq66_e1048_d_n14, (s.dn[199][15] * ddt_scale), (s.dn[199][16] * ddt_scale), (s.dn[199][17] * ddt_scale), (s.dn[199][18] * ddt_scale), (s.dn[199][19] * ddt_scale), (s.dn[199][20] * ddt_scale), (s.dn[199][21] * ddt_scale), (s.dn[199][22] * ddt_scale), (s.dn[199][23] * ddt_scale), (s.dn[199][24] * ddt_scale), (s.dn[199][25] * ddt_scale), (s.dn[199][26] * ddt_scale), (s.dn[199][27] * ddt_scale), (s.dn[199][28] * ddt_scale), (s.dn[199][29] * ddt_scale), (s.db[199][0] * ddt_scale), (s.db[199][1] * ddt_scale), (s.db[199][2] * ddt_scale), (s.db[199][3] * ddt_scale), (s.db[199][4] * ddt_scale), (s.db[199][5] * ddt_scale), (s.db[199][6] * ddt_scale), (s.db[199][7] * ddt_scale), (s.db[199][8] * ddt_scale), (s.db[199][9] * ddt_scale), (s.db[199][10] * ddt_scale), (s.db[199][11] * ddt_scale), (s.db[199][12] * ddt_scale), (s.db[199][13] * ddt_scale), (s.db[199][14] * ddt_scale), (s.db[199][15] * ddt_scale), (s.db[199][16] * ddt_scale), (s.db[199][17] * ddt_scale), (s.db[199][18] * ddt_scale), (s.db[199][19] * ddt_scale), (s.db[199][20] * ddt_scale), (s.db[199][21] * ddt_scale), (s.db[199][22] * ddt_scale), (s.db[199][23] * ddt_scale), (s.db[199][24] * ddt_scale), (s.db[199][25] * ddt_scale), (s.db[199][26] * ddt_scale), (s.db[199][27] * ddt_scale), (s.db[199][28] * ddt_scale), (s.db[199][29] * ddt_scale), (s.db[199][30] * ddt_scale), (s.db[199][31] * ddt_scale), (s.db[199][32] * ddt_scale), (s.db[199][33] * ddt_scale), (s.db[199][34] * ddt_scale), (s.db[199][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1050;
        let eq66_node_derivatives: [f64; 30] = [eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29];
        let eq66_branch_derivatives: [f64; 36] = [eq66_e1050_d_b0, eq66_e1050_d_b1, eq66_e1050_d_b2, eq66_e1050_d_b3, eq66_e1050_d_b4, eq66_e1050_d_b5, eq66_e1050_d_b6, eq66_e1050_d_b7, eq66_e1050_d_b8, eq66_e1050_d_b9, eq66_e1050_d_b10, eq66_e1050_d_b11, eq66_e1050_d_b12, eq66_e1050_d_b13, eq66_e1050_d_b14, eq66_e1050_d_b15, eq66_e1050_d_b16, eq66_e1050_d_b17, eq66_e1050_d_b18, eq66_e1050_d_b19, eq66_e1050_d_b20, eq66_e1050_d_b21, eq66_e1050_d_b22, eq66_e1050_d_b23, eq66_e1050_d_b24, eq66_e1050_d_b25, eq66_e1050_d_b26, eq66_e1050_d_b27, eq66_e1050_d_b28, eq66_e1050_d_b29, eq66_e1050_d_b30, eq66_e1050_d_b31, eq66_e1050_d_b32, eq66_e1050_d_b33, eq66_e1050_d_b34, eq66_e1050_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(14),
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let eq69_e1062: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 50, s.v[200]);
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1065_d_n3: f64 = p.p355;
        let eq69_e1065_d_n14: f64 = (-p.p355);
        let eq69_e1066: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 51, eq69_e1065);
        let eq69_e1067: f64 = (eq69_e1062 + eq69_e1066);
        let eq69_e1067_d_n3: f64 = ((s.dn[200][3] * ddt_scale) + (eq69_e1065_d_n3 * ddt_scale));
        let eq69_e1067_d_n14: f64 = ((s.dn[200][14] * ddt_scale) + (eq69_e1065_d_n14 * ddt_scale));
        let eq69_value: f64 = eq69_e1067;
        let eq69_node_derivatives: [f64; 30] = [(s.dn[200][0] * ddt_scale), (s.dn[200][1] * ddt_scale), (s.dn[200][2] * ddt_scale), eq69_e1067_d_n3, (s.dn[200][4] * ddt_scale), (s.dn[200][5] * ddt_scale), (s.dn[200][6] * ddt_scale), (s.dn[200][7] * ddt_scale), (s.dn[200][8] * ddt_scale), (s.dn[200][9] * ddt_scale), (s.dn[200][10] * ddt_scale), (s.dn[200][11] * ddt_scale), (s.dn[200][12] * ddt_scale), (s.dn[200][13] * ddt_scale), eq69_e1067_d_n14, (s.dn[200][15] * ddt_scale), (s.dn[200][16] * ddt_scale), (s.dn[200][17] * ddt_scale), (s.dn[200][18] * ddt_scale), (s.dn[200][19] * ddt_scale), (s.dn[200][20] * ddt_scale), (s.dn[200][21] * ddt_scale), (s.dn[200][22] * ddt_scale), (s.dn[200][23] * ddt_scale), (s.dn[200][24] * ddt_scale), (s.dn[200][25] * ddt_scale), (s.dn[200][26] * ddt_scale), (s.dn[200][27] * ddt_scale), (s.dn[200][28] * ddt_scale), (s.dn[200][29] * ddt_scale)];
        let eq69_branch_derivatives: [f64; 36] = [(s.db[200][0] * ddt_scale), (s.db[200][1] * ddt_scale), (s.db[200][2] * ddt_scale), (s.db[200][3] * ddt_scale), (s.db[200][4] * ddt_scale), (s.db[200][5] * ddt_scale), (s.db[200][6] * ddt_scale), (s.db[200][7] * ddt_scale), (s.db[200][8] * ddt_scale), (s.db[200][9] * ddt_scale), (s.db[200][10] * ddt_scale), (s.db[200][11] * ddt_scale), (s.db[200][12] * ddt_scale), (s.db[200][13] * ddt_scale), (s.db[200][14] * ddt_scale), (s.db[200][15] * ddt_scale), (s.db[200][16] * ddt_scale), (s.db[200][17] * ddt_scale), (s.db[200][18] * ddt_scale), (s.db[200][19] * ddt_scale), (s.db[200][20] * ddt_scale), (s.db[200][21] * ddt_scale), (s.db[200][22] * ddt_scale), (s.db[200][23] * ddt_scale), (s.db[200][24] * ddt_scale), (s.db[200][25] * ddt_scale), (s.db[200][26] * ddt_scale), (s.db[200][27] * ddt_scale), (s.db[200][28] * ddt_scale), (s.db[200][29] * ddt_scale), (s.db[200][30] * ddt_scale), (s.db[200][31] * ddt_scale), (s.db[200][32] * ddt_scale), (s.db[200][33] * ddt_scale), (s.db[200][34] * ddt_scale), (s.db[200][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(14),
            multiplicity * (eq69_value),
            &eq69_node_derivatives,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq70_e1075, eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29, eq70_e1075_d_b0, eq70_e1075_d_b1, eq70_e1075_d_b2, eq70_e1075_d_b3, eq70_e1075_d_b4, eq70_e1075_d_b5, eq70_e1075_d_b6, eq70_e1075_d_b7, eq70_e1075_d_b8, eq70_e1075_d_b9, eq70_e1075_d_b10, eq70_e1075_d_b11, eq70_e1075_d_b12, eq70_e1075_d_b13, eq70_e1075_d_b14, eq70_e1075_d_b15, eq70_e1075_d_b16, eq70_e1075_d_b17, eq70_e1075_d_b18, eq70_e1075_d_b19, eq70_e1075_d_b20, eq70_e1075_d_b21, eq70_e1075_d_b22, eq70_e1075_d_b23, eq70_e1075_d_b24, eq70_e1075_d_b25, eq70_e1075_d_b26, eq70_e1075_d_b27, eq70_e1075_d_b28, eq70_e1075_d_b29, eq70_e1075_d_b30, eq70_e1075_d_b31, eq70_e1075_d_b32, eq70_e1075_d_b33, eq70_e1075_d_b34, eq70_e1075_d_b35,) = {
    if s.b[761] {
        let eq70_e1072: f64 = (s.v[0] * (nv14 - nv5));
        let eq70_e1072_d_n5: f64 = (-s.v[0]);
        let eq70_e1072_d_n14: f64 = s.v[0];
        let eq70_e1073: f64 = (s.v[190] + eq70_e1072);
        let eq70_e1073_d_n5: f64 = (s.dn[190][5] + eq70_e1072_d_n5);
        let eq70_e1073_d_n14: f64 = (s.dn[190][14] + eq70_e1072_d_n14);
        (eq70_e1073, s.dn[190][0], s.dn[190][1], s.dn[190][2], s.dn[190][3], s.dn[190][4], eq70_e1073_d_n5, s.dn[190][6], s.dn[190][7], s.dn[190][8], s.dn[190][9], s.dn[190][10], s.dn[190][11], s.dn[190][12], s.dn[190][13], eq70_e1073_d_n14, s.dn[190][15], s.dn[190][16], s.dn[190][17], s.dn[190][18], s.dn[190][19], s.dn[190][20], s.dn[190][21], s.dn[190][22], s.dn[190][23], s.dn[190][24], s.dn[190][25], s.dn[190][26], s.dn[190][27], s.dn[190][28], s.dn[190][29], s.db[190][0], s.db[190][1], s.db[190][2], s.db[190][3], s.db[190][4], s.db[190][5], s.db[190][6], s.db[190][7], s.db[190][8], s.db[190][9], s.db[190][10], s.db[190][11], s.db[190][12], s.db[190][13], s.db[190][14], s.db[190][15], s.db[190][16], s.db[190][17], s.db[190][18], s.db[190][19], s.db[190][20], s.db[190][21], s.db[190][22], s.db[190][23], s.db[190][24], s.db[190][25], s.db[190][26], s.db[190][27], s.db[190][28], s.db[190][29], s.db[190][30], s.db[190][31], s.db[190][32], s.db[190][33], s.db[190][34], s.db[190][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1075;
        let eq70_node_derivatives: [f64; 30] = [eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29];
        let eq70_branch_derivatives: [f64; 36] = [eq70_e1075_d_b0, eq70_e1075_d_b1, eq70_e1075_d_b2, eq70_e1075_d_b3, eq70_e1075_d_b4, eq70_e1075_d_b5, eq70_e1075_d_b6, eq70_e1075_d_b7, eq70_e1075_d_b8, eq70_e1075_d_b9, eq70_e1075_d_b10, eq70_e1075_d_b11, eq70_e1075_d_b12, eq70_e1075_d_b13, eq70_e1075_d_b14, eq70_e1075_d_b15, eq70_e1075_d_b16, eq70_e1075_d_b17, eq70_e1075_d_b18, eq70_e1075_d_b19, eq70_e1075_d_b20, eq70_e1075_d_b21, eq70_e1075_d_b22, eq70_e1075_d_b23, eq70_e1075_d_b24, eq70_e1075_d_b25, eq70_e1075_d_b26, eq70_e1075_d_b27, eq70_e1075_d_b28, eq70_e1075_d_b29, eq70_e1075_d_b30, eq70_e1075_d_b31, eq70_e1075_d_b32, eq70_e1075_d_b33, eq70_e1075_d_b34, eq70_e1075_d_b35];
        stamper.stamp_current_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq70_value),
            &eq70_node_derivatives,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29, eq72_e1090_d_b0, eq72_e1090_d_b1, eq72_e1090_d_b2, eq72_e1090_d_b3, eq72_e1090_d_b4, eq72_e1090_d_b5, eq72_e1090_d_b6, eq72_e1090_d_b7, eq72_e1090_d_b8, eq72_e1090_d_b9, eq72_e1090_d_b10, eq72_e1090_d_b11, eq72_e1090_d_b12, eq72_e1090_d_b13, eq72_e1090_d_b14, eq72_e1090_d_b15, eq72_e1090_d_b16, eq72_e1090_d_b17, eq72_e1090_d_b18, eq72_e1090_d_b19, eq72_e1090_d_b20, eq72_e1090_d_b21, eq72_e1090_d_b22, eq72_e1090_d_b23, eq72_e1090_d_b24, eq72_e1090_d_b25, eq72_e1090_d_b26, eq72_e1090_d_b27, eq72_e1090_d_b28, eq72_e1090_d_b29, eq72_e1090_d_b30, eq72_e1090_d_b31, eq72_e1090_d_b32, eq72_e1090_d_b33, eq72_e1090_d_b34, eq72_e1090_d_b35,) = {
    if s.b[907] {
        let eq72_e1083: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 52, s.v[191]);
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1086_d_n5: f64 = (-p.p355);
        let eq72_e1086_d_n7: f64 = p.p355;
        let eq72_e1087: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 53, eq72_e1086);
        let eq72_e1088: f64 = (eq72_e1083 + eq72_e1087);
        let eq72_e1088_d_n5: f64 = ((s.dn[191][5] * ddt_scale) + (eq72_e1086_d_n5 * ddt_scale));
        let eq72_e1088_d_n7: f64 = ((s.dn[191][7] * ddt_scale) + (eq72_e1086_d_n7 * ddt_scale));
        (eq72_e1088, (s.dn[191][0] * ddt_scale), (s.dn[191][1] * ddt_scale), (s.dn[191][2] * ddt_scale), (s.dn[191][3] * ddt_scale), (s.dn[191][4] * ddt_scale), eq72_e1088_d_n5, (s.dn[191][6] * ddt_scale), eq72_e1088_d_n7, (s.dn[191][8] * ddt_scale), (s.dn[191][9] * ddt_scale), (s.dn[191][10] * ddt_scale), (s.dn[191][11] * ddt_scale), (s.dn[191][12] * ddt_scale), (s.dn[191][13] * ddt_scale), (s.dn[191][14] * ddt_scale), (s.dn[191][15] * ddt_scale), (s.dn[191][16] * ddt_scale), (s.dn[191][17] * ddt_scale), (s.dn[191][18] * ddt_scale), (s.dn[191][19] * ddt_scale), (s.dn[191][20] * ddt_scale), (s.dn[191][21] * ddt_scale), (s.dn[191][22] * ddt_scale), (s.dn[191][23] * ddt_scale), (s.dn[191][24] * ddt_scale), (s.dn[191][25] * ddt_scale), (s.dn[191][26] * ddt_scale), (s.dn[191][27] * ddt_scale), (s.dn[191][28] * ddt_scale), (s.dn[191][29] * ddt_scale), (s.db[191][0] * ddt_scale), (s.db[191][1] * ddt_scale), (s.db[191][2] * ddt_scale), (s.db[191][3] * ddt_scale), (s.db[191][4] * ddt_scale), (s.db[191][5] * ddt_scale), (s.db[191][6] * ddt_scale), (s.db[191][7] * ddt_scale), (s.db[191][8] * ddt_scale), (s.db[191][9] * ddt_scale), (s.db[191][10] * ddt_scale), (s.db[191][11] * ddt_scale), (s.db[191][12] * ddt_scale), (s.db[191][13] * ddt_scale), (s.db[191][14] * ddt_scale), (s.db[191][15] * ddt_scale), (s.db[191][16] * ddt_scale), (s.db[191][17] * ddt_scale), (s.db[191][18] * ddt_scale), (s.db[191][19] * ddt_scale), (s.db[191][20] * ddt_scale), (s.db[191][21] * ddt_scale), (s.db[191][22] * ddt_scale), (s.db[191][23] * ddt_scale), (s.db[191][24] * ddt_scale), (s.db[191][25] * ddt_scale), (s.db[191][26] * ddt_scale), (s.db[191][27] * ddt_scale), (s.db[191][28] * ddt_scale), (s.db[191][29] * ddt_scale), (s.db[191][30] * ddt_scale), (s.db[191][31] * ddt_scale), (s.db[191][32] * ddt_scale), (s.db[191][33] * ddt_scale), (s.db[191][34] * ddt_scale), (s.db[191][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1090;
        let eq72_node_derivatives: [f64; 30] = [eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29];
        let eq72_branch_derivatives: [f64; 36] = [eq72_e1090_d_b0, eq72_e1090_d_b1, eq72_e1090_d_b2, eq72_e1090_d_b3, eq72_e1090_d_b4, eq72_e1090_d_b5, eq72_e1090_d_b6, eq72_e1090_d_b7, eq72_e1090_d_b8, eq72_e1090_d_b9, eq72_e1090_d_b10, eq72_e1090_d_b11, eq72_e1090_d_b12, eq72_e1090_d_b13, eq72_e1090_d_b14, eq72_e1090_d_b15, eq72_e1090_d_b16, eq72_e1090_d_b17, eq72_e1090_d_b18, eq72_e1090_d_b19, eq72_e1090_d_b20, eq72_e1090_d_b21, eq72_e1090_d_b22, eq72_e1090_d_b23, eq72_e1090_d_b24, eq72_e1090_d_b25, eq72_e1090_d_b26, eq72_e1090_d_b27, eq72_e1090_d_b28, eq72_e1090_d_b29, eq72_e1090_d_b30, eq72_e1090_d_b31, eq72_e1090_d_b32, eq72_e1090_d_b33, eq72_e1090_d_b34, eq72_e1090_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq72_value),
            &eq72_node_derivatives,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29, eq73_e1100_d_b0, eq73_e1100_d_b1, eq73_e1100_d_b2, eq73_e1100_d_b3, eq73_e1100_d_b4, eq73_e1100_d_b5, eq73_e1100_d_b6, eq73_e1100_d_b7, eq73_e1100_d_b8, eq73_e1100_d_b9, eq73_e1100_d_b10, eq73_e1100_d_b11, eq73_e1100_d_b12, eq73_e1100_d_b13, eq73_e1100_d_b14, eq73_e1100_d_b15, eq73_e1100_d_b16, eq73_e1100_d_b17, eq73_e1100_d_b18, eq73_e1100_d_b19, eq73_e1100_d_b20, eq73_e1100_d_b21, eq73_e1100_d_b22, eq73_e1100_d_b23, eq73_e1100_d_b24, eq73_e1100_d_b25, eq73_e1100_d_b26, eq73_e1100_d_b27, eq73_e1100_d_b28, eq73_e1100_d_b29, eq73_e1100_d_b30, eq73_e1100_d_b31, eq73_e1100_d_b32, eq73_e1100_d_b33, eq73_e1100_d_b34, eq73_e1100_d_b35,) = {
    if s.b[907] {
        let eq73_e1093: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 54, s.v[192]);
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1096_d_n7: f64 = p.p355;
        let eq73_e1096_d_n14: f64 = (-p.p355);
        let eq73_e1097: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 55, eq73_e1096);
        let eq73_e1098: f64 = (eq73_e1093 + eq73_e1097);
        let eq73_e1098_d_n7: f64 = ((s.dn[192][7] * ddt_scale) + (eq73_e1096_d_n7 * ddt_scale));
        let eq73_e1098_d_n14: f64 = ((s.dn[192][14] * ddt_scale) + (eq73_e1096_d_n14 * ddt_scale));
        (eq73_e1098, (s.dn[192][0] * ddt_scale), (s.dn[192][1] * ddt_scale), (s.dn[192][2] * ddt_scale), (s.dn[192][3] * ddt_scale), (s.dn[192][4] * ddt_scale), (s.dn[192][5] * ddt_scale), (s.dn[192][6] * ddt_scale), eq73_e1098_d_n7, (s.dn[192][8] * ddt_scale), (s.dn[192][9] * ddt_scale), (s.dn[192][10] * ddt_scale), (s.dn[192][11] * ddt_scale), (s.dn[192][12] * ddt_scale), (s.dn[192][13] * ddt_scale), eq73_e1098_d_n14, (s.dn[192][15] * ddt_scale), (s.dn[192][16] * ddt_scale), (s.dn[192][17] * ddt_scale), (s.dn[192][18] * ddt_scale), (s.dn[192][19] * ddt_scale), (s.dn[192][20] * ddt_scale), (s.dn[192][21] * ddt_scale), (s.dn[192][22] * ddt_scale), (s.dn[192][23] * ddt_scale), (s.dn[192][24] * ddt_scale), (s.dn[192][25] * ddt_scale), (s.dn[192][26] * ddt_scale), (s.dn[192][27] * ddt_scale), (s.dn[192][28] * ddt_scale), (s.dn[192][29] * ddt_scale), (s.db[192][0] * ddt_scale), (s.db[192][1] * ddt_scale), (s.db[192][2] * ddt_scale), (s.db[192][3] * ddt_scale), (s.db[192][4] * ddt_scale), (s.db[192][5] * ddt_scale), (s.db[192][6] * ddt_scale), (s.db[192][7] * ddt_scale), (s.db[192][8] * ddt_scale), (s.db[192][9] * ddt_scale), (s.db[192][10] * ddt_scale), (s.db[192][11] * ddt_scale), (s.db[192][12] * ddt_scale), (s.db[192][13] * ddt_scale), (s.db[192][14] * ddt_scale), (s.db[192][15] * ddt_scale), (s.db[192][16] * ddt_scale), (s.db[192][17] * ddt_scale), (s.db[192][18] * ddt_scale), (s.db[192][19] * ddt_scale), (s.db[192][20] * ddt_scale), (s.db[192][21] * ddt_scale), (s.db[192][22] * ddt_scale), (s.db[192][23] * ddt_scale), (s.db[192][24] * ddt_scale), (s.db[192][25] * ddt_scale), (s.db[192][26] * ddt_scale), (s.db[192][27] * ddt_scale), (s.db[192][28] * ddt_scale), (s.db[192][29] * ddt_scale), (s.db[192][30] * ddt_scale), (s.db[192][31] * ddt_scale), (s.db[192][32] * ddt_scale), (s.db[192][33] * ddt_scale), (s.db[192][34] * ddt_scale), (s.db[192][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1100;
        let eq73_node_derivatives: [f64; 30] = [eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29];
        let eq73_branch_derivatives: [f64; 36] = [eq73_e1100_d_b0, eq73_e1100_d_b1, eq73_e1100_d_b2, eq73_e1100_d_b3, eq73_e1100_d_b4, eq73_e1100_d_b5, eq73_e1100_d_b6, eq73_e1100_d_b7, eq73_e1100_d_b8, eq73_e1100_d_b9, eq73_e1100_d_b10, eq73_e1100_d_b11, eq73_e1100_d_b12, eq73_e1100_d_b13, eq73_e1100_d_b14, eq73_e1100_d_b15, eq73_e1100_d_b16, eq73_e1100_d_b17, eq73_e1100_d_b18, eq73_e1100_d_b19, eq73_e1100_d_b20, eq73_e1100_d_b21, eq73_e1100_d_b22, eq73_e1100_d_b23, eq73_e1100_d_b24, eq73_e1100_d_b25, eq73_e1100_d_b26, eq73_e1100_d_b27, eq73_e1100_d_b28, eq73_e1100_d_b29, eq73_e1100_d_b30, eq73_e1100_d_b31, eq73_e1100_d_b32, eq73_e1100_d_b33, eq73_e1100_d_b34, eq73_e1100_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(14),
            multiplicity * (eq73_value),
            &eq73_node_derivatives,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29, eq74_e1110_d_b0, eq74_e1110_d_b1, eq74_e1110_d_b2, eq74_e1110_d_b3, eq74_e1110_d_b4, eq74_e1110_d_b5, eq74_e1110_d_b6, eq74_e1110_d_b7, eq74_e1110_d_b8, eq74_e1110_d_b9, eq74_e1110_d_b10, eq74_e1110_d_b11, eq74_e1110_d_b12, eq74_e1110_d_b13, eq74_e1110_d_b14, eq74_e1110_d_b15, eq74_e1110_d_b16, eq74_e1110_d_b17, eq74_e1110_d_b18, eq74_e1110_d_b19, eq74_e1110_d_b20, eq74_e1110_d_b21, eq74_e1110_d_b22, eq74_e1110_d_b23, eq74_e1110_d_b24, eq74_e1110_d_b25, eq74_e1110_d_b26, eq74_e1110_d_b27, eq74_e1110_d_b28, eq74_e1110_d_b29, eq74_e1110_d_b30, eq74_e1110_d_b31, eq74_e1110_d_b32, eq74_e1110_d_b33, eq74_e1110_d_b34, eq74_e1110_d_b35,) = {
    if s.b[907] {
        let eq74_e1103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 56, s.v[193]);
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1106_d_n2: f64 = p.p355;
        let eq74_e1106_d_n5: f64 = (-p.p355);
        let eq74_e1107: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 57, eq74_e1106);
        let eq74_e1108: f64 = (eq74_e1103 + eq74_e1107);
        let eq74_e1108_d_n2: f64 = ((s.dn[193][2] * ddt_scale) + (eq74_e1106_d_n2 * ddt_scale));
        let eq74_e1108_d_n5: f64 = ((s.dn[193][5] * ddt_scale) + (eq74_e1106_d_n5 * ddt_scale));
        (eq74_e1108, (s.dn[193][0] * ddt_scale), (s.dn[193][1] * ddt_scale), eq74_e1108_d_n2, (s.dn[193][3] * ddt_scale), (s.dn[193][4] * ddt_scale), eq74_e1108_d_n5, (s.dn[193][6] * ddt_scale), (s.dn[193][7] * ddt_scale), (s.dn[193][8] * ddt_scale), (s.dn[193][9] * ddt_scale), (s.dn[193][10] * ddt_scale), (s.dn[193][11] * ddt_scale), (s.dn[193][12] * ddt_scale), (s.dn[193][13] * ddt_scale), (s.dn[193][14] * ddt_scale), (s.dn[193][15] * ddt_scale), (s.dn[193][16] * ddt_scale), (s.dn[193][17] * ddt_scale), (s.dn[193][18] * ddt_scale), (s.dn[193][19] * ddt_scale), (s.dn[193][20] * ddt_scale), (s.dn[193][21] * ddt_scale), (s.dn[193][22] * ddt_scale), (s.dn[193][23] * ddt_scale), (s.dn[193][24] * ddt_scale), (s.dn[193][25] * ddt_scale), (s.dn[193][26] * ddt_scale), (s.dn[193][27] * ddt_scale), (s.dn[193][28] * ddt_scale), (s.dn[193][29] * ddt_scale), (s.db[193][0] * ddt_scale), (s.db[193][1] * ddt_scale), (s.db[193][2] * ddt_scale), (s.db[193][3] * ddt_scale), (s.db[193][4] * ddt_scale), (s.db[193][5] * ddt_scale), (s.db[193][6] * ddt_scale), (s.db[193][7] * ddt_scale), (s.db[193][8] * ddt_scale), (s.db[193][9] * ddt_scale), (s.db[193][10] * ddt_scale), (s.db[193][11] * ddt_scale), (s.db[193][12] * ddt_scale), (s.db[193][13] * ddt_scale), (s.db[193][14] * ddt_scale), (s.db[193][15] * ddt_scale), (s.db[193][16] * ddt_scale), (s.db[193][17] * ddt_scale), (s.db[193][18] * ddt_scale), (s.db[193][19] * ddt_scale), (s.db[193][20] * ddt_scale), (s.db[193][21] * ddt_scale), (s.db[193][22] * ddt_scale), (s.db[193][23] * ddt_scale), (s.db[193][24] * ddt_scale), (s.db[193][25] * ddt_scale), (s.db[193][26] * ddt_scale), (s.db[193][27] * ddt_scale), (s.db[193][28] * ddt_scale), (s.db[193][29] * ddt_scale), (s.db[193][30] * ddt_scale), (s.db[193][31] * ddt_scale), (s.db[193][32] * ddt_scale), (s.db[193][33] * ddt_scale), (s.db[193][34] * ddt_scale), (s.db[193][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1110;
        let eq74_node_derivatives: [f64; 30] = [eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29];
        let eq74_branch_derivatives: [f64; 36] = [eq74_e1110_d_b0, eq74_e1110_d_b1, eq74_e1110_d_b2, eq74_e1110_d_b3, eq74_e1110_d_b4, eq74_e1110_d_b5, eq74_e1110_d_b6, eq74_e1110_d_b7, eq74_e1110_d_b8, eq74_e1110_d_b9, eq74_e1110_d_b10, eq74_e1110_d_b11, eq74_e1110_d_b12, eq74_e1110_d_b13, eq74_e1110_d_b14, eq74_e1110_d_b15, eq74_e1110_d_b16, eq74_e1110_d_b17, eq74_e1110_d_b18, eq74_e1110_d_b19, eq74_e1110_d_b20, eq74_e1110_d_b21, eq74_e1110_d_b22, eq74_e1110_d_b23, eq74_e1110_d_b24, eq74_e1110_d_b25, eq74_e1110_d_b26, eq74_e1110_d_b27, eq74_e1110_d_b28, eq74_e1110_d_b29, eq74_e1110_d_b30, eq74_e1110_d_b31, eq74_e1110_d_b32, eq74_e1110_d_b33, eq74_e1110_d_b34, eq74_e1110_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq74_value),
            &eq74_node_derivatives,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29, eq76_e1124_d_b0, eq76_e1124_d_b1, eq76_e1124_d_b2, eq76_e1124_d_b3, eq76_e1124_d_b4, eq76_e1124_d_b5, eq76_e1124_d_b6, eq76_e1124_d_b7, eq76_e1124_d_b8, eq76_e1124_d_b9, eq76_e1124_d_b10, eq76_e1124_d_b11, eq76_e1124_d_b12, eq76_e1124_d_b13, eq76_e1124_d_b14, eq76_e1124_d_b15, eq76_e1124_d_b16, eq76_e1124_d_b17, eq76_e1124_d_b18, eq76_e1124_d_b19, eq76_e1124_d_b20, eq76_e1124_d_b21, eq76_e1124_d_b22, eq76_e1124_d_b23, eq76_e1124_d_b24, eq76_e1124_d_b25, eq76_e1124_d_b26, eq76_e1124_d_b27, eq76_e1124_d_b28, eq76_e1124_d_b29, eq76_e1124_d_b30, eq76_e1124_d_b31, eq76_e1124_d_b32, eq76_e1124_d_b33, eq76_e1124_d_b34, eq76_e1124_d_b35,) = {
    if s.b[907] {
        let eq76_e1117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 58, s.v[195]);
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1120_d_n7: f64 = p.p355;
        let eq76_e1120_d_n9: f64 = (-p.p355);
        let eq76_e1121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 59, eq76_e1120);
        let eq76_e1122: f64 = (eq76_e1117 + eq76_e1121);
        let eq76_e1122_d_n7: f64 = ((s.dn[195][7] * ddt_scale) + (eq76_e1120_d_n7 * ddt_scale));
        let eq76_e1122_d_n9: f64 = ((s.dn[195][9] * ddt_scale) + (eq76_e1120_d_n9 * ddt_scale));
        (eq76_e1122, (s.dn[195][0] * ddt_scale), (s.dn[195][1] * ddt_scale), (s.dn[195][2] * ddt_scale), (s.dn[195][3] * ddt_scale), (s.dn[195][4] * ddt_scale), (s.dn[195][5] * ddt_scale), (s.dn[195][6] * ddt_scale), eq76_e1122_d_n7, (s.dn[195][8] * ddt_scale), eq76_e1122_d_n9, (s.dn[195][10] * ddt_scale), (s.dn[195][11] * ddt_scale), (s.dn[195][12] * ddt_scale), (s.dn[195][13] * ddt_scale), (s.dn[195][14] * ddt_scale), (s.dn[195][15] * ddt_scale), (s.dn[195][16] * ddt_scale), (s.dn[195][17] * ddt_scale), (s.dn[195][18] * ddt_scale), (s.dn[195][19] * ddt_scale), (s.dn[195][20] * ddt_scale), (s.dn[195][21] * ddt_scale), (s.dn[195][22] * ddt_scale), (s.dn[195][23] * ddt_scale), (s.dn[195][24] * ddt_scale), (s.dn[195][25] * ddt_scale), (s.dn[195][26] * ddt_scale), (s.dn[195][27] * ddt_scale), (s.dn[195][28] * ddt_scale), (s.dn[195][29] * ddt_scale), (s.db[195][0] * ddt_scale), (s.db[195][1] * ddt_scale), (s.db[195][2] * ddt_scale), (s.db[195][3] * ddt_scale), (s.db[195][4] * ddt_scale), (s.db[195][5] * ddt_scale), (s.db[195][6] * ddt_scale), (s.db[195][7] * ddt_scale), (s.db[195][8] * ddt_scale), (s.db[195][9] * ddt_scale), (s.db[195][10] * ddt_scale), (s.db[195][11] * ddt_scale), (s.db[195][12] * ddt_scale), (s.db[195][13] * ddt_scale), (s.db[195][14] * ddt_scale), (s.db[195][15] * ddt_scale), (s.db[195][16] * ddt_scale), (s.db[195][17] * ddt_scale), (s.db[195][18] * ddt_scale), (s.db[195][19] * ddt_scale), (s.db[195][20] * ddt_scale), (s.db[195][21] * ddt_scale), (s.db[195][22] * ddt_scale), (s.db[195][23] * ddt_scale), (s.db[195][24] * ddt_scale), (s.db[195][25] * ddt_scale), (s.db[195][26] * ddt_scale), (s.db[195][27] * ddt_scale), (s.db[195][28] * ddt_scale), (s.db[195][29] * ddt_scale), (s.db[195][30] * ddt_scale), (s.db[195][31] * ddt_scale), (s.db[195][32] * ddt_scale), (s.db[195][33] * ddt_scale), (s.db[195][34] * ddt_scale), (s.db[195][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1124;
        let eq76_node_derivatives: [f64; 30] = [eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29];
        let eq76_branch_derivatives: [f64; 36] = [eq76_e1124_d_b0, eq76_e1124_d_b1, eq76_e1124_d_b2, eq76_e1124_d_b3, eq76_e1124_d_b4, eq76_e1124_d_b5, eq76_e1124_d_b6, eq76_e1124_d_b7, eq76_e1124_d_b8, eq76_e1124_d_b9, eq76_e1124_d_b10, eq76_e1124_d_b11, eq76_e1124_d_b12, eq76_e1124_d_b13, eq76_e1124_d_b14, eq76_e1124_d_b15, eq76_e1124_d_b16, eq76_e1124_d_b17, eq76_e1124_d_b18, eq76_e1124_d_b19, eq76_e1124_d_b20, eq76_e1124_d_b21, eq76_e1124_d_b22, eq76_e1124_d_b23, eq76_e1124_d_b24, eq76_e1124_d_b25, eq76_e1124_d_b26, eq76_e1124_d_b27, eq76_e1124_d_b28, eq76_e1124_d_b29, eq76_e1124_d_b30, eq76_e1124_d_b31, eq76_e1124_d_b32, eq76_e1124_d_b33, eq76_e1124_d_b34, eq76_e1124_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq76_value),
            &eq76_node_derivatives,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29, eq77_e1135_d_b0, eq77_e1135_d_b1, eq77_e1135_d_b2, eq77_e1135_d_b3, eq77_e1135_d_b4, eq77_e1135_d_b5, eq77_e1135_d_b6, eq77_e1135_d_b7, eq77_e1135_d_b8, eq77_e1135_d_b9, eq77_e1135_d_b10, eq77_e1135_d_b11, eq77_e1135_d_b12, eq77_e1135_d_b13, eq77_e1135_d_b14, eq77_e1135_d_b15, eq77_e1135_d_b16, eq77_e1135_d_b17, eq77_e1135_d_b18, eq77_e1135_d_b19, eq77_e1135_d_b20, eq77_e1135_d_b21, eq77_e1135_d_b22, eq77_e1135_d_b23, eq77_e1135_d_b24, eq77_e1135_d_b25, eq77_e1135_d_b26, eq77_e1135_d_b27, eq77_e1135_d_b28, eq77_e1135_d_b29, eq77_e1135_d_b30, eq77_e1135_d_b31, eq77_e1135_d_b32, eq77_e1135_d_b33, eq77_e1135_d_b34, eq77_e1135_d_b35,) = {
    if (!s.b[907]) {
        let eq77_e1128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 60, s.v[191]);
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1131_d_n2: f64 = p.p355;
        let eq77_e1131_d_n5: f64 = (-p.p355);
        let eq77_e1132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 61, eq77_e1131);
        let eq77_e1133: f64 = (eq77_e1128 + eq77_e1132);
        let eq77_e1133_d_n2: f64 = ((s.dn[191][2] * ddt_scale) + (eq77_e1131_d_n2 * ddt_scale));
        let eq77_e1133_d_n5: f64 = ((s.dn[191][5] * ddt_scale) + (eq77_e1131_d_n5 * ddt_scale));
        (eq77_e1133, (s.dn[191][0] * ddt_scale), (s.dn[191][1] * ddt_scale), eq77_e1133_d_n2, (s.dn[191][3] * ddt_scale), (s.dn[191][4] * ddt_scale), eq77_e1133_d_n5, (s.dn[191][6] * ddt_scale), (s.dn[191][7] * ddt_scale), (s.dn[191][8] * ddt_scale), (s.dn[191][9] * ddt_scale), (s.dn[191][10] * ddt_scale), (s.dn[191][11] * ddt_scale), (s.dn[191][12] * ddt_scale), (s.dn[191][13] * ddt_scale), (s.dn[191][14] * ddt_scale), (s.dn[191][15] * ddt_scale), (s.dn[191][16] * ddt_scale), (s.dn[191][17] * ddt_scale), (s.dn[191][18] * ddt_scale), (s.dn[191][19] * ddt_scale), (s.dn[191][20] * ddt_scale), (s.dn[191][21] * ddt_scale), (s.dn[191][22] * ddt_scale), (s.dn[191][23] * ddt_scale), (s.dn[191][24] * ddt_scale), (s.dn[191][25] * ddt_scale), (s.dn[191][26] * ddt_scale), (s.dn[191][27] * ddt_scale), (s.dn[191][28] * ddt_scale), (s.dn[191][29] * ddt_scale), (s.db[191][0] * ddt_scale), (s.db[191][1] * ddt_scale), (s.db[191][2] * ddt_scale), (s.db[191][3] * ddt_scale), (s.db[191][4] * ddt_scale), (s.db[191][5] * ddt_scale), (s.db[191][6] * ddt_scale), (s.db[191][7] * ddt_scale), (s.db[191][8] * ddt_scale), (s.db[191][9] * ddt_scale), (s.db[191][10] * ddt_scale), (s.db[191][11] * ddt_scale), (s.db[191][12] * ddt_scale), (s.db[191][13] * ddt_scale), (s.db[191][14] * ddt_scale), (s.db[191][15] * ddt_scale), (s.db[191][16] * ddt_scale), (s.db[191][17] * ddt_scale), (s.db[191][18] * ddt_scale), (s.db[191][19] * ddt_scale), (s.db[191][20] * ddt_scale), (s.db[191][21] * ddt_scale), (s.db[191][22] * ddt_scale), (s.db[191][23] * ddt_scale), (s.db[191][24] * ddt_scale), (s.db[191][25] * ddt_scale), (s.db[191][26] * ddt_scale), (s.db[191][27] * ddt_scale), (s.db[191][28] * ddt_scale), (s.db[191][29] * ddt_scale), (s.db[191][30] * ddt_scale), (s.db[191][31] * ddt_scale), (s.db[191][32] * ddt_scale), (s.db[191][33] * ddt_scale), (s.db[191][34] * ddt_scale), (s.db[191][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1135;
        let eq77_node_derivatives: [f64; 30] = [eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29];
        let eq77_branch_derivatives: [f64; 36] = [eq77_e1135_d_b0, eq77_e1135_d_b1, eq77_e1135_d_b2, eq77_e1135_d_b3, eq77_e1135_d_b4, eq77_e1135_d_b5, eq77_e1135_d_b6, eq77_e1135_d_b7, eq77_e1135_d_b8, eq77_e1135_d_b9, eq77_e1135_d_b10, eq77_e1135_d_b11, eq77_e1135_d_b12, eq77_e1135_d_b13, eq77_e1135_d_b14, eq77_e1135_d_b15, eq77_e1135_d_b16, eq77_e1135_d_b17, eq77_e1135_d_b18, eq77_e1135_d_b19, eq77_e1135_d_b20, eq77_e1135_d_b21, eq77_e1135_d_b22, eq77_e1135_d_b23, eq77_e1135_d_b24, eq77_e1135_d_b25, eq77_e1135_d_b26, eq77_e1135_d_b27, eq77_e1135_d_b28, eq77_e1135_d_b29, eq77_e1135_d_b30, eq77_e1135_d_b31, eq77_e1135_d_b32, eq77_e1135_d_b33, eq77_e1135_d_b34, eq77_e1135_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq77_value),
            &eq77_node_derivatives,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29, eq78_e1146_d_b0, eq78_e1146_d_b1, eq78_e1146_d_b2, eq78_e1146_d_b3, eq78_e1146_d_b4, eq78_e1146_d_b5, eq78_e1146_d_b6, eq78_e1146_d_b7, eq78_e1146_d_b8, eq78_e1146_d_b9, eq78_e1146_d_b10, eq78_e1146_d_b11, eq78_e1146_d_b12, eq78_e1146_d_b13, eq78_e1146_d_b14, eq78_e1146_d_b15, eq78_e1146_d_b16, eq78_e1146_d_b17, eq78_e1146_d_b18, eq78_e1146_d_b19, eq78_e1146_d_b20, eq78_e1146_d_b21, eq78_e1146_d_b22, eq78_e1146_d_b23, eq78_e1146_d_b24, eq78_e1146_d_b25, eq78_e1146_d_b26, eq78_e1146_d_b27, eq78_e1146_d_b28, eq78_e1146_d_b29, eq78_e1146_d_b30, eq78_e1146_d_b31, eq78_e1146_d_b32, eq78_e1146_d_b33, eq78_e1146_d_b34, eq78_e1146_d_b35,) = {
    if (!s.b[907]) {
        let eq78_e1139: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 62, s.v[192]);
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1142_d_n2: f64 = p.p355;
        let eq78_e1142_d_n14: f64 = (-p.p355);
        let eq78_e1143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 63, eq78_e1142);
        let eq78_e1144: f64 = (eq78_e1139 + eq78_e1143);
        let eq78_e1144_d_n2: f64 = ((s.dn[192][2] * ddt_scale) + (eq78_e1142_d_n2 * ddt_scale));
        let eq78_e1144_d_n14: f64 = ((s.dn[192][14] * ddt_scale) + (eq78_e1142_d_n14 * ddt_scale));
        (eq78_e1144, (s.dn[192][0] * ddt_scale), (s.dn[192][1] * ddt_scale), eq78_e1144_d_n2, (s.dn[192][3] * ddt_scale), (s.dn[192][4] * ddt_scale), (s.dn[192][5] * ddt_scale), (s.dn[192][6] * ddt_scale), (s.dn[192][7] * ddt_scale), (s.dn[192][8] * ddt_scale), (s.dn[192][9] * ddt_scale), (s.dn[192][10] * ddt_scale), (s.dn[192][11] * ddt_scale), (s.dn[192][12] * ddt_scale), (s.dn[192][13] * ddt_scale), eq78_e1144_d_n14, (s.dn[192][15] * ddt_scale), (s.dn[192][16] * ddt_scale), (s.dn[192][17] * ddt_scale), (s.dn[192][18] * ddt_scale), (s.dn[192][19] * ddt_scale), (s.dn[192][20] * ddt_scale), (s.dn[192][21] * ddt_scale), (s.dn[192][22] * ddt_scale), (s.dn[192][23] * ddt_scale), (s.dn[192][24] * ddt_scale), (s.dn[192][25] * ddt_scale), (s.dn[192][26] * ddt_scale), (s.dn[192][27] * ddt_scale), (s.dn[192][28] * ddt_scale), (s.dn[192][29] * ddt_scale), (s.db[192][0] * ddt_scale), (s.db[192][1] * ddt_scale), (s.db[192][2] * ddt_scale), (s.db[192][3] * ddt_scale), (s.db[192][4] * ddt_scale), (s.db[192][5] * ddt_scale), (s.db[192][6] * ddt_scale), (s.db[192][7] * ddt_scale), (s.db[192][8] * ddt_scale), (s.db[192][9] * ddt_scale), (s.db[192][10] * ddt_scale), (s.db[192][11] * ddt_scale), (s.db[192][12] * ddt_scale), (s.db[192][13] * ddt_scale), (s.db[192][14] * ddt_scale), (s.db[192][15] * ddt_scale), (s.db[192][16] * ddt_scale), (s.db[192][17] * ddt_scale), (s.db[192][18] * ddt_scale), (s.db[192][19] * ddt_scale), (s.db[192][20] * ddt_scale), (s.db[192][21] * ddt_scale), (s.db[192][22] * ddt_scale), (s.db[192][23] * ddt_scale), (s.db[192][24] * ddt_scale), (s.db[192][25] * ddt_scale), (s.db[192][26] * ddt_scale), (s.db[192][27] * ddt_scale), (s.db[192][28] * ddt_scale), (s.db[192][29] * ddt_scale), (s.db[192][30] * ddt_scale), (s.db[192][31] * ddt_scale), (s.db[192][32] * ddt_scale), (s.db[192][33] * ddt_scale), (s.db[192][34] * ddt_scale), (s.db[192][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1146;
        let eq78_node_derivatives: [f64; 30] = [eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29];
        let eq78_branch_derivatives: [f64; 36] = [eq78_e1146_d_b0, eq78_e1146_d_b1, eq78_e1146_d_b2, eq78_e1146_d_b3, eq78_e1146_d_b4, eq78_e1146_d_b5, eq78_e1146_d_b6, eq78_e1146_d_b7, eq78_e1146_d_b8, eq78_e1146_d_b9, eq78_e1146_d_b10, eq78_e1146_d_b11, eq78_e1146_d_b12, eq78_e1146_d_b13, eq78_e1146_d_b14, eq78_e1146_d_b15, eq78_e1146_d_b16, eq78_e1146_d_b17, eq78_e1146_d_b18, eq78_e1146_d_b19, eq78_e1146_d_b20, eq78_e1146_d_b21, eq78_e1146_d_b22, eq78_e1146_d_b23, eq78_e1146_d_b24, eq78_e1146_d_b25, eq78_e1146_d_b26, eq78_e1146_d_b27, eq78_e1146_d_b28, eq78_e1146_d_b29, eq78_e1146_d_b30, eq78_e1146_d_b31, eq78_e1146_d_b32, eq78_e1146_d_b33, eq78_e1146_d_b34, eq78_e1146_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(14),
            multiplicity * (eq78_value),
            &eq78_node_derivatives,
            &eq78_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29, eq79_e1157_d_b0, eq79_e1157_d_b1, eq79_e1157_d_b2, eq79_e1157_d_b3, eq79_e1157_d_b4, eq79_e1157_d_b5, eq79_e1157_d_b6, eq79_e1157_d_b7, eq79_e1157_d_b8, eq79_e1157_d_b9, eq79_e1157_d_b10, eq79_e1157_d_b11, eq79_e1157_d_b12, eq79_e1157_d_b13, eq79_e1157_d_b14, eq79_e1157_d_b15, eq79_e1157_d_b16, eq79_e1157_d_b17, eq79_e1157_d_b18, eq79_e1157_d_b19, eq79_e1157_d_b20, eq79_e1157_d_b21, eq79_e1157_d_b22, eq79_e1157_d_b23, eq79_e1157_d_b24, eq79_e1157_d_b25, eq79_e1157_d_b26, eq79_e1157_d_b27, eq79_e1157_d_b28, eq79_e1157_d_b29, eq79_e1157_d_b30, eq79_e1157_d_b31, eq79_e1157_d_b32, eq79_e1157_d_b33, eq79_e1157_d_b34, eq79_e1157_d_b35,) = {
    if (!s.b[907]) {
        let eq79_e1150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 64, s.v[193]);
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1153_d_n5: f64 = (-p.p355);
        let eq79_e1153_d_n7: f64 = p.p355;
        let eq79_e1154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 65, eq79_e1153);
        let eq79_e1155: f64 = (eq79_e1150 + eq79_e1154);
        let eq79_e1155_d_n5: f64 = ((s.dn[193][5] * ddt_scale) + (eq79_e1153_d_n5 * ddt_scale));
        let eq79_e1155_d_n7: f64 = ((s.dn[193][7] * ddt_scale) + (eq79_e1153_d_n7 * ddt_scale));
        (eq79_e1155, (s.dn[193][0] * ddt_scale), (s.dn[193][1] * ddt_scale), (s.dn[193][2] * ddt_scale), (s.dn[193][3] * ddt_scale), (s.dn[193][4] * ddt_scale), eq79_e1155_d_n5, (s.dn[193][6] * ddt_scale), eq79_e1155_d_n7, (s.dn[193][8] * ddt_scale), (s.dn[193][9] * ddt_scale), (s.dn[193][10] * ddt_scale), (s.dn[193][11] * ddt_scale), (s.dn[193][12] * ddt_scale), (s.dn[193][13] * ddt_scale), (s.dn[193][14] * ddt_scale), (s.dn[193][15] * ddt_scale), (s.dn[193][16] * ddt_scale), (s.dn[193][17] * ddt_scale), (s.dn[193][18] * ddt_scale), (s.dn[193][19] * ddt_scale), (s.dn[193][20] * ddt_scale), (s.dn[193][21] * ddt_scale), (s.dn[193][22] * ddt_scale), (s.dn[193][23] * ddt_scale), (s.dn[193][24] * ddt_scale), (s.dn[193][25] * ddt_scale), (s.dn[193][26] * ddt_scale), (s.dn[193][27] * ddt_scale), (s.dn[193][28] * ddt_scale), (s.dn[193][29] * ddt_scale), (s.db[193][0] * ddt_scale), (s.db[193][1] * ddt_scale), (s.db[193][2] * ddt_scale), (s.db[193][3] * ddt_scale), (s.db[193][4] * ddt_scale), (s.db[193][5] * ddt_scale), (s.db[193][6] * ddt_scale), (s.db[193][7] * ddt_scale), (s.db[193][8] * ddt_scale), (s.db[193][9] * ddt_scale), (s.db[193][10] * ddt_scale), (s.db[193][11] * ddt_scale), (s.db[193][12] * ddt_scale), (s.db[193][13] * ddt_scale), (s.db[193][14] * ddt_scale), (s.db[193][15] * ddt_scale), (s.db[193][16] * ddt_scale), (s.db[193][17] * ddt_scale), (s.db[193][18] * ddt_scale), (s.db[193][19] * ddt_scale), (s.db[193][20] * ddt_scale), (s.db[193][21] * ddt_scale), (s.db[193][22] * ddt_scale), (s.db[193][23] * ddt_scale), (s.db[193][24] * ddt_scale), (s.db[193][25] * ddt_scale), (s.db[193][26] * ddt_scale), (s.db[193][27] * ddt_scale), (s.db[193][28] * ddt_scale), (s.db[193][29] * ddt_scale), (s.db[193][30] * ddt_scale), (s.db[193][31] * ddt_scale), (s.db[193][32] * ddt_scale), (s.db[193][33] * ddt_scale), (s.db[193][34] * ddt_scale), (s.db[193][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1157;
        let eq79_node_derivatives: [f64; 30] = [eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29];
        let eq79_branch_derivatives: [f64; 36] = [eq79_e1157_d_b0, eq79_e1157_d_b1, eq79_e1157_d_b2, eq79_e1157_d_b3, eq79_e1157_d_b4, eq79_e1157_d_b5, eq79_e1157_d_b6, eq79_e1157_d_b7, eq79_e1157_d_b8, eq79_e1157_d_b9, eq79_e1157_d_b10, eq79_e1157_d_b11, eq79_e1157_d_b12, eq79_e1157_d_b13, eq79_e1157_d_b14, eq79_e1157_d_b15, eq79_e1157_d_b16, eq79_e1157_d_b17, eq79_e1157_d_b18, eq79_e1157_d_b19, eq79_e1157_d_b20, eq79_e1157_d_b21, eq79_e1157_d_b22, eq79_e1157_d_b23, eq79_e1157_d_b24, eq79_e1157_d_b25, eq79_e1157_d_b26, eq79_e1157_d_b27, eq79_e1157_d_b28, eq79_e1157_d_b29, eq79_e1157_d_b30, eq79_e1157_d_b31, eq79_e1157_d_b32, eq79_e1157_d_b33, eq79_e1157_d_b34, eq79_e1157_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq79_value),
            &eq79_node_derivatives,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let eq82_e1169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 66, s.v[194]);
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1172_d_n3: f64 = p.p355;
        let eq82_e1172_d_n5: f64 = (-p.p355);
        let eq82_e1173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 67, eq82_e1172);
        let eq82_e1174: f64 = (eq82_e1169 + eq82_e1173);
        let eq82_e1174_d_n3: f64 = ((s.dn[194][3] * ddt_scale) + (eq82_e1172_d_n3 * ddt_scale));
        let eq82_e1174_d_n5: f64 = ((s.dn[194][5] * ddt_scale) + (eq82_e1172_d_n5 * ddt_scale));
        let eq82_value: f64 = eq82_e1174;
        let eq82_node_derivatives: [f64; 30] = [(s.dn[194][0] * ddt_scale), (s.dn[194][1] * ddt_scale), (s.dn[194][2] * ddt_scale), eq82_e1174_d_n3, (s.dn[194][4] * ddt_scale), eq82_e1174_d_n5, (s.dn[194][6] * ddt_scale), (s.dn[194][7] * ddt_scale), (s.dn[194][8] * ddt_scale), (s.dn[194][9] * ddt_scale), (s.dn[194][10] * ddt_scale), (s.dn[194][11] * ddt_scale), (s.dn[194][12] * ddt_scale), (s.dn[194][13] * ddt_scale), (s.dn[194][14] * ddt_scale), (s.dn[194][15] * ddt_scale), (s.dn[194][16] * ddt_scale), (s.dn[194][17] * ddt_scale), (s.dn[194][18] * ddt_scale), (s.dn[194][19] * ddt_scale), (s.dn[194][20] * ddt_scale), (s.dn[194][21] * ddt_scale), (s.dn[194][22] * ddt_scale), (s.dn[194][23] * ddt_scale), (s.dn[194][24] * ddt_scale), (s.dn[194][25] * ddt_scale), (s.dn[194][26] * ddt_scale), (s.dn[194][27] * ddt_scale), (s.dn[194][28] * ddt_scale), (s.dn[194][29] * ddt_scale)];
        let eq82_branch_derivatives: [f64; 36] = [(s.db[194][0] * ddt_scale), (s.db[194][1] * ddt_scale), (s.db[194][2] * ddt_scale), (s.db[194][3] * ddt_scale), (s.db[194][4] * ddt_scale), (s.db[194][5] * ddt_scale), (s.db[194][6] * ddt_scale), (s.db[194][7] * ddt_scale), (s.db[194][8] * ddt_scale), (s.db[194][9] * ddt_scale), (s.db[194][10] * ddt_scale), (s.db[194][11] * ddt_scale), (s.db[194][12] * ddt_scale), (s.db[194][13] * ddt_scale), (s.db[194][14] * ddt_scale), (s.db[194][15] * ddt_scale), (s.db[194][16] * ddt_scale), (s.db[194][17] * ddt_scale), (s.db[194][18] * ddt_scale), (s.db[194][19] * ddt_scale), (s.db[194][20] * ddt_scale), (s.db[194][21] * ddt_scale), (s.db[194][22] * ddt_scale), (s.db[194][23] * ddt_scale), (s.db[194][24] * ddt_scale), (s.db[194][25] * ddt_scale), (s.db[194][26] * ddt_scale), (s.db[194][27] * ddt_scale), (s.db[194][28] * ddt_scale), (s.db[194][29] * ddt_scale), (s.db[194][30] * ddt_scale), (s.db[194][31] * ddt_scale), (s.db[194][32] * ddt_scale), (s.db[194][33] * ddt_scale), (s.db[194][34] * ddt_scale), (s.db[194][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq82_value),
            &eq82_node_derivatives,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1182, eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29, eq83_e1182_d_b0, eq83_e1182_d_b1, eq83_e1182_d_b2, eq83_e1182_d_b3, eq83_e1182_d_b4, eq83_e1182_d_b5, eq83_e1182_d_b6, eq83_e1182_d_b7, eq83_e1182_d_b8, eq83_e1182_d_b9, eq83_e1182_d_b10, eq83_e1182_d_b11, eq83_e1182_d_b12, eq83_e1182_d_b13, eq83_e1182_d_b14, eq83_e1182_d_b15, eq83_e1182_d_b16, eq83_e1182_d_b17, eq83_e1182_d_b18, eq83_e1182_d_b19, eq83_e1182_d_b20, eq83_e1182_d_b21, eq83_e1182_d_b22, eq83_e1182_d_b23, eq83_e1182_d_b24, eq83_e1182_d_b25, eq83_e1182_d_b26, eq83_e1182_d_b27, eq83_e1182_d_b28, eq83_e1182_d_b29, eq83_e1182_d_b30, eq83_e1182_d_b31, eq83_e1182_d_b32, eq83_e1182_d_b33, eq83_e1182_d_b34, eq83_e1182_d_b35,) = {
    if s.b[908] {
        let eq83_e1179: f64 = (s.v[0] * (nv9 - nv10));
        let eq83_e1179_d_n9: f64 = s.v[0];
        let eq83_e1179_d_n10: f64 = (-s.v[0]);
        let eq83_e1180: f64 = (s.v[166] + eq83_e1179);
        let eq83_e1180_d_n9: f64 = (s.dn[166][9] + eq83_e1179_d_n9);
        let eq83_e1180_d_n10: f64 = (s.dn[166][10] + eq83_e1179_d_n10);
        (eq83_e1180, s.dn[166][0], s.dn[166][1], s.dn[166][2], s.dn[166][3], s.dn[166][4], s.dn[166][5], s.dn[166][6], s.dn[166][7], s.dn[166][8], eq83_e1180_d_n9, eq83_e1180_d_n10, s.dn[166][11], s.dn[166][12], s.dn[166][13], s.dn[166][14], s.dn[166][15], s.dn[166][16], s.dn[166][17], s.dn[166][18], s.dn[166][19], s.dn[166][20], s.dn[166][21], s.dn[166][22], s.dn[166][23], s.dn[166][24], s.dn[166][25], s.dn[166][26], s.dn[166][27], s.dn[166][28], s.dn[166][29], s.db[166][0], s.db[166][1], s.db[166][2], s.db[166][3], s.db[166][4], s.db[166][5], s.db[166][6], s.db[166][7], s.db[166][8], s.db[166][9], s.db[166][10], s.db[166][11], s.db[166][12], s.db[166][13], s.db[166][14], s.db[166][15], s.db[166][16], s.db[166][17], s.db[166][18], s.db[166][19], s.db[166][20], s.db[166][21], s.db[166][22], s.db[166][23], s.db[166][24], s.db[166][25], s.db[166][26], s.db[166][27], s.db[166][28], s.db[166][29], s.db[166][30], s.db[166][31], s.db[166][32], s.db[166][33], s.db[166][34], s.db[166][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1182;
        let eq83_node_derivatives: [f64; 30] = [eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29];
        let eq83_branch_derivatives: [f64; 36] = [eq83_e1182_d_b0, eq83_e1182_d_b1, eq83_e1182_d_b2, eq83_e1182_d_b3, eq83_e1182_d_b4, eq83_e1182_d_b5, eq83_e1182_d_b6, eq83_e1182_d_b7, eq83_e1182_d_b8, eq83_e1182_d_b9, eq83_e1182_d_b10, eq83_e1182_d_b11, eq83_e1182_d_b12, eq83_e1182_d_b13, eq83_e1182_d_b14, eq83_e1182_d_b15, eq83_e1182_d_b16, eq83_e1182_d_b17, eq83_e1182_d_b18, eq83_e1182_d_b19, eq83_e1182_d_b20, eq83_e1182_d_b21, eq83_e1182_d_b22, eq83_e1182_d_b23, eq83_e1182_d_b24, eq83_e1182_d_b25, eq83_e1182_d_b26, eq83_e1182_d_b27, eq83_e1182_d_b28, eq83_e1182_d_b29, eq83_e1182_d_b30, eq83_e1182_d_b31, eq83_e1182_d_b32, eq83_e1182_d_b33, eq83_e1182_d_b34, eq83_e1182_d_b35];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq83_value),
            &eq83_node_derivatives,
            &eq83_branch_derivatives,
            multiplicity,
        );
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29, eq85_e1197_d_b0, eq85_e1197_d_b1, eq85_e1197_d_b2, eq85_e1197_d_b3, eq85_e1197_d_b4, eq85_e1197_d_b5, eq85_e1197_d_b6, eq85_e1197_d_b7, eq85_e1197_d_b8, eq85_e1197_d_b9, eq85_e1197_d_b10, eq85_e1197_d_b11, eq85_e1197_d_b12, eq85_e1197_d_b13, eq85_e1197_d_b14, eq85_e1197_d_b15, eq85_e1197_d_b16, eq85_e1197_d_b17, eq85_e1197_d_b18, eq85_e1197_d_b19, eq85_e1197_d_b20, eq85_e1197_d_b21, eq85_e1197_d_b22, eq85_e1197_d_b23, eq85_e1197_d_b24, eq85_e1197_d_b25, eq85_e1197_d_b26, eq85_e1197_d_b27, eq85_e1197_d_b28, eq85_e1197_d_b29, eq85_e1197_d_b30, eq85_e1197_d_b31, eq85_e1197_d_b32, eq85_e1197_d_b33, eq85_e1197_d_b34, eq85_e1197_d_b35,) = {
    if s.b[1054] {
        let eq85_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 68, s.v[167]);
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1193_d_n7: f64 = p.p355;
        let eq85_e1193_d_n10: f64 = (-p.p355);
        let eq85_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 69, eq85_e1193);
        let eq85_e1195: f64 = (eq85_e1190 + eq85_e1194);
        let eq85_e1195_d_n7: f64 = ((s.dn[167][7] * ddt_scale) + (eq85_e1193_d_n7 * ddt_scale));
        let eq85_e1195_d_n10: f64 = ((s.dn[167][10] * ddt_scale) + (eq85_e1193_d_n10 * ddt_scale));
        (eq85_e1195, (s.dn[167][0] * ddt_scale), (s.dn[167][1] * ddt_scale), (s.dn[167][2] * ddt_scale), (s.dn[167][3] * ddt_scale), (s.dn[167][4] * ddt_scale), (s.dn[167][5] * ddt_scale), (s.dn[167][6] * ddt_scale), eq85_e1195_d_n7, (s.dn[167][8] * ddt_scale), (s.dn[167][9] * ddt_scale), eq85_e1195_d_n10, (s.dn[167][11] * ddt_scale), (s.dn[167][12] * ddt_scale), (s.dn[167][13] * ddt_scale), (s.dn[167][14] * ddt_scale), (s.dn[167][15] * ddt_scale), (s.dn[167][16] * ddt_scale), (s.dn[167][17] * ddt_scale), (s.dn[167][18] * ddt_scale), (s.dn[167][19] * ddt_scale), (s.dn[167][20] * ddt_scale), (s.dn[167][21] * ddt_scale), (s.dn[167][22] * ddt_scale), (s.dn[167][23] * ddt_scale), (s.dn[167][24] * ddt_scale), (s.dn[167][25] * ddt_scale), (s.dn[167][26] * ddt_scale), (s.dn[167][27] * ddt_scale), (s.dn[167][28] * ddt_scale), (s.dn[167][29] * ddt_scale), (s.db[167][0] * ddt_scale), (s.db[167][1] * ddt_scale), (s.db[167][2] * ddt_scale), (s.db[167][3] * ddt_scale), (s.db[167][4] * ddt_scale), (s.db[167][5] * ddt_scale), (s.db[167][6] * ddt_scale), (s.db[167][7] * ddt_scale), (s.db[167][8] * ddt_scale), (s.db[167][9] * ddt_scale), (s.db[167][10] * ddt_scale), (s.db[167][11] * ddt_scale), (s.db[167][12] * ddt_scale), (s.db[167][13] * ddt_scale), (s.db[167][14] * ddt_scale), (s.db[167][15] * ddt_scale), (s.db[167][16] * ddt_scale), (s.db[167][17] * ddt_scale), (s.db[167][18] * ddt_scale), (s.db[167][19] * ddt_scale), (s.db[167][20] * ddt_scale), (s.db[167][21] * ddt_scale), (s.db[167][22] * ddt_scale), (s.db[167][23] * ddt_scale), (s.db[167][24] * ddt_scale), (s.db[167][25] * ddt_scale), (s.db[167][26] * ddt_scale), (s.db[167][27] * ddt_scale), (s.db[167][28] * ddt_scale), (s.db[167][29] * ddt_scale), (s.db[167][30] * ddt_scale), (s.db[167][31] * ddt_scale), (s.db[167][32] * ddt_scale), (s.db[167][33] * ddt_scale), (s.db[167][34] * ddt_scale), (s.db[167][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_value: f64 = eq85_e1197;
        let eq85_node_derivatives: [f64; 30] = [eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29];
        let eq85_branch_derivatives: [f64; 36] = [eq85_e1197_d_b0, eq85_e1197_d_b1, eq85_e1197_d_b2, eq85_e1197_d_b3, eq85_e1197_d_b4, eq85_e1197_d_b5, eq85_e1197_d_b6, eq85_e1197_d_b7, eq85_e1197_d_b8, eq85_e1197_d_b9, eq85_e1197_d_b10, eq85_e1197_d_b11, eq85_e1197_d_b12, eq85_e1197_d_b13, eq85_e1197_d_b14, eq85_e1197_d_b15, eq85_e1197_d_b16, eq85_e1197_d_b17, eq85_e1197_d_b18, eq85_e1197_d_b19, eq85_e1197_d_b20, eq85_e1197_d_b21, eq85_e1197_d_b22, eq85_e1197_d_b23, eq85_e1197_d_b24, eq85_e1197_d_b25, eq85_e1197_d_b26, eq85_e1197_d_b27, eq85_e1197_d_b28, eq85_e1197_d_b29, eq85_e1197_d_b30, eq85_e1197_d_b31, eq85_e1197_d_b32, eq85_e1197_d_b33, eq85_e1197_d_b34, eq85_e1197_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq85_value),
            &eq85_node_derivatives,
            &eq85_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29, eq86_e1207_d_b0, eq86_e1207_d_b1, eq86_e1207_d_b2, eq86_e1207_d_b3, eq86_e1207_d_b4, eq86_e1207_d_b5, eq86_e1207_d_b6, eq86_e1207_d_b7, eq86_e1207_d_b8, eq86_e1207_d_b9, eq86_e1207_d_b10, eq86_e1207_d_b11, eq86_e1207_d_b12, eq86_e1207_d_b13, eq86_e1207_d_b14, eq86_e1207_d_b15, eq86_e1207_d_b16, eq86_e1207_d_b17, eq86_e1207_d_b18, eq86_e1207_d_b19, eq86_e1207_d_b20, eq86_e1207_d_b21, eq86_e1207_d_b22, eq86_e1207_d_b23, eq86_e1207_d_b24, eq86_e1207_d_b25, eq86_e1207_d_b26, eq86_e1207_d_b27, eq86_e1207_d_b28, eq86_e1207_d_b29, eq86_e1207_d_b30, eq86_e1207_d_b31, eq86_e1207_d_b32, eq86_e1207_d_b33, eq86_e1207_d_b34, eq86_e1207_d_b35,) = {
    if s.b[1054] {
        let eq86_e1200: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 70, s.v[168]);
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1203_d_n7: f64 = p.p355;
        let eq86_e1203_d_n9: f64 = (-p.p355);
        let eq86_e1204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 71, eq86_e1203);
        let eq86_e1205: f64 = (eq86_e1200 + eq86_e1204);
        let eq86_e1205_d_n7: f64 = ((s.dn[168][7] * ddt_scale) + (eq86_e1203_d_n7 * ddt_scale));
        let eq86_e1205_d_n9: f64 = ((s.dn[168][9] * ddt_scale) + (eq86_e1203_d_n9 * ddt_scale));
        (eq86_e1205, (s.dn[168][0] * ddt_scale), (s.dn[168][1] * ddt_scale), (s.dn[168][2] * ddt_scale), (s.dn[168][3] * ddt_scale), (s.dn[168][4] * ddt_scale), (s.dn[168][5] * ddt_scale), (s.dn[168][6] * ddt_scale), eq86_e1205_d_n7, (s.dn[168][8] * ddt_scale), eq86_e1205_d_n9, (s.dn[168][10] * ddt_scale), (s.dn[168][11] * ddt_scale), (s.dn[168][12] * ddt_scale), (s.dn[168][13] * ddt_scale), (s.dn[168][14] * ddt_scale), (s.dn[168][15] * ddt_scale), (s.dn[168][16] * ddt_scale), (s.dn[168][17] * ddt_scale), (s.dn[168][18] * ddt_scale), (s.dn[168][19] * ddt_scale), (s.dn[168][20] * ddt_scale), (s.dn[168][21] * ddt_scale), (s.dn[168][22] * ddt_scale), (s.dn[168][23] * ddt_scale), (s.dn[168][24] * ddt_scale), (s.dn[168][25] * ddt_scale), (s.dn[168][26] * ddt_scale), (s.dn[168][27] * ddt_scale), (s.dn[168][28] * ddt_scale), (s.dn[168][29] * ddt_scale), (s.db[168][0] * ddt_scale), (s.db[168][1] * ddt_scale), (s.db[168][2] * ddt_scale), (s.db[168][3] * ddt_scale), (s.db[168][4] * ddt_scale), (s.db[168][5] * ddt_scale), (s.db[168][6] * ddt_scale), (s.db[168][7] * ddt_scale), (s.db[168][8] * ddt_scale), (s.db[168][9] * ddt_scale), (s.db[168][10] * ddt_scale), (s.db[168][11] * ddt_scale), (s.db[168][12] * ddt_scale), (s.db[168][13] * ddt_scale), (s.db[168][14] * ddt_scale), (s.db[168][15] * ddt_scale), (s.db[168][16] * ddt_scale), (s.db[168][17] * ddt_scale), (s.db[168][18] * ddt_scale), (s.db[168][19] * ddt_scale), (s.db[168][20] * ddt_scale), (s.db[168][21] * ddt_scale), (s.db[168][22] * ddt_scale), (s.db[168][23] * ddt_scale), (s.db[168][24] * ddt_scale), (s.db[168][25] * ddt_scale), (s.db[168][26] * ddt_scale), (s.db[168][27] * ddt_scale), (s.db[168][28] * ddt_scale), (s.db[168][29] * ddt_scale), (s.db[168][30] * ddt_scale), (s.db[168][31] * ddt_scale), (s.db[168][32] * ddt_scale), (s.db[168][33] * ddt_scale), (s.db[168][34] * ddt_scale), (s.db[168][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1207;
        let eq86_node_derivatives: [f64; 30] = [eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29];
        let eq86_branch_derivatives: [f64; 36] = [eq86_e1207_d_b0, eq86_e1207_d_b1, eq86_e1207_d_b2, eq86_e1207_d_b3, eq86_e1207_d_b4, eq86_e1207_d_b5, eq86_e1207_d_b6, eq86_e1207_d_b7, eq86_e1207_d_b8, eq86_e1207_d_b9, eq86_e1207_d_b10, eq86_e1207_d_b11, eq86_e1207_d_b12, eq86_e1207_d_b13, eq86_e1207_d_b14, eq86_e1207_d_b15, eq86_e1207_d_b16, eq86_e1207_d_b17, eq86_e1207_d_b18, eq86_e1207_d_b19, eq86_e1207_d_b20, eq86_e1207_d_b21, eq86_e1207_d_b22, eq86_e1207_d_b23, eq86_e1207_d_b24, eq86_e1207_d_b25, eq86_e1207_d_b26, eq86_e1207_d_b27, eq86_e1207_d_b28, eq86_e1207_d_b29, eq86_e1207_d_b30, eq86_e1207_d_b31, eq86_e1207_d_b32, eq86_e1207_d_b33, eq86_e1207_d_b34, eq86_e1207_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq86_value),
            &eq86_node_derivatives,
            &eq86_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29, eq87_e1217_d_b0, eq87_e1217_d_b1, eq87_e1217_d_b2, eq87_e1217_d_b3, eq87_e1217_d_b4, eq87_e1217_d_b5, eq87_e1217_d_b6, eq87_e1217_d_b7, eq87_e1217_d_b8, eq87_e1217_d_b9, eq87_e1217_d_b10, eq87_e1217_d_b11, eq87_e1217_d_b12, eq87_e1217_d_b13, eq87_e1217_d_b14, eq87_e1217_d_b15, eq87_e1217_d_b16, eq87_e1217_d_b17, eq87_e1217_d_b18, eq87_e1217_d_b19, eq87_e1217_d_b20, eq87_e1217_d_b21, eq87_e1217_d_b22, eq87_e1217_d_b23, eq87_e1217_d_b24, eq87_e1217_d_b25, eq87_e1217_d_b26, eq87_e1217_d_b27, eq87_e1217_d_b28, eq87_e1217_d_b29, eq87_e1217_d_b30, eq87_e1217_d_b31, eq87_e1217_d_b32, eq87_e1217_d_b33, eq87_e1217_d_b34, eq87_e1217_d_b35,) = {
    if s.b[1054] {
        let eq87_e1210: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 72, s.v[169]);
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1213_d_n2: f64 = p.p355;
        let eq87_e1213_d_n10: f64 = (-p.p355);
        let eq87_e1214: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 73, eq87_e1213);
        let eq87_e1215: f64 = (eq87_e1210 + eq87_e1214);
        let eq87_e1215_d_n2: f64 = ((s.dn[169][2] * ddt_scale) + (eq87_e1213_d_n2 * ddt_scale));
        let eq87_e1215_d_n10: f64 = ((s.dn[169][10] * ddt_scale) + (eq87_e1213_d_n10 * ddt_scale));
        (eq87_e1215, (s.dn[169][0] * ddt_scale), (s.dn[169][1] * ddt_scale), eq87_e1215_d_n2, (s.dn[169][3] * ddt_scale), (s.dn[169][4] * ddt_scale), (s.dn[169][5] * ddt_scale), (s.dn[169][6] * ddt_scale), (s.dn[169][7] * ddt_scale), (s.dn[169][8] * ddt_scale), (s.dn[169][9] * ddt_scale), eq87_e1215_d_n10, (s.dn[169][11] * ddt_scale), (s.dn[169][12] * ddt_scale), (s.dn[169][13] * ddt_scale), (s.dn[169][14] * ddt_scale), (s.dn[169][15] * ddt_scale), (s.dn[169][16] * ddt_scale), (s.dn[169][17] * ddt_scale), (s.dn[169][18] * ddt_scale), (s.dn[169][19] * ddt_scale), (s.dn[169][20] * ddt_scale), (s.dn[169][21] * ddt_scale), (s.dn[169][22] * ddt_scale), (s.dn[169][23] * ddt_scale), (s.dn[169][24] * ddt_scale), (s.dn[169][25] * ddt_scale), (s.dn[169][26] * ddt_scale), (s.dn[169][27] * ddt_scale), (s.dn[169][28] * ddt_scale), (s.dn[169][29] * ddt_scale), (s.db[169][0] * ddt_scale), (s.db[169][1] * ddt_scale), (s.db[169][2] * ddt_scale), (s.db[169][3] * ddt_scale), (s.db[169][4] * ddt_scale), (s.db[169][5] * ddt_scale), (s.db[169][6] * ddt_scale), (s.db[169][7] * ddt_scale), (s.db[169][8] * ddt_scale), (s.db[169][9] * ddt_scale), (s.db[169][10] * ddt_scale), (s.db[169][11] * ddt_scale), (s.db[169][12] * ddt_scale), (s.db[169][13] * ddt_scale), (s.db[169][14] * ddt_scale), (s.db[169][15] * ddt_scale), (s.db[169][16] * ddt_scale), (s.db[169][17] * ddt_scale), (s.db[169][18] * ddt_scale), (s.db[169][19] * ddt_scale), (s.db[169][20] * ddt_scale), (s.db[169][21] * ddt_scale), (s.db[169][22] * ddt_scale), (s.db[169][23] * ddt_scale), (s.db[169][24] * ddt_scale), (s.db[169][25] * ddt_scale), (s.db[169][26] * ddt_scale), (s.db[169][27] * ddt_scale), (s.db[169][28] * ddt_scale), (s.db[169][29] * ddt_scale), (s.db[169][30] * ddt_scale), (s.db[169][31] * ddt_scale), (s.db[169][32] * ddt_scale), (s.db[169][33] * ddt_scale), (s.db[169][34] * ddt_scale), (s.db[169][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_value: f64 = eq87_e1217;
        let eq87_node_derivatives: [f64; 30] = [eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29];
        let eq87_branch_derivatives: [f64; 36] = [eq87_e1217_d_b0, eq87_e1217_d_b1, eq87_e1217_d_b2, eq87_e1217_d_b3, eq87_e1217_d_b4, eq87_e1217_d_b5, eq87_e1217_d_b6, eq87_e1217_d_b7, eq87_e1217_d_b8, eq87_e1217_d_b9, eq87_e1217_d_b10, eq87_e1217_d_b11, eq87_e1217_d_b12, eq87_e1217_d_b13, eq87_e1217_d_b14, eq87_e1217_d_b15, eq87_e1217_d_b16, eq87_e1217_d_b17, eq87_e1217_d_b18, eq87_e1217_d_b19, eq87_e1217_d_b20, eq87_e1217_d_b21, eq87_e1217_d_b22, eq87_e1217_d_b23, eq87_e1217_d_b24, eq87_e1217_d_b25, eq87_e1217_d_b26, eq87_e1217_d_b27, eq87_e1217_d_b28, eq87_e1217_d_b29, eq87_e1217_d_b30, eq87_e1217_d_b31, eq87_e1217_d_b32, eq87_e1217_d_b33, eq87_e1217_d_b34, eq87_e1217_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(10),
            multiplicity * (eq87_value),
            &eq87_node_derivatives,
            &eq87_branch_derivatives,
            multiplicity,
        );
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29, eq89_e1231_d_b0, eq89_e1231_d_b1, eq89_e1231_d_b2, eq89_e1231_d_b3, eq89_e1231_d_b4, eq89_e1231_d_b5, eq89_e1231_d_b6, eq89_e1231_d_b7, eq89_e1231_d_b8, eq89_e1231_d_b9, eq89_e1231_d_b10, eq89_e1231_d_b11, eq89_e1231_d_b12, eq89_e1231_d_b13, eq89_e1231_d_b14, eq89_e1231_d_b15, eq89_e1231_d_b16, eq89_e1231_d_b17, eq89_e1231_d_b18, eq89_e1231_d_b19, eq89_e1231_d_b20, eq89_e1231_d_b21, eq89_e1231_d_b22, eq89_e1231_d_b23, eq89_e1231_d_b24, eq89_e1231_d_b25, eq89_e1231_d_b26, eq89_e1231_d_b27, eq89_e1231_d_b28, eq89_e1231_d_b29, eq89_e1231_d_b30, eq89_e1231_d_b31, eq89_e1231_d_b32, eq89_e1231_d_b33, eq89_e1231_d_b34, eq89_e1231_d_b35,) = {
    if s.b[1054] {
        let eq89_e1224: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 74, s.v[171]);
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1227_d_n7: f64 = p.p355;
        let eq89_e1227_d_n9: f64 = (-p.p355);
        let eq89_e1228: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 75, eq89_e1227);
        let eq89_e1229: f64 = (eq89_e1224 + eq89_e1228);
        let eq89_e1229_d_n7: f64 = ((s.dn[171][7] * ddt_scale) + (eq89_e1227_d_n7 * ddt_scale));
        let eq89_e1229_d_n9: f64 = ((s.dn[171][9] * ddt_scale) + (eq89_e1227_d_n9 * ddt_scale));
        (eq89_e1229, (s.dn[171][0] * ddt_scale), (s.dn[171][1] * ddt_scale), (s.dn[171][2] * ddt_scale), (s.dn[171][3] * ddt_scale), (s.dn[171][4] * ddt_scale), (s.dn[171][5] * ddt_scale), (s.dn[171][6] * ddt_scale), eq89_e1229_d_n7, (s.dn[171][8] * ddt_scale), eq89_e1229_d_n9, (s.dn[171][10] * ddt_scale), (s.dn[171][11] * ddt_scale), (s.dn[171][12] * ddt_scale), (s.dn[171][13] * ddt_scale), (s.dn[171][14] * ddt_scale), (s.dn[171][15] * ddt_scale), (s.dn[171][16] * ddt_scale), (s.dn[171][17] * ddt_scale), (s.dn[171][18] * ddt_scale), (s.dn[171][19] * ddt_scale), (s.dn[171][20] * ddt_scale), (s.dn[171][21] * ddt_scale), (s.dn[171][22] * ddt_scale), (s.dn[171][23] * ddt_scale), (s.dn[171][24] * ddt_scale), (s.dn[171][25] * ddt_scale), (s.dn[171][26] * ddt_scale), (s.dn[171][27] * ddt_scale), (s.dn[171][28] * ddt_scale), (s.dn[171][29] * ddt_scale), (s.db[171][0] * ddt_scale), (s.db[171][1] * ddt_scale), (s.db[171][2] * ddt_scale), (s.db[171][3] * ddt_scale), (s.db[171][4] * ddt_scale), (s.db[171][5] * ddt_scale), (s.db[171][6] * ddt_scale), (s.db[171][7] * ddt_scale), (s.db[171][8] * ddt_scale), (s.db[171][9] * ddt_scale), (s.db[171][10] * ddt_scale), (s.db[171][11] * ddt_scale), (s.db[171][12] * ddt_scale), (s.db[171][13] * ddt_scale), (s.db[171][14] * ddt_scale), (s.db[171][15] * ddt_scale), (s.db[171][16] * ddt_scale), (s.db[171][17] * ddt_scale), (s.db[171][18] * ddt_scale), (s.db[171][19] * ddt_scale), (s.db[171][20] * ddt_scale), (s.db[171][21] * ddt_scale), (s.db[171][22] * ddt_scale), (s.db[171][23] * ddt_scale), (s.db[171][24] * ddt_scale), (s.db[171][25] * ddt_scale), (s.db[171][26] * ddt_scale), (s.db[171][27] * ddt_scale), (s.db[171][28] * ddt_scale), (s.db[171][29] * ddt_scale), (s.db[171][30] * ddt_scale), (s.db[171][31] * ddt_scale), (s.db[171][32] * ddt_scale), (s.db[171][33] * ddt_scale), (s.db[171][34] * ddt_scale), (s.db[171][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1231;
        let eq89_node_derivatives: [f64; 30] = [eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29];
        let eq89_branch_derivatives: [f64; 36] = [eq89_e1231_d_b0, eq89_e1231_d_b1, eq89_e1231_d_b2, eq89_e1231_d_b3, eq89_e1231_d_b4, eq89_e1231_d_b5, eq89_e1231_d_b6, eq89_e1231_d_b7, eq89_e1231_d_b8, eq89_e1231_d_b9, eq89_e1231_d_b10, eq89_e1231_d_b11, eq89_e1231_d_b12, eq89_e1231_d_b13, eq89_e1231_d_b14, eq89_e1231_d_b15, eq89_e1231_d_b16, eq89_e1231_d_b17, eq89_e1231_d_b18, eq89_e1231_d_b19, eq89_e1231_d_b20, eq89_e1231_d_b21, eq89_e1231_d_b22, eq89_e1231_d_b23, eq89_e1231_d_b24, eq89_e1231_d_b25, eq89_e1231_d_b26, eq89_e1231_d_b27, eq89_e1231_d_b28, eq89_e1231_d_b29, eq89_e1231_d_b30, eq89_e1231_d_b31, eq89_e1231_d_b32, eq89_e1231_d_b33, eq89_e1231_d_b34, eq89_e1231_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq89_value),
            &eq89_node_derivatives,
            &eq89_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29, eq90_e1242_d_b0, eq90_e1242_d_b1, eq90_e1242_d_b2, eq90_e1242_d_b3, eq90_e1242_d_b4, eq90_e1242_d_b5, eq90_e1242_d_b6, eq90_e1242_d_b7, eq90_e1242_d_b8, eq90_e1242_d_b9, eq90_e1242_d_b10, eq90_e1242_d_b11, eq90_e1242_d_b12, eq90_e1242_d_b13, eq90_e1242_d_b14, eq90_e1242_d_b15, eq90_e1242_d_b16, eq90_e1242_d_b17, eq90_e1242_d_b18, eq90_e1242_d_b19, eq90_e1242_d_b20, eq90_e1242_d_b21, eq90_e1242_d_b22, eq90_e1242_d_b23, eq90_e1242_d_b24, eq90_e1242_d_b25, eq90_e1242_d_b26, eq90_e1242_d_b27, eq90_e1242_d_b28, eq90_e1242_d_b29, eq90_e1242_d_b30, eq90_e1242_d_b31, eq90_e1242_d_b32, eq90_e1242_d_b33, eq90_e1242_d_b34, eq90_e1242_d_b35,) = {
    if (!s.b[1054]) {
        let eq90_e1235: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 76, s.v[167]);
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1238_d_n2: f64 = p.p355;
        let eq90_e1238_d_n10: f64 = (-p.p355);
        let eq90_e1239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 77, eq90_e1238);
        let eq90_e1240: f64 = (eq90_e1235 + eq90_e1239);
        let eq90_e1240_d_n2: f64 = ((s.dn[167][2] * ddt_scale) + (eq90_e1238_d_n2 * ddt_scale));
        let eq90_e1240_d_n10: f64 = ((s.dn[167][10] * ddt_scale) + (eq90_e1238_d_n10 * ddt_scale));
        (eq90_e1240, (s.dn[167][0] * ddt_scale), (s.dn[167][1] * ddt_scale), eq90_e1240_d_n2, (s.dn[167][3] * ddt_scale), (s.dn[167][4] * ddt_scale), (s.dn[167][5] * ddt_scale), (s.dn[167][6] * ddt_scale), (s.dn[167][7] * ddt_scale), (s.dn[167][8] * ddt_scale), (s.dn[167][9] * ddt_scale), eq90_e1240_d_n10, (s.dn[167][11] * ddt_scale), (s.dn[167][12] * ddt_scale), (s.dn[167][13] * ddt_scale), (s.dn[167][14] * ddt_scale), (s.dn[167][15] * ddt_scale), (s.dn[167][16] * ddt_scale), (s.dn[167][17] * ddt_scale), (s.dn[167][18] * ddt_scale), (s.dn[167][19] * ddt_scale), (s.dn[167][20] * ddt_scale), (s.dn[167][21] * ddt_scale), (s.dn[167][22] * ddt_scale), (s.dn[167][23] * ddt_scale), (s.dn[167][24] * ddt_scale), (s.dn[167][25] * ddt_scale), (s.dn[167][26] * ddt_scale), (s.dn[167][27] * ddt_scale), (s.dn[167][28] * ddt_scale), (s.dn[167][29] * ddt_scale), (s.db[167][0] * ddt_scale), (s.db[167][1] * ddt_scale), (s.db[167][2] * ddt_scale), (s.db[167][3] * ddt_scale), (s.db[167][4] * ddt_scale), (s.db[167][5] * ddt_scale), (s.db[167][6] * ddt_scale), (s.db[167][7] * ddt_scale), (s.db[167][8] * ddt_scale), (s.db[167][9] * ddt_scale), (s.db[167][10] * ddt_scale), (s.db[167][11] * ddt_scale), (s.db[167][12] * ddt_scale), (s.db[167][13] * ddt_scale), (s.db[167][14] * ddt_scale), (s.db[167][15] * ddt_scale), (s.db[167][16] * ddt_scale), (s.db[167][17] * ddt_scale), (s.db[167][18] * ddt_scale), (s.db[167][19] * ddt_scale), (s.db[167][20] * ddt_scale), (s.db[167][21] * ddt_scale), (s.db[167][22] * ddt_scale), (s.db[167][23] * ddt_scale), (s.db[167][24] * ddt_scale), (s.db[167][25] * ddt_scale), (s.db[167][26] * ddt_scale), (s.db[167][27] * ddt_scale), (s.db[167][28] * ddt_scale), (s.db[167][29] * ddt_scale), (s.db[167][30] * ddt_scale), (s.db[167][31] * ddt_scale), (s.db[167][32] * ddt_scale), (s.db[167][33] * ddt_scale), (s.db[167][34] * ddt_scale), (s.db[167][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_value: f64 = eq90_e1242;
        let eq90_node_derivatives: [f64; 30] = [eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29];
        let eq90_branch_derivatives: [f64; 36] = [eq90_e1242_d_b0, eq90_e1242_d_b1, eq90_e1242_d_b2, eq90_e1242_d_b3, eq90_e1242_d_b4, eq90_e1242_d_b5, eq90_e1242_d_b6, eq90_e1242_d_b7, eq90_e1242_d_b8, eq90_e1242_d_b9, eq90_e1242_d_b10, eq90_e1242_d_b11, eq90_e1242_d_b12, eq90_e1242_d_b13, eq90_e1242_d_b14, eq90_e1242_d_b15, eq90_e1242_d_b16, eq90_e1242_d_b17, eq90_e1242_d_b18, eq90_e1242_d_b19, eq90_e1242_d_b20, eq90_e1242_d_b21, eq90_e1242_d_b22, eq90_e1242_d_b23, eq90_e1242_d_b24, eq90_e1242_d_b25, eq90_e1242_d_b26, eq90_e1242_d_b27, eq90_e1242_d_b28, eq90_e1242_d_b29, eq90_e1242_d_b30, eq90_e1242_d_b31, eq90_e1242_d_b32, eq90_e1242_d_b33, eq90_e1242_d_b34, eq90_e1242_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(10),
            multiplicity * (eq90_value),
            &eq90_node_derivatives,
            &eq90_branch_derivatives,
            multiplicity,
        );
    }
}
