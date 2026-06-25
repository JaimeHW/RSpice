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
        if (scratch.values[187] > 0.0001) {
            scratch.values[306] = scratch.values[187];
            scratch.node_derivatives[306] = scratch.node_derivatives[187];
            scratch.branch_derivatives[306] = scratch.branch_derivatives[187];
        } else {
            scratch.values[306] = 0.0001;
            scratch.node_derivatives[306] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[306] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[188] > 0.0) {
            scratch.values[307] = scratch.values[188];
            scratch.node_derivatives[307] = scratch.node_derivatives[188];
            scratch.branch_derivatives[307] = scratch.branch_derivatives[188];
        } else {
            scratch.values[307] = 0.0;
            scratch.node_derivatives[307] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[307] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[308] = scratch.values[189];
        scratch.node_derivatives[308] = scratch.node_derivatives[189];
        scratch.branch_derivatives[308] = scratch.branch_derivatives[189];

        if ((self.params.mult * scratch.values[11]) > 0.0) {
            scratch.store_ad(25, &AdValue::scale(scratch.ad_value(11), self.params.mult));
        } else {
            scratch.values[25] = 0.0;
            scratch.node_derivatives[25] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[25] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[26] = (if (self.params.factuo > 0.0) { self.params.factuo } else { 0.0 });

        scratch.values[27] = self.params.delvto;

        scratch.values[28] = (if (self.params.factuoedge > 0.0) { self.params.factuoedge } else { 0.0 });

        scratch.values[29] = self.params.delvtoedge;

        scratch.values[1298] = if (scratch.values[6] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1298] != 0.0) {
            scratch.values[213] = scratch.values[212];
            scratch.node_derivatives[213] = scratch.node_derivatives[212];
            scratch.branch_derivatives[213] = scratch.branch_derivatives[212];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[215] = scratch.values[214];
            scratch.node_derivatives[215] = scratch.node_derivatives[214];
            scratch.branch_derivatives[215] = scratch.branch_derivatives[214];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[262] = scratch.values[261];
            scratch.node_derivatives[262] = scratch.node_derivatives[261];
            scratch.branch_derivatives[262] = scratch.branch_derivatives[261];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[264] = scratch.values[263];
            scratch.node_derivatives[264] = scratch.node_derivatives[263];
            scratch.branch_derivatives[264] = scratch.branch_derivatives[263];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[266] = scratch.values[265];
            scratch.node_derivatives[266] = scratch.node_derivatives[265];
            scratch.branch_derivatives[266] = scratch.branch_derivatives[265];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[268] = scratch.values[267];
            scratch.node_derivatives[268] = scratch.node_derivatives[267];
            scratch.branch_derivatives[268] = scratch.branch_derivatives[267];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[256] = scratch.values[255];
            scratch.node_derivatives[256] = scratch.node_derivatives[255];
            scratch.branch_derivatives[256] = scratch.branch_derivatives[255];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[271] = scratch.values[270];
            scratch.node_derivatives[271] = scratch.node_derivatives[270];
            scratch.branch_derivatives[271] = scratch.branch_derivatives[270];
        }

        if (scratch.values[1298] != 0.0) {
            scratch.values[274] = scratch.values[273];
            scratch.node_derivatives[274] = scratch.node_derivatives[273];
            scratch.branch_derivatives[274] = scratch.branch_derivatives[273];
        }

        scratch.store_ad(807, &AdValue::scale(scratch.ad_value(197), 8.8541878176e-12));

        scratch.store_ad(808, &AdValue::div(scratch.ad_value(807), scratch.ad_value(196)));

        scratch.store_ad(809, &AdValue::square(scratch.ad_value(196)));

        scratch.store_ad(810, &AdValue::scale(scratch.ad_value(808), 6.241449993689894e18));

        scratch.store_ad(811, &AdValue::mul(scratch.ad_value(199), scratch.ad_value(198)));

        if (scratch.values[811] > 1e20) {
            scratch.store_ad(811, &{
                if (scratch.values[811] < 1e26) {
                    scratch.ad_value(811)
                } else {
                    AdValue::constant(1e26)
                }
            });
        } else {
            scratch.values[811] = 1e20;
            scratch.node_derivatives[811] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[811] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[812] = 0.0;

        scratch.values[1299] = if (scratch.values[191] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1299] != 0.0) {
            scratch.store_ad(812, &AdValue::scale(AdValue::powf(scratch.ad_value(808), 0.6666666666666666), ((0.4 * 5.951993) * scratch.values[191])));
        }

        scratch.values[1300] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[1299] != 0.0) && (scratch.values[1300] != 0.0)) {
            scratch.store_ad(812, &AdValue::scale(scratch.ad_value(812), (7.448711 / 5.951993)));
        }

        scratch.store_ad(813, &AdValue::scale(scratch.ad_value(808), (1e-8 * 1.0 / (scratch.values[806]))));

        scratch.store_ad(814, &AdValue::scale(scratch.ad_value(234), 0.5));

        scratch.values[815] = 0.5;

        scratch.values[1301] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1301] != 0.0) {
            scratch.store_ad(814, &AdValue::scale(scratch.ad_value(234), 0.3333333333333333));
        }

        if (scratch.values[1301] != 0.0) {
            scratch.values[815] = 0.3333333333333333;
            scratch.node_derivatives[815] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[815] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(816, &AdValue::div_from_scalar(1.0, scratch.ad_value(243)));

        scratch.store_ad(817, &AdValue::div_from_scalar(1.0, scratch.ad_value(247)));

        scratch.store_ad(818, &AdValue::div(scratch.ad_value(807), scratch.ad_value(212)));

        scratch.store_ad(819, &AdValue::div(scratch.ad_value(807), scratch.ad_value(213)));

        scratch.store_ad(820, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(214), ((2.0 * 1.6021918e-19) * (scratch.values[806] * scratch.values[364])))), scratch.ad_value(818)));

        scratch.store_ad(821, &AdValue::div(AdValue::sqrt(AdValue::scale(scratch.ad_value(215), ((2.0 * 1.6021918e-19) * (scratch.values[806] * scratch.values[364])))), scratch.ad_value(819)));

        scratch.store_ad(822, &AdValue::square(scratch.ad_value(820)));

        scratch.store_ad(823, &AdValue::square(scratch.ad_value(821)));

        scratch.store_ad(958, &AdValue::div_from_scalar(1.0, scratch.ad_value(820)));

        scratch.store_ad(959, &AdValue::offset(AdValue::scale(scratch.ad_value(820), 3.1), 8.5));

        scratch.store_ad(824, &AdValue::square(scratch.ad_value(959)));

        scratch.store_ad(960, &AdValue::scale(scratch.ad_value(959), 0.5));

        scratch.values[1302] = if (scratch.values[958] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1302] != 0.0) {
            scratch.store_ad(825, &AdValue::scale(scratch.ad_value(958), 64.0));
        }

        scratch.values[1303] = if (scratch.values[958] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1302] != 0.0)) && (scratch.values[1303] != 0.0)) {
            scratch.store_ad(825, &AdValue::offset(AdValue::scale(scratch.ad_value(958), 22.0), 3.0));
        }

        scratch.values[1304] = if (scratch.values[958] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1302] != 0.0)) && (!(scratch.values[1303] != 0.0))) && (scratch.values[1304] != 0.0)) {
            scratch.store_ad(825, &AdValue::offset(AdValue::scale(scratch.ad_value(958), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1302] != 0.0)) && (!(scratch.values[1303] != 0.0))) && (!(scratch.values[1304] != 0.0))) {
            scratch.values[825] = scratch.values[820];
            scratch.node_derivatives[825] = scratch.node_derivatives[820];
            scratch.branch_derivatives[825] = scratch.branch_derivatives[820];
        }

        scratch.store_ad(826, &AdValue::sub(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(822), 0.5)), AdValue::mul(scratch.ad_value(820), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(822), 0.25)), scratch.ad_value(825))))));

        scratch.store_ad(958, &AdValue::div_from_scalar(1.0, scratch.ad_value(821)));

        scratch.store_ad(959, &AdValue::offset(AdValue::scale(scratch.ad_value(821), 3.1), 8.5));

        scratch.store_ad(827, &AdValue::square(scratch.ad_value(959)));

        scratch.store_ad(960, &AdValue::scale(scratch.ad_value(959), 0.5));

        scratch.values[1305] = if (scratch.values[958] < 0.06) { 1.0 } else { 0.0 };

        if (scratch.values[1305] != 0.0) {
            scratch.store_ad(828, &AdValue::scale(scratch.ad_value(958), 64.0));
        }

        scratch.values[1306] = if (scratch.values[958] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(scratch.values[1305] != 0.0)) && (scratch.values[1306] != 0.0)) {
            scratch.store_ad(828, &AdValue::offset(AdValue::scale(scratch.ad_value(958), 22.0), 3.0));
        }

        scratch.values[1307] = if (scratch.values[958] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(scratch.values[1305] != 0.0)) && (!(scratch.values[1306] != 0.0))) && (scratch.values[1307] != 0.0)) {
            scratch.store_ad(828, &AdValue::offset(AdValue::scale(scratch.ad_value(958), (-7.2)), 15.5));
        }

        if (((!(scratch.values[1305] != 0.0)) && (!(scratch.values[1306] != 0.0))) && (!(scratch.values[1307] != 0.0))) {
            scratch.values[828] = scratch.values[821];
            scratch.node_derivatives[828] = scratch.node_derivatives[821];
            scratch.branch_derivatives[828] = scratch.branch_derivatives[821];
        }

        scratch.store_ad(829, &AdValue::sub(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(823), 0.5)), AdValue::mul(scratch.ad_value(821), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(960), AdValue::scale(scratch.ad_value(823), 0.25)), scratch.ad_value(828))))));

        scratch.store_ad(830, &AdValue::div_from_scalar(1.0, scratch.ad_value(260)));

        scratch.store_ad(831, &AdValue::scale(AdValue::sqrt(AdValue::scale(scratch.ad_value(260), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33)));

        scratch.store_ad(832, &AdValue::mul(scratch.ad_value(831), scratch.ad_value(196)));

        scratch.store_ad(833, &AdValue::mul(scratch.ad_value(831), scratch.ad_value(212)));

        scratch.store_ad(834, &AdValue::mul(scratch.ad_value(831), scratch.ad_value(213)));

        scratch.values[835] = 0.0;

        scratch.values[1308] = if (scratch.values[259] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1308] != 0.0) {
            scratch.store_ad(835, &AdValue::div(AdValue::scale(scratch.ad_value(258), (-0.495)), scratch.ad_value(259)));
        }

        scratch.store_ad(836, &AdValue::pow_from_scalar(scratch.values[361], scratch.ad_value(257)));

        scratch.store_ad(254, &AdValue::mul(scratch.ad_value(254), scratch.ad_value(836)));

        scratch.store_ad(255, &AdValue::mul(scratch.ad_value(255), scratch.ad_value(836)));

        scratch.store_ad(256, &AdValue::mul(scratch.ad_value(256), scratch.ad_value(836)));

        scratch.store_ad(837, &AdValue::div(AdValue::scale(scratch.ad_value(261), 4e-18), AdValue::square(scratch.ad_value(212))));

        scratch.store_ad(838, &AdValue::div(AdValue::scale(scratch.ad_value(262), 4e-18), AdValue::square(scratch.ad_value(213))));

        if ((1.0 + (scratch.values[265] * scratch.values[362])) > 0.0) {
            scratch.store_ad(831, &AdValue::offset(AdValue::scale(scratch.ad_value(265), scratch.values[362]), 1.0));
        } else {
            scratch.values[831] = 0.0;
            scratch.node_derivatives[831] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[831] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(761, &AdValue::mul(scratch.ad_value(263), scratch.ad_value(831)));

        scratch.store_ad(839, &AdValue::scale(AdValue::mul(scratch.ad_value(761), scratch.ad_value(212)), 500000000.0));

        if ((1.0 + (scratch.values[266] * scratch.values[362])) > 0.0) {
            scratch.store_ad(831, &AdValue::offset(AdValue::scale(scratch.ad_value(266), scratch.values[362]), 1.0));
        } else {
            scratch.values[831] = 0.0;
            scratch.node_derivatives[831] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[831] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(762, &AdValue::mul(scratch.ad_value(264), scratch.ad_value(831)));

        scratch.store_ad(840, &AdValue::scale(AdValue::mul(scratch.ad_value(762), scratch.ad_value(213)), 500000000.0));

        scratch.store_ad(766, &AdValue::mul(scratch.ad_value(306), AdValue::pow_from_scalar(scratch.values[361], scratch.ad_value(308))));

        scratch.store_ad(841, &AdValue::scale(scratch.ad_value(276), (9.1093826e-31 * 1000000000.0)));

        scratch.values[1309] = if (scratch.values[299] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1309] != 0.0) {
            scratch.store_ad(842, &AdValue::div_from_scalar(1.0, scratch.ad_value(299)));
        }

        if (!(scratch.values[1309] != 0.0)) {
            scratch.values[842] = 0.0;
            scratch.node_derivatives[842] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[842] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1310] = if (scratch.values[300] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1310] != 0.0) {
            scratch.store_ad(843, &AdValue::div_from_scalar(1.0, scratch.ad_value(300)));
        }

        if (!(scratch.values[1310] != 0.0)) {
            scratch.values[843] = 0.0;
            scratch.node_derivatives[843] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[843] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1311] = if (scratch.values[301] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1311] != 0.0) {
            scratch.store_ad(844, &AdValue::div_from_scalar(1.0, scratch.ad_value(301)));
        }

        if (!(scratch.values[1311] != 0.0)) {
            scratch.values[844] = 0.0;
            scratch.node_derivatives[844] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[844] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1312] = if (scratch.values[302] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1312] != 0.0) {
            scratch.store_ad(845, &AdValue::div_from_scalar(1.0, scratch.ad_value(302)));
        }

        if (!(scratch.values[1312] != 0.0)) {
            scratch.values[845] = 0.0;
            scratch.node_derivatives[845] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[845] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1313] = if (scratch.values[303] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1313] != 0.0) {
            scratch.store_ad(846, &AdValue::div_from_scalar(1.0, scratch.ad_value(303)));
        }

        if (!(scratch.values[1313] != 0.0)) {
            scratch.values[846] = 0.0;
            scratch.node_derivatives[846] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[846] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1314] = if (scratch.values[304] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1314] != 0.0) {
            scratch.store_ad(847, &AdValue::div_from_scalar(1.0, scratch.ad_value(304)));
        }

        if (!(scratch.values[1314] != 0.0)) {
            scratch.values[847] = 0.0;
            scratch.node_derivatives[847] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[847] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1315] = if (scratch.values[305] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1315] != 0.0) {
            scratch.store_ad(848, &AdValue::div_from_scalar(1.0, scratch.ad_value(305)));
        }

        if (!(scratch.values[1315] != 0.0)) {
            scratch.values[848] = 0.0;
            scratch.node_derivatives[848] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[848] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[30] = 0.0;

        scratch.values[31] = 0.0;

        scratch.values[32] = 0.0;

        scratch.values[33] = 0.0;

        scratch.values[34] = 0.0;

        scratch.values[35] = 0.0;

        scratch.values[36] = 0.0;

        scratch.values[37] = scratch.values[314];
        scratch.node_derivatives[37] = scratch.node_derivatives[314];
        scratch.branch_derivatives[37] = scratch.branch_derivatives[314];

        scratch.values[1316] = if (scratch.values[1] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1316] != 0.0) {
            scratch.values[37] = (if (scratch.values[20] > 0.0) { scratch.values[20] } else { 0.0 });
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1317] = if (scratch.values[5] == 3.0) { 1.0 } else { 0.0 };

        if (scratch.values[1317] != 0.0) {
            scratch.values[36] = 1.0;
            scratch.node_derivatives[36] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[36] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[697]));

        scratch.store_ad(31, &AdValue::scale(scratch.ad_value(12), scratch.values[698]));

        scratch.store_ad(32, &AdValue::scale(scratch.ad_value(12), scratch.values[699]));

        scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[724]));

        scratch.store_ad(34, &AdValue::scale(scratch.ad_value(12), scratch.values[725]));

        scratch.store_ad(35, &AdValue::scale(scratch.ad_value(12), scratch.values[726]));

        scratch.values[1318] = if ((scratch.values[5] == 2.0) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[700]));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(31, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[701]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.values[32] = scratch.values[37];
            scratch.node_derivatives[32] = scratch.node_derivatives[37];
            scratch.branch_derivatives[32] = scratch.branch_derivatives[37];
        }

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[727]));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.store_ad(34, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[728]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1318] != 0.0) {
            scratch.values[35] = scratch.values[37];
            scratch.node_derivatives[35] = scratch.node_derivatives[37];
            scratch.branch_derivatives[35] = scratch.branch_derivatives[37];
        }

        scratch.values[1319] = if (((scratch.values[5] == 1.0) || (scratch.values[5] == 2.0)) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(697, &{
                if (scratch.values[30] > 0.0) {
                    scratch.ad_value(30)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(698, &{
                if (scratch.values[31] > 0.0) {
                    scratch.ad_value(31)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(699, &{
                if (scratch.values[32] > 0.0) {
                    scratch.ad_value(32)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(724, &{
                if (scratch.values[33] > 0.0) {
                    scratch.ad_value(33)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(725, &{
                if (scratch.values[34] > 0.0) {
                    scratch.ad_value(34)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[1319] != 0.0) {
            scratch.store_ad(726, &{
                if (scratch.values[35] > 0.0) {
                    scratch.ad_value(35)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[697] = 0.0;
            scratch.node_derivatives[697] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[697] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[698] = 0.0;
            scratch.node_derivatives[698] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[698] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[699] = 0.0;
            scratch.node_derivatives[699] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[699] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[724] = 0.0;
            scratch.node_derivatives[724] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[724] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[725] = 0.0;
            scratch.node_derivatives[725] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[725] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1319] != 0.0)) {
            scratch.values[726] = 0.0;
            scratch.node_derivatives[726] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[726] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[707] = 0.0;

        scratch.values[734] = 0.0;

        scratch.values[709] = 0.0;

        scratch.values[736] = 0.0;

        scratch.values[708] = 0.0;

        scratch.values[735] = 0.0;

        scratch.values[710] = 0.0;

        scratch.values[737] = 0.0;

        scratch.values[705] = 0.0;

        scratch.values[732] = 0.0;

        scratch.values[706] = 0.0;

        scratch.values[733] = 0.0;

        scratch.values[718] = 0.0;

        scratch.values[745] = 0.0;

        scratch.values[719] = 1.0;

        scratch.values[746] = 1.0;

        scratch.values[720] = 0.0;

        scratch.values[747] = 0.0;

        scratch.values[721] = 1.0;

        scratch.values[748] = 1.0;

        scratch.values[722] = 0.0;

        scratch.values[749] = 0.0;

        scratch.values[723] = 1.0;

        scratch.values[750] = 1.0;

        scratch.values[717] = 0.0;

        scratch.values[744] = 0.0;

        scratch.values[711] = 0.0;

        scratch.values[738] = 0.0;

        scratch.values[712] = 0.0;

        scratch.values[739] = 0.0;

        scratch.values[713] = 0.0;

        scratch.values[740] = 0.0;

        scratch.values[714] = 0.0;

        scratch.values[741] = 0.0;

        scratch.values[715] = 0.0;

        scratch.values[742] = 0.0;

        scratch.values[716] = 0.0;

        scratch.values[743] = 0.0;

        scratch.values[702] = 1.0;

        scratch.values[729] = 1.0;

        scratch.values[703] = 1.0;

        scratch.values[730] = 1.0;

        scratch.values[704] = 1.0;

        scratch.values[731] = 1.0;

        scratch.values[529] = 0.0;

        scratch.values[530] = 0.0;

        scratch.values[518] = 0.0;

        scratch.values[519] = 0.0;

        scratch.values[520] = 0.0;

        scratch.values[521] = 0.0;

        scratch.values[522] = 0.0;

        scratch.values[531] = 0.0;

        scratch.values[532] = 0.0;

        scratch.values[533] = 0.0;

        scratch.values[539] = 0.0;

        scratch.values[528] = 0.0;

        scratch.values[1320] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[1321] = if ((scratch.values[443] * scratch.values[697]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1321] != 0.0)) {
            scratch.store_ad(510, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::scale(scratch.ad_value(697), scratch.values[443])), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1321] != 0.0))) {
            scratch.values[510] = 100000000.0;
            scratch.node_derivatives[510] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[510] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1322] = if ((scratch.values[444] * scratch.values[698]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1322] != 0.0)) {
            scratch.store_ad(511, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::scale(scratch.ad_value(698), scratch.values[444])), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1322] != 0.0))) {
            scratch.values[511] = 100000000.0;
            scratch.node_derivatives[511] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[511] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1323] = if ((scratch.values[445] * scratch.values[699]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1323] != 0.0)) {
            scratch.store_ad(512, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::scale(scratch.ad_value(699), scratch.values[445])), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1323] != 0.0))) {
            scratch.values[512] = 100000000.0;
            scratch.node_derivatives[512] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[512] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(705, &AdValue::min(AdValue::min(scratch.ad_value(510), scratch.ad_value(511)), scratch.ad_value(512)));
        }

        scratch.values[1324] = if ((((scratch.values[705] * scratch.values[427])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1324] != 0.0)) {
            scratch.store_ad(706, &AdValue::exp(AdValue::scale(scratch.ad_value(705), scratch.values[427])));
        }

        scratch.values[1325] = if ((scratch.values[705] * scratch.values[427]) < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (!(scratch.values[1324] != 0.0))) && (scratch.values[1325] != 0.0)) {
            scratch.store_ad(706, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(705), scratch.values[427])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(705), scratch.values[427])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(705), scratch.values[427])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (!(scratch.values[1324] != 0.0))) && (!(scratch.values[1325] != 0.0))) {
            scratch.store_ad(706, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(705), scratch.values[427]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(705), scratch.values[427]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(705), scratch.values[427]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[452] = scratch.values[449];
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[453] = scratch.values[450];
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[454] = scratch.values[451];
            scratch.node_derivatives[454] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[454] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[455] = scratch.values[382];
            scratch.node_derivatives[455] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[455] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[456] = scratch.values[383];
            scratch.node_derivatives[456] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[456] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[457] = scratch.values[384];
            scratch.node_derivatives[457] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[457] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[458] = scratch.values[379];
            scratch.node_derivatives[458] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[458] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[459] = scratch.values[380];
            scratch.node_derivatives[459] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[459] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[460] = scratch.values[381];
            scratch.node_derivatives[460] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[460] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1326] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1326] != 0.0)) {
            scratch.values[452] = (scratch.values[450] + scratch.values[451]);
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1326] != 0.0)) {
            scratch.values[455] = (0.9 * (scratch.values[383]).min(scratch.values[384]));
            scratch.node_derivatives[455] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[455] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1326] != 0.0)) {
            scratch.values[458] = (scratch.values[380] + scratch.values[381]);
            scratch.node_derivatives[458] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[458] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1327] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1327] != 0.0)) {
            scratch.values[453] = (scratch.values[449] + scratch.values[451]);
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1327] != 0.0)) {
            scratch.values[456] = (0.9 * (scratch.values[382]).min(scratch.values[384]));
            scratch.node_derivatives[456] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[456] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1327] != 0.0)) {
            scratch.values[459] = (scratch.values[379] + scratch.values[381]);
            scratch.node_derivatives[459] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[459] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1328] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1328] != 0.0)) {
            scratch.values[454] = (scratch.values[449] + scratch.values[450]);
            scratch.node_derivatives[454] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[454] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1328] != 0.0)) {
            scratch.values[457] = (0.9 * (scratch.values[382]).min(scratch.values[383]));
            scratch.node_derivatives[457] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[457] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1328] != 0.0)) {
            scratch.values[460] = (scratch.values[379] + scratch.values[380]);
            scratch.node_derivatives[460] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[460] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(707, &AdValue::min(AdValue::min(scratch.ad_value(452), scratch.ad_value(453)), scratch.ad_value(454)));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(708, &AdValue::scale(scratch.ad_value(707), 0.1));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(433, &AdValue::max(AdValue::max(scratch.ad_value(455), scratch.ad_value(456)), scratch.ad_value(457)));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(709, &AdValue::mul(scratch.ad_value(707), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(433))))));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(710, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(458), scratch.ad_value(459)), scratch.ad_value(460)), (-0.05)));
        }

        scratch.values[1329] = if ((scratch.values[595] * scratch.values[724]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.store_ad(510, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::mul(scratch.ad_value(595), scratch.ad_value(724))), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1329] != 0.0))) {
            scratch.values[510] = 100000000.0;
            scratch.node_derivatives[510] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[510] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1330] = if ((scratch.values[596] * scratch.values[725]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.store_ad(511, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::mul(scratch.ad_value(596), scratch.ad_value(725))), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1330] != 0.0))) {
            scratch.values[511] = 100000000.0;
            scratch.node_derivatives[511] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[511] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1331] = if ((scratch.values[597] * scratch.values[726]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.store_ad(512, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[374], AdValue::mul(scratch.ad_value(597), scratch.ad_value(726))), 1.0)), scratch.values[426]));
        }

        if ((scratch.values[1320] != 0.0) && (!(scratch.values[1331] != 0.0))) {
            scratch.values[512] = 100000000.0;
            scratch.node_derivatives[512] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[512] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(732, &AdValue::min(AdValue::min(scratch.ad_value(510), scratch.ad_value(511)), scratch.ad_value(512)));
        }

        scratch.values[1332] = if ((((scratch.values[732] * scratch.values[427])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1332] != 0.0)) {
            scratch.store_ad(733, &AdValue::exp(AdValue::scale(scratch.ad_value(732), scratch.values[427])));
        }

        scratch.values[1333] = if ((scratch.values[732] * scratch.values[427]) < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (!(scratch.values[1332] != 0.0))) && (scratch.values[1333] != 0.0)) {
            scratch.store_ad(733, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(732), scratch.values[427])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(732), scratch.values[427])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(732), scratch.values[427])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (!(scratch.values[1332] != 0.0))) && (!(scratch.values[1333] != 0.0))) {
            scratch.store_ad(733, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(732), scratch.values[427]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(732), scratch.values[427]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(732), scratch.values[427]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[452] = scratch.values[601];
            scratch.node_derivatives[452] = scratch.node_derivatives[601];
            scratch.branch_derivatives[452] = scratch.branch_derivatives[601];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[453] = scratch.values[602];
            scratch.node_derivatives[453] = scratch.node_derivatives[602];
            scratch.branch_derivatives[453] = scratch.branch_derivatives[602];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[454] = scratch.values[603];
            scratch.node_derivatives[454] = scratch.node_derivatives[603];
            scratch.branch_derivatives[454] = scratch.branch_derivatives[603];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[455] = scratch.values[549];
            scratch.node_derivatives[455] = scratch.node_derivatives[549];
            scratch.branch_derivatives[455] = scratch.branch_derivatives[549];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[456] = scratch.values[550];
            scratch.node_derivatives[456] = scratch.node_derivatives[550];
            scratch.branch_derivatives[456] = scratch.branch_derivatives[550];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[457] = scratch.values[551];
            scratch.node_derivatives[457] = scratch.node_derivatives[551];
            scratch.branch_derivatives[457] = scratch.branch_derivatives[551];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[458] = scratch.values[546];
            scratch.node_derivatives[458] = scratch.node_derivatives[546];
            scratch.branch_derivatives[458] = scratch.branch_derivatives[546];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[459] = scratch.values[547];
            scratch.node_derivatives[459] = scratch.node_derivatives[547];
            scratch.branch_derivatives[459] = scratch.branch_derivatives[547];
        }

        if (scratch.values[1320] != 0.0) {
            scratch.values[460] = scratch.values[548];
            scratch.node_derivatives[460] = scratch.node_derivatives[548];
            scratch.branch_derivatives[460] = scratch.branch_derivatives[548];
        }

        scratch.values[1334] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1334] != 0.0)) {
            scratch.store_ad(452, &AdValue::add(scratch.ad_value(602), scratch.ad_value(603)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1334] != 0.0)) {
            scratch.store_ad(455, &AdValue::scale(AdValue::min(scratch.ad_value(550), scratch.ad_value(551)), 0.9));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1334] != 0.0)) {
            scratch.store_ad(458, &AdValue::add(scratch.ad_value(547), scratch.ad_value(548)));
        }

        scratch.values[1335] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1335] != 0.0)) {
            scratch.store_ad(453, &AdValue::add(scratch.ad_value(601), scratch.ad_value(603)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1335] != 0.0)) {
            scratch.store_ad(456, &AdValue::scale(AdValue::min(scratch.ad_value(549), scratch.ad_value(551)), 0.9));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1335] != 0.0)) {
            scratch.store_ad(459, &AdValue::add(scratch.ad_value(546), scratch.ad_value(548)));
        }

        scratch.values[1336] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1336] != 0.0)) {
            scratch.store_ad(454, &AdValue::add(scratch.ad_value(601), scratch.ad_value(602)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1336] != 0.0)) {
            scratch.store_ad(457, &AdValue::scale(AdValue::min(scratch.ad_value(549), scratch.ad_value(550)), 0.9));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1336] != 0.0)) {
            scratch.store_ad(460, &AdValue::add(scratch.ad_value(546), scratch.ad_value(547)));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(734, &AdValue::min(AdValue::min(scratch.ad_value(452), scratch.ad_value(453)), scratch.ad_value(454)));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(735, &AdValue::scale(scratch.ad_value(734), 0.1));
        }

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(433, &AdValue::max(AdValue::max(scratch.ad_value(455), scratch.ad_value(456)), scratch.ad_value(457)));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(736, &AdValue::mul(scratch.ad_value(734), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(433))))));
        }

        if (scratch.values[1320] != 0.0) {
            scratch.store_ad(737, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(458), scratch.ad_value(459)), scratch.ad_value(460)), (-0.05)));
        }

        scratch.values[1337] = if (scratch.values[417] == 1.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1338] = 0.0;
            scratch.node_derivatives[1338] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1338] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1339] = 0.0;
            scratch.node_derivatives[1339] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1339] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1340] = 0.0;
            scratch.node_derivatives[1340] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1340] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1341] = 0.0;
            scratch.node_derivatives[1341] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1341] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1342] = 0.0;
            scratch.node_derivatives[1342] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1342] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1343] = 0.0;
            scratch.node_derivatives[1343] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1343] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1344] = 0.0;
            scratch.node_derivatives[1344] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1344] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1345] = 0.0;
            scratch.node_derivatives[1345] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1345] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1346] = 0.0;
            scratch.node_derivatives[1346] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1346] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1347] = 0.0;
            scratch.node_derivatives[1347] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1347] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1348] = 0.0;
            scratch.node_derivatives[1348] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1348] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1349] = 0.0;
            scratch.node_derivatives[1349] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1349] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1350] = 0.0;
            scratch.node_derivatives[1350] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1350] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1351] = 0.0;
            scratch.node_derivatives[1351] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1351] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1352] = 0.0;
            scratch.node_derivatives[1352] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1352] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1353] = 0.0;
            scratch.node_derivatives[1353] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1353] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1355] = 0.0;
            scratch.node_derivatives[1355] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1355] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1356] = 0.0;
            scratch.node_derivatives[1356] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1356] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1357] = 0.0;
            scratch.node_derivatives[1357] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1357] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1359] = 0.0;
            scratch.node_derivatives[1359] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1359] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1360] = 0.0;
            scratch.node_derivatives[1360] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1360] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1362] = 0.0;
            scratch.node_derivatives[1362] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1362] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1363] = 0.0;
            scratch.node_derivatives[1363] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1363] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1366] = 0.0;
            scratch.node_derivatives[1366] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1366] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1367] = 0.0;
            scratch.node_derivatives[1367] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1367] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1369] = 0.0;
            scratch.node_derivatives[1369] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1369] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1370] = 0.0;
            scratch.node_derivatives[1370] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1370] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1371] = 0.0;
            scratch.node_derivatives[1371] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1371] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1372] = 0.0;
            scratch.node_derivatives[1372] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1372] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1373] = 0.0;
            scratch.node_derivatives[1373] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1373] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1374] = 0.0;
            scratch.node_derivatives[1374] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1374] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1375] = 0.0;
            scratch.node_derivatives[1375] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1375] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1376] = 0.0;
            scratch.node_derivatives[1376] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1376] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1377] = 0.0;
            scratch.node_derivatives[1377] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1377] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1378] = 0.0;
            scratch.node_derivatives[1378] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1378] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1379] = 0.0;
            scratch.node_derivatives[1379] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1379] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1381] = 0.0;
            scratch.node_derivatives[1381] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1381] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1382] = 0.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[536] = 0.4;
            scratch.node_derivatives[536] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[536] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[537] = 0.65;
            scratch.node_derivatives[537] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[537] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[538] = 0.8;
            scratch.node_derivatives[538] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[538] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(523, &AdValue::scale(AdValue::neg(scratch.ad_value(536)), scratch.values[418]));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(524, &AdValue::scale(AdValue::neg(scratch.ad_value(537)), scratch.values[418]));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(525, &AdValue::scale(AdValue::neg(scratch.ad_value(538)), scratch.values[418]));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[526] = 0.1;
            scratch.node_derivatives[526] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[526] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[527] = 0.2;
            scratch.node_derivatives[527] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[527] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1351] = 0.0;
            scratch.node_derivatives[1351] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1351] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1389] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(523), AdValue::mul(scratch.ad_value(708), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(523), scratch.ad_value(709)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1390] = if (scratch.values[523] < scratch.values[705]) { 1.0 } else { 0.0 };

        scratch.values[1391] = if ((((0.5 * (scratch.values[523] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (scratch.values[1390] != 0.0)) && (scratch.values[1391] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))));
        }

        scratch.values[1392] = if ((0.5 * (scratch.values[523] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (scratch.values[1390] != 0.0)) && (!(scratch.values[1391] != 0.0))) && (scratch.values[1392] != 0.0)) {
            let assign13000_ad_e9924: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign13000_ad_e9924);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (scratch.values[1390] != 0.0)) && (!(scratch.values[1391] != 0.0))) && (!(scratch.values[1392] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (scratch.values[1390] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (!(scratch.values[1390] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(523), scratch.ad_value(705)), scratch.values[427]), 1.0), scratch.ad_value(706)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (!(scratch.values[1390] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1393] = if (scratch.values[523] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (scratch.values[1393] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) && (!(scratch.values[1393] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(523)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(707), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(523), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(523), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(523), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(523), scratch.ad_value(710)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(523), scratch.ad_value(710)), AdValue::sub(scratch.ad_value(523), scratch.ad_value(710))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1389] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(523), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(523), scratch.ad_value(523)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1394] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1394] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1395] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (scratch.values[1395] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1395] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[443]));
        }

        scratch.values[1396] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (scratch.values[1396] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1397] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) && (scratch.values[1397] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) && (!(scratch.values[1397] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[382]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1398] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) && (scratch.values[1398] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) && (!(scratch.values[1398] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[479]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[440]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1396] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[393]));
        }

        scratch.values[1399] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (scratch.values[1399] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[464]), scratch.ad_value(1359)), scratch.values[494]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[491]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1400] = if (((-scratch.values[382]) * scratch.values[467]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (scratch.values[1400] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1400] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[382]) * scratch.values[467])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[491]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[491])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1401] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (scratch.values[1401] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1401] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1402] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (scratch.values[1402] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1402] != 0.0))) {
            let assign13580_ad_e10867: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign13580_ad_e10867);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1403] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (scratch.values[1403] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1404] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1403] != 0.0))) && (scratch.values[1404] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1403] != 0.0))) && (!(scratch.values[1404] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) && (!(scratch.values[1403] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[491]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1399] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[396]));
        }

        scratch.values[1405] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (scratch.values[1405] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1406] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) && (scratch.values[1406] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) && (!(scratch.values[1406] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[482]), scratch.ad_value(1356)), scratch.values[467]));
        }

        scratch.values[1407] = if (((((-scratch.values[497]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) && (scratch.values[1407] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))));
        }

        scratch.values[1408] = if (((-scratch.values[497]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) && (!(scratch.values[1407] != 0.0))) && (scratch.values[1408] != 0.0)) {
            let assign13770_ad_e11194: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign13770_ad_e11194));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) && (!(scratch.values[1407] != 0.0))) && (!(scratch.values[1408] != 0.0))) {
            let assign13780_ad_e11244: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign13780_ad_e11244);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1405] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(523), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[402]));
        }

        scratch.values[1409] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (scratch.values[1409] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1410] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[1411] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1409] != 0.0))) && (scratch.values[1410] != 0.0)) && (scratch.values[1411] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[504]), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1409] != 0.0))) && (scratch.values[1410] != 0.0)) && (!(scratch.values[1411] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[504])), scratch.values[414]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1409] != 0.0))) && (scratch.values[1410] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) && (!(scratch.values[1409] != 0.0))) && (!(scratch.values[1410] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[411])), scratch.values[507]), scratch.values[501]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1394] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1412] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1412] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1413] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1413] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1413] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[444]));
        }

        scratch.values[1414] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1414] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1415] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) && (scratch.values[1415] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) && (!(scratch.values[1415] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[383]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1416] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) && (scratch.values[1416] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) && (!(scratch.values[1416] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[480]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[441]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1414] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[394]));
        }

        scratch.values[1417] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1417] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[465]), scratch.ad_value(1359)), scratch.values[495]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[492]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1418] = if (((-scratch.values[383]) * scratch.values[468]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (scratch.values[1418] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1418] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[383]) * scratch.values[468])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[492]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[492])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1419] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (scratch.values[1419] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1419] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1420] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (scratch.values[1420] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            let assign14330_ad_e12073: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign14330_ad_e12073);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1421] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (scratch.values[1421] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1422] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1421] != 0.0))) && (scratch.values[1422] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1421] != 0.0))) && (!(scratch.values[1422] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1421] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[492]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[397]));
        }

        scratch.values[1423] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1423] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1424] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) && (scratch.values[1424] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) && (!(scratch.values[1424] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[483]), scratch.ad_value(1356)), scratch.values[468]));
        }

        scratch.values[1425] = if (((((-scratch.values[498]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) && (scratch.values[1425] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))));
        }

        scratch.values[1426] = if (((-scratch.values[498]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) && (!(scratch.values[1425] != 0.0))) && (scratch.values[1426] != 0.0)) {
            let assign14520_ad_e12400: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign14520_ad_e12400));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) && (!(scratch.values[1425] != 0.0))) && (!(scratch.values[1426] != 0.0))) {
            let assign14530_ad_e12450: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign14530_ad_e12450);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1423] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(523), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[403]));
        }

        scratch.values[1427] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (scratch.values[1427] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1428] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[1429] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1427] != 0.0))) && (scratch.values[1428] != 0.0)) && (scratch.values[1429] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[505]), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1427] != 0.0))) && (scratch.values[1428] != 0.0)) && (!(scratch.values[1429] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[505])), scratch.values[415]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1427] != 0.0))) && (scratch.values[1428] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) && (!(scratch.values[1427] != 0.0))) && (!(scratch.values[1428] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[412])), scratch.values[508]), scratch.values[502]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1412] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1430] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1430] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1431] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1431] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1431] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[445]));
        }

        scratch.values[1432] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1432] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1433] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

    }
}
