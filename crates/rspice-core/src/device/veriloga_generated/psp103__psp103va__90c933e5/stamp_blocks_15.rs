#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2077, &AdValue::ln(AdValue::div(scratch.ad_value(2351), AdValue::offset(AdValue::add(scratch.ad_value(2351), scratch.ad_value(2350)), 1e-14))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2320, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2319), scratch.ad_value(748)), scratch.ad_value(749)), AdValue::mul(scratch.ad_value(750), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(751), 0.5), scratch.ad_value(2077))))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2321, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2320), 1.0), scratch.ad_value(2318)), scratch.ad_value(2313)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2369, &AdValue::ln(AdValue::div(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(854), scratch.ad_value(2343)), scratch.ad_value(821)), 1.0), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2334), scratch.ad_value(2343)), scratch.ad_value(821)), 1.0))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2370, &AdValue::mul(scratch.ad_value(241), scratch.ad_value(2369)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2371, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(scratch.ad_value(2370), 1.0), AdValue::square(scratch.ad_value(2370)))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(2364), scratch.ad_value(2322)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2323, &AdValue::scale(AdValue::div(scratch.ad_value(2078), AdValue::offset(scratch.ad_value(2078), 100.0)), 100.0));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2372, &AdValue::mul(scratch.ad_value(2321), scratch.ad_value(2371)));
        }

        scratch.values[2440] = if (scratch.values[239] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2440] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(239), scratch.ad_value(2323)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2440] != 0.0))) {
            scratch.store_ad(2076, &AdValue::offset(AdValue::mul(scratch.ad_value(239), scratch.ad_value(2323)), 1.0));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2324, &AdValue::mul(scratch.ad_value(764), AdValue::div(scratch.ad_value(2076), scratch.ad_value(2372))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2373, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2324)), scratch.ad_value(2343)), scratch.ad_value(2343)));
        }

        scratch.values[2441] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2441] != 0.0)) {
            scratch.store_ad(2373, &AdValue::div(scratch.ad_value(2373), AdValue::offset(AdValue::mul(scratch.ad_value(2324), scratch.ad_value(2343)), 1.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2374, &AdValue::scale(AdValue::mul(scratch.ad_value(2372), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2373), 2.0), 1.0)), 1.0)), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2375, &AdValue::div_from_scalar(1.0, scratch.ad_value(2374)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2376, &AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2011)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(2372), scratch.ad_value(2375)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2377, &AdValue::mul(scratch.ad_value(2355), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2373), scratch.ad_value(2076)), scratch.ad_value(2076)), 0.5), 1.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2378, &AdValue::div(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2365)), scratch.ad_value(2377)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[929] = scratch.values[2294];
            scratch.node_derivatives[929] = scratch.node_derivatives[2294];
            scratch.branch_derivatives[929] = scratch.branch_derivatives[2294];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[930] = scratch.values[2368];
            scratch.node_derivatives[930] = scratch.node_derivatives[2368];
            scratch.branch_derivatives[930] = scratch.branch_derivatives[2368];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[931] = scratch.values[2376];
            scratch.node_derivatives[931] = scratch.node_derivatives[2376];
            scratch.branch_derivatives[931] = scratch.branch_derivatives[2376];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[932] = scratch.values[2355];
            scratch.node_derivatives[932] = scratch.node_derivatives[2355];
            scratch.branch_derivatives[932] = scratch.branch_derivatives[2355];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[933] = scratch.values[2343];
            scratch.node_derivatives[933] = scratch.node_derivatives[2343];
            scratch.branch_derivatives[933] = scratch.branch_derivatives[2343];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[934] = scratch.values[2364];
            scratch.node_derivatives[934] = scratch.node_derivatives[2364];
            scratch.branch_derivatives[934] = scratch.branch_derivatives[2364];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[935] = scratch.values[2371];
            scratch.node_derivatives[935] = scratch.node_derivatives[2371];
            scratch.branch_derivatives[935] = scratch.branch_derivatives[2371];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[936] = scratch.values[2378];
            scratch.node_derivatives[936] = scratch.node_derivatives[2378];
            scratch.branch_derivatives[936] = scratch.branch_derivatives[2378];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2028] = scratch.values[2353];
            scratch.node_derivatives[2028] = scratch.node_derivatives[2353];
            scratch.branch_derivatives[2028] = scratch.branch_derivatives[2353];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2029] = scratch.values[2374];
            scratch.node_derivatives[2029] = scratch.node_derivatives[2374];
            scratch.branch_derivatives[2029] = scratch.branch_derivatives[2374];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2030] = scratch.values[2372];
            scratch.node_derivatives[2030] = scratch.node_derivatives[2372];
            scratch.branch_derivatives[2030] = scratch.branch_derivatives[2372];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[929] = scratch.values[2013];
            scratch.node_derivatives[929] = scratch.node_derivatives[2013];
            scratch.branch_derivatives[929] = scratch.branch_derivatives[2013];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[930] = scratch.values[879];
            scratch.node_derivatives[930] = scratch.node_derivatives[879];
            scratch.branch_derivatives[930] = scratch.branch_derivatives[879];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[931] = scratch.values[880];
            scratch.node_derivatives[931] = scratch.node_derivatives[880];
            scratch.branch_derivatives[931] = scratch.branch_derivatives[880];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[932] = scratch.values[2014];
            scratch.node_derivatives[932] = scratch.node_derivatives[2014];
            scratch.branch_derivatives[932] = scratch.branch_derivatives[2014];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[933] = scratch.values[2015];
            scratch.node_derivatives[933] = scratch.node_derivatives[2015];
            scratch.branch_derivatives[933] = scratch.branch_derivatives[2015];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[934] = scratch.values[2016];
            scratch.node_derivatives[934] = scratch.node_derivatives[2016];
            scratch.branch_derivatives[934] = scratch.branch_derivatives[2016];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[935] = scratch.values[881];
            scratch.node_derivatives[935] = scratch.node_derivatives[881];
            scratch.branch_derivatives[935] = scratch.branch_derivatives[881];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[936] = scratch.values[2018];
            scratch.node_derivatives[936] = scratch.node_derivatives[2018];
            scratch.branch_derivatives[936] = scratch.branch_derivatives[2018];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[2028] = scratch.values[882];
            scratch.node_derivatives[2028] = scratch.node_derivatives[882];
            scratch.branch_derivatives[2028] = scratch.branch_derivatives[882];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[2029] = scratch.values[883];
            scratch.node_derivatives[2029] = scratch.node_derivatives[883];
            scratch.branch_derivatives[2029] = scratch.branch_derivatives[883];
        }

        if (!(scratch.values[2404] != 0.0)) {
            scratch.values[2030] = scratch.values[884];
            scratch.node_derivatives[2030] = scratch.node_derivatives[884];
            scratch.branch_derivatives[2030] = scratch.branch_derivatives[884];
        }

        scratch.store_ad(857, &AdValue::add(scratch.ad_value(853), scratch.ad_value(855)));

        scratch.values[2032] = scratch.values[266];
        scratch.node_derivatives[2032] = scratch.node_derivatives[266];
        scratch.branch_derivatives[2032] = scratch.branch_derivatives[266];

        scratch.values[2442] = if (scratch.values[816] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2442] != 0.0) {
            scratch.store_ad(2032, &AdValue::div(scratch.ad_value(266), AdValue::offset(AdValue::mul(scratch.ad_value(816), AdValue::powf(AdValue::offset(AdValue::square(scratch.ad_value(930)), scratch.values[776]), ((-1.0) * 0.16666666666666666))), 1.0)));
        }

        scratch.values[2443] = if (scratch.values[929] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2443] != 0.0) {
            scratch.values[937] = scratch.values[931];
            scratch.node_derivatives[937] = scratch.node_derivatives[931];
            scratch.branch_derivatives[937] = scratch.branch_derivatives[931];
        }

        if (scratch.values[2443] != 0.0) {
            scratch.values[938] = 0.0;
            scratch.node_derivatives[938] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[938] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2443] != 0.0) {
            scratch.values[939] = 0.0;
            scratch.node_derivatives[939] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[939] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2443] != 0.0) {
            scratch.values[940] = scratch.values[937];
            scratch.node_derivatives[940] = scratch.node_derivatives[937];
            scratch.branch_derivatives[940] = scratch.branch_derivatives[937];
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(941, &AdValue::scale(AdValue::div(scratch.ad_value(933), scratch.ad_value(936)), 0.5));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(942, &AdValue::square(scratch.ad_value(941)));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(943, &AdValue::mul(AdValue::sub_from_scalar(1.0, scratch.ad_value(935)), AdValue::sub(scratch.ad_value(934), AdValue::scale(AdValue::mul(scratch.ad_value(932), scratch.ad_value(933)), 0.5))));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(937, &AdValue::add(scratch.ad_value(931), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2028), scratch.ad_value(933)), AdValue::add(AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(941), scratch.ad_value(935)), 0.3333333333333333), (-1.0)), scratch.ad_value(935))), 0.5)));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(2076, &AdValue::scale(AdValue::mul(scratch.ad_value(932), scratch.ad_value(933)), 0.16666666666666666));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(938, &AdValue::add(AdValue::mul(scratch.ad_value(935), AdValue::add(scratch.ad_value(934), AdValue::mul(scratch.ad_value(2076), scratch.ad_value(941)))), scratch.ad_value(943)));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(939, &AdValue::scale(AdValue::add(AdValue::mul(AdValue::square(scratch.ad_value(935)), AdValue::sub(scratch.ad_value(934), AdValue::mul(scratch.ad_value(2076), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(941)), AdValue::scale(scratch.ad_value(942), 0.2))))), AdValue::mul(scratch.ad_value(943), AdValue::offset(scratch.ad_value(935), 1.0))), 0.5));
        }

        if (!(scratch.values[2443] != 0.0)) {
            scratch.store_ad(940, &AdValue::sub(scratch.ad_value(937), scratch.ad_value(938)));
        }

        scratch.store_ad(944, &AdValue::mul(scratch.ad_value(937), scratch.ad_value(2032)));

        scratch.store_ad(945, &AdValue::mul(AdValue::neg(scratch.ad_value(939)), scratch.ad_value(2032)));

        scratch.store_ad(946, &AdValue::mul(AdValue::neg(scratch.ad_value(940)), scratch.ad_value(2032)));

        scratch.store_ad(948, &AdValue::mul(scratch.ad_value(267), scratch.ad_value(893)));

        scratch.store_ad(949, &AdValue::mul(scratch.ad_value(268), scratch.ad_value(894)));

        scratch.store_ad(950, &AdValue::mul(scratch.ad_value(269), scratch.ad_value(857)));

        scratch.store_ad(951, &AdValue::mul(scratch.ad_value(270), scratch.ad_value(860)));

        scratch.store_ad(952, &AdValue::mul(scratch.ad_value(271), scratch.ad_value(863)));

        scratch.values[2041] = 0.0;

        scratch.values[2042] = 0.0;

        scratch.values[2043] = 0.0;

        scratch.values[2044] = 0.0;

        scratch.values[2045] = 0.0;

        scratch.values[2046] = 0.0;

        scratch.values[2047] = 0.0;

        scratch.values[2048] = 0.0;

        scratch.values[2444] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2445] = if (scratch.values[410] == 1.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.values[2448] = 0.0;
            scratch.node_derivatives[2448] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2448] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.values[2449] = 0.0;
            scratch.node_derivatives[2449] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2449] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(648, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(649, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(650, &AdValue::add(scratch.ad_value(858), AdValue::mul(scratch.ad_value(701), scratch.ad_value(649))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(651, &AdValue::add(scratch.ad_value(702), scratch.ad_value(650)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(652, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(650)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(653, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(652)), scratch.ad_value(648))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2449, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(858), scratch.ad_value(702)), AdValue::add(scratch.ad_value(651), scratch.ad_value(653))), 2.0));
        }

        scratch.values[2450] = if (scratch.values[695] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2451] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2450] != 0.0)) && (scratch.values[2451] != 0.0)) {
            scratch.store_ad(2448, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2449), scratch.values[454]))));
        }

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2450] != 0.0)) && (!(scratch.values[2451] != 0.0))) {
            scratch.store_ad(2448, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2449), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2450] != 0.0)) {
            scratch.store_ad(2042, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(2448)), scratch.values[466]), AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(2449)), scratch.values[469])));
        }

        scratch.values[2452] = if (scratch.values[696] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2453] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2452] != 0.0)) && (scratch.values[2453] != 0.0)) {
            scratch.store_ad(2448, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2449), scratch.values[455]))));
        }

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2452] != 0.0)) && (!(scratch.values[2453] != 0.0))) {
            scratch.store_ad(2448, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2449), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2043, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(2448)), scratch.values[467]), AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(2449)), scratch.values[470])));
        }

        scratch.values[2454] = if (scratch.values[697] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2455] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2454] != 0.0)) && (scratch.values[2455] != 0.0)) {
            scratch.store_ad(2448, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2449), scratch.values[456]))));
        }

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2454] != 0.0)) && (!(scratch.values[2455] != 0.0))) {
            scratch.store_ad(2448, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2449), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2454] != 0.0)) {
            scratch.store_ad(2044, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(2448)), scratch.values[468]), AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(2449)), scratch.values[471])));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.values[2448] = 0.0;
            scratch.node_derivatives[2448] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2448] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.values[2449] = 0.0;
            scratch.node_derivatives[2449] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2449] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(648, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(649, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(650, &AdValue::add(scratch.ad_value(859), AdValue::mul(scratch.ad_value(728), scratch.ad_value(649))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(651, &AdValue::add(scratch.ad_value(729), scratch.ad_value(650)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(652, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(650)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(653, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(652)), scratch.ad_value(648))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2449, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(859), scratch.ad_value(729)), AdValue::add(scratch.ad_value(651), scratch.ad_value(653))), 2.0));
        }

        scratch.values[2456] = if (scratch.values[722] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2457] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2456] != 0.0)) && (scratch.values[2457] != 0.0)) {
            scratch.store_ad(2448, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2449), scratch.ad_value(597)))));
        }

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2456] != 0.0)) && (!(scratch.values[2457] != 0.0))) {
            scratch.store_ad(2448, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2449), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2456] != 0.0)) {
            scratch.store_ad(2046, &AdValue::add(AdValue::mul(scratch.ad_value(609), AdValue::sub_from_scalar(1.0, scratch.ad_value(2448))), AdValue::mul(scratch.ad_value(612), AdValue::sub(scratch.ad_value(859), scratch.ad_value(2449)))));
        }

        scratch.values[2458] = if (scratch.values[723] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2459] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2458] != 0.0)) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(2448, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2449), scratch.ad_value(598)))));
        }

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2458] != 0.0)) && (!(scratch.values[2459] != 0.0))) {
            scratch.store_ad(2448, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2449), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2458] != 0.0)) {
            scratch.store_ad(2047, &AdValue::add(AdValue::mul(scratch.ad_value(610), AdValue::sub_from_scalar(1.0, scratch.ad_value(2448))), AdValue::mul(scratch.ad_value(613), AdValue::sub(scratch.ad_value(859), scratch.ad_value(2449)))));
        }

        scratch.values[2460] = if (scratch.values[724] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2461] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2460] != 0.0)) && (scratch.values[2461] != 0.0)) {
            scratch.store_ad(2448, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2449), scratch.ad_value(599)))));
        }

        if ((((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2460] != 0.0)) && (!(scratch.values[2461] != 0.0))) {
            scratch.store_ad(2448, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2449), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2460] != 0.0)) {
            scratch.store_ad(2048, &AdValue::add(AdValue::mul(scratch.ad_value(611), AdValue::sub_from_scalar(1.0, scratch.ad_value(2448))), AdValue::mul(scratch.ad_value(614), AdValue::sub(scratch.ad_value(859), scratch.ad_value(2449)))));
        }

        if ((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) {
            scratch.values[661] = 0.0;
            scratch.node_derivatives[661] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[661] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) {
            scratch.values[658] = 0.0;
            scratch.node_derivatives[658] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[658] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2462] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(648, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(649, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(650, &AdValue::add(scratch.ad_value(858), AdValue::mul(scratch.ad_value(701), scratch.ad_value(649))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(651, &AdValue::add(scratch.ad_value(702), scratch.ad_value(650)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(652, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(650)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(653, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(652)), scratch.ad_value(648))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(655, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(858), scratch.ad_value(702)), AdValue::add(scratch.ad_value(651), scratch.ad_value(653))), 2.0));
        }

        scratch.values[2463] = if (scratch.values[858] < scratch.values[698]) { 1.0 } else { 0.0 };

        scratch.values[2464] = if ((((0.5 * (scratch.values[858] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (scratch.values[2463] != 0.0)) && (scratch.values[2464] != 0.0)) {
            scratch.store_ad(657, &AdValue::exp(AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5))));
        }

        scratch.values[2465] = if ((0.5 * (scratch.values[858] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (scratch.values[2463] != 0.0)) && (!(scratch.values[2464] != 0.0))) && (scratch.values[2465] != 0.0)) {
            let assign51920_ad_e66713: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(657, &assign51920_ad_e66713);
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (scratch.values[2463] != 0.0)) && (!(scratch.values[2464] != 0.0))) && (!(scratch.values[2465] != 0.0))) {
            scratch.store_ad(657, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(858), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (scratch.values[2463] != 0.0)) {
            scratch.store_ad(654, &AdValue::square(scratch.ad_value(657)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (!(scratch.values[2463] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(698)), scratch.values[420]), 1.0), scratch.ad_value(699)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (!(scratch.values[2463] != 0.0))) {
            scratch.store_ad(657, &AdValue::sqrt(scratch.ad_value(654)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(654, &AdValue::offset(scratch.ad_value(654), (-1.0)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(656, &AdValue::div_from_scalar(1.0, scratch.ad_value(657)));
        }

        scratch.values[2466] = if (scratch.values[858] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (scratch.values[2466] != 0.0)) {
            scratch.store_ad(658, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(656), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(656), 1.0), AdValue::offset(scratch.ad_value(656), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) && (!(scratch.values[2466] != 0.0))) {
            scratch.store_ad(658, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(657), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(657), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(657), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(858)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(659, &AdValue::sub(scratch.ad_value(700), scratch.ad_value(658)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(660, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(858), scratch.ad_value(659)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(858), scratch.ad_value(659)), AdValue::sub(scratch.ad_value(858), scratch.ad_value(659))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(661, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(858), scratch.ad_value(703)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(858), scratch.ad_value(703)), AdValue::sub(scratch.ad_value(858), scratch.ad_value(703))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2462] != 0.0)) {
            scratch.store_ad(662, &AdValue::scale(AdValue::sub(scratch.ad_value(858), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(858), scratch.ad_value(858)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[2467] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2467] != 0.0)) {
            scratch.values[2042] = 0.0;
            scratch.node_derivatives[2042] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2042] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2468] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (scratch.values[2468] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(655), scratch.values[454]))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2468] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(655), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) {
            scratch.store_ad(2042, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(663)), scratch.values[466]), AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(655)), scratch.values[469])));
        }

        scratch.values[2469] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(660)));
        }

        scratch.values[2471] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) && (scratch.values[2471] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::scale(scratch.ad_value(666), scratch.values[478])));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) && (!(scratch.values[2471] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::scale(scratch.ad_value(666), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(670, &AdValue::scale(scratch.ad_value(663), scratch.values[472]));
        }

        scratch.values[2472] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(673, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(670), scratch.values[457]), scratch.ad_value(666)), scratch.values[487]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(674, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[484]), scratch.ad_value(673)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(675, &AdValue::square(scratch.ad_value(674)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(676, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(675)), AdValue::offset(AdValue::square(scratch.ad_value(675)), 1.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(677, &AdValue::sqrt(AdValue::abs(scratch.ad_value(676))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(676), scratch.ad_value(677)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(681, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(673), scratch.ad_value(677)), 0.375)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(682, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(674), scratch.ad_value(677)), 2.0), scratch.ad_value(676)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(683, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(674), scratch.values[484]), scratch.ad_value(677)), AdValue::scale(scratch.ad_value(676), scratch.values[484])), AdValue::scale(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(684, &AdValue::mul(AdValue::offset(scratch.ad_value(682), (-1.0)), scratch.ad_value(681)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(645, &AdValue::square(scratch.ad_value(684)));
        }

        scratch.values[2475] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (scratch.values[2475] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2475] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2476] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2477] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2476] != 0.0))) && (scratch.values[2477] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2476] != 0.0))) && (!(scratch.values[2477] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2478] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2479] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) && (scratch.values[2479] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(661)), scratch.values[478])));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) && (!(scratch.values[2479] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(661)), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) {
            scratch.store_ad(688, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(661)), scratch.values[475]), scratch.ad_value(663)), scratch.values[460]));
        }

        scratch.values[2480] = if (((((-scratch.values[490]) / scratch.values[688])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) && (scratch.values[2480] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688))));
        }

        scratch.values[2481] = if (((-scratch.values[490]) / scratch.values[688]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) && (!(scratch.values[2480] != 0.0))) && (scratch.values[2481] != 0.0)) {
            let assign52690_ad_e68044: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign52690_ad_e68044));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) && (!(scratch.values[2480] != 0.0))) && (!(scratch.values[2481] != 0.0))) {
            let assign52700_ad_e68095: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign52700_ad_e68095);
        }

        scratch.values[2482] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2483] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[2484] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2482] != 0.0))) && (scratch.values[2483] != 0.0)) && (scratch.values[2484] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(662), scratch.values[497]), AdValue::scale(scratch.ad_value(662), scratch.values[497])), AdValue::scale(scratch.ad_value(662), scratch.values[497])), AdValue::scale(scratch.ad_value(662), scratch.values[497])));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2482] != 0.0))) && (scratch.values[2483] != 0.0)) && (!(scratch.values[2484] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(662), scratch.values[497])), scratch.values[407]));
        }

        scratch.values[2485] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2485] != 0.0)) {
            scratch.values[2043] = 0.0;
            scratch.node_derivatives[2043] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2043] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2486] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (scratch.values[2486] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(655), scratch.values[455]))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(655), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) {
            scratch.store_ad(2043, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(663)), scratch.values[467]), AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(655)), scratch.values[470])));
        }

        scratch.values[2487] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(660)));
        }

        scratch.values[2489] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) && (scratch.values[2489] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::scale(scratch.ad_value(666), scratch.values[479])));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) && (!(scratch.values[2489] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::scale(scratch.ad_value(666), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(670, &AdValue::scale(scratch.ad_value(663), scratch.values[473]));
        }

        scratch.values[2490] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(673, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(670), scratch.values[458]), scratch.ad_value(666)), scratch.values[488]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(674, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[485]), scratch.ad_value(673)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(675, &AdValue::square(scratch.ad_value(674)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(676, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(675)), AdValue::offset(AdValue::square(scratch.ad_value(675)), 1.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(677, &AdValue::sqrt(AdValue::abs(scratch.ad_value(676))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(676), scratch.ad_value(677)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(681, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(673), scratch.ad_value(677)), 0.375)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(682, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(674), scratch.ad_value(677)), 2.0), scratch.ad_value(676)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(683, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(674), scratch.values[485]), scratch.ad_value(677)), AdValue::scale(scratch.ad_value(676), scratch.values[485])), AdValue::scale(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(684, &AdValue::mul(AdValue::offset(scratch.ad_value(682), (-1.0)), scratch.ad_value(681)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(645, &AdValue::square(scratch.ad_value(684)));
        }

        scratch.values[2493] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2493] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2493] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2494] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2495] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2494] != 0.0))) && (scratch.values[2495] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2494] != 0.0))) && (!(scratch.values[2495] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2496] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2497] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2497] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(661)), scratch.values[479])));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (!(scratch.values[2497] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(661)), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) {
            scratch.store_ad(688, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(661)), scratch.values[476]), scratch.ad_value(663)), scratch.values[461]));
        }

        scratch.values[2498] = if (((((-scratch.values[491]) / scratch.values[688])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2498] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))));
        }

        scratch.values[2499] = if (((-scratch.values[491]) / scratch.values[688]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (!(scratch.values[2498] != 0.0))) && (scratch.values[2499] != 0.0)) {
            let assign53440_ad_e69307: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign53440_ad_e69307));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (!(scratch.values[2498] != 0.0))) && (!(scratch.values[2499] != 0.0))) {
            let assign53450_ad_e69358: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign53450_ad_e69358);
        }

        scratch.values[2500] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2501] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[2502] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2500] != 0.0))) && (scratch.values[2501] != 0.0)) && (scratch.values[2502] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(662), scratch.values[498]), AdValue::scale(scratch.ad_value(662), scratch.values[498])), AdValue::scale(scratch.ad_value(662), scratch.values[498])), AdValue::scale(scratch.ad_value(662), scratch.values[498])));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2500] != 0.0))) && (scratch.values[2501] != 0.0)) && (!(scratch.values[2502] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(662), scratch.values[498])), scratch.values[408]));
        }

        scratch.values[2503] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2503] != 0.0)) {
            scratch.values[2044] = 0.0;
            scratch.node_derivatives[2044] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2044] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2504] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (scratch.values[2504] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(655), scratch.values[456]))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(655), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) {
            scratch.store_ad(2044, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(663)), scratch.values[468]), AdValue::scale(AdValue::sub(scratch.ad_value(858), scratch.ad_value(655)), scratch.values[471])));
        }

        scratch.values[2505] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(660)));
        }

        scratch.values[2507] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) && (scratch.values[2507] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::scale(scratch.ad_value(666), scratch.values[480])));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) && (!(scratch.values[2507] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::scale(scratch.ad_value(666), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(670, &AdValue::scale(scratch.ad_value(663), scratch.values[474]));
        }

        scratch.values[2508] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(673, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(670), scratch.values[459]), scratch.ad_value(666)), scratch.values[489]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(674, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[486]), scratch.ad_value(673)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(675, &AdValue::square(scratch.ad_value(674)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(676, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(675)), AdValue::offset(AdValue::square(scratch.ad_value(675)), 1.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(677, &AdValue::sqrt(AdValue::abs(scratch.ad_value(676))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(676), scratch.ad_value(677)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(681, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(673), scratch.ad_value(677)), 0.375)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(682, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(674), scratch.ad_value(677)), 2.0), scratch.ad_value(676)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(683, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(674), scratch.values[486]), scratch.ad_value(677)), AdValue::scale(scratch.ad_value(676), scratch.values[486])), AdValue::scale(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(684, &AdValue::mul(AdValue::offset(scratch.ad_value(682), (-1.0)), scratch.ad_value(681)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(645, &AdValue::square(scratch.ad_value(684)));
        }

        scratch.values[2511] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2511] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2511] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2512] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2513] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2512] != 0.0))) && (scratch.values[2513] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2512] != 0.0))) && (!(scratch.values[2513] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2514] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2515] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2515] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(661)), scratch.values[480])));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (!(scratch.values[2515] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(661)), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) {
            scratch.store_ad(688, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(661)), scratch.values[477]), scratch.ad_value(663)), scratch.values[462]));
        }

        scratch.values[2516] = if (((((-scratch.values[492]) / scratch.values[688])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2516] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688))));
        }

        scratch.values[2517] = if (((-scratch.values[492]) / scratch.values[688]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (!(scratch.values[2516] != 0.0))) && (scratch.values[2517] != 0.0)) {
            let assign54190_ad_e70570: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign54190_ad_e70570));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (!(scratch.values[2516] != 0.0))) && (!(scratch.values[2517] != 0.0))) {
            let assign54200_ad_e70621: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign54200_ad_e70621);
        }

        scratch.values[2518] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2519] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[2520] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2518] != 0.0))) && (scratch.values[2519] != 0.0)) && (scratch.values[2520] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(662), scratch.values[499]), AdValue::scale(scratch.ad_value(662), scratch.values[499])), AdValue::scale(scratch.ad_value(662), scratch.values[499])), AdValue::scale(scratch.ad_value(662), scratch.values[499])));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2518] != 0.0))) && (scratch.values[2519] != 0.0)) && (!(scratch.values[2520] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(662), scratch.values[499])), scratch.values[409]));
        }

        if ((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) {
            scratch.values[661] = 0.0;
            scratch.node_derivatives[661] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[661] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) {
            scratch.values[658] = 0.0;
            scratch.node_derivatives[658] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[658] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2521] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(648, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(649, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(650, &AdValue::add(scratch.ad_value(859), AdValue::mul(scratch.ad_value(728), scratch.ad_value(649))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(651, &AdValue::add(scratch.ad_value(729), scratch.ad_value(650)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(652, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(650)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(653, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(652)), scratch.ad_value(648))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(655, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(859), scratch.ad_value(729)), AdValue::add(scratch.ad_value(651), scratch.ad_value(653))), 2.0));
        }

        scratch.values[2522] = if (scratch.values[859] < scratch.values[725]) { 1.0 } else { 0.0 };

        scratch.values[2523] = if ((((0.5 * (scratch.values[859] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (scratch.values[2522] != 0.0)) && (scratch.values[2523] != 0.0)) {
            scratch.store_ad(657, &AdValue::exp(AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5))));
        }

        scratch.values[2524] = if ((0.5 * (scratch.values[859] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (scratch.values[2522] != 0.0)) && (!(scratch.values[2523] != 0.0))) && (scratch.values[2524] != 0.0)) {
            let assign54460_ad_e71003: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(657, &assign54460_ad_e71003);
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (scratch.values[2522] != 0.0)) && (!(scratch.values[2523] != 0.0))) && (!(scratch.values[2524] != 0.0))) {
            scratch.store_ad(657, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(859), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (scratch.values[2522] != 0.0)) {
            scratch.store_ad(654, &AdValue::square(scratch.ad_value(657)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(859), scratch.ad_value(725)), scratch.values[420]), 1.0), scratch.ad_value(726)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(657, &AdValue::sqrt(scratch.ad_value(654)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(654, &AdValue::offset(scratch.ad_value(654), (-1.0)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(656, &AdValue::div_from_scalar(1.0, scratch.ad_value(657)));
        }

        scratch.values[2525] = if (scratch.values[859] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (scratch.values[2525] != 0.0)) {
            scratch.store_ad(658, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(656), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(656), 1.0), AdValue::offset(scratch.ad_value(656), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) && (!(scratch.values[2525] != 0.0))) {
            scratch.store_ad(658, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(657), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(657), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(657), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(859)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(659, &AdValue::sub(scratch.ad_value(727), scratch.ad_value(658)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(660, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(859), scratch.ad_value(659)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(859), scratch.ad_value(659)), AdValue::sub(scratch.ad_value(859), scratch.ad_value(659))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(661, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(859), scratch.ad_value(730)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(859), scratch.ad_value(730)), AdValue::sub(scratch.ad_value(859), scratch.ad_value(730))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(662, &AdValue::scale(AdValue::sub(scratch.ad_value(859), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(859), scratch.ad_value(859)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[2526] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2526] != 0.0)) {
            scratch.values[2046] = 0.0;
            scratch.node_derivatives[2046] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2046] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2527] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2527] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(597)))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2527] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) {
            scratch.store_ad(2046, &AdValue::add(AdValue::mul(scratch.ad_value(609), AdValue::sub_from_scalar(1.0, scratch.ad_value(663))), AdValue::mul(scratch.ad_value(612), AdValue::sub(scratch.ad_value(859), scratch.ad_value(655)))));
        }

        scratch.values[2528] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(660)));
        }

        scratch.values[2530] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (scratch.values[2530] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(scratch.ad_value(666), scratch.ad_value(621))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (!(scratch.values[2530] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(scratch.ad_value(666), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(670, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(663)));
        }

        scratch.values[2531] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(673, &AdValue::mul(scratch.ad_value(630), AdValue::div(AdValue::mul(scratch.ad_value(670), scratch.ad_value(600)), scratch.ad_value(666))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(674, &AdValue::div(AdValue::scale(scratch.ad_value(627), 0.666666666666667), scratch.ad_value(673)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(675, &AdValue::square(scratch.ad_value(674)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(676, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(675)), AdValue::offset(AdValue::square(scratch.ad_value(675)), 1.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(677, &AdValue::sqrt(AdValue::abs(scratch.ad_value(676))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(676), scratch.ad_value(677)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(681, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(673), scratch.ad_value(677)), 0.375)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(682, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(674), scratch.ad_value(677)), 2.0), scratch.ad_value(676)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(683, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(627), scratch.ad_value(674)), scratch.ad_value(677)), AdValue::mul(scratch.ad_value(627), scratch.ad_value(676))), AdValue::scale(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(684, &AdValue::mul(AdValue::offset(scratch.ad_value(682), (-1.0)), scratch.ad_value(681)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(645, &AdValue::square(scratch.ad_value(684)));
        }

        scratch.values[2534] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (scratch.values[2534] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2534] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2535] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2536] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2535] != 0.0))) && (scratch.values[2536] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2535] != 0.0))) && (!(scratch.values[2536] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2537] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2538] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) && (scratch.values[2538] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(661)), scratch.ad_value(621))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) && (!(scratch.values[2538] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(661)), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) {
            scratch.store_ad(688, &AdValue::mul(scratch.ad_value(603), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(661)), scratch.ad_value(618)), scratch.ad_value(663))));
        }

        scratch.values[2539] = if (((((-scratch.values[633]) / scratch.values[688])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) && (scratch.values[2539] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688))));
        }

        scratch.values[2540] = if (((-scratch.values[633]) / scratch.values[688]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) && (!(scratch.values[2539] != 0.0))) && (scratch.values[2540] != 0.0)) {
            let assign55230_ad_e72334: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign55230_ad_e72334));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) && (!(scratch.values[2539] != 0.0))) && (!(scratch.values[2540] != 0.0))) {
            let assign55240_ad_e72385: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign55240_ad_e72385);
        }

        scratch.values[2541] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2542] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[2543] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2541] != 0.0))) && (scratch.values[2542] != 0.0)) && (scratch.values[2543] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(662), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2541] != 0.0))) && (scratch.values[2542] != 0.0)) && (!(scratch.values[2543] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        scratch.values[2544] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2544] != 0.0)) {
            scratch.values[2047] = 0.0;
            scratch.node_derivatives[2047] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2047] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2545] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (scratch.values[2545] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(598)))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) {
            scratch.store_ad(2047, &AdValue::add(AdValue::mul(scratch.ad_value(610), AdValue::sub_from_scalar(1.0, scratch.ad_value(663))), AdValue::mul(scratch.ad_value(613), AdValue::sub(scratch.ad_value(859), scratch.ad_value(655)))));
        }

        scratch.values[2546] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(660)));
        }

        scratch.values[2548] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) && (scratch.values[2548] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(scratch.ad_value(666), scratch.ad_value(622))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) && (!(scratch.values[2548] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(scratch.ad_value(666), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(670, &AdValue::mul(scratch.ad_value(616), scratch.ad_value(663)));
        }

        scratch.values[2549] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(673, &AdValue::mul(scratch.ad_value(631), AdValue::div(AdValue::mul(scratch.ad_value(670), scratch.ad_value(601)), scratch.ad_value(666))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(674, &AdValue::div(AdValue::scale(scratch.ad_value(628), 0.666666666666667), scratch.ad_value(673)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(675, &AdValue::square(scratch.ad_value(674)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(676, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(675)), AdValue::offset(AdValue::square(scratch.ad_value(675)), 1.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(677, &AdValue::sqrt(AdValue::abs(scratch.ad_value(676))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(676), scratch.ad_value(677)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(681, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(673), scratch.ad_value(677)), 0.375)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(682, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(674), scratch.ad_value(677)), 2.0), scratch.ad_value(676)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(683, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(628), scratch.ad_value(674)), scratch.ad_value(677)), AdValue::mul(scratch.ad_value(628), scratch.ad_value(676))), AdValue::scale(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(684, &AdValue::mul(AdValue::offset(scratch.ad_value(682), (-1.0)), scratch.ad_value(681)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(645, &AdValue::square(scratch.ad_value(684)));
        }

        scratch.values[2552] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2552] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2552] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2553] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2554] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2553] != 0.0))) && (scratch.values[2554] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2553] != 0.0))) && (!(scratch.values[2554] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2555] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2556] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2556] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(661)), scratch.ad_value(622))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (!(scratch.values[2556] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(661)), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) {
            scratch.store_ad(688, &AdValue::mul(scratch.ad_value(604), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(661)), scratch.ad_value(619)), scratch.ad_value(663))));
        }

        scratch.values[2557] = if (((((-scratch.values[634]) / scratch.values[688])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2557] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688))));
        }

        scratch.values[2558] = if (((-scratch.values[634]) / scratch.values[688]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (!(scratch.values[2557] != 0.0))) && (scratch.values[2558] != 0.0)) {
            let assign55980_ad_e73597: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign55980_ad_e73597));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (!(scratch.values[2557] != 0.0))) && (!(scratch.values[2558] != 0.0))) {
            let assign55990_ad_e73648: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign55990_ad_e73648);
        }

        scratch.values[2559] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2560] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[2561] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2559] != 0.0))) && (scratch.values[2560] != 0.0)) && (scratch.values[2561] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(662), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2559] != 0.0))) && (scratch.values[2560] != 0.0)) && (!(scratch.values[2561] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))), scratch.ad_value(575)));
        }

        scratch.values[2562] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2562] != 0.0)) {
            scratch.values[2048] = 0.0;
            scratch.node_derivatives[2048] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2048] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2563] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (scratch.values[2563] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(599)))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) {
            scratch.store_ad(2048, &AdValue::add(AdValue::mul(scratch.ad_value(611), AdValue::sub_from_scalar(1.0, scratch.ad_value(663))), AdValue::mul(scratch.ad_value(614), AdValue::sub(scratch.ad_value(859), scratch.ad_value(655)))));
        }

        scratch.values[2564] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(660)));
        }

        scratch.values[2566] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) && (scratch.values[2566] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(scratch.ad_value(666), scratch.ad_value(623))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) && (!(scratch.values[2566] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(scratch.ad_value(666), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(670, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(663)));
        }

        scratch.values[2567] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(673, &AdValue::mul(scratch.ad_value(632), AdValue::div(AdValue::mul(scratch.ad_value(670), scratch.ad_value(602)), scratch.ad_value(666))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(674, &AdValue::div(AdValue::scale(scratch.ad_value(629), 0.666666666666667), scratch.ad_value(673)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(675, &AdValue::square(scratch.ad_value(674)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(676, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(675)), AdValue::offset(AdValue::square(scratch.ad_value(675)), 1.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(677, &AdValue::sqrt(AdValue::abs(scratch.ad_value(676))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(676), scratch.ad_value(677)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(681, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(673), scratch.ad_value(677)), 0.375)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(682, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(674), scratch.ad_value(677)), 2.0), scratch.ad_value(676)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(683, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(629), scratch.ad_value(674)), scratch.ad_value(677)), AdValue::mul(scratch.ad_value(629), scratch.ad_value(676))), AdValue::scale(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(684, &AdValue::mul(AdValue::offset(scratch.ad_value(682), (-1.0)), scratch.ad_value(681)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(645, &AdValue::square(scratch.ad_value(684)));
        }

        scratch.values[2570] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2570] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2570] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2571] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2572] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2571] != 0.0))) && (scratch.values[2572] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2571] != 0.0))) && (!(scratch.values[2572] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2573] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2574] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2574] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(661)), scratch.ad_value(623))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (!(scratch.values[2574] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(661)), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) {
            scratch.store_ad(688, &AdValue::mul(scratch.ad_value(605), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(661)), scratch.ad_value(620)), scratch.ad_value(663))));
        }

        scratch.values[2575] = if (((((-scratch.values[635]) / scratch.values[688])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2575] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688))));
        }

        scratch.values[2576] = if (((-scratch.values[635]) / scratch.values[688]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (!(scratch.values[2575] != 0.0))) && (scratch.values[2576] != 0.0)) {
            let assign56730_ad_e74860: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign56730_ad_e74860));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (!(scratch.values[2575] != 0.0))) && (!(scratch.values[2576] != 0.0))) {
            let assign56740_ad_e74911: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign56740_ad_e74911);
        }

        scratch.values[2577] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2578] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[2579] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2577] != 0.0))) && (scratch.values[2578] != 0.0)) && (scratch.values[2579] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(662), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2577] != 0.0))) && (scratch.values[2578] != 0.0)) && (!(scratch.values[2579] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))), scratch.ad_value(576)));
        }

        scratch.store_ad(947, &AdValue::neg(AdValue::add(AdValue::add(scratch.ad_value(944), scratch.ad_value(946)), scratch.ad_value(945))));

        scratch.store_ad(951, &AdValue::add(scratch.ad_value(951), scratch.ad_value(948)));

        scratch.store_ad(952, &AdValue::add(scratch.ad_value(952), scratch.ad_value(949)));

        scratch.store_ad(2041, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(2042)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(2043))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(2044))));

        scratch.store_ad(2045, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(2046)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(2047))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(2048))));

        scratch.values[2589] = if (scratch.values[2002] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2589] != 0.0) {
            scratch.values[2588] = scratch.values[945];
            scratch.node_derivatives[2588] = scratch.node_derivatives[945];
            scratch.branch_derivatives[2588] = scratch.branch_derivatives[945];
        }

        if (scratch.values[2589] != 0.0) {
            scratch.values[945] = scratch.values[947];
            scratch.node_derivatives[945] = scratch.node_derivatives[947];
            scratch.branch_derivatives[945] = scratch.branch_derivatives[947];
        }

        if (scratch.values[2589] != 0.0) {
            scratch.values[947] = scratch.values[2588];
            scratch.node_derivatives[947] = scratch.node_derivatives[2588];
            scratch.branch_derivatives[947] = scratch.branch_derivatives[2588];
        }

        scratch.store_ad(1006, &AdValue::mul(scratch.ad_value(2032), scratch.ad_value(2028)));

        scratch.values[2590] = if (((scratch.values[2013] > 0.0) && (scratch.values[25] > 0.0)) && (scratch.values[760] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[2594] = if ((scratch.values[10] == 1.0) && (scratch.values[763] > 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1006, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2029)), scratch.ad_value(2032)), scratch.ad_value(2028)), AdValue::square(scratch.ad_value(2030))));
        }

        scratch.values[2598] = if (((scratch.values[8] != 0.0) && (scratch.values[283] > 0.0)) && (scratch.values[2061] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2077, &AdValue::div(AdValue::scale(scratch.ad_value(2070), 4.0), scratch.ad_value(767)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2077, &AdValue::scale(scratch.ad_value(814), scratch.values[759]));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2077, &AdValue::mul(scratch.ad_value(2014), scratch.ad_value(2018)));
        }

    }
}
