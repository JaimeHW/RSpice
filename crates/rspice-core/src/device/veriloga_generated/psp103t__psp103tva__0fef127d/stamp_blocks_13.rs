#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
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
            scratch.store_ad(539, &AdValue::scale(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(697), scratch.values[470]), AdValue::scale(scratch.ad_value(698), scratch.values[471])), AdValue::scale(scratch.ad_value(699), scratch.values[472])), scratch.values[419]));
        }

        scratch.values[1690] = if ((scratch.values[697] * scratch.values[470]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1690] != 0.0)) {
            scratch.values[702] = 0.0;
            scratch.node_derivatives[702] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[702] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1691] = if ((scratch.values[698] * scratch.values[471]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.values[703] = 0.0;
            scratch.node_derivatives[703] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[703] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1692] = if ((scratch.values[699] * scratch.values[472]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1692] != 0.0)) {
            scratch.values[704] = 0.0;
            scratch.node_derivatives[704] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[704] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(539, &AdValue::mul(scratch.ad_value(585), AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(613)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(614))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(615)))));
        }

        scratch.values[1995] = if ((scratch.values[724] * scratch.values[613]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1995] != 0.0)) {
            scratch.values[729] = 0.0;
            scratch.node_derivatives[729] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[729] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1996] = if ((scratch.values[725] * scratch.values[614]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.values[730] = 0.0;
            scratch.node_derivatives[730] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[730] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1997] = if ((scratch.values[726] * scratch.values[615]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1997] != 0.0)) {
            scratch.values[731] = 0.0;
            scratch.node_derivatives[731] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[731] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2086] = 0.0;

        scratch.values[2087] = 0.0;

        scratch.values[2088] = 0.0;

        scratch.store_ad(365, &AdValue::offset(AdValue::voltage(ctx, &self.nodes, Some(4), None), scratch.values[360]));

        scratch.store_ad(366, &AdValue::square(scratch.ad_value(365)));

        scratch.store_ad(367, &AdValue::offset(scratch.ad_value(365), (-scratch.values[359])));

        scratch.store_ad(368, &AdValue::div_from_scalar(scratch.values[359], scratch.ad_value(365)));

        scratch.store_ad(369, &AdValue::ln(scratch.ad_value(368)));

        scratch.store_ad(2074, &AdValue::scale(scratch.ad_value(365), (1.3806505e-23 * 6.241449993689894e18)));

        scratch.store_ad(370, &AdValue::div_from_scalar(1.0, scratch.ad_value(2074)));

        scratch.store_ad(371, &AdValue::sub(AdValue::sub_from_scalar(1.179, AdValue::scale(scratch.ad_value(365), 9.025e-5)), AdValue::scale(scratch.ad_value(366), 3.05e-7)));

        scratch.store_ad(372, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(365), 0.00045), 1.045), AdValue::sub(AdValue::offset(AdValue::scale(scratch.ad_value(365), 0.0014), 0.523), AdValue::scale(scratch.ad_value(366), 1.48e-6))), scratch.ad_value(366)), 1.1111111111111112e-5));

        if !(scratch.values[372] > 0.001) {
            scratch.values[372] = 0.001;
            scratch.node_derivatives[372] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[372] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(767, &AdValue::add(AdValue::add(scratch.ad_value(371), scratch.ad_value(206)), AdValue::mul(AdValue::scale(scratch.ad_value(2074), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(198), AdValue::powf(scratch.ad_value(372), (-0.75))), 4e-26)))));

        if !(scratch.values[767] > 0.05) {
            scratch.values[767] = 0.05;
            scratch.node_derivatives[767] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[767] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(768, &AdValue::div(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(198), ((2.0 * 1.6021918e-19) * scratch.values[806])), scratch.ad_value(370))), scratch.ad_value(808)));

        scratch.values[769] = 0.0;

        scratch.values[770] = 0.0;

        scratch.values[2089] = if (scratch.values[208] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(771, &AdValue::div_from_scalar(80000000.0, scratch.ad_value(809)));
        }

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(770, &{
                if (scratch.values[208] > scratch.values[771]) {
                    scratch.ad_value(208)
                } else {
                    scratch.ad_value(771)
                }
            });
        }

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(770, &{
                if (5e24 > scratch.values[770]) {
                    AdValue::constant(5e24)
                } else {
                    scratch.ad_value(770)
                }
            });
        }

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(769, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(808), 2.0), scratch.ad_value(808)), scratch.ad_value(2074)), AdValue::scale(scratch.ad_value(770), (1.6021918e-19 * scratch.values[806]))));
        }

        scratch.store_ad(772, &AdValue::mul(AdValue::scale(scratch.ad_value(2074), 100.0), scratch.ad_value(2074)));

        scratch.values[2090] = if (scratch.values[191] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(773, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(768)), scratch.ad_value(768)), scratch.ad_value(767))));
        }

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(774, &AdValue::mul(AdValue::scale(scratch.ad_value(812), 0.75), AdValue::powf(scratch.ad_value(773), 0.6666666666666666)));
        }

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(767, &AdValue::add(scratch.ad_value(767), scratch.ad_value(774)));
        }

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(768, &AdValue::mul(scratch.ad_value(768), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(774), (2.0 * 0.6666666666666666)), scratch.ad_value(773)), 1.0)));
        }

        scratch.store_ad(775, &AdValue::sqrt(scratch.ad_value(767)));

        scratch.store_ad(776, &AdValue::scale(scratch.ad_value(767), 0.95));

        scratch.store_ad(777, &AdValue::mul(AdValue::scale(scratch.ad_value(767), 0.0025), scratch.ad_value(767)));

        scratch.values[778] = scratch.values[777];
        scratch.node_derivatives[778] = scratch.node_derivatives[777];
        scratch.branch_derivatives[778] = scratch.branch_derivatives[777];

        scratch.store_ad(779, &AdValue::scale(AdValue::sqrt(scratch.ad_value(778)), 0.5));

        scratch.store_ad(780, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(776), scratch.ad_value(779)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(776), scratch.ad_value(779)), AdValue::sub(scratch.ad_value(776), scratch.ad_value(779))), scratch.ad_value(777)))), 0.5));

        scratch.store_ad(781, &AdValue::scale(AdValue::add(scratch.ad_value(767), scratch.ad_value(371)), 0.5));

        scratch.store_ad(782, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(201), scratch.ad_value(767))), scratch.ad_value(775)));

        scratch.store_ad(783, &AdValue::sub(AdValue::sub(AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(201), scratch.ad_value(202)), scratch.ad_value(767))), scratch.ad_value(775)), scratch.ad_value(782)));

        scratch.store_ad(784, &AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(371), scratch.ad_value(206)), scratch.ad_value(207)), AdValue::mul(AdValue::scale(scratch.ad_value(2074), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(811), AdValue::powf(scratch.ad_value(372), (-0.75))), 4e-26)))));

        if !(scratch.values[784] > 0.05) {
            scratch.values[784] = 0.05;
            scratch.node_derivatives[784] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[784] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(785, &AdValue::div(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(811), ((2.0 * 1.6021918e-19) * scratch.values[806])), scratch.ad_value(370))), scratch.ad_value(808)));

        scratch.values[2091] = if (scratch.values[191] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(773, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(785)), scratch.ad_value(785)), scratch.ad_value(784))));
        }

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(774, &AdValue::mul(AdValue::scale(scratch.ad_value(812), 0.75), AdValue::powf(scratch.ad_value(773), 0.6666666666666666)));
        }

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(784, &AdValue::add(scratch.ad_value(784), scratch.ad_value(774)));
        }

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(785, &AdValue::mul(scratch.ad_value(785), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(774), (2.0 * 0.6666666666666666)), scratch.ad_value(773)), 1.0)));
        }

        scratch.store_ad(786, &AdValue::scale(scratch.ad_value(784), 0.95));

        scratch.store_ad(787, &AdValue::mul(AdValue::scale(scratch.ad_value(784), 0.0025), scratch.ad_value(784)));

        scratch.values[788] = scratch.values[787];
        scratch.node_derivatives[788] = scratch.node_derivatives[787];
        scratch.branch_derivatives[788] = scratch.branch_derivatives[787];

        scratch.store_ad(779, &AdValue::scale(AdValue::sqrt(scratch.ad_value(788)), 0.5));

        scratch.store_ad(789, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(786), scratch.ad_value(779)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(786), scratch.ad_value(779)), AdValue::sub(scratch.ad_value(786), scratch.ad_value(779))), scratch.ad_value(787)))), 0.5));

        scratch.store_ad(751, &AdValue::offset(AdValue::add(scratch.ad_value(192), AdValue::mul(AdValue::mul(scratch.ad_value(193), scratch.ad_value(367)), AdValue::offset(AdValue::mul(scratch.ad_value(194), scratch.ad_value(367)), 1.0))), scratch.values[27]));

        scratch.store_ad(790, &AdValue::exp(AdValue::mul(scratch.ad_value(195), scratch.ad_value(369))));

        scratch.store_ad(752, &AdValue::mul(scratch.ad_value(209), scratch.ad_value(790)));

    }

    pub(super) fn stamp_reactive_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.store_ad(753, &AdValue::div(scratch.ad_value(210), scratch.ad_value(368)));

        scratch.store_ad(791, &AdValue::exp(AdValue::mul(scratch.ad_value(223), scratch.ad_value(369))));

        scratch.store_ad(754, &AdValue::mul(scratch.ad_value(222), scratch.ad_value(791)));

        scratch.store_ad(2075, &AdValue::mul(AdValue::scale(scratch.ad_value(754), scratch.values[26]), scratch.ad_value(808)));

        scratch.store_ad(756, &AdValue::mul(scratch.ad_value(226), AdValue::exp(AdValue::mul(scratch.ad_value(227), scratch.ad_value(369)))));

        scratch.store_ad(792, &AdValue::exp(AdValue::mul(scratch.ad_value(225), scratch.ad_value(369))));

        scratch.store_ad(755, &AdValue::mul(scratch.ad_value(224), scratch.ad_value(792)));

        scratch.store_ad(758, &AdValue::mul(scratch.ad_value(230), AdValue::exp(AdValue::mul(scratch.ad_value(231), scratch.ad_value(369)))));

        scratch.store_ad(793, &AdValue::exp(AdValue::mul(scratch.ad_value(229), scratch.ad_value(369))));

        scratch.store_ad(757, &AdValue::mul(scratch.ad_value(228), scratch.ad_value(793)));

        scratch.store_ad(794, &AdValue::exp(AdValue::mul(scratch.ad_value(233), scratch.ad_value(369))));

        scratch.store_ad(759, &AdValue::mul(scratch.ad_value(232), scratch.ad_value(794)));

        scratch.store_ad(795, &AdValue::exp(AdValue::mul(scratch.ad_value(236), scratch.ad_value(369))));

        scratch.store_ad(760, &AdValue::mul(scratch.ad_value(235), scratch.ad_value(795)));

        scratch.store_ad(796, &AdValue::mul(AdValue::scale(scratch.ad_value(2075), 2.0), scratch.ad_value(760)));

        scratch.store_ad(797, &AdValue::exp(AdValue::mul(scratch.ad_value(240), scratch.ad_value(369))));

        scratch.store_ad(2079, &AdValue::mul(scratch.ad_value(239), scratch.ad_value(797)));

        scratch.store_ad(763, &AdValue::mul(scratch.ad_value(249), AdValue::exp(AdValue::mul(AdValue::neg(scratch.ad_value(250)), scratch.ad_value(369)))));

        scratch.store_ad(2078, &AdValue::mul(AdValue::scale(scratch.ad_value(275), (4.0 * 1.3806505e-23)), scratch.ad_value(365)));

        scratch.values[2092] = if ((scratch.values[8] != 0.0) && (scratch.values[286] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(764, &AdValue::offset(AdValue::add(scratch.ad_value(281), AdValue::mul(scratch.ad_value(282), scratch.ad_value(367))), scratch.values[29]));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2081, &AdValue::mul(scratch.ad_value(2074), AdValue::offset(AdValue::mul(scratch.ad_value(285), scratch.ad_value(368)), 1.0)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(799, &AdValue::add(AdValue::add(scratch.ad_value(371), scratch.ad_value(283)), AdValue::mul(AdValue::scale(scratch.ad_value(2081), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(284), AdValue::powf(scratch.ad_value(372), (-0.75))), 4e-26)))));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(799, &{
                if (scratch.values[799] > 0.05) {
                    scratch.ad_value(799)
                } else {
                    AdValue::constant(0.05)
                }
            });
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(800, &AdValue::div(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(284), ((2.0 * 1.6021918e-19) * scratch.values[806])), scratch.ad_value(370))), scratch.ad_value(808)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2082, &AdValue::square(scratch.ad_value(800)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2083, &AdValue::ln(scratch.ad_value(2082)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(801, &AdValue::scale(scratch.ad_value(799), 0.95));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(802, &AdValue::mul(AdValue::scale(scratch.ad_value(799), 0.0025), scratch.ad_value(799)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.values[803] = scratch.values[802];
            scratch.node_derivatives[803] = scratch.node_derivatives[802];
            scratch.branch_derivatives[803] = scratch.branch_derivatives[802];
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(804, &AdValue::scale(AdValue::sqrt(scratch.ad_value(803)), 0.5));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(805, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(801), scratch.ad_value(804)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(801), scratch.ad_value(804)), AdValue::sub(scratch.ad_value(801), scratch.ad_value(804))), scratch.ad_value(802)))), 0.5));
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[764] = 0.0;
            scratch.node_derivatives[764] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[764] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2081] = scratch.values[2074];
            scratch.node_derivatives[2081] = scratch.node_derivatives[2074];
            scratch.branch_derivatives[2081] = scratch.branch_derivatives[2074];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[799] = 0.0;
            scratch.node_derivatives[799] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[799] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[800] = 1.0;
            scratch.node_derivatives[800] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[800] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2082] = 1.0;
            scratch.node_derivatives[2082] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2082] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2083] = 0.0;
            scratch.node_derivatives[2083] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2083] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[801] = 0.0;
            scratch.node_derivatives[801] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[801] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[802] = 0.0;
            scratch.node_derivatives[802] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[802] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[803] = 0.0;
            scratch.node_derivatives[803] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[803] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[804] = 0.0;
            scratch.node_derivatives[804] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[804] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[805] = 0.0;
            scratch.node_derivatives[805] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[805] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[939] = 0.0;

        scratch.values[887] = 0.0;

        scratch.values[888] = 0.0;

        scratch.values[889] = 0.0;

        scratch.values[890] = 0.0;

        scratch.values[2093] = if (scratch.values[0] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(849, &AdValue::voltage(ctx, &self.nodes, Some(6), Some(7)));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(850, &AdValue::voltage(ctx, &self.nodes, Some(8), Some(7)));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(851, &AdValue::voltage(ctx, &self.nodes, Some(7), Some(9)));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(854, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(7), Some(11))));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(855, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(8), Some(12))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(849, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(6), Some(7))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(850, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(8), Some(7))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(851, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(7), Some(9))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(854, &AdValue::voltage(ctx, &self.nodes, Some(7), Some(11)));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(855, &AdValue::voltage(ctx, &self.nodes, Some(8), Some(12)));
        }

        scratch.values[856] = scratch.values[849];
        scratch.node_derivatives[856] = scratch.node_derivatives[849];
        scratch.branch_derivatives[856] = scratch.branch_derivatives[849];

        scratch.values[857] = scratch.values[851];
        scratch.node_derivatives[857] = scratch.node_derivatives[851];
        scratch.branch_derivatives[857] = scratch.branch_derivatives[851];

        scratch.store_ad(858, &AdValue::add(scratch.ad_value(850), scratch.ad_value(851)));

        scratch.store_ad(859, &AdValue::sub(scratch.ad_value(849), scratch.ad_value(850)));

        scratch.store_ad(867, &AdValue::scale(AdValue::neg(scratch.ad_value(856)), scratch.values[364]));

        scratch.store_ad(868, &AdValue::scale(AdValue::neg(scratch.ad_value(859)), scratch.values[364]));

        scratch.values[1999] = 1.0;

        scratch.values[2101] = if (scratch.values[850] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2101] != 0.0) {
            scratch.values[1999] = (-1.0);
            scratch.node_derivatives[1999] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1999] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2101] != 0.0) {
            scratch.store_ad(849, &AdValue::sub(scratch.ad_value(849), scratch.ad_value(850)));
        }

        if (scratch.values[2101] != 0.0) {
            scratch.store_ad(851, &AdValue::add(scratch.ad_value(851), scratch.ad_value(850)));
        }

        if (scratch.values[2101] != 0.0) {
            scratch.store_ad(850, &AdValue::neg(scratch.ad_value(850)));
        }

        scratch.store_ad(852, &AdValue::add(scratch.ad_value(850), scratch.ad_value(851)));

        scratch.store_ad(860, &AdValue::div(AdValue::square(scratch.ad_value(850)), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(850)), 0.01)), 0.1)));

        scratch.store_ad(2086, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sub(scratch.ad_value(852), scratch.ad_value(851))), scratch.ad_value(778)))), 0.5), scratch.ad_value(776)));

        scratch.store_ad(869, &AdValue::add(AdValue::sub(scratch.ad_value(851), AdValue::scale(AdValue::sub(scratch.ad_value(2086), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2086)), scratch.ad_value(777)))), 0.5)), scratch.ad_value(780)));

        scratch.values[870] = scratch.values[869];
        scratch.node_derivatives[870] = scratch.node_derivatives[869];
        scratch.branch_derivatives[870] = scratch.branch_derivatives[869];

        scratch.values[2222] = if ((scratch.values[7] != 0.0) && (scratch.values[200] != 1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(871, &AdValue::add(scratch.ad_value(869), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(872, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(871), scratch.ad_value(767))), scratch.ad_value(775)));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::div(AdValue::scale(AdValue::sub(scratch.ad_value(872), scratch.ad_value(782)), 2.0), scratch.ad_value(783)), (-1.0)));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(873, &AdValue::sub(scratch.ad_value(872), AdValue::mul(AdValue::mul(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(200)), 0.25), scratch.ad_value(783)), AdValue::add(scratch.ad_value(2086), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2086)), 0.4804530139182))))));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(874, &AdValue::add(AdValue::square(scratch.ad_value(873)), AdValue::mul(AdValue::scale(scratch.ad_value(775), 2.0), scratch.ad_value(873))));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(869, &AdValue::sub(scratch.ad_value(874), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));
        }

        scratch.values[2102] = scratch.values[767];
        scratch.node_derivatives[2102] = scratch.node_derivatives[767];
        scratch.branch_derivatives[2102] = scratch.branch_derivatives[767];

        scratch.values[2103] = scratch.values[768];
        scratch.node_derivatives[2103] = scratch.node_derivatives[768];
        scratch.branch_derivatives[2103] = scratch.branch_derivatives[768];

        scratch.values[2104] = scratch.values[869];
        scratch.node_derivatives[2104] = scratch.node_derivatives[869];
        scratch.branch_derivatives[2104] = scratch.branch_derivatives[869];

        scratch.values[2167] = 0.0;

        scratch.values[2183] = 1.0;

        scratch.values[2182] = 0.0;

        scratch.values[2178] = 0.0;

        scratch.values[2155] = 0.0;

        scratch.values[2176] = 0.0;

        scratch.values[2177] = 0.0;

        scratch.values[2190] = 1.0;

        scratch.values[2181] = 0.0;

        scratch.values[2165] = 1.0;

        scratch.values[2186] = 1.0;

        scratch.values[2187] = 1.0;

        scratch.values[2212] = 0.0;

        scratch.values[2117] = 0.0;

        scratch.values[2166] = 0.0;

        scratch.values[2134] = 0.0;

        scratch.values[2129] = 0.0;

        scratch.values[2133] = 1.0;

        scratch.values[2184] = 1.0;

        scratch.values[2147] = 0.0;

        scratch.values[2136] = 0.0;

        scratch.values[2164] = 0.0;

        scratch.store_ad(861, &AdValue::add(scratch.ad_value(849), scratch.ad_value(2104)));

        scratch.store_ad(862, &AdValue::sub(scratch.ad_value(861), scratch.ad_value(751)));

        scratch.store_ad(2105, &AdValue::add(scratch.ad_value(2104), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));

        scratch.values[2223] = if (scratch.values[217] < 1e-10) { 1.0 } else { 0.0 };

        if (scratch.values[2223] != 0.0) {
            scratch.values[863] = scratch.values[860];
            scratch.node_derivatives[863] = scratch.node_derivatives[860];
            scratch.branch_derivatives[863] = scratch.branch_derivatives[860];
        }

        if (!(scratch.values[2223] != 0.0)) {
            scratch.store_ad(863, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(217), scratch.ad_value(860)), 1.0)), (-1.0)), 2.0), scratch.ad_value(217)));
        }

        scratch.store_ad(864, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(863)), AdValue::offset(AdValue::mul(scratch.ad_value(218), scratch.ad_value(2105)), 1.0)));

        scratch.store_ad(2000, &AdValue::mul(AdValue::mul(scratch.ad_value(219), AdValue::offset(AdValue::mul(scratch.ad_value(221), scratch.ad_value(860)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(220), scratch.ad_value(2105)), 1.0)));

        scratch.store_ad(862, &AdValue::add(scratch.ad_value(862), scratch.ad_value(864)));

        scratch.values[2224] = if (scratch.values[205] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2224] != 0.0) {
            scratch.store_ad(2107, &AdValue::mul(AdValue::scale(scratch.ad_value(205), 0.5), AdValue::add(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203))), AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203)))), scratch.ad_value(204))))));
        }

        if (scratch.values[2224] != 0.0) {
            scratch.store_ad(2108, &AdValue::mul(scratch.ad_value(2103), AdValue::sqrt(AdValue::offset(scratch.ad_value(2107), 1.0))));
        }

        if (!(scratch.values[2224] != 0.0)) {
            scratch.values[2108] = scratch.values[2103];
            scratch.node_derivatives[2108] = scratch.node_derivatives[2103];
            scratch.branch_derivatives[2108] = scratch.branch_derivatives[2103];
        }

        scratch.store_ad(2109, &AdValue::square(scratch.ad_value(2108)));

        scratch.store_ad(2110, &AdValue::div_from_scalar(1.0, scratch.ad_value(2109)));

        scratch.values[2006] = 1.0;

        scratch.values[2225] = if (scratch.values[210] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2001, &AdValue::mul(AdValue::scale(scratch.ad_value(862), 2.0), scratch.ad_value(370)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2087, &AdValue::add(scratch.ad_value(2109), scratch.ad_value(2001)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2088, &AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001)), AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001))), 5.0))), 0.5));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2002, &AdValue::scale(AdValue::sub(scratch.ad_value(2087), AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(scratch.ad_value(2088)))), 0.5));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2003, &AdValue::mul(scratch.ad_value(2102), scratch.ad_value(370)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2004, &AdValue::mul(scratch.ad_value(2105), scratch.ad_value(370)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2087, &AdValue::offset(AdValue::add(scratch.ad_value(2003), scratch.ad_value(2004)), 2.0));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2005, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2002), scratch.ad_value(2087)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2002), scratch.ad_value(2087)), AdValue::sub(scratch.ad_value(2002), scratch.ad_value(2087))), 5.0))), 0.5));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(753), AdValue::sub(scratch.ad_value(2005), AdValue::mul(AdValue::offset(scratch.ad_value(211), 1.0), AdValue::add(AdValue::scale(scratch.ad_value(2003), 0.5), scratch.ad_value(2004))))));
        }

        scratch.values[2226] = if (scratch.values[2088] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((scratch.values[2225] != 0.0) && (scratch.values[2226] != 0.0)) {
            scratch.store_ad(2006, &AdValue::exp(scratch.ad_value(2088)));
        }

        if ((scratch.values[2225] != 0.0) && (!(scratch.values[2226] != 0.0))) {
            scratch.store_ad(2006, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.store_ad(2007, &AdValue::offset(AdValue::mul(scratch.ad_value(752), scratch.ad_value(2006)), 1.0));

        scratch.store_ad(2008, &AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(2007)), AdValue::offset(scratch.ad_value(2000), 1.0)));

        scratch.store_ad(2009, &AdValue::div_from_scalar(1.0, scratch.ad_value(2008)));

        scratch.store_ad(2106, &AdValue::mul(scratch.ad_value(862), scratch.ad_value(2009)));

        scratch.store_ad(2111, &AdValue::offset(AdValue::scale(scratch.ad_value(2108), 0.7071067811865475), 1.0));

        scratch.store_ad(2112, &AdValue::div_from_scalar(1.0, scratch.ad_value(2111)));

        scratch.store_ad(2113, &AdValue::mul(scratch.ad_value(2104), scratch.ad_value(2009)));

        scratch.store_ad(2114, &AdValue::add(AdValue::mul(scratch.ad_value(2102), scratch.ad_value(2009)), scratch.ad_value(2113)));

        scratch.values[2227] = if (scratch.values[2114] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (scratch.values[2227] != 0.0) {
            scratch.store_ad(2115, &AdValue::exp(AdValue::neg(scratch.ad_value(2114))));
        }

        if (!(scratch.values[2227] != 0.0)) {
            scratch.store_ad(2115, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2114), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2114), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2114), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.store_ad(2116, &AdValue::scale(scratch.ad_value(2111), 1e-5));

        scratch.values[2228] = if (((scratch.values[2106]) as f64).abs() <= scratch.values[2116]) { 1.0 } else { 0.0 };

        if (scratch.values[2228] != 0.0) {
            scratch.store_ad(2192, &AdValue::scale(AdValue::square(scratch.ad_value(2112)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if (scratch.values[2228] != 0.0) {
            scratch.store_ad(2117, &AdValue::mul(AdValue::mul(scratch.ad_value(2106), scratch.ad_value(2112)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2106), AdValue::sub_from_scalar(1.0, scratch.ad_value(2115))), scratch.ad_value(2108)), scratch.ad_value(2192)), 1.0)));
        }

        scratch.values[2229] = if (scratch.values[2106] < (-scratch.values[2116])) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2194, &AdValue::neg(scratch.ad_value(2106)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2195, &AdValue::scale(AdValue::mul(scratch.ad_value(2194), scratch.ad_value(2112)), 1.25));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2196, &AdValue::scale(AdValue::sub(AdValue::offset(scratch.ad_value(2195), 10.0), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2195), (-6.0)), AdValue::offset(scratch.ad_value(2195), (-6.0))), 64.0))), 0.5));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2194), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2197, &AdValue::add(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::offset(scratch.ad_value(2196), 1.0))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2198, &AdValue::sub(AdValue::scale(scratch.ad_value(2191), 2.0), scratch.ad_value(2109)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2199, &AdValue::sub(AdValue::ln(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2110))), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2197), scratch.ad_value(2198)));
        }

    }

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2199), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.5), scratch.ad_value(2197)))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2200, &AdValue::add(scratch.ad_value(2196), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(962)), scratch.ad_value(2199)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2199)), scratch.ad_value(2199)), scratch.ad_value(2198)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.3333333333333333), scratch.ad_value(2197)))))));
        }

        scratch.values[2230] = if (scratch.values[2200] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) && (scratch.values[2230] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(scratch.ad_value(2200)));
        }

        if (((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) && (!(scratch.values[2230] != 0.0))) {
            scratch.store_ad(2201, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2200), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2200), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2200), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1.0, scratch.ad_value(2201)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2200)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2200)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2200), scratch.ad_value(2191)), scratch.ad_value(2191)), 4.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2191), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2191)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2194), scratch.ad_value(2200)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2192, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2202)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2206, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(2201), (-1.0)), scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), AdValue::sub_from_scalar(1.0, scratch.ad_value(2204)))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2207, &AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::add(AdValue::add(AdValue::offset(AdValue::sub(scratch.ad_value(2201), scratch.ad_value(2200)), (-1.0)), scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), AdValue::sub(AdValue::offset(scratch.ad_value(2200), (-1.0)), scratch.ad_value(2203)))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2201), scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2205))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub(AdValue::square(scratch.ad_value(2206)), AdValue::scale(AdValue::mul(scratch.ad_value(2207), scratch.ad_value(2191)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2117, &AdValue::sub(AdValue::neg(scratch.ad_value(2200)), AdValue::scale(AdValue::div(scratch.ad_value(2207), AdValue::add(scratch.ad_value(2206), AdValue::sqrt(scratch.ad_value(2191)))), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2208, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(2108), 0.7324648775608221), 1.25)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2209, &AdValue::mul(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(2111), 1.25), scratch.ad_value(2208)), (-1.0)), scratch.ad_value(2208)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2210, &AdValue::mul(AdValue::mul(scratch.ad_value(2106), scratch.ad_value(2112)), AdValue::offset(AdValue::mul(scratch.ad_value(2209), scratch.ad_value(2106)), 1.0)));
        }

        scratch.values[2231] = if ((-scratch.values[2210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2231] != 0.0)) {
            scratch.store_ad(2191, &AdValue::exp(AdValue::neg(scratch.ad_value(2210))));
        }

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2210))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2210))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2210))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2211, &AdValue::sub_from_scalar(1.0, scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2212, &AdValue::sub(AdValue::add(scratch.ad_value(2106), AdValue::scale(scratch.ad_value(2109), 0.5)), AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::sub(AdValue::add(scratch.ad_value(2106), AdValue::scale(scratch.ad_value(2109), 0.25)), scratch.ad_value(2211))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2213, &AdValue::offset(scratch.ad_value(2114), 3.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2196, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2213), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2213)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2192, &AdValue::exp(AdValue::neg(scratch.ad_value(2196))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2193, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2196)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2196)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2196), scratch.ad_value(2193)), scratch.ad_value(2193)), 4.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2193), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2193)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2197, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2192), scratch.ad_value(2196)), (-1.0)), AdValue::mul(scratch.ad_value(2115), AdValue::add(AdValue::offset(scratch.ad_value(2196), 1.0), scratch.ad_value(2203))))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2214, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2109), AdValue::sub(scratch.ad_value(2192), AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2205)))), 0.5)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2198, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2199, &AdValue::add(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2196)), AdValue::ln(AdValue::div(scratch.ad_value(2197), scratch.ad_value(2109)))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2197), scratch.ad_value(2198)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2199), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.5), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            let assign42000_ad_e54283: AdValue = AdValue::add(scratch.ad_value(2196), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(962)), scratch.ad_value(2199)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2199)), scratch.ad_value(2199)), scratch.ad_value(2198)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214)))))));
            scratch.store_ad(2215, &assign42000_ad_e54283);
        }

        scratch.values[2232] = if (scratch.values[2215] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(scratch.ad_value(2215)));
        }

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1.0, scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2201, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2201)));
        }

        scratch.values[2233] = if (scratch.values[2215] > (scratch.values[2114] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(AdValue::sub(scratch.ad_value(2215), scratch.ad_value(2114))));
        }

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div(scratch.ad_value(2115), scratch.ad_value(2201)));
        }

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2201, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2215)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2215)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2215), scratch.ad_value(2191)), scratch.ad_value(2191)), 4.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2191), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2191)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2215)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2206, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2202)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2115), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2207, &AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2215)), (-1.0)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2115), AdValue::add(AdValue::offset(scratch.ad_value(2215), 1.0), scratch.ad_value(2203)))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2205))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(AdValue::square(scratch.ad_value(2206)), AdValue::scale(AdValue::mul(scratch.ad_value(2207), scratch.ad_value(2191)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2117, &AdValue::add(scratch.ad_value(2215), AdValue::scale(AdValue::div(scratch.ad_value(2207), AdValue::add(scratch.ad_value(2206), AdValue::sqrt(scratch.ad_value(2191)))), 2.0)));
        }

        scratch.values[2151] = scratch.values[2117];
        scratch.node_derivatives[2151] = scratch.node_derivatives[2117];
        scratch.branch_derivatives[2151] = scratch.branch_derivatives[2117];

        scratch.values[2159] = scratch.values[2117];
        scratch.node_derivatives[2159] = scratch.node_derivatives[2117];
        scratch.branch_derivatives[2159] = scratch.branch_derivatives[2117];

        scratch.values[2152] = 0.0;

        scratch.store_ad(865, &AdValue::scale(scratch.ad_value(2008), 3.912023005));

        scratch.values[2234] = if (scratch.values[2106] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2234] != 0.0) {
            scratch.values[2127] = 0.0;
            scratch.node_derivatives[2127] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2127] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2234] != 0.0) {
            scratch.store_ad(2164, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2117)));
        }

        if (scratch.values[2234] != 0.0) {
            scratch.store_ad(2188, &AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2008)));
        }

        if (scratch.values[2234] != 0.0) {
            scratch.values[2180] = scratch.values[2188];
            scratch.node_derivatives[2180] = scratch.node_derivatives[2188];
            scratch.branch_derivatives[2180] = scratch.branch_derivatives[2188];
        }

        if (scratch.values[2234] != 0.0) {
            scratch.values[866] = scratch.values[865];
            scratch.node_derivatives[866] = scratch.node_derivatives[865];
            scratch.branch_derivatives[866] = scratch.branch_derivatives[865];
        }

        if (scratch.values[2234] != 0.0) {
            scratch.values[2146] = scratch.values[850];
            scratch.node_derivatives[2146] = scratch.node_derivatives[850];
            scratch.branch_derivatives[2146] = scratch.branch_derivatives[850];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2118] = 0.0;
            scratch.node_derivatives[2118] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2118] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2117)), 2.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2119, &AdValue::mul(AdValue::square(scratch.ad_value(2117)), scratch.ad_value(2086)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2120, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(2086)), scratch.ad_value(2086)), 4.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2121, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2086), 8.0), AdValue::scale(scratch.ad_value(2119), 12.0)), scratch.ad_value(2086)), scratch.ad_value(2086)));
        }

        scratch.values[2235] = if (scratch.values[2117] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2118, &AdValue::exp(scratch.ad_value(2117)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2122, &AdValue::div_from_scalar(1.0, scratch.ad_value(2118)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2118, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2118)));
        }

        scratch.values[2236] = if (scratch.values[2117] > (scratch.values[2114] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2236] != 0.0)) {
            scratch.store_ad(2118, &AdValue::exp(AdValue::sub(scratch.ad_value(2117), scratch.ad_value(2114))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2236] != 0.0)) {
            scratch.store_ad(2122, &AdValue::div(scratch.ad_value(2115), scratch.ad_value(2118)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2236] != 0.0))) {
            scratch.store_ad(2118, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2117)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2117)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2117)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2236] != 0.0))) {
            scratch.store_ad(2122, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2117), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2117), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2117), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2123, &AdValue::sub(scratch.ad_value(2118), AdValue::mul(scratch.ad_value(2115), AdValue::add(AdValue::offset(scratch.ad_value(2117), 1.0), scratch.ad_value(2119)))));
        }

        scratch.values[2237] = if (scratch.values[2117] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2124, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2117)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2117), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2117), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2123, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2117)), scratch.ad_value(2117)), scratch.ad_value(2117)), AdValue::offset(AdValue::scale(scratch.ad_value(2117), 1.75), 1.0)), 0.16666666666666666));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2117), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2117), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2166, &AdValue::scale(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(2086)), 0.7071067811865475));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2167, &AdValue::offset(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2108), 0.7071067811865475), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2117), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2117)), 0.16666666666666666))), scratch.ad_value(2086)), 1.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2124, &AdValue::add(AdValue::offset(scratch.ad_value(2117), (-1.0)), scratch.ad_value(2122)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2166, &AdValue::sqrt(scratch.ad_value(2124)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2167, &AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2108), AdValue::sub_from_scalar(1.0, scratch.ad_value(2122))), scratch.ad_value(2166)), 0.5), 1.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2160] = scratch.values[2122];
            scratch.node_derivatives[2160] = scratch.node_derivatives[2122];
            scratch.branch_derivatives[2160] = scratch.branch_derivatives[2122];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2157] = scratch.values[2160];
            scratch.node_derivatives[2157] = scratch.node_derivatives[2160];
            scratch.branch_derivatives[2157] = scratch.branch_derivatives[2160];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2162] = scratch.values[2123];
            scratch.node_derivatives[2162] = scratch.node_derivatives[2123];
            scratch.branch_derivatives[2162] = scratch.branch_derivatives[2123];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2158] = scratch.values[2162];
            scratch.node_derivatives[2158] = scratch.node_derivatives[2162];
            scratch.branch_derivatives[2158] = scratch.branch_derivatives[2162];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2125, &AdValue::div(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(759), 0.2), scratch.ad_value(2105)), 1.0), AdValue::offset(AdValue::mul(scratch.ad_value(759), scratch.ad_value(2105)), 1.0)));
        }

        scratch.values[2238] = if (scratch.values[2123] > 1e-100) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2126, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2124), scratch.ad_value(2123)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2127, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2123)), scratch.ad_value(2008)), AdValue::add(scratch.ad_value(2126), AdValue::mul(scratch.ad_value(2108), scratch.ad_value(2166)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2128, &AdValue::mul(AdValue::mul(scratch.ad_value(2166), scratch.ad_value(2108)), scratch.ad_value(2008)));
        }

        scratch.values[2239] = if (scratch.values[237] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2129, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(237), scratch.ad_value(2105)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2129, &AdValue::offset(AdValue::mul(scratch.ad_value(237), scratch.ad_value(2105)), 1.0));
        }

        scratch.values[2240] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2240] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2127))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2240] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2127)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2130, &AdValue::mul(scratch.ad_value(796), AdValue::mul(AdValue::mul(scratch.ad_value(2129), scratch.ad_value(2086)), scratch.ad_value(2127))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2131, &AdValue::mul(scratch.ad_value(813), AdValue::add(scratch.ad_value(2128), AdValue::mul(scratch.ad_value(814), scratch.ad_value(2127)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2087, &AdValue::ln(AdValue::div(scratch.ad_value(2124), AdValue::offset(AdValue::add(scratch.ad_value(2124), scratch.ad_value(2123)), 1e-14))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2132, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2131), scratch.ad_value(755)), scratch.ad_value(756)), AdValue::mul(scratch.ad_value(757), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(758), 0.5), scratch.ad_value(2087))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2133, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2132), 1.0), scratch.ad_value(2130)), scratch.ad_value(2125)));
        }

        scratch.values[2241] = if (scratch.values[241] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2134, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(241), scratch.ad_value(2105)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2241] != 0.0))) {
            scratch.store_ad(2134, &AdValue::offset(AdValue::mul(scratch.ad_value(241), scratch.ad_value(2105)), 1.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(2127), scratch.ad_value(2134)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2135, &AdValue::scale(AdValue::div(scratch.ad_value(2088), AdValue::offset(scratch.ad_value(2088), 100.0)), 100.0));
        }

        scratch.values[2242] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2242] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2242] != 0.0))) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)), 1.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2136, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(2086), scratch.ad_value(2133))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2137, &AdValue::add(AdValue::div(scratch.ad_value(2127), scratch.ad_value(2167)), scratch.ad_value(2008)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2138, &AdValue::scale(AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2137)), 0.7071067811865475));
        }

        scratch.values[2243] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2243] != 0.0)) {
            scratch.store_ad(2138, &AdValue::div(scratch.ad_value(2138), AdValue::sqrt(AdValue::offset(scratch.ad_value(2138), 1.0))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2139, &AdValue::div_from_scalar(2.0, AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2138), 4.0), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(2139), scratch.ad_value(2138)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2140, &AdValue::mul(AdValue::mul(scratch.ad_value(2137), scratch.ad_value(2139)), AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2087), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2087), scratch.ad_value(2139)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2087)), scratch.ad_value(2139)), 4.0), 1.0)), 0.86), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2141, &AdValue::add(scratch.ad_value(2126), AdValue::scale(scratch.ad_value(2109), 0.5)));
        }

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2142, &AdValue::scale(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2123)), scratch.ad_value(2008)), AdValue::add(scratch.ad_value(2141), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2141)), AdValue::scale(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2123)), 0.98))))), 0.98));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2143, &AdValue::add(scratch.ad_value(2140), scratch.ad_value(2142)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2144, &AdValue::scale(AdValue::mul(scratch.ad_value(2140), scratch.ad_value(2142)), 2.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2145, &AdValue::div(scratch.ad_value(2144), AdValue::add(scratch.ad_value(2143), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2143)), AdValue::scale(scratch.ad_value(2144), 1.98))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(866, &AdValue::sub(scratch.ad_value(2145), AdValue::mul(scratch.ad_value(2008), AdValue::ln(AdValue::offset(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2145), AdValue::sub(scratch.ad_value(2145), AdValue::mul(AdValue::scale(scratch.ad_value(2141), 2.0), scratch.ad_value(2008)))), scratch.ad_value(2110)), AdValue::mul(AdValue::square(scratch.ad_value(2008)), scratch.ad_value(2123))), 1.0)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2238] != 0.0))) {
            scratch.values[866] = scratch.values[865];
            scratch.node_derivatives[866] = scratch.node_derivatives[865];
            scratch.branch_derivatives[866] = scratch.branch_derivatives[865];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::pow(AdValue::div(scratch.ad_value(850), scratch.ad_value(866)), scratch.ad_value(243)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2146, &AdValue::mul(scratch.ad_value(850), AdValue::pow(AdValue::offset(scratch.ad_value(2086), 1.0), AdValue::neg(scratch.ad_value(816)))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2147, &AdValue::mul(scratch.ad_value(2146), scratch.ad_value(2009)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2148, &AdValue::add(scratch.ad_value(2114), scratch.ad_value(2147)));
        }

        scratch.values[2244] = if (scratch.values[2147] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2244] != 0.0)) {
            scratch.store_ad(2149, &AdValue::exp(AdValue::neg(scratch.ad_value(2147))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2244] != 0.0))) {
            scratch.store_ad(2149, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2147), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2147), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2147), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2149)));
        }

        scratch.values[2245] = if (((scratch.values[2106]) as f64).abs() <= scratch.values[2116]) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2245] != 0.0)) {
            scratch.store_ad(2192, &AdValue::scale(AdValue::square(scratch.ad_value(2112)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2245] != 0.0)) {
            scratch.store_ad(2151, &AdValue::mul(AdValue::mul(scratch.ad_value(2106), scratch.ad_value(2112)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2106), AdValue::sub_from_scalar(1.0, scratch.ad_value(2150))), scratch.ad_value(2108)), scratch.ad_value(2192)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2213, &AdValue::offset(scratch.ad_value(2148), 3.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2196, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2213), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2213)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2192, &AdValue::exp(AdValue::neg(scratch.ad_value(2196))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2193, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2196)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2196)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2196), scratch.ad_value(2193)), scratch.ad_value(2193)), 4.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2193), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2193)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2197, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2192), scratch.ad_value(2196)), (-1.0)), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2196), 1.0), scratch.ad_value(2203))))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2214, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2109), AdValue::sub(scratch.ad_value(2192), AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2205)))), 0.5)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2198, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2150), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2199, &AdValue::add(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2196)), AdValue::ln(AdValue::div(scratch.ad_value(2197), scratch.ad_value(2109)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2197), scratch.ad_value(2198)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2199), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.5), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            let assign43230_ad_e55896: AdValue = AdValue::add(scratch.ad_value(2196), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(962)), scratch.ad_value(2199)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2199)), scratch.ad_value(2199)), scratch.ad_value(2198)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214)))))));
            scratch.store_ad(2215, &assign43230_ad_e55896);
        }

        scratch.values[2246] = if (scratch.values[2215] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (scratch.values[2246] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(scratch.ad_value(2215)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (scratch.values[2246] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1.0, scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (scratch.values[2246] != 0.0)) {
            scratch.store_ad(2201, &AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2201)));
        }

        scratch.values[2247] = if (scratch.values[2215] > (scratch.values[2148] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (scratch.values[2247] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(AdValue::sub(scratch.ad_value(2215), scratch.ad_value(2148))));
        }

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (scratch.values[2247] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div(scratch.ad_value(2150), scratch.ad_value(2201)));
        }

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (!(scratch.values[2247] != 0.0))) {
            scratch.store_ad(2201, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (!(scratch.values[2247] != 0.0))) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2215)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2215)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2215), scratch.ad_value(2191)), scratch.ad_value(2191)), 4.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2191), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2191)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2215)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2206, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2202)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2150), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2207, &AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2215)), (-1.0)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2215), 1.0), scratch.ad_value(2203)))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2205))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(AdValue::square(scratch.ad_value(2206)), AdValue::scale(AdValue::mul(scratch.ad_value(2207), scratch.ad_value(2191)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2151, &AdValue::add(scratch.ad_value(2215), AdValue::scale(AdValue::div(scratch.ad_value(2207), AdValue::add(scratch.ad_value(2206), AdValue::sqrt(scratch.ad_value(2191)))), 2.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2152, &AdValue::sub(scratch.ad_value(2151), scratch.ad_value(2117)));
        }

        scratch.values[2248] = if (scratch.values[2152] < 1e-10) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2153, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2117)), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2122)), AdValue::mul(scratch.ad_value(2118), scratch.ad_value(2149))), AdValue::mul(scratch.ad_value(2150), AdValue::offset(scratch.ad_value(2120), 1.0))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2154, &AdValue::mul(AdValue::mul(scratch.ad_value(2109), AdValue::sub_from_scalar(1.0, scratch.ad_value(2149))), scratch.ad_value(2123)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2122), AdValue::mul(scratch.ad_value(2118), scratch.ad_value(2149))), AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2121))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub(AdValue::square(scratch.ad_value(2153)), AdValue::scale(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2154)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2152, &AdValue::scale(AdValue::div(scratch.ad_value(2154), AdValue::add(scratch.ad_value(2153), AdValue::sqrt(scratch.ad_value(2086)))), 2.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2151, &AdValue::add(scratch.ad_value(2117), scratch.ad_value(2152)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2155, &AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2008)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2156, &AdValue::div(AdValue::square(scratch.ad_value(2151)), AdValue::offset(AdValue::square(scratch.ad_value(2151)), 2.0)));
        }

        scratch.values[2249] = if (scratch.values[2151] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2157, &AdValue::exp(AdValue::neg(scratch.ad_value(2151))));
        }

        scratch.values[2250] = if (scratch.values[2151] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2249] != 0.0)) && (scratch.values[2250] != 0.0)) {
            scratch.store_ad(2158, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2150), 0.16666666666666666), scratch.ad_value(2151)), scratch.ad_value(2151)), scratch.ad_value(2151)), AdValue::offset(AdValue::scale(scratch.ad_value(2151), 1.75), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2249] != 0.0)) && (!(scratch.values[2250] != 0.0))) {
            scratch.store_ad(2158, &AdValue::mul(scratch.ad_value(2150), AdValue::sub(AdValue::offset(AdValue::sub(AdValue::div_from_scalar(1.0, scratch.ad_value(2157)), scratch.ad_value(2151)), (-1.0)), scratch.ad_value(2156))));
        }

        scratch.values[2251] = if (scratch.values[2151] > (scratch.values[2148] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(AdValue::sub(scratch.ad_value(2151), scratch.ad_value(2148))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(2157, &AdValue::div(scratch.ad_value(2150), scratch.ad_value(2086)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(2158, &AdValue::sub(scratch.ad_value(2086), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2151), 1.0), scratch.ad_value(2156)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (!(scratch.values[2251] != 0.0))) {
            scratch.store_ad(2157, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2151), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2151), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2151), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (!(scratch.values[2251] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2151)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2151)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2151)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (!(scratch.values[2251] != 0.0))) {
            scratch.store_ad(2158, &AdValue::sub(scratch.ad_value(2086), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2151), 1.0), scratch.ad_value(2156)))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2159, &AdValue::scale(AdValue::add(scratch.ad_value(2117), scratch.ad_value(2151)), 0.5));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2160] = 0.0;
            scratch.node_derivatives[2160] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2160] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(2157), scratch.ad_value(2122)));
        }

        scratch.values[2252] = if (scratch.values[2086] > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2252] != 0.0)) {
            scratch.store_ad(2160, &AdValue::sqrt(scratch.ad_value(2086)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2161, &AdValue::scale(AdValue::add(scratch.ad_value(2123), scratch.ad_value(2158)), 0.5));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2162, &AdValue::add(scratch.ad_value(2161), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2152)), AdValue::sub(scratch.ad_value(2160), AdValue::scale(scratch.ad_value(2110), 2.0))), 0.125)));
        }

        scratch.values[2253] = if (scratch.values[2159] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2163, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2159)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2159), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2159), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2164, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2162), scratch.ad_value(2163)))));
        }

        scratch.values[2254] = if (scratch.values[769] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) && (scratch.values[2254] != 0.0)) {
            scratch.store_ad(2165, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2164)), 1.0))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2159), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2159), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2166, &AdValue::scale(AdValue::mul(scratch.ad_value(2159), scratch.ad_value(2086)), 0.7071067811865475));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2167, &AdValue::add(scratch.ad_value(2165), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2108), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2159), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2159)), 0.16666666666666666))), scratch.ad_value(2086)), 0.7071067811865475)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2163, &AdValue::add(AdValue::offset(scratch.ad_value(2159), (-1.0)), scratch.ad_value(2160)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2164, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2162), scratch.ad_value(2163)))));
        }

        scratch.values[2255] = if (scratch.values[769] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2168, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2160)), AdValue::scale(AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2110)), 2.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2165, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2164)), 1.0))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div(scratch.ad_value(2165), AdValue::offset(scratch.ad_value(2165), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2169, &AdValue::mul(scratch.ad_value(769), AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2086)), scratch.ad_value(2109)), scratch.ad_value(2162))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2170, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2164), scratch.ad_value(2169)), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2160)), scratch.ad_value(2162)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2171, &AdValue::mul(scratch.ad_value(2169), AdValue::sub(scratch.ad_value(2169), AdValue::scale(scratch.ad_value(2164), 2.0))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2172, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2109), AdValue::add(scratch.ad_value(2160), scratch.ad_value(2162))), 0.5)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2173, &AdValue::div(AdValue::mul(scratch.ad_value(2171), scratch.ad_value(2170)), AdValue::sub(AdValue::square(scratch.ad_value(2170)), AdValue::mul(scratch.ad_value(2172), scratch.ad_value(2171)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2159, &AdValue::add(scratch.ad_value(2159), scratch.ad_value(2173)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2174, &AdValue::exp(scratch.ad_value(2173)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2160, &AdValue::div(scratch.ad_value(2160), scratch.ad_value(2174)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2162, &AdValue::mul(scratch.ad_value(2162), scratch.ad_value(2174)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2163, &AdValue::add(AdValue::offset(scratch.ad_value(2159), (-1.0)), scratch.ad_value(2160)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2164, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2162), scratch.ad_value(2163)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2175, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2160)), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2165)), scratch.ad_value(2110)), 2.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2152, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2174)), AdValue::add(scratch.ad_value(2168), scratch.ad_value(2161))), AdValue::add(scratch.ad_value(2175), AdValue::mul(scratch.ad_value(2174), scratch.ad_value(2161)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2155, &AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2008)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2166, &AdValue::sqrt(scratch.ad_value(2163)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2167, &AdValue::add(scratch.ad_value(2165), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2108), AdValue::sub_from_scalar(1.0, scratch.ad_value(2160))), scratch.ad_value(2166)), 0.5)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2176, &AdValue::mul(scratch.ad_value(2008), AdValue::div(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2162)), AdValue::add(scratch.ad_value(2164), AdValue::mul(scratch.ad_value(2108), scratch.ad_value(2166))))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2177, &AdValue::add(scratch.ad_value(2176), AdValue::mul(scratch.ad_value(2008), scratch.ad_value(2167))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2178, &AdValue::mul(AdValue::mul(scratch.ad_value(2166), scratch.ad_value(2108)), scratch.ad_value(2008)));
        }

        scratch.values[2256] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2256] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2176))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2256] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2176)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2130, &AdValue::mul(scratch.ad_value(796), AdValue::mul(AdValue::mul(scratch.ad_value(2129), scratch.ad_value(2086)), scratch.ad_value(2176))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2179, &AdValue::add(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(814), scratch.ad_value(2176))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2180, &AdValue::add(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(815), scratch.ad_value(2176))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2131, &AdValue::mul(scratch.ad_value(813), scratch.ad_value(2179)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2087, &AdValue::ln(AdValue::div(scratch.ad_value(2163), AdValue::offset(AdValue::add(scratch.ad_value(2163), scratch.ad_value(2162)), 1e-14))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2132, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2131), scratch.ad_value(755)), scratch.ad_value(756)), AdValue::mul(scratch.ad_value(757), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(758), 0.5), scratch.ad_value(2087))))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2133, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2132), 1.0), scratch.ad_value(2130)), scratch.ad_value(2125)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2181, &AdValue::ln(AdValue::div(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(850), scratch.ad_value(2155)), scratch.ad_value(817)), 1.0), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2146), scratch.ad_value(2155)), scratch.ad_value(817)), 1.0))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2182, &AdValue::mul(scratch.ad_value(244), scratch.ad_value(2181)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2183, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(scratch.ad_value(2182), 1.0), AdValue::square(scratch.ad_value(2182)))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(2176), scratch.ad_value(2134)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2135, &AdValue::scale(AdValue::div(scratch.ad_value(2088), AdValue::offset(scratch.ad_value(2088), 100.0)), 100.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2184, &AdValue::mul(scratch.ad_value(2133), scratch.ad_value(2183)));
        }

        scratch.values[2257] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2257] != 0.0))) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)), 1.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2136, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(2086), scratch.ad_value(2184))));
        }

    }
}
