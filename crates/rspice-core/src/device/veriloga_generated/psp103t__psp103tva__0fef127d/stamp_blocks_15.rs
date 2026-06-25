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
        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2382, &AdValue::add(scratch.ad_value(2380), AdValue::mul(scratch.ad_value(815), scratch.ad_value(2378))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2333, &AdValue::mul(scratch.ad_value(813), scratch.ad_value(2381)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2087, &AdValue::ln(AdValue::div(scratch.ad_value(2365), AdValue::offset(AdValue::add(scratch.ad_value(2365), scratch.ad_value(2364)), 1e-14))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2334, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2333), scratch.ad_value(755)), scratch.ad_value(756)), AdValue::mul(scratch.ad_value(757), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(758), 0.5), scratch.ad_value(2087))))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2335, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2334), 1.0), scratch.ad_value(2332)), scratch.ad_value(2327)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2383, &AdValue::ln(AdValue::div(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(850), scratch.ad_value(2357)), scratch.ad_value(817)), 1.0), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2348), scratch.ad_value(2357)), scratch.ad_value(817)), 1.0))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2384, &AdValue::mul(scratch.ad_value(244), scratch.ad_value(2383)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2385, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(scratch.ad_value(2384), 1.0), AdValue::square(scratch.ad_value(2384)))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(2378), scratch.ad_value(2336)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2337, &AdValue::scale(AdValue::div(scratch.ad_value(2088), AdValue::offset(scratch.ad_value(2088), 100.0)), 100.0));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2386, &AdValue::mul(scratch.ad_value(2335), scratch.ad_value(2385)));
        }

        scratch.values[2454] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2454] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(2337)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2454] != 0.0))) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(2337)), 1.0));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2338, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(2086), scratch.ad_value(2386))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2387, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2338)), scratch.ad_value(2357)), scratch.ad_value(2357)));
        }

        scratch.values[2455] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2455] != 0.0)) {
            scratch.store_ad(2387, &AdValue::div(scratch.ad_value(2387), AdValue::offset(AdValue::mul(scratch.ad_value(2338), scratch.ad_value(2357)), 1.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2388, &AdValue::scale(AdValue::mul(scratch.ad_value(2386), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2387), 2.0), 1.0)), 1.0)), 0.5));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2389, &AdValue::div_from_scalar(1.0, scratch.ad_value(2388)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2390, &AdValue::mul(scratch.ad_value(2366), scratch.ad_value(2008)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(2386), scratch.ad_value(2389)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(scratch.ad_value(2369), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2387), scratch.ad_value(2086)), scratch.ad_value(2086)), 0.5), 1.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2392, &AdValue::div(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2379)), scratch.ad_value(2391)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[925] = scratch.values[2308];
            scratch.node_derivatives[925] = scratch.node_derivatives[2308];
            scratch.branch_derivatives[925] = scratch.branch_derivatives[2308];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[926] = scratch.values[2382];
            scratch.node_derivatives[926] = scratch.node_derivatives[2382];
            scratch.branch_derivatives[926] = scratch.branch_derivatives[2382];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[927] = scratch.values[2390];
            scratch.node_derivatives[927] = scratch.node_derivatives[2390];
            scratch.branch_derivatives[927] = scratch.branch_derivatives[2390];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[928] = scratch.values[2369];
            scratch.node_derivatives[928] = scratch.node_derivatives[2369];
            scratch.branch_derivatives[928] = scratch.branch_derivatives[2369];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[929] = scratch.values[2357];
            scratch.node_derivatives[929] = scratch.node_derivatives[2357];
            scratch.branch_derivatives[929] = scratch.branch_derivatives[2357];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[930] = scratch.values[2378];
            scratch.node_derivatives[930] = scratch.node_derivatives[2378];
            scratch.branch_derivatives[930] = scratch.branch_derivatives[2378];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[931] = scratch.values[2385];
            scratch.node_derivatives[931] = scratch.node_derivatives[2385];
            scratch.branch_derivatives[931] = scratch.branch_derivatives[2385];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[932] = scratch.values[2392];
            scratch.node_derivatives[932] = scratch.node_derivatives[2392];
            scratch.branch_derivatives[932] = scratch.branch_derivatives[2392];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2025] = scratch.values[2367];
            scratch.node_derivatives[2025] = scratch.node_derivatives[2367];
            scratch.branch_derivatives[2025] = scratch.branch_derivatives[2367];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2026] = scratch.values[2388];
            scratch.node_derivatives[2026] = scratch.node_derivatives[2388];
            scratch.branch_derivatives[2026] = scratch.branch_derivatives[2388];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2027] = scratch.values[2386];
            scratch.node_derivatives[2027] = scratch.node_derivatives[2386];
            scratch.branch_derivatives[2027] = scratch.branch_derivatives[2386];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[925] = scratch.values[2010];
            scratch.node_derivatives[925] = scratch.node_derivatives[2010];
            scratch.branch_derivatives[925] = scratch.branch_derivatives[2010];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[926] = scratch.values[875];
            scratch.node_derivatives[926] = scratch.node_derivatives[875];
            scratch.branch_derivatives[926] = scratch.branch_derivatives[875];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[927] = scratch.values[876];
            scratch.node_derivatives[927] = scratch.node_derivatives[876];
            scratch.branch_derivatives[927] = scratch.branch_derivatives[876];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[928] = scratch.values[2011];
            scratch.node_derivatives[928] = scratch.node_derivatives[2011];
            scratch.branch_derivatives[928] = scratch.branch_derivatives[2011];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[929] = scratch.values[2012];
            scratch.node_derivatives[929] = scratch.node_derivatives[2012];
            scratch.branch_derivatives[929] = scratch.branch_derivatives[2012];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[930] = scratch.values[2013];
            scratch.node_derivatives[930] = scratch.node_derivatives[2013];
            scratch.branch_derivatives[930] = scratch.branch_derivatives[2013];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[931] = scratch.values[877];
            scratch.node_derivatives[931] = scratch.node_derivatives[877];
            scratch.branch_derivatives[931] = scratch.branch_derivatives[877];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[932] = scratch.values[2015];
            scratch.node_derivatives[932] = scratch.node_derivatives[2015];
            scratch.branch_derivatives[932] = scratch.branch_derivatives[2015];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[2025] = scratch.values[878];
            scratch.node_derivatives[2025] = scratch.node_derivatives[878];
            scratch.branch_derivatives[2025] = scratch.branch_derivatives[878];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[2026] = scratch.values[879];
            scratch.node_derivatives[2026] = scratch.node_derivatives[879];
            scratch.branch_derivatives[2026] = scratch.branch_derivatives[879];
        }

        if (!(scratch.values[2418] != 0.0)) {
            scratch.values[2027] = scratch.values[880];
            scratch.node_derivatives[2027] = scratch.node_derivatives[880];
            scratch.branch_derivatives[2027] = scratch.branch_derivatives[880];
        }

        scratch.store_ad(853, &AdValue::add(scratch.ad_value(849), scratch.ad_value(851)));

        scratch.values[2029] = scratch.values[269];
        scratch.node_derivatives[2029] = scratch.node_derivatives[269];
        scratch.branch_derivatives[2029] = scratch.branch_derivatives[269];

        scratch.values[2456] = if (scratch.values[812] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2456] != 0.0) {
            scratch.store_ad(2029, &AdValue::div(scratch.ad_value(269), AdValue::offset(AdValue::mul(scratch.ad_value(812), AdValue::powf(AdValue::add(AdValue::square(scratch.ad_value(926)), scratch.ad_value(772)), ((-1.0) * 0.16666666666666666))), 1.0)));
        }

        scratch.values[2457] = if (scratch.values[925] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2457] != 0.0) {
            scratch.values[933] = scratch.values[927];
            scratch.node_derivatives[933] = scratch.node_derivatives[927];
            scratch.branch_derivatives[933] = scratch.branch_derivatives[927];
        }

        if (scratch.values[2457] != 0.0) {
            scratch.values[934] = 0.0;
            scratch.node_derivatives[934] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[934] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2457] != 0.0) {
            scratch.values[935] = 0.0;
            scratch.node_derivatives[935] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[935] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2457] != 0.0) {
            scratch.values[936] = scratch.values[933];
            scratch.node_derivatives[936] = scratch.node_derivatives[933];
            scratch.branch_derivatives[936] = scratch.branch_derivatives[933];
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(937, &AdValue::scale(AdValue::div(scratch.ad_value(929), scratch.ad_value(932)), 0.5));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(938, &AdValue::square(scratch.ad_value(937)));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(939, &AdValue::mul(AdValue::sub_from_scalar(1.0, scratch.ad_value(931)), AdValue::sub(scratch.ad_value(930), AdValue::scale(AdValue::mul(scratch.ad_value(928), scratch.ad_value(929)), 0.5))));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(933, &AdValue::add(scratch.ad_value(927), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2025), scratch.ad_value(929)), AdValue::add(AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(937), scratch.ad_value(931)), 0.3333333333333333), (-1.0)), scratch.ad_value(931))), 0.5)));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(2086, &AdValue::scale(AdValue::mul(scratch.ad_value(928), scratch.ad_value(929)), 0.16666666666666666));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(934, &AdValue::add(AdValue::mul(scratch.ad_value(931), AdValue::add(scratch.ad_value(930), AdValue::mul(scratch.ad_value(2086), scratch.ad_value(937)))), scratch.ad_value(939)));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(935, &AdValue::scale(AdValue::add(AdValue::mul(AdValue::square(scratch.ad_value(931)), AdValue::sub(scratch.ad_value(930), AdValue::mul(scratch.ad_value(2086), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(937)), AdValue::scale(scratch.ad_value(938), 0.2))))), AdValue::mul(scratch.ad_value(939), AdValue::offset(scratch.ad_value(931), 1.0))), 0.5));
        }

        if (!(scratch.values[2457] != 0.0)) {
            scratch.store_ad(936, &AdValue::sub(scratch.ad_value(933), scratch.ad_value(934)));
        }

        scratch.store_ad(940, &AdValue::mul(scratch.ad_value(933), scratch.ad_value(2029)));

        scratch.store_ad(941, &AdValue::mul(AdValue::neg(scratch.ad_value(935)), scratch.ad_value(2029)));

        scratch.store_ad(942, &AdValue::mul(AdValue::neg(scratch.ad_value(936)), scratch.ad_value(2029)));

        scratch.store_ad(944, &AdValue::mul(scratch.ad_value(270), scratch.ad_value(889)));

        scratch.store_ad(945, &AdValue::mul(scratch.ad_value(271), scratch.ad_value(890)));

        scratch.store_ad(946, &AdValue::mul(scratch.ad_value(272), scratch.ad_value(853)));

        scratch.store_ad(947, &AdValue::mul(scratch.ad_value(273), scratch.ad_value(856)));

        scratch.store_ad(948, &AdValue::mul(scratch.ad_value(274), scratch.ad_value(859)));

        scratch.values[2038] = 0.0;

        scratch.values[2039] = 0.0;

        scratch.values[2040] = 0.0;

        scratch.values[2041] = 0.0;

        scratch.values[2042] = 0.0;

        scratch.values[2043] = 0.0;

        scratch.values[2044] = 0.0;

        scratch.values[2045] = 0.0;

        scratch.values[2458] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2459] = if (scratch.values[417] == 1.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.values[2462] = 0.0;
            scratch.node_derivatives[2462] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2462] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.values[2463] = 0.0;
            scratch.node_derivatives[2463] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2463] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(655, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(656, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(657, &AdValue::add(scratch.ad_value(854), AdValue::mul(scratch.ad_value(708), scratch.ad_value(656))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(658, &AdValue::add(scratch.ad_value(709), scratch.ad_value(657)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(659, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(657)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(660, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(659)), scratch.ad_value(655))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(2463, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(854), scratch.ad_value(709)), AdValue::add(scratch.ad_value(658), scratch.ad_value(660))), 2.0));
        }

        scratch.values[2464] = if (scratch.values[702] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2465] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2464] != 0.0)) && (scratch.values[2465] != 0.0)) {
            scratch.store_ad(2462, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2463), scratch.values[461]))));
        }

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2464] != 0.0)) && (!(scratch.values[2465] != 0.0))) {
            scratch.store_ad(2462, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2463), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2464] != 0.0)) {
            scratch.store_ad(2039, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(2462)), scratch.values[473]), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(2463)), scratch.values[476])));
        }

        scratch.values[2466] = if (scratch.values[703] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2467] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2466] != 0.0)) && (scratch.values[2467] != 0.0)) {
            scratch.store_ad(2462, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2463), scratch.values[462]))));
        }

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2466] != 0.0)) && (!(scratch.values[2467] != 0.0))) {
            scratch.store_ad(2462, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2463), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2466] != 0.0)) {
            scratch.store_ad(2040, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(2462)), scratch.values[474]), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(2463)), scratch.values[477])));
        }

        scratch.values[2468] = if (scratch.values[704] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2469] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2468] != 0.0)) && (scratch.values[2469] != 0.0)) {
            scratch.store_ad(2462, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2463), scratch.values[463]))));
        }

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2468] != 0.0)) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(2462, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2463), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2468] != 0.0)) {
            scratch.store_ad(2041, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(2462)), scratch.values[475]), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(2463)), scratch.values[478])));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.values[2462] = 0.0;
            scratch.node_derivatives[2462] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2462] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.values[2463] = 0.0;
            scratch.node_derivatives[2463] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2463] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(655, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(656, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(657, &AdValue::add(scratch.ad_value(855), AdValue::mul(scratch.ad_value(735), scratch.ad_value(656))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(658, &AdValue::add(scratch.ad_value(736), scratch.ad_value(657)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(659, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(657)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(660, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(659)), scratch.ad_value(655))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(2463, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(855), scratch.ad_value(736)), AdValue::add(scratch.ad_value(658), scratch.ad_value(660))), 2.0));
        }

        scratch.values[2470] = if (scratch.values[729] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2471] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2470] != 0.0)) && (scratch.values[2471] != 0.0)) {
            scratch.store_ad(2462, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2463), scratch.ad_value(604)))));
        }

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2470] != 0.0)) && (!(scratch.values[2471] != 0.0))) {
            scratch.store_ad(2462, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2463), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2470] != 0.0)) {
            scratch.store_ad(2043, &AdValue::add(AdValue::mul(scratch.ad_value(616), AdValue::sub_from_scalar(1.0, scratch.ad_value(2462))), AdValue::mul(scratch.ad_value(619), AdValue::sub(scratch.ad_value(855), scratch.ad_value(2463)))));
        }

        scratch.values[2472] = if (scratch.values[730] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2473] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2472] != 0.0)) && (scratch.values[2473] != 0.0)) {
            scratch.store_ad(2462, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2463), scratch.ad_value(605)))));
        }

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2472] != 0.0)) && (!(scratch.values[2473] != 0.0))) {
            scratch.store_ad(2462, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2463), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2472] != 0.0)) {
            scratch.store_ad(2044, &AdValue::add(AdValue::mul(scratch.ad_value(617), AdValue::sub_from_scalar(1.0, scratch.ad_value(2462))), AdValue::mul(scratch.ad_value(620), AdValue::sub(scratch.ad_value(855), scratch.ad_value(2463)))));
        }

        scratch.values[2474] = if (scratch.values[731] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[2475] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2474] != 0.0)) && (scratch.values[2475] != 0.0)) {
            scratch.store_ad(2462, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2463), scratch.ad_value(606)))));
        }

        if ((((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2474] != 0.0)) && (!(scratch.values[2475] != 0.0))) {
            scratch.store_ad(2462, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2463), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2474] != 0.0)) {
            scratch.store_ad(2045, &AdValue::add(AdValue::mul(scratch.ad_value(618), AdValue::sub_from_scalar(1.0, scratch.ad_value(2462))), AdValue::mul(scratch.ad_value(621), AdValue::sub(scratch.ad_value(855), scratch.ad_value(2463)))));
        }

        if ((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2476] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(655, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(656, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(657, &AdValue::add(scratch.ad_value(854), AdValue::mul(scratch.ad_value(708), scratch.ad_value(656))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(658, &AdValue::add(scratch.ad_value(709), scratch.ad_value(657)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(659, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(657)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(660, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(659)), scratch.ad_value(655))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(662, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(854), scratch.ad_value(709)), AdValue::add(scratch.ad_value(658), scratch.ad_value(660))), 2.0));
        }

        scratch.values[2477] = if (scratch.values[854] < scratch.values[705]) { 1.0 } else { 0.0 };

        scratch.values[2478] = if ((((0.5 * (scratch.values[854] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (scratch.values[2477] != 0.0)) && (scratch.values[2478] != 0.0)) {
            scratch.store_ad(664, &AdValue::exp(AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5))));
        }

        scratch.values[2479] = if ((0.5 * (scratch.values[854] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (scratch.values[2477] != 0.0)) && (!(scratch.values[2478] != 0.0))) && (scratch.values[2479] != 0.0)) {
            let assign52040_ad_e66820: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(664, &assign52040_ad_e66820);
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (scratch.values[2477] != 0.0)) && (!(scratch.values[2478] != 0.0))) && (!(scratch.values[2479] != 0.0))) {
            scratch.store_ad(664, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(854), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (scratch.values[2477] != 0.0)) {
            scratch.store_ad(661, &AdValue::square(scratch.ad_value(664)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (!(scratch.values[2477] != 0.0))) {
            scratch.store_ad(661, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(705)), scratch.values[427]), 1.0), scratch.ad_value(706)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (!(scratch.values[2477] != 0.0))) {
            scratch.store_ad(664, &AdValue::sqrt(scratch.ad_value(661)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(661, &AdValue::offset(scratch.ad_value(661), (-1.0)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1.0, scratch.ad_value(664)));
        }

        scratch.values[2480] = if (scratch.values[854] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (scratch.values[2480] != 0.0)) {
            scratch.store_ad(665, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(663), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(663), 1.0), AdValue::offset(scratch.ad_value(663), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) && (!(scratch.values[2480] != 0.0))) {
            scratch.store_ad(665, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(664), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(664), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(664), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(854)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(707), scratch.ad_value(665)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(667, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(854), scratch.ad_value(666)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(854), scratch.ad_value(666)), AdValue::sub(scratch.ad_value(854), scratch.ad_value(666))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(668, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(854), scratch.ad_value(710)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(854), scratch.ad_value(710)), AdValue::sub(scratch.ad_value(854), scratch.ad_value(710))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.store_ad(669, &AdValue::scale(AdValue::sub(scratch.ad_value(854), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(854), scratch.ad_value(854)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[2481] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2481] != 0.0)) {
            scratch.values[2039] = 0.0;
            scratch.node_derivatives[2039] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2039] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2482] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (scratch.values[2482] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(662), scratch.values[461]))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2482] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(662), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) {
            scratch.store_ad(2039, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(670)), scratch.values[473]), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(662)), scratch.values[476])));
        }

        scratch.values[2483] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(667)));
        }

        scratch.values[2485] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) && (scratch.values[2485] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::scale(scratch.ad_value(673), scratch.values[485])));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) && (!(scratch.values[2485] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::scale(scratch.ad_value(673), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(677, &AdValue::scale(scratch.ad_value(670), scratch.values[479]));
        }

        scratch.values[2486] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(680, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(677), scratch.values[464]), scratch.ad_value(673)), scratch.values[494]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(681, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[491]), scratch.ad_value(680)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(682, &AdValue::square(scratch.ad_value(681)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(683, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(682)), AdValue::offset(AdValue::square(scratch.ad_value(682)), 1.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(684, &AdValue::sqrt(AdValue::abs(scratch.ad_value(683))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(685, &AdValue::mul(scratch.ad_value(683), scratch.ad_value(684)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(688, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(680), scratch.ad_value(684)), 0.375)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(689, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(681), scratch.ad_value(684)), 2.0), scratch.ad_value(683)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(690, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(681), scratch.values[491]), scratch.ad_value(684)), AdValue::scale(scratch.ad_value(683), scratch.values[491])), AdValue::scale(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(691, &AdValue::mul(AdValue::offset(scratch.ad_value(689), (-1.0)), scratch.ad_value(688)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(652, &AdValue::square(scratch.ad_value(691)));
        }

        scratch.values[2489] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (scratch.values[2489] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2489] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2490] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2491] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2491] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2491] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2492] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2493] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) && (scratch.values[2493] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(668)), scratch.values[485])));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) && (!(scratch.values[2493] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(668)), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) {
            scratch.store_ad(695, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(668)), scratch.values[482]), scratch.ad_value(670)), scratch.values[467]));
        }

        scratch.values[2494] = if (((((-scratch.values[497]) / scratch.values[695])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) && (scratch.values[2494] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695))));
        }

        scratch.values[2495] = if (((-scratch.values[497]) / scratch.values[695]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) && (!(scratch.values[2494] != 0.0))) && (scratch.values[2495] != 0.0)) {
            let assign52810_ad_e68151: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, assign52810_ad_e68151));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) && (!(scratch.values[2494] != 0.0))) && (!(scratch.values[2495] != 0.0))) {
            let assign52820_ad_e68202: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(695)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(670, &assign52820_ad_e68202);
        }

        scratch.values[2496] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2497] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[2498] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2497] != 0.0)) && (scratch.values[2498] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(669), scratch.values[504]), AdValue::scale(scratch.ad_value(669), scratch.values[504])), AdValue::scale(scratch.ad_value(669), scratch.values[504])), AdValue::scale(scratch.ad_value(669), scratch.values[504])));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2497] != 0.0)) && (!(scratch.values[2498] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(669), scratch.values[504])), scratch.values[414]));
        }

        scratch.values[2499] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2499] != 0.0)) {
            scratch.values[2040] = 0.0;
            scratch.node_derivatives[2040] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2040] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2500] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (scratch.values[2500] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(662), scratch.values[462]))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2500] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(662), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) {
            scratch.store_ad(2040, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(670)), scratch.values[474]), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(662)), scratch.values[477])));
        }

        scratch.values[2501] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(667)));
        }

        scratch.values[2503] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) && (scratch.values[2503] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::scale(scratch.ad_value(673), scratch.values[486])));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) && (!(scratch.values[2503] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::scale(scratch.ad_value(673), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(677, &AdValue::scale(scratch.ad_value(670), scratch.values[480]));
        }

        scratch.values[2504] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(680, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(677), scratch.values[465]), scratch.ad_value(673)), scratch.values[495]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(681, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[492]), scratch.ad_value(680)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(682, &AdValue::square(scratch.ad_value(681)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(683, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(682)), AdValue::offset(AdValue::square(scratch.ad_value(682)), 1.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(684, &AdValue::sqrt(AdValue::abs(scratch.ad_value(683))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(685, &AdValue::mul(scratch.ad_value(683), scratch.ad_value(684)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(688, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(680), scratch.ad_value(684)), 0.375)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(689, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(681), scratch.ad_value(684)), 2.0), scratch.ad_value(683)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(690, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(681), scratch.values[492]), scratch.ad_value(684)), AdValue::scale(scratch.ad_value(683), scratch.values[492])), AdValue::scale(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(691, &AdValue::mul(AdValue::offset(scratch.ad_value(689), (-1.0)), scratch.ad_value(688)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(652, &AdValue::square(scratch.ad_value(691)));
        }

        scratch.values[2507] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (scratch.values[2507] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2507] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2508] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2509] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2509] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2509] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2510] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2511] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) && (scratch.values[2511] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(668)), scratch.values[486])));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) && (!(scratch.values[2511] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(668)), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) {
            scratch.store_ad(695, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(668)), scratch.values[483]), scratch.ad_value(670)), scratch.values[468]));
        }

        scratch.values[2512] = if (((((-scratch.values[498]) / scratch.values[695])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) && (scratch.values[2512] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695))));
        }

        scratch.values[2513] = if (((-scratch.values[498]) / scratch.values[695]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) && (!(scratch.values[2512] != 0.0))) && (scratch.values[2513] != 0.0)) {
            let assign53560_ad_e69414: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, assign53560_ad_e69414));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) && (!(scratch.values[2512] != 0.0))) && (!(scratch.values[2513] != 0.0))) {
            let assign53570_ad_e69465: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(695)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(670, &assign53570_ad_e69465);
        }

        scratch.values[2514] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2515] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[2516] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2515] != 0.0)) && (scratch.values[2516] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(669), scratch.values[505]), AdValue::scale(scratch.ad_value(669), scratch.values[505])), AdValue::scale(scratch.ad_value(669), scratch.values[505])), AdValue::scale(scratch.ad_value(669), scratch.values[505])));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2515] != 0.0)) && (!(scratch.values[2516] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(669), scratch.values[505])), scratch.values[415]));
        }

        scratch.values[2517] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2517] != 0.0)) {
            scratch.values[2041] = 0.0;
            scratch.node_derivatives[2041] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2041] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2518] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (scratch.values[2518] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(662), scratch.values[463]))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2518] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(662), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) {
            scratch.store_ad(2041, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(670)), scratch.values[475]), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(662)), scratch.values[478])));
        }

        scratch.values[2519] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(667)));
        }

        scratch.values[2521] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) && (scratch.values[2521] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::scale(scratch.ad_value(673), scratch.values[487])));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) && (!(scratch.values[2521] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::scale(scratch.ad_value(673), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(677, &AdValue::scale(scratch.ad_value(670), scratch.values[481]));
        }

        scratch.values[2522] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(680, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(677), scratch.values[466]), scratch.ad_value(673)), scratch.values[496]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(681, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[493]), scratch.ad_value(680)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(682, &AdValue::square(scratch.ad_value(681)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(683, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(682)), AdValue::offset(AdValue::square(scratch.ad_value(682)), 1.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(684, &AdValue::sqrt(AdValue::abs(scratch.ad_value(683))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(685, &AdValue::mul(scratch.ad_value(683), scratch.ad_value(684)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(688, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(680), scratch.ad_value(684)), 0.375)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(689, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(681), scratch.ad_value(684)), 2.0), scratch.ad_value(683)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(690, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(681), scratch.values[493]), scratch.ad_value(684)), AdValue::scale(scratch.ad_value(683), scratch.values[493])), AdValue::scale(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(691, &AdValue::mul(AdValue::offset(scratch.ad_value(689), (-1.0)), scratch.ad_value(688)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(652, &AdValue::square(scratch.ad_value(691)));
        }

        scratch.values[2525] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (scratch.values[2525] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2525] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2526] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2527] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2527] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2527] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2528] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2529] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (scratch.values[2529] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(668)), scratch.values[487])));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (!(scratch.values[2529] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(668)), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(695, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(668)), scratch.values[484]), scratch.ad_value(670)), scratch.values[469]));
        }

        scratch.values[2530] = if (((((-scratch.values[499]) / scratch.values[695])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (scratch.values[2530] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695))));
        }

        scratch.values[2531] = if (((-scratch.values[499]) / scratch.values[695]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (!(scratch.values[2530] != 0.0))) && (scratch.values[2531] != 0.0)) {
            let assign54310_ad_e70677: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, assign54310_ad_e70677));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (!(scratch.values[2530] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            let assign54320_ad_e70728: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(695)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(670, &assign54320_ad_e70728);
        }

        scratch.values[2532] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2533] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[2534] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2532] != 0.0))) && (scratch.values[2533] != 0.0)) && (scratch.values[2534] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(669), scratch.values[506]), AdValue::scale(scratch.ad_value(669), scratch.values[506])), AdValue::scale(scratch.ad_value(669), scratch.values[506])), AdValue::scale(scratch.ad_value(669), scratch.values[506])));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2532] != 0.0))) && (scratch.values[2533] != 0.0)) && (!(scratch.values[2534] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(669), scratch.values[506])), scratch.values[416]));
        }

        if ((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2535] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(655, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(656, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(657, &AdValue::add(scratch.ad_value(855), AdValue::mul(scratch.ad_value(735), scratch.ad_value(656))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(658, &AdValue::add(scratch.ad_value(736), scratch.ad_value(657)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(659, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(657)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(660, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(659)), scratch.ad_value(655))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(662, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(855), scratch.ad_value(736)), AdValue::add(scratch.ad_value(658), scratch.ad_value(660))), 2.0));
        }

        scratch.values[2536] = if (scratch.values[855] < scratch.values[732]) { 1.0 } else { 0.0 };

        scratch.values[2537] = if ((((0.5 * (scratch.values[855] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (scratch.values[2536] != 0.0)) && (scratch.values[2537] != 0.0)) {
            scratch.store_ad(664, &AdValue::exp(AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5))));
        }

        scratch.values[2538] = if ((0.5 * (scratch.values[855] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (scratch.values[2536] != 0.0)) && (!(scratch.values[2537] != 0.0))) && (scratch.values[2538] != 0.0)) {
            let assign54580_ad_e71110: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(664, &assign54580_ad_e71110);
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (scratch.values[2536] != 0.0)) && (!(scratch.values[2537] != 0.0))) && (!(scratch.values[2538] != 0.0))) {
            scratch.store_ad(664, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(855), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (scratch.values[2536] != 0.0)) {
            scratch.store_ad(661, &AdValue::square(scratch.ad_value(664)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (!(scratch.values[2536] != 0.0))) {
            scratch.store_ad(661, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(855), scratch.ad_value(732)), scratch.values[427]), 1.0), scratch.ad_value(733)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (!(scratch.values[2536] != 0.0))) {
            scratch.store_ad(664, &AdValue::sqrt(scratch.ad_value(661)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(661, &AdValue::offset(scratch.ad_value(661), (-1.0)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1.0, scratch.ad_value(664)));
        }

        scratch.values[2539] = if (scratch.values[855] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (scratch.values[2539] != 0.0)) {
            scratch.store_ad(665, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(663), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(663), 1.0), AdValue::offset(scratch.ad_value(663), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) && (!(scratch.values[2539] != 0.0))) {
            scratch.store_ad(665, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(664), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(664), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(664), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(855)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(734), scratch.ad_value(665)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(667, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(855), scratch.ad_value(666)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(855), scratch.ad_value(666)), AdValue::sub(scratch.ad_value(855), scratch.ad_value(666))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(668, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(855), scratch.ad_value(737)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(855), scratch.ad_value(737)), AdValue::sub(scratch.ad_value(855), scratch.ad_value(737))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.store_ad(669, &AdValue::scale(AdValue::sub(scratch.ad_value(855), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(855), scratch.ad_value(855)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[2540] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2540] != 0.0)) {
            scratch.values[2043] = 0.0;
            scratch.node_derivatives[2043] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2043] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2541] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (scratch.values[2541] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(662), scratch.ad_value(604)))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2541] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(662), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) {
            scratch.store_ad(2043, &AdValue::add(AdValue::mul(scratch.ad_value(616), AdValue::sub_from_scalar(1.0, scratch.ad_value(670))), AdValue::mul(scratch.ad_value(619), AdValue::sub(scratch.ad_value(855), scratch.ad_value(662)))));
        }

        scratch.values[2542] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(667)));
        }

        scratch.values[2544] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) && (scratch.values[2544] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::mul(scratch.ad_value(673), scratch.ad_value(628))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) && (!(scratch.values[2544] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::mul(scratch.ad_value(673), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(677, &AdValue::mul(scratch.ad_value(622), scratch.ad_value(670)));
        }

        scratch.values[2545] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(680, &AdValue::mul(scratch.ad_value(637), AdValue::div(AdValue::mul(scratch.ad_value(677), scratch.ad_value(607)), scratch.ad_value(673))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(681, &AdValue::div(AdValue::scale(scratch.ad_value(634), 0.666666666666667), scratch.ad_value(680)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(682, &AdValue::square(scratch.ad_value(681)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(683, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(682)), AdValue::offset(AdValue::square(scratch.ad_value(682)), 1.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(684, &AdValue::sqrt(AdValue::abs(scratch.ad_value(683))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(685, &AdValue::mul(scratch.ad_value(683), scratch.ad_value(684)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(688, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(680), scratch.ad_value(684)), 0.375)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(689, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(681), scratch.ad_value(684)), 2.0), scratch.ad_value(683)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(690, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(634), scratch.ad_value(681)), scratch.ad_value(684)), AdValue::mul(scratch.ad_value(634), scratch.ad_value(683))), AdValue::scale(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(691, &AdValue::mul(AdValue::offset(scratch.ad_value(689), (-1.0)), scratch.ad_value(688)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(652, &AdValue::square(scratch.ad_value(691)));
        }

        scratch.values[2548] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (scratch.values[2548] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2548] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2549] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2550] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2550] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2550] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2551] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2552] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) && (scratch.values[2552] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(668)), scratch.ad_value(628))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) && (!(scratch.values[2552] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(668)), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) {
            scratch.store_ad(695, &AdValue::mul(scratch.ad_value(610), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(668)), scratch.ad_value(625)), scratch.ad_value(670))));
        }

        scratch.values[2553] = if (((((-scratch.values[640]) / scratch.values[695])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) && (scratch.values[2553] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695))));
        }

        scratch.values[2554] = if (((-scratch.values[640]) / scratch.values[695]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) && (!(scratch.values[2553] != 0.0))) && (scratch.values[2554] != 0.0)) {
            let assign55350_ad_e72441: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, assign55350_ad_e72441));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) && (!(scratch.values[2553] != 0.0))) && (!(scratch.values[2554] != 0.0))) {
            let assign55360_ad_e72492: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(695)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(670, &assign55360_ad_e72492);
        }

        scratch.values[2555] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2556] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[2557] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2556] != 0.0)) && (scratch.values[2557] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(669), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2556] != 0.0)) && (!(scratch.values[2557] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        scratch.values[2558] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2558] != 0.0)) {
            scratch.values[2044] = 0.0;
            scratch.node_derivatives[2044] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2044] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2559] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (scratch.values[2559] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(662), scratch.ad_value(605)))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2559] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(662), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) {
            scratch.store_ad(2044, &AdValue::add(AdValue::mul(scratch.ad_value(617), AdValue::sub_from_scalar(1.0, scratch.ad_value(670))), AdValue::mul(scratch.ad_value(620), AdValue::sub(scratch.ad_value(855), scratch.ad_value(662)))));
        }

        scratch.values[2560] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(667)));
        }

        scratch.values[2562] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) && (scratch.values[2562] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::mul(scratch.ad_value(673), scratch.ad_value(629))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) && (!(scratch.values[2562] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::mul(scratch.ad_value(673), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(677, &AdValue::mul(scratch.ad_value(623), scratch.ad_value(670)));
        }

        scratch.values[2563] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(680, &AdValue::mul(scratch.ad_value(638), AdValue::div(AdValue::mul(scratch.ad_value(677), scratch.ad_value(608)), scratch.ad_value(673))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(681, &AdValue::div(AdValue::scale(scratch.ad_value(635), 0.666666666666667), scratch.ad_value(680)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(682, &AdValue::square(scratch.ad_value(681)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(683, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(682)), AdValue::offset(AdValue::square(scratch.ad_value(682)), 1.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(684, &AdValue::sqrt(AdValue::abs(scratch.ad_value(683))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(685, &AdValue::mul(scratch.ad_value(683), scratch.ad_value(684)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(688, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(680), scratch.ad_value(684)), 0.375)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(689, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(681), scratch.ad_value(684)), 2.0), scratch.ad_value(683)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(690, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(635), scratch.ad_value(681)), scratch.ad_value(684)), AdValue::mul(scratch.ad_value(635), scratch.ad_value(683))), AdValue::scale(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(691, &AdValue::mul(AdValue::offset(scratch.ad_value(689), (-1.0)), scratch.ad_value(688)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(652, &AdValue::square(scratch.ad_value(691)));
        }

        scratch.values[2566] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (scratch.values[2566] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2566] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2567] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2568] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2568] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2568] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2569] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2570] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) && (scratch.values[2570] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(668)), scratch.ad_value(629))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) && (!(scratch.values[2570] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(668)), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) {
            scratch.store_ad(695, &AdValue::mul(scratch.ad_value(611), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(668)), scratch.ad_value(626)), scratch.ad_value(670))));
        }

        scratch.values[2571] = if (((((-scratch.values[641]) / scratch.values[695])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) && (scratch.values[2571] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695))));
        }

        scratch.values[2572] = if (((-scratch.values[641]) / scratch.values[695]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) && (!(scratch.values[2571] != 0.0))) && (scratch.values[2572] != 0.0)) {
            let assign56100_ad_e73704: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, assign56100_ad_e73704));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) && (!(scratch.values[2571] != 0.0))) && (!(scratch.values[2572] != 0.0))) {
            let assign56110_ad_e73755: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(695)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(670, &assign56110_ad_e73755);
        }

        scratch.values[2573] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2574] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[2575] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2574] != 0.0)) && (scratch.values[2575] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(669), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2574] != 0.0)) && (!(scratch.values[2575] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        scratch.values[2576] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2576] != 0.0)) {
            scratch.values[2045] = 0.0;
            scratch.node_derivatives[2045] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2045] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2577] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (scratch.values[2577] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(662), scratch.ad_value(606)))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2577] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(662), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) {
            scratch.store_ad(2045, &AdValue::add(AdValue::mul(scratch.ad_value(618), AdValue::sub_from_scalar(1.0, scratch.ad_value(670))), AdValue::mul(scratch.ad_value(621), AdValue::sub(scratch.ad_value(855), scratch.ad_value(662)))));
        }

        scratch.values[2578] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(667)));
        }

        scratch.values[2580] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) && (scratch.values[2580] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::mul(scratch.ad_value(673), scratch.ad_value(630))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) && (!(scratch.values[2580] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::mul(scratch.ad_value(673), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(677, &AdValue::mul(scratch.ad_value(624), scratch.ad_value(670)));
        }

        scratch.values[2581] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(680, &AdValue::mul(scratch.ad_value(639), AdValue::div(AdValue::mul(scratch.ad_value(677), scratch.ad_value(609)), scratch.ad_value(673))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(681, &AdValue::div(AdValue::scale(scratch.ad_value(636), 0.666666666666667), scratch.ad_value(680)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(682, &AdValue::square(scratch.ad_value(681)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(683, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(682)), AdValue::offset(AdValue::square(scratch.ad_value(682)), 1.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(684, &AdValue::sqrt(AdValue::abs(scratch.ad_value(683))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(685, &AdValue::mul(scratch.ad_value(683), scratch.ad_value(684)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(688, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(680), scratch.ad_value(684)), 0.375)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(689, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(681), scratch.ad_value(684)), 2.0), scratch.ad_value(683)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(690, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(681)), scratch.ad_value(684)), AdValue::mul(scratch.ad_value(636), scratch.ad_value(683))), AdValue::scale(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(691, &AdValue::mul(AdValue::offset(scratch.ad_value(689), (-1.0)), scratch.ad_value(688)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(652, &AdValue::square(scratch.ad_value(691)));
        }

        scratch.values[2584] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (scratch.values[2584] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2584] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2585] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2586] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2585] != 0.0))) && (scratch.values[2586] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2585] != 0.0))) && (!(scratch.values[2586] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2587] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[2588] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) && (scratch.values[2588] != 0.0)) {
            scratch.store_ad(670, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(668)), scratch.ad_value(630))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) && (!(scratch.values[2588] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(668)), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) {
            scratch.store_ad(695, &AdValue::mul(scratch.ad_value(612), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(668)), scratch.ad_value(627)), scratch.ad_value(670))));
        }

        scratch.values[2589] = if (((((-scratch.values[642]) / scratch.values[695])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) && (scratch.values[2589] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695))));
        }

        scratch.values[2590] = if (((-scratch.values[642]) / scratch.values[695]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) && (!(scratch.values[2589] != 0.0))) && (scratch.values[2590] != 0.0)) {
            let assign56850_ad_e74967: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, assign56850_ad_e74967));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) && (!(scratch.values[2589] != 0.0))) && (!(scratch.values[2590] != 0.0))) {
            let assign56860_ad_e75018: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(695)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(670, &assign56860_ad_e75018);
        }

        scratch.values[2591] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[2592] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[2593] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2591] != 0.0))) && (scratch.values[2592] != 0.0)) && (scratch.values[2593] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(669), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2591] != 0.0))) && (scratch.values[2592] != 0.0)) && (!(scratch.values[2593] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        scratch.store_ad(943, &AdValue::neg(AdValue::add(AdValue::add(scratch.ad_value(940), scratch.ad_value(942)), scratch.ad_value(941))));

        scratch.store_ad(947, &AdValue::add(scratch.ad_value(947), scratch.ad_value(944)));

        scratch.store_ad(948, &AdValue::add(scratch.ad_value(948), scratch.ad_value(945)));

        scratch.store_ad(2038, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(2039)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(2040))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(2041))));

        scratch.store_ad(2042, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(2043)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(2044))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(2045))));

        scratch.values[2608] = if (scratch.values[1999] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2608] != 0.0) {
            scratch.values[2607] = scratch.values[941];
            scratch.node_derivatives[2607] = scratch.node_derivatives[941];
            scratch.branch_derivatives[2607] = scratch.branch_derivatives[941];
        }

        if (scratch.values[2608] != 0.0) {
            scratch.values[941] = scratch.values[943];
            scratch.node_derivatives[941] = scratch.node_derivatives[943];
            scratch.branch_derivatives[941] = scratch.branch_derivatives[943];
        }

        if (scratch.values[2608] != 0.0) {
            scratch.values[943] = scratch.values[2607];
            scratch.node_derivatives[943] = scratch.node_derivatives[2607];
            scratch.branch_derivatives[943] = scratch.branch_derivatives[2607];
        }

        scratch.store_ad(1002, &AdValue::mul(scratch.ad_value(2029), scratch.ad_value(2025)));

        scratch.values[2609] = if (((scratch.values[2010] > 0.0) && (scratch.values[25] > 0.0)) && (scratch.values[2075] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[2613] = if ((scratch.values[10] == 1.0) && (scratch.values[2078] > 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(1002, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2026)), scratch.ad_value(2029)), scratch.ad_value(2025)), AdValue::square(scratch.ad_value(2027))));
        }

        scratch.values[2617] = if (((scratch.values[8] != 0.0) && (scratch.values[286] > 0.0)) && (scratch.values[2058] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2087, &AdValue::div(AdValue::scale(scratch.ad_value(2067), 4.0), scratch.ad_value(2082)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(810), scratch.ad_value(2074)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(2011), scratch.ad_value(2015)));
        }

    }
}
