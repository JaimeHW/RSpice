#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[28] = (if (self.params.factuoedge > 0.0) { self.params.factuoedge } else { 0.0 });

        scratch.values[29] = self.params.delvtoedge;

        scratch.values[1297] = if (scratch.values[6] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1297] != 0.0) {
            scratch.values[210] = scratch.values[209];
            scratch.node_derivatives[210] = scratch.node_derivatives[209];
            scratch.branch_derivatives[210] = scratch.branch_derivatives[209];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[212] = scratch.values[211];
            scratch.node_derivatives[212] = scratch.node_derivatives[211];
            scratch.branch_derivatives[212] = scratch.branch_derivatives[211];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[259] = scratch.values[258];
            scratch.node_derivatives[259] = scratch.node_derivatives[258];
            scratch.branch_derivatives[259] = scratch.branch_derivatives[258];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[261] = scratch.values[260];
            scratch.node_derivatives[261] = scratch.node_derivatives[260];
            scratch.branch_derivatives[261] = scratch.branch_derivatives[260];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[263] = scratch.values[262];
            scratch.node_derivatives[263] = scratch.node_derivatives[262];
            scratch.branch_derivatives[263] = scratch.branch_derivatives[262];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[265] = scratch.values[264];
            scratch.node_derivatives[265] = scratch.node_derivatives[264];
            scratch.branch_derivatives[265] = scratch.branch_derivatives[264];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[253] = scratch.values[252];
            scratch.node_derivatives[253] = scratch.node_derivatives[252];
            scratch.branch_derivatives[253] = scratch.branch_derivatives[252];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[268] = scratch.values[267];
            scratch.node_derivatives[268] = scratch.node_derivatives[267];
            scratch.branch_derivatives[268] = scratch.branch_derivatives[267];
        }

        if (scratch.values[1297] != 0.0) {
            scratch.values[271] = scratch.values[270];
            scratch.node_derivatives[271] = scratch.node_derivatives[270];
            scratch.branch_derivatives[271] = scratch.branch_derivatives[270];
        }

        scratch.store_ad(811, &AdValue::scale(scratch.ad_value(194), 8.8541878176e-12));

        scratch.store_ad(812, &AdValue::div(scratch.ad_value(811), scratch.ad_value(193)));

        scratch.store_ad(813, &AdValue::square(scratch.ad_value(193)));

        scratch.store_ad(814, &AdValue::scale(scratch.ad_value(812), 6.241449993689894e18));

        scratch.store_ad(815, &AdValue::mul(scratch.ad_value(196), scratch.ad_value(195)));

        if (scratch.values[815] > 1e20) {
            scratch.store_ad(815, &{
                if (scratch.values[815] < 1e26) {
                    scratch.ad_value(815)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[815] = 1e20;
            scratch.node_derivatives[815] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[815] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[816] = 0.0;

        scratch.values[1298] = if (scratch.values[188] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1298] != 0.0) {
            scratch.store_ad(816, &AdValue::scale(AdValue::powf(scratch.ad_value(812), 0.6666666666666666), ((0.4 * 5.951993) * scratch.values[188])));
        }

        scratch.values[1299] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[1298] != 0.0) && (scratch.values[1299] != 0.0)) {
            scratch.store_ad(816, &AdValue::scale(scratch.ad_value(816), (7.448711 / 5.951993)));
        }

        scratch.store_ad(817, &AdValue::scale(scratch.ad_value(812), (1e-8 * 1.0 / (scratch.values[810]))));

        scratch.store_ad(818, &AdValue::scale(scratch.ad_value(231), 0.5));

        scratch.values[819] = 0.5;

        scratch.values[1300] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1300] != 0.0) {
            scratch.store_ad(818, &AdValue::scale(scratch.ad_value(231), 0.3333333333333333));
        }

        if (scratch.values[1300] != 0.0) {
            scratch.values[819] = 0.3333333333333333;
            scratch.node_derivatives[819] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[819] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(820, &AdValue::div_from_scalar(1.0, scratch.ad_value(240)));

        scratch.store_ad(821, &AdValue::div_from_scalar(1.0, scratch.ad_value(244)));

        scratch.store_ad(822, &AdValue::div(scratch.ad_value(811), scratch.ad_value(209)));

        scratch.store_ad(823, &AdValue::div(scratch.ad_value(811), scratch.ad_value(210)));

        scratch.store_ad(824, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(211), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[357])))), scratch.ad_value(822)));

        scratch.store_ad(825, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(212), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[357])))), scratch.ad_value(823)));

        scratch.store_ad(826, &AdValue::square(scratch.ad_value(824)));

        scratch.store_ad(827, &AdValue::square(scratch.ad_value(825)));

        scratch.store_ad(962, &AdValue::div_from_scalar(1.0, scratch.ad_value(824)));

        scratch.store_ad(963, &AdValue::offset(AdValue::scale(scratch.ad_value(824), 3.1), 8.5));

        scratch.store_ad(828, &AdValue::square(scratch.ad_value(963)));

        scratch.store_ad(964, &AdValue::scale(scratch.ad_value(963), 0.5));

        scratch.values[1301] = if (scratch.values[962] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1301] != 0.0) {
            scratch.store_ad(829, &AdValue::scale(scratch.ad_value(962), 64.0));
        }

        scratch.values[1302] = if (scratch.values[962] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1301] != 0.0)) && (scratch.values[1302] != 0.0)) {
            scratch.store_ad(829, &AdValue::offset(AdValue::scale(scratch.ad_value(962), 22.0), 3.0));
        }

        scratch.values[1303] = if (scratch.values[962] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1301] != 0.0)) && (!(scratch.values[1302] != 0.0))) && (scratch.values[1303] != 0.0)) {
            scratch.store_ad(829, &AdValue::offset(AdValue::scale(scratch.ad_value(962), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1301] != 0.0)) && (!(scratch.values[1302] != 0.0))) && (!(scratch.values[1303] != 0.0))) {
            scratch.values[829] = scratch.values[824];
            scratch.node_derivatives[829] = scratch.node_derivatives[824];
            scratch.branch_derivatives[829] = scratch.branch_derivatives[824];
        }

        scratch.store_ad(830, &AdValue::sub(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(826), 0.5)), AdValue::mul(scratch.ad_value(824), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(826), 0.25)), scratch.ad_value(829))))));

        scratch.store_ad(962, &AdValue::div_from_scalar(1.0, scratch.ad_value(825)));

        scratch.store_ad(963, &AdValue::offset(AdValue::scale(scratch.ad_value(825), 3.1), 8.5));

        scratch.store_ad(831, &AdValue::square(scratch.ad_value(963)));

        scratch.store_ad(964, &AdValue::scale(scratch.ad_value(963), 0.5));

        scratch.values[1304] = if (scratch.values[962] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1304] != 0.0) {
            scratch.store_ad(832, &AdValue::scale(scratch.ad_value(962), 64.0));
        }

        scratch.values[1305] = if (scratch.values[962] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1304] != 0.0)) && (scratch.values[1305] != 0.0)) {
            scratch.store_ad(832, &AdValue::offset(AdValue::scale(scratch.ad_value(962), 22.0), 3.0));
        }

        scratch.values[1306] = if (scratch.values[962] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1304] != 0.0)) && (!(scratch.values[1305] != 0.0))) && (scratch.values[1306] != 0.0)) {
            scratch.store_ad(832, &AdValue::offset(AdValue::scale(scratch.ad_value(962), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1304] != 0.0)) && (!(scratch.values[1305] != 0.0))) && (!(scratch.values[1306] != 0.0))) {
            scratch.values[832] = scratch.values[825];
            scratch.node_derivatives[832] = scratch.node_derivatives[825];
            scratch.branch_derivatives[832] = scratch.branch_derivatives[825];
        }

        scratch.store_ad(833, &AdValue::sub(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(827), 0.5)), AdValue::mul(scratch.ad_value(825), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(964), AdValue::scale(scratch.ad_value(827), 0.25)), scratch.ad_value(832))))));

        scratch.store_ad(771, &AdValue::add(AdValue::offset(scratch.ad_value(203), scratch.values[364]), AdValue::scale(AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(195), AdValue::powf(scratch.ad_value(365), (-0.75))), 4e-26)), (2.0 * scratch.values[759]))));

        if !(scratch.values[771] > 0.05) {
            scratch.values[771] = 0.05;
            scratch.node_derivatives[771] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[771] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(772, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(195), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[363])))), scratch.ad_value(812)));

        scratch.values[773] = 0.0;

        scratch.values[774] = 0.0;

        scratch.values[1307] = if (scratch.values[205] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(775, &AdValue::div_from_scalar(80000000.0, scratch.ad_value(813)));
        }

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(774, &{
                if (scratch.values[205] > scratch.values[775]) {
                    scratch.ad_value(205)
                } else {
                    scratch.ad_value(775)
                }
            });
        }

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(774, &{
                if (5e24 > scratch.values[774]) {
                    AdValue::constant(5e24)
                } else {
                    scratch.ad_value(774)
                }
            });
        }

        if (scratch.values[1307] != 0.0) {
            scratch.store_ad(773, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(812), 2.0), scratch.ad_value(812)), scratch.values[759]), AdValue::scale(scratch.ad_value(774), (1.6021918e-19 * scratch.values[810]))));
        }

        scratch.values[776] = ((100.0 * scratch.values[759]) * scratch.values[759]);

        scratch.values[1308] = if (scratch.values[188] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(777, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(772), scratch.values[759]), scratch.ad_value(772)), scratch.ad_value(771))));
        }

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(778, &AdValue::mul(AdValue::scale(scratch.ad_value(816), 0.75), AdValue::powf(scratch.ad_value(777), 0.6666666666666666)));
        }

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(771, &AdValue::add(scratch.ad_value(771), scratch.ad_value(778)));
        }

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(772, &AdValue::mul(scratch.ad_value(772), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(778), (2.0 * 0.6666666666666666)), scratch.ad_value(777)), 1.0)));
        }

        scratch.store_ad(779, &AdValue::sqrt(scratch.ad_value(771)));

        scratch.store_ad(780, &AdValue::scale(scratch.ad_value(771), 0.95));

        scratch.store_ad(781, &AdValue::mul(AdValue::scale(scratch.ad_value(771), 0.0025), scratch.ad_value(771)));

        scratch.values[782] = scratch.values[781];
        scratch.node_derivatives[782] = scratch.node_derivatives[781];
        scratch.branch_derivatives[782] = scratch.branch_derivatives[781];

        scratch.store_ad(783, &AdValue::scale(AdValue::sqrt(scratch.ad_value(782)), 0.5));

        scratch.store_ad(784, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(780), scratch.ad_value(783)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(780), scratch.ad_value(783)), AdValue::sub(scratch.ad_value(780), scratch.ad_value(783))), scratch.ad_value(781)))), 0.5));

        scratch.store_ad(785, &AdValue::scale(AdValue::offset(scratch.ad_value(771), scratch.values[364]), 0.5));

        scratch.store_ad(786, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(198), scratch.ad_value(771))), scratch.ad_value(779)));

        scratch.store_ad(787, &AdValue::sub(AdValue::sub(AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(198), scratch.ad_value(199)), scratch.ad_value(771))), scratch.ad_value(779)), scratch.ad_value(786)));

        scratch.store_ad(788, &AdValue::add(AdValue::add(AdValue::offset(scratch.ad_value(203), scratch.values[364]), scratch.ad_value(204)), AdValue::scale(AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(815), AdValue::powf(scratch.ad_value(365), (-0.75))), 4e-26)), (2.0 * scratch.values[759]))));

        if !(scratch.values[788] > 0.05) {
            scratch.values[788] = 0.05;
            scratch.node_derivatives[788] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[788] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(789, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(815), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[363])))), scratch.ad_value(812)));

        scratch.values[1309] = if (scratch.values[188] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(777, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(789), scratch.values[759]), scratch.ad_value(789)), scratch.ad_value(788))));
        }

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(778, &AdValue::mul(AdValue::scale(scratch.ad_value(816), 0.75), AdValue::powf(scratch.ad_value(777), 0.6666666666666666)));
        }

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(788, &AdValue::add(scratch.ad_value(788), scratch.ad_value(778)));
        }

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(789, &AdValue::mul(scratch.ad_value(789), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(778), (2.0 * 0.6666666666666666)), scratch.ad_value(777)), 1.0)));
        }

        scratch.store_ad(790, &AdValue::scale(scratch.ad_value(788), 0.95));

        scratch.store_ad(791, &AdValue::mul(AdValue::scale(scratch.ad_value(788), 0.0025), scratch.ad_value(788)));

        scratch.values[792] = scratch.values[791];
        scratch.node_derivatives[792] = scratch.node_derivatives[791];
        scratch.branch_derivatives[792] = scratch.branch_derivatives[791];

        scratch.store_ad(783, &AdValue::scale(AdValue::sqrt(scratch.ad_value(792)), 0.5));

        scratch.store_ad(793, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(790), scratch.ad_value(783)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(790), scratch.ad_value(783)), AdValue::sub(scratch.ad_value(790), scratch.ad_value(783))), scratch.ad_value(791)))), 0.5));

        scratch.store_ad(744, &AdValue::offset(AdValue::add(scratch.ad_value(189), AdValue::mul(AdValue::scale(scratch.ad_value(190), scratch.values[360]), AdValue::offset(AdValue::scale(scratch.ad_value(191), scratch.values[360]), 1.0))), scratch.values[27]));

        scratch.store_ad(794, &AdValue::exp(AdValue::scale(scratch.ad_value(192), scratch.values[362])));

        scratch.store_ad(745, &AdValue::mul(scratch.ad_value(206), scratch.ad_value(794)));

        scratch.store_ad(746, &AdValue::scale(scratch.ad_value(207), 1.0 / (scratch.values[361])));

        scratch.store_ad(795, &AdValue::exp(AdValue::scale(scratch.ad_value(220), scratch.values[362])));

        scratch.store_ad(747, &AdValue::mul(scratch.ad_value(219), scratch.ad_value(795)));

        scratch.store_ad(760, &AdValue::mul(AdValue::scale(scratch.ad_value(747), scratch.values[26]), scratch.ad_value(812)));

        scratch.store_ad(749, &AdValue::mul(scratch.ad_value(223), AdValue::exp(AdValue::scale(scratch.ad_value(224), scratch.values[362]))));

        scratch.store_ad(796, &AdValue::exp(AdValue::scale(scratch.ad_value(222), scratch.values[362])));

        scratch.store_ad(748, &AdValue::mul(scratch.ad_value(221), scratch.ad_value(796)));

        scratch.store_ad(751, &AdValue::mul(scratch.ad_value(227), AdValue::exp(AdValue::scale(scratch.ad_value(228), scratch.values[362]))));

        scratch.store_ad(797, &AdValue::exp(AdValue::scale(scratch.ad_value(226), scratch.values[362])));

        scratch.store_ad(750, &AdValue::mul(scratch.ad_value(225), scratch.ad_value(797)));

        scratch.store_ad(798, &AdValue::exp(AdValue::scale(scratch.ad_value(230), scratch.values[362])));

        scratch.store_ad(752, &AdValue::mul(scratch.ad_value(229), scratch.ad_value(798)));

        scratch.store_ad(799, &AdValue::exp(AdValue::scale(scratch.ad_value(233), scratch.values[362])));

        scratch.store_ad(753, &AdValue::mul(scratch.ad_value(232), scratch.ad_value(799)));

        scratch.store_ad(800, &AdValue::mul(AdValue::scale(scratch.ad_value(760), 2.0), scratch.ad_value(753)));

        scratch.store_ad(801, &AdValue::exp(AdValue::scale(scratch.ad_value(237), scratch.values[362])));

        scratch.store_ad(764, &AdValue::mul(scratch.ad_value(236), scratch.ad_value(801)));

        scratch.store_ad(756, &AdValue::mul(scratch.ad_value(246), AdValue::exp(AdValue::scale(AdValue::neg(scratch.ad_value(247)), scratch.values[362]))));

        scratch.store_ad(763, &AdValue::scale(scratch.ad_value(272), (4.0 * (1.3806505e-23 * scratch.values[358]))));

        scratch.store_ad(765, &AdValue::div(AdValue::scale(scratch.ad_value(760), (scratch.values[759] * scratch.values[759])), scratch.ad_value(814)));

        scratch.values[1310] = if ((scratch.values[8] != 0.0) && (scratch.values[283] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(757, &AdValue::offset(AdValue::add(scratch.ad_value(278), AdValue::scale(scratch.ad_value(279), scratch.values[360])), scratch.values[29]));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(802, &AdValue::exp(AdValue::scale(scratch.ad_value(284), scratch.values[362])));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(758, &AdValue::mul(scratch.ad_value(283), scratch.ad_value(802)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(761, &AdValue::mul(AdValue::scale(scratch.ad_value(758), scratch.values[28]), scratch.ad_value(812)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(766, &AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(282), scratch.values[361]), 1.0), scratch.values[759]));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(803, &AdValue::add(AdValue::offset(scratch.ad_value(280), scratch.values[364]), AdValue::mul(AdValue::scale(scratch.ad_value(766), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(281), AdValue::powf(scratch.ad_value(365), (-0.75))), 4e-26)))));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(803, &{
                if (scratch.values[803] > 0.05) {
                    scratch.ad_value(803)
                } else {
                    AdValue::constant(0.05)
                }
            });
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(804, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(281), ((2.0 * 1.6021918e-19) * (scratch.values[810] * scratch.values[363])))), scratch.ad_value(812)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(767, &AdValue::square(scratch.ad_value(804)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(768, &AdValue::ln(scratch.ad_value(767)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(805, &AdValue::scale(scratch.ad_value(803), 0.95));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(806, &AdValue::mul(AdValue::scale(scratch.ad_value(803), 0.0025), scratch.ad_value(803)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.values[807] = scratch.values[806];
            scratch.node_derivatives[807] = scratch.node_derivatives[806];
            scratch.branch_derivatives[807] = scratch.branch_derivatives[806];
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(808, &AdValue::scale(AdValue::sqrt(scratch.ad_value(807)), 0.5));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(809, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(805), scratch.ad_value(808)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(805), scratch.ad_value(808)), AdValue::sub(scratch.ad_value(805), scratch.ad_value(808))), scratch.ad_value(806)))), 0.5));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(769, &AdValue::div(AdValue::scale(scratch.ad_value(761), (scratch.values[759] * scratch.values[759])), scratch.ad_value(814)));
        }

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(770, &AdValue::scale(scratch.ad_value(291), (4.0 * (1.3806505e-23 * scratch.values[358]))));
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[757] = 0.0;
            scratch.node_derivatives[757] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[757] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[802] = 1.0;
            scratch.node_derivatives[802] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[802] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[758] = 0.0;
            scratch.node_derivatives[758] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[758] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[761] = 0.0;
            scratch.node_derivatives[761] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[761] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[766] = scratch.values[759];
            scratch.node_derivatives[766] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[766] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[803] = 0.0;
            scratch.node_derivatives[803] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[803] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[804] = 1.0;
            scratch.node_derivatives[804] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[804] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[767] = 1.0;
            scratch.node_derivatives[767] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[767] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[768] = 0.0;
            scratch.node_derivatives[768] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[768] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[805] = 0.0;
            scratch.node_derivatives[805] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[805] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[806] = 0.0;
            scratch.node_derivatives[806] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[806] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[807] = 0.0;
            scratch.node_derivatives[807] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[807] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[808] = 0.0;
            scratch.node_derivatives[808] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[808] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[809] = 0.0;
            scratch.node_derivatives[809] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[809] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[769] = 0.0;
            scratch.node_derivatives[769] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[769] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[770] = 1.0;
            scratch.node_derivatives[770] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[770] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(834, &AdValue::div_from_scalar(1.0, scratch.ad_value(257)));

        scratch.store_ad(835, &AdValue::scale(AdValue::sqrt(AdValue::scale(scratch.ad_value(257), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33)));

        scratch.store_ad(836, &AdValue::mul(scratch.ad_value(835), scratch.ad_value(193)));

        scratch.store_ad(837, &AdValue::mul(scratch.ad_value(835), scratch.ad_value(209)));

        scratch.store_ad(838, &AdValue::mul(scratch.ad_value(835), scratch.ad_value(210)));

        scratch.values[839] = 0.0;

        scratch.values[1311] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1311] != 0.0) {
            scratch.store_ad(839, &AdValue::div(AdValue::scale(scratch.ad_value(255), (-0.495)), scratch.ad_value(256)));
        }

        scratch.store_ad(840, &AdValue::pow_from_scalar(scratch.values[354], scratch.ad_value(254)));

        scratch.store_ad(251, &AdValue::mul(scratch.ad_value(251), scratch.ad_value(840)));

        scratch.store_ad(252, &AdValue::mul(scratch.ad_value(252), scratch.ad_value(840)));

        scratch.store_ad(253, &AdValue::mul(scratch.ad_value(253), scratch.ad_value(840)));

        scratch.store_ad(841, &AdValue::div(AdValue::scale(scratch.ad_value(258), 4e-18), AdValue::square(scratch.ad_value(209))));

        scratch.store_ad(842, &AdValue::div(AdValue::scale(scratch.ad_value(259), 4e-18), AdValue::square(scratch.ad_value(210))));

        if ((1.0 + (scratch.values[262] * scratch.values[355])) > 0.0) {
            scratch.store_ad(835, &AdValue::offset(AdValue::scale(scratch.ad_value(262), scratch.values[355]), 1.0));
        } else {
            scratch.values[835] = 0.0;
            scratch.node_derivatives[835] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[835] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(754, &AdValue::mul(scratch.ad_value(260), scratch.ad_value(835)));

        scratch.store_ad(843, &AdValue::scale(AdValue::mul(scratch.ad_value(754), scratch.ad_value(209)), 500000000.0));

        if ((1.0 + (scratch.values[263] * scratch.values[355])) > 0.0) {
            scratch.store_ad(835, &AdValue::offset(AdValue::scale(scratch.ad_value(263), scratch.values[355]), 1.0));
        } else {
            scratch.values[835] = 0.0;
            scratch.node_derivatives[835] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[835] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(755, &AdValue::mul(scratch.ad_value(261), scratch.ad_value(835)));

        scratch.store_ad(844, &AdValue::scale(AdValue::mul(scratch.ad_value(755), scratch.ad_value(210)), 500000000.0));

        scratch.store_ad(845, &AdValue::scale(scratch.ad_value(273), (9.1093826e-31 * 1000000000.0)));

        scratch.values[1312] = if (scratch.values[296] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1312] != 0.0) {
            scratch.store_ad(846, &AdValue::div_from_scalar(1.0, scratch.ad_value(296)));
        }

        if (!(scratch.values[1312] != 0.0)) {
            scratch.values[846] = 0.0;
            scratch.node_derivatives[846] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[846] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1313] = if (scratch.values[297] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1313] != 0.0) {
            scratch.store_ad(847, &AdValue::div_from_scalar(1.0, scratch.ad_value(297)));
        }

        if (!(scratch.values[1313] != 0.0)) {
            scratch.values[847] = 0.0;
            scratch.node_derivatives[847] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[847] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1314] = if (scratch.values[298] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1314] != 0.0) {
            scratch.store_ad(848, &AdValue::div_from_scalar(1.0, scratch.ad_value(298)));
        }

        if (!(scratch.values[1314] != 0.0)) {
            scratch.values[848] = 0.0;
            scratch.node_derivatives[848] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[848] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1315] = if (scratch.values[299] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1315] != 0.0) {
            scratch.store_ad(849, &AdValue::div_from_scalar(1.0, scratch.ad_value(299)));
        }

        if (!(scratch.values[1315] != 0.0)) {
            scratch.values[849] = 0.0;
            scratch.node_derivatives[849] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[849] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1316] = if (scratch.values[300] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1316] != 0.0) {
            scratch.store_ad(850, &AdValue::div_from_scalar(1.0, scratch.ad_value(300)));
        }

        if (!(scratch.values[1316] != 0.0)) {
            scratch.values[850] = 0.0;
            scratch.node_derivatives[850] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[850] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1317] = if (scratch.values[301] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1317] != 0.0) {
            scratch.store_ad(851, &AdValue::div_from_scalar(1.0, scratch.ad_value(301)));
        }

        if (!(scratch.values[1317] != 0.0)) {
            scratch.values[851] = 0.0;
            scratch.node_derivatives[851] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[851] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1318] = if (scratch.values[302] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(852, &AdValue::div_from_scalar(1.0, scratch.ad_value(302)));
        }

        if (!(scratch.values[1318] != 0.0)) {
            scratch.values[852] = 0.0;
            scratch.node_derivatives[852] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[852] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[30] = 0.0;

        scratch.values[31] = 0.0;

        scratch.values[32] = 0.0;

        scratch.values[33] = 0.0;

        scratch.values[34] = 0.0;

        scratch.values[35] = 0.0;

        scratch.values[36] = 0.0;

        scratch.values[37] = scratch.values[308];
        scratch.node_derivatives[37] = scratch.node_derivatives[308];
        scratch.branch_derivatives[37] = scratch.branch_derivatives[308];

        scratch.values[1319] = if (scratch.values[1] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1319] != 0.0) {
            scratch.values[37] = (if (scratch.values[20] > 0.0) { scratch.values[20] } else { 0.0 });
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1320] = if (scratch.values[5] == 3.0) { 1.0 } else { 0.0 };

        if (scratch.values[1320] != 0.0) {
            scratch.values[36] = 1.0;
            scratch.node_derivatives[36] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[36] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[690]));

        scratch.store_ad(31, &AdValue::scale(scratch.ad_value(12), scratch.values[691]));

        scratch.store_ad(32, &AdValue::scale(scratch.ad_value(12), scratch.values[692]));

        scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[717]));

        scratch.store_ad(34, &AdValue::scale(scratch.ad_value(12), scratch.values[718]));

        scratch.store_ad(35, &AdValue::scale(scratch.ad_value(12), scratch.values[719]));

        scratch.values[1321] = if ((scratch.values[5] == 2.0) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[693]));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(31, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[694]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.values[32] = scratch.values[37];
            scratch.node_derivatives[32] = scratch.node_derivatives[37];
            scratch.branch_derivatives[32] = scratch.branch_derivatives[37];
        }

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[720]));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(34, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[721]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.values[35] = scratch.values[37];
            scratch.node_derivatives[35] = scratch.node_derivatives[37];
            scratch.branch_derivatives[35] = scratch.branch_derivatives[37];
        }

        scratch.values[1322] = if (((scratch.values[5] == 1.0) || (scratch.values[5] == 2.0)) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(690, &{
                if (scratch.values[30] > 0.0) {
                    scratch.ad_value(30)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(691, &{
                if (scratch.values[31] > 0.0) {
                    scratch.ad_value(31)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(692, &{
                if (scratch.values[32] > 0.0) {
                    scratch.ad_value(32)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(717, &{
                if (scratch.values[33] > 0.0) {
                    scratch.ad_value(33)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(718, &{
                if (scratch.values[34] > 0.0) {
                    scratch.ad_value(34)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(719, &{
                if (scratch.values[35] > 0.0) {
                    scratch.ad_value(35)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[690] = 0.0;
            scratch.node_derivatives[690] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[690] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[691] = 0.0;
            scratch.node_derivatives[691] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[691] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[692] = 0.0;
            scratch.node_derivatives[692] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[692] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[717] = 0.0;
            scratch.node_derivatives[717] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[717] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[718] = 0.0;
            scratch.node_derivatives[718] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[718] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[719] = 0.0;
            scratch.node_derivatives[719] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[719] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[700] = 0.0;

        scratch.values[727] = 0.0;

        scratch.values[702] = 0.0;

        scratch.values[729] = 0.0;

        scratch.values[701] = 0.0;

        scratch.values[728] = 0.0;

        scratch.values[703] = 0.0;

        scratch.values[730] = 0.0;

        scratch.values[698] = 0.0;

        scratch.values[725] = 0.0;

        scratch.values[699] = 0.0;

        scratch.values[726] = 0.0;

        scratch.values[711] = 0.0;

        scratch.values[738] = 0.0;

        scratch.values[712] = 1.0;

        scratch.values[739] = 1.0;

        scratch.values[713] = 0.0;

        scratch.values[740] = 0.0;

        scratch.values[714] = 1.0;

        scratch.values[741] = 1.0;

        scratch.values[715] = 0.0;

        scratch.values[742] = 0.0;

        scratch.values[716] = 1.0;

        scratch.values[743] = 1.0;

        scratch.values[710] = 0.0;

        scratch.values[737] = 0.0;

        scratch.values[704] = 0.0;

        scratch.values[731] = 0.0;

        scratch.values[705] = 0.0;

        scratch.values[732] = 0.0;

        scratch.values[706] = 0.0;

        scratch.values[733] = 0.0;

        scratch.values[707] = 0.0;

        scratch.values[734] = 0.0;

        scratch.values[708] = 0.0;

        scratch.values[735] = 0.0;

        scratch.values[709] = 0.0;

        scratch.values[736] = 0.0;

        scratch.values[695] = 1.0;

        scratch.values[722] = 1.0;

        scratch.values[696] = 1.0;

        scratch.values[723] = 1.0;

        scratch.values[697] = 1.0;

        scratch.values[724] = 1.0;

        scratch.values[522] = 0.0;

        scratch.values[523] = 0.0;

        scratch.values[511] = 0.0;

        scratch.values[512] = 0.0;

        scratch.values[513] = 0.0;

        scratch.values[514] = 0.0;

        scratch.values[515] = 0.0;

        scratch.values[524] = 0.0;

        scratch.values[525] = 0.0;

        scratch.values[526] = 0.0;

        scratch.values[532] = 0.0;

        scratch.values[521] = 0.0;

        scratch.values[1323] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[1324] = if ((scratch.values[436] * scratch.values[690]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1324] != 0.0)) {
            scratch.store_ad(503, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::scale(scratch.ad_value(690), scratch.values[436])), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1324] != 0.0))) {
            scratch.values[503] = 100000000.0;
            scratch.node_derivatives[503] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[503] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1325] = if ((scratch.values[437] * scratch.values[691]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1325] != 0.0)) {
            scratch.store_ad(504, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::scale(scratch.ad_value(691), scratch.values[437])), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1325] != 0.0))) {
            scratch.values[504] = 100000000.0;
            scratch.node_derivatives[504] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[504] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1326] = if ((scratch.values[438] * scratch.values[692]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1326] != 0.0)) {
            scratch.store_ad(505, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::scale(scratch.ad_value(692), scratch.values[438])), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1326] != 0.0))) {
            scratch.values[505] = 100000000.0;
            scratch.node_derivatives[505] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[505] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(698, &AdValue::min(AdValue::min(scratch.ad_value(503), scratch.ad_value(504)), scratch.ad_value(505)));
        }

        scratch.values[1327] = if ((((scratch.values[698] * scratch.values[420])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1327] != 0.0)) {
            scratch.store_ad(699, &AdValue::exp(AdValue::scale(scratch.ad_value(698), scratch.values[420])));
        }

        scratch.values[1328] = if ((scratch.values[698] * scratch.values[420]) < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1327] != 0.0))) && (scratch.values[1328] != 0.0)) {
            scratch.store_ad(699, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(698), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(698), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(698), scratch.values[420])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1327] != 0.0))) && (!(scratch.values[1328] != 0.0))) {
            scratch.store_ad(699, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(698), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(698), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(698), scratch.values[420]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[445] = scratch.values[442];
            scratch.node_derivatives[445] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[445] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[446] = scratch.values[443];
            scratch.node_derivatives[446] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[446] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[447] = scratch.values[444];
            scratch.node_derivatives[447] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[447] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[448] = scratch.values[375];
            scratch.node_derivatives[448] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[448] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[449] = scratch.values[376];
            scratch.node_derivatives[449] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[449] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[450] = scratch.values[377];
            scratch.node_derivatives[450] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[450] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[451] = scratch.values[372];
            scratch.node_derivatives[451] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[451] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[452] = scratch.values[373];
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[453] = scratch.values[374];
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1329] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.values[445] = (scratch.values[443] + scratch.values[444]);
            scratch.node_derivatives[445] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[445] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.values[448] = (0.9 * (scratch.values[376]).min(scratch.values[377]));
            scratch.node_derivatives[448] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[448] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.values[451] = (scratch.values[373] + scratch.values[374]);
            scratch.node_derivatives[451] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[451] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1330] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.values[446] = (scratch.values[442] + scratch.values[444]);
            scratch.node_derivatives[446] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[446] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.values[449] = (0.9 * (scratch.values[375]).min(scratch.values[377]));
            scratch.node_derivatives[449] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[449] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.values[452] = (scratch.values[372] + scratch.values[374]);
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1331] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.values[447] = (scratch.values[442] + scratch.values[443]);
            scratch.node_derivatives[447] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[447] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.values[450] = (0.9 * (scratch.values[375]).min(scratch.values[376]));
            scratch.node_derivatives[450] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[450] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.values[453] = (scratch.values[372] + scratch.values[373]);
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(700, &AdValue::min(AdValue::min(scratch.ad_value(445), scratch.ad_value(446)), scratch.ad_value(447)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(701, &AdValue::scale(scratch.ad_value(700), 0.1));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(426, &AdValue::max(AdValue::max(scratch.ad_value(448), scratch.ad_value(449)), scratch.ad_value(450)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(702, &AdValue::mul(scratch.ad_value(700), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(426))))));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(703, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(451), scratch.ad_value(452)), scratch.ad_value(453)), (-0.05)));
        }

        scratch.values[1332] = if ((scratch.values[588] * scratch.values[717]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1332] != 0.0)) {
            scratch.store_ad(503, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::mul(scratch.ad_value(588), scratch.ad_value(717))), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1332] != 0.0))) {
            scratch.values[503] = 100000000.0;
            scratch.node_derivatives[503] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[503] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1333] = if ((scratch.values[589] * scratch.values[718]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1333] != 0.0)) {
            scratch.store_ad(504, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::mul(scratch.ad_value(589), scratch.ad_value(718))), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1333] != 0.0))) {
            scratch.values[504] = 100000000.0;
            scratch.node_derivatives[504] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[504] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1334] = if ((scratch.values[590] * scratch.values[719]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1334] != 0.0)) {
            scratch.store_ad(505, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::mul(scratch.ad_value(590), scratch.ad_value(719))), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1334] != 0.0))) {
            scratch.values[505] = 100000000.0;
            scratch.node_derivatives[505] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[505] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(725, &AdValue::min(AdValue::min(scratch.ad_value(503), scratch.ad_value(504)), scratch.ad_value(505)));
        }

        scratch.values[1335] = if ((((scratch.values[725] * scratch.values[420])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1335] != 0.0)) {
            scratch.store_ad(726, &AdValue::exp(AdValue::scale(scratch.ad_value(725), scratch.values[420])));
        }

        scratch.values[1336] = if ((scratch.values[725] * scratch.values[420]) < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1335] != 0.0))) && (scratch.values[1336] != 0.0)) {
            scratch.store_ad(726, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(725), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(725), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(725), scratch.values[420])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1335] != 0.0))) && (!(scratch.values[1336] != 0.0))) {
            scratch.store_ad(726, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(725), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(725), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(725), scratch.values[420]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[445] = scratch.values[594];
            scratch.node_derivatives[445] = scratch.node_derivatives[594];
            scratch.branch_derivatives[445] = scratch.branch_derivatives[594];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[446] = scratch.values[595];
            scratch.node_derivatives[446] = scratch.node_derivatives[595];
            scratch.branch_derivatives[446] = scratch.branch_derivatives[595];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[447] = scratch.values[596];
            scratch.node_derivatives[447] = scratch.node_derivatives[596];
            scratch.branch_derivatives[447] = scratch.branch_derivatives[596];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[448] = scratch.values[542];
            scratch.node_derivatives[448] = scratch.node_derivatives[542];
            scratch.branch_derivatives[448] = scratch.branch_derivatives[542];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[449] = scratch.values[543];
            scratch.node_derivatives[449] = scratch.node_derivatives[543];
            scratch.branch_derivatives[449] = scratch.branch_derivatives[543];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[450] = scratch.values[544];
            scratch.node_derivatives[450] = scratch.node_derivatives[544];
            scratch.branch_derivatives[450] = scratch.branch_derivatives[544];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[451] = scratch.values[539];
            scratch.node_derivatives[451] = scratch.node_derivatives[539];
            scratch.branch_derivatives[451] = scratch.branch_derivatives[539];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[452] = scratch.values[540];
            scratch.node_derivatives[452] = scratch.node_derivatives[540];
            scratch.branch_derivatives[452] = scratch.branch_derivatives[540];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[453] = scratch.values[541];
            scratch.node_derivatives[453] = scratch.node_derivatives[541];
            scratch.branch_derivatives[453] = scratch.branch_derivatives[541];
        }

        scratch.values[1337] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(445, &AdValue::add(scratch.ad_value(595), scratch.ad_value(596)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(448, &AdValue::scale(AdValue::min(scratch.ad_value(543), scratch.ad_value(544)), 0.9));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(451, &AdValue::add(scratch.ad_value(540), scratch.ad_value(541)));
        }

        scratch.values[1338] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1338] != 0.0)) {
            scratch.store_ad(446, &AdValue::add(scratch.ad_value(594), scratch.ad_value(596)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1338] != 0.0)) {
            scratch.store_ad(449, &AdValue::scale(AdValue::min(scratch.ad_value(542), scratch.ad_value(544)), 0.9));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1338] != 0.0)) {
            scratch.store_ad(452, &AdValue::add(scratch.ad_value(539), scratch.ad_value(541)));
        }

        scratch.values[1339] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1339] != 0.0)) {
            scratch.store_ad(447, &AdValue::add(scratch.ad_value(594), scratch.ad_value(595)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1339] != 0.0)) {
            scratch.store_ad(450, &AdValue::scale(AdValue::min(scratch.ad_value(542), scratch.ad_value(543)), 0.9));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1339] != 0.0)) {
            scratch.store_ad(453, &AdValue::add(scratch.ad_value(539), scratch.ad_value(540)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(727, &AdValue::min(AdValue::min(scratch.ad_value(445), scratch.ad_value(446)), scratch.ad_value(447)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(728, &AdValue::scale(scratch.ad_value(727), 0.1));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(426, &AdValue::max(AdValue::max(scratch.ad_value(448), scratch.ad_value(449)), scratch.ad_value(450)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(729, &AdValue::mul(scratch.ad_value(727), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(426))))));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(730, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(451), scratch.ad_value(452)), scratch.ad_value(453)), (-0.05)));
        }

        scratch.values[1340] = if (scratch.values[410] == 1.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1341] = 0.0;
            scratch.node_derivatives[1341] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1341] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1342] = 0.0;
            scratch.node_derivatives[1342] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1342] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1343] = 0.0;
            scratch.node_derivatives[1343] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1343] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1344] = 0.0;
            scratch.node_derivatives[1344] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1344] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1345] = 0.0;
            scratch.node_derivatives[1345] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1345] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1346] = 0.0;
            scratch.node_derivatives[1346] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1346] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1347] = 0.0;
            scratch.node_derivatives[1347] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1347] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1348] = 0.0;
            scratch.node_derivatives[1348] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1348] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1349] = 0.0;
            scratch.node_derivatives[1349] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1349] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1350] = 0.0;
            scratch.node_derivatives[1350] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1350] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1351] = 0.0;
            scratch.node_derivatives[1351] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1351] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1352] = 0.0;
            scratch.node_derivatives[1352] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1352] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1353] = 0.0;
            scratch.node_derivatives[1353] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1353] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1355] = 0.0;
            scratch.node_derivatives[1355] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1355] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1356] = 0.0;
            scratch.node_derivatives[1356] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1356] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1357] = 0.0;
            scratch.node_derivatives[1357] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1357] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1359] = 0.0;
            scratch.node_derivatives[1359] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1359] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1360] = 0.0;
            scratch.node_derivatives[1360] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1360] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1362] = 0.0;
            scratch.node_derivatives[1362] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1362] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1363] = 0.0;
            scratch.node_derivatives[1363] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1363] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1366] = 0.0;
            scratch.node_derivatives[1366] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1366] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1367] = 0.0;
            scratch.node_derivatives[1367] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1367] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1369] = 0.0;
            scratch.node_derivatives[1369] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1369] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1370] = 0.0;
            scratch.node_derivatives[1370] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1370] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1371] = 0.0;
            scratch.node_derivatives[1371] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1371] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1372] = 0.0;
            scratch.node_derivatives[1372] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1372] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1373] = 0.0;
            scratch.node_derivatives[1373] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1373] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1374] = 0.0;
            scratch.node_derivatives[1374] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1374] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1375] = 0.0;
            scratch.node_derivatives[1375] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1375] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1376] = 0.0;
            scratch.node_derivatives[1376] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1376] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1377] = 0.0;
            scratch.node_derivatives[1377] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1377] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1378] = 0.0;
            scratch.node_derivatives[1378] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1378] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1379] = 0.0;
            scratch.node_derivatives[1379] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1379] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1381] = 0.0;
            scratch.node_derivatives[1381] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1381] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1382] = 0.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[529] = 0.4;
            scratch.node_derivatives[529] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[529] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[530] = 0.65;
            scratch.node_derivatives[530] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[530] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[531] = 0.8;
            scratch.node_derivatives[531] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[531] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(516, &AdValue::scale(AdValue::neg(scratch.ad_value(529)), scratch.values[411]));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(517, &AdValue::scale(AdValue::neg(scratch.ad_value(530)), scratch.values[411]));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(518, &AdValue::scale(AdValue::neg(scratch.ad_value(531)), scratch.values[411]));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[519] = 0.1;
            scratch.node_derivatives[519] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[519] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[520] = 0.2;
            scratch.node_derivatives[520] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[520] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1357] = 0.0;
            scratch.node_derivatives[1357] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1357] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1392] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(516), AdValue::mul(scratch.ad_value(701), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(516), scratch.ad_value(702)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1393] = if (scratch.values[516] < scratch.values[698]) { 1.0 } else { 0.0 };

        scratch.values[1394] = if ((((0.5 * (scratch.values[516] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (scratch.values[1393] != 0.0)) && (scratch.values[1394] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))));
        }

        scratch.values[1395] = if ((0.5 * (scratch.values[516] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (scratch.values[1393] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (scratch.values[1395] != 0.0)) {
            let assign13940_ad_e10542: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign13940_ad_e10542);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (scratch.values[1393] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1395] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (scratch.values[1393] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (!(scratch.values[1393] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(516), scratch.ad_value(698)), scratch.values[420]), 1.0), scratch.ad_value(699)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (!(scratch.values[1393] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1396] = if (scratch.values[516] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (scratch.values[1396] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(516)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(700), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(516), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(516), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(516), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(516), scratch.ad_value(703)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(516), scratch.ad_value(703)), AdValue::sub(scratch.ad_value(516), scratch.ad_value(703))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1392] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(516), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(516), scratch.ad_value(516)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1397] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1397] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1398] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (scratch.values[1398] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1398] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[436]));
        }

        scratch.values[1399] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (scratch.values[1399] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1400] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (scratch.values[1400] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1400] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[375]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1401] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (scratch.values[1401] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1401] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[472]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[433]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[386]));
        }

        scratch.values[1402] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (scratch.values[1402] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[457]), scratch.ad_value(1362)), scratch.values[487]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[484]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1403] = if (((-scratch.values[375]) * scratch.values[460]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (scratch.values[1403] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (!(scratch.values[1403] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[375]) * scratch.values[460])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[484]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[484])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1404] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (scratch.values[1404] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (!(scratch.values[1404] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1405] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (scratch.values[1405] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (!(scratch.values[1405] != 0.0))) {
            let assign14520_ad_e11485: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign14520_ad_e11485);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1406] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (scratch.values[1406] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1407] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (!(scratch.values[1406] != 0.0))) && (scratch.values[1407] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (!(scratch.values[1406] != 0.0))) && (!(scratch.values[1407] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) && (!(scratch.values[1406] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[484]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[389]));
        }

        scratch.values[1408] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (scratch.values[1408] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1409] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) && (scratch.values[1409] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) && (!(scratch.values[1409] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[475]), scratch.ad_value(1359)), scratch.values[460]));
        }

        scratch.values[1410] = if (((((-scratch.values[490]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) && (scratch.values[1410] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))));
        }

        scratch.values[1411] = if (((-scratch.values[490]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) && (!(scratch.values[1410] != 0.0))) && (scratch.values[1411] != 0.0)) {
            let assign14710_ad_e11812: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign14710_ad_e11812));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) && (!(scratch.values[1410] != 0.0))) && (!(scratch.values[1411] != 0.0))) {
            let assign14720_ad_e11862: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign14720_ad_e11862);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1408] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(516), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[395]));
        }

        scratch.values[1412] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (scratch.values[1412] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1413] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[1414] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1413] != 0.0)) && (scratch.values[1414] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[497]), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1413] != 0.0)) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[497])), scratch.values[407]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1413] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1413] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[404])), scratch.values[500]), scratch.values[494]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1397] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1415] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1415] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1416] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

    }
}
