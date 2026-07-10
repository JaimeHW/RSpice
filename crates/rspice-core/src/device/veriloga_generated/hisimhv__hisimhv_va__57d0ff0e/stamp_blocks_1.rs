#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1320]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1320])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1320])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1320])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1321] = ((p.p39 == 0.0) || (p.p39 == 1.0));s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1321]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1321]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1321]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1321])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1321])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1321])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1289] && s.b[1313]) && (!s.b[1318])) {s.store_scalar(692, 0.0);s.store_scalar(693, 0.0);}
        if s.b[1289] {s.store_scaled_sqrt(139, 155, s.v[639]);s.store_square(694, 139);s.store_scaled_square(140, 394, s.v[640]);s.store_offset_scaled(427, 391, p.p448, p.p447);s.store_scalar(957, p.p193);}
        s.b[1324] = (s.v[957] < 0.0);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1324]) {s.store_scalar(957, 0.0);}
        s.b[1325] = (s.v[957] > 0.005);s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1325]) {s.store_scalar(957, 0.005);}
        s.b[1326] = (s.v[449] > 0.0);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1326]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }
        if (s.b[1289] && s.b[1326]) {s.store_div_from_scalar(794, s.v[569], 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1289] && s.b[1326]) {s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));s.store_div_from_scalar(795, s.v[570], 334);s.store_offset_scaled(959, 387, p.p439, (((((-s.v[764])) * (p.p439))) + (s.v[959])));}
        if (s.b[1289] && s.b[1326]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }
        if (s.b[1289] && s.b[1326]) {s.store_div_from_scalar(787, s.v[567], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));s.store_div_from_scalar(788, s.v[568], 334);s.store_offset_scaled(956, 387, p.p438, (((((-s.v[764])) * (p.p438))) + (s.v[956])));}
        s.b[1328] = (s.v[956] < 0.1);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if ((s.b[1289] && s.b[1326]) && s.b[1328]) {s.store_scalar(956, 0.1);}
        if s.b[1289] {s.store_square(334, 676);s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);}
        s.b[1329] = (p.p48 > 0.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });s.b[1330] = (p.p15 > s.v[632]);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });
        if ((s.b[1289] && s.b[1329]) && s.b[1330]) {s.store_scale(873, 828, p.p13);s.store_scale(874, 830, p.p13);s.store_scale(875, 829, (p.p15 - s.v[632]));s.store_scale(876, 831, (p.p15 - s.v[632]));s.store_scale(877, 836, s.v[632]);s.store_scale(878, 837, s.v[632]);}
        if ((s.b[1289] && s.b[1329]) && (!s.b[1330])) {s.store_scale(873, 828, p.p13);s.store_scale(874, 830, p.p13);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scale(877, 836, p.p15);s.store_scale(878, 837, p.p15);}
        if (s.b[1289] && (!s.b[1329])) {s.store_scale(873, 828, p.p13);s.store_scale(874, 830, p.p13);s.store_scale(875, 829, p.p15);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1289] && (!s.b[1329])) {s.store_scale(876, 831, p.p15);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);}
        if s.b[1289] {s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);}
        s.b[1331] = (s.v[847] > 0.0);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1331]) {s.store_offset(336, 847, 1e-25);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));s.store_exp_mul(851, 848, 850);}
        if s.b[1289] {s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);}
        s.b[1332] = (p.p48 > 0.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });s.b[1333] = (p.p16 > s.v[632]);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if ((s.b[1289] && s.b[1332]) && s.b[1333]) {s.store_scale(879, 828, p.p14);s.store_scale(880, 830, p.p14);s.store_scale(881, 829, (p.p16 - s.v[632]));s.store_scale(882, 831, (p.p16 - s.v[632]));s.store_scale(883, 836, s.v[632]);s.store_scale(884, 837, s.v[632]);}
        if ((s.b[1289] && s.b[1332]) && (!s.b[1333])) {s.store_scale(879, 828, p.p14);s.store_scale(880, 830, p.p14);s.store_scalar(881, 0.0);s.store_scalar(882, 0.0);s.store_scale(883, 836, p.p16);s.store_scale(884, 837, p.p16);}
        if (s.b[1289] && (!s.b[1332])) {s.store_scale(879, 828, p.p14);s.store_scale(880, 830, p.p14);s.store_scale(881, 829, p.p16);s.store_scale(882, 831, p.p16);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);}
        if s.b[1289] {s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);}
        s.b[1334] = (s.v[852] > 0.0);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1334]) {s.store_offset(337, 852, 1e-25);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1289] && s.b[1334]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));s.store_exp_mul(856, 853, 855);}
        if s.b[1289] {s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));}
        s.b[1335] = (p.p15 > s.v[632]);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1335]) {s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));}
        if (s.b[1289] && (!s.b[1335])) {s.store_scalar(833, 0.0);s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));}
        s.b[1336] = (s.v[832] < 0.0);s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1336]) {s.store_scalar(832, 0.0);}
        s.b[1337] = (s.v[833] < 0.0);s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1337]) {s.store_scalar(833, 0.0);}
        s.b[1338] = (s.v[834] < 0.0);s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1338]) {s.store_scalar(834, 0.0);}
        if s.b[1289] {s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);}
        s.b[1339] = ((s.v[841] < 0.01) && (p.p13 > 0.0));s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1339]) {s.store_scalar(841, 0.01);}
        s.b[1340] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1340]) {s.store_scalar(842, 0.01);}
        s.b[1341] = ((s.v[843] < 0.01) && (p.p15 > 0.0));s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1341]) {s.store_scalar(843, 0.01);}
        if s.b[1289] {s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));}
        s.b[1342] = (p.p16 > s.v[632]);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1342]) {s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));}
        if (s.b[1289] && (!s.b[1342])) {s.store_scalar(838, 0.0);s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));}
        s.b[1343] = (s.v[835] < 0.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1343]) {s.store_scalar(835, 0.0);}
        s.b[1344] = (s.v[838] < 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1344]) {s.store_scalar(838, 0.0);}
        s.b[1345] = (s.v[839] < 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1345]) {s.store_scalar(839, 0.0);}
        if s.b[1289] {s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);}
        s.b[1346] = ((s.v[844] < 0.01) && (p.p14 > 0.0));s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1346]) {s.store_scalar(844, 0.01);}
        s.b[1347] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1347]) {s.store_scalar(845, 0.01);}
        s.b[1348] = ((s.v[846] < 0.01) && (p.p16 > 0.0));s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (s.b[1289] && s.b[1348]) {s.store_scalar(846, 0.01);}
        s.store_scaled_voltage(729, ctx, nodes, Some(6), Some(8), p.p87);s.store_scaled_voltage(731, ctx, nodes, Some(7), Some(8), p.p87);s.store_scaled_voltage(728, ctx, nodes, Some(9), Some(8), p.p87);s.store_scaled_voltage(733, ctx, nodes, Some(0), Some(2), p.p87);s.store_scaled_voltage(734, ctx, nodes, Some(7), Some(2), p.p87);s.store_scaled_voltage(735, ctx, nodes, Some(9), Some(2), p.p87);s.store_scaled_voltage(799, ctx, nodes, Some(0), Some(6), p.p87);s.store_scaled_voltage(804, ctx, nodes, Some(8), Some(2), p.p87);s.store_scaled_voltage(857, ctx, nodes, Some(11), Some(2), p.p87);s.store_scaled_voltage(858, ctx, nodes, Some(10), Some(0), p.p87);s.store_scaled_voltage(865, ctx, nodes, Some(9), Some(8), p.p87);s.store_scaled_voltage(866, ctx, nodes, Some(9), Some(6), p.p87);s.copy_ad(859, 857);s.copy_ad(860, 858);s.copy_ad(867, 865);s.copy_ad(868, 866);s.store_scaled_voltage(798, ctx, nodes, Some(4), Some(2), p.p87);
        if (s.v[81] != 0.0) {s.store_voltage(747, ctx, nodes, Some(12), None);s.store_voltage(748, ctx, nodes, Some(13), None);}
        if (s.v[81] == 0.0) {s.store_scalar(747, 0.0);s.store_scalar(748, 0.0);}
        s.store_sub(730, 731, 729);s.store_sub(727, 728, 729);s.b[1349] = (s.v[729] >= 0.0);s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });
        if s.b[1349] {s.store_scalar(949, 1.0);s.copy_ad(790, 729);s.copy_ad(791, 731);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1349] {s.copy_ad(792, 728);s.copy_ad(793, 733);s.copy_ad(796, 734);s.copy_ad(797, 735);}
        if (!s.b[1349]) {s.store_scalar(949, (-1.0));s.store_neg(790, 729);s.copy_ad(791, 730);s.copy_ad(792, 727);s.store_neg(793, 733);s.store_sub(796, 734, 733);s.store_sub(797, 735, 733);}
        s.b[1352] = ((p.p53 > 0.0) && (s.v[541] != 0.0));s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if s.b[1352] {s.store_voltage(732, ctx, nodes, Some(5), None);}
        s.b[1353] = (p.p53 == 2.0);s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1353]) {s.store_offset_sub_from_scalar_ad(781, p.p433, s.ad_value(732), (-(p.p337 * 10.0)));s.store_scalar(782, ((4.0 * p.p433) * (p.p337 * 10.0)));}
        if (s.b[1352] && s.b[1353]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1352] && s.b[1353]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(732, 781, (-0.5), 782, (-0.5), p.p433);}
        if s.b[1352] {s.store_scalar(387, (ctx_temp + p.p11));s.copy_ad(388, 387);s.store_add(387, 387, 732);s.store_offset(389, 388, (-s.v[764]));s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));s.store_offset(391, 387, (-s.v[764]));s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));s.store_scale(676, 387, 1.0 / (s.v[764]));s.store_ln(590, 676);s.store_sub_scaled_inputs_mixed_ai(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 1.0, 392, s.v[456]);s.store_sqrt(677, 393);s.store_div_from_scalar(335, 1.0, 387);s.store_scalar(336, (1.0 / s.v[764]));s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p.p260, 336, (-p.p260), A::square(s.ad_value(335)), p.p261, A::square(s.ad_value(336)), (-p.p261), (s.v[616] + p.p259));s.store_sqrt(192, 337);s.store_mul(193, 337, 192);s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);s.store_div_from_scalar(155, 1.0, 154);s.store_square(156, 154);s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);s.store_exp_scaled_input(335, 590, s.v[480]);s.store_div(679, 335, 573);}
        s.b[1355] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1355]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p.p380);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));s.store_div(973, 973, 334);}
        s.b[1357] = (s.v[973] < 1000.0);s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if ((s.b[1352] && s.b[1355]) && s.b[1357]) {s.store_scalar(973, 1000.0);}
        if (s.b[1352] && s.b[1355]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p.p381));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1352] && s.b[1355]) {s.store_div_mixed_ia(970, 970, A::powf(s.ad_value(676), p.p382));}
        s.b[1358] = (s.v[963] == 3.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if ((s.b[1352] && (!s.b[1355])) && s.b[1358]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p.p380);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));s.store_div(973, 973, 334);}
        s.b[1360] = (s.v[973] < 1000.0);s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if (((s.b[1352] && (!s.b[1355])) && s.b[1358]) && s.b[1360]) {s.store_scalar(973, 1000.0);}
        if ((s.b[1352] && (!s.b[1355])) && s.b[1358]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p.p381));s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));}
        if ((s.b[1352] && (!s.b[1355])) && (!s.b[1358])) {s.store_scalar(961, 0.0);s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));s.store_scalar(977, 0.0);}
        if s.b[1352] {s.store_mul(680, 638, 155);s.store_scale(335, 387, 1.0 / (s.v[764]));s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));}
        s.b[1361] = (p.p39 != 2.0);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1361]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));}
        if (s.b[1352] && (!s.b[1361])) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));}
        s.b[1363] = (p.p39 != 2.0);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1363]) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 1.0, 390, p.p391);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        if (s.b[1352] && (!s.b[1363])) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 1.0, 392, p.p391);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        s.b[1365] = (s.v[682] < 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1365]) {s.store_scalar(682, 0.0);}
        s.b[1367] = (s.v[688] < 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1367]) {s.store_scalar(688, 0.0);}
        s.b[1369] = (s.v[689] < 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1369]) {s.store_scalar(689, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1352] && (p.p53 != 0.0)) {s.store_add_scaled_inputs_mixed_ai(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));}
        s.b[1371] = (s.v[766] < 0.0001);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((s.b[1352] && (p.p53 != 0.0)) && s.b[1371]) {s.store_scalar(766, 0.0001);}
        if s.b[1352] {s.store_add_scaled_inputs_mixed_ai(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 1.0, 390, p.p331);s.store_offset(781, 336, (-0.05));s.store_scalar(782, 0.0);}
        if s.b[1352] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1352] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1352] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1352] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_sqrt_div(684, 335, 586);s.store_sqrt_div(685, 335, 621);}
        s.b[1372] = (s.v[963] == 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1372]) {s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div(335, 394, 586);s.store_square(210, 335);}
        s.b[1373] = (s.v[963] == 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });s.b[1374] = (s.v[459] != 0.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((s.b[1352] && s.b[1373]) && s.b[1374]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));}
        s.b[1375] = (s.v[460] != 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if ((s.b[1352] && s.b[1373]) && s.b[1375]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));}
        s.b[1376] = (s.v[459] != 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if ((s.b[1352] && (!s.b[1373])) && s.b[1376]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));}
        s.b[1377] = (s.v[460] != 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((s.b[1352] && (!s.b[1373])) && s.b[1377]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));}
        s.b[1378] = (s.v[449] == 0.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });s.b[1379] = (s.v[530] > 0.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((s.b[1352] && s.b[1378]) && s.b[1379]) {s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));}
        s.b[1380] = (p.p39 == 1.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && s.b[1380]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, 390, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && s.b[1380]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && s.b[1380]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && (!s.b[1380])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, 392, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && (!s.b[1380])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1379]) && (!s.b[1380])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if ((s.b[1352] && s.b[1378]) && (!s.b[1379])) {s.store_scalar(690, 0.0);}
        s.b[1381] = (s.v[540] > 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1352] && s.b[1378]) && s.b[1381]) {s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));}
        s.b[1382] = (p.p39 == 1.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && s.b[1382]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, 390, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && s.b[1382]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && s.b[1382]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && (!s.b[1382])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, 392, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && (!s.b[1382])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1381]) && (!s.b[1382])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if ((s.b[1352] && s.b[1378]) && (!s.b[1381])) {s.store_scalar(691, 0.0);}
        s.b[1383] = (s.v[538] > 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(336, 782, p.p99, 0.5);}
        s.b[1384] = (s.v[336] < 0.0);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1384]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_div_from_scalar(342, (-p.p98), 336);s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1385] = ((p.p39 == 0.0) || (p.p39 == 1.0));s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1385]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1385]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1385]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1385])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1385])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1385])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1352] && s.b[1378]) && s.b[1383]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1386] = ((p.p39 == 0.0) || (p.p39 == 1.0));s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1386]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1386]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && s.b[1386]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1386])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, 392, s.v[558]);s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1386])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1352] && s.b[1378]) && s.b[1383]) && (!s.b[1386])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));}
        if ((s.b[1352] && s.b[1378]) && (!s.b[1383])) {s.store_scalar(692, 0.0);s.store_scalar(693, 0.0);}
        if s.b[1352] {s.store_scaled_sqrt(139, 155, s.v[639]);s.store_square(694, 139);s.store_scaled_square(140, 394, s.v[640]);s.store_offset_scaled(427, 391, p.p448, p.p447);s.store_scalar(957, p.p193);}
        s.b[1389] = (s.v[957] < 0.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1389]) {s.store_scalar(957, 0.0);}
        s.b[1390] = (s.v[957] > 0.005);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });
        if (s.b[1352] && s.b[1390]) {s.store_scalar(957, 0.005);}
        if (!s.b[1352]) {s.store_scalar(387, (ctx_temp + p.p11));}
        s.store_scalar(164, (s.v[630] * p.p7));s.store_scalar(165, (p.p67 + p.p68));s.store_scalar(160, s.v[462]);s.copy_ad(257, 681);s.store_scalar(161, s.v[617]);s.store_scalar(187, p.p95);s.store_scalar(188, (s.v[161] / s.v[187]));s.store_scalar(189, (1.0 / s.v[188]));s.store_primal_div_from_scalar(412, s.v[161], 543);s.store_scalar(270, (p.p87 * p.p434));s.store_offset_sub_from_scalar_ad(781, 0.8, A::offset(s.ad_value(157), (-p.p262)), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, (-0.5), 782, (-0.5), 0.8);s.copy_ad(69, 335);s.b[1391] = ((s.v[158] - p.p262) < s.v[69]);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if s.b[1391] {s.store_offset(69, 158, (-p.p262));}
        s.b[1392] = ((s.v[159] - p.p262) < s.v[69]);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if s.b[1392] {s.store_offset(69, 159, (-p.p262));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1393] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });s.b[1394] = ((s.v[961] - p.p262) < s.v[69]);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if (s.b[1393] && s.b[1394]) {s.store_offset(69, 961, (-p.p262));}
        s.b[1395] = ((s.v[960] - p.p262) < s.v[69]);s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (s.b[1393] && s.b[1395]) {s.store_offset(69, 960, (-p.p262));}
        s.b[1396] = (s.v[70] > (s.v[69] * 0.5));s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if s.b[1396] {s.store_scale(70, 69, 0.5);}
        s.b[1397] = param_given[338];s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if s.b[1397] {s.store_scalar(72, p.p338);}
        if (!s.b[1397]) {s.copy_ad(72, 69);}
        s.b[1398] = param_given[339];s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if s.b[1398] {s.store_scalar(73, p.p339);}
        s.b[1399] = param_given[338];s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if ((!s.b[1398]) && s.b[1399]) {s.store_scale(73, 72, 0.5);}
        if ((!s.b[1398]) && (!s.b[1399])) {s.copy_ad(73, 70);}
        s.b[1400] = (s.v[73] > (s.v[72] * 0.5));s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if s.b[1400] {s.store_scale(73, 72, 0.5);}
        s.b[1401] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });s.b[1402] = (s.v[448] == 1.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (s.b[1401] && s.b[1402]) {
        (1.0,)
    } else {
        (s.v[74],)
    }
};
        s.store_scalar(74, t0);s.b[1403] = (s.v[448] == 2.0);s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        let (t1,) = {
    if (s.b[1401] && s.b[1403]) {
        (2.0,)
    } else {
        (s.v[74],)
    }
};
        s.store_scalar(74, t1);s.b[1404] = (s.v[448] == 3.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (s.b[1401] && s.b[1404]) {
        (3.0,)
    } else {
        (s.v[74],)
    }
};
        s.store_scalar(74, t2);s.store_scalar(77, 0.0);s.b[1405] = (((s.v[449] == 1.0) && (p.p54 == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if s.b[1405] {s.copy_ad(373, 733);}
        s.b[1406] = (s.v[373] >= 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (s.b[1405] && s.b[1406]) {s.copy_ad(376, 373);s.copy_ad(383, 798);}
        if (s.b[1405] && (!s.b[1406])) {s.store_neg(376, 373);s.store_sub(383, 798, 373);}
        if s.b[1405] {s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[1407] = (s.v[108] < 1e-12);s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if (s.b[1405] && s.b[1407]) {s.store_scalar(108, 1e-12);}
        if s.b[1405] {s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);s.store_sub_scaled_inputs_mixed_ai(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 1.0, 383, p.p332);s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);s.store_scaled_add(335, 334, 782, 0.5);}
        s.b[1408] = (s.v[335] < 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (s.b[1405] && s.b[1408]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if s.b[1405] {s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1405] {s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));}
        if s.b[1405] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1405] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);}
        if (!s.b[1405]) {s.store_scalar(384, 0.0);}
        s.b[1409] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if s.b[1409] {s.copy_ad(373, 733);s.copy_ad(374, 734);s.copy_ad(372, 735);}
        s.b[1410] = (s.v[373] >= 0.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (s.b[1409] && s.b[1410]) {s.store_scalar(370, 1.0);s.store_scalar(371, 0.0);s.copy_ad(376, 373);s.copy_ad(377, 374);s.copy_ad(375, 372);s.copy_ad(383, 798);}
        if (s.b[1409] && (!s.b[1410])) {s.store_scalar(370, 0.0);s.store_scalar(371, 1.0);s.store_neg(376, 373);s.store_sub(377, 374, 373);s.store_sub(375, 372, 373);s.store_sub(383, 798, 373);}
        s.b[1411] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p.p54 == 1.0));s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (s.b[1409] && s.b[1411]) {s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[1412] = (s.v[108] < 1e-12);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if ((s.b[1409] && s.b[1411]) && s.b[1412]) {s.store_scalar(108, 1e-12);}
        if (s.b[1409] && s.b[1411]) {s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);s.store_add(381, 377, 108);s.store_add(382, 375, 108);}
        s.b[1413] = ((p.p34 == 1.0) || (s.v[370] == 1.0));s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);s.store_scalar(782, ((((p.p292 * p.p292) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p.p292, s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(344, 782, p.p292, 0.5);}
        s.b[1414] = (s.v[344] < 0.0);s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if (((s.b[1409] && s.b[1411]) && s.b[1413]) && s.b[1414]) {s.store_scalar(344, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {s.store_mul_scale_offset_mixed_ia(335, 338, A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0)));s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (1.0 + s.v[539]));s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);s.store_scaled_add(337, 335, 782, 0.5);}
        s.b[1415] = (s.v[337] < 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if (((s.b[1409] && s.b[1411]) && s.b[1413]) && s.b[1415]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if ((s.b[1409] && s.b[1411]) && s.b[1413]) {s.store_offset(337, 337, 1e-25);s.copy_ad(334, 366);s.store_mul(366, 366, 337);}
        if ((s.b[1409] && s.b[1411]) && (!s.b[1413])) {s.copy_ad(366, 691);}
        if (s.b[1409] && s.b[1411]) {s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);}
        s.b[1416] = ((p.p34 == 1.0) || (s.v[371] == 1.0));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p.p262));s.store_scalar(344, (p.p292 + 1e-25));s.store_mul_scale_offset_mixed_ia(335, 338, A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0)));s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (1.0 + s.v[539]));s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));}
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);s.store_scaled_add(337, 335, 782, 0.5);}
        s.b[1417] = (s.v[337] < 0.0);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if (((s.b[1409] && s.b[1411]) && s.b[1416]) && s.b[1417]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if ((s.b[1409] && s.b[1411]) && s.b[1416]) {s.store_offset(337, 337, 1e-25);s.copy_ad(334, 367);s.store_mul(367, 367, 337);}
        if ((s.b[1409] && s.b[1411]) && (!s.b[1416])) {s.copy_ad(367, 691);}
        s.b[1418] = (((p.p54 == 1.0) && (p.p34 == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {s.store_sub_scaled_inputs_mixed_ai(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 1.0, 383, p.p332);s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);s.store_scaled_add(335, 334, 782, 0.5);}
        s.b[1419] = (s.v[335] < 0.0);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if (((s.b[1409] && s.b[1411]) && s.b[1418]) && s.b[1419]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));s.store_scalar(334, (s.v[544] / (s.v[459] * (s.v[544] + s.v[459]))));s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1409] && s.b[1411]) && s.b[1418]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p.p334, s.ad_value(384));s.store_mul(334, 366, 340);s.store_mul(335, 367, 340);s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);}
        if ((s.b[1409] && s.b[1411]) && (!s.b[1418])) {s.store_scalar(384, 0.0);}
        if (s.b[1409] && s.b[1411]) {s.copy_ad(4, 366);s.copy_ad(5, 367);}
        if (s.b[1409] && (!s.b[1411])) {s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);}
        if s.b[1409] {s.store_scale(4, 4, 1.0 / (s.v[164]));s.store_scale(5, 5, 1.0 / (s.v[164]));s.store_add_scaled_value_products_indices(4, 4, 1.0, 370, 644, 1.0, 371, 648, 1.0);s.store_add_scaled_value_products_indices(5, 5, 1.0, 370, 648, 1.0, 371, 644, 1.0);s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);}
        s.b[1422] = (s.v[792] > s.v[70]);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if s.b[1422] {s.store_sub(335, 792, 70);s.store_sub(336, 69, 70);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(84, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 84, 1.0);s.store_neg(84, 84);s.store_add(83, 70, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(84, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if (!s.b[1422]) {s.copy_ad(83, 792);s.store_scalar(84, 1.0);}
        s.store_scaled_mul(335, 84, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1423] = (s.v[108] < 1e-12);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if s.b[1423] {s.store_scalar(108, 1e-12);}
        s.store_add(105, 83, 108);s.store_add_scaled_inputs(106, 790, 1.0, 108, 2.0);s.store_add(107, 791, 108);s.store_scale(335, 636, (s.v[189] * s.v[189]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset(336, 791, (-s.v[160]));s.store_offset_mul_ad(337, A::div_from_scalar(2.0, s.ad_value(335)), A::add_scaled_inputs3(s.ad_value(336), 1.0, A::div_from_scalar(1.0, s.ad_value(678)), (-1.0), s.ad_value(83), -1.0), 1.0);s.store_sqrt_square_offset(782, 337, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(339, 337, 782, 0.5, 0.5);s.store_scaled_add(338, 337, 782, 0.5);s.b[1424] = (s.v[338] < 0.0);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if s.b[1424] {s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);}
        s.store_offset(338, 338, 1e-25);s.store_sqrt(332, 338);s.store_add_mul_sub_from_scalar_rhs_indices(128, 336, 335, 1.0, 332);s.store_sub(129, 128, 159);s.store_offset(781, 129, (((-0.1)) + ((-0.05))));s.store_scalar(782, ((4.0 * 0.1) * 0.05));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(129, 781, 0.5, 782, 0.5, 0.1);s.store_div(335, 790, 129);s.copy_ad(781, 335);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(332, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(334, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(332), -1.0, 0.0, 332);s.store_sub_from_scalar(332, 1.0, 332);s.store_neg(334, 334);s.store_square(208, 332);s.b[1425] = (s.v[765] == 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        let (t3,) = {
    if s.b[1425] {
        (0.0,)
    } else {
        (s.v[80],)
    }
};
        s.store_scalar(80, t3);
        let (t4,) = {
    if (!s.b[1425]) {
        (1.0,)
    } else {
        (s.v[80],)
    }
};
        s.store_scalar(80, t4);s.copy_ad(335, 637);s.store_sqrt_mul(336, 335, 158);s.store_add_scaled_inputs_mixed_ai(190, A::offset(s.ad_value(158), s.v[160]), 1.0, 336, s.v[189]);s.b[1426] = (s.v[80] == 0.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if s.b[1426] {s.store_scalar(183, s.v[187]);s.store_scalar(185, s.v[188]);s.store_scalar(186, s.v[189]);s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
        if (!s.b[1426]) {s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p.p236);s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);s.store_scaled_add(336, 339, 782, 0.5);}
        s.b[1427] = (s.v[336] < 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((!s.b[1426]) && s.b[1427]) {s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[1426]) {s.store_offset(336, 336, 1e-25);s.store_div_from_scalar(337, 1.0, 336);s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));s.store_scaled_abs(338, 190, 2.0);s.store_offset_sub(340, 339, 791, s.v[160]);}
        s.b[1428] = (s.v[340] > s.v[338]);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if ((!s.b[1426]) && s.b[1428]) {s.copy_ad(338, 340);}
        if (!s.b[1426]) {s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));}
        if (!s.b[1426]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (!s.b[1426]) {s.store_sqrt_square_add(782, 781, 782);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1426]) {s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));s.store_offset_scaled(184, 336, p.p235, p.p237);s.store_scalar(341, p.p235);}
        s.b[1429] = ((s.v[184] * 1000000000000.0) < s.v[187]);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if ((!s.b[1426]) && s.b[1429]) {s.store_scalar(184, 0.0);}
        let (t5,) = {
    if ((!s.b[1426]) && s.b[1429]) {
        (0.0,)
    } else {
        (s.v[80],)
    }
};
        s.store_scalar(80, t5);
        if (!s.b[1426]) {s.store_offset(183, 184, s.v[187]);s.store_div_from_scalar(185, s.v[161], 183);s.store_div_from_scalar_square_ad(335, (-s.v[161]), s.ad_value(183));s.store_scale(186, 183, 1.0 / (s.v[161]));s.store_scalar(335, (1.0 / s.v[161]));s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
        s.copy_ad(364, 105);s.copy_ad(335, 637);s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);s.store_add_mixed_ai(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);s.copy_ad(123, 158);s.store_scalar(334, 0.95);s.b[338] = (!(s.v[963] > 1.0));s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));
        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p.p366);
        } else {
            s.store_scalar(339, 0.0);
        }
        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);s.store_sqrt(181, 180);s.b[1430] = (p.p140 != 0.0);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if s.b[1430] {s.copy_ad(335, 637);s.store_sub_from_scalar(336, p.p224, 364);s.store_offset(337, 336, 1e-25);s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));s.store_scaled_add(339, 337, 338, 0.5);s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);s.store_div_from_scalar(341, 1.0, 339);s.store_scale(175, 341, p.p223);s.store_mul_scale_offset_indices(342, 341, 175, -1.0, 0.0);s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));}
        if s.b[1430] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1430] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);s.store_div(343, 334, 176);s.store_mul_sub_lhs(177, 239, 176, 186);s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);s.store_add_scaled_product_mixed_aii(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 685);s.store_scalar(338, (1.0 / (p.p140 * p.p140)));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_sub(335, 173, 119);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1430] {s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p.p140)), s.v[465]);s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);s.store_offset(178, 106, p.p221);s.store_square(179, 178);s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);}
        if (!s.b[1430]) {s.store_scalar(174, 0.0);}
        s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[582] - p.p139));s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));s.store_mul_scale_offset_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338], 0.0);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_scale_offset_mixed_ia(341, 181, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 0.0);s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);s.store_scalar(335, (s.v[470] / s.v[582]));s.store_offset_scaled(338, 180, s.v[335], s.v[468]);s.store_add_scaled_product_mixed_iia(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p.p150, 1.0), s.v[469]);s.store_mul(122, 121, 339);s.store_div_from_scalar(335, 1.0, 185);s.store_square(336, 335);s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));s.store_square(338, 337);s.store_sub(339, 335, 337);s.store_mul_sub_rhs(340, 239, 336, 338);s.store_offset_mul(124, 239, 339, (s.v[478] / s.v[580]));s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);let t6: f64 = (s.v[157] + s.v[160]);let t7: f64 = (s.v[336] * s.v[189]);let t8: f64 = (t6 + t7);let t9: f64 = (t8 - s.v[120]);s.store_scalar(118, t9);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_scalar(182, 0.0);s.b[1431] = (s.v[615] == 1.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if s.b[1431] {s.copy_ad(341, 107);s.copy_ad(334, 642);s.store_offset(337, 341, (-p.p152));}
        s.b[1432] = (s.v[337] < (-3.0));s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1432]) {s.store_scalar(340, 0.0);s.store_scalar(182, 0.0);}
        s.b[1433] = (s.v[337] < 0.0);s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if ((s.b[1431] && (!s.b[1432])) && s.b[1433]) {s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if ((s.b[1431] && (!s.b[1432])) && (!s.b[1433])) {s.store_offset_mul_offset_rhs_mixed_ia(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if s.b[1431] {s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);}
    }
}
